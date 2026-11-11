//! kuna fix `kt_contract_import`: a function's `contract { .. }` is written
//! with the import of `contract` and the opt-in to `ExperimentalContracts`.
//!
//! `KContract` writes the contract the metadata records as the call that
//! declares it, `contract { callsInPlace(block, InvocationKind.EXACTLY_ONCE) }`,
//! but only `InvocationKind` reaches the imports: `contract` is a top-level
//! function of `kotlin.contracts`, which Kotlin does not import by default,
//! and the contract DSL needs the opt-in to `ExperimentalContracts`, which
//! its source gives (`@file:OptIn(ExperimentalContracts::class)`) and the
//! class file cannot keep (`@OptIn` has source retention). kotlinc rejects
//! both ("unresolved reference 'contract'", "this declaration needs
//! opt-in"). With the fix `contract` is imported (or written
//! `kotlin.contracts.contract` where the name is taken) and the function is
//! annotated `@OptIn(ExperimentalContracts::class)`.

use crate::decompiler_context::DecompilerContext;
use crate::kotlin::kuna_fix_kt_facade_call;
use crate::text_buffer::TextBuffer;

/// How `contract` is written: bare when imported, else qualified.
pub fn contract_call() -> String {
    let prefix = kuna_fix_kt_facade_call::qualifier("kotlin.contracts", "contract")
        .unwrap_or_default();
    format!("{prefix}contract")
}

/// Appends the line `@OptIn(ExperimentalContracts::class)`.
pub fn append_opt_in(buf: &mut TextBuffer, indent: i32) {
    let name = DecompilerContext::get_import_collector()
        .get_short_name("kotlin.contracts.ExperimentalContracts")
        .unwrap_or_else(|| "kotlin.contracts.ExperimentalContracts".to_string());
    buf.append_indent(indent)
        .append(&format!("@OptIn({name}::class)"))
        .append_line_separator();
}
