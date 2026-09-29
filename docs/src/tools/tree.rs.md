## Summary
Returns the directory tree for the current directory and flow.

## Behavior
Delegates to the transport layer with the configured directory and flow ID, returning its `Vec<String>` or propagating its `Error`.

## Imports
- `super::Tools`: Provides the `Tools` type.
- `crate::Error`: Error type returned by the function.
