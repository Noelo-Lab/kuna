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
NJOBS  ?= $(shell nproc)

# Optional local libbfd prefix (a dir containing usr/include/bfd.h and
# usr/lib/<triplet>/libbfd.so), e.g. produced by tools/fetch_bfd.sh.
BFD_PREFIX ?=
ifeq ($(BFD_PREFIX),)
  BFD_OVERRIDE :=
else
  BFD_TRIPLET := $(shell gcc -dumpmachine)
  BFD_INC     := $(BFD_PREFIX)/usr/include
  BFD_LIBDIR  := $(BFD_PREFIX)/usr/lib/$(BFD_TRIPLET)
  # The embedded quotes are literal: make passes them through verbatim and the
  # recipe shell groups the space-bearing BFDLIB value into a single argument.
  BFD_OVERRIDE := ADDITIONAL_FLAGS="-I$(BFD_INC)" BFDLIB="-L$(BFD_LIBDIR) -Wl,-rpath,$(BFD_LIBDIR) -lbfd"
endif

.PHONY: all binaries sleigh specs test clean check-deps touch-generated

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
endif

# Defensive fix for the bison/flex timestamp pitfall: ensure every committed
# generated parser/lexer output is newer than its .y/.l source so the upstream
# Makefile never invokes bison/flex (which may not be installed).
touch-generated:
	@cd $(CPPDIR) && touch *.y *.l \
	  && touch grammar.cc xml.cc pcodeparse.cc slghparse.cc slghparse.hh slghscan.cc

# The sleigh compiler alone (needs no libbfd) -- used to build .sla spec files.
sleigh: touch-generated
	$(MAKE) -C $(CPPDIR) -j$(NJOBS) sleigh_opt

binaries: check-deps touch-generated
	$(MAKE) -C $(CPPDIR) -j$(NJOBS) sleigh_opt
	$(MAKE) -C $(CPPDIR) -j$(NJOBS) $(BFD_OVERRIDE) decomp_dbg
	$(MAKE) -C $(CPPDIR) -j$(NJOBS) $(BFD_OVERRIDE) decomp_opt
	$(MAKE) -C $(CPPDIR) -j$(NJOBS) $(BFD_OVERRIDE) decomp_test_dbg

# Compile every vendored .slaspec -> .sla (written next to the spec, gitignored).
specs: $(SLEIGH)
	$(SLEIGH) -a $(SPECS)

$(SLEIGH):
	$(MAKE) sleigh

# Run the upstream test harness: 204 C++ unit tests + 83 XML datatests.
# Explicit -sleighpath/-path (never SLEIGHHOME) for reproducibility.
# Exit code = number of failed tests.
test:
	cd $(CPPDIR) && ./decomp_test_dbg -sleighpath $(SPECS) -path $(ROOT)/decompiler/datatests

clean:
	-$(MAKE) -C $(CPPDIR) reallyclean
	find $(SPECS) -name '*.sla' -delete
	find $(SPECS) -name '*.sla.lock' -delete
