//! Embedded CVF metadata uses the normal PE analysis and comment commit paths.
use crate::common;
use common::process;
use object::{Object, ObjectSection};
use std::path::Path;
use std::process::Command;

const SOURCE: &str = "__attribute__((section(\".trace\"), used)) const unsigned char metadata[512] = {1};\n\
int sample(void) { return 7; }\n\
__declspec(dllexport) int exported(void) { return sample() + 1; }\n";

fn routine(trace: &mut Vec<u8>, address: u32, name: &str) {
    trace.extend([2, 0]);
    trace.extend((name.len() as u16).to_le_bytes());
    trace.extend(address.to_le_bytes());
    trace.extend(name.as_bytes());
}

/// Link the sample, then write one trace record over its `.trace` section naming
/// both functions; returns the image and the two addresses.
fn traced_dll(root: &Path, extra: &[&str]) -> (Vec<u8>, u32, u32) {
    let source = root.join("sample.c");
    let binary = root.join(format!("sample{}.dll", extra.len()));
    std::fs::write(&source, SOURCE).unwrap();
    process::required_output(
        Command::new("clang")
            .args(["--target=i686-w64-windows-gnu", "-shared", "-nostdlib", "-O0"])
            .args(["-Wl,--entry,_sample", "-Wl,--image-base,0x10000000"])
            .args(extra)
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    let mut bytes = std::fs::read(&binary).unwrap();
    let (sample, exported, offset) = {
        let image = object::File::parse(bytes.as_slice()).unwrap();
        let exported = image
            .exports()
            .unwrap()
            .into_iter()
            .find(|e| e.name() == b"exported")
            .unwrap()
            .address() as u32;
        let offset = image.section_by_name(".trace").unwrap().file_range().unwrap().0;
        (image.entry() as u32, exported, offset as usize)
    };
    let mut trace = vec![1, 1, 0, 0];
    trace.extend(sample.min(exported).to_le_bytes());
    trace.extend(0x40u32.to_le_bytes());
    trace.extend([0; 4]);
    trace.extend(11u16.to_le_bytes());
    trace.extend(b"minimal.for");
    routine(&mut trace, sample, "FROM_TRACE");
    trace.extend([3, 0, 0, 0, 0, 0xc0, 7]);
    routine(&mut trace, exported, "EXPORTED_TRACE");
    trace.extend([0xc0, 2]);
    let size = trace.len() as u32;
    trace[12..16].copy_from_slice(&size.to_le_bytes());
    bytes[offset..offset + trace.len()].copy_from_slice(&trace);
    (bytes, sample, exported)
}

fn decompile(binary: &Path, target: &str, mode: Option<&str>) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.arg("decompile").arg(binary).arg(target);
    if let Some(mode) = mode {
        command.args(["--option", "cvftrace", mode]);
    }
    let output = process::required_output(
        command.arg("--sleighpath").arg(common::repo_root().join("specs")),
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn pe_trace_names_by_default_and_comments_on_request() {
    if process::optional_output(Command::new("clang").arg("--version")).is_none() {
        return;
    }
    let root = common::scratch_file("cvf-trace", "dir");
    std::fs::create_dir_all(&root).unwrap();
    let (bytes, sample, exported) = traced_dll(&root, &["-Wl,-s"]);
    let binary = root.join("traced.dll");
    std::fs::write(&binary, &bytes).unwrap();
    let entry = format!("0x{sample:x}");
    let named = decompile(&binary, &entry, None);
    assert!(named.contains("FROM_TRACE(void)"), "{named}");
    assert!(!named.contains("minimal.for"), "{named}");
    let lines = decompile(&binary, &entry, Some("lines"));
    assert!(lines.contains("FROM_TRACE(void)") && lines.contains("minimal.for:7"), "{lines}");
    let off = decompile(&binary, &entry, Some("off"));
    assert!(!off.contains("FROM_TRACE") && !off.contains("minimal.for"), "{off}");
    decompile(&binary, "FROM_TRACE", None);
    let export = decompile(&binary, &format!("0x{exported:x}"), None);
    assert!(export.contains("exported(void)") && !export.contains("EXPORTED_TRACE"), "{export}");

    let (symbols, sample, _) = traced_dll(&root, &[]);
    let with_symbols = root.join("symbols.dll");
    std::fs::write(&with_symbols, &symbols).unwrap();
    let kept = decompile(&with_symbols, &format!("0x{sample:x}"), None);
    assert!(kept.contains("_sample(void)") && !kept.contains("FROM_TRACE"), "{kept}");

    let original = root.join("original.obj");
    let only = root.join("only.c");
    std::fs::write(&only, "int sample(void) { return 7; }\n").unwrap();
    let text = |source: &Path, object: &Path| {
        process::required_output(
            Command::new("clang")
                .args(["--target=i686-w64-windows-gnu", "-O0", "-c"])
                .arg(source)
                .arg("-o")
                .arg(object),
        );
        let bytes = std::fs::read(object).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let text = file.section_by_name(".text").unwrap().data().unwrap().to_vec();
        text
    };
    let expected = text(&only, &original);
    for mode in ["off", "lines"] {
        let project = root.join(mode);
        process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile-project")
                .arg(&binary)
                .arg("--addr")
                .arg(&entry)
                .args(["--option", "cvftrace", mode, "--sleighpath"])
                .arg(common::repo_root().join("specs"))
                .arg("--assert")
                .arg(format!("prototype {entry} int sample(void)"))
                .arg("-o")
                .arg(&project),
        );
        let regenerated = text(&project.join("traced.dll.c"), &project.join("regenerated.obj"));
        assert_eq!(expected, regenerated, "{mode}");
    }

    let mut renamed = bytes;
    let section_name = renamed.windows(8).position(|b| b == b".trace\0\0").unwrap();
    renamed[section_name..section_name + 8].copy_from_slice(b".other\0\0");
    std::fs::write(&binary, renamed).unwrap();
    assert_eq!(decompile(&binary, &entry, Some("off")), decompile(&binary, &entry, Some("lines")));
}
