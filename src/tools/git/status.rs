use super::Git;
use crate::Error;

impl Git<'_> {
    pub fn status(&self) -> Result<Vec<String>, Error> {
        self.sirk
            .transport
            .git_status(&self.sirk.directory, &self.sirk.flow_id)
    }
}
