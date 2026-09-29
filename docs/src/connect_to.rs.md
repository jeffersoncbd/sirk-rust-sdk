## Summary
`connect_to` validates an HTTP endpoint, checks its health, creates a flow for the directory, and returns a connected `Sirk`.

## Behavior
It trims trailing slashes and returns a protocol error if the endpoint is empty. It then checks transport health and creates a flow for the owned directory; either operation’s error is propagated. On success, it returns a `Sirk` containing the directory, flow ID, and transport.

## Imports
- `crate::{Error, Sirk}`: Return error type and constructed client
- `crate::http::HttpTransport`: HTTP connection and flow operations
- `crate::transport::Transport`: Provides transport operation methods
- `std::path::Path`: Directory argument type
