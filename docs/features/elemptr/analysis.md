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
What remains is the four `(int)` arms of the `?:` (a `castimplied` question: C's
usual arithmetic conversions already make `cond ? char : 0` an `int`), the
`(unsigned char)` index conversions the value needs, and the truncating stores.

## 2. The rule

See `plan.md` and `docs/spec/05-types.md` ("A pointer used only as an array"). A
pointer-width parameter, an open call return, an unnamed global or a data-section
constant whose every access is one width W through an index the program computes
(scaled by W) gets a `T *` vote; `T` is the integer of width W with the sign its
extensions, orderings, stored values or a declared pointee say, or, for
pointer-width elements, a pointer type something names. Globals are typed only
where every function of a batch agrees.

## 3. Where the casts went, and the functions with more

castbench (45 binaries x O0/O2/O2-noinline, 4,815 functions kuna, IDA and main
share): 37,477 -> 35,327 casts. By shape: `(char *)(...)` -429, `(long)v` -321,
`(unsigned char *)(...)` -312, `(unsigned long)v` -254, `(int *)(...)` -189,
`(unsigned short *)(...)` -115, `(char **)(...)` -95. Gained: `(char *)0x...` +94,
a table `globalref` cannot name (the function also writes its first byte by name,
a symbol covers it, or it is read at two pointee types), which prints
`((char *)0x5c000)[i]` with one cast where main printed two; and `(unsigned int)`
on a subscript +48, the same conversion as before now applied to a subscript.

Every one of the 17 functions with more casts (+22 in all) was read:

- A callee's parameter is now a declared `char *` / `char **` (correctly: `tar`'s
  `to_chars(char *where)`, `argmatch`'s arglist), so an integer-typed argument in
  the caller prints its conversion: `sub_12890(v13,(char *)(v8 + 0x109))` (tar
  O2-noinline 0x132d0, O0 0x1381d, O0 0x15a63, O0 0x135e3, diff3 O2-noinline
  0x4ae0, ls O2 0xc0e0).
- A constant address the caller used at one pointee type is now also passed where
  a callee declares another (`short *` against `unsigned short *` in gzip's
  `make_table`), so `globalref` declines two types and both uses keep a cast (gzip
  O2 0xb700, O2-noinline 0xbb40, 0x67e0, O2 0x63e0).
- A call's result kept in a `char *` stack slot where the callee's recovered return
  type is `long` (`v6 = (char *)sub_8d30(...)`, diff O0 0x8710 / 0x818f), and a
  buffer typed `unsigned char *` from its zero-extended reads that is also passed
  to `strftime` (find O0 0x11c89, O2-noinline 0xeb50).
- A global array of pointers typed `unsigned char **` that is also stepped by a
  byte offset (diff O2 0x8590, O2-noinline 0x8320), and grep O2-noinline 0xbfa0,
  whose record field is now a typed buffer.

## 4. Whole-corpus hunks

`structural.py` checks every changed function (1,196 over the 45 binaries) for the
same callees, string literals, control keywords and program data (a constant
address may become the `dat_<addr>` it names), with statement counts within four.
27 functions break a check; each was read and falls in a documented class:

- constant named (428 functions): `*(int *)(x * 4 + 0x20980)` -> `dat_20980[x]`,
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

`hunks.py` classifies the line-level hunks (declarations, subscripts, casts, reads
printed at their uses, the loop); its unclassified remainder is the same classes
with renamed variables, which `structural.py` covers function by function.

## 5. Known limits

- A single-function `decompile`, a sharded `--jobs` worker and the streaming
  export have no batch ledger: they type a global on the function's own evidence.
- A table read at two pointee types in one function keeps its casts
  (`globalref`'s two-type refusal).
- The sign of an element with no evidence is plain (`short`), which can disagree
  with a callee's `unsigned short *` and cost the caller's name (section 3).
- A pre-existing return-recovery gap, not this option: gcc -O2 `char *f(n) { o =
  malloc(n + 1); ...; return o; }` decompiles as `void f` (rax never copied), in
  every mode.
