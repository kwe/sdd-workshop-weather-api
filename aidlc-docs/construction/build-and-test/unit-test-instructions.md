# Unit Test Execution

## Run
```bash
cargo test --lib
```

## Expected
- 4 tests pass, 0 failures (`src/service.rs`, module `tests`):
  - day-offset to date mapping across a month boundary
  - forecast value copying
  - city normalization (trim, Unicode lowercase `São Paulo` -> `são paulo`)
  - blank city rejected
- Coverage tooling is not configured; the new service helpers are fully covered by these tests.
- No report files; results print to the terminal.

## If tests fail
1. Read the failing assertion in the output.
2. Fix the code (or the test if the expectation is wrong).
3. Rerun `cargo test --lib`.
