# kuna top-level build driver.
#
# Wraps the UNTOUCHED upstream decompiler/cpp/Makefile (see UPSTREAM.md for the
# zero-modification policy). All kuna-specific build logic lives here.
#
# NOTE: the upstream Makefile selects its dependency files via
# `ifeq ($(MAKECMDGOALS),<single-goal>)`, so each binary MUST be built in its
# own make invocation (never `make decomp_dbg decomp_test_dbg` in one call).
#
# libbfd: decomp_dbg / decomp_opt / decomp_test_dbg link against libbfd. Normally
# that comes from the `binutils-dev` system package. If you cannot install it (no
# root), run `tools/fetch_bfd.sh` to fetch it into ./.bfdlocal and then build with
#   make BFD_PREFIX=$(pwd)/.bfdlocal
# The override is injected into the upstream build via ADDITIONAL_FLAGS (present in
# every upstream compile rule) and BFDLIB (the upstream link flag) -- the upstream
# Makefile itself is never edited.

ROOT   := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
CPPDIR := $(ROOT)/decompiler/cpp
SPECS  := $(ROOT)/specs
SLEIGH := $(CPPDIR)/sleigh_opt
NJOBS  ?= $(shell nproc 2>/dev/null || echo 1)

# Parallelism for the inner (upstream) sub-makes. When this Makefile already runs
# under a parent jobserver (`make -jN ...`), pass nothing so the sub-makes share
# the parent's job budget instead of forcing their own -j (which would print
# "warning: -jN forced in submake" and over-subscribe). Recursively expanded on
# purpose: MAKEFLAGS only contains --jobserver-auth at recipe time.
# (For a fully serial build run `NJOBS=1 make`; a bare `make -j1` cannot be
# distinguished from a serial `make` here.)
SUBJOBS = $(if $(findstring jobserver,$(MAKEFLAGS)),,-j$(NJOBS))

# Optional local libbfd prefix (a dir containing usr/include/bfd.h and a
# libbfd.so somewhere under usr/lib/, e.g. usr/lib/x86_64-linux-gnu/), as
# produced by tools/fetch_bfd.sh. The lib dir is located by searching for the
# actual libbfd.so rather than guessing a triplet: `gcc -dumpmachine` does not
# always match the dpkg multiarch directory the .debs use.
BFD_PREFIX ?=
ifeq ($(BFD_PREFIX),)
  BFD_OVERRIDE :=
else
  BFD_INC    := $(BFD_PREFIX)/usr/include
  BFD_SO     := $(firstword $(wildcard $(BFD_PREFIX)/usr/lib/*/libbfd.so) $(wildcard $(BFD_PREFIX)/usr/lib/libbfd.so))
  BFD_LIBDIR := $(patsubst %/,%,$(dir $(BFD_SO)))
  # The embedded quotes are literal: make passes them through verbatim and the
  # recipe shell groups the space-bearing BFDLIB value into a single argument.
  BFD_OVERRIDE := ADDITIONAL_FLAGS="-I$(BFD_INC)" BFDLIB="-L$(BFD_LIBDIR) -Wl,-rpath,$(BFD_LIBDIR) -lbfd"
endif

.PHONY: all binaries sleigh specs test test-stages clean check-deps touch-generated

# This wrapper orchestrates SERIAL sub-makes (each upstream binary must be built
# in its own invocation; `binaries` and `specs` both produce sleigh_opt, so
# running them concurrently would race). Parallelism happens INSIDE each
# sub-make via SUBJOBS / an inherited jobserver.
.NOTPARALLEL:

all: binaries specs

check-deps:
	@command -v g++ >/dev/null || { echo "ERROR: need g++"; exit 1; }
	@echo 'int main(){return 0;}' | g++ -x c++ - -lz -o /dev/null 2>/dev/null \
	  || { echo "ERROR: libz missing -> sudo apt-get install -y zlib1g-dev"; exit 1; }
ifeq ($(BFD_PREFIX),)
	@echo 'int main(){return 0;}' | g++ -x c++ - -lbfd -o /dev/null 2>/dev/null \
	  || { echo "ERROR: libbfd missing -> sudo apt-get install -y binutils-dev"; \
	       echo "       (or, without root: tools/fetch_bfd.sh && make BFD_PREFIX=$(ROOT)/.bfdlocal)"; \
	       exit 1; }
else
	@test -f "$(BFD_INC)/bfd.h" \
	  || { echo "ERROR: no bfd.h under BFD_PREFIX=$(BFD_PREFIX) (run tools/fetch_bfd.sh)"; exit 1; }
	@test -n "$(BFD_SO)" \
	  || { echo "ERROR: no libbfd.so under $(BFD_PREFIX)/usr/lib (run tools/fetch_bfd.sh)"; exit 1; }
endif

# Defensive fix for the bison/flex timestamp pitfall: ensure every committed
# generated parser/lexer output is newer than its .y/.l source so the upstream
# Makefile never invokes bison/flex (which may not be installed).
touch-generated:
	@cd $(CPPDIR) && touch *.y *.l \
	  && touch grammar.cc xml.cc pcodeparse.cc slghparse.cc slghparse.hh slghscan.cc

# The sleigh compiler alone (needs no libbfd) -- used to build .sla spec files.
sleigh: touch-generated
	$(MAKE) -C $(CPPDIR) $(SUBJOBS) sleigh_opt

binaries: check-deps touch-generated
	$(MAKE) -C $(CPPDIR) $(SUBJOBS) sleigh_opt
	$(MAKE) -C $(CPPDIR) $(SUBJOBS) $(BFD_OVERRIDE) decomp_dbg
	$(MAKE) -C $(CPPDIR) $(SUBJOBS) $(BFD_OVERRIDE) decomp_opt
	$(MAKE) -C $(CPPDIR) $(SUBJOBS) $(BFD_OVERRIDE) decomp_test_dbg

# Compile every vendored .slaspec -> .sla (written next to the spec, gitignored).
specs: $(SLEIGH)
	$(SLEIGH) -a $(SPECS)

$(SLEIGH):
	$(MAKE) sleigh

# Run the upstream test harness: 204 C++ unit tests + 83 XML datatests.
# Explicit -sleighpath/-path (never SLEIGHHOME) for reproducibility.
# Exit code = number of failed tests.
# Builds the harness/specs first ONLY if missing (a full `make specs` recompiles
# all 148 slaspecs, so it is not an unconditional prerequisite here).
test:
	@test -x $(CPPDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(CPPDIR) && ./decomp_test_dbg -sleighpath $(SPECS) -path $(ROOT)/decompiler/datatests

# Run the kuna-owned stage-model issue testcases (tests/stages/, see its README).
# Same harness, separate directory: the upstream baseline is never affected.
test-stages:
	@test -x $(CPPDIR)/decomp_test_dbg || $(MAKE) binaries
	@test -n "$$(find $(SPECS) -name '*.sla' -print -quit)" || $(MAKE) specs
	cd $(CPPDIR) && ./decomp_test_dbg -sleighpath $(SPECS) -path $(ROOT)/tests/stages datatests

clean:
	-$(MAKE) -C $(CPPDIR) reallyclean
	find $(SPECS) -name '*.sla' -delete
	find $(SPECS) -name '*.sla.lock' -delete
