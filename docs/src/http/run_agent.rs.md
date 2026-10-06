## Summary
Implements HTTP transport operations for creating flows, running agents, and accessing directory and Git data.

## Behavior
Serializes requests as JSON and sends them to the relevant endpoint. Returns parsed results, reporting network and non-success HTTP responses as errors. Validates required response fields, requires agent responses to contain exactly one of a question or result, and requires Git add to return an `"ok"` status.

## Imports
- `HttpTransport`: Provides the HTTP client and endpoint.
- `Error`: Represents transport, remote, and protocol failures.
- `protocol`: Defines request and response types.
- `Transport`: Defines the operations implemented here.
- `Path`: Represents the working directory.
- `serde_json`: Serializes requests and parses responses.
