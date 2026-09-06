//! S6 -- Variable & storage model: HighVariables, merge, stack layout.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_facing;
pub mod funcdata_merge;
pub mod funcdata_spacebase;
pub mod cover;
pub mod variable;
pub mod merge;
pub mod kuna_stackalias;
pub mod kuna_undefname;
pub mod varmap;
pub mod coreaction_cleanup;
pub mod kuna_callretfold;
pub mod dynamic;
pub mod kuna_dynamichashmax;
pub mod coreaction_stackptr;
pub mod kuna_paramcopyhoist;
pub mod kuna_cookiescramble;
