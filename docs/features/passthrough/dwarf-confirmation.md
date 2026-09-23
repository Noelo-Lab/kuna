# passthrough — every gained parameter, against DWARF

The question round F left open: the option adds parameters and return values the
`type_match` metric cannot check, and `default-on-evaluation.md` confirmed 454 of
506 gained parameters over 26 binaries. This is the same check over the whole
decbench measurement corpus — **444 slices, 8 projects, three optimisation
levels** — with every function that gains something checked against its
unstripped twin's DWARF prototype, not a sample.

Method: `kuna decompile-all <stripped> --json` twice per slice (shipped default
and `--option passthrough on`), the recovered argument list read from
`variables[]` `kind == "arg"`, joined by `low_pc` to the twin's
`DW_TAG_subprogram` (following `DW_AT_abstract_origin` for gcc's concrete
out-of-line instances — without that hop 433 unnamed instances read as `void`
and inflate the contradicted count tenfold). Drivers: `.scratch/armsweep.py`,
`.scratch/dwarfjoin.py`, `.scratch/dwarfproto.py` in this lane's worktree; the
verdicts below are the ones they print.

A gained parameter is **confirmed** when the twin's prototype has a parameter at
that index, **contradicted** when the twin's prototype is shorter and not
variadic, and **no twin** when the twin has no `DW_TAG_subprogram` at that
address at all.

## Parameters

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| **all** | **2,857** | **2,730 (95.6%)** | **4 (0.14%)** | **123 (4.3%)** |
| O0 | 0 | 0 | 0 | 0 |
| O2 | 1,059 | 1,052 | 0 | 7 |
| O2-noinline | 1,798 | 1,678 | 4 | 116 |
| coreutils | 2,087 | 1,995 | 4 | 88 |
| shadow | 372 | 358 | 0 | 14 |
| diffutils | 153 | 148 | 0 | 5 |
| findutils | 102 | 94 | 0 | 8 |
| tar | 88 | 84 | 0 | 4 |
| grep | 50 | 46 | 0 | 4 |
| gzip | 5 | 5 | 0 | 0 |

1,421 functions gain a parameter (519 at O2, 902 at O2-noinline). **No function
and no call site loses one** (`lost = 0` over all 444 slices, and
`lostargs.py` reports `call_lost_args=0 param_lost=0`). Nothing moves at O0,
where the register is already named by an op.

Of the 2,730 confirmed parameters, 2,617 (95.9%) also agree with DWARF on the
type's *class*; 113 do not, and 110 of those are a real pointer parameter kuna
types `long`/`unsigned long` (`chroot` `0x6d10` takes DWARF `char *`, kuna says
`long`). That is the `ptr_char` gap the campaign already tracks, not a
fabrication: the parameter exists, and with the option off it was missing
entirely.

### The 4 contradicted parameters — one function, one shape

| slice | addr | DWARF | gained |
|---|---|---|---|
| coreutils/O2-noinline/mkdir | 0xc340 | `int savewd_save(struct savewd *)` | a1, a2 (`unsigned long`) |
| coreutils/O2-noinline/ginstall | 0x171c0 | same | a1, a2 |

Both are gnulib `savewd_save`, whose only forwarded register reaches
`open_safer(".", O_SEARCH, ...)` — a **variadic** callee whose recovered
prototype states a fixed third parameter. The option's variadic guards catch the
caller-side `xor %eax,%eax` setup and a variadic caller's own `test %al,%al`;
they do not catch a callee that is variadic in source but whose recovered list
closes over one vararg slot. One function in two binaries, 0.14% of the rows.

### The 123 rows with no DWARF twin

69 functions, **every one a single-statement forwarding thunk**
(`unsigned long sub_b3a0(unsigned long a0,unsigned char a1) { return
sub_b360(a0,a1); }`), and every one outside every `DW_TAG_subprogram` range in
the twin — the toolchain emitted these thunks with no debug entry, so DWARF can
neither confirm nor contradict them. Their gained list is exactly the callee's
own recovered list, which is the rule's whole statement. They concentrate in
coreutils/O2-noinline (88 rows), the selinux and `*_safer` wrapper families.

## Return values, and the rule this check tightened

The first pass of this sweep measured **212 contradicted returns** of 4,267 —
one shape, replicated: gnulib `version_etc_ar` is DWARF `void`, and kuna recovers
its callee `version_etc_arn` (also `void`) as returning `long`, because that
function's fallthrough path ends in `__fprintf_chk` and keeps the call's `RAX`
clobber. Stating that recovered return to a caller hands the error on.

`recovered_output` now refuses to state a return the callee did not compute:
every live RETURN must hand back a value produced in **every byte and on every
path** (`kuna_returnuncomputed::every_return_computes`). The existing pair
repair asks the relaxed version of the same question — computed if *any* input
of a phi or a `PIECE` is — which is right when choosing between two halves of a
wide return and wrong when deciding whether to believe the whole value;
`version_etc_arn`'s `CONCAT44(<leftover>, __fprintf_chk(...))` passes the relaxed
test and fails the strict one.

With the gate:

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| **all** | **4,060** | **3,986 (98.2%)** | **5 (0.12%)** | **69** |
| O2 | 1,891 | 1,883 | 1 | 7 |
| O2-noinline | 2,169 | 2,103 | 4 | 62 |

The remaining five are `fts_skip_tree` (mv, rm), `clear_random_data` (shred ×2)
and `print_total_stats` (tar): callees that DWARF says are `void` and whose last
computed value kuna reads as a result. 3,986 functions that printed `void f(...)`
against a non-void twin now return something.

The type class of a gained return agrees with DWARF on 3,464 of 3,986; the 522
that differ are `long`/`unsigned long` where the twin says a pointer, the same
under-typing as the parameters (116 more are a `char (*)[16]` spelling this
check's textual return-type parser cannot compare).

## Verdict

The gained parameters are real: 95.6% DWARF-confirmed, 0.14% contradicted in one
gnulib function, and the remaining 4.3% are thunks DWARF does not describe at
all. Nothing is ever lost. With the return gate the return arm is 98.2%
confirmed. This is the evidence `default-on-evaluation.md` was missing.
