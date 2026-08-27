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
    match bfd_target {
        Some(t) if !t.is_empty() => lines.push(format!("load file {t} {binary}")),
        _ => lines.push(format!("load file {binary}")),
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
    lines.push(format!("openfile write {}", out_path.display()));
    lines.push("print C".into());
    if raw {
        lines.push("print raw".into());
    }
    lines.push("closefile".into());
    if let Some(rp) = regions_path {
        lines.push(format!("openfile write {}", rp.display()));
        lines.push("region blocks".into());
        lines.push("region tree".into());
        lines.push("closefile".into());
    }
    lines.push("quit".into());
    lines.join("\n") + "\n"
}

/// Inspect the combined stdout+stderr for the recognized fatal-error strings —
/// port of `_check_errors`.  Returns an error message if one is found.
fn check_errors(out: &str, target: &str, binary: &str, by_address: bool) -> Option<String> {
    if out.contains("Could not discover root of Ghidra installation") {
        return Some(
            "decomp_dbg could not find SLEIGH specs; pass --sleighpath or set SLEIGHHOME".into(),
        );
    }
    if out.contains("Could not create architecture") {
        return Some(format!(
            "could not build an architecture for {binary} (unsupported/!recognized binary)"
        ));
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
    use super::find_pipeline_failure;

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
}
