use super::HttpTransport;
use crate::{Error, protocol::HealthResponse};

impl HttpTransport {
    pub(crate) fn health(&self) -> Result<(), Error> {
        let response = self
            .agent
            .get(format!("{}/health", self.endpoint))
            .call()
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
        let response: HealthResponse = serde_json::from_str(&body)?;
        if response.status != "ok" {
            return Err(Error::Protocol("health check did not return ok".to_owned()));
        }
        Ok(())
    }
}
