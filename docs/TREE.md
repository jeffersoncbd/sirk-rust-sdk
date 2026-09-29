Cargo.toml - Defines the `sirk-sdk` Rust package and its dependencies for serialization, errors, and HTTP requests.
src/agent.rs - Runs the named agent with the supplied input and returns the transport’s result unchanged.
src/bin/documentation.rs - Connects to Sirk and runs the documentation flow, propagating connection errors and returning its result.
src/connect.rs - Connects to the local service using the current working directory and wraps directory lookup failures in `Error::CurrentDirectory`.
src/connect_to.rs - connect_to validates an HTTP endpoint, checks its health, creates a directory flow, and returns a connected Sirk.
src/error.rs - Error provides typed context for filesystem, JSON, protocol, tool, and remote HTTP failures.
src/http.rs - Defines `HttpTransport` to store a configured HTTP agent and endpoint for transport operations.
src/http/health.rs - Checks `/health` and succeeds only when the response reports status `"ok"`.
src/http/new.rs - Creates an HTTP transport for an endpoint using a `ureq` agent with five-second DNS and connection timeouts.
src/http/read.rs - Reads an HTTP response body as text and returns it with its status code, or reports a body-reading error.
src/http/run_agent.rs - Implements HTTP transport operations for flow creation, agent execution, and directory and Git access.
src/lib.rs - Declares crate modules and re-exports `Error`, `Sirk`, and `Tools` at the crate root.
src/protocol.rs - Defines serializable request and response structures for protocol messages, carrying inputs, results, errors, and status data.
src/sirk.rs - Sirk groups a directory path, flow ID, and transport.
src/tests.rs - The file maps two test modules to test files for agent round trips and unavailable-service reporting.
src/tests/completes_agent_round_trip.rs - Verifies that a Sirk client completes an agent run and related tool requests through a mock HTTP server.
src/tests/reports_unavailable_service.rs - Checks that connecting to an unavailable local service fails with guidance to run `sirk http`.
src/tools.rs - Returns a `Tools` handle that borrows the `Sirk` instance without validation or fallible work.
src/tools/await_confirm.rs - Prompts for terminal input before continuing, returning an error if terminal I/O fails or the user enters `/cancel`.
src/tools/git.rs - Returns a `Git` handle that borrows the `Sirk` instance from `Tools`.
src/tools/git/add.rs - Adds the current flow’s changes to Git in the configured directory and returns any error.
src/tools/git/status.rs - Returns Git status entries for the configured directory and flow, propagating any transport error.
src/tools/tree.rs - Returns the directory tree for the current directory and flow.
src/transport.rs - Defines a thread-safe transport interface for creating flows, running agents, and managing working-directory state.