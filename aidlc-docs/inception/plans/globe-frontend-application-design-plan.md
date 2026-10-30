# Application Design Plan — Globe Frontend (Standard Depth)

Cycle 1's `application-design-plan.md` and `application-design/` artefacts are preserved. Cycle 2 design artefacts go in `aidlc-docs/inception/application-design/` with a `globe-frontend-` prefix.

Please fill in each `[Answer]: A` tag (reply "use defaults" to accept every option marked (default)), then tell me when you're done.

## Part A: Design Questions

## Question 1
Rust side: where should serving live?

A) New `src/frontend.rs` exposing `pub fn router() -> Router<SqlitePool>`-style routes for `/` and `/static/*`, merged in `lib.rs`; assets embedded with `include_str!`/`include_bytes!` from a top-level `static/` directory; a small CSP/headers layer applied only to this router (default)

B) Put the handlers directly in `lib.rs`

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 2
Frontend code structure (no build step).

A) Native ES modules (`<script type="module">`), one small file per concern: `main.js`, `api.js`, `gazetteer.js`, `weather.js` (colour/icon mapping), `sun.js` (solar position), `globe-scene.js`, `ui.js` (panel/list/search/units), `state.js`; each embedded and served from `/static/js/` (default)

B) A single large `app.js`

C) Other (please describe after [Answer]: A tag below)

[Answer]: A — plus ocean specular highlights, a soft twilight band at the terminator, and a Fresnel atmosphere glow (premium look)

## Question 3
CDN libraries and hosts.

A) One host, jsDelivr, pinned exact versions: `globe.gl` (UMD bundle, which includes three.js) with SRI; Earth textures (day, night lights, topology/bump, sky, clouds) from the `three-globe` example images on the same host; CSP allows only `cdn.jsdelivr.net` beyond `'self'` (default)

B) unpkg for libraries and jsDelivr for textures (two hosts)

C) Separate `three` and `globe.gl` modules via an import map

D) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 4
Pin rendering approach (condition icons, pulsing, labels, hover, click).

A) globe.gl `htmlElementsData` layer built with DOM APIs (SVG icon via `createElementNS`, label via `textContent`), plus `ringsData` for pulses; fine for the expected tens of cities (default)

B) WebGL-only objects (`pointsData` + `labelsData`), icons as sprites

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 5
Day/night shading implementation.

A) Custom `ShaderMaterial` on the globe mesh (via `globeMaterial()`) that mixes day and night textures using a sun-direction uniform updated every ~60 s from `sun.js`, plus bump map; clouds as a second slightly larger sphere with slow rotation (default)

B) Day texture only plus a semi-transparent dark overlay polygon for the night side

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 6
Gazetteer packaging.

A) ES module `gazetteer.js` exporting an array of `{name, country, lat, lng}` (~150 cities) plus a normalising matcher (lowercase, accent-stripped, trimmed, with a few aliases such as "New York City"); country is used to disambiguate duplicates such as Paris, France vs Paris, USA (default)

B) A JSON file fetched at start

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 7
CSP style policy. Some globe libraries inject a `<style>` element at runtime, so a strict `style-src 'self'` may break them.

A) `script-src 'self' cdn.jsdelivr.net` (no inline, no eval) and `style-src 'self' 'unsafe-inline'` (inline styles only; app code uses no inline `<style>`); try stricter during Build and Test if the libraries allow it (default)

B) Fully strict `style-src 'self'` (may need the libraries to be patched or may break); fail the build if it does not work

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 8
Cache headers for embedded assets.

A) `Cache-Control: no-cache` on `/` and `/static/*` (browser revalidates every load; assets are embedded so the cost is tiny; avoids stale UI after upgrades) (default)

B) Long `max-age` for `/static/*` with a version query string

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 9
Frontend state and data flow.

A) A tiny publish/subscribe store in `state.js` (cities, selection, unit, forecast cache, load status) that `ui.js` and `globe-scene.js` subscribe to; `api.js` is the only module calling `fetch` (default)

B) Direct calls between modules, no store

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Question 10
Rust test file layout.

A) New `tests/frontend.rs` using the existing in-memory app pattern; `tests/api.rs` not modified (default)

B) Append to `tests/api.rs`

C) Other (please describe after [Answer]: A tag below)

[Answer]: A

## Part B: Design Artefact Checklist (executed after answers)
- [ ] Generate `globe-frontend-components.md` (Rust serving module; JS modules; external CDN dependencies)
- [ ] Generate `globe-frontend-component-methods.md` (Rust function signatures; JS module exports and data shapes)
- [ ] Generate `globe-frontend-services.md` (data-loading/refresh, selection/fly-to, forecast loading orchestration)
- [ ] Generate `globe-frontend-component-dependency.md` (dependency matrix, communication, data-flow diagram)
- [ ] Generate `globe-frontend-application-design.md` (consolidated)
- [ ] Validate design completeness and consistency against `globe-frontend-requirements.md`
