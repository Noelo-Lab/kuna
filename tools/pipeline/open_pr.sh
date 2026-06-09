#!/usr/bin/env bash
# Open (or stage) a PR for the current feature branch, robustly.
#
#   tools/pipeline/open_pr.sh <branch> <title> <body-file>
#
# Pushes <branch> over SSH (always works here), then tries `gh pr create`. If gh cannot
# create the PR (e.g. the gh token lacks access to the org repo, while the SSH key can
# push), it falls back to printing a ready-to-click compare URL. Emits the resulting URL
# on the LAST stdout line (PR URL or compare URL) so the caller can record it.
set -uo pipefail

BRANCH="${1:?need branch}"
TITLE="${2:?need title}"
BODY_FILE="${3:?need body file}"
REPO_SLUG="${PIPELINE_REPO:-Noelo-Lab/kuna}"
BASE="${BASE_BRANCH:-main}"

# 1. push the branch (SSH; pre-authorized)
git push origin "HEAD:refs/heads/$BRANCH" 1>&2 || { echo "ERROR: push failed" 1>&2; exit 1; }

# 2. try gh pr create
if command -v gh >/dev/null 2>&1; then
  URL="$(gh pr create --repo "$REPO_SLUG" --base "$BASE" --head "$BRANCH" \
           --title "$TITLE" --body-file "$BODY_FILE" 2>/tmp/_ghpr.err)" && {
    echo "opened PR: $URL" 1>&2
    echo "$URL"
    exit 0
  }
  echo "gh pr create failed: $(cat /tmp/_ghpr.err 2>/dev/null | head -2)" 1>&2
fi

# 3. fallback: compare URL the human can click to open the PR
CMP="https://github.com/$REPO_SLUG/compare/$BASE...$BRANCH?expand=1"
echo "PR not auto-created (gh token lacks repo access). Branch pushed; open via:" 1>&2
echo "  $CMP" 1>&2
echo "$CMP"
exit 0
