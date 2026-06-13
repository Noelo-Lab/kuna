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
pub mod action;
pub mod options;
pub mod database;
pub mod architecture;
pub mod capability;
pub mod libdecomp;
pub mod overrides;
pub mod comment;
pub mod cpool;
pub mod graph;
pub mod callgraph;
pub mod sleigh_arch;
pub mod raw_arch;
pub mod xml_arch;
pub mod kuna_stages;
pub mod kuna_assert;
pub mod kuna_restartlog;
pub mod heritage;
pub mod cover;
pub mod variable;
pub mod merge;
pub mod subflow;
pub mod condexe;
pub mod blockaction;
pub mod expression;
pub mod transform;
pub mod coreaction_early;
pub mod ruleaction_1;
pub mod ruleaction_2;
pub mod ruleaction_3;
pub mod ruleaction_4;
pub mod ruleaction_5;
pub mod ruleaction_6;
pub mod ruleaction_7;
pub mod ruleaction_8;
pub mod kuna_addcarrychain;
pub mod kuna_booleanmask;
pub mod kuna_ovlesssimplify;
pub mod kuna_flagcompare;
pub mod kuna_stackalias;
pub mod kuna_arraystride;
pub mod kuna_condexeplace;
pub mod fspec;
pub mod varmap;
pub mod modelrules;
pub mod typeop;
pub mod unionresolve;
pub mod rangeutil;
pub mod double;
pub mod bitfield;
pub mod constseq;
pub mod prefersplit;
pub mod coreaction_protos;
pub mod kuna_returnpair;
pub mod kuna_memsetsequence;
pub mod kuna_compareform;
pub mod dynamic;
pub mod kuna_dynamichashmax;
pub mod kuna_stackguard;
