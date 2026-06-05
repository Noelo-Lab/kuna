# kuna

A standalone extraction of [Ghidra](https://github.com/NationalSecurityAgency/ghidra)'s
C++ decompiler (the "deep decompiler", often referred to alongside SLEIGH), broken out of
the Ghidra Java application so the core decompilation pipeline can be studied, instrumented,
and refined stage-by-stage (see `STAGES.md` / `STAGE_MAPPING.md`).

## Layout

| Path | What it is |
|---|---|
| `decompiler/cpp/` | Upstream C++ decompiler source, **byte-identical** to Ghidra (never edit; see `UPSTREAM.md`) |
| `decompiler/unittests/` | Upstream C++ unit tests (204 tests) |
| `decompiler/datatests/` | Upstream XML decompilation regression tests (83 files) |
| `specs/Ghidra/Processors/` | Vendored SLEIGH processor specs (all upstream modules); `.sla` are built artifacts |
| `kuna/` | Python package: test runner + decompiler wrapper |
| `tools/sync_upstream.py` | Ports upstream Ghidra changes into this repo |
| `Makefile` | Top-level build driver (wraps the untouched upstream `decompiler/cpp/Makefile`) |

## Prerequisites

```bash
sudo apt-get install -y binutils-dev zlib1g-dev   # libbfd + libz (required)
sudo apt-get install -y bison flex                # only needed if you edit a .y/.l grammar
```

No root? Fetch libbfd into a local prefix and point the build at it (the SLEIGH
compiler and all `.sla` build fine without libbfd; only the console/test binaries need it):

```bash
./tools/fetch_bfd.sh                      # downloads libbfd into ./.bfdlocal (no root)
make BFD_PREFIX="$(pwd)/.bfdlocal" all    # injected via upstream flags; upstream Makefile untouched
```

## Build

```bash
make            # = binaries + specs
make binaries   # decomp_opt, decomp_dbg, sleigh_opt, decomp_test_dbg
make specs      # compile all .slaspec -> .sla with the freshly built sleigh compiler
make test       # run the upstream test harness (unit tests + datatests)
```

## Python usage

```bash
workon kuna     # ~/.virtualenvs/kuna
pip install -e .

# Run the decompiler test suite with parity checking
python -m kuna.run_tests --baseline docs/baseline.json

# Decompile a function from a real binary
python -m kuna.decompile ./a.out main
python -m kuna.decompile ./stripped.bin 0x401040 --addr
```

## Understanding the decompiler

- `STAGES.md` — the 19 speculated decompiler stages (3 phases).
- `STAGE_MAPPING.md` — every one of the 115 C++ source files mapped to a stage (or to
  infrastructure), anchored to the real pass pipeline (`ActionDatabase::universalAction`).

## Provenance

Vendored from Ghidra commit `cef869af04c4740a71ad31a55704045b1b0d1644`.
See `UPSTREAM.md` for the exact path map and the procedure for porting upstream changes
(`tools/sync_upstream.py`).
