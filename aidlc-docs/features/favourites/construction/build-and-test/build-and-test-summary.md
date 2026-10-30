# Build and Test Summary: favourites

Run 2026-10-30 on branch `feature/favourites` after `git rebase main` (main = 188dbaf claims-check fix; no conflicts; no conflicted files to log).

## Commands
- `cargo fmt --check` -> OK (no output)
- `cargo clippy --all-targets --all-features -- -D warnings` -> `Finished dev profile`, no warnings
- `cargo test` (full suite):
  - lib unit tests: 4 passed
  - tests/api.rs: 17 passed
  - tests/favourites.rs: 8 passed
  - tests/frontend.rs: 7 passed
  - 0 failed

## Live check (PORT=8081, DATABASE_URL=sqlite:///tmp/favourites/w.db)
| Request | Result |
|---|---|
| GET /api/favourites (empty) | 200 `[]` |
| POST {"city":"London"} | 201 favourite with latest weather (id 1) |
| POST {"city":" lONdon "} | 200, same `favouritedAt`, no duplicate |
| POST {"city":"São Paulo"} | 201 |
| POST {"city":"Atlantis"} | 404 standard error body, path `/api/favourites` |
| POST {"city":"   "} / {"city":7} / `{bad` / 101 x chars | 400 each |
| GET /api/favourites | 200, São Paulo then London (newest first) |
| DELETE /api/favourites/LONDON | 204 |
| DELETE /api/favourites/London (again) | 404 |
| DELETE /api/favourites/S%C3%A3o%20Paulo | 204 |
| GET /api/favourites | 200 `[]` |
| GET /v3/api-docs | contains `/api/favourites` and `/api/favourites/{city}` |
| CSP header on /api/favourites | absent (as for other API routes) |
| GET /static/js/favourites.js | 200 `text/javascript`, CSP present |

Server stopped; port 8081 confirmed closed. (A first attempt returned HTTP 000 because the process was tied to a short-lived shell; re-run as a managed async process.)

Not verified: visual rendering of the globe UI in a browser (needs CDN access); covered by asset/forbidden-token tests only.
