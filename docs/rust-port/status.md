# Rust port status ledger

One row per checklist item as it moves; the orchestrator appends/updates rows
at commit time. Source of truth for machine state is
`docs/rust-port/checklist.json` (`status`/`attempts` fields); this ledger is
the human-readable trail. Verdicts reference files in
`docs/rust-port/reviews/` (protocol: `verification.md`).

Columns:

- **item** — checklist item id
- **porter** — agent/person who did the port (or built the infra)
- **verifier** — agent/person who wrote the verdict (must differ from porter)
- **gate** — the item's gate command result (e.g. `cargo test -p kuna-base: ok`)
- **verdict** — ACCEPT / ACCEPT-WITH-LOSSES / REJECT (xN) / BLOCKED / n/a (infra)
- **commit** — short sha of the commit that landed the item

| item | porter | verifier | gate | verdict | commit |
|---|---|---|---|---|---|
| _(none yet)_ | | | | | |

## Blocked items (3 REJECTs — human decision needed)

_(none yet)_

## Wave gates

| gate | milestone | status | evidence |
|---|---|---|---|
| W0-gate | — | todo | |
| W1-gate | — | todo | |
| W2-gate | — | todo | |
| W3-gate | — | todo | |
| W4-gate | — | todo | |
| W5-gate | — | todo | |
| W6-gate | — | todo | |
| W7-gate | — | todo | |
| W8-gate | M1: 207/207 cargo test | todo | |
| W9-gate | M2: datatests end-to-end `--engine rust` | todo | |
| W10-gate | M3: PARITY OK 675/675 + 150/150 + catalog --check | todo | |
| W11-gate | M4: real-ELF decompile + pipeline binding | todo | |

## W1 (2026-06-11)

| item | porter | verifier | gate | verdict | commit |
|---|---|---|---|---|---|
| w1-base-foundation | fleet | independent | cargo test -p kuna-base | ACCEPT | a063f9b |
| w1-base-xml | fleet | independent | corpus+fuzz parity 6485 cases | ACCEPT-WITH-LOSSES | 95d2366 |
| w1-num-float-multiprec | fleet | independent | golden_float 3607 rows | ACCEPT | a2fccc3 |
| w1-base-util | fleet | independent | crc KAT + zlib interop | ACCEPT-WITH-LOSSES | f3c6a8a |
| w1-base-marshal | fleet | independent | packed/xml round-trips | ACCEPT-WITH-LOSSES | 1549215 |
| w1-base-space-address | fleet | independent | golden_addrsort | ACCEPT-WITH-LOSSES | 1549215 |
| w1-num-pcode-semantics | fleet | independent | golden_opbehavior 20227 rows | ACCEPT-WITH-LOSSES | a922c92 |
| w1-harness-unittests | fleet | independent | 23/23 exact-name TESTs | ACCEPT | f4df3da |

## W2 (2026-06-11)

| item | porter | verifier | gate | verdict | commit |
|---|---|---|---|---|---|
| w2-sleigh-translate | fleet | independent | LOSS-012 arms + tests | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-context | fleet | independent | cargo test kuna-sleigh | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-pattern | fleet | independent | cargo test kuna-sleigh | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-symbol | fleet | independent | cargo test kuna-sleigh | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-semantics | fleet | independent | cargo test kuna-sleigh | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-pcodeparse | fleet(opus) | independent(opus) | 22+ snippet accept tests | ACCEPT |  |
| w2-sleigh-loadimage | fleet | independent | corpus binaryimage loads | ACCEPT |  |
| w2-sleigh-core | fleet(opus) | independent(opus) | golden_lift 16/16 (1171 insns) | ACCEPT-WITH-LOSSES |  |
| w2-sleigh-emulate | fleet | independent | memstate/emulate tests | ACCEPT-WITH-LOSSES |  |
| w2-harness-floatemu | fleet | independent | 31/31 exact-name TESTs | ACCEPT-WITH-LOSSES |  |

## W3 (2026-06-12)

| item | porter | verifier | gate | verdict | commit |
|---|---|---|---|---|---|
| w3-ir-varnode | fleet(opus) | independent(opus) | comparator cross-check >100k pairs | ACCEPT-WITH-LOSSES |  |
| w3-ir-op | fleet(opus) | independent(opus) | intrusive-list model tests | ACCEPT-WITH-LOSSES |  |
| w3-ir-userop-inject | fleet(opus) | independent(opus) | spec payload decodes | ACCEPT-WITH-LOSSES |  |
| w3-ir-block | fleet(opus) | independent(opus) | dominator matrices | ACCEPT-WITH-LOSSES |  |
| w3-ir-funcdata | fleet(opus) | independent(opus) | splice/structureReset | ACCEPT-WITH-LOSSES |  |
| w3-ir-funcdata-op | fleet(opus) | independent(opus) | SeqNum allocation order | ACCEPT-WITH-LOSSES |  |
| w3-ir-funcdata-varnode | fleet(opus) | independent(opus) | bank-state outcomes | ACCEPT-WITH-LOSSES |  |
| w3-ir-flow | fleet(opus) | independent(opus) | FlowInfo vs lift fixtures | ACCEPT-WITH-LOSSES |  |
| w3-ir-jumptable | fleet(opus) | independent(opus) | PathMeld/guard graphs | ACCEPT-WITH-LOSSES |  |
| w3-kuna-flow-pack | fleet(opus) | independent(opus) | five hook logic tests | ACCEPT |  |

## W6 (2026-06-13)

| item | porter | verifier | gate | verdict |
|---|---|---|---|---|
| w6-s4-fspec-1 | fleet(opus) | independent | ParamEntry/ParamList | ACCEPT |
| w6-s4-fspec-2 | fleet(opus) | independent | ProtoModel/FuncProto, 51 tests | ACCEPT-WL |
| w6-s4-fspec-3 | fleet(opus) | independent | FuncCallSpecs, 14 tests | ACCEPT-WL |
| w6-s4-modelrules | fleet(opus) | independent | cspec modelrule decode | ACCEPT-WL |
| w6-s5-type-1 | fleet(opus) | independent | Datatype total orders | ACCEPT |
| w6-s5-type-2 | fleet(opus) | independent | struct/union/enum | ACCEPT-WL |
| w6-s5-type-3 | fleet(opus) | independent | TypeFactory findAdd | ACCEPT-WL |
| w6-s5-typeop | fleet(opus) | independent | per-opcode inst table | ACCEPT |
| w6-s5-unionresolve | fleet(opus) | independent | ScoreUnionFields | ACCEPT-WL |
| w6-s5-rangeutil | fleet(opus) | independent | CircleRange/ValueSet | ACCEPT-WL |
| w6-s5-double | fleet(opus) | independent | double-precision rules | ACCEPT-WL |
| w6-s5-bitfield | fleet(opus) | independent | bitfield place/recover | ACCEPT-WL |
| w6-s5-constseq-prefersplit | fleet(opus) | independent | StringSequence base | ACCEPT-WL |
| w6-s4-coreaction-protos | fleet(opus) | independent | S4 proto actions | ACCEPT-WL |
| w6-kuna-s4s5-pack | fleet(opus) | independent | returnpair/memset/compareform | ACCEPT |
| w6-harness-unittests | fleet(opus) | independent | testcirclerange 117 + names | ACCEPT-WL |

## W7 (2026-06-13)

| item | gate | verdict |
|---|---|---|
| w7-s6-variable-cover | HighVariable+Cover, Varnode::high wiring | ACCEPT-WL |
| w7-s6-merge | HighVariable formation passes | ACCEPT-WL |
| w7-s6-varmap | stack-layout core (ScopeLocal deferred) | ACCEPT-WL |
| w7-s6-dynamic-pack | DynamicHash + stackguard | ACCEPT-WL |
| w7-s7-blockaction | CF structuring / goto engine | ACCEPT-WL |
| w7-s7-kuna-regiongraph | angr digraph (deterministic order) | ACCEPT-WL |
| w7-s7-kuna-regionid | RegionIdentifier re-host | ACCEPT-WL |
| w7-s7-kuna-loweredswitch | GCC lowered-switch recovery | ACCEPT-WL |
| w7-harness-kunaregion | 3 region unit tests (exact names) | ACCEPT |
| w7-s37-coreaction-cleanup | S6/S7 cleanup+merge actions | ACCEPT-WL |
| w7-m1-closure | M1 name-parity 207/207; bodies re-diagnosed to W8/W9 | ACCEPT-WL |

## W8 (2026-06-13)

| item | gate | verdict |
|---|---|---|
| w8-s9-printlanguage-cast | PrintLanguage base + CastStrategy | ACCEPT-WL |
| w8-s9-printc | 3.9k-LOC C emitter, opXXX tokens | ACCEPT-WL |
| w8-s9-prettyprint | line-break engine vs real Ghidra | ACCEPT-WL |
| w8-sig-pack | signature/analyzesigs/paramid | ACCEPT-WL |
| w8-s9-stringmanage-pack | string escape + namestyle + arraynotation | ACCEPT |
| w8-s9-coreaction-render | S9 render actions | ACCEPT-WL |
| w8-fw-universalaction | B0 byte-equal (244->252 passes) | ACCEPT-WL |
| w8x-allowlist-closure | 10 scattered passes -> empty allowlist | ACCEPT-WL |
