# Business Logic Model: weather-alerts

Parallel-mode note: no clarifying questions were needed; every assumption is listed in `business-rules.md` under "Assumptions" and can be overridden at approval.

## Backend flow: `GET /api/alerts`

```text
handler get_alerts
  -> service::get_alerts(pool, today_utc)
       1. thresholds  = repo: SELECT enabled rows from alert_thresholds
       2. records     = repo: all weather_records (existing list query)
       3. forecasts   = repo: all forecasts rows (one query, ordered by city_key, day_offset)
       4. latest      = pick latest record per city key (see rules R-1)
       5. alerts      = evaluate(thresholds, latest, forecasts_by_key, today)   // pure
       6. sort(alerts) per R-8
  <- Json(Vec<Alert>)
```

Three queries regardless of city count. `evaluate` is pure (no DB, no clock) and is the main unit-test target.

### evaluate(thresholds, latest_records, forecasts_by_key, today)
For each latest record `r`:
1. `current`: for each threshold `t`, the matching metric is
   heat/cold -> `r.temperature_celsius`, wind -> `r.wind_speed_kmh`, humidity -> `r.humidity`.
   If the metric is NULL, skip. Otherwise `classify(t, value)`; when it returns a severity, emit an alert with `source = current`, `date = r.recorded_at[0..10]`.
2. `forecast`: look up rows by `lower(trim(city))`. For each row (day_offset 0-4, date = today + day_offset), heat -> `high_temperature_celsius`, cold -> `low_temperature_celsius`; same classify; emit with `source = forecast`. Wind and humidity thresholds are not evaluated for forecasts. No rows -> nothing emitted, no error.

### classify(t, value) -> Option<Severity>
- `gte`: severe if value >= t.severe, warning if value >= t.warning, watch if value >= t.threshold.
- `lte`: severe if value <= t.severe, warning if value <= t.warning, watch if value <= t.threshold.
- else None. Inclusive boundaries.

## Frontend flow

```text
boot -> refreshWeather(ui) + refreshAlerts(ui)          (same call sites)
setInterval(60s)  [existing]  -> refreshWeather + refreshAlerts   (no new timer)
visibilitychange  [existing]  -> refreshWeather + refreshAlerts
refreshAlerts: skip if in flight; getAlerts()
   ok   -> setState({ alerts, alertsStatus: 'ready' })
   fail -> setState({ alerts: [], alertsStatus: 'error' })
subscribe(state) -> ui renders banner + panel list; scene.setAlertRings(byCity)
```

- `alerts.js` pure helpers: `cityKeyOf(alert)` (uses gazetteer `normalize`/`normalizeCountry`, same key as `buildCities`), `groupByCity(alerts)` -> `Map<key, {worst, alerts[]}>`, `countBySeverity(alerts)`, `severityRank`.
- Ring layer: the globe has a single rings layer used for the selected-city pulse. `scene.setAlertRings` merges alert rings and the selection ring into one `ringsData` array, so both coexist. Each ring datum carries colour by severity; period/speed are constant.
- Fly-to: banner item click calls the existing `select(key)` path (same as clicking a pin), which flies and opens the detail panel.
- Reduced motion: when `matchMedia('(prefers-reduced-motion: reduce)').matches`, alert ring datum uses `ringPropagationSpeed 0` and `ringRepeatPeriod 0` semantics, rendered as a static ring (single full-opacity ring at fixed radius); CSS banner transitions disabled under `@media (prefers-reduced-motion: reduce)`. Re-evaluate on `change` event of the media query.
