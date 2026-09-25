# elemptr — a pointer used only as an array of one element type

## 1. The problem

A textbook base64 decoder (`decoding_table[(unsigned char)data[i]]`, a table built
from a constant alphabet into a `malloc`'d global, a `malloc`'d output buffer)
decompiles on main `c960fb18d` as (gcc -O0, stripped, fixture
`decompiler/crates/kuna-analysis/tests/fixtures/elemptr_x86_64.c` has the same
shapes):

```c
void * sub_123f(long a0,unsigned long a1,unsigned long *a2)
  if (*(char *)(a0 + (a1 - 1)) == '=')
  v2 = (*(char *)(a0 + v7) != '=') ? (int)*(char *)((unsigned long)*(unsigned char *)(a0 + v7) + dat_4070) : 0;
  *(char *)((long)v6 + (long)v8) = (char)(v5 >> 0x10);
void sub_11e9(void)
  *(char *)((unsigned long)*(unsigned char *)((long)v1 + 0x4020) + (long)dat_4070) = (char)v1;
```

Every access is an integer sum behind casts because nothing declares the three
arrays: `TypeOpIntAdd` votes an integer for both operands of `p + i` and refuses to
carry the loaded element's pointer back over the add (`typeop.cc:1217`), so the
input string is `long`, the buffer `malloc` returned is `void *`, the global that
holds the table is `long` where it is read, and the constant alphabet has no
declaration at all. IDA prints the same shapes as `*((char *)ptr + *(unsigned
__int8 *)(a1 + v9))`, `v26[v12] = BYTE2(v19)` (`_BYTE *v26`) and
`*((_BYTE *)ptr + byte_5020[i]) = i`.

With `elemptr` (default on):

```c
char * sub_123f(char *a0,unsigned long a1,unsigned long *a2)
  if (a0[a1 - 1] == '=')
  v2 = (a0[v7] != '=') ? (int)dat_4070[(unsigned char)a0[v7]] : 0;
  v6[v8] = (char)(v5 >> 0x10);
void sub_11e9(void)
  dat_4070[dat_4020[v1]] = (char)v1;
```

The decoder goes from 38 casts to 15 and the table build from 6 to 1 (castcount).
What remains is the four `(int)` arms of the `?:`, the `(unsigned char)` index
conversions the value needs, and the truncating stores. `castternary` leaves out
such an `(int)` when it knows the arm's C type; with `elemptr` on, `castimplied`
reads a subscript's element from its declared base (a parameter `char *a0`), but
the decoder's table is an unnamed global, which no declaration in the function's
text types, so its arms keep the cast.

## 2. The rule

See `plan.md` and `docs/spec/05-types.md` ("A pointer used only as an array"). A
pointer-width parameter, an open call return, an unnamed global or a data-section
constant whose every access is one width W through an index the program computes
(scaled by W) gets a `T *` vote; `T` is the integer of width W, or, for
pointer-width elements, a pointer type something names. The sign of `T` comes only
from evidence (section 7): the function's return type where it returns the
element, then extensions, orderings, shifts and divisions of a loaded element,
then a character compare (plain `char`), then the type the load has without the
rule; a wider element nothing votes for is unsigned. Globals and constant tables
are typed only where every function of a batch agrees on the element, sign
included.

## 3. Where the casts went, and the functions with more

castbench (45 binaries x O0/O2/O2-noinline, 4,815 functions kuna, IDA and main
share), on main `850e8c692`, which already has `castindex` and `castternary`:
35,588 -> 33,588 casts (0.941x -> 0.888x IDA; 188.2 -> 177.7 per 1,000 lines, 29.9 ->
28.3 per 100 statements), 339 functions with fewer (-2,019) and 16 with more (+19).
The first version of this PR reached 33,472; the difference is casts it removed by
declaring a sign it had no evidence for, or a table two functions read at two
signs (section 7). By shape: `(char *)(...)` -382, `(unsigned char *)(...)` -280,
`(unsigned long)v` -248, `(long)v` -244, `(int *)(...)` -168, `(unsigned short
*)(...)` -115, `(char **)(...)` -91. Gained: `(char *)0x...`,
a table `globalref` cannot name (the function also writes its first byte by name,
a symbol covers it, or it is read at two pointee types), which prints
`((char *)0x5c000)[i]` with one cast where main printed two; and `(unsigned int)`
on a subscript +48, the same conversion as before now applied to a subscript.

Every one of the 16 functions with more casts (+19 in all) was read:

- A callee's parameter is now a declared `char *` / `char **` (correctly: `tar`'s
  `to_chars(char *where)`, `argmatch`'s arglist), so an integer-typed argument in
  the caller prints its conversion: `sub_12890(v13,(char *)(v8 + 0x109))` (tar
  O2-noinline 0x132d0, O0 0x1381d, O0 0x15a63, O0 0x135e3, diff3 O2-noinline
  0x4ae0, ls O2 0xc0e0).
- A constant address the caller used at one pointee type is now also passed where
  a callee declares another (`short *` against `unsigned short *` in gzip's
  `make_table`), so `globalref` declines two types and both uses keep a cast (gzip
  O2-noinline 0x67e0, O2 0x63e0; gzip O2 0xb700 is named again, since an undefined
  word and the unsigned word of its size are one object to `globalref` now).
- A call's result kept in a `char *` stack slot where the callee's recovered return
  type is `long` (`v6 = (char *)sub_8d30(...)`, diff O0 0x8710 / 0x818f), and a
  buffer typed `unsigned char *` from its zero-extended reads that is also passed
  to `strftime` (find O0 0x11c89, O2-noinline 0xeb50).
- A global array of pointers typed `unsigned char **` that is also stepped by a
  byte offset (diff O2 0x8590, O2-noinline 0x8320), and grep O2-noinline 0xbfa0,
  whose record field is now a typed buffer.

## 4. Whole-corpus hunks

`structural.py` checks every changed function (1,077 over the 45 binaries) for the
same callees, string literals, control keywords and program data (a constant
address may become the `dat_<addr>` it names), with statement counts within four.
25 functions break a check; each was read and falls in a documented class:

- constant named (425 functions): `*(int *)(x * 4 + 0x20980)` -> `dat_20980[x]`,
  the name `globalref` gives the array (IDA: `dword_205E0[...]` at the same
  address). The check flags the ones whose constant was printed in decimal or
  biased by a literal offset.
- caller constant unnamed: section 3, second bullet.
- register variable split: a register that held several values of different types
  (find O0 0x160f6, 0xdd20) splits once the element types differ, so its copy
  statements disappear (93 -> 54 statements) and the return type follows the
  returned value.
- tail duplicated: find O0 0x173d3's six `goto`s to a no-return error tail become
  six copies of the tail (`returndup`), because the tail's statement is now under
  the duplication budget (`a1[*a2][(long)v2 + -1]` instead of
  `*(char *)((long)v2 + -1 + *(long *)(a1 + (long)*a2 * 8))`).
- stores split / merged: tar O2-noinline 0x12990 / 0x12cb0, a 512-byte header a
  callee now types `char *`: an 8-byte zero store prints as eight byte stores and
  the magic `"ustar"` stores as `builtin_strncpy` (the same bytes).
- read printed at its uses: a byte or word read used twice is now implied at both
  uses (upstream's expression-duplication bound counts a subscript's terms, fewer
  than the integer sum's); no store intervenes in any case read (diff O2-noinline
  0xa650, gzip O0 0xc6f9, O0 0x4677).
- while to for: one loop absorbs its iterator (regionstructure-loop's stage test
  has the same shape).

A second, disjoint sweep over the eleven binaries the review chose (coreutils
`cksum` and `od` at O0, `tr`, `ptx` and `shuf` at O2, zlib, `xmlwf`, `dash`,
`mirai`, `libbsd`, `init`) changes 113 functions, 11,384 -> 11,030 casts, 91 with
fewer and 6 with more, each read: a constant used at two pointee types (`tr` main's
translation table and buffer, `cksum` 0x461a), a call result kept in a `char *`
slot whose callee returns `long` (`mirai` 0x7efb, 0xa015), a `char **` parameter
whose elements a callee with an unrecovered return type fills (`init` 0x6ad0), and
`strlen((char *)v)` on a buffer typed `unsigned char *` from its zero-extended
reads (`ptx` 0x6c40).

`hunks.py` classifies the line-level hunks (declarations, subscripts, casts, reads
printed at their uses, the loop); its unclassified remainder is the same classes
with renamed variables, which `structural.py` covers function by function.

## 5. Speed

Interleaved min-of-15 whole-binary `decompile-all --json`, `--option elemptr off`
against the default on one build: fmt +0.10%, ls -0.02%, sort +1.19%, bash
+1.37%. The first measurement read bash +12.86%: the batch redo decompiled 23
functions again (14.7 s, the parser's largest among them). Deciding each global
once per pass before classifying it, and blocking a disputed global for every
function decompiled after the first disagreement, brings that to 17 functions and
3.6 s.

## 6. Known limits

- A single-function `decompile`, a sharded `--jobs` worker and the streaming
  export have no batch ledger: they type a global on the function's own evidence.
- A table read at two pointee types in one function keeps its casts
  (`globalref`'s two-type refusal).
- A sharded `--jobs` export cannot adopt a batch's sign: where two workers type a
  table at two elements the header declares neither (a compile error, never a
  silent value change).
- A pre-existing return-recovery gap, not this option: gcc -O2 `char *f(n) { o =
  malloc(n + 1); ...; return o; }` decompiles as `void f` (rax never copied), in
  every mode.

## 7. Value preservation

A type this rule commits is not only spelling. Three ways it could change what the
printed C computes were found by compiled round trips and closed:

- **A defaulted sign reaching a return.** The first version declared a 2- or
  4-byte element with no sign evidence signed (`short`, `int`). A function that
  returns the element returns it at that type, so `unsigned long f(unsigned i) {
  return wtab[i & 63]; }` (gcc zero-extends in the callee) printed as `short f`,
  and its caller's `f(i) + 0x10000` sign-extended: 98304 became 32768, and
  `itab[1] + 1` became 18446744071562067969. A wider element with no evidence is
  now unsigned, which is how the undefined word main prints reads; the element's
  own readers' fold (the type the load has without the rule) comes before that
  default; and a function that returns the element never lets a batch change its
  sign.
- **One table, two signs, one header.** A constant table one function zero-extends
  and another sign-extends was declared once (`extern char dat_339e0[];`, grep O0
  0xfc0e reads it with `movzbl`), so the other body computed with the wrong
  extension; an `unsigned int` table shifted in one function and declared `int`
  from another made `>> 1` arithmetic (3221225472 became 1073741824). The ledger
  now covers constant tables beside globals and compares the whole element, sign
  included: two signs evidence chose block the table everywhere (each body keeps
  its own `*(T *)` cast), and a function whose sign was only the default is
  decided again at the evidenced sign, so it prints consistently with the one
  declaration. The header declares no array two functions index at different
  elements (a sharded `--jobs` worker cannot consult the ledger).
- **Index and base swapped.** `char f(char *p, unsigned long i) { if ((unsigned
  long)p & 7) return 0; return p[i]; }` declared `i` the `char *` because the mask
  made `p` look like a number; a mask no longer counts as an index's use. A value
  used as the index of another base (a `PTRADD` index, an add to a constant already
  typed a pointer, a second index of another scale) refuses, which keeps zlib's
  `deflate_state` a record, and a global copied into a register the function
  returns another value in refuses (`libbsd`'s `user_from_uid`).

The round trip in `decompile_all_cli.rs` now also reads tables whose elements have
the top bit set: `unsigned short` and `unsigned int` elements returned to callers
that widen them, one shifted, one only compared, and a byte table read at two
signs. The review's own fixtures (`cy.c`, `cx.c`), exported, compiled with gcc and
clang and run, print with the option on exactly what they print with it off on
gcc -O0, clang -O0, gcc -O2 and clang -O2 builds (where the off arm itself differs
from the binary, a pre-existing gap in main, the on arm differs identically).

