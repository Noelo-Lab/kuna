# castsign: default-on evaluation

Measured on the branch merged onto `52202840d` (origin/main when measured,
castarith #722 landed). castbench's base arm is a fresh build of `52202840d`
itself; `--option castsign off` on the branch build is byte-identical to it on
all 45 castbench binaries. The other criteria compare `--option castsign off`
with the default on the same branch build.

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675 PARITY OK. No assertion moved; `docs/baseline.json` untouched. |
| (b) | `make test-stages` | 1362/1362 PARITY OK. Against main's baseline the only new keys are the 9 `castsign #N` assertions of `kuna-castsign.xml`; `docs/baseline-stages.json` was re-recorded for them (1353 + 9), nothing else moved. |
| (c) | `make test-cli` | 237/237 with the new default; no probe moved. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | origin/main arm 1,615 perfect, mean .3697; branch 1,615, mean .3697. 0 moved on, 0 moved off, 0 improved, 0 worse; all 10,748 functions score identically. `typesweep-report.md`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json` | SPEED_PLACEHOLDER |
| (f) | whole-corpus `decompile-all` before/after, every hunk classified | The 45 castbench binaries: 112 functions changed, 93 declaration flips, 164 sign casts dropped on a re-declared variable, 94 widening casts dropped on assignment; 0 other lines, 0 flips that remove no cast. Fifteen disjoint binaries (`corpus-extra.py`, 8,841 functions): 40 changed, 14 flips, 45 sign casts, 102 widening casts, 0 other, 0 no-cast flips. The review's fourteen disjoint binaries (`corpus-third.py`: factor O0/O2, dd, date, expr, cksum, stat, seq, init, groupadd, mirai, certtool, e2fsck O0, ip O2; 6,547 functions): 39 changed, 34 flips, 58 sign casts, 18 widening casts, 0 other, 0 no-cast flips. Ten binaries WITH DWARF (4,254 functions): 23 changed, 8 flips, 8 sign casts, 18 widening casts plus the known `len = (unsigned int)tree[n].dl.freq;` on a register local, classified by hand; 0 locked declarations re-signed. `variables[]` is byte-identical off vs on in all 39 non-castbench binaries. Every dropped `lhs = (T)rhs;` re-checked: 27/27, 16/16, 3/3, 4/4. Every flipped variable scanned for `+ - * <<` (only arithmetic on its sign extension or truncation, the same value in both arms) and for top-bit literals on its lines (`widescan.py`: 0 decimal; the only hex ones are constants assigned to it). `corpus-hunks.json`. |
| (g) | `modes.rs` | Coherent. The catalog default is on, so every preset inherits it. `aggressive_carries_every_default_off_option` needs no entry. |
| (h) | castbench full (45 binaries, 4,815 functions shared with IDA) | 38,703 -> 38,602 casts (-101, -0.26%), 203.1 -> 202.5 per kloc, 1.023 -> 1.021 x IDA. 41 functions fewer, 0 functions more. By level: O0 1.039 -> 1.032, O2 1.057 -> 1.056, O2-noinline 0.971 -> 0.970. |

## Review round 3: a wide literal next to the variable

The review found a line that does not change and still computes a different
value. `ntohl(*p)` compared with `3000000000u` (gcc and clang `-O0`): main
declares the frame local `unsigned int v2` and prints `if (v2 == 3000000000)`;
the round-2 build declared it `int v2` and printed the same line. `3000000000`
is a `long` literal in C, so the signed `v2` is sign-extended and the comparison
is false for every `v2`: the binary returns 2, that C returns 1 under gcc and
clang at `-O0` and `-O2`. `10000000000000000000` (a 128-bit literal in gcc) did
the same to `long` locals.

A high only `castsign` admits is now left alone when a `==`, `!=`, `&`, `|` or
`^` meets it, or the expression its value is printed into, with a constant whose
top bit is set (`kuna_castsign.rs (wide_literal)`). Hex literals count too, since
the printer's choice of base is not visible to the rule. `castsign_eq_x86_64.c`
(gcc and clang `-O0`: `d_eq32`, `c_eq64`, `c_ne64`, `c_or64` and the control
`c_eq7`) joins the round trip; on the round-2 build it fails
(`castsign_eq_gcc_O0_x86_64 printed with option on and built by gcc -O0 computes
a different value`, `d_eq32 1 1 0` for `2 1 0`), and with the rule every build
passes and only `c_eq7` is re-declared. The review's own repro sources
(`cw.c`, `cz.c`, built with gcc and clang at `-O0`, `-O1` and `-O2`) now print
the same C with the option on as off, byte for byte, in all twelve builds.

Cost: two castbench declarations that the round-2 build re-signed stay unsigned,
both beside hex literals (`ls` O2 `(v17 & v25) != 0xffffffffffffffff`, five
casts; `tar` O0 `v12 != 0xffffffffffffffff`, outside the IDA-shared set), and one
in the disjoint corpus (`bash` O2 `v23 != 0xffffffff`). Against the round-2
build on the same merged base the review measured 38,703 -> 38,597; this build
gives 38,602.

Not fixed here, reported: `signedness auto` re-declares register locals by the
same walk without this check. clang `-O0` of `unsigned v = ntohl(*p); if (v ==
3000000000u) return 2; return (int)v < 0;` prints `int v1; // eax` beside
`if (v1 != 3000000000)` on origin/main, and that C returns 1 where the binary
returns 2. The spec (§9.3) states it.

Also reported by the review and left as is: for a flipped frame local, the JSON
`variables[]` keeps the frame Symbol's type (`unsigned long`) while the C
declaration says `long`. The option changes C declarations only, which is what
keeps `variables[]` and type_match byte-identical; the spec says so.

## Review round 2: arithmetic, and flips that removed no cast

The version first opened removed 190 casts (1.150 x IDA). An adversarial review
showed that it could print C that computes a different value from the binary:
re-declaring `v` signed makes every `v + k`, `v - k`, `v * k` the body prints a
signed operation, which is undefined on overflow, and gcc folds on that at
`-O0`, clang at `-O2`. `dec_neg` (`while ((long)(v - 1) >= 0 && n < 5) { v -= 2;
n++; }`, gcc -O0) with `v == 2^63` loops 5 times in the binary and 0 times in
that version's C; `count_down` (`(long)--v >= 0`, printed `v1 = v1 - 1, 0 <= v1`)
fails the same way under gcc -O2 and clang -O2. 23 of its 234 flips on the
castbench corpus also removed no cast at all.

Now a high only `castsign` admits is left alone when `+ - * <<` or unary `-`
reads it, directly or through the expression its value is printed into, and is
re-declared only when a cast the new declaration makes a no-op prints today.
Of the 234 flips, 139 are gone and none was added: 138 print arithmetic on the
variable (69 of them update the variable itself, `v = v - 1`), and one
(find -O2 `v84`, read only inside `(long)(v83 + v86 | v84 | v18)`) removed no
cast. The 84 casts the first version removed beyond this one all sat on
variables whose re-declared C is exact only under `-fwrapv`.

## What is left of the 596

Counted with the census counter over the 4,815 shared functions (a cast whose
target differs from its bare operand variable's declared type only in
signedness), unsigned-to-signed casts on locals go 669 -> 617. The first version
took them to 533; the 84 between sat on variables that `+ - *` read, where the
signed declaration would have changed what the C computes. The rest are not safe
to take by re-declaring:

- Variables the body also increments, decrements or offsets (the largest group
  now).
- -O0 register temporaries (`// eax`) that merge unrelated values, one read
  unsigned and another signed.
- Values that leave through a zero-extension whose high bits are read: a 64-bit
  return from a function kuna types `unsigned long`, or a varargs slot.
- Values really compared both ways.

Parameters carry 95 more (unchanged). Re-signing one is a prototype change: the
signature, every caller and the `decompile-project` header spell it. That is a
separate lever.

The opposite direction (declared signed, read unsigned) is 84 casts on locals
(unchanged), and much of it is `char`/`unsigned char`, so a mirror rule was not
built.

## Value preservation

`kuna-cli/tests/decompile_all_cli.rs`
`a_signed_only_variable_round_trips_through_the_printed_c` decompiles
`castsign_wrap_{gcc,clang}_O0_x86_64` and `castsign_wrap_gcc_O1_x86_64` (new;
`castsign_wrap_x86_64.c`) and the original `castsign_{gcc,clang}_O{0,1}_x86_64`,
option off and on, builds the printed functions with gcc and clang at `-O0` and
`-O2`, and requires every build to print what the binary prints. The wrap
fixture's inputs are `0`, `1`, `2`, `2^63 - 1`, `2^63`, `2^63 + 1`, `2^64 - 1`
and the 32-bit edges `2^31 - 1`, `2^31`, `2^31 + 1`, over `dec_neg`
(`(long)(v - 1) >= 0`), `cnt_wrap` (`(long)(v + 1) > (long)v`), `spin`
(`(long)(deadline - now) > 0`), `count_down` (`(long)--v >= 0`), `dec_neg32`
(`(int)(v - 1) >= 0`), and two compare-only functions, `sign_of` and
`sign_of32`, plus `peek`, a register local also used as an index. The five
arithmetic functions must print unchanged with the option on; the other three
must be re-declared. On the build before the arithmetic rule the test fails
(`castsign_wrap_gcc_O0_x86_64 printed with option on and built by gcc -O0
computes a different value`: `dec_neg`, `cnt_wrap` and `dec_neg32` wrong at
gcc -O0, all five wrong at gcc -O2 and clang -O2); with it every one of the 56
builds passes. The -O1 build is checked on `sign_of` and `peek` only: its
arithmetic shapes are register locals, which `signedness` decides and this
option does not touch (main's own C for them is exact only under `-fwrapv`).
`castsign_leaves_a_locked_declaration_alone` checks that a locked type is never
re-declared: a `--assert type` on a stack local (`sign_of`) and on a register
local (`-O1` `peek`), and a DWARF local the source declares `unsigned long`
(`castsign_dwarf_gcc_O0_x86_64` `sign_of`), keep the type and its `(long)` with
the option on and off, while the same variables unlocked are declared `long`.
Stage pass 3 checks the same with a console `map addr` lock.

## Found during evaluation

- The first build re-declared type-locked locals. The lock of a `--assert
  type`, a DWARF local or a Ghidra-committed type is on the Symbol, and the walk
  checked only the member varnodes. Over 10 DWARF binaries (4,254 functions) that
  re-signed 3 declarations the source makes unsigned: wc `wc` `uintmax_t words,
  chars`, bzip2 `fallbackQSort3` `UInt32 med`. The lock check restores exactly
  those 3 lines. The stripped corpora (castbench's 45 binaries, the 6 extra) are
  byte-identical before and after it, so (d), (f) and (h) do not move.

- Taking a type-locked unsigned parameter or an unsigned store target as unsigned
  evidence (the first cut) put 46 casts back over the corpus. It undid 20
  functions' existing `signedness` flips. gnulib's signed `idx_t` goes to
  `size_t` parameters (`xpalloc` -> `realloc`). Dropped; see analysis.md.
- The first cut of the assignment arm required the operand's C type to be stated
  in the text. That left casts behind that the re-declaration added:
  `v18 = (unsigned long)(dat_dd86c << 3);`, gzip -O0 `0xd846` and `0xd0a0`. The
  same-width modular rule covers them without looking at the operand's type.
- Unrelated to this option: gcc -O1/-O2 `long f(char *s) { long i = strlen(s) - 1;
  ... return i; }` is decompiled `void` (the counter is still in `rax` at `ret`),
  on `dbe854ba3` too. An ARM firmware function prints `if (a3 - 1U <= (int)v2)`,
  where the `U` suffix makes C compare unsigned, with or without this option.

## Final gates

On the final code (commit `158df9544`; later commits are docs only):
`make test` 675/675 PARITY OK; `make test-stages` 1350/1350 PARITY OK;
`make test-cli` 237/237; `make rust-test` RC=0 (7,484 passed, 0 failed, lane
started after the code commit); `make check-spec` and `--strict` OK;
`kuna catalog --check` OK; `counters --check` no drift; `docs/options.md` and
`phase_catalog.json` byte-fresh.

## Output languages

C only. Rust has no implicit integer conversions, so re-declaring `v1` as `i64`
would leave `v1 = strlen(a0);` assigning a `u64`. The printer passes the option
only when the language's `integer_promotion` capability is set (the gate
`castimplied` uses). The round-trip test asserts that Rust output is identical
with the option off and on.

## Decision

Default ON.
