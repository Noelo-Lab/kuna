# kuna

A standalone **Rust port of [Ghidra](https://github.com/NationalSecurityAgency/ghidra)'s
decompiler** (the "deep decompiler", built around SLEIGH) and its **SLEIGH compiler**, broken
out of the Ghidra Java application so the core decompilation pipeline can be studied,
instrumented, and refined stage-by-stage in a memory-safe, modular engine.

The engine was ported line-faithfully from C++ to Rust and verified against the original
(**675/675** decompiler regression assertions; the SLEIGH compiler produces **148/148**
language specs to byte-identical `.sla`), then the C++ tree was removed. See
**[`docs/RUST_PORT.md`](docs/RUST_PORT.md)** for the full story — what was ported, why, how,
and how the port was validated.

## Build

You only need a **Rust toolchain** (`cargo`). No C/C++ compiler, libbfd, or bison/flex — those
were only for the now-removed C++ tree.

```bash
make binaries   # cargo-build the decompiler (decomp_dbg, decomp_test_dbg),
                # the SLEIGH compiler (slacomp), and the `kuna` CLI
make specs      # compile every SLEIGH .slaspec -> .sla with slacomp (the decoder needs these)
make            # = binaries + specs
```

Everything lands in `rust/target/release/`. For development, work in the cargo workspace
directly: `cd rust && cargo build` / `cargo test --workspace`.

## Run the decompiler

The user-facing command is the single Rust binary **`kuna`** (`rust/target/release/kuna`):

```bash
KUNA=rust/target/release/kuna

# Decompile a function by name, or by address in a stripped binary
$KUNA decompile ./a.out main
$KUNA decompile ./stripped.bin 0x401040 --addr
```

Example output for a tiny `int add(int a,int b){return a+b;} int main(){return add(2,3);}`:

```c
void main(void)

{
  add(2,3);
  return;
}
```

kuna reads ELF/PE/etc. via the `object` crate and selects the SLEIGH language automatically.
`decompile` drives `decomp_dbg` and captures the emitted C cleanly (interactive prompts never
pollute the output).

### The stage-model control surface

kuna exposes Ghidra's pipeline as an explicit **stage model** with per-run, flippable sub-stage
assertions (the "LLM control API"):

```bash
$KUNA catalog --json                                   # discover the settable assertions
$KUNA decompile ./a.out main --option compareform canonical
$KUNA decompile ./sparc.elf main --option returnpair single
```

`catalog --markdown` regenerates `docs/assertions.md`; `catalog --check` fails on
catalog/registration drift (CI).

## Test

```bash
make test        # the 675/675 decompiler regression parity (datatests + docs/baseline.json)
make rust-test   # the full cargo workspace suite (ported unit tests, golden differential
                 # vectors, SLEIGH-compiler .sla content-parity, ...)
```

`make test` compiles the specs with the Rust SLEIGH compiler and decodes the XML regression
corpus (`decompiler/datatests/`, 83 files / 675 assertions) with the Rust decompiler, end to
end — the self-sufficient correctness gate. `docs/baseline.json` is the recorded oracle.

## Layout

| Path | What it is |
|---|---|
| `rust/` | The engine — a cargo workspace. `kuna-base`/`kuna-num`/`kuna-sleigh`/`kuna-decomp` (decompiler), `kuna-console` (the `decomp_dbg`/`decomp_test_dbg` binaries), `kuna-slacomp` (SLEIGH compiler, `slacomp`), `kuna-cli` (the `kuna` binary) |
| `decompiler/datatests/` | Upstream XML decompilation regression tests (83 files → 675 assertions); the corpus `make test` runs |
| `specs/Ghidra/Processors/` | Vendored SLEIGH processor specs; `.sla` are build artifacts produced by `slacomp` |
| `Makefile` | Top-level build/test driver (Rust-only) |
| `docs/RUST_PORT.md` | The port summary (what/why/how/validation) |
| `kuna/` | Remaining Python: the autonomous feature `pipeline/` + a few helpers (the user-facing CLI is now the Rust `kuna` binary) |
| `tools/sync_upstream.py` | Pulls upstream Ghidra `specs/` + `datatests/` updates |

## Understanding the decompiler

kuna is organized around an explicit **stage model** (a P0 knowledge plane + S1–S9 with a
Band-B fixed point and typed feedback edges), not a linear pipeline:

- `docs/stages.md` — the stage model at a glance; `docs/stage-model.md` — the full normative
  model (sub-stage catalogs, feedback edges, code anchors).
- `docs/stage-mapping.md` — every source file mapped to a stage, anchored to the real pass
  pipeline (`universalAction`) and the runtime registry (queryable via the `stage list/map/
  catalog` console commands).
- `docs/assertions.md` — the LLM-settable sub-stage assertions; `docs/divergences.md` records
  kuna's intentional default changes.

## Provenance

Ported from Ghidra commit `cef869af04c4740a71ad31a55704045b1b0d1644`. The SLEIGH specs and the
XML regression corpus are still vendored from upstream; the C++ source that the engine was
ported from is recorded at that commit and recoverable from git history (the tree was removed
once the port was proven — see `UPSTREAM.md` and `docs/RUST_PORT.md`).
