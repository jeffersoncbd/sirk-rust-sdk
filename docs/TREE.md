Cargo.toml - Cargo.toml defines the sirk-sdk package metadata and dependencies for serialization, errors, and HTTP requests.
src/agent.rs - Runs an agent interactively, prompting for answers until it returns a result or an error.
src/bin/documentation.rs - Connects to Sirk and runs the documentation flow, propagating connection errors and returning its result.
src/connect.rs - Connects to the local service using the current working directory and wraps directory lookup failures in `Error::CurrentDirectory`.
src/connect_to.rs - connect_to validates an HTTP endpoint, checks its health, creates a directory flow, and returns a connected Sirk.
src/error.rs - Error provides typed context for filesystem, JSON, protocol, tool, and remote HTTP failures.
src/http.rs - Defines `HttpTransport` to store a configured HTTP agent and endpoint for transport operations.
src/http/health.rs - Checks `/health` and succeeds only when the response reports status `"ok"`.
src/http/new.rs - Creates an HTTP transport for an endpoint using a `ureq` agent with five-second DNS and connection timeouts.
src/http/read.rs - Reads an HTTP response body as text and returns it with its status code, or reports a body-reading error.
src/http/run_agent.rs - Implements HTTP transport for creating flows, running agents, and accessing directory and Git operations.
src/lib.rs - Declares crate modules and re-exports `Error`, `Sirk`, and `Tools` at the crate root.
src/protocol.rs - Defines JSON protocol types for requests and responses, including directory, agent, input, flow, path, and status data.
src/sirk.rs - Sirk groups a directory path, flow ID, and transport.
src/tests.rs - The file maps two test modules to test files for agent round trips and unavailable-service reporting.
src/tests/completes_agent_round_trip.rs - Tests that a Sirk client completes an agent run and Git add request against a mock HTTP server.
src/tests/reports_unavailable_service.rs - Checks that connecting to an unavailable local service fails with guidance to run `sirk http`.
src/tools.rs - Returns a `Tools` handle that borrows the `Sirk` instance without validation or fallible work.
src/tools/await_confirm.rs - Prompts for terminal input before continuing, returning an error if terminal I/O fails or the user enters `/cancel`.
src/tools/git.rs - Returns a `Git` handle that borrows the `Sirk` instance from `Tools`.
src/tools/git/add.rs - Adds the current flow’s changes to Git in the configured directory and returns any error.
src/tools/git/status.rs - Returns Git status entries for the configured directory and flow, propagating any transport error.
src/tools/tree.rs - Returns the directory tree for the current directory and flow.
src/transport.rs - Defines transport operations for creating flows, running agents, inspecting directories and Git status, and staging changes.