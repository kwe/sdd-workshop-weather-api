# Claims: weather-alerts

- route: GET /api/alerts
- table: alert_thresholds
- migration: 20261030130300_create_alert_thresholds.sql
- file: src/alerts.rs (create)
- file: src/lib.rs (edit)
- file: src/frontend.rs (edit)
- file: tests/alerts.rs (create)
- file: tests/alerts_frontend.rs (create)
- file: static/index.html (edit)
- file: static/css/app.css (edit)
- file: static/js/api.js (edit)
- file: static/js/state.js (edit)
- file: static/js/main.js (edit)
- file: static/js/ui.js (edit)
- file: static/js/globe-scene.js (edit)
- file: README.md (edit)
- js-module: static/js/alerts.js (create)
