## Summary
Defines the interface for transport operations on a working directory and flow.

## Behavior
The `Transport` trait requires implementations to create flows, run agents, retrieve a directory tree and Git status, and stage changes. Each method returns its result or an `Error`; the trait is `Send + Sync`.

## Imports
- `crate::Error`: Error type returned by transport operations.
- `std::path::Path`: Represents the working directory.
