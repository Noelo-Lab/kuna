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

A focused whole-command timing on the optimized ELF example (ten measured runs
after two warmups) had median times of 167 ms before this change, 158 ms with
the option off, and 162 ms with it on. These process-level measurements show no
observed slowdown on this fixture; they do not establish a corpus-wide speed bound.

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

For the earlier seven-function cross-ISA fixture, the following whole-command timings
use two warmups and seven interleaved measurements per setting, with no build or
other test running concurrently. These are medians; the option remains off by
default. The measurement includes loading, analysis and C emission.

| Target | Off (ms) | On (ms) | Delta |
| --- | ---: | ---: | ---: |
| aarch64-linux-gnu | 180.6 | 179.1 | -0.8% |
| x86_64-linux-gnu | 197.0 | 192.8 | -2.1% |
| riscv32-linux-gnu | 63.1 | 56.0 | -11.4% |
| armv7-linux-gnueabihf | 137.4 | 133.3 | -3.0% |
| riscv64-linux-gnu | 71.9 | 70.5 | -1.8% |
| sparc-linux-gnu | 102.1 | 110.7 | +8.5% |
| sparcv9-linux-gnu | 115.9 | 120.9 | +4.3% |
| i686-linux-gnu | 189.2 | 175.5 | -7.2% |

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

Allocation lowering runs immediately after active call-argument recovery, while
the original SP relationships can still distinguish outgoing arguments from
untouched registers and the caller frame. An unasserted caller/callee regression
checks both inferred arities. CI installs Clang, LLD, QEMU user emulators and the
SPARC GNU linker; its dedicated round-trip step requires every tool and emulator
and prints the per-target native/recompiled comparison counts.
