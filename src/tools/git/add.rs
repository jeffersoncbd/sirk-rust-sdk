use super::Git;
use crate::Error;

impl Git<'_> {
    pub fn add(&self) -> Result<(), Error> {
        self.sirk
            .transport
            .git_add(&self.sirk.directory, &self.sirk.flow_id)
    }
}
