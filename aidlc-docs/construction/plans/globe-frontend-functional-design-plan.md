# Functional Design Plan — unit `globe-frontend`

Units Generation was skipped, so the single unit is `globe-frontend` covering `globe-frontend-requirements.md` (FR-1..FR-15, NFR-1..11). Cycle 1's `forecast-api-functional-design-plan.md` is unchanged.

**Per your instruction, every answer below is an AI-proposed default (marked `[Answer]: A` with the reasoning given in the option) so you can review it.** The generated artefacts already incorporate them; tell me which to change and I will update the design. Pre-approved by you: if `onBeforeCompile` patching fails during Code Generation, switch to the import-map + CSP script-hash fallback without asking.

## Part A: Questions (with proposed defaults)

## Question 1
Temperature colour scale for pins, legend and forecast bars.

A) Piecewise-linear gradient over stops: -20C violet `#7c5cff`, -10C blue `#3b82f6`, 0C cyan `#22d3ee`, 10C green `#34d399`, 20C yellow `#facc15`, 30C orange `#fb923c`, 40C red `#ef4444`; clamped beyond the ends; unknown temperature `#94a3b8` grey; plain sRGB interpolation (simple and predictable)

B) Discrete buckets of 10C

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
Mapping the free-text `description`/`conditions` to an icon kind (the API has no fixed vocabulary; cities can be created by anyone).

A) Ordered, case-insensitive keyword rules, first match wins: storm/thunder -> storm; snow/sleet/blizzard/flurr -> snow; rain/drizzle/shower -> rain; fog/mist/haze/smog -> fog; partly/scattered/mostly sunny -> partly; overcast/cloud -> cloud; wind/breez/gust -> wind; humid/muggy -> humid; clear -> clear (sun by day, moon at night, decided by the sun elevation at the city); sun/warm/hot/fair -> sun; anything else (or null) -> unknown (neutral thermometer icon). Text itself is always shown next to the icon

B) Exact match against a fixed list only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Gazetteer matching and ambiguity rules.

A) Normalise (NFD, strip diacritics, lowercase, remove punctuation, collapse spaces) and apply a small alias table (e.g. "new york city", "nyc", "bombay", "calcutta", "peking", "saigon"). Country is normalised too with aliases (UK/United Kingdom/England/GB, USA/US/United States/America). Rules: one candidate and no record country -> accept; one candidate and a recognised record country that differs -> not placed (avoids a wrong pin); several candidates -> choose the one whose country matches, else if no record country the first (gazetteer is ordered by prominence), else not placed. Gazetteer includes a few same-name cities (e.g. Paris FR/US, London UK/CA) to exercise this

B) Name-only matching

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
Duplicate records for the same city (the API can hold several).

A) City key = normalised name + `|` + normalised country; the record with the latest `recordedAt` represents the city on the globe, in the list and in the panel; the list shows one row per key. Forecasts are requested by city name (the API has no country parameter), cached per key

B) Show every record as its own pin

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
Day/night, specular, twilight, Fresnel and cloud parameters (premium look).

A) Sun lighting factor `d = dot(surfaceNormal, sunDir)`. Day weight `smoothstep(-0.10, 0.20, d)`; night-lights weight `1 - smoothstep(-0.20, 0.05, d)` scaled 1.4; warm twilight band tint `#ff8a4c` with strength 0.25 peaking where `d` is about 0.03 (`exp(-(d/0.12)^2)`); ocean specular uses `earth-water.png` as mask, Blinn-Phong with shininess 60 and strength 0.9 (warm white, day side only); Fresnel rim `pow(1 - dot(N, V), 3)` in `#5aa9ff`, strength 0.35 on the day side and 0.15 on the night side; library atmosphere `#4da6ff`, altitude 0.2; bump via `earth-topology.png` (library bump scale 4); day texture `earth-blue-marble.jpg`, night `earth-night.jpg`; procedural clouds = 5-octave fbm noise sampled on the sphere direction, coverage `smoothstep(0.52, 0.78)`, drifting by rotating the sample direction about the polar axis at 0.002 rad/s, white lit by the sun with a faint darkening on the night side, max opacity 0.85. If any texture fails to load the globe falls back to the plain day texture without custom effects

B) Simple blend only, no twilight/specular/Fresnel

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 6
Camera behaviour.

A) Initial altitude 2.5 (intro sweep from altitude 6 over 2.5 s); fly-to altitude 0.9 over 1.6 s; min distance 130 (about altitude 0.3), max distance 600; rotate speed 0.6, zoom speed 0.8, damping on; auto-rotate speed 0.25, resumes 8 s after the last interaction when nothing is selected; reset view returns to altitude 2.5 at latitude 20 and the current longitude; reduced motion: no intro, no auto-rotate, no cloud drift, no pulse rings, fly-to becomes an instant jump

B) Different values (describe)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7
Pin and label presentation (avoid clutter, keep it legible).

A) Each pin is a DOM element: colour-coded dot with a soft glow, condition icon, and a temperature chip (whole degrees). City name appears on hover/focus, when selected, and for every pin when the camera altitude is below 1.4. Pins behind the globe fade out. Pulse rings (CSS-independent `ringsData`) on the selected city and slowly on all pins at low strength. Clicking or pressing Enter/Space on a focused pin selects it

B) Dots only, labels only on hover

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8
Layout and responsive behaviour.

A) Desktop (>= 900 px): globe fills the viewport; left glass column (320 px) with title, search, unit toggle and city list; right glass detail panel (380 px) when a city is selected; bottom-left legend; top-right reset view. Tablet/phone (< 900 px): list collapses to a bottom drawer (toggle button), detail opens as a bottom sheet up to 65% of the height, controls stay reachable with touch targets at least 44 px

B) Same layout at all sizes

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 9
Data display rules.

A) Temperatures: pins whole degrees, panel one decimal; Fahrenheit conversion `C*9/5+32`; wind stays km/h; humidity as %; missing values shown as an em dash. `recordedAt` is a naive date-time string with no zone and is displayed as given (e.g. "17 Jul 2026, 08:00") without any timezone conversion. Forecast `date` is displayed as a short weekday + day/month computed in UTC. Precipitation shown as a bar plus a percentage. Long strings are trimmed for display (city 60 chars, description 120 chars, control characters removed)

B) Raw API values only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 10
Exact CSP and headers for `/` and `/static/*`.

A) `default-src 'none'; script-src 'self' https://cdn.jsdelivr.net; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: https://cdn.jsdelivr.net; connect-src 'self'; font-src 'self'; worker-src 'self' blob:; object-src 'none'; base-uri 'self'; form-action 'none'; frame-ancestors 'none'`, plus `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `Cache-Control: no-cache`. App code uses only CSSOM (`style.setProperty`), never `style` attributes or `<style>` blocks, so tightening `style-src` stays possible. `connect-src` and `worker-src` are tightened/loosened during Build and Test only as evidence requires, and any change is recorded. If the import-map fallback is used, its script hash is added to `script-src`

B) Looser policy (`https:` wildcards)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 11
Error, empty and fallback states.

A) Initial load: skeleton/loading veil with a spinner and the title; failure: friendly message with Retry and the HTML list hidden; empty list from the API (`[]`): "No weather records yet" message; refresh failure: keep data, show a small "Offline - showing data from HH:MM" chip; forecast 404: "No forecast available for this city"; forecast other error: message with Retry; no WebGL or CDN failure: banner "3D globe unavailable" and a plain list/table of the weather data plus working panel/forecasts; fetch timeout 10 s

B) Alert dialogs for errors

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 12
Accessibility and keyboard.

A) Canvas has `role="img"` and an `aria-label`; city list is a list of buttons (`aria-pressed` for the selected one); detail panel is a non-modal `role="complementary"` region labelled by the city name; a polite live region announces loads, selection and forecast results; `/` focuses search, `Esc` closes the panel, arrow Up/Down moves in the list; visible focus rings; colour is never the only signal (temperature text on every pin and list item); contrast at least 4.5:1 for text on glass panels (panels use a dark translucent background with a blur and a solid fallback)

B) Minimal ARIA only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Part B: Artefact Checklist
- [x] `globe-frontend/functional-design/business-logic-model.md`
- [x] `globe-frontend/functional-design/business-rules.md`
- [x] `globe-frontend/functional-design/domain-entities.md`
- [x] `globe-frontend/functional-design/frontend-components.md`
