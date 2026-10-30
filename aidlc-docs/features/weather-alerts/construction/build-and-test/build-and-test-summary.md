# Build and Test Summary - weather-alerts

Branch `feature/weather-alerts`, rebased twice. The first rebase (onto 188dbaf) was clean. `main` then moved (favourites merged, 36ca732), so I rebased again.

Second rebase conflicts (all resolved keeping BOTH features; nothing of favourites dropped):
`src/frontend.rs` (asset list), `src/lib.rs` (paths, schemas, tags), `static/js/main.js` (imports, helper functions, subscribe), `static/js/state.js`, `static/js/ui.js` (import, render calls). `globe-scene.js`, `api.js`, `index.html` and `app.css` merged automatically. Gate and live check re-run after this rebase.

## Results
| Command | Result |
| --- | --- |
| `cargo fmt --check` | OK |
| `cargo clippy --all-targets --all-features -- -D warnings` | Finished, no warnings |
| `cargo test` (full) | all suites pass: 15 lib (unit), 13 `alerts`, 5 `alerts_frontend`, 17 + 8 + 7 existing suites (incl. favourites), 0 failures |

## Live check (PORT=8082, fresh temporary DB, server stopped afterwards)
- `GET /api/alerts` -> 200, 10 alerts. First entry:
  `{"city":"Mumbai","country":"India","type":"heat","severity":"watch","value":31.7,"threshold":30.0,"source":"current","date":"2026-07-17"}`
- `GET /api/favourites` -> 200 (favourites coexists). `GET /` -> 200. `GET /v3/api-docs` contains `/api/alerts`.
- Not verified: the globe UI in a real browser (rings, banner, reduced motion); only static checks cover it.

## Seed alert count: design said 12, code yields 10
The code is correct; the design assumption was wrong. The first draft assumed Tokyo's forecast contributes all 5 days. The seeded Tokyo highs are 30.5, 31.0, 29.0, 28.5 and 30.0, so only days 0, 1 and 4 reach the 30 C threshold. Tokyo gives 1 current + 3 forecast = 4. Mumbai's highs (32.0, 31.5, 31.0, 32.5, 32.0) all qualify: 1 + 5 = 6. Total 10. The design (business-rules A4), plan step 9 and `tests/alerts.rs` were corrected earlier.

## Third rebase and real-browser check (main e353ad1)
Rebased onto `main` e353ad1 (favourites 'null' label fix + nullish append() scanner); clean, no conflicts. Gate re-run: `cargo fmt --check` OK, clippy `-D warnings` clean, full `cargo test` all suites pass (15 lib, 13 alerts, 5 alerts_frontend, 17, 8, 9 existing), including the new append scanner over `alerts.js`.

Browser check: server on PORT=8082 with a temp DB; `POST /api/favourites {"city":"Tokyo"}` -> 201; then `node /tmp/weather-alerts/cdp.mjs http://localhost:8082/`:
- No PAUSED/EXC lines (no JS exceptions).
- All NET lines 200: every local asset (incl. `alerts.js`, `favourites.js`), `/api/weather`, `/api/favourites`, `/api/alerts`, and the CDN globe.gl/textures.
- No literal `null`/`undefined` in the output.
- Page text contains the alerts banner `10 alerts` (script truncates text at 300 chars, so the banner summary chips are cut off).
- Tokyo is a favourite: a `FAVOURITES` section lists `Tokyo 30°`, and `★` appears on its pin.
- Not checked: ring pulsing visually, banner click flying to a city, reduced motion (the script only dumps text/network).
Server stopped (port 8082 free); temp DB removed.
