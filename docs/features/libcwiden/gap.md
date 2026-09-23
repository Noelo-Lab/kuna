# Where the last of the `char *` gap to binja is

Re-derived on `main` `31ac2b26e` (engine-identical to `810b7dc86`, the round-F
binary — the only commit between them is the results document), with the metric
pinned to decbench `625e892` exactly as `final-c/pindb.py` pins it. The kuna
decisions are round F's own `--dump-decisions` rows
(`docs/decbench/typecampaign/final-f/`), the rival decisions the round-A replay.
Every script that produced a number here is in this folder.

## The gap today

Over the campaign's 444 slices there are 14,645 ground-truth `char *` variables.
kuna is right on 5,060 (34.6%), binja on 5,323 of the 14,469 it is scored on
(36.8%) — a **net 263**. That net is two much larger opposed piles: binja alone is
right on **878** and kuna alone on **556**.

| storage | shared GT | kuna | binja | binja-only | kuna-only | net |
|---|---:|---:|---:|---:|---:|---:|
| argument | 4,064 | 2,843 | 2,907 | 384 | 320 | −64 |
| stack local | 3,311 | 2,158 | 2,365 | 443 | 236 | −207 |
| register only | 7,094 | 0 | 51 | 51 | 0 | −51 |

| opt | shared GT | kuna | binja | binja-only | kuna-only | net |
|---|---:|---:|---:|---:|---:|---:|
| O0 | 4,137 | 3,081 | 3,111 | 316 | 286 | −30 |
| O2-noinline | 5,476 | 1,190 | 1,352 | 319 | 157 | −162 |
| O2 | 4,856 | 730 | 860 | 243 | 113 | −130 |

`slotptr` (#710) moved the -O0 stack pile; what is left is no longer an -O0
problem. It is an **-O2 problem**, which is where the register-only ceiling and
the missing inlined-callee variables also sit.

## The 878 binja-only variables, split three ways

`attrib.py` joins each row to the DWARF twin's `DW_TAG_subprogram` (is the
containing function variadic?) and to an `objdump` call graph (does it call a
libc import kuna has no signature for?).

| bucket | rows | arg | stack | reg |
|---|---:|---:|---:|---:|
| **(a)** the containing function is variadic | 52 | 52 | 0 | 0 |
| **(b)** the function calls a libc import kuna cannot type | 158 | 59 | 99 | 0 |
| **(c)** register-only ground truth — nothing can reach it | 51 | 0 | 0 | 51 |
| **(c)** everything else | 617 | 273 | 344 | 0 |

What kuna spells instead, per bucket:

| bucket | the spelling kuna exports |
|---|---|
| (a) | `unsigned long` 49, no variable 3 |
| (b) | `integer` 70, `void *` 36, `undefinedN` 22, a named record `*` 19, `int *` 8, `T **` 3 |
| (c) other | `undefinedN` 174, `integer` 252, `void *` 70, no variable 76, `int *` 23, named `*` 13, `struct_N *` 7 |

### (a) The variadic register save area — 52 rows, one shape

Every one of the 52 is an **argument**, in one of eleven functions, replicated
over 13 binaries and all three optimisation levels: `copy_attr_error` and
`copy_attr_allerror` (9 each), `prog_fprintf` (9), `test_syntax_error` (6),
`nl_error`, `wrapf`, `printf_errno`, `checked_fprintf`, `report_difference` (3
each), `rpl_fprintf`, `__printf__` (2 each). The ground-truth variable is always
the format string (`fmt`, `format`, `message`), and kuna always spells it
`unsigned long`.

kuna is not wrong about the type. It is wrong about the **index**. coreutils cp
-O0 `copy_attr_error(struct error_context *, char const *fmt, ...)` at `0x7b11`
decompiles to

```
void sub_7b11(unsigned long a0, ..., unsigned long a13)   /* fourteen parameters */
```

because the SysV varargs prologue stores the argument registers into the frame
and nothing reads them first:

```
7b2e:  mov %rdx,-0xa0(%rbp)     7b35:  mov %rcx,-0x98(%rbp)
7b3c:  mov %r8, -0x90(%rbp)     7b43:  mov %r9, -0x88(%rbp)
7b4a:  test %al,%al   je 7b6e
7b4e:  movaps %xmm0,-0x80(%rbp)  ...  7b6a: movaps %xmm7,-0x10(%rbp)
```

Prototype recovery reads those twelve stores as twelve unused input parameters,
orders the XMM ones first, and the real `char *fmt` (rsi) ends up at `a9`. The
metric compares ground-truth argument 1 against kuna's `a1`, which is `xmm1`.

Measured over 42 variadic functions in six binaries (`variadic.py` plus a
per-function `decompile-all --addr`): **26 of them declare exactly 14 parameters
against a DWARF arity of 1 to 4**, and the sample carries **306 phantom
parameters** in total. The three that kuna gets right (`open_safer`,
`openat_safer`, `rpl_fcntl`) are the ones gcc compiled without a save area at
all. Across the 441 DWARF twins there are 735 variadic function instances under
28 distinct names.

The shape is the ABI's own `reg_save_area`, and it is identifiable without
guessing: there is a base `B` such that the stored general registers are a
**contiguous suffix** of `(rdi, rsi, rdx, rcx, r8, r9)` at `B + 8i`, and all
eight XMM registers sit at `B + 48 + 16j` under a `test %al,%al` guard — the
176-byte block `va_start` records in the `__va_list_tag`. In the `cp` case
`B = rbp-0xb0`: `rdx` is at `B+16`, `r9` at `B+40`, `xmm0` at `B+48`,
`xmm7` at `B+160`. The registers from the suffix start onward are not
parameters, and the named parameters are exactly the prefix before it.

**What kuna should export for those slots.** Nothing, as parameters. They are
the callee's private copy of registers the *caller* passed, and the only thing
that ever reads them is `va_arg`; a `%s` conversion that `formatstring` resolves
gives a type to the *caller's* argument, not to the callee's save slot, which is
a different object in a different frame. So the change is an arity change in
p4_calls, not a typing change - worth 52 `ptr_char` true positives and, far more
than that, 306 phantom parameters over 42 functions in a six-binary sample.

The seam is the trial-gathering loop in `ActionInputPrototype::apply`
(`p4_calls/coreaction_protos.rs`, at `active.register_trial(&addr, size)`): an
input register whose only descendant is a store into the geometry above is not a
parameter, and a `kuna_<slug>.rs` module beside `kuna_inputparamgap` is where
that test belongs. It is a separate PR behind its own option and is not in this
one.

### (b) Imports kuna has no signature for — 158 rows

35 distinct missing imports are called by a function holding one of the 878.
Ranked by how many of those rows sit in a function that calls it (an upper bound
per name — the import is *a* plausible source of the type, not proven to be the
only one), with the number of the 665 corpus binaries that import it, and whether
this PR's table now carries it:

| binja-only rows | fns | import | corpus binaries | shipped here |
|---:|---:|---|---:|---|
| 65 | 33 | `renameat` | 21 | yes |
| 47 | 20 | `setfscreatecon` | 18 | yes |
| 35 | 10 | `utimensat` | 30 | yes |
| 33 | 12 | `faccessat` | 54 | yes |
| 33 | 9 | `mkdirat` | 18 | yes |
| 30 | 6 | `mkfifoat` | 12 | yes |
| 26 | 5 | `fchmodat` | 27 | yes |
| 25 | 4 | `fchownat` | 30 | yes |
| 24 | 20 | `freecon` | 51 | yes |
| 24 | 3 | `readlinkat` | 15 | yes |
| 17 | 13 | `linkat` | 21 | yes |
| 17 | 13 | `symlinkat` | 21 | yes |
| 13 | 6 | `execvp` | 93 | yes |
| 10 | 7 | `getcon` | 24 | yes |
| 8 | 7 | `strtoimax` | 45 | **no — 64-bit return** |
| 6 | 4 | `lchmod` / `mkfifo` | 12 | yes |
| 5 | 3 | `putenv` | 18 | yes |
| 5 | 2 | `security_check_context` | 6 | yes |
| 4 | 4 | `__openat_2` | 6 | yes |
| 4 | 1 | `setexeccon` | 3 | yes |
| 3 | 3 | `lsetfilecon` / `ngettext` | 27 / 9 | yes |
| 3 | 3 | `strtoumax` | 108 | **no — 64-bit return** |
| ≤2 | | `chroot` `dcngettext` `getlogin` `getpass` `ttyname` `chmod` `euidaccess` `execl` `rpmatch` `mkdtemp` `pathconf` | | yes |

33 of the 35 are in the table this PR ships. The two that are not are rejected by
the same width rule the original table documents: `intmax_t` / `uintmax_t` have
no honest `Ty` spelling, so `strtoimax`, `strtoumax`, `strtoll`, `strtoull` and
`llabs` are left out rather than approximated — the same reason `lseek`, `time`
and `qsort` are absent today. `strtoll` (130 corpus binaries) and `strtoumax`
(108) are the two most widely imported names kuna still cannot type at all;
admitting them needs a fixed-width 64-bit member of `Ty`, which is its own
decision and its own PR.

### (c) The rest — 617 rows, and the 51 nobody can reach

The 51 register-only rows are the class's share of the campaign-wide ceiling:
kuna exports no register locals at all, and binja is right on 51 of the 7,091 it
is scored on there. Nothing in this lane touches them.

The other 617 have no single shape. 174 are stack slots kuna still exports as a
width-only `undefinedN` — `slotptr`'s residue, the slots whose stored value is
itself untyped. 252 are spelled as an integer (171 of them arguments), which is
the forwarding chain `protoorder cycles` shortened but did not close: the chain
now stops wherever the bottom of it is an indirect call, a `void *`-returning
allocator, or a function whose own parameter nothing types. 70 are `void *` —
`ptrfromuse` found the pointer and nothing named the pointee, which is exactly
what `charptr` (still off by default) exists to refine. 76 have no kuna variable
at the ground truth's storage at all.

By optimisation level the 617 split 244 / 235 / 138 across O0 / O2-noinline / O2,
so it is not concentrated the way (a) and (b) are.

## Ranked

1. **The libc table** — bucket (b), 158 rows upper bound, shipped in this PR and
   measured in `record.json`. Pure table growth, no new inference.
2. **The variadic register save area** — bucket (a), 52 rows, and 306 phantom
   parameters in a six-binary sample. A p4_calls arity change behind its own
   option.
3. **A fixed-width 64-bit `Ty`** — `strtoll` and `strtoumax` alone are 238
   corpus binaries' worth of untypable calls, 11 rows of this gap.
4. **Bucket (c)** has no lever of its own: its three halves belong to `slotptr`,
   `protoorder` and `charptr` respectively.

## Scripts

`imports.py` (per-slice dynamic imports), `corpus_imports.py` (the 665-binary
histogram), `callgraph.py` (per-function PLT call sets from the DWARF twins),
`variadic.py` (DWARF `DW_TAG_unspecified_parameters`), `reduce.py` (the
`gcc -aux-info` reducer), `cands.py` / `gentable.py` (selection and code
generation), `attrib.py` (the three-way split above). `gapcensus.py` is round E's
census, re-run unchanged on the round-F rows.
