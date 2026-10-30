# Technical Constraints & Decisions

> Notes from Platform Team tech huddle — 2026-06-25
> Attendees: Marcus (Tech Lead), Priya (Senior Dev), Tom (DevOps)

---

## Agreed Constraints

### 1. No External API Integrations (for now)

We're NOT integrating with a third-party weather provider in this iteration.
Reasons:
- Adds a runtime dependency and failure mode
- Requires API key management and cost tracking
- Procurement/legal review would add 4–6 weeks

**Decision:** Seed forecast data in the database. This demonstrates the API
contract and lets consuming teams build against it. We can swap in a real
provider later without changing the interface.

### 2. Same Service, Same Database

The forecast feature lives in the existing Weather API service. We're not
spinning up a new microservice for this.

Reasons:
- The domain is the same (weather data for cities)
- Adding a service adds deployment/networking complexity
- The existing service handles current load comfortably

### 3. Follow Existing Patterns

Whatever we build must look like it was always part of the service:
- Same Rust module structure (`models`, `repository`, `service`, `handlers`)
- Same structured API error response (`timestamp`, `status`, `error`, `message`, `path`)
- Same API conventions (`/api/weather/...` prefix, JSON responses)
- Same Rust test conventions (unit tests and async HTTP integration tests)

### 4. No Authentication Required

The existing API is unauthenticated. We're not adding auth for this feature.
Auth is a separate initiative (tracked in PLATFORM-892).

### 5. SQLite Database

Use SQLite for local development and persistence. Tests should use an
in-memory SQLite database so they are isolated and do not modify local data.

---

## Open Questions (to resolve during design)

| # | Question | Options | Leaning |
|---|----------|---------|---------|
| 1 | Should forecast use a separate table or extend weather_records? | Separate table (cleaner) vs. add a `type` column | Separate table |
| 2 | Should city matching be case-insensitive? | Yes (better UX) vs. No (simpler) | Yes |
| 3 | What if a city exists in weather_records but has no forecast data? | 404 vs. empty array | 404 (consistent with existing behaviour) |
| 4 | How many days of seed data? | 5 (minimum) vs. 7 (gives buffer) | 5 (matches the API contract) |

---

## Non-Functional Requirements

- **Response time:** < 200ms (same as existing endpoints)
- **Availability:** Same as existing service (no additional SLA)
- **Data freshness:** N/A (seeded data, not real-time)
- **Breaking changes:** Zero. Existing endpoints must not change.
