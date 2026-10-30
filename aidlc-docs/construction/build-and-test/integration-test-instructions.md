# Integration Test Instructions

## Purpose
Verify handler -> service -> repository -> SQLite (migrations 0001 + 0002) and the router/OpenAPI wiring.

## Scenarios (`tests/api.rs`, in-memory SQLite, `tower::ServiceExt::oneshot`)
1. Five-day forecast with rolling UTC dates and field shapes (S1).
2. Matching by case, whitespace, percent-encoding and Unicode (S2).
3. 400 blank city, 404 unknown city / `NYC` / known city without forecast, 405 non-GET, standard error body with `path` (S3).
4. OpenAPI contains the forecast path and `ForecastDay`; `/api/weather/{id}` still works (S4).
5. Seed covers 10 cities x 5 days, `city_key` equals Rust `to_lowercase()`, CHECK and UNIQUE constraints hold, `0001` data unchanged (S5).
6. All pre-existing tests (regression).

## Setup
None. Tests create an isolated in-memory database each; no services or env vars needed.

## Run
```bash
cargo test --test api
```
Expected: 16 passed, 0 failed.

## Optional live smoke test
```bash
DATABASE_URL="sqlite:///tmp/smoke.db?mode=rwc" ./target/release/sdd-workshop-weather-api &
curl -s localhost:8080/api/weather/forecast/london
curl -s -o /dev/null -w "%{http_code}\n" localhost:8080/api/weather/forecast/atlantis   # 404
kill <PID>; rm /tmp/smoke.db
```

## Cleanup
Nothing for automated tests; stop the server and delete the temp DB after a smoke test.
