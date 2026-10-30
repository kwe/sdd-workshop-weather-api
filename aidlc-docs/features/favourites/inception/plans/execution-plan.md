# Execution Plan: favourites

## Detailed Analysis
- **Change type**: New feature, brownfield, backend + frontend, single crate (one unit of work).
- **Risk**: Low-moderate. Additive only (new table, new routes, new JS module); shared-file hooks are small. Main risks: merge conflicts with the parallel feature in `src/lib.rs`, `src/frontend.rs`, `static/css/app.css`, `static/js/ui.js`/`main.js`, README; and CSP/XSS regressions.
- **User impact**: Direct (new UI), single persona (globe user).

## Stage Plan
| Stage | Decision | Rationale |
|---|---|---|
| Workspace Detection | Done | |
| Reverse Engineering | Skipped (delta note written) | Parallel mode §2 |
| Requirements Analysis | Done | requirements.md |
| User Stories | SKIP | One persona, acceptance criteria already in FR/TR; override if you want stories |
| Workflow Planning | This document | |
| Claims + overlap check | EXECUTE after approval | Parallel mode §3 |
| Application Design | SKIP (dedicated stage) | Design is small and captured below; folded into the code generation plan |
| Units Generation | SKIP | Single unit |
| Functional Design | SKIP | Rules are fully specified in requirements (FR-3..FR-8) |
| NFR Requirements / NFR Design | SKIP | Extensions opted out; NFRs covered in requirements |
| Infrastructure Design | SKIP | No infra change |
| Code Generation | EXECUTE (plan, claims re-check, generate) | |
| Build and Test | EXECUTE | Parallel mode §9 gate: commit, rebase on main, fmt/clippy/test, curl on port 8081 |

## Design Sketch (to be detailed in the code generation plan)
Backend (new files first):
- `migrations/<UTC ts>_create_favourites.sql`
- `src/favourites.rs`: models (`Favourite`, `CreateFavouriteRequest`), repository fns, service fns, handlers with `#[utoipa::path]`, and `pub fn router() -> Router<SqlitePool>` (merge). Matching/latest-record logic reuses `repository::list_weather_records` semantics (Rust `to_lowercase`).
- `src/lib.rs`: add `pub mod favourites;`, add handlers/schemas to `ApiDoc`, add `.merge(favourites::router())` appended after existing routes.
Frontend:
- `static/js/favourites.js` (new): pure helpers (favourite key matching, section model) with DOM-free logic.
- `static/js/api.js`: add `getFavourites`, `addFavourite`, `removeFavourite` (only file allowed to call fetch).
- `static/js/state.js`: add `favourites` field (additive).
- `static/js/main.js`: load favourites at boot and on refresh; `onToggleFavourite` action; pass favourites to scene.
- `static/js/ui.js`: star button wiring in detail, Favourites section render (text nodes only).
- `static/js/globe-scene.js`: `favourite` class + star badge + aria-label on pins.
- `static/index.html`, `static/css/app.css`: additive markup/styles. `src/frontend.rs`: register `js/favourites.js`.
Tests: `tests/favourites.rs` (new), additions to `tests/frontend.rs`. Docs: README `### Favourites` subsection.

## Anticipated Claims (written after approval)
- route: GET /api/favourites; POST /api/favourites; DELETE /api/favourites/{city}
- table: favourites; migration: <timestamp>_create_favourites.sql
- files: src/favourites.rs (create), tests/favourites.rs (create), src/lib.rs, src/frontend.rs, tests/frontend.rs, README.md, static/index.html, static/css/app.css (edit)
- js-modules: static/js/favourites.js (create); api.js, state.js, main.js, ui.js, globe-scene.js (edit)

## Success Criteria
All FR/NFR/TR in requirements.md met; fmt, clippy `-D warnings` and full `cargo test` green after rebase on `main`; curl evidence recorded on port 8081.
