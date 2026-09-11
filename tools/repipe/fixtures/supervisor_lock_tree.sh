#!/usr/bin/env bash
# A foreground shell -> shell -> shell tree used to exercise inherited supervisor ownership.
set -uo pipefail

STATE="$1"
ROLE="$2"

case "$ROLE" in
  owner)
    echo "$$" > "$STATE/owner.pid"
    trap 'touch "$STATE/owner-term"; exit 0' TERM
    bash "$0" "$STATE" child
    touch "$STATE/owner-child-returned"
    ;;
  child)
    echo "$$" > "$STATE/child.pid"
    bash "$0" "$STATE" grandchild
    touch "$STATE/child-returned"
    ;;
  grandchild)
    echo "$$" > "$STATE/grandchild.pid"
    touch "$STATE/tree-ready"
    while [ ! -e "$STATE/release-tree" ]; do sleep 0.01; done
    touch "$STATE/grandchild-released"
    ;;
  *)
    exit 2
    ;;
esac
