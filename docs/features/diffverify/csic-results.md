# diffverify — CSIC: counterfactual witnesses for a `DIFFERS`

`kuna-verify intervene` takes a run that differs and replays it with one change to kuna's
side at a time. Each change after which the run agrees is a **counterfactual witness**:
evidence that the result depends on that op. The idea comes from Fission RFC #1101's CSIC
track. The code is kuna's own (`src/tools/intervene.rs`). AEQ, the RFC's other track, is
design only (`aeq-design.md`).

```sh
kuna-verify intervene tests/stages/kuna-narrowext.xml csc --localize
```

```
csc (decompile 1)  DIFFERS (run 0)
  first_observable_mismatch  machine a0 = 0xffffffffffffff94; kuna int8 csc(int4 k) = 0x0000000000000294
  causal_status  WITNESS_FOUND
  minimality  NOT_PROVEN
  witness 1  (value)
    candidate_origin  0x10036 INT_ZEXT (block 0, op 2)  [rule propagatecopy application 8 of 11]
    intervention  do(register:0x2050:8 := machine's value) ...
    counterfactual_replay  AGREES (run 0)
  witness 2  (structural)
    candidate_origin  0x10036 INT_ZEXT (block 0, op 2)  [rule propagatecopy application 8 of 11]
    intervention  do(INT_ZEXT -> INT_SEXT)
    counterfactual_replay  AGREES (run 0); over every run: AGREES(256)
```

## 1. Method

1. Check the function. Without a `DIFFERS` there is nothing to search.
2. Replay the differing run once while recording both sides. kuna's side records each op's
   output as it runs. The machine side records, after each instruction that a kuna op
   carries the address of, the storage that op writes (register, stack slot, memory).
   The probe saves and restores the state's read tracking, so recording does not change
   the run.
3. Build candidates, in the order their op first ran:
   - **value**: an op whose output differs from the machine's value at the same
     instruction (the n-th write of that storage at that address against the n-th
     execution of the instruction). The change sets every execution of the op to the
     machine's values. This holds for the recorded run only.
   - **structural**: for each op that ran, another opcode of its family (`+ - * & | ^`,
     the four orderings, `== !=`, the three shifts, zero/sign extension, signed/unsigned
     division and remainder), swapped inputs of an ordered op, an inverted branch, or a
     constant moved by one bit or by ±1.
4. Replay the recorded run once per candidate, at most 400 times (`MAX_REPLAYS`). A
   structural witness is then replayed over every run (at most 8 per search).
5. Report every witness, the earliest first, with `causal_status` (`WITNESS_FOUND` or
   `NONE`) and `minimality: NOT_PROVEN`. `--localize` adds the rule application
   `localize` names at a witness's address.

## 2. What it does not claim

- **Not a cause.** One change that makes the run agree is a counterfactual dependence
  on that run. No joint interventions or Halpern–Pearl contingencies are searched, so a
  witness is never called the cause, the only cause, or a minimal one.
- **Several witnesses are normal.** An error propagates. Setting any op downstream of
  it to the machine's value also repairs the run, and flipping the compare that feeds a
  branch is equivalent to flipping the branch. The earliest witness is a ranking, not a
  proof.
- **Value witnesses hold for one run.** Their values come from that run's machine side.
- **No witness is not evidence of an action.** It means only that no single change in
  these families repairs the run.

## 3. Measured

**Planted faults**, the test corpus (`tests/datatests`, `tests/stages`), 12,112 caught
mutants. "any" is a witness on the mutated op. "first" is the earliest witness on it.

| operator | searched | witness | first | any | value on it | structural on it |
|---|---:|---:|---:|---:|---:|---:|
| `opcode` | 3,178 | 99.9% | 92.2% | 99.8% | 50.7% | 99.5% |
| `constant` | 3,548 | 99.8% | 91.2% | 99.8% | 38.2% | 99.7% |
| `swap` | 438 | 100.0% | 82.9% | 99.8% | 37.9% | 99.8% |
| `branch` | 1,025 | 99.7% | 8.2% | 99.4% | 0.0% | 99.4% |
| `drop-store` | 695 | 6.5% | 0.0% | 0.0% | 0.0% | 0.0% |
| `call-target` | 1,629 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| `drop-arg` | 712 | 0.4% | 0.0% | 0.0% | 0.0% | 0.0% |
| `noreturn` | 887 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| all | 12,112 | 67.9% | 54.6% | 67.4% | 25.9% | 67.3% |

Read these with care:

- **The structural numbers hold by construction.** Every mutation operator's inverse is in
  the structural families, so a structural witness on an `opcode`, `constant`, `swap` or
  `branch` mutant only shows that the search runs. **The value column is the honest
  one.** It does not know the operators. It lands on the mutated op in 26% of caught
  mutants overall, and in 51% of `opcode` mutants. It misses where the op's output is a
  temporary with no machine counterpart.
- **`branch` "first" is 8%** because the compare feeding the branch ran earlier, and
  flipping it is the same change.
- **Call facts and removed ops are out of reach.** A wrong target, a dropped argument, a
  false no-return or a dropped store is not an op value or an opcode. A dropped store
  sometimes gets a value witness at the load that reads it (6.5%).
- Searches took 3 ms at the median and 101 ms at the 90th percentile, 5,102 s in all.
  The tail is long: 0.5% hit the replay budget, and one function with a long loop
  (`kuna-loadguardreads` `gather`) took 334 s for a single mutant, since each of its 400
  replays runs the whole loop. `--intervene` is a measurement flag, not something to
  turn on for every `mutate` run.

**Real bugs**: the 91 stage-test functions whose bug pass differs in
`docs/baseline-verify.json`. 26 get a witness (value 18, structural 14). None hit the budget.
`kuna-narrowext`'s `csc`, `pi` and `pu` get `INT_ZEXT -> INT_SEXT`, which agrees on every
run. That is exactly the extension the `narrowext` option fixes. Most of the 65 without a
witness are system calls, return-register and calling-convention cases
(`kuna-syscallregs-*`, `kuna-reentereddispatch`, `kuna-calleeretpreserves`): faults in call
facts or missing outputs, which the two families cannot change.

## 4. Limits and next steps

- Corpus only. `ls` and `find` were not measured.
- Correspondence is by instruction address and storage. A temporary, an op moved to
  another address, or an instruction that writes the same register twice gives no value
  candidate, or a wrong one. The replay rejects a wrong one.
- Next: interventions on call facts (target, argument count, no-return) to reach the
  families with 0%, and pairs of interventions to report contingencies.
