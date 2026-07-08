# docs/history — archived documents

Frozen documents retained for provenance. Each was superseded by the live docs
(`docs/spec/`, `docs/stages.md`, `docs/options.md`) or describes a completed campaign.
Terminology inside is pre-rename (stage = phase, S-codes = P-codes).

- [analysis-port-buildplan.md](analysis-port-buildplan.md) — Build-order plan for the analyzer-tier port (complete).
- [analysis-port-frontier.md](analysis-port-frontier.md) — Feasibility frontier for the analyzer-tier port (the feasible set shipped).
- [analysis-port-log.md](analysis-port-log.md) — Working log of the analyzer-tier port campaign (complete); the live description is `docs/spec/01-program-prep.md`.
- [analysis-port-plan.md](analysis-port-plan.md) — Planning doc for the analyzer-tier port (complete).
- [fid-design.md](fid-design.md) — Design doc for FID function identification (landed).
- [gh558-prototype.md](gh558-prototype.md) — Prototype notes for the GH-558 comparison-canonicalization work (landed as the compareform option).
- [issue-coverage.md](issue-coverage.md) — Session-state ledger for a finished improvement campaign.
- [listing-tier-design.md](listing-tier-design.md) — Design doc for the Listing tier (landed); live description in `docs/spec/01-program-prep.md`.
- [metadata-analyzers-design.md](metadata-analyzers-design.md) — Design doc for the metadata analyzers (landed).
- [multiformat-loader-design.md](multiformat-loader-design.md) — Design doc for the multi-format loader (landed).
- [region-structurer-roadmap.md](region-structurer-roadmap.md) — Roadmap for the angr Phoenix/SAILR region structurer -- landed and default-on (DIV-12); live description in `docs/spec/08-structuring.md`.
- [stage-critique.md](stage-critique.md) — Empirical critique of the stage model (ablations, navigation scoring); retained as evidence.
- [stage-implementation.md](stage-implementation.md) — Per-session changelog of aligning the code to the phase model; superseded by the phase folders themselves and `docs/spec/`.
- [stage-mapping.md](stage-mapping.md) — Maps the 115 upstream C++ translation units (a tree removed after the Rust port) to phases, with the legacy 19-stage numbering. The Rust tree is phase-foldered and self-describing; see `docs/spec/`.
- [stage-model.md](stage-model.md) — Theory/design-rationale document (the 2026-06 Ghidra/angr/Reko side-by-side study). Its upstream C++/Python anchors describe trees not in this repo. The live, normative description of what kuna actually does is the chaptered spec under `docs/spec/` (until that lands: `docs/stages.md`).
