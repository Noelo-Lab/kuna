//! `kuna specs` — a thin alias for the Rust SLEIGH compiler (`slacomp`).
//!
//! `kuna specs <slaspec>...`     compile the given `.slaspec` files.
//! `kuna specs -a <dir>`         compile every `.slaspec` under <dir> (slacomp's
//!                               recursive `-a` mode).
//! `kuna specs --diff`           print the note that the old C++ differential
//!                               (`kuna/slacomp.py`, which diffed against
//!                               `sleigh_opt`) is moot now the C++ tree is gone.
//!
//! The byte-for-byte oracle the Python `kuna.slacomp` used (`sleigh_opt`) no
//! longer exists in-tree; its result (148/148 content-identical) is recorded in
//! docs/rust-port/README.md and is subsumed by `kuna test` (the Rust-built specs decode
//! to 675/675).  So `--diff` is a documentation note, not a live comparison.

use std::io::Read;
use std::process::{Command, Stdio};

use crate::paths;

const DIFF_NOTE: &str = "\
kuna specs --diff: the C++ differential is moot.

The Python `kuna.slacomp` differential compiled each .slaspec with both the C++
`sleigh_opt` and the Rust `slacomp` and required byte-identical .sla content.
The C++ tree was removed (see docs/rust-port/README.md); there is no in-tree oracle to
diff against anymore.  The recorded result was 148/148 content-identical, and it
is subsumed by `kuna test`: rebuilding all specs with the Rust `slacomp` and
re-running the datatest corpus yields 675/675, proving the Rust-built specs
decode identically to the C++-built ones.  Use `kuna specs -a <dir>` to compile,
and `kuna test --datatests --baseline docs/baseline.json` as the end-to-end gate.";

pub fn run(args: &[String]) -> i32 {
    // Ahead of the passthrough: slacomp owns no help flag and answers `-h` with
    // `Unknown option` and exit 1, so the alias has to describe itself.
    if args.iter().any(|a| a == "-h" || a == "--help") {
        usage();
        return 0;
    }
    if args.iter().any(|a| a == "--diff") {
        return crate::output::emit_with_status(&format!("{DIFF_NOTE}\n"), 0);
    }
    let bin = paths::slacomp();
    if !bin.exists() {
        eprintln!(
            "slacomp not built at {} -- run `make binaries` \
             (or `cargo build --release -p kuna-slacomp`)",
            bin.display()
        );
        return 2;
    }
    // Pass the remaining args straight through to slacomp (it owns `-a <dir>` and
    // the bare `<slaspec>...` forms).
    //
    // slacomp's progress is on stdout and its diagnostics on stderr, and for the
    // unlocated warnings ("1 NOP constructors found") the `Compiling <spec>:` line
    // above them is the only thing naming the spec they came from.  Capturing both
    // streams would print all 1004 warnings of a 148-spec run ahead of all 149
    // progress lines, after 17s of silence.  So stderr stays inherited and stdout
    // is copied through as it arrives: the two interleave as they did when this
    // was a bare `status()` passthrough, and the write still goes through the
    // fallible boundary.
    let mut child = match Command::new(&bin).args(args).stdout(Stdio::piped()).spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to run slacomp: {e}");
            return 2;
        }
    };
    let mut pipe = child.stdout.take().expect("stdout was piped above");
    let mut buf = [0u8; 8192];
    let mut write_err = None;
    loop {
        match pipe.read(&mut buf) {
            Ok(0) => break,
            Ok(n) if write_err.is_none() => {
                if let Err(e) = crate::output::emit_bytes(&buf[..n]) {
                    write_err = Some(e);
                }
            }
            // Past a write failure the reads continue to EOF rather than stopping:
            // slacomp is writing .sla files, so it is left to finish instead of
            // being killed by the EPIPE that closing this end early would hand it.
            Ok(_) => {}
            Err(e) => {
                write_err = Some(e);
                break;
            }
        }
    }
    // Closing before the wait matters only on the read-error break above, where
    // slacomp may still be writing: without it the child could block on a full
    // pipe that nothing is draining.
    drop(pipe);
    let status = match child.wait() {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("error: failed to run slacomp: {e}");
            return 2;
        }
    };
    match write_err {
        Some(err) => crate::output::status_after(err, status),
        None => status,
    }
}

fn usage() {
    eprintln!(
        "usage: kuna specs [-a <dir>] [<file.slaspec>..] [--diff]\n\
         \n\
         Compile SLEIGH processor specifications with the Rust SLEIGH compiler\n\
         (`slacomp`), which this command is a thin alias for -- every other flag is\n\
         passed straight through, and it takes upstream `sleigh_opt`'s CLI.\n\
         \n\
         -a <dir> compiles every .slaspec under <dir> recursively (= make specs over\n\
         specs/); a bare list of files compiles just those.  A .sla is a build\n\
         artifact and is gitignored; the engine finds them under KUNA_SPECS /\n\
         SLEIGHHOME.\n\
         --diff prints why the old C++ byte-for-byte differential is moot."
    );
}
