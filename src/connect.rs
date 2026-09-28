use crate::{Error, Sirk};

impl Sirk {
    pub fn connect() -> Result<Self, Error> {
        let directory = std::env::current_dir().map_err(Error::CurrentDirectory)?;
        Self::connect_to("http://127.0.0.1:8080", directory)
    }
}
