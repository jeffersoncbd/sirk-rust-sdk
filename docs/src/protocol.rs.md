## Summary
The file defines protocol request and response types for JSON messages.

## Behavior
Requests serialize directory, agent, and input data, omitting `conversationId` when absent. Responses deserialize optional agent, flow, path, and status fields; health responses require a status.

## Imports
- `serde`: Provides serialization and deserialization derives and attributes.
- `std::path::Path`: Represents directory paths in requests.
