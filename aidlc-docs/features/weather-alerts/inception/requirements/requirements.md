# Weather Alerts - Requirements

## Intent Analysis
- **User request**: Highlight dangerous weather on the globe: threshold table in SQLite, `GET /api/alerts`, OpenAPI docs, globe rings / detail-panel list / alerts banner, reduced-motion support, existing XSS + CSP rules kept, API and frontend tests.
- **Request type**: New feature (backend + frontend)
- **Scope**: Multiple components (migration, repository, service, handler, OpenAPI, 3 frontend areas, tests)
- **Complexity**: Moderate
- **Depth**: Standard
- **Answers**: all defaults accepted except Q8 (user chose 60-second refresh reusing the existing timer). Source: `weather-alerts-requirement-questions.md`.

## Functional Requirements

### Data (FR-1)
- FR-1.1 A new migration `migrations/<UTC timestamp>_create_alert_thresholds.sql` creates `alert_thresholds`: `alert_type` (PK: `heat|cold|wind|humidity`), `comparison` (`gte|lte`), `threshold` REAL (the `watch` level), `warning_threshold` REAL, `severe_threshold` REAL, `unit` TEXT, `enabled` INTEGER (0/1). CHECKs enforce valid type/comparison and that warning/severe are not less severe than the watch level.
- FR-1.2 Seed: heat gte 30 (warning 35, severe 40) C; cold lte 0 (-5, -15) C; wind gte 40 (60, 90) km/h; humidity gte 85 (90, 95) %.
- FR-1.3 Existing migrations are not edited. Thresholds are read-only via the API (no PUT).

### Evaluation (FR-2)
- FR-2.1 For each city (distinct, case-insensitive trimmed), take the latest `weather_records` row (greatest `recorded_at`, tie greatest `id`).
- FR-2.2 Current checks: temperature -> heat/cold, `wind_speed_kmh` -> wind, `humidity` -> humidity. NULL values produce no alert.
- FR-2.3 Forecast checks (5 days): forecast `high` -> heat, forecast `low` -> cold. Wind/humidity are never evaluated for forecasts.
- FR-2.4 One alert per breaching forecast day. A city without forecast rows still gets current alerts and no error. Cities are matched to forecasts by lowercase key.
- FR-2.5 Severity: `severe` if value reaches the severe cut-off, else `warning` if it reaches the warning cut-off, else `watch` if it reaches the threshold. Comparison inclusive (>= / <=). Disabled thresholds are ignored.
- FR-2.6 Pure evaluation logic lives in a unit-testable function (no DB or clock inside); "today" for forecast dates is injected, as in `get_forecast`.

### API (FR-3)
- FR-3.1 `GET /api/alerts` returns 200 with a bare JSON array (possibly empty). Each item (camelCase): `city`, `country` (nullable), `type`, `severity`, `value`, `threshold` (see FR-3.3), `source` (`current|forecast`), `date`.
- FR-3.2 `date`: current = `YYYY-MM-DD` of the record's `recordedAt`; forecast = UTC calendar date of that forecast day.
- FR-3.3 `threshold` is the base (`watch`) threshold for the type, so the response matches the product owner's configured values.
- FR-3.4 Order: severity descending (severe, warning, watch), then city (case-insensitive), then date, then source (current first).
- FR-3.5 Errors follow the existing `ApiError` JSON format; internal DB errors return the standard 500 without leaking details.
- FR-3.6 Route is added as an appended `.route(...)` (parallel-mode rule); documented in the OpenAPI spec (`ApiDoc` path + `Alert` schema, tag `alerts`).

### Globe UI (FR-4)
- FR-4.1 `api.js` gains `getAlerts()`; alerts are stored in shared state (`alerts`, `alertsStatus`).
- FR-4.2 Cities with active alerts get a pulsing ring coloured by their highest severity: watch yellow, warning orange, severe red. Severity is also conveyed non-visually (text labels / `aria`), not by colour alone.
- FR-4.3 The detail panel for the selected city lists that city's alerts (type, severity, value vs threshold, source, date). Cities with none show nothing extra.
- FR-4.4 A compact banner shows total alert count and per-severity counts and expands to a list; clicking an item selects the city and flies the globe to it. Zero alerts: banner shows "No active alerts".
- FR-4.5 Alerts refresh on the existing 60-second cycle in `main.js` (reuse the same `setInterval` and the `visibilitychange` handler; no separate timer). A refresh is skipped while the previous one is in flight.
- FR-4.6 If `/api/alerts` fails the globe and weather features continue; the banner shows "Alerts unavailable" and rings are cleared; previously shown alerts are not kept stale after failure.
- FR-4.7 Alerts for cities not placed on the globe (no gazetteer coordinates) still appear in the banner and can be selected where the existing code supports it, but get no ring.

## Non-Functional Requirements
- NFR-1 **Reduced motion**: under `prefers-reduced-motion: reduce` rings do not pulse (static ring), and fly-to follows the existing reduced-motion behaviour; the same applies to any banner animation.
- NFR-2 **XSS**: all server-provided strings rendered with `textContent`/DOM APIs only; no `innerHTML`, `insertAdjacentHTML` or inline handlers.
- NFR-3 **CSP**: the Content-Security-Policy is unchanged; no inline scripts/styles added, no new origins.
- NFR-4 **Accessibility**: banner is a keyboard-operable button/list with an accessible name and live count announcement via the existing announcer.
- NFR-5 **Performance**: a constant number of queries (thresholds, latest records, forecasts), not per-city queries.
- NFR-6 **Compatibility**: existing endpoints and tests unchanged in behaviour.
- NFR-7 **Parallel-mode discipline**: additive edits to shared files (`src/lib.rs`, `static/js/main.js`, `ui.js`, `state.js`, `index.html`, `app.css`, README); new code in new files where practical (`src/alerts.rs`, `static/js/alerts.js`, `tests/alerts.rs`).

## Testing Requirements
- API tests (`tests/alerts.rs`): seeded thresholds present; empty-data case; each type at, below and above the boundary; severity tiers; forecast-sourced alerts and dates; latest-record selection; NULL fields; city with no forecast; ordering; OpenAPI contains `/api/alerts`.
- Unit tests for the pure evaluator.
- Frontend tests (extend `tests/frontend.rs` pattern): static assertions that new modules are served, no `innerHTML`/inline handlers/CSP change, `prefers-reduced-motion` rule exists for rings, alerts hooked into the shared 60s timer (no second `setInterval`).

## Extension Configuration
Security baseline: No; Resiliency baseline: No; Property-based testing: No (XSS/CSP kept as explicit requirements above).

## Out of Scope
Editing thresholds via API, alert notifications/push, historical alert storage, wind/humidity forecasts, per-user thresholds.
