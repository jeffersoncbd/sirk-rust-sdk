## Summary
Tests that a Sirk client runs an agent and completes follow-up tree and Git requests through a mock HTTP server.

## Behavior
The mock server validates request paths, project directory, flow ID, and agent input, then returns JSON responses. The test checks the agent result, tree and Git status, sends a Git add request, and joins the server thread; failures panic through `unwrap` or assertions.

## Imports
- `crate::Sirk`: Connects to the server and makes client requests.
- `std::io`: Reads HTTP requests and writes mock responses.
- `std::net::TcpListener`: Accepts local HTTP connections.
- `std::thread`: Runs the mock server concurrently.
