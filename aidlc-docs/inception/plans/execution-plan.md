# Execution Plan

## Detailed Analysis Summary

### Transformation Scope (Brownfield Only)
- **Transformation Type**: Single component (additive feature)
- **Primary Changes**: New `forecasts` table (migration `0002`), forecast model, repository query, service logic, handler, route and OpenAPI registration in the existing crate; new tests; README update.
- **Related Components**: None (no infra, no shared packages, no client libs in this repo). Downstream consumers (Dashboard, Mobile, partners) are unaffected until they adopt the endpoint.

### Change Impact Assessment
- **User-facing changes**: Yes - new read-only endpoint `GET /api/weather/forecast/{city}`
- **Structural changes**: No - same layers and modules
- **Data model changes**: Yes - new `forecasts` table; `weather_records` unchanged
- **API changes**: Yes, additive only; no existing contract changes
- **NFR impact**: Minimal - single indexed query, target < 200 ms p95

### Component Relationships
- **Primary Component**: sdd-workshop-weather-api (modify: models, repository, service, handlers, lib; add migration 0002; tests; README)
- **Infrastructure / Shared / Supporting**: none
- **Dependent Components**: Dashboard, Mobile, partner clients (additive, no change required)

| Component | Change Type | Reason | Priority |
|---|---|---|---|
| src/models.rs | Minor | new `ForecastDay` schema | Critical |
| src/repository.rs | Minor | forecast query | Critical |
| src/service.rs | Minor | city validation, date calculation, 404 | Critical |
| src/handlers.rs | Minor | new handler + utoipa path | Critical |
| src/lib.rs | Minor | route + ApiDoc | Critical |
| migrations/0002_*.sql | New | table + seed + CHECKs | Critical |
| tests/api.rs and unit tests | Minor | S1-S5 coverage | Critical |
| README.md | Minor | docs | Important |

### Risk Assessment
- **Risk Level**: Low
- **Rollback Complexity**: Easy (revert commit; a dropped-table down-migration is not needed for a dev SQLite DB, but note `weather.db` will have 0002 applied after a local run)
- **Testing Complexity**: Simple

## Workflow Visualization

```mermaid
flowchart TD
    Start(["User Request"])

    subgraph INCEPTION["INCEPTION PHASE"]
        WD["Workspace Detection<br/><b>COMPLETED</b>"]
        RE["Reverse Engineering<br/><b>COMPLETED</b>"]
        RA["Requirements Analysis<br/><b>COMPLETED</b>"]
        US["User Stories<br/><b>COMPLETED</b>"]
        WP["Workflow Planning<br/><b>COMPLETED</b>"]
        AD["Application Design<br/><b>EXECUTE (minimal)</b>"]
        UG["Units Generation<br/><b>SKIP</b>"]
    end

    subgraph CONSTRUCTION["CONSTRUCTION PHASE"]
        FD["Functional Design<br/><b>EXECUTE</b>"]
        NFRA["NFR Requirements<br/><b>SKIP</b>"]
        NFRD["NFR Design<br/><b>SKIP</b>"]
        ID["Infrastructure Design<br/><b>SKIP</b>"]
        CG["Code Generation<br/><b>EXECUTE</b>"]
        BT["Build and Test<br/><b>EXECUTE</b>"]
    end

    subgraph OPERATIONS["OPERATIONS PHASE"]
        OPS["Operations<br/><b>PLACEHOLDER</b>"]
    end

    Start --> WD
    WD --> RE
    RE --> RA
    RA --> US
    US --> WP
    WP --> AD
    AD --> FD
    FD --> CG
    CG --> BT
    BT --> End(["Complete"])

    style WD fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style RE fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style RA fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style US fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style WP fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style CG fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style BT fill:#4CAF50,stroke:#1B5E20,stroke-width:3px,color:#fff
    style FD fill:#FFA726,stroke:#E65100,stroke-width:3px,stroke-dasharray: 5 5,color:#000
    style AD fill:#FFA726,stroke:#E65100,stroke-width:3px,stroke-dasharray: 5 5,color:#000
    style UG fill:#BDBDBD,stroke:#424242,stroke-width:2px,stroke-dasharray: 5 5,color:#000
    style NFRA fill:#BDBDBD,stroke:#424242,stroke-width:2px,stroke-dasharray: 5 5,color:#000
    style NFRD fill:#BDBDBD,stroke:#424242,stroke-width:2px,stroke-dasharray: 5 5,color:#000
    style ID fill:#BDBDBD,stroke:#424242,stroke-width:2px,stroke-dasharray: 5 5,color:#000
    style OPS fill:#FFF59D,stroke:#F57F17,stroke-width:2px,color:#000
    style Start fill:#CE93D8,stroke:#6A1B9A,stroke-width:3px,color:#000
    style End fill:#CE93D8,stroke:#6A1B9A,stroke-width:3px,color:#000
    style INCEPTION fill:#BBDEFB,stroke:#1565C0,stroke-width:3px,color:#000
    style CONSTRUCTION fill:#C8E6C9,stroke:#2E7D32,stroke-width:3px,color:#000
    style OPERATIONS fill:#FFF59D,stroke:#F57F17,stroke-width:3px,color:#000
    linkStyle default stroke:#333,stroke-width:2px
```

Text alternative: Workspace Detection -> Reverse Engineering -> Requirements -> User Stories -> Workflow Planning -> Application Design (minimal) -> Functional Design -> Code Generation -> Build and Test. Skipped: Units Generation, NFR Requirements, NFR Design, Infrastructure Design.

## Phases to Execute

### INCEPTION PHASE
- [x] Workspace Detection (COMPLETED)
- [x] Reverse Engineering (COMPLETED)
- [x] Requirements Analysis (COMPLETED)
- [x] User Stories (COMPLETED)
- [x] Workflow Planning (COMPLETED - approved with Application Design added)
- [ ] Application Design - EXECUTE (minimal depth) - *added at user request*
  - **Rationale**: The feature adds new types and methods in models/repository/service/handlers; capture signatures and layering before Functional Design.
- [ ] Units Generation - SKIP
  - **Rationale**: One cohesive unit of work (single crate, single endpoint). Treated as one unit named `forecast-api`.

### CONSTRUCTION PHASE (unit: forecast-api)
- [ ] Functional Design - EXECUTE
  - **Rationale**: New schema/data model and business rules (rolling dates, city normalisation, error semantics, seed values) are worth pinning down before coding.
- [ ] NFR Requirements - SKIP
  - **Rationale**: Tech stack fixed; NFRs already in requirements.md (NFR-1..6); security/resiliency/PBT extensions opted out.
- [ ] NFR Design - SKIP
  - **Rationale**: No NFR requirements stage; no new patterns needed.
- [ ] Infrastructure Design - SKIP
  - **Rationale**: No infrastructure changes (same service and SQLite DB).
- [ ] Code Generation - EXECUTE (ALWAYS)
  - **Rationale**: Implementation planning and code generation.
- [ ] Build and Test - EXECUTE (ALWAYS)
  - **Rationale**: Run fmt, clippy, tests; write build/test instructions.

### OPERATIONS PHASE
- [ ] Operations - PLACEHOLDER

## Package Change Sequence
Single package. Order within the unit: migration -> models -> repository -> service -> handlers -> lib.rs routing/OpenAPI -> tests -> README.

## Estimated Timeline
- **Total Stages Remaining**: 4 (Application Design, Functional Design, Code Generation, Build and Test)
- **Estimated Duration**: well within Sprint 1 (small additive change)

## Success Criteria
- **Primary Goal**: `GET /api/weather/forecast/{city}` delivers stories S1-S5.
- **Key Deliverables**: migration 0002, Rust code across 5 modules, tests, OpenAPI and README updates, build/test instructions.
- **Quality Gates**: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test` all pass; all pre-existing tests unchanged and passing.
- **Integration Testing**: HTTP-level tests on in-memory SQLite.
