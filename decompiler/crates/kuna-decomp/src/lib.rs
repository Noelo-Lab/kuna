//! kuna-decomp: the decompiler core of the kuna Rust port.
//!
//! Ports all of DECCORE plus the kuna phase-model additions
//! (`decompiler/cpp/`):
//!
//! - `varnode`, `op`, `block`, `funcdata*` (the SSA function IR -- slotmap
//!   arenas with id newtypes per ADR 0001)
//! - `type*` (`type.cc`, `typeop.cc`, `cast.cc`, `unionresolve.cc`, ...)
//! - `fspec` (prototype models), `database` (symbol scopes), `heritage`
//!   (SSA construction), `merge` (HighVariable merging), `dynamic`
//!   (DynamicHash), `jumptable`, `flow` (instruction following)
//! - `ruleaction` (split by rule family), `coreaction` (split by phase),
//!   `blockaction` (structuring), `subflow`, `transform`, `condexe`,
//!   `paramid`, `varmap`
//! - `print*` (`printlanguage.cc`, `printc.cc`, `printjava.cc`, `prettyprint.cc`)
//! - the kuna phase registry (`kuna_stages.cc` and friends -- regenerated from
//!   `phases.toml` per ADR 0006)
//!
//! Action/Rule dispatch and scheduling follow ADR 0005; re-runability and the
//! P0 store follow ADR 0007.
//!
//! # Source layout (phase-aligned)
//!
//! The module **files** are grouped on disk into phase-named folders that mirror
//! the normative phase model (`docs/phases.md`; normative walk:
//! `docs/spec/00-overview.md`); the
//! module **names** stay flat (`kuna_decomp::flow`, `crate::dtype`) via the
//! `pub use <folder>::*;` re-exports below, so the public and internal APIs are
//! unchanged. An outsider can read the tree to see what each file does in the
//! pipeline (each folder's `mod.rs` carries a one-line phase gloss):
//!
//! - `substrate/`     -- shared IR & containers used by every phase (varnode/op/block/funcdata*, dtype, ...)
//! - `p0_knowledge/`  -- P0: knowledge & configuration plane (symbol DB, options, overrides, the phase registry)
//! - `p1_partition/`  -- P1: image & code partition (architecture/loader binding)
//! - `p2_lift/`       -- P2: flow & op-graph recovery (lift, CFG, jump tables, injection)
//! - `p3_dataflow/`   -- P3: definition web (SSA/heritage + the simplification rule pools)
//! - `p4_calls/`      -- P4: call & prototype model
//! - `p5_types/`      -- P5: value & type facts (type system + inference)
//! - `p6_variables/`  -- P6: variable & storage model (HighVariables, merge, stack layout)
//! - `p7_regions/`    -- P7: region hierarchy (the angr RegionIdentifier port)
//! - `p8_structure/`  -- P8: structured AST & goto quality (the structuring engine)
//! - `p9_emit/`       -- P9: surface rendering & refinement (PrintC, casts, strings, naming)
//! - `infra/`         -- orchestration & framework (the schedule, the Action/Rule engine, ...)
//!
//! The phase folders are private module groups; the `pub use ::*` re-exports keep
//! every module reachable by its flat name (`kuna_decomp::flow`). Module names are
//! unique across folders, so the glob re-exports never collide. Two conventions:
//!
//! - **The folders are a taxonomy, not the schedule.** Execution order is the
//!   declarative `SchedNode` tree built by `infra/universalaction.rs`
//!   (`universal_sched`); phases P3-P6 co-iterate to a mutual fixpoint inside
//!   `mainloop` (Band B), and restarts re-run the whole tree. The normative
//!   pipeline walk is `docs/spec/00-overview.md`.
//! - `foo.rs` + a sibling `foo/tests.rs` (wired by a trailing `mod tests;`) is
//!   the crate-wide unit-test convention -- a same-named directory next to a
//!   module file holds its tests, not a second module.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

mod substrate;
mod p0_knowledge;
mod p1_partition;
mod p2_lift;
mod p3_dataflow;
mod p4_calls;
mod p5_types;
mod p6_variables;
mod p7_regions;
mod p8_structure;
mod p9_emit;
mod infra;

pub use substrate::*;
pub use p0_knowledge::*;
pub use p1_partition::*;
pub use p2_lift::*;
pub use p3_dataflow::*;
pub use p4_calls::*;
pub use p5_types::*;
pub use p6_variables::*;
pub use p7_regions::*;
pub use p8_structure::*;
pub use p9_emit::*;
pub use infra::*;
