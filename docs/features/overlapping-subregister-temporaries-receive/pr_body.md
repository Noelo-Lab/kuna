## What was broken

The round-11 `Anti CrackMe V2.exe` witness emitted overlapping whole and subregister
locals under the same identifier, producing conflicting C declarations in both default
and Ghidra naming styles.

## Mechanism

The fix selects one deterministic whole-owner declaration for each overlapping local
group, reserves identifiers across locals, parameters, globals, and direct callees, and
allocates collision-free suffixes for distinct surviving objects. The unique-overlap-owner
invariant is unconditional, including when `dedupvardecls` is disabled.

## Evidence

Canonical acceptance `a-c6276af322b1` for probe `p-e2e8e9ceb38a` passes on the checked-in
`overlaplocals_i386` fixture and preserves both low/high byte references. Default, Ghidra,
and dedup-off CLI probes pass. The exact dataset function emits 103 declarations and no
duplicate declared identifier in all three modes; whole-binary JSON covers 107 functions
without errors. Clang accepts all three extracted declaration blocks and all three full
project exports of the authored fixture. The dataset function's full body retains unrelated
pre-existing pseudo-C gaps, which this PR does not claim to fix.

The durable need closes in round 12, attempt 1, on PR #590. The authoritative index preserves
all 197 active and 1 rejected rows and changes only this need plus the aggregate status totals:
136 closed, 60 open, and 1 blocked. A full regeneration was deliberately rejected because the
sparse branch source set and mutable root working set both introduce unrelated semantic drift;
the durable need decision log records that audit.

## Gates

- `make test`: 675/675, parity OK
- `make test-stages`: 797/797, parity OK
- `make test-cli`: 144/144
- `cargo test -p kuna-decomp --lib`: 2464/2464
- full `kuna-cli` package suite: green
- strict spec, catalog, and options freshness: green
- acceptance suite: 1/1 pass, no regressions

`mergecheck` reports five reviewed shape-C prose-key replacements: four obsolete stage
baseline labels and one obsolete stage README row. The replacement assertions deliberately
change the feature coverage documented in the need; no assertion row or shared counter is
lost.

🤖 Generated with [OpenAI Codex](https://openai.com/codex/).
