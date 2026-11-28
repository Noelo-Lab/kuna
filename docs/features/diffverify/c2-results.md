# diffverify — slice C2 results

C2 adds memory and calls to Tier C (proposal, Slices). The machine side and the
decompiled side now load and store anywhere, and every call runs the same stub on both.
This file records how the model works, what it caught, and what is left for C4.

```sh
decompiler/target/release/kuna-verify check callmem.o --all
decompiler/target/release/kuna-verify corpus tests/datatests tests/stages
```

## 1. The model

**Memory.** The initial state defines every byte. A random pointer argument therefore
reads the same bytes on both sides, and the check never needs to know which arguments
are pointers. The observable is every byte either side leaves different outside the
stack, compared at each call and at the exit.

**Calls.** Each call is compared in order on the call instruction's address. A computed
target is compared too, when both sides compute it (`CALLIND`). Its target is wrapped to
the code space and loses the ARM instruction-set bit. kuna's arguments are compared at
the storage its callee prototype gives them. A difference is `DIFFERS` only when that
prototype is declared (`is_input_locked`). A recovered one makes it a
`PROTOTYPE_SUSPECT`, as a return value is without a contract. Then the call has the same
effect on both sides (`calls.rs`, `State::call`):
- it kills the registers its model lists as killed-by-call and every output register,
  with a value fixed by the run and the call's index;
- it pops `extrapop` bytes of stack;
- it may write the image's writable data and 4 KiB past every pointer passed to a
  call so far. The memory is rewritten lazily: a byte this side did not write since
  reads a value fixed by the call's index.

That reachable set is never larger than what kuna itself treats as changed by a call. So
a value kuna keeps across a call stays equal unless kuna is wrong to keep it. A first
version that let a call write all memory outside the stack also rewrote the TLS stack
canary between the prologue and the epilogue. kuna knows the canary does not change, so
that version reported false `DIFFERS` (§4).

**Calls kuna does not make.** Where kuna has no call at an instruction that calls code
in the image, kuna inlined, injected or dropped it. The machine runs that callee for
real, and its RETURN comes back to the caller. A deliberate removal then agrees, and a
wrong one shows up in what the callee does. A call kuna lacks to code outside the image
gets a stub and is reported as a call only the machine makes.

**INDIRECT.** An INDIRECT stands before the op whose effect it describes, so the
compiled program moves each one after that op. Its output is then:
- the call's value for a register the call kills;
- what memory now holds for a stack or memory location;
- its input otherwise.

**Limits.** A run makes at most 256 calls, so a loop of calls bounded by an argument
ends in `UNKNOWN(budget)` and does not grow without bound. No-return calls end both runs.
Memory builtins (`memcpy`, `strlen`, …) are `UNKNOWN(unsupported-op)`.

## 2. The proposal's two checks

Neither bug exists in kuna today, and their options were removed when the fixes became
unconditional (#1524). Each was put back for one run, in a local build only, behind an
environment variable that is not in this branch.

| injected fault | function | default build | with the fault |
|---|---|---|---|
| #801: `Funcdata::stack_store_guard()` returns `false` | `inspect_bytes` (`kuna-stackstoreguard.xml`) | `AGREES(256)` | `DIFFERS`: `memory at exit: ram:0x50000000: machine 0x80, kuna 0x08`, with the note that the machine ran `helper` itself where kuna has no call |
| `guard_calls` adds no INDIRECT for `ram` | `twice` (`int a = counter; tick(); return a + counter;`) | `AGREES(256)` | `DIFFERS`: `machine EAX = 0xb1186181`, `kuna int4 twice() = 0x0000000a` (the value loaded before the call, used twice) |

The second fixture is `kuna-verify/tests/call_effects.rs`, which pins the default
build's `AGREES`.

## 3. The corpus

`tests/datatests` and `tests/stages`, 3,979 decompiled functions, 256 runs each.

| verdict | C1 | C2 |
|---|---:|---:|
| `AGREES(256)` | 696 | 2,482 |
| `DIFFERS` | 4 | 500 |
| `DIVERGES` | 3 | 3 |
| `UNKNOWN` | 331 | 994 |
| `SKIPPED` | 2,945 | 0 |

Every function C1 found to agree still agrees, and its `DIFFERS` and `DIVERGES` are
unchanged, compared function by function.

The 496 `DIFFERS` beyond C1's four are not triaged; that is C4's job. Their first differences, by
kind:

| first difference | count |
|---|---:|
| return value | 147 |
| calls from different instructions (the two sides took different paths) | 83 |
| a call only the machine makes | 77 |
| memory before a call | 76 |
| memory at the exit | 41 |
| an argument to a declared callee | 42 |
| a call only kuna makes | 17 |
| a computed call target | 13 |

A sample of the largest kind has most of them reading a stack local that a callee wrote
through a pointer: hidden return slots (`gh854-hiddenretarg-*`), and locals whose
address escaped (`varcross`, `partialmerge`). That is where the model's stack handling
gets checked next.

## 4. Harness defects found while building C2

| defect | effect before the fix |
|---|---|
| A call's target read from the CALL op's first input, which by the final `Funcdata` names a call spec rather than an address | 1,560 false target differences |
| A call allowed to write all memory outside the stack, the TLS canary included | the canary check failed in every function with a stack protector, and the machine called `__stack_chk_fail` |
| A switch row whose address is not an out-edge's block start sent to the default edge | 1 false `DIFFERS` (`regionstructure-switch`); in C1 it hid behind a `void` return |
| A branch inside a delay-slot call instruction taken for a tail call | false extra calls on SPARC |
| Arguments read from the CALL op's input varnodes, which are temporaries | 746 false argument differences |
| Calls in a loop each keeping a memory snapshot | one function took 462 s; now 32 ms, capped at 256 calls |

## 5. Cost

Check time 50.3 s, wall time 11.0 s on 14 worker processes, peak RSS 332 MB for one
process. The check time grew because 2,945 skipped functions now run. Wall time fell
from 20.6 s: functions that loop over calls now stop at the call limit.
