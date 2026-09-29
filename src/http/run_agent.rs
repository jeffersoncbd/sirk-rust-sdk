use super::HttpTransport;
use crate::{
    Error,
    protocol::{
        AgentRequest, AgentResponse, DirectoryRequest, FlowResponse, PathsResponse, StatusResponse,
    },
    transport::Transport,
};
use std::path::Path;

impl Transport for HttpTransport {
    fn create_flow(&self, directory: &Path) -> Result<String, Error> {
        let body = serde_json::to_string(&DirectoryRequest { directory })?;
        let response = self
            .agent
            .post(format!("{}/v1/flows", self.endpoint))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send(body)
            .map_err(|source| Error::Unavailable {
                endpoint: self.endpoint.clone(),
                source,
            })?;
        let (status, body) = super::read::read(response, &self.endpoint)?;
        if status != 201 {
            return Err(Error::Remote {
                status,
                message: body,
            });
        }
        let response: FlowResponse = serde_json::from_str(&body)?;
        response
            .flow_id
            .ok_or_else(|| Error::Protocol("response did not contain a flowId".to_owned()))
    }

    fn run_agent(
        &self,
        directory: &Path,
        flow_id: &str,
        agent: &str,
        input: &str,
    ) -> Result<String, Error> {
        let body = serde_json::to_string(&AgentRequest {
            directory,
            agent,
            input,
        })?;
        let response = self
            .agent
            .post(format!("{}/v1/agent/run", self.endpoint))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-Sirk-Flow-Id", flow_id)
            .send(body)
            .map_err(|source| Error::Unavailable {
                endpoint: self.endpoint.clone(),
                source,
            })?;
        let (status, body) = super::read::read(response, &self.endpoint)?;
        if status != 200 {
            let message = serde_json::from_str::<AgentResponse>(&body)
                .ok()
                .and_then(|response| response.error)
                .unwrap_or(body);
            return Err(Error::Remote { status, message });
        }
        let response: AgentResponse = serde_json::from_str(&body)?;
        response
            .result
            .ok_or_else(|| Error::Protocol("response did not contain a result".to_owned()))
    }

    fn tree(&self, directory: &Path, flow_id: &str) -> Result<Vec<String>, Error> {
        self.paths("/v1/tree", directory, flow_id)
    }

    fn git_status(&self, directory: &Path, flow_id: &str) -> Result<Vec<String>, Error> {
        self.paths("/v1/git/status", directory, flow_id)
    }

    fn git_add(&self, directory: &Path, flow_id: &str) -> Result<(), Error> {
        let body = serde_json::to_string(&DirectoryRequest { directory })?;
        let response = self
            .agent
            .post(format!("{}/v1/git/add", self.endpoint))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-Sirk-Flow-Id", flow_id)
            .send(body)
            .map_err(|source| Error::Unavailable {
                endpoint: self.endpoint.clone(),
                source,
            })?;
        let (status, body) = super::read::read(response, &self.endpoint)?;
        if status != 200 {
            return Err(Error::Remote {
                status,
                message: body,
            });
        }
        let response: StatusResponse = serde_json::from_str(&body)?;
        if response.status.as_deref() != Some("ok") {
            return Err(Error::Protocol(
                "response did not contain an ok status".to_owned(),
            ));
        }
        Ok(())
    }
}

impl HttpTransport {
    fn paths(&self, path: &str, directory: &Path, flow_id: &str) -> Result<Vec<String>, Error> {
        let body = serde_json::to_string(&DirectoryRequest { directory })?;
        let response = self
            .agent
            .post(format!("{}{}", self.endpoint, path))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-Sirk-Flow-Id", flow_id)
            .send(body)
            .map_err(|source| Error::Unavailable {
                endpoint: self.endpoint.clone(),
                source,
            })?;
        let (status, body) = super::read::read(response, &self.endpoint)?;
        if status != 200 {
            return Err(Error::Remote {
                status,
                message: body,
            });
        }
        let response: PathsResponse = serde_json::from_str(&body)?;
        response
            .paths
            .ok_or_else(|| Error::Protocol("response did not contain paths".to_owned()))
    }
}
