//! P0 -- Knowledge & configuration plane: symbol DB, options, overrides, the stage registry.
//!
//! Stage-aligned module group; declared flatly at the crate root via re-export in
//! `lib.rs` so public paths (`kuna_decomp::<module>`) are unchanged.

pub mod options;
pub mod modes;
pub mod database;
pub mod overrides;
pub mod kuna_phases;
pub mod kuna_assert;
pub mod kuna_restartlog;
// (kuna) i386-PIE PLT-stub decode option gate (angr test_decompiling_nl_i386_pie).
pub mod kuna_i386_pie_plt;
// (kuna) x86-64 IFUNC (IRELATIVE) PLT-stub naming option gate (load-time, env-bridged).
pub mod kuna_ifuncfpret;
// (kuna) DWARF full-depth type-resolution option gate (load-time, env-bridged).
pub mod kuna_typedepth;
// (kuna) `<eval_current_prototype>`: the compiler spec's model for evaluating the
// current function's own unknown prototype (register-parameter recovery).
pub mod kuna_evalcurrentproto;
