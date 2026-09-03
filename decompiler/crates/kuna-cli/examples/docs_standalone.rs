//! `kuna docs` as a binary of its own — `src/docs.rs` given a `main`.
//!
//! It exists to demonstrate the property the feature is *for*, independently of
//! the `kuna` dispatch table: build it, copy the executable anywhere — an empty
//! directory, a machine with no checkout — and every document is still there,
//! because they are `include_str!`-ed into the executable rather than read from
//! `docs/` at runtime.
//!
//! ```sh
//! cargo build --release -p kuna-cli --example docs_standalone
//! decompiler/target/release/examples/docs_standalone options | head
//! ```
//!
//! It is not part of `make binaries` and is not shipped; `kuna docs` is the
//! product surface.

#[path = "../src/docs.rs"]
mod docs;
#[allow(dead_code)]
#[path = "../src/jsonfmt.rs"]
mod jsonfmt;
#[allow(dead_code)]
#[path = "../src/output.rs"]
mod output;

fn main() -> std::process::ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    std::process::ExitCode::from(docs::run(&argv) as u8)
}
