# castsign: default-on evaluation

Measured on base `5458b7ab5` (origin/main when measured, castimplied #721 and
calleevoteperf #719 landed). Both arms come from the same final build: the
default-on binary, with `--option castsign off` as the old default. For castbench
the base arm is main's own binary, and `off` is byte-identical to it (see (h)).

| | criterion | result |
|---|---|---|
| (a) | `make test` with the new default | 675/675 PARITY OK. No assertion moved; `docs/baseline.json` untouched. |
| (b) | `make test-stages` | Before the new stage test: 1341/1341 PARITY OK, no assertion moved. With `kuna-castsign.xml`: 1347/1347 PARITY OK. `docs/baseline-stages.json` gains only its 6 `castsign #N` keys. |
| (c) | `make test-cli` | 237/237 with the new default; no probe moved. |
| (d) | 444-slice typesweep (pinned metric, 8 projects x O0/O2/O2-noinline) | 1,615 -> 1,615 perfect; mean .3697 -> .3697; 0 moved on, 0 moved off, 0 improved, 0 worse. All 10,748 functions score identically, and the off arm matches origin/main's sweep row for row. `typesweep-report.md`. |
| (e) | speed, interleaved min-of-15, `decompile-all --json` | fmt -O2 -0.83% min / -4.52% median; ls -O2 -0.43% / +2.34%; sort -O2 +1.06% / +1.27%; bash -O2 -1.74% / -2.47%. Worst min +1.06% (within +5%). Loads 5 to 21 while other lanes ran. `speed.json`, `speed.py`. |
| (f) | whole-corpus `decompile-all` before/after, every hunk classified | The 45 castbench binaries: 203 functions changed. 234 declaration flips, 371 sign casts dropped on a re-declared variable, 94 widening casts dropped on assignment. 0 other lines. Six more binaries (bash O2, dash O2-noinline, cf2.elf O2-noinline on ARM Cortex-M, kmod O2-noinline, bzip2 O0, crontab O0; 6,673 functions): 55 changed. 44 flips, 176 sign casts, 94 widening casts, 0 other lines. `variables[]` is byte-identical in all six. Every dropped `lhs = (T)rhs;` was re-checked against the printed declarations: 38/38 have `lhs` an integer of `T`'s width and `rhs` an integer. `corpus-hunks.json`, `hunkclass.py`, `assigncheck.py`. |
| (g) | `modes.rs` | Coherent. The catalog default is on, so every preset inherits it. `aggressive_carries_every_default_off_option` needs no entry. |
| (h) | castbench full (45 binaries, 4,815 functions shared with IDA) | 43,673 -> 43,483 casts (-190, -0.44%), 229.2 -> 228.2 per kloc, 1.155 -> 1.150 x IDA. 72 functions fewer, 0 functions more. By level: O0 1.198 -> 1.187, O2 1.168 -> 1.166, O2-noinline 1.096 -> 1.094. The off arm is byte-identical to main's castbench output (`--option castsign off` over all 45 binaries: 0 files differ from main's own run). |

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
`a_signed_only_variable_round_trips_through_the_printed_c` compiles the printed
functions of `castsign_{gcc,clang}_O{0,1}_x86_64` with the option off and on,
using gcc and clang, and requires every build to print what the binary prints.
The inputs cover indexes that run below zero, an all-space string, lengths from an
`unsigned int` table, and a logically shifted value. The two must-stay-unsigned
functions are also asserted to keep `unsigned long v1;` and their casts.
`p9_emit/kuna_castsign/tests.rs` pins the arithmetic behind each relaxation:
- the modular conversion behind the assignment drop, over every operand width and
  sign;
- the pointer-index identity;
- the pointer-conversion identity.

## Found during evaluation

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

## Output languages

C only. Rust has no implicit integer conversions, so re-declaring `v1` as `i64`
would leave `v1 = strlen(a0);` assigning a `u64`. The printer passes the option
only when the language's `integer_promotion` capability is set (the gate
`castimplied` uses). The round-trip test asserts that Rust output is identical
with the option off and on.

## Decision

Default ON.
