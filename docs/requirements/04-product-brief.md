# Product Brief — 5-Day Weather Forecast API

> **Author:** James Hargreaves, Head of Product
> **Date:** 2026-06-30
> **Status:** Approved for delivery

---

## Problem

Internal product teams and external partners need weather forecast data to
build planning features (logistics, travel, events, agriculture). The
Weather API currently only serves point-in-time observations — "what is the
weather right now?" — but cannot answer "what will the weather be like this
week?"

Without this capability:
- 3 internal teams maintain their own workarounds (inconsistent, unmaintained)
- Partners are citing this as a competitive gap
- 1 partner has flagged churn risk explicitly

## Opportunity

Adding a 5-day forecast endpoint to the existing Weather API would:
- Eliminate 3 separate internal workarounds
- Unblock partner features for 5 requesting partners
- Close a competitive gap flagged in 3 partner conversations
- Establish the API contract that a future real-time provider can fulfil

## Target Users

| User | Need | How They'll Use It |
|------|------|-------------------|
| Internal Dashboard Team | Show "this week's weather" in city dashboards | `GET /api/weather/forecast/London` on page load |
| Internal Mobile Team | Display 5-day outlook in the mobile app | Same endpoint, rendered in a horizontal scroll |
| Partner: Acme Analytics | Logistics route planning based on weather | Batch-call for multiple cities, cache results |
| Partner: TravelWise | "Weather at your destination" feature | Call on trip detail page for destination city |
| Partner: GreenField | Irrigation scheduling based on precipitation | Daily poll for agricultural regions |

## Scope

### In Scope
- New REST endpoint: `GET /api/weather/forecast/{city}`
- Response: array of 5 daily forecasts
- Each day: date, high temp (°C), low temp (°C), conditions, precipitation %
- Seeded data for all cities already in the system
- Swagger/OpenAPI documentation
- Unit and integration tests
- Standard error handling (404 for unknown city)

### Out of Scope
- Real-time data from external providers (future phase)
- Hourly forecasts
- Authentication/authorisation
- Caching/rate limiting
- UI/frontend changes
- Historical forecast accuracy tracking

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Internal team adoption | 3/3 teams migrated from workarounds within 2 sprints | Usage logs |
| Partner adoption | ≥3 partners calling the endpoint within 4 weeks of release | API analytics |
| Zero breaking changes | 0 regressions on existing endpoints | Test suite + monitoring |
| API response time | < 200ms p95 | APM dashboard |

## Timeline

- **Sprint 1:** Design + implement core feature
- **Sprint 2:** Partner documentation + migration support

## Dependencies

- None (self-contained within Weather API service)

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Partners expect real-time data, not seeded | Medium | Medium | Clear documentation that this is static data; real-time integration is Phase 2 |
| Scope creep to hourly/7-day/14-day | High | Low | Scope Brief explicitly limits to 5-day daily |
| Existing tests break | Low | High | CI pipeline catches; no changes to existing code paths |
