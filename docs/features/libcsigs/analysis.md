# `libcsigs` — the measured libc signature extension

## 1. The opportunity

`kuna-analysis/src/analyzers/protos/mod.rs` carries a **27-entry** libc prototype
table. Its own header says why it is small and calls the shortfall out as a
documented LOSS: Ghidra applies parsed C headers shipped as binary `.gdt` data-type
archives, that format is not vendored into the kuna tree, and the table is "a
faithful, minimal stand-in". Nothing about the constraint is licensing — the
signatures are standard C library declarations, and `docs/missing-ghidra-analyses.md`
classifies the whole item as a dependency/data substitution of the same kind as
BFD → `object`. **The table is small because nobody has grown it, not because it
must be.**

The cost is that every libc callee outside those 27 leaves its callers' arguments
as an inferred `unsigned long`:

```
kuna: unsigned long authenticate(unsigned long a0, char *a1)   /* a0 is a path */
      v4 = open(a0, 0);                                        /* v4 is unsigned int */
```

This is kuna's weakest metric on plain C — third on `type_match` behind binja and
angr — and 50% of the 1,587 cases where IDA scores a perfect `type_match` and kuna
does not have an integer-typed kuna parameter where IDA has a concrete type.

## 2. Which names — measured, not guessed

Two independent rankings over the **frozen** decbench C corpus
(`~/github/decbench/results/full_run`, read-only):

1. **Breadth.** `objdump -d` over the 547 C binaries that carry a scored case; a
   histogram of every `call <name@plt>`. 1,930 distinct imported names.
2. **Metric alignment.** The 1,587 `type_match` cases where IDA is perfect and kuna
   is not, each mapped into its containing function block, counting which imports
   that function calls. This answers "which callee, if typed, would move the
   metric", rather than "which callee is popular".

The head of (2), with (1)'s call-site counts alongside — everything already in the
27-entry table removed:

| cases | corpus sites | binaries | name |
|---:|---:|---:|---|
| 119 | 5,456 | 469 | `exit` |
| 91 | 8,311 | 121 | `gettext` |
| 87 | 24,084 | 456 | `__errno_location` |
| 63 | 2,093 | 165 | `strcasecmp` |
| 55 | 5,112 | 327 | `close` |
| 50 | 29,055 | 466 | `__stack_chk_fail` |
| 42 | 5,587 | 217 | `error` |
| 38 | 310 | 86 | `strncasecmp` |
| 35 | 1,384 | 276 | `open` |
| 35 | 4,553 | 211 | `strdup` |
| 31 | 6,276 | 220 | `strerror` |
| 30 | 14,664 | 201 | `dcgettext` |
| 23 | 1,337 | 238 | `fcntl` |
| 20 | 641 | 53 | `syslog` |
| — | 12,824 | 292 | `__fprintf_chk` |
| — | 3,806 | 260 | `__printf_chk` |

The selection rule, applied mechanically: **>= 100 corpus call sites or >= 3 of the
1,587 cases**, plus a named-witness set for the low-frequency names that motivated
the work (`canonicalize_file_name`, `dirname`, `asprintf`, the `err`/`warn`
family). 206 entries result.

Two measured negatives worth recording. The `err`/`warn`/`errx` family the brief
suggested does **not** appear in this corpus at all (`warnx` 12 sites, everything
else 0) — `error(3)` is what GNU userland actually calls; they are in the table
because they are free and correct, not because they were measured. And no
`__isoc99_*` symbol appears either except `__isoc99_sscanf` (273 sites) and
`__isoc99_fscanf` (45).

## 3. Which signatures — machine-derived, never from memory

A wrong prototype is worse than a missing one: it asserts a false type where
`unsigned long` was merely uninformative. So no signature here was written by hand.

* **The bulk (193 entries)** come from `gcc -aux-info` over a translation unit that
  includes the platform headers with `_GNU_SOURCE` and `_FORTIFY_SOURCE=2` — 1,520
  prototypes straight out of `/usr/include`. Only the `NC` lines (a pure header
  prototype) are used; the `NF` lines are the fortified headers' inline definitions,
  which carry parameter names that would have to be stripped heuristically.
  Each declaration is reduced to the module's vocabulary by a fixed rule.
* **`__isoc99_{scanf,sscanf,fscanf}`** are not modelled by analogy.
  `/usr/include/stdio.h` lines 434-441 declare them via `__REDIRECT` *from*
  `fscanf`/`scanf`/`sscanf`, which is the header stating that the symbols are
  ABI-identical to the names they replace.
* **The FORTIFY `*_chk` family** has no plain header prototype for the `mem`/`str`
  members — they exist only as GCC builtins. Their signatures are checked against
  GCC's own builtin table with `_Static_assert(__builtin_types_compatible_p(...))`;
  all nine assertions compile.
* **`__stack_chk_fail`** has no declaration anywhere. It is `void (void)` per glibc
  `debug/stack_chk_fail.c`.

### Rejected rather than approximated

`Ty` deliberately has no spelling for a type whose width is not stable across
ILP32 and LP64. A declaration containing one is dropped, not modelled:

| name | corpus sites | rejected because |
|---|---:|---|
| `lseek` | 746 | returns `__off_t` |
| `signal` | 574 | returns `__sighandler_t` (a function pointer) |
| `time` | 399 | returns `time_t` |
| `qsort` | 323 | takes `__compar_fn_t` |
| `strtoll` | 247 | returns `long long int` — 8 bytes even on ILP32 |
| `fseeko` | 228 | takes `__off_t` |
| `mmap` | 167 | takes `__off_t` |
| `poll` | 162 | takes `nfds_t` |
| `iconv` | 144 | takes `iconv_t` |
| `connect` | 126 | takes `__CONST_SOCKADDR_ARG` (a transparent union) |
| `pthread_once` | 70 | takes `void (*)(void)` |

`strtoll` is the one that matters: an earlier pass of the reducer classified
`long long int` as a 4-byte integer and would have shipped `int strtoll(...)`. It
was caught by the width rule, not by review.

`sockaddr`/`stat`/`DIR`/`FILE` pointer parameters are kept — only the *pointee*
layout is unknown, and the slot itself is pointer-width everywhere.

## 4. The wrongness axis: imports only

A table entry matches by name. The 27-entry base table matches a name whether the
image *imports* it or *defines* it, and for the base table's spellings that is
harmless. It is not harmless at 206 entries.

Measured directly: `readelf` over all 677 binaries in the frozen
`O0`/`O2`/`O2-noinline` trees, counting definitions (not imports) of a new-table
name. The surface is small — and almost all of it is NuttX, an RTOS that implements
the genuine POSIX API, where the prototype would be right anyway. But one is a real
collision:

```
zlib/minigzip:  void error(const char *msg);     /* shndx 16, never UND */
glibc:          void error(int, int, const char *, ...);
```

Typing that call `error(0, 0, …)` would be strictly worse than the `unsigned long`
it replaces. `LibcSigsPass` therefore applies an entry **only to a name the image
imports and does not itself define** — undefined FUNC symbols plus the format
import resolver's names (PE IAT/INT, Mach-O `__stubs`), minus every defined name.
A PLT/IAT import named `error` is definitively the platform's `error`; a defined
one is the program's own function that happens to share a spelling. This also
matches Ghidra's model, which applies archives to *external* functions.

The base table's matching is left exactly as it was.

## 5. The FORTIFY entry points are not aliases

`__printf_chk` is **not** `printf` under another name — it takes a leading
`int flag`, so the format string is at slot 1, not slot 0. `__fprintf_chk` takes
`FILE *` then the flag; `__snprintf_chk` takes `char *, size_t, int, size_t`. With
12,824 `__fprintf_chk` and 3,806 `__printf_chk` call sites in the corpus, treating
either as its plain namesake would shift every argument of the single most frequent
call in the C corpus. `fortify_printf_family_carries_the_extra_flag_parameter`
pins all four variadic slots.

Note that `formatstring/apply.rs`'s `VARIADIC_SUBSTRINGS` matches by `contains`, so
`__printf_chk` *does* match `"printf"` there and is treated as a format-string
callee whose format is at slot 0. That is a pre-existing, separate defect; it is
**not** touched by this PR, and is recorded as a follow-up in `plan.md`.

## 6. What the option decision rests on

This is broad — it changes emitted C in every dynamically linked C binary — and
choosing *which* signatures to assert is a judgment call, not a mechanical bug fix.
It ships behind `--option libcsigs on|off`, default ON, so the contribution is
ablatable in one flag; `off` renders exactly what the 27-entry table alone renders.
The gate lives at the analysis commit boundary
(`engine.rs::analysis_pass_enabled`), the same seam `cortexmvectors` /
`fdeinterior` / `cppsig` use, because the producing pass runs at `load file`,
upstream of any `option` command.
