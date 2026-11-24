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
40-minute limit, and `--all` printed only at the end, so they left nothing. `u-boot`
(`O2`) and `tar` (`O0`) were stopped by hand after two and a half hours.

Since then `check` runs its functions over worker processes (`--jobs`, default the
core count). The functions go in fixed chunks of 32, one process per chunk, so which
functions share a process does not depend on `--jobs`. Each row prints as its function
finishes. A function with no verdict after `--fn-seconds` (600) is `UNKNOWN(timeout)`,
and the rest of its chunk goes to a new worker. `coreutils` `ls` (`O2`, 405 functions)
takes 31 s instead of 206 s with ten workers, with the same verdict for every function. `bash`
(`O2`), which left nothing before, finishes its 2,542 functions in 26 minutes on a
loaded machine, three of them `UNKNOWN(timeout)`. `u-boot` (`O2`) finishes its 2,013
functions in 14 minutes with no timeout: it never hung, it needs 1.9 CPU-hours of checks,
which one process could not finish within the limit. Two lookups the model made for every
byte it read were linear scans: the loader's read-only ranges (925 in `bash`) and, for a
pointer outside the image, a load of that byte. They are now binary searches over merged
ranges. `bash` `set_pwd` checks in 5.2 s of CPU instead of 15.4 s, and
`make verify-corpus` takes 60 s instead of 110 s, with the same verdicts.

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

## 5. Closing the largest `UNKNOWN` causes

The model now runs what made most of §3's `UNKNOWN(unsupported-op)`:
- **x87's 80-bit format** (`src/x87.rs`): conversions from integers and to and from
  IEEE single and double (rounding to nearest, ties to even), sign operations,
  comparisons, the NaN test and truncation are exact. Arithmetic on the format stays
  unsupported. A `float` or `double` returned in `ST0` is rounded from the 80-bit value
  before the compare. An integer wider than eight bytes converts to a float exactly
  (kuna's IR widens an unsigned 64-bit integer to nine bytes first).
- **Cortex-M system registers**: the code runs privileged, `mrs` of the masks, base
  priority and exception number reads 0 on both sides, and the writes change nothing
  either side reads back.
- **kuna's memory builtins** (`memcpy`, `memset`, `memset16`/`32`/`64`, `memcmp`, which
  `repstring` prints for x86's `rep` string instructions) run on the decompiled side,
  spending the budget a byte at a time, as the loop they replace does. A `memcpy`
  whose destination starts inside its source, ahead of it, is the overlap `rep movs`
  defines and `memcpy` does not: declared as `DIVERGES(copy-overlap)`.

Running more code showed two more defects in the model, now fixed:
- A store into read-only data or code (a random pointer into a firmware image at
  address 0, or into a PIE's text) went through, and a later load of a literal pool
  read the stored value. A store that changes such memory now traps; a run where it
  traps is not compared, like a run that traps on both sides.
- `callee_writable` and the image lookups were linear scans (§3).

| | before | after |
|---|---:|---:|
| sample set `AGREES` | 157 | 172 |
| sample set `DIFFERS` | 1 | 0 |
| `ssh-add` (`O0`) `AGREES` / `UNKNOWN` | 294 / 697 | 885 / 107 |
| `RTOSDemo` (`O2`) `AGREES` | 86 | 91 |
| test corpus `UNKNOWN` turned `AGREES` | | 32 |

The sample set's one `DIFFERS`, `chThdSetPriority`, now agrees: its scheduler callees
read and write `basepri`, and with the system registers modelled they run. §4 put it
down to a context switch the model cannot run; that does not hold for this function, and
the other ChibiOS cases of §4 are not re-triaged yet.
`CMSIS_DAP.axf`'s CMSIS-RTOS calls now reach their `svc` and are
`DIVERGES(syscall-result)`. `RTOSDemo`'s new `DIFFERS`, `xQueueReceiveFromISR`, is a call
argument kuna does not pass, of §4's dropped-argument class.

## 6. The rerun with the current harness

Every function of the same 74 binaries again, after §5, with main merged up to #1914 (so
with the fixes of #1889–#1911), then the `DIFFERS` rechecked after the fixes below.

| | functions | `AGREES` | `DIFFERS` | `DIVERGES` | `UNKNOWN` |
|---|---:|---:|---:|---:|---:|
| first full run (§3–§4, partial) | 28,233 | 22,683 | 243 | — | 5,306 |
| before this section | 47,384 | 41,164 | 481 | 206 | 5,533 |
| now | 47,756 | 41,964 (88%) | 249 | 206 | 5,337 |

`O2` has 159 `DIFFERS` and `O0` 90. Only the `DIFFERS` were rechecked after the last
fixes; a function that agreed before them was not run again.

**Harness defects this rerun showed, all fixed** (`triage.md` §2 lists them with the
rest):
- kuna-verify loaded a binary without the options `kuna decompile` turns on: the
  Listing, which no-return propagation reads, and off x86-64 the discovery passes. So it
  checked a different decompilation from the one the CLI prints, in which calls to
  internal fatal wrappers (`ohshite`, `die`, `app_exit`) returned.
- A call followed only by padding and then another function, and a call kuna ends with a
  `CALL_RETURN` override (`error (1, ...)`), ran on into the next function.
- An RTOS context switch returns into another thread on another stack; the callee is
  now given the stub. A return to an enclosing call's return address on the same stack
  (libgcc's soft-float routines) unwinds to that call instead.
- A call result kuna keeps in a joined register pair (`r1:r0`) was not read back.

**What the 249 are:**

| cause | functions | where |
|---|---:|---|
| kuna's C reads a variable nothing assigns (a call output or a register a call kills) | 54 | across projects; one mechanism each, not triaged one by one |
| a call drops arguments | 42 | #1916 (a stack argument), #1918 (an import without a prototype, and the PLT entries themselves); the rest not matched to an issue yet |
| the set or order of calls differs | 31 | not triaged |
| memory differs | 31 | includes 10 ChibiOS `rt_test_*` around scheduler calls, most likely the context-switch limit, not kuna |
| functions discovery found in data | 23 | Cortex-M firmware `sub_*` whose bytes are strings: `funcstart_patterns`/`aif` take them for code, and `decompile-all` prints them too |
| u-boot's aligned stack buffers | 16 | #1919, directly (`part_test_dos`, `fs_devread`, `print_partition_extended`) or through callees run for real; not each checked |
| a return value or an argument value | 23 | not triaged |
| `memcpy`'s size dropped | 10 | Betaflight: `memcpy(dst, src)` for a three-argument call; `--option dwarf off` restores the size, so the DWARF import loses it; not filed yet |
| `fdt_getprop()` with no arguments | 7 | #1917 |
| a call through a function pointer drops arguments | 6 | `tar` `tar_sparse_*` |
| i386 stack offsets after `memset`/`memcpy` | 4 | #1920 |
| a return widened in a tail-call cycle | 2 | #1921 |

`transfer_entries` (`c5-results.md` §2) does not differ again: `--concolic 64` over
eight seeds gives `AGREES` or `UNKNOWN(budget)`.
