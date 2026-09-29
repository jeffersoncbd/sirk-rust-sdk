## Summary
`Sirk` stores a directory, flow ID, and transport.

## Behavior
The struct groups these values; it defines no function or execution flow.

## Imports
- `crate::transport::Transport`: Transport trait used by the transport field.
- `std::path::PathBuf`: Path type used by the directory field.
