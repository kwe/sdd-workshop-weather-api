# Business Rules — forecast-api

| ID | Rule | Story / Req |
|---|---|---|
| BR-1 | Response is a JSON array of exactly 5 `ForecastDay`, ordered by `date` ascending. | S1 / FR-1 |
| BR-2 | `date` = UTC calendar date of the request + `day_offset` (0-4), formatted `YYYY-MM-DD`. "Today" is `Utc::now().date_naive()`; documented in OpenAPI and README. | S1 / FR-3 |
| BR-3 | City input is trimmed. If empty after trimming -> 400 `City is required`. | S2 / FR-5 |
| BR-4 | Lookup key = trimmed input lowercased with Unicode `to_lowercase()`; matched by equality on `city_key`. Percent-decoding is done by axum before the service. | S2 / FR-4 |
| BR-5 | No rows for the key (unknown city or known city without forecast) -> 404 `No forecast found for city {city}`, where `{city}` is the trimmed input in the caller's original casing. | S3 / FR-6 |
| BR-6 | No special handling for long or odd input: it just does not match -> 404. | S3 |
| BR-7 | Error body uses the standard shape (`timestamp,status,error,message,path`); `path` is the request path (percent-encoded form as received by `Uri`). | S3 / NFR-3 |
| BR-8 | Only GET is routed; other verbs return 405. No aliases (e.g. `NYC` -> 404). No authentication. | S4 / FR-9 |
| BR-9 | Data integrity enforced by CHECKs: high >= low; chance 0-100; offset 0-4; UNIQUE (`city_key`,`day_offset`). | S5 / FR-8 |
| BR-10 | Seed `city_key` must equal Rust `to_lowercase()` of `city` for all rows (verified by test). No consistency rule between `conditions` and `precipitation_chance` is enforced beyond the CHECKs; seeded values are kept sensible by hand. | S5 / FR-7 |
| BR-11 | Temperatures are stored with one decimal place and serialised as-is. | S1 |
| BR-12 | A database failure -> 500 `An unexpected error occurred` via existing `From<sqlx::Error>`. | NFR-3 |

## Validation Order
1. Trim -> blank check (400)
2. Normalise -> query
3. Empty -> 404
4. Map rows -> 200
