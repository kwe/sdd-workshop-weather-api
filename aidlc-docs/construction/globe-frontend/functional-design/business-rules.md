# Business Rules — globe-frontend

## BR-1 Safe rendering (NFR-2)
- BR-1.1 All API strings reach the DOM only via `el()`/`svgEl()` (strings become text nodes) or `textContent`. Forbidden everywhere in `static/js`: `innerHTML`, `outerHTML`, `insertAdjacentHTML`, `document.write`, `eval`, `new Function`, `setAttribute('on*')`, `setAttribute('style')`, string-form `setTimeout`.
- BR-1.2 API strings are sanitised for display: control characters (U+0000-U+001F, U+007F-U+009F) removed, whitespace collapsed, trimmed, then truncated (city/country 60, description/conditions 120 chars, with an ellipsis).
- BR-1.3 API values are never used as attribute names, element names, URLs, class names or CSS. `aria-label`/`title` values are set with `setAttribute(name, text)` using fixed names only. Numbers are validated with `Number.isFinite` before use, otherwise displayed as an em dash.
- BR-1.4 The city in the forecast URL is `encodeURIComponent(record.city)` (the original name, trimmed), so `/`, `?`, `#` and `%` in a crafted name cannot alter the path.
- BR-1.5 Only CSSOM (`style.setProperty`, custom properties) is used for dynamic styling.
- BR-1.6 A Rust test scans every embedded JS/HTML file for the forbidden tokens in BR-1.1 and fails the build on a hit.

## BR-2 CSP and headers (NFR-3, Q10 A)
`/` and every `/static/*` response carry:
```text
Content-Security-Policy: default-src 'none'; script-src 'self' https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https://cdn.jsdelivr.net; connect-src 'self'; font-src 'self'; worker-src 'self' blob:; object-src 'none'; base-uri 'self'; form-action 'none'; frame-ancestors 'none'
X-Content-Type-Options: nosniff
Referrer-Policy: no-referrer
Cache-Control: no-cache
```
- BR-2.1 `index.html` has no inline `<script>` and no inline event handlers. Its only external script besides `/static/js/main.js` (type=module) is globe.gl from `cdn.jsdelivr.net`, with `integrity="sha384-..."` and `crossorigin="anonymous"`, version pinned in the URL.
- BR-2.2 API routes, Swagger UI and `/v3/api-docs` do **not** receive these headers.
- BR-2.3 Fallback (pre-approved by the product owner): if patching the globe material cannot be made to work, use an import map plus a `'sha256-...'` script hash in `script-src`; document the change in the Code Generation summary.
- BR-2.4 Content types: `.html` `text/html; charset=utf-8`; `.js` `text/javascript; charset=utf-8`; `.css` `text/css; charset=utf-8`; `.svg` `image/svg+xml`; `.json` `application/json`.
- BR-2.5 Asset lookup is an exact table match; `..`, encoded slashes, absolute paths and empty paths return 404. `/static` and `/static/` return 404.

## BR-3 Gazetteer and city placement (FR-8, FR-10; Q3, Q4)
- BR-3.1 `normalize(s)`: NFD, strip combining marks, lowercase, remove `.,'’-()`, replace `&`/`_` with space, collapse whitespace, trim. Empty string stays empty.
- BR-3.2 Name aliases (normalised): "new york city", "nyc" -> "new york"; "bombay" -> "mumbai"; "calcutta" -> "kolkata"; "peking" -> "beijing"; "saigon" -> "ho chi minh city"; "mexico df", "cdmx" -> "mexico city"; "washington dc", "washington d c" -> "washington". Country aliases: uk/gb/great britain/england/united kingdom -> "united kingdom"; usa/us/america/united states/united states of america -> "united states"; others map by normalised name.
- BR-3.3 `locate(record)`: candidates = index[aliasOrName]. 0 -> null. If the record has a country (non-empty after normalisation): candidates filtered to the same normalised country; empty result -> null (no wrong pin); first remaining wins. If no record country: first candidate (most prominent) wins.
- BR-3.4 The gazetteer (~150 cities) must include the 10 seeded cities with their seeded countries, and a few same-name entries (e.g. Paris FR/US, London GB/CA) ordered most-prominent first. Coordinates are decimal degrees; a sanity check at module load drops (and `console.warn`s) any entry with out-of-range coordinates.
- BR-3.5 City key = `normalize(name) + "|" + normalize(country ?? "")`. For equal keys, the record with the lexicographically greatest `recordedAt` wins (ISO strings), ties broken by greater `id`.
- BR-3.6 Cities with `onGlobe = false` appear in the list with a "not on globe" tag and can be selected (panel opens, no camera move).

## BR-4 Temperature colour (Q1)
Stops `[-20 #7c5cff, -10 #3b82f6, 0 #22d3ee, 10 #34d399, 20 #facc15, 30 #fb923c, 40 #ef4444]`, linear interpolation per channel in sRGB between adjacent stops, clamped to the end colours; `null`/non-finite -> `#94a3b8`. The legend renders the same stops as a gradient with labels at -20, 0, 20, 40 in the active unit.

## BR-5 Condition mapping (Q2)
Text is lowercased and matched with these ordered rules, first match wins:
1. `storm|thunder` -> storm; 2. `snow|sleet|blizzard|flurr` -> snow; 3. `rain|drizzle|shower` -> rain; 4. `fog|mist|haze|smog` -> fog; 5. `partly|scattered|mostly sunny` -> partly; 6. `overcast|cloud` -> cloud; 7. `wind|breez|gust` -> wind; 8. `humid|muggy` -> humid; 9. `clear` -> sun when the sun is above the horizon at the city, moon otherwise (unknown location -> sun); 10. `sun|warm|hot|fair` -> sun; else/null -> unknown.
Seeded values check: Cloudy cloud; Sunny sun; Humid humid; Breezy wind; Partly Cloudy partly; Overcast cloud; Rainy rain; Warm sun; Windy wind; Clear sun/moon; Stormy storm.

## BR-6 Units and formatting (Q9)
- BR-6.1 `F = C*9/5 + 32`. Pins show whole degrees (`Math.round`; avoid "-0"), the panel one decimal. Unit symbols C / F. Wind always km/h with one decimal. Humidity integer %.
- BR-6.2 `recordedAt` "YYYY-MM-DDTHH:MM:SS(.fff)?" is parsed with a regex and formatted "17 Jul 2026, 08:00" without timezone conversion; unparsable -> shown trimmed as received.
- BR-6.3 Forecast `date` "YYYY-MM-DD" formatted via `Intl.DateTimeFormat('en-GB', {weekday:'short', day:'numeric', month:'short', timeZone:'UTC'})`; the first entry is labelled "Today" only when its date equals the browser's current UTC date.
- BR-6.4 Precipitation bar width = clamp(chance, 0, 100)% set via `style.setProperty('--p', ...)`; high/low shown with a range bar normalised across the five days' min-max.
- BR-6.5 A forecast is valid only when it is an array; each day with non-finite numbers is rendered with em dashes; if fewer than 5 days arrive, those received are shown.

## BR-7 Sun and lighting (FR-5; Q5)
- BR-7.1 Subsolar point: `N` = day of year (UTC); declination `δ = -23.44° * cos(360°/365 * (N + 10))`; `B = 360°/365 * (N - 81)`; equation of time `EoT = 9.87 sin 2B - 7.53 cos B - 1.5 sin B` (minutes); `lat_sun = δ`; `lng_sun = -15° * (UTChours - 12 + EoT/60)` normalised to [-180, 180]. Accuracy target: within about 1 degree (visual use only).
- BR-7.2 World coordinates follow three-globe: `phi = (90 - lat)`, `theta = (90 - lng)`, `x = sin(phi) cos(theta)`, `y = cos(phi)`, `z = sin(phi) sin(theta)` (unit vector).
- BR-7.3 Shader parameters are those in Q5 A (day weight, night weight, twilight tint, specular, Fresnel, clouds). Uniforms: `uSunDir`, `uNightMap`, `uWaterMask`, `uTime`, `uReduced`. Sun uniform refreshed every 60 s; cloud drift advances with `uTime` only when motion is allowed.
- BR-7.4 A city is "day" when `dot(cityVector, sunVector) > 0`.
- BR-7.5 If the night/water texture fails, or `onBeforeCompile` patching throws, the scene falls back to the plain day texture with the library atmosphere. During Code Generation the import-map approach (BR-2.3) is attempted first if patching cannot be made to work.

## BR-8 Camera and interaction (Q6)
- BR-8.1 Initial pose lat 20, lng from the sun's meridian (so the lit hemisphere faces the user), altitude 2.5 after an intro sweep from altitude 6 (2.5 s). Fly-to: altitude 0.9, 1.6 s. Reset: altitude 2.5, latitude 20, current longitude, 1.6 s.
- BR-8.2 Controls: min distance 130, max distance 600, rotate 0.6, zoom 0.8, damping on, pan disabled. Auto-rotate speed 0.25.
- BR-8.3 Auto-rotate stops on `pointerdown`, `wheel`, `touchstart`, `keydown` and while a city is selected; resumes 8 s after the last interaction if nothing is selected.
- BR-8.4 `prefers-reduced-motion: reduce` (evaluated live via `matchMedia` change events): no intro, no auto-rotate, no cloud drift, no pulse rings, transitions 0 ms, fly-to is a jump.
- BR-8.5 Rendering pauses (`pauseAnimation`) when `document.hidden` and resumes on visibility; device pixel ratio capped at 2; resize via `ResizeObserver`.

## BR-9 Pins and labels (Q7)
- BR-9.1 One pin per `City` with `onGlobe`. Element contents: glow dot (colour from BR-4), icon, temperature chip (text); name label hidden by default.
- BR-9.2 Name label visible when hovered, focused, selected, or when camera altitude < 1.4.
- BR-9.3 Pins facing away from the camera (behind the globe) are hidden (opacity 0, no pointer events) via the library's visibility hook.
- BR-9.4 Each pin is a `button` (`aria-label` "City, temperature, conditions"), Enter/Space select, hover/focus show the tooltip card.
- BR-9.5 Pulse rings: selected city strong (maxR 4, period 1600 ms); other pins faint (maxR 2, period 3200 ms), suppressed under reduced motion and when more than 60 pins are present (selected only).

## BR-10 Loading, refresh and errors (FR-15; Q11)
- BR-10.1 Fetch timeout 10 s (`AbortController`). Weather refresh every 60 s; paused while hidden; one request in flight.
- BR-10.2 Initial failure -> `status.phase = error`, message and Retry. Later failure -> keep data, `stale = true`, chip "Offline - showing data from HH:MM"; a later success clears it.
- BR-10.3 Empty array -> "No weather records yet".
- BR-10.4 Forecast: HTTP 404 -> `none` ("No forecast available for this city"); other failures -> `error` with Retry. Successful forecasts are cached for the session; `none` is cached for 60 s; `error` is never cached.
- BR-10.5 Selection that disappears after a refresh (record deleted): panel shows the last known data with a note "This record is no longer available" and closing resets the view.
- BR-10.6 No WebGL, globe.gl load failure or render context loss -> banner "3D globe unavailable"; list and panel remain usable.
- BR-10.7 Selecting a city while an older forecast request is in flight aborts the older request.

## BR-11 Accessibility (NFR-6/7; Q8, Q12)
- BR-11.1 Keyboard: `/` focuses search; `Esc` closes the panel (and clears search if focused); Up/Down move through the list; Enter selects; focus returns to the originating element on close.
- BR-11.2 Live region (polite) announces "Loaded N cities", "Selected London", "Forecast loaded for London", "Forecast unavailable", "Offline".
- BR-11.3 `prefers-reduced-motion` as in BR-8.4; `prefers-color-scheme` ignored (dark UI by design); touch targets >= 44 px; focus rings visible with at least 3:1 contrast.
- BR-11.4 The canvas container has `role="img"` and `aria-label="Interactive 3D globe showing current weather by city"`; the city list is the accessible alternative.

## BR-12 Preservation (NFR-1)
No change to existing source files other than adding `pub mod frontend;` and a router merge in `lib.rs`; `tests/api.rs` and unit tests unchanged.
