# Proposal: caller-proved register-pair returns

Status: implementation approved by the user and validated on merged base
`74d0bab7238ceacccedc6d1d888d56762b9f71f2`; ready for review. The default
remains off in every mode. Approval authorizes this bounded
feature, not default activation or closing #904. The MIPS sealed-entry
precondition below tightens the original direct-transfer prerequisite; the
returnable baseline fragment remains an explicit refusal control. Initial
admission is sealed MIPS only; ARM remains an unchanged refusal control.

## Decision and initial scope

Add `callwide off|on`, initially default off, owned by one P4 module,
`kuna-decomp/src/p4_calls/kuna_callwide.rs`. With `callwide on` and
`bejoin evidence`, a complete proof may retain both output words and give a
producer and all its calls one ABI-order eight-byte integer representation.
`callwide off` preserves current output. Existing `bejoin off` and `bejoin on`
semantics remain unchanged. Declared producer output/storage always wins.

Initial targets are sealed non-PIC MIPS32-BE executables with a known
two-register integer ABI. No input-parameter regrouping, inferred typed
consumers, returned-call recursion, float/aggregate/vector/custom storage,
PPC/SPARC/AVR extension, new entry discovery, or general CFG/alias/points-to
framework is included. Missing proof is refusal. Open, address-taken, unknown
or indirect consumers cannot justify promotion.

The MIPS admission component below is part of this one feature because it
exists only to establish its complete caller set. General PIC/JALR/callback
closure would require a separate prerequisite design, not expansion here.

## Required proof

All four conditions must hold before publishing a producer verdict:

1. **Positive typed use.** At least one direct call's two return words reach
   ONE declared eight-byte integer formal of a known direct consumer, unchanged
   and in ABI order. Freeze declaration/DWARF/assertion provenance before
   recovery; a prototype lock or two inferred four-byte formals is insufficient.
   Verify exact ABI storage/ordinal/model. A bounded straight-line raw-pcode
   walk allows copies and exact byte-preserving PIECE/SUBPIECE only. Reject
   partial writes, swaps, arithmetic, spills/reloads, branches, intervening
   calls, opaque operations and incomplete delay bundles. Instruction-local
   temporaries do not persist into another instruction.
2. **Computed pair on every return.** Independently prove full producer-written
   provenance of BOTH words at EVERY live return. Neither entry leftovers nor
   unknown call clobbers qualify. Initially use straight-line leaf producers;
   reject calls, loops, indirect flow, incomplete paths, partial writes and
   work exhaustion. A typed consumer alone cannot prove a stale word computed.
3. **Complete callable consumer set.** Use the architecture-specific closure
   proof below and reconcile it against the final instruction/reference and
   established-entry model. Reject recursive or inter-function tail-call
   candidates initially. No address-taking/unknown call, unowned edge or
   missing reference may be ignored. A site limit is an all-candidate refusal,
   never sampling. Do not borrow x86-only closed-caller evidence for BE code.
4. **Representable use at every site.** Each call either ignores the result,
   has the same complete typed-forwarding proof, or reads the first register as
   a supported high piece. Conflicting declared output, unsupported low-only/mixed-width
   use or incomplete caller decoding refuses the producer globally. Preserve
   byte/short/boolean offsets within the BE high word. The positive site does
   not overrule another caller. An ignored output requires both words to be
   definitely overwritten before any read, or an explicit void boundary;
   an unknown return boundary is not proof of a dead result.

Proposed hard bounds: at most 64 KiB / 16,384 aligned words for the MIPS image
proof, 256 producer sites, 32 instructions / 1,024 p-code operations per local
forwarding walk, and 512 instructions / 20,000 operations per producer proof.
Enforce an aggregate per-image record/operation budget as well; no product of
these maxima may allocate unboundedly. Any exhaustion returns ordinary output
for the entire affected producer without retained tentative facts.

### ARM remains outside initial admission

The existing `CallGraph::arm_closed_sites` scan does not prove the complete
executed transfer set or canonical return-token provenance required here.
Computed BX tails and modified-LR returns cannot borrow the MIPS proof.
ARM callwide preflight therefore refuses; its full option-off/on function
records must remain identical. Its existing CLI scanner and consumers retain
their original behavior. A comparable ARM control proof is a separate future
prerequisite, not a second architecture framework in this implementation.

### MIPS completeness proves a sealed executable transfer set

Require fixed-width legacy-delay-slot MIPS32-BE ET_EXEC, no PIC/CPIC/abicalls,
interpreter, dynamic loader, imports, exports, alternate loader entrypoints,
initializers/finalizers, IFUNC, unsupported relocations, writable code,
executable stack, MIPS16/microMIPS/R6 or interworking. Scan every aligned word
in every initialized executable PT_LOAD, including headers, unclaimed gaps
and full delay bundles. Missing bytes or incomplete ownership refuse. Unowned
words must be NOPs.

The ELF entry is the sole external root and must be exactly one owned 16-byte
stub: `jal known_coordinator; nop; j root+8; nop`. The root's jump is checked
using the machine's actual pseudo-direct high-PC rule, including 256-MiB
boundaries. All 16 bytes must come from one immutable executable region. No
other transfer may enter any root instruction; its own jump to root+8 and the
coordinator's proved return to that link are the only permitted arrivals.
The root never returns to an unknown external EntryRA. It is not a promotion
candidate. The coordinator is an ordinary function, not a special unclassified
caller of the candidate producer.

Every other function is straight-line apart from direct JAL calls and proved
canonical returns. Conditional/unconditional branches, indirect calls, opaque
operations/syscalls, unknown targets, tails and recursion refuse the image.
Every JAL agrees exactly with the complete reference/ownership model. A JR-ra
or SLEIGH RETURN label alone is insufficient: prove its effective target is
the function's entry return-address token. Accept untouched leaf ra, or its
exact initialized fixed-private-frame save/reload. CALL link tokens differ
from entry tokens, and the post-delay RA must still be that call's link.
Prove SP/FP restoration after the complete return delay bundle. Every load
and store must use an exact aligned, same-function initialized private-frame
cell; unknown/outside or overlapping accesses refuse. Restored frame/control
tokens cannot escape through ordinary scalar registers. Called functions
keep their frames below entry SP, away from the caller's saved tokens.
All ABI-unaffected integer registers must also retain their entry values.

The fixed MIPS language emits `setISAMode` inside JR-ra's raw p-code. A separate
sealed-MIPS publication path must recompute the complete control proof before
allowing that one architectural mode operation. Match native `03e00008`, its
complete eight-byte delay bundle, the exact named user-op and
`ISAModeSwitch`/ra/pc storage, and the full mask/switch/PC dependency sequence.
Canonical internal JAL links are aligned, so this mode is zero. Skip only that
CALLOTHER; preserve the delay instructions and final RETURN. Other opaque
operations, modes, raw shapes or incomplete proofs still refuse.

This proves the **executed callable transfer set**, not that arithmetic can
never equal a code address. Numeric addresses may be constructed, but no
allowed instruction can execute them as indirect targets, publish them to
external memory, pass them to an external call or return them outside the
process. Induct over the validated direct-call DAG: each JAL has a known
internal target/link, every return uses its caller's exact token, and the
root then remains in its fixed self-loop. No symbolic arithmetic/points-to
framework or constructor blacklist is needed. Ordinary valid initial stack
state and execution from the sole ELF entry are the boundary assumptions;
external debugger injection is not part of ordinary program execution.

A returnable/open image is refused even if its direct calls otherwise look
complete. Keep all constructor witnesses without the sealed root as exact
refusals; with the root, numeric values alone do not create extra callers.
The native harness retains the identical unused root stub and all original
producer/caller/consumer addresses and bytes. Native execution uses a separate
entry outside the admitted proof image; this checks matched function behavior,
not a claim that the native harness itself meets sealed-process admission.
The original XOR consumer stays in scope when these control conditions hold.
This is a partial family, and #904 remains open for returnable/PIC/indirect or
otherwise unsupported images.

## One stable verdict, all affected outputs

Preflight against final code, full inventory and positive declarations before
any function or header is rendered, independently of target filtering,
callee-first order or worker scheduling. Publish immutable address/model keyed
facts, never live IR IDs. A caller encountered first cannot create a verdict
unavailable to a producer encountered first.

Stage positive declarations until the raw proof accepts a producer. Bind only
its used contracts, direct caller declarations and typed sinks; discard failed
candidate dependencies and park nothing on a zero verdict. If binding revokes
the first proof, permit one bounded republish before rendering and require the
exact same model, accepted producers and dependency set. Never mint a new
positive from replayed/inferred prototypes.

Newly required DWARF return types narrower than four bytes need a compatible
existing output-extension policy. The admitted MIPS ABI uses `narrowext compiler`
for this; `abi` and `off` refuse those candidates with unchanged output. This
feature never changes that option. Without this prerequisite, newly typed
narrow calls can print uninitialized padding in a wider-register consumer.
Keep the untouched helper/native oracle and default-policy refusal controls.

For an admitted producer, a targeted return-recovery hook retains both actual
trials before `derive_output_map`, even when a word feeds carry computation.
Do not globally relax ancestor-use scoring or return stripping. Own-return and
call joins use the same ABI order. Extend the existing `build_high_read`
interface so a first-register-only caller extracts the high piece of that
same wide output, including subregister offsets. Reject unsupported sites
before publication. No parameter or general function-pointer typing change is
part of this feature.

Project headers must declare an unselected proved-wide producer. Build that
declaration from the actual post-drive operands and finalized word storage,
including the ordinary empty recovered call-prototype store. Use a local
prototype snapshot without assignment, input regrouping or global JOIN
allocation. Canonical unknown eight-byte results retain the existing C
unsigned projection; an ignored result obtains only its width and physical
pair from the live proof. Unsupported actual contracts fail the export.

Carry private entry/type descriptors through workers and streaming. A selected
producer's own prototype may resolve signed versus unsigned four-byte input
words because their ABI bits agree; result representation and arity must agree.
External-only conflicting declarations remain errors. A stream can defer only
a conflict for an exact producer in its complete preknown selection, retaining
records in requested order within fixed caps until the actual selected
definition resolves it. Missing or incompatible definitions leave an explicit
failed export. Selection never supplies proof facts. Preserve ordinary
declaration capture after temporary C rendering and before body publication.

Maintain proof dependencies on effective image bytes, established entries,
flow/model, positive prototypes and options. Clear/rebuild on changed byte,
function, flow or prototype assertions, architecture clear/unload or ABI
changes. `Database::kuna_generation` alone is not an image-byte epoch. Reflows
seed fresh Funcdata views from the same immutable verdict; cached OpId or
VarnodeId values cannot survive. Late direct-target resolution may consume an
already complete verdict, not create a new promotion after consumers render.

A worker must reproduce the same full proof or receive validated immutable
facts for the same image/inventory/declarations. Chunk-local evidence is
insufficient. Streamed functions must not acquire changed contracts later.
If a remote front-end cannot provide complete facts or safe invalidation,
refuse promotion there explicitly; do not invent partial evidence.

Supported Architecture byte/context/model mutation APIs revoke already-seeded
Funcdata views. Read-only context access does not revoke an unchanged batch.
Embeddings that mutate raw translator or loader interior handles must explicitly
clear/reload the proof first; such external writes are not automatically observed.

## Integration seams

Paths below are relative to `decompiler/crates/`.

| Area | Change |
|---|---|
| `kuna-decomp/src/p4_calls/kuna_callwide.rs` (new) | bounded proof, MIPS admission, immutable verdict and refusal reasons |
| `kuna-decomp/src/p4_calls/kuna_bejoin{,/evidence}.rs` | combine source-specific verdicts; consistent joins and high-only reads |
| `kuna-decomp/src/p4_calls/coreaction_protos.rs` | narrow proven-pair retention hook before output-map derivation |
| `kuna-decomp/src/infra/{architecture,decompile_drive}.rs`, `substrate/funcdata.rs` | dependency lifetime, per-drive views and restart seeding |
| `kuna-analysis/src/analyzers/dwarf/callwide.rs` (new) | bounded explicit scalar declaration provenance; ordinary DWARF behavior unchanged |
| `kuna-console/src/{engine,project}.rs` | positive declaration provenance, shared preflight before rendering, invalidation |
| `kuna-cli/src/decompile_all.rs`, `decompile.rs`, worker/project drivers | full proof before selection or export; text console and JSON agreement |

Single JSON uses `decompile_json -> load_program`; plain text launches the
external console instead. A hook only in `load_program` is insufficient.
Whole/project headers and bodies must share the proof. Existing --assert plus
jobs>1 is unsupported: test supported parallel/stream paths with linked DWARF
on the consumer only, compiled in a separate translation unit, without giving
the producer debug types. Do not add assertion-worker support to this feature.

Option/catalog counts, generated docs, stage count/baseline and natural-language
P4 spec prose belong to this same implementation. This is a proposal-scale
cross-function contract, not a local join-order bug fix.

## Acceptance and default decision

The implemented sealed-MIPS subset passes all seven CLI acceptance tests and the
seven-assertion firing stage on the final actual-base release snapshot. The
actual generated C and headers pass selected, whole, parallel and stream MIPS
O0/O2 replay with eight vectors and both DWARF composition helpers. All four
gates, tooling, lint, strict spec and catalog checks pass. Default-off paired CPU
changes are -0.34%, +2.44% and -0.40% on the healthy154, sealed11 and sealed256
workloads. Explicit activation costs +1.35%, +1.90% and +9.52% CPU respectively;
the default remains off. [The analysis](analysis.md#implemented-feature-validation)
and `record.json` retain the exact evidence and remaining scope limits.

- The MIPS repro must retain the low word and replay correctly using its actual
  generated C AND header at O0/O2. Compare original target execution under an
  emulator or hardware, distinct-word/carry vectors and narrow controls.
  Include a narrow caller of the SAME admitted producer, not just a separate
  high-word producer. ARM remains a full-record unchanged refusal control.
- Refute typed-but-unrelated consumers, two 32-bit formals, wrong storage/slot,
  swapped halves, partial writes, spills, overwrite/branch/call bypass,
  leftover word on one return, a late 33rd site, low-only use, address tables,
  exports, gaps, budget exhaustion and declared narrow producer conflicts.
  MIPS negatives include JALR, moved/function-pointer-to-ra then JR-ra, unknown
  `ra` writes, wrong restore cell, aliasing stack write, wrong SP restoration in the
  return delay slot, PIC/abicalls and undecodable executable words. Refusals
  require full output equality with the off arm.
- Test single text/JSON, selected/whole, project C/header, serial/parallel and
  streaming where supported; reordered targets, declaration/byte/flow changes,
  clear/reload and restarts must preserve deterministic verdicts and output.
- Add a firing end-to-end stage, durable machine/runtime controls and before/
  after corpus sweeps classifying EVERY changed function. Preserve the existing
  typed/DWARF `bejoin off` controls. No claimed coverage or speed result may
  come from cases where the feature never fires.
- Run all four gates, required tooling/lint checks and catalog validation.
  Measure off/on whole-run time, including preflight, on small eligible BE
  images, large refusal images and non-BE controls, at least five repeats.
  Default-on requires zero changes to the 675 datatest assertions, no known
  wrong-C or interface mismatch, and no more than 5% slowdown. Otherwise ship
  default off. Never repin the upstream baseline to absorb regressions.

The proposal covers one typed caller-evidence family. Other architectures,
open/PIC/indirect consumers, ambiguous body-only inference, returned-call
recursion and ET_REL debug relocation remain outside it. #904 stays open until
sound default behavior is validated. Approval is for implementation only.
