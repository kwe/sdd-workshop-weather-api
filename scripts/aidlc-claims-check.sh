#!/usr/bin/env bash
# Compare this feature's claims with every other active worktree's claims.
# Exit 0: no blocking overlaps. Exit 1: blocking overlap. Exit 2: usage error.
set -euo pipefail

root=$(git rev-parse --show-toplevel)
marker="$root/.aidlc-feature"
[ -f "$marker" ] || { echo "ERROR: no .aidlc-feature marker in $root" >&2; exit 2; }
me=$(awk -F': *' '$1=="feature"{print $2}' "$marker" | awk '{print $1}')
mine="$root/aidlc-docs/features/$me/claims.md"
[ -f "$mine" ] || { echo "ERROR: missing $mine" >&2; exit 2; }

# "- file: src/lib.rs (edit)" -> "file: src/lib.rs"
norm() {
  sed -E -n 's/^- (file|route|table|migration|js-module): *([^(]*[^ (]).*/\1: \2/p' "$1" | sort -u
}

blocking=0
shared=0
seen_other=0
while IFS= read -r wt; do
  [ "$wt" = "$root" ] && continue
  for other in "$wt"/aidlc-docs/features/*/claims.md; do
    [ -f "$other" ] || continue
    oid=$(basename "$(dirname "$other")")
    [ "$oid" = "$me" ] && continue
    seen_other=1
    while IFS= read -r line; do
      [ -z "$line" ] && continue
      case "${line%%:*}" in
        route|table|migration) echo "BLOCKING    [$oid] $line"; blocking=1 ;;
        *) echo "SHARED-FILE [$oid] $line"; shared=1 ;;
      esac
    done < <(comm -12 <(norm "$mine") <(norm "$other"))
  done
done < <(git worktree list --porcelain | sed -n 's/^worktree //p')

if [ "$seen_other" = 0 ]; then
  echo "INFO: no other active feature claims found"
elif [ "$blocking" = 1 ]; then
  echo "RESULT: BLOCKING overlap; ask the user before generating code"
elif [ "$shared" = 1 ]; then
  echo "RESULT: shared files only (allowed; keep edits minimal and additive)"
else
  echo "OK: no overlaps"
fi
exit "$blocking"
