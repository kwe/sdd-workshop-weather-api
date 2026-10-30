# Domain Entities: weather-alerts

## Table `alert_thresholds` (new)
| Column | Type | Notes |
|---|---|---|
| alert_type | TEXT PRIMARY KEY | `heat`, `cold`, `wind`, `humidity` |
| comparison | TEXT NOT NULL | `gte` or `lte` |
| threshold | REAL NOT NULL | watch level (spec defaults) |
| warning_threshold | REAL NOT NULL | |
| severe_threshold | REAL NOT NULL | |
| unit | TEXT NOT NULL | `C`, `km/h`, `%` (informational) |
| enabled | INTEGER NOT NULL DEFAULT 1 | 0/1 |

Seeds: heat gte 30/35/40 C; cold lte 0/-5/-15 C; wind gte 40/60/90 km/h; humidity gte 85/90/95 %.

## Rust types (`src/alerts.rs`)
- `AlertType` enum (Heat, Cold, Wind, Humidity), `Severity` enum (Watch < Warning < Severe, `Ord`), `AlertSource` enum (Current, Forecast); all serde lowercase + `ToSchema`.
- `Threshold` (FromRow): mirrors the table.
- `Alert` (Serialize, ToSchema, camelCase): `city: String`, `country: Option<String>`, `type: AlertType` (field `alert_type` renamed to `type`), `severity: Severity`, `value: f64`, `threshold: f64`, `source: AlertSource`, `date: String`.

## Example response
```json
[
  {"city":"Mumbai","country":"India","type":"heat","severity":"watch","value":31.7,"threshold":30.0,"source":"current","date":"2026-07-17"}
]
```

## Frontend state additions (`state.js`)
- `alerts: []` (Alert objects as returned), `alertsStatus: 'loading' | 'ready' | 'error'`.
- Derived (not stored): per-city map from `groupByCity(alerts)`.
