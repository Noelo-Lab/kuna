# castsign: default-on evaluation

Measured on base `5458b7ab5` (origin/main when measured, castimplied #721 and
calleevoteperf #719 landed). Both arms come from the same final build: the
default-on binary, with `--option castsign off` as the old default. For castbench
the base arm is main's own binary, and `off` is byte-identical to it (see (h)).

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675 PARITY OK. No assertion moved; `docs/baseline.json` untouched. |
| (b) | `make test-stages` | 1350/1350 PARITY OK. No assertion outside `kuna-castsign.xml` moved. That file's assertions were rewritten for the arithmetic rule (9 now, from 8), so `docs/baseline-stages.json` changes only in its `castsign #N` keys and the totals. |
| (c) | `make test-cli` | 237/237 with the new default; no probe moved. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | 1,615 -> 1,615 perfect; mean .3697 -> .3697; 0 moved on, 0 moved off, 0 improved, 0 worse. All 10,748 functions score identically to the off arm, row for row. `typesweep-report.md`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json` | SPEED_PLACEHOLDER |
| (f) | whole-corpus `decompile-all` before/after, every hunk classified | The 45 castbench binaries: 114 functions changed. 95 declaration flips, 171 sign casts dropped on a re-declared variable, 94 widening casts dropped on assignment. 0 other lines, and 0 flips that remove no cast. Fifteen more binaries, disjoint from castbench (bash O2, dash O2-noinline, cf2.elf O2-noinline on ARM Cortex-M, kmod O2-noinline, bzip2 O0, crontab O0, and od, numfmt, pr, last, chage O0, csplit O2-noinline, minigzip, xmlwf, dpkg-query O2; 8,841 functions): 41 changed. 15 flips, 50 sign casts, 102 widening casts, 0 other lines, 0 flips that remove no cast; `variables[]` byte-identical in all fifteen. Every dropped `lhs = (T)rhs;` was re-checked against the printed declarations: 27/27 and 16/16 have `lhs` an integer of `T`'s width and `rhs` an integer. Ten binaries WITH DWARF, where every stack local is type-locked (4,254 functions): 23 changed, 8 flips, 8 sign casts, 18 widening casts plus the known `len = (unsigned int)tree[n].dl.freq;` on a register local, classified by hand; 0 locked declarations re-signed, `variables[]` identical in all ten. Every flipped variable was also scanned for `+ - * <<` in its function's printed text: the only matches are arithmetic on its sign extension or truncation (`(long)v4 + v5 * 0x1f` where main printed `(long)(int)v4 + v5 * 0x1f` at the same place), which computes the same value in both arms. `corpus-hunks.json`, `hunkclass.py`, `assigncheck.py`, `corpus-extra.py`, `corpus-dwarf.py`. |
| (g) | `modes.rs` | Coherent. The catalog default is on, so every preset inherits it. `aggressive_carries_every_default_off_option` needs no entry. |
| (h) | castbench full (45 binaries, 4,815 functions shared with IDA) | 43,673 -> 43,567 casts (-106, -0.24%), 229.2 -> 228.6 per kloc, 1.155 -> 1.152 x IDA. 42 functions fewer, 0 functions more. By level: O0 1.198 -> 1.192, O2 1.168 -> 1.167, O2-noinline 1.096 -> 1.095. The off arm is byte-identical to main's castbench output. |

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
Of the 234 flips, 162 are gone (148 print arithmetic on the variable, 69 of them
updating the variable itself; the rest are arithmetic printed as `v4 -= v7` or
flips that removed nothing), none was added, and 23 more come back through the
second rule's cast-under-a-conversion case (`SEXT816((long)v34)`,
`(long)(int)v8`). The arithmetic rule is what the lost 84 casts cost; each of
them sat on a variable whose re-declared C is exact only under `-fwrapv`.

## What is left of the 596

On the shared set, locals carried 526 unsigned-to-signed casts and now carry 390.
The rest are not safe to take by re-declaring:

- -O0 register temporaries (`// eax`) that merge unrelated values, one read
  unsigned and another signed.
- Values that leave through a zero-extension whose high bits are read: a 64-bit
  return from a function kuna types `unsigned long`, or a varargs slot.
- Values really compared both ways.

Parameters carry 70 more. Re-signing one is a prototype change: the signature,
every caller and the `decompile-project` header spell it. That is a separate lever.

The opposite direction (declared signed, read unsigned) is 20 casts on locals, so
a mirror rule was not built.

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

GATES_PLACEHOLDER

## Output languages

C only. Rust has no implicit integer conversions, so re-declaring `v1` as `i64`
would leave `v1 = strlen(a0);` assigning a `u64`. The printer passes the option
only when the language's `integer_promotion` capability is set (the gate
`castimplied` uses). The round-trip test asserts that Rust output is identical
with the option off and on.

## Decision

Default ON.
