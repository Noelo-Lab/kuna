# `callbacktype` default-ON evaluation

Both arms of the SAME build -- be75f8cc8, this PR rebased onto 0096e984d (every cast lever on:
castimplied, castarith, castsign, globalref, castindex, castternary, callpush; structheadless
opt-in). `--option callbacktype off` is the old default; `on` is the shipped one. The `off` arm is
main: 1,615 perfect on the typesweep, and its castbench output is byte-identical to main's own arm
at 0096e984d on all 45 binaries.

**What this build changes.** A park changes only the parked function. The park round runs after
every other decompile of the whole-binary run -- the callee-first pass, `calleevote`'s rounds and
the convergence pass -- and decompiles the parked callbacks again and nothing else, so every other
function, a direct caller of a callback included, prints exactly what it prints with the option
off. The previous builds decompiled a parked callback's direct callers again, so that their calls
would follow the declaration, and four review rounds in a row found a caller that lost its own
parameter or return types or gained casts that way (a bsearch helper's `WORD *`, a forwarding
caller's `unsigned long *`, and last a thread's direct caller forwarding its own `long` id into the
parked `void *` slot, which became `void *a0`). That redo, the kept-statement vote that patched
it, and their plumbing are deleted. A caller now keeps the call it printed, so where that call
contradicts the declaration the park is refused instead: the call passes another number of
arguments than the declaration lists, or uses a result the declaration does not return
(`direct-call-disagrees`), or it sits in code the run did not decompile (`caller-not-seen`).

## (a) `make test` — the datatest corpus

```
datatests: 675/675 assertions passed
PARITY OK
```

**0 of 675 assertions move.** The corpus decompiles one function from a byte image, and this
option only ever speaks from a recorded call site on a whole-binary run.

## (b) `make test-stages`

```
datatests: 1400/1400 assertions passed
PARITY OK
```

0 assertions move. `docs/baseline-stages.json` is re-recorded only for the three assertions this
PR's own stage test adds (1,397 on main at 0096e984d + 3). That test is a negative control: the
one-function path has no recorded call site, so pass 1 (`option callbacktype off`) and pass 2
(`on`) print the same thing. The off/on coverage is in the `tests/cli` probes.

## (c) `make test-cli`

```
tests/cli: 259/259 passed
```

259 = main's 244 + fifteen callbacktype probes over five fixtures. Three `calleevote` probes use a
`qsort` comparator, `by_used`, as calleevote's control; under the new default the slot declares it
(`int by_used(void *a0,void *a1)` where its own body gives `bool`), which is this option's effect on
the comparator itself, so the three probes pass `--option callbacktype off` and keep main's
expectations. Nothing else moves.

| probe | fixture function | pins |
|---|---|---|
| `declares-a-callback-from-the-slot-it-is-passed-to` | `by_key`, `on_int`, `by_name`, `by_count`, `warn_path` | the parks |
| `off-leaves-a-callback-to-its-own-body` | the same | the `off` form |
| `is-refused-with-jobs` | -- | `--jobs N` is refused |
| `leaves-a-direct-caller-of-a-parked-callback-as-it-was` | `compare_g`/`_c`, `by_name_g2`, `worker_g6` and their callers | parks; every direct caller prints its `off` form |
| `refuses-a-callback-whose-direct-call-passes-another-count` | `compare_z`, `by_name_c2`, `worker_c7` | direct-call-disagrees |
| `refuses-a-forwarder-cast-into-a-slot` | `cmp3` | recovered-more-inputs |
| `refuses-a-second-use-of-one-hoisted-address` | `cleanup` | escapes (one `lea`, two uses) |
| `refuses-an-address-carried-past-a-phi-or-into-a-global` | `quit_either`, `quit_kept` | escapes |
| `refuses-a-nullary-callback-that-is-also-called-directly` | `wrap_up` | recovered-fewer-inputs |
| `refuses-a-shorter-comparator-that-is-also-called-directly` | `score1` | recovered-fewer-inputs |
| `refuses-a-void-body-in-a-slot-that-hands-back-a-value` | `start_puts`, `warn_count` | returns-no-value |
| `refuses-a-parameter-wider-than-the-slot-declares` | `cleanup`, `add_total` | recovered-wider-input |
| `refuses-a-return-wider-than-the-slot-declares` | `lcmp`, `lcmp_tail` | recovered-wider-output |
| `refuses-a-callee-whose-direct-caller-reads-a-wider-return` | `zcmp`, `cmp_inner`/`cmp_outer` | caller-reads-a-wider-return |
| `refuses-a-narrow-return-whose-upper-bytes-nothing-clears` | `ccmp`, `cw`, `bw` refused; `zcmp`, `zw`, `iw` parked | recovered-narrower-output |

The two new probes run on `callbacktype_forward_x86_64`, which now also carries the round-9
review's pthread shape: a start routine handed an integer id through its `void *`, with one direct
caller that forwards its own `long` and uses the result and one that ignores it, at gcc -O2
(`worker_g6`, parked) and clang -O2 (`worker_c7`, whose caller forwards its register untouched and
prints `worker_c7()` with no argument, refused). Both probes fail on the previous head (5ced9725b's
code): `long run_share_g6(void *a0)`, `void run_quiet_g6(void *a0)`,
`return (long)worker_g6(a0) * 3;`, the three-argument `compare_z` call cut to two, and
`ent_before_c2(unsigned long *a0,unsigned long *a1)` where `off` prints `(void)`.

## (d) the 444-slice typesweep (new default vs old default)

Metric pinned at decbench 625e892 (`final-c/pindb.py`), `DECBENCH_NO_CACHE=1`, base arm = the
default, test arm = `off`:

```
# typesweep callbacktype=off
type_match PERFECT: base 1625 -> test 1615 (-10)
aggregate type_match: base 3991.04 -> test 3973.97 (-17.07)
moved OFF perfect 10, moved ONTO perfect 0, worsened 19, improved 0
byte-identical variables in both arms: 10711 functions, 0 scored differently (must be 0)
```

Read as the flip: **1,615 -> 1,625 perfect (+10), mean .3697 -> .3713, 29 improved, 0 worse.
All of the previous build's +10 survives the re-scope**: `rows.json` is identical to the previous
build's run, row for row and arm for arm (only the file order differs), because every function the
metric scores changed only inside the callbacks themselves, and no scored callback has a direct
caller whose printed call contradicts its declaration. By level: O0 1,110 -> 1,114, O2 89 -> 92,
O2-noinline 416 -> 419. By project: coreutils 989 -> 993, shadow 89 -> 95, bzip2 and tar improve
without a new perfect. There is no worse row to read. The 37 rows whose exported variables differ
between the arms (`vars_sig`) are all callbacks: the qsort comparators (`compare_ranges`,
`compare_words`, `compare_occurs`, `struct_month_cmp`, `userid_compare`, `compare_dirnames`), the
signal handlers (`mySignalCatcher`, `mySIGSEGVorSIGBUScatcher`, `catch_signals`, `alarm_handler`,
`sigstat`) and `sort`'s `sortlines_thread`.

**Arity counters beside the metric**, over the 76 slices that hold a callback argument
(`arity-counters.py`): **13 functions gain a parameter, 0 lose one, all 13 DWARF-confirmed,
0 phantom**, and no direct call site to one of them changes its argument count -- rows identical to
the previous build's. The 13 are `void (int)` signal handlers (bzip2 x6, shadow `login`/`sulogin`/
`expiry` x7) whose bodies never read the register and that nothing calls directly; each exports the
new parameter with empty `line_numbers`/`addresses`, the by-design case in spec 04. No call to a
parked function changes its count either: a caller is never decompiled again, and a call whose
count differs from the declaration refuses the park.

## (e) speed — interleaved, min-of-15

| binary | off (min ms) | on (min ms) | delta min | delta median |
|---|---|---|---|---|
| `libselinux-O0` | 5360.2 | 5353.5 | -0.12% | -3.41% |
| `libselinux-O2-noinline` | 4963.2 | 4928.8 | -0.69% | -0.80% |
| `fmt-O2` | 4227.4 | 4193.4 | -0.80% | -0.59% |
| `ls-O2` | 14470.8 | 14549.8 | +0.55% | +4.73% |
| `sort-O2` | 14956.8 | 14500.1 | -3.05% | -1.28% |
| `bash-O2` | 94212.7 | 93627.8 | -0.62% | +4.61% |

Same build (419b1b6a2, whose engine code be75f8cc8 shares), both arms, `kuna decompile-all --json
--max-fn-seconds 120`, the arms alternated per iteration and the minimum of 15 taken (`speed.py`,
raw samples in `record.json`), while other campaign lanes ran. **Worst delta +0.55% (`ls-O2`), inside
the +5% budget.** `libselinux` is the binary the round-6 review measured at +12.67% (-O0) and
+13.17% (-O2-noinline) CPU when the park redid the thread routine's one large direct caller; the
bounded redo brought it to +1.17% / +1.76% wall, and with no caller ever decompiled again the
option's whole cost is decompiling the parked callbacks once more, after everything else.
`libselinux-O2-noinline` and `sort-O2` are a second run: in the first, one `off` run of
`libselinux-O2-noinline` was killed from outside (SIGTERM) and its partial time became the arm's
minimum (+6.84% against it; +0.80% without it), and `sort-O2` read -7.68%; `speed.py` now drops a
sample whose run fails.

## (f) whole-corpus `decompile-all`, 70 binaries + 5 fixtures / 40,724 functions

The implementer's 12 binaries and the 9, 12, 14, 12 and 11 that successive reviews chose to be
disjoint from the ones before, plus the five callbacktype fixtures, classified by `hunks-corpus.py`,
which now counts ANY changed function that is not parked as unexplained. **128 functions change,
and every one is a parked callback: 115 in the decbench binaries, 13 in the fixtures. No function
that is not parked changes at all -- not a caller, not a parameter type, not a return type, not a
cast (`UNEXPLAINED-own-params` 0, caller hunks 0, `UNEXPLAINED` 0).** The 115 decbench rows are the
same functions with the same diffs as the previous build's parked rows. The previous build also
redid 4 callers in the forwarding fixture; they are byte-identical between the arms now. The
round-9 review's own counterexample, a pthread routine taking an integer id through its `void *`
with direct callers `run_share` and `run_quiet` at gcc and clang -O0 and -O2, was run through the
same script: its callers are byte-identical to main's output in all four builds, the routine is
parked at gcc -O0, gcc -O2 and clang -O0, and refused at clang -O2, where `run_share` forwards its
register untouched and prints `sub_1170()` against the declared one parameter. Per-function table:
`docs/features/callbacktype/hunks.md`.

**Casts grow inside the parked callbacks, and that is the declared type.** Over the 115 changed
decbench functions the cast count goes from 553 to 769 (+216): 37 print more, one prints fewer
(`xmlwf`'s `nsattcmp`, whose `(unsigned long)` on the returned difference goes with the declared
`int`). In all 37 the body's own guess for the parameter was a pointer to something (a synthesized
`struct_N *`, `char *`, `unsigned long *`, `long *`, `int *`) and the slot declares `void *`, which
DWARF confirms for every one. The program converts that `void *` to its record type inside the
function, which a stripped binary does not show, so each field read prints its own cast off the
`void *`, as IDA's output does. The largest: `libselinux`'s `selinux_restorecon_thread` (+21 at
-O2-noinline, +18 at -O0), `tar`'s `hol_entry_qcmp` (+15), `ptx -O0`'s `compare_words` (+13),
`certtool -O0`'s `setof_compar` (+9), `e2fsck`'s `process_inode_cmp` and `read_bitmaps_thread`.
No caller's casts move.

## (g) `p0_knowledge/modes.rs`

A shipped default of `on` takes the option out of the preset invariant's scope entirely (it
enumerates `on|off` options whose shipped default is `off`), so no `AGGRESSIVE_OVERRIDES`,
`EXCLUDED_ON_PURPOSE` or `UNEVALUATED` entry is needed and none is added.

## (h) castbench (full set, 4,815 functions shared with IDA)

Against main's own arm at 0096e984d (`/home/mahaloz/kwt/castbench/main-0096e984d`); the `off` arm of
the same build is byte-identical to it on all 45 binaries.

```
ida    casts  37,821  /kloc 155.4  /100stmt 27.0  vs ida 1.000
main   casts  35,588  /kloc 188.2  /100stmt 29.9  vs ida 0.941
on     casts  35,609  /kloc 188.3  /100stmt 29.9  vs ida 0.942
functions: fewer casts 0, more casts 3 (+21), unchanged 4,812
```

**+21 casts (+0.06%), in three functions, all the same one:** `sort`'s `pthread_create` start
routine at -O0, -O2 and -O2-noinline (+7 each). The source is `static void *sortlines_thread (void
*data)`, and DWARF and IDA both give it that prototype; its own body gave `unsigned long
sub_ba80(unsigned long *a0)`, a wrong parameter and a wrong return. Declared `void *`, each of its
seven field reads casts off the `void *` to the type of the parameter it is passed to (`*(long
*)a0`, `((unsigned long *)a0)[1]`, `((FILE **)a0)[5]`, ...), the same seven IDA spends. No cast is
removed, so no computed value can change, and no type is weakened: the three functions move from a
wrong type to the declared one. The re-scope moves nothing here: the same +21 in the same three
functions as every previous base.

## Verdict

**All criteria pass, so the option ships `on`.** Against main (0096e984d): +10 perfect and 29
improved with 0 worse -- all of the previous build's gain survives the re-scope, row for row; 13
gained parameters, all DWARF-confirmed and none fabricated; 128 changed functions of 40,724 over 70
binaries and the five fixtures, every one a parked callback, and not one caller, parameter type,
return type or cast changes anywhere else; no function gains a `CONCAT`; speed within budget on
every binary measured (worst +0.55%, `ls-O2`); and +21 casts on castbench in the one function whose declared
type is `void *`.

What the re-scope gives up is consistency between a callback and a caller that already agreed
with it in count and result. Such a caller keeps the call its first decompile printed, argument
types included: `run_share_g6` still passes its `long` into `worker_g6`'s declared `void *`, and a
caller that read a comparator's result through `(int)` while the comparator printed `void` keeps
the cast now that it prints `int`. Both are what that caller prints with the option off, and the
declaration is the callee's own; a caller's types are never taken from it. Where a printed call
disagrees in count or in using a result, the callback is refused and keeps what its body
recovered (`compare_z`, `by_name_c2`, `worker_c7` in the forwarding fixture; none of the corpus's
115 parks). Inside the parked callbacks the parameters move from an inferred pointee to the
declared type, which DWARF gives, and each field read spells its conversion off the `void *`, as
IDA's output does; where the inferred pointee was a synthesized record (`tar`, `e2fsck`, `gnutls`,
`ptx`), that record is what the declaration takes away, and `--option callbacktype off` is there
for an operator who wants the pointee guess back.
