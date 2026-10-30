# Parallel Feature Mode

Several AI-DLC agents can work on different features at the same time, each in its own git
worktree and branch. These rules exist so they do not corrupt shared records or each other's code.

## Activation

Parallel Feature Mode is ON when a file named `.aidlc-feature` exists in the workspace root.
It is a per-worktree, git-ignored marker written by the orchestrator:

```text
feature: <feature-id>      # kebab-case, e.g. favourites
branch: feature/<feature-id>
port: <port>               # PORT to use when running the server
base: main                 # branch to rebase onto before the final gate
```

When the marker is absent, the standard single-feature workflow applies unchanged.

When ON, the rules in this file OVERRIDE path and process rules elsewhere in the workflow.

## 1. Per-Feature Records (replaces shared state and audit)

| Standard path | Parallel-mode path |
|---|---|
| `aidlc-docs/aidlc-state.md` | `aidlc-docs/features/<feature-id>/aidlc-state.md` |
| `aidlc-docs/audit.md` | `aidlc-docs/features/<feature-id>/audit.md` |
| `aidlc-docs/inception/...` (new artifacts) | `aidlc-docs/features/<feature-id>/inception/...` |
| `aidlc-docs/construction/...` (new artifacts) | `aidlc-docs/features/<feature-id>/construction/...` |

- Every rule that says "update aidlc-state.md" or "log in audit.md" means the feature-scoped file.
- Only this feature's agent writes inside `aidlc-docs/features/<feature-id>/`. No other writer
  exists, so these files never conflict on merge.
- The first line of the feature state file MUST be `# AI-DLC State: <feature-id>` and it MUST
  contain a line `- **Current Stage**: <stage>` and a line `- **Status**: <in-progress|awaiting-approval|complete>`.
  `scripts/aidlc-index.sh` parses these.

## 2. Shared Documents Are Read-Only

These are owned by the `main` branch and MUST NOT be edited by a feature agent:

- `aidlc-docs/aidlc-state.md`, `aidlc-docs/audit.md`
- `aidlc-docs/inception/**` and `aidlc-docs/construction/**` (artifacts from earlier cycles)
- `aidlc-docs/features/INDEX.md` (generated)
- Any other feature's `aidlc-docs/features/<other-id>/**`

Shared reverse-engineering docs may be stale. **The code is the source of truth.** Skip a full
Reverse Engineering re-run; instead read the current code and, if something differs from the shared
docs, record it in `aidlc-docs/features/<feature-id>/inception/reverse-engineering-delta.md`.

## 3. Claims and Overlap Check (new mandatory step)

Immediately after Workflow Planning is approved, and again before Code Generation starts, the
agent MUST:

1. Write `aidlc-docs/features/<feature-id>/claims.md` using exactly this line format:

   ```text
   - file: <repo-relative path> (create|edit)
   - route: <METHOD> <path>
   - table: <sql table name>
   - migration: <migration file name>
   - js-module: <static/js/... path> (create|edit)
   ```

2. Run `scripts/aidlc-claims-check.sh` from the workspace root and paste its output into the audit
   log entry.
3. Act on the result:
   - **BLOCKING** (same `route`, `table` or `migration` claimed by another active feature): stop and
     ask the user how to resolve it, in a questions file, before generating code.
   - **SHARED-FILE** warnings: allowed, but the design MUST keep edits to those files minimal and
     additive (append a route, append a module, add a section), avoid reformatting or reordering
     existing lines, and list each shared file under a `## Coordination` heading in the code
     generation plan.

## 4. Code Rules That Prevent Merge Pain

- **Migrations**: name new migrations with a UTC timestamp prefix,
  `migrations/YYYYMMDDHHMMSS_<description>.sql`, never the next sequential number. Never edit an
  existing migration.
- **Router (`src/lib.rs`)**: add new routes as a separate `.merge(<feature>::router())` or as
  new `.route(...)` lines appended at the end of the chain. Do not reorder existing routes.
- **Modules**: prefer new files (`src/<feature>.rs`, `static/js/<feature>.js`, `tests/<feature>.rs`)
  over growing shared files. Shared files should only gain a small hook (a `mod` line, an import,
  a single call).
- **README**: add a new `###` subsection; do not rewrite existing sections or tables beyond
  appending rows.
- **No drive-by refactors** of code outside the feature's claims.

## 5. Runtime Isolation

- Run the server with the port from the marker: `PORT=<port> cargo run`. Never use 8080.
- Use a worktree-local database (the default relative `sqlite://weather.db` already is) or a
  temporary one under `/tmp/<feature-id>/`. Never point at another worktree's database.
- Each worktree has its own `target/` directory; do not set `CARGO_TARGET_DIR` to a shared path.

## 6. Path Discipline

- The workspace root is the worktree directory. Use **absolute paths inside the workspace root**
  or plain repo-relative paths. Never use `..` to leave the workspace.
- Never read or write sibling worktrees directly. `scripts/aidlc-claims-check.sh` is the only
  sanctioned way to see other features' claims.
- Temporary files go under `/tmp/<feature-id>/`.

## 7. Git Discipline

- Work only on the marker's `branch`. Commit at each approved gate with a message prefixed
  `[<feature-id>]`. Never push, never commit to `main`, never merge other feature branches.
- Do not commit `.aidlc-feature`, `weather.db*`, or `target/`.

## 8. Lighter Questioning

- Default depth is Minimal or Standard.
- Only ask questions whose answer would change the design. For everything else, write the
  assumption into the questions file as a pre-filled `[Answer]:` marked `(default, assumed)`, so the
  user can override it but does not have to answer it.
- Batch all questions for a stage into one file.

## 9. Build and Test Gate (strengthened)

Before presenting Build and Test completion, the agent MUST:

1. Commit all work on the feature branch.
2. Rebase onto the latest base branch: `git rebase <base>`. Resolve conflicts preserving BOTH
   features' behaviour; never drop another feature's code. Log each conflicted file in the audit.
3. Run, and paste the summary lines of each command into
   `aidlc-docs/features/<feature-id>/construction/build-and-test/build-and-test-summary.md`:
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
   - `cargo test` (full suite, not just the feature's tests)
4. Start the server on the marker port, exercise the new endpoints with `curl`, record the
   responses, then stop the server.
5. Set the feature state `Status` to `awaiting-approval`.

The orchestrator merges approved feature branches into `main` one at a time. After a merge, any
other in-flight feature must repeat step 9 against the new `main` before its own approval.

## 10. Orchestrator Duties (not the feature agent)

- Create worktrees and the `.aidlc-feature` marker.
- Answer gates, merge to `main` sequentially, re-run the full test suite on `main`.
- Regenerate `aidlc-docs/features/INDEX.md` with `scripts/aidlc-index.sh` on `main` after each merge.
