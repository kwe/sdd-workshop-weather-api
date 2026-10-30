# Components — Globe Frontend (Cycle 2)

Cycle 1 design docs (`components.md`, etc.) are unchanged. Inputs: `globe-frontend-requirements.md`, answers in `plans/globe-frontend-application-design-plan.md` (all A; Q5 extended with specular, twilight band and Fresnel glow).

## Design findings from verification (2026-10-30)
- globe.gl `2.46.2` UMD bundle on jsDelivr (1.88 MB, CORS `*`, SRI sha384 computable) bundles three.js; depends on `three >=0.179`.
- `three-globe@2.45.6/example/img/` provides: `earth-day.jpg`, `earth-blue-marble.jpg`, `earth-night.jpg`, `earth-topology.png` (bump), `earth-water.png` (ocean mask used for specular), `night-sky.png`; all return 200 with CORS `*`. There is **no cloud texture** in that package.
- Consequences: (1) clouds are **procedural** (fractal noise in the globe shader, no extra texture or origin); (2) the UMD bundle exposes no `THREE` global, so the scene effects are implemented by patching the globe's existing `MeshPhongMaterial` with `onBeforeCompile` (uniforms/GLSL injected), and any needed three classes are derived from live instances (e.g. `material.map.constructor`). Fallback if this proves unworkable in Code Generation: import map with a CSP script hash (will be raised with the user before changing the design).

## Rust components

### R1 — `src/frontend.rs` (new)
- **Purpose**: serve the embedded SPA and attach frontend-only security headers.
- **Responsibilities**: static asset table (path, content type, bytes) built with `include_str!`/`include_bytes!` from `static/`; `GET /`; `GET /static/{*path}`; 404 for unknown paths; response headers `Content-Security-Policy`, `Cache-Control: no-cache`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer` applied via a layer on this router only.
- **Interface**: `pub fn router() -> Router<SqlitePool>`.

### R2 — `src/lib.rs` (modified)
- Merge `frontend::router()` into `app()`; declare `pub mod frontend;`. No existing route changes; not added to OpenAPI.

### R3 — `static/` tree (new, embedded)
`static/index.html`, `static/css/app.css`, `static/js/*.js` (see F1-F9), optional `static/favicon.svg`.

## Frontend components (browser, ES modules)

| ID | File | Purpose |
|---|---|---|
| F1 | `main.js` | Bootstrap: capability checks (WebGL), wire modules, start refresh loop, intro sequence. |
| F2 | `state.js` | Tiny pub/sub store: cities, selection, unit, forecast cache, load status, search filter. |
| F3 | `api.js` | Only module calling `fetch`: `getWeather()`, `getForecast(city)` with timeouts, status mapping and URL-encoding. |
| F4 | `gazetteer.js` | ~150 cities `{name,country,lat,lng}` plus `normalize()` and `locate(record)` matcher with aliases and country disambiguation. |
| F5 | `weather.js` | Pure helpers: temperature to colour scale, description/conditions to icon kind, unit conversion/formatting. |
| F6 | `sun.js` | Pure solar position: `sunDirection(date)` (sub-solar lat/lng), used by the shader. |
| F7 | `globe-scene.js` | globe.gl setup: textures, patched day/night + specular + twilight + Fresnel + procedural clouds material, atmosphere, starfield, auto-rotate, pins (HTML elements), rings, fly-to, resize, visibility pause. |
| F8 | `ui.js` | DOM UI: detail panel, forecast cards, city list/search, unit toggle, legend, reset, status/error banners, live region, keyboard handling. All text via `textContent`/DOM APIs. |
| F9 | `dom.js` | Safe DOM builders: `el(tag, props, children)`, `svgEl(...)`; no HTML-string API. Icon SVG factories. |

## External dependencies (browser)
- `https://cdn.jsdelivr.net/npm/globe.gl@2.46.2/dist/globe.gl.min.js` with SRI sha384 + `crossorigin="anonymous"` (hash recorded in `index.html` and Build and Test docs).
- Textures from `https://cdn.jsdelivr.net/npm/three-globe@2.45.6/example/img/` (pinned exact version).
- No fonts or other third-party origins (system font stack).
