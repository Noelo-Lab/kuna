# Analysis — `test_ternary_propagation_2::print_only_size` (angr ternary propagation)

## Opportunity

- angr testcase: `test_ternary_propagation_2`, function `print_only_size`
- Binary: `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/du.o` (ELF **ET_REL** `.o`, x86-64)
- angr version: 9.2.213

## What angr does better

angr collapses a value-producing control-flow **diamond** into a single C **ternary**:

```c
long long print_only_size(unsigned long long a0)
{
    char *v2;
    ...
    v2 = (a0 == 18446744073709551615 ? dcgettext(NULL, "Infinity", 5) : human_readable());
    fputs_unlocked(v2, stdout);
    return ...;
}
```

The underlying shape (from the disassembly) is:

```
if (rdi == -1)              ; cmp rdi,-1 / je
    v = dcgettext(NULL,"Infinity",5);
else
    v = human_readable(...);
fputs_unlocked(v, stdout);  ; both arms converge here
```

i.e. an `if/else` whose **both arms are side-effecting CALLs** that each assign the
merge variable `v`, followed by a single use of `v`. angr's region simplification
("ternary propagation") rewrites this into `v = cond ? f() : g()`.

## What kuna does

**kuna produces no output at all.** Loading *any* function in `du.o` — even the trivial
`timetostr` stub — fails immediately at `load function` time:

```
/home/.../du.o successfully loaded: x86:LE:64:default:gcc
Execution error: Unable to load 512 bytes at r0x00405290
```

So this is a **whole-file loader failure**, not a per-function or structuring problem.
The full side-by-side is in `angr-vs-kuna.txt`.

## Root cause (two independent gaps)

### Gap 1 — loader (the immediate blocker)

`du.o` is an ET_REL object, loaded by the **`relocobjects`** feature
(`kuna-analysis/src/s1_loader/elf_reloc.rs` + `loadimage_object.rs`, default-on, DIV-7).
That loader lays the `SHF_ALLOC` sections at a synthetic base and binds *undefined*
symbols into a synthetic "extern area" above them. Per the code's own comment
(`elf_reloc.rs:239`):

> Pure data externs (e.g. `stdout`, referenced by PC32 to an extern slot) …

are given a synthetic address (here `0x405290`) but the synthesized load image provides
**no backing bytes** for that extern region. `print_only_size` references `stdout` via
`R_X86_64_PC32` (`mov rbp, [rip+disp] # stdout`), so any read through the extern region
aborts the whole load with `Unable to load 512 bytes at r0x00405290`
(`loadimage_object.rs:598`). The object additionally uses `R_X86_64_REX_GOTPCREL`
relocations in other functions which the loader logs as
`unhandled kind GotRelative (skipped)`.

Owning stage/tier: **S1 loader** (`relocobjects`).

Fix sketch: back the synthetic data-extern region with zero bytes (so reads through a
data extern like `stdout` succeed) and/or handle `R_X86_64_REX_GOTPCREL` by materialising
a GOT slot that points at the extern. This is the angr CLE "extern object" backing-page
behaviour.

### Gap 2 — structuring (the named angr advantage)

Even with a working loader, kuna (like Ghidra) would render the diamond as a literal
`if/else`, **not** a ternary. Ghidra deliberately emits `?:` only from its
`RuleConditionalMove` — a *single, side-effect-free* `MULTIEQUAL`. Here both arms contain
side-effecting CALLs (`dcgettext`, `human_readable`), so no existing rule fires.

Producing angr's ternary needs a new **S8 structuring** rewrite that recognises an
`if (c) { v = f(); } else { v = g(); }` diamond merging into a single use of `v`, and
rewrites it to `v = c ? f() : g();` — comparable in spirit to the existing S8 readability
rewrites `kuna_branchflip.rs` / `kuna_gotoreduce.rs`, but harder: it must fold two
assignment arms plus their merge into one statement and likely add `?:` emit support for
the side-effecting case.

Owning stage: **S8 structuring** (real pass order in `universalaction.rs` /
`s8_structure/coreaction_*`).

## Hypothesis / why this is a PROPOSAL, not a one-pass feature

Closing this opportunity requires **two independent pieces across two tiers** (S1 loader +
S8 structuring), each non-trivial, plus new `?:` emit support — and the named target cannot
even be loaded until the loader piece lands, so no end-to-end before/after witness exists
today and a firing stage test could only be synthesized from a contrived externless
bytechunk. This exceeds the worker's "one option-gated `kuna_*.rs` Action/Rule" budget on
multiple axes, so per Hard rule 7 it is routed to a `[PROPOSAL]` draft PR for human go/no-go
rather than implemented. See `proposal.md`.
