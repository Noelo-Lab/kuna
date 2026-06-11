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
