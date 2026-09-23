# calleevote — analysis

## 1. The measured symptom

Round E's O2 census (`docs/decbench/typecampaign/final-e/o2census/`) ranked the
O2 blockers. The largest O2-noinline bucket is a program struct pointer that kuna
declares `void *` in a function that is otherwise right: 143 functions at
O2-noinline and 20 at O2 where fixing that one variable makes the function
perfect under decbench#93 crediting, 66 of them gnulib `Hash_table` getters.

```
$ kuna decompile-all O2-noinline/coreutils/stripped/chcon --addr 0x68f0
unsigned long sub_68f0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x10);
}
```

DWARF: `size_t hash_get_n_buckets (const Hash_table *table)`.

The census named two causes. Both were measured before anything was built
(`census-bucket.py`, `census-callers.py`, `census-votes.py` in this folder).

## 2. What each candidate fix reaches

**(a) The reverse vote.** For the 166 solo functions of the bucket (O2 +
O2-noinline, round-E rows): 150 have callers, 11 have none, 5 have their address
taken. What the callers pass in that argument, read off their own C:

| callers pass | functions |
|---|---:|
| no argument at all (the caller forwards the register untouched) | 44 |
| `void *` (the caller's own parameter) | 31 |
| mixed / disagreeing | 34 |
| `unsigned long *`, `int *`, `long`, locals of other types | ~30 |
| a `struct_N *` | 4 |

So a caller-to-callee vote reaches at most ~8 of the 166: the callers of a
`Hash_table` getter hold `void *` too. It is still worth building, for a
different population. Over every wrong argument at O2/O2-noinline, a parameter
the callee typed only `void *` or an integer, all of whose callers pass one
committed pointer, is 131 wrong-to-right under the pinned metric (mostly
`char *`: `gzip_error (char const *m)`-shaped wrappers whose callers pass
literals), 41 more under #93 crediting, and 18 where the callers' type is wrong.

**(b) A lone field.** `structsynth` declines a pointer read at one offset: it
wants two distinct offsets including 0 (`kuna_structsynth.rs` `accepts_record`).
Relaxing that alone did nothing, and the reason is a second blocker the census
did not name. `ActionStructSynth` fires only once `ActionInferTypes` reports a
pass that changed nothing (`kuna_infertypes_settled`). A short function's main
loop ends on the pass that first typed it (nothing after it changes an op, so no
second pass runs) and the synthesizer is never offered the function at all.
With both the relaxation and one forced settle pass, over the 296 O2/O2-noinline
slices:

| arm | pinned perfect | #93-credited perfect | worse |
|---|---:|---:|---:|
| main | 455 | 557 | - |
| settle pass only | 455 | 557 | 0 |
| settle pass + lone field (any width) | 455 | 704 | 24 |
| settle pass + lone field (width >= 4) | 455 | 694 | 22 |

All 22-24 worse rows are one shape: a `qsort` comparator or hash-table callback
(`proctab_hasher`, `wd_comparator`, `name_compare`, ...) whose contract declares
`void const *` and which reads one field of what it is handed exactly as a
getter does. What separates them is not in the function: their address is taken.

**(c) Both, gated on the complete caller set.** A whole-binary run already has
the call graph `protoorder` orders by. A function whose callers are all known
direct calls (its address taken nowhere: not in code, not as a pointer-width
word of any section the image loads, not by a Mach-O chained fixup, a dynamic
relocation, an exported symbol or the entry point; and only in an x86-64 image;
sections 5 and 6 have what the first two versions of this check missed) is
"closed". The reverse vote needs that anyway, and the lone-field rule is
admitted only there. That removes every callback false positive:

| arm (296 O2 slices) | pinned perfect | credited perfect | improved / worse |
|---|---:|---:|---:|
| main | 455 | 557 | - |
| `types` (reverse vote) | 475 | 590 | 145 / 2 |
| `fields` (vote + closed lone field) | 475 | 716 | 145 / 2 pinned, 589 / 2 credited |

Of the census bucket's 166 solo functions, 97 now take a struct pointer and all
97 are perfect under #93 crediting. The rest are callbacks, byte fields and
parameters that are only forwarded.

## 3. Things measured and not shipped

* **Refusing a value the callee hands to a declared `void *` parameter.** It
  would keep gnulib `xmemdup (void const *p, size_t s)` right (3 functions), but
  a wrapper around `memcpy`/`fwrite` is exactly what `dired_outbuf` and
  `samedir_template` are, with `char *` parameters: on the 444 slices it lost 8
  perfect functions and 28 improvements to gain those 3.
* **A byte or halfword lone field.** +10 credited perfect, 2 more worse rows
  (bzip2 `copyFileName` reads a `Char *` at a fixed index); a byte at a fixed
  offset is as often a character of a buffer as a field.
* **A settle pass for every structsynth candidate.** Measured alone it moves no
  scored variable and only changes C text, so it is not part of this option.

## 4. A pre-existing instability the redo exposed

Decompiling a function twice in one session can give a different prototype the
second time. `bash` -O2 `sub_7ea70` gains an unused `unsigned int a6` on its
second decompile with every option at its default and `calleevote` off:

```
load file bash / read symbols / load addr 0x7ea70 / decompile / print C / decompile / print C
int8 sub_7ea70(unsigned long a0,int8 a1,unsigned long a2,char *a3,uint4 a4,uint4 a5)
int8 sub_7ea70(unsigned long a0,int8 a1,unsigned long a2,char *a3,uint4 a4,uint4 a5,unsigned int a6)
```

Turning off each of 24 call/arity options one at a time does not remove it
(`Funcdata::clear` still stubs `localmap->clearUnlocked`). It is not this
option's to fix. The redo guards against it instead: a redo whose parameter or
variable count differs from the first decompile's keeps the first body and drops
that function's records, so nothing is decided from a discarded body
(`same_arity`, `Ledger::forget`). Without the guard two bash functions gained a
parameter; with it none does, on any of the 12 swept binaries or the 444 slices.

## 5. What the first closed-set check missed

The first version read only raw words of non-executable sections plus dynamic
relocations. Three shapes of stored callback escaped it, and in each a
callback that is also called directly was voted on:

| shape | witness | before the fix | after |
|---|---|---|---|
| a relocatable object: the table word is 0 in the file and an `R_X86_64_64` fills it | `calleevote_ops_x86_64.o` `get_c` (also `cx3.o`, `cx5.o`) | `get_c(struct_0 *a0)`, `fwd(char *a0)` | `get_c(void *a0)`, `fwd(unsigned long a0)`, as `off` |
| an ARM Thumb address stored as `addr\|1` | crazyflie O2-noinline `cf2.elf` `sub_801d6a4`, `sub_801d6a8`, `sub_801d6b0`, `sub_801d658` | `struct_105 *a0` etc. | `void *a0`, as `off` |
| a const ops table placed in `.text` | the same four: their table sits at `0x8047604` in `.text` | | |

The check then scanned every allocated section, code included, on the pointer
grid, read each word a second time with bit 0 cleared on ARM and MIPS (section
6 replaces that: no ARM or MIPS function is closed at all), added
Mach-O chained-fixup rebase targets, and declared no function closed in a
relocatable object (whose data references the reference walk does not classify
at all) or an image whose sections hold none of its entries. A word that only
happens to equal an entry leaves that function open, the safe direction. On the
444 slices this moved nothing: every x86-64 PIE table was already a data word.

Two readability shapes the review found are fixed as well:

* **A `char *` over a wide constant store.** cf2 `sub_801ab08` printed
  `*(unsigned int *)((int)a0 + 0x34) = 0xffffffff;` as four `a0[0x34] = '\xff';`
  stores and `sub_8016760` lost its `unsigned short *`. The vote is now refused
  when the callee stores a constant wider than a byte through the value
  (`kuna_protoorder::input_refuses`); both functions print as `off` again, and
  over 21 whole binaries no new run of byte stores appears.
* **An array walked from a constant index.** dash O2-noinline `sub_7460`
  (`char **argv`) read `argv[1]` and then stepped `argv + 2` through a loop; the
  lone-field rule made that a record `{char field_0x0[8]; char *field_0x8;}`
  stepped at the record's size. A base from which an address at a constant
  offset reaches a phi is no longer a lone field, and `sub_7460` now takes the
  `char **` its callers pass. A broader guard (any use of such an address but a
  load or store, and any second access the prune dropped) was measured first
  and cost 21 functions whose record decbench credits (`file_has_acl_cache`,
  `getfilecon_cache`, `xheader_store`, ...: they pass `&f->member` to a call),
  so only the loop is refused.

What stays, and is documented as the option's effect: an array read at one
constant index and nothing else (`return a[2];`) reads exactly like a getter
and becomes a one-field record; where the callers' record is smaller than what
the function reaches through a pointer derived from the value, the access
prints as an index past the record (kmod `0x10ae0`
`*(unsigned long *)v1[1].field_0x0 = a1;`, dpkg `0x13570`), which the review
checked equivalent in 7 functions over 12 binaries; and a caller local whose `void *` came only
from the callee's old `void *` through `protoorder` keeps no pointer type when
the callee's new record does not fit what the caller does with it, so it is
declared as an integer and may merge with another local (dash `sub_eb30`).

## 6. Addresses built from two instructions, and code at address 0

The second review found two more ways a callback reached the closed set.

* **An address taken in two instructions.** The cross-reference walk reads one
  instruction at a time. AArch64 takes a function's address as `adrp`+`add`,
  MIPS (non-PIC) as `lui`+`addiu`, ARM as `movw`+`movt`, PowerPC and RISC-V
  as two halves, i386 PIC as a base register plus an offset; neither half is a
  reference to the function, so a callback also called directly counted as
  closed. AArch64 `sg.c`: `run(getc16, buf)` plus two direct calls printed
  `unsigned long sub_401018(struct_0 *a0)`; MIPS `calleevote_callback_mipsel`
  printed `unsigned int get_c(struct_0 *a0)` and gave `run` a `char *`
  parameter from its one caller. Joining the halves would mean either a
  per-architecture pair matcher in the walk or reading the constants out of
  every decompiled body, which is complete only when every function in the
  image decompiles and every switch table is read. The option is measured and
  scored on x86-64 only, so it now closes functions only in an x86-64 image,
  where every address code takes is one instruction's operand (a RIP-relative
  `lea`, an immediate). Every other architecture prints exactly as `off`.
* **A table in code loaded at address 0.** The stored-word scan skipped every
  section at address 0, meaning to skip the sections the image does not load;
  firmware loads `.text` at 0, and a const ops table in it was never read
  (`sub_20(struct_0 *a0)`, `sub_30(struct_1 *a0)`). Sections are now chosen by
  `SHF_ALLOC` on ELF, whatever their address. Exported symbols (`exports()`,
  which also covers PE and Mach-O export tables) are open as well.

Witnesses: `calleevote_callback_mipsel` and `calleevote_zero_x86_64` (fixtures,
`tests/cli` probes, `calleevote_stored_tests`). The reviewer's AArch64, MIPS and
zero-address builds (`sg`, `sf`, `se`, `si`) now print exactly as with the
option off. What is still not seen, and is documented: an address kept as an
offset from another one (a 32-bit offset table, a relative C++ vtable, a
self-relative pointer).

### The redo's cost, and a vote the rounds dropped

The review measured kmod O2-noinline at +11.3% (min of 9; +7.4% median),
outside the four budget binaries. kmod's first round decompiled 73 of its 541
functions again, and 42 of those printed exactly as before. Most were a getter
whose one-field record `protoorder` had handed to its callers, which then
passed the same record back: a statement that repeats the callee's own type.
Such a first statement is no longer made (round 1: 73 -> 49 redos).

Reading the rounds for that turned up a real loss. Each round decided against
the parameters the LAST decompile recovered, so a parameter that had taken a
vote was no longer a candidate, fell out of the next statement (which
replaces the old one), and lost its type in the next redo; a third round
sometimes stated it again and sometimes had no round left. tar O2-noinline
`0x35910` needed all three rounds; coreutils `ginstall` `need_copy` (O0) and
`copy` (O2-noinline) ended with `unsigned long` where every caller passes a
`char *`. Every round now decides against the parameters of the function's
first decompile. Over the 21 binaries the default output is byte-identical to
the build before either change; on the 444 slices the two `ginstall`
functions improve (334 up / 3 down, was 332 / 3).

What the second decompile still costs (interleaved min of 15, same build, the
option off vs default, on main with #712): fmt, ls, sort and bash -O2 +0.42,
+0.93, +0.88 and +0.63%; kmod -O2-noinline +11.3% (55 of its 541 functions
decompiled again) and cmp -O0 +7.9% (9 of 214, most of it one 217-line
function whose `char *` votes its own uses refuse), at load 2-3. On the base
before #712 the same build measured kmod +9.8% and cmp +7.7-7.9%. Whether a vote will be
refused is known only by running the pipeline with it, since the refusals read
the function's own uses in the redo, so no cheaper filter was found.


## 7. Review round 4: a frame record's `char **`, and a declared `int *`

Two defects, both from the reviewer's reductions (`cx5.c`, `cx2.c`), now the fixture
`calleevote_frame_x86_64`:

- `main` keeps `struct cfg` in its frame, first member `const char *name`, so it passes `&cfg` as
  a `char **`. `add` stores a malloc'd node through the record (`cfg->head = s`), the node took
  the pointee `char *`, and its `unsigned int`/`unsigned long` constant stores printed as twelve
  character stores (kmod -O2 `cfg_search_add` 0x9240). The `char *` wide-constant refusal only
  looked at the input itself. Now a `char **` is refused one level down: any pointer-width value
  loaded through the input or stored through it that has a wider-than-byte constant stored
  through it. Separately, a pointer-to-pointer argument that is the address of a caller's frame
  object is marked, and the callee's redo refuses it where it reaches past `[0, 8)`: `drop(&cfg)`
  reading `cfg->head` keeps its own type, `advance(&cursor)` takes `char **`. Dropping every frame
  address instead loses 17 functions whose out-parameter really is `char **` (`parse_line`,
  `next_field`, `simple_strtoul`, `quote_name_buf`). kmod's real functions use a constant the
  recovery took for the pointer (`((char *)0x1020)[a0]`, identical with the option off), so the
  walk finds no access through `a0`; any vote for a value used as a `PTRADD` index is refused,
  which keeps `cfg_search_add` and `cfg_free` `long`. `cfg_kernel_matches` reads only
  `cfg->kversion` at offset 0 and keeps `char **`.
- `int mkpipe(int *p){ return pipe(p) + p[1]; }` became `struct_1 *` under `fields`: the lone
  field path accepted any pointer. The reviewer's rule (only `void *`/untyped pointees) was
  measured: -137 functions' credited score, because find's `parser_table *entry` parameters are
  `int *` only by a `protoorder` statement from a callee that reads the enum at offset 0. The
  shipped rule keeps a pointee a declared prototype the parameter is handed to gave it (`pipe
  (int *)`), and a taken caller vote when the field fits it (a record; a `char **` read as whole
  aligned pointers); a `char *` or a `char **` read as an `int` at offset 4 (coreutils `expr`
  `getsize`) yields to the record, as in round 3. Typesweep: identical to round 3, value for
  value.

Whole corpus (31 binaries, the round-3 21 plus the reviewer's 10): 1,854 of 16,843 functions
change, 0 variable-count or error changes, 0 character-store splits (a hunk counts only when both
the character-literal stores and the statement count go up; 19 one-byte `= 0` -> `= '\0'`
spellings do not), 4 folded expressions read.

## 8. Review round 5: what a shell commits to, and a bound on the redo

**A name is not a commitment.** `committed()` has always required a record to
carry its layout (`!is_incomplete()`), but the catalog, the spec, the CLI manual
and the PR body all said "a named record, a libc shell, a `FILE *`". Under the
default `libctypes` the `FILE` shell is interned with a name, a size and no
fields, so it never passed that test and no `FILE *` was ever voted. The claim
now says what the code does: a record that carries its layout — a synthesized
`struct_N`, or one the program declares — and `FILE *` only under
`--option libctypes glibc`. Letting the shells count was the other way out and
was not taken: a shell says no more about the object than `void *` does, and the
refusals that read the pointee's members (`reaches_past_the_pointee`, the
member-fit test) have nothing to read for a type with no fields. The unit test
now asserts an incomplete `FILE` shell is not committed and a laid-out record of
the same name is.

### The redo pass, bounded

The second decompile is the option's whole cost, and on three binaries outside
the budget set it was over the project's +5%: kmod -O2-noinline +11.5%, crontab
-O2-noinline +9.0%, dpkg-divert -O2 +8.1%. Every redo was logged (its length,
its milliseconds, and whether the body it produced differs) to see where the
time goes:

| binary | redos | ms | redos that change the body | ms spent on ones that do not |
|---|---|---|---|---|
| kmod -O2-noinline | 58 | 571 | 38 | 147 |
| dpkg-divert -O2 | 30 | 316 | 25 | 179 |
| cmp -O0 | 9 | 151 | 2 | 145 (one 217-line function) |
| crontab -O2-noinline | 1 | 177 | 1 | 0 |

So a perfect oracle that skipped every redo which changes nothing would leave
kmod at about +8.5% and crontab at +9.0% — the wasted redos are not the
problem. The length of the body is: the kmod redos past 32 lines are 67% of its
redo time, and crontab's whole cost is one 189-line redo.

A function whose first decompile printed more than `CALLEE_VOTE_MAX_LINES` (32)
lines is therefore dropped from the ledger before anything is decided
(`too_long_to_vote_on`): nothing is stated about it, no round pays for it, and
`seed` and the convergence sweep have nothing to apply. kmod goes from 58 redos
to 31. The fixture `calleevote_long_x86_64` is the end-to-end witness —
`scan_short` (19 printed lines) and `scan_long` (45) do the same thing with the
pointer `use` hands them, and only the short one takes the record; with the
bound removed `scan_long` takes it too.

What the bound costs, on the 444 slices (perfect, `calleevote off` = 1,522):

| bound | perfect | credited (#93) | kmod redo time kept |
|---|---|---|---|
| none | 1,575 | 2,142 | 100% |
| 40 lines | 1,570 | 2,136 | 45% |
| **32 lines (shipped)** | **1,567** | **2,133** | **33%** |
| 25 lines | 1,555 | 2,120 | 26% |

### Measuring it on a loaded box

The campaign's speed rule is an interleaved min of 15. That estimator broke down
here: these runs share the machine with about eight other agents, the load
average sat at 30-40 throughout, and a 4-second binary's own CPU time varied
from 4.0 s to 6.4 s with it. An arm's minimum is its luckiest run, so which arm
looks faster depends on which one drew the quiet window — the same kmod build
measured +9.4% by min and +2.3% by median in one interleaved run of 15.

Both estimators are reported. At the quietest window the box gave (load 12-21
rather than 34-43) the campaign's own estimator, the interleaved min of 15,
lands inside the budget on every binary: kmod -O2-noinline +2.71%, dpkg-divert
-O2 +3.80%, cmp -O0 +1.36%, crontab -O2-noinline -15.4% (it does no redo at
all now), fmt 0.00%, sort +3.56%, ls -1.79%, bash -O2 -1.53%. The second number
is the median of the per-pair the arms alternate, and each iteration's `default` is divided by that
same iteration's `off` before the median is taken, which cancels a slow window
that hits both arms. The bound's effect is also visible without any timing at
all, in the redone lines (the table above), which is deterministic.

## 9. The redo pass measured directly, and the budget that replaced the bound

### The instrument

Every earlier number for this option came from differencing two whole-binary
wall times (`--option calleevote off` against the default). On this box that
estimator is worse than the effect it measures: section 8 records the same kmod
build at +9.4% by min-of-15 and +2.3% by median in one run.

The redo pass is a phase of its own, though — one call, after the first pass and
before the convergence sweep — so what it costs can be read off a single run
instead of inferred from two. Timing `callee_vote_rounds` and dividing by the
run's own wall clock gives a number with no second arm in it, and the pass's
line spend (`KUNA_CALLEEVOTE_TRACE=1`, `redo pass: N decompiles, S of B budget
lines spent`) is deterministic.

At the shipped flat 32-line bound the redo pass is:

| binary | redo pass | redos |
|---|---|---|
| kmod -O2-noinline | 3.4% | 36 |
| check_subid_range -O0 (shadow) | 3.7% | 7 |
| dpkg-divert -O2 | 2.0% | 21 |
| cmp -O0 | 1.3% | 7 |
| mv -O0 | 1.0% | 22 |
| fmt / ls / sort -O2 | 0.2% | 3 / 8 / 5 |
| bash -O2 | 0.1% | 8 |
| crontab -O2-noinline | 0.0% | 0 |

So the option was already inside the project's +5% on all of them, crontab's
189-line redo included (it makes none on current main). What the flat bound
actually did was refuse the same long bodies on every binary, including the ones
spending a fifth of a percent.

### Why the redo cannot be made incremental here

A redo costs exactly what the first decompile of the same function cost — median
1.01x over the 55 functions kmod redoes, ranging 0.97 to 1.05 — so there is no
overhead of the redo's own to remove. The only saving available is to stop
redoing work the vote cannot change, and the measurements say where that would
have to start:

| part of a redo (kmod, 59 redos) | share |
|---|---|
| flow follow | 12.3% |
| action pipeline before the first effective `ActionInferTypes` | 58.5% |
| the rest | 29.2% |

The vote reaches the pipeline at exactly one place, `input_vote` in
`ActionInferTypes::buildLocaltypes`, and `ActionInferTypes` does nothing until
`ActionStartTypes` has run, which is in `fullloop` after a whole `mainloop`. So
70.8% of a redo is work the vote provably cannot change, and re-running only
"type propagation and the emit tier" is not an option either: `ActionInferTypes`
sits inside the repeating `mainloop`, and the pools that rewrite ops from the
types it decides (`ptrarith`, `ptrsubundo`, `structoffset0`, `structsynth`) run
after it in the same iteration, so the body a vote produces is not a re-emission
of the first body.

Reusing that 70.8% therefore means checkpointing the `Funcdata` at the moment
`ActionStartTypes` first fires and resuming the action tree from there — a
`Funcdata` deep copy (it owns the varnode and op banks, both block graphs, the
HighVariable arena, the heritage state, the local scope and the call specs) plus
positional re-entry into the action tree, which today only knows how to run a
named root from its start. That is a change to the pipeline substrate, not to
this option, and it is the one lever that would make an unbounded redo free: it
would cut kmod's unbounded 10.5% to about 3%.

### The budget

Since a redo costs what the first decompile cost, and the first decompile's
length is known when the decision is made, the length is what a redo charges.
The flat bound becomes the floor of a budget:

* the pass may reprint `CALLEE_VOTE_BUDGET_PCT` (5) percent of the lines the
  whole first pass printed, one budget for all three rounds;
* each round admits from what it decided shortest first, ties to the lower
  address, and charges each redo the lines it reprints;
* a function printing at most `CALLEE_VOTE_MAX_LINES` (32) lines is admitted
  even once the budget is gone, so nothing the option was built for is ever
  refused;
* anything else — `Ledger::decline`: the statement just decided is withdrawn,
  no later round proposes it, and the convergence sweep has nothing to apply.

Charging the short functions to the same budget is what makes the share
self-limiting. A binary that leans on the vote spends the budget on them and
buys no long bodies: `kmod -O2-noinline` reprints 708 lines, 5.3% of its own
output, and 4 of its 40 redos are new. One that barely uses it has the room to
spare: `fmt -O2` reprints 96 of the 185 lines it is given and redoes every
candidate it has. A flat allowance on top of the old set does neither — it
would hand `kmod` the same extra percent as `fmt`, which is what pushed the
option over budget on `kmod` in the first place.

With the budget at 0 the admitted set is the old one, function for function:
only the short functions are admitted, the declined statements are withdrawn
before anything is decompiled, and nothing else reads them. That is what makes
the mechanism provably output-neutral, and it is also checked — the same
`decompile-all --json` over twelve binaries, byte-identical to the build before
it, and the 444-slice typesweep reproducing main's 1,609 perfect functions and
.3688 mean exactly.

### What it costs and what it buys

Typesweep over the 444 slices, four builds of this tree differing only in the
constant:

| build | perfect | mean | improved | worse |
|---|---|---|---|---|
| budget 0 (today's flat bound) | 1,609 | .3688 | — | — |
| **budget 5% (shipped)** | **1,615** | **.3697** | **69** | **0** |
| no bound at all | 1,618 | .3699 | 88 | 0 |
| a flat +2% on top of today (not shipped) | 1,615 | .3696 | 47 | 0 |

The flat +2% variant reaches the same perfect count by spending more: it hands
`kmod` the same extra percent as `fmt`, 6.2% of kmod's own lines against the
shipped 5.3%, for fewer functions improved.

Speed, the two builds at their defaults, interleaved, on a box running two other
workspace-test lanes (load 6-20), reported both by the campaign's min-of-N and
by the median of the per-iteration ratio (which cancels a window that is slow
for both arms):

| binary | n | min | median of ratios |
|---|---|---|---|
| fmt -O2 | 15 | -0.05% | -3.69% |
| ls -O2 | 9 | +0.48% | +0.25% |
| sort -O2 | 9 | +0.59% | -0.18% |
| bash -O2 | 5 | -1.9% | +2.0% (a lane's compile landed inside the run; its redo pass is 0.6% of the binary) |
| kmod -O2-noinline | 15 | +1.64% | -1.55% |
| dpkg-divert -O2 | 15 | +1.61% | +2.07% |
| crontab -O2-noinline | 15 | -0.17% | -0.28% |
| cmp -O0 | 15 | +1.58% | +1.96% |

and the same thing without a stopwatch: the redo pass now reprints 708 of kmod's
13,366 lines where the flat bound reprinted about 570, which is the +1% those
runs measure.
