## Summary
Verifies that connecting to an unavailable service returns an error with startup guidance.

## Behavior
Attempts to connect to a local address where no service is expected; the test fails if the connection succeeds and otherwise checks that the error mentions `sirk http`.

## Imports
- `crate::Sirk`: Provides the connection method under test.
