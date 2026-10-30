# Services (Orchestration) — Globe Frontend (Cycle 2)

The frontend has no server-side service layer; the Rust API layers (`handlers/service/repository`) are unchanged. Orchestration lives in `main.js` using the store.

## S1 — Weather loading and refresh
- On start: `status = loading`; `api.getWeather()`; map records to `City[]` via `gazetteer.locate`; keep the most recent `recordedAt` per normalised city name; `status = ready`; `globe.setCities()`.
- Every 60 s (and when the tab becomes visible after being hidden): refetch silently; on success diff-update cities and keep camera and selection; on failure set `status.stale = true` and keep the previous data visible.
- Initial failure: `status = error` with retry action; the plain-HTML list fallback is shown if cities exist.
- Single in-flight request guard with `AbortController`; timer paused while `document.hidden`.

## S2 — Selection and fly-to
- Sources: pin click, list item, search result, `Esc`/close.
- `selectCity(key)`: set `selectedKey`; `globe.flyTo(city)` (jump instead of animate if `prefers-reduced-motion`); `ui` opens the panel and moves focus to it.
- `clearSelection()`: `globe.resetView()`; focus returns to the previously focused list item or pin.
- Cities with no coordinates can be selected from the list: the panel opens with data but the camera does not move.

## S3 — Forecast loading
- On selection, if `forecasts[key]` is missing or `error`: set `loading`, `api.getForecast(record.city)`; success to `ready`; `ApiNotFound` to `none`; other errors to `error` (panel retry). Cached per city for the session; a refresh does not clear it.
- Late responses for a no-longer-selected city still populate the cache but do not alter the panel.

## S4 — Environment and lifecycle
- Sun direction recomputed every 60 s and on resume; fed to the shader uniform.
- Auto-rotate pauses on pointer/keyboard interaction and when a city is selected; resumes after ~8 s idle with no selection (disabled under reduced motion).
- Render loop paused when the tab is hidden; resize observer updates renderer size; device pixel ratio capped at 2.
- Capability failure (no WebGL, or CDN script/texture failure): show an explanatory banner and the plain-HTML weather list; the rest of the app (list, units, forecasts) remains functional.
