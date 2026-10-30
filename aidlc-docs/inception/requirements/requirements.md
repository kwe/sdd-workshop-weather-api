# Requirements — 5-Day Weather Forecast API

## Intent Analysis
- **User request**: Add a 5-day weather forecast feature to the Weather API.
- **Request type**: New Feature
- **Scope**: Single component (existing crate)
- **Complexity**: Simple-Moderate
- **Depth**: Standard
- **Sources**: docs/requirements/01-05, requirement-verification-questions.md (all answered, no contradictions)

## Functional Requirements
- **FR-1** `GET /api/weather/forecast/{city}` returns HTTP 200 with a bare JSON array of exactly 5 daily forecasts, ordered by date ascending.
- **FR-2** Each day object (camelCase): `date` (`YYYY-MM-DD`), `highTemperatureCelsius` (number), `lowTemperatureCelsius` (number), `conditions` (string), `precipitationChance` (integer 0-100).
- **FR-3** Dates are rolling: seed stores day offsets 0-4; `date` = today's UTC date + offset, computed at request time.
- **FR-4** City matching is case-insensitive, Unicode-aware and trimmed (same behaviour as the existing `?city=` filter). Percent-encoded names (e.g. `New%20York`) are decoded.
- **FR-5** A blank/whitespace-only city returns 400 "City is required".
- **FR-6** An unknown city, or a known city with no forecast data, returns 404 with the standard error body and a message such as "No forecast found for city {city}".
- **FR-7** Forecast data is stored in a new `forecasts` table created by migration `0002` (migration 0001 untouched), seeded with 5 days for each of the 10 existing cities (London, New York, Tokyo, Sydney, Paris, Berlin, Mumbai, São Paulo, Cape Town, Toronto).
- **FR-8** The table has CHECK constraints: high >= low, precipitation between 0 and 100, day offset between 0 and 4.
- **FR-9** Read-only: no POST/DELETE for forecasts. No city aliases (e.g. NYC). No external provider.
- **FR-10** Endpoint is documented in OpenAPI/Swagger (path and new schemas registered in `ApiDoc`) and in the README (endpoint table, seed data, project structure, note that data is static/seeded).

## Non-Functional Requirements
- **NFR-1 Performance**: < 200 ms p95 (same as existing endpoints); a single indexed query per request.
- **NFR-2 Compatibility**: zero changes to existing endpoints, schema or responses; all existing tests must pass.
- **NFR-3 Consistency**: follows `models`/`repository`/`service`/`handlers` layering; reuses `ApiError`/`ErrorResponse` (`timestamp,status,error,message,path`); `/api/weather/...` prefix; JSON.
- **NFR-4 Testing**: unit tests plus async HTTP integration tests on in-memory SQLite (including a seed-invariant test covering all 10 cities x 5 days, case-insensitivity, encoded names, 400, 404, and OpenAPI inclusion).
- **NFR-5 Quality gates**: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` pass.
- **NFR-6 Auth/Caching**: none (out of scope; auth tracked in PLATFORM-892).

## Out of Scope
Hourly or >5-day forecasts, historical accuracy, real provider integration, authentication, caching/rate limiting, UI changes, city aliases, forecast write endpoints.

## Extension Configuration
- Security baseline: **No** (skipped)
- Resiliency baseline: **No** (skipped)
- Property-based testing: **No** (skipped)

## Key Decisions Resolved (from 03-technical-constraints.md open questions)
1. Separate `forecasts` table. 2. Case-insensitive matching. 3. No forecast data -> 404. 4. 5 days seeded (rolling offsets 0-4).

## Risks
- Partners may expect real-time data: mitigate via README/OpenAPI note that data is seeded and static.
- Rolling dates make seed values look "fresh" though static; documented in the same note.
