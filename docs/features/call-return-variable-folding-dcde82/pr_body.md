## call-return variable folding — option `foldcallret`

Closes an angr-better gap from `test_call_return_variable_folding`
(`x86_64/decompiler/ls_gcc_O0::print_long_format`, angr 9.2.213).

See [`docs/features/call-return-variable-folding-dcde82/analysis.md`](../docs/features/call-return-variable-folding-dcde82/analysis.md)
for the full analysis.

### The gap

angr inlines a call's return value into its single use site
(`if (timespec_cmp(...) <= -1)`, `... && localtime_rz(...) != NULL ...`).
kuna spills **every** call return to a named local first:

```c
v5 = timespec_cmp();
if (v5 <= -1) { ... }
```

Root cause: S6 `ActionMarkExplicit::baseExplicit` (`coreaction.cc:3105`) forces
every call output **explicit** (`if (op->isCall()) return -1;`). This is
conservative-by-design — making a call output *implied* moves the call's
evaluation to the use site, which would reorder it past any intervening
side-effecting statement.

### The mechanism

New option **`foldcallret`** (S6 explicit-marking sub-stage, opt-in **default-OFF**).
A new module `kuna_callretfold.rs` exposes an order-safety predicate,
`call_output_foldable`, and `baseExplicit`'s `is_call()` arm falls through to the
normal implied path when the flag is on **and** the predicate holds. The predicate
folds a call output only when it is provably order-safe:

1. it has **exactly one** descendant op, and that use is **not** an SSA marker
   (MULTIEQUAL/INDIRECT phi);
2. the use op is in the **same basic block** as the call, ordered after it;
3. **no** op strictly between the call and its use is a call (`is_call`) or a
   memory op (`LOAD`/`STORE`/`CALLOTHER`).

Keeping `LOAD` in the forbidden set guards a read-after-write hazard (the call may
`STORE` memory an intervening `LOAD` reads). The predicate strictly tightens the
printer's pre-existing single-use inlining, preferring false negatives (stay
explicit) over reordering bugs.

### Flip it

```sh
kuna decompile <bin> <fn> --option foldcallret on    # fold single-use call returns
# off (default) is upstream byte-identical
```

Discoverable via `kuna catalog --json` (`source_decompiler: "angr"`,
`change_kind: "presentation-default"`).

### Ablation / parity / speed

- **Default-OFF is byte-identical**: `make test` → 675/675, **PARITY OK**.
- Flipping default-ON changes **5/675** datatest assertions (Deindirect Output #1,
  Inlining #8, Local cross #2, Modified conditional constant #2/#3) → stays
  **opt-in** (no DIV entry).
- **Speed**: off 650.95 ms / on 630.17 ms = **−3.19%** (within the 5% budget),
  median of 5 on the target function.
- New stage test `tests/stages/ghangr-call-return-variable-folding-dcde82.xml`
  (two-pass: off = the spill, on = the fold); `make test-stages` **PARITY OK**.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
