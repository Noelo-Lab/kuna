# structheadless: a record read only past its start

## The gap

`structsynth` takes a pointer as a record only when something reads it at
offset 0. A function that reads a record's later members and never its first
keeps the pointee it had, which for a pointer known only by its dereferences is
`void *`, and every access costs two casts. coreutils `ls` -O0:

```text
unsigned int sub_53e7(void *a0)
{
  if ((dat_2b358) && (*(char *)((long)a0 + 0xb9)))
    v1 = *(unsigned int *)((long)a0 + 0xac);
  else
    v1 = *(unsigned int *)((long)a0 + 0x30);
```

0x30, 0xac and 0xb9 are `struct fileinfo`'s `stat.st_mode`, `linkmode` and
`linkok`. `sub_5297` (`statx_to_stat`) reads eighteen members of a `struct
statx` and never `stx_mask` at 0, so its first parameter is `void *` while its
second, which it writes at 0, is `struct_4 *`.

## Census: what structsynth refuses and why (measured before any design)

Instrument: `census.patch` (a temporary build of the pass, never committed)
prints one line per candidate base with the function, the base's kind and
storage, its offsets and the first condition that refused it, driven over the
45 castbench binaries (x86-64 coreutils fmt/ls/sort/du/cp/tail/wc, grep, gzip,
diffutils cmp/diff/diff3/sdiff, tar, findutils find, each at -O0, -O2 and
-O2-noinline) on main dbe854ba3.

**Most functions are never offered.** The pass reads the type lattice only once
a propagation pass changed nothing, and 12,888 of the 20,230 functions never
reach that state: their main loop ends on the pass that first typed them.
Asking every function with a candidate for one more pass fires 1,475 more of
them and accepts **no** new record under today's rules; every candidate there is
refused, 287 of them for having no offset-0 access. A record read at 0 gives the
pointer-arithmetic rules a `*a0` to rewrite, which is what makes a function
iterate again; a headless one gives them nothing.

**Bases with two or more constant offsets, by kind and first refusal** (with the
extra pass):

| kind | bases | accepted | refused, by reason |
|---|---:|---:|---|
| parameter | 3,860 | 1,716 | no offset 0: 1,238; uniform run 362; named pointee 170; index 148; phi 94; type-locked 67; frame/global 39; not a pointer 15; integer use 11 |
| pointer a call returned (`locals`) | 1,023 | 246 | locals rule 534; not a pointer 112; uniform run 98; index 17; integer use 8; named 5; no offset 0: 3 |
| phi output | 1,618 | -- | not a base |
| frame (the stack pointer) | 799 | -- | not a base |
| pointer loaded from memory | 736 | -- | not a base |
| global | 104 | -- | not a base |

Stack-resident records stay declined: a frame base is the stack pointer, whose
offsets are every local's, and a record there is a local the frame layout owns;
slot typing does not change that. Integer-typed pointer parameters are 15
(`ptrfromuse` already makes nearly every dereferenced parameter a pointer).
Phi-reached parameters (94) and phi outputs (1,618) are loop walks and joins, a
different hypothesis with its own array-walk risk, and are not taken here. The
one large refusal that is a rule rather than a missing base is **no offset 0:
1,238 parameters**.

**Ground truth for the refused parameters** (DWARF, joined by function address
and argument register):

| bucket | struct pointer | `void *` | other pointer | integer | DWARF blind (-O2 abstract origin) |
|---|---:|---:|---:|---:|---:|
| no offset 0, closed function (965) | 883 | 0 | 2 (`char **`) | 3 | 75 |
| no offset 0, address stored (273) | 202 | 53 | 0 | 0 | 17 |

In a function every caller of which is a known direct call (`calleevote
fields`' closed set) a headless parameter is a struct pointer in 883 of the 890
cases DWARF can speak to. Where the function's address is stored, 53 of 255 are
`void *` -- `qsort` comparators and hash callbacks, whose contracts declare
`void *` and which the metric scores against it.

**The cast census's `void *`-base sites.** Of the 1,080 `*(T *)((long)p + K)`
sites over a `void *` base on the shared (kuna and IDA) function set: 366 at 49
bases are closed-function headless parameters; 228 at 74 bases are headless in a
function whose address is stored; 117 at 91 are one-field reads in such a
function (declined on purpose by the lone-field rule); 143 at 31 are stack-passed
parameters or functions the census could not join; about 150 are locals (phi
outputs, loaded pointers, refused call returns); 34 are indexed bases.

**Nesting (why `nest` claims five members).** Under `structsynth all`, over the
eight layout builds, the pointer-width members of accepted parameter records
are: 970 loaded but never dereferenced in the loading function (almost all
8-byte integers compared or added; the few real pointers reach only `void *` or
code consumers), 251 never loaded, 26 whose loads reach a phi, an index or an
integer operation, 13 read at one offset, 6 read as a uniform run, 2 with no
offset 0, and none clean. Within one function the nesting evidence is exhausted;
the remaining route is interprocedural (a callee's parameter record flowing back
into the member), which this option feeds: nesting goes 3/5 -> 6/9.

## The rule

`structheadless closed`: a parameter read at two or more constant offsets, none
of them zero, is a record when every caller of its function is a known direct
call and nothing outside the function gave the parameter its pointee. Every
other `structsynth` condition applies, plus the one `locals` adds for a partial
view: no address formed at or past the end of every access (`points_past`); cp's
`(char *)a1 + 0x100` otherwise printed as `&a1[2].field_0x30`. The bytes before
the first access are `undefined1` filler. Such a function asks for the one extra
propagation pass a closed lone field already asks for.

Two call-site guards, both measured into the design:

* **A declared named record outranks a callee's synthesized one.** shadow
  `newgrp`'s `main` holds `getgrnam`'s `group *` and hands it to a function that
  reads the group past its start; that callee's record retyped the variable
  `struct_2 *` (type_match 0.8261 -> 0.7826 on the function) until the vote was
  refused there.
* **A refused headless record falls back to `void *`.** A headless record is the
  callee's partial view; five of `fts_build`'s callees each read their own part of
  one `FTSENT`, the caller's reads disagree with every one, the votes were
  refused, and the caller's `v21 = a0->field_0x0` fell from `void *` to `long` --
  a cast removed by weakening a type. With the fallback, pointer-to-integer
  declaration changes over the 45 binaries fall from 59 to 12. The first version
  applied it to any synthesized record, which strengthened integers the
  option-off arm never typed (`find`'s predicate walk became `void *` and gained
  27 casts), so it stands in for headless records only.

## Results (both arms of one build, e2071591f, on main dbe854ba3)

| instrument | off | closed |
|---|---|---|
| castbench, 4,815 shared functions | 45,126 casts, 236.8/kloc, 1.193 x IDA | **42,008**, 220.4/kloc, **1.111 x IDA** (-3,118, -6.9%) |
| castbench x IDA by opt | O0 1.234, O2 1.208, O2-noinline 1.135 | O0 1.115, O2 1.159, O2-noinline 1.053 |
| castbench functions | -- | 221 fewer (-3,150), 20 more (+32) |
| 444-slice typesweep (pinned metric) | 1,609 perfect, mean .3688 | 1,609 perfect, mean .3687; 1 improved, 2 worse, 0 perfect lost |
| decbench#93 replay | 2,181 credited perfect, mean .4532 | **2,305** (+124), mean .4691; +607 credited variables |
| layout, 8 builds, fields only (`structsynth param`) | P .8713 (880/1010), R .0932, F1 .1684 | P .8298 (1068/1287), R .1131, F1 .1991 |
| layout at depth (`layoutdepth.py`) | P .9594 (969/1010) | P .9526 (1226/1287) |
| nesting | 3/5, F1 .0036 | 6/9, F1 .0072 |
| TRex Fig. 6, 8 builds | O0 4.4747, O2 1.9520 | O0 4.5027, O2 1.9594 |

**Precision.** The published instrument scores a claimed field only against the
top-level members of the DWARF record and keeps a named embedded record as one
member (`struct stat stat`, 0x90 bytes at 0x18). A headless reader is very often
one that reads inside such a member -- `f->stat.st_mode` claims 4 bytes at 0x30
-- so its correct claims are top-level misses. `layoutdepth.py` scores the same
claims with named embedded records, arrays of records and unions flattened: the
282 claims the option adds are real members at depth in 262 cases (92.9%),
against 95.9% for the records `param` already makes. The residue is padding, a
read at a member's offset with a different width, and four reads past the DWARF
record's end (gnulib's `FTSENT` name array).

**The two worse typesweep rows** are one function at two optimization levels:
bzip2's `BZ2_bzReadClose (int *bzerror, BZFILE *b)`. `BZFILE` is `typedef void`;
the function casts `b` to its own `bzFile *` and reads it past the start. The
record is the truer type and DWARF scores it a miss; nothing in a stripped binary
tells an API's opaque handle from a record pointer. The improved row is grep's
`bmexec_trans`.

**The 20 functions with more casts** (+32): most are one or two casts where a
local's pointer type changed and a read is respelled at the same address
(`v6[1]` <-> `*(int8 *)(v6 + 8)`); `find` -O2 `sub_c9f0` (+4) and grep
`sub_6350` (+2) receive a callee's partial record for a value whose own reads it
lacks (right kind, missing members); tar -O2-noinline `sub_13140`/`sub_13240`
(+4, +2) are answered by a one-member record where the option-off arm inherited
the nine-function `struct_13` through `calleevote`, because their callees now mint
their own partial views (the priced cost below); tar -O2 `sub_13570` spells three
`return NULL` as `return (char *)0` while dropping 76 casts elsewhere.

**Whole-corpus hunks** (`hunks.py`, 45 binaries, 20,230 functions): 3,450
change; 2,308 only in `struct_N` numbering, 1,073 only in fields, declared types
and casts, 8 skeleton deltas, 33 declaration-count deltas and 28 other, all read
(`hunks.md`).

## The priced cost: one object, several names

A headless record declares the members its reader measured and nothing before
them. Readers that measure different parts of one object get different names
unless the ledger's containment rule answers one with another, so an object the
option-off arm typed once (through `calleevote`'s vote) or not at all can carry
several `struct_N` names. The tar -O2-noinline `tar_stat_info` cluster goes from
one name over nine functions to five, and 12 declarations move from a pointer to
an integer where a function is answered by a record whose member at that offset
is typed differently. `structmerge siblings` (off by default) is the rule that
unions agreeing readers; it is not flipped here.

## Default

Off. Two flip criteria fail as written: fields-only precision on the published
instrument falls below .8713 (.8298), and the typesweep has one improved row
against two worse. `default-on-evaluation.md` lists every criterion.
