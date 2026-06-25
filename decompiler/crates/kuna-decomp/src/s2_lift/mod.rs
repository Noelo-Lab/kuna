//! S2 -- Flow & op-graph recovery: lift, CFG, jump tables, p-code injection.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_resolveflow;
pub mod flow;
pub mod jumptable;
pub mod kuna_emulatefunction;
pub mod userop;
pub mod pcodeinject;
pub mod inject_sleigh;
pub mod kuna_v850indbranch;
pub mod kuna_switchmodbound;
pub mod kuna_switchguardbound;
pub mod kuna_sparcstructret;
pub mod kuna_stackprobeloop;
pub mod kuna_loweredswitch;
pub mod kuna_tailcalljump;
