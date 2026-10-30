# Functional Design Plan — unit `forecast-api`

Units Generation was skipped, so the single unit is `forecast-api` covering stories S1-S5. Please fill in each `[Answer]:` tag, then tell me when you're done.

## Part A: Questions

## Question 1
Seed data profile: how realistic should the 10 cities' 5-day values be?

A) Plausible and distinct per city: climate-appropriate highs/lows (relative to the existing seed observations, e.g. London ~18C, Mumbai ~31C), varied conditions and rain chances

B) Simple uniform pattern for every city (same shape, shifted temperatures)

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 2
Conditions vocabulary (the `conditions` string).

A) Reuse the existing seed style, a small fixed set: `Sunny`, `Partly Cloudy`, `Cloudy`, `Overcast`, `Rainy`, `Stormy`, `Windy`, `Clear`

B) Free-form per row, no fixed vocabulary

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Consistency rule between conditions and precipitation chance in the seed.

A) Loosely consistent: `Rainy`/`Stormy` >= 60%, `Sunny`/`Clear` <= 20%, others in between; asserted by a seed test

B) No consistency rule, only the 0-100 CHECK

C) Other (please describe after [Answer]: tag below)

[Answer]: B — no source document asks for this; keep values sensible in the seed but enforce only the CHECK constraints

## Question 4
Numeric precision of temperatures in the JSON.

A) One decimal place in the seed (e.g. 19.5), serialised as-is

B) Whole degrees only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
404 message wording and city echo.

A) `No forecast found for city {city}` where `{city}` is the trimmed city as the caller typed it (original casing)

B) A generic message without echoing the input

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 6
"Today" definition for rolling dates.

A) UTC date (`Utc::now().date_naive()`), documented in OpenAPI and README

B) Server local date

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7
Behaviour for very long or odd city input (e.g. > 100 chars, control characters).

A) No special handling: it simply doesn't match and returns 404

B) Reject over 100 characters with 400 "City is too long"

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Part B: Artifact Checklist
- [x] Step 1: Create `construction/forecast-api/functional-design/domain-entities.md`
- [x] Step 2: Create `construction/forecast-api/functional-design/business-rules.md`
- [x] Step 3: Create `construction/forecast-api/functional-design/business-logic-model.md` (including the exact seed table)
- [x] Step 4: Validate against stories S1-S5 and update state and audit
