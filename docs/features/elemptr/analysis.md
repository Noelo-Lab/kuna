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
included, and a function that holds a global untyped agrees only when nothing it
prints depends on the pointee: `gp++` prints `dat_4018 += 4`, which a header's
`int *dat_4018` would make four elements, so that function refuses the global. A
function that names a table's first element directly (`dat_4040 + 1`) blocks every
function that named the table an array. With the option on, a narrow load
through a pointer whose pointee was inferred rather than declared stays narrow
(`*(unsigned short *)&a0[1]`, not upstream's widened `(unsigned short)a0[1]`).

## 3. Where the casts went, and the functions with more

castbench (45 binaries x O0/O2/O2-noinline, 4,815 functions kuna, IDA and main
share), against a build of main `f96e80805` (which already has `castindex`,
`castternary`, `callrettype`, `castobject` and `castwiden`; the `off` arm is
byte-identical to it on all 45 binaries): 33,289 -> 32,073 casts (0.880x -> 0.848x
IDA; 176.0 -> 169.6 per 1,000 lines, 28.0 -> 27.0 per 100 statements; O0 0.871x ->
0.815x, O2 0.927x -> 0.907x, O2-noinline 0.838x -> 0.816x), 301 functions with
fewer (-1,242) and 18 with more (+26). The same engine measured 34,829 -> 33,241
on `b273c2259` and 35,588 -> 34,000 on `0096e984d`: `castwiden` and `castobject`
now drop some of the casts this rule also removes.
On `0096e984d` the engine before the batch agreement reached 33,505. The difference is casts it
removed by typing a global another function steps by bytes or uses as an integer,
or a table another function names the first element of (section 7): those
functions now print main's form, where the one header declaration computes what
the binary does in every function.

Every one of the 18 functions with more casts (+26 in all) was read:

- A callee's parameter is now a declared `char *` / `char **` (correctly: `tar`'s
  `to_chars(char *where)`, `argmatch`'s arglist), so an integer-typed argument in
  the caller prints its conversion: `sub_12890(v13,(char *)(v8 + 0x109))` (tar
  O2-noinline 0x132d0; O0 0x1381d, 0x135e3, 0x15a63; diff3 O2-noinline 0x4ae0;
  ls O2 0xc0e0).
- A constant address a caller used at one pointee type is now also passed where
  a callee declares another, so `globalref` declines two types and both uses keep
  a cast (gzip O2-noinline 0x67e0, O2 0x63e0, `(unsigned long *)0x19040`).
- A call's result kept in a `char *` or `unsigned char *` variable where the
  callee's recovered return type is `long` (`v2 = (char *)sub_4ac50(dat_84a40)`,
  tar O2-noinline 0xf760 +3, 0x1a320, 0x13140; O2 0x1ca20; find O0 0xb22b):
  `callrettype` now gives the call that stated integer type, and this rule types
  the value from its uses. A buffer typed `unsigned char *` from its
  zero-extended reads that is also passed to `strftime` (find O0 0x11c89,
  O2-noinline 0xeb50); grep O2-noinline 0xbfa0, whose record field is now a typed
  buffer.
- The narrow load: cp O2 0x76f0 and tar O2-noinline 0x16890 read a byte or a
  4-byte field of an inferred 8-byte element. Upstream widened the load and
  truncated it, `(char)v43[6]`; it now reads what the binary reads,
  `*(char *)&v43[6]`, one cast either way. cp's two extra casts are a masked
  byte that was a variable and is now printed at both its uses; tar's one is the
  counter reading `if (v3)` before an unbraced statement as a cast, where main
  printed `if ((int)v3)` of an 8-byte read.

## 4. Whole-corpus hunks

`structural.py` checks every changed function (1,255 over the 45 binaries; 1,084
before the narrow load, which respells a truncated wide read in functions the
rule's types never reach) for the same callees, string literals, control keywords
and program data (a constant address may become the `dat_<addr>` it names), with
statement counts within four. 19 functions break a check, all of them among the
26 an earlier engine flagged; each was read and falls in a documented class:

- constant named (429 functions): `*(int *)(x * 4 + 0x20980)` -> `dat_20980[x]`,
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

- a caller constant printed as its string: cmp O2-noinline 0x2940 passes
  `0x9682` to a callee whose second parameter is now `char *`, so it prints
  `sub_4520("Torbjorn Granlund","Torbjörn Granlund")`, the `proper_name_utf8` call
  the source makes.

A second, disjoint sweep over 32 more binaries (bzip2 and `bzip2recover`, libacl,
`crontab`, coreutils `cut`, `factor`, `printf`, `dircolors`, `numfmt`, `cksum`,
`od`, `tr`, `ptx` and `shuf`, `update-passwd`, kmod, libedit, zlib, `xmlwf`,
`dash`, `mirai`, libbsd, `init`, `dpkg-query`, `dpkg-divert`, `useradd`,
`usermod`, `newusers`, `gnutls-cli`, `gnutls-cli-debug`, libselinux, `rtmon`)
changes 556 functions on main `f96e80805`, 34,230 -> 33,337 casts, 260 with fewer
and 25 with more (+48); the `off` arm is byte-identical to a build of main on all
32. On `0096e984d`, against the engine before the batch agreement, 111 functions
changed: the globals and tables it typed while another function stepped them by
bytes or named their first element now print main's form (dash `dat_26a40`,
zlib's `dat_14c40`), and the narrow load reads what the binary reads. Each
function with more than main was read. Four kmod functions and `init` 0x87b0 are
the narrow load: a byte read through an inferred `short *` was widened and
truncated, `(int)(char)*v11`, and now reads a byte, `(int)*(char *)v11`, printed
at both its uses; `init`'s `testb $2` printed `*v7 & 2`, a 4-byte read, and now
prints `*(unsigned char *)v7 & 2`. `numfmt` 0x27a0, `dpkg-divert` 0x6100 and
`crontab` 0x606b keep a call's result in a variable typed the other way from the
return `callrettype` states (`(unsigned long)sub_8d50(...)`, `(char
**)sub_9417()`); `ptx` 0x53b0 prints a difference of two table reads at both its
uses. The
rest: a constant used at two pointee types (`tr` main's
translation table and buffer, `cksum` 0x461a), a call result kept in a `char *`
slot whose callee returns `long` (`mirai` 0x7efb, 0xa015), a `char **` parameter
whose elements a callee with an unrecovered return type fills (`init` 0x6ad0), and
`strlen((char *)v)` or `strncmp((char *)v, ...)` on a buffer typed `unsigned char
*` from its zero-extended reads (`ptx` 0x6c40, kmod 0xfac0), and a caller whose
argument a callee now declares `int *` or `char *` (`bzip2` 0xb9c1, kmod 0x15930).

`hunks.py` classifies the line-level hunks (declarations, subscripts, casts, reads
printed at their uses, the loop); its unclassified remainder is the same classes
with renamed variables, which `structural.py` covers function by function.

## 5. Speed

Interleaved min-of-15 whole-binary `decompile-all --json`, `--option elemptr off`
against the default on one build, on main `f96e80805`: fmt +0.14%, ls +0.17%, sort
+0.21%, bash +0.39% (on `b273c2259` the same engine read fmt +0.05%, ls +0.11%,
sort +0.32%, bash +1.65%; on `0096e984d` the engine before the batch agreement read fmt
+0.42%, ls +0.39%, sort +0.32%, bash -0.30%). A first cut of the agreement read
bash +17.85%: it filed a global whose fold already names a pointee (a libc
`FILE *`) as a type, so one function refusing that global decided 135 of bash's
functions twice; a claim is now evidence and never decided again (31 functions,
the count before the agreement). An engine that decided a defaulted table sign only after the batch read
sort +57%: its `main` was decompiled again for one table. A function decompiled
after evidence has settled a sign now reads it at once (`Ledger::settled`). The first measurement read bash +12.86%: the batch redo decompiled 23
functions again (14.7 s, the parser's largest among them). Deciding each global
once per pass before classifying it, and blocking a disputed global for every
function decompiled after the first disagreement, brings that to 17 functions and
3.6 s.

## 6. Known limits

- A single-function `decompile`, a sharded `--jobs` worker and the streaming
  export have no batch ledger: they type a global on the function's own evidence.
- `decompile` and `decompile-all` print no declaration for a global or table
  (neither does main for any name `globalref` gives): only `decompile-project`'s
  header states the element a subscript reads.
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
printed C computes were found by compiled round trips and closed, and two ways
it declared a wrong type without changing a value:

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
- **A length taken for the base.** `put(char *buf, size_t len, const char *s)`
  compares `(size_t)(b - buf)` against `len`. The difference lowers to
  `b + buf * -1`, and the multiply by -1 made `buf` look like a number, so the
  first type pass declared `len` the `char *` base of `buf + len` (gcc and clang,
  -O0 to -O2; libedit's `keymacro__decode_str`, whose callers then passed
  `(char *)0x400`). The right-hand side of a difference of two pointers (the sum
  compared or returned, not dereferenced) is no longer a number use, an add
  whose other operand subtracts one more pointer than it adds (`b + (-buf - n)`)
  is a difference, not an element's address, and an add operand dereferenced
  through a copy or phi (`b = buf; *b++`) is the base. An operand that is a whole
  difference already stays an index: `d[&v[-7] - a0] = 0` keeps `d` `char *`
  (charptr's `dropsuffix`). As a
  net under both, a later pass that finds a parameter or call return an earlier
  pass typed indexing another base blocks it and restarts the function, since
  the `PTRADD` the earlier pass built keeps its type otherwise (it fires on
  grep O0 `print_line_tail`, which then prints what main prints). The values
  round-tripped before; the declaration was wrong.
- **A counter re-signed by its table.** `fmap[i] = i` with `unsigned *fmap` and
  `int i` propagated the element's `unsigned int` onto the counter, which then
  needed `(int)` at every signed compare and index (bzip2 O0 `fallbackSort`:
  +45 casts). A value stored through a pointer the rule typed now keeps its own
  sign when the two differ only in sign (C converts it bit for bit on the
  store), and a stored value's sign only breaks a tie for a table nothing reads
  in that function, so a write-only function no longer blocks the sign its
  reader established (gzip O0's `huft` tables).
- **A global stepped by bytes in another function.** `int *gp` indexed in
  `init` and `rd`, and stepped in `bump` (`gp++`): `bump` holds the global
  untyped and says nothing about its element, so the batch typed it `int *` and
  `bump` printed `dat_4018 += 4`, which against that one declaration steps four
  elements (the compiled bodies computed 5997 where the binary computes 2997; so
  did a 2-byte and a 6-byte advance of an `unsigned short *` and a `long`). A
  table read by subscript in one function and by its first element's name in
  another (`return dat_4040 + 1;` for `tbl[0] + 1`) did the same the other way:
  the array's declaration makes the scalar the array. A function that holds a
  global untyped now refuses it unless every use of it and its copies prints the
  same whatever the declared pointee (copied, dereferenced at its own address,
  stored, passed, returned, compared); a global whose vote names a pointee files
  that pointee; and a function that names a table's first element blocks every
  function that named it an array, keeping its own `(&dat_4040)[i]`.
- **An element address in the returned register.** coreutils -O0 `expand`'s
  `get_next_tab_column` indexes the global `tab_list` in the `rax` it returns
  `tab_list[i]` in. Typed, the address and the returned values merged into one
  `unsigned long *` variable, the function's return type followed, and on a
  main with `callrettype` its callers declared their column counters
  `unsigned long *` (the one function the 444-slice sweep scored worse, .714 ->
  .429). A held global whose element is loaded through an address computed in
  the register, at the width, the function returns that element in is not typed
  there (`address_in_returned_register`, beside the copy-in-return-register
  guard); no function of the cast corpus changes. The fixture's `w_nexttab` is
  that function: the stage test and the compiled round trip pin its return. A first cut that declined
  every address sharing the returned register untyped a base64 decoder's table,
  whose function returns its output buffer in that register.
- **A narrow read made wide.** `c_hdr(const unsigned *p)` reads the 2-byte field
  at `p + 4` beside a call `c_sum4(p, 1)` whose parameter this rule declares
  `unsigned int *`. The caller's `p` takes that type, and upstream's
  `RuleExpandLoad` then widens the 2-byte load to the 4-byte element and
  truncates it, `(unsigned long)(unsigned short)a0[1]`: the same value, but a
  4-byte read, which faults when the object ends a page (main, which never typed
  `p`, printed the 2-byte read). With the option on, a pointer whose pointee was
  inferred rather than declared keeps the load's own width,
  `*(unsigned short *)&a0[1]`; a type-locked pointer keeps upstream's form, and
  no datatest moves.

The round trip in `decompile_all_cli.rs` now also reads tables whose elements have
the top bit set: `unsigned short` and `unsigned int` elements returned to callers
that widen them, one shifted, one only compared, and a byte table read at two
signs. Two further fixtures of the same shapes, exported, compiled with gcc and
clang and run, print with the option on exactly what they print with it off on
gcc -O0, clang -O0, gcc -O2 and clang -O2 builds (where the off arm itself differs
from the binary, a pre-existing gap in main, the on arm differs identically).
It also reads a global `int *` two functions index and two others step by bytes,
a table one function indexes and two others name the first element of, and a
2-byte field at the very end of a readable page through a pointer a callee reads
as `unsigned int *`; a global the header declines as read at two types is
compiled at the pointer that header comment lists, so a disagreement cannot hide
behind a guessed declaration. Four more fixtures of those shapes (globals stepped
by 4, 2 and `k` bytes, tables read by their first element's name, narrow fields
through a wider callee pointee, and the signed, unsigned, negative and
pointer-difference indexes of the earlier rounds) print with the option on what
the option off prints, field by field, on gcc and clang at -O0, -O1 and -O2, and
the narrow field at a page end no longer faults.

