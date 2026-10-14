//! The `decomp_dbg` console binary ([`kuna_console::consolemain`]).

use std::process::ExitCode;

fn main() -> ExitCode {
    kuna_console::consolemain::main(std::env::args().collect())
}
