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
//! Lints are inherited from the workspace (`[lints] workspace = true`).
pub mod varnode;
pub mod op;
pub mod block;
pub mod funcdata;
pub mod funcdata_block;
pub mod funcdata_op;
pub mod funcdata_varnode;
pub mod flow;
pub mod jumptable;
pub mod userop;
pub mod pcodeinject;
pub mod inject_sleigh;
pub mod dtype;
pub mod seams;
pub mod kuna_v850indbranch;
pub mod kuna_switchmodbound;
pub mod kuna_thumbfuncptr;
pub mod kuna_inferfuncentry;
pub mod kuna_sparcstructret;
