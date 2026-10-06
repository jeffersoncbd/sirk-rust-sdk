use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize)]
pub(super) struct AgentRequest<'a> {
    pub directory: &'a Path,
    pub agent: &'a str,
    #[serde(rename = "conversationId", skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<&'a str>,
    pub input: &'a str,
}

#[derive(Serialize)]
pub(super) struct DirectoryRequest<'a> {
    pub directory: &'a Path,
}

#[derive(Deserialize)]
pub(super) struct AgentResponse {
    pub ask: Option<String>,
    #[serde(rename = "conversationId")]
    pub conversation_id: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::AgentRequest;
    use std::path::Path;

    #[test]
    fn serializes_an_optional_conversation_id() {
        let without_conversation = AgentRequest {
            directory: Path::new("/workspace/project"),
            agent: "profile-interviewer",
            conversation_id: None,
            input: "Start",
        };
        let with_conversation = AgentRequest {
            directory: Path::new("/workspace/project"),
            agent: "profile-interviewer",
            conversation_id: Some("conversation-18f-1234-0"),
            input: "Ada",
        };

        let without_conversation = serde_json::to_value(without_conversation).unwrap();
        let with_conversation = serde_json::to_value(with_conversation).unwrap();

        assert!(without_conversation.get("conversationId").is_none());
        assert_eq!(
            with_conversation["conversationId"],
            "conversation-18f-1234-0"
        );
    }
}
