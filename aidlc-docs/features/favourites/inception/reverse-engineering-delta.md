# Reverse Engineering Delta (favourites)

Full re-run skipped per parallel-features.md §2. Facts verified from code relevant to this feature:

- Backend: axum + sqlx SQLite. `src/lib.rs` builds the router (`.route(...)` chain, utoipa `ApiDoc` with explicit `paths(...)`/`schemas(...)`, `.with_state(pool)`).
  Layers: `handlers.rs` -> `service.rs` -> `repository.rs`; `error.rs` (`ApiError` bad_request/not_found/internal, `with_path`, `json_error`).
- Migrations: `0001_create_weather_records.sql`, `0002_create_forecasts.sql` run by `sqlx::migrate!` in `db::migrate`. New one must be timestamp-named.
- Cities are not a table: a "city" is the `city` text of `weather_records` rows; matching is case-insensitive in Rust (`to_lowercase`) because SQLite `lower()` is ASCII-only. Forecasts use a `city_key` column with the same rule.
- Frontend: `static/js/*` ES modules embedded via `include_str!` in `src/frontend.rs` `ASSETS` (a new JS file must be registered there). CSP forbids inline scripts/styles attr via JS; `dom.js` `el()` is text-only by design; `tests/frontend.rs` forbids `innerHTML` etc. and `fetch(` outside `api.js`.
- Frontend city identity key is `normalize(name)|normalizeCountry(country)`; pins are built in `globe-scene.js` (`buildPin`/`updatePin`), list in `ui.js` (`renderList`), detail panel in `renderDetail`, fed by `main.js` actions.
- No JS test runner exists in the repo; frontend tests are Rust integration tests over embedded assets (`tests/frontend.rs`). Node v24 is available locally.
- Shared docs under `aidlc-docs/inception|construction` are untouched (read-only).
