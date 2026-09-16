# `libctypes` — named libc/POSIX aggregate pointers in the prototype tables

## The opportunity

kuna's two built-in prototype tables share one type vocabulary (`Ty`,
`kuna-analysis/src/analyzers/protos/mod.rs`), and that vocabulary is
**width-stable by construction**: every slot is `void`, exactly 4 bytes, or
exactly pointer-width on every ILP32/LP64 target. There is therefore no spelling
for an aggregate, and all 108 aggregate-pointer slots across the two tables are
`void *`.

That is honest about the width and silent about the pointee — and the pointee is
the one thing the table actually knows. `int fclose(FILE *)` is a declaration,
not an inference.

Measured consequence on the scored surface: over a 92-binary DWARF sample,
named-struct pointers are 22.5% of matchable ground-truth variables, and kuna
scores **0.0%** on that class while ida scores 432 / ghidra 382 / binja 523 true
positives on the same sample. The libc/POSIX-namable slice of it is ~4.1% of
matchable ground truth (`FILE` 605 occurrences, `stat` 255, `passwd` 136, …).
`void *` never matches `FILE *`: the metric's normalizer has no pointer entry in
its width-only generosity table.

## What ships

An option `libctypes` (`off|opaque`, default **off**), a P1 analysis-tier
enablement, owning one new module
`kuna-analysis/src/analyzers/protos/kuna_libctypes.rs` and one new `Ty` variant.

* **One new variant, pointer-only.** `Ty::NamedPtr(&'static str)`. No slot in
  these tables is taken or returned by value, because no libc declaration
  restated here does that — but that is a property of the TABLE, not of the
  emitted C, and it is the WIDTH that makes the by-value case safe. Both hazards
  `kuna_dwarfstructs.rs` documents are hazards of a SIZELESS aggregate: a
  by-value parameter of unknown width degrades to a raw integer, and a sizeless
  aggregate RETURN is classified as a hidden-return-buffer call that grows a
  phantom first parameter. Type propagation does reach by-value positions on its
  own — `timespec sub_10210(void) { timespec v1; clock_gettime(0,&v1); return
  v1; }` on `-O2` `ls`, correctly, since a 16-byte `timespec` is returned in a
  register pair. Measured over the 6-binary sweep: 3 by-value named returns
  (`ls`/`gzip`/`tar`, all that same gnulib `gettime` wrapper), 0 by-value named
  parameters, 0 `rethidden` in either arm, and nothing wider than a register
  pair in any return slot. See §"By-value named aggregates" below.
* **17 aggregate names**, the bare DWARF spelling: `FILE`, `DIR`, `dirent`,
  `stat`, `passwd`, `group`, `tm`, `option`, `timespec`, `timeval`, `sigaction`,
  `sigset_t`, `mbstate_t`, `termios`, `sockaddr`, `pthread_mutex_t`.
* **75 named slots over 74 signatures**: 3 restatements of base-`LIBC` entries
  (matched, like that table, against any unambiguous name the image carries) and
  62 restatements of `LIBC_EXT` entries plus 9 new names (imports only, like that
  table).

## The measurements this design rests on

### 1. Widths — `sizeof` / `_Alignof` on this box, not from memory

```
FILE 216/8   stat 144/8   passwd 48/8   group 32/8   tm 56/8   option 32/8
dirent 280/8 timespec 16/8 timeval 16/8 sigaction 152/8 sigset_t 128/8
mbstate_t 8/4 termios 60/4 sockaddr 16/2 pthread_mutex_t 40/8
```
(`.scratch/sizes.c`, gcc on glibc x86-64.) `DIR` gets width **1**: the platform
publishes no layout for it at all, and what the pointer-arithmetic seam needs is
merely a non-zero width.

### 2. Why the shells must be sized

A zero-width pointee is not opaque, it is broken. `AddTreeState` has no size-0
early out, and `RulePtrsubUndo`'s no-field arm short-circuits on `typesize != 0`
(`substrate/dtype.rs`), so a `PTRSUB` into a zero-width pointee is declared
MATCHING and survives to the printer, which renders it in FUNCTIONAL form —
literal `PTRSUB(p,0x28)` inside the C. The sites that would take that path are
exactly the ones this table exists to type: `fmt`'s `(long)stdout + 0x28` and
`get_prefix`'s six `*(unsigned char **)(a0 + 8)` accesses.

Verified with the shipped build, whole binary, `decompile-all`:

| binary | `PTRSUB(` off | `PTRSUB(` on |
|---|---:|---:|
| coreutils fmt O2 | 0 | 0 |
| coreutils ls O2 | 0 | 0 |
| grep O2 | 0 | 0 |
| gzip O2 | 0 | 0 |
| tar O2 | 0 | 0 |
| findutils find O0 | 0 | 0 |
| diffutils diff O0 | 0 | 0 |

and the accesses render as the field form instead:

```
-      v4 = *(unsigned char **)((long)stdout + 0x28);
+      v4 = *(unsigned char **)&stdout->field_0x28;
```

### 3. What happens on a binary that has real debug info

An image built `-g` already carries the platform's own `struct stat`, and DWARF
interns it under the identical bare spelling. Two things had to be settled: who
wins, and what happens to the pointers.

**Who wins: DWARF, and it is not close.** The pass is registered AFTER
`DwarfPass` (`kuna-analysis/src/passes.rs`) and `named_aggregate` adopts a held
aggregate of the declared width — under the name, or under the spelling the
platform's headers use for it (`FILE` -> `struct _IO_FILE`; without that alias
the image carries two stream types and the body casts between them). A name held
by anything else — a different width, a zero-width forward declaration, a
non-struct — declines the signature. This table never completes, re-keys or
alters a type it did not establish.

**What the adopted spelling costs on the metric (nothing today, something
later).** decbench's `normalize_type` accepts `FILE *` for a ground-truth
`FILE *` and does NOT accept `_IO_FILE *` — `_POINTEE_MAP` in
`decbench/metrics/type_match.py` has no alias row for it, so the intersection is
empty. Every number in `record.json` is measured on STRIPPED images, where no
DWARF type is held and the table mints and prints the bare `FILE`, so none of
them is affected. A future measurement over `-g` images would score each adopted
stream slot a miss; the answer there is an alias row in the metric, not a second
spelling of a type the image itself defines.

**Why the order is the correctness condition, not a preference.** A pointee is
captured as an `Rc<Datatype>` when the signature is built, and completing a
struct RE-KEYS it into a new `Rc` (`TypeFactory::set_fields_struct`; the C++
mutates a `TypeStruct` in place, the Rust clones). A shell minted first and
completed by DWARF afterwards is therefore completed for everyone EXCEPT the
pointers this table already built. Measured on the first cut of this PR, which
minted first:

```c
/* int f(const char *p, struct stat *st) built with gcc -g -O1 */
off:    v1 = (stat(p,st)) ? -1 : st->st_mode + (int)st->st_size;
minted: v1 = (stat(p,st)) ? -1 : *(int *)&st->field_0x18 + *(int *)&st->field_0x30;
```

`type_incomplete` is still passed back through `set_fields_struct_raw` for a
shell this table mints itself, for one remaining reason: the `.h` emitter prints
`typedef struct FILE FILE; /* opaque */` with no body instead of a struct with a
width and no members.

**Measured, `decompile-all` over the UNSTRIPPED twins** (`off` vs `opaque`;
"named field" = a `->name` access that is not `->field_0xNN`):

| twin | named field off -> on | `field_0x` off -> on | `(FILE *)` casts off -> on |
|---|---|---|---|
| findutils O0 find | 2722 -> 2748 | 4 -> 23 | 0 -> 0 |
| coreutils O2 du   | 1007 -> 1048 | 0 -> 7  | 0 -> 0 |
| coreutils O2 ls   | 625 -> 718   | 0 -> 8  | 0 -> 0 |
| coreutils O0 ls   | 819 -> 826   | 3 -> 3  | 0 -> 0 |
| coreutils O2 sort | 876 -> 888   | 0 -> 0  | 0 -> 0 |
| diffutils O0 diff | 983 -> 983   | 1 -> 6  | 0 -> 0 |

Named-field accesses go UP or stay flat on every twin: the adopted definitions
are the platform's, so a `stat *` parameter keeps `st_mode`. The `field_0x`
count also rises, and every one of those is an access that was RAW POINTER
ARITHMETIC in the `off` arm — `*(char *)((long)v4 + 0x14)` becoming
`v4->field_0x14` on a `dirent *` that DWARF never interned here, because no
DWARF-described variable in that binary has the type. Checked mechanically: of
the 39 `field_0x` accesses the option introduces across the six twins, **0** are
on a base that had a NAMED field in the `off` arm.

### 4. Gate shape — the env bridge, not a flag read in the pass

Load-time analysis passes run inside `bootstrap_from_object`, which
`decompile-all` calls BEFORE `apply_runtime_options`, so an `arch.analysis_*`
read inside a pass's `run()` sees the constructor default silently (the recorded
PR #248 failure mode). A `libcsigs`-style commit-boundary gate is also wrong
here for two separate reasons:

* the pass interns its named shells into the type factory **while building the
  signatures**, so with the gate off it must not run at all, and a commit gate
  runs it anyway;
* the commit gate reads the CURRENT `Architecture`, and a two-pass stage test
  does `option … / clear architecture / load file`, which rebuilds the
  Architecture with the constructor default — measured: with the commit arm in
  place, pass 2 of the stage test produced `fclose(void *)`.

So the gate is the load-time env bridge `KUNA_LIBCTYPES`, the same one
`dwarfstructs` and `typedepth` use: set by the console's `option libctypes` arm
before `load file`, exported by `kuna decompile` onto the subprocess and by
`decompile-all` before the bootstrap, and read once at the top of the pass. The
same `libctypes_enabled()` call gates `declared_libc_prototype`, so the operator
path agrees:

```
kuna decompile fmt --addr 0x2f30 --define-function 0x2f30=fclose --option libctypes off
  -> int fclose(void *a0)
                                                            ... --option libctypes opaque
  -> int fclose(FILE *a0)
```

### 5. Provenance of the new names, and what was rejected

Every name new to this table was reduced from `gcc -aux-info` over the platform
headers with `_GNU_SOURCE` + `_FORTIFY_SOURCE=2`, and each one's corpus weight
was measured as PLT call sites over the 524 stripped O0+O2 decbench ELFs:

| name | corpus call sites | verdict |
|---|---:|---|
| `rewind` | 117 | in |
| `__uflow` | 77 | in (the fmt witness's sole type evidence) |
| `fgetc` | 57 | in |
| `freopen` | 32 | in |
| `funlockfile` | 20 | in |
| `getdelim` | 18 | in |
| `flockfile` | 18 | in |
| `popen` / `pclose` | 17 / 17 | in |
| `fgetc_unlocked` | 0 | **rejected** |
| `fmemopen` | 0 | **rejected** |
| `__underflow` | 0 | **rejected** — and no machine-readable declaration exists (glibc declares it only in its uninstalled `libioP.h`) |

`getdelim`'s stream is its **fourth** slot (`char **, size_t *, int delim,
FILE *`), where `getline`'s is its third — pinned by a unit test, because the
off-by-one would be invisible in output.

Not taken in this PR, and measured for the next one: `obstack`
(`_obstack_newchunk` 42 call sites, `obstack_free` 24) and `regex_t` (`regcomp`
29, `regexec` 32, `regfree` 30), which together account for a further ~170
matchable ground-truth pointees in the 92-binary sample.

### 6. The `va_list` trap

`vasprintf` p2, `vsnprintf` p3, `__vasprintf_chk` p3, `__vfprintf_chk` p3,
`__vsnprintf_chk` p5, `verr` p2 and `vwarn` p1 are `va_list`, not `FILE *`. A
blanket `void * -> FILE *` retarget would assert a false type at every one of
those call sites. The retarget is enumerated slot by slot; a unit test asserts
the whole `v*` family is absent from the table and that `__vfprintf_chk` names
only its p0.

### 7. By-value named aggregates — what is and is not guaranteed

The table only ever names a POINTER slot, but that does not make a by-value
named aggregate impossible in the output, and the earlier phrasing of this
document claimed it did. Type propagation reaches by-value positions on its own:

```
$ kuna decompile-all <O2/coreutils/stripped/ls> --addr 0x10210 --option libctypes off
undefined16 sub_10210(void)  { char v1 [16]; clock_gettime(0,v1); return v1._0_16_; }
$ kuna decompile-all <O2/coreutils/stripped/ls> --addr 0x10210 --option libctypes opaque
timespec sub_10210(void)     { timespec v1;  clock_gettime(0,&v1); return v1; }
```

The `on` rendering is right: gnulib's `gettime` really does return a
`struct timespec`, 16 bytes, in `RAX:RDX`. It is right because the shell is
SIZED — both hazards in §2's neighbour (`kuna_dwarfstructs.rs`) are hazards of a
SIZELESS aggregate, and the second of them is exactly this slot: an aggregate
RETURN whose width the ABI classifier cannot see is classified as a
hidden-return-buffer call, which grows a phantom `rethidden` first parameter and
shifts every real one. A width-0 shell here would have produced that.

Measured over the 6 sweep binaries (`ls`/`grep`/`gzip`/`tar` O2,
`find`/`diff` O0), grepping every `.on.c` for a named aggregate in a declarator
position:

| shape | off | on |
|---|---:|---:|
| named aggregate returned BY VALUE | 0 | 3 (`ls` 0x10210, `gzip` 0x10bb0, `tar` 0x43230 — all the same `gettime` wrapper, all `timespec`) | 
| named aggregate as a by-value PARAMETER | 0 | 0 |
| `rethidden` anywhere | 0 | 0 |

Nothing wider than a register pair reaches a return slot anywhere in the corpus
— no `stat` (144), `sigaction` (152) or `FILE` (216) does — but that is a
corpus observation, not an invariant the table enforces. The invariant is the
width: whichever way the classifier is asked, it has the real number.

## Whole-corpus behavior (`decompile-all`, 6 binaries + fmt, 3,702 functions)

Per-line classification and a control-flow/call/constant skeleton comparison are
in `corpus-hunk-classification.txt`. Summary:

* **0 functions lost or gained**; `PTRSUB(` count unchanged (0) everywhere.
* The dominant line categories are `pointee-named` (the intended change),
  `field-form` (`*(T **)(p + 8)` -> `*(T **)&p->field_0x8`) and `var-renumber`
  (a renumbered local — the scored JSON surface excludes register locals, so this
  is cosmetic).
* 5,517 changed lines, of which 5,155 land in a named bucket (`pointee-named`
  589, `var-renumber` 3,077, `decl-block` 617, `field-form` 222,
  `aggregate-slot` 164, `const-offset-access` 126, `proto-line` 109,
  `cast-width` 46, added/removed 105). The residual 362 are dumped in full in
  the attached file with a shape histogram: 159 are a constant offset spelled as
  a field of a named pointee, 129 the same store or load through a different
  stack-slot decomposition, 50 one aggregate slot read as a sub-piece, 23 a
  named declaration or cast, 1 block skew.
* The one line shape that goes measurably UP is the piece accessor
  `vN._off_size_`: 1,002 -> 1,231 over the seven binaries (+22.9%; tar
  615 -> 699, grep 79 -> 116, find 37 -> 83, diff 14 -> 58, gzip 19 -> 38, fmt
  flat, ls 149 -> 148). It is the cost of naming an aggregate that has no field
  at the offset being touched — always true of a fieldless shell, and true of the
  program's own on-stack aggregates once a `stat *`/`sigaction *` argument pulls
  their scattered slots into one object — and the follow-up `glibc` value (real
  `_IO_FILE` field names) retires the stdio half of it. The neighbouring functional forms barely move: `SUBxy(` 214 -> 215,
  `CONCATxy(` 266 -> 268 (find +2, tar -1/+2), and `PTRSUB(` stays 0 in both
  arms, which is the form the sizing rule exists to prevent.
* 140 of 3,702 functions (3.8%) differ in the skeleton, every inspected one
  because an offset constant became a field name or an `&` appeared where a cast
  was. Two representative cases, both improvements:

```
  diffutils 0x10e15:  20 scattered `unsigned long vN; // stack - 0x..` slots
                      collapse into one `void *v2[17]` covering the on-stack
                      `struct sigaction`; the calls become
                      sigaction(sig, NULL, (sigaction *)v2)

  gzip      0x65d0:   `void *v1` (the localtime result) becomes `tm *v1`, and
                      *(unsigned int *)((long)v1 + 0xc) becomes
                      *(unsigned int *)&v1->field_0xc
```

## Witnesses (the scored surface)

`O2/coreutils/stripped/fmt`, `decompile-all --addr 0x2f30 --addr 0x3420`:

```
off:  void sub_2f30(long a0)                  on:  void sub_2f30(FILE *a0)
off:  void sub_3420(long a0,unsigned int a1)  on:  void sub_3420(FILE *a0,unsigned int a1)
```

and the JSON the benchmark actually scores:

```json
{"name": "param_1", "type": "FILE *", "kind": "arg", "arg_index": 0, "size": 8}
```

Ground truth for both is `int get_prefix(FILE *f)` / `get_line(FILE *f, int c)`.

## `off` is inert, checked against the tree it branched from

`kuna decompile-all` over the whole `O2/coreutils/stripped/fmt`, this branch with
`--option libctypes off` vs the main tree's own build: **0 lines differ** (4,069
lines each). With the gate off the pass returns before it interns anything, so
there is no named shell in the type factory for anything downstream to find.

## The project export: the `.h` stays valid C, the `.c` does not

`kuna decompile-project <bin> --option libctypes opaque` on
`tests/fixtures/stripped_dynamic_x86_64`:

```c
/* <name>.h */
typedef struct FILE FILE; /* opaque */
...
unsigned long fread(void *a0,unsigned long a1,unsigned long a2,FILE *a3);
int fclose(FILE *a0);
FILE * fopen(char *a0,char *a1);
```

The bare name is what round-trips: the printer spells a named base bare, and the
`.h` emitter's own `typedef struct X X;` line (which skips the body for an
incomplete struct) is what makes it a declared type rather than a tag-only one.

One thing that typedef cannot coexist with is a FUNCTION of the same name, and
POSIX supplies several: `stat`, `sigaction` and `group` are each a struct tag and
a function. C keeps both in one file-scope namespace, so the header stopped
parsing at the clash — and took every declaration after it down. That is not new
to this option; it reproduces on `main` with any `-g` image that calls `stat()`:

```
$ gcc -g -O1 -o t2g t2.c && kuna decompile-project t2g -o P    # main, no options
$ cc -std=c99 -fsyntax-only P/t2g.h
P/t2g.h:67:5: error: 'sigaction' redeclared as different kind of symbol
   67 | int sigaction(int a0,void *a1,void *a2);
P/t2g.h:43:26: note: previous declaration of 'sigaction' with type 'sigaction'
```

`build_header` now resolves it in favour of the TYPE — every other signature in
the file depends on it, while the prototype is one line and is still printed:

```c
/* `sigaction` is a type name above; prototype omitted: int sigaction(int a0,sigaction *a1,sigaction *a2); */
```

`decompiler/crates/kuna-cli/tests/decompile_project_cli.rs
(header_syntax_checks_when_a_type_shares_a_name_with_a_function)` runs
`cc -std=c99 -fsyntax-only` over the generated header in both arms.

**That claim is about the `.h` only, and the `.c` is where the cost lands.** The
exported body was never compilable — the export is a reading aid, not a
rebuildable source tree — but turning this option on makes it measurably less
so, because the shells stay `type_incomplete`: the `.h` says
`typedef struct FILE FILE; /* opaque */` while the `.c` declares objects of that
type and reads fields out of them.

```
$ kuna decompile-project <O2/coreutils/stripped/ls> -o D --option libctypes off
$ cc -std=c99 -fsyntax-only -I D D/ls.c 2>&1 | grep -c 'error:'
911
$ kuna decompile-project <O2/coreutils/stripped/ls> -o D --option libctypes opaque
$ cc -std=c99 -fsyntax-only -I D D/ls.c 2>&1 | grep -c 'error:'
969
```

+58 errors in 15 classes that appear only with the option on. Three of them
follow directly from the incomplete shell and are the bulk of it:

| class | count |
|---|---:|
| `invalid use of incomplete typedef` (`tm` 32, `FILE` 6, `dirent` 5, `passwd` 1, `group` 1) | 45 |
| `storage size of 'vN' isn't known` | 5 |
| `return type is an incomplete type` | 1 |

The rest are the type-name/function-name clash arriving in the body, where the
header's fix does not reach — `ls.c:177` `'sigaction' redeclared as different
kind of symbol`, `ls.c:558` the same for `stat`, and `ls.c:2375`
`sigaction(v6,NULL,(sigaction *)v3);` failing to parse for the same reason. The
`.h` itself is 0 errors in BOTH arms.

The follow-up `glibc` value, which installs the public field layouts instead of
an opaque shell, retires the three incomplete-typedef classes; the
name-clash-in-the-body half is a `build_body` question the header fix in this PR
does not answer.

## Out of scope here

* `stdin`/`stdout`/`stderr` as typed `FILE *` DATA symbols — the largest body
  churn of the family and its own sweep. (The `stdout->field_0x28` rendering
  above arrives through inference from the typed call arguments, not from a data
  symbol.)
* The `glibc` value that installs the public `_IO_FILE` field names
  (`_IO_read_ptr` @8, `_IO_write_ptr` @0x28, …) so the shells print real field
  names instead of `field_0x28` — a follow-up PR on the same table.
