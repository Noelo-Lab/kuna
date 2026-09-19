# `protoorder` — plan

**Status: implemented, default ON (`types`).** The proposal round is over: the
locking design rounds 1–4 measured is kept as the opt-in `lock` value, and the
default is the types-only channel round 5 built, which cannot change a call's
arity at all.

## What ships

One option, `protoorder off|types|lock`, default `types`, P4, subphase
`prototype-source`, tier `transform`, `change_kind analysis-enablement`.

| Piece | Where |
|---|---|
| The mode, the recovered-types record, the converter (recovered `FuncProto` → `PrototypePieces`) and the whole acceptance policy | `decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs` |
| The per-image table of stated types, and the per-function copy of it | `infra/architecture.rs` (`kuna_protoorder_types`), `substrate/funcdata.rs`, `infra/decompile_drive.rs` (`seed_protoorder_types`) |
| The vote and every refusal (`call_argument_vote`) | `decompiler/crates/kuna-decomp/src/p4_calls/kuna_protoorder.rs` |
| The consumption: the recovered arm of `TypeOpCall::getInputLocal`, and `declared_input_type_local` for the cast path | `decompiler/crates/kuna-decomp/src/p5_types/coreaction_infertypes.rs`, `p9_emit/coreaction_casts.rs` |
| The statement hook (per target, after the function's own decompile) | `decompiler/crates/kuna-console/src/project.rs` (`DecompileOptions::park_recovered_proto`) |
| The order (call graph → Tarjan SCC → reverse-topological), the buffered address-order emission, the narrowed-run skip, the `--jobs` exclusion and note, and the inert-surface warning | `decompiler/crates/kuna-cli/src/decompile_all.rs` (used from `decompile_project.rs`, `decompile_graph.rs`) |
| The option row / arch field / live value | `phases.toml`, `infra/architecture.rs`, `p0_knowledge/options.rs`, `kuna-console/src/kuna_console.rs` |
| Spec prose | `docs/spec/04-calls-and-prototypes.md` — *(kuna) `protoorder` — callee-first prototypes* |
| Tests | 23 unit tests in `p4_calls/kuna_protoorder/tests.rs`; seven `tests/cli` probes over four in-repo fixtures: the off arm, the default arm, the arity pin and the `lock` register-save decline (`protoorder_x86_64`), a stack array and a `struct stat` that a vote used to split (`protoorder_stack{array,struct}_x86_64`, gcc -O0), and a Thumb function address a pointer vote would print as `&sub_8130[1]` (`protoorder_codeptr_thumb_le32`); `kuna-cli` tests for the `--jobs` contract, the `--jobs` note, the narrowed-run skip and the raw-image order |
| Corpus tools | `corpus-diff.py` (every changed function classified, frame layout and use-before-def checked), `label-review.py` + `hand-labels.json` (the residue labelled), `callsite-metrics.py`, `invariants.py`, `underarity.py` |

## Steps taken

1. Converter + policy + unit tests (no driver).
2. Option wiring and counter sites; `docs/options.md` regenerated.
3. Driver: call graph, Tarjan SCC, buffered emission, `--jobs` exclusivity.
4. Fixture + `tests/cli` probes.
5. Rounds 2–4: the open parked tail, the unjustified-tail trim, the
   under-recovery decline, the register-file decline, the 22-binary corpus split
   by calling convention, the fabricated-parameter counter reported beside
   `type_match`, and the conditional-close experiment measured and rejected.
6. Round 5: the types-only channel as the default, matched by
   `FuncCallSpecs::final_input_storage`; the three differ bugs that made a type
   change look like an arity change; the corpus, `type_match`, speed and
   invariant re-measurement; the default flip.
7. Round 6 (review): the vote never sets a cast's required type; votes refused
   at frame objects, on declared/global/frame-memory values, against a
   different-class reader or writer and on float disagreement; the result vote
   dropped; narrowed runs skip the call graph; `--jobs` notes its difference; a
   whole-corpus differ with layout and use-before-def checks over 46 binaries.
8. Round 7 (review): a strict fix in `RuleExpandLoad` (an undefined narrow load
   keeps its width instead of printing as a sign-extending `(short)p[k]`), with
   a compiled round-trip test; pointer votes refused on constants inside code;
   the false "never adds a cast" claims replaced by what a winning vote does
   change; a raw image is not callee-first unless the option is named; rebased
   onto the raw-image inventory (d3617d64) and every measurement re-taken
   against that main.
9. Round 8: the `RuleExpandLoad` fix split out and landed first as #677; this
   branch rebased onto it (e1139df9) and carries none of it. Off arm re-checked
   byte-identical to that main; speed re-measured main vs this branch,
   min-of-15; the four fixtures catalogued in the fixture README; a main
   defect found on the way (a lost 16-bit truncation at a zero-extended call
   argument) recorded in `analysis.md` §10 and handed off.
10. Round 9 (review): a float vote is refused on a value any integer op
   computes with, addition and comparison included (MIPS o32 printed
   `(int)v1 + 3` for a word passed as a float's bits); a MIPS fixture, two
   probes and a compiled round-trip test; the `--jobs` help no longer claims
   the pool equals `--jobs 1` under the default; the load-widening spelling
   documented beside the store split; `type_match`, firmware and the off-arm
   identity re-measured.
11. Round 10 (review): a float vote is also refused on bits the caller
   truncates or byte-extracts (`SUBPIECE`, `PIECE`; MIPS o32 and ARM softfp
   printed `(short)((unsigned int)v1 >> 0x10)` on a `float v1`), and -- found
   while checking that -- on a value that is stored (x86-64 printed
   `v1[1] = (int)v3` for a float stored beside an int), returned, passed to
   another call or produced by one, except in a register the calling
   convention assigns to floats. The MIPS fixture gains g20/g22/g24, a new
   x86-64 fixture, two probes, the round trip covers seven callers; the corpus,
   `type_match`, off-arm identity (13 binaries) and speed (all eight slices)
   re-measured.
11. **Round 11 (review, rebase onto structsynth-on main 6e4f6fa5).** A pointer
   vote is checked against what the caller loads and stores through it: a float
   or composite pointee refuses an access or address outside the object, an
   access across or inside members, and integer bits moved through a float
   member; a float type never reaches a caller parameter in a register the
   convention would not give a float; a NaN constant never takes a float vote.
   New fixture `protoorder_floatpointee_x86_64` with a two-arm compile-back
   round trip and a probe; two probes and the MIPS round trip follow main's
   `struct_N` spellings. Every number re-measured on the rebased tree: corpus
   (60 binaries, including the reviewer's 14), `type_match`, off-arm identity,
   speed and all gates.
12. **Round 12 (found by the `ptrfromuse` default evaluation).** A vote whose
   pointee is a non-character integer, `bool` or pointer is refused when the
   caller stores a constant wider than the pointee through the pointer at a
   fixed place or a record stride, which `SplitDatatype` printed as one store per
   element (gzip's `".tar"` and betaflight's `1.0f` at the default; bzip2 and tar
   with `--option ptrfromuse void`). New fixture `protoorder_narrowvote_x86_64`
   with a CLI test over the default and `ptrfromuse void`, and a probe. Taken
   from the `ptrfromuse` branch without that branch's default flip.

## What is deliberately NOT in it

- **A second round** (`protofeedback`): re-decompiling after a full pass costs
  +77–94% and is a separate, permanently opt-in option.
- **The reverse direction** — joining the observed argument types at a call site
  into an unknown callee's parameters — which is angr's `is_prototype_guessed`
  path and a separate option.
- **Making `lock` compose with `argclobber`.** `drop_clobber_tail_arg` returns
  early on a locked-and-variadic call spec, which every stated `lock` site is.
  Teaching it that such a spec still has trials past the locked prefix is the
  named next lever; `types` composes with `argclobber` today.
- **A `tests/stages` XML.** The option only exists on the whole-binary
  `decompile-all` surface, which the stage harness does not drive; the two-pass
  evidence is the fourteen `tests/cli` probes over nine in-repo fixtures, which
  run the real binary→decompile path, and the two compile-back round trips.
- **The `RuleExpandLoad` fix.** It is #677, on main before this PR.
