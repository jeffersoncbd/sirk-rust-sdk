use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize)]
pub(super) struct AgentRequest<'a> {
    pub directory: &'a Path,
    pub agent: &'a str,
    pub input: &'a str,
}

#[derive(Serialize)]
pub(super) struct DirectoryRequest<'a> {
    pub directory: &'a Path,
}

#[derive(Deserialize)]
pub(super) struct AgentResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct FlowResponse {
    #[serde(rename = "flowId")]
    pub flow_id: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct PathsResponse {
    pub paths: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub(super) struct StatusResponse {
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct HealthResponse {
    pub status: String,
}
