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
// (kuna) Relocatable-object analysis-fact rebase option gate (load-time, env-bridged).
pub mod kuna_relocrebase;
// (kuna) Linked-image dynamic-relocation application option gate (load-time, env-bridged).
pub mod kuna_dynrelocs;
// (kuna) PE chained-`UNWIND_INFO` `.pdata` entry suppression option gate
// (load-time, env-bridged).
pub mod kuna_pdatachained;
// (kuna) DWARF aggregate-layout import option gate (load-time, env-bridged).
pub mod kuna_dwarfstructs;
// (kuna) DWARF variant-part (discriminated-union) import option gate
// (load-time, env-bridged) plus the recovered-layout side table.
pub mod kuna_dwarfvariants;
// (kuna) DWARF full-depth type-resolution option gate (load-time, env-bridged).
pub mod kuna_typedepth;
// (kuna) Degenerate-symbol-name repair option gate (load-time, env-bridged).
pub mod kuna_symbolnamerepair;
// (kuna) Symbol-name character sanitizer + option gate (load-time, env-bridged).
pub mod kuna_symbolnamechars;
// (kuna) Symbol-name scope-path resource bound (load-time, env-bridged): the
// ceiling on how many Scopes one qualified symbol name may nest.
pub mod kuna_symbolnamebound;
// (kuna) MSVC `__real@` FP-constant COMDAT recovery option gate (load-time, env-bridged).
pub mod kuna_msvcfpconst;
// (kuna) `<eval_current_prototype>`: the compiler spec's model for evaluating the
// current function's own unknown prototype (register-parameter recovery).
pub mod kuna_evalcurrentproto;
