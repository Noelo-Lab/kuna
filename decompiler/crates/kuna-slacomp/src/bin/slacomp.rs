//! The `slacomp` SLEIGH compiler binary ([`kuna_slacomp::slacompmain`]).

use std::process::ExitCode;

fn main() -> ExitCode {
    kuna_slacomp::slacompmain::main(std::env::args().collect())
}
