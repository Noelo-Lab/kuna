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
pub mod kuna_switchsharedcase;
pub mod kuna_sparcstructret;
pub mod kuna_stackprobeloop;
pub mod kuna_loweredswitch;
pub mod kuna_noreturn_externmatch; // (kuna) angr incorrect-duplication-chcon: name-matched extern no-return
pub mod kuna_noreturnextern; // (kuna) angr tail-tail-bytes-ret-dup: undefined-extern name-based no-return
pub mod kuna_tailcalljump;
pub mod kuna_funcboundflow; // (kuna) bound fall-through at a known function entry (no cross-function merge)
pub mod kuna_tailcallframe; // (kuna) recover a frame-teardown tail jump whose callee was never discovered
pub mod kuna_overlapbranch; // (kuna) a conditional branch target inside its own fall-through instruction (anti-disassembly overlap)
pub mod kuna_msvcftol; // (kuna) MSVC __ftol family call-fixup: recover the x87 (ST0) argument
pub mod kuna_cortexmpriv; // (kuna) fold the Cortex-M isCurrentModePrivileged() guard around MRS/MSR
pub mod kuna_cleanupcode; // (kuna) oxidizer CleanupCodeRemover: delete rust drop/dealloc call sites
pub mod kuna_linuxsyscall; // (kuna) name the 32-bit Linux int 0x80 sites instead of calling through swi()
