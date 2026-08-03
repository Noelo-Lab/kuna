# tailcallentry — implementation plan

Owning phase: **P1 code/data partition**, the Listing tier's recursive-descent
walk (`docs/spec/01-program-prep.md` §1.6). Option: `tailcallentry`, tier
`analysis`, `change_kind` `analysis-enablement`, default-OFF.

## 1. The pass — `listing/kuna_tailcallentry.rs` (new module)

Sits next to `walk.rs`, because it is the walk's missing tail-call notion, and
reads the walk's completed output rather than running inside it. `analysis.md` §3
proves the split cannot change the walk's decoded-instruction closure, which is
what makes reading it afterwards both legal and strictly better-informed.

* `tail_call_entries(file, listing) -> Vec<u64>` — the ARM gate, then the
  candidate collection and the acceptance filter, address-sorted.
* `branch_targets(listing, entries)` — every `FlowKind::UnconditionalBranch`
  static target that is a decoded instruction, is not already a function entry,
  and is not the branch itself, keyed by target with **all** of its branch sites
  (the guards need to see every site of a target at once).
* `probe(listing, entries, target, srcs) -> Result<(), Guard>` — the whole
  acceptance predicate, returning the [`Guard`] that rejected so the precision
  model is one readable function:
  1. `Guard::ImpurePredecessor` — some reference into the target is a
     fall-through or a conditional branch;
  2. `Guard::InCallerRegion` — some site does not leave its own entry-ordered
     function region (**the containment guard**);
  3. `Guard::EpilogueStart` — the target opens with a stack restore;
  4. `Guard::NotTerminating` — the target's bounded forward flow closure reaches
     no `RETURN` / computed jump.
* `region_of` / `next_entry` — the entry-ordered region key, the same granularity
  `Listing::function_containing` reads, over the sorted entry vector.
* `terminates` — the bounded (`MAX_REGION_INSNS` = 4000, `followSubFlows`' cap)
  forward closure, with no length floor.
* `restores_frame` — the `POP` / `VPOP` / `LDM SP!` / `ADD SP` epilogue shape.

Nothing in `listing/mod.rs` changes except one `pub mod kuna_tailcallentry;` —
the module reads only the public `Listing` facade.

**Rejected, with measurements in `analysis.md` §4.1:** the stack-discipline model
(reject a branch taken with an unmatched `PUSH`/`SUB SP` open). Implemented as a
forward may-analysis and dominated by region containment on precision (89.9% vs
94.6%) and on splits (18 vs 0).

**Rejected, with measurements in `analysis.md` §4.2:** requiring inter-function
alignment padding before the target. 1.6pp of precision for 186 ground-truth
functions.

**Rejected, with measurements in `analysis.md` §7:** folding in the AIF gap-start
re-walk. It is a walk *re-seed*, which changes the decoded-instruction closure;
measured at 109 lost entries (87 ground truth) and 1013 split bodies, and
`tailcallentry` makes it slightly worse rather than safe.

## 2. Wiring — `passes.rs::run_listing_consumers`

Runs after every re-walk stage and after `run_aif`, so it sees the richest
Listing, gated by `arch.analysis_listing && arch.analysis_tailcallentry`. Its
output is pushed as a `("tailcallentry", entries)` pair, committed by the existing
`entries` arm. The Listing is **never** rebuilt, which is what makes "never
removes an entry" a property of the wiring (§3). No load-time stash is needed:
`run_listing_consumers` is already the deferred commit point, so `--option` is in
effect when it runs.

## 3. Registration

* `Architecture::analysis_tailcallentry` (field, `reset_defaults` = false,
  `set_kuna_option` arm).
* `engine.rs::analysis_pass_enabled` `"tailcallentry"` arm (the defensive commit
  gate; the pre-invocation check in `passes.rs` is the live one).
* `p0_knowledge/options.rs` `KUNA_OPTION_NAMES`.
* `phases.toml` `[[settable]]` row, every field populated, `source_decompiler =
  "kuna"` (this came from kuna's own recall measurement), `inspiration` prefixed
  `decbench:`.
* Hard-coded catalog counts, all re-derived from a green build rather than by
  arithmetic after the rebase onto merged `ptrentry`: `kuna_phases/tests.rs`
  (settable 88 -> 89, analysis tier 27 -> 28, the `PASS_GATES` list, the JSON row
  count 87 -> 88) and `catalog_bytecompat.rs` (88 -> 89 plus the recaptured
  `tests/fixtures/phase_catalog.json`, re-captured from `decomp_dbg` after the
  rebase). `tests/stages/kuna-catalog.xml` needs no edit: its exact-count asserts
  key on `source_decompiler = "angr"` and the `structure-recovery` /
  `opt-in-tool` change_kinds, none of which this row joins; the two
  `min="10" max="160"` asserts absorb 88 -> 89. `make test-stages` is 394/394
  **PARITY OK** with no baseline re-record.

## 4. Tests

* Unit (`kuna_tailcallentry.rs`): the entry-ordered region lookup, the
  containment predicate in both directions (same region ⇒ not a tail call, across
  a discovered entry ⇒ tail call, no containing entry ⇒ reject), and the
  epilogue-shape matcher including its near-misses (`ldmia r3!` is not a stack
  restore; `ldr r0,[sp,#4]` is not either).
* e2e (`kuna-console/tests/verify_tailcallentry.rs`): four tests over
  `cortexm_tailcall_le32` — option off, the tail-called routine has no entry and
  its body is emitted inside `_start` (the bug); option on, it registers and
  decompiles to its own constant (the fix); the three near-miss shapes stay
  rejected **with the option on**; and the discovery set is a strict superset.

**No `tests/stages/*.xml`.** The pass reads the Listing tier, which is built only
from an `ObjectLoadImage`. The datatest harness embeds raw `<bytechunk>`s and
never constructs one, so the recursive-descent walk — and therefore this option —
is unobservable there, the same reason every sibling analysis-tier discovery flag
(`funcstart_patterns`, `aif`, `eh_frame_full`, `cortexmvectors`, `ptrentry`,
`i386_pie_plt`) ships with a cross-crate `verify_*` gate instead.
`kuna-base/src/xml.rs`' corpus file count and `docs/baseline-stages.json` are
therefore untouched.

## 5. Fixture

`decompiler/crates/kuna-analysis/tests/fixtures/cortexm_tailcall_le32` (437
bytes) plus its generator `cortexm_tailcall_le32.py` and a fixtures-README row.
Hand-assembled because no cross toolchain on this host emits a bare-metal STM32
link layout. Its vector table is deliberately the *shipped*-signature shape, so
nothing in this PR's e2e depends on `cortexmvectors`. It carries one genuine tail
call and the three near-miss shapes — all four reached ONLY by an unconditional
`B`, so the naive rule takes every one of them and the containment model takes
exactly one.

## 6. Docs

`docs/spec/01-program-prep.md` §1.6 (the tail-call paragraph after the `ptrentry`
one), regenerated `docs/options.md`, and this bundle. No `docs/history.md` DIV
row: default-OFF, so default output is byte-identical.
