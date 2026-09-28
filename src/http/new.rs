use super::HttpTransport;
use std::time::Duration;

impl HttpTransport {
    pub(crate) fn new(endpoint: String) -> Self {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .timeout_resolve(Some(Duration::from_secs(5)))
            .timeout_connect(Some(Duration::from_secs(5)))
            .build();
        Self {
            agent: config.new_agent(),
            endpoint,
        }
    }
}
