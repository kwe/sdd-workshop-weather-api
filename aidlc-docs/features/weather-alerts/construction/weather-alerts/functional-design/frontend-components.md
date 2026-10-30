# Frontend Components: weather-alerts

## Modules
| Module | Status | Responsibility |
|---|---|---|
| `static/js/alerts.js` | new | Pure helpers (`cityKeyOf`, `groupByCity`, `countBySeverity`, label/colour tables) and DOM builders `renderBanner(el, state, onSelect)` and `renderCityAlerts(el, alerts)` using `createElement` + `textContent` only. |
| `static/js/api.js` | edit (append) | `getAlerts(signal)` - same `getJson`, validates array. |
| `static/js/state.js` | edit (add two fields) | `alerts`, `alertsStatus`. |
| `static/js/main.js` | edit (small hook) | `refreshAlerts(ui)` with in-flight guard, called next to every `refreshWeather(ui)` call, including inside the existing `setInterval` and `visibilitychange` handler. No new timer. |
| `static/js/ui.js` | edit (small hook) | Mount banner and per-city alert section on render; announce count changes via existing announcer. |
| `static/js/globe-scene.js` | edit (small hook) | `setAlertRings(groups, reducedMotion)`; merges with the selection ring. |
| `static/index.html` | edit | Banner container (`#alerts-banner`) and detail-panel container (`#city-alerts`); no inline script/style. |
| `static/css/app.css` | edit (append) | Banner, severity chips, `@media (prefers-reduced-motion: reduce)` overrides. |
| `src/frontend.rs` | edit (one ASSETS line) | Serve `js/alerts.js`. |

## Banner (`#alerts-banner`)
- A `<button aria-expanded>` pill: "N alerts" plus severity chips (e.g. "2 severe 1 warning"); zero -> "No active alerts"; error -> "Alerts unavailable"; loading -> hidden.
- Expanding shows a `<ul>` of items "City - Type (Severity)"; each item is a `<button>`; click calls `onSelect(cityKey)` then collapses on small screens. Escape collapses. Count change is announced politely.

## Detail panel section (`#city-alerts`)
- Shown only for the selected city when it has alerts. Heading "Active alerts"; list rows: "Heat - Severe: 41.0 C (threshold 30 C), forecast, 2026-10-31". Values are formatted with `Number(...).toFixed(1)` and units from a fixed table; city/type/severity strings go through textContent.
- Unit toggle: temperature alert values honour the existing C/F toggle using the existing conversion helper in `weather.js` (threshold shown the same way).

## Ring colours / motion
- Colours per R-12. Ring datum: `{lat, lng, color, kind: 'alert'}`; selection ring keeps its blue.
- Reduced motion: static ring (no propagation). Tested via the presence of the `prefers-reduced-motion` handling in `globe-scene.js`/CSS.

## API integration
- Only `GET /api/alerts`. No writes.
