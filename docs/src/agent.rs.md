## Summary
Runs the named agent with the supplied input and returns its output.

## Behavior
Passes the directory, flow ID, agent name, and input to the transport; returns its `Result<String, Error>` unchanged.

## Imports
- `crate::{Error, Sirk}`: Defines the error type and `Sirk` implementation target.
