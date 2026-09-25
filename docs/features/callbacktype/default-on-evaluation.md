# `callbacktype` default-ON evaluation

Both arms of the SAME build — d10b3586c, this PR rebased onto ef49332b2 (#720 structheadless,
opt-in, on top of #723 globalref, #719 calleevoteperf, #721 castimplied, #722 castarith, #724
castsign) — measured on that commit, and (a)-(d), (f) and (h) measured again on fe6672881, the
same code rebased onto f434b6a60 (#727 castindex and #728 castternary, both on by default), with
the same results except (h)'s base count, which those two lower in both arms. `--option callbacktype off` is the old default; `on` is the
shipped one. The `off` arm is main: 1,615 perfect on the typesweep, and its castbench output is
byte-identical to main's own arm (`castbench/main-c960fb18d`) on all 45 binaries, so #720 moved
nothing there either.

The round-6 review measured `libselinux.so.1` at +12.67% (-O0) and +13.17% (-O2-noinline), over
the +5% budget, almost all of it decompiling the `pthread_create` start routine's one large direct
caller again, and six parks that changed nothing. This build bounds that redo: a declaration that
is exactly the signature the body already printed is not parked (`body-agrees`), and a direct
caller is decompiled again only when its call passes another number of arguments or uses the
result. Every number below is re-measured on it; (d), (h) and the arity counters are unchanged
row for row, (f) loses two callers and two cosmetic parks, and (e) now includes `libselinux`.

## (a) `make test` — the datatest corpus

```
datatests: 675/675 assertions passed
PARITY OK
```

**0 of 675 assertions move.** The corpus decompiles one function from a byte image, and this
option only ever speaks from a recorded call site, so nothing in it can reach the option.

## (b) `make test-stages`

```
datatests: 1392/1392 assertions passed
PARITY OK
```

0 assertions move. `docs/baseline-stages.json` is re-recorded only for the three assertions this
PR's own stage test adds (1,389 on main at f434b6a60 + 3; 1,372 + 3 on ef49332b2). That test is a negative control: the one-function
path has no recorded call site, so pass 1 (`option callbacktype off`) and pass 2 (`on`) print the
same thing. The off/on coverage is in the `tests/cli` probes below.

## (c) `make test-cli`

```
tests/cli: 253/253 passed
```

Three `calleevote` probes use a `qsort` comparator, `by_used`, as calleevote's control. Under the
new default the slot declares it (`int by_used(void *a0,void *a1)` where its own body gives
`bool`), which is this option's effect, not calleevote's; the three probes now pass
`--option callbacktype off` and keep main's expectations, so they test calleevote alone. The
callbacktype probes pin the declared form. Nothing else moves, and nothing moved in the rebases onto
castarith/castsign, globalref and structheadless: main's own castarith expectations for `by_used` and `mark` are
kept as they are, and the thirteen callbacktype probes pass unchanged.

The thirteen callbacktype probes over four fixtures, and what each pins:

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
there is no worse row to read. By level: O0 1,110 -> 1,114, O2 89 -> 92, O2-noinline 416 -> 419.
By project: coreutils 989 -> 993, shadow 89 -> 95, bzip2 and tar improve without a new perfect.
The 29 moved rows are the same 29, function for function, as before the redo was bounded (on
c960fb18d) and on the two bases before that (003db2dd8, 5458b7ab5), and before the narrow-return
refusal (the same rows in `moved.csv`, the same per-project block): the refusal fires nowhere in
the corpus, the `body-agrees` skip drops only parks that change no scored type, and
castarith/castsign/globalref change how some of these bodies print but no scored type.

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
0 phantom**, and **no direct call site to a moved function changes its argument count** in
either arm (re-run on d9af28b9b, 1d38ab44c and d10b3586c: the same 13 rows). The 13 are `void (int)` signal handlers (bzip2 ×6, shadow `login`/`sulogin`/`expiry`
×7) whose bodies never read the register and that nothing calls directly; each exports the new
parameter with empty `line_numbers`/`addresses`, the by-design case in spec 04.

## (e) speed — interleaved, min-of-15

| binary | off (min ms) | on (min ms) | delta min | delta median |
|---|---|---|---|---|
| `libselinux-O0` | 5348.5 | 5408.3 | +1.12% | -2.02% |
| `libselinux-O2-noinline` | 4860.0 | 4887.7 | +0.57% | +2.88% |
| `fmt-O2` | 4198.8 | 4161.5 | -0.89% | -0.41% |
| `ls-O2` | 14098.7 | 13853.2 | -1.74% | -0.06% |
| `sort-O2` | 15782.5 | 15311.8 | -2.98% | -3.52% |
| `bash-O2` | 88053.5 | 89467.3 | +1.61% | +0.11% |

Same build (d10b3586c, the bounded redo; fcf175aac only re-derives count sites on top of it), both
arms, `kuna decompile-all --json --max-fn-seconds 120`, the arms alternated per iteration and the
minimum of 15 taken (`speed.py`, raw samples in `record.json`), while other campaign lanes and this
PR's `make rust-test` lane ran. **Worst delta +1.61% (`bash-O2`), inside the +5% budget.** CPU time
(user+sys, `speed-cpu.py`, interleaved min-of-15 on fcf175aac) agrees for `libselinux`: -O0 +0.17%
(median -0.48%), -O2-noinline +1.12% (median +6.06% at load 6-9; the minimums of three separate runs
of the bounded build are +0.57% wall, +3.01% and +1.12% CPU).

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

## (f) whole-corpus `decompile-all`, 47 binaries / 22,749 functions

The implementer's 12 binaries, and the 9, 12 and 14 the round-2, -3 and -4 reviews chose to be
disjoint from the ones before. **64 functions change, all of them parked callbacks; no caller
changes. 0 unexplained.** The run parks 64 functions and 25 more are `body-agrees`: the declaration
is the signature the body already printed. Before the bound, 23 of those 25 printed the same text
parked, and two changed only in rendering (`e2fsck -O2`'s `sub_2f400` moved one `if`/`return`
through a temporary; `gnutls-serv -O2`'s `terminate` printed `(long)a0 / 10` for `a0 / 10`); both
now print what `off` prints. The two callers the unbounded redo changed, `libselinux
-O2-noinline`'s `selinux_restorecon_xattr` and `sub_1ce10` (`a1` narrowed to `unsigned int`, the
same values), are no longer decompiled again: the `pthread_once` routine they call agrees with its
body, and their call to the `pthread_create` routine passes the declared one argument and ignores
the result. No changed function gains a `CONCAT`, and no call to a parked function changes its
argument count. Nineteen binaries print no changed function. Per-function table, with each
function's casts in both arms: `docs/features/callbacktype/hunks.md`.

**Casts grow outside castbench, and that is the declared type.** Over the 64 changed functions the
cast count goes from 330 to 457 (+127): 18 functions print more, none fewer. In all 18 the body's own guess for the parameter was a pointer to something (a
synthesized `struct_N *`, `char *`, `unsigned long *`) and the slot declares `void *`, which DWARF
confirms for every one. The program converts that `void *` to its record type inside the function,
which a stripped binary does not show, so each field read prints its own cast off the `void *`, as
IDA's output does. The largest: `libselinux`'s `selinux_restorecon_thread` (`char *` -> `void *`,
+21 at -O2-noinline, +18 at -O0), `tar`'s `hol_entry_qcmp` (`struct_70 *`, +15), `e2fsck`'s
`process_inode_cmp` (`struct_126 *`, +8) and `read_bitmaps_thread` (`struct_42 *`, 0 -> 7; it was
0 -> 13 before castarith kept `void *` arithmetic in pointer terms). No type is weakened: each
parameter moves from an inferred pointee to the declared type, and each return from a wrong
`unsigned long`/`uint8` to the declared `void *`/`int`.

## (g) `p0_knowledge/modes.rs`

A shipped default of `on` takes the option out of the preset invariant's scope entirely (it
enumerates `on|off` options whose shipped default is `off`), so no `AGGRESSIVE_OVERRIDES`,
`EXCLUDED_ON_PURPOSE` or `UNEVALUATED` entry is needed and none is added.

## (h) castbench (full set, 4,815 functions shared with IDA)

Measured on fe6672881, this PR rebased onto f434b6a60 (#727 castindex and #728 castternary, on
by default), against main's own arm at that commit (the castternary lander's arm, built from the
tree main now has); the `off` arm of the same build is byte-identical to it on all 45 binaries.

```
ida    casts  37,821  /kloc 155.4  /100stmt 27.0  vs ida 1.000
main   casts  36,614  /kloc 192.1  /100stmt 30.5  vs ida 0.968
on     casts  36,635  /kloc 192.2  /100stmt 30.5  vs ida 0.969
functions: fewer casts 0, more casts 3 (+21), unchanged 4,812
```

The same +21 in the same three functions on every base since round 5: 36,617 -> 36,638 on
04f693d82, 37,477 -> 37,498 on
c960fb18d (and on ef49332b2 with the bounded redo, whose `off` arm is byte-identical to
`castbench/main-c960fb18d`), 38,602 -> 38,623 on 003db2dd8.

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
castbench's shared set the same effect adds more (+127 over the 64 changed functions of (f)), for
the same reason.

## Verdict

**All criteria pass, so the option ships `on`.** Against main (f434b6a60): +10 perfect and 29
improved with 0 worse, 13 gained parameters all DWARF-confirmed and none fabricated, no call site's
argument count moved, 64 changed functions of 22,749 (all parked callbacks; no caller changes), no
function that gains a `CONCAT`, speed within budget on every binary measured including the one the
round-6 review found over it (worst +1.61%, `bash-O2`; `libselinux` +1.12% and +0.57%), and +21
casts on castbench in the one function whose declared type is `void *`. Outside castbench the parked
`void *` parameters add casts where the body's own guess was a record or typed pointer (+127 over
the 64 changed functions): the declared type is the true one, and each field read then spells its
own conversion, as IDA's output does. The claim is the program's own declaration rather than an
inference, and it is refused wherever the body or a direct caller shows a different width,
including a computed return whose upper bytes the machine code leaves as the caller left them. The
one place it takes something away is a synthesized `struct_N *` on a `void *` parameter (`tar`,
`e2fsck`, `gnutls`), where the declaration and the ground truth agree against the synthesizer;
`--option callbacktype off` is there for an operator who wants the pointee guess back.
