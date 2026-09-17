# `libctypes` — the default flip (`off` → `opaque`)

The option shipped default-off in the PR that introduced it, with the flip held
back as its own change. This is that change: one default, plus the two
expectations in the tree that pinned the old spelling.

Everything measured here is `KUNA_LIBCTYPES=off` (= `main`'s default, verified
byte-identical below) against the new default.

## What moved in the corpora

**`make test` — nothing, structurally.** `grep -l 'load file' tests/datatests/*.xml`
is **0 of 83**: no datatest opens a file, so the whole analysis tier is invisible
to that corpus. `datatests: 675/675 assertions passed / PARITY OK`, as it must be.
It is not evidence about this option in either direction, which is why the sweep
below is.

**`make test-stages` — no assertion moved, and the baseline did not have to be
re-recorded.** `datatests: 1038/1038 assertions passed / PARITY OK`, and a
`--save-baseline` re-record of `docs/baseline-stages.json` on the flipped build is
**byte-identical** to the file already in the tree (100,769 bytes each). The list
of moved assertions is therefore empty.

That is a claim about a gate, so it was checked two further ways rather than
taken from a green line:

* *The default really reaches the stage harness.* A throwaway fixture that loads
  a real ELF and asserts `int4 fclose(FILE \*a0)` with **no `option` command at
  all** passes on this build and fails under `KUNA_LIBCTYPES=off`. The harness is
  seeing the new default, not a stale one.
* *Only one of the 16 stage fixtures that `load file` changes output at all.*
  Each of the 16 scripts was replayed through `decomp_dbg` in both arms (its own
  `option libctypes` commands removed so the env decides): **15 produce
  byte-identical output**, and the one that differs is `kuna-libctypes.xml`
  itself, exactly as designed —

```
-int4 fclose(void *a0)                   -void * fopen(char *a0,char *a1)
+int4 fclose(FILE *a0)                   +FILE * fopen(char *a0,char *a1)
```

  plus, in its third pass (the real coreutils `fmt`), one shadow local folding
  away — `uint4 c` and `uint4 v3` were a copy pair (`c = v3;`, `v3 = c;`,
  `c = (uint4)*v2;` beside `v3 = (uint4)*v2;`) and merge into `v3` once `f` is a
  stream. One fewer declaration; no statement lost.

The fixture's own script now restores `opaque` in its last command instead of
`off`. The gate is a process environment variable and the corpus runs in one
process, so the old restore would have silently disabled the table for every
test after it.

**`make test-cli` — one expectation rewritten.**
`tests/cli/changing-namestyle-invalidates-discovered.json` pinned
`sub_1249\(void \*param_1\)`. That parameter is passed to `fread` and `fclose`
and nothing else, so it is now `FILE *`; the pattern becomes
`sub_1249\(FILE \*param_1\)`. The probe exists to prove that a name
`kuna functions` reports still selects its function when `--option namestyle
ghidra` changes how names are PRINTED, and `param_1` (rather than `a0`) is what
pins that the style applied — that is untouched, and the assertion is exactly as
tight as it was: same anchor, same parameter, one spelling.

## Type recovery — the sweep the default rests on

`scripts.decbench.typesweep`, decbench main's `TypeMatchMetric`,
`DECBENCH_NO_CACHE=1`, `KUNA_BIN` pinned to this branch's release build. Base arm
is `KUNA_LIBCTYPES=off`, test arm `--option libctypes opaque`, so "improved"
reads in the direction of the flip.

8 projects × `O0` / `O2` / `O2-noinline` = **444 slices, 10,748 scored functions**.

| | base (off) | test (default) |
|---|---:|---:|
| `type_match` PERFECT | 889 | **959** (+70) |
| aggregate `type_match` | 2913.85 | **3037.05** (+123.20) |
| moved ONTO perfect | | 70 |
| moved OFF perfect | | **0** |
| improved (not perfect) | | 650 |
| **worsened** | | **1** |

| project | n | perfect off → on | mean off → on | improved | worse |
|---|---:|---|---|---:|---:|
| bzip2 | 267 | 15 → 19 | .3454 → .3664 | 35 | 0 |
| coreutils | 6422 | 560 → 592 | .2629 → .2726 | 410 | 0 |
| diffutils | 420 | 43 → 51 | .3633 → .3907 | 50 | 0 |
| findutils | 790 | 31 → 33 | .1465 → .1588 | 49 | 0 |
| grep | 247 | 37 → 37 | .3658 → .3689 | 7 | 0 |
| gzip | 368 | 83 → 88 | .4654 → .4844 | 30 | 0 |
| shadow | 686 | 22 → 29 | .2768 → .2879 | 53 | 0 |
| tar | 1548 | 98 → 110 | .2670 → .2791 | 86 | 1 |
| `O0` | 4286 | 637 → 686 | .4252 → .4395 | 317 | 0 |
| `O2` | 2394 | 49 → 52 | .1378 → .1461 | 163 | 0 |
| `O2-noinline` | 4068 | 203 → 221 | .1872 → .1975 | 240 | 1 |

Harness controls: the base arm agrees with the tree's published `type_match` on
10,075 of 10,748 functions (93.74% — the tree was scored before this campaign's
other merges); 9,907 functions hand the metric byte-identical `variables[]` in
both arms and **0** of them score differently; 0 slice errors.

### The one worsened function, in full

`tar::O2-noinline::tar` `write_directory_file_entry`, .167 → .0. Ground truth is
`int write_directory_file_entry (void *entry, void *data)` — the
`hash_do_for_each` callback signature — and kuna's `data` went from `void *` (a
match) to `FILE *` (a miss).

It is not the defect shape the sweep is watching for: the ground truth is a
POINTER, not a scalar that we have started calling a pointer. And kuna's answer
is what the body does:

```
$ kuna decompile-all <O2-noinline/tar/stripped/tar> --addr 0x1f1c0
unsigned long sub_1f1c0(int8 a0,FILE *a1)
{
  ...
  fwrite_unlocked(v4,2,1,a1);
  fwrite_unlocked(v4,strlen(v4) + 1,1,a1);      /* 8 of these */
```

`data` is tar's `FILE *fp`, cast out of the `void *` in the first line of the
real function. The declaration is what the metric holds; the stream is what the
code has. One function in 10,748.

## Whole-binary output — 8 binaries, 4,187 functions

`kuna decompile-all` off vs default over `fmt`/`ls`/`grep`/`gzip`/`tar` `-O2`,
`find`/`diff` `-O0` and shadow `useradd` `-O2 -fno-inline`. Every changed line is
bucketed and, independently, every function's **skeleton** — its control-flow
keyword sequence, its call-target multiset and its numeric-literal multiset,
identifiers and types normalised away — is compared.

* **0 functions lost, 0 gained.**
* **The call-target multiset is identical in all 4,187 functions.** Nothing calls
  anything different.
* **131 functions differ in the skeleton at all.** 129 of them differ *only* in
  literals, and in every case the literals are offsets absorbed into a field name
  (`*(long *)&a0[4]` → `a0->field_0x10`) or an index respelled against a
  different base (`v12[v9 * 0x26]` → `(long)v12 + v9 * 0x130`, same address,
  scale folded into the constant). The other 2 are the `__uflow` PLT thunk in
  `fmt` and `find`, which gains a `return` because the new table entry gives it a
  return type:

```
-void __uflow(void)                    +int __uflow(FILE *a0)
-{ (*dat_be40)(); }                    +{ int v1; v1 = (*dat_be40)(); return v1; }
```

| binary | functions | changed lines | skeleton differs | residual bucket |
|---|---:|---:|---:|---:|
| O2 coreutils fmt | 151 | 91 | 9 | 5 |
| O2 coreutils ls | 404 | 828 | 19 | 43 |
| O2 grep grep | 449 | 511 | 6 | 30 |
| O2 gzip gzip | 204 | 135 | 8 | 11 |
| O2 tar tar | 1125 | 2081 | 45 | 73 |
| O0 findutils find | 862 | 1377 | 19 | 27 |
| O0 diffutils diff | 507 | 843 | 7 | 70 |
| O2-noinline shadow useradd | 485 | 247 | 18 | 19 |

6,113 changed lines, of which 5,835 land in a named bucket: `var-renumber` 2,817,
`decl-block` 684, removed/added lines 919, `field-form` 463, `proto-line` 453,
`aggregate-slot` 180, `pointee-named` 194, `cast-width` 48, `const-offset-access`
1. The 278 residual lines are dominated by the line-pairing of a function whose
locals renumbered wholesale; the skeleton comparison above, not the bucket count,
is what carries the "nothing does anything different" claim.

### What the flip buys and what it costs, in the body

Declarations go **down** on 7 of the 8 binaries — 16,509 → 16,370 net — because
scattered stack slots collapse into the aggregate they really are:

```
  ls  0x76b0   17 `unsigned long vN; // stack - 0x..` slots at -0xd8..-0x58 become
               one `void *v3 [17]`, and the calls become
               sigaction(v6,NULL,(sigaction *)v3) / sigemptyset((sigset_t *)0x26220)

  useradd 0x16640  `char v2 [16]` + a loose `char *v3` become `char v2 [152]`, and
               the bare read that printed "(nlink: %u)" becomes v2._16_8_ --
               offset 16 of the struct stat the call before it filled
```

The cost is the piece accessor `vN._off_size_`, which is what an aggregate with
no field at the touched offset renders as: **1,014 → 1,265** over the eight
binaries (+24.7%), concentrated in `tar`/`grep`/`find`/`diff`; `ls` goes down by
one. The neighbouring functional forms barely move (`SUB` 216 → 217, `CONCAT`
276 → 278), and `PTRSUB(` — the form the sizing rule exists to prevent — is
**0 in both arms on every binary**. The follow-up `glibc` value, which installs
the public `_IO_FILE`/`stat` field names, retires the stdio half of the piece
accessors.

### The by-value question, re-checked on the flipped default

`Ty::NamedPtr` is pointer-only, but propagation still reaches by-value positions.
Over the same 8 binaries: named aggregates returned **by value** 0 → 3 (`ls`
0x10210, `gzip` 0x10bb0, `tar` 0x43230 — the same gnulib `gettime` wrapper each
time, all `timespec`, all correct: a 16-byte `timespec` comes back in a register
pair). By-value named **parameters** 0 → 0. `rethidden` 0 → 0. This is the case
the sized shells exist for, and it is still clean.

## Speed

Interleaved `off` / default pairs, alternating which arm runs first, **minimum of
15 pairs** (the honest read on a shared box; the medians are contention):

| case | min off | min default | min Δ | median Δ |
|---|---:|---:|---:|---:|
| `fmt` `decompile-all` (152 fn) | 4190.0 ms | 4158.6 ms | **−0.75%** | −6.59% |
| `ls` `decompile-all` (404 fn) | 13783.9 ms | 13957.7 ms | **+1.26%** | +0.26% |
| `fmt` `get_prefix` (one fn) | 178.4 ms | 181.4 ms | **+1.64%** | −4.38% |
| `fmt functions` (load only) | 171.2 ms | 171.8 ms | **+0.37%** | −0.45% |

Worst +1.64% against a 5% budget. The load-only row is the table's own cost
(~20 `get_type_struct`/`find_by_name` calls, +0.37%); the whole-binary rows carry
the rest, which is a named 216-byte pointee participating in type propagation
where a `void *` did not.

## `off` is still `main`

`kuna decompile-all` over all eight whole binaries, this branch's build with
`KUNA_LIBCTYPES=off` against a release binary built from `origin/main`
(f02365cc): **byte-identical on every one** — fmt 4,069 lines, ls 13,677, grep
18,257, gzip 9,358, tar 56,725, find 20,839, diff 13,102, useradd 11,487.

So every "off" column above is `main`'s own output, not a second implementation
of it, and `--option libctypes off` is a real ablation of the flip.
