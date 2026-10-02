mod common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::process::Command;

// Authored ARM instructions implement narrow(x), use_narrow(x), cast_only(x),
// mixed_input(x,y), multiple(n,x,y), and its forwarding caller. No toolchain is needed.
fn image(abi: Option<u8>) -> Vec<u8> {
    let words: [u32; 34] = [
        0xee300b00, 0xeeb70bc0, 0xe12fff1e, 0xe92d4010, 0xebfffffa, 0xeef77a00, 0xee300a27,
        0xe8bd8010, 0xeeb70bc0, 0xe12fff1e, 0xeeb07b40, 0xeeb00a41, 0xeeb57bc0, 0xeef1fa10,
        0xc12fff1e, 0xee277b07, 0xeeb70bc7, 0xe12fff1e, 0xee070a10, 0xeeb87bc7, 0xee370b00,
        0xee300b01, 0xe12fff1e, 0xe92d4010, 0xebfffff8, 0xe8bd8010, 0xe12fff1e, 0xe92d4010,
        0xebffffe2, 0xeeb70ac0, 0xebffffe0, 0xeef77a00, 0xee300a27, 0xe8bd8010,
    ];
    let functions = [
        ("narrow", 0, 12),
        ("use_narrow", 12, 20),
        ("cast_only", 32, 8),
        ("mixed_input", 40, 32),
        ("multiple", 72, 20),
        ("use_multiple", 92, 12),
        ("unknown", 104, 4),
        ("after_clobber", 108, 28),
    ];
    image_from_words(&words, &functions, abi)
}

fn image_from_words(words: &[u32], functions: &[(&str, u64, u64)], abi: Option<u8>) -> Vec<u8> {
    let mut obj = Object::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
    let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    obj.append_section_data(
        text,
        &words
            .iter()
            .copied()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>(),
        4,
    );
    for &(name, value, size) in functions {
        obj.add_symbol(Symbol {
            name: name.as_bytes().to_vec(),
            value,
            size,
            kind: SymbolKind::Text,
            scope: SymbolScope::Linkage,
            weak: false,
            section: SymbolSection::Section(text),
            flags: SymbolFlags::None,
        });
    }
    if let Some(value) = abi {
        let section = obj.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
        let mut bytes = b"A\x11\0\0\0aeabi\0\x01\x07\0\0\0\x1c".to_vec();
        bytes.push(value);
        obj.append_section_data(section, &bytes, 1);
    }
    obj.write().unwrap()
}

fn decompile(bytes: &[u8], option: &str) -> String {
    let path = common::scratch_file("arm-float-inputs", "o");
    std::fs::write(&path, bytes).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            path.to_str().unwrap(),
            "--mode",
            "aggressive",
            "--option",
            "armfloatreturn",
            "on",
            "--option",
            "armfloatargs",
            option,
            "--option",
            "protoorder",
            "types",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn decompile_with(bytes: &[u8], options: &[&str]) -> String {
    let path = common::scratch_file("arm-float-inputs-with", "o");
    std::fs::write(&path, bytes).unwrap();
    let mut args = vec![
        "decompile-all",
        path.to_str().unwrap(),
        "--mode",
        "aggressive",
    ];
    for pair in options.chunks(2) {
        args.extend(["--option", pair[0], pair[1]]);
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(&args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

// clang -O2 hard-float ARM output of bf(float,double,float), dfi(double,float,int)
// and use3(n,x,y) = dfi(x,y,n) + bf(y,x,y*2): the bf arguments are written into
// s0, s1 and d1 over the d0 result of dfi.
fn overlapping_result_image() -> Vec<u8> {
    image_from_words(
        &[
            0xeeb02a08, 0xeeb71bc1, 0xee001a02, 0xee310a60, 0xe12fff1e, 0xee020a10, 0xeeb82ac2,
            0xee221a01, 0xeeb71ac1, 0xee300b41, 0xe12fff1e, 0xe92d4800, 0xed2d8b06, 0xeeb08a41,
            0xeeb09b40, 0xebfffff4, 0xeeb0ab40, 0xee780a08, 0xeeb00a48, 0xeeb01b49, 0xebffffea,
            0xeeb70ac0, 0xee3a0b00, 0xecbd8b06, 0xe8bd8800,
        ],
        &[("bf", 0, 20), ("dfi", 20, 24), ("use3", 44, 56)],
        Some(1),
    )
}

// clang -O2 hard-float ARM output of chk(x): x goes to a base-AAPCS helper in r0 and
// r1 and is returned, so no instruction reads d0 whole.
fn word_input_image() -> Vec<u8> {
    image_from_words(
        &[
            0xe92d4800, 0xed2d8b02, 0xec510b10, 0xeeb08b40, 0xeb000006, 0xe3500000, 0x0a000001,
            0xe3a00022, 0xeb000003, 0xeeb00b48, 0xecbd8b02, 0xe8bd8800, 0xe12fff1e, 0xe12fff1e,
        ],
        &[("chk", 0, 48), ("isbad", 48, 4), ("set_errno", 52, 4)],
        Some(1),
    )
}

#[test]
fn single_arguments_written_over_a_double_result_are_read_as_written() {
    let code = decompile(&overlapping_result_image(), "on");
    for expected in [
        "float bf(float a0,float a1,double a2)",
        "double use3(double a0,float a1,int a2)",
        "v1 = dfi(a0,a1,a2);",
        "return v1 + (double)bf(a1,a1 + a1,a0);",
    ] {
        assert!(code.contains(expected), "missing {expected}\n{code}");
    }
    assert!(!code.contains("SUB84"), "{code}");
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let source = common::scratch_file("arm-float-overlap", "c");
    let executable = common::scratch_file("arm-float-overlap", "exe");
    std::fs::write(
        &source,
        format!("{code}\nint main(void) {{ return use3(2.0, 1.5f, 3) != 1.0; }}\n"),
    )
    .unwrap();
    let compile = Command::new("cc")
        .args(["-std=c11", "-O2", "-Werror", "-o"])
        .arg(&executable)
        .arg(&source)
        .env("TMPDIR", source.parent().unwrap())
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "{}\n{code}",
        String::from_utf8_lossy(&compile.stderr)
    );
    assert!(
        Command::new(executable).status().unwrap().success(),
        "{code}"
    );
}

#[test]
fn armfloatargs_without_armfloatreturn_changes_nothing() {
    for bytes in [image(Some(1)), word_input_image(), backfill_image(false)] {
        assert_eq!(
            decompile_with(&bytes, &["armfloatargs", "on"]),
            decompile_with(&bytes, &["armfloatargs", "off"])
        );
    }
}

#[test]
fn a_double_read_only_as_integer_words_is_left_to_armfloatreturn() {
    let on = decompile(&word_input_image(), "on");
    assert_eq!(on, decompile(&word_input_image(), "off"));
    // Cortex-M4F (single-precision FPU) at -O0: scale(int *, double, int)
    // hands its double to __aeabi_dmul as two words, and use() calls it.
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/armfloatargs_m4f.o");
    let bytes = std::fs::read(fixture).unwrap();
    let on = decompile(&bytes, "on");
    assert!(
        !on.contains("float scale(unsigned int a0,unsigned int a1,int *a2"),
        "{on}"
    );
    assert_eq!(on, decompile(&bytes, "off"));
}

/// The argument count of every call to `name` in an indented statement.
fn call_arities(code: &str, name: &str) -> Vec<usize> {
    let pattern = format!("{name}(");
    let skip = pattern.len();
    code.lines()
        .filter(|line| line.starts_with(' '))
        .flat_map(|line| {
            line.match_indices(pattern.as_str())
                .map(move |(at, _)| &line[at + skip..])
                .collect::<Vec<_>>()
        })
        .map(|args| {
            let (mut depth, mut count, mut any) = (0, 1, false);
            for c in args.chars() {
                match c {
                    '(' => depth += 1,
                    ')' if depth == 0 => break,
                    ')' => depth -= 1,
                    ',' if depth == 0 => count += 1,
                    _ => {}
                }
                any |= !c.is_whitespace();
            }
            if any {
                count
            } else {
                0
            }
        })
        .collect()
}

// clang -O0 and -O2 hard-float ARM output of g1(float,float,float,double) and
// top(n) = g1(n*1.0f, n*2.0f, n*3.0f, n*4.0). The double goes to d2, so s3 is a
// back-fill slot no parameter occupies; at -O2 the caller also leaves 3.0f in s6.
fn backfill_image(optimized: bool) -> Vec<u8> {
    if optimized {
        image_from_words(
            &[
                0xee300a60, 0xeeb71ac1, 0xeeb70ac0, 0xee010b02, 0xe12fff1e, 0xe92d4800, 0xee000a10,
                0xeeb12b00, 0xeeb81bc0, 0xeeb80ac0, 0xeeb03a08, 0xee212b02, 0xee201a03, 0xee700a00,
                0xebfffff0, 0xeebd0bc0, 0xee100a10, 0xe8bd8800,
            ],
            &[("g1", 0, 20), ("top", 20, 52)],
            Some(1),
        )
    } else {
        image_from_words(
            &[
                0xe24dd018, 0xed8d0a05, 0xedcd0a04, 0xed8d1a03, 0xed8d2b00, 0xed9d0a05, 0xed9d1a04,
                0xee300a41, 0xeeb70ac0, 0xed9d1a03, 0xeeb71ac1, 0xed9d2b00, 0xee010b02, 0xe28dd018,
                0xe12fff1e, 0xe92d4800, 0xe1a0b00d, 0xe24dd008, 0xe58d0004, 0xe59d0004, 0xee000a10,
                0xeeb80ac0, 0xeeb71a00, 0xee200a01, 0xe59d0004, 0xee010a10, 0xeeb81ac1, 0xeeb02a00,
                0xee610a02, 0xe59d0004, 0xee010a10, 0xeeb81ac1, 0xeeb02a08, 0xee211a02, 0xe59d0004,
                0xee020a10, 0xeeb82bc2, 0xeeb13b00, 0xee222b03, 0xebffffd7, 0xeebd0bc0, 0xee100a10,
                0xe1a0d00b, 0xe8bd8800,
            ],
            &[("g1", 0, 60), ("top", 60, 116)],
            Some(1),
        )
    }
}

#[test]
fn a_backfill_slot_below_a_double_is_neither_a_parameter_nor_an_argument() {
    for optimized in [false, true] {
        let code = decompile(&backfill_image(optimized), "on");
        for expected in [
            "double g1(float a0,float a1,float a2,double a3)",
            "unsigned int top(int a0)",
        ] {
            assert!(code.contains(expected), "missing {expected}\n{code}");
        }
        assert_eq!(call_arities(&code, "g1"), [4], "{code}");
        let off = decompile(&backfill_image(optimized), "off");
        assert!(
            off.contains("double g1(unsigned long long a0,float a1,float a2,double a3)"),
            "{off}"
        );
        if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
            continue;
        }
        let source = common::scratch_file("arm-float-backfill", "c");
        let executable = common::scratch_file("arm-float-backfill", "exe");
        std::fs::write(
            &source,
            format!(
                "{code}\nint main(void) {{ return g1(1.0f, 2.0f, 3.0f, 4.0) != 11.0 || top(3) != 105; }}\n"
            ),
        )
        .unwrap();
        let compile = Command::new("cc")
            .args(["-std=c11", "-O2", "-Werror", "-o"])
            .arg(&executable)
            .arg(&source)
            .env("TMPDIR", source.parent().unwrap())
            .output()
            .unwrap();
        assert!(
            compile.status.success(),
            "{}\n{code}",
            String::from_utf8_lossy(&compile.stderr)
        );
        assert!(
            Command::new(executable).status().unwrap().success(),
            "{code}"
        );
    }
}

// clang 14 hard-float ARM and Thumb builds of `armfloatargs_calls.c`: a wrapper
// that passes its double on in another slot (x3), a non-leaf callee with a
// back-fill slot (v2), an unused float an -O0 callee spills (fn5) and an
// unused double ahead of a used one (yd), all called from `top`.
#[test]
fn calls_take_exactly_the_vfp_inputs_their_callee_states() {
    let fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures");
    let has_cc = common::process::optional_output(Command::new("cc").arg("--version")).is_some();
    let driver = common::scratch_file("arm-float-calls-driver", "c");
    std::fs::write(
        &driver,
        "#include <stdio.h>\nint top(int);\nint main(void) { long s = 0;\n\
         for (int n = 1; n < 9; n++) s = s * 31 + top(n);\nprintf(\"%ld\\n\", s); return 0; }\n",
    )
    .unwrap();
    let expected = has_cc.then(|| {
        let reference = common::scratch_file("arm-float-calls-reference", "exe");
        compile_and_run(
            &[fixtures.join("armfloatargs_calls.c"), driver.clone()],
            &reference,
        )
    });
    for build in ["arm-O0", "arm-O2", "thumb-O0", "thumb-O2", "thumb-Os"] {
        let bytes = std::fs::read(fixtures.join(format!("armfloatargs_calls_{build}.o"))).unwrap();
        let code = decompile_with(&bytes, &["armfloatreturn", "on", "armfloatargs", "on"]);
        let spilled = build.ends_with("O0");
        let fn5 = if spilled {
            ("fn5", "double fn5(float a0,float a1,double a2)", 3)
        } else {
            ("fn5", "double fn5(float a0,double a1)", 2)
        };
        for (name, signature, arity) in [
            ("x3", "double x3(float a0,float a1,float a2,double a3)", 4),
            ("v2", "double v2(float a0,float a1,float a2,double a3)", 4),
            ("yd", "double yd(float a0,double a1,double a2)", 3),
            fn5,
        ] {
            assert!(
                code.contains(signature),
                "{build}: missing {signature}\n{code}"
            );
            assert_eq!(
                call_arities(&code, name),
                [arity],
                "{build}: {name}\n{code}"
            );
        }
        if spilled {
            assert!(
                code.contains("(float)(long long)a0 * 3.0,"),
                "{build}\n{code}"
            );
        }
        let Some(expected) = &expected else {
            continue;
        };
        let emitted = common::scratch_file("arm-float-calls-emitted", "c");
        let executable = common::scratch_file("arm-float-calls-emitted", "exe");
        std::fs::write(&emitted, &code).unwrap();
        assert_eq!(
            &compile_and_run(&[emitted, driver.clone()], &executable),
            expected,
            "{build}\n{code}"
        );
    }
}

// clang 14 hard-float ARM and Thumb builds of `armfloatargs_words.c`, whose
// wrappers forward their double to `wl` in d0 while heritage reads d0 as two
// words, and `armfloatargs_hole.c`, whose `top` leaves `hole6`'s back-fill
// slot s7 unwritten before the call.
#[test]
fn stated_doubles_pass_whole_and_backfill_holes_stay_empty() {
    let fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures");
    let has_cc = common::process::optional_output(Command::new("cc").arg("--version")).is_some();
    let driver = common::scratch_file("arm-float-words-driver", "c");
    std::fs::write(
        &driver,
        "#include <stdio.h>\nint top(int);\nint main(void) { long s = 0;\n\
         for (int n = 1; n < 9; n++) s = s * 31 + top(n);\nprintf(\"%ld\\n\", s); return 0; }\n",
    )
    .unwrap();
    let cases: [(&str, &[(&str, &str, &[usize])]); 2] = [
        (
            "armfloatargs_words",
            &[
                ("wl", "double wl(double a0,float a1)", &[2, 2, 2]),
                ("w8", "float w8(double a0)", &[1]),
                ("w10", "float w10(double a0,float a1)", &[2]),
            ],
        ),
        (
            "armfloatargs_hole",
            &[
                (
                    "hole6",
                    "double hole6(double a0,double a1,double a2,float a3,double a4,double a5)",
                    &[6],
                ),
                (
                    "fill6",
                    "double fill6(double a0,double a1,double a2,float a3,float a4,float a5)",
                    &[6],
                ),
                ("top", "top(int a0)\n", &[]),
            ],
        ),
    ];
    for (source, checks) in cases {
        let expected = has_cc.then(|| {
            let reference = common::scratch_file(&format!("{source}-reference"), "exe");
            compile_and_run(&[fixtures.join(format!("{source}.c")), driver.clone()], &reference)
        });
        for build in ["arm-O2", "thumb-O2", "thumb-Os"] {
            let bytes = std::fs::read(fixtures.join(format!("{source}_{build}.o"))).unwrap();
            let code = decompile_with(&bytes, &["armfloatreturn", "on", "armfloatargs", "on"]);
            assert!(!code.contains("SUB84"), "{source} {build}\n{code}");
            for (name, signature, arities) in checks {
                assert!(
                    code.contains(signature),
                    "{source} {build}: missing {signature}\n{code}"
                );
                assert_eq!(
                    &call_arities(&code, name),
                    arities,
                    "{source} {build}: {name}\n{code}"
                );
            }
            let Some(expected) = &expected else {
                continue;
            };
            let emitted = common::scratch_file(&format!("{source}-emitted"), "c");
            let executable = common::scratch_file(&format!("{source}-emitted"), "exe");
            std::fs::write(&emitted, &code).unwrap();
            assert_eq!(
                &compile_and_run(&[emitted, driver.clone()], &executable),
                expected,
                "{source} {build}\n{code}"
            );
        }
    }
}

// clang 14 hard-float ARM and Thumb builds of `armfloatargs_ignored.c`, whose
// callees ignore a leading double: the wrappers leave their own float in s0, so
// d0 at the call is that float plus an s1 they never write (w3 as two words,
// u1/u2/u3 as one d0 read). `top` also leaves q2's float result in s0 and the
// s1 above it for k3, which never reads its second parameter, and wk calls kd,
// which ignores its leading double, holding no value of its own for s1.
#[test]
fn ignored_stated_doubles_keep_the_callers_float_parameters() {
    let fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../kuna-analysis/tests/fixtures");
    let has_cc = common::process::optional_output(Command::new("cc").arg("--version")).is_some();
    let driver = common::scratch_file("arm-float-ignored-driver", "c");
    std::fs::write(
        &driver,
        "#include <stdio.h>\nint top(int);\nint main(void) { long s = 0;\n\
         for (int n = 1; n < 9; n++) s = s * 31 + top(n);\nprintf(\"%ld\\n\", s); return 0; }\n",
    )
    .unwrap();
    let expected = has_cc.then(|| {
        let reference = common::scratch_file("arm-float-ignored-reference", "exe");
        compile_and_run(
            &[fixtures.join("armfloatargs_ignored.c"), driver.clone()],
            &reference,
        )
    });
    let checks: [(&str, &str, &[usize]); 11] = [
        ("i4", "double i4(double a0,float a1,float a2,double a3)", &[4]),
        ("i2", "double i2(double a0,double a1)", &[2, 2]),
        ("i3", "double i3(double a0,float a1,double a2)", &[3]),
        ("w3", "float w3(float a0)", &[1]),
        ("u1", "float u1(float a0)", &[1]),
        ("u2", "float u2(float a0,float a1)", &[2]),
        ("u3", "float u3(float a0,double a1)", &[2]),
        ("q2", "float q2(float a0,float a1)", &[2]),
        ("k3", "float k3(float a0,float a1,float a2)", &[3]),
        ("gk", "float gk(float a0)", &[1]),
        ("wk", "float wk(float a0)", &[1]),
    ];
    for build in ["arm-O1", "arm-O2", "thumb-O2", "thumb-Os"] {
        let bytes =
            std::fs::read(fixtures.join(format!("armfloatargs_ignored_{build}.o"))).unwrap();
        let code = decompile_with(&bytes, &["armfloatreturn", "on", "armfloatargs", "on"]);
        assert!(
            !code.contains("SUB84") && !code.contains("CONCAT44") && !code.contains(">> 0x20"),
            "{build}\n{code}"
        );
        for (name, signature, arities) in checks {
            assert!(
                code.contains(signature),
                "{build}: missing {signature}\n{code}"
            );
            assert_eq!(&call_arities(&code, name), arities, "{build}: {name}\n{code}");
        }
        let Some(expected) = &expected else {
            continue;
        };
        let emitted = common::scratch_file("arm-float-ignored-emitted", "c");
        let executable = common::scratch_file("arm-float-ignored-emitted", "exe");
        std::fs::write(&emitted, &code).unwrap();
        assert_eq!(
            &compile_and_run(&[emitted, driver.clone()], &executable),
            expected,
            "{build}\n{code}"
        );
    }
}

/// Compile `sources` with the host compiler and return what the program prints.
fn compile_and_run(sources: &[std::path::PathBuf], executable: &std::path::Path) -> String {
    let compile = Command::new("cc")
        .args(["-std=c11", "-O0", "-w", "-o"])
        .arg(executable)
        .args(sources)
        .env("TMPDIR", executable.parent().unwrap())
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "{}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let run = Command::new(executable).output().unwrap();
    assert!(run.status.success());
    String::from_utf8(run.stdout).unwrap()
}

// Authored ARM instructions: narrowing callees, tail/ordinary wrappers, and
// callers forwarding d0 plus d1 or s2 before consuming the s0 result.
fn forwarding_image() -> Vec<u8> {
    image_from_words(
        &[
            0xee300b00, 0xeeb70bc0, 0xe12fff1e, 0xeafffffb, 0xe92d4010, 0xebfffff9, 0xe8bd8010,
            0xee300b01, 0xeeb70bc0, 0xe12fff1e, 0xe92d4010, 0xebfffffa, 0xeef77a00, 0xee300a27,
            0xe8bd8010, 0xeeb71ac1, 0xee300b01, 0xeeb70bc0, 0xe12fff1e, 0xe92d4010, 0xebfffff9,
            0xeef77a00, 0xee300a27, 0xe8bd8010,
        ],
        &[
            ("narrow", 0, 12),
            ("tail_wrapper", 12, 4),
            ("call_wrapper", 16, 12),
            ("pair_narrow", 28, 12),
            ("use_pair_narrow", 40, 20),
            ("mixed_narrow", 60, 16),
            ("use_mixed_narrow", 76, 20),
        ],
        Some(1),
    )
}

#[test]
fn narrowing_wrappers_preserve_tail_and_ordinary_call_results() {
    let code = decompile(&forwarding_image(), "on");
    for name in ["tail_wrapper", "call_wrapper"] {
        assert!(code.contains(&format!("float {name}(double a0)")), "{code}");
    }
    assert_eq!(code.matches("return narrow(a0);").count(), 2, "{code}");
}

#[test]
fn narrowing_calls_keep_additional_double_and_float_inputs() {
    let code = decompile(&forwarding_image(), "on");
    for expected in [
        "float pair_narrow(double a0,double a1)",
        "float use_pair_narrow(double a0,double a1)",
        "return pair_narrow(a0,a1) + 1.0;",
        "float mixed_narrow(double a0,float a1)",
        "float use_mixed_narrow(double a0,float a1)",
        "return mixed_narrow(a0,a1) + 1.0;",
    ] {
        assert!(code.contains(expected), "missing {expected}\n{code}");
    }
}

#[test]
fn narrowing_forwarded_contracts_compile_and_preserve_values() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let code = decompile(&forwarding_image(), "on");
    let source = common::scratch_file("arm-narrow-forwarded", "c");
    let executable = common::scratch_file("arm-narrow-forwarded", "exe");
    std::fs::write(
        &source,
        format!(
            "{code}\nint main(void) {{\n\
        if (tail_wrapper(1.25) != 2.5f || call_wrapper(1.5) != 3.0f) return 1;\n\
        if (pair_narrow(1.25,2.5) != 3.75f || use_pair_narrow(1.25,2.5) != 4.75f) return 2;\n\
        if (mixed_narrow(1.25,2.5f) != 3.75f || use_mixed_narrow(1.25,2.5f) != 4.75f) return 3;\n\
        return 0;\n}}\n"
        ),
    )
    .unwrap();
    let compile = Command::new("cc")
        .args(["-std=c11", "-O2", "-Werror", "-o"])
        .arg(&executable)
        .arg(&source)
        .env("TMPDIR", source.parent().unwrap())
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "{}\n{code}",
        String::from_utf8_lossy(&compile.stderr)
    );
    assert!(
        Command::new(executable).status().unwrap().success(),
        "{code}"
    );
}

#[test]
fn scalar_inputs_and_overlapping_call_results_keep_their_widths() {
    let code = decompile(&image(Some(1)), "on");
    for expected in [
        "float narrow(double a0)",
        "float use_narrow(double a0)",
        "return narrow(a0) + 1.0;",
        "float after_clobber(double a0)",
        "float cast_only(double a0)",
        "return (float)a0;",
        "float mixed_input(double a0,float a1)",
        "return (float)(a0 * a0);",
        "double multiple(double a0,double a1,int a2)",
        "double use_multiple(double a0,double a1,int a2)",
        "return multiple(a0,a1,a2);",
    ] {
        assert!(code.contains(expected), "missing {expected}\n{code}");
    }
    assert!(!code.contains("CONCAT44"), "{code}");
    assert!(code.contains("void unknown(void)"), "{code}");
    let off = decompile(&image(Some(1)), "off");
    assert!(
        off.contains("float cast_only(unsigned int a0,unsigned int a1)"),
        "{off}"
    );
    assert!(off.contains("CONCAT44"), "{off}");
}

#[test]
fn absent_soft_and_ambiguous_abis_are_unchanged() {
    for abi in [None, Some(0), Some(2), Some(3)] {
        assert_eq!(decompile(&image(abi), "off"), decompile(&image(abi), "on"));
    }
}

#[test]
fn declared_cross_bank_order_outranks_recovery() {
    let path = common::scratch_file("arm-float-declared", "o");
    std::fs::write(&path, image(Some(1))).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            path.to_str().unwrap(),
            "--mode",
            "aggressive",
            "--option",
            "armfloatreturn",
            "on",
            "--option",
            "armfloatargs",
            "on",
            "--assert",
            "prototype multiple double multiple(int n,double x,double y)",
            "--assert-strict",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let code = String::from_utf8(output.stdout).unwrap();
    assert!(
        code.contains("double multiple(int n,double x,double y)"),
        "{code}"
    );
    assert!(
        code.contains("return (double)(long long)n + x + y;"),
        "{code}"
    );
    assert!(code.contains("multiple(a2,a0,a1)"), "{code}");
}

#[test]
fn recovered_scalar_contracts_compile_and_compute_the_expected_values() {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        return;
    }
    let code = decompile(&image(Some(1)), "on");
    let source = common::scratch_file("arm-float-roundtrip", "c");
    let executable = common::scratch_file("arm-float-roundtrip", "exe");
    std::fs::write(
        &source,
        format!(
            "{code}\nint main(void) {{\n\
        if (narrow(1.5) != 3.0f || use_narrow(1.5) != 4.0f || after_clobber(1.5) != 7.0f) return 1;\n\
        if (cast_only(2.25) != 2.25f) return 2;\n\
        if (mixed_input(2.0, 7.5f) != 7.5f || mixed_input(-2.0, 7.5f) != 4.0f) return 3;\n\
        if (multiple(1.25, 2.5, 7) != 10.75 || use_multiple(1.25, 2.5, 7) != 10.75) return 4;\n\
        return 0;\n}}\n"
        ),
    )
    .unwrap();
    let compile = Command::new("cc")
        .args(["-std=c11", "-O2", "-Werror", "-o"])
        .arg(&executable)
        .arg(&source)
        .env("TMPDIR", source.parent().unwrap())
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "{}\n{code}",
        String::from_utf8_lossy(&compile.stderr)
    );
    assert!(
        Command::new(executable).status().unwrap().success(),
        "{code}"
    );
}

#[test]
fn mixed_input_banks_and_variadic_base_storage_agree() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/fmtlf_armhf");
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "decompile-all",
            fixture.to_str().unwrap(),
            "--mode",
            "aggressive",
            "--option",
            "armfloatreturn",
            "on",
            "--option",
            "armfloatargs",
            "on",
            "--option",
            "formatstring",
            "full",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let code = String::from_utf8(output.stdout).unwrap();
    assert!(code.contains("void show2(double a0,int a1)"), "{code}");
    assert!(
        code.contains("__printf_chk(2,\"n=%d v=%lf\\n\",a1,a0);"),
        "{code}"
    );
    assert!(
        code.contains("show2((double)(long long)a0 + (double)(long long)a0,a0);")
            || (code.contains("v2 = (double)(long long)a0;")
                && code.contains("show2(v2 + v2,a0);")),
        "{code}"
    );
}

#[test]
fn compiled_arm_and_thumb_sources_preserve_mixed_values_and_declared_order_controls() {
    if common::process::optional_output(Command::new("arm-linux-gnueabihf-gcc").arg("--version"))
        .is_none()
    {
        eprintln!("compiled_arm_and_thumb_sources: skipping (no `arm-linux-gnueabihf-gcc`)");
        return;
    }
    let source = common::scratch_file("arm-float-mixed-source", "c");
    std::fs::write(&source, include_str!("arm_float_arguments.c")).unwrap();
    for mode in ["-marm", "-mthumb"] {
        for level in ["-O1", "-O2", "-O3", "-Os"] {
            let image = common::scratch_file("arm-float-mixed-linked", "so");
            let compile = Command::new("arm-linux-gnueabihf-gcc")
                .args([
                    level,
                    mode,
                    "-mfpu=vfpv3-d16",
                    "-fno-ipa-ra",
                    "-fno-optimize-sibling-calls",
                    "-shared",
                    "-fPIC",
                    "-Wl,-Bsymbolic",
                    "-o",
                ])
                .arg(&image)
                .arg(&source)
                .env("TMPDIR", source.parent().unwrap())
                .output()
                .unwrap();
            assert!(
                compile.status.success(),
                "{}",
                String::from_utf8_lossy(&compile.stderr)
            );
            let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
                .args([
                    "decompile-all",
                    image.to_str().unwrap(),
                    "--mode",
                    "aggressive",
                    "--option",
                    "armfloatreturn",
                    "on",
                    "--option",
                    "armfloatargs",
                    "on",
                    "--option",
                    "formatstring",
                    "full",
                ])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let code = String::from_utf8(output.stdout).unwrap();
            for expected in [
                "void show2(double a0,int a1)",
                "void reverse(double a0,int a1)",
                "void caller(double a0,int a1)",
                "show2(a0,a1);",
                "void reverse_caller(double a0,int a1)",
                "reverse(a0,a1);",
                "printf(\"%d %lf\\n\",a1,a0);",
                "float wrap_narrow(double a0)",
                "return narrow(a0);",
                "float pair_narrow(double a0,double a1)",
                "float use_pair_narrow(double a0,double a1)",
                "return pair_narrow(a0,a1) + 1.0;",
                "float mixed_narrow(double a0,float a1)",
                "float use_mixed_narrow(double a0,float a1)",
                "return mixed_narrow(a0,a1) + 1.0;",
                "float use_narrow(double a0)",
                "narrow(a0)",
                "float cast_only(double a0)",
                "float mixed_input(double a0,float a1)",
            ] {
                assert!(
                    code.contains(expected),
                    "{mode} {level}: missing {expected}\n{code}"
                );
            }
            assert!(!code.contains("(error:"), "{mode} {level}\n{code}");
            if common::process::optional_output(Command::new("cc").arg("--version")).is_some() {
                let functions: String = code
                    .split("// Function: ")
                    .filter(|part| {
                        [
                            "show2",
                            "reverse",
                            "caller",
                            "reverse_caller",
                            "narrow",
                            "wrap_narrow",
                            "pair_narrow",
                            "use_pair_narrow",
                            "mixed_narrow",
                            "use_mixed_narrow",
                        ]
                        .iter()
                        .any(|name| part.starts_with(&format!("{name} @")))
                    })
                    .filter_map(|part| part.split_once('\n').map(|(_, body)| body))
                    .collect();
                let emitted = common::scratch_file("arm-float-mixed-emitted", "c");
                let executable = common::scratch_file("arm-float-mixed-emitted", "exe");
                std::fs::write(&emitted, format!("#include <stdio.h>\n{functions}\n\
                    int main(void) {{ show2(2.5,7); reverse(3.5,8); caller(4.5,9); reverse_caller(5.5,10);
                    if (wrap_narrow(1.5) != 3.0f) return 1;
                    if (pair_narrow(1.25,2.5) != 3.75f || use_pair_narrow(1.25,2.5) != 4.75f) return 2;
                    if (mixed_narrow(1.25,2.5f) != 3.75f || use_mixed_narrow(1.25,2.5f) != 4.75f) return 3;
                    return 0; }}\n")).unwrap();
                let compile = Command::new("cc")
                    .args(["-std=c11", "-O2", "-Werror", "-o"])
                    .arg(&executable)
                    .arg(&emitted)
                    .env("TMPDIR", source.parent().unwrap())
                    .output()
                    .unwrap();
                assert!(
                    compile.status.success(),
                    "{}\n{functions}",
                    String::from_utf8_lossy(&compile.stderr)
                );
                let result = Command::new(&executable).output().unwrap();
                assert!(result.status.success());
                assert_eq!(
                    String::from_utf8(result.stdout).unwrap(),
                    "7 2.500000\n8 3.500000\n9 4.500000\n10 5.500000\n",
                    "{mode} {level}"
                );
            }
        }
    }
}
