# Application Design Plan (Minimal Depth)

Please fill in each `[Answer]:` tag, then tell me when you're done.

## Part A: Design Questions

## Question 1
Where should the forecast code live? (Constraints require the same `models`/`repository`/`service`/`handlers` structure.)

A) Add to the existing files (`models.rs`, `repository.rs`, `service.rs`, `handlers.rs`), mirroring the current layout

B) Create new forecast-specific modules (e.g. `forecast_service.rs`)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
How should the service compute rolling dates so tests are deterministic?

A) Service function takes `today: NaiveDate` as a parameter (handler passes `Utc::now().date_naive()`), keeping date logic pure and unit-testable

B) Service calls `Utc::now()` internally (simpler signature, harder to unit test)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Repository return type for forecast rows.

A) Internal row struct (`ForecastRow` with `day_offset`, high, low, conditions, precipitation chance) that the service maps to the public `ForecastDay` DTO with a computed `date`

B) Repository returns the public `ForecastDay` directly, computing the date in SQL (`date('now', '+N day')`)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
City lookup in the repository (existing code filters in Rust for Unicode-safe case-insensitivity).

A) Fetch forecast rows joined/keyed by city name, filter in Rust with `to_lowercase()` comparison, same approach as `list_weather_records` (acceptable for 10 cities)

B) Store a normalised `city_key` column (lowercased) in `forecasts` and query by equality in SQL

C) Other (please describe after [Answer]: tag below)

[Answer]: B — store a city_key column, hand-written lowercase in the seed (e.g. "são paulo"), with an index; the service normalises input with trim + Rust to_lowercase() and the repository queries by equality. Satisfies NFR-1 (single indexed query) and FR-4 Unicode case-insensitivity.

## Question 5
Handler input handling.

A) Extract `Path<String>` (like existing ID handlers) plus `Uri` for error paths; service trims and rejects blank with 400

B) Validate blank city in the handler, service receives a clean string

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Part B: Mandatory Artifacts (Generation Checklist)
- [x] Step 1: Generate `application-design/components.md`
- [x] Step 2: Generate `application-design/component-methods.md`
- [x] Step 3: Generate `application-design/services.md`
- [x] Step 4: Generate `application-design/component-dependency.md`
- [x] Step 5: Generate consolidated `application-design/application-design.md`
- [x] Step 6: Validate completeness/consistency against stories S1-S5 and update state and audit
