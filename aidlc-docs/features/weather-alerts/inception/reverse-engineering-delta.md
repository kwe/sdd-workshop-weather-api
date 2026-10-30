# Reverse Engineering Delta: weather-alerts

Code read on 2026-10-30; no contradiction with shared docs found that affects this feature. Facts relevant to the design:

- `src/lib.rs`: `app(pool)` builds the router; OpenAPI via `utoipa` `ApiDoc` (paths + schemas lists); routes are flat `.route(...)` calls, `frontend::router()` merged near the end.
- Layering: `handlers.rs` -> `service.rs` -> `repository.rs`; `models.rs` holds structs; `error.rs` has `ApiError`.
- `weather_records` has multiple rows per city possible (POST creates more); "latest" = greatest `recorded_at` (tie: greatest `id`). Columns nullable: temperature, humidity, wind.
- `forecasts`: 5 rows per city (`day_offset` 0-4, `high/low_temperature_celsius`, `conditions`, `precipitation_chance`), keyed by lowercase `city_key`. Forecast has NO wind or humidity, so forecast alerts can only be heat/cold.
- Migrations: `migrations/0001_*.sql`, `0002_*.sql` (sequential). New one must use UTC-timestamp name.
- Frontend: ES modules in `static/js/` (api, dom, gazetteer, globe-scene, main, state, sun, ui, weather); `tests/frontend.rs` and `tests/api.rs` are the test suites.
