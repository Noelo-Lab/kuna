#!/usr/bin/env bash
# Create a kuna worktree branched off main, with the main tree's built .sla specs
# symlinked into the worktree's gitignored specs/ (never `make specs` in a worktree).
#
#   scripts/wt_setup.sh <branch-name> [worktree-path]
#
# Prints the worktree path on success.
set -euo pipefail

MAIN_TREE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BRANCH="${1:?usage: wt_setup.sh <branch-name> [worktree-path]}"
SLUG="${BRANCH##*/}"
WT="${2:-${MAIN_TREE}-wt-${SLUG}}"

if [ -e "$WT" ]; then
    echo "wt_setup: $WT already exists" >&2
    exit 1
fi

git -C "$MAIN_TREE" worktree add -b "$BRANCH" "$WT" main >&2

n=0
while IFS= read -r sla; do
    rel="${sla#"$MAIN_TREE"/}"
    dst="$WT/$rel"
    mkdir -p "$(dirname "$dst")"
    ln -sf "$sla" "$dst"
    n=$((n + 1))
done < <(find "$MAIN_TREE/specs" -name '*.sla')
echo "wt_setup: symlinked $n .sla files into $WT/specs" >&2

echo "$WT"
