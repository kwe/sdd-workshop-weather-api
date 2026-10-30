# Code Summary: favourites

## Created
- `migrations/20261030130305_create_favourites.sql`: `favourites(id, city, city_key UNIQUE, created_at)`.
- `src/favourites.rs`: models, validation (trim, 1-100 chars), service, handlers with utoipa docs, `router()`.
- `static/js/favourites.js`: DOM-free helpers (`favouriteKeyOf`, `isFavourite`, `favouriteItems`).
- `tests/favourites.rs`: 8 API tests.

## Edited (additive hooks only)
- `src/lib.rs`: `pub mod`, ApiDoc paths/schemas/tag, `.merge(favourites::router())`.
- `src/frontend.rs`: register `js/favourites.js`.
- `static/index.html`: Favourites section, star button.
- `static/css/app.css`: appended `/* favourites */` block (gold ring pin is pure CSS; `ringsData` untouched).
- `static/js/api.js`: `getFavourites`, `addFavourite`, `removeFavourite`.
- `static/js/state.js`: `favourites: []`.
- `static/js/main.js`: load on refresh, `toggleFavourite`, scene hook.
- `static/js/ui.js`: `renderFavourites`, `renderStar`.
- `static/js/globe-scene.js`: `favourite` class, star badge, aria-label, optional third `setCities` arg.
- `tests/frontend.rs`: one added test.
- `README.md`: endpoint rows + `### Favourites`.

## Verified so far
`cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` (all pass).
Not yet done: rebase on main and curl run on port 8081 (Build and Test gate).
