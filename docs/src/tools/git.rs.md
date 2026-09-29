## Summary
Creates a `Git` handle that borrows the `Sirk` instance from `Tools`.

## Behavior
`Tools::git` returns a `Git` containing a reference to `self.sirk`; it performs no validation or other side effects.

## Imports
- `super::Tools`: Defines the `Tools` implementation.
- `crate::Sirk`: Type referenced by `Git`’s borrowed field.
