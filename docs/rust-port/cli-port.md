# CLI port — the user-facing `kuna` binary

The four Python user-facing entry points were ported to a single Rust binary,
`kuna` (crate `rust/crates/kuna-cli`, built by `make binaries` to
`rust/target/release/kuna`), so the whole project lives under one Rust paradigm
now that the C++ tree is gone.

| Python module (removed) | Rust subcommand | Strategy |
|---|---|---|
| `kuna/decompile.py` (CLI `main`) | `kuna decompile` | shell out to `decomp_dbg`, same console script, capture C via `openfile write`/`closefile` |
| `kuna/run_tests.py` | `kuna test` | shell out to `decomp_test_dbg`, parse stderr (unit) + stdout (datatest) streams, baseline passing-set diff, exit-code rules |
| `kuna/catalog.py` | `kuna catalog` | shell out to `decomp_dbg stage catalog`, extract+reformat JSON (`--json`/`--markdown`/text); `--check` is **in-process** |
| `kuna/slacomp.py` | `kuna specs` | alias `slacomp`; `--diff` is a note (the C++ `sleigh_opt` oracle is gone) |

**Approach.** Most subcommands shell out to the already-built engine binaries
(exactly as the Python did) and parse their streams in Rust — guaranteeing
byte-identical output. The one in-process bit is `catalog --check`: the old
`check_drift` parsed the C++ `options.cc` + `kuna_*.hh` for `registerOption(new
OptionXxx()); // (kuna)`, which no longer exist; the Rust replacement links
`kuna-decomp` and cross-checks the catalog the binary emits against the
registered option set `kuna_decomp::options::KUNA_OPTION_NAMES` (a registered
option with no catalog row is undiscoverable; a catalog row matching no
registered option is stale). The build-time codegen (`stages.toml` →
`SETTABLE_TABLE`) already fences the table/catalog consistency at compile time;
this end-to-end check fences the *emitted* catalog against the registration list.

Note: the Python `catalog --check` is **broken** in the Rust-only world (it
`FileNotFoundError`s on the removed `decompiler/cpp/options.cc`); the Rust
`kuna catalog --check` is the working replacement, so for that one subcommand the
differential is "Rust passes (exit 0); Python errors" rather than equal output.

## JSON byte-parity (`jsonfmt.rs`)

`catalog --json` and `test --json`/`--save-baseline` must match CPython's
`json.dumps(..., indent=2[, sort_keys=True])` byte-for-byte. A small hand JSON
parser + pretty-printer reproduces it: 2-space indent, `": "`/`",\n"` separators,
`ensure_ascii=True` (`\uXXXX` for ≥0x7F), object key order preserved (or sorted
under `sort_keys`). No serde dependency — the catalog JSON is the binary's own
controlled ASCII output. `catalog --json` re-dumps the list form; `test`/baseline
use the sorted form. The `extract_json_span` helper ports `_extract_json`
(skip the `[decomp]>` prompt brackets, then balance brackets ignoring string
literals).

## Differential checks run (all matched)

Against the **still-working** Python modules this wave, with `KUNA_ROOT` and
`KUNA_ENGINE=rust` pointed at the same tree/binaries:

1. `kuna catalog --json` vs `python3 -m kuna.catalog --json` → **byte-identical**.
2. `kuna catalog` (text) and `kuna catalog --markdown` vs the Python → **byte-identical**
   (the `⚠️ opt-in` emoji and all).
3. `kuna catalog --check` → exit 0 ("catalog OK: documents exactly the registered
   kuna options"). (Python `--check` is broken here, see above.)
4. `kuna test --datatests --json` vs `python3 -m kuna.run_tests --datatests --json`
   → **byte-identical** (footer `[675, 675]`, same 675-key passing set, same exit 0).
5. `kuna test --datatests --baseline docs/baseline.json` → `PARITY OK`, exit 0.
6. `kuna test --datatests --save-baseline F` vs Python → **byte-identical** baseline JSON.
7. `kuna test --datatests --name <real>` / text summary / bad-name warning vs Python
   → **byte-identical** (warning quoting matched to Python `%r` single-quotes; the
   harness's `-path` dir is `os.path.abspath`'d so file-level error lines match).
8. `kuna test --datatests --datatests-dir tests/stages --baseline docs/baseline-stages.json`
   vs Python → **byte-identical** (both exit 1 — the stage corpus has pre-existing
   failures on this branch, unchanged by the port).
9. `kuna decompile /tmp/t.elf add` (a tiny `gcc -O0` ELF) vs `python3 -m kuna.decompile`
   → **byte-identical**; `--option compareform canonical` likewise.

Unit `#[test]`s in the crate lock the pure logic: `jsonfmt` (round-trip, empty
containers, prompt-bracket skipping, ASCII high-char escaping) and `test` (unit/data
stream parsing, the `*.xml` error forms, `dedup_keys` repeat suffixing, the sorted
baseline record shape).

## Cleanup

- `Makefile` `test`/`test-stages` now run `$(BINDIR)/kuna test ...` (was
  `python3 -m kuna.run_tests`); `binaries` also builds `kuna-cli` (and now
  `kuna-harness`, so `decomp_test_dbg` is actually produced — it was a missing
  dep before). `make test` → exit 0, 675/675 PARITY OK.
- Removed: `kuna/run_tests.py`, `kuna/catalog.py`, `kuna/slacomp.py`. Their
  `console_scripts` (`kuna-decompile`, `kuna-test`) were dropped from
  `pyproject.toml`.
- `kuna/decompile.py` was reduced to a **thin library shim**: it keeps only the
  `decompile()` function + `DecompileError` family that the out-of-scope
  `kuna/pipeline/` imports (`from .. import decompile as kdecompile`); the
  `argparse` `main()` (the CLI) was removed.
- `kuna/paths.py` and `kuna/__init__.py` are kept (the package + the lazy
  `decompile` re-export the pipeline relies on).

## Follow-up

`kuna/pipeline/` (the autonomous feature loop) is still Python and out of scope;
it depends on the `kuna/decompile.py` library shim and `kuna/paths.py`. Porting
the pipeline to Rust would let the shim and the rest of the `kuna/` Python package
be removed entirely.
