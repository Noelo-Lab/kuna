# kuna top-level build driver (Rust).
#
# The decompiler and the SLEIGH compiler are a Rust port of Ghidra's C++
# originals (see docs/rust-port/README.md); the vendored C++ tree was removed once the
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

.PHONY: all binaries specs test test-stages rust rust-test clean check-spec

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

# Spec honesty gate: docs/spec/ anchors resolve, every phase folder is owned by
# exactly one chapter, and (strict) every settable option is mentioned.
check-spec:
	python3 tools/check_spec.py

# The Rust workspace's own unit/integration tests (the ported TEST() suites, the
# golden differential vectors, the SLEIGH-compiler .sla content-parity tests, ...).
rust-test:
	cd $(ENGINE) && cargo test --workspace --no-fail-fast

clean:
	cd $(ENGINE) && cargo clean
	find $(SPECS) -name '*.sla' -delete
	find $(SPECS) -name '*.sla.lock' -delete
