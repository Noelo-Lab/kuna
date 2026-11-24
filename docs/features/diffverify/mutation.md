# diffverify — mutation testing of the check

Tests that pass say the check does what its tests ask. They do not say how much of what
can go wrong in a decompile it would see. `kuna-verify mutate` measures that: it plants
one semantic fault at a time in kuna's side of a function that agrees, and counts how
many the check reports as `DIFFERS`.

```sh
kuna-verify mutate tests/datatests tests/stages --jobs 10            # the test corpus
kuna-verify mutate <binary> --all --jobs 10 [--concolic 16]          # a binary
kuna-verify mutate <binary> <function>... --per-function 6 --json    # some functions
```

## 1. What a mutant is

A mutant changes one op of the compiled decompiled side (`src/program.rs`) or one call
site, never the machine side. Up to `--per-function` mutants (6) are taken per function,
one operator after another, so each kind is tried where the function has one.

| operator | the fault |
|---|---|
| `opcode` | an arithmetic, logic, comparison or extension op becomes a near one: `+`/`-`, `&`/`|`, signed/unsigned compare, `<=`/`<`, `==`/`!=`, zero/sign extension, logical/arithmetic shift, signed/unsigned division |
| `constant` | a constant input's lowest bit flips |
| `swap` | the inputs of an op that does not commute trade places |
| `branch` | a conditional branch goes the other way |
| `drop-store` | a store does nothing |
| `call-target` | a direct call goes to another function: the call site's target, which is what the printed C names a direct callee by |
| `drop-arg` | a call loses its last argument. A recovered prototype loses the parameter too, as when kuna recovers one too few; a declared one (DWARF, libc types, a datatest's `parse line`) keeps it, as in #1922 |

A mutant whose op no run reaches cannot be seen by any check, so recall counts only the
mutants a run reached. A mutant that ends `UNKNOWN` (a budget overrun, a trap) is counted
apart, neither caught nor missed. Only functions that agree unmutated are mutated.

## 2. What it found in the check

The first runs showed three places where a real fault would have passed. All are fixed:

- **A direct call's target was compared only when both addresses were code in the
  image.** A datatest's callee usually lies outside the bytes it maps, and a PIE's PLT
  stubs, followed through `BIND_NOW` GOT slots that the file leaves zero, all led to the
  same address. A call to the wrong function, the wrong import included, passed. Targets
  are now compared whenever each is code or outside the image, and two PLT stubs that only
  meet at a non-code address are different calls. A target in mapped data is still left
  alone: kuna binds 32-bit PowerPC PIC imports to their GOT slot.
- **A call that passed fewer arguments than the prototype kuna gives its callee was not
  compared.** The argument comparison zipped the two sides' argument lists and stopped at
  the shorter. It now reports `kuna passes N arguments, and the prototype it gives the
  callee takes M`.
- **The first `drop-arg` was not a fair test.** It shortened the call and left the
  prototype, so the check above caught every one by construction. A recovered prototype
  now shrinks with the call.

None of these changes the verdict of any function of the corpus, `ls` (`O2`, `O0`), `find`
or the decbench sample set.

## 3. Recall

| | corpus | `ls` (`O2`) | `ls`, `--concolic 16` | `find` (`O2`) |
|---|---:|---:|---:|---:|
| mutants reached | 15,211 | 1,142 | 1,152 | 1,905 |
| caught | 11,726 (77.1%) | 902 (79.0%) | 913 (79.3%) | 1,510 (79.3%) |
| `branch` | 86.9% | 86.7% | 89.2% | 81.6% |
| `call-target` | 84.7% | 85.0% | 67.0%* | 84.4% |
| `constant` | 77.9% | 57.8% | 70.4% | 72.7% |
| `drop-arg` | 47.5% | 91.1% | 91.2% | 86.2% |
| `drop-store` | 89.5% | 66.2% | 70.3% | 82.2% |
| `opcode` | 78.5% | 79.8% | 82.2% | 75.0% |
| `swap` | 78.2% | 89.3% | 89.3% | 78.9% |

\* measured before the PLT fix of §2.

What is left, by kind:

- **Values random runs rarely hit.** `x <= 9` against `x < 9`, a constant `0x3b9aca07`
  against `0x3b9aca06`: the two differ on one input. `--concolic` raises `constant` on `ls`
  from 57.8% to 70.4%.
- **Arguments a stub never reads.** A callee outside the image gets the stub, which does
  not depend on its integer arguments. An argument kuna drops along with a recovered
  parameter is seen only where the callee runs for real and reads it (`the callee reads
  RDI; kuna's call passes nothing there`). In the corpus most callees lie outside the
  bytes a datatest maps, so `drop-arg` with a recovered prototype is caught in 23%; on `ls`
  and `find`, whose callees are in the image or declared, in 86–91%.
- **Calls the check leaves alone by design.** A call bound past a PLT stub or replaced by
  an injection (`resolved`), and a target in mapped data, are not compared.
- **Mutants that change nothing.** Flipping a constant that only feeds a mask's unused
  bits, swapping the inputs of a comparison both sides of which are equal on every run,
  or a store to a stack slot nothing reads again: no check can see these.

## 4. What it does not measure

Mutants change the final IR, and the check runs that IR. A fault in the C printer (a cast,
an operator's precedence, a type's spelling) is not in the IR, so `AGREES` cannot rule it
out and this recall does not cover it. That is Tier P's axis (`proposal.md`), and it is
measured separately when that tier exists.
