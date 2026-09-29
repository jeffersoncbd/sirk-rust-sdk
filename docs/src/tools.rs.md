## Summary
`Sirk::tools` returns a `Tools` handle that borrows the `Sirk` instance.

## Behavior
Calling the function creates a `Tools` value containing a reference to `self`. It performs no validation or fallible work.

## Imports
- `crate::Sirk`: Type of the instance borrowed by `Tools`.
