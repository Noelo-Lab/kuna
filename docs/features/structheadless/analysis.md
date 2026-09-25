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

Four guards, all measured into the design:

* **A type from outside the recovery outranks a headless record.** coreutils
  `tail`'s `parse_obsolete_option (argc, argv, n_units)` reads only `argv[1]`
  and `argv[2]`, so its first decompile under `closed` took a record, and
  `calleevote` never voted on it: it votes only on a parameter the callee left
  as `void *`, an integer or a one-field record. A headless record is now open to
  the vote too, for a `char *`, a `char **` or a declared record every caller
  passes, and a parameter that took its callers' type is never a headless record
  (`admits`). A caller's synthesized record does not replace one: that is one
  more partial view, and letting it in spent the vote's redo budget on
  record-for-record swaps (tar -O2-noinline declined twelve redos against four
  and lost `decode_timespec`'s `char **arg_lim`). Before the fix, with #719's
  redo budget in, the option lost `parse_obsolete_option` at -O0 (1.0 -> .923).
* **A declared pointer outranks a callee's synthesized record at a call site.**
  shadow `newgrp`'s `main` holds `getgrnam`'s `group *` and hands it to a
  function that reads the group past its start, and `tail`'s `main` hands
  `getopt_long` the `argv` it passes to `parse_obsolete_option`; the callee's
  record retyped the caller's variable (`struct_2 *`, `struct_19 *argv`) until
  any declared pointer with a pointee refused the vote
  (`yields_to_a_declared_pointer`).
* **A headless record that types a dereferenced member as a word is refused as
  a vote.** tar's `wsnode_remove` reads `struct wordsplit` only at `ws_head` and
  `ws_tail` and copies them as words; `wordsplit_varexp` and `wordsplit_cmdexp`,
  which walk the node list from `ws_head`, took that record at the calls they
  make and declared their node pointer `int8`. Applied to every synthesized
  record the refusal also refused offset-0 records a function was right to take
  (grep -O2 `kwsprep` +8 casts, +35 over the 45 binaries), so it is kept to
  headless records.
* **A refused headless record falls back to `void *`.** A headless record is the
  callee's partial view; five of `fts_build`'s callees each read their own part of
  one `FTSENT`, the caller's reads disagree with every one, the votes were
  refused, and the caller's `v21 = a0->field_0x0` fell from `void *` to `long` --
  a cast removed by weakening a type. With the fallback, pointer-to-integer
  declaration changes over the 45 binaries fell from 59 to 12 (8 now, one of them
  a real loss). The first version applied it to any synthesized record, which
  strengthened integers the option-off arm never typed (`find`'s predicate walk
  became `void *` and gained 27 casts), so it stands in for headless records only.

## Results (both arms of one build, a54805cab, on main c960fb18d)

main c960fb18d has castimplied, castarith, castsign and globalref on, so the
option-off arm already prints a `void *` base's reads as one cast each
(`((unsigned int *)a0)[0x2b]` where the first measurement, on dbe854ba3, had
`*(unsigned int *)((long)a0 + 0xac)`) and a constant address as `&dat_<addr>`;
the option removes the member casts, and the count it removes is about half what
it was before castarith. The code under measurement is the code measured on
b3878d32e (build ce4e278b2); only the base moved.

| instrument | off | closed |
|---|---|---|
| castbench, 4,815 shared functions | 37,477 casts, 196.6/kloc, 0.991 x IDA | **36,040**, 189.1/kloc, **0.953 x IDA** (-1,437, -3.8%) |
| castbench x IDA by opt | O0 1.003, O2 1.028, O2-noinline 0.937 | O0 0.945, O2 1.008, O2-noinline 0.900 |
| castbench per 100 statements | 31.2 (IDA 27.0) | 30.0 |
| castbench functions | -- | 219 fewer (-1,584), 57 more (+147) |
| 444-slice typesweep (pinned metric) | 1,615 perfect, mean .3697 | 1,615 perfect, mean .3697; 1 improved, 2 worse, 0 perfect lost |
| arity and phantoms, 20,230 functions | 32,810 parameters, 2,362 phantom locals, 621 phantom arguments | the same in every function; declarations 82,620 -> 82,630 (21 functions +, 12 -) |
| decbench#93 replay | 2,188 credited perfect, mean .4543 | **2,313** (+125), mean .4702 |
| layout, 8 builds, fields only | P .8713 (880/1010), R .0932, F1 .1684 | P .8298 (1068/1287), R .1131, F1 .1991 |
| layout, 14 builds (+ grep, tar -O0/-O2, find, diff -O2) | P .8592 (3300/3841), R .0780 | P .8283 (4695/5668), R .1110 |
| layout at depth (`layoutdepth.py`, 8 builds) | P .9594 (969/1010) | P .9526 (1226/1287) |
| nesting, 8 builds | 3/5, F1 .0036 | 6/9, F1 .0072 |
| TRex Fig. 6, 8 builds | O0 4.4770, O2 1.9597 | O0 4.5050, O2 1.9671 |

The option-off arm is byte-identical to castbench's main c960fb18d arm (0 files
differ), and its typesweep rows equal main's. Every number but the cast counts is
what it was on b3878d32e: globalref changes no variable and no type.

**Precision.** The published instrument scores a claimed field only against the
top-level members of the DWARF record and keeps a named embedded record as one
member (`struct stat stat`, 0x90 bytes at 0x18). A headless reader is very often
one that reads inside such a member -- `f->stat.st_mode` claims 4 bytes at 0x30
-- so its correct claims are top-level misses. Of the 89 top-level misses the
option adds, 69 are members of an embedded record, an array element or a union
member at depth, 6 padding, 5 a member at the same offset read at another width,
4 inside a member, 1 an array element and 4 past the record's end (gnulib's
`FTSENT` name array). Removing every one of the 20 that are not members at depth
would leave precision at .843, still under the .8713 gate; the gate cannot pass
without declining records that are right.

**Records over primitive pointers.** 122 declarations move from a primitive
pointer (`int *` 44, `unsigned long *` 38, `int8 *` 16, `uint8 *` 15, `long *` 9)
to a `struct_N *`. Joined to DWARF, the 102 parameters among them are struct
pointers in 100 cases (find's `parser_table`, sdiff's `line_filter`, grep's `dfa`,
tar's `tar_stat_info`, `wordsplit` and `argp_fmtstream`, du's `FTSENT`), the
primitive pointer being a guess from one member's width; one is tar's
`usage_argful_short_opt` `void *cookie`, an `argp_fmtstream` passed through a
generic callback parameter, and one has no DWARF. No replaced primitive pointer is
the ground truth; the `char **` class the round-I rebase exposed is gone.

**The two worse typesweep rows** are one function at two optimization levels:
bzip2's `BZ2_bzReadClose (int *bzerror, BZFILE *b)`. `BZFILE` is `typedef
void`; the function casts `b` to its own `bzFile *` and reads `writing` at
0x1394, `lastErr` at 0x13e8, `initialisedOk` at 0x13ec, and hands `&strm` at
0x1398 to `BZ2_bzDecompressEnd`. Of its five callers four pass a value they
typed as an integer and one a record of its own, so there is no caller vote, and
nothing in a stripped binary tells an exported API's opaque handle from a record
parameter. The record is the type the body uses; DWARF, which records the
declared `void *`, scores it a miss. It stays, as the one priced false positive
of the kind, and it is one of the two criteria the flip fails. The improved row
is grep's `bmexec_trans`. (The round-I rebase also showed find
`check_path_safety` onto perfect; that was `calleevote`'s redo budget reaching
the function in one arm and not the other, and both arms now decline it.)

**The 57 functions with more casts** (+147). 36 of them (+117) are globalref
declining where it named the address in the option-off arm. A constant address
stays `(T *)0x<addr>` when the function uses it at two pointed-to types, or when
it also reads the storage directly at another start, width or type. A headless
record meets both: tar -O2 `0x23e80` passes `0x82840` (tar's `current_stat_info`)
to `sub_22c60`, whose parameter is now `struct_17 *`, and to `sub_2fc00` and
`sub_22570`, whose parameters are `unsigned long *` in both arms (a guess from
one member's width), so all 17 uses keep a cast (+16). tar -O0 `0x1b745` passes
`0x9fa20` as the 264-byte `struct_231 *` its callee now takes and reads
`dat_9fa90` inside that record directly, which the option-off arm's unsized
`void *` object never overlapped (+1). The record is the right kind in every one
of these; the casts come back because globalref declares one object per address
and refuses rather than print the other uses as casts of it, and because kuna
cannot yet print a direct read inside a named record as its member. They are
priced here, not fixed: the rule that would remove them belongs to globalref.
The other 21 (+30) are the functions the b3878d32e measurement listed: eleven are a call that now passes an
address inside the caller's record to a callee that took a headless record
(`sub_19f5a((struct_N *)&a2->field_0x39[7], ...)`, `(struct_N *)&a1[0x2d]`,
`(struct_N *)(a0->field_0x18 + 0x168)`): the callee's type is right and the
caller has no member to name there; tar -O2-noinline `sub_13140`/`sub_13240`
(+4, +2) read `(char *)a0->field_0x8` where the option-off arm inherited the
nine-function `struct_13` through `calleevote` and now a callee's partial view
answers (the priced cost below); find -O2 `sub_c9f0` (+3) receives a callee's
partial record for a value whose own reads it lacks; sdiff `lf_copy`/`lf_skip`
(+1 each) respell `a0[1]` as a field with one cast at a `rawmemchr` result; the
rest are one or two casts where a member a headless record types as a word is
handed to a `void *` parameter (tar `(void *)a0->field_0x18`), a pointer local is
split off an argument (find `v = (struct_N *)((long)a2 + 0x38)`), or locals are
re-partitioned (grep -O2-noinline `sub_6350`).

**Whole-corpus hunks** (`hunks.py`, 45 binaries, 20,230 functions): 3,378
change; 2,252 only in `struct_N` numbering, 1,050 only in fields, declared types
and casts, 18 only where globalref names a constant address in one arm and not
the other, 0 skeleton deltas, 33 declaration-count deltas and 25 other, all read
(`hunks.md`). The 33 and the 25 are the functions the b3878d32e measurement read.

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
against two worse (bzip2's opaque handle). `default-on-evaluation.md` lists
every criterion.
