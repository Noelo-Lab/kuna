//! Format-string varargs typing — the **pure parser half** of the kuna analog
//! of Ghidra's `FormatStringAnalyzer` ("Variadic Function Signature Override").
//!
//! Ghidra origin:
//! `Ghidra/Features/DecompilerDependent/src/main/java/ghidra/app/plugin/core/string/variadic/`
//! — chiefly `FormatStringParser.java` (the pure spec-parsing logic ported here)
//! and `FormatArgument.java` (the `(length-modifier, conversion-specifier)` pair,
//! mirrored by [`FmtArg`]). The driver `FormatStringAnalyzer.java` is **not**
//! ported here (it is the deferred half — see *A vs B* below).
//!
//! # What this module does (the faithful `printf`/`scanf` spec grammar)
//!
//! Given a C `printf`/`scanf` format string, it parses the ordered list of
//! conversion specifiers and maps each to the implied C argument type. It is a
//! line-faithful transcription of `FormatStringParser`'s state machine:
//! `parseFormatString` (split `%…spec` runs) → `convertToFormatArguments`
//! (the length-modifier + conversion-specifier state machine) →
//! `convertToOutputDataTypes` / `convertToInputDataTypes` →
//! `convertPairToDataType` / `conversionSpecifierToDataType` (the spec→type
//! table). It handles:
//!
//! - integer specs `d`/`i` (signed int), `u`/`o`/`x`/`X` (unsigned int);
//! - length modifiers `h`/`hh`/`l`/`ll`/`q`/`j`/`z`/`t`/`L` (short/char/long/
//!   long-long/intmax/size_t/ptrdiff_t/long-double widths);
//! - floats `f`/`F`/`e`/`E`/`g`/`G`/`a`/`A` → `double` (`L` → `long double`);
//! - `c` → `unsigned char` (`lc` → wide-char pointer); `s` → `char *`
//!   (`ls` → wide-char pointer); `C`/`S` → wide-char pointer;
//! - `p` → `void *`; `n` → `int *` (with the length-modifier-widened pointee);
//! - `%%` → literal, consumes no argument (`emitPercent`);
//! - field-width / precision `*` → consumes an `int` argument;
//! - positional `%N$…` arguments (`analyzeFormatStringWithParameters`).
//!
//! On any of Ghidra's "undefined behavior" branches (malformed / truncated
//! specs) the parser returns an **empty** list (Ghidra returns `null` → no
//! override installed), so it never panics on garbage input.
//!
//! `printf`-vs-`scanf` is the `isOutputType` flag: [`parse_output_types`]
//! (`printf`-family) returns the bare value types; [`parse_input_types`]
//! (`scanf`-family) wraps each non-pointer value type in an extra pointer
//! (`convertToInputDataTypes`, `FormatStringParser.java:597`), since `scanf`
//! takes addresses to write into.
//!
//! # A vs B: this is the **A (pure parser) half only**
//!
//! Ghidra's `FormatStringAnalyzer` is **DecompilerDependent**: to type a
//! `printf`/`scanf` call's varargs it needs the per-call-site *format-string
//! constant* and *which argument is the format* — both of which only exist
//! **after** the caller is lifted to p-code (the constant char* pointer in the
//! `CALL` op's argument varnode). That is genuinely not available at the
//! load-time `AnalysisCtx`/`AnalysisOutput` seam the other S1 passes use.
//!
//! The feature therefore splits in two:
//!
//! - **A (this module):** a self-contained, well-tested format-string → argument
//!   type parser. It produces no [`crate::pass::AnalysisOutput`] facts and is
//!   **not** registered in `passes.rs` — it is a *library* the future B will
//!   call. It touches only `kuna-analysis` and is structurally incapable of
//!   moving any parity gate.
//! - **B (deferred, a wave-3 engine change — NOT in this module):** the
//!   decompile→inspect→override→re-decompile loop (the analog of Ghidra's
//!   `ParallelDecompiler` + `PcodeFunctionParser` + `HighFunctionDBUtil.writeOverride`).
//!   After the first decompile of a caller it walks the `CALL` ops, reads the
//!   format-string constant from the call arg at the callee's fixed-param slot
//!   (`PcodeFunctionParser.java:99`), parses it with **this** module, builds a
//!   per-call-site `PrototypePieces` (callee fixed params ++ parsed varargs) and
//!   installs it via the existing `pending_proto_overrides` →
//!   `Override::insert_proto_override` plumbing, then re-decompiles. That is an
//!   engine-driver change and is gated OFF by default; it is intentionally left
//!   out of this commit.
//!
//! # Losses
//!
//! kuna has no `.gdt` typedef archive, so the typedef-backed extended-precision
//! types (`intmax_t`/`uintmax_t`/`size_t`/`ptrdiff_t`,
//! `FormatStringParser.getIntegralPointerType`/`getSizeT`/…) are modeled as
//! their plain integer fallbacks ([`spec_to_datatype`] follows the same
//! `getIntegralPointerType` width choice Ghidra's fabricated typedefs wrap) — a
//! documented stand-in matching the existing `s1_protos` libc-prototype table.

/// The **application half (B)** of `FormatStringAnalyzer`: the pure,
/// unit-testable call-site classification ([`apply::classify_variadic_call`])
/// and per-call-site override-pieces builder ([`apply::build_override_pieces`])
/// that the parser (this module) feeds.  The Funcdata pcode-walk + the
/// decompile→override→re-decompile loop live in the `kuna-console` driver.
pub mod apply;

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};

/// A single parsed format argument: the optional length modifier plus the
/// conversion specifier. Faithful mirror of Ghidra's `FormatArgument` (the
/// `(lengthModifier, conversionSpecifier)` pair). The conversion specifier is
/// `"*"` for a field-width/precision `*` that consumes an `int` argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FmtArg {
    /// Length modifier (`h`/`hh`/`l`/`ll`/`q`/`j`/`z`/`t`/`L`), or `None`.
    pub length_modifier: Option<String>,
    /// Conversion specifier (`d`/`i`/`u`/…/`s`/`p`/`n`), or `"*"` for a
    /// `*`-consumed field-width / precision argument.
    pub conversion_specifier: String,
}

impl FmtArg {
    fn new(length_modifier: Option<&str>, conversion_specifier: &str) -> Self {
        FmtArg {
            length_modifier: length_modifier.map(str::to_string),
            conversion_specifier: conversion_specifier.to_string(),
        }
    }
}

/// The implied C argument type for a conversion specifier — the result space of
/// Ghidra's `convertPairToDataType` / `conversionSpecifierToDataType`. The
/// extended-precision typedefs (`IntMaxT`/`UIntMaxT`/`SizeT`/`PtrDiffT`) are
/// distinct variants so callers can render them faithfully; [`spec_to_datatype`]
/// lowers them to their integer fallback (kuna has no `.gdt` typedefs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spec {
    /// `int` — `d`/`i`, and the value behind a `*` field width.
    Int,
    /// `unsigned int` — `o`/`u`/`x`/`X`.
    UInt,
    /// `short` — `hd`/`hi`.
    Short,
    /// `unsigned short` — `ho`/`hu`/`hx`/`hX`.
    UShort,
    /// `char` — `hhd`/`hhi`.
    Char,
    /// `unsigned char` — `c`, and `hho`/`hhu`/`hhx`/`hhX`.
    UChar,
    /// `long` — `ld`/`li`.
    Long,
    /// `unsigned long` — `lo`/`lu`/`lx`/`lX`.
    ULong,
    /// `long long` — `lld`/`lli`/`qd`/`qi`.
    LongLong,
    /// `unsigned long long` — `llo`/`llu`/…/`qo`/…
    ULongLong,
    /// `double` — `f`/`F`/`e`/`E`/`g`/`G`/`a`/`A`.
    Double,
    /// `long double` — `Lf`/`Le`/…
    LongDouble,
    /// `intmax_t` — `jd`/`ji` (typedef-backed; long long fallback).
    IntMaxT,
    /// `uintmax_t` — `jo`/`ju`/`jx`/`jX` (typedef-backed; unsigned long long fallback).
    UIntMaxT,
    /// `size_t` — `z…` (typedef-backed; `getIntegralPointerType(false)` fallback).
    SizeT,
    /// `ptrdiff_t` — `t…` signed (typedef-backed; `getIntegralPointerType(true)` fallback).
    PtrDiffT,
    /// `char *` — `s`.
    CharPtr,
    /// `wchar_t *` — `C`/`S`, `lc`/`ls`.
    WideCharPtr,
    /// `void *` — `p`.
    VoidPtr,
    /// `int *` — `n` (the length modifier widens the pointee, but the pointer
    /// metatype is the same; the pointee width is carried via the `*Ptr`
    /// variants below).
    IntPtr,
    /// `short *` — `hn`.
    ShortPtr,
    /// `char *` (signed) — `hhn`. Distinct from [`Spec::CharPtr`] (`s`, which is
    /// the plain `char` element type) only in origin; rendered identically.
    CharSignedPtr,
    /// `long *` — `ln`.
    LongPtr,
    /// `long long *` — `lln`/`qn`.
    LongLongPtr,
    /// `intmax_t *` / `uintmax_t *` — `jn` (typedef-backed).
    IntMaxTPtr,
    /// `size_t *` — `zn` (typedef-backed).
    SizeTPtr,
    /// `ptrdiff_t *` / `size_t *` — `tn` (typedef-backed).
    PtrDiffTPtr,
}

// ---------------------------------------------------------------------------
// Character-class predicates — `FormatStringParser.java:822-885`.
// ---------------------------------------------------------------------------

/// `isFlag` — `FormatStringParser.java:856`. Flag set `"0+ -#'"`.
fn is_flag(c: char) -> bool {
    "0+ -#'".contains(c)
}

/// `detectLengthModifier` — `FormatStringParser.java:877`. Set `"hljztLq"`.
fn detect_length_modifier(c: char) -> Option<String> {
    if "hljztLq".contains(c) {
        Some(c.to_string())
    } else {
        None
    }
}

/// `detectConversionSpecifier` — `FormatStringParser.java:882`. Set
/// `"diuofeaFEApcsxXgGnCS"`.
fn detect_conversion_specifier(c: char) -> Option<String> {
    if "diuofeaFEApcsxXgGnCS".contains(c) {
        Some(c.to_string())
    } else {
        None
    }
}

/// `isConversionSpecifier` — `FormatStringParser.java:869`.
fn is_conversion_specifier(c: char) -> bool {
    detect_conversion_specifier(c).is_some()
}

/// `isLengthModifier` — `FormatStringParser.java:873`.
fn is_length_modifier(c: char) -> bool {
    detect_length_modifier(c).is_some()
}

/// `isUnsignedInteger` — `FormatStringParser.java:832`. Set `"ouxX"`.
fn is_unsigned_integer(spec: &str) -> bool {
    spec.chars().next().is_some_and(|c| "ouxX".contains(c))
}

/// `isSignedInteger` — `FormatStringParser.java:838`. Set `"di"`.
fn is_signed_integer(spec: &str) -> bool {
    spec.chars().next().is_some_and(|c| "di".contains(c))
}

/// `isInteger` — `FormatStringParser.java:822`.
fn is_integer(spec: &str) -> bool {
    is_unsigned_integer(spec) || is_signed_integer(spec)
}

/// `isDouble` — `FormatStringParser.java:826`. Set `"aAeEfFgG"`.
fn is_double(spec: &str) -> bool {
    spec.chars().next().is_some_and(|c| "aAeEfFgG".contains(c))
}

/// `isIntegerPointer` — `FormatStringParser.java:844`. Set `"n"`.
fn is_integer_pointer(spec: &str) -> bool {
    spec.chars().next() == Some('n')
}

/// `isVoidPointer` — `FormatStringParser.java:850`. Set `"p"`.
fn is_void_pointer(spec: &str) -> bool {
    spec.chars().next() == Some('p')
}

/// `Character.isDigit` — ASCII-digit semantics (the format grammar is ASCII).
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

// ---------------------------------------------------------------------------
// Indexing helpers. Java indexes a `String` by UTF-16 char; the format grammar
// is ASCII, so we work over a `Vec<char>` to keep the index arithmetic a
// faithful 1:1 transcription of the Java (charAt / length / ++i).
// ---------------------------------------------------------------------------

/// `formatString.charAt(i)` — caller guarantees `i` is in bounds (the Java code
/// would throw `StringIndexOutOfBoundsException` otherwise; our callers bound it
/// exactly as the Java does).
#[inline]
fn char_at(s: &[char], i: usize) -> char {
    s[i]
}

// ---------------------------------------------------------------------------
// Mutable parse state — `FormatParsingData` (`FormatStringParser.java:896`).
// ---------------------------------------------------------------------------

#[derive(Default)]
struct FormatParsingData {
    conversion_specifier: Option<String>,
    length_modifier: Option<String>,
    field_width_complete: bool,
    precision_complete: bool,
    in_conversion: bool,
    parameter_index: i32,
}

impl FormatParsingData {
    /// `clearData` — `FormatStringParser.java:953`.
    fn clear_data(&mut self) {
        self.precision_complete = false;
        self.field_width_complete = false;
        self.length_modifier = None;
        self.conversion_specifier = None;
    }
}

// ---------------------------------------------------------------------------
// parseFormatString — `FormatStringParser.java:81`.
// Splits the format string into the "%…spec" run substrings (e.g.
// "%d %4.2s" → ["d", "4.2s"]). Returns None on truncation (Java returns null).
// ---------------------------------------------------------------------------

fn parse_format_string(fmt: &[char]) -> Option<Vec<Vec<char>>> {
    let mut format_argument_list: Vec<Vec<char>> = Vec::new();
    let mut current: Vec<char> = Vec::new();
    let mut i = 0usize;
    while i < fmt.len() {
        let mut c = char_at(fmt, i);
        if c == '%' {
            if emit_percent(fmt, i) {
                i += 1;
            } else {
                i += 1;
                if i >= fmt.len() {
                    // Java: charAt(i) throws → undefined behavior → null.
                    return None;
                }
                c = char_at(fmt, i);
                while !is_conversion_specifier(c) {
                    current.push(c);
                    i += 1;
                    if i >= fmt.len() {
                        return None;
                    }
                    c = char_at(fmt, i);
                }
                current.push(c);
                format_argument_list.push(std::mem::take(&mut current));
            }
        }
        i += 1;
    }
    Some(format_argument_list)
}

/// `emitPercent` — `FormatStringParser.java:554`. Two consecutive `%` → literal.
fn emit_percent(fmt: &[char], i: usize) -> bool {
    char_at(fmt, i) == '%' && i + 1 < fmt.len() && char_at(fmt, i + 1) == '%'
}

// ---------------------------------------------------------------------------
// convertToFormatArguments — `FormatStringParser.java:122`.
// Converts one "%…spec" run substring into FmtArg(s), appending to `out`.
// Returns false on any malformed branch.
// ---------------------------------------------------------------------------

fn convert_to_format_arguments(
    fmt: &[char],
    out: &mut Vec<FmtArg>,
    is_output_type: bool,
) -> bool {
    let mut data = FormatParsingData::default();
    let mut i = 0usize;
    while i < fmt.len() {
        let c = char_at(fmt, i);
        match preprocess_char(fmt, i as i64, is_output_type) {
            -1 => return false,
            ni => i = ni as usize,
        }
        if is_flag(c) {
            i += 1;
            continue;
        }
        if data.length_modifier.is_some() {
            return add_argument_with_modifier(c, &mut data, out);
        }
        data.length_modifier = detect_length_modifier(c);
        if data.length_modifier.is_none() {
            data.conversion_specifier = detect_conversion_specifier(c);
            if let Some(cs) = data.conversion_specifier.clone() {
                if !verify_conversion_pair(data.length_modifier.as_deref(), &cs) {
                    return false;
                }
                out.push(FmtArg::new(data.length_modifier.as_deref(), &cs));
                return true;
            }
            // No length modifier and no conversion spec: an unknown char here
            // is undefined behavior unless it is part of a width/precision.
            if data.precision_complete {
                return false;
            }
            if !is_digit(c) && c != '.' && c != '*' {
                return false;
            }
            let ni = if is_output_type {
                handle_output_conversion_argument(fmt, i, &mut data, out)
            } else {
                handle_input_conversion_argument(fmt, i, &mut data, out)
            };
            if ni == -1 {
                return false;
            }
            i = ni as usize;
        } else if i + 1 < fmt.len() {
            i = initiate_length_modifier_extension(fmt, i, &mut data);
        }
        i += 1;
    }
    true
}

/// `preprocessChar` — `FormatStringParser.java:179`. Returns -1 on the `$`
/// (positional) branch and the not-output flag branch; otherwise the (possibly
/// flag-skipped) index. Uses `i64` so the `-1` sentinel survives.
fn preprocess_char(fmt: &[char], i: i64, is_output_type: bool) -> i64 {
    let c = char_at(fmt, i as usize);
    if c == '$' {
        return -1;
    }
    if is_flag(c) {
        if !is_output_type {
            return -1;
        }
        return skip_flags(fmt, i as usize) as i64;
    }
    i
}

/// `initiateLengthModifierExtension` — `FormatStringParser.java:194`.
fn initiate_length_modifier_extension(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
) -> usize {
    let tmp = extend_length_modifier(
        data.length_modifier.as_deref().unwrap_or(""),
        char_at(fmt, i + 1),
    );
    if let Some(tmp) = tmp {
        i += 1;
        data.length_modifier = Some(tmp);
    }
    i
}

/// `addArgumentWithModifier` — `FormatStringParser.java:205`.
fn add_argument_with_modifier(
    c: char,
    data: &mut FormatParsingData,
    out: &mut Vec<FmtArg>,
) -> bool {
    data.conversion_specifier = detect_conversion_specifier(c);
    match data.conversion_specifier.clone() {
        Some(cs) if verify_conversion_pair(data.length_modifier.as_deref(), &cs) => {
            out.push(FmtArg::new(data.length_modifier.as_deref(), &cs));
            true
        }
        _ => false,
    }
}

/// `handleOutputConversionArgument` — `FormatStringParser.java:218`.
fn handle_output_conversion_argument(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
    out: &mut Vec<FmtArg>,
) -> i64 {
    let c = char_at(fmt, i);
    if !data.precision_complete && !data.field_width_complete && c != '.' {
        if c == '*' {
            out.push(FmtArg::new(None, "*"));
        } else {
            let ni = skip_integers(fmt, i);
            if ni == -1 {
                return ni;
            }
            i = ni as usize;
        }
        data.field_width_complete = true;
    } else if data.field_width_complete && c != '.' {
        return -1;
    } else if !data.precision_complete && c == '.' {
        if i + 1 < fmt.len() && char_at(fmt, i + 1) == '*' {
            i += 1;
            out.push(FmtArg::new(None, "*"));
        } else {
            let ni = skip_integers(fmt, i + 1);
            if ni == -1 {
                return ni;
            }
            i = ni as usize;
        }
        data.precision_complete = true;
    } else {
        return -1;
    }
    i as i64
}

/// `handleInputConversionArgument` — `FormatStringParser.java:255`.
fn handle_input_conversion_argument(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
    out: &mut Vec<FmtArg>,
) -> i64 {
    let c = char_at(fmt, i);
    if c == '*' {
        out.push(FmtArg::new(None, "*"));
    } else if is_digit(c) {
        let ni = skip_integers(fmt, i + 1);
        if ni == -1 {
            return ni;
        }
        i = ni as usize;
        data.precision_complete = true;
    } else {
        return -1;
    }
    i as i64
}

// ---------------------------------------------------------------------------
// analyzeFormatStringWithParameters — `FormatStringParser.java:334`.
// Positional ("%n$") format strings. Ghidra uses Map<Integer,FormatArgument>;
// we use a sparse Vec<Option<FmtArg>> indexed by (parameterIndex-1), and
// reproduce convertMapToList exactly (1..=len, null if any slot missing).
// ---------------------------------------------------------------------------

fn analyze_format_string_with_parameters(fmt: &[char]) -> Option<Vec<FmtArg>> {
    let mut data = FormatParsingData::default();
    // Sparse map keyed by parameter index (1-based) → FmtArg.
    let mut format_argument_map: Vec<Option<FmtArg>> = Vec::new();
    let mut i = 0usize;
    while i < fmt.len() {
        let c = char_at(fmt, i);
        if c == ' ' {
            i += 1;
            continue;
        }
        if c == '%' {
            if emit_percent(fmt, i) {
                // `++i`, then FALL THROUGH to the `isInConversion` block (Java
                // has no `continue` on this branch). The trailing `i += 1` is
                // the for-loop's increment, so `%%` advances by two.
                i += 1;
            } else {
                data.in_conversion = true;
                data.clear_data();
                data.parameter_index = locate_parameter_index(fmt, i);
                if data.parameter_index == 0 {
                    return None; // $ operand number is required
                }
                // i should advance to the '$'.
                i += data.parameter_index.to_string().len() + 1;
                if i + 1 >= fmt.len() {
                    return None; // Java charAt(i+1) would throw → undefined.
                }
                if is_flag(char_at(fmt, i + 1)) {
                    i = skip_flags(fmt, i + 1);
                }
                i += 1; // for-loop increment after the `continue`
                continue;
            }
        }
        if data.in_conversion {
            if data.length_modifier.is_some() {
                data.conversion_specifier = detect_conversion_specifier(c);
                match data.conversion_specifier.clone() {
                    None => return None,
                    Some(cs) => {
                        map_put(
                            &mut format_argument_map,
                            data.parameter_index,
                            FmtArg::new(data.length_modifier.as_deref(), &cs),
                        );
                        data.in_conversion = false;
                        i += 1;
                        continue;
                    }
                }
            }
            data.length_modifier = detect_length_modifier(c);
            if data.length_modifier.is_none() {
                let ni = search_with_null_modifier(fmt, i, &mut data, &mut format_argument_map);
                if ni == -1 {
                    return None;
                }
                i = ni as usize;
            }
        }
        i += 1;
    }
    convert_map_to_list(&format_argument_map)
}

/// `searchWithNullModifier` — `FormatStringParser.java:385`.
fn search_with_null_modifier(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
    map: &mut Vec<Option<FmtArg>>,
) -> i64 {
    let c = char_at(fmt, i);
    data.conversion_specifier = detect_conversion_specifier(c);
    if let Some(cs) = data.conversion_specifier.clone() {
        map_put(
            map,
            data.parameter_index,
            FmtArg::new(data.length_modifier.as_deref(), &cs),
        );
        data.in_conversion = false;
    } else {
        if data.precision_complete {
            return -1;
        }
        if !is_digit(c) && c != '.' && c != '*' {
            return -1;
        }
        if !data.precision_complete && !data.field_width_complete && c != '.' {
            let ni = handle_output_conversion_for_parameters(fmt, i, data, map);
            if ni == -1 {
                return -1;
            }
            i = ni as usize;
        } else if data.field_width_complete && c != '.' {
            return -1;
        } else if !data.precision_complete && c == '.' {
            let ni = handle_precision_for_parameters(fmt, i, data, map);
            if ni == -1 {
                return -1;
            }
            i = ni as usize;
        } else {
            return -1;
        }
    }
    i as i64
}

/// `handlePrecisionForParameters` — `FormatStringParser.java:426`.
fn handle_precision_for_parameters(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
    map: &mut Vec<Option<FmtArg>>,
) -> i64 {
    if i + 1 < fmt.len() && char_at(fmt, i + 1) == '*' {
        i += 1;
        let precision_idx = locate_parameter_index(fmt, i);
        if precision_idx == 0 {
            return -1;
        }
        i += precision_idx.to_string().len() + 1;
        map_put(map, precision_idx, FmtArg::new(None, "d"));
    } else {
        let ni = skip_integers(fmt, i + 1);
        if ni == -1 {
            return -1;
        }
        i = ni as usize;
    }
    data.precision_complete = true;
    i as i64
}

/// `handleOutputConversionForParameters` — `FormatStringParser.java:449`.
fn handle_output_conversion_for_parameters(
    fmt: &[char],
    mut i: usize,
    data: &mut FormatParsingData,
    map: &mut Vec<Option<FmtArg>>,
) -> i64 {
    let c = char_at(fmt, i);
    if c == '*' {
        let field_width_idx = locate_parameter_index(fmt, i);
        if field_width_idx == 0 {
            return i as i64;
        }
        i += field_width_idx.to_string().len() + 1;
        map_put(map, field_width_idx, FmtArg::new(None, "d"));
    } else {
        let ni = skip_integers(fmt, i);
        if ni == -1 {
            return ni;
        }
        i = ni as usize;
    }
    data.field_width_complete = true;
    i as i64
}

/// `convertMapToList` — `FormatStringParser.java:472`. Iterate 1..=size; any
/// missing slot → null (None).
fn convert_map_to_list(map: &[Option<FmtArg>]) -> Option<Vec<FmtArg>> {
    // Ghidra's `formatArgumentMap.size()` is the count of distinct keys; we
    // store sparsely, so the equivalent is the number of populated entries.
    let size = map.iter().filter(|e| e.is_some()).count();
    let mut list = Vec::with_capacity(size);
    for idx in 1..=size {
        match map.get(idx - 1).and_then(|e| e.clone()) {
            None => return None,
            Some(arg) => list.push(arg),
        }
    }
    Some(list)
}

/// Insert into the sparse 1-based map (`formatArgumentMap.put(idx, arg)`).
fn map_put(map: &mut Vec<Option<FmtArg>>, idx: i32, arg: FmtArg) {
    if idx < 1 {
        return;
    }
    let slot = (idx - 1) as usize;
    if slot >= map.len() {
        map.resize(slot + 1, None);
    }
    map[slot] = Some(arg);
}

/// `locateParameterIndex` — `FormatStringParser.java:494`. Reads the `n` from
/// `%n$` / `*n$`; 0 on any problem.
fn locate_parameter_index(fmt: &[char], mut i: usize) -> i32 {
    let mut c = char_at(fmt, i);
    if c == '%' || c == '*' {
        i += 1;
        if i >= fmt.len() {
            return 0;
        }
        c = char_at(fmt, i);
    } else {
        return 0;
    }
    let mut param_index_string = String::new();
    while is_digit(c) {
        param_index_string.push(c);
        i += 1;
        if i >= fmt.len() {
            return 0;
        }
        c = char_at(fmt, i);
    }
    if c != '$' || param_index_string.is_empty() {
        return 0;
    }
    match param_index_string.parse::<i32>() {
        Ok(0) | Err(_) => 0,
        Ok(n) => n,
    }
}

// ---------------------------------------------------------------------------
// skipFlags / skipIntegers — `FormatStringParser.java:523` / `:539`.
// ---------------------------------------------------------------------------

/// `skipFlags` — returns the index of the last flag char (one before the next
/// non-flag). The caller's `++i` then steps past it.
fn skip_flags(fmt: &[char], mut i: usize) -> usize {
    while i < fmt.len() && is_flag(char_at(fmt, i)) {
        i += 1;
    }
    i.wrapping_sub(1)
}

/// `skipIntegers` — `FormatStringParser.java:539`. Returns the index of the last
/// digit, or -1 on undefined input. (Length modifier / conversion specifier
/// immediately here means a zero-width run, returning `i-1`.)
fn skip_integers(fmt: &[char], i: usize) -> i64 {
    if i >= fmt.len() {
        return -1;
    }
    let c = char_at(fmt, i);
    if !is_digit(c) {
        if is_length_modifier(c) || is_conversion_specifier(c) {
            return i as i64 - 1;
        }
        return -1;
    }
    let mut j = i;
    while j < fmt.len() && is_digit(char_at(fmt, j)) {
        j += 1;
    }
    j as i64 - 1
}

// ---------------------------------------------------------------------------
// verifyConversionPair / extendLengthModifier — `:608` / `:861`.
// ---------------------------------------------------------------------------

/// `verifyConversionPair` — `FormatStringParser.java:608`.
fn verify_conversion_pair(length_modifier: Option<&str>, conversion_specifier: &str) -> bool {
    match length_modifier {
        None => true,
        Some("l") => true,
        Some("L") if is_double(conversion_specifier) => true,
        Some(lm) if lm != "L" && (is_integer(conversion_specifier)
            || is_integer_pointer(conversion_specifier)) =>
        {
            true
        }
        _ => false,
    }
}

/// `extendLengthModifier` — `FormatStringParser.java:861`. Only `h→hh`, `l→ll`.
fn extend_length_modifier(length_modifier: &str, next_char: char) -> Option<String> {
    if (length_modifier == "h" && next_char == 'h')
        || (length_modifier == "l" && next_char == 'l')
    {
        Some(format!("{length_modifier}{next_char}"))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// convertToFormatArgumentList — `FormatStringParser.java:286`.
// The public spec-list entry point (printf-vs-scanf via is_output_type).
// ---------------------------------------------------------------------------

/// Port of `convertToFormatArgumentList`. Returns the ordered [`FmtArg`] list,
/// or `None` on any of Ghidra's "undefined behavior" branches.
pub fn convert_to_format_argument_list(fmt: &str, is_output_type: bool) -> Option<Vec<FmtArg>> {
    let chars: Vec<char> = fmt.chars().collect();
    let format_str_argument_list = parse_format_string(&chars)?;
    let mut format_argument_list: Vec<FmtArg> = Vec::new();
    for format_str_argument in &format_str_argument_list {
        let ok = convert_to_format_arguments(
            format_str_argument,
            &mut format_argument_list,
            is_output_type,
        );
        if !ok {
            // Java: if any substring contains '$', retry as a positional string.
            let has_positional = format_str_argument_list
                .iter()
                .any(|s| s.iter().any(|&c| c == '$'));
            if has_positional {
                return analyze_format_string_with_parameters(&chars);
            }
            return None;
        }
    }
    Some(format_argument_list)
}

// ---------------------------------------------------------------------------
// convertPairToDataType / conversionSpecifierToDataType — `:620` / `:651`,
// plus the length-modification handlers `:685`-`:820`. Lowered to the `Spec`
// enum (the typedef-backed extended types are kept as distinct variants).
// ---------------------------------------------------------------------------

/// `convertPairToDataType` — `FormatStringParser.java:620`. Returns `None` for
/// the default (unhandled) branches.
fn convert_pair_to_spec(length_modifier: Option<&str>, conversion_specifier: &str) -> Option<Spec> {
    let cs = conversion_specifier;
    if length_modifier.is_none() || cs == "c" || cs == "s" || cs == "C" || cs == "S" {
        return conversion_specifier_to_spec(cs);
    }
    match length_modifier.unwrap() {
        "h" => Some(short_length_modification(cs)),
        "hh" => Some(char_length_modification(cs)),
        "l" => Some(long_length_modification(cs)),
        "ll" | "q" => Some(long_long_length_modification(cs)),
        "j" => Some(intmax_t_length_modification(cs)),
        "z" => Some(size_t_length_modification(cs)),
        "t" => Some(ptrdiff_t_length_modification(cs)),
        "L" => Some(long_double_length_modification(cs)),
        _ => None,
    }
}

/// `conversionSpecifierToDataType` — `FormatStringParser.java:651`.
fn conversion_specifier_to_spec(conversion_specifier: &str) -> Option<Spec> {
    match conversion_specifier.chars().next()? {
        'd' | 'i' => Some(Spec::Int),
        'o' | 'u' | 'x' | 'X' => Some(Spec::UInt),
        'p' => Some(Spec::VoidPtr),
        's' => Some(Spec::CharPtr),
        'n' => Some(Spec::IntPtr),
        'c' => Some(Spec::UChar),
        'a' | 'A' | 'g' | 'G' | 'e' | 'E' | 'f' | 'F' => Some(Spec::Double),
        'S' | 'C' => Some(Spec::WideCharPtr),
        _ => None,
    }
}

/// `longLengthModification` — `FormatStringParser.java:685`.
fn long_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        return Spec::LongPtr;
    }
    if cs == "s" || cs == "c" {
        return Spec::WideCharPtr;
    }
    if is_signed_integer(cs) {
        Spec::Long
    } else {
        Spec::ULong
    }
}

/// `longLongLengthModification` — `FormatStringParser.java:696`.
fn long_long_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        return Spec::LongLongPtr;
    }
    if is_signed_integer(cs) {
        Spec::LongLong
    } else {
        Spec::ULongLong
    }
}

/// `shortLengthModification` — `FormatStringParser.java:705`.
fn short_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        return Spec::ShortPtr;
    }
    if is_signed_integer(cs) {
        Spec::Short
    } else {
        Spec::UShort
    }
}

/// `charLengthModification` — `FormatStringParser.java:714`.
fn char_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        return Spec::CharSignedPtr;
    }
    if is_signed_integer(cs) {
        Spec::Char
    } else {
        Spec::UChar
    }
}

/// `intmax_t_LengthModification` — `FormatStringParser.java:797`.
fn intmax_t_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        return Spec::IntMaxTPtr;
    }
    if is_unsigned_integer(cs) {
        Spec::UIntMaxT
    } else {
        Spec::IntMaxT
    }
}

/// `size_t_LengthModification` — `FormatStringParser.java:804`.
fn size_t_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        Spec::SizeTPtr
    } else {
        Spec::SizeT
    }
}

/// `ptrdiff_t_LengthModification` — `FormatStringParser.java:811`.
/// (Unsigned → `size_t`, signed → `ptrdiff_t`, matching Ghidra.)
fn ptrdiff_t_length_modification(cs: &str) -> Spec {
    if is_integer_pointer(cs) {
        if is_unsigned_integer(cs) {
            Spec::SizeTPtr
        } else {
            Spec::PtrDiffTPtr
        }
    } else if is_unsigned_integer(cs) {
        Spec::SizeT
    } else {
        Spec::PtrDiffT
    }
}

/// `longDoubleLengthModification` — `FormatStringParser.java:818`.
fn long_double_length_modification(_cs: &str) -> Spec {
    Spec::LongDouble
}

/// True if this [`Spec`] is already a pointer type (so `convertToInputDataTypes`
/// does not wrap it again, except for the void pointer `p`, which it does wrap —
/// `FormatStringParser.java:597`).
fn spec_is_pointer(spec: Spec) -> bool {
    matches!(
        spec,
        Spec::CharPtr
            | Spec::WideCharPtr
            | Spec::VoidPtr
            | Spec::IntPtr
            | Spec::ShortPtr
            | Spec::CharSignedPtr
            | Spec::LongPtr
            | Spec::LongLongPtr
            | Spec::IntMaxTPtr
            | Spec::SizeTPtr
            | Spec::PtrDiffTPtr
    )
}

// ---------------------------------------------------------------------------
// Public type-list entry points — convertToOutputDataTypes / InputDataTypes.
// ---------------------------------------------------------------------------

/// Parse a `printf`-family format string into the ordered implied argument
/// types. Port of `convertToFormatArgumentList(fmt, true)` →
/// `convertToOutputDataTypes` (`FormatStringParser.java:562`). A `*`
/// field-width/precision spec contributes an [`Spec::Int`]. Returns an empty
/// list on any undefined-behavior branch (Ghidra returns null → no override).
pub fn parse_output_types(fmt: &str) -> Vec<Spec> {
    let Some(args) = convert_to_format_argument_list(fmt, true) else {
        return Vec::new();
    };
    let mut specs = Vec::with_capacity(args.len());
    for arg in &args {
        // convertToOutputDataTypes: "*" → "d" (an int), else the pair's type.
        let cs = if arg.conversion_specifier == "*" {
            "d"
        } else {
            arg.conversion_specifier.as_str()
        };
        match convert_pair_to_spec(arg.length_modifier.as_deref(), cs) {
            Some(s) => specs.push(s),
            None => return Vec::new(), // dataTypeList.contains(null) → null
        }
    }
    specs
}

/// Parse a `scanf`-family format string into the ordered implied argument types.
/// Port of `convertToFormatArgumentList(fmt, false)` → `convertToInputDataTypes`
/// (`FormatStringParser.java:576`): a `*` spec skips the *following* argument
/// (assignment-suppression), and every non-pointer value type is wrapped in an
/// extra pointer (scanf takes addresses). Returns an empty list on any
/// undefined-behavior branch.
pub fn parse_input_types(fmt: &str) -> Vec<Spec> {
    let Some(args) = convert_to_format_argument_list(fmt, false) else {
        return Vec::new();
    };
    let mut specs = Vec::new();
    let mut i = 0usize;
    while i < args.len() {
        let arg = &args[i];
        if arg.conversion_specifier == "*" {
            // Two consecutive '*' is undefined → null.
            if i + 1 < args.len() && args[i + 1].conversion_specifier == "*" {
                return Vec::new();
            }
            i += 2; // skip the suppressed argument
            continue;
        }
        match convert_pair_to_spec(arg.length_modifier.as_deref(), &arg.conversion_specifier) {
            None => return Vec::new(),
            Some(value) => {
                // convertToInputDataTypes (`:597`): wrap in an extra pointer
                // unless already a pointer — except the void pointer `p`, which
                // IS wrapped (`isVoidPointer`).
                if !spec_is_pointer(value) || is_void_pointer(&arg.conversion_specifier) {
                    specs.push(pointer_to(value));
                } else {
                    specs.push(value);
                }
            }
        }
        i += 1;
    }
    specs
}

/// The pointer-to-`spec` [`Spec`] for the scanf wrapping. The value types map to
/// their `*Ptr` cousin; a void pointer becomes itself wrapped (modeled as
/// [`Spec::VoidPtr`], a `void **` — kuna has no second-level pointer variant, so
/// this is the documented approximation, matching the libproto `void*` stand-in).
fn pointer_to(spec: Spec) -> Spec {
    match spec {
        Spec::Int => Spec::IntPtr,
        Spec::Short => Spec::ShortPtr,
        Spec::Char | Spec::UChar => Spec::CharSignedPtr,
        Spec::Long => Spec::LongPtr,
        Spec::LongLong => Spec::LongLongPtr,
        Spec::IntMaxT | Spec::UIntMaxT => Spec::IntMaxTPtr,
        Spec::SizeT => Spec::SizeTPtr,
        Spec::PtrDiffT => Spec::PtrDiffTPtr,
        // UInt/ULong/ULongLong/UShort: model the pointee as the same-width
        // signed pointer (kuna's *Ptr variants are width-, not signedness-,
        // carrying for the pointee; documented approximation).
        Spec::UInt => Spec::IntPtr,
        Spec::UShort => Spec::ShortPtr,
        Spec::ULong => Spec::LongPtr,
        Spec::ULongLong => Spec::LongLongPtr,
        Spec::Double | Spec::LongDouble => Spec::VoidPtr,
        // Already a pointer (p, etc.): another level — modeled as void*.
        _ => Spec::VoidPtr,
    }
}

/// Convenience alias matching the task brief's named entry point: parse a
/// `printf`-style format string into the ordered argument specifiers. Identical
/// to [`parse_output_types`].
pub fn parse_format_specifiers(fmt: &str) -> Vec<Spec> {
    parse_output_types(fmt)
}

// ---------------------------------------------------------------------------
// spec_to_datatype — the kuna analog of building the DataType, mirroring
// s1_protos::build_ty. The typedef-backed extended types are lowered to the
// same integer fallback Ghidra's getIntegralPointerType / fabricated typedefs
// pick (kuna has no .gdt typedefs — a documented LOSS).
// ---------------------------------------------------------------------------

/// Build the kuna [`Datatype`] for a [`Spec`] using the architecture's type
/// factory — the analog of `s1_protos::build_ty`. `word_size` is the addressable
/// word size (the `.2` of `Architecture::data_org`). The extended-precision
/// typedefs are modeled as their integer fallback per
/// `FormatStringParser.getIntegralPointerType` (kuna has no `.gdt` typedefs).
pub fn spec_to_datatype(
    spec: Spec,
    types: &dyn TypeFactory,
    word_size: uint4,
) -> KunaResult<Rc<Datatype>> {
    let ptr = types.get_size_of_pointer();
    let int_sz = types.get_size_of_int();
    let long_sz = types.get_size_of_long();
    let char_sz = types.get_size_of_char();
    let wchar_sz = types.get_size_of_wchar();

    // getIntegralPointerType(signed): pointer-width int if int<=ptr<long, else long.
    let integral_pointer = |signed: bool| -> (int4, type_metatype) {
        let m = if signed {
            type_metatype::TYPE_INT
        } else {
            type_metatype::TYPE_UINT
        };
        if ptr < long_sz && ptr >= int_sz {
            (int_sz, m)
        } else {
            (long_sz, m)
        }
    };

    // Build a base integer/float of (size, metatype).
    let base = |s: int4, m: type_metatype| types.get_base(s, m);
    // Build a pointer to a base integer/float of (size, metatype).
    let ptr_to_base = |s: int4, m: type_metatype| -> KunaResult<Rc<Datatype>> {
        let pointee = types.get_base(s, m)?;
        types.get_type_pointer(ptr, pointee, word_size)
    };

    match spec {
        Spec::Int => base(int_sz, type_metatype::TYPE_INT),
        Spec::UInt => base(int_sz, type_metatype::TYPE_UINT),
        Spec::Short => base(2, type_metatype::TYPE_INT),
        Spec::UShort => base(2, type_metatype::TYPE_UINT),
        Spec::Char => base(char_sz, type_metatype::TYPE_INT),
        Spec::UChar => base(char_sz, type_metatype::TYPE_UINT),
        Spec::Long => base(long_sz, type_metatype::TYPE_INT),
        Spec::ULong => base(long_sz, type_metatype::TYPE_UINT),
        Spec::LongLong => base(8, type_metatype::TYPE_INT),
        Spec::ULongLong => base(8, type_metatype::TYPE_UINT),
        Spec::Double => base(8, type_metatype::TYPE_FLOAT),
        Spec::LongDouble => base(8, type_metatype::TYPE_FLOAT),
        // Typedef-backed: integer fallback (no .gdt typedefs in kuna).
        Spec::IntMaxT => base(8, type_metatype::TYPE_INT),
        Spec::UIntMaxT => base(8, type_metatype::TYPE_UINT),
        Spec::SizeT => {
            let (s, m) = integral_pointer(false);
            base(s, m)
        }
        Spec::PtrDiffT => {
            let (s, m) = integral_pointer(true);
            base(s, m)
        }
        Spec::CharPtr => {
            let c = types.get_type_char(char_sz)?;
            types.get_type_pointer(ptr, c, word_size)
        }
        Spec::WideCharPtr => {
            let wc = types.get_type_char(wchar_sz)?;
            types.get_type_pointer(ptr, wc, word_size)
        }
        Spec::VoidPtr => {
            let v = types.get_type_void()?;
            types.get_type_pointer(ptr, v, word_size)
        }
        Spec::IntPtr => ptr_to_base(int_sz, type_metatype::TYPE_INT),
        Spec::ShortPtr => ptr_to_base(2, type_metatype::TYPE_INT),
        Spec::CharSignedPtr => {
            let c = types.get_type_char(char_sz)?;
            types.get_type_pointer(ptr, c, word_size)
        }
        Spec::LongPtr => ptr_to_base(long_sz, type_metatype::TYPE_INT),
        Spec::LongLongPtr => ptr_to_base(8, type_metatype::TYPE_INT),
        Spec::IntMaxTPtr => ptr_to_base(8, type_metatype::TYPE_INT),
        Spec::SizeTPtr => {
            let (s, m) = integral_pointer(false);
            ptr_to_base(s, m)
        }
        Spec::PtrDiffTPtr => {
            let (s, m) = integral_pointer(true);
            ptr_to_base(s, m)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- The portable unit gate (testcase tier 1, plan §s1-formatstring). ---

    #[test]
    fn basic_printf_specs() {
        assert_eq!(parse_output_types("%d %s"), vec![Spec::Int, Spec::CharPtr]);
        assert_eq!(parse_output_types("%lu"), vec![Spec::ULong]);
        assert_eq!(parse_output_types("%p"), vec![Spec::VoidPtr]);
        assert!(parse_output_types("%%").is_empty());
        // The brief's named alias agrees with parse_output_types.
        assert_eq!(parse_format_specifiers("%d %s"), vec![Spec::Int, Spec::CharPtr]);
    }

    #[test]
    fn brief_examples() {
        // "%d %s" → [Int, CharPtr]
        assert_eq!(parse_output_types("%d %s"), vec![Spec::Int, Spec::CharPtr]);
        // "%5.2f" → [Double]
        assert_eq!(parse_output_types("%5.2f"), vec![Spec::Double]);
        // "%ld%%%p" → [Long, VoidPtr]  (the %% is a literal, no arg)
        assert_eq!(parse_output_types("%ld%%%p"), vec![Spec::Long, Spec::VoidPtr]);
        // "%*d" → [Int(width), Int]
        assert_eq!(parse_output_types("%*d"), vec![Spec::Int, Spec::Int]);
        // "%2$s %1$d" → positional, ordered by parameter index → [Int, CharPtr]
        assert_eq!(
            parse_output_types("%2$s %1$d"),
            vec![Spec::Int, Spec::CharPtr]
        );
    }

    #[test]
    fn integer_conversion_table() {
        assert_eq!(parse_output_types("%i"), vec![Spec::Int]);
        assert_eq!(parse_output_types("%u"), vec![Spec::UInt]);
        assert_eq!(parse_output_types("%x"), vec![Spec::UInt]);
        assert_eq!(parse_output_types("%X"), vec![Spec::UInt]);
        assert_eq!(parse_output_types("%o"), vec![Spec::UInt]);
        assert_eq!(parse_output_types("%c"), vec![Spec::UChar]);
        assert_eq!(parse_output_types("%n"), vec![Spec::IntPtr]);
    }

    #[test]
    fn length_modifiers() {
        assert_eq!(parse_output_types("%ld"), vec![Spec::Long]);
        assert_eq!(parse_output_types("%lu"), vec![Spec::ULong]);
        assert_eq!(parse_output_types("%lld"), vec![Spec::LongLong]);
        assert_eq!(parse_output_types("%llu"), vec![Spec::ULongLong]);
        assert_eq!(parse_output_types("%qd"), vec![Spec::LongLong]);
        assert_eq!(parse_output_types("%hd"), vec![Spec::Short]);
        assert_eq!(parse_output_types("%hu"), vec![Spec::UShort]);
        assert_eq!(parse_output_types("%hhd"), vec![Spec::Char]);
        assert_eq!(parse_output_types("%hhu"), vec![Spec::UChar]);
        assert_eq!(parse_output_types("%jd"), vec![Spec::IntMaxT]);
        assert_eq!(parse_output_types("%ju"), vec![Spec::UIntMaxT]);
        assert_eq!(parse_output_types("%zd"), vec![Spec::SizeT]);
        assert_eq!(parse_output_types("%zu"), vec![Spec::SizeT]);
        assert_eq!(parse_output_types("%td"), vec![Spec::PtrDiffT]);
        assert_eq!(parse_output_types("%Lf"), vec![Spec::LongDouble]);
    }

    #[test]
    fn wide_string_and_char() {
        // Faithful to Ghidra: convertPairToDataType's guard (`:622`) returns
        // *before* the length-modifier switch when the spec is s/c/S/C, so the
        // length modifier is ignored for these. `%ls`/`%lc` therefore map via
        // conversionSpecifierToDataType (char* / unsigned char) — the wide-char
        // branch in longLengthModification is unreachable for the printf path.
        assert_eq!(parse_output_types("%ls"), vec![Spec::CharPtr]);
        assert_eq!(parse_output_types("%lc"), vec![Spec::UChar]);
        // Capital C/S ARE wide (conversionSpecifierToDataType, `:677`).
        assert_eq!(parse_output_types("%S"), vec![Spec::WideCharPtr]);
        assert_eq!(parse_output_types("%C"), vec![Spec::WideCharPtr]);
    }

    #[test]
    fn float_family_is_double() {
        for spec in ["%f", "%F", "%e", "%E", "%g", "%G", "%a", "%A"] {
            assert_eq!(parse_output_types(spec), vec![Spec::Double], "spec {spec}");
        }
    }

    #[test]
    fn n_with_length_modifier_widens_pointee() {
        assert_eq!(parse_output_types("%ln"), vec![Spec::LongPtr]);
        assert_eq!(parse_output_types("%lln"), vec![Spec::LongLongPtr]);
        assert_eq!(parse_output_types("%hn"), vec![Spec::ShortPtr]);
        assert_eq!(parse_output_types("%hhn"), vec![Spec::CharSignedPtr]);
    }

    #[test]
    fn flags_width_precision() {
        // Flags + width + precision consume no extra args (only the value).
        assert_eq!(parse_output_types("%-+ #05.3d"), vec![Spec::Int]);
        assert_eq!(parse_output_types("%5.2f"), vec![Spec::Double]);
        // '*' for both width and precision: two ints then the value.
        assert_eq!(
            parse_output_types("%*.*f"),
            vec![Spec::Int, Spec::Int, Spec::Double]
        );
        // '*' precision only: int then value.
        assert_eq!(parse_output_types("%.*d"), vec![Spec::Int, Spec::Int]);
    }

    #[test]
    fn multiple_specifiers_mixed() {
        assert_eq!(
            parse_output_types("name=%s age=%d weight=%5.2f ptr=%p"),
            vec![Spec::CharPtr, Spec::Int, Spec::Double, Spec::VoidPtr]
        );
    }

    #[test]
    fn literal_percent_consumes_no_arg() {
        assert!(parse_output_types("%%").is_empty());
        assert!(parse_output_types("100%% done").is_empty());
        assert_eq!(parse_output_types("%d%%"), vec![Spec::Int]);
        assert_eq!(parse_output_types("%%%d"), vec![Spec::Int]);
    }

    #[test]
    fn no_specifiers_is_empty() {
        // The fauxware negative case: printf("Go away!") → no varargs.
        assert!(parse_output_types("Go away!").is_empty());
        assert!(parse_output_types("plain text\n").is_empty());
        assert!(parse_output_types("").is_empty());
    }

    #[test]
    fn positional_arguments() {
        // Reordered: index 1 is %d (int), index 2 is %s (char*).
        assert_eq!(
            parse_output_types("%2$s %1$d"),
            vec![Spec::Int, Spec::CharPtr]
        );
        // Same order positional.
        assert_eq!(
            parse_output_types("%1$d %2$s"),
            vec![Spec::Int, Spec::CharPtr]
        );
        // A gap (1 and 3, no 2) is unsupported → empty.
        assert!(parse_output_types("%1$d %3$s").is_empty());
    }

    #[test]
    fn malformed_truncated_no_panic() {
        // Truncated specs / garbage must not panic and yield empty.
        for bad in [
            "%",      // dangling percent
            "%l",     // length modifier, no conversion
            "%ll",    // double length modifier, no conversion
            "%5",     // width, no conversion
            "%5.",    // precision dot, no conversion
            "%.",     // dot, nothing
            "%#",     // flag only
            "%z",     // length modifier only
            "%hZ",    // bad conversion after modifier
            "%*",     // width star, no conversion
            "%1$",    // positional, no conversion
            "%$d",    // positional with empty index
            "%0$d",   // positional index zero
            "abc%",   // text then dangling percent
        ] {
            let out = parse_output_types(bad);
            // Either empty (rejected) — must never panic.
            let _ = out;
        }
        // A few that specifically must be empty (clearly undefined).
        assert!(parse_output_types("%").is_empty());
        assert!(parse_output_types("%l").is_empty());
        assert!(parse_output_types("%hZ").is_empty());
        assert!(parse_output_types("%0$d").is_empty());
    }

    #[test]
    fn unicode_in_literal_text_no_panic() {
        // Multi-byte chars in the literal portion must not break char indexing.
        assert_eq!(parse_output_types("café %d ☕"), vec![Spec::Int]);
        assert!(parse_output_types("naïve %%").is_empty());
    }

    // --- scanf (isOutputType = false): non-pointers get an extra pointer. ---

    #[test]
    fn scanf_wraps_non_pointers() {
        // scanf("%d") expects an int* (address to write into).
        assert_eq!(parse_input_types("%d"), vec![Spec::IntPtr]);
        // %s is already char* — not double-wrapped.
        assert_eq!(parse_input_types("%s"), vec![Spec::CharPtr]);
        // %f → double*, modeled as VoidPtr (no double-pointer Spec variant).
        assert_eq!(parse_input_types("%f"), vec![Spec::VoidPtr]);
        // %ld → long*.
        assert_eq!(parse_input_types("%ld"), vec![Spec::LongPtr]);
        // mixed
        assert_eq!(
            parse_input_types("%d %s"),
            vec![Spec::IntPtr, Spec::CharPtr]
        );
    }

    #[test]
    fn scanf_star_suppresses_next_arg() {
        // %*d: assignment-suppressed, consumes no argument; trailing %d → int*.
        assert_eq!(parse_input_types("%*d %d"), vec![Spec::IntPtr]);
        // A lone suppressed conversion yields no argument.
        assert!(parse_input_types("%*d").is_empty());
    }

    #[test]
    fn scanf_p_is_wrapped() {
        // convertToInputDataTypes wraps the void pointer too (line 597-599).
        assert_eq!(parse_input_types("%p"), vec![Spec::VoidPtr]);
    }

    // --- FmtArg-level intermediate, faithful to FormatArgument. ---

    #[test]
    fn fmtarg_intermediate_pairs() {
        let args = convert_to_format_argument_list("%ld %s", true).unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].length_modifier.as_deref(), Some("l"));
        assert_eq!(args[0].conversion_specifier, "d");
        assert_eq!(args[1].length_modifier, None);
        assert_eq!(args[1].conversion_specifier, "s");
    }
}
