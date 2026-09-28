use super::Git;
use crate::Error;
use std::process::Command;

impl Git {
    pub fn add(&self) -> Result<(), Error> {
        let output = Command::new("git")
            .args(["add", "--all", "--", "."])
            .current_dir(&self.directory)
            .output()
            .map_err(|error| Error::Tool(format!("could not run `git add`: {error}")))?;
        if output.status.success() {
            return Ok(());
        }
        Err(Error::Tool(format!(
            "`git add` exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}
