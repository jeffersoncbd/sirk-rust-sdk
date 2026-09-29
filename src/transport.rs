use crate::Error;
use std::path::Path;

pub(super) trait Transport: Send + Sync {
    fn create_flow(&self, directory: &Path) -> Result<String, Error>;

    fn run_agent(
        &self,
        directory: &Path,
        flow_id: &str,
        agent: &str,
        input: &str,
    ) -> Result<String, Error>;

    fn tree(&self, directory: &Path, flow_id: &str) -> Result<Vec<String>, Error>;

    fn git_status(&self, directory: &Path, flow_id: &str) -> Result<Vec<String>, Error>;

    fn git_add(&self, directory: &Path, flow_id: &str) -> Result<(), Error>;
}
