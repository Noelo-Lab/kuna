//! `kuna_wasm` — the in-browser decompiler entry point.
//!
//! Usage (argv is supplied by the WASI host / shell):
//!   kuna_wasm <binary> <spec-root> list
//!   kuna_wasm <binary> <spec-root> decompile [<name>|0x<addr>]
//!   kuna_wasm <binary> <spec-root> project [<display-name>]
//!
//! `<binary>` and `<spec-root>` are paths in the (virtual) filesystem. Writes a
//! JSON document to stdout (`list`/`decompile`: the shape of
//! `kuna decompile-all --json` plus a per-function `"kind"`; `project`: the
//! whole-binary `.c`/`.h`/`.asm`/`README.md` artifacts — named after
//! `<display-name>`, default the binary's basename — as one document); errors
//! go to stderr with a nonzero exit code. See `kuna_wasm::run` and
//! `docs/web-integration.md`.

use std::process::ExitCode;

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 4 {
        eprintln!(
            "usage: {} <binary> <spec-root> <list|decompile|project> [name|0xaddr|display-name]",
            argv[0]
        );
        return ExitCode::from(64);
    }
    let binary = &argv[1];
    let spec_root = &argv[2];
    let cmd = &argv[3];
    let arg = argv.get(4).map(|s| s.as_str());

    match kuna_wasm::run(binary, spec_root, cmd, arg) {
        Ok(payload) => {
            println!("{payload}");
            ExitCode::SUCCESS
        }
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::FAILURE
        }
    }
}
