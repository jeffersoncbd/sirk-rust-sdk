use crate::Error;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub(super) struct Project {
    pub(super) root: PathBuf,
    pub(super) prefix: String,
}

pub(super) fn project(directory: &Path) -> Result<Project, Error> {
    let root = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(directory)
        .output()
        .map_err(|error| Error::Tool(format!("could not locate the Git project: {error}")))?;
    if !root.status.success() {
        return Err(Error::Tool(format!(
            "Git tools require an accessible Git working tree: {}",
            String::from_utf8_lossy(&root.stderr).trim()
        )));
    }
    let prefix = Command::new("git")
        .args(["rev-parse", "--show-prefix"])
        .current_dir(directory)
        .output()
        .map_err(|error| Error::Tool(format!("could not resolve the Git project path: {error}")))?;
    if !prefix.status.success() {
        return Err(Error::Tool(format!(
            "could not resolve the Git project path: {}",
            String::from_utf8_lossy(&prefix.stderr).trim()
        )));
    }
    let root = String::from_utf8(root.stdout)
        .map_err(|error| Error::Tool(format!("Git project path is not UTF-8: {error}")))?;
    let prefix = String::from_utf8(prefix.stdout)
        .map_err(|error| Error::Tool(format!("Git project path is not UTF-8: {error}")))?;
    Ok(Project {
        root: PathBuf::from(root.trim_end_matches(['\r', '\n'])),
        prefix: prefix.trim_end_matches(['\r', '\n']).to_owned(),
    })
}
