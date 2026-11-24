use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Output};

const HINT: &str = "jumptable 401009 401050 401020 401030 401020";

fn run(command: &str, args: &[&str]) -> Output {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg(command)
        .arg(root.join("tests/fixtures/stated-jump/dispatch.elf"))
        .args(args)
        .arg("--sleighpath")
        .arg(root.join("specs"))
        .output()
        .unwrap()
}

fn json(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn code<'a>(doc: &'a Value, function: &str) -> &'a str {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == function)
        .unwrap()["code"]
        .as_str()
        .unwrap()
}

#[test]
fn the_same_addresses_reach_single_and_worker_decompilation() {
    let plain = json(run(
        "decompile-all",
        &["--json", "--jobs", "2", "--option", "protoorder", "off"],
    ));
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(
        file.path(),
        format!("# repeated target, non-address order\n{HINT}\n"),
    )
    .unwrap();
    let flag = format!("@{}", file.path().display());
    let pooled = json(run(
        "decompile-all",
        &[
            "--json",
            "--jobs",
            "2",
            "--option",
            "protoorder",
            "off",
            "--assert",
            &flag,
            "--assert-strict",
        ],
    ));
    let single = json(run(
        "decompile",
        &[
            "dispatch",
            "--json",
            "--option",
            "protoorder",
            "off",
            "--assert",
            &flag,
            "--assert-strict",
        ],
    ));
    let body = code(&pooled, "dispatch");
    assert_eq!(body, code(&single, "dispatch"));
    assert_eq!(body.matches("case ").count(), 3, "{body}");
    for label in [
        "case 0x401020:",
        "case 0x401030:",
        "case 0x401050:",
        "return 0x4d;",
    ] {
        assert!(body.contains(label), "{body}");
    }
    assert!(!code(&plain, "dispatch").contains("switch("));
    assert!(code(&plain, "ordinary").contains("switch("));
    for other in ["other", "ordinary"] {
        assert_eq!(code(&pooled, other), code(&plain, other));
    }
    assert_eq!(pooled["assertions"][0]["status"], "applied");
}

#[test]
fn invalid_or_conflicting_target_sets_are_rejected_atomically() {
    for (hint, reason) in [
        ("jumptable 401000 401020", "expected one computed branch"),
        ("jumptable 401009 401020 ffffffff", "unmapped"),
    ] {
        let strict = run(
            "decompile",
            &["dispatch", "--assert", hint, "--assert-strict"],
        );
        assert!(!strict.status.success());
        assert!(String::from_utf8_lossy(&strict.stderr).contains(reason));
        let result = json(run(
            "decompile-all",
            &["--json", "--jobs", "2", "--assert", hint],
        ));
        assert_eq!(result["assertions"][0]["status"], "rejected");
        assert!(!code(&result, "dispatch").contains("switch("));
    }
    let result = json(run(
        "decompile-all",
        &[
            "--json",
            "--jobs",
            "2",
            "--assert",
            HINT,
            "--assert",
            "jumptable 401009 401020",
        ],
    ));
    assert_eq!(result["assertions"][1]["status"], "rejected");
    assert!(result["assertions"][1]["detail"]
        .as_str()
        .unwrap()
        .contains("conflicting"));
    assert_eq!(code(&result, "dispatch").matches("case ").count(), 3);
}

#[test]
fn word_addressed_branches_use_the_native_selector_units() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let image = tempfile::NamedTempFile::new().unwrap();
    // AVR IJMP through Z. Each destination loads a distinct constant, writes
    // it to the same I/O register, and returns. NOPs separate the blocks.
    let words: [u16; 13] = [
        0x9409, 0, 0xe001, 0xb905, 0x9508, 0, 0xe002, 0xb905, 0x9508, 0, 0xe003, 0xb905, 0x9508,
    ];
    let bytes: Vec<_> = words.into_iter().flat_map(u16::to_le_bytes).collect();
    std::fs::write(image.path(), bytes).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(image.path())
        .args([
            "0x100",
            "--addr",
            "--raw-image",
            "--target",
            "avr8:LE:16:default",
            "--base",
            "0x100",
            "--assert",
            "jumptable 100 102 106 10a",
            "--assert-strict",
            "--sleighpath",
        ])
        .arg(root.join("specs"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body = String::from_utf8(output.stdout).unwrap();
    assert!(body.contains("switch(Z)"), "{body}");
    for label in ["case 0x102:", "case 0x106:", "case 0x10a:"] {
        assert!(body.contains(label), "{body}");
    }
    assert_eq!(body.matches("case ").count(), 3);
}

fn raw_jump(language: &str, bytes: &[u8], hint: &str) -> Output {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), bytes).unwrap();
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(file.path())
        .args([
            "0x100",
            "--addr",
            "--raw-image",
            "--target",
            language,
            "--base",
            "0x100",
            "--sleighpath",
        ])
        .arg(root.join("specs"))
        .args(["--assert", hint, "--assert-strict"])
        .output()
        .unwrap()
}

#[test]
fn branches_use_their_effective_address_and_validate_the_full_set() {
    // mtctr r3; bctr; li r3,11; blr; li r3,22; blr
    let ppc: Vec<_> = [
        0x7c6903a6u32,
        0x4e800420,
        0x3860000b,
        0x4e800020,
        0x38600016,
        0x4e800020,
    ]
    .into_iter()
    .flat_map(u32::to_be_bytes)
    .collect();
    let language = "PowerPC:BE:32:default";
    let ok = raw_jump(language, &ppc, "jumptable 104 108 110");
    let body = String::from_utf8(ok.stdout).unwrap();
    assert!(
        ok.status.success(),
        "{}",
        String::from_utf8_lossy(&ok.stderr)
    );
    assert!(
        body.contains("0xfffffffc") && body.contains("case 0x108:") && body.contains("case 0x110:"),
        "{body}"
    );
    for at in ["109", "10a"] {
        let invalid = raw_jump(language, &ppc, &format!("jumptable 104 108 {at}"));
        assert!(!invalid.status.success());
        assert!(!String::from_utf8_lossy(&invalid.stdout).contains("case "));
        assert!(String::from_utf8_lossy(&invalid.stderr).contains("instruction alignment"));
    }

    let ret = raw_jump("x86:LE:64:default", &[0xc3, 0xc3], "jumptable 100 101");
    assert!(!ret.status.success());
    assert!(String::from_utf8_lossy(&ret.stderr).contains("expected one computed branch"));

    // br x0; mov w0,#11; ret; mov w0,#22; ret
    let a64: Vec<_> = [
        0xd61f0000u32,
        0x52800160,
        0xd65f03c0,
        0x528002c0,
        0xd65f03c0,
    ]
    .into_iter()
    .flat_map(u32::to_le_bytes)
    .collect();
    let ok = raw_jump("AARCH64:LE:64:v8A", &a64, "jumptable 100 104 10c");
    let body = String::from_utf8(ok.stdout).unwrap();
    assert!(
        ok.status.success(),
        "{}",
        String::from_utf8_lossy(&ok.stderr)
    );
    assert!(
        body.contains("case 0x104:") && body.contains("case 0x10c:"),
        "{body}"
    );
}

#[test]
fn exception_handler_transfers_cannot_accept_jump_destinations() {
    for (lang, bytes, hint) in [
        (
            "x86:LE:64:default",
            &[0x0f, 0x0b, 0xc3][..],
            "jumptable 100 102",
        ),
        (
            "AARCH64:LE:64:v8A",
            &[0x20, 0x00, 0x20, 0xd4, 0xc0, 0x03, 0x5f, 0xd6][..],
            "jumptable 100 104",
        ),
    ] {
        let bad = raw_jump(lang, bytes, hint);
        assert!(!bad.status.success(), "{lang}");
        assert!(
            String::from_utf8_lossy(&bad.stderr).contains("exception transfer"),
            "{}",
            String::from_utf8_lossy(&bad.stderr)
        );
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let bad = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(root.join("decompiler/crates/kuna-analysis/tests/fixtures/kernel_bugtable_x86_64.ko"))
        .args(["kwarn_check", "--sleighpath"])
        .arg(root.join("specs"))
        .args(["--assert", "jumptable 400085 400060", "--assert-strict"])
        .output()
        .unwrap();
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("exception transfer"));
}

#[test]
fn worker_pool_explains_how_to_apply_mixed_assertions() {
    let out = run(
        "decompile-all",
        &[
            "--jobs",
            "2",
            "--assert",
            HINT,
            "--assert",
            "comment dispatch::401009 jump",
        ],
    );
    assert!(!out.status.success());
    let diagnostic = String::from_utf8_lossy(&out.stderr);
    assert!(
        diagnostic.contains("jumptable directives alone") && diagnostic.contains("--jobs 1"),
        "{diagnostic}"
    );
}

#[test]
fn summaries_report_the_same_rejection_with_and_without_strict_mode() {
    for jobs in ["1", "2"] {
        let mut args = vec![
            "--jobs",
            jobs,
            "--summary",
            "--json",
            "--assert",
            "jumptable 401000 401020",
        ];
        let loose = json(run("decompile-all", &args));
        assert_eq!(loose["assertions"][0]["status"], "rejected");
        args.push("--assert-strict");
        let strict = run("decompile-all", &args);
        assert!(!strict.status.success());
        assert!(String::from_utf8_lossy(&strict.stderr).contains("expected one computed branch"));
        let again: Value = serde_json::from_slice(&strict.stdout).unwrap();
        assert_eq!(again["assertions"], loose["assertions"]);
        let valid = json(run(
            "decompile-all",
            &[
                "--jobs",
                jobs,
                "--summary",
                "--json",
                "--assert",
                HINT,
                "--assert-strict",
            ],
        ));
        assert_eq!(valid["assertions"][0]["status"], "applied");
    }
}

#[test]
fn a_runtime_pointer_cannot_be_compared_to_a_pie_vma() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for command in ["decompile", "decompile-all"] {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
        cmd.arg(command)
            .arg(root.join("tests/fixtures/stated-jump/pie.elf"));
        if command == "decompile" {
            cmd.arg("dispatch");
        } else {
            cmd.args(["--functions", "dispatch", "--jobs", "2"]);
        }
        let out = cmd
            .args([
                "--json",
                "--assert",
                "jumptable 1005 1007 100d",
                "--assert-strict",
                "--sleighpath",
            ])
            .arg(root.join("specs"))
            .output()
            .unwrap();
        let all = format!(
            "{} {}",
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
        assert!(all.contains("runtime load base"), "{all}");
        assert!(!all.contains("case 0x1007:"), "{all}");
    }
}

#[test]
fn unlabelable_branches_reject_the_directive_and_keep_the_function() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let file = tempfile::NamedTempFile::new().unwrap();
    // bx r0; mov r0,#11; bx lr; mov r0,#22; bx lr
    let arm: Vec<_> = [0xe12fff10u32, 0xe3a0000b, 0xe12fff1e, 0xe3a00016, 0xe12fff1e]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    std::fs::write(file.path(), arm).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decompile")
        .arg(file.path())
        .args(["0x100", "--addr", "--raw-image", "--target", "ARM:LE:32:v8", "--isa", "arm"])
        .args(["--base", "0x100", "--assert", "jumptable 100 104 10c", "--sleighpath"])
        .arg(root.join("specs"))
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let body = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{stderr}");
    assert!(stderr.contains("rejected") && stderr.contains("interworking"), "{stderr}");
    assert!(body.contains("jump-as-call") && !body.contains("case "), "{body}");
}

#[test]
fn summaries_leave_per_function_directives_to_the_decompile() {
    let doc = json(run(
        "decompile-all",
        &["--summary", "--json", "--assert", "comment dispatch::401009 jump", "--assert-strict"],
    ));
    assert!(doc.get("assertions").is_none(), "{doc}");
}

fn kuna(command: &str, binary: &std::path::Path, args: &[&str]) -> Output {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg(command)
        .arg(binary)
        .args(args)
        .arg("--sleighpath")
        .arg(root.join("specs"))
        .output()
        .unwrap()
}

#[test]
fn loaders_that_can_move_code_reject_destinations() {
    use kuna_analysis::loadimage_te::synthetic::{TeImage, CODE_RVA, IMAGE_BASE};
    // jmp rdi; mov eax,11; ret; mov eax,22; ret; then one 12-byte relocation block.
    let mut code = vec![0xff, 0xe7, 0xb8, 0x0b, 0, 0, 0, 0xc3, 0xb8, 0x16, 0, 0, 0, 0xc3];
    let block = code.len() as u32;
    code.extend_from_slice(&[0, 0x10, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0]);
    let fixed = TeImage::arm(&code).machine(0x8664);
    let entry = IMAGE_BASE + u64::from(CODE_RVA);
    let hint = format!("jumptable {entry:x} {:x} {:x}", entry + 2, entry + 8);
    for (image, movable) in [(fixed.clone(), false), (fixed.base_relocations(CODE_RVA + block, 12), true)] {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), image.build()).unwrap();
        let entry = format!("{entry:#x}");
        let out = kuna("decompile", file.path(), &[&entry, "--addr", "--assert", &hint, "--assert-strict"]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        let body = String::from_utf8_lossy(&out.stdout);
        assert_eq!(out.status.success(), !movable, "{stderr}");
        assert_eq!(stderr.contains("runtime load base is unknown"), movable, "{stderr}");
        assert_eq!(body.contains("case 0x401002:"), !movable, "{body}");
    }

    // DOS chooses the load segment: jmp ax; mov ax,11; ret; mov ax,22; ret.
    let module = [0xff, 0xe0, 0xb8, 0x0b, 0x00, 0xc3, 0xb8, 0x16, 0x00, 0xc3];
    let mut exe = vec![0u8; 0x20];
    exe[..2].copy_from_slice(b"MZ");
    let total = exe.len() + module.len();
    for (at, value) in [(2, total % 512), (4, total.div_ceil(512)), (8, 2), (0x10, 0x100), (0x18, 0x1c)] {
        exe[at..at + 2].copy_from_slice(&(value as u16).to_le_bytes());
    }
    exe.extend_from_slice(&module);
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), exe).unwrap();
    let out = kuna("decompile", file.path(), &["0x10000", "--addr", "--assert", "jumptable 10000 10002 10006"]);
    assert!(String::from_utf8_lossy(&out.stderr).contains("runtime load base is unknown"));
    assert!(!String::from_utf8_lossy(&out.stdout).contains("case "));
}

#[test]
fn segmented_and_compressed_destinations_cannot_label_cases() {
    // Each jump returns 11 or 22: mov ax,11; ret; mov ax,22; ret (eax in 32-bit code).
    let real = "x86:LE:16:Real Mode:default";
    let near = [0xff, 0xe0, 0xb8, 0x0b, 0x00, 0xc3, 0xb8, 0x16, 0x00, 0xc3];
    let far = [0xff, 0x2f, 0xb8, 0x0b, 0x00, 0xc3, 0xb8, 0x16, 0x00, 0xc3];
    let ax32 = [0x66, 0xff, 0xe0, 0xb8, 0x0b, 0, 0, 0, 0xc3, 0xb8, 0x16, 0, 0, 0, 0xc3];
    // jmp eax with operand- and address-size prefixes: SLEIGH omits the segment operation.
    let eax16 = [0x66, 0x67, 0xff, 0xe0, 0x66, 0xb8, 0x0b, 0, 0, 0, 0xc3, 0x66, 0xb8, 0x16, 0, 0, 0, 0xc3];
    let rejected: [(&[u8], &str, &str, &str, &str); 5] = [
        (&near, real, "0x10000", "jumptable 10000 10002 10006", "segmented destination"),
        (&far, real, "0x10000", "jumptable 10000 10002 10006", "segmented destination"),
        (&far, real, "0x100", "jumptable 100 102 106", "segmented destination"),
        (&eax16, real, "0x10000", "jumptable 10000 10004 1000b", "segmented destination"),
        (&ax32, "x86:LE:32:default", "0x401000", "jumptable 401000 401003 401009", "destination 0x401003 does not fit the 2-byte value"),
    ];
    for (bytes, target, base, directive, reason) in rejected {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), bytes).unwrap();
        let mut args = vec![base, "--addr", "--raw-image", "--target", target, "--base", base];
        let plain = kuna("decompile", file.path(), &args);
        args.extend(["--assert", directive]);
        let out = kuna("decompile", file.path(), &args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(out.status.success() && stderr.contains(reason), "{stderr}");
        assert_eq!(out.stdout, plain.stdout);
        args.push("--assert-strict");
        assert!(!kuna("decompile", file.path(), &args).status.success());
    }
    let fits = raw_jump("x86:LE:32:default", &ax32, "jumptable 100 103 109");
    let body = String::from_utf8_lossy(&fits.stdout);
    assert!(fits.status.success() && body.contains("case 0x103:") && body.contains("case 0x109:"), "{body}");

    // jr t9; nop; li v0,11; jr ra; nop; li v0,22; jr ra; nop
    let mips: Vec<_> = [0x03200008u32, 0, 0x2402000b, 0x03e00008, 0, 0x24020016, 0x03e00008, 0]
        .into_iter()
        .flat_map(u32::to_be_bytes)
        .collect();
    let standard = raw_jump("MIPS:BE:32:micro", &mips, "jumptable 100 108 114");
    let body = String::from_utf8_lossy(&standard.stdout);
    assert!(body.contains("case 0x108:") && body.contains("case 0x114:"), "{body}");
    for compressed in ["10a", "10b"] {
        let out = raw_jump("MIPS:BE:32:micro", &mips, &format!("jumptable 100 108 {compressed}"));
        assert!(String::from_utf8_lossy(&out.stderr).contains("4-byte instruction alignment"));
    }
}
