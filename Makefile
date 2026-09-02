# kuna top-level build driver (Rust).
#
# The decompiler and the SLEIGH compiler are a Rust port of Ghidra's C++
# originals (see docs/history.md); the vendored C++ tree was removed once the
# port reached parity. Everything below builds and tests the Rust engine under
# decompiler/. The decompiler binaries keep the upstream names
# (decomp_dbg / decomp_test_dbg); the SLEIGH compiler is `slacomp`.

ROOT   := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
SPECS  := $(ROOT)/specs
ENGINE := $(ROOT)/decompiler
PROFILE ?= release
BINDIR  := $(ENGINE)/target/$(PROFILE)
SLACOMP := $(BINDIR)/slacomp
PYTHON  ?= python3
# Where `test-ghidra` tees its output for the two false-green canaries below
# (inside target/, which is gitignored, so a failed run leaves no repo litter).
GHIDRA_SIM_LOG := $(ENGINE)/target/ghidra-sim.log

.PHONY: all binaries specs test test-stages test-cli test-ghidra rust rust-test clean check-spec version

all: binaries specs

# Build the decompiler console binaries (decomp_dbg, decomp_test_dbg), the
# SLEIGH compiler (slacomp), and the user-facing `kuna` CLI (kuna-cli).
# Cargo manages parallelism + incrementality. (decomp_test_dbg lives in
# kuna-harness; kuna-cli pulls it in as a sibling at build time.)
binaries:
	cd $(ENGINE) && cargo build --$(PROFILE) -p kuna-console -p kuna-harness -p kuna-slacomp -p kuna-cli

rust: binaries

# Compile every vendored .slaspec -> .sla (next to the spec, gitignored) with the
# SLEIGH compiler. End-to-end correctness is `make test` (the built specs decode
# the corpus to 675/675).
specs: $(SLACOMP)
	$(SLACOMP) -a $(SPECS)

$(SLACOMP):
	cd $(ENGINE) && cargo build --$(PROFILE) -p kuna-slacomp

# Run the datatest harness (decomp_test_dbg) over the vendored XML regression
# tests, with baseline parity checking. Builds binaries/specs if missing. Driven
# by the `kuna` CLI; exit code is nonzero on any failure or baseline regression.
test: $(BINDIR)/kuna
	@test -x $(BINDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(ROOT) && $(BINDIR)/kuna test --datatests --baseline docs/baseline.json

# Same harness over the kuna-owned stage-model issue testcases (tests/stages/).
test-stages: $(BINDIR)/kuna
	@test -x $(BINDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(ROOT) && $(BINDIR)/kuna test --datatests \
	  --datatests-dir tests/stages --baseline docs/baseline-stages.json

$(BINDIR)/kuna:
	cd $(ENGINE) && cargo build --$(PROFILE) -p kuna-cli

# The ghidra-mode differential harness (kuna-ghidra's ghidra-sim tests,
# docs/ghidra-integration.md §11): drives the decompile-process wire protocol
# in-process against real vendored ELFs and pins the GUI-path quality numbers.
# Release profile (the dev profile costs minutes for the same answer);
# --include-ignored picks up the heavier sort/grep breadth test.
#
# TWO canaries, because this target has twice been a FALSE GREEN while CI's
# equivalent step was red:
#   * the skip canary (the same grep CI runs): with the `.sla` specs missing or
#     unusable every ghidra-sim test prints a skip notice and returns early --
#     by design, so a specs-less checkout is a visible skip rather than a false
#     failure.  A green `make test-ghidra` that skipped everything proves
#     nothing.  Worktrees hit this constantly: `KUNA_SPECS`/`SLEIGHHOME` do not
#     reach the cargo suites (AGENTS.md), so the harness resolves <repo>/specs
#     relative to its own crate and finds nothing.
#   * the breadth canary: `ghidra_sim_sort_grep_breadth` is `#[ignore]`d (it is
#     the heavy one), so ANY run that loses `--include-ignored` reports it
#     "ignored" and still exits 0 -- which is exactly how a broken
#     `<vardecl symref>` on the sort/grep fixtures reached CI green-locally.
#     Demand the line that proves it actually ran.
test-ghidra:
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	@bash -c 'set -o pipefail; cd $(ENGINE) && \
	  cargo test --$(PROFILE) -p kuna-ghidra -- --include-ignored --nocapture \
	    2>&1 | tee $(GHIDRA_SIM_LOG)'
	@if grep -qE 'skipping \(.*(\.sla|make specs|specs tree)' $(GHIDRA_SIM_LOG); then \
	  echo "ERROR: ghidra-sim tests skipped for missing/unusable SLEIGH specs -- false green"; \
	  exit 1; \
	fi
	@grep -q 'test ghidra_sim_sort_grep_breadth \.\.\. ok' $(GHIDRA_SIM_LOG) || { \
	  echo "ERROR: the sort/grep breadth test did not RUN (needs --include-ignored) -- false green"; \
	  exit 1; \
	}

# Spec honesty gate: docs/spec/ anchors resolve, every phase folder is owned by
# exactly one chapter, and (strict) every settable option is mentioned.
check-spec:
	python3 tools/check_spec.py

# The RE-friction loop's regression corpus: every gap it closes leaves the acceptance probe
# that proved the fix in tests/cli/ (docs/re-pipeline.md sec.3). Without a runner they are
# inert -- a regression would be found a round later by a tester instead of a minute later
# here. Every case is `in-repo` targeted, so this needs no dataset. An empty corpus passes.
test-cli:
	PYTHONPATH=$(ROOT) python3 -m scripts.repipe.clitests

# Print the repo version as MAJOR.MINOR (VERSION file + commit count -- the
# scheme release CI tags with; see docs/release.md).
version:
	@bash scripts/version.sh

# The Rust workspace's own unit/integration tests (the ported TEST() suites, the
# golden differential vectors, the SLEIGH-compiler .sla content-parity tests, ...).
rust-test:
	cd $(ENGINE) && cargo test --workspace --no-fail-fast

clean:
	cd $(ENGINE) && cargo clean
	find $(SPECS) -name '*.sla' -delete
	find $(SPECS) -name '*.sla.lock' -delete
