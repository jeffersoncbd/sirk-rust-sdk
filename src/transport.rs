use crate::Error;
use std::path::Path;

pub(super) trait Transport: Send + Sync {
    fn run_agent(&self, directory: &Path, agent: &str, input: &str) -> Result<String, Error>;
}
