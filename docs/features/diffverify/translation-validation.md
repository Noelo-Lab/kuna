# Integer translation validation

`kuna-verify prove` compares a straight-line x86-64 raw p-code interval with
final-IR paths to the selected return. It gives the raw interpreter
byte-addressed register and temporary storage. The IR interpreter binds its
own SSA slots to terms. They share the user-declared input bytes and the
strict bit-vector operation encoding.

## Run it

Build a fixed-address x86-64 ELF as shown in the
[function oracle guide](function-oracle.md). The example below uses its
`add` fixture. Install Z3 and build `kuna-verify`.

```sh
set -- $(llvm-nm --format=posix --print-size fixture.elf | awk '$1 == "add" { print $3, $4 }')
ret=$(printf '0x%x' "$((0x$1 + 0x$2 - 1))")
K=decompiler/target/release/kuna-verify
"$K" prove fixture.elf add --stop-at "$ret" --input RDI --input RSI \
  --return RAX:8 --sleighpath specs > proof.json
```

`--stop-at` names the address of the one-byte near return. The proof
interval ends immediately before that instruction. The return projection
specifies a physical integer register and a width in bytes. Each
`--input` contributes all eight bytes of an x86-64 general-purpose register.
Reads of other entry registers are outside the model. `--bind REG VALUE`
constrains an input register to an explicit value. The report archives the
ELF and SLEIGH image digests, the solver version, both SMT-LIB queries and
their digests.

```sh
jq -r '.obligation.domain_smt2' proof.json | z3 -in -smt2
jq -r '.obligation.difference_smt2' proof.json | z3 -in -smt2
```

The first query checks that the input restrictions admit a state. The
second looks for different modeled arithmetic outcomes or return bits.
The arithmetic model preserves exact widths and signedness, and includes
the p-code division-by-zero and 64-bit signed overflow conditions. The
first query must return `sat` and the second `unsat` for
`MODEL_EQUIVALENCE`. Its scope is the selected interval and observation.
The encoder handles forward branches within a machine instruction and
selects final-IR SSA join inputs using each predecessor edge. Arithmetic
flags are modeled as one-bit inputs and included in the replay witness.
Memory effects, calls, branches across machine instructions, loops, floating point and
unmodeled operations yield `INCONCLUSIVE`, as do unsatisfiable input
restrictions and solver failures. The solver deadline is a wall-clock
limit shared by the queries. [SMT-LIB fixed-size bit-vectors](https://smt-lib.org/theories-FixedSizeBitVectors.shtml)

## Replay a model counterexample

The optional opcode mutation uses the existing DiffVerify entry-block
mutator. Mutation zero for the `add` fixture turns its IR addition into
subtraction while the raw p-code and ELF stay the same.

```sh
"$K" prove fixture.elf add --stop-at "$ret" --input RDI --input RSI \
  --return RAX:8 --sleighpath specs --opcode-mutant 0 > counterexample.json
python3 scripts/diffverify/proof_replay.py counterexample.json fixture.elf --sleighpath specs
```

`prove` exits 1 for a model counterexample and records the witness register
values. Replay feeds them to both Kuna interpreters and Unicorn. It checks
the ELF and SLEIGH digests and the mutation description before comparing
the results. Replay confirms a difference when Unicorn and raw p-code
agree against the final IR at the scoped observation. The independent
execution evidence remains separate from the SMT model result.

The x86-64 fixture checks addition, sign and zero extension, logic and
shifts, rotation, and a conditional move. The mutation controls replace
addition with subtraction and replace a sign extension with a zero
extension. Replaying the latter with `RDI=0x80000000` gives
`0xffffffff80000000` in Unicorn and raw p-code and `0x80000000` in the
mutated final IR.

The 26-case Python integration suite completed in 6.90 s with an 86 MiB
reported peak resident set on macOS arm64 using Clang 22.1.8, Unicorn 2.1.2
and Z3 4.13.0. This run includes fixture compilation, subprocess startup
and the concrete oracle tests.
