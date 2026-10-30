# Requirements Verification Questions — Interactive Globe Frontend

Request (verbatim): "let's add a frontend to this api. Make a interractive globe view of the world which the user can rotate and zoom into. Impress me. Add the weather from the api."
Intent: New Feature (second cycle, builds on the completed 5-day forecast cycle) | Scope: Multiple components (new static frontend + one new Axum route) | Complexity: Moderate | Depth: Standard.

Already decided by the product owner (not re-asked): single-page app served by Axum at `/`; static HTML/JS/CSS in the repo, no Node build step; three.js / globe.gl and Earth textures from a public CDN approved; data from `GET /api/weather` and `GET /api/weather/forecast/{city}`; visually impressive and polished; existing API behaviour and tests unchanged.

Please fill in the letter after each [Answer]: tag. Reply "use defaults" to accept every option marked (default).

## Question 1
The API returns no coordinates. How should the frontend place a city on the globe?

A) Built-in client-side gazetteer: a JS table of ~150 major world cities (lat/lng) matched by name/country, containing the 10 seeded cities; records for unknown cities are listed in the side panel but not pinned (default)

B) Gazetteer as in A, plus a live geocoding fallback (e.g. OpenStreetMap Nominatim) for unknown cities (adds a second third-party dependency and rate limits)

C) Gazetteer as in A, but unknown cities fall back to their country's centroid (needs a country table too)

D) Add lat/lng columns to the API/database (changes the API; conflicts with "existing API behaviour must not change")

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
Visual style of the globe — "impress me" is subjective, so which direction?

A) Cinematic night-Earth: NASA "blue marble"/night-lights texture, bump map, glowing atmosphere, starfield background, slow auto-rotate that pauses on interaction (default)

B) Realistic day-side Earth with live sun-position day/night terminator shading

C) Stylised/minimal dark "data-viz" globe (hex-bin land, glowing arcs, neon)

D) Combination of A and B (night lights on the dark side, day texture on the lit side, driven by the real sun position)

X) Other (please describe after [Answer]: tag below)

[Answer]: D — real-sun day/night blend (day texture on lit side, city night-lights on the dark side) PLUS everything in A: bump map, glowing atmosphere, starfield background, slow auto-rotate that pauses on interaction

## Question 3
How should current weather be shown on the globe?

A) Glowing pins/rings per city, colour-coded by temperature, with a floating label showing city + temperature; hover/click for details (default)

B) A) plus weather-condition icons/animations on the pins (sun, cloud, rain, wind) derived from `description`

C) Heat-map style temperature overlay across the globe (interpolated between cities)

D) Pins only, minimal labels

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 4
What should happen when the user selects a city?

A) Camera flies smoothly to the city and zooms in; a glass-style detail panel slides in with current conditions (temp, humidity, wind, description, recorded-at) and the 5-day forecast (high/low, conditions, precipitation chance) loaded from `/api/weather/forecast/{city}`; Esc/close flies back out (default)

B) Same as A but forecast opens in a modal instead of a side panel

C) Detail shown in a tooltip only; no camera fly-to

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
Additional UI features beyond the globe itself (pick the set).

A) Searchable/filterable city list or search box with fly-to, a Celsius/Fahrenheit toggle, a legend, and a "reset view" control (default)

B) A) plus a "wind particles" or cloud-layer ambient effect

C) Only the globe and the detail panel

X) Other (please describe after [Answer]: tag below)

[Answer]: X — A plus a slowly drifting semi-transparent cloud layer (no wind particles)

## Question 6
Data freshness and failure behaviour.

A) Load `/api/weather` on start, auto-refresh every 60 s without disturbing the camera; show a loading state and a clear, friendly error/retry state if the API or the CDN fails; forecast per city fetched on demand and cached (default)

B) Load once at start; manual refresh only

C) Live refresh every 10 s

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7
How should the three.js / globe.gl libraries and textures be loaded?

A) Pinned versions from a public CDN (e.g. jsDelivr/unpkg) with Subresource Integrity hashes where practical; textures from the CDN too; no vendoring (default)

B) Pinned CDN versions without SRI

C) Vendor the JS libraries into the repo (offline-capable JS) but load textures from the CDN

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8
How should the Axum app serve the frontend?

A) Embed the static files into the binary (e.g. `include_str!`/`include_bytes!`) and serve `/` plus `/static/*` routes with no new crate; single self-contained binary (default)

B) Add `tower-http` `ServeDir` serving a `static/` directory at runtime (new dependency; files must be present next to the binary)

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 9
Device, browser and accessibility targets.

A) Latest evergreen desktop browsers (Chrome/Edge/Safari/Firefox) plus responsive/touch support for tablets and phones (touch rotate, pinch zoom); keyboard-accessible controls, ARIA labels, `prefers-reduced-motion` disables auto-rotate/animations (default)

B) Desktop only, mouse interaction

C) A) without the accessibility work

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 10
Verification approach for the frontend (there is no JS build/test toolchain).

A) Rust integration tests for the new routes (`/` returns 200 HTML, static assets served with correct content types, existing tests untouched) plus a manual smoke checklist and a headless-browser screenshot check during Build and Test if a browser is available (default)

B) Rust integration tests only

C) Add a JS unit-test setup (introduces Node tooling; conflicts with "no Node build step")

X) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 11
Security Extensions: should security extension rules be enforced for this feature? (Previously opted out for the forecast cycle; re-asked because this feature adds browser-facing code and third-party CDN dependencies.)

A) Yes — enforce all SECURITY rules as blocking constraints

B) No — skip all SECURITY rules (carry over the earlier decision) (default)

X) Other (please describe after [Answer]: tag below)

[Answer]: X — Do not enable the full security extension, but add these as explicit requirements: all API-sourced strings (city, country, description, conditions) must be rendered XSS-safely (textContent / DOM APIs or escaping; never innerHTML with API data), because cities can be created by anyone via POST /api/weather; and the frontend responses should send a Content-Security-Policy header restricting scripts/styles/images/connect to self plus the specific pinned CDN host(s).

## Question 12
Resiliency Extensions: should resiliency extension rules be enforced for this feature?

A) Yes — enforce all resiliency rules as blocking constraints

B) No — skip all resiliency rules (carry over the earlier decision) (default)

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 13
Property-Based Testing Extension: should property-based testing rules be enforced for this feature?

A) Yes — enforce all PBT rules as blocking constraints

B) Partial — enforce PBT rules only for pure functions and serialization round-trips

C) No — skip all PBT rules (carry over the earlier decision) (default)

X) Other (please describe after [Answer]: tag below)

[Answer]: C
