# S.I.R.K. Rust SDK

This package is a standalone HTTP client for the S.I.R.K. service. Start the
service before connecting:

```bash
sirk http
```

```rust
use sirk_sdk::Sirk;

let sirk = Sirk::connect()?;
let response = sirk.agent("code-explainer", "Explain this module")?;

let changed = sirk.tools().git().status()?;
let files = sirk.tools().tree()?;
sirk.tools().git().add()?;

sirk.tools().await_confirm()?;
```

`Sirk::connect` uses `http://127.0.0.1:8080` and sends the calling process's
current directory. Use `Sirk::connect_to` when the service has a different
address or the project has a different server-visible path:

```rust
let sirk = Sirk::connect_to("http://sirk:8080", "/workspace/project")?;
```

The SDK checks `GET /health` and creates a flow transcript with `POST /v1/flows`
while connecting. It keeps the returned ID internally and sends it as the
required `X-Sirk-Flow-Id` header on later requests. If it cannot reach the
service, its error instructs the user to start it with `sirk http`. Filesystem,
user interaction, and workflow control remain the responsibility of the host
program. File-operation requests made by the agent remain internal to S.I.R.K.

The SDK provides remote project tools that use the configured S.I.R.K. server:
`tools().git().status()`, `tools().git().add()`, and `tools().tree()`. Each uses
the flow created automatically for the same directory.
`tools().await_confirm()` remains a host-side interaction tool and waits for
Enter (or `/cancel`) on the terminal. Workflow control remains the
responsibility of the host program.
