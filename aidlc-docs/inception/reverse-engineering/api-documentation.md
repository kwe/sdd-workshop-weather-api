# API Documentation

## REST APIs
| Method | Path | Purpose | Success | Errors |
|---|---|---|---|---|
| GET | /api/weather?city= | List records, optional case-insensitive city filter (blank ignored) | 200 `[WeatherRecord]` | 500 |
| GET | /api/weather/{id} | Get record | 200 | 400 non-integer id, 404 |
| POST | /api/weather | Create record | 201 | 400 validation/malformed JSON |
| DELETE | /api/weather/{id} | Delete record | 204 | 400, 404 |
| GET | /swagger-ui.html, /swagger-ui/, /v3/api-docs | Docs | 200/307 | - |

## Internal APIs
- service: `list_weather_records`, `get_weather_record`, `create_weather_record`, `delete_weather_record` (all return `Result<_, ApiError>`)
- repository: `list_weather_records`, `get_weather_record`, `insert_weather_record`, `delete_weather_record` (return `sqlx::Error`)

## Data Models
- **WeatherRecord**: id, city, country?, temperatureCelsius?, humidity?, description?, windSpeedKmh?, recordedAt (string)
- **ErrorResponse**: timestamp, status, error, message, path
- **Validation**: city required; temperature -90..60; humidity 0..100
- **Table weather_records**: 10 seeded cities (London, New York, Tokyo, Sydney, Paris, Berlin, Mumbai, São Paulo, Cape Town, Toronto)
