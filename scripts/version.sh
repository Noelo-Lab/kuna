#!/usr/bin/env bash
# Print the kuna version as MAJOR.MINOR (see docs/release.md).
#
# MAJOR is the top-level VERSION file, edited only by maintainers (CODEOWNERS
# gates it). MINOR counts releases: a commit already tagged vMAJOR.N is MAJOR.N,
# any other commit is one past the highest vMAJOR.* tag, and a MAJOR with no
# tags yet starts at MAJOR.0. Requires the release tags, so no shallow clones.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

if [ "$(git rev-parse --is-shallow-repository)" = "true" ]; then
    echo "error: shallow clone -- the minor version is read from the release tags" >&2
    echo "       (git fetch --unshallow --tags, or checkout fetch-depth: 0)" >&2
    exit 1
fi

major="$(tr -d '[:space:]' < VERSION)"
case "$major" in
    ''|*[!0-9]*) echo "error: VERSION must hold a single integer, got '$major'" >&2; exit 1 ;;
esac

minors() { sed -n "s/^v${major}\.\([0-9][0-9]*\)$/\1/p" | sort -n; }

here="$(git tag --points-at HEAD | minors | tail -n 1)"
if [ -n "$here" ]; then
    echo "${major}.${here}"
    exit 0
fi

last="$(git tag -l "v${major}.*" | minors | tail -n 1)"
if [ -z "$last" ]; then
    echo "${major}.0"
else
    echo "${major}.$((last + 1))"
fi
