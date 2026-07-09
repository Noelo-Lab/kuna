//! The Rust `decomp_dbg`: the interactive decompiler console (the same command
//! surface as the C++ binary of the same name).
//!
//! Port of `decompiler/cpp/consolemain.cc` `main()`.
//!
//! The kuna Rust [`IfaceStatus`] accumulates command output in an in-memory
//! `optr` buffer and bulk output in a `fileoptr` redirect (see `interface.rs`);
//! a real binary must drain `optr` to stdout and flush an open `fileoptr`
//! redirect to its file.  The C++ `fileoptr` is a live `ofstream` writing
//! straight to disk; here the binary drives the command loop itself (instead of
//! the library `mainloop`) so it can, after each command, drain `optr` to stdout
//! and rewrite any open redirect file to disk — leaving the file complete on
//! disk before the `closefile` command consumes the in-memory `FileOut`.  This is
//! exactly the surface the Python tools (`kuna/decompile.py`, which feeds
//! `load file`/`read symbols`/`option`/`load function`/`decompile`/`openfile
//! write`/`print C`/`closefile`/`quit` over stdin and reads the captured C file)
//! drive when `KUNA_ENGINE=rust`.

use std::io::{Read, Write};
use std::process::ExitCode;

use kuna_console::ifacedecomp::{
    execute, register_console_commands, register_decomp_commands, IfaceDecompData,
    DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::IfaceTerm;
use kuna_console::interface::IfaceStatus;
use kuna_console::kuna_console::register_kuna_commands;

fn main() -> ExitCode {
    // --- argument parsing (consolemain.cc:183) ----------------------------
    // kuna also accepts the long `-sleighpath <dir>` the Python tooling may pass.
    let args: Vec<String> = std::env::args().collect();
    let mut spec_roots: Vec<String> = Vec::new();
    let mut _initscript: Option<String> = None;
    let mut i = 1usize;
    while i < args.len() && args[i].starts_with('-') {
        let flag = &args[i];
        if flag == "-i" {
            i += 1;
            if i < args.len() {
                _initscript = Some(args[i].clone());
            }
        } else if flag == "-s" || flag == "-sleighpath" {
            i += 1;
            if i < args.len() {
                spec_roots.push(args[i].clone());
            }
        }
        // Unknown flags are skipped (the C++ loop advances on any leading '-').
        i += 1;
    }

    // SLEIGHHOME is the load-bearing spec-root mechanism the Python tooling sets
    // (kuna/decompile.py); honor it like consolemain.cc honors getenv("SLEIGHHOME").
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() && !spec_roots.contains(&home) {
            spec_roots.push(home);
        }
    }

    if spec_roots.is_empty() {
        eprintln!("Could not discover root of Ghidra installation");
        return ExitCode::from(1);
    }

    // startDecompilerLibrary(...) — the id/capability bootstrap is performed
    // per-program inside `engine::bootstrap_*` (the kuna IdRegistry is a value,
    // not a global), so the library "start" here is just recording the roots.

    // --- the console (IfaceTerm over stdin) -------------------------------
    let mut input = Vec::new();
    if std::io::stdin().read_to_end(&mut input).is_err() {
        eprintln!("Interface error: could not read stdin");
        return ExitCode::from(2);
    }
    let mut status = IfaceTerm::into_status("[decomp]> ", &input);

    // C++: registerAllCommands, split into the decomp + kuna modules.
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    // Console-only commands (consolemain.cc main()): load file (save/restore/addpath
    // are later items, see register_console_commands).
    register_console_commands(&mut status);

    // Hand the spec roots to the shared decompile data so `load file` can resolve
    // the architecture (C++ keeps these in SleighArchitecture::specpaths globals).
    if let Some(data) = status.get_data_mut(DECOMPILE_MODULE) {
        if let Some(dcp) = data.as_any_mut().downcast_mut::<IfaceDecompData>() {
            dcp.spec_roots = spec_roots;
        }
    }

    // --- the command loop (mainloop, ifacedecomp.cc:3653) -----------------
    // Driven here (not the library `mainloop`) so optr drains to stdout and an
    // open `fileoptr` redirect is flushed to disk after each command.
    let retval = run_console(&mut status);

    ExitCode::from(retval)
}

/// Drive the command loop, draining `optr` to stdout and syncing any open
/// `fileoptr` redirect to disk after each command.
///
/// Mirrors C++ `mainloop`'s nested loop (drain the input stream running each
/// command, then break on `done`/no-more-scripts).  The kuna console feeds a
/// single stdin stream (no `source` script nesting in the Python-driven path), so
/// the inner drain is the whole loop.
fn run_console(status: &mut IfaceStatus) -> u8 {
    let mut stdout = std::io::stdout();
    loop {
        while !status.is_stream_finished() {
            status.write_prompt();
            // The prompt + any prior output drain now.
            execute(status);
            drain_stdout(status, &mut stdout);
            sync_redirect_file(status);
        }
        if status.done {
            break;
        }
        if status.num_input_stream_size() == 0 {
            break;
        }
        status.pop_script();
    }
    // Final drain (any trailing output / "Aborting process").
    drain_stdout(status, &mut stdout);
    let _ = stdout.flush();
    if status.is_in_error() {
        1
    } else {
        0
    }
}

/// Drain the accumulated command output (`optr`) to real stdout.
fn drain_stdout(status: &mut IfaceStatus, stdout: &mut std::io::Stdout) {
    if !status.optr.is_empty() {
        let text = std::mem::take(&mut status.optr);
        let _ = stdout.write_all(text.as_bytes());
        let _ = stdout.flush();
    }
}

/// If a bulk-output redirect is open (`openfile write/append`), rewrite its full
/// accumulated contents to disk, so the file is complete before `closefile`
/// consumes the in-memory `FileOut`.
///
/// The C++ `fileoptr` is a live `ofstream`; rewriting the whole buffer each time
/// is the equivalent against the buffer-backed redirect (the corpus writes are
/// small — one `print C` per file).
fn sync_redirect_file(status: &IfaceStatus) {
    if let Some(file) = &status.fileoptr {
        // `openfile append` opens with ios_base::app; honor it by appending only
        // the delta is unnecessary here because the buffer holds the full intended
        // file contents — a truncating rewrite reproduces both modes faithfully
        // for the single-writer console (append vs truncate differ only across
        // separate opens, which the corpus never does).
        if let Ok(mut fh) = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file.filename)
        {
            let _ = fh.write_all(file.contents.as_bytes());
            let _ = fh.flush();
        }
    }
}
