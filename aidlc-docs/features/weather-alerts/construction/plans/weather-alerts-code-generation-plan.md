# Code Generation Plan: weather-alerts (single source of truth)

Unit: `weather-alerts` (single unit). Stories: none (skipped); traceability is to `requirements.md` FR/NFR ids. Design: `construction/weather-alerts/functional-design/`.
Workspace root = worktree root. All code at repo root paths below, never in `aidlc-docs/`.

## Coordination
Claims check #2 (2026-10-30): `RESULT: shared files only` with feature `favourites` on `README.md`, `src/frontend.rs`, `src/lib.rs`, `static/css/app.css`, `static/index.html`. The orchestrator also notes `static/js/globe-scene.js` is touched by favourites (pin colour/size + star label; no rings). Rules for every shared file: additive only, no reformatting or reordering, smallest possible hook.

| Shared file | My change (additive) |
|---|---|
| `src/lib.rs` | `pub mod alerts;` appended after the existing mod lines; `alerts::get_alerts` and the alert schemas appended at the END of the existing `paths(...)`/`schemas(...)` lists (single `ApiDoc`); one `.route("/api/alerts", get(alerts::get_alerts))` added after the last existing `.route(...)`, before `.merge(SwaggerUi...)` is not reordered. |
| `src/frontend.rs` | one `asset!("js/alerts.js", JS),` line appended at the end of `ASSETS`. |
| `README.md` | new `### Weather Alerts` subsection appended; append one row to the endpoint table only if one exists. |
| `static/index.html` | two new empty containers (`#alerts-banner`, `#city-alerts`) at new lines; no existing lines changed. |
| `static/css/app.css` | new block appended at end of file (alert styles + `prefers-reduced-motion` overrides). |
| `static/js/globe-scene.js` | (a) new `alertGroups`/`alertRings` locals and a new returned method `setAlertRings(groups)`; (b) `syncRings()` concatenates alert ring data with the existing selection ring (one expression changed); (c) `ringColor` accessor made datum-aware (one line); (d) one call `applyAlertPins()` at end of `setCities`. No restructuring; `updatePin` untouched (favourites likely edits it). Alert pin marking uses a class + CSS variable set from my own function, so it does not conflict with pin colour/size styling. |
| `static/js/main.js`, `ui.js`, `state.js`, `api.js` | small hooks (see steps); `api.js`/`state.js` appended only. |

Migration: `migrations/20261030130300_create_alert_thresholds.sql` (timestamp name, never edits `0001`/`0002`).

## Steps

### A. Database
- [ ] 1. Create `migrations/20261030130300_create_alert_thresholds.sql`: table + CHECKs (R-11) + 4 seed rows (A1). (FR-1)

### B. Backend (new file `src/alerts.rs`)
- [ ] 2. Types: `AlertType`, `Severity` (Ord), `AlertSource`, `Threshold` (FromRow), `Alert` (Serialize + ToSchema, camelCase, `type` rename). (FR-3.1)
- [ ] 3. Pure `classify` and `evaluate` plus `sort_alerts` per R-1..R-9; `today` injected. (FR-2, FR-3.4)
- [ ] 4. Unit tests in-module: boundaries (at/below/above) for each type and both comparison directions, severity tiers, NULL metric, disabled threshold, forecast date mapping across month boundary, wind/humidity not evaluated for forecast, city without forecast, latest-record selection incl. tie on `recorded_at`, case-insensitive city matching with Unicode (`São Paulo`), ordering.
- [ ] 5. Repository fns in `alerts.rs`: `list_thresholds`, `list_all_forecasts` (reuses existing `list_weather_records`). Three queries total. (NFR-5)
- [ ] 6. `service`-style `get_alerts(pool, today)` and handler `get_alerts` with `#[utoipa::path(get, path = "/api/alerts", tag = "alerts", responses 200 [Alert], 500 ErrorResponse)]`; errors through `with_path`. (FR-3.5)
- [ ] 7. `src/lib.rs` hooks per Coordination table; add `Alert`, `AlertType`, `Severity`, `AlertSource` to OpenAPI schemas and `alerts` tag. (FR-3.6)

### C. API tests (`tests/alerts.rs`, new, follows `tests/api.rs` in-memory pattern)
- [ ] 8. Seeded thresholds exist (4 rows, expected values).
- [ ] 9. `GET /api/alerts` on seed data returns exactly the 12 expected `watch` alerts (A4) with correct fields, camelCase keys, ordering; forecast dates = today+offset.
- [ ] 10. Insert records via `POST /api/weather` to trigger cold/wind/humidity and severity tiers; boundary at/below; newer record supersedes older; city without forecast; empty DB -> `[]`.
- [ ] 11. Disabled threshold (direct SQL update) suppresses alerts; DB failure path returns 500 JSON (drop table).
- [ ] 12. `/v3/api-docs` documents `/api/alerts` and the `Alert` schema.

### D. Frontend
- [ ] 13. `static/js/alerts.js` (new): pure helpers + `renderBanner`, `renderCityAlerts` with `createElement`/`textContent` only; fixed label/colour/unit tables. (FR-4, NFR-2)
- [ ] 14. `api.js`: append `getAlerts`. `state.js`: add `alerts: []`, `alertsStatus: 'loading'`. (FR-4.1)
- [ ] 15. `main.js`: add `refreshAlerts(ui)` with in-flight guard and call it beside each existing `refreshWeather(ui)` (initial, existing `setInterval`, `visibilitychange`); NO new timer. Errors -> `alerts: []`, `alertsStatus: 'error'`. (FR-4.5/4.6)
- [ ] 16. `ui.js`: render banner + selected-city alert list from state; announce count changes; banner item click uses the existing select path. (FR-4.3/4.4, NFR-4)
- [ ] 17. `globe-scene.js`: hooks per Coordination table. Non-reduced: pulsing alert rings (colour per severity) concatenated with selection ring. Reduced: no animated rings; pins get a static CSS ring (`.has-alert` + `--alert-color`). (FR-4.2, NFR-1)
- [ ] 18. `index.html` containers; `app.css` appended block: banner, chips, `.has-alert` static ring, `@media (prefers-reduced-motion: reduce)` disabling transitions/animations. No inline style/script. (NFR-1/3)
- [ ] 19. `src/frontend.rs`: add `js/alerts.js` asset line.

### E. Frontend tests (`tests/alerts_frontend.rs`, new; same style as `tests/frontend.rs` plus pure-logic checks)
- [ ] 20. `/static/js/alerts.js` served as JS with CSP header unchanged (`frontend::CONTENT_SECURITY_POLICY`); index contains the two containers.
- [ ] 21. Static-source rules over `ASSETS` (incl. `alerts.js`): no `innerHTML`/`outerHTML`/`insertAdjacentHTML`/`document.write`/`eval`; `main.js` still has exactly one `setInterval` and it references `refreshAlerts`; `app.css` contains a `prefers-reduced-motion` block mentioning alert classes; `globe-scene.js` suppresses alert rings when `reduced`.
- [ ] 22. Pure-logic JS tests: if `node` is available, run `alerts.js` helper assertions (grouping, worst severity, counts, key matching) via `std::process::Command` and skip with a printed note if `node` is missing. (Decision: no new dev-dependencies.)

### F. Documentation
- [ ] 23. README `### Weather Alerts` (endpoint, response example, thresholds table, tuning via SQL, UI behaviour, reduced motion).
- [ ] 24. `aidlc-docs/features/weather-alerts/construction/weather-alerts/code/summary.md` (markdown summary of files created/changed).

### G. Verification (then Build and Test stage)
- [ ] 25. `cargo build`, `cargo test` (feature tests) during generation; full gate (fmt, clippy -D warnings, full test, rebase on `main`, curl on port 8082) in Build and Test.

## Totals
25 steps; 6 new files (`alerts.rs`, migration, `alerts.js`, two test files, summary) and ~11 additive edits to existing files.
