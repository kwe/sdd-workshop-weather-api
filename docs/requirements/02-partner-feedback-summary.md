# Partner Feedback Summary — Q2 2026

> Extracted from Zendesk tickets and partner call notes by the Partner Success team.

---

## Forecast-Related Requests (12 tickets, 5 partners)

### Acme Analytics (Enterprise tier)
> "We're building a logistics planning dashboard for our customers. Weather
> forecasts are critical for route planning — we need at least 5 days ahead.
> Currently we're using a third-party service which adds latency and cost.
> If SkyCast offered this natively we'd switch immediately."
>
> — Ticket #4891, 2026-05-12

### TravelWise App (Growth tier)
> "Users keep asking us 'what's the weather going to be on my trip?' and we
> can't answer it. Our competitors show 5-day forecasts. We're losing users
> over this."
>
> — Ticket #5023, 2026-06-01

### GreenField Agriculture (Enterprise tier)
> "Precipitation forecasts are essential for our irrigation scheduling feature.
> We need daily precipitation probability for the next 5–7 days. Accuracy
> matters less than availability — even ballpark figures help farmers plan."
>
> — Ticket #5104, 2026-06-08

### WeatherBoard (Startup tier)
> "Your current API is great for 'right now' but useless for 'what's coming'.
> We need forecasts. Even basic ones. Honestly anything is better than nothing."
>
> — Ticket #5156, 2026-06-14

### CityPulse Events (Growth tier)
> "Event organisers need to know if it's going to rain on their outdoor event
> this weekend. A simple 5-day forecast with conditions + precipitation chance
> would solve 80% of their use case."
>
> — Ticket #5201, 2026-06-19

---

## Summary

| Metric | Value |
|--------|-------|
| Total forecast-related tickets (Q2) | 12 |
| Unique partners requesting | 5 of 23 active partners (22%) |
| Stated willingness to pay more | 2 (Acme Analytics, GreenField) |
| Competitor feature gap mentioned | 3 times |
| Churn risk cited | 1 (TravelWise) |

## Common Requirements Across Partners

1. **5-day minimum** forecast window (7-day preferred but 5 is acceptable)
2. **Daily granularity** (not hourly)
3. **Temperature range** (high/low per day)
4. **Conditions** (text description — sunny, cloudy, rainy, etc.)
5. **Precipitation probability** (percentage)
6. **Same API style** as existing endpoints (JSON, RESTful, standard errors)
