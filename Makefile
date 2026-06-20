# kuna top-level build driver (Rust-only).
#
# The C++ decompiler + SLEIGH compiler have been fully ported to Rust and the
# vendored C++ tree was removed (see docs/RUST_PORT.md). Everything below builds
# and tests the Rust port. The decompiler binaries keep the upstream names
# (decomp_dbg / decomp_test_dbg); the SLEIGH compiler is `slacomp`.

ROOT  := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
SPECS := $(ROOT)/specs
RUSTDIR := $(ROOT)/rust
PROFILE ?= release
BINDIR  := $(RUSTDIR)/target/$(PROFILE)
SLACOMP := $(BINDIR)/slacomp
PYTHON  ?= python3

.PHONY: all binaries specs test test-stages rust rust-test clean

all: binaries specs

# Build the Rust decompiler console binaries (decomp_dbg, decomp_test_dbg) + the
# Rust SLEIGH compiler (slacomp). Cargo manages parallelism + incrementality.
binaries:
	cd $(RUSTDIR) && cargo build --$(PROFILE) -p kuna-console -p kuna-slacomp

rust: binaries

# Compile every vendored .slaspec -> .sla (next to the spec, gitignored) with the
# Rust SLEIGH compiler. End-to-end correctness is `make test` (the Rust-built specs
# decode the corpus to 675/675). `python -m kuna.slacomp --all` byte-diffs the .sla
# element stream against a C++ sleigh_opt oracle (built out-of-tree if you want it).
specs: $(SLACOMP)
	$(SLACOMP) -a $(SPECS)

# Back-compat alias (the port used `specs-rust` while the C++ `specs` still existed).
specs-rust: specs

$(SLACOMP):
	cd $(RUSTDIR) && cargo build --$(PROFILE) -p kuna-slacomp

# Run the datatest harness (the Rust decomp_test_dbg) over the vendored XML
# regression tests, with baseline parity checking. Builds binaries/specs if
# missing. Exit code is nonzero on any failure or baseline regression.
test:
	@test -x $(BINDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(ROOT) && KUNA_ENGINE=rust $(PYTHON) -m kuna.run_tests --datatests --baseline docs/baseline.json

# Same harness over the kuna-owned stage-model issue testcases (tests/stages/).
test-stages:
	@test -x $(BINDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(ROOT) && KUNA_ENGINE=rust $(PYTHON) -m kuna.run_tests --datatests \
	  --datatests-dir tests/stages --baseline docs/baseline-stages.json

# The Rust workspace's own unit/integration tests (the ported TEST() suites, the
# golden differential vectors, the SLEIGH-compiler .sla content-parity tests, ...).
rust-test:
	cd $(RUSTDIR) && cargo test --workspace --no-fail-fast

clean:
	cd $(RUSTDIR) && cargo clean
	find $(SPECS) -name '*.sla' -delete
	find $(SPECS) -name '*.sla.lock' -delete
