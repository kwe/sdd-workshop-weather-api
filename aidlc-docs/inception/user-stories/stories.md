# User Stories — 5-Day Forecast

Approach: hybrid (persona voice, shared feature-level acceptance criteria). Format: Given/When/Then. Priority: MoSCoW. Examples assume "today" is 2026-10-30 (UTC).

## S1 — Retrieve a 5-day forecast for a city
**As** an internal product developer (P1) or partner integration developer (P2), **I want** `GET /api/weather/forecast/{city}` to return daily predictions for the next 5 days, **so that** I can show planning data without scraping or placeholders.
- **Priority**: Must
- **Covers**: FR-1, FR-2, FR-3, FR-7 (data availability)
- **Personas**: P1, P2

**Acceptance criteria**
- Given the seeded database, when I call `GET /api/weather/forecast/London`, then I receive HTTP 200 and a JSON array of exactly 5 objects.
- Given the response, then entries are ordered by `date` ascending, starting with today's UTC date and incrementing by one day (rolling dates).
- Given any entry, then it contains `date` (`YYYY-MM-DD`), `highTemperatureCelsius`, `lowTemperatureCelsius`, `conditions` (non-empty string) and `precipitationChance` (integer 0-100), in camelCase.
- Given each of the 10 seeded cities (London, New York, Tokyo, Sydney, Paris, Berlin, Mumbai, São Paulo, Cape Town, Toronto), when I request its forecast, then I receive 5 entries.
- Given the same request is made on two different days, then `date` values shift with the day while the seeded values are unchanged.

**Example 200 (abbreviated to 2 of 5 entries)**
```json
[
  { "date": "2026-10-30", "highTemperatureCelsius": 19.0, "lowTemperatureCelsius": 11.0, "conditions": "Cloudy", "precipitationChance": 40 },
  { "date": "2026-10-31", "highTemperatureCelsius": 17.5, "lowTemperatureCelsius": 10.0, "conditions": "Rain", "precipitationChance": 80 }
]
```

## S2 — Forgiving city lookup
**As** a partner integration developer (P2) passing user-entered names, **I want** city matching to ignore case, surrounding whitespace and URL encoding, **so that** `LONDON`, `new%20york` and `são paulo` work without client-side normalisation.
- **Priority**: Must
- **Covers**: FR-4, FR-5
- **Personas**: P1, P2

**Acceptance criteria**
- Given `GET /api/weather/forecast/LONDON` (or `london`), when called, then the response equals the response for `London`.
- Given `GET /api/weather/forecast/New%20York`, when called, then I receive New York's 5 entries.
- Given `GET /api/weather/forecast/s%C3%A3o%20paulo`, when called, then I receive São Paulo's 5 entries (Unicode-aware).
- Given a city segment with surrounding spaces (`%20London%20`), when called, then it matches London.
- Given a blank/whitespace-only city (`/api/weather/forecast/%20`), when called, then I receive 400 with message "City is required" in the standard error body.

## S3 — Clear errors for missing forecasts
**As** a developer (P1 or P2), **I want** an unambiguous 404 in the standard error format when no forecast exists, **so that** my error handling is identical to the existing endpoints.
- **Priority**: Must
- **Covers**: FR-6, NFR-3
- **Personas**: P1, P2

**Acceptance criteria**
- Given a city not in the data (e.g. `Atlantis`), when called, then I receive 404.
- Given a city present in `weather_records` but without forecast rows, when called, then I also receive 404 (not an empty array).
- Given any error, then the body contains `timestamp`, `status`, `error`, `message`, `path`, with `path` = the request path (e.g. `/api/weather/forecast/Atlantis`) and `message` like "No forecast found for city Atlantis".

**Example 404**
```json
{ "timestamp": "2026-10-30T11:14:00", "status": 404, "error": "Not Found", "message": "No forecast found for city Atlantis", "path": "/api/weather/forecast/Atlantis" }
```

## S4 — Predictable, read-only integration
**As** a partner or internal developer (P1/P2), **I want** the endpoint to follow the conventions of the existing API and be read-only, **so that** I can integrate and cache without learning a new pattern or risking data changes.
- **Priority**: Should
- **Covers**: FR-9, NFR-3, NFR-6
- **Personas**: P1, P2

**Acceptance criteria**
- Given the `/api/weather/` prefix, JSON responses and camelCase fields, when I compare with existing endpoints, then conventions match.
- Given `POST`, `PUT` or `DELETE` on `/api/weather/forecast/London`, when called, then no forecast data is created or deleted (405 Method Not Allowed).
- Given no credentials, when I call the endpoint, then it responds normally (no authentication).
- Given aliases such as `NYC`, when called, then 404 (aliases are out of scope).

## S5 — Maintainable, validated delivery
**As** a platform maintainer (P3), **I want** the feature built with the existing layering, validated seed data, tests and docs, **so that** it ships with zero regressions and can later swap to a real provider without an interface change.
- **Priority**: Must
- **Covers**: FR-7, FR-8, FR-10, NFR-1, NFR-2, NFR-4, NFR-5
- **Personas**: P3

**Acceptance criteria**
- Given migration `0002`, when applied on a fresh and on an existing database, then the `forecasts` table is created with 5 seeded rows for each of the 10 cities and `0001` is unchanged.
- Given the table, when inserting a row with high < low, precipitation outside 0-100, or day offset outside 0-4, then the database rejects it (CHECK constraints).
- Given the test suite, when run, then all pre-existing tests pass unmodified and new unit and HTTP integration tests (in-memory SQLite) cover S1-S4, including a seed-invariant test over all 10 cities x 5 days.
- Given `/v3/api-docs`, when fetched, then it includes `/api/weather/forecast/{city}` and the forecast schema; and the README lists the endpoint, seed data, structure and a note that data is static/seeded.
- Given `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`, then all succeed.
- Given a local load of the endpoint, then response time stays under 200 ms p95 (single query per request).

## INVEST Check
All stories are independent enough to test separately (S2-S4 build on the S1 route), negotiable in detail, valuable to a named persona, estimable (small), and testable through the criteria above.

## Requirement Coverage
| Requirement | Stories |
|---|---|
| FR-1, FR-2, FR-3 | S1 |
| FR-4, FR-5 | S2 |
| FR-6 | S3 |
| FR-7 | S1, S5 |
| FR-8 | S5 |
| FR-9 | S4 |
| FR-10 | S5 |
| NFR-1, NFR-2, NFR-4, NFR-5 | S5 |
| NFR-3 | S3, S4 |
| NFR-6 | S4 |
