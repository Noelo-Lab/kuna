# diffverify — slice C4 on the decbench binaries

`c4-results.md` ran the check on the corpus kuna already has. This file runs it on
real binaries: the decbench dataset (`noelo-lab/decbench-dataset`, 39 C projects,
x86-64 and i386 ELF/PE and Cortex-M firmware, at `O0`, `O2` and `O2-noinline`).

```sh
kuna-verify check <binary> <function>... --json     # the sample set
kuna-verify check <binary> --all --json             # every function of a binary
```

kuna was built from `feat/diffverify-c2`, before any of the fixes in §4 merged.

## 1. Harness defects real binaries showed

The test corpus has small, hand-made binaries. Real ones found six defects in the
model, all fixed here:

| defect | effect | fix |
|---|---|---|
| a call kuna knows does not return by its callee's name (`exit`, `__assert_fail`) was run on past | the machine ran into the next function | a call flow plants a no-return halt after is a no-return site (`calls.rs`) |
| a stubbed `bool` callee returned any byte | `x \|\| y` over its result differed from kuna's, which relies on 0 or 1 | the stub leaves 0 or 1 in a `bool` return register (`abi.rs`) |
| x86's `FS_OFFSET`/`GS_OFFSET` took random values, some inside the image | the stack-protector guard at `fs:0x28` changed across a call, and the machine took `__stack_chk_fail` | segment bases point at a fixed block outside the image and the stack (`check.rs`) |
| a Thumb `bx` refused (`setISAMode` before a branch) | every Cortex-M function with an interworking branch was `UNKNOWN` | a switch to the mode the target already decodes in runs (`machine.rs`) |
| memory barriers and x86's `lock` refused | `UNKNOWN(unsupported-op)` | no effect on one thread |
| ARM's exclusive monitor (`ldrex`/`strex`) refused | `UNKNOWN(unsupported-op)` | one thread always holds it: `strex` succeeds |

On the test corpus these turned 82 `UNKNOWN` into `AGREES` and lost no `AGREES`
(`docs/baseline-verify.json` is re-recorded). They also exposed two `DIFFERS` there:
- `ghdec-isamode-inject` `dispatch`: kuna reads r3 after a call as if the call killed
  it, but GCC's IPA-RA kept it there (the bug PR #1892 fixes);
- `kuna-armfloat-preset` `fixit`: the pass that turns `armfloatreturn` off, the bug
  the test pins.

## 2. The sample set

decbench's published `sample-set`: 250 functions over 224 binaries.

| verdict | before §1 | after §1 |
|---|---:|---:|
| `AGREES(256)` | 109 | 157 |
| `UNKNOWN` | 138 | 92 |
| `DIFFERS` | 3 | 1 |

Two of the three `DIFFERS` before were the defects of §1 (`diff` `read_files`: a
`bool`; `touch` `main`: the stack guard). The one left is a harness limit (§4).

## 3. Every function of a binary

One binary per project, the one the sample set picks, every function:

| | binaries done | functions | `AGREES` | `DIFFERS` | `UNKNOWN` | check time |
|---|---:|---:|---:|---:|---:|---:|
| `O2` | 32 of 37 | 12,641 | 9,941 (79%) | 126 | 2,574 | 0.9 CPU-h |
| `O0` | 34 of 37 | 15,592 | 12,742 (82%) | 117 | 2,732 | 1.6 CPU-h |

Not done: `bash`, `cf2.elf`, `betaflight` and `cleanflight` (both levels) ran past a
40-minute limit, and `--all` prints only at the end, so they left nothing. `u-boot`
(`O2`) and `tar` (`O0`) ran on past the limit and were stopped by hand.

`UNKNOWN` is half budget (2,785), half operations the model cannot run (2,274). Of
those:
- 1,263 are x87 80-bit floats (`fldz` and friends: `INT2FLOAT` into ten bytes);
- 444 are Cortex-M's `isCurrentModePrivileged`, which the model could answer with a
  constant both sides read;
- 122 are ARM/Thumb switches to the mode the target is not already in;
- 106 are jumps to a random address, mostly through unrelocated GOT slots;
- 95 are `INT_AND` with an output narrower than its inputs (the IR defect a separate
  fix handles);
- 93 are calls with unknown stack cleanup.

## 4. The 243 `DIFFERS`

| class | functions | what it is |
|---|---:|---|
| RTOS context switch | 74 | ChibiOS (57) and other RTOS calls into the scheduler. The machine runs the switch, which loads another thread's stack pointer from random memory: a harness limit. `chThdSetPriority` is the sample-set one |
| PLT stub of a function the object itself exports | 54 | `libedit`, `libselinux`, `libz`: kuna prints the `jmp [got]` stub as `(*dat_19018)()` and drops the arguments it forwards. A kuna defect, of little consequence |
| a call's output never connected | 37 | the result of a call kuna has no prototype for. `find` `pred_newer`: `difftime(a, b)` printed as `difftime()`, its `double` result in xmm0 read as an unassigned `v3` |
| argument dropped | 21 | tail-call and wrapper arguments. `rsyslogd` `multiSubmitFlush` (#1835 fixes it); `tar` `rpl_getopt_long`, whose callee takes a seventh argument on the stack, still prints `rpl_getopt_internal()` with #1835 |
| return value | 18 | `find` `optionl_stat`: an `int` returned as `unsigned long` over a tail call (#1896 fixes it) |
| call set, argument value, memory | 39 | among them `dexter.dll` `_GetOSVersion`, where i386 stack offsets after a cdecl `__memset` are off by the pushes (a return address is stored into `si.dwPageSize`; #1888 does not fix it), and `_AddCache`, whose `memcpy` source is an unassigned variable instead of `ptr`. The `O0` `hash_*` and `*_hasher` cases are not triaged yet |

New kuna bugs that no open PR fixes, each with a repro:
- an unprototyped callee's arguments and `double` result: `find` `pred_newer`;
- a wrapper whose callee takes stack arguments: `tar` `rpl_getopt_long`;
- i386 stack depth after an in-image cdecl call: `dexter.dll` `_GetOSVersion`;
- a merged variable that is never assigned: `dexter.dll` `_AddCache`;
- a PLT stub's forwarded arguments: `libz.so` `crc32_z` at 0x3340.
