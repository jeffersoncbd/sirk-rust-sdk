mod add;
mod status;

use super::Tools;
use std::path::PathBuf;

pub struct Git {
    pub(super) directory: PathBuf,
}

impl Tools {
    pub fn git(&self) -> Git {
        Git {
            directory: self.directory.clone(),
        }
    }
}
