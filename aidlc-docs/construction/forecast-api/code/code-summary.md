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
