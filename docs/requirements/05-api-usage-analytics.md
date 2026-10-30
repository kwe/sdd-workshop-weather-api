# API Usage Analytics — Weather API

> Report period: 2026-04-01 to 2026-06-30 (Q2)
> Source: API Gateway logs

---

## Current Endpoint Usage

| Endpoint | Method | Calls/day (avg) | Top Consumers |
|----------|--------|----------------|---------------|
| `/api/weather` | GET | 12,400 | Dashboard Team (45%), Partner APIs (35%), Mobile (20%) |
| `/api/weather?city={city}` | GET | 8,900 | Partner APIs (60%), Dashboard Team (30%), Mobile (10%) |
| `/api/weather/{id}` | GET | 2,100 | Internal tooling (80%), Ad-hoc (20%) |
| `/api/weather` | POST | 340 | Data Ingestion Service (100%) |
| `/api/weather/{id}` | DELETE | 12 | Admin tooling (100%) |

## Most Requested Cities

| Rank | City | % of Queries |
|------|------|-------------|
| 1 | London | 24% |
| 2 | New York | 18% |
| 3 | Tokyo | 12% |
| 4 | Sydney | 9% |
| 5 | Paris | 8% |
| 6 | Berlin | 7% |
| 7 | Mumbai | 6% |
| 8 | Toronto | 5% |
| 9 | São Paulo | 4% |
| 10 | Cape Town | 3% |
| — | Other | 4% |

## Failed Requests (404s)

| Pattern | Count/week | Likely Cause |
|---------|-----------|--------------|
| `/api/weather/forecast/*` | 230 | Teams attempting forecast endpoint that doesn't exist |
| `/api/weather?city=LONDON` | 45 | Case-sensitivity issue (uppercase) |
| `/api/weather?city=NYC` | 28 | Abbreviation not matching "New York" |

## Performance

| Metric | Value |
|--------|-------|
| p50 response time | 23ms |
| p95 response time | 89ms |
| p99 response time | 156ms |
| Error rate | 0.02% |
| Uptime (Q2) | 99.97% |

---

## Key Insight

> **230 requests/week are already hitting `/api/weather/forecast/*`** — teams
> are building against an endpoint that doesn't exist yet. This confirms
> demand and suggests the URL pattern `GET /api/weather/forecast/{city}` is
> already expected by consumers.
