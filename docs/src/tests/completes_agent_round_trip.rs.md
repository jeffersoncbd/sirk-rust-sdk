## Summary
Tests that a Sirk client completes an agent run and subsequent tool requests against a mock HTTP server.

## Behavior
The mock server checks request paths, payloads, and flow IDs, then returns JSON responses. The test verifies the agent result, tree and Git status, sends a Git add request, and joins the server thread; failures panic through `unwrap` or assertions.

## Imports
- `crate::Sirk`: Connects to the server and makes client requests.
- `std::io`: Reads HTTP requests and writes mock responses.
- `std::net::TcpListener`: Accepts local HTTP connections.
- `std::thread`: Runs the mock server concurrently.
