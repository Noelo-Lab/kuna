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
use kuna_decomp::kuna_formatstring::VarargAbi;

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
/// `:325`); `None` is faithful to a null Ghidra return type.  Parameters are
/// left ANONYMOUS — see the note in the body.
///
/// Returns `None` when there is nothing to type (no fixed params and no format
/// specs — Ghidra `createParameters` returns `null` for `numberOfParameters ==
/// 0`, `:296`), when the format asks for a `long double` (see the body), and
/// when `abi` says the target does not pass one of the format's arguments the
/// way it passes a named one of that type: the closed prototype would then move
/// the argument rather than type it (Apple AArch64 passes every vararg on the
/// stack, and a closed `%d` read `w1`). A `spec_to_datatype` failure propagates
/// as an `Err`.
pub fn build_override_pieces(
    name: &str,
    outtype: Option<Rc<Datatype>>,
    fixed_param_types: &[Rc<Datatype>],
    format_specs: &[Spec],
    types: &dyn TypeFactory,
    word_size: uint4,
    abi: VarargAbi,
) -> KunaResult<Option<PrototypePieces>> {
    let number_of_parameters = fixed_param_types.len() + format_specs.len();
    if number_of_parameters == 0 {
        // createParameters `:296`: numberOfParameters == 0 → null (invalid).
        return Ok(None);
    }
    // A `long double` conversion (`%Lf`) declines the whole site. kuna has no
    // extended-precision base type, so `Spec::LongDouble` lowers to an 8-byte
    // float — and an 8-byte float is a different ABI CLASS from a `long double`:
    // on x86-64 SysV the former is passed in an SSE register and the latter in
    // memory. Asserting the wrong class does not mistype one argument, it shifts
    // every argument after it, and the caller grows a phantom `double` parameter
    // for the vector register the override claimed. coreutils `numfmt`
    // `simple_strtod_human` prints `%Lf` and was recovered with seven parameters
    // instead of five. Saying nothing is the honest answer until the width is.
    if format_specs.contains(&Spec::LongDouble) {
        return Ok(None);
    }
    let mut intypes: Vec<Rc<Datatype>> = Vec::with_capacity(number_of_parameters);
    // i < initialFunctionParameters.size(): the callee's recovered fixed params.
    for ty in fixed_param_types {
        intypes.push(Rc::clone(ty));
    }
    // else: the format-string-derived var-arg types.
    for spec in format_specs {
        intypes.push(spec_to_datatype(*spec, types, word_size)?);
    }
    let ptr_size = types.get_size_of_pointer();
    if !intypes[fixed_param_types.len()..].iter().all(|ty| abi.admits(ty, ptr_size)) {
        return Ok(None);
    }
    // The parameters are deliberately ANONYMOUS, where Ghidra's
    // `ParameterDefinitionImpl("param" + i, …)` (`:303`/`:307`) names them
    // positionally. A call-site override's parameter names reach the CALLER's
    // locals, and a positional name is the same at every site: coreutils `fmt`
    // renders `error(0,*v2,"%s: %s",param3,param3)` for two different values,
    // because one took `param3` from the two-vararg site and the other from the
    // one-vararg site in the branch beside it — one name, two variables, and
    // only one of them declared. The built-in prototype tables leave `innames`
    // empty for the same reason (`protos::build_pieces`), and the engine then
    // keeps the caller's own naming.
    let innames: Vec<String> = vec![String::new(); number_of_parameters];
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
    fn a_long_double_conversion_declines_the_site() {
        // `%Lf` has no honest width here, and the wrong width is the wrong ABI
        // class, which shifts every argument after it.
        let types = kuna_decomp::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        types
            .set_core_type("char", 1, kuna_decomp::dtype::type_metatype::TYPE_INT, true)
            .expect("char core type");
        types.cache_core_types().expect("cache core types");
        let fixed: Vec<Rc<Datatype>> = Vec::new();
        assert!(build_override_pieces(
            "printf",
            None,
            &fixed,
            &[Spec::Int, Spec::LongDouble],
            &types,
            8,
            VarargAbi::Named
        )
        .unwrap()
        .is_none());
        // The same format without the `L` is typed.
        assert!(build_override_pieces(
            "printf",
            None,
            &fixed,
            &[Spec::Int, Spec::Double],
            &types,
            8,
            VarargAbi::Named
        )
        .unwrap()
        .is_some());
    }

    #[test]
    fn a_vararg_the_target_passes_unlike_a_named_argument_declines_the_site() {
        let types = kuna_decomp::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(4), 4, 4);
        types
            .set_core_type("char", 1, kuna_decomp::dtype::type_metatype::TYPE_INT, true)
            .expect("char core type");
        types.cache_core_types().expect("cache core types");
        let fixed: Vec<Rc<Datatype>> = Vec::new();
        let build = |specs: &[Spec], abi| {
            build_override_pieces("printf", None, &fixed, specs, &types, 1, abi).unwrap()
        };
        // ARM32 hard-float / RISC-V: a `%f` travels in core registers, a named
        // double in `d0`/`fa0`.
        assert!(build(&[Spec::Int, Spec::CharPtr], VarargAbi::Integer).is_some());
        assert!(build(&[Spec::Int, Spec::Double], VarargAbi::Integer).is_none());
        // A double-width integer is not admitted where the pointer is narrower.
        assert!(build(&[Spec::LongLong], VarargAbi::Integer).is_none());
        // Apple AArch64: nothing is passed like a named argument.
        assert!(build(&[Spec::Int], VarargAbi::Nothing).is_none());
        assert!(build(&[Spec::Int, Spec::Double], VarargAbi::Named).is_some());
    }

    #[test]
    fn classify_non_variadic_none() {
        assert_eq!(classify_variadic_call("puts"), None);
        assert_eq!(classify_variadic_call("memcpy"), None);
        assert_eq!(classify_variadic_call("main"), None);
        assert_eq!(classify_variadic_call(""), None);
    }
}
