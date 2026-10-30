# Domain Entities — forecast-api

## ForecastEntry (persisted, table `forecasts`)
| Field | Type | Rules |
|---|---|---|
| id | integer PK autoincrement | internal |
| city | text NOT NULL | display name, e.g. `São Paulo` |
| city_key | text NOT NULL | lowercase of `city` using Unicode lowercasing (equals Rust `to_lowercase()`), e.g. `são paulo`; lookup key |
| day_offset | integer NOT NULL | 0-4 (CHECK); day 0 = today |
| high_temperature_celsius | real NOT NULL | one decimal place |
| low_temperature_celsius | real NOT NULL | CHECK high >= low |
| conditions | text NOT NULL | non-empty; seed vocabulary: Sunny, Partly Cloudy, Cloudy, Overcast, Rainy, Stormy, Windy, Clear (not enforced by constraint) |
| precipitation_chance | integer NOT NULL | CHECK 0-100 |

**Constraints**: UNIQUE (`city_key`, `day_offset`); index on `city_key`; no foreign key to `weather_records`.

## ForecastRow (internal)
Repository result: `day_offset`, `high_temperature_celsius`, `low_temperature_celsius`, `conditions`, `precipitation_chance`.

## ForecastDay (public DTO, JSON camelCase)
| Field | Type | Source |
|---|---|---|
| date | string `YYYY-MM-DD` | UTC today + `day_offset` |
| highTemperatureCelsius | number | row |
| lowTemperatureCelsius | number | row |
| conditions | string | row |
| precipitationChance | integer 0-100 | row |

## Relationships
`ForecastEntry` rows are grouped by `city_key` (5 per city). No relation to `WeatherRecord`; a city in one table need not be in the other.
