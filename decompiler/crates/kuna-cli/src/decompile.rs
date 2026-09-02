//! `kuna decompile` — the Rust port of `kuna/decompile.py`.
//!
//! Drives `decomp_dbg` with the same console command language the datatests use
//! (`load file` / `read symbols` / `option` / `load function` | `load addr` /
//! `decompile` / `openfile write <tmp>` / `print C` / `closefile`), capturing the
//! decompiled C through the bulk-output redirect so interactive prompts never
//! pollute it, and prints the captured C — byte-identical to the Python tool.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

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
/// reached through `--decomp-dbg`, which would not understand a quote.
fn console_path(path: &str) -> String {
    if !path.contains(|c: char| c.is_ascii_whitespace() || c == '"') {
        return path.to_string();
    }
    format!("\"{}\"", path.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Build the stdin script fed to `decomp_dbg` — port of `_build_script`.
fn build_script(
    binary: &str,
    target: &str,
    by_address: bool,
    bfd_target: Option<&str>,
    raw: bool,
    out_path: &Path,
    options: &[(String, String)],
    kasserts: &[String],
    regions_path: Option<&Path>,
) -> String {
    let mut lines: Vec<String> = Vec::new();
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
    // (kuna, Ghidra-gap / DIV-15) Build the Listing when neither the selected
    // mode nor the caller names `listing`, matching `decompile-all`. The
    // Listing drives the no-return analysis that prevents a function ending in
    // a no-returning call from absorbing its neighbour. The default `auto`
    // policy names Listing off through `fast` for binaries at least 2 MiB.
    if !options.iter().any(|(name, _)| name == "listing") {
        lines.push("option listing on".into());
    }
    for (name, value) in options {
        lines.push(format!("option {name} {value}"));
    }
    lines.push("read symbols".into());
    if by_address {
        let addr = if target.starts_with("0x") || target.starts_with("0X") {
            target.to_string()
        } else {
            format!("0x{target}")
        };
        lines.push(format!("load addr {addr}"));
    } else {
        lines.push(format!("load function {target}"));
    }
    for ka in kasserts {
        lines.push(format!("kassert {ka}"));
    }
    lines.push("decompile".into());
    lines.push(format!("openfile write {}", console_path(&out_path.display().to_string())));
    lines.push("print C".into());
    if raw {
        lines.push("print raw".into());
    }
    lines.push("closefile".into());
    if let Some(rp) = regions_path {
        lines.push(format!("openfile write {}", console_path(&rp.display().to_string())));
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
    if !by_address && (out.contains("Unknown function name:") || out.contains("Bad namespace:")) {
        return Some(format!(
            "no function {target:?} in {binary}; for a stripped binary pass an address with --addr"
        ));
    }
    None
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
}

/// Run the decompile and return its [`DecompileOutcome`].
fn decompile(args: &DecompileArgs) -> Result<DecompileOutcome, String> {
    let binary = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let binary = binary.to_string_lossy().to_string();

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

    let result = (|| {
        let script = build_script(
            &binary,
            &args.target,
            by_address,
            args.bfd_target.as_deref(),
            args.raw,
            &out_path,
            &args.options,
            &args.kasserts,
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
            .map_err(|e| format!("failed to run decomp_dbg: {e}"))?;

        let stdout_text = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr_text = String::from_utf8_lossy(&output.stderr).into_owned();
        let combined = format!("{stdout_text}\n{stderr_text}");
        if let Some(msg) = check_errors(&combined, &args.target, &binary, by_address) {
            return Err(msg);
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
                });
            }
            return Err(format!(
                "no C output for {:?} in {}; decompiler said:\n{}",
                args.target,
                binary,
                combined.trim().chars().take(2000).collect::<String>()
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
        Ok(DecompileOutcome { c: c_text, regions: regions_text, failure })
    })();

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

/// Entry point for `kuna decompile`.
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
            let status = match out.failure {
                Some(msg) => {
                    eprintln!("error: {msg}");
                    1
                }
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

#[cfg(test)]
mod tests {
    use super::{
        arch_failure_reason, build_script, check_errors, console_path, find_pipeline_failure,
        read_symbols_failure,
    };
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
            &[],
            &[],
            None,
        );
        assert!(
            script.contains("load file x86:LE:64:default \"/home/u/test dir/a.out\"\n"),
            "got:\n{script}"
        );
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
            &[],
            &[],
            None,
        );
        assert!(script.contains("load file /home/u/a.out\n"), "got:\n{script}");
        assert!(script.contains("openfile write /tmp/kuna.c\n"), "got:\n{script}");
        assert!(!script.contains('"'), "no quoting where none is needed:\n{script}");
    }
}
