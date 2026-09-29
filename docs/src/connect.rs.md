## Summary
Connects to the local service at `http://127.0.0.1:8080` using the current directory.

## Behavior
Gets the current working directory and passes it with the local service URL to `connect_to`. If reading the directory fails, returns the error wrapped as `Error::CurrentDirectory`; otherwise returns the connection result.

## Imports
- `crate::{Error, Sirk}`: Connection type and error type.
