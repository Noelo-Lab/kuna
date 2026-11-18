# Runtime-sized stack allocation

`stackalloc on` recovers ordinary 16-, 32- and 64-bit stack allocation for C output.
It is opt-in; it does not model switching to an unrelated stack or coroutine context.

The transformation runs after heritage, before stack expressions are distributed
among individual loads and stores. It follows definitions feeding the physical stack pointer across basic blocks,
including allocation in conditional branches and loops. Function-lifetime storage
emits `__builtin_alloca` at its original execution point. A proved shorter native
lifetime can instead become a nested, aligned VLA whose C scope ends at the native
restoration boundary. Repeated execution receives fresh storage without retaining
expired loop-local arrays.

A page-probe loop can advance a pointer and remaining-byte counter by equal
constant steps. Their difference is invariant. Predecessor-aligned PHIs, including
the zero-iteration exit, recover the original base and requested size without
assuming a trip count or removing probe side effects.

GCC's `-fstack-clash-protection` (the default in Ubuntu, Debian and Fedora
compilers) instead lowers SP itself one page per iteration until it reaches
`SP - (size & -4096)`, then subtracts the residual `size & 4095`. That residual
is not the buffer, so a base carried around a loop by a nonzero constant step is
declined; recovering it as an allocation emitted `__builtin_alloca(size & 4095)`
and smashed the stack once `size` reached a page.

The cycle proof retains every member of an unfinished strongly connected
component, including completed PHI branches. Otherwise, visiting a zero-step
path first can hide a later page-sized step through the same nodes and wrongly
allocate only the residual again. Reconverging conditional probes are negative
regressions; zero-step cycles and acyclic joins with different offsets remain
valid controls.

Lower and upper SSA analyses bound stack adjustments and derived addresses on
both sides of each allocation, including outgoing arguments and memory access
widths. Alignment contributes its maximum downward slack.
Fresh allocation roots separate successive buffers. Nonconvergent pointer bounds
are widened to unknown; if an unknown bound reaches a stack-pointer write, the
transformation declines. This preserves the distinction between an indexed memory
access and an outgoing area whose size cannot be bounded.

Reservation analysis is read-only: displaced aliases are modeled as virtual
constant-offset additions until the entire reservation succeeds. Rewriting them
before a declined reservation can fight arithmetic cleanup indefinitely, as in
SPARC32 `dash`'s parser during jump-table recovery. The commit phase applies the
proved aliases and allocations together. An IR regression checks that a declined
reservation leaves the original alias definitions unchanged.

Equivalent address calculations must use the same allocated buffer. Replacement
requires matching allocation origins and dominance of every replaced use; PHI
uses are checked on their incoming edges. Matching scalar expressions does not
identify distinct loads, calls, or PHIs.

Freestanding Clang examples exercise PE32, PE64 and ELF64; multiple allocations,
branches, loops, alignment and explicit page-probe loops exercise the broader
analysis. Regenerated consumers compare native values and helper effects.
Recompilation treats incompatible pointer diagnostics as errors. These controls
do not establish support for arbitrary stack manipulation.

Allocation recovery participates in ordinary type propagation: arguments and
callees can acquire named record types even outside the allocation itself.
Function-body C is not a self-contained translation unit. For recompilation,
use a project export including its type header, or `--option structdefs on` for
the per-function definitions and JSON `types` records. With `structdefs` off,
that JSON list is empty by design. Replacing a recovered record with an integer
typedef does not supply the required type context.

A restored frame pointer can reach a later allocation through a memory load.
The allocation is identified by its runtime subtraction feeding physical SP,
not by requiring that the base's entire history remain visible in SSA. The
explicit option declares ordinary stack-buffer lifetimes; it does not implement
saved-context switching or intentional aliasing between live allocation objects.
The PE32 restored-frame consumer compares native and regenerated values, helper
call counts and relative buffer addresses, not the number of emitted allocas.

Requiring provenance only from the subtraction's base made equivalent instruction
orders disagree: assigning a restored base to SP before subtracting admitted it,
while subtracting in a temporary and then assigning SP rejected it. Rooting the
analysis at physical-SP definitions makes these orders use the same model.

## Scoped lifetimes and cross-ISA controls

Short-lived storage needs both a proved native restoration and a representable
lexical region. Basic-block splits preserve its beginning and end through both
structurers; ordinary block coalescing and conditional execution cannot erase those
boundaries. Values needed outside a scope are captured at their existing
definitions rather than replaying an expired array expression at the later use.
Pointer escapes and stores remain real operations.

A restore alone does not force a nested block: the current rule requires a later
observer of the restored frame. An allocation followed immediately by function
exit can therefore remain a function-lifetime `__builtin_alloca`.

The SSA transformation reads the stack register and ABI from the selected compiler
specification rather than matching a different instruction sequence for each CPU.
The round-trip matrix covers x86, AMD64, ARM, Thumb, AArch64, RISC-V32/64,
SPARC32/64, PPC32, PPC64 in both endians, and MIPS32/64 in both endians: 16 targets.

`stack_allocations_isa` uses independent freestanding C fixtures. Its 25 source
cases cover ordinary and consecutive buffers, branches, loops, alignment, outgoing
arguments, lexical VLA lifetimes, early exits, sibling and sequential scopes,
variadics, and scalar or pointer values used after a scope. Separate ascending and
descending assembly loops must decline unbounded outgoing-SP growth.

Reference code is compiled at O0/O2. Regenerated C is compiled for the same ABI at
O0/O2 and run under QEMU for 513 sizes per case, comparing return values, helper
call counts and ordered input digests. C AST checks also require the shorter
lifetimes to remain nested and end before later observers; a loop-local VLA must
remain inside its loop. No application artifacts or compiler runtime helper
implementations are required.

Run `cargo test -p kuna-cli --test integration stack_allocations_isa:: -- --nocapture`.
The tests use Clang, LLD and QEMU user emulators; SPARC32 additionally uses
`sparc64-linux-gnu-ld -m elf32_sparc`, overridable with `KUNA_TEST_SPARC_LD`.
Installed tools failing to compile, link or run are failures. Missing tools cause
explicit skips unless `KUNA_REQUIRE_STACKALLOC_ISA` requires complete coverage.
Each test retains its source and regenerated exports in the printed scratch
directory.

Embedded gcc-ia16 and Turbo C binaries exercise 16-bit allocation and saved-SP
recovery without requiring a DOS compiler in CI. Separate ABI fixtures check
Windows stack-probe reservations and Darwin AArch64 stack-passed variadic tails.



Alignment masks must retain at least one address bit: an AND with zero erases
the pointer and cannot establish alignment. Only an AND whose other operand is
derived from a recovered allocation contributes alignment slack; unrelated
integer masks cannot enlarge or suppress the backing reservation. Isolated IR
tests cover copied zero masks, both operand orders at 32/64 bits, and an unrelated
64-byte mask beside a one-byte access below the allocation.

Function-lifetime lowering still requires every enclosing loop latch and every
later reservation to retain the current backing. A reset to older storage cannot
be emitted as another function-lifetime alloca merely because its size is bounded.
When the shorter lifetime is proved, lexical lowering supplies the required end;
otherwise the transformation declines. Unknown cleanup, irreducible regions and
unproved pointer relationships do not establish retention.

Lifetime graph queries have a 65,536-edge budget and SSA queries have their own
bounded walks. Saved-frame read spans also have a cumulative 4,000,000-operation
limit; exceeding it declines recovery for the function rather than accepting
unproved reads. These are recovery limits, not permission to erase memory effects.

Before native stack definitions are held or an allocation is materialized, the
alias gather requires a physical-SP reservation witness. Callgrind exposed ordinary
negative frame indexing triggering recovery's fallback and a second full
decompilation. Reusing native-retention discovery avoids that restart; after
successful lowering, the settled state retains the pre-existing alias-gather pruning.


Saved frame links establish a shared origin only through bounded full-width
store/load or stack-SSA proofs. Intervening writes must be disjoint. Across a
call, the word must lie above the callee's finalized incoming stack area, and
its storage address must not escape through arguments or earlier publications.
These checks also apply when the word has a dynamic native-SP coordinate rather
than a fixed entry-frame offset. Existing stack upper bounds can prove that an
earlier stored frame pointer lies above the entire word; that is not publication
of the word's address. Unknown call footprints, aliases, memory effects and
exhausted budgets decline the proof. A local save is not itself an address
escape, and a value already loaded into SSA does not depend on later slot writes.
As with fixed-frame ownership, an object above the word is assumed not to index
backward into it through an enclosing frame pointer.

Allocation lowering follows active call-argument recovery and waits for every
live call's input recovery to finish. Recovery spans multiple heritage passes:
visiting its action once is not a completion barrier. Lowering SP earlier can
turn an alignment spill into phantom arguments or leave real outgoing stack
arguments disconnected from the call. Dead calls do not block the transformation.
After the required stack-SSA finalization pass, an empty candidate set withdraws
recovery directly, without rebuilding frame proofs. The initial proof and
additional heritage pass are unchanged.
Previously discovered, still-used roots also remain proof obligations while
arithmetic changes normal form. A temporarily hidden subtraction cannot turn a
failed whole-set proof into permission to lower just the other reservation.
Zero-sum constant steps around a negated size retain the original subtraction;
constant differences between equivalent sizes contribute to the alias offset.

Recovery can remove the COPY assigning an allocation expression to SP while the
same expression remains an argument-register value. A later call's return-address
push may then be the surviving physical-SP definition. While input recovery is
pending, existing root discovery follows actual physical-SP writes and records
the generational IDs of allocation expressions feeding them. An earlier expression
is eligible only when the assignment follows in the same basic block without an
intervening effect or trapping division; recording this provenance does not change
the IR. Late discovery revalidates equal width, the same SSA amount and a proved
constant displacement from the same base. Its backward walk stays in one basic
block and stops at calls, userops and control-flow boundaries. An arbitrary future
pointer calculation is not evidence that storage was already reserved. Per-function
reset clears the provenance, and filtered action clones begin empty.
Later displaced copies use the same backing reservation. The loop-lifetime proof
recognizes an affine copy only when the current reservation dominates that copy.
An older reservation, including equal reservations joined from alternative
predecessors, cannot prove that the current lifetime is carried around the loop.
Restoration of the allocation's older base is rejected as well.

An unasserted x86 caller is compiled with the real helper declaration, then
compared against native helper counts and values while that helper clobbers its
own stack. This rejects a discarded alloca or a pointer still derived from a
fixed local without relying on its printed name. A separate
Thumb fixture saves an unused incoming register before a three-argument call and
allocates its buffer afterward. Its regenerated caller must compile with the
real callee declaration, then preserve native helper counts and ordered effects
for 513 sizes at both C optimization levels. These are consumer checks, not
assertions about scheduler layout.

CI installs Clang, LLD, QEMU user emulators and the
SPARC GNU linker; its dedicated round-trip step requires every tool and emulator
and prints the per-target native/recompiled comparison counts.

## Interpretation limits

Recovered compiler backing and native frame coordinates are not source-object
boundaries. An ABI record can remain an initialized prefix when its unobserved
tail cannot be recovered. Fixed-spill folding can also remove variable/type
metadata without changing the consumer's operations.

Body-only recompilation needs the actual recovered record definitions and callee
context; replacing records with integer typedefs is not a valid substitute.
Byte comparison must distinguish instructions from literal-pool data, relocation
values and duplicate static names. Static audits and bounded authored consumers
do not establish whole-program or universal semantic equivalence.

Validation and performance measurements are reported in the
[PR body](https://github.com/Noelo-Lab/kuna/pull/1235).


## Bounds transfer preparation

Each bounds invocation now prepares its immutable transfer descriptions once.
Scalar folds, alignment choices and known call-pop facts are not recomputed
on every fixed-point sweep. The plan exists only while `Funcdata` is immutably
borrowed; no facts survive an IR mutation. PHIs retain an operation ID rather
than copying their input vectors. Root/displaced precedence, absent versus
empty values, operation order, unknown bounds and overflow behavior are
unchanged. Every scalar fold retains its independent 32-node budget, and the
64-sweep/eight-widening schedule and its exact change sets remain intact.
Allocation candidates, completed call-input recovery, alias/reservation proofs,
private saved-slot obligations and loop/sequential lifetime checks are unchanged.

## Native call effects and emitted storage

Held stack-pointer definitions consume a call's solved relative stack effect.
The call-associated `INDIRECT` becomes native-input-plus-pop, not an entry-SP
approximation or an unchanged pre-return value. A saved register must therefore
be read from its own native slot, not the call's return-IP word.

Function-lifetime allocation requests and scoped VLA extents convert the native
decrement to an unsigned integer at native width before adding compiler-only
backing slack in a wider type when one is available. A pointer-typed carrier
does not turn a byte count into a pointer argument or a non-integer VLA extent.
Recompilation requires that the full backing extent fit the consumer's `size_t`
and that storage plus the compiler's frame fit its stack. A native 16-bit
reservation near the segment limit can fit before padding but not after it;
using a wider expression prevents IR truncation, not truncation by a 16-bit
compiler's allocation builtin.

Explicit native pointer-alignment evidence is captured before ordinary mask
simplification. Alignment recovery does not infer physical-SP alignment from the
abstract frame's assumed low zero bits. Lowering preserves address congruence
through shifted bases, backing headroom and scaled counts. A rounded size alone
does not justify aligning an address. The exact full-width rotate-by-zero lift
can be normalized for the witness input without accepting different sources,
widths, arithmetic shifts or nonzero rotations.

Fixed-byte frame-view preservation retains an already witnessed finite byte
interval, bounded by current writes and independent read, lock, gap and width
limits. A short directly written prefix cannot replace a live callee-affected
tail. When no valid replacement is representable, recovery withdraws before
changing hints and fresh recovery-off analysis reconstructs the consumer object.
No callee-filled tail is inferred.

