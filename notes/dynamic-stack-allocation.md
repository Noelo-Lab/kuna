# Runtime-sized stack allocation

`stackalloc on` recovers ordinary 32- and 64-bit stack allocation for C output. It is opt-in;
it does not model switching to an unrelated stack or coroutine context.

The transformation runs after heritage, before stack expressions are distributed
among individual loads and stores. It follows definitions feeding the physical stack pointer across basic blocks,
including allocation in conditional branches and loops. Each allocation site emits
`__builtin_alloca` at its original execution point. Repeated execution therefore
receives fresh storage.

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

Fresh Clang examples cover PE32, PE64, and ELF64; multiple allocations, branches,
loops, alignment, and explicit page-probe loops exercise the broader
analysis. ELF cases compare native and regenerated functions over thousands of
sizes at both C optimization levels. Windows cases recompile with Clang and treat
incompatible pointer diagnostics as errors. These checks do not establish support
for arbitrary stack manipulation.

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
An isolated saved-frame fixture maps the fixed frame through the existing console
interface, then checks that both allocations receive storage.

Requiring provenance only from the subtraction's base made equivalent instruction
orders disagree: assigning a restored base to SP before subtracting admitted it,
while subtracting in a temporary and then assigning SP rejected it. Rooting the
analysis at physical-SP definitions makes these orders use the same model.

## Cross-ISA validation

The same SSA transformation now runs on x86, AMD64, ARM, AArch64, RISC-V32/64,
and SPARC32/64. It reads the stack register from the selected compiler specification.
It does not match an instruction sequence for each CPU.

Constants may still be copies or pure integer expressions immediately after
heritage. A shared 32-node budget evaluates these expressions with p-code integer
semantics. Treating an uncollapsed copy of a constant as a runtime adjustment
previously delayed recovery until stack expressions had already been distributed
into individual accesses. ARM BIC/BFC masks also require this early evaluation.
Alignment masks are recognized in either operand position.

Positive offsets matter as well as negative ones. SPARC's register-window area
and outgoing arguments can lie above the architectural SP. The upper analysis
reserves these addresses, including addresses stored in fixed frame slots before
being passed to another function. In the selected SPARC V9 biased register-window
ABI, the local-range header ends at 0x8ae and the physical SP is biased by 2047.
The transformation preserves that bias in its integer address result rather than
mistaking it for a misaligned C buffer address. The ABI rule is documented in the
[Oracle SPARC V9 ABI guide](https://docs.oracle.com/cd/E37838_01/html/E66175/advanced-2.html).
An unrelated custom stack convention requires its own verified model.

`stack_allocations_isa` uses independent freestanding C fixtures. Each target has
six positive cases: one buffer, consecutive buffers, conditional allocation,
repeated allocation, 64-byte alignment, and 12-argument calls with
stack-passed values. Separate ascending and descending assembly loops must decline allocation
recovery when outgoing SP growth is unbounded. Reference code is compiled at
O0/O2; regenerated C is compiled for the same target at O0/O2 and compared under
QEMU for 513 sizes per case, comparing returned values, helper call counts and
an ordered digest of observed input data and sizes. No libc, application artifacts, or compiler runtime
helper implementation is required.

Run `cargo test -p kuna-cli --test integration stack_allocations_isa:: -- --nocapture`.
The tests use Clang, LLD, and each QEMU user emulator. SPARC32 uses GNU
`sparc64-linux-gnu-ld -m elf32_sparc` because LLD lacks that emulation; an alternate
linker path can be supplied with `KUNA_TEST_SPARC_LD`. Its Clang-generated 32-bit
code runs with `qemu-sparc32plus`. Missing compiler/linker tools cause an explicit skip; missing QEMU skips only
execution while retaining compilation and decompilation checks. Installed tools
failing to compile, link, or run are failures. For complete local
coverage, all eight emulators and both linkers must be present.

The SPARC runs disable `noreturn_disc`:
the fixture helpers are authored as returning functions and verified to return
under QEMU. Existing no-return inference otherwise incorrectly truncates these
callers. Those controls isolate stack-allocation recovery; they do not establish
that default SPARC no-return inference is correct.

The complete fixture diff covers 96 functions: six functions, eight targets and
two original optimization levels. Every helper call site remains present. The
changes are classified by source case:

| Case | Output change checked |
| --- | --- |
| One buffer | Raw frame-relative address becomes allocated backing. |
| Consecutive buffers | Each live buffer receives independent storage. |
| Conditional allocation | Branch-local storage or the compiler's merged size is preserved. |
| Repeated allocation | New backing remains inside the loop. |
| Scoped VLA | Recovery is declined when a loop restores its earlier SP; compiler alloca cannot preserve the shorter lifetime. |
| Aligned buffer | Alignment masks and bounded slack preserve 64-byte alignment. |
| Outgoing arguments | Stack argument slots and buffer accesses fit their reserved backing. |

Each test prints its scratch directory and retains `O0-off`, `O0-on`, `O2-off`
and `O2-on` project exports, so the full per-function diff is reproducible. Values,
helper call counts and ordered input digests are compared for all 513 sizes; this
also checks that control-flow simplification has not removed or reordered calls.

Engine CPU measurements use a 138-function, 19-binary cohort covering all ten
corpus ISAs, including large looping functions, allocation consumers, the
SPARC32 `dash` parser, and DecBench's unoptimized `dash` `readtoken1`. Three
independent persistent workers run the base, default-off head, and enabled head
on the same pinned CPU in one allocation. Each arm has two warmups and seven
measured rounds; arm order rotates and reverses between rounds.

`CLOCK_THREAD_CPUTIME_ID` clocks only the actual `decompile_one` API and,
separately, the existing `print_c` call. Startup, loading, preparation,
serialization, and destruction are excluded. The combined median is calculated
from each round's sum, not by adding the two separate medians.

| Median cohort CPU (s) | Base | Head, default off | Head, enabled |
| --- | ---: | ---: | ---: |
| Decompilation API | 33.797 | 34.094 | 38.637 |
| C emission | 0.285 | 0.289 | 0.287 |
| Combined | 34.082 | 34.382 (+0.88%) | 38.922 (+14.20%) |

All measured C and status outputs are stable within each arm. The enabled
analysis cost is real, not a startup effect or a corpus-wide upper bound.
`readtoken1` has combined medians of 2.093 s at base, 2.049 s default-off,
and 2.119 s enabled. The SPARC32 parser's corresponding medians are 0.258,
0.269, and 0.401 s; its full-corpus CLI run also completes within the unchanged
60-second per-function budget.

Alignment masks must retain at least one address bit: an AND with zero erases
the pointer and cannot establish alignment. Only an AND whose other operand is
derived from a recovered allocation contributes alignment slack; unrelated
integer masks cannot enlarge or suppress the backing reservation. Isolated IR
tests cover copied zero masks, both operand orders at 32/64 bits, and an unrelated
64-byte mask beside a one-byte access below the allocation.

Loop lifetime proof is separate from outgoing-area bounds. For every enclosing
natural loop, each latch must carry the allocation through its SP PHI; a restored
base, a missing proof, or an irreducible loop is declined. The graph walk has a
65,536-edge budget and the shared SSA proof a 4,096-node budget. Scoped VLAs are
negative cases, including a 20,000-iteration allocation that would exhaust the
stack if emitted as alloca. True function-lifetime allocations in loops retain
their existing round-trip coverage.

Allocation lowering follows active call-argument recovery and waits for every
live call's input recovery to finish. Recovery spans multiple heritage passes:
visiting its action once is not a completion barrier. Lowering SP earlier can
turn an alignment spill into phantom arguments or leave real outgoing stack
arguments disconnected from the call. Dead calls do not block the transformation.

Recovery may also retarget the original subtraction's result to an argument
register, leaving a later call's return-address push as the surviving physical-SP
definition. While input recovery is pending, the action records the generational
IDs of actual physical-SP definitions without changing the IR. Root discovery
can promote only one of those witnessed reservations, with an equal-width
subtraction of the same SSA amount and a proved constant displacement from the
same base. Its backward walk stays in one basic block and stops at calls,
userops, and control-flow boundaries; an arbitrary future pointer calculation
is not evidence that storage was already reserved. Per-function reset clears
the provenance, and filtered action clones begin empty.
Later displaced copies use the same backing reservation. The loop-lifetime proof
recognizes those same affine copies, but not restoration of the allocation's
older base; scoped VLA rejection remains independent of address recovery.

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
