//! Console-script construction and path quoting for single-function requests.

use std::borrow::Cow;
use std::path::Path;
use kuna_console::engine::EntrySelector;
use super::DecompileArgs;

/// An addressed VMA, excluding section coordinates and invalid numbers.
fn selected_vma(target: &str, by_address: bool) -> Option<u64> {
    if !by_address {
        return None;
    }
    if matches!(
        EntrySelector::parse(target),
        EntrySelector::SectionOffset { .. } | EntrySelector::SectionIndexOffset { .. }
    ) {
        return None;
    }
    let digits = target.strip_prefix("0x").or_else(|| target.strip_prefix("0X")).unwrap_or(target);
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(digits, 16).ok()
}

/// Borrow ordinary paths; quote only characters the console would split.
pub(super) fn console_path(path: &str) -> Cow<'_, str> {
    if !path.as_bytes().iter().any(|b| b.is_ascii_whitespace() || *b == b'"') {
        return Cow::Borrowed(path);
    }
    Cow::Owned(format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\"")))
}

/// The line-based transport cannot carry embedded newlines, even when quoted.
pub(super) fn reject_unquotable(what: &str, path: &str) -> Result<(), String> {
    match path.find(['\n', '\r']) {
        None => Ok(()),
        Some(_) => Err(format!(
            "{what} contains a newline, which the decomp_dbg console script \
             (one command per line) cannot carry: {path:?}"
        )),
    }
}

/// Lower the parsed request using this attempt's resolved input and defaults.
pub(super) fn build_script_for_input(
    args: &DecompileArgs,
    binary: &str,
    by_address: bool,
    out_path: &Path,
    injected: &[(&'static str, &'static str)],
    regions_path: Option<&Path>,
) -> String {
    let DecompileArgs {
        target, bfd_target, raw_image, base, raw, options, kasserts, func_decls, assertions, ..
    } = args;
    let target = target.as_str();
    let bfd_target = bfd_target.as_deref();
    let mut lines: Vec<String> = Vec::new();
    // The function this run selected, when it was named rather than addressed:
    // a directive qualified with it binds to the selection, and one qualified
    // with any other function does not (`crate::assertdecl::console_form`).
    let selected = if by_address { None } else { Some(target) };
    // The console lines of one slot, in the order the caller gave them.  A
    // directive that does not bind this run has no line and is reported by
    // `assertion_outcomes` instead.
    let forms = |slot| {
        assertions
            .iter()
            .filter_map(|d| crate::assertdecl::console_form(d, selected).ok())
            .filter(move |f| f.slot == slot)
    };
    let image = console_path(binary);
    if *raw_image {
        let language = bfd_target.expect("raw parser requires --target");
        let base = base.expect("raw parser requires --base");
        lines.push(format!("load raw {language} 0x{base:x} {target} {image}"));
    } else {
        match bfd_target {
            Some(t) if !t.is_empty() => lines.push(format!("load file {t} {image}")),
            _ => lines.push(format!("load file {image}")),
        }
    }
    // `option` lines MUST precede `read symbols`: the kuna_analysis passes are
    // committed (gated by the per-pass `--option <id> on|off` flags) inside
    // `read symbols` (IfcReadSymbols -> commit_pending_analysis). Emitting the
    // options first lets a per-run pass gate take effect; an option after the
    // commit would be a no-op (the analysis-port conflict #4 ordering fix). The
    // upstream/printer options here are order-independent w.r.t. `read symbols`.
    //
    // The driver defaults this attempt takes, from the shared table
    // (`decompile_all::driver_default_options`) — the DIV-15 Listing always, and
    // the DIV-20/DIV-68 non-x86-64 discovery bundle only on the retry (see
    // `decompile`).
    for (name, value) in injected {
        lines.push(format!("option {name} {value}"));
    }
    // (kuna `--assert`) A `readonly` range is inert unless read-only propagation
    // is on, and that option is default-off; asserting the range turns it on.
    // Emitted BEFORE the caller's own `--option` lines so an explicit
    // `--option readonly off` still wins.
    if kuna_console::assertions::implies_readonly_propagation(assertions) {
        lines.push("option readonly on".into());
    }
    for (name, value) in options {
        lines.push(format!("option {name} {value}"));
    }
    // (kuna `--assert`) IMAGE-scoped directives -- a read-only or volatile
    // memory range -- must precede `read symbols`: mapping a symbol folds the
    // range property into its SymbolEntry and never looks at the range again.
    for form in forms(crate::assertdecl::Slot::Image) {
        lines.push(form.line);
    }
    lines.push("read symbols".into());
    // `--define-function` AFTER the analysis commit and BEFORE the load: a
    // caller-declared boundary is an assertion that outranks whatever discovery
    // decided about the same address, and the load below is what consults the
    // declared extent (`ConsoleProgram::declared_extent`).
    for decl in func_decls {
        lines.push(decl.console_line());
    }
    // (kuna, RE-need `prototype-assertion-rejects-explicit`) A by-address run
    // ensures that a function symbol exists where it points: `load addr` follows flow
    // from the address without installing a `FunctionSymbol`, so a directive
    // naming the very address this run decompiles was answered `no function
    // starts at 0x…` while the body was emitted in full.  This is the same
    // install `--define-function <start>` performs — skipped when the caller
    // already declared that start, whose extent a second bare declaration would
    // clear.
    if let Some(vma) = selected_vma(target, by_address) {
        if !func_decls.iter().any(|decl| decl.start == vma) {
            lines.push(format!("function symbol {vma:#x}"));
        }
    }
    // (kuna `--assert`) The PROGRAM-scoped directives -- a parsed type, a
    // declared prototype, a named global -- go here, after the analysis commit
    // and before the selection, so the function is loaded against them.
    for form in forms(crate::assertdecl::Slot::Program) {
        lines.push(form.line);
    }
    if by_address {
        match EntrySelector::parse(target) {
            EntrySelector::SectionOffset { .. } | EntrySelector::SectionIndexOffset { .. } => {
                lines.push(format!("load function {target}"));
            }
            _ => {
                let addr = if target.starts_with("0x") || target.starts_with("0X") {
                    target.to_string()
                } else {
                    format!("0x{target}")
                };
                lines.push(format!("load addr {addr}"));
            }
        }
    } else {
        lines.push(format!("load function {target}"));
    }
    // FUNCTION-scoped directives need a loaded function and are consumed at flow
    // time, so they precede the first `decompile`.
    for form in forms(crate::assertdecl::Slot::Function) {
        lines.push(form.line);
    }
    for ka in kasserts {
        lines.push(format!("kassert {ka}"));
    }
    lines.push("decompile".into());
    // SYMBOL-scoped directives name a LOCAL, which does not exist until a
    // decompile has produced it (`rename v2 buf` before the first one answers
    // `No symbol named: v2`), so they run between two decompiles. The second
    // `decompile` is emitted ONLY when there is such a directive, so every other
    // invocation keeps its current cost.
    if crate::assertdecl::needs_second_pass(assertions, selected) {
        for form in forms(crate::assertdecl::Slot::Symbol) {
            lines.push(form.line);
        }
        lines.push("decompile".into());
    }
    let out_display = out_path.display().to_string();
    lines.push(format!("openfile write {}", console_path(&out_display)));
    lines.push("print C".into());
    if *raw {
        lines.push("print raw".into());
    }
    lines.push("closefile".into());
    if let Some(rp) = regions_path {
        let rp_display = rp.display().to_string();
        lines.push(format!("openfile write {}", console_path(&rp_display)));
        lines.push("region blocks".into());
        lines.push("region tree".into());
        lines.push("closefile".into());
    }
    lines.push("quit".into());
    lines.join("\n") + "\n"
}
