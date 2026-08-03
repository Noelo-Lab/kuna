# ptrentry — implementation plan

Owning phase: **P1 code/data partition**, ARM discovery Stage 3b, inside the
Listing tier (`docs/spec/01-program-prep.md` §1.6). Option: `ptrentry`, tier
`analysis`, `change_kind` `analysis-enablement`, default-OFF.

## 1. The pass — `analyzers/aif/kuna_ptrentry.rs` (new module)

* `code_pointer_sites(file, exec_ranges) -> BTreeMap<target, Vec<site>>` — every
  4-byte-aligned word in every allocated section whose bit 0 is set and whose
  masked value lands in an executable range, keyed by target so the containment
  guards can see *all* of a target's referencing words at once (the shipped Stage-3
  scan keeps only the target set and loses the sites).
* `probe_target(decoder, listing, claimed, target, sites) -> Result<body, Guard>` —
  the whole acceptance predicate, returning the [`Guard`] that rejected so the
  precision model is one readable function:
  1. `Guard::NotUndefined` — `!listing.is_undefined(target)` or already claimed
     (the shipped never-split-a-discovered-function guard, kept verbatim);
  2. `Guard::SiteIsInstruction` — any site's `code_unit_at` is `Instruction`;
  3. `Guard::SameFunction` — any site's `function_containing` equals the target's;
  4. `Guard::NotTerminating` — the terminating-routine walk rejects.
* `check_terminating_routine(...)` — `super::check_valid_subroutine` minus
  `MIN_SUBROUTINE_INSNS`, with the *strict* escape policy (an escape outside
  `[entry, gap_hi)` that is not an existing instruction start is a reject) and the
  same `MAX_FOLLOW_INSNS` bound.
* `pointer_entry_seeds(...)` — ARM gate, then the ascending-target loop with the
  body-claim dedup, returning the accepted entries address-sorted.

Nothing in `aif/mod.rs` changes except one `pub mod kuna_ptrentry;`: a child module
can read its parent's private items, so `GapDecoder`, `ProbedInsn` and
`MAX_FOLLOW_INSNS` are reachable without widening any visibility.

## 2. Wiring — `passes.rs::run_listing_consumers`

Runs after the Stage-3 code-pointer re-walk (so it sees the richest Listing) and
before the consumer dispatch, gated by `arch.analysis_listing &&
arch.analysis_ptrentry`. Its output is pushed as a `("ptrentry", entries)` pair,
committed by the existing `entries` arm.

No load-time stash is needed and none is used: `run_listing_consumers` is already
the *deferred* commit point, so `--option` is in effect when it runs. (Step 1's
trap — reading an arch flag inside a load-time `AnalysisPass::run`, where the flag
is still at its default — does not apply to a Listing consumer.)

**Rejected, with measurements in `analysis.md` §5:** re-seeding the walk with the
accepted targets and rebuilding the Listing, the way Stages 2 and 3 do and the way
proposal step 2 item 4 suggested. Measured at 974 split bodies and 734 lost
entries; the additive wiring is what makes "never removes an entry" structural.

**Rejected, with measurements in `analysis.md` §3.1:** the proposal's table-run
corroboration. It cannot separate switch tables (a switch table is a run) and costs
311 ground-truth functions.

## 3. Registration

* `Architecture::analysis_ptrentry` (field, `reset_defaults` = false,
  `set_kuna_option` arm).
* `engine.rs::analysis_pass_enabled` `"ptrentry"` arm (the defensive commit gate;
  the pre-invocation check in `passes.rs` is the live one).
* `p0_knowledge/options.rs` `KUNA_OPTION_NAMES`.
* `phases.toml` `[[settable]]` row, every field populated, `source_decompiler =
  "kuna"` (this came from kuna's own recall measurement), `inspiration` prefixed
  `decbench:`.
* Hard-coded catalog counts, every one **derived from a fresh `phase catalog`
  capture on the rebased tree rather than by arithmetic**: `kuna_phases/tests.rs`
  (settable 87 -> 88, analysis tier 26 -> 27, the `PASS_GATES` list, the JSON row
  count 86 -> 87) and `catalog_bytecompat.rs` (87 -> 88 plus the recaptured
  `tests/fixtures/phase_catalog.json`). The live-value count stays at **29** —
  this row declares no codegen `live_field`, so it joins `PASS_GATES` instead.
  `tests/stages/kuna-catalog.xml` needs no edit: its exact-count asserts key on
  `source_decompiler = "angr"` and the `structure-recovery` / `opt-in-tool`
  change_kinds, none of which this row joins. No stages XML is added, so the
  `kuna-base/src/xml.rs` corpus count (verified by counting the files: 83
  datatests + 107 stages = 190) and `docs/baseline-stages.json` are untouched.

## 4. Tests

* Unit (`kuna_ptrentry.rs`): the collector over the committed fixture — every code
  pointer found with its site list, the shared-target case, the non-candidate cases
  (even word, zero slot, target outside the executable ranges), and the ARM gate
  proven load-bearing (the scan itself is architecture-blind).
* e2e (`kuna-console/tests/verify_ptrentry.rs`): four tests over
  `tests/fixtures/cortexm_ptrentry_le32` — option off, the pointer-only leaf is in
  no symbol table and its bytes produce no C; option on, it is registered and
  decompiles to its real constant; the switch-table case stays rejected **with the
  option on**; and the discovery set is a strict superset.

**No `tests/stages/*.xml`.** The pass keys on ELF structure *and* on the Listing
tier, which is built only from an `ObjectLoadImage`. The datatest harness embeds
raw `<bytechunk>`s and never constructs one, so the option is unobservable there —
the same reason every sibling analysis-tier discovery flag (`funcstart_patterns`,
`aif`, `eh_frame_full`, `cortexmvectors`, `i386_pie_plt`) ships with a cross-crate
`verify_*` gate instead. `kuna-base/src/xml.rs`' corpus file count and
`docs/baseline-stages.json` are therefore untouched.

## 5. Fixture

`decompiler/crates/kuna-analysis/tests/fixtures/cortexm_ptrentry_le32` (405 bytes)
plus its generator `cortexm_ptrentry_le32.py` and a fixtures-README row.
Hand-assembled because no cross toolchain on this host emits a bare-metal STM32
link layout. Its vector table is deliberately the *shipped*-signature shape, so
nothing in this PR's e2e depends on `cortexmvectors`.

## 6. Docs

`docs/spec/01-program-prep.md` §1.6 (the ARM seed scans and the `ptrentry`
paragraph), regenerated `docs/options.md`, and this bundle. No `docs/history.md`
DIV row: default-OFF, so default output is byte-identical.
