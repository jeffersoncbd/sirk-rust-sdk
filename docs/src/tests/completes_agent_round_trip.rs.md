## Summary
Verifies that a Sirk client completes an agent run and related tool requests through an HTTP server.

## Behavior
Starts a local mock server, checks the client’s request paths, payloads, and flow ID, and returns JSON responses. It then verifies the agent result and tree and Git status outputs, sends a Git add request, and joins the server thread; failures panic through `unwrap` or assertions.

## Imports
- `crate::Sirk`: Connects to the server and makes client requests.
- `std::io`: Reads HTTP requests and writes mock responses.
- `std::net::TcpListener`: Accepts local HTTP connections.
- `std::thread`: Runs the mock server concurrently.
