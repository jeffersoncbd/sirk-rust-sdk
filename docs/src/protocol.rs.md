## Summary
The file defines request and response data structures for serializing and deserializing protocol messages.

## Behavior
Request structs borrow directory, agent, and input data; response structs hold optional result, error, flow ID, paths, or status fields. The health response requires a status value.

## Imports
- `serde`: Provides serialization and deserialization derives and field renaming.
- `std::path::Path`: Represents directory paths in requests.
