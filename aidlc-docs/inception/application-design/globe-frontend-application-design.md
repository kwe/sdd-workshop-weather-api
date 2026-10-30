# Application Design (Consolidated) — Globe Frontend (Cycle 2)

Consolidates `globe-frontend-components.md`, `globe-frontend-component-methods.md`, `globe-frontend-services.md` and `globe-frontend-component-dependency.md`. Cycle 1 `application-design.md` is unchanged.

## Summary
- **Rust**: one new module `src/frontend.rs` (embedded assets, `GET /` and `GET /static/{*path}`, frontend-only security headers incl. CSP) merged into the router in `lib.rs`. No change to API code, DB, OpenAPI or existing tests. New `static/` tree embedded with `include_str!`/`include_bytes!`.
- **Browser**: nine small ES modules (`main`, `state`, `api`, `gazetteer`, `weather`, `sun`, `dom`, `globe-scene`, `ui`), no build step, pub/sub store, `api.js` as sole network access.
- **Third parties**: globe.gl 2.46.2 (bundles three.js) with SRI and `three-globe@2.45.6` textures, all from `cdn.jsdelivr.net`. CSP allows only `'self'` plus that host (scripts strict; inline styles allowed initially, tightened if the libraries allow).
- **Visuals**: patched globe material for day/night blend, ocean specular (water mask), twilight band, Fresnel rim glow and bump; procedural clouds (the package has no cloud texture); library atmosphere; starfield; DOM-built pins with condition icons and pulse rings.
- **Safety**: all API strings via `textContent`/DOM builders; `dom.js` offers no HTML-string API.

## Decisions (Application Design plan, all A)
Q1 `frontend.rs` + `static/`; Q2 ES modules per concern; Q3 jsDelivr, globe.gl UMD + three-globe images, SRI; Q4 HTML-element pins + rings; Q5 patched material for day/night (+ specular, twilight, Fresnel; clouds procedural); Q6 `gazetteer.js` with matcher; Q7 `script-src 'self' cdn.jsdelivr.net`, `style-src 'self' 'unsafe-inline'`; Q8 `Cache-Control: no-cache`; Q9 pub/sub store; Q10 new `tests/frontend.rs`.

## Feasibility findings (verified 2026-10-30)
- globe.gl 2.46.2 UMD and three-globe 2.45.6 textures are reachable with CORS `*`; SRI hash computable.
- No cloud texture exists, so clouds are procedural.
- The UMD exposes no `THREE` global, so effects patch the existing material with `onBeforeCompile`. Fallback (to be raised with the user first): import map with a CSP script hash.

## Open items carried to Functional Design
Exact CSP string and minimal `img-src`/`connect-src`; colour scale; icon set; shader GLSL and parameters; gazetteer city list and aliases; reduced-motion rules; focus management; fallback behaviours.
