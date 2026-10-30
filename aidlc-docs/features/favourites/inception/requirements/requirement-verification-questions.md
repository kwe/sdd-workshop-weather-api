# Requirement Verification Questions: favourites

Depth: Standard. Every question has a pre-filled `(default, assumed)` answer; change it only if you disagree.

## Question 1: City identity
The API has no cities table; a city is the `city` text in `weather_records`. How should favourites match cities?
A) Case-insensitive, trimmed match on city name only; store the canonical spelling from the latest weather record
B) Match on city + country
X) Other

[Answer]: A (default, assumed)

## Question 2: "Latest weather" definition
A city can have several records. Which is "latest"?
A) Highest `recordedAt`, ties broken by highest `id`
B) Highest `id`
X) Other

[Answer]: A (default, assumed)

## Question 3: GET /api/favourites response shape
A) `[{"city": "London", "favouritedAt": "2026-10-30T13:00:00", "weather": <WeatherRecord>|null}]`, newest favourite first
B) Flat WeatherRecord array
X) Other

[Answer]: A (default, assumed)

## Question 4: POST /api/favourites response body
A) 201/200 both return the favourite object (same shape as one GET item); 404 if no weather records for the city; 400 for missing/blank/non-string `city` or malformed JSON, or city longer than 100 chars
B) Empty body
X) Other

[Answer]: A (default, assumed)

## Question 5: Favourite whose weather records were later deleted
A) Keep it listed with `"weather": null` (user can still remove it)
B) Hide it from the list
X) Other

[Answer]: A (default, assumed)

## Question 6: DELETE /api/favourites/{city}
A) Case-insensitive match, URL-decoded; 204 on success, 404 if not a favourite, 400 if blank
B) Exact match only
X) Other

[Answer]: A (default, assumed)

## Question 7: Scope of favourites
A) One global list (the API has no users/auth); persisted server-side only, no localStorage
B) Per-client list
X) Other

[Answer]: A (default, assumed)

## Question 8: Globe UI
A) Star toggle button in the detail panel header (`aria-pressed`, keyboard accessible); a "Favourites" section above the city list (hidden when empty) whose items select the city (flying the globe to it); favourite pins get a gold ring and a star badge. Optimistic-free: UI updates after the API call succeeds, errors announced via the live region/banner
B) Other (describe)

[Answer]: A (default, assumed)

## Question 9: Frontend tests
No JS runner exists. A) Add a dependency-free Node test (`node --test`) for the pure favourites logic plus Rust tests in `tests/frontend.rs` (assets served, XSS/CSP rules still hold, new module registered); no npm packages. B) Rust tests only
X) Other

[Answer]: B - Rust tests only, no node (user override: frontend tests stay in Rust like tests/frontend.rs; API tests in tests/favourites.rs)

## Question 10: Extension opt-ins (all default to NO for this small feature)
Security baseline (A yes / B no): [Answer]: B (default, assumed)
Resiliency baseline (A yes / B no): [Answer]: B (default, assumed)
Property-based testing (A yes / B partial / C no): [Answer]: C (default, assumed)
