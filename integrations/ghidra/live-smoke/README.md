# Live-smoke: kuna_ghidra inside a real Ghidra

A **manual/dev** rig (not CI) that runs the one thing the in-tree ghidra-sim
harness (`make test-ghidra`, see `docs/ghidra-integration.md` §11) cannot: a
REAL Ghidra as the other end of the decompile-process pipe. It decompiles the
same functions twice — once with the stock native core, once with
`kuna_ghidra` swapped in — and writes a side-by-side report carrying the same
badness-scanner counts the sim pins (raw-register leaks, `Unique<hex>` tokens,
`sub_`/`dat_` placeholder rate, the `unsigned_long__`-style `getC()` mangling
signature, and a kuna-vs-stock line-diff ratio).

The core swap uses the same seam as the `KunaDecompiler` extension
(`integrations/ghidra/KunaDecompiler/`): a reflection write of
`DecompileProcessFactory.exepath`, which the factory consults before spawning
the native process. No extension install is needed for the smoke.

## Setup

1. A Ghidra release (12.x), e.g. `~/tools/ghidra_12.1.2_PUBLIC`.
2. A venv with **pyghidra**. Ghidra bundles the wheels, so this works fully
   offline (the PyPI `pyghidra` also works when you have network):

   ```bash
   python3 -m venv ~/.virtualenvs/kuna-ghidra
   source ~/.virtualenvs/kuna-ghidra/bin/activate
   pip install --no-index --find-links \
       "$GHIDRA_INSTALL_DIR/Ghidra/Features/PyGhidra/pypkg/dist" pyghidra
   ```

3. The `kuna_ghidra` binary — NOT part of `make binaries`; build it directly:

   ```bash
   cd decompiler && cargo build --release -p kuna-ghidra
   ```

   (produces `decompiler/target/release/kuna_ghidra`).

## Run

```bash
source ~/.virtualenvs/kuna-ghidra/bin/activate
export GHIDRA_INSTALL_DIR=~/tools/ghidra_12.1.2_PUBLIC
export KUNA_SMOKE_BINARY=$PWD/tests/bug-repro/faillog   # any program
# optional:
#   export KUNA_GHIDRA_EXE=…/decompiler/target/release/kuna_ghidra  (default)
#   export KUNA_SMOKE_FUNCTIONS=main,usage    (default: main + 2 largest)
#   export KUNA_SMOKE_OUT=./live-smoke-out    (default)
python3 integrations/ghidra/live-smoke/kuna_vs_stock.py
```

Outputs, under `KUNA_SMOKE_OUT`:

- `stock_<fn>.c` / `kuna_<fn>.c` — the `getC()` text per core, per function;
- `REPORT.md` — completion status, per-function scanner counts, and the
  kuna-vs-stock diff ratio.

A working local example (the machine this rig was built on):
`GHIDRA_INSTALL_DIR=/home/mahaloz/ctf/tools/ghidra_12.1.2_PUBLIC` with the venv
`/home/mahaloz/.virtualenvs/kuna-ghidra`.

## Notes / learnings baked in

- **`analyze=True` matters**: the stock analyzers create the `FUN_…` function
  symbols and types that Java answers queries from; an unanalyzed program
  makes both cores look artificially bad.
- **`getC()` is not the GUI panel**: `DecompileResults.getDecompiledFunction()`
  runs every funcname/variable/type/field/label token through Java's
  `IllegalCharCppTransformer`, so a token kuna emits with spaces/stars inside
  (`"unsigned long *"` as ONE `<type>` token) reads back as
  `unsigned_long__…`. The GUI panel renders token text verbatim. The sim
  harness replicates the `getC()` behavior, so its numbers and this rig's line
  up.
- The python scanners here are a small re-implementation of the rust ones
  (`decompiler/crates/kuna-ghidra/tests/ghidra_sim/mod.rs`) with a fixed
  x86-64 register set; the rust harness (which harvests the register list from
  the live Sleigh) is authoritative for pinned numbers.
- Decompile timeouts are process murder (Java disposes the child); if the kuna
  pass hangs, look for the spawned `kuna_ghidra` PID and its stderr.
