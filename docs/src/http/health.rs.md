## Summary
Checks that the HTTP endpoint is healthy and returns success only when its response confirms `"ok"`.

## Behavior
Sends a GET request to `/health`, maps request failures to `Error::Unavailable`, and reads the response. Non-200 statuses become `Error::Remote` with the response body; invalid JSON propagates a deserialization error, and any status other than `"ok"` becomes `Error::Protocol`.

## Imports
- `HttpTransport`: Provides the endpoint and HTTP agent.
- `Error`: Represents request, remote, and protocol failures.
- `HealthResponse`: Deserializes the health response JSON.
