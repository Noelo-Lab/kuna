# Independent RV64 instruction semantics

`sail_rv64.py` checks a fixed-address RV64 ELF leaf containing one 32-bit
integer ALU instruction and an ordinary return. It takes the instruction
bytes from the executable and runs the published Sail model through Isla.
Kuna separately exports the raw p-code and final-IR terms for the return
value. The two comparisons share the same symbolic 64-bit argument
registers and the same full-width `a0` projection.

```sh
clang --target=riscv64-linux-gnu -march=rv64i -mabi=lp64 -c \
  scripts/diffverify/fixtures/formal_rv64.s -o rv64.o
ld.lld -static -z separate-loadable-segments -e rv_add rv64.o -o rv64.elf
python3 scripts/diffverify/sail_rv64.py rv64.elf rv_add \
  --isla "$ISLA" --architecture "$SAIL_IR" --config "$ISLA_CONFIG" > rv64-proof.json
```

The tested setup uses [Isla e9b5d94](https://github.com/rems-project/isla/tree/e9b5d945394277656593a0d429466d7fa0a2b4b3)
and [the published rv64d.ir Sail snapshot](https://github.com/rems-project/isla-snapshots/blob/d8b31014643035a3b11071e56ef30001de3f52ab/rv64d.ir).
Build `isla-footprint` with `cargo build --release --bin isla-footprint`.
Use Isla's `configs/riscv64.toml`. On hosts without the named GNU
cross-toolchain, set the configured tool entries to installed tools
such as `clang --target=riscv64-linux-gnu -march=rv64i -c -x assembler`,
`llvm-objdump`, `llvm-nm`, and `ld.lld`. The proof sends bytes directly
to Isla. It records the executable, snapshot, config, ELF, SLEIGH and
solver identities.

The checker first proves that the union of imported Sail traces covers
the input domain. Then it checks that Sail agrees with raw p-code and
that raw p-code agrees with final IR. Each comparison has its own
reproducible obligation. The two comparisons compose only for this
projection and these input assumptions.

The supported leaf model admits a subset of RV64I ALU instructions, with
inputs in `a0` through `a7` or the zero register. It checks `a0` at the
instruction boundary before the return. The ELF must contain the standard
32-bit `jalr x0,0(ra)` at the return site. Memory, PC updates, return
execution, compressed instructions, and narrower recovered returns lie
outside this projection. Unknown trace effects, uncovered inputs and
failed or timed-out queries yield `INCONCLUSIVE`.

The assembly fixture exercises register addition, signed-immediate
addition and arithmetic right shift. An ADD-to-SUB rule mutation in the
final IR produces a witness for `raw-vs-final-ir`. Changing the ADD lift
locally induces the same error in both Kuna paths and produces a witness
for `sail-vs-raw`. Unicorn can replay the instruction from the witness
registers when installed. The proof relies on the Sail model, Isla's
symbolic evaluation, the trace adapter, the Kuna encoders and the solver.
