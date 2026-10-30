CREATE TABLE weather_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    city TEXT NOT NULL,
    country TEXT,
    temperature_celsius REAL,
    humidity INTEGER,
    description TEXT,
    wind_speed_kmh REAL,
    recorded_at TEXT NOT NULL
);

INSERT INTO weather_records
    (city, country, temperature_celsius, humidity, description, wind_speed_kmh, recorded_at)
VALUES
    ('London', 'UK', 18.5, 68, 'Cloudy', 14.2, '2026-07-17T08:00:00'),
    ('New York', 'USA', 27.1, 59, 'Sunny', 11.4, '2026-07-17T03:00:00'),
    ('Tokyo', 'Japan', 30.3, 74, 'Humid', 9.7, '2026-07-17T16:00:00'),
    ('Sydney', 'Australia', 15.6, 61, 'Breezy', 19.8, '2026-07-17T17:00:00'),
    ('Paris', 'France', 22.0, 57, 'Partly Cloudy', 10.1, '2026-07-17T09:00:00'),
    ('Berlin', 'Germany', 20.4, 63, 'Overcast', 13.6, '2026-07-17T09:30:00'),
    ('Mumbai', 'India', 31.7, 82, 'Rainy', 17.3, '2026-07-17T12:30:00'),
    ('São Paulo', 'Brazil', 24.2, 71, 'Warm', 8.5, '2026-07-17T05:30:00'),
    ('Cape Town', 'South Africa', 13.9, 66, 'Windy', 22.4, '2026-07-17T10:30:00'),
    ('Toronto', 'Canada', 19.3, 54, 'Clear', 12.7, '2026-07-17T04:30:00');
