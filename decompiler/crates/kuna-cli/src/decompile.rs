//! `kuna decompile` — the Rust port of `kuna/decompile.py`, plus its
//! machine-readable mode.
//!
//! The TEXT surface drives `decomp_dbg` with the same console command language
//! the datatests use (`load file` / `read symbols` / `option` /
//! `load function` | `load addr` / `decompile` / `openfile write <tmp>` /
//! `print C` / `closefile`), capturing the decompiled C through the bulk-output
//! redirect so interactive prompts never pollute it, and prints the captured C —
//! byte-identical to the Python tool.
//!
//! `--json` answers the same question in `decompile-all`'s record shape: a
//! `functions` array holding the one function that was asked for, with its
//! address, size, variables, line mappings and per-record `error`. It is not a
//! second dialect and not a second implementation — [`run_json`] routes the
//! selection through `decompile_all`'s own parser, decompile loop and
//! serializer, so an agent parses ONE shape whichever command it reached for.
//! That path loads the binary in-process rather than spawning `decomp_dbg`,
//! which is why the `decomp_dbg`-only flags are refused rather than ignored.

use std::borrow::Cow;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use kuna_console::engine::EntrySelector;

use crate::decompile_all::{self, Args as AllArgs, DriverDefaults};
use crate::paths;

/// Options parsed for a `decompile` invocation.
pub struct DecompileArgs {
    pub binary: String,
    pub target: String,
    pub by_address: bool,
    pub bfd_target: Option<String>,
    pub raw: bool,
    pub regions: bool,
    pub options: Vec<(String, String)>,
    pub kasserts: Vec<String>,
    /// `--define-function <start[-end][=name] | @file>` (repeatable): the
    /// caller-declared function boundaries, lowered to `function bounds` lines.
    pub func_decls: Vec<crate::funcdecl::FuncDecl>,
    /// `--assert <directive> | @FILE` (repeatable): the caller-supplied
    /// assertions, lowered to console lines at the slots `build_script`
    /// documents (`crate::assertdecl`).
    pub assertions: Vec<kuna_console::assertions::Directive>,
    /// `--assert-strict`: a rejected directive makes the run exit non-zero.
    pub assert_strict: bool,
    pub decomp_dbg: Option<String>,
    pub sleighpath: Option<String>,
    /// Mach-O fat / universal slice override (`--slice <arch>`, e.g. `x86_64` /
    /// `arm64`). Picks which arch slice of a universal binary is loaded; absent
    /// ⇒ the deterministic default (x86-64 → arm64 → first present). Exported as
    /// `KUNA_MACHO_SLICE` onto the subprocess (read at the dispatch slice peel).
    pub slice: Option<String>,
}

/// Whether an `--option` value selects the "on" state (the `on_or_off` token set
/// the console accepts), used to decide whether `macho-arm64e` exports its
/// load-time env gate.
fn is_on(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "on" | "true" | "1" | "yes")
}

fn last_option_value<'a>(options: &'a [(String, String)], name: &str) -> Option<&'a str> {
    options
        .iter()
        .rev()
        .find(|(option_name, _)| option_name == name)
        .map(|(_, value)| value.as_str())
}

/// A 0x-prefixed token auto-selects address mode (a bare hex-looking token is a
/// function name; use `--addr` for bare numeric addresses) — `_looks_like_addr`.
pub(crate) fn looks_like_addr(target: &str) -> bool {
    target.starts_with("0x") || target.starts_with("0X")
}

/// Quote a path for the console script when — and only when — it needs it.
///
/// The console reads a filename with `CommandStream::read_filename`, which
/// tokenizes on whitespace unless the argument opens with `"`. An unquoted path
/// containing a space therefore splits into two arguments: `load file` loads the
/// wrong file, and `openfile write` truncates a file at the split point.
///
/// Quoting is conditional so that every path that works today keeps producing a
/// byte-identical script — the corpus transcripts, and any older `decomp_dbg`
/// reached through `--decomp-dbg`, which would not understand a quote. The
/// [`Cow`] says so in the type: borrowed (and unallocated) for every path that
/// needs no quoting, which is nearly all of them.
///
/// The scan is byte-wise because the console's own splitter is
/// (`CommandStream::is_ws` is the ASCII set): the producer tests exactly the
/// bytes the consumer would split on.
fn console_path(path: &str) -> Cow<'_, str> {
    if !path.as_bytes().iter().any(|b| b.is_ascii_whitespace() || *b == b'"') {
        return Cow::Borrowed(path);
    }
    Cow::Owned(format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\"")))
}

/// A newline is the one whitespace character quoting cannot rescue: the script
/// is fed to `decomp_dbg` as lines, so an embedded `\n` ends the command no
/// matter how it is quoted, and the console answers with a load failure that
/// reads like a defect in the binary.
///
/// Legal on unix and vanishingly rare. Diagnose it here rather than emit a
/// script that cannot mean what it says.
fn reject_unquotable(what: &str, path: &str) -> Result<(), String> {
    match path.find(['\n', '\r']) {
        None => Ok(()),
        Some(_) => Err(format!(
            "{what} contains a newline, which the decomp_dbg console script \
             (one command per line) cannot carry: {path:?}"
        )),
    }
}

/// Build the stdin script fed to `decomp_dbg` — port of `_build_script`.
fn build_script(
    binary: &str,
    target: &str,
    by_address: bool,
    bfd_target: Option<&str>,
    raw: bool,
    out_path: &Path,
    injected: &[(&'static str, &'static str)],
    options: &[(String, String)],
    kasserts: &[String],
    func_decls: &[crate::funcdecl::FuncDecl],
    assertions: &[kuna_console::assertions::Directive],
    regions_path: Option<&Path>,
) -> String {
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
    match bfd_target {
        Some(t) if !t.is_empty() => lines.push(format!("load file {t} {image}")),
        _ => lines.push(format!("load file {image}")),
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
    if raw {
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

/// The console prompt `decomp_dbg` writes before echoing each command; a
/// transcript line can therefore carry it as a prefix.
const CONSOLE_PROMPT: &str = "[decomp]>";

/// The console's exception→prefix grammar
/// (`decompiler/crates/kuna-console/src/ifacedecomp.rs (execute)`), which that
/// module documents as byte-faithful and load-bearing: a command that raised
/// prints exactly one of these and the session continues.
const CONSOLE_DIAGNOSTICS: [&str; 7] = [
    "Execution error: ",
    "Command parsing error: ",
    "Low-level ERROR: ",
    "Parse ERROR: ",
    "Function ERROR: ",
    "Decoding ERROR: ",
    "ERROR: ",
];

/// Strip the prompt a transcript line may carry, and the surrounding whitespace.
fn console_text(trimmed: &str) -> &str {
    trimmed.strip_prefix(CONSOLE_PROMPT).unwrap_or(trimmed).trim()
}

/// The real reason `load file` failed, recovered from the transcript.
///
/// `IfcLoadFile`'s error arm (`decompiler/crates/kuna-console/src/ifacedecomp.rs`)
/// writes `{e.explain()}` and then `Could not create architecture`, both to
/// stdout, so the escaped `LowlevelError` is the line before the trigger. `None`
/// means nothing but the command echo precedes it — no reason was printed, and
/// the caller keeps the generic wording.
fn arch_failure_reason(out: &str) -> Option<String> {
    let mut prev: Option<&str> = None;
    for raw in out.lines() {
        let trimmed = raw.trim();
        let line = console_text(trimmed);
        if line == "Could not create architecture" {
            let reason = prev?;
            if reason.starts_with(CONSOLE_PROMPT) {
                return None;
            }
            return Some(reason.to_string());
        }
        if !line.is_empty() {
            prev = Some(trimmed);
        }
    }
    None
}

/// The reason the analysis commit failed, recovered from the transcript.
///
/// `IfcReadSymbols` maps a failed `commit_pending_analysis` to an
/// `IfaceExecutionError` and the console prints it and **keeps the session
/// alive**, so `print C` still renders C — built from a program whose debug
/// facts were only partially applied and cannot be re-committed. The diagnostic
/// is attributed to the command whose echo precedes it, so a later command's
/// error is never misreported as this one.
fn read_symbols_failure(out: &str) -> Option<String> {
    let mut in_read_symbols = false;
    for raw in out.lines() {
        let trimmed = raw.trim();
        let line = console_text(trimmed);
        if line.is_empty() {
            continue;
        }
        if trimmed.starts_with(CONSOLE_PROMPT) {
            in_read_symbols = line.split_whitespace().eq(["read", "symbols"]);
            continue;
        }
        if !in_read_symbols {
            continue;
        }
        for prefix in CONSOLE_DIAGNOSTICS {
            if let Some(reason) = line.strip_prefix(prefix) {
                let reason = reason.trim();
                if !reason.is_empty() {
                    return Some(reason.to_string());
                }
            }
        }
    }
    None
}

/// The reason the selection command failed, recovered from the transcript.
///
/// `load function` / `load addr` resolve through the shared selector model
/// (`kuna_console::engine::ConsoleProgram::resolve_entry`), whose ambiguity
/// report spans several lines — the diagnostic line plus one line per candidate
/// — so everything the console printed for that command is the reason.
fn selection_failure(out: &str) -> Option<String> {
    let mut in_load = false;
    let mut reason: Option<String> = None;
    for raw in out.lines() {
        let trimmed = raw.trim();
        let line = console_text(trimmed);
        if trimmed.starts_with(CONSOLE_PROMPT) {
            if reason.is_some() {
                break;
            }
            let mut words = line.split_whitespace();
            in_load = words.next() == Some("load")
                && matches!(words.next(), Some("function") | Some("addr"));
            continue;
        }
        if !in_load || line.is_empty() {
            continue;
        }
        match &mut reason {
            Some(collected) => {
                collected.push('\n');
                collected.push_str(raw.trim_end());
            }
            None => {
                for prefix in CONSOLE_DIAGNOSTICS {
                    if let Some(text) = line.strip_prefix(prefix) {
                        let text = text.trim();
                        if !text.is_empty() {
                            reason = Some(text.to_string());
                        }
                        break;
                    }
                }
            }
        }
    }
    reason
}

/// Inspect the combined stdout+stderr for the recognized fatal-error strings —
/// port of `_check_errors`.  Returns an error message if one is found.
///
/// The architecture arm must stay ahead of the analysis-commit arm: a failed
/// `load file` leaves no image, so every later command — `read symbols`
/// included — answers `No load image present`, which is a consequence, not the
/// reason.
fn check_errors(out: &str, target: &str, binary: &str, by_address: bool) -> Option<String> {
    if out.contains("Could not discover root of Ghidra installation") {
        return Some(
            "decomp_dbg could not find SLEIGH specs; pass --sleighpath or set SLEIGHHOME".into(),
        );
    }
    if out.contains("Could not create architecture") {
        // Byte-identical to the in-process surfaces'
        // `decompile_all.rs (load_program)` wording, so all four commands answer
        // one binary-load failure the same way; the generic string survives only
        // where the console printed no reason at all.
        return Some(match arch_failure_reason(out) {
            Some(reason) => format!("could not build an architecture for {binary}: {reason}"),
            None => format!(
                "could not build an architecture for {binary} (unsupported/!recognized binary)"
            ),
        });
    }
    if let Some(reason) = read_symbols_failure(out) {
        return Some(format!("read symbols (analysis commit) failed: {reason}"));
    }
    if !by_address && is_unknown_function(out) {
        return Some(format!(
            "no function {target:?} in {binary}; for a stripped binary pass an address with --addr"
        ));
    }
    // An ambiguous or unmapped selector is answered by the selector model, whose
    // report names every candidate. Return it verbatim: the transcript dump the
    // caller falls back to is capped at its FIRST 2000 characters, which in the
    // default mode is all option chatter, so the answer would be cut off. The
    // unmapped-entry probe stays ahead of it — an external is not a bad selector.
    if !is_unmapped_entry(out) {
        if let Some(reason) = selection_failure(out) {
            return Some(reason);
        }
    }
    None
}

/// Whether the console answered "there is no function by that name here".
///
/// The three spellings are the selector model's own
/// (`ConsoleProgram::resolve_entry` → `EntryLookupError`) plus the engine's
/// `Unknown function name:`. It is a MISS, not an ambiguity and not a load
/// failure, which is what makes it safe to answer with a second, wider attempt.
fn is_unknown_function(out: &str) -> bool {
    out.contains("Unknown function name:")
        || out.contains("no function matches")
        || out.contains("Bad namespace:")
}

/// Whether the console transcript says the selected entry has no mapped bytes
/// (`LoadImage::load_fill`'s "Unable to load N bytes at <addr>", raised the
/// moment the flow-follower asks for the first instruction).
///
/// That is the signature of an **external**: an entry that carries an address
/// for call naming but whose definition is in another module. It is not
/// reachable for a real function — a mapped entry that fails mid-pipeline
/// surfaces as the `Skipping <name>` notice below instead.
fn is_unmapped_entry(out: &str) -> bool {
    out.contains("Unable to load ") && out.contains(" bytes at ")
}

/// The console's per-function abort notice: `IfcDecompile` catches a
/// recoverable pipeline abort, prints `Skipping <name>: <reason>` and keeps
/// going (so `print C` still renders a shell).  Without this the CLI would
/// report success for a function that produced nothing.
///
/// Returns `(function name, reason)` for the first such notice.
fn find_pipeline_failure(out: &str) -> Option<(String, String)> {
    for line in out.lines() {
        // The console prompt is written before the command echo, so an output
        // line can carry it as a prefix.
        let line = line.trim_start().strip_prefix("[decomp]>").unwrap_or(line).trim_start();
        let Some(rest) = line.strip_prefix("Skipping ") else {
            continue;
        };
        let rest = rest.trim();
        if rest.is_empty() {
            continue;
        }
        return Some(match rest.split_once(": ") {
            Some((name, reason)) => (name.to_string(), reason.trim().to_string()),
            None => (rest.to_string(), String::new()),
        });
    }
    None
}

/// Recover one [`Outcome`](kuna_console::assertions::Outcome) per `--assert`
/// directive from the `decomp_dbg` transcript.
///
/// The console's exception grammar is byte-faithful and load-bearing
/// (`ifacedecomp::execute`, and [`CONSOLE_DIAGNOSTICS`] already parses it): a
/// command that raised prints exactly one diagnostic prefix and the session
/// continues.  So a directive is `applied` unless a diagnostic follows the echo
/// of the console line it lowered to — which is the only signal this surface
/// has, since the C comes back through a file redirect and not the transcript.
///
/// A directive whose echo is absent entirely (the script never got that far —
/// the load failed) is reported as rejected rather than silently applied.
fn assertion_outcomes(
    out: &str,
    directives: &[kuna_console::assertions::Directive],
    selected: Option<&str>,
) -> Vec<kuna_console::assertions::Outcome> {
    use kuna_console::assertions::Outcome;
    directives
        .iter()
        .map(|directive| {
            let (kind, phase, subphase) = directive.body.coords();
            let detail = match crate::assertdecl::console_form(directive, selected) {
                Ok(form) => command_failure(out, &form.line),
                // Nothing was emitted for it: the directive names a function this
                // run did not decompile, and the reason is the report row.
                Err(detail) => Some(detail),
            };
            Outcome {
                directive: directive.raw.clone(),
                kind,
                phase,
                subphase,
                status: if detail.is_none() { "applied" } else { "rejected" },
                detail,
            }
        })
        .collect()
}

/// The diagnostic the console printed for the command `line`, or `None` when it
/// ran clean.  `Some` for a command whose echo never appears at all — a script
/// that stopped early did not apply it.
fn command_failure(out: &str, line: &str) -> Option<String> {
    let mut in_command = false;
    let mut seen = false;
    for raw in out.lines() {
        let trimmed = raw.trim();
        let text = console_text(trimmed);
        if trimmed.starts_with(CONSOLE_PROMPT) {
            in_command = text == line;
            seen |= in_command;
            continue;
        }
        if !in_command || text.is_empty() {
            continue;
        }
        for prefix in CONSOLE_DIAGNOSTICS {
            if let Some(reason) = text.strip_prefix(prefix) {
                let reason = reason.trim();
                if !reason.is_empty() {
                    return Some(reason.to_string());
                }
            }
        }
    }
    if seen {
        None
    } else {
        Some("the console script did not reach this directive".into())
    }
}

/// A unique temp path under the system temp dir (no external dep; mirrors
/// `tempfile.NamedTemporaryFile(delete=False)`'s role — a private scratch file we
/// delete in the `finally`).
fn temp_path(prefix: &str, suffix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!("{prefix}{pid}_{nanos}{suffix}"));
    dir
}

/// One `kuna decompile` run: the rendered C, the optional `--regions` dump, and
/// — when the pipeline aborted for the requested function — the failure report
/// (`kuna decompile` exits non-zero on it; see `docs/cli.md`).
struct DecompileOutcome {
    c: String,
    regions: Option<String>,
    failure: Option<String>,
    /// One row per `--assert` directive, recovered from the transcript.
    assertions: Vec<kuna_console::assertions::Outcome>,
}

/// Run the decompile and return its [`DecompileOutcome`].
fn decompile(args: &DecompileArgs) -> Result<DecompileOutcome, String> {
    let binary = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let binary = binary.to_string_lossy().to_string();
    reject_unquotable("binary path", &binary)?;

    let bin_path = if let Some(d) = &args.decomp_dbg {
        PathBuf::from(d)
    } else {
        paths::decomp_dbg()
    };
    if !bin_path.exists() {
        return Err(format!(
            "decomp_dbg not built at {} -- run `make binaries` \
             (or `cargo build --release -p kuna-console`)",
            bin_path.display()
        ));
    }

    let specs = match &args.sleighpath {
        Some(s) => PathBuf::from(s),
        None => paths::specs_dir(),
    };

    let mut by_address = args.by_address;
    if !by_address && looks_like_addr(&args.target) {
        by_address = true;
    }

    let out_path = temp_path("kuna_c_", ".c");
    let regions_path = if args.regions {
        Some(temp_path("kuna_regions_", ".txt"))
    } else {
        None
    };
    // These are ours, but their directory is the caller's `TMPDIR`.
    reject_unquotable("temp directory", &out_path.display().to_string())?;
    if let Some(rp) = &regions_path {
        reject_unquotable("temp directory", &rp.display().to_string())?;
    }

    // (kuna, RE-need `analysis-generated-function-name`) The driver defaults, split
    // in two. `base` is what this surface injects on the FIRST attempt — the DIV-15
    // Listing and the DIV-120 instruction-budget policy, neither of which touches the
    // entry set.
    // `discovery` is the DIV-20/DIV-68 non-x86-64 bundle the IN-PROCESS drivers also
    // inject (`decompile_all::load_program`), and it is held back for a second attempt
    // rather than injected up front.
    //
    // Held back because the bundle is not free: it changes the ENTRY SET, and the
    // entries it adds are not all real. On i386 and PPC64 the prologue matcher seeds a
    // start a few bytes inside a function it already knew (PPC64 ELFv2's local entry
    // point, 8 bytes past the global one), and `funcboundflow` then truncates the outer
    // function's flow at that seed — so injecting it unconditionally would turn a
    // correct `kuna decompile __do_global_ctors_aux` body into an empty husk on every
    // non-x86-64 image. Measured, not assumed: a before/after sweep over every function
    // of all 33 non-x86-64 fixtures found 8 such truncations and no other difference.
    //
    // So the wider inventory is paid for only where it is the ANSWER: a by-name
    // selection that missed. That is exactly the reported gap — `kuna functions` prints
    // a discovery-generated name that `kuna decompile` then refuses — and nothing that
    // already resolved changes at all.
    let full = decompile_all::driver_default_options(&binary, true, true, &args.options);
    let (base, discovery): (Vec<_>, Vec<_>) = full
        .into_iter()
        .partition(|(name, _)| matches!(*name, "listing" | "errortoomanyinstructions"));

    // The selected function's name, or `None` for an `--addr` run — what decides
    // whether a `<func>::`-qualified directive binds to the selection.
    let selected: Option<&str> = if by_address { None } else { Some(args.target.as_str()) };
    let attempt = |injected: &[(&'static str, &'static str)]| {
        let script = build_script(
            &binary,
            &args.target,
            by_address,
            args.bfd_target.as_deref(),
            args.raw,
            &out_path,
            injected,
            &args.options,
            &args.kasserts,
            &args.func_decls,
            &args.assertions,
            regions_path.as_deref(),
        );

        // (kuna) The `relocobjects` option gates the ET_REL loader, which runs at
        // `load file` — before the `option` lines in the script are processed.
        // Bridge it to the subprocess env var the loader reads at load time so the
        // off-switch (and the before/after demo) work for the single-shot CLI.
        let reloc_env: Option<&'static str> =
            last_option_value(&args.options, "relocobjects").map(|value| {
                if matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "0" | "off" | "false" | "no"
                ) {
                    "0"
                } else {
                    "1"
                }
            });

        let mut cmd = Command::new(&bin_path);
        cmd.arg("-s").arg(&specs).env("SLEIGHHOME", &specs);
        if let Some(v) = reloc_env {
            cmd.env(kuna_decomp::options::RELOC_OBJECTS_ENV, v);
        }
        if let Some(slice) = args.slice.as_deref().filter(|s| !s.trim().is_empty()) {
            // Mach-O fat / universal slice override: read at the dispatch peel.
            cmd.env("KUNA_MACHO_SLICE", slice);
        }
        // (PR-8 §3.7) Mach-O arm64e Apple-Silicon spec selection is a LOAD-time
        // decision (the spec is chosen before any console `option` command runs),
        // so `--option macho-arm64e on` must reach the subprocess as an env gate,
        // not just a console `option` line. Export it when requested; the
        // `option macho-arm64e on` line still flows (so the option is recognized
        // and recorded), but the env var is what makes the spec selection live.
        if let Some(value) = last_option_value(&args.options, "macho-arm64e") {
            if is_on(value) {
                cmd.env("KUNA_MACHO_ARM64E", "1");
            } else {
                cmd.env_remove("KUNA_MACHO_ARM64E");
            }
        }
        // (kuna) Loader-tier `i386_pie_plt` gate: the PLT→name map is baked at
        // `load file`, *before* the `option` lines in the script run, so an
        // `--option i386_pie_plt off` must reach the loader via the env var
        // (`kuna_i386_pie_plt::I386_PIE_PLT_ENV`) set on the subprocess up front.
        // (The harmless `option i386_pie_plt …` line still runs for the catalog
        // confirmation; it just can't retro-resolve the already-loaded image.)
        if let Some(value) = last_option_value(&args.options, "i386_pie_plt") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env("KUNA_I386_PIE_PLT", if on { "on" } else { "off" });
        }
        // (kuna) Load-time `ifuncfpret` gate (default-off, opt-in): the IFUNC
        // stub naming runs at `load file`, so `--option ifuncfpret on` must reach
        // the loader via the env var on the subprocess up front.
        if let Some(value) = last_option_value(&args.options, "ifuncfpret") {
            let on = matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "on" | "1" | "true" | ""
            );
            cmd.env("KUNA_IFUNCFPRET", if on { "on" } else { "off" });
        }
        // (kuna, GH-289) Load-time `relocrebase` gate: the analyzer tier runs
        // inside `load file`, so an `--option relocrebase off` must reach the
        // subprocess as an env var set up front.
        if let Some(value) = last_option_value(&args.options, "relocrebase") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_relocrebase::RELOCREBASE_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna, DIV-84) Load-time `dynrelocs` gate: the dynamic relocations are
        // applied while the loader snapshots the image, so an `--option dynrelocs
        // off` must reach the subprocess as an env var set up front.
        if let Some(value) = last_option_value(&args.options, "dynrelocs") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_dynrelocs::DYNRELOCS_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna, DIV-117) Load-time `pdatachained` gate: the PE `.pdata` entry
        // oracle runs inside `load file`, so an `--option pdatachained off` must
        // reach the subprocess as an env var set up front.
        if let Some(value) = last_option_value(&args.options, "pdatachained") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_pdatachained::PDATACHAINED_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna, DIV-96) Load-time `msvcfpconst` gate: the decoded `__real@`
        // bytes are materialised while the loader lays the object out, so an
        // `--option msvcfpconst off` must reach the subprocess as an env var set
        // up front.
        if let Some(value) = last_option_value(&args.options, "msvcfpconst") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_msvcfpconst::MSVCFPCONST_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna) Load-time `symbolnamerepair` gate: the symbol table is installed
        // inside `load file`, so an `--option symbolnamerepair off` must reach the
        // subprocess as an env var set up front.
        if let Some(value) = last_option_value(&args.options, "symbolnamerepair") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_symbolnamerepair::SYMBOLNAMEREPAIR_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna, GH-340) Load-time `symbolnamechars` gate: symbol names are
        // minted inside `load file`, so the mode must reach the subprocess as an
        // env var set up front.
        if let Some(value) = last_option_value(&args.options, "symbolnamechars") {
            let mode = kuna_decomp::kuna_symbolnamechars::NameChars::parse(value).unwrap_or_default();
            cmd.env(
                kuna_decomp::kuna_symbolnamechars::SYMBOLNAMECHARS_ENV,
                mode.as_str(),
            );
        }
        // (kuna) Load-time `symbolnamebound` gate, same seam: the Scopes are
        // nested while the symbol table is installed inside `load file`, so the
        // ceiling has to be on the subprocess before it starts. Valued, so the
        // token is forwarded verbatim (an unparseable one falls back to the
        // default rather than failing the load).
        if let Some(value) = last_option_value(&args.options, "symbolnamebound") {
            cmd.env(kuna_decomp::kuna_symbolnamebound::SYMBOLNAMEBOUND_ENV, value.trim());
        }
        // (kuna) Load-time `typedepth` gate: the DWARF type mapper runs inside
        // `load file`, so an `--option typedepth off` must reach it via the env
        // var (`kuna_typedepth::TYPEDEPTH_ENV`) set on the subprocess up front.
        if let Some(value) = last_option_value(&args.options, "typedepth") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(kuna_decomp::kuna_typedepth::TYPEDEPTH_ENV, if on { "on" } else { "off" });
        }
        // (kuna) Load-time `dwarfstructs` gate: the aggregate layout is installed
        // on the interned type inside `load file`, so an `--option dwarfstructs
        // off` must reach the subprocess through the env var too.
        if let Some(value) = last_option_value(&args.options, "dwarfstructs") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV,
                if on { "on" } else { "off" },
            );
        }
        // (kuna) Load-time `dwarfvariants` gate: the variant overlay is installed
        // on the interned type inside `load file`, same as `dwarfstructs` above.
        if let Some(value) = last_option_value(&args.options, "dwarfvariants") {
            let on = !matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "off" | "0" | "false"
            );
            cmd.env(
                kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV,
                if on { "on" } else { "off" },
            );
        }
        let output = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                if let Some(mut sin) = child.stdin.take() {
                    let _ = sin.write_all(script.as_bytes());
                }
                child.wait_with_output()
            })
            .map_err(|e| (format!("failed to run decomp_dbg: {e}"), false))?;

        let stdout_text = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr_text = String::from_utf8_lossy(&output.stderr).into_owned();
        let combined = format!("{stdout_text}\n{stderr_text}");
        if let Some(msg) = check_errors(&combined, &args.target, &binary, by_address) {
            return Err((msg, !by_address && is_unknown_function(&combined)));
        }

        let mut c_text = String::new();
        if let Ok(mut fh) = std::fs::File::open(&out_path) {
            let _ = fh.read_to_string(&mut c_text);
        }
        let c_text = trim_newlines(&c_text);
        if c_text.trim().is_empty() {
            // An EXTERNAL, not a failure: the selected entry has no mapped bytes
            // because its definition lives in another module (a relocatable
            // object's undefined symbol, a PE import slot). Those carry an
            // address only so a call to one renders by name. Say so, rather than
            // dumping a console transcript whose "Unable to load N bytes" reads
            // like a decompiler defect. The whole-binary surfaces answer the same
            // way through `kuna_console::project::decompile_targets`, which asks
            // the engine directly (`ConsoleProgram::entry_bytes_mapped`); this
            // path drives `decomp_dbg` as a subprocess and so reads its report.
            if is_unmapped_entry(&combined) {
                return Ok(DecompileOutcome {
                    c: format!(
                        "// {}: external symbol -- no code at this address in this module",
                        args.target
                    ),
                    regions: None,
                    failure: None,
                    assertions: assertion_outcomes(&stdout_text, &args.assertions, selected),
                });
            }
            return Err((
                format!(
                    "no C output for {:?} in {}; decompiler said:\n{}",
                    args.target,
                    binary,
                    combined.trim().chars().take(2000).collect::<String>()
                ),
                false,
            ));
        }
        // The pipeline aborted for this function: the console kept the session
        // alive and `print C` rendered the un-decompiled shell above, so the C
        // is non-empty and only this notice distinguishes the failure from a
        // genuinely empty function.  Report it; `run` exits non-zero.
        let failure = find_pipeline_failure(&stdout_text).map(|(func, reason)| {
            let mut msg = format!("decompilation failed for {func} in {binary}: {reason}");
            let note = stderr_text.trim();
            if !note.is_empty() {
                msg.push_str("\nnote: decomp_dbg stderr:\n");
                msg.push_str(&note.chars().take(2000).collect::<String>());
            }
            msg
        });

        let mut regions_text = None;
        if let Some(rp) = &regions_path {
            let mut buf = String::new();
            if let Ok(mut fh) = std::fs::File::open(rp) {
                let _ = fh.read_to_string(&mut buf);
            }
            regions_text = Some(trim_newlines(&buf));
        }
        Ok(DecompileOutcome {
            c: c_text,
            regions: regions_text,
            failure,
            assertions: assertion_outcomes(&stdout_text, &args.assertions, selected),
        })
    };

    let mut result = attempt(&base);
    if !discovery.is_empty() && matches!(&result, Err((_, true))) {
        let widened: Vec<(&'static str, &'static str)> =
            base.iter().chain(discovery.iter()).copied().collect();
        result = attempt(&widened);
    }
    let result = result.map_err(|(message, _)| message);

    let _ = std::fs::remove_file(&out_path);
    if let Some(rp) = &regions_path {
        let _ = std::fs::remove_file(rp);
    }
    result
}

/// Python `str.strip("\n")`: trim leading/trailing newline characters only.
fn trim_newlines(s: &str) -> String {
    s.trim_matches('\n').to_string()
}

/// The text surface of `kuna decompile`.
///
/// Exit codes: `0` on success, `1` on a run-level error (no such function, no
/// architecture, no C at all) **and on a per-function pipeline abort** — the
/// recovered shell still goes to stdout (its comment names the reason), the
/// reason goes to stderr.  `docs/cli.md` documents the contract.
pub fn run(args: &DecompileArgs) -> i32 {
    match decompile(args) {
        Ok(out) => {
            let text = if args.regions {
                format!(
                    "{}\n\n// ==== kuna regions (S7) ====\n{}\n",
                    out.c,
                    out.regions.unwrap_or_default()
                )
            } else {
                format!("{}\n", out.c)
            };
            // The pipeline verdict is reported and returned whether or not stdout
            // survived (DIV-45): a closed reader is not evidence the decompile
            // worked.  Emitting first keeps the stdout-then-stderr order.
            let written = crate::output::emit(&text);
            // A rejected assertion is reported and the run continues;
            // `--assert-strict` makes it the verdict.
            let rejected = decompile_all::report_rejected_assertions(&out.assertions);
            let status = match out.failure {
                Some(msg) => {
                    eprintln!("error: {msg}");
                    1
                }
                None if args.assert_strict && rejected => 1,
                None => 0,
            };
            match written {
                Ok(()) => status,
                Err(err) => crate::output::status_after(err, status),
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}


// --- command entry point -----------------------------------------------------

/// Entry point for `kuna decompile`: parse the command line, then render either
/// the text surface ([`run`], a `decomp_dbg` subprocess) or, with `--json`, the
/// `decompile-all` record shape for the one selected function ([`run_json`]).
pub fn main(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut target: Option<String> = None;
    let mut addr = false;
    let mut json = false;
    let mut bfd_target: Option<String> = None;
    let mut raw = false;
    let mut regions = false;
    let mut options: Vec<(String, String)> = Vec::new();
    // The `--option` / `--language` tokens in argv order, replayed verbatim into
    // the `--json` argument list: option precedence and the `--language auto`
    // policy are position-sensitive, so both surfaces resolve them in one parser
    // rather than in two that can disagree.
    let mut forwarded: Vec<String> = Vec::new();
    let mut saw_language = false;
    let mut mode: Option<String> = None;
    let mut kasserts: Vec<String> = Vec::new();
    let mut func_decls: Vec<crate::funcdecl::FuncDecl> = Vec::new();
    let mut assertions: Vec<kuna_console::assertions::Directive> = Vec::new();
    let mut assert_strict = false;
    let mut decomp_dbg: Option<String> = None;
    let mut engine: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut slice: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--addr" => addr = true,
            "--json" => json = true,
            "--raw" => raw = true,
            "--regions" => regions = true,
            "--slice" => slice = take_value(argv, &mut i, "--slice"),
            "--target" => {
                bfd_target = take_value(argv, &mut i, "--target");
            }
            "--option" => {
                // nargs=2
                if i + 2 >= argv.len() {
                    eprintln!("error: --option requires NAME VALUE");
                    return 2;
                }
                if let Err(msg) = crate::optname::check(&argv[i + 1]) {
                    eprintln!("error: {msg}");
                    return 2;
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                forwarded.extend(argv[i..i + 3].iter().cloned());
                i += 2;
            }
            // (kuna outlang) `--language` is the first-class surface for the
            // output language; it lowers to the upstream `setlanguage` option, so
            // it reaches every downstream consumer (the console script here, the
            // in-process option applier in decompile-all) with no new plumbing.
            // Pushed in argv order, so a later `--option setlanguage` still wins.
            "--language" => match take_value(argv, &mut i, "--language") {
                Some(value) => {
                    match decompile_all::parse_language_flag(&value) {
                        Ok(Some(lang)) => options.push(("setlanguage".into(), lang.into())),
                        Ok(None) => {}
                        Err(msg) => {
                            eprintln!("error: {msg}");
                            return 2;
                        }
                    }
                    forwarded.push("--language".into());
                    forwarded.push(value);
                    saw_language = true;
                }
                None => return 2,
            },
            "--mode" => match take_value(argv, &mut i, "--mode") {
                Some(value) => mode = Some(value),
                None => return 2,
            },
            "--kassert" => {
                if let Some(v) = take_value(argv, &mut i, "--kassert") {
                    kasserts.push(v);
                }
            }
            "--define-function" => match take_value(argv, &mut i, "--define-function") {
                Some(value) => match crate::funcdecl::parse_flag(&value) {
                    Ok(decls) => {
                        func_decls.extend(decls);
                        forwarded.push("--define-function".into());
                        forwarded.push(value);
                    }
                    Err(msg) => {
                        eprintln!("error: {msg}");
                        return 2;
                    }
                },
                None => return 2,
            },
            "--assert" => match take_value(argv, &mut i, "--assert") {
                Some(value) => match crate::assertdecl::parse_flag(&value) {
                    Ok(parsed) => {
                        assertions.extend(parsed);
                        forwarded.push("--assert".into());
                        forwarded.push(value);
                    }
                    Err(msg) => {
                        eprintln!("error: {msg}");
                        return 2;
                    }
                },
                None => return 2,
            },
            "--assert-strict" => {
                assert_strict = true;
                forwarded.push("--assert-strict".into());
            }
            "--decomp-dbg" => decomp_dbg = take_value(argv, &mut i, "--decomp-dbg"),
            "--engine" => engine = take_value(argv, &mut i, "--engine"),
            "--sleighpath" => sleighpath = take_value(argv, &mut i, "--sleighpath"),
            "-h" | "--help" => {
                usage();
                return 0;
            }
            "--timeout" => {
                // Accepted for compatibility; the in-process child has no timeout
                // wall (the Python timeout guarded a hung subprocess — out of scope
                // here, but we must consume the value so it isn't read as a positional).
                let _ = take_value(argv, &mut i, "--timeout");
            }
            s if s.starts_with("--") => {
                eprintln!("error: unknown option {s}");
                return 2;
            }
            _ => {
                if binary.is_none() {
                    binary = Some(a.to_string());
                } else if target.is_none() {
                    target = Some(a.to_string());
                } else {
                    eprintln!("error: unexpected argument {a:?}");
                    return 2;
                }
            }
        }
        i += 1;
    }

    let (binary, target) = match (binary, target) {
        (Some(b), Some(t)) => (b, t),
        _ => {
            eprintln!("error: decompile requires <binary> and <func>");
            return 2;
        }
    };
    // Honor `--engine cpp|rust` like the Python tools: set `KUNA_ENGINE`.  In the
    // Rust-only world `rust` is already the default and `cpp` would fail to
    // resolve, but the flag is accepted (and exported) for compatibility with
    // existing invocations / the pipeline.
    if let Some(e) = &engine {
        std::env::set_var("KUNA_ENGINE", e);
    }
    addr |= looks_like_addr(&target);

    if json {
        // Refused, not ignored: each of these is a `decomp_dbg` transcript the
        // in-process JSON path never produces, and silently dropping half a
        // request is the failure mode `--json` exists to end.
        for (flag, requested) in [
            ("--raw", raw),
            ("--regions", regions),
            ("--kassert", !kasserts.is_empty()),
            ("--decomp-dbg", decomp_dbg.is_some()),
        ] {
            if requested {
                eprintln!("error: {flag} is not supported with --json");
                return 2;
            }
        }
        return run_json(&JsonRequest {
            binary: &binary,
            target: &target,
            by_address: addr,
            forwarded: &forwarded,
            mode: mode.as_deref(),
            bfd_target: bfd_target.as_deref(),
            slice: slice.as_deref(),
            sleighpath: sleighpath.as_deref(),
        });
    }

    // (kuna outlang, DIV-80) The auto policy -- follow the binary when the caller
    // named no language. See `decompile_all::detected_output_language`.
    if !saw_language && !options.iter().any(|(n, _)| n == "setlanguage") {
        if let Some(lang) = decompile_all::detected_output_language(&binary) {
            options.push(("setlanguage".into(), lang.into()));
        }
    }

    let explicit_fast_funcdisc = options.iter().any(|(name, _)| name == "fast_funcdisc");
    // Omitted mode is the size-driven `auto` policy. Preset overrides are
    // prepended so explicit `--option` pairs remain last-write-wins in the
    // generated console script.
    match decompile_all::mode_options_for_binary(mode.as_deref(), &binary, options) {
        Ok(merged) => options = merged,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    }
    if addr && !explicit_fast_funcdisc {
        options.push(("fast_funcdisc".into(), "off".into()));
    }

    run(&DecompileArgs {
        binary,
        target,
        by_address: addr,
        bfd_target,
        raw,
        regions,
        options,
        kasserts,
        func_decls,
        assertions,
        assert_strict,
        decomp_dbg,
        sleighpath,
        slice,
    })
}

/// Consume the value following a flag at `argv[i]`, advancing `i` past it.
fn take_value(argv: &[String], i: &mut usize, flag: &str) -> Option<String> {
    if *i + 1 < argv.len() {
        *i += 1;
        Some(argv[*i].clone())
    } else {
        eprintln!("error: {flag} requires a value");
        None
    }
}

fn usage() {
    eprintln!(
        "usage: kuna decompile <binary> <name|0xaddr> [--addr] [--json] [--raw] [--regions] \\\n\
         \x20                     [--language auto|c|rust] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                     [--option N V].. [--kassert ARGS].. \\\n\
         \x20                     [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                     [--assert DIRECTIVE|@FILE].. [--assert-strict] \\\n\
         \x20                     [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile ONE function.  The target is a name, or an address with --addr\n\
         (a `0x`-prefixed target implies it).  --json emits the decompile-all record\n\
         for that one function ({{binary,count,functions:[{{name,address,code,variables,..}}]}});\n\
         without it, the C text alone.\n\
         \n\
         --option NAME VALUE (repeatable) flips one phase-model decision for this run;\n\
         `kuna catalog` lists them and `kuna docs options` explains the tiers.\n\
         --mode applies an option preset (`kuna modes`); omitted, `auto` picks one by\n\
         input size.  Explicit --option values win over the preset.\n\
         --language selects the output language; omitted, it follows the binary.\n\
         \n\
         --define-function <start[-end][=name] | @file> (repeatable) declares where a\n\
         function starts and ends: start names an entry discovery missed, the\n\
         exclusive end bounds its flow so it stops swallowing its neighbours.\n\
         \n\
         --assert <directive | @file> (repeatable) states a fact the engine could not\n\
         derive.  The vocabulary is function, typedef, prototype, data, param, return,\n\
         name, type, comment, flow, readonly, volatile -- for instance\n\
         `prototype login int login(char *user,char *pw)`, `type v2 char[16]`,\n\
         `name v2 credbuf`, `readonly 0x404028+8`, `flow 0x1405 return`.\n\
         @FILE holds one per line with `#` comments, which is what makes an override\n\
         durable across invocations.  A directive the engine declines is reported and\n\
         the run still succeeds; --assert-strict makes it exit 1 instead.\n\
         \n\
         Whole-binary runs are `kuna decompile-all` / `kuna functions`."
    );
}

// --- the --json surface ------------------------------------------------------

/// What `--json` needs from the parsed command line: the selection, plus every
/// flag `decompile-all` also understands.
struct JsonRequest<'a> {
    binary: &'a str,
    target: &'a str,
    by_address: bool,
    /// The `--option` / `--language` tokens, in argv order.
    forwarded: &'a [String],
    mode: Option<&'a str>,
    bfd_target: Option<&'a str>,
    slice: Option<&'a str>,
    sleighpath: Option<&'a str>,
}

/// `kuna decompile --json`: `decompile-all`'s in-process load narrowed to the one
/// selected function, emitted through `decompile-all`'s own serializer.
///
/// The selection is expressed as a `decompile-all` argument list and re-parsed by
/// [`decompile_all::parse_args`] on purpose: the mode presets, the `--language`
/// auto policy, option precedence, the address-selection `fast_funcdisc` default
/// and the per-function watchdog budget then all resolve exactly once, in the
/// parser that owns them, instead of being re-derived here where they would
/// drift.
fn run_json(req: &JsonRequest) -> i32 {
    let mut argv: Vec<String> = vec![req.binary.to_string(), "--json".into()];
    if req.by_address {
        argv.push("--addr".into());
        argv.push(if looks_like_addr(req.target) {
            req.target.to_string()
        } else {
            format!("0x{}", req.target)
        });
    } else {
        argv.push("--functions".into());
        argv.push(req.target.to_string());
    }
    argv.extend(req.forwarded.iter().cloned());
    for (flag, value) in [
        ("--mode", req.mode),
        ("--target", req.bfd_target),
        ("--slice", req.slice),
        ("--sleighpath", req.sleighpath),
    ] {
        if let Some(value) = value {
            argv.push(flag.into());
            argv.push(value.to_string());
        }
    }

    let args = match decompile_all::parse_args(&argv, "decompile-all") {
        Ok(args) => args,
        Err(e) => {
            eprintln!("error: {e}");
            return 2;
        }
    };
    // stdout carries a document either way: a caller that asked for JSON gets
    // JSON, with the reason in the run-level `error`, rather than an empty stream
    // it has to reconcile against a prose stderr line — the missing error channel
    // is half of why this flag exists. The verdict follows the document, matching
    // `run`'s stdout-then-stderr ordering.
    let (text, failure) = match decompile_json(&args, req.target) {
        Ok(answered) => answered,
        Err(message) => (
            decompile_all::render_result_json(
                &args.binary,
                &[],
                &args.options,
                Some(&message),
                &[],
            ),
            Some(message),
        ),
    };
    let status = crate::output::emit_with_status(&text, i32::from(failure.is_some()));
    if let Some(message) = failure {
        eprintln!("error: {message}");
    }
    status
}

/// Load once, resolve the one selected function, decompile it, and render the
/// document.
///
/// Returns the JSON text plus the failure the caller reports on stderr and exits
/// `1` on. A function whose pipeline aborted is a failed run here — `decompile`
/// was asked about exactly that function — so `--json` cannot turn the text
/// surface's non-zero verdict into a success; the reason is in the document too,
/// as the record's `error` and as the run-level one.
fn decompile_json(args: &AllArgs, target: &str) -> Result<(String, Option<String>), String> {
    let mut prog = decompile_all::load_program(args, DriverDefaults::Decompile)?;
    let targets = decompile_all::resolve_targets(&prog, args)?;
    if targets.is_empty() {
        // Two different answers, and the caller acts on the difference: a binary
        // that yielded NOTHING is the whole-binary discovery failure (which names
        // a packer when it can see one), while a binary that yielded functions
        // but not this one is the text surface's "no function <name>".
        return Err(if prog.function_entries_executable().is_empty() {
            decompile_all::zero_discovery_error(&args.binary)
                .unwrap_or_else(|| format!("no function {target:?} in {}", args.binary))
        } else {
            format!(
                "no function {target:?} in {}; for a stripped binary pass an address with --addr",
                args.binary
            )
        });
    }

    let funcs = decompile_all::decompile_entries(&mut prog, args, targets);
    let assertions = prog.assertion_outcomes();
    let mut failure = funcs.iter().find_map(|f| {
        f.error.as_ref().map(|reason| {
            format!("decompilation failed for {} in {}: {reason}", f.name, args.binary)
        })
    });
    // A rejected assertion is reported and the run continues; `--assert-strict`
    // makes it the run's verdict (see `report_rejected_assertions`).
    let rejected = decompile_all::report_rejected_assertions(&assertions);
    let text = decompile_all::render_result_json(
        &args.binary,
        &funcs,
        &args.options,
        failure.as_deref(),
        &assertions,
    );
    if failure.is_none() && args.assert_strict && rejected {
        failure = Some(format!("--assert directive rejected in {}", args.binary));
    }
    Ok((text, failure))
}

#[cfg(test)]
mod tests {
    use super::{
        arch_failure_reason, build_script, check_errors, console_path, decompile_all,
        find_pipeline_failure, is_unknown_function, read_symbols_failure, reject_unquotable,
    };
    use std::borrow::Cow;

    /// What `decompile_all::driver_default_options` yields for the FIRST attempt
    /// on every architecture: the DIV-120 instruction-budget policy and the
    /// DIV-15 Listing, neither of which touches the entry set.
    const LISTING: &[(&str, &str)] =
        &[("errortoomanyinstructions", "off"), ("listing", "on")];

    /// The DIV-20/DIV-68 non-x86-64 bundle, held back for the retry.
    const WIDENED: &[(&str, &str)] = &[
        ("errortoomanyinstructions", "off"),
        ("listing", "on"),
        ("funcstart_patterns", "on"),
        ("aif", "on"),
    ];
    use std::path::Path;

    /// Recorded `decomp_dbg` transcript: the empty-scope load failure DIV-88's
    /// `symbolnamerepair` guards (`--option symbolnamerepair off`).
    const EMPTY_SCOPE: &str = "\
[decomp]> load file /x/hostile_scope_x86_64
Non-global scope has empty name
Could not create architecture
[decomp]> option listing on
Execution error: No load image present
[decomp]> read symbols
Execution error: No load image present
[decomp]> quit
";

    /// Recorded transcript: `-s /nonexistent`, i.e. a SPECS problem the generic
    /// wording misdiagnoses as a problem with the binary.
    const MISSING_SLA: &str = "\
[decomp]> load file /x/a.out
No sleigh specification for x86:LE:64:default
Could not create architecture
[decomp]> quit
";

    /// Recorded transcript: 200 bytes of junk, i.e. neither an object format nor
    /// a `<binaryimage>` document.
    const JUNK_MAGIC: &str = "\
[decomp]> load file /x/junk.bin
syntax error
Could not create architecture
[decomp]> quit
";

    /// Recorded transcript: an ELF whose `st_size` overflows the type factory's
    /// domain.  The commit fails, the console keeps going, and `print C` renders
    /// C with every debug fact stripped (GH-339's silent half).
    const COMMIT_FAILED: &str = "\
[decomp]> load file /x/sz.elf
/x/sz.elf successfully loaded: x86:LE:64:default:gcc
[decomp]> option listing on
Listing/xref disassembly tier turned on
[decomp]> read symbols
Execution error: g_a symbol created with zero size type
[decomp]> load function main
[decomp]> decompile
Clearing old decompilation
Decompiling main
Decompilation complete
[decomp]> quit
";

    #[test]
    fn recovers_the_real_load_failure_reason() {
        assert_eq!(
            arch_failure_reason(EMPTY_SCOPE).as_deref(),
            Some("Non-global scope has empty name")
        );
        assert_eq!(
            arch_failure_reason(MISSING_SLA).as_deref(),
            Some("No sleigh specification for x86:LE:64:default")
        );
        assert_eq!(arch_failure_reason(JUNK_MAGIC).as_deref(), Some("syntax error"));
    }

    /// The recovered reason is reported in the in-process surfaces' wording
    /// (`decompile_all.rs (load_program)`), so all four commands agree.
    #[test]
    fn load_failure_matches_the_in_process_wording() {
        assert_eq!(
            check_errors(EMPTY_SCOPE, "main", "/x/hostile_scope_x86_64", false).as_deref(),
            Some(
                "could not build an architecture for /x/hostile_scope_x86_64: \
                 Non-global scope has empty name"
            )
        );
    }

    /// No reason printed: the generic wording is the fallback, not the default.
    #[test]
    fn a_bare_trigger_keeps_the_generic_wording() {
        let out = "Could not create architecture\n";
        assert_eq!(arch_failure_reason(out), None);
        assert_eq!(
            check_errors(out, "main", "/x/a.out", false).as_deref(),
            Some("could not build an architecture for /x/a.out (unsupported/!recognized binary)")
        );
    }

    /// The command echo is not a reason: a prompt-prefixed previous line means
    /// the console printed nothing between `load file` and the trigger.
    #[test]
    fn the_command_echo_is_not_mistaken_for_a_reason() {
        let out = "[decomp]> load file /x/a.out\nCould not create architecture\n[decomp]> quit\n";
        assert_eq!(arch_failure_reason(out), None);
        assert!(
            check_errors(out, "main", "/x/a.out", false)
                .expect("still an error")
                .ends_with("(unsupported/!recognized binary)")
        );
    }

    /// GH-339's silent half: the analysis commit failed, so the C that follows
    /// is degraded.  Reported in `decompile_all.rs (load_program)`'s wording.
    #[test]
    fn reports_the_analysis_commit_failure() {
        assert_eq!(
            read_symbols_failure(COMMIT_FAILED).as_deref(),
            Some("g_a symbol created with zero size type")
        );
        assert_eq!(
            check_errors(COMMIT_FAILED, "main", "/x/sz.elf", false).as_deref(),
            Some("read symbols (analysis commit) failed: g_a symbol created with zero size type")
        );
    }

    /// A diagnostic belonging to another command is not reported as the commit
    /// failure, and a failed `load file` is diagnosed by its own arm — not by
    /// the `No load image present` every later command then echoes.
    #[test]
    fn a_diagnostic_is_attributed_to_its_own_command() {
        let other = "\
[decomp]> read symbols
[decomp]> load function nosuch
Execution error: Unknown function name: nosuch
[decomp]> quit
";
        assert_eq!(read_symbols_failure(other), None);
        assert!(
            check_errors(EMPTY_SCOPE, "main", "/x/hostile_scope_x86_64", false)
                .expect("still an error")
                .starts_with("could not build an architecture"),
            "the load failure wins over the No-load-image consequence"
        );
    }

    /// A healthy transcript is untouched by both recoveries.
    #[test]
    fn a_clean_transcript_reports_nothing() {
        let out = "\
[decomp]> load file /x/a.out
/x/a.out successfully loaded: x86:LE:64:default:gcc
[decomp]> read symbols
[decomp]> decompile
Decompilation complete
[decomp]> quit
";
        assert_eq!(read_symbols_failure(out), None);
        assert_eq!(arch_failure_reason(out), None);
        assert_eq!(check_errors(out, "main", "/x/a.out", false), None);
    }

    /// The real console transcript shape (`decomp_dbg` echoes the prompt, then
    /// the command's output lines).
    #[test]
    fn finds_the_console_abort_notice() {
        let out = "[decomp]> decompile\nClearing old decompilation\nDecompiling sub_3994\n\
                   Skipping sub_3994: decompile pipeline reached an un-ported seam (LOSS-131): \
                   called `Option::unwrap()` on a `None` value\n[decomp]> print C\n";
        let (func, reason) = find_pipeline_failure(out).expect("the notice is recognized");
        assert_eq!(func, "sub_3994");
        assert!(reason.contains("LOSS-131"), "{reason}");
        assert!(reason.contains("Option::unwrap()"), "the real panic text survives: {reason}");
    }

    /// A prompt sharing the line with the notice is still recognized.
    #[test]
    fn finds_a_prompt_prefixed_notice() {
        let out = "[decomp]> Skipping main: boom\n";
        assert_eq!(
            find_pipeline_failure(out),
            Some(("main".to_string(), "boom".to_string()))
        );
    }

    /// A clean run reports no failure — the exit code stays 0.
    #[test]
    fn clean_transcript_has_no_failure() {
        let out = "[decomp]> decompile\nDecompiling main\nDecompilation complete\n";
        assert_eq!(find_pipeline_failure(out), None);
    }

    /// The C body mentioning the word must not be mistaken for the notice (only
    /// a line that *starts* with it counts, and the C never reaches this text).
    #[test]
    fn body_text_is_not_a_failure() {
        let out = "[decomp]> print C\n  /* Skipping is fine here */\n";
        assert_eq!(find_pipeline_failure(out), None);
    }

    /// A path without whitespace is emitted exactly as before — the corpus
    /// transcripts and any older `decomp_dbg` behind `--decomp-dbg` depend on
    /// the script staying byte-identical for every path that works today.
    #[test]
    fn console_path_leaves_ordinary_paths_alone() {
        assert_eq!(console_path("/home/u/a.out"), "/home/u/a.out");
        assert_eq!(console_path("./a.out"), "./a.out");
        assert_eq!(console_path(r"C:\Users\u\a.out"), r"C:\Users\u\a.out");

        // Not merely equal — untouched. The borrow is the contract: a path that
        // needs no quoting is passed through, never rebuilt.
        assert!(matches!(console_path("/home/u/a.out"), Cow::Borrowed(_)));
        assert!(matches!(console_path("/home/u/test dir/a.out"), Cow::Owned(_)));
    }

    /// A path with a space is quoted, with `\` and `"` escaped so the console's
    /// `read_filename` recovers the original bytes.
    #[test]
    fn console_path_quotes_whitespace() {
        assert_eq!(console_path("/home/u/test dir/a.out"), "\"/home/u/test dir/a.out\"");
        assert_eq!(console_path("/a\tb/c.out"), "\"/a\tb/c.out\"");
        assert_eq!(
            console_path(r"C:\Users\John Doe\a.out"),
            r#""C:\\Users\\John Doe\\a.out""#
        );
        assert_eq!(console_path("/odd \"name\"/a.out"), r#""/odd \"name\"/a.out""#);
    }

    /// The declarations land AFTER `read symbols` and BEFORE the load: the commit
    /// is what discovery writes, and the load is what reads the declared extent.
    #[test]
    fn build_script_declares_boundaries_between_read_symbols_and_the_load() {
        let decls = crate::funcdecl::parse_flag("0x1400-0x1480=decrypt").expect("parses");
        let script = build_script(
            "/tmp/a.out",
            "0x1400",
            true,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &decls,
            &[],
            None,
        );
        let line = |needle: &str| {
            script
                .lines()
                .position(|l| l == needle)
                .unwrap_or_else(|| panic!("{needle:?} missing from:\n{script}"))
        };
        assert!(
            line("read symbols")
                < line("function bounds 0x1400 0x1480 as decrypt")
                    && line("function bounds 0x1400 0x1480 as decrypt") < line("load addr 0x1400"),
            "wrong order in:\n{script}"
        );
    }

    /// The assertion slots, which are forced rather than stylistic: a parsed
    /// prototype must precede the load, a `map param` needs the loaded function,
    /// and a `rename` of a LOCAL needs a function that has already been
    /// decompiled — before that the console answers `No symbol named: v2`, which
    /// is exactly the bug that makes `--kassert p9 naming-policy` inert today.
    #[test]
    fn build_script_puts_each_assertion_in_its_own_slot() {
        let directives: Vec<_> = [
            "prototype authenticate int4 authenticate(char *u)",
            "param 0 RDI char *u",
            "name v2 credbuf",
        ]
        .iter()
        .map(|spec| crate::assertdecl::parse_one(spec).expect("parses"))
        .collect();
        let script = build_script(
            "/tmp/a.out",
            "authenticate",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &directives,
            None,
        );
        let line = |needle: &str| {
            script
                .lines()
                .position(|l| l == needle)
                .unwrap_or_else(|| panic!("{needle:?} missing from:\n{script}"))
        };
        assert!(line("read symbols") < line("map prototype authenticate int4 authenticate(char *u);"));
        assert!(
            line("map prototype authenticate int4 authenticate(char *u);")
                < line("load function authenticate")
        );
        assert!(line("load function authenticate") < line("map param 0 %RDI char *u"));
        assert!(line("map param 0 %RDI char *u") < line("decompile"));
        assert!(line("decompile") < line("rename v2 credbuf"));
        // The symbol-scoped directive forces a SECOND decompile after it.
        assert_eq!(
            script.lines().filter(|l| *l == "decompile").count(),
            2,
            "a symbol-scoped directive needs a second pass:\n{script}"
        );
        assert!(line("rename v2 credbuf") < script.lines().count());
    }

    /// Without a symbol-scoped directive there is no second `decompile`, so an
    /// assertion an agent passes never doubles the cost of a run that does not
    /// need it.
    #[test]
    fn build_script_emits_one_decompile_without_a_symbol_scoped_assertion() {
        let directives = vec![crate::assertdecl::parse_one("data 0x601048 char *pw")
            .expect("parses")];
        let script = build_script(
            "/tmp/a.out",
            "main",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &directives,
            None,
        );
        assert_eq!(script.lines().filter(|l| *l == "decompile").count(), 1, "{script}");
        assert!(script.contains("map address 0x601048 char *pw"), "{script}");
    }

    /// The transcript reader is what gives the human surface a report at all: a
    /// console diagnostic under a directive's echo is that directive's rejection,
    /// and a clean echo is an application.
    #[test]
    fn assertion_outcomes_read_the_console_diagnostic_under_each_echo() {
        let directives: Vec<_> = ["name v2 credbuf", "type v9 char[4]"]
            .iter()
            .map(|spec| crate::assertdecl::parse_one(spec).expect("parses"))
            .collect();
        let transcript = "\
[decomp]> decompile
Decompiling authenticate
[decomp]> rename v2 credbuf
[decomp]> retype v9 char[4]
Execution error: No symbol named: v9
[decomp]> decompile
";
        let report = super::assertion_outcomes(transcript, &directives, Some("authenticate"));
        assert_eq!(report[0].status, "applied");
        assert_eq!(report[1].status, "rejected");
        assert_eq!(report[1].detail.as_deref(), Some("No symbol named: v9"));
    }

    /// A script that never reached a directive did not apply it, and saying so is
    /// the difference between a report and a guess.
    #[test]
    fn an_unreached_directive_is_rejected_not_assumed_applied() {
        let directives = vec![crate::assertdecl::parse_one("name v2 buf").expect("parses")];
        let report =
            super::assertion_outcomes("[decomp]> load file /tmp/a.out\n", &directives, Some("main"));
        assert_eq!(report[0].status, "rejected");
        assert!(report[0].detail.as_deref().unwrap_or_default().contains("did not reach"));
    }

    /// No declarations ⇒ the script is byte-identical to what it always was.
    #[test]
    fn build_script_without_declarations_emits_no_boundary_lines() {
        let script = build_script(
            "/tmp/a.out",
            "main",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[],
            None,
        );
        assert!(!script.contains("function bounds"), "got:\n{script}");
    }

    /// The whole script: a spaced binary path and a spaced output path must both
    /// reach the console as ONE argument each.  Unquoted, `load file` reads the
    /// tail as the filename and `openfile write` truncates a file at the split.
    #[test]
    fn build_script_quotes_spaced_paths() {
        let script = build_script(
            "/home/u/test dir/a.out",
            "main",
            false,
            None,
            false,
            Path::new("/tmp/out dir/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[],
            Some(Path::new("/tmp/out dir/kuna.txt")),
        );
        assert!(
            script.contains("load file \"/home/u/test dir/a.out\"\n"),
            "the image path must be one quoted argument, got:\n{script}"
        );
        assert!(
            script.contains("openfile write \"/tmp/out dir/kuna.c\"\n"),
            "the C output path must be one quoted argument, got:\n{script}"
        );
        assert!(
            script.contains("openfile write \"/tmp/out dir/kuna.txt\"\n"),
            "the regions path must be one quoted argument, got:\n{script}"
        );
    }

    /// An explicit `--target` still yields `load file <target> <path>` with the
    /// path quoted — the 3-token shape that silently dropped the path tail.
    #[test]
    fn build_script_quotes_the_path_after_a_bfd_target() {
        let script = build_script(
            "/home/u/test dir/a.out",
            "main",
            false,
            Some("x86:LE:64:default"),
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[],
            None,
        );
        assert!(
            script.contains("load file x86:LE:64:default \"/home/u/test dir/a.out\"\n"),
            "got:\n{script}"
        );
    }

    /// A checked-in fixture path, so the architecture classification behind the
    /// discovery bundle reads a real object rather than a guess.
    fn fixture(name: &str) -> String {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(name)
            .canonicalize()
            .expect("fixture")
            .to_str()
            .expect("utf-8 fixture path")
            .to_string()
    }

    /// The two attempts are one table split in two: what a non-x86-64 image
    /// holds back is exactly what the in-process drivers inject up front, so the
    /// widened retry reaches the inventory `kuna functions` reports and not some
    /// third configuration.
    #[test]
    fn the_retry_widens_to_the_in_process_drivers_bundle() {
        let arm = fixture("entrymain_arm");
        let full = decompile_all::driver_default_options(&arm, true, true, &[]);
        assert_eq!(full, WIDENED, "the non-x86-64 bundle");
        let (base, discovery): (Vec<_>, Vec<_>) = full
            .into_iter()
            .partition(|(name, _)| matches!(*name, "listing" | "errortoomanyinstructions"));
        assert_eq!(base, LISTING, "the first attempt");
        assert_eq!(discovery, &WIDENED[2..], "held back for the retry");
    }

    /// x86-64 has nothing to hold back, so there is no second attempt to make:
    /// the Listing is measured entry-neutral there and the gap-walk can
    /// over-produce, which is why the bundle is non-x86-64 only.
    #[test]
    fn there_is_no_retry_to_make_on_x86_64() {
        let full = decompile_all::driver_default_options(&fixture("fauxware"), true, true, &[]);
        assert_eq!(full, LISTING);
    }

    /// Naming an option skips its injection on this surface exactly as it does
    /// on the in-process ones, so `--option aif off` is not silently re-enabled
    /// by the retry.
    #[test]
    fn the_bundle_yields_to_a_named_option() {
        let options = vec![("aif".to_string(), "off".to_string())];
        let full = decompile_all::driver_default_options(&fixture("entrymain_arm"), true, true, &options);
        assert_eq!(
            full,
            &[("errortoomanyinstructions", "off"), ("listing", "on"), ("funcstart_patterns", "on")]
        );
    }

    /// The widened attempt emits its extra options where the console can still
    /// act on them: ahead of `read symbols`, which is where the analysis passes
    /// are committed.
    #[test]
    fn the_widened_script_puts_the_bundle_before_the_commit() {
        let script = build_script(
            &fixture("entrymain_arm"),
            "sub_410",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            WIDENED,
            &[],
            &[],
            &[],
            &[],
            None,
        );
        let (before, after) = script.split_once("read symbols").expect("read symbols");
        for line in ["option listing on", "option funcstart_patterns on", "option aif on"] {
            assert!(before.contains(&format!("{line}\n")), "missing {line}:\n{script}");
        }
        assert!(after.contains("load function sub_410"), "got:\n{script}");
    }

    /// The retry fires on a MISS and on nothing else: an ambiguous selector, a
    /// load failure or a pipeline abort all mean the name was understood, and
    /// widening the inventory would only change the answer for the worse.
    #[test]
    fn only_a_name_miss_is_retryable() {
        assert!(is_unknown_function("Execution error: no function matches \"sub_410\""));
        assert!(is_unknown_function("Unknown function name: nope"));
        assert!(is_unknown_function("Bad namespace: a::b"));
        assert!(!is_unknown_function("Execution error: selector \"main\" is ambiguous"));
        assert!(!is_unknown_function(EMPTY_SCOPE));
        assert!(!is_unknown_function(COMMIT_FAILED));
    }

    /// A script over ordinary paths is unchanged, quotes and all absent.
    #[test]
    fn build_script_is_unchanged_for_ordinary_paths() {
        let script = build_script(
            "/home/u/a.out",
            "main",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[],
            None,
        );
        assert!(script.contains("load file /home/u/a.out\n"), "got:\n{script}");
        assert!(script.contains("openfile write /tmp/kuna.c\n"), "got:\n{script}");
        assert!(!script.contains('"'), "no quoting where none is needed:\n{script}");
    }

    /// Quoting cannot rescue a newline — the transport is one command per line —
    /// so it is diagnosed instead of silently producing a broken script.
    #[test]
    fn a_newline_in_the_path_is_diagnosed_not_quoted() {
        assert!(reject_unquotable("binary path", "/home/u/a.out").is_ok());
        assert!(reject_unquotable("binary path", "/home/u/test dir/a.out").is_ok());

        let err =
            reject_unquotable("binary path", "/home/u/nl\ndir/a.out").expect_err("must be rejected");
        assert!(err.contains("binary path contains a newline"), "got: {err}");
        assert!(err.contains("one command per line"), "got: {err}");
        assert!(reject_unquotable("temp directory", "/tmp/cr\rdir/x.c").is_err());
    }

    /// The contract that matters is the round trip: whatever `console_path`
    /// emits, the console's own `read_filename` must read back as the original
    /// path — one argument, byte for byte.
    ///
    /// The producer (`kuna-cli`) and the consumer (`kuna-console`) are different
    /// crates, so nothing but this test holds their escaping rules together; an
    /// edit to either side that breaks the pairing fails here rather than in a
    /// user's spaced directory.
    #[test]
    fn console_path_round_trips_through_the_console_reader() {
        use kuna_console::interface::CommandStream;

        for original in [
            "/home/u/a.out",
            "/home/u/test dir/a.out",
            "/home/u/two  spaces/a.out",
            " /leading/space",
            "/trailing/space ",
            r"C:\Users\John Doe\a.out",
            r"C:\Users\u\a.out",
            "/odd \"name\"/a.out",
            r"/back\slash and space/a.out",
            "/tab\there/a.out",
        ] {
            let emitted = console_path(original);
            let mut s = CommandStream::new(&emitted);
            assert_eq!(
                s.read_filename(),
                original,
                "round trip failed for {original:?} (emitted {emitted:?})"
            );
        }
    }

    /// An IMAGE-scoped directive is emitted before `read symbols`, and a
    /// `readonly` one turns read-only propagation on ahead of the caller's own
    /// `--option`s so an explicit `--option readonly off` still wins.
    ///
    /// The order is load-bearing, not cosmetic: mapping a symbol folds the range
    /// property into its `SymbolEntry` and never consults the range again, so a
    /// `readonly` emitted after `read symbols` is silently inert over every
    /// address the loader named.
    #[test]
    fn a_range_directive_precedes_read_symbols_and_turns_readonly_on() {
        let script = build_script(
            "/tmp/a.out",
            "sample",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[("readonly".into(), "off".into())],
            &[],
            &[],
            &[
                crate::assertdecl::parse_one("readonly 0x404028+8").unwrap(),
                crate::assertdecl::parse_one("volatile 0x50000000+4").unwrap(),
            ],
            None,
        );
        let at = |needle: &str| {
            script
                .lines()
                .position(|l| l == needle)
                .unwrap_or_else(|| panic!("{needle:?} missing from:\n{script}"))
        };
        assert!(at("option readonly on") < at("option readonly off"), "{script}");
        assert!(at("readonly 0x404028 8") < at("read symbols"), "{script}");
        assert!(at("volatile 0x50000000 4") < at("read symbols"), "{script}");
        // No symbol-scoped directive ⇒ still exactly one `decompile`.
        assert_eq!(script.lines().filter(|l| *l == "decompile").count(), 1, "{script}");
    }

    /// With no range directive the script is untouched — no `option readonly`
    /// line appears from nowhere.
    #[test]
    fn no_range_directive_leaves_the_readonly_option_alone() {
        let script = build_script(
            "/tmp/a.out",
            "sample",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[crate::assertdecl::parse_one("name v2 buf").unwrap()],
            None,
        );
        assert!(!script.contains("option readonly"), "{script}");
    }

    /// The same round trip inside a whole `load file` line, which is how the
    /// console actually sees it: the command words are consumed first and the
    /// path must survive as the single remaining argument.
    #[test]
    fn a_spaced_path_is_one_argument_in_the_load_file_line() {
        use kuna_console::interface::CommandStream;

        let original = "/home/u/test dir/a.out";
        let script = build_script(
            original,
            "main",
            false,
            None,
            false,
            Path::new("/tmp/kuna.c"),
            LISTING,
            &[],
            &[],
            &[],
            &[],
            None,
        );
        let line = script.lines().next().expect("the script opens with load file");

        let mut s = CommandStream::new(line);
        assert_eq!(s.read_token(), "load");
        assert_eq!(s.read_token(), "file");
        let filename = s.read_filename();
        s.skip_ws();
        assert!(s.eof(), "the path must exhaust the line, not leave a second argument");
        assert_eq!(filename, original);
    }
}
