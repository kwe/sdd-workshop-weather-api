# Performance Test Instructions

## Requirement
- NFR: forecast response time < 200 ms. No throughput or concurrency targets were specified.

## Approach
The endpoint is one indexed query (`city_key`) returning at most 5 rows from a seeded, static table, so no load-test harness is warranted. A latency check against the release binary is sufficient.

## Run
```bash
cargo build --release
DATABASE_URL="sqlite:///tmp/perf.db?mode=rwc" ./target/release/sdd-workshop-weather-api &
for i in $(seq 1 50); do
  curl -s -o /dev/null -w "%{time_total}\n" localhost:8080/api/weather/forecast/london
done | sort -n | tail -1
kill <PID>; rm /tmp/perf.db
```

## Result (single smoke request, local)
- Response time: 1.4 ms (target < 200 ms) - pass.
- If a load test is needed later, use `k6` or `hey` against `/api/weather/forecast/{city}`.
