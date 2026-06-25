# [PROPOSAL] SAILR-style condition folding / crossing-edge goto reduction (S7)

**Opportunity:** `test_decompiling_tr_build_spec_list::build_spec_list`
**Binary:** `binaries/tests/x86_64/decompiler/tr.o` (coreutils `tr`, x86-64; ET_REL)
**Proposed option:** `condfold` (S7 region structuring, default-OFF)
**Scope:** **LARGE** — needs human go/no-go before any implementation worker is spent.

## The problem

For `build_spec_list`, kuna's full-pipeline output (`kuna-fullpipeline.c`) is structurally
close to angr's (`angr-vs-kuna.txt`) — DWARF types and loops match, `__stack_chk_fail` is
already no-return — **except** that kuna emits **11 gotos** with *crossing edges* where angr
emits **4**, because angr collapses short-circuit control flow into compound conditions.

Concretely, the bracket-class dispatch:

```c
// angr — one compound condition, no crossing goto
if (!(char)es_match(ptr, cur, '[') ... )
if (!(char)es_match() && !(char)es_match() || !(char)find_closing_delim(ptr, v11, ..., &v7)) {
LABEL_400d08: ...
}
```

```c
// kuna — nested if/else with a crossing-edge goto back into the sibling branch
v2 = es_match(v15, v10, ':');
if (v2) { ... find_closing_delim ... if (!v2) goto label_4024b8; ... }
else {
  v2 = es_match(v15, v14, '=');
  if (v2) goto label_40242e;     // <-- jumps from the else branch back into the then branch
label_4024b8: ...
}
```

The `goto label_40242e` is a crossing edge that angr removes by recovering
`es_match(':') || es_match('=')` as a single compound condition.

## The angr reference

angr's decompiler (Basque et al., *SAILR*, USENIX Security 2024) produces this via:

- **`RegionIdentifier` + `Phoenix` structuring** — synthesizes compound `&&`/`||` condition
  nodes when two condition blocks share a successor (the short-circuit schema).
  (`angr/analyses/decompiler/structuring/phoenix.py`, `_make_condition_node` /
  `_match_acyclic_schemas`.)
- **SAILR optimization passes** — `CrossJumpReverter`, `DuplicationReverter`,
  `ConditionConstantPropagation` (`angr/analyses/decompiler/optimization_passes/`) that undo
  compiler control-flow duplication so the crossing edges disappear.

Ghidra (and thus kuna today) structures with the BlockGraph collapse rules
(`blockaction.cc`), which do **not** synthesize short-circuit compound conditions across
distinct basic blocks — they emit a goto for the crossing edge instead.

## Why this is LARGE (Hard Rule 7)

- It is **S7 structuring** logic, not a single gated early-return: it must add condition
  synthesis to the region collapse and/or a pre-structuring deoptimization pass.
- It is a **new pass type / infrastructure**, not one `Action`/`Rule` modeled on
  `kuna_loweredswitch.rs` (which manufactures an S2 artifact and reuses the existing
  structurer). Compound-condition recovery has no existing seam to reuse.
- It will likely touch **>3 ported-core anchor files** (`blockaction.cc`/the S7 region
  collapse, `block.cc` for a new compound-condition block kind, the C-printer for `&&`/`||`
  emission, plus the option/registration anchors) and **>1 module**.

Any one of these alone triggers the LARGE classification.

## Proposed multi-step implementation plan (for the approved worker)

1. **Detect the short-circuit schema** in the S7 region graph: two `BlockCondition`s whose
   true/false edges share a target (the `A && B` / `A || B` shapes), pre-collapse — port of
   `Phoenix._make_condition_node`. Gate behind `option condfold`.
2. **Synthesize a compound-condition region node** (a new `BlockCondition` whose predicate is
   `BoolAnd`/`BoolOr` of the two operands) and rewire edges so the crossing goto becomes a
   structured edge.
3. **C-printer support** for emitting the folded `&&`/`||` (likely already present for
   in-block boolean ops; verify it renders for the synthesized node).
4. **Crossing-edge deoptimization** (SAILR `CrossJumpReverter`) as a follow-up sub-step if
   step 1–2 do not remove all crossing edges on the corpus.
5. **Stage test** `tests/stages/ghangr-tr-build-spec-list-ea32be.xml`: pass 1 (`option
   condfold off`) asserts the crossing `goto`; pass 2 (default-on for the test) asserts the
   compound condition and goto reduction.

## Speed / risk assessment

- **Risk: HIGH.** S7 structuring changes are the riskiest in the engine — the 675-datatest
  parity gate is very sensitive to block-collapse order. Expect a large ablation; the option
  must ship **default-OFF** opt-in regardless of speed.
- **Speed:** an extra schema-match scan over region nodes is O(blocks); modest, but must be
  measured on `build_spec_list` off-vs-on per Hard Rule 6 before shipping.
- **Generalizes:** yes — short-circuit `&&`/`||` lowering is ubiquitous in `-O2` GCC/Clang
  output, so a correct port helps far beyond this one function.

## Recommendation

Approve as a **dedicated S7-structuring implementation worker** (not this single-feature
slot). The smallest viable first cut is steps 1–3 (compound-condition synthesis for the
acyclic short-circuit schema), gated `option condfold`, default-off, with the stage test
above; defer the SAILR deoptimizers (step 4) to a follow-up if needed.

> Note: the *target binary itself* (`tr.o`) cannot be loaded by kuna (ET_REL loader gap,
> proposal PR #37). The approved worker should use the linked standalone ELF approach
> documented in `analysis.md` (`stubs.c` + `tr_synth.elf`) or a relocated bytechunk
> (`probe.xml`) for its stage test, exactly as done here.
