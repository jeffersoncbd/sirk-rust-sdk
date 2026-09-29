## Summary
`Error` represents failures from filesystem access, JSON handling, S.I.R.K. protocol and tools, and remote HTTP requests.

## Behavior
The enum carries context for each error, including source errors for current-directory lookup, JSON conversion, and unavailable endpoints; remote errors include an HTTP status and message.

## Imports
- `thiserror`: Provides error formatting and conversion derives.
- `std::io`: Supplies the current-directory error type.
- `serde_json`: Supplies the JSON error type.
- `ureq`: Supplies the connection error type.
