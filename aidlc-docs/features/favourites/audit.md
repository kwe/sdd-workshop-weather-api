# Audit Log: favourites

## Workspace Detection / Initial Request
**Timestamp**: 2026-10-30T13:01:13Z
**User Input**: "Using AI-DLC, add a Favourite Cities feature to this Weather API. This workspace is in Parallel Feature Mode (see .aidlc-feature and .aidlc-rule-details/common/parallel-features.md) - another agent is building a different feature in parallel in another worktree. Request from the product owner: Users want to star cities on the globe so they can get back to them quickly. Persist favourites in SQLite (new table via a new migration). Add GET /api/favourites (list, with each favourite's latest weather), POST /api/favourites (body {\"city\":\"...\"}; 201 on create, 200 if already a favourite, 404 if the city has no weather records, 400 on invalid input), and DELETE /api/favourites/{city} (204, or 404 if not a favourite). Document them in the OpenAPI spec. In the globe UI, add a star toggle in the city detail panel and a Favourites section that lists starred cities and flies to them when clicked; favourite pins should look distinct. Keep the existing XSS rules (textContent only) and CSP. Add API and frontend tests. Begin the workflow."
**AI Response**: Parallel Feature Mode active: feature=favourites, branch=feature/favourites, port=8081, base=main. Brownfield; reverse engineering skipped in favour of a delta note. Extensions scanned (opt-in files only): security, resiliency, property-based testing.
**Context**: Workspace Detection complete; proceeding to Requirements Analysis.

---

## Correction from user (path discipline)
**Timestamp**: 2026-10-30T13:00:21Z
**User Input**: "I rejected that command because it used a relative path (../..) that resolved OUTSIDE your worktree, to /Users/evanke02/Github/course/weather-api-worktrees/aidlc-docs. Per parallel-features.md section 6, always use absolute paths under /Users/evanke02/Github/course/weather-api-worktrees/favourites (e.g. .../favourites/aidlc-docs/aidlc-state.md). Re-run the read with absolute paths and continue the workflow."
**AI Response**: Acknowledged; all subsequent file access uses absolute paths under the worktree root. Shared (read-only) aidlc-docs files were only read, never written.
**Context**: Process correction.

---

## Requirements Analysis - Questions issued
**Timestamp**: 2026-10-30T13:02:00Z
**AI Response**: Created inception/requirements/requirement-verification-questions.md (Standard depth, all questions pre-filled with assumed defaults per parallel-features.md §8). Awaiting user answers or "accept defaults".
**Context**: Requirements Analysis, Step 6 gate.

---

## Requirements Analysis - Answers received
**Timestamp**: 2026-10-30T13:02:00Z
**User Input**: "Defaults accepted with two changes: (1) Tests: no node dependency - keep frontend tests in Rust like the existing tests/frontend.rs (served-asset checks, forbidden-token scan); API tests in a new tests/favourites.rs. (2) POST validation: trim city; reject empty or > 100 characters with 400. Proceed to write requirements, then Workflow Planning."
**AI Response**: Recorded Q9=B override and trim/length rule; wrote inception/requirements/requirements.md; wrote inception/plans/execution-plan.md (User Stories, Application Design, Units, Functional/NFR/Infra design proposed as SKIP). Extensions all opted out.
**Context**: Requirements Analysis complete; Workflow Planning approval requested.

---

## Workflow Planning - Approval prompt
**Timestamp**: 2026-10-30T13:03:00Z
**AI Response**: Presented execution plan; awaiting explicit approval before writing claims.md and running scripts/aidlc-claims-check.sh.
**Context**: Workflow Planning gate.

---

## Workflow Planning - Approval
**Timestamp**: 2026-10-30T13:03:02Z
**User Input**: "Approve & Continue."
**AI Response**: Wrote claims.md and ran scripts/aidlc-claims-check.sh. Output:
```
SHARED-FILE [weather-alerts] file: README.md
SHARED-FILE [weather-alerts] file: src/frontend.rs
SHARED-FILE [weather-alerts] file: src/lib.rs
SHARED-FILE [weather-alerts] file: static/css/app.css
SHARED-FILE [weather-alerts] file: static/index.html
RESULT: shared files only (allowed; keep edits minimal and additive)
```
No BLOCKING overlap (routes/table/migration unique). Wrote construction/plans/favourites-code-generation-plan.md with a Coordination section listing the 5 shared files.
**Context**: Code Generation Part 1 (planning); approval requested.

---

## Code Generation Plan - Approval (with orchestrator updates)
**Timestamp**: 2026-10-30T13:04:40Z
**User Input**: "Orchestrator: the claims-check script had a bug (js-module vs file claims of the same path were not compared). Fixed on main as commit 188dbaf. Run: git cherry-pick 188dbaf, then re-run scripts/aidlc-claims-check.sh - it now reports 10 shared files incl. api.js, state.js, main.js, ui.js, globe-scene.js. Update the Coordination section of your code generation plan to cover those JS files too (small additive hooks only). Coordination decision: weather-alerts owns the globe ring layer (ringsData); do NOT use it - your 'gold ring pin' must be pure CSS on the HTML pin element, which is fine. With those updates the plan is approved: log this, then generate the code."
**AI Response**: Cherry-picked 188dbaf (now c85c614 on feature branch; scripts/aidlc-claims-check.sh only). Re-ran claims check:
```
SHARED-FILE [weather-alerts] file: README.md
SHARED-FILE [weather-alerts] file: src/frontend.rs
SHARED-FILE [weather-alerts] file: src/lib.rs
SHARED-FILE [weather-alerts] file: static/css/app.css
SHARED-FILE [weather-alerts] file: static/index.html
SHARED-FILE [weather-alerts] file: static/js/api.js
SHARED-FILE [weather-alerts] file: static/js/globe-scene.js
SHARED-FILE [weather-alerts] file: static/js/main.js
SHARED-FILE [weather-alerts] file: static/js/state.js
SHARED-FILE [weather-alerts] file: static/js/ui.js
RESULT: shared files only (allowed; keep edits minimal and additive)
```
No BLOCKING. Coordination section updated (10 shared files; no ringsData use; gold ring is pure CSS).
**Context**: Code Generation plan approved; starting Part 2 (generation).

---

## Code Generation - Part 2 complete
**Timestamp**: 2026-10-30T13:15:00Z
**AI Response**: Generated all 16 plan steps (see construction/favourites/code/code-summary.md). Shared-file edits kept additive; ringsData not used. Local checks: cargo fmt --check OK; clippy -D warnings OK; cargo test all pass (favourites: 8 tests; frontend: +1). Approval requested before Build and Test (rebase on main + curl gate).
**Context**: Code Generation gate.

---

## Code Generation - Approval
**Timestamp**: 2026-10-30T13:09:24Z
**User Input**: "Code reviewed - approved. Continue to the Build and Test gate exactly per parallel-features.md section 9 (commit, rebase onto main, fmt, clippy -D warnings, full cargo test, curl on PORT=8081, then stop the server)."
**Context**: Code Generation approved.

---

## Build and Test gate
**Timestamp**: 2026-10-30T13:11:00Z
**AI Response**: Work already committed; `git rebase main` succeeded with no conflicts (skipped already-applied cherry-pick c85c614). fmt OK; clippy -D warnings OK; full cargo test: 4 + 17 + 8 + 7 passed, 0 failed. Server run on PORT=8081 with /tmp/favourites/w.db, all endpoints exercised with curl, server stopped, port closed. Details in construction/build-and-test/build-and-test-summary.md. Status set to awaiting-approval.
**Context**: Build and Test gate; approval requested for merge.

---

## Build and Test - Approval
**Timestamp**: 2026-10-30T13:12:23Z
**User Input**: "Approved. Orchestrator verified independently (fmt, clippy, 36 tests pass, no forbidden DOM sinks). Log the approval, set Status: complete and Current Stage: Complete (merged by orchestrator) in your feature state file, commit on your branch, then stop - do not merge."
**AI Response**: Set feature state to Status: complete, Current Stage: Complete (merged by orchestrator). Committed on feature/favourites; not merged or pushed.
**Context**: Workflow complete (Operations is a placeholder).

---

## Post-merge defect: literal "null" on non-favourite pins
**Timestamp**: 2026-10-30T14:00:00Z
**User Input**: "Orchestrator bug report (found in a real-browser check after merge): every NON-favourite pin on the globe shows the literal text 'null'. Cause: static/js/globe-scene.js updatePin passes `favourite ? el(...) : null` into node.append(); DOM append() stringifies null into a 'null' text node. Please: (1) git rebase main (your branch is already merged; main has moved), (2) fix it so nothing is appended for non-favourites, (3) add a regression test in tests/frontend.rs that would have failed before the fix (e.g. assert no '? ... : null' argument inside an append( call in static/js, or a narrower check you justify), (4) log it in your feature audit as a post-merge defect with root cause and why your tests missed it, (5) run fmt/clippy/full cargo test, commit on feature/favourites as [favourites] Fix ..., set Status awaiting-approval, and stop."
**Root cause**: `updatePin` passed `favourite ? el(...) : null` to native `Node.append()`, which stringifies null into a "null" text node. The project's `el()` helper skips null children, and I wrongly assumed the same for `append()`.
**Fix**: The star badge is now appended in a separate `if (favourite) node.append(...)`; nothing is appended for non-favourites (CSS `order` keeps the layout).
**Regression test**: `append_calls_never_receive_nullish_conditional_arguments` in tests/frontend.rs scans every embedded .js asset for `.append(` arguments at top level containing `? ... : null/undefined/false`, `&&` or `|| null`. Nested `el()` children are ignored since `el()` handles them. `append_scanner_detects_nullish_arguments` checks the scanner on good and bad samples. Verified the test fails on the pre-fix file and passes on the fix.
**Why tests missed it**: There is no JS runtime or DOM in the test setup (Rust-only, by decision), so frontend tests only check served assets, forbidden tokens and string presence. The star test asserted that code existed, not rendered output. Code review reasoned from `el()` semantics and nobody rendered the page in a browser. Residual risk: behaviour-level UI bugs remain untestable without a DOM; a manual browser check should be part of the gate.
**Verification**: fmt OK, clippy -D warnings OK, cargo test all pass (4 + 17 + 8 + 9).
**Context**: Post-merge defect fix; status awaiting-approval.

---
