# diffverify — slice C4 results (precision)

C4 is the corpus run and its precision and recall (proposal, Slices). This file records
the first half on the corpus kuna already has: every `DIFFERS` over `tests/datatests`
and `tests/stages` triaged into a kuna bug, a bug its test pins on purpose, a harness
limit, or a declared divergence. The harness defects that triage found were fixed on the
way. Recall (reverting the visible P3/P4 fixes) and the decbench binaries are still open.

```sh
decompiler/target/release/kuna-verify corpus tests/datatests tests/stages --trace
```

`--trace` adds both sides' calls, in order, to every `DIFFERS`. An argument the callee's
prototype does not recover prints as `?`.

## 1. The corpus

3,979 decompiled functions, 256 runs each.

| verdict | C2 | C4 |
|---|---:|---:|
| `AGREES(256)` | 2,482 | 2,952 |
| `DIFFERS` | 500 | 29 |
| `DIVERGES` | 3 | 11 |
| `UNKNOWN` | 994 | 987 |

`UNKNOWN` by reason: 744 unsupported operations (SIMD and float user ops, memory
builtins, ISA-mode switches), 219 budget, 16 traps on both sides, and 8 undefined reads
of the frame.

Of the 29 `DIFFERS`, 5 are kuna bugs, 8 are bugs a test pins on purpose, 3 are
source-level undefined behavior, and 13 are harness limits (§2). None is an unexplained
harness defect.

Every function C1 found to agree still agrees. Of C2's 2,482, eight no longer agree:
- `enum ptrenumlow` is `UNKNOWN(unsupported-op)`: its IR has an `INT_AND` whose output
  is narrower than its inputs (§3).
- Four are `UNKNOWN(budget)`. In each, a loop waits for an in-image callee to change
  memory. The callee is a `ret` placeholder that the machine now runs for real, where C2
  gave it the stub and let the stub write the memory. The machine side loops forever
  because the program does.
- Two halt on an operation the machine cannot run: a Thumb `setISAMode` before a
  branch, and a decompiled flow that ends in a halt.
- One reads an uninitialized local on both sides.

The 11 `DIVERGES` match their predicates: `synthesized-call` (the MSVC string append,
outlined and AVR wrapper calls kuna makes up), `gowritebarrier` and
`switchrunon-cut-row`.

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

**Stack stores remain unobserved.** Memory in the stack window is not an observable.
That is right for the caller's argument area and Win64 home slots: the ABI gives them
to the callee. It misses one case: a store to a local whose address an *out-of-image*
call receives. The stub does not read memory, so a store kuna drops there is not
seen unless the function reads the local back, as `kuna-stackviews` does. Comparing
the bytes after an escaped stack pointer catches it, but only within the local's
extent. A fixed window also covers register spills that only the machine writes. The
extent has to come from kuna's local symbol for that address, which is left for a
later slice.

## 6. Cost

Check time 132.7 s, wall time 20.7 s on 14 worker processes. Peak RSS is 216 MB for the
heaviest single test. Check time grew from C2's 50.3 s because functions that used to
stop early now run: callees run for real, calls are kept in loops up to the 256-call
limit, and fewer runs halt on harness errors. One function,
`kuna-pointerwalk-budget-mips` `ldso_d2f0`, takes 9.2 s and ends as `UNKNOWN`.
