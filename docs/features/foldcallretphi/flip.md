# foldcallretphi — the default flip

`foldcallretphi` shipped off in #654 with its corpus sweep recorded and two
reasons to hold the default: a `tests/cli` probe addressed a variable by its
auto-generated `vN` name, and a pass that moves where a call is evaluated should
earn its default with its own evidence. This is that evidence, measured on
`e1139df9` with one build: the new default against `--option foldcallretphi off`.
`boolbyte` (`f1ec42a7`) and then `structsynth` (`6e4f6fa5`) went default-on
while this was measured, so the branch was rebased twice, and each time the
gates, the 32-binary sweep, the typesweep and the speed pairs were run again on
the rebased build. Tables below are the final base, `6e4f6fa5`; where a number
moved between bases, the others are given beside it.

A third review found two more ways a fold could change what the C does, both
reproduced as a compile-and-run mismatch, and both now decline (see *The width
guard* and *The short-circuit guard*). Every measurement below was run again on
that build.

`--option foldcallretphi off` is `main`: over 24 of the 32 sweep binaries below,
text and `--json`, the `off` dumps are byte-identical to dumps two reviews made
with a release binary built from `main` (48 of 48 files).

## What it looks like

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware authenticate
```

```diff
-  int v1; // eax
-  char v2 [8];
+  char v1 [8];
 ...
-  v1 = strcmp(a1,sneaky);
-  if (!v1)
+  if (!strcmp(a1,sneaky))
     return 1;
```

and in coreutils `sort` -O2 `main` (`kuna decompile-all sort`, case `0x85`):

```diff
         if (v98) {
-          v15 = strcmp(v98,optarg);
           v31 &= 0xffffffff;
-          if (v15) {
+          if (strcmp(v98,optarg)) {
```

## The width guard

The first version of this flip also folded calls whose result the caller reads
narrower than the callee returns. `sort` -O2 `main` ends with

```
  if ((dat_1d418) && (v15 = sub_c5b0(stdin), v15 == -1))      // off, int v15
  if ((dat_1d418) && (sub_c5b0(stdin) == -1))                 // that version
```

and `sub_c5b0` is `rpl_fclose`: the binary returns `mov $0xffffffff,%eax`, and
kuna prints it `unsigned long sub_c5b0(FILE *a0)` ending in `return 0xffffffff;`.
Read against that prototype, the folded compare can never be true, so the
`close failed` path disappears. The spill's `int` declaration was the only thing
narrowing `rax` to `eax`. Over 32 binaries the review found 9 such new
always-false `-1` checks (`sort` O2 and O0 `main`, `du` O0 `sub_11107`, `od`
O2-noinline `main`, `scp` `sub_167b0` and `sub_17720`, `pr` O2-noinline `main`,
`sftp` `sub_16ef0` and `sub_17e60`), and about 45 folds per dozen binaries that
dropped an `int`/`char` narrowing in the same way.

The discount now declines unless the call's output carries everything the
callee returns (`kuna_foldcallretphi.rs` `call_output_is_full_width`): a locked
output (a declared or library prototype) qualifies, and an unlocked one
qualifies only when it is as wide as the prototype model's return entry that
holds it. An unlocked `eax` read out of `rax` keeps its spill. On the same 32
binaries `narrowcount.py` now finds **0** folds that drop a narrowing (it found
44 over the core eight and 45 over the next twelve before), and the uncast
`f(..) == -1` / `!= -1` count on a callee printed with an 8-byte return is 45
on `off` and 46 on the default. The one extra is `du` O0 `sub_11107`'s
`getline(&v13,&v12,v19) != -1`, whose spill was `long v6` and whose callee is
declared `long`, so that compare is right.

The stage test pins both sides: with `helper` undeclared the default keeps the
spill (its output is `eax`, `helper` writes `rax`), and after
`parse line extern int4 helper(int8 a);` the same call folds.

That first guard only asked the return *register* entries, so it missed the
joined pairs. On i386 a `long long` comes back in `edx:eax`, which
`x86gcc.cspec` declares as a join entry for 5 to 8 bytes, and the guard's
size filter skipped it; on ARM, `r0:r1` is a model rule and not an entry at all.
An undeclared call whose caller reads only `eax` therefore still folded:

```
long long big(void *f) { return f ? 0xffffffffLL : 1; }
int t(void) { int r = (int)big(g); if (r == -1) return 1; return 7; }   // gcc -m32 -O2

off:      v1 = big(dat_1050); if (v1 != -1) return 7; return 1;
before:   if (big(dat_1050) != -1) return 7; return 1;     // big printed unsigned long long
```

The binary compares `eax` alone and returns 1; read against kuna's own
`unsigned long long big()` the folded form returns 7. The guard now also asks
the model where it would return an `int` of each power-of-two size above the
output's, up to `long long`, and declines when that storage contains the output
(`wider_int_return_contains`). On x86-64 this changes nothing: an 8-byte `rax`
output has no wider size to ask about, and a narrower one was already declined
by the register entry. Pairs wider than `long long` (`rdx:rax`, `x0:x1`) are not
asked about; they carry `__int128` or a two-word struct, and counting them would
mark every 64-bit output narrowed. On a 32-bit target the result is that only
declared callees fold under this option: over the three ARM binaries in the
sweep, 53 functions go back to the `off` text (52 in `cf2.elf`, 1 in
`RTOSDemo.out`). `tests/stages/kuna-foldcallretphi-join.xml` is the i386 case:
undeclared `big` keeps its spill by default, and declared `int4 big(int4)` folds.

## The short-circuit guard

P-code `BOOL_AND`/`BOOL_OR` evaluate both operands. C's `&&`/`||` skip the
right-hand one when the left one decides. gcc -O2 compiles

```
int r = fflush(stdout); gflag = (r == 0) && (a > 5);
```

to a single block (`call fflush; test eax,eax; sete dl; cmp ebx,5; setg al;
and eax,edx; mov gflag,eax`). Every op the folded expression travels through is
in the call's block, so the span guard passed it, and the default printed

```
off:      v1 = fflush(stdout); gflag = (unsigned int)(5 < a0 && !v1);
before:   gflag = (unsigned int)(5 < a0 && !fflush(stdout));
```

which calls `fflush` only when `a0 > 5`; the binary always calls it. The discount
now declines when the call's value reaches a `BOOL_AND` or `BOOL_OR` as its
right-hand operand anywhere along the print chain
(`chain_reaches_short_circuit_rhs`). The left-hand operand is always evaluated,
and the printer emits input 0 on the left, so `(ferror(stdin) || 5 < a0)` still
folds. A structured `if (a && b)` built from two blocks is a different thing:
there the call sits in the second block and the binary skips it too, so
`faillog`'s `(!fsync(v5)) && (!fclose(dat_6298))` is unchanged.

Over the 48 binaries of the sweep below no fold of this option reached a
right-hand operand, and the review's scanner finds none in either build. On the
review's `cx3`/`cx7` test programs (gcc -O0 to -Os, seven binaries) the old
default put a call on the right of `&&`/`||` 16 times; now 0 times. The 19 lines
that fold a call into the left-hand operand print the same in both builds.

`foldcallret` alone, on by default before this PR, has the same hole for calls
that take no global operand (`if (a0 <= 5 || getpid())`). Fixing it changes
today's default output, so it is GH-684 and not part of this change.

## (a)-(c) Parity gates, new default

| gate | result |
|---|---|
| `make test` | 675/675, PARITY OK — 0 assertions move |
| `make test-stages` | 1160/1160 on `6e4f6fa5` with both new guards (1156 before them, 1141 on `e1139df9`), PARITY OK. The option's stage test now runs its default passes first with no option command, so the corpus pins the shipped default: the undeclared-`helper` pass (spill kept), the declared one (folded), `target2` (global write blocks), and `target3`, gcc -O2's setcc/and shape for `(r == 0) && (a > 5)` (spill kept); then `option foldcallretphi off`. `kuna-foldcallretphi-join.xml` is new: i386 `edx:eax`, spill kept for undeclared `big`, fold once it is declared `int4`. Its five old assertion names are re-recorded and four new rows added in `docs/baseline-stages.json`; no other row moves. The four new assertions fail on the build before the guards |
| `make test-cli` | 189/189 on `6e4f6fa5` with both new guards (185 on `e1139df9`); one probe re-pointed, see below |

`tests/cli/no-cli-rename-or-prototype-override` runs
`kuna decompile fauxware authenticate --assert 'type v2 char[16]' --assert 'name v2 credbuf'`.
The directives are about the 8-byte `read` buffer at `stack - 0x18`, and `v2`
was that buffer's name only because `v1` was the spilled `strcmp` result. With
the fold `if (!strcmp(pass,sneaky))` there is no `v1` spill, the buffer is `v1`,
and `v2` is the byte at `stack - 0x10`: the old directives would retype that
byte into a 16-byte `credbuf` and leave a separate `char v2 [8]` read buffer. The
probe now says `v1`, and gains one clause, `read\(\w+,credbuf,8\)`, which is the
thing it meant all along: the buffer the directives name is the one `read`
fills. Its derived id moves `a-a58fc408288b` -> `a-76f56989c6d3`. With `v1` the
default output is the `off` output minus the `strcmp` spill.

The need record `docs/re-needs/no-cli-rename-or-prototype-override.md` carries
the same probe fenced in its `## Acceptance` section, and `verify` runs that copy,
not the `tests/cli` one. It is re-pointed the same way (fenced probe,
front-matter `acceptance_id`, its `index.json` row, and a decision-log line), so
`verify --acceptance-suite --need no-cli-rename-or-prototype-override` reports
`PASS unchanged` on the new default. Over all 155 acceptances the suite reads
pass 49 / regressed 1 (`typed-mach-o-tail`), the same as `main`.

`make rust-test` has three more fences on default output, and each moves
because a spill is gone:

* `kuna-console/tests/verify_assertplane.rs` (6 cases). The file's register-local
  cases target `int v1; // eax` in `fauxware` `authenticate`, which is the
  `strcmp(a1,sneaky)` spill; folded, `authenticate` has no register local left
  and its buffer is `v1`. The fixture now loads with `foldcallretphi off`, so
  every directive names what it named before; the plane is under test there,
  not the fold.
* `kuna-console/tests/verify_libcsigs.rs` `callee_return_type_reaches_the_local_that_holds_it`
  asserted `unsigned int v4;` / `int4 v4;` for `open`'s fd, which is `v3` once
  the spill above it goes. It now finds the local assigned `open(...)` and
  checks that declaration's type, so it no longer depends on a number.
* `kuna-ghidra/tests/ghidra_sim_e2e.rs` `ghidra_sim_faillog_pins`: the flattened
  C line counts of `faillog` move 287 -> 286 (`sub_2620`, `main`) and 39 -> 37
  (`sub_3320`). In `main` `fstat(fileno(dat_6298),..)` and `!fclose(dat_6298)`
  fold; in `sub_3320` `fread` and `fwrite` on `dat_6298` fold into their tests
  and one declaration goes. The `sub_3de0`/`sub_31a0`/`sub_3900` and `fseeko`
  results, which the first version folded, keep their spills under the width
  guard. The CLI path loses the same 1 and 2 lines. `sub_3ad0`, the getPcode /
  getMappedSymbols traffic and every other measurement are unchanged.

## (d) type_match — 444-slice typesweep

`scripts.decbench.typesweep --option foldcallretphi off` with the flipped build,
over coreutils, grep, gzip, diffutils, bzip2, findutils, tar and shadow at O0,
O2 and O2-noinline (`x` has no slices in this tree): 444 slices, 10,748 scored
functions. The baseline arm is the new default and the test arm is `off`, so the
columns below are written from the default's side.

| project | slices | functions | perfect off -> on | mean off -> on | improved | worse |
|---|---:|---:|---|---|---:|---:|
| bzip2 | 6 | 267 | 19 -> 19 | 0.3664 -> 0.3664 | 0 | 0 |
| coreutils | 294 | 6422 | 610 -> 610 | 0.2813 -> 0.2813 | 0 | 0 |
| diffutils | 12 | 420 | 52 -> 52 | 0.4004 -> 0.4004 | 0 | 0 |
| findutils | 3 | 790 | 35 -> 35 | 0.1622 -> 0.1622 | 0 | 0 |
| grep | 3 | 247 | 39 -> 39 | 0.3870 -> 0.3870 | 0 | 0 |
| gzip | 3 | 368 | 88 -> 88 | 0.4858 -> 0.4858 | 0 | 0 |
| shadow | 120 | 686 | 29 -> 29 | 0.2897 -> 0.2897 | 0 | 0 |
| tar | 3 | 1548 | 114 -> 114 | 0.2824 -> 0.2824 | 0 | 0 |
| **pooled** | **444** | **10748** | **986 -> 986** | **0.2895 -> 0.2895** | **0** | **0** |

That table is `6e4f6fa5`. On every base the run is neutral in both directions:

| base | perfect off -> on | aggregate `type_match` (both arms) | improved | worse |
|---|---|---|---:|---:|
| `e1139df9` | 959 -> 959 | 3037.19 | 0 | 0 |
| `f1ec42a7` (`boolbyte` on) | 987 -> 987 | 3112.68 | 0 | 0 |
| `6e4f6fa5` (`structsynth` on) | 986 -> 986 | 3111.18 | 0 | 0 |
| `6e4f6fa5`, both new guards in | 986 -> 986 | 3111.18 | 0 | 0 |

Controls (every base): 10,525 functions carry byte-identical `variables[]` in
both arms and all score identically; the other 223 differ only in `vN` names and
line spans, and score identically too. (The first version, without the width
guard, measured 10,298 and 450.)

The counters `type_match` cannot see do not move either. On `6e4f6fa5`
`decomp_vars` is 96,697, `tp` 18,396, `fp` 19,788, `fn` 27,531 and `gt_vars`
65,715 in both arms (on `e1139df9`: 96,697 / 17,965 / 20,217 / 27,533 / 65,715,
both arms), and on every base the per-function variable count is identical in
all 10,748 functions (97,376 exported variables each arm), so no argument or
phantom variable is added or removed. With the two new guards the run and all
five counters are unchanged (986 -> 986, 3111.18, 0 moved; 96,697 / 18,396 /
19,788 / 27,531 / 65,715 in both arms, `decomp_vars` equal in all 10,748
functions): every function the guards change is on ARM, and the typesweep
projects are x86-64.

## (e) Speed

Interleaved `--option foldcallretphi off` / default pairs of `kuna decompile-all`,
alternating which arm runs first, **minimum of 15 pairs**, same build, guards
in. The child's CPU time (user+sys) is recorded beside the wall clock and
tracks it within 0.02% (`decompile-all` is one thread). The box was shared with
the other campaign lanes, so medians carry contention; every delta over 5% was
re-run.

With all three guards (the final build):

| case | min off | min default | min delta | median delta | CPU min delta |
|---|---:|---:|---:|---:|---:|
| `fmt` O2 (151 fn), during a concurrent `rust-test` run | 4093.6 ms | 4206.7 ms | **+2.76%** | -1.68% | +2.76% |
| `fmt` O2, re-run on a quieter box | 4026.3 ms | 4034.7 ms | **+0.21%** | +0.14% | +0.21% |
| `ls` O2 (404 fn) | 13355.0 ms | 13266.6 ms | **-0.66%** | +0.03% | -0.66% |
| `sort` O2 (343 fn) | 13725.4 ms | 13647.4 ms | **-0.57%** | -0.42% | -0.57% |
| `bash` O2 (1.3 MB, `reliable` mode) | 77299.4 ms | 77238.5 ms | **-0.08%** | -0.30% | -0.08% |

Worst minimum +2.76%, the `fmt` read taken under load; on x86-64 the new probe
never runs (an 8-byte `rax` has no wider size to ask about, and a narrower
output is declined by the register entry first), so the guards add a loop over
at most eight ops per candidate. With the width guard alone (round 2):

| case | min off | min default | min delta | median delta | CPU min delta |
|---|---:|---:|---:|---:|---:|
| `fmt` O2 (151 fn) | 4120.9 ms | 4130.9 ms | **+0.24%** | +1.15% | +0.25% |
| `ls` O2 (404 fn) | 13299.8 ms | 13342.2 ms | **+0.32%** | -1.68% | +0.32% |
| `sort` O2 (343 fn) | 13981.2 ms | 14091.4 ms | **+0.79%** | -1.09% | +0.79% |
| `bash` O2 (1.3 MB, `reliable` mode) | 76624.4 ms | 77298.8 ms | **+0.88%** | -0.19% | +0.88% |

That table is `6e4f6fa5` before round 3, worst minimum +0.88% against the 5% budget. The
same pairs on the other bases: `e1139df9` fmt +1.73%, ls -3.69%, sort -0.02%, bash -1.15%;
`f1ec42a7` fmt +0.18%, ls +0.26%, sort -2.62%, bash +4.17% (`sort` first read +6.04%
under load, median -2.10%, and the re-run is the number given). The first
version, without the width guard, measured a worst minimum of +3.76% on `bash`.

## (f) Whole-binary decompile-all, every hunk classified

`decompile-all` off vs the new default, text and `--json`, over 48 stripped
binaries (18,196 functions): `fmt`, `ls`, `sort`, `du` at O0 and O2; O2 `grep`,
`gzip`, `diff`, `find`, `bzip2`, `dash`, `scp`, `ssh-add`, O0 `gzip`, `cp`,
O2-noinline `od` and the ARM firmware `ch.elf`; O2 `tail`, `kmod`, `crond`,
`chage`, `init`, `dpkg-statoverride`, `mirai`, `sftp`, `ocsptool`, the ARM
`RTOSDemo.out`, O0 `diff` and O2-noinline `pr`; and the sixteen a reviewer added,
O2 `csplit`, `expr`, `cmp`, `crontab`, `xmlwf`, `minigzip`, `ssh-keyscan`,
`psktool`, `tar`, `rtmon` and the ARM `cf2.elf`, O0 `head`, `useradd`, `stat`,
O2-noinline `shutdown` and `ptx`. Function sets and error records are identical
in both arms, and the `off` arm is byte-identical, text and `--json`, to the
dumps the reviewer made from a `main` build of the sixteen (32 of 32 files).
300 functions change: 187 of the first 32 binaries' 11,279 and 113 of the
sixteen's 6,917. Against the round-2 build the only difference is 53 functions
that are now printed exactly as `off` prints them, all on ARM (52 in `cf2.elf`,
1 in `RTOSDemo.out`): the joined-return guard. No x86-64 function changes.

`hunkclass.py` (per-function list in `flip-hunks.txt`) compares each function
with every `vN` renamed to `V`, so a pure renumbering is not a hunk, and
classifies the 1,088 hunks that remain:

| class | hunks |
|---|---:|
| fold: `V = f(...);` gone, `f(` at the use | 411 |
| fold inside a condition's comma expression: `(V = f(..), V == K)` -> `(f(..) == K)` | 80 |
| the same, with an argument moved into its own comma clause ahead of the call (`V = &a2[V], !memcmp(V,V,V)`) | 13 |
| fold split across two hunks (the spill and its use in different hunks) | 14 |
| declaration lines (a removed spill's declaration, or a declaration list reordered by renumbering) | 562 |
| a load given its own statement at its defining op (`V = a0->field_0x0;`) | 2 |
| anything else, read by hand | 6 |

The two load hoists are `tar` O2 `sub_12b40` and `ssh-keyscan` O2 `sub_56650`
(`v7 = 0` printed at its defining op). Of the six read by hand, `cf2.elf`
`sub_80340bc` folds a float call into the global store that follows it
(`dat = (float)f(..); dat = dat * 1.1;` -> `dat = (float)f(..) * 1.1;`, the
binary's single `vstr`), `ssh-keyscan` O2 `sub_56650` is the other half of its
hoist, and the remaining four are the ones the round-2 sweep read:

`du` O0 `main` is a fold inside `||`
(`(V = ferror_unlocked(stdin), V || ..)` -> `(ferror_unlocked(stdin) || ..)`);
`gzip` O0 `sub_5f11` folds `fsync(..)` and `close(..)` in place inside one
compound condition; `find` O2 `sub_f400` and `ocsptool` O2 `sub_8c70` give
loads a statement of their own at their defining op, just ahead of the
`__snprintf_chk` / `if` they feed (in `sub_8c70` a load the `off` text printed
twice is now read once). Those hoists are the knock-on the original analysis
describes: marking one value implied re-dirties its operands' covers, and a
neighbour that loses its own implied test is printed at its defining op. That
never moves an evaluation later; it only stops one from being inlined. No
function's count of `if`/`else`/`while`/`for`/`switch`/`goto`/`return`/`?`
changes in either direction, so no hunk alters control-flow shape.

What the folded calls moved past, with the three metrics #654 shipped
(`flip-metrics.txt`):

* `evalorder.py` — three functions change their event set: the two load hoists
  above (a hoisted load counts as a statement of its own; the `sub_8c70`
  duplicate read is gone) and `cf2.elf` `sub_80340bc`, whose two stores to one
  global become the binary's one. Twelve differ in event order: `ls` O0
  `sub_c308`, `dash` O2 `sub_5940`, `ssh-add` O2 `sub_a6b0`, `sub_217a0`,
  `sub_25c40`, `crontab` O2 `sub_4b40`, `csplit` O2 `sub_39e0`, `ssh-keyscan` O2
  `sub_25a00`, `sub_49f70`, and `tar` O2 `sub_14c50`, `sub_185e0`, `sub_18fd0`.
  Each is an adjacent fold such as `V[strcspn(V,"\n")] = 0;`,
  `*(char *)mempcpy(..) = 0;`, `__sprintf_chk(&V[strlen(dat_f208)],..)`, or a
  call folded into the leftmost operand of an `&&`/`||` chain
  (`if ((!fgets(V,0xfff,stdin)) || ..)`). The metric orders those innermost-first,
  while C still runs the call before the store or the next operand.
* `foldmove.py` — 477 removed spills located, 253 mapped to a landing statement:
  249 land on the very next statement, 4 after one intervening statement, and
  **0 hazards**. The four cross an assignment to a local: `v11 = (unsigned long
  *)0x180a0` (`gzip` O2 `sub_6e90`, a register), `v1 = 0` (`kmod` O2 `sub_15390`,
  `int v1; // eax`), `V = &V` (`stat` O0 `sub_809b`) and a string-constant
  assignment (`shutdown` O2-noinline `main`). The 224 unmapped are its named
  buckets (154 call text not unique in the function, 60 call printed
  differently because an argument was hoisted or folded, 10 landing not in the
  hunk); the hunk table above covers them.
* `callorder.py` — 0 functions change call-to-call order.

A last pass (on the first 32 binaries) lists, for every removed spill whose text is unique in its function,
the statements between it and the spill's first read in the `off` text. Other
than the two above, it finds `v31 &= 0xffffffff` in `sort` O2 `main` (a register
local, no `&v31`) and `v17 = 0xa04c` / `v17 = 0xa0d3` in `ssh-add` O2 `sub_9cb0`
(the phantom return-address slot, no `&v17`). The `ssh-add` O2 `sub_a6b0` stack
store the first version crossed is gone: `sub_9b10` returns an `int`, so the
width guard keeps that spill.

`--json`: in all 18,196 functions the `variables[]` multiset of (kind, type,
size, stack offset, argument index) is identical in both arms, no function's
variable or argument count moves, and no field other than `code`,
`line_mappings` and the variables' names and addresses differs.

### Compile-and-run

`foldrun.c` transcribes sites from the dumps, both renderings verbatim, and runs
them against stub callees that log every call with its arguments and write the
globals the site reads (a call may write any global: the stub `strcmp` rewrites
`optarg`, `fileno`/`fclose`/`fstat` swap `dat_6298`, `fclose` bumps the
`dat_6276` that `exit` reads). Each pair must agree on the trace, the return
value and every global afterwards:

| site | shape | cases |
|---|---|---:|
| A `sort` O2 `main` | `v31 &= 0xffffffff; if (strcmp(v98,optarg))`, the call moved past a register self-op | 4 |
| C `faillog` `main` | `(!fsync(v5)) && (!fclose(dat_6298))`, then `exit(dat_6276)` | 8 |
| D `faillog` `main` | `v4 = fstat(fileno(dat_6298),(stat *)0x61e0);` | 2 |
| E `du` O0 `sub_11107` | `while (getline(&v13,&v12,v19) != -1)` (loop body reduced to a counter), an 8-byte `-1` | 4 |
| B `sort` O2 `main` | the default keeps `v15 = sub_c5b0(stdin), v15 == -1` | 4 |
| F `pr` O2-noinline `main` | the default keeps `v4 = sub_60c0(stdin), v4 == -1` | 4 |
| B' / F' the pre-guard fold `sub_c5b0(stdin) == -1` / `sub_60c0(stdin) == -1`, callee returning 0 | agree | 3 + 3 |
| control B' / F': the same, callee returning `0xffffffff` as the binary and kuna's own callee text do | must differ | 1 + 1 |
| control K: `stages/kuna-foldcallretphi.xml` `target2` folded past `k = 42` | must differ | 1 |
| G `cx7` z1 -O2 | `(ferror(stdin) \|\| 5 < a0)`, the call folded into the left operand | 8 |
| G `cx7` z3 -O2 | the default keeps `v1 = fflush(stdout); (5 < a0 && !v1)` | 8 |
| G' the pre-guard fold `(5 < a0 && !fflush(stdout))`, `a0 > 5` | agree | 4 |
| control G': the same, `a0 <= 5` | must differ (no `fflush` in the trace) | 4 |
| H i386 `kuna-foldcallretphi-join` | the default keeps `v1 = big(dat_1050); if (v1 != -1)` | 2 |
| H' the pre-guard fold `big(dat_1050) != -1`, callee returning 1 | agree | 1 |
| control H': the same, callee returning `0xffffffff` in an `unsigned long long` | must differ | 1 |

`gcc -O0` and `-O2`: `FOLDRUN OK: 63/63 cases as expected` (35/35 before the
two new guards' sites were added). The B' control
reports `sub_7480(close failed,-)` for `off` and nothing for the old fold, the
F' control `error(1,9,standard input)` against a silent `return 3`, K
`ok=5` against `ok=42`, G' a trace with `fflush(0)` against an empty one, and H'
`ret=1` against `ret=7`, so the check sees every defect class: the narrowing
the width guard keeps (register and joined pair), the move past a write the
order guard refuses, and the skipped call the short-circuit guard refuses.

### What a fold can still drop

The width guard keeps every narrowing of an integer up to `long long`. What a full-width unlocked output still
carries only in the caller is its signedness: kuna types it from this function's
own reads, so a callee printed `unsigned long` whose result the caller spilled
into a `long` loses that conversion when folded. That matters only under a
relational comparison, a shift or a division. Over the 32 binaries, 32 folded
calls feed a relational comparison and none feeds a shift or division; all 32
are declared library calls (`__fprintf_chk(..) < 0`, `5 <= strlen(v10)`,
`fwrite(..) < v1`, ...) or already print a cast (`(long)lseek(a0,v22,0) < 0`).

The width guard belongs to this option's discount only. `foldcallret`, on by
default before this PR, folds calls that take no global operand without asking
about width (`if (!sub_c5b0(a0))`, a few functions away from `sort` `main`, is
one), and the 45 uncast `-1` compares on the `off` side of the narrowcount are
its folds, not this option's. Guarding it too would change today's default
output and needs its own evidence, so it is left to its own change.

### Goal 2 — varcensus

`scripts/pipeline/varcensus.py --text` over the core eight dumps (declarations
excludes nothing; single-def/single-use counts a declared name assigned once
and read once):

| binary | declarations | single-def/single-use |
|---|---|---|
| fmt O2 | 352 -> 352 | 78 -> 78 |
| ls O2 | 1322 -> 1321 | 255 -> 254 |
| sort O2 | 1125 -> 1125 | 203 -> 203 |
| du O2 | 1021 -> 1018 | 176 -> 173 |
| fmt O0 | 302 -> 302 | 78 -> 79 |
| ls O0 | 1208 -> 1205 | 255 -> 254 |
| sort O0 | 1145 -> 1143 | 215 -> 214 |
| du O0 | 991 -> 986 | 233 -> 231 |
| **total** | **7466 -> 7452 (-14)** | **1493 -> 1486 (-7)** |

That table is `e1139df9`. On `f1ec42a7` the totals are 7460 -> 7446 and
1493 -> 1486, and on `6e4f6fa5` 7464 -> 7450 and 1491 -> 1484: the same -14
and -7 on every base, from starting points the two default flips had already
moved. The hunk list, `narrowcount.py`, the three order metrics and the
`variables[]` comparison are identical on both rebased sweeps.

Params 5570 both arms, functions 2918 both, `[16]` blobs 66 both. The first
version, which also folded the narrowed results, measured -39 and -19; the width
guard gives back 25 declarations and 12 temporaries, the spills of undeclared
callees whose output is narrower than the register they return in. `fmt` O0's
single-def/single-use count rises by one where an operand hoist gives a value a
statement of its own.

## (g) Modes

`foldcallretphi` came off `modes.rs`'s `UNEVALUATED` list: the
`aggressive_carries_every_default_off_option` invariant only covers options that
ship off, and `aggressive`/`reliable`/`fast` carry no entry for it, so every mode
now runs the default.

## Decision

Flipped. Every criterion holds with all three guards in, on `e1139df9` and again
on each rebase (`f1ec42a7`, `6e4f6fa5`), and once more on `6e4f6fa5` after the
joined-return and short-circuit guards: 0 of 675 datatest assertions move and
`tests/stages` is PARITY OK with no opt-out added (the option's own stage test
runs its default passes with no option command, gains the short-circuit pass,
and gets an i386 sibling for the joined pair); the typesweep is neutral in both
directions (0 improved, 0 worse, the same perfect count in both arms, every
arity and phantom counter identical); the worst min-of-15 speed delta is
+2.76% on the final build (`fmt` O2 under load; +0.21% re-run); every hunk over the 48 binaries is a fold, the
knock-on operand hoist, a renumbering or a declaration. No folded call crosses a
call, a load, a store through a pointer, a global write or a branch, and none
lands in the right-hand operand of `&&` or `||`. The most a fold moves past is
an assignment to a local whose address is never taken (a register, or the
phantom return-address slot). No fold drops a narrowing of an integer up to
`long long`: a call whose output is narrower than a return register or joined
pair that could hold its callee's result keeps its spill, so on 32-bit targets
only declared callees fold. What the default buys is goal 2 only, 14 fewer
declared locals and 7 fewer single-use temporaries over the core eight
binaries. What it costs is that a `--assert type vN` / `name vN` directive
written against pre-flip output can name a different variable, which is why one
`tests/cli` probe and three `rust-test` fences moved with it, plus the
same-width signedness case above, which no fold in the 32 binaries exercises.
`foldcallret`'s own folds can still land in the right-hand operand of `&&`/`||`
(GH-684); that is today's default and is left to its own change.
