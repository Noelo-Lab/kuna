mod common;

use std::process::{Command, Output};

const EDGE: &[u8] = &[
    0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0xbb, 5, 0, 0, 0,
];

fn fixture(bits: u8) -> std::path::PathBuf {
    named_fixture("boundary", bits)
}

fn named_fixture(kind: &str, bits: u8) -> std::path::PathBuf {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    root.join(format!(
        "../kuna-analysis/tests/fixtures/mapped_flow_{kind}_{bits}.elf"
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
    for (kind, warning) in [("boundary", " flows to "), ("straddle", "runs past the mapped bytes")] {
        for bits in [32, 64] {
            let path = named_fixture(kind, bits);
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
                assert!(stdout.contains(warning), "{stdout}");
                assert!(!stdout.contains("external symbol"), "{stdout}");
            }
            let off = decompile(&path, &["--option", "mappedflowboundary", "off", "--json"]);
            assert!(!off.status.success());
            let json = String::from_utf8_lossy(&off.stdout);
            assert!(json.contains("\"code\": null"), "{json}");
        }
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
    for (code, diagnostic) in [
        (
            &[0xeb, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x85, 0xc0, 0x74, 0xf6, 0xbb, 5, 0, 0, 0][..],
            "halt_missing",
        ),
        (
            &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x85, 0xdb, 0x75, 1, 0x90, 0x90][..],
            "halt_missing",
        ),
        (
            &[0xb8, 7, 0, 0, 0, 0x85, 0xdb, 0x75, 1, 0xe8, 0xc3][..],
            "overlapbranch:",
        ),
    ] {
        let path = with_code(code);
        let out = decompile(&path, &["--json"]);
        let text = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{text}\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(text.contains("return 7;"), "{text}");
        assert!(text.contains(diagnostic), "{text}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_truncated_entry_instruction_keeps_a_failure_in_text_and_json() {
    let path = with_code(&[0xb8, 7, 0]);
    for extra in [&[][..], &["--json"][..]] {
        let out = decompile(&path, extra);
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
fn an_instruction_running_past_the_mapped_end_ends_only_its_own_path() {
    let overlap_off = ["--option", "overlapbranch", "off"];
    for (code, options, kept) in [
        (
            &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x8e, 0xc8, 0, 0, 0][..],
            &[][..],
            "return 7;",
        ),
        (
            &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0][..],
            &[][..],
            "return 7;",
        ),
        (
            &[0xb8, 7, 0, 0, 0, 0x85, 0xdb, 0x75, 1, 0xe8, 0xc3][..],
            &overlap_off[..],
            "return 7;",
        ),
        (
            &[0xb8, 7, 0, 0, 0, 0x85, 0xdb, 0x75, 1, 0xe8, 0xb8][..],
            &[][..],
            "if (",
        ),
    ] {
        let path = with_code(code);
        for json in [false, true] {
            let mut extra = options.to_vec();
            if json {
                extra.push("--json");
            }
            let out = decompile(&path, &extra);
            let text = String::from_utf8_lossy(&out.stdout);
            assert!(
                out.status.success(),
                "{text}\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
            assert!(text.contains(kept), "{text}");
            assert!(text.contains("halt_missing"), "{text}");
            assert!(text.contains("runs past the mapped bytes"), "{text}");
        }
        let mut off = options.to_vec();
        off.extend(["--option", "mappedflowboundary", "off", "--json"]);
        let out = decompile(&path, &off);
        assert!(
            !out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
        std::fs::remove_file(path).unwrap();
    }
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

#[test]
fn a_declared_range_exit_at_a_known_entry_keeps_its_own_out_of_bounds_stub() {
    let mut code = vec![0x85, 0xc0, 0x74, 5, 0xe8, 0x17, 0, 0, 0, 0xb8, 7, 0, 0, 0, 0xc3];
    code.resize(32, 0xcc);
    code.extend_from_slice(&[0x31, 0xc0, 0xc3]);
    let raw = common::scratch_file("funcbound-declared-range", "bin");
    std::fs::write(&raw, &code).unwrap();
    let elf = with_code(&code);
    let bounds = [
        "--define-function",
        "0x10000-0x10009=f",
        "--define-function",
        "0x10009=g",
        "--define-function",
        "0x10020=die",
    ];
    let raw_args = ["--raw-image", "--target", "x86:LE:32:default:gcc", "--base", "0x10000"];
    for (path, image) in [(&raw, &raw_args[..]), (&elf, &[][..])] {
        let mut texts = Vec::new();
        for value in ["on", "off"] {
            let mut extra = image.to_vec();
            extra.extend(bounds);
            extra.extend(["--option", "mappedflowboundary", value]);
            let out = decompile(path, &extra);
            let text = String::from_utf8_lossy(&out.stdout).into_owned();
            assert!(
                out.status.success(),
                "{text}\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
            assert!(text.contains("if (a0)"), "{text}");
            assert!(text.contains("die();"), "{text}");
            assert!(text.contains("halt_missing();"), "{text}");
            assert!(
                text.contains("Function flow out of bounds: r0x00010002 flows to r0x00010009"),
                "{text}"
            );
            assert!(!text.contains("return-dupe"), "{text}");
            texts.push(text);
        }
        assert_eq!(texts[0], texts[1]);
    }
    std::fs::remove_file(raw).unwrap();
    std::fs::remove_file(elf).unwrap();
}
