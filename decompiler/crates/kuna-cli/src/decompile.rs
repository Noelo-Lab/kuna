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

/// A 0x-prefixed token auto-selects address mode (a bare hex-looking token is a
/// function name; use `--addr` for bare numeric addresses) — `_looks_like_addr`.
fn looks_like_addr(target: &str) -> bool {
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

/// Run the decompile and return either the C (regions=false) or `(c, regions)`.
fn decompile(args: &DecompileArgs) -> Result<(String, Option<String>), String> {
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
        let reloc_env: Option<&'static str> = args
            .options
            .iter()
            .rev()
            .find(|(n, _)| n == "relocobjects")
            .map(|(_, v)| {
                if matches!(v.trim(), "0" | "off" | "false" | "no" | "OFF") {
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
        if args
            .options
            .iter()
            .any(|(n, v)| n == "macho-arm64e" && is_on(v))
        {
            cmd.env("KUNA_MACHO_ARM64E", "1");
        }
        // (kuna) Loader-tier `i386_pie_plt` gate: the PLT→name map is baked at
        // `load file`, *before* the `option` lines in the script run, so an
        // `--option i386_pie_plt off` must reach the loader via the env var
        // (`kuna_i386_pie_plt::I386_PIE_PLT_ENV`) set on the subprocess up front.
        // (The harmless `option i386_pie_plt …` line still runs for the catalog
        // confirmation; it just can't retro-resolve the already-loaded image.)
        for (name, value) in &args.options {
            if name == "i386_pie_plt" {
                let on = !matches!(value.trim().to_ascii_lowercase().as_str(), "off" | "0" | "false");
                cmd.env("KUNA_I386_PIE_PLT", if on { "on" } else { "off" });
            }
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

        let combined = format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if let Some(msg) = check_errors(&combined, &args.target, &binary, by_address) {
            return Err(msg);
        }

        let mut c_text = String::new();
        if let Ok(mut fh) = std::fs::File::open(&out_path) {
            let _ = fh.read_to_string(&mut c_text);
        }
        let c_text = trim_newlines(&c_text);
        if c_text.trim().is_empty() {
            return Err(format!(
                "no C output for {:?} in {}; decompiler said:\n{}",
                args.target,
                binary,
                combined.trim().chars().take(2000).collect::<String>()
            ));
        }
        if regions_path.is_none() {
            return Ok((c_text, None));
        }
        let mut regions_text = String::new();
        if let Some(rp) = &regions_path {
            if let Ok(mut fh) = std::fs::File::open(rp) {
                let _ = fh.read_to_string(&mut regions_text);
            }
        }
        Ok((c_text, Some(trim_newlines(&regions_text))))
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
pub fn run(args: &DecompileArgs) -> i32 {
    match decompile(args) {
        Ok((c, regions)) => {
            if args.regions {
                println!("{c}");
                println!();
                println!("// ==== kuna regions (S7) ====");
                println!("{}", regions.unwrap_or_default());
            } else {
                println!("{c}");
            }
            0
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}
