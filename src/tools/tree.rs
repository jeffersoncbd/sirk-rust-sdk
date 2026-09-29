use super::Tools;
use crate::Error;

impl Tools<'_> {
    pub fn tree(&self) -> Result<Vec<String>, Error> {
        self.sirk
            .transport
            .tree(&self.sirk.directory, &self.sirk.flow_id)
    }
}
