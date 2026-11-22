# Bounded constructor-return recovery for stderr

The bounded implementation follows the approved docs-first proposal #1773.
It transports a complete image-verified callee profile into P9 under the existing
`rustfmt on/all` option. The requirements below remain the acceptance contract.
No new option is introduced.

## First profile and consumer

Accept only Rust 1.93.1 x86-64 little-endian SysV ELF, one byte per address,
unambiguous full `core::fmt::Arguments::new` identity, and the complete 17-byte
body in analysis.md. Require one executable immutable mapped extent of exactly
that symbol size with no conflicting alias, relocation, overlay, or extra tail.
Do not accept a matching prefix or select a profile from the printed short name.
The body specifies input0→low output word and input1→high output word, plus its
exact stack scratch writes. At constructor entry RSP = S, the writes are
[S-16,S-8) and [S-8,S). For a normal near CALL, the caller pre-call RSP = C
and S = C-8, so the written ranges are [C-24,C-16) and [C-16,C-8). Prove
this relation at the actual call site; the return-address slot is [C-8,C).
A changed byte or reversed mapping is a refusal.

Prefer the existing protected-span/profile pattern in
`kuna-analysis/src/analyzers/sourcelang/kuna_rustfmtio.rs` and
`kuna-decomp/src/p9_emit/kuna_rustfmtio.rs`. Factor only the protected-span code
needed for this exact profile. The profile belongs to the per-image architecture
alongside the existing formatter/writer facts. Retain address and protected bytes
and revalidate them against the current loader at consumption. Do not add an
unbounded callee decompiler or a generic ABI inference pass.

The first P9 consumer accepts only a direct `_eprint` call selected by the
existing `kuna_rustfmtliteral::print_sink`, with exactly two 8-byte inputs, no
result, Rust output, supported version and `rustfmt` enabled. Both inputs must be
bounded COPY/CAST chains ending in the low and high SUBPIECEs of the *same*
16-byte constructor CALL output and exact SSA definition. Match offsets 0/8
and widths 8/8 exactly, with no
reordering, PHI, partial owner, redefinition, alternate producer, or guessed field
name. Require the actual target and call contract to match the protected profile:
RDI/RSI inputs, RAX/RDX outputs, a complete 16-byte result and no hidden return
pointer. COPY/CAST transport must preserve each word's exact width and bits.
Explicit target/prototype overrides remain authoritative; conflicting
arity, storage or result contracts refuse recovery.

Restrict the first case to one default u64 Display argument and one aligned
16-byte descriptor, with the exact primitive callback already recognized by the
current formatter identity table. No nominal custom Display/Debug type is added.
Require the constructor and sink in one basic block in order, with no intervening
call, STORE, volatile operation, descriptor change, or other unclassified effect.
Prove the descriptor and live u64 payload are disjoint from the constructor's
exact scratch writes using the caller's current stack pointer and physical frame
provenance, exact byte ranges, and non-wrapping address arithmetic. For the
authored caller, the u64 payload is [C,C+8), the descriptor is [C+8,C+24),
and the immutable template is in a distinct mapped rodata range. These ranges
are disjoint from the profile writes, but the implementation must prove that
from current IR rather than trust the example. The first implementation also
requires both complete objects in [S+8, function-entry SP) and the same verified
frame at the sink; disjoint scratch alone cannot establish ownership of storage
below the caller SP. `stack_offset` relative to
function entry alone is insufficient for
that comparison: if the current call-frame relation cannot be proved by existing
IR, refuse this profile and implement that bounded prerequisite first. Do not
infer disjointness from source-language convention or stack-variable names.

Reuse `recover_inputs_at` at the constructor, after its result/transport proof,
with slots 1/2 and `MacroKind::Print(true)`. This bypasses the wrapper that requires
a result-free consumer while retaining the existing readonly-template decoder,
callback proof and `stack_cells` before the constructor. The adjacency and scratch
disjointness proof must establish that those cells and payload still describe
what `_eprint` reads. Render at the original sink with the original pointer
varnodes, preserving their live identities and evaluation counts. No call input,
prototype, global type, IR statement or earlier producer is rewritten or removed.

The first implementation intentionally leaves constructor routes to `_print`
unchanged. Ordinary already-supported stdout/stderr macro recovery stays byte
identical. A later stdout extension may reuse a proved profile, with separate
acceptance; do not change stderr destination as a shortcut.

## Acceptance before promotion into the active corpus

Add the seven-predicate isolated expectation to `tests/stages` with the
implementation and normal generated ledger/corpus-count update: whole stderr
macro, removal of the corresponding sink call, one producer on each route,
unchanged constructor-to-stdout, and the two optimized ordinary macros. Keep
its current two red outcomes as evidence until they actually pass. Preserve the
same-source optimized positive, option-off byte identity, C/Go output identity,
changed-constructor refusal, unsupported-version refusal and single producer in
all cases. The model alone is not pinned R13 parity.

Add focused IR negatives for mixed result owners, swapped or partial pieces,
wrong constructor output width, missing/duplicate descriptor cells, changed
callback/trait, writable or volatile template, aliased scratch, intervening
store/call, ambiguous symbol aliases, stale code/data overlays and explicit
per-call overrides. Keep the original runtime call in every refusal. Test the
same owner whose unrelated result use remains live; preserving the constructor
call must retain that use rather than reevaluating its inputs.

A native observer must compile only newly emitted code or an explicitly labeled
macro/effect projection. Bind the exact u64 descriptor storage and increment a
counter in an authored `r13_tick`; check 0,1,high-bit and maximum values,
wraparound, one invocation, exact stderr bytes and empty stdout. Wrong destination,
altered literal, duplicated producer and descriptor/payload mutation must fail.
Existing nominal `NonNull`/ArgumentType emission may prevent whole-function
recompilation; report that boundary and do not claim a projection proves it away.
Never execute the analyzed fixtures or pinned upstream binaries.

Before publication, run the four repository gates, tools and lint; measure a
small serial paired CPU sweep with the same frozen source/spec/compiler inputs.
Keep collection restricted to exact named constructors on the supported target;
all other images should pay only the existing target/name filter.

The implemented sink also accepts an unlocked empty parameter store only when
existing `final_input_storage` proves the two eight-byte RDI/RSI inputs in order.
Recovery must be finalized, error-free, known-model and unshifted. This consumes
physical call facts already recorded by P4; it does not invent a prototype.
