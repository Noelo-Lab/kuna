# Proposal: bounded automatic local stack views

This approved feature preserves native writes and declared pointer uses for
bounded local frame footprints, then represents incompatible complete object
views with the existing shared backing. The feature must close the original
67-result witness, the runtime 67/162 branch counterexample, and default
escaped-alias behavior without separating addresses or moving writes across
observers. The user approved this design. The bounded implementation passes
the native, default-parity, speed, repository and whole-corpus checks below.

The option is `localstackviews` (`off|on`), P6 `alias-facets`, rewind P2.
Implementation was developed default-OFF and now defaults ON after native
correctness, 675-assertion parity and speed checks. Existing full
`stackviews on` remains the broader explicit policy. The local option must have
its own ablation handle rather than silently widening `stackalias` or #1122.
When full `stackviews` is enabled, it takes precedence: the bounded policy
records no evidence and supplies no preservation or publication facts. This
keeps an explicitly broader policy from inheriting locally admitted scalar
views that its own mapping cannot safely combine.

This builds on mgostIH's PR #1162 and the already-merged #1163 emitter. It
supersedes the unsafe **admission criterion** of the failed local experiment;
it preserves its useful fixed-extent/type checks and the contributor's shared
storage model. The implementation continues on the approved proposal branch
and credits that recognition design. PR #1162 stays unchanged.

## One contract from early stores to typed uses

The policy owns two related records, scoped to the current function analysis:

- A local footprint: actual stack-space identity, exact fixed start and byte
  extent, declared pointer-use contracts, and escape/observation facts.
- Native write correspondence: original positive-width STORE extent, source
  point and operation identity, plus only proven descendants of that physical
  event. A same-address reconstruction or merge COPY does not become a native
  write merely because it reaches a call guard.

The single invariant is: **automatic local views cannot hide the last physical
write needed by an observable use, and every typed pointer view denotes the
same physical address as its native use**. Byte-definition families may name
different contents; they must not allocate independent C objects when addresses
can still alias.

### Early collection and preservation

Reuse the existing `RuleStoreVarnode` collection seam introduced for #1122.
Its `check_spacebase` result already proves an actual stack-space address, and
the STORE's full physical width is available before heritage narrows, combines
or drops the output. Add separate local evidence; do not insert local ranges
into `stack_parameter_store_extents` or change `eligible_extent`.

Collect positive local pointer-use evidence once in the first local native
STORE-lowering heritage window. Later evidence cannot authorize admission or
withdraw preservation after it has changed allocation. The scheduler runs
`ActionPrototypeTypes` and `ActionFuncLink` before the Band B loop, so locked
call contracts and materialized arguments exist. Explicit/DWARF global pointer
contracts also precede the drive. A fixed address must be established from the
current normalized graph; no later optimized type may stand in for that proof.

Two triggers belong to this same local-storage policy:

1. A fixed local extent has at least two distinct, complete nominal struct/union
   pointee contracts of the same size at declared call uses. Retain the original
   exact-component/full-coverage constraints before automatic union mapping.
2. A fixed local address is actually published to a declared persistent pointer
   receiver (or equivalent explicitly typed escape). The bounded pointee extent
   and native writes supply the footprint. Preserve the scalar slot even when
   no union is necessary; do not require an incompatible second type to keep a
   saved-pointer read correct.

For admitted footprints, extend the existing per-range store-hold query with
local evidence. Reuse byte-overwrite/observer handling rather than pinning all
frame stores. Unknown reads, indirect calls, unknown stores, partial overlaps,
loops and budget exhaustion cannot prove a write unobservable. Retain uncertain
writes within the positive footprint; do not infer a whole-function policy.
Before admitting any footprint, every live STORE or LOAD must prove a contained
or disjoint physical extent, using the actual stored-input or LOAD-output width.
Reads only supply negative admission evidence and never construct a backing or
native-write record. A reloaded receiver can prove containment only through
an uninterrupted same-block native publication; a merely possible relation can
refuse crossing but cannot prove equality. Unknown early evidence, crossing
writes and overlapping locked layouts decline the entire footprint. This
deliberately leaves some loop, callback and cross-block shapes unchanged.

Use the same positive footprint in `kuna_stackeffects::enabled_for` so an
unresolved write through an escaped/reloaded pointer starts a new memory
version of the admitted frame bytes. Reuse existing heritage barriers without
changing their callee-effect rules. Known literal global addresses remain
separate from unresolved aliasing pointers. No claim of `const` or noescape
follows from a pointee's declaration.

Physical-event identity must survive legitimate refinement using established
origin relationships. Do not set `stack_store` on arbitrary late copies, invent
initial bytes, or replay the arithmetic at a later observer. Reset local records
on `Funcdata::clear`/reflow, and bound storage and graph walks.

### Mapping and typed publication

Only after the early preservation contract is established may
`kuna_stackviews::gather` admit a local shared backing. Require the exact same
fixed component, complete equal-size aggregate contracts and full coverage.
Preserve all existing layout-lock, unresolved-index, view-count and overlap
gates. Keep local and narrow-formal admission reasons distinct.

Let existing `kuna_stackobjects` byte origins identify contents at declared
uses. A surviving native write is evidence; an implied arithmetic source behind
an unmarked reconstruction is not. Unknown effects stay unknown. Independent
snapshots or names are allowed only where existing proofs show they cannot
change saved or simultaneous aliases. There is always one physical backing for
an admitted shared address.

Capture the receiving pointer contract at the native publication/use boundary
before forced/speculative high-variable merges obscure the persistent receiver.
Keep the exact original publication from propagating into INDIRECT or
MULTIEQUAL markers, so existing `kuna_stackuses::prepare_casts` can still query
its receiver contract after mapping. Original raw STORE publications to exact
literal default-data-space destinations use the existing locked STORE input
contract before lowering; a known external frame publication without a validated
pending record declines admission.
Select an exact matching existing member address, or the existing explicit cast
to that declared pointer type at the same byte address. A generated union
pointer cannot be assigned to a locked `int *` merely because their bits agree.
An ambiguous receiver contract declines automatic view binding.

Keep `kuna_stackbytes`, `kuna_stackversions` and `printc`'s #1163 safeguards.
The design fixes their missing prerequisite rather than teaching them to
resurrect arbitrary hidden arithmetic or to ignore mutable-read timing.

## Owning modules and implemented seams

These modules define the approved implementation scope. The lift callback records original memory-operation identities before any optimizer can synthesize copies.

| Module | Responsibility |
|---|---|
| New `p6_variables/kuna_localstackviews.rs` | Bounded local policy, proof records and queries; one feature module. |
| `substrate/funcdata.rs` | Per-analysis state and reset lifecycle. |
| `p2_lift/flow.rs::FlowEmit::dump` | Capture bounded original STORE and persistent memory COPY identities before optimization. |
| `p3_dataflow/ruleaction_4.rs::RuleStoreVarnode::apply_op` | Record actual native STORE before replacement, next to #1122's hook. |
| `p3_dataflow/ruleaction_3.rs::RulePropagateCopy::apply_op` | Retain only validated native publications at their execution point through marker readers. |
| `p6_variables/kuna_stackalias.rs::{prepare,holds_store}` | Consult the local footprint when retaining physical writes; retain existing observer/overwrite logic. |
| `p3_dataflow/kuna_stackeffects.rs::enabled_for` | Add only positive local byte ranges to the existing scoped barrier query. |
| `p6_variables/kuna_stackviews.rs::{gather,prepare_casts}` | Admit exact protected local components and feed existing write/view selection. |
| `p6_variables/kuna_stackobjects.rs::recover` | Admit only these local backings to existing object-family recovery. |
| Existing `p6_variables/kuna_stackuses.rs` | Consume the preserved receiver contract with existing typed member binding; no changes to this module. |
| P0 metadata/dispatch, architecture reset, module registration | One flag, complete catalog row and supported ablation. |
| Existing owning spec chapters 02, 03, 06 and 09 | Describe original lift evidence, preservation, local eligibility and resulting C contract. |

`kuna_stackvalueorder` is a required review point: retaining an original write
must not allow its source to publish the backing before the native store.
Exercise the existing veto with the now-retained native event. If a change is
needed, it must be scoped to this contract and separately ablated; widening the
old width predicate alone is already disproved. No generic merge or emitter
bypass is authorized by this proposal.

No `kuna_stackvalueorder`, cleanup or emission algorithm change was needed.
The reviewed memory-summary/global-proof modules are consumers of existing
facts, not modification targets here. #1122's formal eligibility and default
oversize exclusions remain unchanged.

## Conservative gates and exclusions

- Initial support: byte-addressed architectural stack spaces, fixed local
  extents of 1..256 bytes, bounded per-function proof/scan budgets.
- Exclude incoming formal storage, overlaps with another formal, unrelated
  address spaces, wrapped/overflowing extents and unbounded/dynamic indices.
- Do not remap explicit locked user layouts or infer views from incomplete,
  incompatible-size or ambiguous contracts. A complete pointee type is not
  proof of read-only or noescape behavior.
- Keep unknown native memory effects and partial definitions conservative.
  Budget exhaustion before admission invalidates all local candidates. Evidence
  is bounded and frozen before preservation, so no later rescan can discard an
  already-applied decision. It cannot authorize a snapshot, dead-store removal
  or new default mapping.
- No global `stackviews`/`stackalias` default flip, arbitrary stack-object
  lifetime splitting, generic type synthesis, interprocedural alias framework,
  new memory-summary proof, unrelated representation fixes or baseline repair.
- Unsafe or unsupported cases may retain existing mapping; no successful
  native-output claim is made for a declined case. The original #810 witnesses
  are mandatory acceptance cases and cannot be relabeled out of scope.

## Resolved implementation questions

Original branch STORE70 lowers at stop191, with complete declared contracts
and fixed frame addresses already available. DCE214 releases the native local
write, copy propagation225 detaches its last reader, and early removal230
deletes it. The implementation decides admission in that first STORE-lowering
window and holds the original write before DCE. Reconstructed COPY238 does not
authorize admission. No discovery or reflow pass was added.

The saved receiver COPY13 first propagates into INDIRECT151 at stop183 and is
removed at187; the late high-variable merge then obscures its original pointer
contract. Original publication identities captured during lift now survive
INDIRECT and MULTIEQUAL propagation at their original execution point. Raw
STORE publications use the same locked receiver contract before lowering.
Existing typed-member binding then receives the original declared type.

Native correspondence uses bounded generational operation IDs, source points,
original widths and receiver storage. Admission and candidates are frozen in
the first window; live queries still validate original identity. Funcdata reset
clears the state, and partial jump-table recovery uses a fresh Funcdata. No
synthetic operation is relabeled as a physical event.

## Acceptance and default decision

The rejected write-only bounds check produced wrong output in all four strict
GCC/Clang O0/O2 cells for a declared four-byte receiver followed by an eight-byte
native read at offset four. That read spans two initialized adjacent eight-byte
regions; mapping the first region to an eight-byte union lost their physical
correspondence. The final early check refuses the whole footprint using the
actual read width before preservation or mapping. The same fixture now retains
exact prior C, including its existing invalid aggregate cast; no successful
emitted-C claim is made for that decline. A contained four-byte read remains a
strict native positive control.

A compiler-portability rerun exposed a second retained-write failure: a
register-packed Pair source was retyped as the backing union through a memory
marker and then hidden as a self-copy, returning 55 instead of 62. The original
STORE was still live. Preserving its exact COPY-to-marker edge now retains the
numeric `CONCAT44(b,a)` initializer; strict GCC/Clang O0/O2 checks cover the added
GCC SLP input. Read-only compiler constants are supplied from actual bounded ELF
bytes in the replay translation unit; emitted bodies remain untouched.

The mandatory originals pass strict GCC/Clang O0/O2 execution: 67, runtime
67/162, escaped alias 7, saved pointer 122 and simultaneous aliases 56. Independent
review also preserves all 192 narrow-formal regression cells. Of 168 experimental
local cells, 104 are supported native passes and 64 deliberately retain exact
prior C. Eight additional callback-crossing cells also retain exact prior C.
The read-width refusal conservatively declines eight previously supported cells
with an unknown external source LOAD. Loops, opaque effects and cross-block
shapes are not claimed as general new support. A separate corpus global-Handle
case also returns to its unchanged, already unsupported default C because its
final nested pointer read cannot be proved disjoint from the local component.
This deliberately loses an experimental repair rather than weaken bounds.
ARM-BE32 and MIPS-BE32 preserve the original scalar receiver type.

Actual default-OFF and default-ON both pass 675/675 without changing
`docs/baseline.json`. Twelve paired CPU-pinned whole-image rounds measure
default time changes of +0.32% on 154 healthy functions and +0.19% on 23 typed functions;
their wall-time 95% upper bounds are +0.48% and +0.49%, below 5%. The official
31-repeat firing measurement is +0.10%; these measurements stay within the
speed budget. Record.json retains
the CPU measurements and the earlier inconclusive unpinned run. All four
repository gates pass: 675 datatests, 3,135 stage assertions, 9,178 workspace
tests and spec validation. Tooling, lint, catalog and merge guards also pass.
The final 1,071-image comparison covers 192,504 functions per arm without C,
metadata or discovery changes. An additional 50 typed images cover 724
functions per arm: all 60 changed hunks across 48 functions were reviewed. Nine
prior experimental changes return to exact baseline C under the read refusal;
five lose an intended repair. Existing watchdog and fixture-load failures stay
unsupported, with no new failure identities. The cp/mv outputs match the full
validated baseline JSON without errors. The additional hunk only removes a
storage annotation; its statements are unchanged and strict native replay passes.

The approved acceptance checklist remains:

1. Promote one end-to-end local policy stage fixture: option off shows the
   issue; option on preserves the shared views and writes. Add persistent native
   CLI tests using separate helper TUs and strict prototypes. Reuse the existing
   issue fixtures where possible; re-record the stage baseline, never
   `docs/baseline.json`.
2. Native controls must include constant `stack_reuse=67`, runtime register
   inputs, repeated/partial writes, runtime zero/nonzero/negative branches,
   both branch arms, loops, byte/word stores, saved source values and both GCC
   and Clang optimized input shapes. Compile emitted C with GCC/Clang O0/O2,
   strict aliasing and incompatible-pointer/int-conversion errors enabled.
3. Default escaped-alias controls: original result7, saved pointer after
   replacement122, simultaneous view/address equality56, two reads around a
   write, partial updates, escaped indirect writes, callbacks, captured-pointer
   writes and observers before/after native stores. Calls with unknown effects
   and unresolved stores must not be treated as preserving memory.
4. Boundary controls: neighboring/overlapping locals and formals, wrong space,
   incomplete types, unequal aggregate sizes, locked layouts, bounded versus
   unbounded indexing, unknown source bytes, oversized stores and exhausted
   budgets. Rerun #1122's native/boundary tests with no eligibility expansion.
5. For all functions whose C changes in the selected whole-binary corpus,
   retain before/after output and classify every hunk for semantic correctness.
   Any moved/deleted/re-anchored statement needs an explicit native or source
   explanation. A second reviewer must build the implementation and attack
   wrong output, not merely confirm that the option fires.
6. Develop default-OFF and confirm unchanged default output. A default-ON trial
   must change 0/675 datatest assertions and pass all four repository gates,
   tools tests, lint, catalog validation and counter/baseline guards on the
   rebased tree. Re-measure after each rebase; no baseline re-pin.
7. Record interleaved repeated `scripts.pipeline.timeit` measurements for the
   changed witnesses and representative unaffected small/large functions, plus
   whole-binary runs. Default-ON requires the <=5% speed budget with a stated
   noise floor. Over-budget or unmeasured behavior remains opt-in and does not
   count as completing the requested default fix.

Only the completed, reviewed implementation and green native/default gates
justify closing #810. A design approval, original constant-only success, or
successful proposal documentation gates do not establish that closure.
