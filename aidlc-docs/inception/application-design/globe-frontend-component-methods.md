# Component Methods — Globe Frontend (Cycle 2)

Business rules (colour scale values, shader math, matching rules, state machine) are detailed in Functional Design.

## Rust

```rust
// src/frontend.rs
pub fn router() -> Router<SqlitePool>;               // routes "/" and "/static/{*path}" + header layer
async fn index() -> Response;                        // 200 text/html; charset=utf-8
async fn static_asset(Path(path): Path<String>) -> Response; // 200 with content type, or 404
fn find_asset(path: &str) -> Option<&'static Asset>; // exact table lookup, so no path traversal is possible
async fn frontend_headers(response: Response) -> Response;   // CSP, Cache-Control, nosniff, Referrer-Policy (map_response)
struct Asset { path: &'static str, content_type: &'static str, body: &'static [u8] }
pub const CONTENT_SECURITY_POLICY: &str;             // single source of truth, asserted by tests
```

`lib.rs`: `Router::new()...merge(frontend::router())` before `.with_state(pool)`.

## JavaScript data shapes
```text
WeatherRecord  (API)  { id, city, country|null, temperatureCelsius|null, humidity|null,
                        description|null, windSpeedKmh|null, recordedAt }
ForecastDay    (API)  { date, highTemperatureCelsius, lowTemperatureCelsius, conditions, precipitationChance }
City           (view) { key, name, country, record: WeatherRecord, lat|null, lng|null, onGlobe: boolean }
Status         (view) { phase: 'loading'|'ready'|'error', stale: boolean, lastUpdated: Date|null, message|null }
ForecastEntry  (view) { state: 'loading'|'ready'|'none'|'error', days: ForecastDay[] }
```

## Module exports
```text
state.js        createStore(initial) -> { get(), set(patch), subscribe(fn) -> unsubscribe }
                store: { cities, selectedKey, unit('C'|'F'), forecasts{key->ForecastEntry}, status, query }
api.js          getWeather({signal}) -> Promise<WeatherRecord[]>        // GET /api/weather
                getForecast(city, {signal}) -> Promise<ForecastDay[]>   // GET /api/weather/forecast/{encodeURIComponent(city)}; 404 -> ApiNotFound
                class ApiError { status, message }  class ApiNotFound extends ApiError
gazetteer.js    CITIES: {name,country,lat,lng}[]
                normalize(str) -> string
                locate(record) -> {lat,lng}|null
weather.js      tempColor(celsius|null) -> '#rrggbb'
                conditionKind(text|null) -> 'sun'|'clear-night'|'partly'|'cloud'|'rain'|'storm'|'snow'|'wind'|'fog'|'humid'|'unknown'
                convertTemp(celsius, unit) -> number ; formatTemp(celsius, unit) -> string ; formatWind(kmh) -> string
                formatRecordedAt(str) -> string
sun.js          sunDirection(date: Date) -> { lat, lng, vector: [x,y,z] }
globe-scene.js  createGlobe(container, {onSelect, onHover, reducedMotion}) ->
                  { setCities(cities), flyTo(city), resetView(), setUnit(u), pauseAutoRotate(), dispose() }
ui.js           mountUI(root, store, actions) -> { destroy() }
                actions: { selectCity(key), clearSelection(), setUnit(u), setQuery(q), retryWeather(), retryForecast(key) }
dom.js          el(tag, attrs, ...children) -> Element     // strings become text nodes only
                svgEl(tag, attrs, ...children) -> SVGElement
                icon(kind, size) -> SVGElement
main.js         start() -> void
```
