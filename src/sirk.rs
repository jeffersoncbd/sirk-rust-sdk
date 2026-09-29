use crate::transport::Transport;
use std::path::PathBuf;

pub struct Sirk {
    pub(super) directory: PathBuf,
    pub(super) flow_id: String,
    pub(super) transport: Box<dyn Transport>,
}
