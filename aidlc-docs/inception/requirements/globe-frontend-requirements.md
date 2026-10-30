# Requirements — Interactive Globe Frontend (Cycle 2)

## Intent Analysis
- **User request**: "let's add a frontend to this api. Make a interractive globe view of the world which the user can rotate and zoom into. Impress me. Add the weather from the api."
- **Request type**: New Feature (on top of the completed 5-day forecast cycle)
- **Scope**: Multiple components (static frontend assets + new Axum routes/headers)
- **Complexity**: Moderate | **Depth**: Standard
- **Sources**: product-owner brief and decisions, globe-frontend-requirement-questions.md (all 13 answered; no contradictions found)
- **Cycle 1 requirements** (`requirements.md`) are unchanged and still apply (NFR-2 compatibility in particular).

## Fixed Decisions (product owner)
Single-page app served by the existing Axum app at `/`; static HTML/JS/CSS in the repo; no Node build step; three.js / globe.gl and Earth textures from a public CDN; data only from `GET /api/weather` and `GET /api/weather/forecast/{city}`; polished, high-impact visuals; existing API behaviour and tests unchanged.

## Functional Requirements

### Serving
- **FR-1** `GET /` returns the single-page app (HTML, 200, `text/html; charset=utf-8`). Assets are served under `/static/*` (JS, CSS, any data files) with correct content types.
- **FR-2** All frontend files are embedded in the binary (`include_str!`/`include_bytes!`); no new crate and no runtime file dependency. Unknown `/static/*` paths return 404.
- **FR-3** Frontend routes must not alter any existing route, response or status code (`/api/*`, `/swagger-ui`, `/v3/api-docs`, `/swagger-ui.html`). Frontend routes are not listed in OpenAPI.

### Globe visuals
- **FR-4** Interactive 3D globe: drag/touch to rotate, scroll/pinch to zoom, with damping and sensible min/max zoom. Slow auto-rotate that pauses on user interaction and resumes after idle.
- **FR-5** Day/night blend driven by the real current sun position: day texture on the lit side, city night-lights texture on the dark side, with a smooth terminator that updates over time.
- **FR-6** Bump map relief, glowing atmosphere, starfield background, and a slowly drifting semi-transparent cloud layer (no wind particles).
- **FR-7** Polished overall design: glass-style UI chrome, consistent typography, smooth intro animation (camera sweep-in), loading and transition animations.

### Weather on the globe
- **FR-8** City location: a built-in client-side gazetteer of ~150 major world cities (lat/lng, matched by normalised name, with country as disambiguator), including all 10 seeded cities. Matching is case-, accent- and whitespace-insensitive.
- **FR-9** Each pinned city shows a glowing pin/ring colour-coded by temperature, a floating label (city + temperature in the selected unit), and a weather-condition icon/animation derived from `description` (sun, cloud, partly cloudy, rain, storm, wind, fog/humid etc.). Unknown descriptions fall back to a neutral icon.
- **FR-10** Records whose city is not in the gazetteer are listed in the city list (marked "not on globe") but not pinned. If several records exist for the same city, the most recent `recordedAt` is the one pinned.
- **FR-11** Hover shows a tooltip; click selects the city.

### City selection and detail
- **FR-12** Selecting a city (pin or list/search) flies the camera smoothly to it and zooms in, and opens a glass detail panel with current conditions (temperature, humidity, wind speed, description, recorded-at) and the 5-day forecast (date, high/low, conditions, precipitation chance) fetched on demand from `GET /api/weather/forecast/{city}` (city URL-encoded) and cached per city. Close / `Esc` / reset flies back out.
- **FR-13** Forecast failure states are per-panel: 404 shows "No forecast available for this city" (not an error banner); other failures show a retry control.

### Additional UI
- **FR-14** Searchable/filterable city list with fly-to, Celsius/Fahrenheit toggle (preference persisted in `localStorage`; API values are Celsius, wind stays km/h), a temperature colour legend, and a "reset view" control.

### Data lifecycle
- **FR-15** Load `/api/weather` on start; auto-refresh every 60 s without disturbing camera or selection; pause refresh while the tab is hidden. Show a loading state initially and a friendly error/retry state when the API fails (stale data stays visible after a failed refresh, with a subtle "offline/stale" indicator). A CDN/WebGL failure shows a clear fallback message plus a plain-HTML list of the weather data so the page is still useful.

## Non-Functional Requirements
- **NFR-1 Compatibility**: zero changes to existing API behaviour; all existing tests (4 unit, 17 integration) pass unmodified.
- **NFR-2 XSS safety (explicit)**: every API-sourced string (city, country, description, conditions) is rendered via `textContent`/DOM APIs or escaped; never `innerHTML`/`insertAdjacentHTML`/`outerHTML` with API data; cities can be created by anyone via `POST /api/weather`. Values are also never used to build URLs unencoded or inline event handlers.
- **NFR-3 CSP (explicit)**: frontend responses (`/` and `/static/*`) send a `Content-Security-Policy` header limiting script, style, img, font and connect sources to `'self'` plus only the specific pinned CDN host(s). No inline scripts; `object-src 'none'`, `base-uri 'self'`, `frame-ancestors 'none'`. API/Swagger responses do not get the CSP header (Swagger UI must keep working).
- **NFR-4 CDN integrity**: libraries pinned to exact versions on a public CDN, loaded with Subresource Integrity (`integrity` + `crossorigin`) hashes; textures loaded from the CDN over HTTPS with CORS.
- **NFR-5 Performance**: first meaningful paint (HTML, loading state) < 1 s locally; steady 60 fps target on a modern laptop with a ~150-city gazetteer and ~10-50 pins; capped device pixel ratio; textures at 2k-class resolution; animation loops pause when the tab is hidden.
- **NFR-6 Responsive and touch**: latest evergreen desktop browsers (Chrome, Edge, Safari, Firefox), plus tablets/phones (touch rotate, pinch zoom); panel becomes a bottom sheet on narrow screens.
- **NFR-7 Accessibility**: keyboard-operable controls and city list, visible focus, ARIA labels/roles, live region for load/error state, sufficient colour contrast, temperature not conveyed by colour alone (labels), `prefers-reduced-motion` disables auto-rotate, cloud drift, pulsing and fly-to animation (jump instead).
- **NFR-8 Quality gates**: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` pass.
- **NFR-9 Testing**: Rust integration tests for `GET /` (200, HTML content type, CSP header present, references no inline script), `/static/*` content types and 404, CSP not on `/api/weather`; manual smoke checklist; headless-browser screenshot check during Build and Test if a browser is available. No JS test toolchain.
- **NFR-10 Documentation**: README gets a "Frontend" section (URL, features, CDN dependencies, CSP, offline caveat) and updated project structure.
- **NFR-11 Maintainability**: plain ES modules/vanilla JS split into small files (data/gazetteer, API client, weather mapping, globe scene, UI panels), no build step; no secrets; no tracking.

## Extension Configuration (Cycle 2)
| Extension | Enabled | Notes |
|---|---|---|
| security-baseline | No | Full extension not enabled. Explicit requirements NFR-2 (XSS-safe rendering) and NFR-3 (CSP) added instead. |
| resiliency-baseline | No | Carried over. |
| property-based-testing | No | Carried over. |

## Out of Scope
Changes to the API, database or OpenAPI; lat/lng persistence; geocoding services; wind particles; authentication; a JS build/test toolchain; offline/vendored libraries (CDN is approved; offline use is unsupported).

## Assumptions / Risks
- Public CDN availability (and SRI hashes must be computed from the pinned files at build time; if the network is unavailable the pinned versions/hashes will need verification).
- CSP must allow what globe.gl/three.js need (e.g. `blob:`/`data:` images, workers) while staying restricted; to be settled in Application/Functional Design.
- Texture host(s) are extra CDN origins that CSP and CORS must cover.
