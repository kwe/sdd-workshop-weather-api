# Code Summary - forecast-api

## Created
- `migrations/0002_create_forecasts.sql` - `forecasts` table (CHECKs, UNIQUE city_key+day_offset, index), seed for 10 cities x 5 days.

## Modified
- `src/models.rs` - `ForecastRow` (internal), `ForecastDay` (camelCase, `ToSchema`).
- `src/repository.rs` - `list_forecast_rows(pool, city_key)`.
- `src/service.rs` - `get_forecast`, `normalize_city`, `to_forecast_day`, plus 4 unit tests.
- `src/handlers.rs` - `get_forecast` with OpenAPI annotation.
- `src/lib.rs` - route `GET /api/weather/forecast/{city}`; `ApiDoc` path and schema.
- `tests/api.rs` - 7 new tests (rolling dates, matching, 400/404/405, OpenAPI, seed, constraints, known city without forecast). Existing tests untouched.
- `README.md` - endpoint, example, seed note.

## Not applicable
- Step 13: no deployment artifacts in this repo.

## Verification
`cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings` clean; `cargo test`: 4 unit + 16 integration tests pass.

## Code Review Fixes (post-workflow, minimal depth)
- `src/service.rs`: `get_forecast` requires exactly 5 rows with `day_offset` 0..=4; otherwise logs and returns a generic 500 (`ApiError::internal()`, now `pub` in `src/error.rs`). Zero rows still 404.
- `src/handlers.rs`: OpenAPI documents the 500 response.
- `tests/api.rs`: new test deleting one seeded row asserts the 500 standard body; rolling-date test now captures UTC date before/after the request and checks consecutive offsets from that baseline.
- Result: fmt and clippy `-D warnings` clean; 4 unit + 17 integration tests pass.
