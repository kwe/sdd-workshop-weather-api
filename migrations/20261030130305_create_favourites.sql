CREATE TABLE favourites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    city TEXT NOT NULL,
    -- city_key is the lowercase form of city and must match Rust's str::to_lowercase().
    city_key TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);
