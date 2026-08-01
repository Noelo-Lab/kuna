//! S3 -- Definition web: SSA/heritage + the simplification rule pools.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod heritage;
pub mod subflow;
pub mod condexe;
pub mod condconst;
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
pub mod kuna_arraystride;
pub mod kuna_condexeplace;
pub mod kuna_compareform;
pub mod kuna_inputtile;
