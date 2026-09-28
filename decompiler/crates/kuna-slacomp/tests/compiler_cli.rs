use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn write_spec(directory: &Path, name: &str) {
    fs::create_dir_all(directory).unwrap();
    fs::write(
        directory.join(name),
        include_str!("golden/snips/data_le_64.slaspec"),
    )
    .unwrap();
    fs::write(
        directory.join("data.sinc"),
        include_str!("golden/snips/data.sinc"),
    )
    .unwrap();
}

fn compile(arguments: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_slacomp"))
        .args(arguments)
        .output()
        .expect("run slacomp")
}

fn assert_compiled(result: Output, output: &Path) {
    assert!(
        result.status.success(),
        "slacomp failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(fs::read(output).unwrap().starts_with(b"sla\x04"));
}

#[test]
fn omitted_input_extension_in_dotted_directory() {
    let scratch = tempfile::tempdir().unwrap();
    let directory = scratch.path().join("source.v1");
    write_spec(&directory, "data_le_64.slaspec");
    let result = compile(&[&directory.join("data_le_64")]);
    assert_compiled(result, &directory.join("data_le_64.sla"));
}

#[test]
fn omitted_output_extension_in_dotted_directory() {
    let scratch = tempfile::tempdir().unwrap();
    write_spec(scratch.path(), "data_le_64.slaspec");
    let directory = scratch.path().join("output.v1");
    fs::create_dir(&directory).unwrap();
    let result = compile(&[
        &scratch.path().join("data_le_64.slaspec"),
        &directory.join("compiled"),
    ]);
    assert_compiled(result, &directory.join("compiled.sla"));
}

#[test]
fn explicit_extensions_accept_dotted_names_and_dotfiles() {
    for (source, output) in [("data.v1.slaspec", "compiled.v1.sla"), (".slaspec", ".sla")] {
        let scratch = tempfile::tempdir().unwrap();
        write_spec(scratch.path(), source);
        let destination = scratch.path().join(output);
        let result = compile(&[&scratch.path().join(source), &destination]);
        assert_compiled(result, &destination);
    }
}

#[test]
fn unknown_filename_extensions_are_rejected() {
    let scratch = tempfile::tempdir().unwrap();
    write_spec(scratch.path(), "data_le_64.slaspec");
    for name in ["data.unknown", "data.", ".data", "data.slaspec.bak"] {
        let result = compile(&[&scratch.path().join(name)]);
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&result.stderr).contains("Unknown input file type:"));
    }
    for name in [
        "compiled.unknown",
        "compiled.",
        ".compiled",
        "compiled.sla.bak",
    ] {
        let destination = scratch.path().join(name);
        let result = compile(&[&scratch.path().join("data_le_64.slaspec"), &destination]);
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&result.stderr).contains("Unknown output file type:"));
        assert!(!destination.exists());
    }
}

#[test]
fn extra_filenames_are_rejected_before_compilation() {
    let scratch = tempfile::tempdir().unwrap();
    write_spec(scratch.path(), "data_le_64.slaspec");
    let destination = scratch.path().join("compiled.sla");
    let result = compile(&[
        &scratch.path().join("data_le_64.slaspec"),
        &destination,
        &scratch.path().join("extra.sla"),
    ]);
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("Too many parameters"));
    assert!(!destination.exists());
}

#[test]
fn xml_output_matches_pinned_ghidra_in_single_and_recursive_modes() {
    for recursive in [false, true] {
        let scratch = tempfile::tempdir().unwrap();
        let directory = scratch.path().join("nested");
        fs::create_dir(&directory).unwrap();
        fs::write(
            directory.join("xml_ops.slaspec"),
            include_str!("golden/snips/xml_ops.slaspec"),
        )
        .unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_slacomp"));
        command.arg("-y");
        if recursive {
            command.arg("-a").arg(scratch.path());
        } else {
            command.arg(directory.join("xml_ops.slaspec"));
        }
        let result = command.output().expect("run slacomp -y");
        assert!(
            result.status.success(),
            "slacomp -y failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let produced = fs::read_to_string(directory.join("xml_ops.sla"))
            .expect("slacomp -y must write UTF-8 XML");
        assert_eq!(produced, include_str!("golden/xml_ops.xml"));
    }
}
