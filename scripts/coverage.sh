#!/usr/bin/env bash
# Merged line coverage for kuna: the cargo workspace suite plus the datatest,
# stages and tests/cli corpora, all through instrumented binaries, rolled up
# into observatory/data/coverage.json + an lcov/HTML report. Heavy; run via
# `make coverage`. See docs/observatory.md for the knobs and caveats.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/observatory}"
cd "$ROOT/decompiler"

command -v cargo-llvm-cov >/dev/null 2>&1 || {
  echo "coverage: cargo-llvm-cov not found; install with:" >&2
  echo "  rustup component add llvm-tools-preview && cargo install cargo-llvm-cov" >&2
  exit 1
}
rustup component list --installed 2>/dev/null | grep -q llvm-tools || {
  echo "coverage: llvm-tools missing; run: rustup component add llvm-tools-preview" >&2
  exit 1
}
find "$ROOT/specs" -name '*.sla' -print -quit | grep -q . || {
  echo "coverage: no built specs; run: make specs" >&2
  exit 1
}

# Pre-set the target dir BEFORE show-env: in show-env mode cargo-llvm-cov
# otherwise instruments into the normal decompiler/target, and its clean
# step would wipe the release binaries (observed, not hypothetical).
export CARGO_TARGET_DIR="$ROOT/decompiler/target/llvm-cov"
eval "$(cargo llvm-cov show-env --sh)"
export CARGO_INCREMENTAL=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_RELEASE_DEBUG=0
cargo llvm-cov clean --workspace
TARGET="$CARGO_LLVM_COV_TARGET_DIR"
# Pool-merged profile files: the suites spawn kuna/decomp_dbg thousands of
# times and per-pid patterns would explode the file count. They must sit
# directly in $TARGET -- `cargo llvm-cov report` does not scan subdirs.
export LLVM_PROFILE_FILE="$TARGET/kuna-%16m.profraw"

fail=0
echo "== coverage 1/4: cargo workspace suite =="
cargo test --workspace --no-fail-fast || fail=1

echo "== coverage 2/4..3/4: datatest + stages corpora =="
cargo build --release -p kuna-cli -p kuna-harness -p kuna-console
KUNA="$TARGET/release/kuna"
export KUNA_DECOMP_TEST="$TARGET/release/decomp_test_dbg"
(cd "$ROOT" && "$KUNA" test --datatests --baseline docs/baseline.json) || fail=1
(cd "$ROOT" && "$KUNA" test --datatests --datatests-dir tests/stages \
  --baseline docs/baseline-stages.json) || fail=1

echo "== coverage 4/4: tests/cli probes =="
(cd "$ROOT" && REPIPE_KUNA="$KUNA" PYTHONPATH="$ROOT" python3 -m scripts.repipe.clitests) || fail=1

echo "== coverage: reports =="
mkdir -p "$OUT/coverage"
cargo llvm-cov report --json --summary-only > "$TARGET/coverage-summary.json"
cargo llvm-cov report --lcov --output-path "$OUT/coverage/lcov.info"
cargo llvm-cov report --html --output-dir "$OUT/coverage"
PYTHONPATH="$ROOT" python3 -m scripts.observatory.coverage \
  --summary "$TARGET/coverage-summary.json" --runs-failed "$fail" --out "$OUT"
if [ "$fail" != 0 ]; then
  echo "coverage: some suite runs failed; the report covers what ran" >&2
fi
