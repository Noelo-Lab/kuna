use kuna_harness::testfunction::run_test_files_with_specs;
use std::path::PathBuf;

#[test]
fn byte_aligned_bitfields_read_and_write_by_name() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let fixture = root.join("tests/cli/fixtures/bitfield-byte-pull/byte-aligned-bitfields.xml");
    let specs = root.join("specs");
    let mut output = String::new();
    let failures = run_test_files_with_specs(
        &[fixture.to_str().unwrap().to_string()],
        &[specs.to_str().unwrap().to_string()],
        &mut output,
    );
    assert_eq!(failures, 0, "{output}");
    assert!(output.contains("Total passing tests = 5"), "{output}");
}
