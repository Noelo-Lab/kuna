#!/usr/bin/env bash
# Fetch the benchmark dataset into $JAVABENCH_WORKSPACE/dataset/<project>/{classes.jar,sources/,deps/,meta.json}
# and the tools (coursier 2.1.26, Temurin JDK 25.0.5, Vineflower 1.12.0) into $JAVABENCH_WORKSPACE/tools/.
# Every download is verified against its pinned SHA-256 (pins/<project>.json, projects.py, fetch.py).
# Usage: JAVABENCH_WORKSPACE=DIR scripts/javabench/fetch.sh [--project NAME ...] [--force]
set -euo pipefail
ROOT=$(cd "$(dirname "$(readlink -f "$0")")/../.." && pwd)
PYTHONPATH="$ROOT${PYTHONPATH:+:$PYTHONPATH}" exec python3 -m scripts.javabench fetch "$@"
