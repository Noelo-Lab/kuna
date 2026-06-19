//! WS4b end-to-end gate: the `SleighCompile` driver compiles a `.slaspec` all
//! the way to a `.sla` whose **decompressed element stream** is byte-identical
//! to the committed C++ `sleigh_opt` output.
//!
//! Whole-file raw byte-identity is blocked by LOSS-010 (flate2 deflate != C
//! zlib), so the gate compares the *decompressed* packed element streams,
//! exactly as `encode_roundtrip.rs` (WS5) does.
//!
//! The byte-identical subset is the specs with no semantic p-code sections
//! (`data-le-64` / `data-be-64`: the simplest of the 148 vendored specs, a
//! single `:nop is test=1 unimpl` constructor).  Specs whose constructors carry
//! RTL bodies / macros / operand definitions need the p-code section path that
//! `kuna-sleigh` does not yet port (`ConstructTpl::fillinBuild` /
//! `Constructor::setMainSection` / `markSubtableOperands`); they are deferred to
//! WS6/WS7 and not asserted here.

use std::path::PathBuf;

use kuna_slacomp::slgh_compile::SleighCompile;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

/// Decompress a `.sla` body (everything after the 4-byte `sla\x04` header) into
/// the raw packed element stream.
fn inflate(sla: &[u8]) -> Vec<u8> {
    use std::io::Read;
    assert_eq!(&sla[..4], b"sla\x04", "missing sla header");
    let mut dec = flate2::read::ZlibDecoder::new(&sla[4..]);
    let mut out = Vec::new();
    dec.read_to_end(&mut out).expect("inflate .sla body");
    out
}

/// Compile `rel` (a `.slaspec` under `specs/`) with the Rust driver and assert
/// its decompressed element stream equals the committed C++ `.sla`.
fn check_byte_identical(rel: &str) {
    let spec = repo_root().join(rel);
    let gold_path = repo_root().join(rel.replace(".slaspec", ".sla"));
    let gold = match std::fs::read(&gold_path) {
        Ok(b) => b,
        Err(e) => panic!(
            "read {} failed ({e}); is the .sla symlinked/built?",
            gold_path.display()
        ),
    };

    let out = std::env::temp_dir().join(format!(
        "ws4b_{}",
        spec.file_name().unwrap().to_string_lossy().replace(".slaspec", ".sla")
    ));
    let mut compiler = SleighCompile::new();
    let code = compiler
        .run_compilation(&spec.to_string_lossy(), &out.to_string_lossy())
        .expect("run_compilation");
    assert_eq!(code, 0, "{rel}: compiler returned nonzero exit code");

    let produced = std::fs::read(&out).expect("read produced .sla");
    let gold_stream = inflate(&gold);
    let rust_stream = inflate(&produced);

    assert_eq!(
        rust_stream.len(),
        gold_stream.len(),
        "{rel}: stream length differs ({} vs {} bytes)",
        rust_stream.len(),
        gold_stream.len()
    );
    if rust_stream != gold_stream {
        let i = rust_stream
            .iter()
            .zip(gold_stream.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(gold_stream.len());
        panic!(
            "{rel}: decompressed element stream diverges at byte {i} \
             (rust={:#04x} gold={:#04x})",
            rust_stream.get(i).copied().unwrap_or(0),
            gold_stream.get(i).copied().unwrap_or(0),
        );
    }
}

#[test]
fn ws4b_compile_data_le_64() {
    check_byte_identical("specs/Ghidra/Processors/DATA/data/languages/data-le-64.slaspec");
}

#[test]
fn ws4b_compile_data_be_64() {
    check_byte_identical("specs/Ghidra/Processors/DATA/data/languages/data-be-64.slaspec");
}
