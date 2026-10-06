use super::HttpTransport;
use crate::{
    Error,
    protocol::{
        AgentRequest, AgentResponse, DirectoryRequest, FlowResponse, PathsResponse, StatusResponse,
    },
    transport::{AgentRunResponse, Transport},
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
        conversation_id: Option<&str>,
        input: &str,
    ) -> Result<AgentRunResponse, Error> {
        let body = serde_json::to_string(&AgentRequest {
            directory,
            agent,
            conversation_id,
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
        agent_run_response(response)
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

fn agent_run_response(response: AgentResponse) -> Result<AgentRunResponse, Error> {
    let conversation_id = response.conversation_id.ok_or_else(|| {
        Error::Protocol("agent response did not contain a conversationId".to_owned())
    })?;
    match (response.ask, response.result) {
        (Some(question), None) => Ok(AgentRunResponse::Ask {
            question,
            conversation_id,
        }),
        (None, Some(result)) => Ok(AgentRunResponse::Result(result)),
        _ => Err(Error::Protocol(
            "agent response must contain exactly one of ask or result".to_owned(),
        )),
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

#[cfg(test)]
mod tests {
    use super::agent_run_response;
    use crate::{protocol::AgentResponse, transport::AgentRunResponse};

    #[test]
    fn recognizes_an_agent_question() {
        let response = agent_run_response(AgentResponse {
            ask: Some("What is your name?".to_owned()),
            conversation_id: Some("conversation-18f-1234-0".to_owned()),
            result: None,
            error: None,
        })
        .unwrap();

        assert!(
            matches!(response, AgentRunResponse::Ask { question, conversation_id } if question == "What is your name?" && conversation_id == "conversation-18f-1234-0")
        );
    }

    #[test]
    fn rejects_an_agent_response_without_a_question_or_result() {
        let error = agent_run_response(AgentResponse {
            ask: None,
            conversation_id: Some("conversation-18f-1234-0".to_owned()),
            result: None,
            error: None,
        })
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "S.I.R.K. protocol error: agent response must contain exactly one of ask or result"
        );
    }

    #[test]
    fn rejects_an_agent_response_without_a_conversation_id() {
        let error = agent_run_response(AgentResponse {
            ask: Some("What is your name?".to_owned()),
            conversation_id: None,
            result: None,
            error: None,
        })
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "S.I.R.K. protocol error: agent response did not contain a conversationId"
        );
    }
}
