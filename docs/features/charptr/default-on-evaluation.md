# `charptr` default-on evaluation — HELD OPT-IN, and no longer a candidate

## Re-evaluation on main `ce008ce2b` (after #695, #704 and #692)

The flip was measured again on current main, both arms from one build: the
option off (the shipped default) against `("charptr", "on")` in
`AGGRESSIVE_OVERRIDES`, which is what `--mode auto` runs under 500 KiB. It fails
two criteria, so there is no flip PR.

| criterion | result |
|---|---|
| (c) `make test-cli` with the new default | **FAIL**, 213/219. The same six probes move. Three of them catch a real rendering regression, described below, so they cannot be re-pinned. |
| (d) 444-slice typesweep, new default against old | **FAIL**. Perfect stays at 1,353, the aggregate goes from 3670.31 to 3669.74 (-0.57), and 2 functions improve while 4 get worse. The criterion needs improved >= worse. |
| (a), (b), (e), (f) | Not re-run, because (c) and (d) already decide the flip. (a) and (b) cannot move anyway, since the datatest harness applies no mode. |

The "known state" the flip lane was started from (1,349 -> 1,353, +4.93, 14 up
and 6 down) was measured before #704. #704 made a constant `PTRADD` index count
as a field offset, and that removed nearly all of the gain. On this base the
typesweep moves six functions:

| slice | function | GT variable | off | on | what it is |
|---|---|---|---|---|---|
| coreutils O0 tac | `output` | `char *start` (arg) | `void *` miss | `char *` hit | better |
| tar O0 tar | `read_header` | `char *bp` (stack) | `unsigned long *` miss | `char *` hit | better |
| coreutils O2-noinline head | `elide_tail_bytes_pipe` | `__off_t current_pos` (arg 4) | `unsigned long`, credited | `char *` miss | worse. An integer the body adds byte counts to and passes to `elseek`. The census walk gets `ev=byte` on its stack copy (`stack@-152`), so an integer accumulator becomes a pointer. This is a false positive. |
| tar O0 / O2 / O2-noinline | `check_compressed_archive` | `_Bool temp` (stack) | `undefined1`, credited on width | `char` miss | worse, three rows for one function. The byte is read through `pshort`, which kuna already types `char *` in both arms. With the option on, that commit spreads to the byte, and the metric then scores `char` exactly against `_Bool`. |

The two `od print_long_double` rows from the earlier table no longer move.

### What (c) actually catches

```
$ kuna decompile-all decompiler/crates/kuna-analysis/tests/fixtures/protoorder_widefill_x86_64 --option charptr on
int fill_words(char *a0,int a1)
      builtin_strncpy(v2,"\b\a\x06\x05\x04\x03\x02\x01",8);      <-- was *v2 = 0x102030405060708;
int fill_many(char *a0,int a1)
  a0[0x10] = 'u';                                               <-- 520 eight-byte stores become
  a0[0x11] = 'u';                                                   4,160 one-byte stores
```

The narrow-vote fixture shows the same thing. `fill`'s single
`*(unsigned long *)((long)a0 + 0x10) = 0x2020726174737575;` becomes eight byte
stores, and the `int` store beside it moves ahead of them. The cause is an
interaction between the two options, not the flip's own vote.
`kuna_protoorder::pointee_refuses` refuses a narrow pointer vote when the caller
stores wider constants through it. It exempts a *character* pointee on purpose,
because byte stores through a `char *` are what the string-copy idiom prints as
`builtin_strncpy`. With `charptr` on, `peek` recovers `char *` instead of
`unsigned char *`, and the exemption lets that vote reach every caller that
fills the buffer a word at a time. With `--option protoorder off`, `fill_words`
stays `unsigned long *`. The three probes that pin whole wide stores
(`protoorder-types-keeps-a-wide-store-whole`,
`protoorder-types-keeps-a-word-fill-whole`,
`ptrfromuse-default-declares-a-dereferenced-parameter-void`) are right to fail.
The other three (`protoorder-types-the-callers-argument`,
`protoorder-off-loses-the-callee-type`,
`protoorder-lock-declines-a-register-saturating-callee`) change only in
signedness: `unsigned char *` becomes `char *`, and a `(unsigned char)` cast
appears on each `movzbl` read. That spelling change is legitimate but reads
worse.

A future flip needs `pointee_refuses` to stop exempting a character pointee
when the wide store is a non-string constant. The metric cost has its own
causes, which `next-levers.md` covers.

## Update after the offset rule was completed

Once a constant `PTRADD` index counts as a field offset, as a literal `INT_ADD`
always did, criterion (d) fails as well: the 444-slice sweep with the option on
moves no function onto or off a perfect `type_match`, 2 up and 4 down, aggregate
-0.57 (it was +4 onto perfect, 14 up, 6 down, +4.93). Nearly all of the earlier
gain was the skip-the-first-character idiom, which has the shape of a struct
field (see `analysis.md`, "The offset rule, completed"). There is no longer a
measured case for the flip, whatever happens to the six probes below. The table
and the rest of this page are the evaluation as it stood before that change.


Evaluated on `feat/charptr` over base `724381149` (re-run after the rebase that
brought in #689 `argclobber`, a default-on change to call-site argument lists;
the first evaluation was on `2e28ece4a` and every criterion below came back the
same, with the sweep's moved rows byte-identical). "Default on" for an option of
this kind means membership in `AGGRESSIVE_OVERRIDES`
(`p0_knowledge/modes.rs`), because `--mode auto` picks `aggressive` under
500 KiB and that is the mode `decompile-all`, the web front-end and the
benchmark all run.

| criterion | result |
|---|---|
| (a) `make test` with the new default | **pass** — 675/675, PARITY OK. The datatest harness applies no mode, so no assertion moves. |
| (b) `make test-stages` | **pass** — 1,262/1,262, PARITY OK; the stage test's pass 1 carries its own `option charptr off`. |
| (c) `make test-cli` | **FAIL** — 208/214. The same six probes move. |
| (d) 444-slice typesweep, new default vs old | **pass** — perfect 1,349 -> 1,353, aggregate 3657.76 -> 3662.69, 4 onto perfect, 0 off perfect, 14 improved, 6 worse. improved (18) >= worse (6); every worse row is read in `record.json`. |
| (e) `timeit` interleaved min-of-15, fmt/ls/sort -O2 + bash -O2 | **pass** — re-run on the rebased base, option-on against the same build with it off: fmt +3.88%, ls +2.11%, sort +4.77%, bash +2.45%; worst +4.77%, inside the +5% budget. (First run, quieter box: -1.59 / -0.37 / +2.67 / +4.36.) |
| (f) whole-corpus `decompile-all` over 8 binaries, every hunk classified | **pass** — every hunk falls in a documented class; 0 arity changes over 2,290 functions. |
| (g) `modes.rs` coherent | held as an `EXCLUDED_ON_PURPOSE` entry with this evaluation cited. |

## The failing criterion

With `("charptr", "on")` in `AGGRESSIVE_OVERRIDES`, six `tests/cli` probes fail,
all of them pinning the pointer spelling a *different* feature produces (a
seventh, `cold-load-xref-lookup`, is the known `wall_ms` contention flake and
passes on a re-run):

```
FAIL protoorder-types-the-callers-argument
     expected int callee\(unsigned char \*a0,int a1\)                     actual <no match>
     expected unsigned long caller\(unsigned char \*a0,int a1\)            actual <no match>
     expected caller\(\(unsigned char \*\)0x402000,3\);                   actual <no match>
FAIL protoorder-lock-declines-a-register-saturating-callee
FAIL protoorder-off-loses-the-callee-type
FAIL protoorder-types-keeps-a-wide-store-whole
     expected \*\(unsigned long \*\)\(\(long\)a0 \+ 0x10\) = 0x2020726174737575;
FAIL protoorder-types-keeps-a-word-fill-whole
FAIL ptrfromuse-default-declares-a-dereferenced-parameter-void
     expected int fill\(void \*a0,int a1\)                                actual <no match>
```

Each of those probes would have to be re-pinned to `char *`, and one of them
(`protoorder-types-keeps-a-wide-store-whole`) pins a *store rendering* that
moves with the pointee.

## The decision

Held opt-in. The gain is real and one-directional but small — +4.93 of an
aggregate 3657.76, which is +0.13% of the corpus — and it does not buy
re-pinning two other features' regression probes. The class census in
`analysis.md` says why the gain is small and where the rest of the `char *` gap
actually is (register-only ground-truth variables, and frame slots kuna reports
as `undefined8`), so the case for flipping this option should be re-made only
together with that work, not on its own.

## Addendum (review round 2)

Two costs were under-recorded and are now in `record.json` and in the option's
`phases.toml` / `docs/spec/05-types.md` prose. A flip attempt has to read both:

1. The fabricated string literal (diffutils `diff`, `sub_2180a`) — a *wrong
   output* in the option arm, root-caused to the read-only painting of ELF
   loader tables and fixed in this PR as a strict fix. It is pinned by
   `tests/cli/loader-table-bytes-print-as-a-string.json`.
2. The reversal class: a correct `char *` becoming `unsigned char *` as a
   knock-on. 13 lines in 7 functions on dash `-O0`, which no 444-slice project
   carries, so criterion (d) cannot see it.

Criterion (c) still fails, so the default stays `off`.
