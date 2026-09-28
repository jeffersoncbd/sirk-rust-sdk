use crate::Error;
use std::{fs, path::Path};

pub(super) fn visible_file(root: &Path, path: &str) -> Result<bool, Error> {
    match fs::symlink_metadata(root.join(path)) {
        Ok(metadata) => Ok(metadata.is_file() || metadata.file_type().is_symlink()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(Error::Tool(format!("could not inspect `{path}`: {error}"))),
    }
}
