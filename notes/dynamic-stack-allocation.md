# Runtime-sized stack allocation

`stackalloc on` recovers ordinary x86 stack allocation for C output. It is opt-in;
it does not model switching to an unrelated stack or coroutine context.

The transformation runs after heritage, before stack expressions are distributed
among individual loads and stores. It follows SSA definitions across basic blocks,
including allocation in conditional branches and loops. Each allocation site emits
`__builtin_alloca` at its original execution point. Repeated execution therefore
receives fresh storage.

A page-probe loop can advance a pointer and remaining-byte counter by equal
constant steps. Their difference is invariant. Predecessor-aligned PHIs, including
the zero-iteration exit, recover the original base and requested size without
assuming a trip count or removing probe side effects.

A second SSA analysis bounds stack adjustments below each allocation and reserves
space for outgoing arguments. Alignment contributes its maximum downward slack.
Fresh allocation roots separate successive buffers. Nonconvergent pointer bounds
are widened to unknown; if an unknown bound reaches a stack-pointer write, the
transformation declines. This preserves the distinction between an indexed memory
access and an outgoing area whose size cannot be bounded.

Equivalent address calculations must use the same allocated buffer. Replacement
requires matching allocation origins and dominance of every replaced use; PHI
uses are checked on their incoming edges. Matching scalar expressions does not
identify distinct loads, calls, or PHIs.

Fresh Clang examples cover PE32, PE64, and ELF64; multiple allocations, branches,
loops, scoped VLAs, alignment, and explicit page-probe loops exercise the broader
analysis. ELF cases compare native and regenerated functions over thousands of
sizes at both C optimization levels. Windows cases recompile with Clang and treat
incompatible pointer diagnostics as errors. These checks do not establish support
for arbitrary stack manipulation.

A focused whole-command timing on the optimized ELF example (ten measured runs
after two warmups) had median times of 167 ms before this change, 158 ms with
the option off, and 162 ms with it on. These process-level measurements show no
observed slowdown on this fixture; they do not establish a corpus-wide speed bound.
