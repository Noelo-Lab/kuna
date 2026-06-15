# Verification verdict: `w11-pipeline-binding` (Round 1)

**Verdict: ACCEPT_WITH_LOSSES**

Independent VERIFIER review of `rport/w11-pipeline-binding` (`243b63b`) against base
`rust-port`. The branch wires the kuna **Rust** engine into the existing comparison
pipeline as a first-class reference/under-test engine (`KUNA_ENGINE=rust`, the W11
`object`-crate ELF `LoadImage`), so a comparison row can be `kuna-rust vs kuna-cpp`
(pure cross-port differential) or `kuna-rust vs angr`.

## What the branch changes (Python + docs only)

```
docs/pipeline.md                        # +31  (cross-engine section, honest about the angr-venv gate)
kuna/decompile.py                       # +engine= param, resolves engine binary w/o env mutation
kuna/paths.py                           # +engine arg + engine-suffixed overrides (_CPP/_RUST)
kuna/pipeline/compare.py                # +kuna_engine threading + kuna_engine row field
kuna/pipeline/reference/__init__.py     # register kuna-cpp / kuna-rust adapters
kuna/pipeline/reference/kuna_adapter.py # NEW 116-line adapter (real ReferenceDecompiler)
```

No C++ source, no specs, **no rust crate source** touched
(`git diff rust-port...HEAD -- decompiler/ specs/ rust/` all empty).

## (1) IT REALLY WORKS — proven on a DIFFERENT real ELF than the porter's

The porter's docs demo is `f` in `/tmp/p.c`. I built an unrelated program
`/tmp/verify_w11.c` (`-O1`, PIE) with two non-trivial functions:
`collatz_steps` (`while`/`if`/global accumulator) and `checksum` (shift/xor loop).

- `compare /tmp/verify_w11.elf collatz_steps --reference kuna-cpp --kuna-engine rust`
  → BOTH engines actually decompiled the function via the real `decomp_dbg` subprocess.
  kuna-cpp emitted `dat_4014 = dat_4014 + v1; return;`; kuna-rust emitted a near-identical
  loop with genuine port differentials (`SUB(a0,0)`, `return v1`, missing global write).
  A populated comparison row with `kuna_engine: "rust"` and structural metrics. **Not a stub.**
- Adversarial #2 (`checksum`): real, distinct rust output (`v2/v3` temporaries,
  `(int8)(a1 & a1)` idiom) vs cpp — different code, same machinery.
- Adversarial #3 (address-mode `0x1129`): `kuna_mode=addr`, both engines OK.
- Adversarial #4 (three-way): `--reference angr --kuna-engine rust` →
  angr **9.2.213** reference text + kuna-rust under-test text + metrics, both present.
  The angr-vs-rust path is genuinely reachable on this host.

**Decisive negative control** (proves the rust binary is really driven, no cpp
fallthrough): on `collatz_steps`, `--kuna-engine cpp` → output BYTE-IDENTICAL to the cpp
reference; `--kuna-engine rust` → output DIFFERS. The engine switch routes to the actual
6.8 MB Rust binary (cpp is 72 MB).

## (2) FAITHFULNESS

`kuna_adapter.py` is a real `ReferenceDecompiler` subclass; its `text` field is *only*
ever assigned from `kdecompile.decompile(...)` (the real subprocess driver) — no hardcoded
output strings, no checked-in/cached comparison JSON anywhere in the diff. The cpp side is
the real main-tree C++ engine. The W11 ELF loader (`loadimage_object.rs`, 802 lines,
already on `rust-port`) is the real `object`-crate `LoadImage`, present on this branch.
`paths.binary(engine=...)` semantics verified directly:
- engine-suffixed `_CPP`/`_RUST` overrides resolve per engine;
- a bare `KUNA_DECOMP_DBG` does **not** hijack an explicitly-requested engine (cannot
  clobber the other engine mid-comparison) — the concurrency-safety claim holds;
- explicit `engine=` overrides ambient `KUNA_ENGINE` **without** mutating the env.

## (3) NO SPECIAL-CASING

Grep of all changed files for binary/function-name/address/output hardcoding: clean. The
only literal `_start`/`@entry` handling is a documented general ELF-entry convention, not
a per-binary hack. No `.elf`/`/tmp/`/`sub_<addr>`/magic constants in the added src.

## (4) NO REGRESSION

- **C++ oracle**: `207/207` unit + `675/675` datatest, **PARITY OK** against
  `docs/baseline.json`. C++/specs trees byte-untouched (diff empty).
- **Cargo workspace**: `cargo build --workspace --release` clean.
- **Rust datatests**: harness reports `200/668` assertions passing with `2` exec-failures
  (`displayformat.xml`, `indproto.xml`) — the documented pre-existing rust-port baseline.
  Because the branch changes **zero** rust crate source, the rust harness binary is
  identical to `rust-port`'s and the passing SET cannot have regressed.

## (5) Env / angr gap — honestly documented

`docs/pipeline.md` qualifies the three-way as "reachable when the angr venv is present,"
and pins the engine tag + resolved `decomp_dbg` path as the reference "version" for
reproducibility. On this host angr 9.2.213 is in fact available, so the angr-vs-rust path
runs. The pre-existing angr version-skew caveat (9.2.213 vs the checkout's 9.2.222) still
applies and is already documented in `docs/pipeline.md` "Known caveats" — see the LOSS note.

## Conclusion

A faithful PARTIAL exactly as the prompt allows: a real `kuna-rust-vs-kuna-cpp`
differential on a real ELF through the existing pipeline structure + the real W11 ELF
loader, no special-casing, no fabricated data, no oracle/datatest regression. The only
non-ideal is the inherited angr version skew, recorded as a LOSS.

**ACCEPT_WITH_LOSSES.**
