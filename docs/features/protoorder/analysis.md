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
| (major) Evidence was against e1139df9; with structsynth on, a callee's partial `struct_N` reached callers as a new class: 374 `aN[k].field_`/`._i_j_` occurrences in 50 functions (`*(int **)&a0[0x33].field_0x4`, a 4-byte load printed as four byte pieces) | Rebased onto 6e4f6fa5, then onto d96e3408 (charbyte). A composite pointee (struct, array, union) refuses an access or a derived address outside it, and an access that is not exactly one member (across two members, or part of one). A hole inside a structure is allowed: it prints as a cast offset, as main does | reviewer's 14 binaries (7,368 functions): `aN[k].field_` 21 main / 21 here, `._i_j_` 419 / 419 (0 new); `--option protoorder off` byte-identical to main on 14 of 14; nuttx `sub_80059e8` is now identical to main, `sub_8002294` keeps `*(unsigned int *)(a4 + 0xc)`. Re-measured corpus, typesweep and speed are §5-§7 |
| (found here, same class) The 60-binary re-measure found four more `vN[k].field_` subscripts the first pointee rule let through: grep `sub_14e90` stepping by 0x48 over a 16-byte `struct_N` (`a0 = (unsigned long *)&a0[4].field_0x8`), sort `sub_a180` walking 32-byte records through a 16-byte view (`v18[-2].field_0x8`), and e2fsck `getblk` / `e2fsck_setup_icount`, where a vote on one load of `a0->field_0x0` typed the field and so another load of it (`*(int8 *)&v2[4].field_0x4c[0x1c]`) | A composite pointee needs a step of exactly one element; a pointer member loaded or stored through the pointee is checked for its own pointee, two levels deep; and a value loaded from memory is checked together with every other load of the same field (same base, offset and width) | 60 binaries: array subscripts of a structure 152 on main and 152 here (157 before these); `type_match` rows unchanged by each step |
| (minor) gzip `sub_69d0`: `builtin_strncpy(v1,".tar",5)` became five byte stores | Not changed: the vote types `v1` `unsigned char *` (its callee's parameter), and the string-store idiom only recognises `char`; the bytes are the same. Recorded in §9 | -- |
| (minor) The four rows the typesweep scores worse | Read (§6); three userland rows remain, the fourth is gone. Each is a callee's own over-typing the call site cannot contradict | `typesweep-moved.csv` |
| (nit) PR body over a screen, numbers stale | Rewritten to one screen with the numbers of this tree | -- |

## 3f. Round 12: the narrow primitive pointee (found by the `ptrfromuse` evaluation)

| Finding | Change | Evidence |
|---|---|---|
| A pointer vote whose pointee is a non-character integer narrower than a constant the caller stores through the pointer split the store into one store per element, because `SplitDatatype` reads the pointer as an array of its pointee. At the default: gzip `sub_6e90`'s `".tar"` as five byte stores, betaflight `sub_8041e24`'s `1.0f` as four, e2fsck `sub_3a380`'s 8-byte zero as two 4-byte stores. With `--option ptrfromuse void` (the `ptrfromuse` branch evaluating that value as a default found it): bzip2 `sub_5a10` printed `*(unsigned long *)((long)a0 + 0x5c) = 0x100` as eight byte stores (`a0[0x5c] = 0; a0[0x5d] = 1; ...`), and the same in bzip2 `sub_60e0` and tar `sub_133d0`, because with `ptrfromuse` on a callee's `long` parameter becomes a pointer, the two votes agree in class, and the more specific `unsigned char *` wins. Ten of the twelve `store-width` rows round 11 listed as a residual in §5 were this, and gzip `sub_69d0`'s lost `builtin_strncpy` too | `pointee_refuses`: a vote whose pointee is a non-character integer, `bool` or pointer is refused when the caller stores a constant wider than the pointee through the pointer at a fixed place or at a record stride. A constant stored at a stride of its own width (a buffer filled a word at a time) was allowed (round 13 dropped that, §3g), as are a computed value stored wider (it prints as a cast) and a character pointee (its byte stores are what the string-copy idiom prints as `builtin_strncpy`). Taken from the `ptrfromuse` branch's commit without that branch's default flip | fixture `protoorder_narrowvote_x86_64` (`fill` stores `"ustar  "` through the buffer it hands a byte-reading `peek`): the pre-fix engine prints `int fill(unsigned char *a0,int a1)` and eight byte stores at the default and with `ptrfromuse void`, the fixed one `*(unsigned long *)(a0 + 0x10) = 0x2020726174737575;` in both, as main does. CLI test `a_byte_pointee_vote_keeps_the_callers_wide_stores` (both arms) and probe `protoorder-types-keeps-a-wide-store-whole` fail on the pre-fix engine. bzip2 `sub_5a10` and `sub_60e0` under `ptrfromuse void` are now identical to main; tar `sub_133d0` and gzip `sub_6e90` differ from main only in the casts a winning vote adds |
| What the refusal changes | Nothing else: 0 of 10,748 `type_match` rows move, in either `ptrfromuse` arm, and 0 of 3,451 firmware rows | 60 binaries at the default: 26 of 32,406 functions change against the pre-fix engine -- 11 splits undone, 9 callers whose argument cast follows the callee's new parameter type, and 6 where a refused vote gives back main's own type (crazyflie `sub_801ffc8` `char`, ip `sub_3e220`/`sub_3eee0` `int *` and their two callers' locals). `ptrfromuse void` over 13 binaries: 14 change, 9 splits undone and 5 caller casts. Call arguments, argument rows and never-assigned arguments unchanged (§5) |

## 3g. Round 13: two ways past the narrow-pointee refusal

| Finding | Change | Evidence |
|---|---|---|
| §3f allowed a constant stored at a stride of its own width, as a buffer filled a word at a time, and that printed the same per-element split: a loop storing an eight-byte constant per word printed eight byte stores per iteration under `unsigned char *a0` where main prints `unsigned long *a0` and `*v2 = 0x102030405060708;`, and betaflight `sub_801379c`'s sector fill `*(unsigned int *)(a1 + v1 * 4) = 0xefbeadde` printed as four byte stores per word | The exemption is gone: a non-character integer, `bool` or pointer pointee refuses any constant the caller stores through the pointer wider than itself | fixture `protoorder_widefill_x86_64` `fill_words`: the round-12 engine prints the split at the default and with `ptrfromuse void`; the fixed one prints main's function byte for byte in both. betaflight `sub_801379c` is identical to main in the 60-binary corpus, at the default and with `ptrfromuse void` its fill is one word store again |
| The narrow primitive rule read "the walk gave up" (more than 511 steps over the caller's derived addresses) as "no wide store", so a caller with that many addresses took the vote unchecked: 520 eight-byte constants stored through the buffer a byte-reading callee gets printed as 4,160 byte stores under `unsigned char *a0` | That rule refuses when the walk gives up, as the float and composite rules already did | fixture `fill_many`: the round-12 engine prints the split in both arms; the fixed one prints main's function byte for byte. CLI test `a_byte_pointee_vote_keeps_word_fills_and_long_callers_whole` (both functions, both arms) and probe `protoorder-types-keeps-a-word-fill-whole` fail on the round-12 engine |
| What the two changes do elsewhere | Nothing else | 60 binaries: 4 of 32,406 functions change against the round-12 engine, the same 4 in both arms (§5). `type_match`: 0 of 10,748 rows move in either arm; of 3,451 firmware rows one does, betaflight `read_data_sector` (`sub_801379c`), back to main's 0.143 from 0.286: the byte-pointer parameter the split had come with matched its `uint8_t *sector`. Call arguments, argument rows and never-assigned arguments unchanged |
| Not changed: a vote with a wider pointee over the caller's own byte reads (stat O2 `sub_df20`'s `char *a0` becomes `int *a0`, printed `v1 = (char)*a0;`) | Kept | The wide-read class of §9: the same value on a little-endian target, read at a different width. It is the same class main prints for any typed pointer, and no store is split by it |

## 3h. Round 13: the rebase onto #675 (structsynth shares one layout)

| Finding | Change | Evidence |
|---|---|---|
| #675 decompiles once more, after an address-order batch, the functions that name a structure a later, larger one superseded (`converge_synthesized_structs`). The callee-first path decompiles each target on its own and never reached that sweep, so under the default a function kept the superseded name: `tests/cli/structsynth-sweep-mints-no-third-name` failed on the rebased branch (`long fb(struct_0 *a0)` where main prints `struct_1`, `fc`'s) | The callee-first loop ends with the same sweep, in plan order, each function stating its recovered types again where the plan let it; not under `lock` (§9) | CLI test `callee_first_runs_the_structsynth_convergence_sweep` (the three readers make no direct calls, so the default document must equal the `protoorder off` one) fails on the rebased engine without the sweep and passes with it; the probe passes. Re-measured on a fresh 3831d703 build: `--option protoorder off` byte-identical to main on 60 of 60 binaries, 9,654 functions change (9,652 on a3bf6a15) with the same 100 REVIEW functions under the same labels and the same arity counts (§5); `type_match` identical in both arms and on firmware (§6) |

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
  and struct-copy callers of `protoorder_floatpointee_x86_64`, §3e). For a
  non-character integer, `bool` or pointer pointee: any constant the caller stores
  wider than it (`protoorder_narrowvote_x86_64`, §3f; `protoorder_widefill_x86_64`,
  §3g). Every pointee rule refuses when the walk over the caller's addresses
  gives up (§3g).

## 5. The corpus

Round 11 re-measures everything on the tree that lands: main d96e3408
(structsynth, boolbyte and charbyte on by default) against this branch, `decompile-all
--json`. The corpus is the 46 binaries of earlier rounds plus the reviewer's 14
(cleanflight, crazyflie `firmware.elf`, riot-os, nuttx -O2-noinline; numfmt, od,
sort, dd; gzip and xmlwf -O2-noinline; mirai and bzip2 -O0; x0r-usb.exe,
dexter.dll): **60 stripped binaries, 32,406 functions**, x86-64 userland at
-O0/-O2/-O2-noinline, ARM Cortex-M firmware and two PE files. Rounds 6-10
measured the same things against e1139df9; their numbers are superseded by
these and kept in `git log`. Round 12 re-ran the default arm on the same 60
binaries with §3f's refusal: 26 functions differ from the round-11 engine.
Round 13 re-ran all 60 in seven arms against a fresh build of main a3bf6a15 (the
round-12 engine, this branch at the default, off and with `ptrfromuse void`, and
main with and without it), and measured again after the rebase onto 3831d703
(#675, #686) with §3h's sweep. The tables below are the last measurement, after
the rebase onto e76fd044 (#673, formatstring on by default), against a fresh
e76fd044 build: §3g changes 4 functions against the round-12 engine
in either arm (crond `sub_6b7b`, the walk-limit case, whose two argument casts
go back to main's spelling; betaflight `sub_801379c`, its sector fill whole again
and the function identical to main, and the two callers its vote had typed,
`sub_8013818` now identical to main and `sub_80130a4` keeping only integer
votes). Every table below is the round-13 tree's.

**Off arm vs main:** `--option protoorder off` is byte-identical to a fresh
e76fd044 build on **60 of 60** binaries (and to 3831d703 and a3bf6a15 before those rebases, d96e3408 in round 11), and the round-12 engine's off arm on
the 10 of them re-run (fmt, ls, kmod, libselinux, betaflight, nuttx, dash, grep
-O0, zlib, crazyflie), and again on those 10 after the rebase onto d6c5862f,
where their call arguments (67,928), argument rows (18,930) and never-assigned
arguments (468) are also the same in both arms. (The same held against 6e4f6fa5 before
the charbyte rebase, and every number below is within a few functions of that
measurement.)

**The reviewer's classes** (every function, both arms): `aN[k].field_` /
`vN[k].field_` array subscripts of a structure 152 on main, 152 here; `._i_j_`
piece assignments 4,048 and 4,048; `NAN` 55 and 55; `= (double)(` 14 and 14.
`= (float)` goes 1,306 → 1,345 (1,312 → 1,351 on d96e3408): per function, 199 appear and 159 disappear on
hard-float call results read from `s0` (`v3 = (float)sub_800dd06(...)`, a callee
that returns `float`, the cast moving with a temporary), one is main's own
int-to-float conversion moved into a temporary (cleanflight `sub_80275ec`), and
two of main's go away (crazyflie `sub_8029494`). The four
refinements that took those classes back to main's counts are §3e's pointee
rules: an access outside the object or across members, a step of other than one
element (grep `sub_14e90`, sort `sub_a180`), a pointer member's own pointee, and
the sibling loads of the field a vote's value came from (e2fsck `getblk`).

**Arity** (`callsite-metrics.py`): **258,449** call arguments in both arms, 0
added, 0 deleted; **57,931** `variables[]` argument rows in both arms (no
function gains a parameter); arguments that are a never-assigned local 1,186 in
both arms; call sites with fewer arguments than the callee's declaration 13,695
in both arms (`underarity.py`); string-literal arguments 30,532 → 31,063. (#673's
format-string arity moved main's own counts from 258,249 / 57,938 / 1,179 /
13,853; both arms moved together.)

**Invariants** (`invariants.py`): 0 calls lost; `goto` delta 0; `return` delta
0; `;` delta +30 (+98 on 3831d703, +127 before §3f undid the byte stores). The one "gained call" is a string literal the default arm
recovers, `"Calling prvGetRegistersFromStack() from fault handler"`, which the
call regex reads as a call.

**Frame checks** (`corpus-diff.py`, every changed function): stack objects split
**0**, merged **2**, stack locals read and never written **0**, stack locals
whose first use is a read **0**. The two merges are nuttx `sub_8007bd0` (-O2
and -O2-noinline), two adjacent `char`s becoming the `char tmp[]` the code fills
and prints with `%s`.

**Every changed function classified** (9,670 of 32,406), by the first normalizer
under which the two bodies agree:

| class | functions | what differs |
|---|---|---|
| casts | 4,441 | C casts, NULL/0, char/negative/hex spellings, `x += y` as `x = x + y` |
| declarations | 2,734 | local numbering, declaration types, which locals share a name |
| struct-number | 1,426 | only the `struct_N` numbers: a callee-first run mints the synthesized structures in another order |
| member-reach | 435 | the same bytes through a retyped pointer or a struct member (`*(int *)(a0 + 0x28)` ↔ `a0[10]` ↔ `a0->field_0x28`) |
| literal | 398 | a number became a string or float literal |
| pointer-arith | 119 | `&p[k]` ↔ `p + k`, `p[k]` ↔ `*(p + k)` |
| merge | 17 | a parameter and a local trade which one holds a value |
| REVIEW | 100 | none of the above -- labelled below |

The residue is labelled by `label-review.py` (automatic for store width, `?:`,
parenthesisation and literal spellings) and `hand-labels.json` (each read in the
raw diff, with a reason); every one of the 100 also has the same calls, `goto`s
and `return`s in both arms:

| label | functions | what |
|---|---|---|
| spelling | 45 | the same addresses spelled for a retyped pointer (byte offsets as element indices, `p + k` ↔ `&p[k]`, casts, 0 ↔ NULL) |
| member-reach | 14 | the same bytes through a struct member or a pointer member the vote typed |
| regroup | 12 | a temporary inlined, extracted or re-used for the same value |
| store-width | 1 | libselinux `sub_1d4c0`: a constant stored through a `char *` as `'\0'` stores, the same bytes at a narrower access, the character-pointee shape §3f allows (§9); betaflight's word fill, the other one before §3g, is identical to main now |
| literal | 9 | a small address printed as its string, or a constant at another width |
| ternary | 5 | an if/else assignment printed as `?:`, or back |
| builtin-string | 4 | constant stores ↔ `builtin_strncpy` or `char` stores: two strings gained; a 2-byte constant printed as two `char` stores in ginstall `sub_dd10` and tar `sub_3f9b0` (a `char *` vote, §9) |
| float-literal | 4 | a float member's bits printed as the exact literal (`0` → `0.0`, `0xbf800000` → `-1.0`) |
| wide-read | 4 | a narrow load printed as the truncated wider element (`(short)v1[0x19]`): the same value (§3b) |
| stack-merge | 2 | nuttx `char tmp[]` (an improvement) |

## 6. `type_match`

Campaign corpus, 444 slices / 10,748 functions (coreutils, grep, gzip,
diffutils, bzip2, findutils, tar, shadow at -O0/-O2/-O2-noinline; the
campaign's `x` project has no slice here), `DECBENCH_NO_CACHE=1`. A fresh
d96e3408 build and this branch's final build are each scored with `typesweep
--baseline-only` and merged into one report (`typesweep-report.md`,
`typesweep-moved.csv`). Every intermediate round-11 engine (§3e's four pointee
refinements) scored the same 10,748 rows identically, so the pointee refusals
cost no row. Round 12's refusal (§3f) moves none either, measured both at the
default and with `--option ptrfromuse void` (1,309 perfect with and without it;
the firmware rows too, 217 and 217), against the pre-§3f engine in the same
sweep. After the rebase onto d6c5862f (`foldcallretphi` on by default), main
and this branch score exactly as above: main 986 perfect (0 rows move from
d96e3408), this branch 1,105 at the default and 1,309 with `ptrfromuse void`,
firmware 208 → 217. Round 13 re-scored everything on a3bf6a15 (#687 moves two
of main's rows onto perfect), with the round-12 engine in the same sweep: §3g
moves none of the 10,748 rows in either arm, and one firmware row back to main's
score (§3g). The numbers below are that sweep's: a fresh a3bf6a15 build against
this branch. After the rebase onto 3831d703 (#675, structsynth sharing one layout
across functions, and #686) and §3h's sweep, a fresh 3831d703 build and this
branch score exactly the same rows in both `ptrfromuse` arms and on firmware:
#675 and #686 move no row on main, the rebase none on this branch, and §3h's
sweep none (it rewrites the variables of 144 functions, all scored the same).
After the rebase onto e76fd044 (#673, formatstring on by default, which moves
38 of main's rows onto perfect), the numbers below are that base's: a fresh
e76fd044 build against this branch.

- PERFECT **1,026 → 1,147 (+121)**; aggregate 3,157.00 → 3,413.35 (+256.35)
  (on a3bf6a15 and 3831d703: 988 → 1,107, 838 improved, 3 worse)
- moved ONTO perfect **121**, moved OFF perfect **0**
- improved (not perfect) **827**, worsened **2**
- control: 6,837 functions with byte-identical `variables` score identically
- with `--option ptrfromuse void` in both arms: 1,026 → 1,351, none off perfect,
  827 improved, 17 worse (988 → 1,311 and 18 on the earlier bases). Twelve of those are the ls, dir and vdir
  `*_df_extension` comparators, which main scores 0 at the default and a third
  with `ptrfromuse void` (`void *`), and this branch 0 in both: the vote gives
  them their callee `sub_71f0`'s own `undefined8 *` (§10); none scores worse than
  this branch's own default

`improved` below counts the rows that reach perfect too.

| project | n | perfect off → on | mean off → on | improved | worse |
|---|---|---|---|---|---|
| bzip2 | 267 | 22 → 31 | 0.3857 → 0.4077 | 14 | 0 |
| coreutils | 6,422 | 620 → 675 | 0.2819 → 0.2978 | 429 | 1 |
| diffutils | 420 | 58 → 58 | 0.4192 → 0.4287 | 19 | 0 |
| findutils | 790 | 36 → 41 | 0.1690 → 0.2199 | 153 | 0 |
| grep | 247 | 47 → 51 | 0.4131 → 0.4328 | 16 | 1 |
| gzip | 368 | 90 → 102 | 0.4917 → 0.5202 | 26 | 0 |
| shadow | 686 | 30 → 31 | 0.2940 → 0.2988 | 11 | 0 |
| tar | 1,548 | 123 → 158 | 0.2902 → 0.3457 | 280 | 0 |

Firmware corpus (nuttx, mirai, betaflight, freertos at -O2): PERFECT 207 →
216, aggregate 506.08 → 538.88 (+32.81), moved off perfect 0, improved
111, worsened 4 (the same four as round 12, read below), on e76fd044 as on
a3bf6a15 and 3831d703.

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
| coreutils O0 tail | `file_lines` | `long end_pos` → `void *` | callee over-typed: `dump_remainder` reads BUFSIZ 0x2000 as the address `_DT_INIT` and recovers its count as `void *`; on e76fd044 this branch scores it as main does (0.636), so it is no longer a worse row |
| coreutils O0 tail | `tail_forever` | a `long` local → `void *` | the same callee |
| grep O2-noinline | `buf_has_encoding_errors` | `char *` → `unsigned long *` | callee over-typed: a word-at-a-time scan reads the buffer 8 bytes at a time |
| freertos O2 | `vQueueWaitForMessageRestricted` | `TickType_t` → `int *` | callee over-typed |
| freertos O2 | `printi` | `int width` → `unsigned char *` | callee over-typed: `prints` recovers its width as a pointer |
| nuttx O2 | `nxsig_timeout` | `wdparm_t` → `int *` | the ground truth is an integer the program casts to a pointer |
| betaflight O2 | `rpmFilterApply` | `int axis` slot → `float` | kuna orders the `s0` float parameter first; the vote types that float correctly and the metric aligns it with the `int` |

## 7. Speed

Main (a fresh d96e3408 build; 75f832e3 differs only in docs) against this
branch's round-12 engine, and the round-11 engine (before §3f) as a third arm,
whole-binary `decompile-all --json`, the three rotating inside each iteration
(order rotated and reversed every iteration), min and median of 15, three slice
groups side by side (box load 9-41, other lanes running). `scripts.pipeline.timeit`
cannot measure this option: it times `kuna decompile`, where the option is inert.

| slice | main min / median | this branch min / median | delta min / median | vs round 11, delta min |
|---|---|---|---|---|
| fmt -O2 (46 KiB) | 4,451 / 4,649 ms | 4,221 / 4,529 ms | -5.2% / -2.6% | -0.9% |
| kmod -O2 (158 KiB) | 7,316 / 8,247 ms | 7,495 / 8,320 ms | +2.4% / +0.9% | -1.6% |
| ls -O2 (146 KiB) | 14,126 / 15,553 ms | 14,218 / 15,662 ms | +0.7% / +0.7% | -0.8% |
| libselinux -O2 (176 KiB) | 6,808 / 7,482 ms | 6,807 / 7,633 ms | -0.0% / +2.0% | -1.8% |
| ip -O2 (678 KiB) | 31,228 / 34,533 ms | 30,906 / 34,034 ms | -1.0% / -1.4% | -0.1% |
| e2fsck -O2 (706 KiB) | 26,856 / 27,957 ms | 27,233 / 28,623 ms | +1.4% / +2.4% | -0.4% |
| tar -O2 (517 KiB) | 44,911 / 49,683 ms | 45,645 / 51,681 ms | +1.6% / +4.0% | -4.7% |
| betaflight -O2 (520 KiB, ARM) | 55,045 / 57,491 ms | 55,802 / 59,080 ms | +1.4% / +2.8% | -3.1% |

Every slice is inside the +5% budget on the minimum; the worst is kmod at +2.4%.
ip is a re-run (load 9 at its start, 41 at its end): the first run gave -0.4% against main
and +6.6% against the round-11 arm, whose two fastest samples were 2 s under
any other sample of that run; the re-run gives -0.1% against it. §3f adds a walk
of the loads and stores through a pointer vote's value for a non-character
primitive pointee, as §3e does for a float or composite one; the walk stops at
512 steps. Round 11's table (the same comparison without §3f, load 2-9) is in
`record.json` under `speed.round11_on_d96e3408`.

After the rebase onto d6c5862f (`foldcallretphi` on), a spot check against a
d6c5862f build, with a second main arm as the noise floor: kmod +1.1%,
libselinux +2.7% and fmt +3.9% on the minimum (main against itself: -2.9%,
+3.0%, -0.1%); fmt re-run three times, +0.4%, -0.1% and -0.4%.

Round 13 (§3g) against a fresh a3bf6a15 build, interleaved min-of-15: fmt
+1.0%, kmod +2.2%, ls +0.3%, libselinux +4.1%, tar +2.8%. libselinux re-run
beside a second main arm: +2.9% against main, with main against itself at
+2.2%. §3g only refuses earlier; it adds no walk.

After the rebase onto 3831d703 and §3h's sweep, against a fresh 3831d703 build:
fmt +1.9%, kmod +1.5%, ls +1.9%, libselinux -0.1%, tar +3.8%. tar re-run
beside a second main arm: +3.6% against main and +4.0% against the second arm,
main against itself -0.4%. The sweep re-decompiles the functions that name a
superseded structure, as main's address-order run does; a vote that spreads a
structure pointer to a caller gives it more such functions to redo.

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
- `lock` runs no `structsynth` convergence sweep (§3h): a prototype it parked is
  declared, so a second decompile of that function would read its own first
  answer back, and a function can keep a superseded `struct_N` that the default
  and an address-order run replace.
- A winning vote changes spellings, not only declarations: casts at the typed
  argument, a signed use cast back after an unsigned vote (`(int)a0 >> 2`), and,
  in the one shape §3f allows, a character pointee splitting one wide constant
  store through it into character stores of the same bytes: libselinux
  `sub_1d4c0` prints `*(unsigned long *)(a0 + 0xa0) = 0` as eight `'\0'` stores
  on a structure the callee reads bytes of, and ginstall and tar print a 2-byte
  constant as two `char`s. Memory ends up the same; the access width does not,
  which matters on memory-mapped I/O. Any other non-character primitive pointee
  narrower than a constant stored through it is refused (§3f, §3g), a buffer
  filled a word at a time included, and a structure or float pointee never
  splits a store: an access across its members refuses the vote (§3e).
- A pointer typed by a vote also lets `RuleExpandLoad` print a known-typed
  narrow load as the truncated element: `(short)v1[0x11]`, and stat O2
  `sub_df20`, whose `char *a0` takes an `int *` passed up from `sub_e1d0`
  through `sub_dfe0`, prints its format-string reads as `v1 = (char)*a0;`. The
  same value on a little-endian target, read at a different width, as main
  prints for any typed pointer (§3b); no store is split by it.
- A vote of the same class can still override the caller's own evidence: an
  unsigned vote makes a parameter the caller shifts and divides as signed
  `unsigned int`, with `(int)a0` casts keeping those uses correct. The refusals
  are by class (pointer, integer, float), not by the finer type, except where the
  finer type would split a store (§3f): gzip's `unsigned char *` beside
  `strcmp`'s declared `char *` no longer turns its `builtin_strncpy(v,".tar",5)`
  into five byte stores. A same-width vote can still lose a character spelling:
  pr O0 `sub_2d9c`'s `char *` becomes `unsigned char *`, and `*v3 == ':'`
  prints as `*v3 == 0x3a`.
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

A follow-up measured here and left for its own PR: refusing a vote whose
non-character primitive pointee (undefined types included) is wider than an
access the caller makes through the pointer. It is the wide-read class of §9:
stat O2 `sub_df20` keeps main's `char *a0` instead of printing
`v1 = (char)*a0;` through an `int *`, and under `--option ptrfromuse void` the
twelve ls, dir and vdir `*_df_extension` comparators keep `void *` instead of the
`undefined8 *` their callee `sub_71f0` recovers (main prints `void *` there, and
they are twelve of the eighteen rows that score worse against main in that arm).
Against this branch, over the 444 slices: 3 rows improve at the default (+0.57),
18 with `ptrfromuse void` (+4.72), none gets worse and no perfect row moves; the
3,451 firmware rows do not move. It is not taken here because it also changes
209 functions over 33 of the 60 corpus binaries (68 of them back to main's
text, the rest a refused vote among others kept, mostly firmware register
accesses no longer spelled through an element type), and each of those needs
reading before it ships.
