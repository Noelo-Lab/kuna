//! `kuna decompile-project` — whole-binary **project export**.
//!
//! ```text
//!   kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..]
//!                          [--addr 0xVMA].. [--max-fn-seconds N]
//!                          [--mode auto|reliable|aggressive|fast] [--option N V]..
//!                          [--isa auto|arm|thumb] [--slice ARCH] [--target T]
//!                          [--sleighpath D]
//! ```
//!
//! Loads + analyzes the binary once (the `decompile-all` in-process path, with
//! omitted `--mode` resolved from file size) and writes a **project folder** — default
//! `<binary-filename>.kuna/` next to the binary, `-o DIR` overrides — designed
//! so a human or LLM can study the binary and attempt recompilation:
//!
//! * `<name>.c`   — every selected executable function (`// Function: <name> @ <addr>`
//!   headers, exactly the `decompile-all` rendering), `#include "<name>.h"`.
//! * `<name>.h`   — include-guarded recompile prelude (core scalar +
//!   `undefined` typedefs), the user-defined type definitions
//!   (`print_c_types`), and one prototype per decompiled function
//!   (`print_c_prototype` — token-identical to the `.c` definition line).
//! * `<name>.asm` — full labeled linear disassembly of every CODE section:
//!   `<name>:` labels matching the `.c` function names, per-function
//!   `; arg:` / `; stack:` comments mapping decompiled variables to stack
//!   offsets, undecodable bytes as `db` lines, and a `; --- data ---` tail
//!   listing the named globals plus every `dat_<hex>` address the `.c`
//!   references, with raw bytes.
//! * `README.md`  — binary metadata (size, arch, entry point, sections,
//!   function counts) and the artifact/labeling conventions.
//!
//! Exit code 0 on success **even if individual functions failed** (matching
//! `decompile-all` — failures are recorded in the artifacts); nonzero on load
//! errors, an empty target set, or I/O errors.
//!
//! The decompile loop and the four artifact builders live in the shared
//! decompile-project core (`kuna_console::project` — also reused by the
//! `kuna_wasm` front-end); this module keeps the CLI wrapper + orchestration.

use std::path::PathBuf;

use kuna_console::project::{
    build_asm, build_c, build_header, build_readme, collect_dat_addrs, decompile_targets,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};

use crate::decompile_all::{load_program, parse_args, resolve_targets, Args, DriverDefaults};

/// `kuna decompile-project` entry point.
pub fn run(argv: &[String]) -> i32 {
    // Wrapper parse: extract `-o/--output`, intercept help, reject the
    // decompile-all-only flags, and hand the remainder to the shared parser.
    let mut output: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--output" => {
                if i + 1 >= argv.len() {
                    eprintln!("error: {} requires a value", argv[i]);
                    usage();
                    return 2;
                }
                output = Some(argv[i + 1].clone());
                i += 1;
            }
            "-h" | "--help" => {
                usage();
                return 0;
            }
            "--json" | "--no-vars" => {
                eprintln!("error: {} is not a decompile-project option", argv[i]);
                usage();
                return 2;
            }
            other => rest.push(other.to_string()),
        }
        i += 1;
    }
    let args = match parse_args(&rest, "decompile-project") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    match decompile_project(&args, output.as_deref()) {
        Ok(summary) => crate::output::emit_with_status(&summary, 0),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn usage() {
    eprintln!(
        "usage: kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..] \\\n\
         \x20                   [--addr 0xVMA].. [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                   [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                   [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile a whole binary in one in-process load and write a project folder\n\
         (default `<binary-filename>.kuna/` next to the binary; -o DIR overrides):\n\
         \x20 <name>.c    every selected executable function (#include \"<name>.h\")\n\
         \x20 <name>.h    recompile prelude + type definitions + prototypes\n\
         \x20 <name>.asm  labeled disassembly (function labels, stack-var comments,\n\
         \x20             dat_<hex> data labels with raw bytes)\n\
         \x20 README.md   binary metadata (size, arch, entry, sections, counts)\n\
         Unfiltered fast exports default to 10 seconds per function; other runs\n\
         default to 120. --max-fn-seconds overrides that policy (0 disables).\n\
         Individual function failures are recorded in the artifacts; the run still\n\
         exits 0 (load errors / an empty target set / I/O errors exit nonzero)."
    );
}

/// The whole flow: load once → decompile every target (with prototypes) →
/// build the four artifacts → write them all at the end.
fn decompile_project(args: &Args, output: Option<&str>) -> Result<String, String> {
    let binary_path = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let file_name = binary_path
        .file_name()
        .ok_or_else(|| format!("binary has no file name: {}", binary_path.display()))?
        .to_string_lossy()
        .into_owned();
    let out_dir: PathBuf = match output {
        Some(dir) => PathBuf::from(dir),
        None => binary_path
            .parent()
            .ok_or_else(|| format!("binary has no parent directory: {}", binary_path.display()))?
            .join(format!("{file_name}.kuna")),
    };

    let mut prog = load_program(args, DriverDefaults::Decompile)?;
    // Per-function watchdog — same driver policy as `decompile-all` (10 s for
    // an unfiltered fast export, 120 s otherwise, 0 disables): a non-converging
    // function becomes its own error record instead of hanging the export.
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }
    // (kuna outlang) The project export is C-shaped end to end: a `.c`/`.h`
    // split, a `#include`-bearing recompile prelude, and one C prototype per
    // function. Emitting a non-C body into that skeleton would produce artifacts
    // that are neither valid C nor a usable module, so this surface refuses
    // rather than half-honouring the request. `decompile` and `decompile-all`
    // carry the other languages.
    if prog.arch().print().get_name() != "c-language" {
        return Err(format!(
            "project export is C-only in this release (got {}); use `kuna decompile` or \
             `kuna decompile-all --json` for other output languages",
            prog.arch().print().get_name()
        ));
    }
    let targets = resolve_targets(&prog, args)?;
    if targets.is_empty() {
        return Err(format!("no functions selected/discovered in {}", args.binary));
    }

    let mut results =
        decompile_targets(
            &mut prog,
            targets,
            /* no_vars= */ false,
            /* want_proto= */ true,
            /* want_provenance= */ false,
        );
    // Every artifact is address-ordered (resolve_targets only guarantees that
    // for the no-filter default; --addr/--functions arrive in user order).
    results.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));

    // `print_c_types` AFTER the decompile loop: user-defined types are interned
    // into the factory as functions decompile.
    let prelude = print_c_recompile_prelude(prog.arch());
    let types = print_c_types(prog.arch_mut());

    let header = build_header(&file_name, &prelude, &types, &results);
    let c_file = build_c(&file_name, &results);
    let dat_addrs = collect_dat_addrs(&results);
    let asm = build_asm(&prog, &results, &dat_addrs, &file_name);
    // The CLI's `| Path |` row prints the canonicalized on-disk path (the wasm
    // front-end passes a virtual label instead), so output stays byte-identical.
    let readme =
        build_readme(&binary_path, &binary_path.display().to_string(), &file_name, &prog, &results);

    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("cannot create {}: {e}", out_dir.display()))?;
    let mut sizes: Vec<(String, usize)> = Vec::new();
    for (base, text) in [
        (format!("{file_name}.c"), &c_file),
        (format!("{file_name}.h"), &header),
        (format!("{file_name}.asm"), &asm),
        ("README.md".to_string(), &readme),
    ] {
        let path = out_dir.join(&base);
        std::fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        sizes.push((base, text.len()));
    }

    let ok = results.iter().filter(|r| r.error.is_none()).count();
    let failed = results.len() - ok;
    let files = sizes
        .iter()
        .map(|(n, s)| format!("{n} ({s} bytes)"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(format!(
        "wrote {}: {files}; functions: {ok} ok, {failed} failed\n",
        out_dir.display()
    ))
}
