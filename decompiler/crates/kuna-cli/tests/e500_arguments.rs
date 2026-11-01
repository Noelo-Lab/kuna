use crate::common;
use common::process;
use std::process::Command;

const FRAME_CALL: &[u32] = &[
    0x9421fff0, 0x7c0802a6, 0x90010014, 0x7c6c1b78, 0x7c832378, 0x7d8903a6, 0x4e800421, 0x80010014,
    0x7c0803a6, 0x38210010, 0x4e800020,
];

fn decompile(words: &[u32], little: bool, variant: &str, overlap: Option<&str>) -> String {
    let path = common::scratch_file("e500-call-slots", "bin");
    let bytes: Vec<_> = words
        .iter()
        .flat_map(|word| {
            if little {
                word.to_le_bytes()
            } else {
                word.to_be_bytes()
            }
        })
        .collect();
    std::fs::write(&path, bytes).unwrap();
    let target = format!("PowerPC:{}:32:{variant}", if little { "LE" } else { "BE" });
    let mut args = vec![
        "decompile",
        path.to_str().unwrap(),
        "0x1000",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        &target,
        "--mode",
        "fast",
    ];
    if let Some(overlap) = overlap {
        args.extend(["--option", "calloverlap", overlap]);
    }
    let (out, err, rc) = common::run_kuna(&args);
    std::fs::remove_file(path).unwrap();
    assert_eq!(rc, 0, "{err}\n{out}");
    out
}

// Supply the independently known callback type; execute the recovered body unchanged.
fn execute(code: &str, harness: &str) {
    if process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("cc unavailable; skipping only emitted-C execution");
        return;
    }
    let source = common::scratch_file("e500-call-execute", "c");
    let binary = common::scratch_file("e500-call-execute", "exe");
    std::fs::write(&source, format!("{code}\n{harness}\n")).unwrap();
    process::required_output(
        Command::new("cc")
            .args(["-O2", "-Werror"])
            .arg(&source)
            .arg("-o")
            .arg(&binary),
    );
    process::required_output(&mut Command::new(&binary));
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(binary).unwrap();
}

#[test]
fn untyped_e500_calls_keep_scalar_arguments() {
    // Indirect calls through CTR: constant, arithmetic, load, and frame variants.
    let cases: &[(&[u32], &str)] = &[
        (
            &[0x7c6c1b78, 0x7d8903a6, 0x38600007, 0x4e800420],
            "(*a0)(7)",
        ),
        (
            &[0x7c6c1b78, 0x7d8903a6, 0x38640007, 0x4e800420],
            "(*a0)(a1 + 7)",
        ),
        (
            &[0x7c6c1b78, 0x7d8903a6, 0x80640000, 0x4e800420],
            "(*a0)(*a1)",
        ),
        (FRAME_CALL, "(*a0)(a1)"),
    ];
    for little in [false, true] {
        for &(words, expected) in cases {
            let old = decompile(words, little, "e500", Some("off"));
            assert!(old.contains("(*a0)()"), "{old}");
            for overlap in [None, Some("auto"), Some("full")] {
                let code = decompile(words, little, "e500", overlap);
                assert!(code.contains(expected), "{code}");
                assert!(!code.contains("CONCAT"), "{code}");
            }
        }
    }
}

#[test]
fn e500_call_result_is_defined_by_the_call() {
    let mut words = FRAME_CALL[..FRAME_CALL.len() - 1].to_vec();
    words.extend([0x38630001, 0x4e800020]); // addi r3,r3,1; blr.
    for little in [false, true] {
        let old = decompile(&words, little, "e500", Some("off"));
        assert!(old.contains("(*a0)()"), "{old}");
        let code = decompile(&words, little, "e500", None);
        assert!(code.contains("(*a0)(a1)"), "{code}");
        assert!(!code.contains("// _r3"), "{code}");
        execute(&code.replace("void *a0", "int (*a0)(int)"),
            "static int cb(int x) { return x * 3; }\nint main(void) { for (int x=-10;x<=10;++x) if (sub_1000(cb,x)!=x*3+1) return 1; return 0; }");
    }
}

#[test]
fn ordinary_powerpc_registers_keep_existing_call_recovery() {
    for little in [false, true] {
        for variant in ["default", "e500mc"] {
            let off = decompile(FRAME_CALL, little, variant, Some("off"));
            let auto = decompile(FRAME_CALL, little, variant, Some("auto"));
            assert_eq!(auto, off);
            assert!(auto.contains("(*a0)(a1)"), "{auto}");
        }
    }
}

#[test]
fn spe_values_keep_both_words_across_a_scalar_call() {
    // Save r14; evldd r14,0(r4); call r3(7); evstdd r14,0(r15); restore.
    let words = [
        0x9421ffe0, 0x7c0802a6, 0x90010024, 0x11c10b21, 0x91e10010, 0x7caf2b78, 0x11c40301,
        0x7c6903a6, 0x38600007, 0x4e800421, 0x11cf0321, 0x11c10b01, 0x81e10010, 0x80010024,
        0x7c0803a6, 0x38210020, 0x4e800020,
    ];
    for little in [false, true] {
        let code = decompile(&words, little, "e500", None);
        assert!(
            code.contains("unsigned long long *a1,unsigned long long *a2"),
            "{code}"
        );
        assert!(code.contains("v1 = *a1;"), "{code}");
        assert!(code.contains("(*a0)(7);"), "{code}");
        assert!(code.contains("*a2 = v1;"), "{code}");
        assert!(!code.contains("CONCAT"), "{code}");
        execute(&code.replace("void *a0", "void (*a0)(int)"),
            "static unsigned long long input,output; static int calls;\nstatic void cb(int x) { calls+=x; input=0; }\nint main(void) { const unsigned long long values[]={0x1234567887654321ULL,0xffffffff00000000ULL,0x76543210ffffffffULL}; for(int i=0;i<3;++i) { input=values[i]; output=0; calls=0; sub_1000(cb,&input,&output); if(output!=values[i]||input!=0||calls!=7) return 1; } return 0; }");
    }
}
