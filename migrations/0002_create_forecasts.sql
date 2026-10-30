CREATE TABLE forecasts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    city TEXT NOT NULL,
    city_key TEXT NOT NULL,
    day_offset INTEGER NOT NULL CHECK (day_offset BETWEEN 0 AND 4),
    high_temperature_celsius REAL NOT NULL,
    low_temperature_celsius REAL NOT NULL,
    conditions TEXT NOT NULL CHECK (length(trim(conditions)) > 0),
    precipitation_chance INTEGER NOT NULL CHECK (precipitation_chance BETWEEN 0 AND 100),
    CHECK (high_temperature_celsius >= low_temperature_celsius),
    UNIQUE (city_key, day_offset)
);

CREATE INDEX idx_forecasts_city_key ON forecasts (city_key);

-- city_key is the lowercase form of city and must match Rust's str::to_lowercase().
INSERT INTO forecasts
    (city, city_key, day_offset, high_temperature_celsius, low_temperature_celsius, conditions, precipitation_chance)
VALUES
    ('London', 'london', 0, 19.0, 11.0, 'Cloudy', 40),
    ('London', 'london', 1, 17.5, 10.0, 'Rainy', 80),
    ('London', 'london', 2, 18.0, 9.5, 'Overcast', 55),
    ('London', 'london', 3, 20.5, 11.5, 'Partly Cloudy', 25),
    ('London', 'london', 4, 21.0, 12.0, 'Sunny', 10),

    ('New York', 'new york', 0, 27.5, 19.0, 'Sunny', 10),
    ('New York', 'new york', 1, 28.0, 20.0, 'Partly Cloudy', 20),
    ('New York', 'new york', 2, 25.5, 18.5, 'Cloudy', 45),
    ('New York', 'new york', 3, 23.0, 17.0, 'Rainy', 75),
    ('New York', 'new york', 4, 24.5, 16.5, 'Partly Cloudy', 30),

    ('Tokyo', 'tokyo', 0, 30.5, 25.0, 'Cloudy', 35),
    ('Tokyo', 'tokyo', 1, 31.0, 25.5, 'Partly Cloudy', 30),
    ('Tokyo', 'tokyo', 2, 29.0, 24.5, 'Rainy', 70),
    ('Tokyo', 'tokyo', 3, 28.5, 24.0, 'Stormy', 85),
    ('Tokyo', 'tokyo', 4, 30.0, 25.0, 'Sunny', 15),

    ('Sydney', 'sydney', 0, 17.0, 11.5, 'Windy', 25),
    ('Sydney', 'sydney', 1, 18.5, 12.0, 'Sunny', 10),
    ('Sydney', 'sydney', 2, 19.5, 12.5, 'Clear', 5),
    ('Sydney', 'sydney', 3, 16.0, 11.0, 'Cloudy', 40),
    ('Sydney', 'sydney', 4, 15.5, 10.5, 'Rainy', 65),

    ('Paris', 'paris', 0, 22.5, 13.0, 'Partly Cloudy', 20),
    ('Paris', 'paris', 1, 23.5, 14.0, 'Sunny', 10),
    ('Paris', 'paris', 2, 21.0, 13.5, 'Cloudy', 35),
    ('Paris', 'paris', 3, 19.5, 12.0, 'Rainy', 70),
    ('Paris', 'paris', 4, 20.5, 11.5, 'Overcast', 45),

    ('Berlin', 'berlin', 0, 20.5, 11.0, 'Overcast', 40),
    ('Berlin', 'berlin', 1, 19.0, 10.5, 'Rainy', 65),
    ('Berlin', 'berlin', 2, 18.0, 9.5, 'Cloudy', 45),
    ('Berlin', 'berlin', 3, 21.0, 10.0, 'Partly Cloudy', 20),
    ('Berlin', 'berlin', 4, 22.5, 11.5, 'Sunny', 10),

    ('Mumbai', 'mumbai', 0, 32.0, 26.0, 'Rainy', 85),
    ('Mumbai', 'mumbai', 1, 31.5, 25.5, 'Stormy', 90),
    ('Mumbai', 'mumbai', 2, 31.0, 25.5, 'Rainy', 80),
    ('Mumbai', 'mumbai', 3, 32.5, 26.0, 'Cloudy', 55),
    ('Mumbai', 'mumbai', 4, 32.0, 26.5, 'Rainy', 75),

    ('São Paulo', 'são paulo', 0, 25.0, 17.5, 'Partly Cloudy', 30),
    ('São Paulo', 'são paulo', 1, 26.5, 18.0, 'Sunny', 15),
    ('São Paulo', 'são paulo', 2, 24.0, 17.0, 'Cloudy', 45),
    ('São Paulo', 'são paulo', 3, 22.0, 16.0, 'Rainy', 70),
    ('São Paulo', 'são paulo', 4, 23.5, 16.5, 'Partly Cloudy', 35),

    ('Cape Town', 'cape town', 0, 15.0, 8.5, 'Windy', 20),
    ('Cape Town', 'cape town', 1, 16.5, 9.0, 'Sunny', 10),
    ('Cape Town', 'cape town', 2, 17.5, 9.5, 'Clear', 5),
    ('Cape Town', 'cape town', 3, 14.0, 8.0, 'Cloudy', 40),
    ('Cape Town', 'cape town', 4, 13.5, 7.5, 'Rainy', 65),

    ('Toronto', 'toronto', 0, 20.0, 12.0, 'Clear', 10),
    ('Toronto', 'toronto', 1, 21.5, 13.0, 'Sunny', 10),
    ('Toronto', 'toronto', 2, 19.0, 12.5, 'Partly Cloudy', 25),
    ('Toronto', 'toronto', 3, 17.0, 11.0, 'Rainy', 70),
    ('Toronto', 'toronto', 4, 18.5, 10.5, 'Cloudy', 40);
