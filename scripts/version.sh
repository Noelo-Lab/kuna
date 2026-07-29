#!/usr/bin/env bash
# Print the kuna version as MAJOR.MINOR (see docs/release.md).
#
# MAJOR is the top-level VERSION file, edited only by maintainers (CODEOWNERS
# gates it). MINOR is the number of commits since VERSION last changed, so
# every commit bumps it automatically and a MAJOR edit resets it to 0.
# Requires full git history: the minor number is a commit count.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

if [ "$(git rev-parse --is-shallow-repository)" = "true" ]; then
    echo "error: shallow clone -- the minor version is a commit count and needs" >&2
    echo "       full history (git fetch --unshallow, or checkout fetch-depth: 0)" >&2
    exit 1
fi

major="$(tr -d '[:space:]' < VERSION)"
case "$major" in
    ''|*[!0-9]*) echo "error: VERSION must hold a single integer, got '$major'" >&2; exit 1 ;;
esac

last_bump="$(git log -1 --format=%H -- VERSION)"
if [ -z "$last_bump" ]; then
    echo "error: VERSION is not tracked by git yet -- commit it first" >&2
    exit 1
fi

minor="$(git rev-list --count "${last_bump}..HEAD")"
echo "${major}.${minor}"
