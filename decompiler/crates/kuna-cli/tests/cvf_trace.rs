//! Embedded CVF metadata uses the normal PE analysis and comment commit paths.
use crate::common;
use common::process;
use object::{Object, ObjectSection};
use std::process::Command;

#[test]
fn pe_trace_names_and_lines_are_opt_in() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none()
        || process::optional_output(Command::new("lld-link").arg("--version")).is_none()
    {
        return;
    }
    let root = common::scratch_file("cvf-trace", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("sample.c");
    let object = root.join("sample.obj");
    let binary = root.join("sample.dll");
    std::fs::write(&source,"__declspec(allocate(\".trace\")) const unsigned char metadata[128]={0};\nint sample(void){return 7;}\n").unwrap();
    process::required_output(
        Command::new("clang")
            .args(["--target=i686-pc-windows-msvc", "-O0", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object),
    );
    process::required_output(
        Command::new("lld-link")
            .args([
                "/dll",
                "/entry:sample",
                "/nodefaultlib",
                "/fixed",
                "/base:0x10000000",
                "/include:_metadata",
            ])
            .arg(format!("/out:{}", binary.display()))
            .arg(&object),
    );
    let mut bytes = std::fs::read(&binary).unwrap();
    let (entry, offset) = {
        let image = object::File::parse(bytes.as_slice()).unwrap();
        (
            image.entry(),
            image
                .section_by_name(".trace")
                .unwrap()
                .file_range()
                .unwrap()
                .0 as usize,
        )
    };
    let mut trace = vec![1, 1, 0, 0];
    trace.extend((entry as u32).to_le_bytes());
    trace.extend(16u32.to_le_bytes());
    trace.extend([0; 4]);
    trace.extend(11u16.to_le_bytes());
    trace.extend(b"minimal.for");
    trace.extend([2, 0]);
    trace.extend(10u16.to_le_bytes());
    trace.extend((entry as u32).to_le_bytes());
    trace.extend(b"FROM_TRACE");
    trace.extend([3, 0, 0, 0, 0, 0xc0, 7]);
    let size = trace.len() as u32;
    trace[12..16].copy_from_slice(&size.to_le_bytes());
    bytes[offset..offset + trace.len()].copy_from_slice(&trace);
    std::fs::write(&binary, &bytes).unwrap();
    let run = |enabled: &str| {
        process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile")
                .arg(&binary)
                .arg(format!("0x{entry:x}"))
                .args(["--option", "cvftrace", enabled, "--sleighpath"])
                .arg(common::repo_root().join("specs")),
        )
    };
    let off = run("off");
    let on = run("on");
    assert!(!String::from_utf8_lossy(&off.stdout).contains("minimal.for:7"));
    assert!(
        String::from_utf8_lossy(&on.stdout).contains("minimal.for:7"),
        "{}",
        String::from_utf8_lossy(&on.stdout)
    );
    process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile")
            .arg(&binary)
            .arg("FROM_TRACE")
            .args(["--option", "cvftrace", "on", "--sleighpath"])
            .arg(common::repo_root().join("specs")),
    );
    for mode in ["off", "on"] {
        let project = root.join(mode);
        process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile-project")
                .arg(&binary)
                .arg("--addr")
                .arg(format!("0x{entry:x}"))
                .args(["--option", "cvftrace", mode, "--sleighpath"])
                .arg(common::repo_root().join("specs"))
                .arg("--assert")
                .arg(format!("prototype 0x{entry:x} int sample(void)"))
                .arg("-o")
                .arg(&project),
        );
        let regenerated = project.join("regenerated.obj");
        process::required_output(
            Command::new("clang")
                .args(["--target=i686-pc-windows-msvc", "-O0", "-c"])
                .arg(project.join("sample.dll.c"))
                .arg("-o")
                .arg(&regenerated),
        );
        let original = std::fs::read(&object).unwrap();
        let regenerated = std::fs::read(regenerated).unwrap();
        let original = object::File::parse(original.as_slice()).unwrap();
        let regenerated = object::File::parse(regenerated.as_slice()).unwrap();
        assert_eq!(
            original.section_by_name(".text").unwrap().data().unwrap(),
            regenerated
                .section_by_name(".text")
                .unwrap()
                .data()
                .unwrap()
        );
    }
    let section_name = bytes.windows(8).position(|b| b == b".trace\0\0").unwrap();
    bytes[section_name..section_name + 8].copy_from_slice(b".other\0\0");
    std::fs::write(&binary, bytes).unwrap();
    assert_eq!(run("off").stdout, run("on").stdout);
}
