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
