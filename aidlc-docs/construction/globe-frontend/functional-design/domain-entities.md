# Domain Entities — globe-frontend

## Server-side (unchanged API contracts)
```text
WeatherRecord  { id: int, city: string, country: string|null, temperatureCelsius: number|null,
                 humidity: int|null, description: string|null, windSpeedKmh: number|null,
                 recordedAt: string  // naive "YYYY-MM-DDTHH:MM:SS", no zone }
ForecastDay    { date: "YYYY-MM-DD" (UTC), highTemperatureCelsius: number, lowTemperatureCelsius: number,
                 conditions: string, precipitationChance: int 0-100 }
ErrorResponse  { timestamp, status, error, message, path }   // only status is used by the frontend
```
All API fields are **untrusted text/numbers** (cities can be created by anyone through `POST /api/weather`).

## Frontend entities

### GazetteerEntry (static data)
`{ name: string, country: string, lat: number (-90..90), lng: number (-180..180) }`. Ordered by prominence. Index: `Map<normalizedName, GazetteerEntry[]>`.

### City (view model)
```text
key        string   normalize(name) + "|" + normalize(country ?? "")
name       string   sanitised display name (<= 60 chars)
country    string|null
record     WeatherRecord   // latest by recordedAt for this key
lat, lng   number|null     // null when not in gazetteer
onGlobe    boolean         // lat/lng present
kind       IconKind        // conditionKind(record.description)
color      string          // tempColor(record.temperatureCelsius)
isDay      boolean|null    // sun elevation at city > 0 (null if not on globe)
```

### Forecast entry (per city key)
`{ state: 'loading'|'ready'|'none'|'error', days: ForecastDay[], error?: string }`. `none` = HTTP 404.

### Status
`{ phase: 'loading'|'ready'|'error', stale: boolean, lastUpdated: Date|null, message: string|null }`

### UI state
`{ selectedKey: string|null, unit: 'C'|'F', query: string, drawerOpen: boolean (mobile), hoverKey: string|null }`.
`unit` persisted in `localStorage['weather.unit']` (validated on read; fall back to 'C').

### Scene entities
- `SunState { lat, lng, vector: [x,y,z] }` (sub-solar point and unit vector in globe world coordinates).
- `Pin { city, element: HTMLElement }`, `Ring { lat, lng, color, maxR, speed, period }`.
- `CameraPose { lat, lng, altitude }`.

### Icon kinds
`sun | moon | partly | cloud | rain | storm | snow | wind | fog | humid | unknown` (moon is the night variant of the `clear` rule). Refinement of Application Design: `conditionKind(text, isDay)` takes the day flag.

## Relationships
`WeatherRecord (many) -> City (one per key, latest record)`; `City 0..1 -> GazetteerEntry`; `City 1 -> ForecastEntry (0..1, lazy)`; `City 0..1 -> Pin`.
