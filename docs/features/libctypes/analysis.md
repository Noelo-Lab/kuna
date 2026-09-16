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

* **One new variant, pointer-only.** `Ty::NamedPtr(&'static str)`. There is no
  by-value and no return-by-value named variant, so the two hazards
  `kuna_dwarfstructs.rs` documents — a by-value aggregate of unknown width
  degrading to a raw integer, and a sizeless aggregate RETURN being classified as
  a hidden-return-buffer call that grows a phantom first parameter — are
  **unrepresentable**, not merely avoided.
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

### 3. Why the shells stay INCOMPLETE

`set_fields_struct` re-ORs the extra flags it is given, so passing
`flags::type_incomplete` back produces a **sized but still incomplete** struct.
That is what lets the DWARF importer complete the same bare name IN PLACE:
`analyzers::dwarf::kuna_dwarfstructs::intern_aggregate` looks the name up with
`find_by_name`, returns a COMPLETE hit verbatim, and populates an INCOMPLETE one.
The factory refuses a second, different definition of a held name
(`find_add`: "Trying to alter definition of type"), and DWARF interns `stat`,
`passwd`, `tm` and `option` under the identical bare spelling — so without this,
a `-g` run would lose the real layouts or error.

Proof, `--option dwarf on --option libctypes opaque` on the UNSTRIPPED twins:

```
fmt  @0x2f30   int get_prefix(_IO_FILE *f)   ...  f->_IO_read_ptr    (no error)
ls   @0xd820   stat v13; // stack - 0x368               (no error)
```

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

## Whole-corpus behavior (`decompile-all`, 6 binaries + fmt, 3,702 functions)

Per-line classification and a control-flow/call/constant skeleton comparison are
in `corpus-hunk-classification.txt`. Summary:

* **0 functions lost or gained**; `PTRSUB(` count unchanged (0) everywhere.
* The dominant line categories are `pointee-named` (the intended change),
  `field-form` (`*(T **)(p + 8)` -> `*(T **)&p->field_0x8`) and `var-renumber`
  (a renumbered local — the scored JSON surface excludes register locals, so this
  is cosmetic).
* 129 of 3,702 functions (3.5%) differ in the skeleton, every inspected one
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

## The project export stays valid C

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

## Out of scope here

* `stdin`/`stdout`/`stderr` as typed `FILE *` DATA symbols — the largest body
  churn of the family and its own sweep. (The `stdout->field_0x28` rendering
  above arrives through inference from the typed call arguments, not from a data
  symbol.)
* The `glibc` value that installs the public `_IO_FILE` field names
  (`_IO_read_ptr` @8, `_IO_write_ptr` @0x28, …) so the shells print real field
  names instead of `field_0x28` — a follow-up PR on the same table.
