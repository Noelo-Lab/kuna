//! The `kuna_ghidra` binary: kuna's ghidra-mode process (upstream's
//! `decompile` executable) — port of `main()` at
//! `decompiler/cpp/ghidra_process.cc:518-537`.
//!
//! No arguments are parsed; the process communicates exclusively over
//! stdin/stdout and silently waits for the first command burst (there is
//! no startup handshake — the Java side only waits 200 ms and checks
//! liveness before sending registerProgram).
//!
//! Departures from the C++ main, all deliberate:
//! - `signal(SIGSEGV, segvHandler)` / `_Exit(1)` is not ported: safe Rust
//!   has no expected SIGSEGV, and the OS default termination pops no
//!   dialog on the platforms kuna targets.
//! - `_setmode(..., _O_BINARY)` (Windows) is unnecessary: Rust's std
//!   performs no newline translation on any platform.
//! - `AttributeId::initialize()` / `ElementId::initialize()` /
//!   `CapabilityPoint::initializeAll()` are the per-session `IdRegistry`
//!   and the command dispatch inside [`GhidraProcess`].
//!
//! Exit status mirrors upstream: 0 after a deregisterProgram terminates
//! the loop cleanly (C++ falls off main), 1 when the pipe closes or the
//! protocol breaks mid-command (the C++ `exit(1)` sites,
//! ghidra_arch.cc:95-96).

use std::io;

use kuna_ghidra::process::GhidraProcess;

fn main() {
    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();
    let mut process = GhidraProcess::new(stdin, stdout);
    match process.run() {
        Ok(_status) => std::process::exit(0),
        Err(e) => {
            // Pipe closed => our parent process is probably dead, exit to
            // avoid a runaway process (ghidra_arch.cc:95-96).  Any other
            // I/O or unrecovered protocol failure exits the same way.
            eprintln!("kuna_ghidra: {e}");
            std::process::exit(1);
        }
    }
}
