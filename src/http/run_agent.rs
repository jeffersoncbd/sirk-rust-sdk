use super::HttpTransport;
use crate::{
    Error,
    protocol::{AgentRequest, AgentResponse},
    transport::Transport,
};
use std::path::Path;

impl Transport for HttpTransport {
    fn run_agent(&self, directory: &Path, agent: &str, input: &str) -> Result<String, Error> {
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
}
