# Proposal: bounded i386 switch successors in the discovery Listing

This design was approved before implementation under standing requirement 5
and §4 of `docs/improvement-pipeline.md`. It is now implemented behind the
default-off `x86listingswitch` option. The sections below retain the approved
contract; current behavior, corpus recall limitations, and evidence are in
[`analysis.md`](analysis.md) and [`record.json`](record.json).

## Scope

Add one P1 analysis option, `x86listingswitch`, initially default off, and one
owning module, `kuna-analysis/src/listing/kuna_x86listingswitch.rs`. Its first
scope is linked i386 ELF and PE absolute-pointer switches whose index range can
be proved locally. It supplies same-function successors to the Listing used by
entry discovery. Cases are never directly promoted to function entries; calls
from newly reached cases use the existing function-claim rule.

Reuse the existing immutable `PoolImage`, bounded pointer reader, p-code
folding, and guard facilities. Do not copy the direct xrefs arm's
`targets(..., cases=None)` fallback. A missing proof leaves the dispatch
unresolved and preserves the previous discovery behavior.

## Admission contract

1. The terminal operation is a computed jump, not a call or an import veneer.
   Its own p-code proves a pointer-width load from `table + index * 4`. The
   table base and load stride must be known; unrelated address constants in
   the instruction do not establish a table.
2. A checked unsigned range bounds that exact index, after accounting for any
   proven normalization. The proof connects the comparison operand to the
   address expression and rejects writes or partial-register clobbers between
   them. An unrelated nearby comparison, signed test that admits negative
   indices, or a lower-bound check alone is insufficient. Comparison width,
   index width, scale, and address arithmetic must agree: a byte check does not
   bound unknown upper bits of a word index. Normalization and base addition
   must be proved at the target's width, including wrap; mathematical integers
   must not substitute for the actual machine address calculation.
3. The admitted range-check edge dominates this dispatch along the recognized
   local path. Discovery of a nearby check is insufficient if another incoming
   edge can bypass it. The initial implementation may restrict itself to an
   unambiguous straight-line block and guarded fall-through; more general CFG
   reasoning must not be silently added to this feature.
4. Read exactly the proven cardinality, subject to `jumptablemax`, from aligned,
   allocated, non-writable, file-backed words. Targets must lie in the
   dispatcher's executable section. Reject an invalid in-bound entry rather
   than infer a shorter table. A deliberately truncated prefix may be followed
   only under the existing ceiling semantics; it never licenses reading beyond
   either the proven bound or the ceiling.
5. Queue accepted targets as instruction successors and file code references
   when requested. Preserve explicit function seeds and names. A branch to an
   independently established function must not erase that function's identity.
   Existing tail-call handling remains responsible for actual inter-function
   jumps; this reader supplies no new tail-call inference.

Instruction membership, not the minimum/maximum case addresses, establishes
coverage. Valid case blocks may surround an independent function or a data gap.
Only instructions actually reached from admitted targets and their ordinary
successors become decoded. No range from the first case to the last is claimed
as the dispatcher's body, and no function entry inside that range is suppressed
because it lies between cases. Preserve the existing seed-first function map
and direct-call claim behavior; if a proposed case conflicts with an established
entry and its role cannot be resolved by existing policy, refuse that dispatch
rather than erase or reclassify the entry.

The negative fixture in `analysis.md` is mandatory: the neighbouring callback
word must not become a fifth successor. No AIF rejection predicate or global
count-based filter is part of this feature.

## Serial and parallel behavior

`walk::step` remains the common successor producer. Immutable image/table
metadata must be available to every lane, with each lane using its own decoder
and address spaces. A reader may inspect only predecessor evidence whose
availability cannot depend on lane scheduling. Missing evidence requires a
bounded deterministic deferred check after predecessor reconciliation, or the
same conservative refusal in both paths. It must never mean “serial reads the
table, parallel skips it.”

Use shared post-walk reconciliation, after either the serial or parallel
ordinary walk has finished. The implementation route is:

1. Preserve the completed ordinary Listing as the fallback. Collect unresolved
   computed jumps in address order from that reconciled partition.
2. Evaluate the local proof against this shared snapshot. Keep proposed edges,
   proof evidence, and the expanded instruction/reference/function model in an
   isolated candidate; do not mutate the fallback or publish provisional
   coverage to AIF. Reject unavailable or ambiguous proofs.
3. Walk accepted targets as instruction roots, using the ordinary successor
   rules. Newly discovered direct callees keep the existing function-claim
   rules. Reconcile the resulting partition before evaluating newly reached
   dispatches; no lane-local decode availability grants admission.
4. Repeat only while new dispatch edges or instructions are added. Each
   dispatch/target pair is deduplicated, each table is bounded by its proved
   cardinality and the live ceiling, and reconciliation has a fixed sixteen-round
   limit. If another round is required, discard the candidate and return the
   ordinary Listing. A memory-budget refusal likewise returns that fallback.
5. Before publishing, revalidate every admitted proof against the complete
   candidate partition, including new incoming edges and entry conflicts. If
   any proof is invalidated or cannot establish stability, discard the whole
   candidate and return the ordinary Listing. Otherwise publish the candidate
   once, then let entry discovery/AIF consume it.

This deliberately uses a conservative whole-candidate fallback instead of
retaining successors whose proof failed or introducing rollback of dependent
subgraphs. The resume path and cold-build path must produce identical final
partitions and refusal decisions.

Admission cannot rely on the absence of a bypass edge that a later round might
discover. A proof whose dominance or index provenance could be invalidated by
newly reached instructions must be refused until stable evidence is available;
such a proof must not add successors early and retain them after invalidation.
If that restriction cannot be met with the proposed bounded local proof, return
to the design review before expanding into a new CFG analysis.

This approved design crosses Listing construction, successor production, and
parallel reconciliation; it is more than a thin reader call.

## Implementation source ownership

| Path | Functions / responsibility |
|---|---|
| `kuna-analysis/src/listing/kuna_x86listingswitch.rs` (new) | proof, bounded read, unresolved-dispatch records, deterministic reconciliation |
| `kuna-analysis/src/listing/mod.rs` | module registration; `Listing::build_resuming` and resume integration |
| `kuna-analysis/src/listing/walk.rs` | `StepCtx`, `walk`, `step`: common same-function successor seam |
| `kuna-analysis/src/listing/kuna_poolref.rs` | explicit complete bounded-read interface; existing reader behavior preserved |

Paths above are relative to `decompiler/crates/`. Do not change the separate
xrefs walk's policy in this PR; its demonstrated overread needs its own scoped
fix. If sharing a helper would change existing xrefs behavior, preserve its
existing entry point and add an explicitly bounded interface instead.

The implementation also includes the architecture option field/reset and
dispatch, complete `phases.toml` metadata, fresh catalog fixture/counts and
`docs/options.md`, an end-to-end stage, stage corpus count/baseline, and prose in
`docs/spec/01-program-prep.md`. Parallel lanes use the unchanged ordinary walk;
the new shared post-walk reconciliation needs no lane-local proof or change to
`kuna_pdecode.rs`.

## Acceptance before implementation can ship

- Prove all four public fixture cases are Listing instruction starts, the
  default and direct callee stay present, and no adjacent callback word is
  consumed. Add native tests for missing/wrong-index guards, overwritten index,
  bypass edges, signed negative indices, partial-width comparisons/clobbers,
  normalization/address wrap, import veneers, writable/truncated tables,
  invalid entries, repeated targets, and the live table ceiling. For every
  refusal, assert there is no fallback to an unbounded reader.
- Add a reproducible i386 image with enough discovered functions for AIF and
  independent return-terminated cases. The off/on stage must drive
  `binary -> decompile` and inventory checks: false case entries disappear,
  all case behavior remains in the dispatcher, calls in cases recover real
  callees, and an adjacent genuinely unreferenced function stays discoverable.
  Add interleaved case blocks around a genuinely independent function and a
  gap: preserve its established entry in one arm and its later discoverability
  when initially unreferenced in another. Neither intervening bytes nor the
  independent entry may be claimed by a min/max case extent.
  Native execution of a compiler/assembly equivalent must agree for every case
  and out-of-range input. First verify the off arm actually reproduces the
  mechanism, rather than relying on a different prologue scanner's result.
- Require identical instruction, reference, and function models for serial,
  forced multi-lane, varied interval partitions, and resumed cold builds.
  Exercise a dispatch whose guard/cases cross interval boundaries.
- Re-run paired entry sets against symbol twins: the 107 non-x86-64 decbench
  images plus preserved 17 Wine DLLs and 16 MinGW crackmes. Keep shipped,
  candidate, AIF-off, and candidate/AIF-off arms. Account for every lost true
  start; none is acceptable for a default-on recommendation. Inspect changed
  i386 function bodies and table bounds, not just net entry counts. ARM and
  x86-64 are unchanged controls. Existing u-boot/CMSIS recall tripwires remain.
- Measure discovery and whole-binary decompilation speed on affected i386
  targets and unaffected controls, with interleaved repetitions. Record the
  actual delta; default on needs 0/675 changed datatest assertions and the
  documented at-most-five-percent speed budget. Otherwise remain opt-in.
- Run all four repository gates, tooling tests, lint, catalog freshness, and
  merge guards on the exact final base. Re-record only stage expectations
  intentionally added by this feature; never repin `docs/baseline.json`.

Report measured partial improvements as such. #299 stays open unless its
original PE/crackme artifacts become available and the issue's remaining
acceptance criteria are demonstrated directly.
