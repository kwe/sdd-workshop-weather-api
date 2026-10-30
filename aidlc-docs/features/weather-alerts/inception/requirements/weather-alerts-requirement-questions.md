# Weather Alerts - Requirements Clarification Questions

Depth: Standard. Parallel Feature Mode: every question has a pre-filled `(default, assumed)` answer. Edit only what you disagree with, then tell me to continue.

## Question 1
Which forecast fields are evaluated? (Forecast has only high/low temperature, conditions, precipitation - no wind/humidity.)

A) Heat uses forecast high; cold uses forecast low; wind/humidity are evaluated for current only

B) Heat and cold both use daily high only

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 2
Severity levels and how they derive from a threshold breach?

A) Three levels (`watch`, `warning`, `severe`): breach by <10% of margin = watch, <25% = warning, otherwise severe, where margin = max(|threshold|, 10) for cold/heat/wind/humidity scaling; computed in code, not stored

B) Each threshold row stores its own warning and severe cut-offs in extra columns

C) Single severity `warning` for everything

X) Other (please describe after [Answer]: tag below)

[Answer]: B (default, assumed) - table columns: `alert_type`, `comparison` (gte/lte), `threshold`, `unit`, `warning_threshold`, `severe_threshold`, `enabled`; the specified defaults (30C/0C/40 km/h/85%) are the lowest level (`watch`), with seeded warning/severe values of heat 35/40, cold -5/-15, wind 60/90, humidity 90/95.

## Question 3
Are thresholds editable through the API in this feature?

A) No - read-only in the DB (changed via SQL/migrations); only `GET /api/alerts` is added

B) Yes - add `GET/PUT /api/alerts/thresholds`

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 4
Alert response shape and ordering?

A) `{ "alerts": [...], "count": N }`... fields per alert: `city`, `country`, `type` (`heat|cold|wind|humidity`), `severity`, `value`, `threshold`, `source` (`current|forecast`), `date` (current = date of the record's `recordedAt`; forecast = UTC calendar date of that forecast day). Sorted by severity desc, then city, then date

B) Bare JSON array of alerts, same fields and order

X) Other (please describe after [Answer]: tag below)

[Answer]: B (default, assumed) - bare array matches the product owner's "returns a list" and the existing list endpoints.

## Question 5
Forecast alerts: one alert per breaching day, or collapsed to the worst day per city and type?

A) One alert per breaching forecast day

B) Worst day only per city + type + source

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 6
City matching between `weather_records` and `forecasts`, and cities without a forecast?

A) Case-insensitive on trimmed city (same as the forecast endpoint); city with no forecast still gets current alerts, no error

B) Skip cities lacking a forecast

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 7
Globe UI behaviour for banner and ring colours?

A) Banner = compact pill in the existing HUD showing total count plus per-severity counts, expands to a list; click an item flies to that city and opens its panel. Ring colours: watch = yellow, warning = orange, severe = red (also distinguishable by shape/label, not colour only). City with several alerts uses its highest severity. Ring pulses via CSS/`requestAnimationFrame`; under `prefers-reduced-motion` the ring is static (no pulse), fly-to uses the existing reduced-motion behaviour

B) Banner is always-expanded list

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 8
Should the UI refresh alerts periodically?

A) Load once at startup with the weather data; re-fetch when the page is reloaded (no polling)

B) Poll every 60 seconds

X) Other (please describe after [Answer]: tag below)

[Answer]: B - user override: refresh alerts on the same 60-second cycle as the existing weather refresh (reuse that timer in `main.js`, no separate poller).

## Question 9
Failure behaviour if `/api/alerts` fails in the UI?

A) Globe and existing features keep working; banner shows a small "Alerts unavailable" text; no rings

B) Show a blocking error

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)

## Question 10
Extension opt-ins (all previously opted out in earlier cycles). Security-relevant constraints (textContent only, CSP unchanged) are explicit requirements regardless.

A) Security baseline: No. Resiliency baseline: No. Property-based testing: No

B) Enable one or more (name them after [Answer]:)

X) Other (please describe after [Answer]: tag below)

[Answer]: A (default, assumed)
