CREATE TABLE alert_thresholds (
    alert_type TEXT PRIMARY KEY CHECK (alert_type IN ('heat', 'cold', 'wind', 'humidity')),
    comparison TEXT NOT NULL CHECK (comparison IN ('gte', 'lte')),
    -- threshold is the lowest ("watch") level; warning and severe are progressively more extreme.
    threshold REAL NOT NULL,
    warning_threshold REAL NOT NULL,
    severe_threshold REAL NOT NULL,
    unit TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    CHECK (
        (comparison = 'gte' AND threshold <= warning_threshold AND warning_threshold <= severe_threshold)
        OR (comparison = 'lte' AND threshold >= warning_threshold AND warning_threshold >= severe_threshold)
    )
);

INSERT INTO alert_thresholds
    (alert_type, comparison, threshold, warning_threshold, severe_threshold, unit, enabled)
VALUES
    ('heat', 'gte', 30.0, 35.0, 40.0, 'C', 1),
    ('cold', 'lte', 0.0, -5.0, -15.0, 'C', 1),
    ('wind', 'gte', 40.0, 60.0, 90.0, 'km/h', 1),
    ('humidity', 'gte', 85.0, 90.0, 95.0, '%', 1);
