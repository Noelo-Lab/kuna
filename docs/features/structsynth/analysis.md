# structsynth — analysis

## 1. The symptom

`put_word` in a stripped `-O2` coreutils `fmt` takes one pointer and reads two
fields of the object behind it. kuna proves what the pointer points at and stops:

```
$ kuna decompile-all .../O2/coreutils/stripped/fmt --addr 0x3000
void sub_3000(unsigned long *a0)
{
  v2 = *(unsigned int *)&a0[1];
  v3 = (unsigned char *)*a0;
  ...
  v6 = *(int *)&a0[1];
}
```

The real type is `WORD { char const *text; int length; int space; ... }`. Nothing
in the output says "offset 0 is a character pointer, offset 8 is a 4-byte count",
so the reader carries that in their head across every function that touches the
same object — and in `ls` the same problem covers gnulib's `struct hash_table`,
whose readers are among the 71 of 404 functions this option changes.

## 2. What is missing, and what is not

The rendering half already works. Supplying the type by hand in the console —
`parse line struct WORDX { char *text; int length; };` plus a prototype — turns
the same p-code into `w->length` / `w->text` with no engine change: `down_chain`,
`get_type_pointer_rel`, `RuleStructOffset0`/`RulePushPtr`/`RulePtrArith` and the
printer all do the right thing unprompted. What is absent is anything that
*produces* the type: `set_fields_struct_raw` has exactly one production caller in
the tree (`kuna_pebnames.rs`, for the Windows `TEB`), and the `aggregate-union`
subphase row in `phases.toml` has advertised "when to synthesize struct/array"
with nothing behind the struct half.

That same console path is also the precedence proof: with
`parse line struct WORDX {...}` and `map function 0x3000 putword` in place,
`structsynth param` still prints `w->length` / `w->text`, not `field_0x8` /
`field_0x0` — a named composite pointee declines. `libctypes` (#651, default-on
since #658) is the production source of such names, and since the flip the
precedence is exercised by the corpus rather than only by the console.

**What the flip changed.** An earlier round of this document said the rule
protects a parameter something *names* and protects nothing where nothing names
it, and counted nine DWARF `FILE *` among the retyped parameters — `rpl_fseeko`
in all eight census binaries and `fmt`'s `fmt` @0x3700 — on the reasoning that
these are gnulib wrappers with no import prototype for `libctypes` to supply.
That reasoning was wrong about *where* the name comes from. `libctypes` declares
`int fileno(FILE *)`, and ordinary propagation carries that back from the call
site into the wrapper's own parameter, so the wrapper is named after all. All
nine decline now:

```
$ kuna decompile-all fmt --addr 0x42d0 --option structsynth param
unsigned long sub_42d0(FILE *a0,unsigned long a1,unsigned int a2)
$ kuna decompile-all fmt --addr 0x42d0 --option structsynth param --option libctypes off
unsigned long sub_42d0(struct_0 *a0,unsigned long a1,unsigned int a2)
```

`fmt` @0x3700 is the same pair, and `get_prefix` @0x2f30 and `get_line` @0x3420
— the two the refuters used to demand the pointer precondition — now print
`FILE *a0` with the option on. The census is 292 with the current default and
301 with `libctypes off`; the 9 is the whole difference. The ARM census still
carries one `__sFILE`, on `RTOSDemo`'s `__sflush_r` @0x118: newlib, which the
glibc tables do not name. What remains true is the ordering claim — a type lock
installed by this pass would win over a later inference, and this pass runs
first — so a pass that named `FILE *` *after* P5 would still collide.

## 3. The opportunity pool

`structscore.py --census` over the four coreutils used for the type campaign,
splitting the pool by what kind of evidence each candidate base has (a
field-committing `*(T *)(B + K)` versus an index-only `B[k]`, which an array walk
reads the same way):

| | O0 fns | O0 candidates | O2 fns | O2 candidates |
|---|---:|---:|---:|---:|
| fmt | 191 | 5 | 151 | 7 |
| ls | 589 | 21 | 404 | 45 |
| sort | 479 | 25 | 343 | 16 |
| du | 441 | 40 | 320 | 30 |

On the field-committed half, the number of candidates whose current type *already*
matches DWARF is **0** at O2 and **1** across all four at O0 — so the match→miss
channel this lane could spend is essentially empty before it starts.

## 4. Why the obvious design is wrong in four places

Each of these was proven necessary against the tree before the code was written.

**The base must already be a pointer.** With no such precondition, `param` fires
on `fmt`'s `get_line`, whose first parameter is the real `FILE *` but is still
typed `long`: two constant offsets, no offset-0 access, no non-constant term,
nothing else to decline on — and the pass would lock a phantom two-field layout
onto a libc type a named-type pass can still recover. Pointer-ness is the one fact
this pass does not invent.

**Phis must not be peeled.** Chasing `COPY`/`MULTIEQUAL` to a root turns every
strided walk into constant offsets on the root: `ls`'s `last_component` and
`hash_string` (`char *` in DWARF, `while (*p) p = &p[1]` in the body) report
offsets `{0, 1}` and become two-field structures over a string.

**`has_type_recovery_started` is not a settled lattice.**
`ActionInferTypes::apply` returns 0 unconditionally, so type propagation never
drives the enclosing `REPEAT` and nothing waits for a fixpoint; the flag is armed
at the end of the *first* `fullloop`, so an action gated on it reads the first of
up to seven passes. Measured with `KUNA_ACTION_PROF` on `fmt` O2: `put_word` runs
6 `infertypes` passes and `main` 9.

**`return 1` marks nothing.** `Action::perform` compares `lcount < count` and reads
the return value only for `res < 0`, so the extra `mainloop` iteration the whole
propagation story depends on needs `self.base.count += 1`.

**`update_type_locked` alone is a no-op downstream.** `HighVariable::update_type`
returns at its first line unless `type_dirty()` was called, so `high_get_type` —
and with it the printed prototype — keeps reading the stale type.

## 5. The layout rules, and why each direction

**Widest wins an offset.** A field wider than an access renders as a cast of the
field (`struct W2 { long a; long b; }` on `put_word` gives `v2 = (uint4)w->b;`); a
field narrower than an access loses the field name altogether
(`struct W9 { int a; int b; int c; }` gives `v3 = *(uint1 **)w;`). Both are wrong,
only one is readable.

**No `variable_length`.** It looks like the guard against a past-the-end offset
becoming element `[1]` of an array of the structure, but `propagate_from_pointer`
refuses a variable-length pointee outright and `AddTreeState` sets `size = 0` for
one, which makes every `TypePointerRel` bail. A fixed size is what ships; what
happens past the end is below.

**Uniform run ⇒ array.** Howard NDSS'11 §4.5 in its static form. Three or more
equal-width offsets on a regular grid is that evidence outright; two counts only
when the element is narrower than a pointer. This rule alone removed 10 of the 14
functions the option cost on `type_match` (§6.3), at zero cost to layout-F1.

The run does not have to be contiguous. Demanding exact contiguity made **any
byte of a buffer the function does not touch** turn that buffer into a
structure — four byte stores at 0, 1, 3 and 4 gave
`void gapped(struct_0 *a0) { a0->field_0x0 = 0x41; … }` while the same function
writing 0,1,2,3 correctly gave `char *`. So below pointer width a gap is
tolerated; at pointer width it is not, because a record whose observed members
are all pointer-sized with an untouched one between them is the common case
(`gnulib`'s `struct hash_table` is ten pointer-sized members whose readers touch
different subsets of them — over `ls` O2's 36 synthesized layouts the commonest
widths are two, three and four claimed fields). Cost of the relaxation,
measured: 3 of 304 retyped parameters.

**An index is an index whether or not this function dereferences it.** `p + n`
for a non-constant `n` is the array signal even where only a callee reads
through the result. `ls` O2's `mpsort` @0x131b0 — DWARF `void **base` — reads
`base[0]` and `base[1]` itself and passes `&base[n]` to its recursive callee;
with the signal restricted to dereference sites it was retyped, and the emitted
call argument went from `&a0[a1]` to `&(&a0->field_0x0)[a1]`, which indexes a
fixed layout by a runtime count in the same breath. It now declines and the
function is byte-identical off and on.

It takes `int fd[2]` only in its plain form. `sort` O2's `rpl_pipe2` @0xe990 —
DWARF `int fd[2]` — also saves and restores the pair as one 8-byte word, so its
evidence is `{0: width 8, 4: width 4}`, the widths differ and the uniform-run
test says nothing. The layout prune below is what declines it: offset 4 sits
inside the bytes offset 0 claims, so one field is left and one field is not a
layout. Off and on are byte-identical on that function.

**The layout has to survive a C compiler.** `kuna decompile-project` writes the
synthesized declarations into the project header, as source, and the bodies
alongside them. Neither is worth anything unless the compiler that reads them
back puts each member where the body says it is, and the body text cannot tell
you whether it does — it goes on naming the fields either way. Three shapes of
access break the *declaration*:

* a width the exported prelude respells: `undefined3` is `unsigned int` there
  (4 bytes), `undefined5`/`6`/`7` are `unsigned long long` (8);
* an offset the field's own alignment does not divide — `unsigned long
  field_0x1c` is placed at 0x20 and everything after it moves;
* an access inside a wider field's bytes — `long field_0x0; int field_0x4;`
  puts `field_0x4` at 8 and `field_0x8` at 16.

Each is dropped. What survives is naturally aligned and 1, 2, 4 or 8 bytes wide,
and two such ranges either nest or are disjoint — the surviving layout provably
cannot overlap.

Two more shapes break the *body* against its own header, and the declaration
check cannot see either:

* **an offset in a hole.** `printc.cc:1015-1033` resolves an address inside a
  structure to the field that contains it and, when no field does, invents the
  member name `field_0x<hex>` anyway, while the exported header renders the same
  gap as `undefined1 _pad<hex>[N]`. Address arithmetic reaches those offsets
  without a dereference — `p + 8` handed to a callee prints `&p->field_0x8` —
  so it happened: over 10 exported projects, **44** `error: 'struct_3' has no
  member named 'field_0x8'`, `fmt` O2 among them. Every gap is now a real
  `undefined1 field_0x<hex>[N]` member of alignment 1 at its own offset.
* **a size that is not a multiple of the structure's alignment.** A layout whose
  last field is a byte at 0x14 is 0x15 bytes to the decompiler and 0x18 to the
  compiler, so `a0[1]` in the exported body is a different address. `chibios`'s
  `chPipeReset` indexes its parameter exactly that way. The size is rounded up
  to the widest field, with the tail covered by filler like any other gap.

Past the end there is no member and no invented name, and the printer falls back
to an element of the structure array plus a byte offset: a pruned read at
0x10001 of a 16-byte layout renders as
`*(unsigned int *)((long)&a0[0x1000].field_0x0 + 1)`. That compiles and computes
the right address; it is readable only by accident.

Filler is always an array, one byte included, so an array-typed member is
exactly the padding and never a field the pass claims — which is how §6.1
separates the two readings of precision.

The checks are mechanical, and there are two, because the declaration check
cannot see the body:

* `offsetcheck.py` compiles the exported header with a
  `_Static_assert(offsetof(struct_N, field_0xK) == K)` per member. Over the
  eight sweep binaries the prototype **without** the prune fails 10 of 154
  structures (30 bad offsets in `ls` O2 alone, and `du` O2's `struct_32`
  misplaces 9 of its 10 fields). The branch: **0 of 352** over 14 exported
  projects, 2,174 members. (The two ARM32 projects need `--m32`; a 64-bit host
  compiler gives their `T *` members 8 bytes and moves every later field. That
  is the instrument.)
* `kuna decompile-project` + `gcc -fsyntax-only` over the whole project compiles
  the *bodies*. The export does not compile today for reasons unrelated to this
  option, so the reading is the delta: over 14 binaries the option adds five
  errors in total, all of them the pre-existing `'NULL' undeclared`, and **zero**
  `has no member named 'field_0x…'` — against 44 of them before the layout was
  made dense. `docs/features/structsynth/sweep.md` has the table.

The end-to-end version of the declaration failure: on

```c
unsigned long lo = *(unsigned long *)p;
int           hi = *(int *)((char *)p + 4);
*(unsigned long *)((char *)p + 8) = lo + hi;
```

the unpruned export gave `struct_0 { long field_0x0; int field_0x4; long
field_0x8; }` with a body that assigns `a0->field_0x8`, and recompiling that
header and body verbatim ran a different program (`b[1]=0 b[2]=137438953473`
against the original's `b[1]=137438953505 b[2]=0`). Pruned, the export is
`{long field_0x0; long field_0x8;}` with
`a0->field_0x8 = (long)*(int *)((long)&a0->field_0x0 + 4) + a0->field_0x0;`,
which recompiles to `b[1]=137438953505 b[2]=0` — the original.
`tests/stages/structsynth-overlap-layout.xml` is that case.

## 6. Measured

### 6.1 Layout precision (`structscore.py` + `layoutscore.py`, 8 binaries)

Layout-F1 is field-level P/R/F1 against the DWARF twin's binary layout,
restricted to pointer-to-struct parameters (RecStruct §4.1 Task 1). kuna scored
**0.0 everywhere** before, because it synthesized no structures at all — which
makes F1 a tautology on its own. **Precision is the number that decides this
option**: of every field the pass emits for a parameter DWARF says is a struct
pointer, the share that lands on a real field at the same offset and size.

A synthesized layout has two kinds of member, and they must be read separately.
A **claimed field** is a scalar or a pointer at an offset something
dereferenced. **Filler** is the `undefined1 field_0x<hex>[N]` that covers a gap
so the printer cannot name an offset the header does not declare; it is padding,
which is what `measure_layout` already drops on the ground-truth side. Filler is
always an array type and a claimed field never is, so the two are separable —
`docs/features/structsynth/layoutscore.py` reports both.

| binary | GT fields | claimed fields | matched | **precision** | recall | F1 before → after |
|---|---:|---:|---:|---:|---:|---|
| fmt O0 | 556 | 12 | 11 | **0.917** | 0.020 | 0.0 → 0.039 |
| fmt O2 | 402 | 6 | 6 | **1.000** | 0.015 | 0.0 → 0.029 |
| ls O0 | 1761 | 124 | 113 | **0.911** | 0.064 | 0.0 → 0.120 |
| ls O2 | 800 | 122 | 107 | **0.877** | 0.134 | 0.0 → 0.232 |
| sort O0 | 1779 | 168 | 144 | **0.857** | 0.081 | 0.0 → 0.148 |
| sort O2 | 1113 | 112 | 98 | **0.875** | 0.088 | 0.0 → 0.160 |
| du O0 | 1926 | 126 | 116 | **0.921** | 0.060 | 0.0 → 0.113 |
| du O2 | 1106 | 117 | 109 | **0.932** | 0.099 | 0.0 → 0.178 |
| **pooled** | **9,443** | **787** | **704** | **0.8945** | **0.0746** | **0.0 → 0.138** |

**About 1 in 10 claimed fields is not a DWARF field at that offset and size.**
Counting filler as a prediction the number is 798/1,021 = 0.782, and that is the
figure to quote only if padding counts as a claim on both sides, which it does
not.

Measured on the rebase, so `libctypes` is on. Before it, the same table pooled
846 claimed and 762 matched (0.9007): the nine `FILE *` parameters of §2 carried
59 claimed fields, 58 of which matched, because `_IO_FILE` is a real structure
whose offsets the pass read correctly — it was the *name* that was wrong. Losing
them costs 0.006 of precision and is the right trade.

An earlier round of this proposal reported 772/1,091 = 0.708 for the same
measurement. That number was deflated by the instrument, not by the pass:
`header_layouts` parses the exported `undefined1 _pad<hex>[N]` members as fields
(`FIELD_OFFSET_RE` matches `_pad`) and `measure_layout` counts every member of
the predicted layout while dropping `kind == "pad"` from the ground truth, so
roughly 240 padding members were scored as failed predictions. The dense layout
removed `_pad` from synthesized structures entirely; the reading above separates
what replaced it.

A miss is usually a field the function reads at a narrower width than it was
declared (an `int` read out of a `long`), an access to an interior member of an
embedded structure, or a read of padding. Recall is structurally low
(0.03 – 0.14) and always will be: the layout only ever has the fields *this
function* touched.

For calibration, RecStruct's published coreutils-O2 layout F1: RecStruct 22.06,
TypeForge 20.44, ReSym 15.86, TRex 7.76. Read that comparison with care — the
ground-truth normalization is ours, not theirs, and the restriction to
parameters is the same but the binary set is not.

The TRex Fig. 6 prioritized score and its `IsCStruct` step, re-run on the
rebased build:

| binary | TRex mean off → on | `IsCStruct` passed off → on |
|---|---|---:|
| fmt O0 | 3.9952 → 4.0242 | 273 → 277 |
| fmt O2 | 1.8318 → 1.8460 | 126 → 128 |
| ls O0 | 3.6197 → 3.6692 | 1110 → 1139 |
| ls O2 | 1.4919 → 1.4956 | 401 → 361 |
| sort O0 | 3.6384 → 3.7040 | 892 → 925 |
| sort O2 | 1.6549 → 1.7092 | 308 → 330 |
| du O0 | 3.7085 → 3.7782 | 886 → 918 |
| du O2 | 1.6644 → 1.7366 | 302 → 329 |
| **net** | up on all eight | **+109** |

`ls` O2 is the one binary whose `IsCStruct` step regresses, and the reason is
§6.2's `void *` class: 64 of its retyped parameters are `qsort` comparators
DWARF spells `void const *`, so naming their real record loses the step.

### 6.2 What gets retyped, and what is a false positive

Every parameter the option retypes to `struct_N *`, matched positionally against
the DWARF twin (`docs/features/structsynth/fprate.py`). **Two binary sets are
used in this document and they are not the same eight binaries**: the *census
set* is `fmt`/`ls`/`sort`/`du` at O0 and O2 (292 retyped parameters), and the
*sweep set* in `sweep.md` is `fmt` O0/O2, `ls` O2, `sort` O2, `du` O0/O2,
`gzip` O2, `bzip2` O2 (230), plus three non-x86-64 binaries.

| what DWARF says the parameter is | census set | ARM32 set |
|---|---:|---:|
| a pointer to a structure | 182 | 29 |
| a pointer to something that is not a structure | 71 | 1 |
| no resolvable type (`.isra`/`.constprop` clone DIEs) | 39 | 3 |
| no such function in DWARF | 0 | 4 |
| **not a pointer at all** | **0** | **0** |
| total | **292** | **37** |

Measured on the rebased tree, so `libctypes` is on. With `--option libctypes off`
the census set is 301 and the nine extra rows are the `_IO_FILE` of §2.
`fprate.py` filters the count to the `struct_N` names this pass mints: with
`libctypes` on, 289 parameters of the same eight binaries are struct-typed with
`structsynth off`, and counting every struct-typed parameter would read 581.

The ARM32 set is `chibios` and `freertos` at O2 — 32-bit targets, where the
`width < ptr_size` half of the array rule can never fire for a 4-byte element.
It is measured because every other number here is 64-bit gcc ELF.

By DWARF **pointee spelling**, which is the reading that says whether a
named-type pass would have owned the parameter, the census set is: 70 `void`,
66 `hash_table`, 39 with no resolvable type, 8 each `quoting_options` / `stat` /
`fileinfo` / `obstack`, 7 each `scratch_buffer` / `line` / an anonymous `fts`
struct, 6 each `hash_tuning` / `argv_iterator`, 5 `merge_node`, then a tail of 4
and fewer down to one each of `pending`, `char`, `md5_ctx`, `patopts`,
`_ftsent`, `mount_entry`, `I_ring`. There is no `_IO_FILE` row any more — the
one a named-type pass would have owned, `libctypes` now owns.

Of the 71 non-struct pointers, **70 are `void *`** — `ls`'s `qsort` comparators
(`xstrcoll_atime`, `rev_strcmp_df_mtime`, `dev_ino_compare`, …), `triple_hash`,
`du`'s `AD_compare` — whose bodies genuinely read `struct fileinfo` /
`struct AD_ent` fields through that `void *`. The recovered layout is right about
the object and wrong about the declaration.

**One is an ordinary false positive**, and it is worth naming: `ls` O2
`strmode` @0xfa20, DWARF `void strmode(mode_t, char *str)`. The body writes
`str[0]` … `str[9]` one byte at a time and then `str[10..11]` as a single 2-byte
store, so the evidence is ten 1-byte offsets and one 2-byte offset. The uniform
run needs one width and sees two, and the layout prune cannot collapse anything
because the wide access is past the narrow ones rather than over them, so an
11-field structure is minted over a `char[12]` buffer.

**"One in 292" is a count, not a rate.** It counts only the shape the census can
adjudicate: a ground truth that is not a pointer at all, or a pointer to a
non-aggregate. A `void *` that really does point at a record is not counted
against the pass, and neither is a layout that is wrong about a parameter DWARF
agrees is a struct pointer — §6.1's precision is where that shows up. The
gap-tolerant array rule closed the one class the earlier round could not see
(any buffer with an untouched byte); `strmode`'s mixed-width tail is the class
that remains, and closing it means declining a real `{int,int,int,int,long}`
record that tiles its bytes the same way.

### 6.3 `type_match` (bidirectional typesweep, 214 slices, 5,298 functions)

| | off | on |
|---|---:|---:|
| perfect | 594 | 593 |
| aggregate | 1696.15 | 1695.24 |

One sweep against the pinned decbench corpus (`coreutils`, `grep`, `gzip`,
`diffutils`, `bzip2`, `findutils`, O0 and O2, `DECBENCH_NO_CACHE=1`), re-run on
the final build; the earlier build gave the same four rows and the same
aggregate to the cent.

0 improved, 4 worsened, 1 of them off perfect. All four are `powm2` and
`millerrabin2` in coreutils `factor`, at both optimisation levels: GMP limb
arrays (`mp_limb_t *`) read at offsets 0 and 8, which the uniform-run rule keeps
as a structure because both elements are pointer-sized. Declining every
`{0:8, 8:8}` layout would erase this last loss and cost a fifth of the recovered
struct parameters, which is why the rule stops where it does.

The lane is therefore **slightly metric-negative** (−0.05% of the aggregate) and
that is expected: `type_match` compares pointee spellings by name, so a perfect
`struct_0 *` can never intersect a ground-truth `WORD *`. This is a quality
feature measured by `structscore`, not a `type_match` mover, which is also why it
is default-off and not in `aggressive`.

### 6.4 Speed

Interleaved off/param, min-of-15 (`.scratch/speed.py`). Three runs, because this
box is shared and the load average during the last one was 20:

| case | run 1 (quiet) | run 2 (contended) | run 3 (load 20) |
|---|---:|---:|---:|
| whole binary `fmt` O2 (5/151 fire) | +0.37% | +0.68% | −0.06% |
| whole binary `ls` O2 (71/404 fire) | −0.27% | −1.23% | +1.25% |
| whole binary `grep` O0 (inert control, nothing fires) | −1.83% | **+5.00%** | **+0.66%** |
| one function, `kuna decompile fmt` @0x3000 | +0.20% | −0.86% | +2.84% |

The inert control is the reading: nothing can fire there, so its movement is the
noise floor. The whole-binary cases sit inside it in all three runs and in both
directions. The single-function case is 163 ms, so its +2.84% is 4.6 ms and it
is the least trustworthy row — it was +0.20% and −0.86% on the other two runs.
The design's predicted +8% for the forced extra `mainloop` iteration does not
survive measurement — `ActionInferTypes` is 1–4% of whole-binary time, only the
2–18% of functions that fire pay the extra round, and the collector itself costs
0.3 ms on `fmt` and 2.1 ms on `ls`. Default-off is a metric decision, not a speed
one.

### 6.5 What the corpus sweep says

`docs/features/structsynth/sweep.md`, 11 binaries and 2,801 functions: 213
functions change, and **one** of them has a control-flow or call-token delta
(`chibios` O2 `sub_800397c`, a `for` that becomes a `while` with the increment
hoisted; read by hand, semantically identical). The eight x86-64 binaries have
none, which is what an earlier round of this proposal reported as a property of
the pass — it is a property of those eight binaries. Eleven functions declare
one more or one fewer variable, and 27 are read by hand there, grouped by shape.

## 7. What a reader gets

`ls` O2, `hash_do_for_each` @0x107a0 — the gnulib `struct hash_table`, offsets
0x0/0x8 = `bucket`/`bucket_limit`:

```c
-unsigned long sub_107a0(unsigned long *a0)
+unsigned long sub_107a0(struct_20 *a0)
-  v2 = (long *)*a0;
+  v2 = a0->field_0x0;
   do {
-    if ((long *)a0[1] <= v2)
+    if (a0->field_0x8 <= v2)
```

and @0x10600 — DWARF `transfer_entries(struct hash_table *dst, struct
hash_table *src, bool safe)` — where the same object is read at six offsets,
`0x0`/`0x8`/`0x10`/`0x18`/`0x30`/`0x48`, every one of them exactly `bucket`,
`bucket_limit`, `n_buckets`, `n_buckets_used`, `hasher` and `free_entry_list`.

The widest one in this corpus is `hash_rehash` @0x10f10, whose `struct_28` is

```c
struct struct_28 {
    void *field_0x0;            /* bucket           */
    void *field_0x8;            /* bucket_limit     */
    unsigned long field_0x10;   /* n_buckets        */
    unsigned long field_0x18;   /* n_buckets_used   */
    char field_0x20[8];         /* n_entries -- filler, never read here */
    long field_0x28;            /* tuning           */
    unsigned long field_0x30;   /* hasher           */
    unsigned long field_0x38;   /* comparator       */
    unsigned long field_0x40;   /* data_freer       */
    unsigned long field_0x48;   /* free_entry_list  */
};
```

— nine of the ten members of the real `struct hash_table`, each at its own
offset. (The comments are this document's, not kuna's: the pass has no names.)
