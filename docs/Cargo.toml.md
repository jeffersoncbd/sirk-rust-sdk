## Summary
`Cargo.toml` defines the `sirk-sdk` package and its dependencies.

## Behavior
It sets package metadata, including version `0.3.1` and Rust edition 2024, and declares dependencies for serialization, JSON handling, error types, and HTTP requests.

## Imports
- `serde`: Provides serialization and deserialization derives.
- `serde_json`: Supports JSON handling.
- `thiserror`: Provides error type helpers.
- `ureq`: Provides HTTP requests with default features disabled.
