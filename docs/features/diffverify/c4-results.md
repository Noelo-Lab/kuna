# diffverify — slice C4 results (precision and recall)

C4 is the corpus run and its precision and recall (proposal, Slices). This file records
both on the corpus kuna already has. Precision: every `DIFFERS` over `tests/datatests`
and `tests/stages` is triaged into a kuna bug, a bug its test pins on purpose, a harness
limit, or a declared divergence, and the harness defects that triage found were fixed on
the way. Recall: past P3/P4 fixes were put back one at a time in a local build, to see
which the check catches (§6). The decbench binaries are in `c4-decbench-results.md`.

```sh
decompiler/target/release/kuna-verify corpus tests/datatests tests/stages --trace
```

`--trace` adds both sides' calls, in order, to every `DIFFERS`. An argument the callee's
prototype does not recover prints as `?`.

## 1. The corpus

4,030 decompiled functions, 256 runs each. C2 saw 3,979: it missed the 51 a script
decompiles with an abbreviated command (§5).

| verdict | C2 | C4 |
|---|---:|---:|
| `AGREES(256)` | 2,482 | 3,246 |
| `DIFFERS` | 500 | 113 |
| `DIVERGES` | 3 | 24 |
| `UNKNOWN` | 994 | 647 |

`UNKNOWN` by reason: 399 unsupported operations (SIMD and barrier user ops, memory
builtins, ISA-mode switches, float formats other than IEEE single and double), 226
budget, 16 traps on both sides, and 6 undefined reads of the frame.

Of the 113 `DIFFERS`:
- 28 are the ones §2 triages;
- 49 come from the two checks of §6a and are triaged in §6b;
- 15 come from float support and are triaged in §6c;
- 21 come from system calls and are triaged in §6d.

None is an unexplained harness defect.

Every function C1 found to agree still agrees. Of C2's 2,482, twelve no longer agree:
- `enum ptrenumlow` is `UNKNOWN(unsupported-op)`: its IR has an `INT_AND` whose output
  is narrower than its inputs (§3).
- Four are `UNKNOWN(budget)`. In each, a loop waits for an in-image callee to change
  memory. The callee is a `ret` placeholder that the machine now runs for real, where C2
  gave it the stub and let the stub write the memory. The machine side loops forever
  because the program does.
- Two halt on an operation the machine cannot run: a Thumb `setISAMode` before a
  branch, and a decompiled flow that ends in a halt.
- Two read an uninitialized local on both sides. In `kuna-msvcbyval` `sub_401000` the
  argument is read from frame bytes outside the object the callee was given, which the
  stub no longer writes (§5).
- Three, in `kuna-stacklifetimes-cfg`, give a frame object to a callee that uses a
  float op the model cannot run, so the stub's effect is a guess:
  `UNKNOWN(guessed-callee)`.

The 24 `DIVERGES` match their predicates: `synthesized-call` (the MSVC string append,
outlined, AVR wrapper and `linuxsyscall` calls kuna makes up), `gowritebarrier`,
`switchrunon-cut-row`, `nan-quieting` (§6c), and `syscall-result`, `panic-check` and
`ptrauth-trap` (§6d).

## 2. Every `DIFFERS`

| class | test, function (pass) | count | what differs |
|---|---|---:|---|
| kuna bug | `gh8777-flagcompare-sborrow` `FUN_0` | 1 | `RuleSborrowGe` takes `0 <= V-K`, which is *not* N, for N. So it rewrites the loop's `V < K` continuation to `7 <= v1`, and the loop runs zero times where the binary runs seven. Assertion #2 pins the wrong output. |
| kuna bug | `kuna-repframecopy` `sub_4012c0` (`unsettled`) | 1 | The stack-pop solver guesses wrong for the call through a pointer to `guess`, which pops 4. kuna then reads a stack slot the binary never reads: machine `EAX = 0x8008`, kuna `0xffff`. The test header says the guess moves the block. |
| kuna bug | `kuna-stackviews` `obj_global_reuse` (pass 1, `stackviews off`, the default) | 1 | The IR drops the stores `mov [rsp],5`, `mov [rsp+4],7` and `mov [rsp],rdx` (`&datum`), although the function publishes RSP to the globals `object_pair` and `object_handle` and reads back through them. The C reads `object_pair->x` from a never-written local: machine `EAX = 0x43` (5 + 7 + 0x37), kuna `0x7ffffffe`. Pass 2 (`stackviews on`) keeps the stores and agrees. A frame address stored to a global has escaped, whatever the option. |
| kuna bug (prototype) | `kuna-libctypes` `next_byte` | 2 | The source returns `int` from `__uflow`. kuna claims `uint8` and zero-extends the 32-bit call result, but the x86-64 ABI leaves RAX's upper half undefined. |
| bug the test pins | `kuna-narrowext` `pi`, `pu`, `psc` (`narrowext off`) | 3 | The pass that reproduces #794. |
| bug the test pins | `kuna-indexedstoreguard` `cb_m8`, `-a64` `m8` | 2 | The known limitation the test documents. |
| bug the test pins | `overlapbranch` `overlap` (pass 2) | 1 | The overlapping-branch limitation the test documents. |
| bug the test pins | `ghangr-optimized-memcpy-6301a9` `memcpy_opt` (pass 1) | 1 | `jtsharepartial` and `unrolledguard` off: the indirect jump becomes a call. |
| bug the test pins | `ghdec-indexaliasguard` `transform` (pass 1, `indexaliasguard off`) | 1 | The pass deletes the stores the test pins (0x401344 to 0x40135d). kuna XORs into a local nothing wrote, where the binary XORs into the table. Pass 2, the fix, ends in `UNKNOWN(budget)`, so the fix itself is not checked. |
| source-level UB | `kuna-arrayextent` `d4_clang` | 3 | The source indexes past its array. The two sides read different neighbors. |
| harness limit | `kuna-nullable-relocations` `guard`, `same` | 8 | A relocation to an undefined weak symbol is not resolved to 0 in the image. |
| harness limit | `kuna-mipsgot-weak` `same` | 2 | The same: the direct reference to the undefined weak `optional` is not 0, while its GOT entry is. |
| harness limit | `injectoverride` `callind_inject` | 1 | kuna replaces the call with a call-fixup injection. The machine calls the real target. |
| harness limit | `kuna-calleeretpreserves` `f` | 1 | `int 0x29` (`__fastfail`) does not return. The machine treats it as returning. |
| harness limit | `kuna-calleescratchbody` `f` | 1 | The function reads the TEB through `gs`, which the initial state does not model. |

Three `DIFFERS` that showed up mid-triage were not kuna's: `partialsplit` `loadalone`,
and `kuna-arrayextent` `q_clang` and `w_clang`. Each indexes past its array, into the
return address a call pushed (`loadalone`) or into the spill of `rbx` (`q_clang`,
`w_clang`). That is undefined in C, and the harness now reports it so (§4).

## 3. kuna findings that are not `DIFFERS`

- **IR size mismatch.** `enum ptrenumlow`, `enum ptrenumhigh` and `kuna-narrowload`
  `t_bit` each have an `INT_AND` whose 1- or 4-byte output is narrower than its inputs
  (`AL:1 = u:4 & #0x3000:4`). The interpreter refuses mixed sizes, so these are
  `UNKNOWN(unsupported-op)`. The printed C is right in the three cases checked. The
  IR still breaks the invariant that a p-code op's sizes match.
- **Address-tied locals.** In `kuna-msvcstrappend` `near_miss`, heritage puts no
  INDIRECT guard on a STORE through `&local + i`. The local is address-tied, so the
  printed C reads the variable and agrees. C2's interpreter read the stale SSA value
  and called this `DIFFERS`.

## 4. Harness defects found while triaging

| defect | effect before the fix |
|---|---|
| Arguments read from the stack storage overlapping SP, not only the callee's prototype params | false argument differences |
| A machine `RETURN` always ended the function | `kuna-entryretdispatch` (a pushed-address `ret` that kuna reads as a call) differed |
| In-image tail calls (`BRANCH`, `CBRANCH`, `BRANCHIND` kuna made calls) got the stub | the kill value against the callee's real effect (`tailcallframe`, `memcpy_opt`) |
| A call value kept in a temporary read as 0 | 1-byte to 8-byte call results lost after a cast into a register: `gh510-phiopflags`, `kuna-arm-float-return`, `ghdec-realtypes-pointee`, `ghdec-undefname`, `kuna-libctypes` `fopen` |
| A return folded to a constant compared at `const:0` | `UNKNOWN` or a false `DIFFERS` (`kuna-bejoin-sparc`, `kuna-sparc-zeropair`, `kuna-gobool`); now compared at the register the model returns that type in |
| A `bool` parameter on the stack started as a random byte | 10 false `DIFFERS` in `kuna-stackobjects`: `!flag` is `flag ^ 1` |
| A callee run for real spent the caller's step budget | in-image callees ended their callers in `UNKNOWN(budget)`. Callees now share a pool of their own, and a callee that exhausts it gets the stub |
| Tracked registers (PowerPC TOC, MIPS `t9`) and the return address not seeded, and the MIPS ISA-mode bit of a computed target kept | false target and memory differences |
| Mixed-size ops evaluated by truncating | false `DIFFERS`; now `UNKNOWN(unsupported-op)` |
| A read of an uninitialized local on kuna's side alone counted as undefined behavior | a kuna read of the wrong slot hid as `UNKNOWN`. Only a run where both sides read the frame where C defines nothing is undefined now |
| A call's return address, and a callee's frame, stayed written below the stack pointer after the call returned | an out-of-bounds read there looked defined on the machine side only. Both sides now see uninitialized stack (`partialsplit` `loadalone`) |
| The prologue's spill of a callee-saved register counted as a defined local | reading past an array into it looked defined on the machine side only (`q_clang`, `w_clang`). A load of a spill is now a read of uninitialized stack unless it restores the same register, directly or through the temporary the instruction copies there |
| A stack-protector failure, and a machine write to the return address, compared as a result | `UNKNOWN(uninitialized-local)`: the stub let a call overwrite the frame |

## 5. What a verdict claims

A review of the C2 harness found four places where a verdict claimed more than the
runs showed. All four are fixed:
- **Traps.** A run that traps on both sides used to count as agreeing. Division by
  zero is undefined in C, so the two sides' effects before the trap are not compared
  either. Such a run now counts as not compared: `AGREES(n)` gives the runs that did
  agree, with a note on how many trapped. A function whose every run traps is
  `UNKNOWN(trap)`.
- **Declared divergences.** A return value that matched a declared divergence skipped
  the memory comparison. The divergence now covers only the return value, so a store
  that also differs is `DIFFERS`.
- **Call targets.** A target was compared only when both sides made an indirect call.
  It is now also compared when one side resolved the call to a direct one, if both
  targets are code in the image. An import slot or an unrelocated GOT entry is not
  code, so these calls are not compared.
- **CLI.** `--runs 0` is a usage error. A corpus file that cannot be checked, a
  worker crash included, makes the exit status 3. `DIFFERS` is 1. `UNKNOWN` is not a
  failure.

`tests/verdict_guards.rs` pins the first and last. The second and third change no
verdict in the corpus.

A second review found seven more. All are fixed, and each fix keeps the corpus's
`DIFFERS` the same 29:
- **Frame objects a callee receives.** A store to a local whose address a call
  receives was not compared, because stack bytes were never an observable. At each
  call both sides now compare the object the pointer is into, with its extent taken
  from kuna's local symbol for that address. kuna's side looks among the CALL op's
  inputs, and the machine looks in the convention's argument registers. Only objects
  both sides find are compared, so a stale pointer left in an unused register costs
  nothing.
  - A callee run for real counts only the bytes it reads. A callee kuna proved
    read-only (#810's callee memory summaries) may then lose an unread initializer.
  - Bytes a callee's own code left inside the object (a stack probe's pushes in a
    buffer C never initialized) are not compared.
  - The stub writes only that object, not a window past the pointer. A window
    reached the stack canary and made `ptrtoarray` `passPtrToArray` call
    `__stack_chk_fail`.
  - An in-image callee the model cannot run (a float op) and that gets a frame
    object makes later differences `UNKNOWN(guessed-callee)`: the stub's write is a
    guess where kuna may know the callee only reads.
  - Changing `int x = 1; sink(&x);` to store 2 on kuna's side, in a local build, now
    gives `DIFFERS: memory before call 1 to sink: machine 0x01, kuna 0x02`.
- **Empty loops.** A block visit, and an instruction with no p-code, now spend the
  step budget, so a loop of empty blocks ends in `UNKNOWN(budget)`.
- **Narrow returns.** The stub extends a callee's return value across its register
  as `narrowext abi` documents. An RV64 `int` comes back sign-extended.
- **Preserved registers.** The stub kills what the call's own effect list says,
  so what kuna proved a callee preserves (`calleepreserves`) stays.
- **Abbreviations.** `dec` and any other abbreviation of `decompile` is checked:
  the console now reports the command a line resolved to. This adds 51 functions.
- **Pointer contracts.** A contract returning a pointer asks the model where a
  pointer goes: A0 on ColdFire, not D0.
- **Failed loads.** A `decompile` after a `load` that failed is not checked: it
  would check the function loaded before again.

`tests/verdict_guards.rs` pins the trap, empty-loop, frame-object, failed-load,
abbreviation and CLI cases.

A third review found seven more, all fixed:
- **Kept registers.** A stub that a callee falls back to now keeps the registers
  `calleepreserves` proved it leaves alone, which the effect list does not carry.
- **Earlier escapes.** A frame object passed to any earlier call is compared at every
  later call (`save(&x); x = 1; consume();`).
- **Direct targets.** These are compared too, after following thunks (a jump, an
  `endbr64`, a PLT slot). Calls kuna bound past a PLT stub or replaced with an injection
  are not compared, since the machine runs no dynamic linker and no call fixup.
- **Contracts.** `__int128` is 16 bytes, and an unknown word in a contract type is an
  error rather than ignored.
- **Register return addresses.** A return address kept in a register (RISC-V `ra`,
  ARM `lr`) is written by every call and is no longer read as a smashed frame.
- **Empty script lines** clear the console's resolved command.
- **Missing corpus paths** are a usage error.

## 6. Recall

Of the 56 P3/P4 fixes since June 2026, 39 have a stage fixture. The 27 whose fixture the
check could run were measured. Five were measured without a code change: a pass that sets
the fix's option off, or C1 and C2's fault injections. The other 22 were put back by hand
in a local build, never committed. A plain revert no longer applies, because the sources
moved into phase folders since, so each fault is a guard returning its pre-fix answer.
Each fixture's own assertions confirmed the old output was back.

| outcome | fixes |
|---|---|
| caught, no contract | #801, #828 (`narrowext off`), #536 (`indexaliasguard off`), #893, #879 (1 of its 7 functions) |
| caught only with a C contract | #794, #855, #755, #797, #879 (all 7) |
| missed | #561, #554, #252, #564, #1500, #290, #1501, #542, #565, #559, #455, #875, #763 |
| nothing to catch: the IR computes the same value | #1537 (a temporary named), #251 (`SUBnn` printed), #249 (a masked target the CPU masks too) |
| a crash fix: shows as a file error | #227 |
| bug did not come back | #238 |

Of the 22 that put a semantic bug back, 5 are caught without a contract (23%) and 9 with
one (41%). The misses fall into a few gaps, most of them in how the model treats calls:
- **A callee run for real shares its effect.** kuna's side replays what the machine's
  run of the callee did, so a dropped or wrong argument changes nothing on kuna's side.
  Only arguments kuna claims are compared, and a variadic one has no storage to compare
  at. Missed: #564, #1500, #455, #875.
- **Storage kuna never initialized holds the machine's value.** A register kuna reads
  without a parameter for it, or a call output it never connects, holds what the
  machine has there, so C reading an unassigned `v1` or `_r3` computes the right value.
  At most a `PROTOTYPE_SUSPECT` note appears. Missed: #290, #565, #1500, #1501.
- **A return claimed too narrow, or void, compares nothing without a contract.** With
  one, #855, #755, #797 and #879 are caught. #763 (swapped halves) was not run with
  one. #559 claims a return too wide, and its extra half really holds what the claim
  says.
- **Frame objects at an in-image callee that reads nothing.** In #561 and #554 the wrong
  bytes reach only a buffer passed to `sink`, which is a bare `ret` in these fixtures.
  The program never reads them. A probe whose `sink` reads the buffer sees the
  difference, as `UNKNOWN(uninitialized-local)` there, since the buffer's source is
  never initialized either.
- **Writes into read-only memory.** #252 stores a literal's own bytes back into it,
  which is not a value difference, and nothing checks for writes to read-only memory.
- **C that is false but runs the same.** #542 (`-sub_1100()` equals the RAX the real
  `__chkstk` leaves) and #1501 (a jump made a call into in-image code) execute
  identically. Only the printed-C tier can see them.

### 6a. Two checks for the call gaps

Two checks were added for the first two gaps, and the misses re-measured on them:
- **A callee's inputs.**
  - Recording: a callee run for real has the argument-register bytes it reads before
    writing them recorded. Bytes it only spills (a `push`, a home-slot store, a varargs
    prologue) are not recorded.
  - Placement: each of kuna's CALL inputs is placed at the callee's parameter storage,
    else at its own register for an entry-register input, else at the next integer
    argument register. Register pairs are split into their pieces.
  - Confirmation: where kuna passes other bytes or none, the machine is rerun with
    kuna's bytes in the callee's registers. Only a changed result is `DIFFERS`: the
    function's own, or the callee's return value or memory writes.
  - A callee with a declared prototype makes it a `PROTOTYPE_SUSPECT` instead, since kuna
    follows the declaration.
- **What kuna's C never assigns.** kuna's side is rerun with other values in:
  - argument or scratch registers it reads at entry with no parameter for them;
  - output registers of a call it never connects;
  - other registers a call kills, read after it.

  A changed kuna result is `DIFFERS`. For entry registers and non-output killed
  registers, the machine is rerun with the same values first: when its own result
  changes too, the binary reads that storage itself (a `push ecx` reserving a slot, a
  hand-written routine reading `r10`), and the dependence is not kuna's.

Re-measured with both checks:
- #565 and #1500 are now caught without a contract, through a call output kuna never
  connects (`_r3` after the call).
- The other eleven misses stand:
  - **Path:** the bug is on a path random inputs never take (#564: `end == cap`). That
    needs C5's concolic inputs.
  - **A smaller claim:** kuna claims less than the binary does. A void caller ignores the
    callee result (#455). A return is narrowed or dropped (#559, #763). An argument
    is dropped that the fixture's callee ignores (#875).
  - **The binary reads the register too:** #290's `this` arrives in ECX, which the
    binary reads at entry, so the missing parameter is a prototype claim.
  - **No reader:** #561 and #554 reach only a callee that reads nothing.
  - **No check:** #252 needs a read-only store check. A first try flagged AVR's
    memory-mapped registers and XML fixtures' read-only chunks, and was dropped.
  - **Printed C only:** #542 and #1501 are false only in the printed C.

| | without a contract | with one |
|---|---:|---:|
| before the two checks | 5 of 22 | 9 of 22 |
| with them | 7 of 22 | 11 of 22 |

### 6b. What the two checks found in the corpus

The two checks add 49 `DIFFERS` over the corpus, and remove one (`kuna-calleescratchbody`
`f`, whose stub no longer overwrites what kuna proved the callee keeps). Triage found
no false positives in the 49. The false-positive classes the first version had were
removed before this count: pair arguments, Swift's `self`, declared prototypes, and
registers the binary itself reads.

The largest group is one kuna defect: **a caller that passes its own entry argument
registers straight through to a callee loses the arguments.** kuna prints `v1 = add64();`
where the source is `add64(a, b)`. The same pattern shows up on six targets:
- x86-64: `narrowret-x86` `returns_word`, `libcnames` `ordered`;
- ARM;
- PPC: `bejoin-callers`, `bejoin-residue`, `relocobj-ppc-mips`;
- MIPS: `mipsunaligned`, `mipsgot-local`, `bejoin-mipspic`;
- eBPF: `ebpf-elf-map-reloc`;
- Rust and Swift: `rustabi`, `swiftcall`.

Several of these tests pin the dropped spelling as expected output.

Other real defects:
- **A register the callee leaves alone is treated as killed.** `loadguardreads`
  `gather` passes `reg(&s0, 0)` with the 0 set once before an earlier call; `regpiece`
  `sh_openpipe` does the same with EDX.
- **An assigned argument is dropped:** `pointerwalk-memo` `f` prints `ext()` for
  `ext(buf[0])`.
- **A call output is never connected**, read as an unassigned `v1`: the off or bug passes
  of `calleeretpreserves`, `callretpair`, `callretpairhalf` and
  `declaring-double-score-return`.

The rest are passes a test pins on purpose:
- the off passes of `calleearitycut`, `calleearityscratch`, `calleereadarg`, `bodyreads`
  and `msvcstrappend`;
- `narrowext`'s RISC-V bug pass, which a guard had hidden as `UNKNOWN` (§5).

`injectoverride` `inject_collapse` is a harness limit: the machine does not run kuna's
call-fixup injection.

### 6c. Floats

Float ops now run on IEEE single and double through the same `OpBehavior` table,
for both sides. Other float encodings (x87's 80 bits, half precision) stay
`UNKNOWN(unsupported-op)`. Two parts are exact rather than going through the host's
floats:
- a sign change (`FLOAT_NEG`, `FLOAT_ABS`);
- a NaN's format conversion.

These keep a NaN's payload, as IEEE defines them and as a binary that clears a sign bit
with an integer AND does. A float argument a callee reads is placed at the float
register kuna's call input sits in.

This judges 242 more functions (2,932 to 3,174 agree). Of the 15 new `DIFFERS`:
- **NaN compares on ARM and AArch64 (kuna defects):** `armfloat-preset` `scale_err`,
  `floatret` `logish`, `prefersplit` `ccw_mixed`.
  - The machine's `vcmp`/`fcmp` sets V on a NaN, so `ble` and `cset gt` take the
    unordered arm.
  - kuna prints `a != b && !(a < b)`, which is true for a NaN. `kuna_floatnegation`
    folds only x86's unordered guard; with `nanignore none` all three agree.
- **Missing float arguments (kuna defects):**
  - `arm-float-return`'s on pass prints `(float8)half()`, although `half` reads d0;
  - `earlyrestart` `main` prints `set_gi_bits()` for `set_gi_bits(-7.5f)`.
- **Off passes and pinned outputs:** the off passes of `ghdec-calloverlap` and
  `arm-float-return`, and `declaring-double-score-return`'s two clobber controls.
- **A harness limit:** `declaring-double-score-return` `score`, `wide_exact_caller`
  and `refined_score`.
  - kuna proves the MSVC cookie checker leaves XMM0's low lane alone, but the stub
    still kills it. The proof is not in the call's effect list.
  - Using kuna's per-call return-preserve proofs in the stub removed these five and
    added about fifteen elsewhere, so it was dropped.

`kuna-floatprecision` `widen_roundtrip` is a declared divergence, `nan-quieting`:
`nanignore compare` treats a float conversion round trip as the identity, so a
signalling NaN comes back unquieted. A float return that differs only in the quiet bit
of a NaN matches it.

### 6d. System calls and traps

A system call (`syscall`, `svc`, `ecall`, `sc`, `swi`) is now an event on both sides
instead of an unsupported op. Each side records the address, the user op and its
inputs, and the two lists are compared in order:
- **Inputs.** The SLEIGH op takes no registers. kuna's `syscallregs` and `x64syscall`
  attach the number register and then some of the argument registers. Each one kuna
  attaches is matched, in ABI order, with what the machine's register file holds there.
- **Result.** The machine writes the kernel's result register (and MIPS `a3`, PowerPC
  `cr0`), as Linux and the BSDs do. The value is one both sides share, and kuna's call
  output gets it too.
- **Traps.** A trap (`brk`, `udf`) in the function's own code ends the run as a
  no-return. One reached inside a callee the machine runs for real is not an event.

`syscallregs auto`, the default, writes nothing on an image that names no operating
system, and every XML fixture is one. There the machine writes nothing either. The run
is then repeated with the kernel's write, and if that changes the return, a store or a
call, the function is `DIVERGES(syscall-result)`.

Two more options cut code the machine runs, knowingly:
- `gobounds` and `securitycheck` remove the branch to a Go or Rust check's panic
  helper: `DIVERGES(panic-check)` when the machine calls one kuna does not.
- `ptrauthtrap` cuts an arm64e pointer-authentication check. The machine does not
  authenticate (SLEIGH hides `autibsp`), so the return address it tests is whatever
  was loaded: `DIVERGES(ptrauth-trap)` when it stops in the `brk #0xc47x`.

This judges 72 more functions (3,174 to 3,246 agree; unsupported ops 509 to 399):
- **21 new `DIFFERS`, all off passes the tests pin.** These are `syscallregs off` in
  the ARM, AArch64, RISC-V, MIPS, MIPS `a3` and PowerPC `cr0` tests, `x64syscall off`
  in `kuna-x64syscall`, and both passes of `kuna-linuxsyscall-x64-compat`. In each,
  the register read after the call still holds the number or an argument.
- **12 new `DIVERGES`:**
  - 6 `syscall-result`: the `auto` passes and `kuna-ownreturn` `sys2nr`, which returns
    `a0 + 1` where a kernel returns its result + 1;
  - 2 `panic-check`: `kuna-gobounds` and `oxidizer-securitycheck` `pick` with the
    option on;
  - 2 `ptrauth-trap`: `x30check` and `_tail_auth` with `ptrauthtrap on`;
  - 2 `synthesized-call`: `linuxsyscall`'s calls, which the machine used to stop short
    of at the `int 0x80`.

The `on` and `abi` passes of every system-call test agree.

## 7. Cost

Check time 152.9 s, wall time 22.9 s on 14 worker processes, over 4,030 functions. Peak RSS is 216 MB for the
heaviest single test. Check time grew from C2's 50.3 s because functions that used to
stop early now run: callees run for real, calls are kept in loops up to the 256-call
limit, and fewer runs halt on harness errors. One function,
`kuna-pointerwalk-budget-mips` `ldso_d2f0`, takes 9.2 s and ends as `UNKNOWN`.
