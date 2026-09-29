## Summary
Creates an HTTP transport for the given endpoint.

## Behavior
Builds a `ureq` agent that treats HTTP status codes as responses and sets five-second DNS resolution and connection timeouts, then stores it with the endpoint.

## Imports
- `super::HttpTransport`: The transport type being constructed.
- `std::time::Duration`: Specifies the timeout durations.
