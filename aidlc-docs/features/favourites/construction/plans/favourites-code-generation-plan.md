# Code Generation Plan: favourites (single unit)

Status legend: [ ] pending, [x] done. Port 8081; DB under /tmp/favourites/ for manual runs.

## Coordination (shared files also claimed by `weather-alerts`)
Edits must be minimal and additive; no reformatting/reordering.
- `src/lib.rs`: add one `pub mod favourites;` line; append `favourites::*` paths, `Favourite`/`CreateFavouriteRequest` schemas and a `favourites` tag to `ApiDoc` as new lines; add `.merge(favourites::router())` after the last existing `.merge(...)`/`.route(...)` before `.with_state`.
- `src/frontend.rs`: append one `asset!("js/favourites.js", JS),` line at the end of `ASSETS`.
- `static/index.html`: add the Favourites `<section>` before `#city-list` and the star button inside `.detail-head`; no other changes.
- `static/css/app.css`: append a clearly marked `/* favourites */` block at the end of the file.
- `README.md`: add a new `### Favourites` subsection and append endpoint rows to the endpoints table only.
- Non-shared edits to JS modules (`api.js`, `state.js`, `main.js`, `ui.js`, `globe-scene.js`) are small hooks.

## Steps
- [ ] 1. Migration `migrations/20261030130305_create_favourites.sql`: `favourites(id INTEGER PRIMARY KEY AUTOINCREMENT, city TEXT NOT NULL, city_key TEXT NOT NULL UNIQUE, created_at TEXT NOT NULL)`.
- [ ] 2. `src/favourites.rs`: `Favourite {city, favouritedAt, weather: Option<WeatherRecord>}` (Serialize, ToSchema, camelCase); tolerant request body (`city` parsed as `serde_json::Value` or optional string) so a non-string `city` gives 400, not 422; validation (trim, non-empty, <=100 chars via `chars().count()`); repository fns (list, insert-or-get by `city_key`, delete by key, latest record per city via `repository::list_weather_records`); service returning `ApiError`; handlers (GET/POST/DELETE) with `#[utoipa::path]`; `pub fn router() -> Router<SqlitePool>`. POST: 404 if no records (nothing stored), 201 new, 200 existing (`favouritedAt` unchanged). DELETE 204/404/400. Ordering `created_at DESC, id DESC`; `created_at` from `Utc::now().naive_utc()` formatted like `recordedAt`.
- [ ] 3. `src/lib.rs` hooks per Coordination.
- [ ] 4. `tests/favourites.rs` per TR-1 (in-memory SQLite, style of tests/api.rs).
- [ ] 5. Run `cargo test --test favourites` and `cargo clippy` for the backend slice.
- [ ] 6. `static/js/api.js`: `getFavourites`, `addFavourite`, `removeFavourite` (JSON body, `encodeURIComponent` on city, errors mapped to ApiError/ApiNotFound, DELETE 204 has no body).
- [ ] 7. `static/js/favourites.js`: DOM-free helpers (`favouriteKeyOf(name)`, `isFavourite(favourites, city)`, `favouriteItems(favourites, cities)`).
- [ ] 8. `static/js/state.js`: add `favourites: []`.
- [ ] 9. `static/js/main.js`: load favourites at boot and with each refresh (failure tolerated); `onToggleFavourite(key)` calls API then updates state and announces result/failure; favourite item click reuses `select(key)`.
- [ ] 10. `static/index.html` + `static/js/ui.js`: star button (`aria-pressed`, label via `setAttribute`/`textContent`) and Favourites section render (text nodes via `el()`); list signature includes favourites; section hidden when empty.
- [ ] 11. `static/js/globe-scene.js`: `favourite` class, star badge (`el`/`svgEl`), aria-label includes "favourite"; extra `favouriteKeys` hook in `setCities`.
- [ ] 12. `static/css/app.css` appended block: gold ring pin, star badge, star button states, Favourites section; reduced-motion respected; focus-visible styles.
- [ ] 13. `src/frontend.rs`: register `js/favourites.js`.
- [ ] 14. `tests/frontend.rs` additions per TR-2.
- [ ] 15. README `### Favourites` subsection and endpoint rows.
- [ ] 16. Write `construction/favourites/code/code-summary.md`; tick boxes here; update state/audit.

## Verification (Build and Test gate, parallel-mode §9)
Commit, `git rebase main`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, full `cargo test`, run on `PORT=8081` with curl evidence for all endpoints and status codes, recorded in `construction/build-and-test/build-and-test-summary.md`.

## Traceability
FR-1 -> 1; FR-2..FR-9 -> 2,4; FR-10,11 -> 2,3,4; FR-12..16 -> 6-12; NFR-1 -> 10,11,13,14; NFR-2 -> Coordination; TR-1 -> 4; TR-2 -> 14.
