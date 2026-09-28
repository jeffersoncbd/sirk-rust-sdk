mod await_confirm;
mod git;
mod project;
mod tree;

use crate::Sirk;
use std::path::PathBuf;

pub struct Tools {
    pub(super) directory: PathBuf,
}

pub use git::Git;

impl Sirk {
    pub fn tools(&self) -> Tools {
        Tools {
            directory: self.local_directory.clone(),
        }
    }
}

#[cfg(test)]
#[path = "tools/tests.rs"]
mod tests;
