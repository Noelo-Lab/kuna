# Proposal: preserve one native frame STORE16 as one backing object

A nonconstant sixteen-byte copy can become two independent eight-byte locals,
while the program passes the address of the original sixteen-byte destination
to another function. Preserve one proved native STORE16 footprint as one
physical byte object so every derived lane and escaped address refers to the
same storage. The [stock reproduction](analysis.md) demonstrates the failure.
This is a design proposal; it changes no engine behavior or acceptance baseline.

## Initial scope

Start with ordinary nonvolatile memory in Linux AMD64 SysV, one original
STORE16 to a fixed address in the analyzed function's own local frame, and a
same-block use passing that exact address to a direct call whose complete memory
contract is bounded to the same footprint. The value may be
zero or a supported nonconstant copy. Admission proves storage only: it never
asserts that a register is zero or that bytes have a nominal source-language
type. Keep the native writes and their order; do not insert a late synthetic
memset in place of their SSA effects.

The intended visible result is an actual sixteen-byte backing, with all bytes
written before the observer receives its address. Either a supported full-width
byte copy or ordered representation copies into that backing are acceptable.
Two unrelated scalar locals handed to a value adapter are not acceptance.

This is a strict correctness fix, with no new option whose disabled state merely
restores the false storage relationship. Existing explicit layout, full
`stackviews` policy and emission options retain their authority. New logic
belongs in a small P6 module using the existing provenance interfaces; the
implementation must update the owning variables/merge spec and any affected
memory-effect or emission contract. Multiple phase consumers make this a
proposal-sized change even though its first accepted footprint is sixteen bytes.

## Admission and refusal

An accepted candidate must satisfy every condition below before any shared
backing, type hint or alias-protection state is installed.

| Obligation | Admit | Refuse |
|---|---|---|
| Origin | One live original STORE, unchanged source point and exact width16, captured before transformations | Same-PC clone, reconstructed COPY, stale operation generation, width mismatch |
| Address | Fixed full-width byte address; own-local frame; nonwrapping `[base,base+16)` | Dynamic index, wrong space, incoming formal/return-address overlap, unresolved frame base |
| Containment | Every intersecting known access is inside the exact footprint and retains its ordering | Crossing access, unresolved alias that can invalidate the proof, incompatible overlapping candidate |
| Authority | Unlocked raw storage or an existing compatible complete layout | Conflicting user-locked declaration; inference from nominal name or pointer pointee alone |
| Use | Exact footprint address reaches one direct same-block observer with a complete bounded access/escape contract | Pointer declaration alone, incomplete callee summary, unbounded read/write/escape, different displaced address, branch-sensitive use beyond the initial proof |
| Value | Existing zero or a named/snapshot value that can be emitted without changing evaluation time | Invented zero; unsupported wide expression; implicit reload across a call or possible write |
| Effects | Nonvolatile operations, real definitions retained, contained later writes represented at their original point | Volatile/atomic behavior changed, missing lane definition, unsupported register overlap |
| Limits | Existing finite scan/footprint caps; complete evidence during the early window | Budget exhaustion, unavailable early evidence, or a previously refused footprint |

### Unresolved consumer contract

The stock fixture's authored observer body reads exactly the two eight-byte
halves, but the current normal importer does not carry that complete memory
contract. An `unsigned char *` declaration, direct target, or matching base
address is insufficient. The existing local-access scan covers caller accesses;
callee preserves/unobserved queries do not establish a complete bounded reader.

Before implementation can accept this escaped-object case, identify an existing
complete body-derived or declared memory contract and bind it to the actual
pointer origin. Prove every read, write and possible escape is contained. If
that evidence is unavailable, decline admission. A new consumer-summary
analysis is a separately reviewed prerequisite, not an assumed part of this
backing change. The reproducible defect remains valid; this proposal does not
claim its safe admission is already available from one store alone.

Two scalar STORE8 operations do not gain this proof merely because they are
adjacent. A pair of lanes derived from the captured STORE16 may share its
backing because the original physical extent is independently known. Multiple
STORE16 ranges, gap filling and larger unions remain separate work.

## Transactional implementation plan

1. **Collect without mutation.** Reuse the original-operation record and fixed
   frame-address resolution. Form a bounded candidate containing the exact
   extent, source identities, lane definitions, intersecting accesses and the
   escaped-address use. A rejected or incomplete candidate changes nothing.
2. **Plan the entire representation.** Validate frame ownership and locked
   layout before selecting a backing and all lane/view placements. Prove a
   supported emission for every affected write and address. Preserve a loaded
   snapshot; do not replace it by a later reread. For memory copies, use a
   memmove-safe path or prove disjointness before selecting memcpy.
3. **Commit the whole footprint.** Install the backing and its placed views
   together before heritage consumes the required definitions. Preserve the
   original memory operations or equivalent complete definitions and alias
   ordering. No intermediate state may map only the first lane or protect only
   some writes. If existing mapping APIs cannot guarantee this, stage their
   changes in a temporary plan and validate again before applying them.
4. **Revalidate identities on reuse.** Reflow clears derived records. Changed
   operation generations, late conflicting declarations, unavailable source
   values or incomplete containment invalidate the candidate rather than
   reopening earlier refusal. Before commit this discards the plan. After commit
   it requires complete rollback or a full function reflow that clears the
   backing, hints, view placements and alias-preservation state together; merely
   dropping the proof record is insufficient. Unrelated regions remain unchanged.
5. **Use existing emitters narrowly.** Retain `emit_wide_copy` where its complete
   mapped-piece/source contract applies. Scalar lane representation copies
   must target offsets of the same backing and avoid invalid typed aliasing.
   The current helper gates are not a license to widen all scalar widths or
   enable the broader stack-view policy.

## Separation from existing features

| Existing behavior | What remains unchanged |
|---|---|
| R14 user names, per-output types and C/Rust isolation | This proposal infers no nominal Rust Result type, persists no new user assertion, and cannot replace a locked layout. Repeat C→Rust→C and explicit name/type controls must remain stable. |
| Native-width retyping and partial-access emission | Definition-bound retypes and parameter assertions enforce their native widths. Explicit composite stack declarations may intentionally cover a wider interval under wrap/overlap checks. This proposal preserves both authorities and changes neither scalar/register widths nor Rust STORE syntax. |
| Full `stackviews` and typed local publications | Their broader policy and refusal rules remain intact. No automatic sixteen-byte contract is inferred from two nominal names or adjacent locals. |
| Rust array zero/initialization support | Value emission does not prove variable grouping or pointer reachability. No generic wide-expression lowering is bundled here. |
| REP and bounded byte-copy loop recovery | No loop is removed or expanded; their current frame/effect guards stay intact. |
| Stack-object and Go standard-library metadata | A byte-origin record means a write occurred, not that bytes are zero. No metadata schema or standard-library signature is added. |

The explicit-width distinction is implemented by
`kuna-console/src/kuna_hightarget.rs` (`check_stack_width` versus `check_width`)
and `kuna-console/src/kuna_paramasserts.rs`. Existing explicit composite stack
assertions are user authority, not automatic physical-store evidence.

## Acceptance before implementation can ship

The first end-to-end stage must retain one complete sixteen-byte backing and
its real observer call for the stock copy red. A native observer must read all
sixteen bytes through that address, with distinct input halves and canaries;
repeat at optimized and unoptimized C settings. It must not adapt the observer
to receive separate scalar locals. Check that the source is evaluated once and
that contained low/high/byte writes keep their order and leave other bytes intact.

| Positive/control | Required refusal or unchanged comparison |
|---|---|
| Explicit zero16, nonzero copy16, distinct source halves | Unknown XMM15 remains unknown; no Go authority is inferred |
| Unaligned sixteen-byte local and safe snapshot copy | No invented vector alignment; overlap cannot use unproved memcpy |
| Two lanes with one original STORE16 identity | Unrelated STORE8, duplicate source point, dead/stale original operation |
| Contained byte/high-half overwrite | Crossing write, ambiguous alias, volatile operation |
| One exact local backing and escaped address | Incoming stack formal, conflicting explicit local layout, dynamic address |
| Repeated analysis, source-preserving reflow | No stale backing after changed declarations or a refused early proof |

Retain the existing local-stack-view refusal tests, native-width stages and
C/Rust isolation controls. Run the complete repository gates and relevant Python
tooling before a commit, preserve the vendored 675-assertion baseline, regenerate
the actual stage ledger only after the new complete run, and measure the
default-path speed impact with matched tools. No speed or runtime result is
claimed by this proposal.

## Relation to Go stack-clear parity

The pinned Go 1.27.1 stack-clear test expects zero-filled extents 200 and 264 with
its counted loops removed. It separately requires a proved zero source at every
store, a stride/count/tail union, and preservation of later overwrites. The
narrow XMM15 return proof and this physical-store proof address different facts.
This proposal neither admits the parameterless pinned main to that classifier
nor proves the larger loop, joins its adjacent stores, changes the 256-byte
metadata cap, or implements other standard-library idioms. G19 remains open.

## Open questions from review

The escaped-address gate looks inverted. Keeping one 16-byte object for the
footprint of a single STORE16 is never less faithful than two split
eight-byte locals, whatever the callee reads, because the program wrote
those 16 contiguous bytes before passing their address. A callee contract
would be needed to justify splitting the footprint, not to keep it whole.
As written, the gate requires a contract that the proposal says is not
available, so its own witness stays unfixed. Can the escaped use be
admitted without a callee contract?
