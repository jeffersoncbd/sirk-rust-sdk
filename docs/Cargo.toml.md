## Summary
`Cargo.toml` defines the `sirk-sdk` Rust package and its dependencies.

## Behavior
It sets package metadata for version `0.2.0` using Rust edition 2024, then declares dependencies for serialization, JSON handling, error types, and HTTP requests.

## Imports
- `serde`: Provides serialization and deserialization derives.
- `serde_json`: Supports JSON handling.
- `thiserror`: Provides error type helpers.
- `ureq`: Provides HTTP requests with default features disabled.
