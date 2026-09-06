//! Format-string varargs typing — the **application half (B)** of the kuna
//! analog of Ghidra's `FormatStringAnalyzer` ("Variadic Function Signature
//! Override").  This is the call-site-classification + override-building logic
//! that the *parser* ([`super`], half A) feeds; the actual
//! decompile→inspect→override→re-decompile *loop* lives in the console driver
//! (`kuna-console`'s `IfcDecompile`), which is the kuna analog of Ghidra's
//! `ParallelDecompiler` + `PcodeFunctionParser` + `HighFunctionDBUtil.writeOverride`.
//!
//! Ghidra origin:
//! `Ghidra/Features/DecompilerDependent/src/main/java/ghidra/app/plugin/core/string/variadic/`
//! — chiefly `FormatStringAnalyzer.java` (the driver: `VARIADIC_SUBSTRINGS`
//! call-name test `:42`/`:128`, `INPUT_FUNCTION_SUBSTRING` output/input choice
//! `:59`/`:273`, `createParameters` `:292`, `initSignature` `:313`) and
//! `PcodeFunctionParser.java` (the format-constant read at the call arg slot
//! `:99`).
//!
//! # What this module does (the pure, unit-testable application logic)
//!
//! - [`classify_variadic_call`]: given a recovered call-site callee *name*,
//!   decide whether it is a `printf`/`scanf`-family variadic format function and,
//!   if so, whether it takes *output* (`printf`-family) or *input*
//!   (`scanf`-family) argument types.  Faithful to Ghidra
//!   `FormatStringAnalyzer.run` (`:127-128`, the `name.contains(substring)` test
//!   over `VARIADIC_SUBSTRINGS = {"printf","scanf"}`) and `parseParameters`
//!   (`:273`, `isOutputType = !callFunctionName.contains("scanf")`).
//! - [`build_override_pieces`]: given the callee's *fixed* parameter types (the
//!   already-recovered prototype, the analog of Ghidra's
//!   `namesToParameters.get(callFunctionName)`), the callee return type, and the
//!   format-derived [`Spec`](super::Spec) list, build the concrete
//!   [`PrototypePieces`] for the per-call-site override — fixed types ++ the
//!   parsed format types, with `first_var_arg_slot = -1` (the override is the now
//!   *fixed* signature, no longer varargs — matching Ghidra's
//!   `FunctionDefinitionDataType` with no var-args flag set, installed by
//!   `HighFunctionDBUtil.writeOverride`).  The analog of
//!   `createParameters`/`initSignature` (`:292`/`:313`).
//!
//! The Funcdata pcode-walk (find the `CALL` ops, read the format constant from
//! the call arg at slot `paramCount`, resolve the constant through its defining
//! op) is genuinely `Funcdata`-dependent and lives in the console driver, not
//! here, so this module stays a pure, portable, unit-tested library.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::types::uint4;
use kuna_decomp::dtype::{Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use super::{spec_to_datatype, Spec};

/// Ghidra `FormatStringAnalyzer.VARIADIC_SUBSTRINGS` (`:42`): a call-site callee
/// whose name *contains* one of these is a candidate variadic format function.
pub const VARIADIC_SUBSTRINGS: &[&str] = &["printf", "scanf"];

/// Ghidra `FormatStringAnalyzer.INPUT_FUNCTION_SUBSTRING` (`:59`): a callee whose
/// name contains this takes *input* argument types (`scanf`-family: it writes
/// through pointer arguments); otherwise it takes *output* types
/// (`printf`-family).  `parseParameters` `:273`:
/// `isOutputType = !callFunctionName.contains("scanf")`.
pub const INPUT_FUNCTION_SUBSTRING: &str = "scanf";

/// Classify a recovered call-site callee by name (Ghidra
/// `FormatStringAnalyzer.run`, `:127-128` + `parseParameters` `:273`).
///
/// Returns:
/// - `Some(true)`  — a `printf`-family *output* format function (use
///   [`super::parse_output_types`] on the format string);
/// - `Some(false)` — a `scanf`-family *input* format function (use
///   [`super::parse_input_types`]);
/// - `None`        — not a recognized variadic format function.
///
/// The test is the same substring containment Ghidra uses: e.g. `printf`,
/// `fprintf`, `sprintf`, `snprintf`, `vprintf` all match `"printf"`; `scanf`,
/// `sscanf`, `fscanf` all match `"scanf"`.  A name containing `"scanf"` is
/// classified input regardless of whether it also contains `"printf"` (Ghidra's
/// `isOutputType` is `!contains("scanf")`, so the `scanf` test wins).
pub fn classify_variadic_call(name: &str) -> Option<bool> {
    let is_variadic = VARIADIC_SUBSTRINGS.iter().any(|sub| name.contains(sub));
    if !is_variadic {
        return None;
    }
    // isOutputType = !callFunctionName.contains(INPUT_FUNCTION_SUBSTRING) (`:273`).
    Some(!name.contains(INPUT_FUNCTION_SUBSTRING))
}

/// Build the concrete per-call-site [`PrototypePieces`] override (Ghidra
/// `createParameters` `:292` + `initSignature` `:313`).
///
/// The new signature is the callee's already-recovered *fixed* parameters
/// (`fixed_param_types`, the analog of `namesToParameters.get(callFunctionName)`
/// = `function.getParameters()` minus the trailing `"..."`) followed by the
/// `format_specs`-derived argument types ([`spec_to_datatype`] per spec).  The
/// result is a *fixed* prototype: `first_var_arg_slot = -1` (no longer varargs),
/// exactly as Ghidra installs a plain `FunctionDefinitionDataType` (no var-args
/// flag) via `HighFunctionDBUtil.writeOverride`.
///
/// `name` is the callee name (carried only for the pieces' identifier, the analog
/// of `FunctionDefinitionDataType(callFunctionName)`).  `outtype` is the callee's
/// recovered return type (`signature.setReturnType(namesToReturn.get(...))`,
/// `:325`); `None` is faithful to a null Ghidra return type.  Each parameter is
/// given a positional `param{i}` name (Ghidra `ParameterDefinitionImpl("param" +
/// i, …)`, `:303`/`:307`).
///
/// Returns `None` when there is nothing to type (no fixed params and no format
/// specs — Ghidra `createParameters` returns `null` for `numberOfParameters ==
/// 0`, `:296`).  A `spec_to_datatype` failure propagates as an `Err`.
pub fn build_override_pieces(
    name: &str,
    outtype: Option<Rc<Datatype>>,
    fixed_param_types: &[Rc<Datatype>],
    format_specs: &[Spec],
    types: &dyn TypeFactory,
    word_size: uint4,
) -> KunaResult<Option<PrototypePieces>> {
    let number_of_parameters = fixed_param_types.len() + format_specs.len();
    if number_of_parameters == 0 {
        // createParameters `:296`: numberOfParameters == 0 → null (invalid).
        return Ok(None);
    }
    let mut intypes: Vec<Rc<Datatype>> = Vec::with_capacity(number_of_parameters);
    let mut innames: Vec<String> = Vec::with_capacity(number_of_parameters);
    // i < initialFunctionParameters.size(): the callee's recovered fixed params.
    for (i, ty) in fixed_param_types.iter().enumerate() {
        intypes.push(Rc::clone(ty));
        innames.push(format!("param{i}"));
    }
    // else: the format-string-derived var-arg types.
    for (j, spec) in format_specs.iter().enumerate() {
        let ty = spec_to_datatype(*spec, types, word_size)?;
        intypes.push(ty);
        innames.push(format!("param{}", fixed_param_types.len() + j));
    }
    Ok(Some(PrototypePieces {
        name: name.to_string(),
        outtype,
        intypes,
        innames,
        // The override is now a *fixed* signature — no longer varargs (Ghidra
        // installs a FunctionDefinitionDataType with no var-args flag set).
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- classify_variadic_call (Ghidra VARIADIC_SUBSTRINGS / scanf test) -----

    #[test]
    fn classify_printf_family_output() {
        // printf-family → Some(true) (output types).
        assert_eq!(classify_variadic_call("printf"), Some(true));
        assert_eq!(classify_variadic_call("fprintf"), Some(true));
        assert_eq!(classify_variadic_call("sprintf"), Some(true));
        assert_eq!(classify_variadic_call("snprintf"), Some(true));
        assert_eq!(classify_variadic_call("vprintf"), Some(true));
        assert_eq!(classify_variadic_call("__printf_chk"), Some(true));
    }

    #[test]
    fn classify_scanf_family_input() {
        // scanf-family → Some(false) (input types).
        assert_eq!(classify_variadic_call("scanf"), Some(false));
        assert_eq!(classify_variadic_call("sscanf"), Some(false));
        assert_eq!(classify_variadic_call("fscanf"), Some(false));
        // A name containing "scanf" is input even if it also contains "printf"
        // (Ghidra isOutputType = !contains("scanf"); the scanf test wins).
        assert_eq!(classify_variadic_call("printf_scanf_weird"), Some(false));
    }

    #[test]
    fn classify_non_variadic_none() {
        assert_eq!(classify_variadic_call("puts"), None);
        assert_eq!(classify_variadic_call("memcpy"), None);
        assert_eq!(classify_variadic_call("main"), None);
        assert_eq!(classify_variadic_call(""), None);
    }
}
