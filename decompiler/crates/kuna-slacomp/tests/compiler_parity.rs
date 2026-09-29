//! Compare compiled specs with hashes captured from the pinned Ghidra compiler.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;

use kuna_slacomp::slgh_compile::SleighCompile;
use sha2::{Digest, Sha256};

const ORACLE: &str = include_str!("golden/compiler.sha256");

#[test]
fn compiled_specs_match_pinned_ghidra() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let scratch = tempfile::tempdir().expect("create compiler test directory");
    let output = scratch.path().join("compiled.sla");
    let mut seen = BTreeSet::new();

    for line in ORACLE
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let (expected, relative) = line.split_once("  ").expect("SHA256 and spec path");
        assert!(seen.insert(relative), "duplicate oracle entry: {relative}");
        let spec = root.join(relative);
        let code = SleighCompile::new()
            .run_compilation(&spec.to_string_lossy(), &output.to_string_lossy())
            .unwrap_or_else(|error| panic!("{relative}: {error}"));
        assert_eq!(code, 0, "{relative}: compilation failed");

        let produced = std::fs::read(&output).expect("read compiled .sla");
        assert!(
            produced.starts_with(b"sla\x04"),
            "{relative}: invalid .sla header"
        );
        let mut stream = Vec::new();
        flate2::read::ZlibDecoder::new(&produced[4..])
            .read_to_end(&mut stream)
            .expect("decompress compiled .sla");
        let actual: String = Sha256::digest(&stream)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(
            actual, expected,
            "{relative}: differs from the pinned Ghidra oracle"
        );
    }

    assert!(!seen.is_empty(), "compiler oracle contains no specs");
}
