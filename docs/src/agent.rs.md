## Summary
Runs an agent, answering interactive questions until it returns a result.

## Behavior
Sends the directory, flow ID, agent name, and current input to the transport. For each question, prompts on the terminal and uses the answer for the next request; returns the final result or propagates an error.

## Imports
- `crate::{Error, Sirk, AgentRunResponse}`: Provides the method types and response variants.
- `std::fs::OpenOptions`: Opens the interactive terminal.
- `std::io::{BufRead, BufReader, Write}`: Reads answers and writes prompts.
