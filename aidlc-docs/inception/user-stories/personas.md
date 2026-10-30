# Personas — 5-Day Forecast

## P1 — Internal Product Developer (Dashboard and Mobile teams)
- **Who**: Engineers on the Dashboard and Mobile teams (sponsor: Sarah Chen).
- **Goals**: Show "this week's weather" on city dashboards and a 5-day outlook in the mobile app; retire the scraping and hardcoded-placeholder workarounds.
- **Usage pattern**: One call per city on page/screen load (e.g. `GET /api/weather/forecast/London`), rendered as a list or horizontal scroll.
- **Needs**: Same URL style and error format as current endpoints, stable JSON shape, fast responses.
- **Stories**: S1, S2, S3, S4

## P2 — Partner Integration Developer
- **Who**: Developers at Acme Analytics (batch, logistics), TravelWise (trip-destination weather), GreenField (daily precipitation polling), WeatherBoard, CityPulse.
- **Goals**: Plan routes, trips, irrigation and events from daily highs/lows, conditions and rain chance.
- **Usage pattern**: Batch-calls for many cities and caches results; daily polling; user-typed city names (mixed case, multi-word).
- **Needs**: Forgiving city matching, clear 404s, predictable schema, honest documentation that data is seeded/static.
- **Stories**: S1, S2, S3, S4

## P3 — Platform / API Maintainer
- **Who**: Platform Team (Marcus, Priya, Tom).
- **Goals**: Ship the feature with zero regressions, follow existing patterns, keep it swappable for a real provider later.
- **Usage pattern**: Reviews code, runs CI (`fmt`, `clippy`, `test`), reads Swagger/README, operates the service.
- **Needs**: Layered code, validated seed data, tests, docs, latency within existing bounds.
- **Stories**: S5

## Persona-to-Story Map
| Story | P1 | P2 | P3 |
|---|---|---|---|
| S1 Retrieve 5-day forecast | X | X | |
| S2 Forgiving city lookup | X | X | |
| S3 Clear errors | X | X | |
| S4 Predictable integration | X | X | |
| S5 Maintainable, validated delivery | | | X |
