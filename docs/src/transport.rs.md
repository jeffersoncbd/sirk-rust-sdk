## Summary
Defines the interface for transport operations on a directory and flow.

## Behavior
Implementations create flows, run agents, retrieve directory trees and Git status, and stage changes; operations return results or `Error`.

## Imports
- `crate::Error`: Error type returned by transport operations.
- `std::path::Path`: Represents the working directory.
