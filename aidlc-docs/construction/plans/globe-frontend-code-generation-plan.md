# Code Generation Plan - unit `globe-frontend`

**This plan is the single source of truth for Code Generation.** Brownfield: only `src/lib.rs` is modified in place; everything else is new. Application code lives at the workspace root (`/Users/evanke02/Github/course/weather-api`), never in `aidlc-docs/`. Existing API behaviour and the 21 existing tests must not change. No commit will be made.

## Unit Context
- **Requirements**: FR-1..15, NFR-1..11 (`inception/requirements/globe-frontend-requirements.md`)
- **Design inputs**: `inception/application-design/globe-frontend-*.md`; `construction/globe-frontend/functional-design/{business-rules,business-logic-model,domain-entities,frontend-components}.md`
- **Dependencies**: no new crates. Runtime CDN: globe.gl 2.46.2 (SRI-pinned) and three-globe 2.45.6 textures from jsDelivr.
- **Interfaces consumed**: `GET /api/weather`, `GET /api/weather/forecast/{city}` (unchanged)
- **Interfaces provided**: `GET /` (HTML), `GET /static/{*path}` (embedded assets), CSP and security headers on those routes only
- **Extensions**: security, resiliency, PBT disabled; explicit XSS-safe rendering and CSP (FR/NFR) are implemented and tested instead
- **Fallback (pre-approved)**: if `onBeforeCompile` patching fails, use import map + CSP `'sha256-...'` script hash and record it in the summary

## Steps

### Rust (static serving)
- [x] **Step 1**: Create `src/frontend.rs` - `router() -> Router<SqlitePool>`, routes `/` and `/static/{*path}`, exact-match embedded asset table (`include_str!`/`include_bytes!`), content types, `pub const CONTENT_SECURITY_POLICY`, `map_response` layer adding CSP, `X-Content-Type-Options`, `Referrer-Policy`, `Cache-Control: no-cache` to frontend routes only. *(FR-1, NFR CSP)*
- [x] **Step 2**: Modify `src/lib.rs` - `pub mod frontend;` and `.merge(frontend::router())` before `.with_state(pool)`. Not added to OpenAPI.

### Static assets (`static/`)
- [x] **Step 3**: Compute the real SRI hash for globe.gl 2.46.2; create `static/index.html` (no inline script/style, SRI + crossorigin on globe.gl script, module entry, semantic landmarks, noscript/fallback container).
- [x] **Step 4**: Create `static/css/app.css` (design tokens, glass panels, desktop layout and mobile bottom sheet, reduced-motion, focus styles) and `static/favicon.svg`.
- [x] **Step 5**: Create pure-logic modules: `js/state.js` (pub/sub store), `js/api.js` (only `fetch` user; timeout, `encodeURIComponent`, forecast cache), `js/gazetteer.js` (~150 cities, normalisation, aliases, country disambiguation), `js/weather.js` (colour scale, condition keyword rules incl. day/night, formatters, sanitise/truncate), `js/sun.js` (subsolar point). *(BR-1..6)*
- [x] **Step 6**: Create `js/dom.js` (safe element builder, no HTML-string API) and `js/ui.js` (city list/search, detail panel with 5-day forecast, stale/error chips, live region, keyboard shortcuts, plain-HTML fallback).
- [x] **Step 7**: Create `js/globe-scene.js` (globe.gl setup, textures, Phong `onBeforeCompile` patch: day/night, twilight band, ocean specular, Fresnel rim, procedural drifting clouds; pins/pulses, camera intro/fly-to/auto-rotate, reduced-motion) and `js/main.js` (bootstrap, 60 s refresh, wiring). *(BR-7..9)*

### Tests
- [x] **Step 8**: Create `tests/frontend.rs` (existing test files untouched): `/` 200 + HTML + CSP; no inline script and SRI present; static JS/CSS content types; unknown path and `..` -> 404; `/api/weather` has no CSP header; forbidden-token scan (`innerHTML`, `outerHTML`, `insertAdjacentHTML`, `document.write`, `eval(`, `new Function`, `setAttribute('style'`, `on*` handlers) over embedded JS.

### Documentation
- [x] **Step 9**: Modify `README.md` - "Frontend" section (usage at `/`, CDN dependencies, CSP, offline caveat, structure). Create `aidlc-docs/construction/globe-frontend/code/code-summary.md`.

### Verification (full detail in Build and Test)
- [x] **Step 10**: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` (21 existing + new tests); live run with curl; headless Chrome screenshot (`/Applications/Google Chrome.app`); check browser console for CSP violations; apply fallback if shader patching fails.

## Requirement Traceability
| Requirement area | Steps |
|---|---|
| Served at `/`, no Node build (FR-1) | 1, 2, 3 |
| Globe visuals, shader, camera (FR visuals) | 4, 7 |
| API data, gazetteer, forecast (FR data) | 5, 6 |
| XSS safety and CSP (Q11) | 1, 5, 6, 8 |
| Tests / docs | 8, 9, 10 |

**Total**: 10 steps; ~16 new files, 1 modified source file, 1 modified README.
