# `protoorder` — analysis

## 1. The gap

Every prototype a caller can read about a callee is a **declared** one.
`TypeOpCall::getInputLocal` (`p5_types/coreaction_infertypes.rs`) types a call
argument from the callee's parameter only when a libc/`libctypes` entry, DWARF,
a demangled name, `parse line extern` or `--assert prototype` declared it.
Nothing states what a *recovery* found, so an internal callee tells its callers
nothing:

```console
$ gcc -O2 -s -o po po.c    # nextbyte(FILE *f) { return fgetc(f); }  twice(void *p) { return nextbyte(p) + nextbyte(p); }
$ kuna decompile-all ./po --option protoorder off | grep sub_1170
int sub_1170(unsigned long a0)
$ kuna decompile-all ./po | grep sub_1170
int sub_1170(FILE *a0)
```

## 2. The mechanism

`decompile-all` orders its loop by the call graph `kuna xrefs` publishes (Tarjan
SCCs, callees first; output still in address order). After each function the
module records the parameter types its recovery found, against the storage they
were recovered in. At a later call, argument `i` takes the callee's type for
parameter `i` when position and storage agree with the caller's own
`FuncCallSpecs::final_input_storage`. The type is a **vote** in
`Varnode::getLocalType`'s `type_order` fold, never a lock, so no call's arity can
move. The spec chapter (`docs/spec/04-calls-and-prototypes.md`, *(kuna)
`protoorder`*) is the normative description; this file is the evidence.

## 3. Round 6: what the review found, and what each became

| Finding | Change | Evidence |
|---|---|---|
| A vote at a stack object's address split the aggregate; tail's `tail_bytes` read an unwritten `st_blksize` local | Votes refused at frame addresses, on frame memory (loaded, stored, address-taken, indexed region, struct/array symbol), and on a family that contains a frame address | fixtures `protoorder_stack{array,struct}_x86_64` split on 87723a06 and stay whole now; 0 splits, 0 unwritten, 0 read-first over the corpus (§5) |
| "Less specific loses" was false: `(long)stderr`, `(long)argv`, `(long)&local` | The vote is never the type a cast is measured against (`declared_input_type_local`); votes refused on declared and global values | useradd `main` renders `sub_16da9(stderr)` and `sub_11de5("-R",argc,argv)` as off does |
| `--jobs` output silently differed from serial | stderr note; `cli.md` scopes the pool contract to `--option protoorder off`; the identity test runs both fixtures with it, a new test pins the note | `kuna-cli` tests `jobs_output_is_byte_identical_to_serial`, `jobs_notes_that_the_default_callee_first_order_is_serial_only` |
| A narrowed run paid for the call graph | Narrowed or single-target runs skip it unless the option is named; the note prints only then | `a_narrowed_run_orders_callees_first_only_when_asked`; §7 |
| Return-type and bit-typed votes contaminated callers | The result vote is dropped (§6); votes refused on float disagreement and in float-class registers | expr `sub_2dfe` keeps `size_t` parameters; cf2's `fabsf` call prints `sub_80043b8(v7)` |
| 2,166 hunks unclassified, no layout class | `corpus-diff.py`: every changed function classified, layout and use-before-def checked; `label-review.py` + `hand-labels.json` label the residue | §5 |
| Stale comments, gate staleness, PR footer and length | fixed; gates re-run on the rebased tree | PR body, gate block in `record.json` |

The review's witnesses were not the end of it. Running the new differ over 46
binaries surfaced four more shapes, each now refused: betaflight's
`uint4 [5]` record array cut short by a vote on an element read (`v11[3]`),
nuttx's `char *argv[13]` split by a vote on a value loaded through an indexed
frame pointer, freertos's `vTaskStartScheduler` merging two locals because a vote
typed a slot whose address the function passes on, and dash's `int pip[2]`
retyped `long` through a phi that carried `&pip`. Tar's regex routines rendered
`uint8 *******a0` — a pointer vote seeding the `T == ptr(T)` equation of a value
loaded through itself — until that too was refused.

## 3b. Round 7: what the review found, and what each became

| Finding | Change | Evidence |
|---|---|---|
| A zero-extended narrow load read through a wider-typed pointer printed as `(short)p[k]`, which C sign-extends: ce2 round-trips 4294941372 where the binary gives 39612; cleanflight `sub_80275ec` (`ldrh.w r0,[r5,#0x68]`) printed `sub_8028b34((short)v1[0x1a])`. The defect is on main (ce3, ce4); the votes reached it at new sites | Strict fix, no option, split out of this PR and landed first as #677 (`RuleExpandLoad`'s truncation form declines a loaded value whose type is still undefined). This branch is rebased onto it and carries none of it: `git diff origin/main` over `ruleaction_8.rs`, spec 03 and the `expandload_zext_x86_64` fixture and round-trip test is empty. The load now prints `sink(*(unsigned short *)&a0[0x1a])`, `sub_8028b34(*(unsigned short *)&v1[0x1a])` in both arms | #677's `a_zero_extended_narrow_load_round_trips_through_the_printed_c` and `docs/features/expandload/` |
| A Thumb function address typed as a data pointer rendered `&sub_80014f4[1]` (invalid C): riot 0 -> 4, cleanflight 3 -> 16, freertos 4 (labelled `literal` in round 6) | A pointer vote is refused on a constant that resolves into a code symbol (the global container lookup `ActionConstantPtr` uses) | fixture `protoorder_codeptr_thumb_le32` (hand-assembled Thumb, `.py` generator) + probe `protoorder-types-refuses-a-pointer-into-code`: `sub_8120(0x8131)`, not `&sub_8130[1]`; riot 4 -> 0, cleanflight 16 -> 4 (main's own 4); `label-review.py` now labels the shape `code-subscript` (a defect, checked before any benign label): 0 over the corpus |
| "`types` cannot change what the program does" and "a stated type never adds a cast" were false | Removed from the `phases.toml` row, `docs/cli.md` and the PR body; the row and spec 04 now list what a winning vote does change: casts at the argument (`caller((unsigned char *)0x402000,3)`), a wide store split through a guessed pointee (`*(unsigned long *)(a0 + 0xe8) = 0` -> two 4-byte stores, same bytes, different access width), and an unsigned vote's casts on signed uses (`(int)a0 >> 2`) | `docs/options.md` regenerated |
| A default raw-image `decompile-all` would take the callee-first path after the rawdiscover rebase and print `no call graph` | A raw image orders callees first only when the option is named; the no-call-graph warning prints only then | `raw_image_discovers_called_functions_beyond_its_seeds` asserts a silent default and the warning when named |
| `kuna-catalog.xml` #6 did not list `protoorder` | listed; stage baseline re-recorded for the renamed assertion | 1141/1141 |

What round 7 did not change: a load whose value is KNOWN signed or unsigned
still takes the truncation form (`(short)v1[0x11]` for `ldrsh.w r3,[r5,#0x44]`):
the value is right, the read is the whole element. Narrowing that too moves two
pinned stage assertions (`kuna-tiedphitrim.xml` #13/#14, `(int4)v2[1]`), so it
is a separate, gated choice, not a strict fix. The reviewer's instrument (new
`(short|char)p[k]` spellings, which also counts renumbered names) reads 197
over the 46 binaries main -> PR. The truncation form behind them now fires only
on a value whose type is known, so the cast it prints is the value's own type,
the one main prints for the same value through a narrow load
(`*(short *)&v1[0x19]` -> `(short)v1[0x19]`).

## 3c. Round 9: what the review found, and what each became

| Finding | Change | Evidence |
|---|---|---|
| A float vote on a value passed in a general register won even where the caller adds or compares that value as an integer. MIPS o32 (`h(int, float)` takes the float in `a1`): main prints `v1 = *(int *)(a1 + 0xc); return (int)(float)h(a0,v1) + v1 + 3;`, the branch printed `float v1` and `(int)v1 + 3`, `(int)v1 < 0x3fc00000`, `v1 == 1.5000001`, `(unsigned int)v1` -- value conversions where the machine works on bits (for bits 0x3fc00001 the source adds to 1069547524, the printed C to 4) | A float vote is refused when any member of the value's family is read or produced by an integer op, including the ones a pointer vote ignores: `INT_ADD`, `INT_SUB`, the six comparisons, `INT_CARRY`/`SCARRY`/`SBORROW`, `INT_ZEXT`/`INT_SEXT`, `POPCOUNT`/`LZCOUNT` (`integer_but_pointer_neutral`). Before, a float vote was refused only on a family that was integer-used and never float-used, and addition and comparison counted as neither. Pointer and integer votes are unchanged | fixture `protoorder_floatgpr_mipsel` (`mipsel-linux-gnu-gcc -O2 -fno-inline -fno-ipa-ra`), probes `protoorder-types-keeps-a-float-in-a-gpr-an-integer` (default: `v1 + 3`, `v1 < 0x3fc00000`, `v1 == 0x3fc00001`, `0x3fc00000 < v1` stay integer, no `(int)v1`) and `protoorder-off-float-in-a-gpr`; `a_float_in_a_general_register_keeps_its_integer_uses_round_trip` compiles the printed `g3`/`g5`/`g6`/`g9` against a bit-preserving `h` and compares them with the source (round-8 output: `11 8 7 7` against `1069547534 10 110 11`; now equal). The off arm is byte-identical to main on the fixture; the default differs from main only in `g*(int a0, ...)`, `h`'s `int k`. The same source as an ARM softfp object (`clang --target=armv7a-linux-gnueabi -mfloat-abi=softfp -mfpu=vfpv3 -O2 -c`) printed `(int)v1 + (int)(float)h(a0,v1) + 3` and `(int)v1 <= 0x3fbfffff` in round 8 and now differs from main only in the same `int a0`; soft-float (`-mfloat-abi=soft`) and RV32 `ilp32` objects never produced a float vote |
| `decompile-all --help` and `docs/cli.md` still said `--jobs N` output is identical to `--jobs 1` | Both now say the pool matches the serial run only with `--option protoorder off` on both; the `jobs.rs` determinism note says the same | -- |
| The load-widening spelling was undocumented: a typed pointee also prints a narrow load as a truncation of the wider element (`*(short *)(a0 + 0xc)` -> `(short)a0[3]`, rsyslogd's `*(char *)&V[2]` -> `(char)V[2]`) | One clause next to the store split in the `phases.toml` row and spec 04: the same value, read at a different width | `docs/options.md` regenerated |
| (nit) A same-class vote can override the caller's own evidence: an unsigned vote over signed operations (`int a0` -> `unsigned int a0` with correct `(int)a0` casts), gzip's `char *` -> `unsigned char *` over `strcmp`'s declared `char *` (its `builtin_strncpy(v,".tar",5)` becomes five byte stores) | Not changed: recorded in §9. The casts are correct; a class-level refusal is what the vote guarantees | -- |

## 3d. Round 10: what the review found, and what each became

| Finding | Change | Evidence |
|---|---|---|
| The round-9 float refusal missed truncation. On MIPS o32 and ARM softfp a caller that takes a halfword or a byte out of the word whose bits go to `h(int, float)` printed `float v1` and `*a2 = (short)((unsigned int)v1 >> 0x10)` -- a float-to-int value conversion (source 3fc0/12/c0 for bits 0x3fc01234, the printed C 0000/00/00; main prints the same statement with `unsigned int v1`). `(char)(bits >> 8)` folds to `SUBPIECE(v, 1)`, which no refusal set held | `SUBPIECE` and `PIECE` join the integer ops that refuse a float vote (`integer_but_pointer_neutral`, still neutral for a pointer vote) | fixture `protoorder_floatgpr_mipsel` rebuilt with `g20` and `g22`; probe `protoorder-types-keeps-extracted-float-bits-an-integer`; the round trip compiles them too and fails on 0ebbecb7 (`missing a2[1] = v1`, and 0000/00/00 against 3fc0/12/c0) |
| (found here, same class) A float vote on a value the caller also stores printed the store as a value conversion. Not only a general-register ABI: on x86-64, `struct S {int i; float f;}` filled with `s->i = (int)r; s->f = x;`, kuna types `s` as `int *` and the vote printed `v1[1] = (int)v3`. ARM softfp and MIPS o32 print the same (`a2[1] = (int)v1`) | A float vote is refused when the family is stored, and when it is returned, passed to another call with no stated class, or produced by a call with no declared result -- unless that register is one the calling convention assigns to floats (`s0`, `xmm0`), which is itself the float declaration. The first version refused those three outright; over the 46 binaries that took back 20 correct hard-float votes (betaflight `v1 = 1500.0` became `v1 = 0x44bb8000`), so the float-register case agrees | fixture `protoorder_floatstore_x86_64` + probe `protoorder-types-keeps-a-stored-float-bitwise`; MIPS `g24` in the round trip; the float-register case: fixture `protoorder_floatreg_armhf.o` + probe `protoorder-types-keeps-a-float-in-a-float-register` (`1000.0`; the engine without the case printed `v1 = 0x447a0000`). Self-review sets (`fi`/`fj`/`fk`/`fr`: stores, returns, other calls, call results, phi constants, x86-64 / MIPS o32 / ARM softfp, soft and hard-float): the default differs from main only in `int` parameters from `h`'s stated `int k` and in bit-exact float literals (`h(a0,1.5005555)` for 0x3fc01234) |
| The PR body's speed list mixed rounds (tar, e2fsck, betaflight were round-8 numbers) | All eight re-measured on the round-10 engine (§7) | `record.json` `speed.round10` |
| (nit) u-boot's `.bss` overlays `.rel.dyn`, so a `char *` vote on a buffer there prints a relocation byte as a string literal (`sub_6087101c("\x17")`, 5 sites in 4 functions) | Not changed: the ELF's static map says those bytes are read-only data; recorded in §9 | -- |

## 3e. Round 11: what the review found, and what each became

| Finding | Change | Evidence |
|---|---|---|
| (blocker) A pointer vote whose POINTEE is float-class was never checked, so integer bits the caller stores or copies through the pointer printed as value conversions: `*a0 = (double)(a1 + 1)` for `u->l[0] = v + 1` through a union a callee sums as doubles, `(double)((long)a0[2] * 3)`, `a0[1] = NAN` for the bits 0x7ff0000000000001. After the rebase onto structsynth-on main, a plain `*dst = *src` of `struct {int i; float f; double d;}` printed `a0->field_0x4 = (float)v2` (f=2 became 1.07e9 compiled back) | `pointee_refuses`: every load and store the caller makes through the value's family is walked (constant offsets, index strides, loop steps, copies). An access landing on a float element or member holds the value stored or loaded there to the float-vote rule one level down: refused if an integer op computes with it, if it is a NaN constant, a caller parameter in a register no float arrives in, handed on outside a float register, or (stored) loaded through another pointer that nothing reads as a float. A float vote on a NaN constant is refused too (every NaN prints `NAN`) | fixture `protoorder_floatpointee_x86_64` (a3's union callers and a6's struct copies in one binary); `a_float_pointee_keeps_the_callers_integer_stores_round_trip` compiles the six printed callers, default and `protoorder off`, and compares the bytes they store with the source: all six differ on the pre-fix engine, none now; probe `protoorder-types-keeps-a-float-pointee-bitwise` fails on the pre-fix engine and passes on main and here |
| (major) The float pointee reached the caller's own integer-register parameters: `s3(struct P *dst, long a, long b)` printed `double a1, double a2` in `rsi`/`rdx` | A float type never lands on a caller parameter in a register the convention would not give a float at that position. The model is asked (a float after an integer goes to `xmm0` on SysV, to `a1` on MIPS o32), not a register table, so MIPS o32's float-in-`a1` keeps its vote | the same fixture (`s3`, `cp3` keep `unsigned long a1, a2`); the MIPS o32 round trip still passes |
| (major) Evidence was against e1139df9; with structsynth on, a callee's partial `struct_N` reached callers as a new class: 374 `aN[k].field_`/`._i_j_` occurrences in 50 functions (`*(int **)&a0[0x33].field_0x4`, a 4-byte load printed as four byte pieces) | Rebased onto 6e4f6fa5. A composite pointee (struct, array, union) refuses an access or a derived address outside it, and an access that is not exactly one member (across two members, or part of one). A hole inside a structure is allowed: it prints as a cast offset, as main does | reviewer's 14 binaries (7,368 functions): `aN[k].field_` 21 main / 21 here, `._i_j_` 419 / 419 (0 new); `--option protoorder off` byte-identical to main on 14 of 14; nuttx `sub_80059e8` is now identical to main, `sub_8002294` keeps `*(unsigned int *)(a4 + 0xc)`. Re-measured corpus, typesweep and speed are §5-§7 |
| (minor) gzip `sub_69d0`: `builtin_strncpy(v1,".tar",5)` became five byte stores | Not changed: the vote types `v1` `unsigned char *` (its callee's parameter), and the string-store idiom only recognises `char`; the bytes are the same. Recorded in §9 | -- |
| (minor) The four rows the typesweep scores worse | Read (§6); three userland rows remain, the fourth is gone. Each is a callee's own over-typing the call site cannot contradict | `typesweep-moved.csv` |
| (nit) PR body over a screen, numbers stale | Rewritten to one screen with the numbers of this tree | -- |

## 4. What the vote may not do

Each refusal, with the witness that motivated it (`call_argument_vote`):

- **Frame address.** ce2 (`protoorder_stackarray_x86_64`): `char buf[256]`
  handed to `int *`/`short *` callees became `char [20]` + `unsigned int [61]`.
- **Frame memory, declared or global values.** tail `tail_bytes` shape
  (`protoorder_stackstruct_x86_64`): `dump_remainder`'s count recovered `void *`
  (it compares with 0x2000, the address `_DT_INIT`), and through `end_pos` that
  split `struct stat`. useradd's `stderr` and `argv`.
- **A reader or writer of another class.** `lseek`'s declared `off_t` beside
  `dump_remainder`'s `void *`; `strlen`'s declared `size_t` beside `mbslen`.
- **Float disagreement.** cf2's `fabsf` masking the sign bit is `unsigned int`
  to its own recovery and a float to its caller. A float vote is also refused
  on a value any integer op computes with, addition, comparison, truncation and
  byte extraction included: on MIPS o32 a word whose bits go to a `float`
  parameter and which is also added to as an integer printed `(int)v1 + 3`
  (§3c), and one the caller takes a halfword out of printed
  `(short)((unsigned int)v1 >> 0x10)` on a `float v1` (§3d). And it is refused
  on a value that is stored, or returned, passed to another call or produced by
  one outside a float register: x86-64 printed `v1[1] = (int)v3` for a float
  stored beside an int (§3d).
- **Loaded through itself, or deeper than two pointer levels.** tar's regex
  functions.
- **A pointee the caller's accesses disagree with.** For a float, struct, array
  or union pointee: an access or address outside it (nuttx's
  `*(int **)&a0[0x33].field_0x4`), an access across or inside members (four byte
  pieces for one word), or integer bits moved through a float member (the union
  and struct-copy callers of `protoorder_floatpointee_x86_64`, §3e).

## 5. The corpus

Round 11 re-measures everything on the tree that lands: main 6e4f6fa5
(structsynth and boolbyte on by default) against this branch, `decompile-all
--json`. The corpus is the 46 binaries of earlier rounds plus the reviewer's 14
(cleanflight, crazyflie `firmware.elf`, riot-os, nuttx -O2-noinline; numfmt, od,
sort, dd; gzip and xmlwf -O2-noinline; mirai and bzip2 -O0; x0r-usb.exe,
dexter.dll): **60 stripped binaries, 32,406 functions**, x86-64 userland at
-O0/-O2/-O2-noinline, ARM Cortex-M firmware and two PE files. Rounds 6-10
measured the same things against e1139df9; their numbers are superseded by
these and kept in `git log`.

**Off arm vs main:** `--option protoorder off` is byte-identical to a fresh
6e4f6fa5 build on **60 of 60** binaries.

**The reviewer's classes** (every function, both arms): `aN[k].field_` /
`vN[k].field_` array subscripts of a structure 152 on main, 152 here; `._i_j_`
piece assignments 4,004 and 4,004; `NAN` 55 and 55; `= (double)(` 13 and 13.
`= (float)` goes 1,312 → 1,351: per function, 199 appear and 159 disappear on
hard-float call results read from `s0` (`v3 = (float)sub_800dd06(...)`, a callee
that returns `float`, the cast moving with a temporary), one is main's own
int-to-float conversion moved into a temporary (cleanflight `sub_80275ec`), and
two of main's go away (crazyflie `sub_8029494`). The four
refinements that took those classes back to main's counts are §3e's pointee
rules: an access outside the object or across members, a step of other than one
element (grep `sub_14e90`, sort `sub_a180`), a pointer member's own pointee, and
the sibling loads of the field a vote's value came from (e2fsck `getblk`).

**Arity** (`callsite-metrics.py`): **258,249** call arguments in both arms, 0
added, 0 deleted; **57,938** `variables[]` argument rows in both arms (no
function gains a parameter); arguments that are a never-assigned local 1,179 in
both arms; call sites with fewer arguments than the callee's declaration 13,853
in both arms (`underarity.py`); string-literal arguments 30,518 → 31,001.

**Invariants** (`invariants.py`): 0 calls lost; `goto` delta 0; `return` delta
0; `;` delta +129. The one "gained call" is a string literal the default arm
recovers, `"Calling prvGetRegistersFromStack() from fault handler"`, which the
call regex reads as a call.

**Frame checks** (`corpus-diff.py`, every changed function): stack objects split
**0**, merged **2**, stack locals read and never written **0**, stack locals
whose first use is a read **0**. The two merges are nuttx `sub_8007bd0` (-O2
and -O2-noinline), two adjacent `char`s becoming the `char tmp[]` the code fills
and prints with `%s`.

**Every changed function classified** (9,669 of 32,406), by the first normalizer
under which the two bodies agree:

| class | functions | what differs |
|---|---|---|
| casts | 4,450 | C casts, NULL/0, char/negative/hex spellings, `x += y` as `x = x + y` |
| declarations | 2,754 | local numbering, declaration types, which locals share a name |
| struct-number | 1,420 | only the `struct_N` numbers: a callee-first run mints the synthesized structures in another order |
| member-reach | 432 | the same bytes through a retyped pointer or a struct member (`*(int *)(a0 + 0x28)` ↔ `a0[10]` ↔ `a0->field_0x28`) |
| literal | 360 | a number became a string or float literal |
| pointer-arith | 122 | `&p[k]` ↔ `p + k`, `p[k]` ↔ `*(p + k)` |
| merge | 17 | a parameter and a local trade which one holds a value |
| REVIEW | 114 | none of the above -- labelled below |

The residue is labelled by `label-review.py` (automatic for store width, `?:`,
parenthesisation and literal spellings) and `hand-labels.json` (each read in the
raw diff, with a reason); every one of the 114 also has the same calls, `goto`s
and `return`s in both arms:

| label | functions | what |
|---|---|---|
| spelling | 45 | the same addresses spelled for a retyped pointer (byte offsets as element indices, `p + k` ↔ `&p[k]`, casts, 0 ↔ NULL) |
| member-reach | 13 | the same bytes through a struct member or a pointer member the vote typed |
| regroup | 12 | a temporary inlined, extracted or re-used for the same value |
| store-width | 12 | a constant stored through a byte pointer as byte stores: the same bytes, a narrower access (§9) |
| literal | 11 | a small address printed as its string, or a constant at another width |
| ternary | 6 | an if/else assignment printed as `?:`, or back |
| builtin-string | 5 | constant stores ↔ `builtin_strncpy`: three gained, gzip `sub_69d0` lost (§9) |
| float-literal | 4 | a float member's bits printed as the exact literal (`0` → `0.0`, `0xbf800000` → `-1.0`) |
| wide-read | 4 | a narrow load printed as the truncated wider element (`(short)v1[0x19]`): the same value (§3b) |
| stack-merge | 2 | nuttx `char tmp[]` (an improvement) |

## 6. `type_match`

Campaign corpus, 444 slices / 10,748 functions (coreutils, grep, gzip,
diffutils, bzip2, findutils, tar, shadow at -O0/-O2/-O2-noinline; the
campaign's `x` project has no slice here), `DECBENCH_NO_CACHE=1`. A fresh
6e4f6fa5 build and this branch's final build are each scored with `typesweep
--baseline-only` and merged into one report (`typesweep-report.md`,
`typesweep-moved.csv`). Every intermediate round-11 engine (§3e's four pointee
refinements) scored the same 10,748 rows identically, so the pointee refusals
cost no row.

- PERFECT **986 → 1,105 (+119)**; aggregate 3,111.18 → 3,363.95 (+252.77)
- moved ONTO perfect **119**, moved OFF perfect **0**
- improved (not perfect) **838**, worsened **3**
- control: 6,822 functions with byte-identical `variables` score identically

`improved` below counts the rows that reach perfect too.

| project | n | perfect off → on | mean off → on | improved | worse |
|---|---|---|---|---|---|
| bzip2 | 267 | 19 → 28 | 0.3664 → 0.3884 | 14 | 0 |
| coreutils | 6,422 | 610 → 663 | 0.2813 → 0.2972 | 428 | 2 |
| diffutils | 420 | 52 → 52 | 0.4004 → 0.4096 | 19 | 0 |
| findutils | 790 | 35 → 40 | 0.1622 → 0.2121 | 149 | 0 |
| grep | 247 | 39 → 44 | 0.3870 → 0.4087 | 17 | 1 |
| gzip | 368 | 88 → 100 | 0.4858 → 0.5142 | 26 | 0 |
| shadow | 686 | 29 → 30 | 0.2897 → 0.2962 | 19 | 0 |
| tar | 1,548 | 114 → 148 | 0.2824 → 0.3353 | 285 | 0 |

Firmware corpus (nuttx, mirai, betaflight, freertos at -O2): PERFECT 208 →
217, aggregate 506.93 → 539.88 (+32.95), moved off perfect 0, improved
112, worsened 4.

A type the vote is right about can still score nothing: a `struct_N *` never
equals a program-defined ground-truth name, and the campaign's decision to
credit any struct pointer against a struct pointer is not in decbench yet.

**The result vote** was measured and dropped: stating the callee's recovered
return type is worth one perfect function over the 444 slices (1,075 vs 1,074)
and accounts for four of the eight worsened rows at that point, the
`memchr_kwset` family among them. **`void *` votes** were measured and kept
(+11 perfect).

**Every worsened row, read:**

| slice | function | off → on | why |
|---|---|---|---|
| coreutils O0 tail | `file_lines` | `long end_pos` → `void *` | callee over-typed: `dump_remainder` reads BUFSIZ 0x2000 as the address `_DT_INIT` and recovers its count as `void *` |
| coreutils O0 tail | `tail_forever` | a `long` local → `void *` | the same callee |
| grep O2-noinline | `buf_has_encoding_errors` | `char *` → `unsigned long *` | callee over-typed: a word-at-a-time scan reads the buffer 8 bytes at a time |
| freertos O2 | `vQueueWaitForMessageRestricted` | `TickType_t` → `int *` | callee over-typed |
| freertos O2 | `printi` | `int width` → `unsigned char *` | callee over-typed: `prints` recovers its width as a pointer |
| nuttx O2 | `nxsig_timeout` | `wdparm_t` → `int *` | the ground truth is an integer the program casts to a pointer |
| betaflight O2 | `rpmFilterApply` | `int axis` slot → `float` | kuna orders the `s0` float parameter first; the vote types that float correctly and the metric aligns it with the `int` |

## 7. Speed

Main (a fresh `origin/main` 6e4f6fa5 build, structsynth on) against this
branch's final engine, whole-binary `decompile-all --json`, the two alternating
inside each iteration (order swapped every iteration), min and median of 15,
three slice groups side by side (box load 3-14). `scripts.pipeline.timeit`
cannot measure this option: it times `kuna decompile`, where the option is inert.

| slice | main min / median | this branch min / median | delta min / median |
|---|---|---|---|
| fmt -O2 (46 KiB) | 4,134 / 4,177 ms | 4,158 / 4,203 ms | +0.6% / +0.6% |
| kmod -O2 (158 KiB) | 7,167 / 7,367 ms | 7,321 / 7,438 ms | +2.1% / +1.0% |
| ls -O2 (146 KiB) | 13,669 / 15,076 ms | 14,048 / 15,617 ms | +2.8% / +3.6% |
| libselinux -O2 (176 KiB) | 6,499 / 6,634 ms | 6,680 / 6,784 ms | +2.8% / +2.3% |
| ip -O2 (678 KiB) | 27,375 / 29,249 ms | 27,967 / 28,836 ms | +2.2% / -1.4% |
| e2fsck -O2 (706 KiB) | 26,724 / 27,313 ms | 27,248 / 27,878 ms | +2.0% / +2.1% |
| tar -O2 (517 KiB) | 45,384 / 47,024 ms | 45,838 / 47,939 ms | +1.0% / +1.9% |
| betaflight -O2 (520 KiB, ARM) | 54,652 / 55,689 ms | 55,697 / 56,625 ms | +1.9% / +1.7% |

Every slice is inside the +5% budget on the minimum; the worst are ls and
libselinux at +2.8%. ip, e2fsck, tar and betaflight are the binaries over
500 KiB. The pointee check walks the loads and stores through a pointer vote's
value only for a float or composite pointee, and stops at 512 steps.

Rounds 8-10 measured the branch against e1139df9 (all within +5% on the
minimum after re-runs); their tables are in `record.json` under `speed.round8`,
`round9` and `round10`, and round 6 measured off against on inside one build
(within +2.4%).

## 8. `lock`, kept opt-in

Unchanged from round 5. It also states the callee's arity, which collapses a
caller that over-recovered, but fabricates parameters where the callee's own
recovery over-counted; `type_match` cannot see a fabricated parameter at all.

| `lock`, 22 binaries / 14,142 functions | 20 x86-64 userland | 2 ARM Cortex-M |
|---|---|---|
| call arguments gained | +818 at 418 sites | +2,371 at 1,834 sites |
| call arguments deleted | 28 at 21 sites | 881 at 518 sites |
| functions gaining a `variables[]` arg row | **155 of 7,553** | **630 of 6,589** |

## 9. What it cannot do

- What is stated is a **caller-side fact**: the callee has already been
  decompiled when its types are recorded.
- `kuna decompile` forks one `decomp_dbg` per function and cannot see what
  another function stated, so `decompile` and `decompile-all` may disagree about
  a call's argument types (angr has the same property).
- A callee whose own recovery is wrong about a parameter states that type, and
  the caller takes it wherever nothing at the call site contradicts it (§6).
- `decompile-project` and `decompile-graph` are not callee-first; a `--jobs` pool
  states nothing and says so.
- A winning vote changes spellings, not only declarations: casts at the typed
  argument, a signed use cast back after an unsigned vote (`(int)a0 >> 2`), and
  a byte pointee guessed for a pointer splitting one wide constant store through
  it into byte stores of the same bytes (the 12 `store-width` rows of §5;
  betaflight `sub_8041e24` stores 0x3f800000 through a `uint1 *` as four bytes).
  Memory ends up the same; the access width does not, which matters on
  memory-mapped I/O. A structure or float pointee never splits a store: an access
  across its members refuses the vote (§3e). A pointer typed by a vote also lets `RuleExpandLoad` print a
  known-typed narrow load as the truncated element (`(short)v1[0x11]`): the same
  value, a wider read, as main prints for any typed pointer (§3b).
- A vote of the same class can still override the caller's own evidence: an
  unsigned vote makes a parameter the caller shifts and divides as signed
  `unsigned int`, with `(int)a0` casts keeping those uses correct, and gzip's
  `char *` becomes `unsigned char *` beside `strcmp`'s declared `char *`, which
  turns its `builtin_strncpy(v,".tar",5)` into five byte stores. The refusals
  are by class (pointer, integer, float), not by the finer type.
- A float vote is refused on any value the function also stores, even where
  the store lands in float memory (`*a2 = v1` through what becomes a `float *`):
  the vote cannot see which type the store's pointer will settle on, and an
  integer one makes the printer convert. Main's typing is kept there.
- The pointee rules refuse a composite vote whenever the caller indexes an array
  member inside the structure (`p->buf[i]`), because the index strides by the
  element rather than the structure; that costs the vote, and the function
  prints as on main.
- On u-boot the `.bss` overlays `.rel.dyn`, which the ELF marks allocated and
  read-only, so a `char *` vote on a buffer there prints the relocation byte it
  overlays as a string literal (`sub_6087101c("\x17")` where main prints
  `sub_6087101c(0x6089ff14)`; 5 sites in 4 functions). That is what the file's
  static map says; x86 `.bss` does not overlay anything and prints the symbol.

## 10. Found here, not fixed here

A main defect, out of scope for this PR and handed to the `truncarg` lane: a
16-bit truncation folded into a zero-extended call argument is lost. Compiled
with `gcc -O2`,

```c
unsigned acc;
__attribute__((noinline)) void sink(unsigned x) { acc = acc * 31u + x / 3u; }
__attribute__((noinline)) void f(unsigned v) { sink((unsigned short)(v * 3u)); }
```

`f` is `lea (%rdi,%rdi,2),%edi; movzwl %di,%edi; jmp sink`, and main and this
branch both print

```c
void f(short a0)
{
  sink(a0 * 3); // tail-call
}
```

which passes the untruncated product (0x16665 for `v = 0x7777`, where the binary
passes 0x6665) and sign-extends the parameter. `protoorder` neither causes nor
changes it: the off arm, main and the default arm print the same `f`. The
`truncarg` lane has since fixed it on main (#678, after this branch's base).

A second main defect, out of scope and unchanged by this PR: on x86-64 a
`float h(int k, float x)` is recovered with the float first (`float h(float
a0,int a1)`, `xmm0` before `edi`), and its callers print one argument,
`v2 = (float)h(v3)`, and leave the pointer they keep in `rdx` as an
uninitialized local (`protoorder_floatstore_x86_64`'s `fill`, where main, the
off arm and the default all print it). The round-10 fixture was chosen so its
probe does not depend on that spelling.
