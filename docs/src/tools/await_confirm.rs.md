## Summary
`await_confirm` prompts for interactive terminal input before allowing execution to continue.

## Behavior
It opens `/dev/tty`, displays and flushes a prompt, then reads one line. Terminal access, prompt I/O, and closed input return `Error::Tool`; entering `/cancel` also returns an error. Any other line returns `Ok(())`.

## Imports
- `super::Tools`: Provides the `Tools` type.
- `crate::Error`: Supplies the function’s error type.
- `std::fs::OpenOptions`: Opens the terminal for reading and writing.
- `std::io`: Reads input, clones the terminal, and writes the prompt.
