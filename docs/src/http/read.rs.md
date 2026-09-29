## Summary
Reads an HTTP response body into a string and returns it with the status code.

## Behavior
Gets the response status, then reads the body as text. If reading fails, returns `Error::Unavailable` with the endpoint and underlying cause; otherwise returns the status and body.

## Imports
- `crate::Error`: Represents body-reading failures.
- `ureq`: Provides the HTTP response and body reader.
