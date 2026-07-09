# The kuna decompiler spec

This is the **live, normative description of the algorithms kuna runs** — what each
phase computes, how the simple case is decided, what the pathological-case machinery
does, and how each phase feeds the next. It is chaptered by pipeline phase; every
chapter is anchored to the code that implements it.

> This is the decompiler *algorithm* spec. The SLEIGH processor specifications live
> at the repo root under `specs/` — different thing entirely.

| Chapter | Covers |
|---|---|
| [00 — Overview & machinery](00-overview.md) | the two tiers, front-ends, IR substrate, knowledge plane, the scheduler |
| [01 — Program preparation](01-program-prep.md) | `kuna-analysis`: loading, markup, metadata analyzers, the Listing, no-return |
| [02 — Lift & flow recovery](02-lift-and-flow.md) | p-code lift, CFG, injection, jump tables & switch recovery |
| [03 — SSA & simplification](03-ssa-and-simplification.md) | heritage, the rule pools, sub-variable flow |
| [04 — Calls & prototypes](04-calls-and-prototypes.md) | prototype models, parameter recovery |
| [05 — Types](05-types.md) | the type lattice, inference, unions, double precision |
| [06 — Variables & merge](06-variables-and-merge.md) | HighVariables, cover/merge, the stack frame |
| [07 — Region identification](07-regions.md) | (angr) the region graph & identifier |
| [08 — Structuring](08-structuring.md) | collapse structuring, (angr) region structurer, goto quality |
| [09 — Emission](09-emission.md) | casts, PrintC, naming, strings |

## Rules (read before editing)

1. **The spec is live.** Any change that affects decompiler behavior — a new pass, a
   changed heuristic or threshold, a changed default, a new option, a new analysis —
   updates the affected chapter **in the same commit/PR**. Find the chapter via its
   `Anchors:` front-matter (chapters map 1:1 onto the phase folders). A pure
   move/rename updates only anchor paths. `make check-spec` enforces the basics.
2. **Live code only.** Every claim is anchored to a kuna path plus a symbol name in
   parentheses — `decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs
   (JumpBasic::recoverModel)`. **No line numbers** (they rot). Upstream citations
   (`coreaction.cc:5722`, angr `.py` paths) appear only as parenthetical
   provenance/history, never as the basis of a claim.
3. **Provenance tags.** Untagged prose describes the Ghidra-derived port (the default
   lineage). Everything else is tagged inline at the section heading and per pass:
   `(angr)`, `(ida)`, `(kuna)` — agreeing with `source_decompiler` in `phases.toml`
   and `docs/features/*/record.json`.
4. **Depth contract.** For each algorithm: (a) the decision rule for the simple case,
   in one or two sentences of natural language; (b) the pathological-case machinery
   (restarts, fallbacks, thresholds, caps) with the *reason* each exists; (c) the
   failure mode when it loses. No code recaps, no API documentation.
5. **Numbers are read from the code at writing time** (rule counts, thresholds,
   budgets). Anything unverifiable is cut or marked `UNVERIFIED:` — and an
   `UNVERIFIED:` marker must not survive to a commit.
6. **Link, don't duplicate.** Option metadata (defaults, tiers, symptoms, flip
   guidance) lives in the generated catalog (`docs/options.md`); the spec names an
   option exactly where its algorithm is described and links there.

## Where things that are not algorithms live

- Option catalog (the on/off control surface): `docs/options.md` (generated).
- Intentional default divergences from upstream + measurements: `docs/divergences.md`.
- The phase model at a glance: `docs/phases.md`.
- Port history, ADRs, losses: `docs/RUST_PORT.md`, `docs/rust-port/`.
- Archived design docs and the original phase-model derivation study: `docs/history/`.
