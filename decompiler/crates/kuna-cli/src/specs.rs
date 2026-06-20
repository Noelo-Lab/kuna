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
//! docs/RUST_PORT.md and is subsumed by `kuna test` (the Rust-built specs decode
//! to 675/675).  So `--diff` is a documentation note, not a live comparison.

use std::process::Command;

use crate::paths;

const DIFF_NOTE: &str = "\
kuna specs --diff: the C++ differential is moot.

The Python `kuna.slacomp` differential compiled each .slaspec with both the C++
`sleigh_opt` and the Rust `slacomp` and required byte-identical .sla content.
The C++ tree was removed (see docs/RUST_PORT.md); there is no in-tree oracle to
diff against anymore.  The recorded result was 148/148 content-identical, and it
is subsumed by `kuna test`: rebuilding all specs with the Rust `slacomp` and
re-running the datatest corpus yields 675/675, proving the Rust-built specs
decode identically to the C++-built ones.  Use `kuna specs -a <dir>` to compile,
and `kuna test --datatests --baseline docs/baseline.json` as the end-to-end gate.";

pub fn run(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--diff") {
        println!("{DIFF_NOTE}");
        return 0;
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
    let status = Command::new(&bin).args(args).status();
    match status {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("error: failed to run slacomp: {e}");
            2
        }
    }
}
