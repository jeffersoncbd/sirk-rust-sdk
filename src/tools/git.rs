mod add;
mod status;

use super::Tools;
use crate::Sirk;

pub struct Git<'client> {
    pub(super) sirk: &'client Sirk,
}

impl<'client> Tools<'client> {
    pub fn git(&self) -> Git<'client> {
        Git { sirk: self.sirk }
    }
}
