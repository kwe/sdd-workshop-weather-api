# Build and Test Summary - weather-alerts

Branch `feature/weather-alerts`, rebased onto `main` (188dbaf). The rebase was clean (no conflicted files); the cherry-picked claims-script fix was skipped as already upstream.

## Results
| Command | Result |
| --- | --- |
| `cargo fmt --check` | OK |
| `cargo clippy --all-targets --all-features -- -D warnings` | Finished, no warnings |
| `cargo test` (full) | all suites pass: 15 lib (unit), 13 `alerts`, 5 `alerts_frontend`, 17 + 6 existing suites, 0 failures |

## Live check (PORT=8082, fresh temporary DB, server stopped afterwards)
- `GET /api/alerts` -> 200, 10 alerts. First entry:
  `{"city":"Mumbai","country":"India","type":"heat","severity":"watch","value":31.7,"threshold":30.0,"source":"current","date":"2026-07-17"}`
- `GET /` -> 200. `GET /v3/api-docs` contains `/api/alerts`.
- Not verified: the globe UI in a real browser (rings, banner, reduced motion); only static checks cover it.

## Seed alert count: design said 12, code yields 10
The code is correct; the design assumption was wrong. The first draft assumed Tokyo's forecast contributes all 5 days. The seeded Tokyo highs are 30.5, 31.0, 29.0, 28.5 and 30.0, so only days 0, 1 and 4 reach the 30 C threshold. Tokyo gives 1 current + 3 forecast = 4. Mumbai's highs (32.0, 31.5, 31.0, 32.5, 32.0) all qualify: 1 + 5 = 6. Total 10. The design (business-rules A4), plan step 9 and `tests/alerts.rs` were corrected earlier.
