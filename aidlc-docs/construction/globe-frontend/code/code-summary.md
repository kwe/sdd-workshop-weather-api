# Code Summary - globe-frontend

## Created (application code)
- `src/frontend.rs` - `router()` serving `/` and `/static/{*path}` from an exact-match embedded asset table; `CONTENT_SECURITY_POLICY` and security headers (CSP, nosniff, no-referrer, no-cache, X-Frame-Options) applied to these routes only.
- `static/index.html` - no inline script; globe.gl 2.46.2 with SRI (sha384) and `crossorigin`.
- `static/css/app.css`, `static/favicon.svg`.
- `static/js/`: `main.js` (bootstrap, refresh, forecast loading), `state.js` (store), `api.js` (only `fetch` user), `gazetteer.js` (~150 cities, aliases, country disambiguation), `weather.js` (colours, condition rules, formatting, sanitising), `sun.js` (subsolar point), `dom.js` (safe builders, icons, no HTML-string API), `globe-scene.js` (globe.gl, shader patch, pins, camera), `ui.js` (list, search, detail panel, status, keyboard).
- `tests/frontend.rs` - 6 tests (headers, no inline script + SRI, content types, 404/traversal, API has no CSP, forbidden-token scan + fetch isolation).

## Modified
- `src/lib.rs` - `pub mod frontend;` and `.merge(frontend::router())`.
- `README.md` - "Frontend (Weather Globe)" section and project structure.

No changes to existing handlers, models, migrations or `tests/api.rs`. Nothing committed.

## Decisions and deviations
- **Shader**: patching the globe's Phong material with `onBeforeCompile` worked, so the import-map + script-hash fallback was NOT needed. Classes are derived from live instances (`material.map.constructor`); patch installs once the day texture has loaded. Failure falls back to plain textures.
- **Visual tuning after live screenshots** (BR-7 values adjusted): ocean specular shininess 60 -> 120 and strength 0.9 -> 0.45 (the original produced a washed-out white hotspot); cloud noise frequency 4.5 and smoothstep 0.55-0.80 (original blobs too large). Other parameters as designed.
- Pins are wrapped in a root `div` because globe.gl positions elements with an inline transform that CSS animations would override.
- Extra header `X-Frame-Options: DENY` (defence in depth alongside `frame-ancestors 'none'`).
- `style-src` keeps `'unsafe-inline'` as designed (globe.gl injects inline styles on its own elements); app code uses CSSOM only.
- Deployment artefacts: N/A.
