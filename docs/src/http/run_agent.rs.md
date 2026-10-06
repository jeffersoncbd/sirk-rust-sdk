## Summary
Implements HTTP transport operations for flows, agents, directory paths, and Git data.

## Behavior
Sends JSON requests to the corresponding endpoints and parses successful responses. Network and non-success HTTP responses return errors; required fields are validated, agent replies must contain exactly one of a question or result, and Git add requires an `"ok"` status.

## Imports
- `HttpTransport`: Provides the HTTP client and endpoint.
- `Error`: Represents transport, remote, and protocol failures.
- `protocol`: Defines request and response types.
- `Transport`: Defines the implemented transport operations.
- `Path`: Represents the working directory.
- `serde_json`: Serializes requests and parses responses.
