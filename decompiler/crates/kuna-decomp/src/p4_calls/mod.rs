//! S4 -- Call & prototype model.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod funcdata_callsite;
pub mod fspec;
pub mod modelrules;
pub mod coreaction_protos;
pub mod kuna_calleedeadarg;
pub mod kuna_calleepreserves; // (kuna) the decoded callee's writes narrow the cspec killedbycall set
pub mod kuna_callsitestackargs;
pub mod kuna_dfunaffected;
pub mod kuna_noreturnretuse;
pub mod kuna_returnpair;
pub mod kuna_retinputhalf;
pub mod kuna_returnuncomputed;
pub mod kuna_spillargtrial;
pub mod kuna_varargstackargs; // (kuna) the variadic call's stack tail is its own fillinMap section
pub mod kuna_calleearity; // (kuna) one callee, one argument list across its call sites
pub mod kuna_calleearityfwd; // (kuna) reconcile against a sibling call that finalizes later
pub mod kuna_calleearitylive; // (kuna) extend a partial argument list when the callee body agrees
pub mod kuna_inputparamgap; // (kuna) an unused-argument-register run must not veto a later live-in
pub mod kuna_rustabi; // (kuna) the rustc two-register return: keep the pair, connect it at the call
pub mod kuna_langabi; // (kuna) the ABI seam: per-language `extern` rendering
