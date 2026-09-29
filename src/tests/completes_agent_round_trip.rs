use crate::Sirk;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread,
};

#[test]
fn completes_agent_round_trip() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        for (expected_path, response_body) in [
            ("/health", r#"{"status":"ok"}"#),
            ("/v1/flows", r#"{"flowId":"flow-18f-1234-0"}"#),
            ("/v1/agent/run", r#"{"result":"documented"}"#),
            ("/v1/tree", r#"{"paths":["README.md","src/lib.rs"]}"#),
            ("/v1/git/status", r#"{"paths":["src/lib.rs"]}"#),
            ("/v1/git/add", r#"{"status":"ok"}"#),
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut first_line = String::new();
            reader.read_line(&mut first_line).unwrap();
            assert!(first_line.contains(expected_path));
            let mut content_length = 0;
            let mut flow_id = None;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                    content_length = value.trim().parse().unwrap();
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("x-sirk-flow-id:") {
                    flow_id = Some(value.trim().to_owned());
                }
            }
            let mut request_body = vec![0; content_length];
            reader.read_exact(&mut request_body).unwrap();
            if expected_path != "/health" {
                let request: serde_json::Value = serde_json::from_slice(&request_body).unwrap();
                assert_eq!(request["directory"], "/workspace/project");
            }
            if expected_path != "/health" && expected_path != "/v1/flows" {
                assert_eq!(flow_id.as_deref(), Some("flow-18f-1234-0"));
            }
            if expected_path == "/v1/agent/run" {
                let request: serde_json::Value = serde_json::from_slice(&request_body).unwrap();
                assert_eq!(request["directory"], "/workspace/project");
                assert_eq!(request["agent"], "code-explainer");
                assert_eq!(request["input"], "Explain this file");
            }
            let status = if expected_path == "/v1/flows" {
                "201 Created"
            } else {
                "200 OK"
            };
            write!(
                stream,
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            )
            .unwrap();
        }
    });
    let sirk = Sirk::connect_to(endpoint, "/workspace/project").unwrap();
    assert_eq!(
        sirk.agent("code-explainer", "Explain this file").unwrap(),
        "documented"
    );
    assert_eq!(sirk.tools().tree().unwrap(), ["README.md", "src/lib.rs"]);
    assert_eq!(sirk.tools().git().status().unwrap(), ["src/lib.rs"]);
    sirk.tools().git().add().unwrap();
    server.join().unwrap();
}
