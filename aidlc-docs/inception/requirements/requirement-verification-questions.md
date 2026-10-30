# Requirements Verification Questions

Request: add `GET /api/weather/forecast/{city}` (5-day forecast) to the Weather API.
Intent: New Feature | Scope: Single component | Complexity: Simple-Moderate | Depth: Standard.
Most of the scope is settled by docs/requirements/. The questions below close the remaining gaps (including the 4 open questions in 03-technical-constraints.md).

Please fill in the letter after each [Answer]: tag.

## Question 1
Seeded forecast dates: how should the `date` field be produced?

A) Fixed calendar dates stored in the seed (simple, but go stale as time passes)

B) Rolling dates: store day offsets (0-4) and compute `date` from today's UTC date at request time (always shows "next 5 days")

C) Fixed dates, starting from the existing seed observation date (2026-07-17)

D) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 2
Response body shape (brief says "array of 5 daily forecasts").

A) Bare JSON array of day objects (consistent with `GET /api/weather` returning a bare array)

B) Wrapper object, e.g. `{ "city": "London", "country": "UK", "forecast": [ ... ] }`

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Field names for each day (camelCase like existing API).

A) `date`, `highTemperatureCelsius`, `lowTemperatureCelsius`, `conditions`, `precipitationChance`

B) `date`, `highCelsius`, `lowCelsius`, `description`, `precipitationPercent` (closer to existing `description` field)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
Precipitation chance representation.

A) Integer 0-100 (percent)

B) Decimal 0.0-1.0

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
City matching (constraints doc Q2). Existing `?city=` filter is case-insensitive and Unicode-aware.

A) Case-insensitive, Unicode-aware, trimmed (same as existing behavior; `LONDON`, `são paulo` work)

B) Exact match only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 6
City exists in `weather_records` but has no forecast, or city is unknown (constraints doc Q3).

A) 404 with standard error body for both cases (message distinguishes e.g. "No forecast found for city X")

B) 404 for unknown city, 200 with empty array when city is known but has no forecast

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7
URL-encoded multi-word cities (e.g. `/api/weather/forecast/New%20York`) and a blank/whitespace city segment.

A) Decode (axum does this), trim, and treat blank as 400 "City is required"

B) Decode and trim; blank simply results in 404

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8
Forecast data storage and seeding (constraints doc Q1, Q4).

A) New `forecasts` table in migration `0002`, 5 days seeded for each of the 10 existing cities; migration 0001 untouched

B) 7 days seeded per city, API returns only the first 5

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 9
Write access for forecasts (POST/DELETE).

A) Read-only: no create/delete forecast endpoints this iteration (matches brief scope)

B) Include POST/DELETE for forecasts

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 10
Validation of seeded data invariants (high >= low, precipitation 0-100).

A) Enforce with SQL CHECK constraints in the migration, plus a test that verifies the seed

B) Test-only verification, no CHECK constraints

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 11
Documentation: the brief requires Swagger/OpenAPI. Also update README (endpoint table, seed data, project structure)?

A) Yes: OpenAPI registration plus README updates

B) OpenAPI only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 12
City abbreviations / aliases (e.g. `NYC` -> New York appears in 404 analytics).

A) Out of scope (no aliases)

B) In scope: add a small alias mapping

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question: Security Extensions
Should security extension rules be enforced for this project?

A) Yes — enforce all SECURITY rules as blocking constraints (recommended for production-grade applications)

B) No — skip all SECURITY rules (suitable for PoCs, prototypes, and experimental projects)

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question: Resiliency Extensions
Should the resiliency baseline be applied to this project?
(Directional, design-time best practices from the AWS Well-Architected Reliability Pillar. It is not a production-readiness certification.)

A) Yes — apply the resiliency baseline as directional best practices and design-time guidance

B) No — skip the resiliency baseline (suitable for PoCs, prototypes, and experimental projects)

X) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question: Property-Based Testing Extension
Should property-based testing (PBT) rules be enforced for this project?

A) Yes — enforce all PBT rules as blocking constraints

B) Partial — enforce PBT rules only for pure functions and serialization round-trips

C) No — skip all PBT rules (suitable for simple CRUD applications or thin integration layers)

X) Other (please describe after [Answer]: tag below)

[Answer]: C
