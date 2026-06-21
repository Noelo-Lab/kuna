//! kuna-decomp: the decompiler core of the kuna Rust port.
//!
//! Ports all of DECCORE plus the kuna stage-model additions
//! (`decompiler/cpp/`):
//!
//! - `varnode`, `op`, `block`, `funcdata*` (the SSA function IR -- slotmap
//!   arenas with id newtypes per ADR 0001)
//! - `type*` (`type.cc`, `typeop.cc`, `cast.cc`, `unionresolve.cc`, ...)
//! - `fspec` (prototype models), `database` (symbol scopes), `heritage`
//!   (SSA construction), `merge` (HighVariable merging), `dynamic`
//!   (DynamicHash), `jumptable`, `flow` (instruction following)
//! - `ruleaction` (split by rule family), `coreaction` (split by stage),
//!   `blockaction` (structuring), `subflow`, `transform`, `condexe`,
//!   `paramid`, `varmap`
//! - `print*` (`printlanguage.cc`, `printc.cc`, `printjava.cc`, `prettyprint.cc`)
//! - the kuna stage registry (`kuna_stages.cc` and friends -- regenerated from
//!   `stages.toml` per ADR 0006)
//!
//! Action/Rule dispatch and scheduling follow ADR 0005; re-runability and the
//! P0 store follow ADR 0007.
//!
//! # Source layout (stage-aligned)
//!
//! The module **files** are grouped on disk into stage-named folders that mirror
//! the normative stage model (`docs/stages.md`, `docs/stage-mapping.md`); the
//! module **names** stay flat (`kuna_decomp::flow`, `crate::dtype`) via the
//! `pub use <folder>::*;` re-exports below, so the public and internal APIs are
//! unchanged. An outsider can read the tree to see what each file does in the
//! pipeline (each folder's `mod.rs` carries a one-line stage gloss):
//!
//! - `substrate/`     -- shared IR & containers used by every stage (varnode/op/block/funcdata*, dtype, ...)
//! - `p0_knowledge/`  -- P0: knowledge & configuration plane (symbol DB, options, overrides, the stage registry)
//! - `s1_partition/`  -- S1: image & code partition (architecture/loader binding)
//! - `s2_lift/`       -- S2: flow & op-graph recovery (lift, CFG, jump tables, injection)
//! - `s3_dataflow/`   -- S3: definition web (SSA/heritage + the simplification rule pools)
//! - `s4_calls/`      -- S4: call & prototype model
//! - `s5_types/`      -- S5: value & type facts (type system + inference)
//! - `s6_variables/`  -- S6: variable & storage model (HighVariables, merge, stack layout)
//! - `s7_regions/`    -- S7: region hierarchy (the angr RegionIdentifier port)
//! - `s8_structure/`  -- S8: structured AST & goto quality (the structuring engine)
//! - `s9_emit/`       -- S9: surface rendering & refinement (PrintC, casts, strings, naming)
//! - `infra/`         -- orchestration & framework (the schedule, the Action/Rule engine, ...)
//!
//! The stage folders are private module groups; the `pub use ::*` re-exports keep
//! every module reachable by its flat name (`kuna_decomp::flow`). Module names are
//! unique across folders, so the glob re-exports never collide.
//!
//! Lints are inherited from the workspace (`[lints] workspace = true`).

mod substrate;
mod p0_knowledge;
mod s1_partition;
mod s2_lift;
mod s3_dataflow;
mod s4_calls;
mod s5_types;
mod s6_variables;
mod s7_regions;
mod s8_structure;
mod s9_emit;
mod infra;

pub use substrate::*;
pub use p0_knowledge::*;
pub use s1_partition::*;
pub use s2_lift::*;
pub use s3_dataflow::*;
pub use s4_calls::*;
pub use s5_types::*;
pub use s6_variables::*;
pub use s7_regions::*;
pub use s8_structure::*;
pub use s9_emit::*;
pub use infra::*;
