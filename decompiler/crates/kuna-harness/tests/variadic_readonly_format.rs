use kuna_harness::testfunction::run_test_files_with_specs;
use std::path::PathBuf;

#[test]
fn variadic_format_evidence_requires_an_immutable_conversion_and_terminator() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap();
    let fixtures = ["mixed-format.xml", "named-format.xml"].map(|name| {
        root.join("tests/cli/fixtures/variadic-secondary-reader")
            .join(name)
            .to_str()
            .unwrap()
            .to_string()
    });
    let specs = root.join("specs");
    let mut output = String::new();
    let failures = run_test_files_with_specs(
        &fixtures,
        &[specs.to_str().unwrap().to_string()],
        &mut output,
    );
    assert_eq!(failures, 0, "{output}");
    assert!(output.contains("Total tests applied = 8"), "{output}");
    assert!(output.contains("Total passing tests = 8"), "{output}");
}
