# diffverify — slice C1 results

C1 is Tier C on register-only leaf functions (proposal, Slices), built as the dev-only
crate `decompiler/crates/kuna-verify`. This file records what it measured and what
building it changed in the design.

```sh
make binaries specs
cd decompiler && cargo build --release -p kuna-verify && cd ..
decompiler/target/release/kuna-verify check ins16.o ins16 --contract ins16.c --option returnpair single
decompiler/target/release/kuna-verify corpus tests/datatests tests/stages
decompiler/target/release/kuna-verify check ins16.o --all --json
```

Run `corpus` from the repository root: stage tests load their fixtures by repo-relative
path. It runs one worker process per core (`--jobs N` to change it), and a file that
crashes the engine costs only its own verdicts. Every number below is from 256 runs per
function with the default seed.

## 1. #794 against its source contract

`--option returnpair single` is the pre-#794 behavior (the fix's commit message says the
option gives the word back). The fixture is the 32-bit ARM `ins16` from
`kuna-cli/tests/zext_pair_returns.rs`, rebuilt in `kuna-verify/tests/zext_pair.rs`.

```
$ kuna-verify check zext_pair.o ins16 --contract zext_pair.c --option returnpair single
ins16  DIFFERS  (run 1 of 256)
  contract u64 ins16(unsigned a, unsigned short b)  (zext_pair.c)
  input    a0=0x00007fff (r0)  a1=0x0000007f (r1)
  machine  r1:r0 = 0x000000008000007f
  kuna     int4 ins16(int4 a0, uint4 a1) = 0x8000007f; as u64 0xffffffff8000007f
```

The same run without `--contract` is `AGREES(256)` with
`PROTOTYPE_SUSPECT(the machine sets r1 to 0x00000000 on every run; the claimed return
leaves it out)`. The physical ABI cannot settle the prototype, and the check does not
claim it can. With the fix, `ins16`, `zadd` and `zsel` agree with their contracts.

## 2. Verdicts over `tests/datatests` and `tests/stages`

782 files, 3,979 decompiled functions (a script that decompiles a function twice counts
twice), no file errors.

| verdict | functions |
|---|---:|
| `AGREES(256)` | 696 |
| `DIFFERS` | 4 |
| `DIVERGES` | 3 |
| `UNKNOWN` | 331 |
| `SKIPPED` | 2,945 |

`SKIPPED` is outside C1: 2,003 functions call something, and 942 touch memory beyond their
own frame and read-only image data (C2). `UNKNOWN` is mostly honest: 155 CALLOTHERs
(syscalls, privileged and cache ops), about 150 floating-point ops, 15 step-budget
exhaustions, 7 lift failures, 4 decompiled flows that end in a halt, 1 return value kuna
stores in a temporary.

The datatest corpus alone (83 files, 133 functions): 22 agree, 7 unknown, 104 skipped, no
difference, 7.3 s wall time.

## 3. Every `DIFFERS` and `DIVERGES`

| test | function | verdict | cause |
|---|---|---|---|
| `kuna-narrowext` | `pi`, `pu`, `psc` | `DIFFERS` | **The bug the test pins.** Its first pass sets `narrowext off`, the compiler spec's zero extension; on RV64 the machine relies on the sign-extended argument. The `abi` pass of the same functions agrees. Found without a contract. |
| `kuna-nullable-relocations` | `same` | `DIFFERS` | **Harness.** `cmpq $imm, %rdi` carries `R_X86_64_32S optional` against a weak symbol. kuna reads the weak address through a nullable slot; the machine side reads the loader's placeholder in the immediate. |
| `kuna-switchrunon`, `kuna-switchrunonguard` | `pick`, `pick`, `pick2` | `DIVERGES(switchrunon-cut-row)` | The run-on check cuts a bounds-free table, by design. On a selector past the kept rows the machine runs the next function's code. |

Precision on this corpus: of 4 `DIFFERS`, 3 are a known bug and 1 is a harness limit.

## 4. Harness defects found while building C1

Each of these first showed up as a `DIFFERS` or an inflated `AGREES`, and each would have
been reported as a kuna bug. They are why the corpus run is part of the slice. The first
run with current specs had 30 `DIFFERS`; 3 of them were the `narrowext` bug.

| defect | effect before the fix |
|---|---|
| Compiled specs older than their sources in the main tree | 212 files failed to load, and every ARM, x86 and RISC-V verdict was about old semantics |
| Random upper bits in an argument register the ABI says the caller extends | 13 false `DIFFERS` on RV64, MIPS64 and PPC64 (`narrowext`'s fixed pass, `wide-shifts`, `constptrdeclared-riscv`, `mips-o32-register-slots`) |
| A written stack varnode not visible to a later LOAD of the same slot | 12 false `DIFFERS` (`arrayextent`, 4 functions decompiled 3 times) |
| A return value kuna keeps in a temporary, read from the machine's temporaries | 1 false `DIFFERS` (`gobool`) |
| Float ops whose names lack `FLOAT_` (`TRUNC`, `INT2FLOAT`) failing on both sides, which counted as agreement | functions reported `AGREES` though neither side ran the op |
| The jump-table placeholder label `NO_LABEL` masked to the selector's width matching a real selector | 1 false `DIFFERS` (`pick2` on `sel = 0xe1`) |

## 5. Cost

| | first build | now |
|---|---:|---:|
| check time, whole corpus | 49.8 s | 3.6 s |
| wall time, whole corpus | 111 s | 20.6 s (14 workers), 66 s (1) |
| wall time, `tests/datatests` | 7.3 s | 0.9 s |

- The first build spent 47.2 s of its check time in the 15 functions that exhaust the
  step budget on every random input (loops bounded by an argument). A function now stops
  after 4 such runs (`Options::budget_runs`); its verdict is `UNKNOWN(budget)` either way.
- The decompiled function is compiled once into a flat program (`program.rs`) that every
  run reuses, and registers live in a dense file that memoizes their initial bytes.
- Wall time is now mostly decompiling. Peak RSS is 454 MB for one process.
- Every verdict is identical before and after these changes, compared function by
  function over the corpus, sequential and parallel alike. No solver and no compiler are
  involved.

## 6. What building C1 changed in the design

1. **Machine side.** `EmulatePcodeCache` needs an `Rc<dyn Translate>`, and the
   `Architecture` owns its translator. The machine side therefore lifts each instruction
   with the program's own translator (`one_instruction`) and runs the ops itself, with the
   same `OpBehavior` table `EmulatePcodeCache` uses. Both sides share that table; integer
   ops on values of 9 to 16 bytes (a 64-bit multiply-high) are defined in `kuna-verify`,
   and a unit test checks them against the table at 1, 2 and 4 bytes.
2. **Inputs keep the ABI's promises.** A 32-bit `int` reaches a 64-bit RISC-V or MIPS
   register sign-extended, and a caller cannot pass anything else. Inputs follow the
   documented rule (`narrowext abi`, through `kuna_narrowext::rules_for`) whatever the
   run's own `narrowext`, so a run with the option off is checked against the real
   convention. The rule applies to the **claimed** parameter widths. A prototype that claims
   a parameter too narrow constrains the inputs to its claim; parameter types in contracts
   remove that limit.
3. **Switches.** After jump-table normalization a BRANCHIND reads the switch variable,
   not an address. The decompiled side takes the row whose label matches the selector,
   else the table's default edge. A selector no row carries, on a table without a
   default, is `UNKNOWN(no switch row)`.
4. **Declared divergences are predicates over observables, not engine flags.** The final
   `Funcdata` does not keep whether `switchrunon` cut a table. `switchrunon-cut-row` matches
   instead when a switch meets a selector no row carries and the machine's indirect branch
   lands outside every block of the decompiled function.
5. **Exit-only user ops.** ARM's `bx lr` emits `setISAMode` before the RETURN. It is a
   no-op before the function's exit and refused before a branch inside it.
6. **Halting flow.** A RETURN that kuna marks as a halt (a cut or undecodable flow) has no
   defined value, so a run that reaches one is `UNKNOWN`.
7. **Relocated immediates** are a known limit (§3): the machine side reads the loader's
   placeholder where kuna reads the relocation's target. C2 needs the relocation's runtime
   value on the machine side.
