#!/usr/bin/env bash
# Fetch what the nightly compares against: the most recent main-branch nightly's
# results into PREV_DIR, and the kuna build that produced them into BASE_DIR
# (kuna, specs/, LABEL). When that run left no build, the latest release stands in
# as the speed baseline. Needs GH_TOKEN with actions:read.
#
#   scripts/nightly/previous.sh PREV_DIR BASE_DIR
set -euo pipefail

prev_dir=$1
base_dir=$2
repo=${GITHUB_REPOSITORY:?}
mkdir -p "$prev_dir" "$base_dir"

run=$(gh api -X GET "repos/$repo/actions/artifacts" -f name=decbench-nightly -f per_page=100 \
  --jq "[.artifacts[] | select(.expired | not)
         | select(.workflow_run.head_branch == \"main\")
         | select(.workflow_run.id != ${GITHUB_RUN_ID:-0})]
        | sort_by(.created_at) | last | .workflow_run.id // empty")

if [ -n "$run" ]; then
  echo "previous nightly: run $run"
  gh run download "$run" -R "$repo" -n decbench-nightly -D "$prev_dir" || echo "no results in run $run"
  if gh run download "$run" -R "$repo" -n kuna-nightly-build -D "$base_dir"; then
    tar -xzf "$base_dir/specs.tgz" -C "$base_dir"
    chmod +x "$base_dir/kuna"
  fi
else
  echo "no previous nightly on main"
fi

if [ ! -x "$base_dir/kuna" ]; then
  tag=$(gh release view -R "$repo" --json tagName -q .tagName)
  echo "speed baseline: release $tag"
  tmp=$(mktemp -d)
  gh release download "$tag" -R "$repo" -D "$tmp" \
    -p "kuna-$tag-linux-x86_64.tar.gz" -p "kuna-$tag-specs.tar.gz"
  tar -xzf "$tmp/kuna-$tag-linux-x86_64.tar.gz" -C "$tmp"
  cp "$tmp/kuna-$tag-linux-x86_64/kuna" "$base_dir/kuna"
  tar -xzf "$tmp/kuna-$tag-specs.tar.gz" -C "$base_dir"
  echo "release $tag" > "$base_dir/LABEL"
fi
test -d "$base_dir/specs/Ghidra" || { echo "baseline has no spec tree"; exit 1; }
echo "baseline build: $(cat "$base_dir/LABEL")"
