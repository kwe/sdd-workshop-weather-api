# Business Rules: weather-alerts

| ID | Rule |
|---|---|
| R-1 | Latest record per city: group `weather_records` by `lower(trim(city))` (Rust `to_lowercase`); latest = greatest `recorded_at` string, tie greatest `id` (same as the frontend `buildCities`). The alert reports that record's `city` and `country`. |
| R-2 | Current alerts use only that latest record. NULL metric -> no alert for that metric. |
| R-3 | Forecast alerts: heat from forecast `high`, cold from forecast `low`. No wind/humidity forecast alerts. One alert per breaching day. |
| R-4 | Severity tiers per threshold row: `watch` at the base threshold, `warning` at `warning_threshold`, `severe` at `severe_threshold`; inclusive; highest tier reached wins. |
| R-5 | Disabled rows (`enabled = 0`) are ignored. A missing type simply yields no alerts for that type. |
| R-6 | Alert `threshold` = the row's base threshold (watch level), regardless of severity. `value` = measured/forecast value (humidity cast to f64). |
| R-7 | `date`: current = first 10 chars of `recorded_at` (`YYYY-MM-DD`); forecast = `today(UTC) + day_offset`. |
| R-8 | Order: severity (severe, warning, watch), city (Unicode-lowercase, ascending), date ascending, source (`current` before `forecast`), type (heat, cold, wind, humidity). |
| R-9 | Cities with no forecast rows still receive current alerts. An incomplete forecast is NOT an error here (unlike `/forecast/{city}`). |
| R-10 | DB errors -> existing `ApiError::internal` JSON 500 with path `/api/alerts`; details are logged, not returned. |
| R-11 | Table constraints: `alert_type IN ('heat','cold','wind','humidity')`, `comparison IN ('gte','lte')`, `enabled IN (0,1)`; for `gte` rows `threshold <= warning_threshold <= severe_threshold`, for `lte` rows `threshold >= warning_threshold >= severe_threshold`. |
| R-12 | UI city highest severity = max rank among its alerts; ring colour: watch #ffd400, warning #ff8c00, severe #ff2d2d. Severity is always also shown as text. |
| R-13 | UI banner: count = total alerts; per-severity counts; items sorted as the API orders them; empty -> "No active alerts"; error -> "Alerts unavailable". |
| R-14 | UI text from the API is written only via `textContent`/`createElement`; `type` and `severity` are mapped through fixed label tables, unknown values fall back to a neutral label. |
| R-15 | A banner item for a city that is not on the globe (no coordinates) still selects the city/opens the detail panel via the existing select path, with no ring. |

## Assumptions (defaults, assumed)
- A1: Seeded cut-offs: heat 30/35/40, cold 0/-5/-15, wind 40/60/90, humidity 85/90/95.
- A2: Severity keys are lowercase strings `watch|warning|severe`; types `heat|cold|wind|humidity`; source `current|forecast`.
- A3: Alert dates for current readings come from the stored timestamp without timezone conversion.
- A4: Expected result on the seed data (all `watch`): Tokyo heat current (30.3) + forecast days 0, 1, 4 (30.5, 31.0, 30.0); Mumbai heat current (31.7) + all 5 forecast days. 10 alerts total (corrected from an initial miscount of 12); no cold, wind or humidity alerts (max wind 22.4, max humidity 82, min forecast low 7.5).
