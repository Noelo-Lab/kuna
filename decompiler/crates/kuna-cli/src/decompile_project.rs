//! `kuna decompile-project` — whole-binary **project export**.
//!
//! ```text
//!   kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..]
//!                          [--addr 0xVMA].. [--max-fn-seconds N] [--stream]
//!                          [--mode auto|reliable|aggressive|fast] [--option N V]..
//!                          [--jobs N|auto] [--jobs-chunk N] [--jobs-full-load]
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
//! `--stream` writes the same folder INCREMENTALLY — the artifacts appear
//! before the run finishes, entry-point-first, with an `index.jsonl` per-function
//! feed and a `.streaming` status file — at the cost of a decompile-ordered
//! `.c` and an `.asm` whose variable comments move to their own section
//! ([`crate::project_stream`]).
//!
//! Individual failures remain records and exit 0 while at least one body was
//! produced. A non-empty selection that produced zero bodies exits nonzero only
//! after every artifact is complete (and a streamed export has removed its
//! `.streaming` marker), with the summary on stdout and a run-level diagnostic
//! on stderr. Load errors, an empty target set, and I/O errors also exit nonzero.
//!
//! The decompile loop and the four artifact builders live in the shared
//! decompile-project core (`kuna_console::project` — also reused by the
//! `kuna_wasm` front-end); this module keeps the CLI wrapper + orchestration.

use std::path::PathBuf;

use kuna_console::project::{
    build_asm, build_c, build_header, build_readme, collect_dat_addrs, decompile_targets,
    BatchOutcome,
};
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};

use crate::decompile_all::{
    decompile_targets_pooled, load_program, parse_args, resolve_targets, Args, DriverDefaults,
};

/// `kuna decompile-project` entry point.
pub fn run(argv: &[String]) -> i32 {
    // Wrapper parse: extract `-o/--output`, intercept help, reject the
    // decompile-all-only flags, and hand the remainder to the shared parser.
    let mut output: Option<String> = None;
    let mut stream = false;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--stream" => stream = true,
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
    // An unqualified `--assert` directive binds to "the function under
    // decompile", which a whole-binary streamed run would apply to every one of
    // them in turn.
    if stream && !args.assertions.is_empty() {
        eprintln!(
            "error: --assert and --stream are exclusive: a streamed export decompiles every \
             function in turn, so an unqualified directive would bind to all of them. Re-run \
             without --stream."
        );
        return 2;
    }
    let run = if stream {
        crate::project_stream::run(&args, output.as_deref())
    } else {
        decompile_project(&args, output.as_deref())
    };
    match run {
        Ok(completion) => emit_completion(completion),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

pub(crate) struct ProjectCompletion {
    pub(crate) summary: String,
    pub(crate) error: Option<String>,
}

pub(crate) fn emit_completion(completion: ProjectCompletion) -> i32 {
    let status = crate::output::emit_with_status(
        &completion.summary,
        i32::from(completion.error.is_some()),
    );
    if let Some(error) = completion.error {
        eprintln!("error: {error}");
    }
    status
}

fn usage() {
    eprintln!(
        "usage: kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..] \\\n\
         \x20                   [--addr 0xVMA].. [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                   [--jobs N|auto] [--jobs-chunk N] [--jobs-full-load] [--stream] \\\n\
         \x20                   [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                   [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \x20                   [--raw-image --target T --base VMA (--entry|--addr VMA)..]\n\
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
         --jobs N spreads the per-function decompile over N worker processes\n\
         (auto = this machine's parallelism, capped at 16; 1, the default, is\n\
         serial). The artifacts are identical to a --jobs 1 run without --stream;\n\
         progress goes to stderr, and peak memory is roughly N times one worker's RSS.\n\
         --stream writes the folder as the run goes instead of at the end: the entry\n\
         point and what it calls are written first, index.jsonl announces each\n\
         finished function and .streaming reports progress until the export\n\
         completes. The .c is then in decompile order. Not available with --assert.\n\
         Individual function failures are recorded in the artifacts. A mixed run\n\
         exits 0; a selected set that produced no body exits 1 only after all\n\
         artifacts are complete. Load errors / an empty target set / I/O errors\n\
         also exit nonzero."
    );
}

/// The whole flow: load once → decompile every target (with prototypes) →
/// build the four artifacts → write them all at the end.
fn decompile_project(args: &Args, output: Option<&str>) -> Result<ProjectCompletion, String> {
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

    let load_started = std::time::Instant::now();
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
    let load_seconds = load_started.elapsed().as_secs_f64();
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

    // `--jobs N`: the per-function loop fans out over worker processes, but the
    // parent's program stays loaded — the `.asm` section sweep and the README
    // metadata are whole-program artifacts only it can build. The `.h` type
    // block is the exception: `print_c_types` renders types the DECOMPILE
    // interns, so in a sharded run it has to come back from the workers.
    let (mut results, pooled_types) = if args.jobs > 1 {
        let inventory = prog.function_entries_canonical();
        let pooled = decompile_targets_pooled(
            args,
            &targets,
            &inventory,
            /* want_proto= */ true,
            /* want_provenance= */ false,
            /* want_types= */ true,
            load_seconds,
        )?;
        (pooled.results, pooled.types)
    } else {
        (
            decompile_targets(
                &mut prog,
                targets,
                /* no_vars= */ false,
                /* want_proto= */ true,
                /* want_provenance= */ false,
            ),
            None,
        )
    };
    // Every artifact is address-ordered (resolve_targets only guarantees that
    // for the no-filter default; --addr/--functions arrive in user order).
    results.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));

    // `print_c_types` AFTER the decompile loop: user-defined types are interned
    // into the factory as functions decompile.
    let prelude = print_c_recompile_prelude(prog.arch());
    let types = pooled_types.unwrap_or_else(|| print_c_types(prog.arch_mut()));

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
    let error = BatchOutcome::of(&results).all_failed_error(&args.binary);
    let files = sizes
        .iter()
        .map(|(n, s)| format!("{n} ({s} bytes)"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(ProjectCompletion {
        summary: format!(
            "wrote {}: {files}; functions: {ok} ok, {failed} failed\n",
            out_dir.display()
        ),
        error,
    })
}
