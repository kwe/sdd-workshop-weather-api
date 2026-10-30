# Frontend Components — globe-frontend

Maps to the modules in `globe-frontend-application-design.md`. Props are plain function arguments (no framework).

## Component hierarchy (DOM)
```text
body
  #app
    #globe            (role=img, aria-label)  <- globe-scene.js owns the canvas and pin elements
    aside.sidebar     (left glass column)
      header: h1 title, .status-chip, .unit-toggle (2 buttons)
      .search: input[type=search]
      ul.city-list > li > button.city-item
    aside.detail      (right glass panel, role=complementary, aria-labelledby)
      header: h2 city, button.close
      section.current: icon, big temperature, description, stats (humidity, wind, recorded)
      section.forecast: 5 x article.day (weekday, icon, high/low bar, precipitation bar)
      .state: loading skeleton | none message | error + Retry
    .legend           (gradient + labels)
    button.reset-view
    .banner           (globe unavailable / offline / errors)
    .veil             (initial loading)
    #live             (polite live region, visually hidden)
```

## Modules, state and props

| Module | Inputs | Owns state | Outputs |
|---|---|---|---|
| main.js | none | timers, abort controllers | calls store/globe/ui |
| state.js | initial | the store (cities, selectedKey, unit, forecasts, status, query, drawerOpen) | subscriptions |
| api.js | city name, AbortSignal | none | parsed arrays or ApiError/ApiNotFound |
| ui.js | store, actions | focus return target, scroll positions | DOM updates, action calls |
| globe-scene.js | container, callbacks, reducedMotion | globe instance, sun state, pin elements, timers | onSelect(key), onHover(key or null) |

## User interaction flows
1. **Explore**: drag/touch rotates, wheel/pinch zooms; auto-rotate pauses; hover pin shows tooltip; click selects.
2. **Select from list/search**: type to filter (debounce 80 ms, normalised substring on name or country), arrows move, Enter selects; same effects as pin click.
3. **Detail panel**: opens with current conditions immediately, forecast skeleton, then five forecast cards; Retry on error; close/Esc/reset returns the camera.
4. **Units**: toggle updates pins, list, panel and legend instantly; persisted.
5. **Mobile**: list drawer toggle; detail is a bottom sheet; pinch zoom; touch targets >= 44 px.
6. **Failure flows**: see BR-10.

## Form/input validation
- Search input: max 60 chars, control characters stripped, normalised before matching; never inserted as HTML; empty query shows all cities.
- Unit value read from `localStorage` must equal 'C' or 'F', otherwise 'C'.
- No other user input exists.

## API integration points

| Component | Endpoint | Usage | Errors |
|---|---|---|---|
| main.js via api.getWeather | `GET /api/weather` | start + every 60 s | network/timeout/500 -> BR-10.2 |
| main.js via api.getForecast | `GET /api/weather/forecast/{encodeURIComponent(city)}` | on selection | 404 -> none, other -> error |

No other endpoint is called; `POST`/`DELETE` are never used by the UI.

## Visual design tokens
- Typography: system UI stack (`ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`); title weight 600, tracking 0.08em, uppercase; tabular numbers for temperatures.
- Palette: deep space background `#05070d`, panel glass `rgba(12, 18, 32, 0.62)` with `backdrop-filter: blur(18px) saturate(140%)` and 1 px border `rgba(255,255,255,0.10)` (fallback solid `#0c1220` when blur unsupported), accent `#5aa9ff`, text `#e8eefc` / secondary `#9fb0d0`.
- Motion: 240 ms ease-out panel slide, staggered card fade-in (40 ms steps), pin pop-in on load, intro camera sweep; all disabled under reduced motion.
- Glow: pins use `box-shadow` glow in the temperature colour via a CSS custom property set with CSSOM.

## Static asset inventory (embedded)
`static/index.html`, `static/css/app.css`, `static/js/{main,state,api,gazetteer,weather,sun,dom,globe-scene,ui}.js`, `static/favicon.svg`.
