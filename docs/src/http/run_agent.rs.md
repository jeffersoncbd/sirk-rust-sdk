## Summary
Implements HTTP transport operations for creating flows, running agents, and accessing directory and Git data.

## Behavior
Serializes request data as JSON, sends it to the corresponding endpoint, and returns parsed response values. Network failures, non-success HTTP statuses, malformed responses, and missing required fields produce errors; Git add also requires an `"ok"` status.

## Imports
- `HttpTransport`: Provides the HTTP client and endpoint.
- `Error`: Represents transport, remote, and protocol failures.
- `protocol`: Defines JSON request and response types.
- `Transport`: Defines the operations implemented here.
- `Path`: Represents the working directory.
- `serde_json`: Serializes requests and parses responses.
