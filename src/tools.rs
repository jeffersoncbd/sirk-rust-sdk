mod await_confirm;
mod git;
mod tree;

use crate::Sirk;

pub struct Tools<'client> {
    pub(super) sirk: &'client Sirk,
}

#[allow(unused_imports)]
pub use git::Git;

impl Sirk {
    pub fn tools(&self) -> Tools<'_> {
        Tools { sirk: self }
    }
}
