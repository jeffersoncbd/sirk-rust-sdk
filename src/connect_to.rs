use crate::{Error, Sirk, http::HttpTransport};
use std::path::Path;

impl Sirk {
    pub fn connect_to(
        endpoint: impl AsRef<str>,
        directory: impl AsRef<Path>,
    ) -> Result<Self, Error> {
        let endpoint = endpoint.as_ref().trim_end_matches('/');
        if endpoint.is_empty() {
            return Err(Error::Protocol(
                "HTTP endpoint must not be empty".to_owned(),
            ));
        }
        let transport = HttpTransport::new(endpoint.to_owned());
        transport.health()?;
        Ok(Self {
            directory: directory.as_ref().to_owned(),
            local_directory: std::env::current_dir().map_err(Error::CurrentDirectory)?,
            transport: Box::new(transport),
        })
    }
}
