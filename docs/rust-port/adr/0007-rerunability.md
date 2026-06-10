# ADR 0007: Re-runability — typed P0Store, artifact versions, GhidraExact default

## Context

The C++ engine can restart a function mid-decompilation (`Funcdata` restart
semantics, `maxrestarts`), and kuna's stage model wants more: re-running a
stage after flipping a P0 assertion without redoing the whole function. The
C++ implementation entangles "configuration that survives a rewind" with
mutable engine state, and restart behavior is observable in the baseline
(restart counts, warning comments), so the port must reproduce it exactly
before improving on it.

## Decision

Three pieces:

1. **Typed `P0Store`** — options + overrides + kasserts in one immutable-ish
   typed struct that survives every rewind *by construction* (it is input to
   stage execution, never mutated by it).
2. **`FunctionContext`** with per-stage `ArtifactVersions` counters, bumped
   inside the `Funcdata` mutators of ADR 0001, plus stage-boundary `Funcdata`
   snapshots — `Clone` is correct because slotmap keys survive the copy.
3. **`RewindMode`**: `GhidraExact` is the default and is baseline-tested — the
   C++ restart semantics transcribed, `maxrestarts = 1`, no snapshots taken.
   `Scoped` (rewind to the cheapest stage boundary whose artifacts a flipped
   assertion invalidates, per the version counters) is opt-in with its own
   stage tests.

A `PassRegistration` API (`Rule` / `Action` / `Analysis` + anchor point)
generalizes the three existing C++ extension idioms; registering a new group
REQUIRES a `stages.toml` row (ADR 0006), so extension and documentation cannot
decouple.

## Consequences

- Baseline parity is protected: the default path takes no snapshots and pays
  no overhead relative to C++.
- Versioned artifacts give `Scoped` rewind a sound invalidation oracle instead
  of heuristics, but only because ADR 0001 funnels all mutation through
  counted `Funcdata` methods — bypassing them is a correctness bug.
- Snapshot memory cost exists only when `Scoped` is enabled; bounded by
  snapshotting at stage boundaries, not per-action.
- Third-party passes get one documented registration door, and the stages.toml
  coupling means the catalog (and `kuna.catalog` discovery) always reflects
  reality.
