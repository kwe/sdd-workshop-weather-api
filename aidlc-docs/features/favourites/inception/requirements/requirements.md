# Requirements: Favourite Cities

## Intent Analysis
- **User request**: Let users star cities on the globe to return to them quickly; persist in SQLite; expose list/add/remove API; document in OpenAPI; globe UI star toggle + Favourites section; distinct favourite pins; keep XSS rules and CSP; add API and frontend tests.
- **Request type**: New feature (backend + frontend)
- **Scope**: Multiple components (migration, repository/service/handlers, OpenAPI, static JS/CSS/HTML, tests)
- **Complexity**: Moderate
- **Depth**: Standard
- **Extensions**: security-baseline, resiliency-baseline, property-based-testing all opted OUT (defaults accepted).

## Decisions (from answered questions)
Q1 A, Q2 A, Q3 A, Q4 A (+ user change: trim, reject empty or >100 chars), Q5 A, Q6 A, Q7 A, Q8 A, Q9 B (Rust-only tests, user override), Q10 all No.

## Functional Requirements

### Data
- FR-1: New migration `migrations/<UTC timestamp>_create_favourites.sql` creates table `favourites` (`id` PK, `city` display name TEXT NOT NULL, `city_key` TEXT NOT NULL UNIQUE = `city.to_lowercase()`, `created_at` TEXT NOT NULL). Existing migrations untouched. No seed rows.
- FR-2: Favourites are one global list (no users/auth). Stored server-side only.

### City matching
- FR-3: City names are trimmed and matched case-insensitively (Unicode-aware, via Rust `to_lowercase`, consistent with existing code). City only, not country.
- FR-4: The stored/displayed name is the canonical spelling of the latest weather record for that city.
- FR-5: "Latest weather" = record with greatest `recordedAt`, ties broken by greatest `id`.

### API
- FR-6: `GET /api/favourites` -> 200, array of `{city, favouritedAt, weather}` ordered newest favourite first (ties by id desc). `weather` is the latest `WeatherRecord` (camelCase) or `null` if no records remain. Empty list -> `[]`.
- FR-7: `POST /api/favourites` body `{"city": "..."}`:
  - 201 + favourite object when newly created.
  - 200 + existing favourite object when already a favourite (idempotent; no duplicate, `favouritedAt` unchanged).
  - 404 if no weather record matches the city (nothing stored).
  - 400 if body is not valid JSON, `city` missing/not a string, blank after trim, or longer than 100 characters after trim.
- FR-8: `DELETE /api/favourites/{city}` (URL-decoded, case-insensitive) -> 204 on removal; 404 if not a favourite; 400 if blank after trim.
- FR-9: Errors use the existing `ErrorResponse` JSON shape (timestamp/status/error/message/path).
- FR-10: All three operations documented in the OpenAPI spec (`utoipa` paths + schema `Favourite`, tag `favourites`) with all status codes above; visible in `/v3/api-docs` and Swagger UI.
- FR-11: New API routes do not carry the frontend CSP headers (existing behaviour for `/api/*` preserved).

### Globe UI
- FR-12: City detail panel has a star toggle button (`aria-pressed`, accessible label "Add <city> to favourites" / "Remove <city> from favourites", keyboard operable). Clicking calls POST/DELETE; UI state changes only after the API succeeds; failures are announced (live region) without breaking the panel.
- FR-13: A "Favourites" section sits above the city list in the sidebar, hidden when empty. Each item shows city name and temperature; clicking it selects the city, which flies the globe to it and opens the detail panel (same as picking from the list). The Favourites section is not affected by the search filter.
- FR-14: Favourite pins are visually distinct (gold ring + star badge) from normal pins and the selected state remains distinguishable; `aria-label` of favourite pins mentions "favourite".
- FR-15: Favourites are loaded on startup and refreshed with the existing 60 s weather refresh; if the favourites API is unavailable, the rest of the app keeps working. A favourite whose city is not in the loaded cities (or not on the globe) is still listed by name; selecting it is a no-op if the city cannot be resolved.
- FR-16: Frontend city key (`name|country`) maps to a favourite by normalized city name only.

## Non-Functional Requirements
- NFR-1 (XSS/CSP): All dynamic text via `el()`/`textContent`/text nodes; no `innerHTML`, inline scripts/handlers or `style` attribute writes; CSP constant unchanged; `fetch(` only in `api.js`; any new JS file registered in `frontend::ASSETS`.
- NFR-2 (Merge safety, parallel mode): new files preferred (`src/favourites.rs`, `static/js/favourites.js`, `tests/favourites.rs`); edits to shared files minimal and additive; route added at end of the router chain; no reformatting; README gets a new `###` subsection only.
- NFR-3 (Quality): `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, full `cargo test` pass.
- NFR-4 (Runtime isolation): run on port 8081 with a worktree-local or `/tmp/favourites/` database.
- NFR-5 (Accessibility): star toggle and favourites items focusable, labelled, visible focus; reduced-motion respected for any animation.

## Testing Requirements
- TR-1: `tests/favourites.rs` (in-memory SQLite, existing `tests/api.rs` style): empty list; create 201; repeat 200 with no duplicate; case/whitespace-insensitive duplicate; 404 unknown city (and nothing stored); 400 for missing/blank/non-string/>100 chars/malformed JSON; list ordering newest first with latest weather chosen by recordedAt then id; `weather: null` after records deleted; DELETE 204 then 404; DELETE case-insensitive and URL-encoded (e.g. `S%C3%A3o%20Paulo`); OpenAPI doc contains the paths.
- TR-2: `tests/frontend.rs` additions only (no Node): `/static/js/favourites.js` served with JS type + CSP; existing forbidden-token scan and fetch-only-in-api.js scan cover the new file; index.html contains favourites section/star toggle ids; CSS contains favourite pin selector; CSP constant unchanged.

## Out of Scope
Per-user favourites, auth, reordering, localStorage persistence, favourites count limits, schema/behaviour changes to weather or forecast endpoints.
