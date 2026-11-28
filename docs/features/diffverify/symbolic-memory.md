# Symbolic memory and acyclic functions

`kuna-verify prove --whole-function` checks an x86-64 ELF function over the
extent of its text symbol. It explores raw machine instruction targets
and Kuna IR successors independently. The IR encoder selects SSA join
inputs by predecessor. Whole-function observations include the chosen
integer return, arithmetic definedness, and final bytes in observed
memory windows.

```sh
K=decompiler/target/release/kuna-verify
"$K" prove fixture.elf store_if --whole-function --return RAX:8 \
  --input RDI --input RSI --sleighpath specs \
  --memory 0x60000100:16 --range RDI 0x60000100 0x60000108 > memory-proof.json
```

Build `fixture.elf` as described in the [function oracle guide](function-oracle.md).
The inclusive pointer range lets each eight-byte store fit in the sixteen-byte
window, including unaligned and overlapping accesses.

- `--memory ADDRESS:BYTES` declares independently quantified initial bytes and
  observes their final values. Backings include writable ELF data, the synthetic
  heap at `0x60000000`, and the caller page at `0x70003000`.
- `--private-memory ADDRESS:BYTES` models ephemeral stack storage below the
  fixed entry stack pointer `0x70002ff8`. Its final contents are outside the
  projection. A final-IR read of a private byte before a write limits coverage.
- `--image-memory ADDRESS:BYTES` reads fixed bytes from a readable,
  nonwritable ELF mapping, including jump-table entries.
- `--indirect-target SITE:DESTINATION` declares a possible raw jump target.
  Repeat it for each candidate. The raw and final-IR path encoders determine
  which targets their selectors can take. The coverage query checks that each
  permitted input stays within the declared targets.

The selected memory windows may span up to 256 bytes in total. Multi-byte
accesses respect byte order, length and permissions. The encoders propagate
memory effects along each path using byte address equality. The initial
bytes and symbolic register constraints are shared. Whole-function mode
recognizes the one-byte x86 near return with its original stack slot and
checks restoration of the entry stack pointer. Cycles require an explicit
loop bound. Calls, external effects and unsupported instructions are
reported as incomplete.

Proof artifacts include the input satisfiability, coverage and mismatch
queries. A satisfiable mismatch in the covered subset takes priority over
incomplete coverage elsewhere. When the mismatch query is unsatisfiable,
the coverage query must also be unsatisfiable before the report can say
`MODEL_EQUIVALENCE`. Its evidence applies to the declared inputs and
observables.

```sh
"$K" prove fixture.elf caller_write --whole-function --return RAX:8 \
  --input RDI --sleighpath specs --memory 0x70003000:8 > store-counterexample.json
python3 scripts/diffverify/proof_replay.py store-counterexample.json fixture.elf --sleighpath specs
```

The witness carries its initial register and memory bytes. Replay verifies
the archived queries, ELF and SLEIGH image, restores those bytes, then runs
Kuna and Unicorn. On `caller_write`, the independent x86 backend and raw
p-code observe the store that the final IR omits.

## Bounded loops

```sh
"$K" prove fixture.elf count_down --whole-function --return RAX:8 \
  --input RDI --loop-bound 4 --sleighpath specs > bounded-proof.json
```

`--loop-bound N` permits 1 to 32 visits to each raw instruction and each
final-IR block along a path. The encoders count visits separately, select the
incoming SSA values at each loop entry, and merge the terminal states by
path predicate. The report records the visit, encoding step, depth and solver
limits. Calls and recursion are outside this model.

- `BOUNDED_EQUIVALENCE` covers inputs for which both executions reach a
  modeled return or arithmetic trap within the limit. The solver must find
  an input in that domain and establish that no such input differs.
- `MODEL_EQUIVALENCE` also requires the solver to rule out inputs that
  reach either loop cut. For `count_down`, add `--range RDI 1 4` to obtain
  that result with four visits.
- `INCONCLUSIVE` covers a witnessed memory or control-flow coverage gap,
  an empty bounded domain, a spent encoding or solver budget, and unsupported
  execution. Adding `--range RDI 5 10` to the example yields an empty bounded
  domain.

The artifact includes separate queries for reaching a cut and for the
satisfiability of the jointly completed input domain, with hashes for both.
A mismatch in that domain yields a counterexample even if a different
input reaches a cut. For example, run `add_with_loop` with `--input RDI
--input RSI --input RDX --loop-bound 4 --opcode-mutant 0`. The fixture's
entry addition changes to subtraction in the final IR, and
`proof_replay.py` can replay the resulting mismatch in Unicorn.
