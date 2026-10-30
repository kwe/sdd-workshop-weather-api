# Build and Test Summary - globe-frontend (cycle 2)

Cycle 1 build-and-test files in this directory are preserved; this file covers the globe frontend only.

## Build
- `cargo build` / `cargo run` - no new crates, no Node/npm build step. Frontend assets are embedded with `include_str!` (rebuild after editing `static/`).

## Automated results
| Check | Result |
|---|---|
| `cargo fmt --check` | pass |
| `cargo clippy --all-targets --all-features -- -D warnings` | pass |
| `cargo test` | 27 passed: 4 unit + 17 existing integration (unchanged) + 6 new in `tests/frontend.rs` |
| JS syntax (`node --check` as modules) | pass for all 9 modules |

## Live run (headless Chrome via puppeteer-core in /tmp, SwiftShader WebGL)
Server on :8080 with a temporary SQLite DB (deleted afterwards).
- `curl /`: 200, `text/html`, CSP + nosniff + no-referrer + no-cache headers present; `/api/weather` unchanged and without CSP.
- Page load: no CSP violations, no page errors; the only console message is the expected 404 for a city without a forecast.
- Shader patch active: day/night terminator, night-light map, specular, Fresnel rim and drifting clouds render; 10 pins placed at gazetteer positions.
- Interactions verified: select from list (fly-to, detail panel, 5-day forecast), °C/°F toggle, search filter ("tok" -> 1 result), close panel, mobile viewport (390x844) bottom sheet.
- XSS probe: a record with `<img src=x onerror=alert(1)>/?#`, `<b>x</b>` and a `<script>` description rendered as literal text; 0 injected nodes, 0 dialogs; forecast request URL-encoded and 404 handled as "No forecast available".
- Screenshots (session files): `files/home.png`, `files/detail-f.png`, `files/mobile.png` (copies in `/tmp/wg/`).

## Manual checklist (not automatable here)
- Real GPU browser: drag/zoom feel, auto-rotate resume after 8 s, intro sweep, pins hidden behind the globe.
- `prefers-reduced-motion`, keyboard (`/`, `Esc`, arrows), screen-reader announcements.
- Offline CDN: city list stays usable with the "3D globe unavailable" banner.

## Known limitations
- No JS unit-test toolchain (by design: no Node build); logic is covered by the live run and the Rust asset tests.
- Globe needs internet access to jsDelivr (SRI-pinned).
- Mobile: the camera is not offset to keep the selected pin above the bottom sheet.

Performance, contract and security-extension tests: N/A (extensions disabled; no performance requirement).
