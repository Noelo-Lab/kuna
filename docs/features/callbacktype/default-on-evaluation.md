# `callbacktype` default-ON evaluation

Both arms of the SAME build -- bfa1d32e0, this PR rebased onto 850e8c692 (#726 callpush, on by
default, after #728 castternary, #727 castindex, #720 structheadless opt-in, #723 globalref, #719
calleevoteperf, #721 castimplied, #722 castarith and #724 castsign); the typesweep and castbench ran
on c83b6121d, whose engine code bfa1d32e0 shares (it adds docs and the catalog text). `--option callbacktype off` is
the old default; `on` is the shipped one. The `off` arm is main: 1,615 perfect on the typesweep, and
its castbench output is byte-identical to main's own arm at 850e8c692 (the callpush lander's arm,
built from the tree main now has) on all 45 binaries.

The round-7 review found a caller that lost a type. A bsearch helper that calls a parked comparator
directly with its own pointer, and does nothing else with it, took that pointer's type from the
comparator's recovered `struct_N *`; decompiled again after the park, it took the slot's `void *`
instead (`ptx -O0`'s `search_table`, `struct_2 *` -> `void *`, DWARF `WORD *`). The build after it
kept the comparator's first-decompile statement for its call sites, so where the slot says only
`void *` it still types what a caller passes, and redid a caller that uses the result only when the
declared return differs from the one the callback printed.

The round-8 review found the same loss where that statement is a pointer to an unknown 8-byte value
(`undefined8 *`, printed `unsigned long *`): a comparator that hands only the first word of each
record to `strcmp` prints `void` first, so its callers that use the result are redone, and the kept
statement refused an unknown pointee, so a forwarding caller and an array-scanning caller took
`void *` for their own parameters and return and printed `&((char *)a0)[0x18]` for `&a0[3]`. This
build (99f849ef6) keeps any pointer but `void *`, which is what `protoorder` voted at the same call
before the park. Every number below is measured on it; (d), (h) and the arity counters are
unchanged row for row, and (f) now covers 70 binaries and the forwarding fixture, with columns for
a caller's own parameter and return types.

## (a) `make test` — the datatest corpus

```
datatests: 675/675 assertions passed
PARITY OK
```

**0 of 675 assertions move.** The corpus decompiles one function from a byte image, and this
option only ever speaks from a recorded call site, so nothing in it can reach the option.

## (b) `make test-stages`

```
datatests: 1397/1397 assertions passed
PARITY OK
```

0 assertions move. `docs/baseline-stages.json` is re-recorded only for the three assertions this
PR's own stage test adds (1,394 on main at 850e8c692 + 3). That test is a negative control: the one-function
path has no recorded call site, so pass 1 (`option callbacktype off`) and pass 2 (`on`) print the
same thing. The off/on coverage is in the `tests/cli` probes below.

## (c) `make test-cli`

```
tests/cli: 254/254 passed
```

Three `calleevote` probes use a `qsort` comparator, `by_used`, as calleevote's control. Under the
new default the slot declares it (`int by_used(void *a0,void *a1)` where its own body gives
`bool`), which is this option's effect, not calleevote's; the three probes now pass
`--option callbacktype off` and keep main's expectations, so they test calleevote alone. The
callbacktype probes pin the declared form. Nothing else moves, and nothing moved in the rebases onto
castarith/castsign, globalref, structheadless, castindex, castternary and callpush: main's own
castarith expectations for `by_used` and `mark` are kept as they are, and the callbacktype probes
pass unchanged (254 = main's 240 + the fourteen).

The fourteen callbacktype probes over five fixtures, and what each pins:

| probe | fixture function | pins |
|---|---|---|
| `declares-a-callback-from-the-slot-it-is-passed-to` | `by_key`, `on_int`, `by_name`, `by_count`, `warn_path` | the parks, including a value returned by a tail call and by a non-tail call |
| `off-leaves-a-callback-to-its-own-body` | the same | the `off` form |
| `is-refused-with-jobs` | — | `--jobs N` is refused |
| `refuses-a-forwarder-cast-into-a-slot` | `cmp3` | recovered-more-inputs |
| `refuses-a-second-use-of-one-hoisted-address` | `cleanup` | escapes (one `lea`, two uses) |
| `refuses-an-address-carried-past-a-phi-or-into-a-global` | `quit_either`, `quit_kept` | escapes (past a phi, into a global) |
| `refuses-a-nullary-callback-that-is-also-called-directly` | `wrap_up` | recovered-fewer-inputs, direct caller |
| `refuses-a-shorter-comparator-that-is-also-called-directly` | `score1` | recovered-fewer-inputs, direct caller |
| `refuses-a-void-body-in-a-slot-that-hands-back-a-value` | `start_puts`, `warn_count` | returns-no-value |
| `refuses-a-parameter-wider-than-the-slot-declares` | `cleanup`, `add_total` | recovered-wider-input |
| `refuses-a-return-wider-than-the-slot-declares` | `lcmp`, `lcmp_tail` | recovered-wider-output |
| `refuses-a-callee-whose-direct-caller-reads-a-wider-return` | `zcmp`, `cmp_inner`/`cmp_outer` | caller-reads-a-wider-return |
| `refuses-a-narrow-return-whose-upper-bytes-nothing-clears` | `ccmp`, `cw`, `bw` refused; `zcmp`, `zw`, `iw` parked | recovered-narrower-output |
| `keeps-the-pointer-a-caller-forwards-to-a-parked-comparator` | `search_g`, `search_c`, `search_z`, `ent_before_g2`/`_c2`, `minimum_g2`/`_c2` | direct callers keep their `struct_N *` or `unsigned long *` and print no `((char *)a0)[..]`; clang -O0's call drops the phantom `rdx` argument |

The last three are the round-2 review's counterexamples (`x7`, `x6`, `x2` + a body that never
writes the register) and **fail on 081fdd26d**: `wrap_up(v6)`, `score1(&v2[1],v5)`,
`void * start_puts(void *a0) { return (void *)CONCAT44(dat_4,puts(a0)); }`, and
`int warn_count(...) { HITS += a1; return v1; }` on an unset `v1`. The three width probes are the
round-3 review's counterexamples (`y7c`, `y7b`, `y9`) plus the three shapes found while fixing them,
all in `callbacktype_width_x86_64`. All three fail on the reviewed build (a891245ac, whose code
ce27aa5eb shares): `cleanup((int)G)` over a body that rebuilds its pointer as `CONCAT44(v2,a0)`,
`add_total((int)strtol(v1,NULL,0) * 0x3b9aca07)`, `int lcmp(...)` with
`CONCAT44(dat_4,lcmp(v6,&v4))` in `main`, and `CONCAT44(dat_4,zcmp(&v4,v6))`. The third also fails
on the first width fix (7092f2332), where `cmp_outer` returned a variable no path through the call
to `cmp_inner` sets; its first clause pins that the variable assigned from the call is the one
returned. The narrow-return probe is the round-4 review's counterexample (`t5`, `t6`), built from
clang -O2 (`ccmp`, `cw`: `mov (%rdi),%al` then a byte op) and gcc -O2 (`bw`: `setg %al; ret`) in
`callbacktype_narrow_x86_64`, and fails on the reviewed build (edc492b84, whose code f5d22a61c
shares): `int ccmp(void *a0,void *a1)`
returned `CONCAT31((undefined3)((unsigned int)v1 >> 8),...)` and `void * bw(void *a0)` returned
`(void *)CONCAT71(v1,...)`, `v1` never set. Run, the fixture shows why: `bw`'s thread result comes
back as `0x72fe5f7ff601`, the caller's leftover upper bytes over the `1`.

The forwarding probe is the round-7 review's counterexample: one `WORD` comparator handed to `qsort`
and called directly by a bsearch helper `search(WORD *w, WORD *tab, long n)`, built at gcc -O0
(`search_g`, which `calleevote` decompiles again for its `tab`), clang -O2 -fno-inline (`search_c`,
redone for the result it uses) and clang -O0 (`search_z`, redone because its call passes the `idiv`
remainder left in `rdx` as a third argument). On the reviewed build (fe6672881's code, rebased as
0949bca88) all three print `void *a0` where `off` prints `struct_0 *a0` / `struct_1 *a0`, and the
probe's four caller clauses fail; on this build all three keep
the record pointer and `search_z` calls `compare_z(a0,(struct_0 *)&a1[v2 * 2])` with two arguments.
The same fixture carries the round-8 review's counterexample at gcc -O2 and clang -O2: `by_name_*`
returns `strcmp(x->name, y->name)`, so its first decompile states `unsigned long *` for both
parameters and prints `void`; `ent_before_*` forwards its own two pointers and uses the result, and
`minimum_*` scans an array of 24-byte records with it. On the reviewed build (11206e5b5's code) all
four callers print `void *a0` and `minimum_*` indexes `&((char *)a0)[0x18]`, and the probe's eight
new clauses fail; on this build they print `unsigned long *` and `&a0[3]`, as `off` does, and the
three original parts of the fixture decompile byte-identically in the relinked binary.

## (d) the 444-slice typesweep (new default vs old default)

Metric pinned at decbench 625e892 (`final-c/pindb.py`), `DECBENCH_NO_CACHE=1`, base arm = the
default, test arm = `off`:

```
# typesweep callbacktype=off
type_match PERFECT: base 1625 -> test 1615 (-10)
aggregate type_match: base 3991.04 -> test 3973.97 (-17.07)
moved OFF perfect 10, moved ONTO perfect 0, worsened 19, improved 0
byte-identical variables in both arms: 10710 functions, 0 scored differently (must be 0)
```

Read as the flip: **1,615 -> 1,625 perfect (+10), mean .3697 -> .3713, 29 improved, 0 worse** —
there is no worse row to read. Run on 99f849ef6 (the kept statement carrying an unknown pointee), and
its `rows.json` is identical to the run on c83b6121d, every arm of every row. By level: O0 1,110 -> 1,114, O2 89 -> 92, O2-noinline 416 -> 419.
By project: coreutils 989 -> 993, shadow 89 -> 95, bzip2 and tar improve without a new perfect.
The 29 moved rows are the same 29, function for function, as on every base since round 5
(850e8c692, f434b6a60, c960fb18d, 003db2dd8, 5458b7ab5), before the redo was bounded and before the
narrow-return refusal (the same rows in `moved.csv`, the same per-project block): the refusal fires
nowhere in the corpus, the `body-agrees` skip drops only parks that change no scored type, and
castarith/castsign/globalref/callpush change how some of these bodies print but no scored type.

**Every row whose variables change, read.** `type_match` cannot see a caller that loses a type to
a synthesized record it never scores (`struct_N` never equals `WORD`), so every row whose exported
variables differ between the arms (`vars_sig`) is listed, scored or not. There are 37, and all 37
are the callbacks themselves: the qsort comparators (`compare_ranges`, `compare_words`,
`compare_occurs`, `struct_month_cmp`, `userid_compare`, `compare_dirnames`), the signal handlers
(`mySignalCatcher`, `mySIGSEGVorSIGBUScatcher`, `catch_signals`, `alarm_handler`, `sigstat`) and
`sort`'s `sortlines_thread`. The 38th row of the reviewed build, `ptx -O0`'s `search_table`, a
bsearch helper that calls `compare_words` directly and printed `void *` for its `WORD *` there, is
byte-identical between the arms now (byte-identical functions 10,710 -> 10,711).

Against the head the round-3 review read (a891245ac: 32 improved), three rows go back, none of them
perfect, and all are one pair of `ptx` comparators:

| rows | function | refusal |
|---|---|---|
| ptx -O2, -O2-noinline (improved, not perfect) | `compare_words` | caller-reads-a-wider-return: `compare_occurs` hands its whole `rax` on through its own `ret` |
| ptx -O2-noinline (improved, not perfect) | `compare_occurs` | recovered-wider-output: on that path its value is a call's full register, which nothing bounds |

DWARF declares both `int (const void *, const void *)`, so these are misses. The reviewed build
parked both and was consistent; the first width fix (7092f2332) refused only `compare_occurs` and
left it returning an unset variable, which is what the caller-read refusal closes. The first width
fix also refused coreutils' `compare_ranges` (cut and numfmt, every level, two of them perfect),
whose `movzbl %al,%eax; cmovl %edx,%eax` recovers an 8-byte `rax` output although the upper half is
provably zero; measuring the value's bits instead of its storage parks it again, and DWARF declares
it `int`.

A trace of the round-3 build and the round-4 build over the 33 binaries of the previous (f)
differed in exactly the four `ptx` functions above, and a trace of the round-4 build (f5d22a61c)
and the narrow-return fix (1d2522699) over all 47 binaries of (f) parks the same 89 functions: the narrow-return refusal
fires nowhere in the corpus.

Earlier rounds, for the record: the round-2 fixes (`recovered-fewer-inputs`, `returns-no-value`)
took back `su`'s `kill_child` at -O2 and -O2-noinline (both perfect: `su` also calls it directly,
`kill_child(0)`), and `diff`'s `compare_names_for_qsort` (a join) and `find`'s
`cost_table_comparison` (past a stack protector's branch), which the straight-line return proof
does not cross.

**Arity counters beside the metric**, over the 76 slices that hold a callback argument
(`arity-counters.py`): **13 functions gain a parameter, 0 lose one, all 13 DWARF-confirmed,
0 phantom**, and no direct call site to one of those 13 changes its argument count (re-run on
99f849ef6, bfa1d32e0, d10b3586c, 1d38ab44c and d9af28b9b: the same 13 rows). A call to a parked function
whose own count does NOT move can change, and it is the one place the declaration removes an
argument: a call that passed more than the declared list now passes the list. None does in the
70 binaries of (f); the clang -O0 build of the round-7 review's bsearch helper is one. Before
calling the comparator its `mid = (lo + hi) / 2` leaves the `idiv` remainder in `rdx`, recovery
printed `compare_z(a0,(struct_0 *)&a1[v2 * 2],(v4 + v3) % 2)`, and the declared two-parameter list
prints `compare_z(a0,(struct_0 *)&a1[v2 * 2])`, which is the call the source makes
(`callbacktype_forward_x86_64`, pinned by the forwarding probe). The 13 are `void (int)` signal handlers (bzip2 ×6, shadow `login`/`sulogin`/`expiry`
×7) whose bodies never read the register and that nothing calls directly; each exports the new
parameter with empty `line_numbers`/`addresses`, the by-design case in spec 04.

## (e) speed — interleaved, min-of-15

| binary | off (min ms) | on (min ms) | delta min | delta median |
|---|---|---|---|---|
| `libselinux-O0` | 5401.8 | 5460.2 | +1.08% | +1.85% |
| `libselinux-O2-noinline` | 4859.4 | 4957.3 | +2.01% | +2.09% |
| `fmt-O2` | 4180.8 | 4172.2 | -0.21% | +0.62% |
| `ls-O2` | 13774.4 | 13779.3 | +0.04% | -0.35% |
| `sort-O2` | 14649.5 | 14672.0 | +0.15% | +2.65% |
| `bash-O2` | 88378.2 | 87401.9 | -1.10% | -1.12% |

Same build (bfa1d32e0, whose engine code 95dbe4d23 shares), both arms, `kuna decompile-all --json
--max-fn-seconds 120`, the arms alternated per iteration and the minimum of 15 taken (`speed.py`,
raw samples in `record.json`), while this PR's `make rust-test` lane and other campaign lanes ran.
**Worst delta +2.01% (`libselinux-O2-noinline`), inside the +5% budget.** The previous base (f434b6a60, before the
kept statement): libselinux -O0 +1.12%, -O2-noinline +0.57%, fmt -0.89%, ls -1.74%, sort
-2.98%, bash +1.61%. The kept statement costs one map lookup per call at the seed and one per
argument of a call to a parked callback; the consumed-result bound removes redos.

**`libselinux` is the binary the round-6 review measured over budget**: +12.67% at -O0 and +13.17%
at -O2-noinline (CPU, min-of-11), confirmed on two more runs. Almost all of it was the redo. With
`pthread_create`'s start routine parked, its one direct caller -- a large function -- was decompiled
again, though its call passes the declared one argument and ignores the result, so it printed the
same call either way. Six `pthread_once` routines were also parked and decompiled again, with their
callers, to print exactly the `void f(void)` their bodies already gave. The bound skips both: a
declaration that is exactly the printed signature is not parked (`body-agrees`), and a direct caller
is decompiled again only when a call to the parked function passes another number of arguments or
consumes the result. What remains at -O2-noinline is decompiling the start routine itself again,
which is the change the option makes. The same bound takes nothing out of (d), (h) or the arity
counters.

The previous bases, unbounded and on the four canonical binaries only: fmt +1.15%, ls +0.42%,
sort -0.29%, bash +4.06% (+0.23% on a second run) on c960fb18d; fmt +1.39%, ls +8.15% (median
-3.21%, at load ~46, byte-identical output), sort -10.18%, bash +0.11% on 003db2dd8.

## (f) whole-corpus `decompile-all`, 70 binaries + the fixture / 40,613 functions

The implementer's 12 binaries, and the 9, 12, 14, 12 and 11 the round-2, -3, -4, -7 and -8 reviews
chose to be disjoint from the ones before, plus `callbacktype_forward_x86_64`. **124 functions
change: the 120 parked callbacks and 4 callers, all four in the fixture. In the 70 decbench
binaries every changed function is a parked callback. No function that is not parked changes a
parameter type it had or its return type, and nothing is unexplained.** The 59 binaries of the
previous run give the same 91 rows, diff for diff; the round-8 review's 11 add 24 parks. No changed
function gains a `CONCAT`, and 25 binaries print no changed function. Per-function table, with each
function's parameter types, return types and casts in both arms:
`docs/features/callbacktype/hunks.md`.

`hunks-corpus.py` compares every changed function's own printed parameter and return types. A
function that is not parked whose parameter types move is `UNEXPLAINED-own-params`, with one
exception, `caller-gained-a-forwarded-parameter`: a caller that keeps every type it had and gains,
after them, only parameters it hands straight to a parked function whose call grew to exactly the
declared list. On the round-7 reviewed build the column flagged `ptx -O0`'s `search_table`
(`struct_2 *, struct_3 *` -> `void *, struct_3 *`) and the fixture's three bsearch helpers; on the
round-8 reviewed build it flags the fixture's `ent_before_*` and `minimum_*`; on this build it flags
nothing. What closes both: the comparator's `protoorder` statement, which is what typed the caller
in the first place, is kept for the comparator's call sites, and at a call whose declared parameter
is `void *` it offers its own pointer type, whatever it points at except `void`, as a vote on the
argument (the value's type only; the argument is still converted to the declared `void *`). A
caller decompiled again -- by the park round, or by a `calleevote` round for another parameter,
which is what redoes gcc -O0's helper -- therefore types its forwarded pointer as it did before the
park. The park round also redoes fewer callers: one that uses the result only when the declared
return differs from the return the callback printed.

**What a caller still changes, and why.** The four fixture callers are the whole list. `minimum_g2`
and `minimum_c2` read the comparator's result as the declared `int`, so `(int)by_name_g2(v2,a0)`
loses its `(int)` (one cast fewer each). `ent_before_c2` (clang -O2) printed `(void)` and called the
comparator with no arguments; the declared call passes the two registers it forwards, so it gains
`unsigned long *a0,unsigned long *a1`, typed by the same vote. `search_z` drops the `idiv`
remainder its call passed past the declared list. A caller that returns the result straight on can
also take the declared return: none in the corpus does; a constructed
`int next_cmp(struct ent *p) { return p->next ? by_name(p, p->next) : 0; }` at clang -O0 goes from
`unsigned int` to `int`, the source's type and the same 32 bits. Over the review's constructed
programs (fourteen builds of four sources, gcc and clang at -O0 to -O2) every caller keeps each
parameter type `off` prints; the differences are the same three kinds: parameters gained that are
forwarded, `(int)` casts gone from the result, and that one return.

**Casts grow outside castbench, and that is the declared type.** Over the 115 changed functions of
the 70 decbench binaries the cast count goes from 553 to 769 (+216): 37 functions print more, one
fewer (`xmlwf`'s `nsattcmp`, whose `(unsigned long)` on the returned difference goes with the declared
`int`). In all 37 the body's own guess for the parameter was a pointer to something (a synthesized
`struct_N *`, `char *`, `unsigned long *`, `long *`, `int *`, `uint4 *`) and the slot declares
`void *`, which DWARF confirms for every one.
The program converts that `void *` to its record type inside the function, which a stripped binary
does not show, so each field read prints its own cast off the `void *`, as IDA's output does. The
largest: `libselinux`'s `selinux_restorecon_thread` (`char *` -> `void *`, +21 at -O2-noinline, +18
at -O0), `tar`'s `hol_entry_qcmp` (`struct_70 *`, +15), `ptx -O0`'s `compare_words` (+13),
`certtool -O0`'s `setof_compar` (+9), `e2fsck`'s `process_inode_cmp` (+8, +6, +6) and
`read_bitmaps_thread` (0 -> 7 at each level). One of `process_inode_cmp`'s is an address handed to
an import whose recovered parameter is an integer: `ext2fs_const_inode(&a0->field_0x4[0x14])`
prints `ext2fs_const_inode((int8)a0 + 0x18)`, the same value, in the `(long)v` form castarith
leaves as integer arithmetic. No caller's type moves. Inside the callbacks each parameter
moves from an inferred pointee to the declared type, which DWARF gives, and each return from a wrong
`unsigned long`/`uint8` to the declared `void *`/`int`; where the inferred pointee was a
synthesized record, that record is what the declaration takes away (see the verdict).

## (g) `p0_knowledge/modes.rs`

A shipped default of `on` takes the option out of the preset invariant's scope entirely (it
enumerates `on|off` options whose shipped default is `off`), so no `AGGRESSIVE_OVERRIDES`,
`EXCLUDED_ON_PURPOSE` or `UNEVALUATED` entry is needed and none is added.

## (h) castbench (full set, 4,815 functions shared with IDA)

Measured on 99f849ef6, this PR rebased onto 850e8c692 (#726 callpush, on by default), against
main's own arm at that commit (the callpush lander's arm, built from the tree main now has); the
`off` arm of the same build is byte-identical to it on all 45 binaries. The numbers are those of
c83b6121d, before the unknown-pointee fix.

```
ida    casts  37,821  /kloc 155.4  /100stmt 27.0  vs ida 1.000
main   casts  35,588  /kloc 188.2  /100stmt 29.9  vs ida 0.941
on     casts  35,609  /kloc 188.3  /100stmt 29.9  vs ida 0.942
functions: fewer casts 0, more casts 3 (+21), unchanged 4,812
```

The same +21 in the same three functions on every base since round 5: 36,614 -> 36,635 on
f434b6a60, 36,617 -> 36,638 on 04f693d82, 37,477 -> 37,498 on c960fb18d, 38,602 -> 38,623 on
003db2dd8. Neither fix for a caller's parameter changes a castbench function: neither shape is in
the shared set.

**+21 casts (+0.06%), in three functions, all the same one:** `sort`'s `pthread_create` start
routine at -O0, -O2 and -O2-noinline (+7 each; +13 each before castarith). The source is
`static void *sortlines_thread (void *data)`, and DWARF and IDA both give it that prototype; its
own body gave `unsigned long sub_ba80(unsigned long *a0)`, a wrong parameter and a wrong return.
Declared `void *`, each of its seven field reads casts off the `void *` to the type of the
parameter it is passed to (`*(long *)a0`, `((unsigned long *)a0)[1]`, `((FILE **)a0)[5]`,
`((char **)a0)[6]`, ...), one cast each, the same seven IDA spends (`*((long long *)a1 + 1)`).
Main prints the same call with no cast only because it reads the `void *` as an
`unsigned long *`. No cast is removed, so no computed value can change,
and no type is weakened: the three functions move from a wrong type to the declared one. Outside
castbench's shared set the same effect adds more (+216 over the 115 changed functions of (f)), for
the same reason.

## Verdict

**All criteria pass, so the option ships `on`.** Against main (850e8c692): +10 perfect and 29
improved with 0 worse, 13 gained parameters all DWARF-confirmed and none fabricated, 124 changed
functions of 40,613 over 70 binaries and the forwarding fixture (in the 70 decbench binaries only
the 115 parked callbacks; no function that is not parked changes a parameter type it had or its
return type), no function that gains a `CONCAT`, speed within budget on every binary measured
(SPEED_WORST), and +21 casts on castbench in the one function whose declared type is `void *`.
Outside castbench the parked `void *` parameters add casts where the body's own guess was a record
or typed pointer (+216 over the 115 changed functions, DWARF agreeing with the slot in all 37 that
gain one): the declared type is the true one, and each field read then spells its own conversion,
as IDA's output does.

A direct caller keeps the types it had: where the slot says only `void *`, the pointer type the
callback's first decompile gave the parameter, whatever it points at except `void`, still types
what the caller passes, so a forwarding or array-scanning caller prints the `struct_N *` or
`unsigned long *` and the indexing it printed before. What the declaration changes in a caller is
the call it rebuilds, and the fixture shows each kind: the call passes the declared arguments, so a
call that passed more drops the extra one and a caller that forwarded registers its first decompile
never read gains them as parameters after the ones it had; and the result is the declared `int`, so
an `(int)` cast on it goes, and a caller that returns it straight on can return `int`. The claim is
the program's own declaration rather than an inference, and it is refused wherever the body or a
direct caller shows a different width, including a computed return whose upper bytes the machine
code leaves as the caller left them. The one place it takes something away is inside the callback
itself: a synthesized `struct_N *` on a `void *` parameter (`tar`, `e2fsck`, `gnutls`, `ptx`),
where the declaration and the ground truth agree against the synthesizer; `--option callbacktype
off` is there for an operator who wants the pointee guess back.
