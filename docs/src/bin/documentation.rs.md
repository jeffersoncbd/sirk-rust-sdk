## Summary
Connects to Sirk and runs the documentation flow.

## Behavior
`main` connects to Sirk, propagating connection errors, then passes the connection to `documentation::run` and returns its result.

## Imports
- `sirk_sdk::Sirk`: Provides the Sirk connection.
- `documentation`: Provides the documentation flow's `run` function.
