# diffverify — slice C5 results (concolic inputs)

C5 adds inputs that random runs do not reach. A machine run can record an expression
over the function's input bytes next to each value, and the condition of each branch the
function's own code takes. Negating one branch, with the branches before it held, is a
question for z3 whose model is an input that goes the other way. The model's bytes
replace those inputs in a new run, and both sides run and compare as on any run. z3 only
picks inputs; it never decides a verdict.

```sh
kuna-verify check <binary> <function>... --concolic 16
```

`--concolic N` spends up to `N` z3 queries per function after its random runs. With 0,
the default, nothing changes: the corpus baseline is the same.

## 1. How it works

- **Input bytes** (`Env::input_place`) are the ones the run's random state chooses:
  - register and stack arguments;
  - memory outside the image and the frame, before any call.

  Not inputs: tracked registers, the stack pointer, the extended part of an argument, and
  temporaries. A `bool` argument's byte is constrained to 0 or 1.
- **Expressions** (`src/concolic.rs`) cover integer arithmetic, logic, shifts,
  comparisons, carries, extensions, `PIECE` and `SUBPIECE`. A value that goes through
  anything else, through a call, or through memory a call may change, is concrete from
  there on. Each byte's expression is kept with the byte's concrete value. A later
  overwrite the recording does not follow (a callee, a stub) changes the value and drops
  the expression.
- **Branches:** only the function's own code is recorded, internal p-code branches
  included (an x86 `cmov` is one). Up to 256 branches per path are kept, from the first 16
  random runs and from every concolic run.
- **Choosing a query:** the worklist takes, over every recorded path, the first branch
  whose other direction no run has taken. A branch is its instruction's address and
  the index of its p-code op there, so the branches inside one instruction (a `rep`
  prefix's count test and the loop's) are told apart. A direction is asked about once
  per path prefix, the branches before it and how they went: unsatisfiable after one
  prefix, it can be satisfiable after another.
- **z3:** one process per function, each query pushed and popped, with 300 ms per
  query. The scope is popped whatever the answer; an answer that is not `sat`,
  `unsat` or `unknown`, or a model that does not parse, ends the use of that process.
  A model's bytes are merged with the base run's, so the rest of the state stays the
  base run's.
- **Booleans:** p-code's `BOOL_AND`, `BOOL_OR`, `BOOL_XOR` and `BOOL_NEGATE` read any
  nonzero byte as true, as the concrete evaluation does; the encoding tests each input
  against zero before combining them, so a byte other than 0 or 1 is not read
  bitwise.
- **Reports:** a `DIFFERS` on a concolic run says so:
  `concolic an input z3 gave to take the branch at 0x… the other way`. Every function
  checked with `--concolic` gets a line `concolic N z3 queries, M inputs found`, and
  in `--json` a `z3` object (`queries`, `models`, `solver`: `ran`, `unused`,
  `unavailable` or `broken`). Without z3 the line says that no concolic run was made,
  so an `AGREES` from the random runs alone is not read as a concolic one.
- **Determinism:** results are the same from run to run and for any `--jobs`, because
  the queries are made in a fixed order.

## 2. Results

**A branch random runs miss.** `magic(x)` returns 7 when `x == 0x1badb002`. A local
fault, never committed, made kuna compare against `0x1badb003`:

| | without `--concolic` | `--concolic 16` |
|---|---|---|
| `magic`, fault in kuna | `AGREES(256)` | `DIFFERS`: `x=0x1badb002`, `machine EAX = 7`, `kuna = 0` |

**The decbench sample set** (250 functions), `--concolic 16`:
- 49 functions get one or more concolic runs, 80 runs in all.
- No new `DIFFERS`.
- Two functions become `DIVERGES(divide-by-zero)`: Betaflight `mspProcessInCommand`
  and `usart_set_baudrate`. z3 found a zero divisor no random run had. ARM and AArch64
  divide by zero to 0, which the machine side now models. kuna's C `/` is undefined
  there, so this is a declared divergence, not a kuna bug.
- One function becomes `UNKNOWN(budget)`: the concolic input sent `parseRFCStructuredData`
  around a long loop.

**`coreutils` `ls`** (`O2`, 405 functions, ten workers), as first measured:
- 105 functions get 237 concolic runs.
- Wall time goes from 3.3 s to 13.4 s.
  - With a z3 process per query it was 41 s.
  - With 2 s per query it was 58 s.

With the path-aware worklist, z3 runs for 140 functions: 697 queries, 454 inputs found,
33.6 s against 13.2 s without `--concolic` on a machine busy with other builds. The
verdicts are the same.

One run with a z3 process per query reported `transfer_entries` `DIFFERS`: kuna's loop
calls the hash function once more before `abort()` than the machine does. The
persistent solver gives other models and does not reach it. It is not triaged yet.

## 3. Recall

C4 §6's misses are mostly not path problems. Of its eleven remaining misses, only #564
(`end == cap`) is one. #564's fix is no longer a guard that can be turned back alone:
forcing `access_cannot_reach_call` false leaves the stage test passing. So the recall
table is not re-measured.
