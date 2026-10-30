# Story Generation Plan — 5-Day Forecast

Please fill in every `[Answer]:` tag in Part A, then tell me when done. Your answers are recorded in the audit trail.

## Part A: Planning Questions

## Question 1
Story breakdown approach (see options in Part C).

A) Feature-based: stories grouped by capability (retrieve forecast, city handling, errors, docs)

B) Persona-based: stories grouped by consumer (Dashboard, Mobile, partners)

C) Hybrid: persona-driven "as a..." stories, with a shared set of feature-level acceptance criteria

D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 2
Personas to create (select the set to model).

A) Three: Internal product developer (Dashboard + Mobile), Partner integration developer, Platform/API maintainer

B) Five: Dashboard dev, Mobile dev, Partner (logistics/batch), Partner (consumer travel app), Platform maintainer

C) Two: Internal consumer, External partner

D) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 3
Story granularity.

A) Coarse: about 4-5 stories, each covering a full behaviour with several acceptance criteria

B) Fine: about 8-10 small stories, one behaviour each

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
Acceptance criteria format.

A) Given/When/Then (Gherkin-style) bullets

B) Plain checklist of testable statements

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 5
Include non-functional/maintainer stories (response time, no regressions, OpenAPI and README docs, seed validity)?

A) Yes, as separate stories for the platform maintainer persona

B) No, keep NFRs in requirements.md only

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 6
Traceability and prioritisation.

A) Each story lists the FR/NFR IDs it covers, plus a MoSCoW priority

B) FR/NFR IDs only, no priority

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 7
Should stories include example JSON request/response snippets (e.g. a London 200 response and a 404 body)?

A) Yes, one 200 and one 404 example in the stories

B) No, field definitions in requirements.md are enough

C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Part B: Mandatory Artifacts (Generation Checklist)
- [x] Step 1: Create `aidlc-docs/inception/user-stories/personas.md` (archetypes, needs, usage patterns, mapped to stories)
- [x] Step 2: Create `aidlc-docs/inception/user-stories/stories.md` following INVEST, with acceptance criteria in the chosen format
- [x] Step 3: Map personas to stories; add FR/NFR traceability per Q6
- [x] Step 4: Verify every FR-1..FR-10 and NFR-1..NFR-5 is covered by at least one story
- [x] Step 5: Validate content (Markdown/Mermaid) and update aidlc-state.md and audit.md

## Part C: Breakdown Approaches and Trade-offs
- **User Journey-Based**: follows workflows; good for UIs, weak here (single call).
- **Feature-Based**: maps cleanly to code and tests; less user voice.
- **Persona-Based**: highlights differing consumer needs; risk of duplicate criteria.
- **Domain-Based**: little value for one domain.
- **Epic-Based**: overhead for a small feature.
- **Hybrid** (persona voice + feature criteria): keeps user voice and test mapping; slightly more structure.
