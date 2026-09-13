mod common;

use std::process::{Command, Output};

const EDGE: &[u8] = &[
    0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0xbb, 5, 0, 0, 0,
];

fn fixture(bits: u8) -> std::path::PathBuf {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    root.join(format!(
        "../kuna-analysis/tests/fixtures/mapped_flow_boundary_{bits}.elf"
    ))
}

fn decompile(path: &std::path::Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", path.to_str().unwrap(), "0x10000", "--addr"])
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn mapped_return_survives_another_path_falling_off_the_image() {
    for bits in [32, 64] {
        let path = fixture(bits);
        for extra in [vec![], vec!["--json"]] {
            let out = decompile(&path, &extra);
            let stdout = String::from_utf8_lossy(&out.stdout);
            assert!(
                out.status.success(),
                "{stdout}\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
            assert!(stdout.contains("return 7;"), "{stdout}");
            assert!(stdout.contains("halt_missing"), "{stdout}");
            assert!(stdout.contains("unmapped memory"), "{stdout}");
            assert!(!stdout.contains("external symbol"), "{stdout}");
        }
        let off = decompile(&path, &["--option", "mappedflowboundary", "off", "--json"]);
        assert!(!off.status.success());
        let json = String::from_utf8_lossy(&off.stdout);
        assert!(json.contains("\"code\": null"), "{json}");
    }
}

#[test]
fn final_byte_return_needs_no_mapped_prefetch_padding() {
    let path = common::scratch_file("final-byte-return", "elf");
    let mut image = std::fs::read(fixture(32)).unwrap();
    let ph = u32::from_le_bytes(image[28..32].try_into().unwrap()) as usize;
    let offset = u32::from_le_bytes(image[ph + 4..ph + 8].try_into().unwrap()) as usize;
    image[ph + 16..ph + 20].copy_from_slice(&6u32.to_le_bytes());
    image[ph + 20..ph + 24].copy_from_slice(&6u32.to_le_bytes());
    image.truncate(offset);
    image.extend_from_slice(&EDGE[4..10]);
    std::fs::write(&path, image).unwrap();
    let out = decompile(&path, &[]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("return 7;"), "{text}");
    assert!(!text.contains("halt_missing"), "{text}");
    std::fs::remove_file(path).unwrap();
}

fn with_code(code: &[u8]) -> std::path::PathBuf {
    let path = common::scratch_file("mapped-flow-code", "elf");
    let mut image = std::fs::read(fixture(32)).unwrap();
    image[68..72].copy_from_slice(&(code.len() as u32).to_le_bytes());
    image[72..76].copy_from_slice(&(code.len() as u32).to_le_bytes());
    image.truncate(84);
    image.extend_from_slice(code);
    std::fs::write(&path, image).unwrap();
    path
}

#[test]
fn queued_paths_resolve_before_and_after_a_missing_edge_is_cut() {
    for code in [
        &[0xeb, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x85, 0xc0, 0x74, 0xf6, 0xbb, 5, 0, 0, 0][..],
        &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x85, 0xdb, 0x75, 1, 0x90, 0x90][..],
    ] {
        let path = with_code(code);
        let out = decompile(&path, &[]);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{text}\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(text.contains("return 7;"), "{text}");
        assert!(text.contains("halt_missing"), "{text}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn truncated_instruction_keeps_a_failure_in_text_and_json() {
    let path = with_code(&[0xb8, 7, 0]);
    for extra in [vec![], vec!["--json"]] {
        let out = decompile(&path, &extra);
        assert!(
            !out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
        assert!(String::from_utf8_lossy(&out.stderr).contains("not mapped"));
        assert!(!String::from_utf8_lossy(&out.stdout).contains("external symbol"));
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_mismatched_x86_width_does_not_enable_boundary_recovery() {
    let out = decompile(
        &fixture(32),
        &["--target", "x86:LE:64:default:gcc", "--json"],
    );
    assert!(
        !out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("Unable to load"));
}

#[test]
fn effective_return_and_callreturn_overrides_precede_boundary_recovery() {
    let path = with_code(&[0xb8, 7, 0, 0, 0, 0xff, 0xd2]);
    for kind in ["return", "callreturn"] {
        let directive = format!("flow 0x10005 {kind}");
        let out = decompile(&path, &["--assert", &directive]);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{text}\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(!text.contains("halt_missing"), "{text}");
        assert!(!text.contains("unmapped memory"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn shared_whole_image_and_project_paths_retain_the_recovered_body() {
    let path = fixture(32);
    let single = decompile(&path, &["--json"]);
    let all = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile-all", path.to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    assert!(
        all.status.success(),
        "{}",
        String::from_utf8_lossy(&all.stderr)
    );
    assert_eq!(single.stdout, all.stdout);
    let dir = common::scratch_file("mapped-flow-project", "export");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-project",
            path.to_str().unwrap(),
            "--output",
            dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut text = String::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let file = entry.unwrap().path();
        if file.extension().is_some_and(|ext| ext == "c") {
            text.push_str(&std::fs::read_to_string(file).unwrap());
        }
    }
    assert!(text.contains("return 7;"), "{text}");
    assert!(text.contains("halt_missing"), "{text}");
    std::fs::remove_dir_all(dir).unwrap();
}
