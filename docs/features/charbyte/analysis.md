# charbyte — census and analysis

Why a byte read through a `char *` is declared `unsigned char`, how often it
happens, and what the fix changes. Measured on the decbench
`full_run_address_2026-09-11` stripped binaries, coreutils `fmt`, `ls`, `sort` and
`grep` at `-O0` and `-O2` (3,248 functions), with `kuna decompile-all --json`
under the default `--mode auto`.

**Metric effect: zero by construction for the signedness it fixes.** decbench's
`type_match` strips `unsigned` before comparing, so `unsigned char` and `char` are
the same answer to it. This is a readability item; the typesweep in
`record.json` is there to show that nothing *else* moved.

## 1. Root cause

x86 loads a byte with `movzx` whether the program meant it signed or not, so `*p`
lifts to `LOAD` followed by `INT_ZEXT`.

1. `TypeOpIntZext::get_input_local` votes `get_base(1, TYPE_UINT)` = `uint1` for
   the byte. In `get_local_type`'s fold `SUB_UINT_PLAIN` (16) outranks
   `SUB_INT_CHAR` (19), so the byte is seeded `uint1`.
2. When a `char *` later reaches the `LOAD`'s address, `propagate_from_pointer`
   offers `char` for the byte and `propagate_type_edge` refuses it (`char` ranks
   lower). A census hook counted **40** distinct `LOAD` sites of this shape
   ("blocked") over the 8 binaries.
3. When the byte is visited first, it pushes `uint1 *` over the `char *` pointer,
   and that push wins: **65** distinct `LOAD` sites ("infect").

The C shape (fmt -O2 `get_line`, and every gnulib `mbrtowc` wrapper):

```c
v1 = (unsigned char *)*dat_c100;
v7 = strchr("([\'`\"",(int)(char)*v1);

unsigned long sub_79b0(unsigned int *a0,unsigned char *a1,unsigned long a2,mbstate_t *a3)
  v1 = mbrtowc(a0,(char *)a1,a2,a3);
```

## 2. Census (the gate for doing the item at all: witness class >= 30)

Declarations and parameters spelled `unsigned char` whose byte is read through a
`char *` (what the option moves):

| binary | funcs changed | `unsigned char` decls -> `char` | signature lines |
|---|---|---|---|
| fmt -O0 | 1 / 191 | 0 | 1 |
| ls -O0 | 2 / 589 | 4 | 2 |
| sort -O0 | 4 / 479 | 12 | 2 |
| grep -O0 | 2 / 642 | 1 | 1 |
| fmt -O2 | 2 / 151 | 1 (+ a split, see §4) | 1 |
| ls -O2 | 4 / 404 | 9 | 3 |
| sort -O2 | 5 / 343 | 12 | 4 |
| grep -O2 | 1 / 449 | 0 | 1 |
| **total** | **21 / 3,248** | **39** | **15** |

**Witness class = 54 (39 declarations + 15 signatures) >= 30**, so the option was
finished. For scale, the off arm spells `unsigned char` 492 times in declarations
and parameters over the same binaries (197 scalar and 162 pointer locals, 73
scalar and 60 pointer parameters); the other ~438 are bytes the program really
uses as numbers (masks, unsigned compares, table bytes) or that no `char *`
reaches, and the option leaves them alone.

## 3. The first design was wrong, and why

The pre-census WIP overrode the edge: on the pointer-to-byte `LOAD` edge, install
`char` although the lattice ranks it lower, and refuse the reverse `uint1 *` push.
On the 8 binaries it produced byte-identical output to the final design, but the
stage fixture's `ishigh` (`movzx eax,[rdi]; mov [rsi],eax; cmp al,0xe9`) showed
the flaw:

```c
char v1;                       // edge override: WRONG C
v1 = *s;
return (int4)(v1 == 0xe9);     // a char is never 0xe9 in C
```

The constant had already been typed `uint1` by the byte before the pointer's
`char` arrived, and the override does not revisit it. The final design never
overrides an edge: it records the byte, rebuilds the local types with the byte
seeded `char`, and propagates again by the unchanged rules, so the constant takes
`char` with the byte (`v1 == '\xe9'`, true for 0xe9).

## 3b. A guard: the re-propagation may only move `uint1` to `char`

A fresh propagation from different seeds can tip a type contest the rule has no
stake in. bash -O2 `param_expand` (0x757a0) showed it: a string pointer that
shares a join with the `int *quoted_dollar_at_p` parameter had come out
`unsigned char *` (the byte's vote won the join); with the byte re-seeded `char`,
`int *` won instead, and the string pointer printed as `int4 *v14` with
`(char)*v14` reads. The second propagation is now compared with the first,
Varnode by Varnode, and kept only if every type is unchanged or is the same type
with `uint1` read as `char` through any depth of pointers and arrays
(`kuna_charbyte::keep_or_restore`); otherwise the first propagation's types are
restored and the function prints exactly as with the option off. On the 8
binaries the guard never fires (their output is byte-identical with and without
it); on bash -O2 it restores `param_expand`, leaving 12 changed functions.

## 4. Whole-corpus hunk classification (8 binaries, both arms, one build)

Every changed line of the 21 functions falls into one of:

| class | example |
|---|---|
| byte / pointer / parameter / return type `unsigned char` -> `char` | `unsigned char *a1` -> `char *a1` |
| a cast back to `char` dropped | `mbrtowc(a0,(char *)a1,...)` -> `mbrtowc(a0,a1,...)`; `(int)(char)*v1` -> `(int)*v1` |
| zero-extension of a `char` printed as the cast it is | `(unsigned int)*a1` -> `(unsigned int)(unsigned char)*a1`; ctype index `(unsigned long)(unsigned char)v2 * 2` |
| character constants as literals | `v3 != 9` -> `v3 != '\t'`; `*v4 == 0x2d` -> `*v4 == '-'`; `*v5 = 0` -> `*v5 = '\0'` |
| an unsigned suffix on byte arithmetic | `(unsigned char)(v2 + 0x9f)` -> `(unsigned char)(v1 + 0x9fU)` |
| merge / renumbering (3 functions) | see below |

Merge changes, from §6 joining only variables of one type:

* fmt -O2 `get_line` @0x3420: `unsigned char *v10` held both the FILE read pointer
  and a pointer into the line buffer; they are now `unsigned char *v1` and
  `char *v11` (10 -> 11 declarations).
* ls -O2 @0x7920: `unsigned char v2` held both the string byte and an unrelated flag
  stored to `*a5`; they are now `char v1` and `unsigned char v3` (17 -> 18).
* ls -O2 @0x6d40: two `char *` locals with disjoint lifetimes join (18 -> 17).

Parameters 65 -> 65 and exported JSON `variables[]` 286 -> 286 over the 21
functions: no argument or variable is fabricated or lost. No hunk falls outside
these classes.

**bash -O2 (1.3 MB, 2,538 functions), beyond the 8.** 12 functions change, in
the same classes; declarations 227 -> 224 (three `unsigned char` bytes join an
existing `char` variable of the same storage), parameters 38 -> 38, JSON
variables 216 -> 216. One more class appears there: a constant added to the byte
takes `char` with it and prints the way kuna already prints arithmetic on any
`char` (`v = v - 0x30` becomes `v += '\xd0'`; 6 lines, where the off arm already
has 14 such lines). It is value-preserving (oracle below) but no easier to read.

**Comparison oracle.** `oracle.c` (this directory) evaluates every distinct
value-bearing hunk form - all 33 changed comparison lines of the 8 binaries,
every changed widening, narrowing and store, and the bash -O2 arithmetic and
comparison forms - with the off arm's types and with the on arm's
types, for all 256 byte values (256 x 256 for the byte-vs-global and
byte-vs-byte comparisons), under x86-64 gcc where plain `char` is signed. §5
extends it to the 34-binary census and to the unchanged lines: **149,248
checks, 0 differ**, and three controls are caught (a `char` compared with
`0xe9`, the edge-override bug; a `char` passed to a variadic call; a `char`
switch variable against `case 0x80`). Rebuild with
`gcc -O0 oracle.c -o oracle && ./oracle`.

## 5. Review round 2: what a changed line cannot show

The first audit read only the lines the option changed. A retyped byte can
also change the meaning of a line whose text stays the same, wherever C
converts the byte with no cast to show it. The printer's cast machinery
covers every op that cares about signedness (comparisons, extensions, shifts,
divisions); it does not cover three places:

* **A call argument or a returned value.** kuna folds the `movzx` of a byte
  passed to a call into a one-byte argument. `logit("%d",c)` prints the same
  in both arms, but with `char c` C passes -128 for the byte 0x80 where the
  binary passes 128. The reviewer's gcc -O2 recompile gave 128 of 256 values
  wrong for the recorded byte (`c13`) and for a sibling `a0[1]` read through
  the retyped pointer (`c10`); on the corpus, mirai -O2 0xce90 passed `*v28` to
  its printf this way.
* **A switch.** The switch header prints the byte itself and its labels as
  numbers. gcc -O0 `switch ((unsigned char)s[0]) { case 0x80: ... }` printed
  `switch(*a0) { case 0x80:` over a `char *a0`, which never takes that case.
  (This one was found by the audit below, not by the review.)
* **One-byte arithmetic that reaches either of those** (`f(c + 1)`): C does
  the arithmetic on the promoted value, so the sign of `c` reaches the call.

`kuna_charbyte::read_without_cast` walks each byte forward through copies,
joins and one-byte arithmetic and reports a `CALL`, `CALLIND`, `CALLOTHER`,
`RETURN` or `BRANCHIND` reader. Such a byte is never recorded, and if the
second propagation retypes one anyway (a sibling read through the retyped
pointer), `keep_or_restore` restores the first propagation for the function.

**Counters.** A byte whose own one-byte sum or difference is stored back
through the address it was loaded from (`c = *p; *p = c + 1;`) met a `char *`
only through that sum's default signed vote (`INT_ADD` outputs are
`get_base(1, TYPE_INT)` = `char`), so a genuine `unsigned char *` counter
became `char *` with `*p = c + '\x01'` (gcc -O0 `inc`/`dec`/`bump`, mirai -O2
0xce90 `resolv_lookup`'s `uint8_t` counter). `kuna_charbyte::is_counter`
excludes it the same way. The first version matched any stored sum and also
excluded mirai's printf copies, whose `*buf = d + '0'` is character arithmetic;
the rule now requires the store to go back through the load's own address
(same Varnode up to COPYs, or the same op over the same inputs).

**Census after the fix** (`kuna decompile-all --json`, both arms of one build
on f1ec42a7 + this branch, 34 binaries, 13,747 functions: the 8 above, the
reviewer's 26 - bash/tar/ssh/libedit/dash/diff/cut/tr/expr/od/xmlwf/gzip/
bzip2/mirai at -O2 and the coreutils/cronie/dash/diff/gzip/xmlwf/bzip2/mirai
set at -O0 - and the libopencm3 ARM image):

| | before the fix | after |
|---|---|---|
| functions changed, 8 binaries | 21 | 21 (byte-identical output) |
| functions changed, 34 binaries | 67 | 66 |
| mirai -O2 0xce90 (counter + call argument) | changed | unchanged |
| mirai -O2 printf copies (10) | changed | changed (same hunks) |
| tar -O2 0x41370 (a retyped byte reaches the switch) | changed | changed, the switch byte stays `uint1` |
| parameters / JSON `variables[]` | 246 / 1,210 both arms | 246 / 1,210 both arms |

Every type that moves in `variables[]` moves only from `unsigned char` to
`char` (scalars, pointers, `[24]` and `[125]` arrays); no size moves and no
variable or argument is added or removed.

**Unchanged-line audit.** `audit.py` (kept with the census, not in the tree)
lists, for every changed function, every line - changed or not - that
mentions an identifier whose declared type moved from `unsigned char` to
`char`, and classifies each occurrence by context. Over the 66 functions:
no retyped scalar is a direct call argument, returned value or switch
variable (the three `CALLARG` hits are tar's `char v8`, which is `char` in
both arms and passes the `char a3` parameter to the recursive call); the
unchanged reads are byte copies and stores (`v = *p`, `*p = v`, `a0[1] = v`),
truth tests (`if (!v)`, `while (*p)`), pointer copies, comparisons and
arithmetic, and a retyped pointer passed as a pointer. Every unchanged
comparison either compares two retyped bytes with each other or a byte with a
character literal. Each value-bearing form is in `oracle.c`.

**Readability cost the review measured.** ssh -O2 0x52dd0 prints
`(uint4)(uint1)(char)v2[1]` and a walker in gcc -O2 splits one declaration
into `unsigned char *v5; v5 = (unsigned char *)&a0[1];`. Both are the cast
tail printing a zero-extension of a `char`; neither changes a value.
