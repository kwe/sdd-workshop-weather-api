# Code Summary - weather-alerts

## Backend
- `migrations/20261030130300_create_alert_thresholds.sql` (new): `alert_thresholds` table with tier CHECKs and 4 seeds.
- `src/alerts.rs` (new): types, pure `classify`/`evaluate`, DB reads, `get_alerts`, handler with utoipa docs, 11 unit tests.
- `src/lib.rs` (additive): module, path, schemas, `alerts` tag, `/api/alerts` route.
- `src/frontend.rs` (additive): `js/alerts.js` asset.

## Frontend (small additive hooks)
- `static/js/alerts.js` (new): grouping, severity tables, banner and per-city list (DOM API only).
- `api.js`: `getAlerts`. `state.js`: `alerts`, `alertsStatus`.
- `main.js`: `refreshAlerts` beside every `refreshWeather`; reuses the single 60 s timer; pushes rings to the scene.
- `ui.js`: banner, city alert list, alerts in detail signature, Escape collapses banner.
- `globe-scene.js`: `setAlertRings`, datum-aware ring colour, rings concatenated with the selection ring, suppressed under reduced motion; static `.has-alert` pin ring.
- `index.html`: `#alerts-banner`, `#city-alerts`. `app.css`: appended alert block and reduced-motion override.

## Tests
- `tests/alerts.rs` (13): seeds, tiers, boundaries, latest record, disabled thresholds, 500 path, OpenAPI.
- `tests/alerts_frontend.rs` (5): static checks (served with CSP, containers, no HTML-string APIs, single timer, reduced motion).

## Docs
- `README.md`: endpoint row and `### Weather Alerts`.
