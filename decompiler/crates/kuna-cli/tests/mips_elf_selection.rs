use crate::common;
use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};
use std::{path::Path, process::Command};

fn elf(little: bool, flags: u32, words: &[u32]) -> Vec<u8> {
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let half = |v: u16| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let mut data = b"\x7fELF".to_vec();
    data.extend([1, if little { 1 } else { 2 }, 1]);
    data.resize(16, 0);
    data.extend([2, 8].into_iter().flat_map(half));
    data.extend([1, 0x1000, 52, 0, flags].into_iter().flat_map(word));
    data.extend([52, 32, 1, 40, 0, 0].into_iter().flat_map(half));
    let size = words.len() as u32 * 4;
    data.extend(
        [1, 0x100, 0x1000, 0x1000, size, size, 5, 4]
            .into_iter()
            .flat_map(word),
    );
    data.resize(0x100, 0);
    data.extend(words.iter().copied().flat_map(word));
    data
}

fn elf_with_fpr(little: bool, flags: u32, words: &[u32], fpr_size: u8, fp_abi: u8) -> Vec<u8> {
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let half = |v: u16| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let mut data = elf(little, flags, words);
    data.resize(data.len().next_multiple_of(8), 0);
    let flags_offset = data.len() as u32;
    data.extend([0, 0, 64, 2, 2, fpr_size, 0, fp_abi]);
    data.extend([0; 16]);
    let names = b"\0.MIPS.abiflags\0.shstrtab\0.text\0";
    let names_offset = data.len() as u32;
    data.extend(names);
    data.resize(data.len().next_multiple_of(4), 0);
    let sections_offset = data.len() as u32;
    data.extend([0; 40]);
    for section in [
        [1, 0x7000_002a, 0, 0, flags_offset, 24, 0, 0, 8, 24],
        [16, 3, 0, 0, names_offset, names.len() as u32, 0, 0, 1, 0],
        [26, 1, 6, 0x1000, 0x100, words.len() as u32 * 4, 0, 0, 4, 0],
    ] {
        data.extend(section.into_iter().flat_map(word));
    }
    data[32..36].copy_from_slice(&word(sections_offset));
    data[48..50].copy_from_slice(&half(4));
    data[50..52].copy_from_slice(&half(2));
    data
}

fn declared(path: &Path, prototype: &str) -> String {
    asserted(path, &[&format!("prototype 0x1000 {prototype}")])
}

fn asserted(path: &Path, assertions: &[&str]) -> String {
    asserted_options(path, assertions, &[])
}

fn asserted_options(path: &Path, assertions: &[&str], options: &[&str]) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile")
        .arg(path)
        .args(["0x1000", "--mode", "fast", "--assert-strict"])
        .args(options);
    for assertion in assertions {
        command.args(["--assert", assertion]);
    }
    let output = command.output().unwrap();
    assert!(output.status.success(), "{output:?}");
    String::from_utf8(output.stdout).unwrap()
}

fn run(path: &Path, command: &str, target: Option<&str>) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args([command])
        .arg(path)
        .args(["0x1000", "--mode", "fast"]);
    if command == "disassemble" {
        cmd.args(["--count", "3"]);
    }
    if let Some(target) = target {
        cmd.args(["--target", target]);
    }
    cmd.output().unwrap()
}

#[test]
fn mips_elf32_automatic_selection_matches_the_declared_abi() {
    let path = common::scratch_file("mips-elf-isa", "elf");
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        for (flags, model) in [(0x8000_0020, "n32"), (0x8000_2000, "o64")] {
            // dsll v0,a0,1; jr ra; nop: 64-bit data with a 32-bit ELF address.
            std::fs::write(&path, elf(little, flags, &[0x00041078, 0x03e00008, 0])).unwrap();
            let target = format!("MIPS:{endian}:64:64-32addr:{model}");
            for command in ["disassemble", "decompile"] {
                let expected = run(&path, command, Some(&target));
                assert!(
                    expected.status.success(),
                    "{}",
                    String::from_utf8_lossy(&expected.stderr)
                );
                for auto in [None, Some("default")] {
                    let actual = run(&path, command, auto);
                    assert!(
                        actual.status.success(),
                        "{}",
                        String::from_utf8_lossy(&actual.stderr)
                    );
                    assert_eq!(actual.stdout, expected.stdout, "{command}, {target}");
                    let text = String::from_utf8_lossy(&actual.stdout);
                    assert!(!text.contains("halt_baddata"), "{text}");
                    assert!(
                        text.contains(if command == "decompile" {
                            "return"
                        } else {
                            "dsll"
                        }),
                        "{text}"
                    );
                }
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn uncertain_mips_abi_is_visible() {
    let path = common::scratch_file("mips-elf-assumptions", "elf");
    std::fs::write(&path, elf(false, 0x8000_0000, &[0x00041078, 0x03e00008, 0])).unwrap();
    for command in ["decompile", "disassemble"] {
        let output = run(&path, command, None);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("assuming O64"), "{stderr}");
        let output = run(&path, command, Some("MIPS:BE:64:64-32addr:o32"));
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!stderr.contains("assuming O64"), "{stderr}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn n32_recovers_the_sixth_register_argument_as_a_64_bit_value() {
    let path = common::scratch_file("mips-n32-sixth", "elf");
    for little in [false, true] {
        // dsll v0,a5,1; jr ra; nop. The same register is caller scratch in O64.
        std::fs::write(
            &path,
            elf(little, 0x8000_0020, &[0x00091078, 0x03e00008, 0]),
        )
        .unwrap();
        let output = run(&path, "decompile", None);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("long long a5)"), "{text}");
        assert!(text.contains("return a5 << 1;"), "{text}");
        assert!(!text.contains("// t1"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_wide_isa_preserves_inferred_register_pair_and_scalar_returns() {
    let path = common::scratch_file("mips-o32-returns", "elf");
    // lui/ori v0,0x12345678; lui/ori v1,0x9abcdef0; jr ra; nop.
    let pair = [
        0x3c021234, 0x34425678, 0x3c039abc, 0x3463def0, 0x03e00008, 0,
    ];
    for flags in [0x8000_1000, 0x8000_1100] {
        std::fs::write(&path, elf(true, flags, &pair)).unwrap();
        let output = run(&path, "decompile", None);
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("return 0x9abcdef012345678;"), "{text}");
    }
    for little in [false, true] {
        // addiu v0,zero,42; jr ra; nop: a scalar still uses only 32 bits.
        std::fs::write(
            &path,
            elf(little, 0x8000_1000, &[0x2402002a, 0x03e00008, 0]),
        )
        .unwrap();
        let output = run(&path, "decompile", None);
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("unsigned int sub_1000(void)"), "{text}");
        assert!(text.contains("return 0x2a;"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_wide_instructions_return_a_declared_pair_in_abi_order() {
    let path = common::scratch_file("mips-o32-wide-return", "elf");
    for little in [false, true] {
        // Build 0x123400005678 with dsll32, then split it into v0 and v1.
        let words = [
            0x34021234, 0x0002103c, 0x34425678, 0x0002183e, 0x00021000, 0x03e00008, 0,
        ];
        std::fs::write(&path, elf(little, 0x8000_1000, &words)).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile")
            .arg(&path)
            .args([
                "0x1000",
                "--mode",
                "fast",
                "--assert",
                "prototype 0x1000 unsigned long long value(void)",
            ])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        let value = if little {
            "0x123400005678"
        } else {
            "0x567800001234"
        };
        assert!(text.contains(&format!("return {value};")), "{text}");
        assert!(!text.contains("halt_baddata"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_wide_arguments_keep_pairs_alignment_and_stack_slots() {
    let path = common::scratch_file("mips-o32-arguments", "elf");
    let cases: &[(&[u32], &str, &str)] = &[
        // move v0,a0; jr ra; move v1,a1.
        (
            &[0x00801021, 0x03e00008, 0x00a01821],
            "unsigned long long f(unsigned long long x)",
            "x",
        ),
        // An initial int leaves a1 unused; the pair occupies a2/a3.
        (
            &[0x00c01021, 0x03e00008, 0x00e01821],
            "unsigned long long f(int pad,unsigned long long x)",
            "x",
        ),
        // lw v0,16(sp); jr ra; lw v1,20(sp).
        (
            &[0x8fa20010, 0x03e00008, 0x8fa30014],
            "unsigned long long f(int pad,unsigned long long x,unsigned long long y)",
            "y",
        ),
        (
            &[0x00c01021, 0x03e00008, 0x00e01821],
            "unsigned long long f(double pad,unsigned long long x)",
            "x",
        ),
        // O32 long double is an eight-byte value: mov.d f0,f12; jr ra; nop.
        (
            &[0x46206006, 0x03e00008, 0],
            "long double f(long double x)",
            "x",
        ),
    ];
    for little in [false, true] {
        for (fpr, abi) in [(1, 1), (2, 6)] {
            for (words, prototype, value) in cases {
                std::fs::write(&path, elf_with_fpr(little, 0x8000_1000, words, fpr, abi)).unwrap();
                let text = declared(&path, prototype);
                assert!(
                    text.contains(&format!("return {value};")),
                    "{prototype}, FPR={fpr}: {text}"
                );
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mips_fpr_width_is_independent_of_gpr_width() {
    let path = common::scratch_file("mips-fpr-width", "elf");
    let pairs = [0x44800000, 0x3c083ff0, 0x44880800, 0x03e00008, 0];
    // dsll requires the 64-bit integer decoder even with 32-bit FPRs.
    let wide_pairs = [
        0x34083ff0, 0x00084438, 0x44880800, 0x44800000, 0x03e00008, 0,
    ];
    // dsll32; dmtc1 writes the whole double, using a pair in FP32 mode.
    let whole = [0x3c083ff0, 0x0008403c, 0x44a80000, 0x03e00008, 0];
    for little in [false, true] {
        for (words, fpr, abi) in [
            (&pairs[..], 1, 1),
            (&wide_pairs[..], 1, 1),
            (&whole[..], 1, 1),
            (&whole[..], 2, 6),
        ] {
            std::fs::write(&path, elf_with_fpr(little, 0x8000_1000, words, fpr, abi)).unwrap();
            let text = declared(&path, "double f(void)");
            assert!(text.contains("return 1.0;"), "FPR={fpr}: {text}");
            assert!(!text.contains("halt_baddata"), "{text}");
        }
        // Legacy O32 without FP metadata retains its paired FPR convention.
        std::fs::write(&path, elf(little, 0x8000_1000, &pairs)).unwrap();
        let text = declared(&path, "double f(void)");
        assert!(text.contains("return 1.0;"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_full_register_reads_recover_extended_word_arguments() {
    let path = common::scratch_file("mips-o32-register-extension", "elf");
    for little in [false, true] {
        for (fpr, abi) in [(1, 1), (2, 6)] {
            // slt v0,a0,zero; jr ra; nop.
            let words = [0x0080102a, 0x03e00008, 0];
            std::fs::write(&path, elf_with_fpr(little, 0x8000_1000, &words, fpr, abi)).unwrap();
            let text = asserted(&path, &[]);
            assert!(text.contains("sub_1000(int a0)"), "{text}");
            assert!(!text.contains("; //"), "{text}");
            check_emitted_c(
                &format!("#include <stdbool.h>\n{text}"),
                "sub_1000(-1) != 1 || sub_1000(-2147483647-1) != 1 || sub_1000(0) != 0 || sub_1000(2147483647) != 0",
            );
            for (ty, expression) in [
                ("int", "sub_1000(-1) != 1 || sub_1000(1) != 0"),
                (
                    "unsigned int",
                    "sub_1000(0x80000000u) != 1 || sub_1000(0xffffffffu) != 1 || sub_1000(1) != 0",
                ),
                ("signed char", "sub_1000(-128) != 1 || sub_1000(127) != 0"),
                ("unsigned char", "sub_1000(255) != 0 || sub_1000(0) != 0"),
            ] {
                let text = declared(&path, &format!("int f({ty} x)"));
                assert!(!text.contains("; //"), "{ty}: {text}");
                check_emitted_c(&text, expression);
            }
            // The last argument register has the same extension guarantee.
            let words = [0x00e0102a, 0x03e00008, 0];
            std::fs::write(&path, elf_with_fpr(little, 0x8000_1000, &words, fpr, abi)).unwrap();
            let text = declared(&path, "int f(int a,int b,int c,int x)");
            assert!(text.contains("x < 0"), "{text}");
            assert!(!text.contains("; //"), "{text}");
            check_emitted_c(
                &text,
                "sub_1000(11,22,33,-1) != 1 || sub_1000(11,22,33,1) != 0",
            );
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_aggregate_returns_keep_the_hidden_pointer_and_argument_slots() {
    let path = common::scratch_file("mips-o32-hidden-result", "elf");
    let cases: &[(&[u32], &str, &str, &[&str], &str)] = &[
        (
            &[0xac850000, 0x00801021, 0x03e00008, 0],
            "typedef struct Result { int value; } Result;",
            "Result f(int x)",
            &["rethidden->value = x;"],
            "sub_1000(&result,-7) != &result || result.value != -7",
        ),
        (
            &[0xac850000, 0x00801021, 0x03e00008, 0],
            "typedef union Result { int value; unsigned int bits; } Result;",
            "Result f(int x)",
            &["rethidden->value = x;"],
            "sub_1000(&result,-7) != &result || result.value != -7",
        ),
        (
            &[0xac850000, 0xac860004, 0xac870008, 0x00801021, 0x03e00008, 0],
            "typedef struct Result { int x; int y; int z; } Result;",
            "Result f(int x,int y,int z)",
            &["rethidden->x = x;", "rethidden->y = y;", "rethidden->z = z;"],
            "sub_1000(&result,-7,17,0x12345678) != &result || result.x != -7 || result.y != 17 || result.z != 0x12345678",
        ),
        (
            &[0xac860000, 0xac870004, 0x00801021, 0x03e00008, 0],
            "typedef struct Result { unsigned long long value; } Result;",
            "Result f(unsigned long long x)",
            &["rethidden->value = x;"],
            "sub_1000(&result,0x9abcdef012345678ull) != &result || result.value != 0x9abcdef012345678ull",
        ),
    ];
    for little in [false, true] {
        for (fpr, abi) in [(1, 1), (2, 6)] {
            for (words, typedef, prototype, stores, expression) in cases {
                std::fs::write(&path, elf_with_fpr(little, 0x8000_1000, words, fpr, abi)).unwrap();
                let text = asserted(&path, &[typedef, &format!("prototype 0x1000 {prototype}")]);
                assert!(text.contains("Result *rethidden"), "{text}");
                assert!(text.contains("return rethidden;"), "{text}");
                assert!(!text.contains("; //"), "{text}");
                for store in *stores {
                    assert!(text.contains(store), "{text}");
                }
                check_emitted_c(
                    &format!("{typedef}\nstatic Result result;\n{text}"),
                    expression,
                );
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

fn check_emitted_c(code: &str, expression: &str) {
    if common::process::optional_output(Command::new("cc").arg("--version")).is_none() {
        eprintln!("cc is unavailable; skipping only the emitted-C execution check");
        return;
    }
    let source = common::scratch_file("elf-abi-result", "c");
    let executable = common::scratch_file("elf-abi-result", "exe");
    std::fs::write(
        &source,
        format!("{code}\nint main(void) {{ return {expression}; }}\n"),
    )
    .unwrap();
    let output = Command::new("cc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        Command::new(&executable).status().unwrap().success(),
        "{code}"
    );
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
}

fn call_image(little: bool, fpr: u8, operations: &[u32], pair: bool) -> Vec<u8> {
    // Save ra; jal callee; caller operations; restore ra and return.
    let mut words = vec![0x27bdfff0, 0xafbf000c, 0x0c000410, 0];
    words.extend(operations);
    words.extend([0x8fbf000c, 0x03e00008, 0x27bd0010]);
    let caller_size = words.len() as u32 * 4;
    words.resize(16, 0);
    words.extend([0x2402ffff, 0x03e00008, if pair { 0x2403ffff } else { 0 }]);
    with_symbols(
        little,
        elf_with_fpr(
            little,
            0x8000_1000,
            &words,
            fpr,
            if fpr == 1 { 1 } else { 6 },
        ),
        &[("caller", 0x1000, caller_size), ("callee", 0x1040, 12)],
    )
}

fn with_symbols(little: bool, mut data: Vec<u8>, symbols: &[(&str, u32, u32)]) -> Vec<u8> {
    let word = |v: u32| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let half = |v: u16| {
        if little {
            v.to_le_bytes()
        } else {
            v.to_be_bytes()
        }
    };
    let read = |b: &[u8]| {
        if little {
            u32::from_le_bytes(b.try_into().unwrap())
        } else {
            u32::from_be_bytes(b.try_into().unwrap())
        }
    };
    let sh = read(&data[32..36]) as usize;
    let mut headers: Vec<Vec<u32>> = data[sh..sh + 160]
        .chunks_exact(40)
        .map(|row| row.chunks_exact(4).map(read).collect())
        .collect();
    let mut strings = vec![0];
    let mut offsets = Vec::new();
    for (name, _, _) in symbols {
        offsets.push(strings.len() as u32);
        strings.extend(name.as_bytes());
        strings.push(0);
    }
    let strings_offset = data.len() as u32;
    data.extend(&strings);
    data.resize(data.len().next_multiple_of(4), 0);
    let symbols_offset = data.len() as u32;
    data.extend([0; 16]);
    for (name, (_, address, size)) in offsets.into_iter().zip(symbols) {
        data.extend([name, *address, *size].into_iter().flat_map(word));
        data.extend([0x12, 0]);
        data.extend(half(3));
    }
    let names = b"\0.MIPS.abiflags\0.shstrtab\0.text\0.symtab\0.strtab\0";
    let names_offset = data.len() as u32;
    data.extend(names);
    data.resize(data.len().next_multiple_of(4), 0);
    headers[2][4] = names_offset;
    headers[2][5] = names.len() as u32;
    headers.extend([
        vec![
            32,
            2,
            0,
            0,
            symbols_offset,
            16 * (symbols.len() as u32 + 1),
            5,
            1,
            4,
            16,
        ],
        vec![
            40,
            3,
            0,
            0,
            strings_offset,
            strings.len() as u32,
            0,
            0,
            1,
            0,
        ],
    ]);
    let sh = data.len() as u32;
    data[32..36].copy_from_slice(&word(sh));
    data[48..50].copy_from_slice(&half(headers.len() as u16));
    for row in headers {
        data.extend(row.into_iter().flat_map(word));
    }
    data
}

#[test]
fn o32_call_results_extend_words_without_widening_return_slots() {
    let path = common::scratch_file("mips-o32-call-results", "elf");
    for little in [false, true] {
        for fpr in [1, 2] {
            for (ty, values) in [
                ("int", "-1,0,1,(-2147483647-1),2147483647"),
                ("unsigned int", "0xffffffffu,0,1,0x80000000u,0x7fffffffu"),
            ] {
                // slt v0,v0,zero uses all 64 bits of a 32-bit call result.
                std::fs::write(&path, call_image(little, fpr, &[0x0040102a], false)).unwrap();
                let text = asserted(
                    &path,
                    &[
                        "prototype 0x1000 int caller(void)",
                        &format!("prototype 0x1040 {ty} callee(void)"),
                    ],
                );
                assert!(!text.contains("; //"), "{text}");
                assert!(text.contains("callee()") && text.contains("< 0"), "{text}");
                check_emitted_c(&format!("static {ty} result;\n{ty} callee(void) {{ return result; }}\n{text}\nstatic int check(void) {{ {ty} values[] = {{{values}}}; for (int i=0;i<5;i++) {{ result=values[i]; if (caller() != ((int)result < 0)) return 1; }} return 0; }}"), "check()");
            }
            for (ops, return_type, expected) in [
                (&[][..], "unsigned long long", "result"),
                (
                    &[0x0040102a][..],
                    "int",
                    if little {
                        "((int)result < 0)"
                    } else {
                        "((int)(result >> 32) < 0)"
                    },
                ),
                (
                    &[0x0060102a][..],
                    "int",
                    if little {
                        "((int)(result >> 32) < 0)"
                    } else {
                        "((int)result < 0)"
                    },
                ),
            ] {
                std::fs::write(&path, call_image(little, fpr, ops, true)).unwrap();
                let text = asserted(
                    &path,
                    &[
                        &format!("prototype 0x1000 {return_type} caller(void)"),
                        "prototype 0x1040 unsigned long long callee(void)",
                    ],
                );
                assert!(!text.contains("; //"), "{text}");
                check_emitted_c(&format!("static unsigned long long result;\nunsigned long long callee(void) {{ return result; }}\n{text}\nstatic int check(void) {{ unsigned long long values[] = {{0,0xffffffffffffffffull,0x9abcdef012345678ull,0x123456789abcdef0ull}}; for (int i=0;i<4;i++) {{ result=values[i]; if (caller() != {expected}) return 1; }} return 0; }}"), "check()");
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_call_results_are_recovered_without_a_callee_prototype() {
    let path = common::scratch_file("mips-o32-inferred-call", "elf");
    for little in [false, true] {
        for fpr in [1, 2] {
            std::fs::write(&path, call_image(little, fpr, &[0x0040102a], false)).unwrap();
            let text = asserted(&path, &["prototype 0x1000 int caller(void)"]);
            assert!(!text.contains("; //"), "{text}");
            assert!(text.contains("callee()") && text.contains("< 0"), "{text}");
            check_emitted_c(
                &format!("static int result;\nint callee(void) {{ return result; }}\n{text}"),
                "(result=-1,caller()!=1) || (result=0,caller()!=0)",
            );
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_subword_call_results_extend_by_type_before_extending_the_register() {
    let path = common::scratch_file("mips-o32-subword-call", "elf");
    for little in [false, true] {
        for fpr in [1, 2] {
            std::fs::write(&path, call_image(little, fpr, &[0x0040102a], false)).unwrap();
            for (ty, value, expected) in [("signed char", "-128", 1), ("unsigned char", "255", 0)] {
                let text = asserted_options(
                    &path,
                    &[
                        "prototype 0x1000 int caller(void)",
                        &format!("prototype 0x1040 {ty} callee(void)"),
                    ],
                    &["--option", "narrowext", "compiler"],
                );
                assert!(!text.contains("; //"), "{text}");
                check_emitted_c(
                    &format!("{ty} callee(void) {{ return {value}; }}\n{text}"),
                    &format!("caller() != {expected}"),
                );
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mips_elf32_entry_addresses_extend_to_the_selected_register_width() {
    let path = common::scratch_file("mips-entry-address", "elf");
    for little in [false, true] {
        let endian = if little { "LE" } else { "BE" };
        let word = |value: u32| {
            if little {
                value.to_le_bytes()
            } else {
                value.to_be_bytes()
            }
        };
        for (flags, fpr, target) in [
            (0x8000_1000, 1, None),
            (0x8000_1000, 2, None),
            (0x8000_0020, 2, None),
            (0x8000_2000, 2, None),
            (0x8000_1100, 1, None),
            (0x8000_1100, 1, Some("64:64-32addr-fp32:o32")),
            (0x8000_1000, 1, Some("32:default:default")),
        ] {
            for entry in [0x1000, 0x7fff_f000, 0x8000_1000, 0xffff_f000] {
                // slt v0,t9,zero; jr ra; nop: inspect the full entry register.
                let mut bytes = elf_with_fpr(
                    little,
                    flags,
                    &[0x0320102a, 0x03e00008, 0],
                    fpr,
                    if fpr == 1 { 1 } else { 6 },
                );
                let sh: [u8; 4] = bytes[32..36].try_into().unwrap();
                let sh = if little {
                    u32::from_le_bytes(sh)
                } else {
                    u32::from_be_bytes(sh)
                } as usize;
                for offset in [24, 60, 64, sh + 3 * 40 + 12] {
                    bytes[offset..offset + 4].copy_from_slice(&word(entry));
                }
                std::fs::write(
                    &path,
                    with_symbols(little, bytes, &[("entry_sign", entry, 12)]),
                )
                .unwrap();
                let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
                cmd.arg("decompile")
                    .arg(&path)
                    .args([
                        "entry_sign",
                        "--mode",
                        "fast",
                        "--assert-strict",
                        "--assert",
                    ])
                    .arg(format!("prototype {entry:#x} int entry_sign(void)"));
                if let Some(target) = target {
                    cmd.args(["--target", &format!("MIPS:{endian}:{target}")]);
                }
                let output = cmd.output().unwrap();
                assert!(
                    output.status.success(),
                    "{flags:#x} {entry:#x} {target:?}: {output:?}"
                );
                let text = String::from_utf8(output.stdout).unwrap();
                let expected = u8::from((entry as i32) < 0);
                assert!(
                    text.contains(&format!("return {expected};")),
                    "{endian} {flags:#x} FPR{fpr} {entry:#x} {target:?}:\n{text}"
                );
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn release6_micromips_metadata_keeps_ordinary_functions() {
    use object::{Object as _, ObjectSection as _};
    let path = common::scratch_file("mips-r6-mixed-mode", "elf");
    for little in [false, true] {
        // Ordinary addiu v0,a0,1; jrc ra, followed by a microMIPS function.
        let compact = if little {
            [0x0001_3044, 0x0000_47e3]
        } else {
            [0x3044_0001, 0x47e3_0000]
        };
        let mut data = elf_with_fpr(
            little,
            0x9200_1405,
            &[0x2482_0001, 0xd81f_0000, compact[0], compact[1]],
            2,
            6,
        );
        data[0x112..0x115].copy_from_slice(&[32, 6, 1]);
        data = with_symbols(
            little,
            data,
            &[("normal", 0x1000, 8), ("compact", 0x1008, 6)],
        );
        let file = object::File::parse(data.as_slice()).unwrap();
        let symtab = file
            .section_by_name(".symtab")
            .unwrap()
            .file_range()
            .unwrap()
            .0 as usize;
        data[symtab + 2 * 16 + 13] = 0x80;
        std::fs::write(&path, data).unwrap();
        let endian = if little { "LE" } else { "BE" };
        let target = format!("MIPS:{endian}:32:R6:default");
        let mut outputs = Vec::new();
        for explicit in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
            command
                .arg("decompile")
                .arg(&path)
                .args(["normal", "--mode", "fast"]);
            if explicit {
                command.args(["--target", &target]);
            }
            let output = command.output().unwrap();
            assert!(output.status.success(), "{output:?}");
            let text = String::from_utf8(output.stdout).unwrap();
            assert!(text.contains("return a0 + 1;"), "{text}");
            outputs.push(text);
        }
        assert_eq!(outputs[0], outputs[1]);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn fp32_default_compiler_keeps_o32_unsigned_argument_extension() {
    let path = common::scratch_file("mips-fp32-default-abi", "elf");
    for little in [false, true] {
        std::fs::write(
            &path,
            elf(little, 0x8000_1000, &[0x0080102a, 0x03e00008, 0]),
        )
        .unwrap();
        let endian = if little { "LE" } else { "BE" };
        let mut outputs = Vec::new();
        for compiler in ["default", "o32"] {
            let target = format!("MIPS:{endian}:64:64-32addr-fp32:{compiler}");
            let text = asserted_options(
                &path,
                &["prototype 0x1000 int f(unsigned char x)"],
                &["--target", &target],
            );
            assert!(text.contains("return 0;"), "{target}: {text}");
            check_emitted_c(&text, "sub_1000(255) != 0 || sub_1000(0) != 0");
            outputs.push(text);
        }
        assert_eq!(outputs[0], outputs[1]);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn o32_aggregate_arguments_keep_memory_order_with_wide_registers() {
    let path = common::scratch_file("mips-o32-aggregate-placement", "elf");
    for little in [false, true] {
        for (fpr, abi) in [(1, 1), (2, 6)] {
            for kind in ["struct", "union"] {
                for size in [1usize, 2, 4, 8] {
                    let fields = if kind == "struct" {
                        (0..size)
                            .map(|i| format!("unsigned char m{i};"))
                            .collect::<String>()
                    } else {
                        format!("unsigned char m0; unsigned char storage[{size}];")
                    };
                    let typedef = format!("typedef {kind} T {{ {fields} }} T;");
                    for padding in [0usize, 3, 4] {
                        for index in if kind == "struct" && size > 1 {
                            vec![0, size - 1]
                        } else {
                            vec![0]
                        } {
                            let slot = padding + index / 4;
                            let words = if slot < 4 {
                                let shift = if little { index % 4 } else { 3 - index % 4 } * 8;
                                vec![
                                    ((4 + slot as u32) << 16)
                                        | (2 << 11)
                                        | ((shift as u32) << 6)
                                        | 2,
                                    0x304200ff,
                                    0x03e00008,
                                    0,
                                ]
                            } else {
                                vec![
                                    0x93a20000 | (16 + (slot as u32 - 4) * 4 + index as u32 % 4),
                                    0x03e00008,
                                    0,
                                ]
                            };
                            std::fs::write(
                                &path,
                                elf_with_fpr(little, 0x8000_1000, &words, fpr, abi),
                            )
                            .unwrap();
                            let args: String = (0..padding).map(|i| format!("int p{i},")).collect();
                            let text = asserted(
                                &path,
                                &[&typedef, &format!("prototype 0x1000 int f({args}T x)")],
                            );
                            assert!(
                                !text.contains("; //"),
                                "{little} {fpr} {kind} {size} {padding} {index}: {text}"
                            );
                            let values = "11,".repeat(padding);
                            let replay = format!("{typedef}\n{text}\nstatic int check(void) {{ T x = {{0}}; for (unsigned int i = 0; i < 256; ++i) {{ x.m{index} = i; if (sub_1000({values}x) != i) return 1; }} return 0; }}");
                            if size == 1 {
                                assert!(!text.contains(">>"), "{text}");
                                check_emitted_c(&replay, "check()");
                            }
                            check_emitted_mips_c(little, &replay, "check()");
                        }
                    }
                }
            }
        }
    }
    std::fs::remove_file(path).unwrap();
}

fn check_emitted_mips_c(little: bool, code: &str, expression: &str) {
    let qemu = if little { "qemu-mipsel" } else { "qemu-mips" };
    for tool in ["clang", "ld.lld", qemu] {
        if common::process::optional_output(Command::new(tool).arg("--version")).is_none() {
            eprintln!("{tool} is unavailable; skipping only the MIPS emitted-C replay");
            return;
        }
    }
    let source = common::scratch_file("mips-o32-aggregate-replay", "c");
    let executable = common::scratch_file("mips-o32-aggregate-replay", "elf");
    std::fs::write(
        &source,
        format!(
            r#"{code}
void _start(void) {{
    int status = {expression};
    register int number __asm__("$2") = 4001;
    register int result __asm__("$4") = status;
    __asm__ volatile("syscall" : "+r"(number) : "r"(result) : "memory");
    __builtin_unreachable();
}}
"#
        ),
    )
    .unwrap();
    let output = Command::new("clang")
        .arg(if little {
            "--target=mipsel-linux-gnu"
        } else {
            "--target=mips-linux-gnu"
        })
        .args([
            "-march=mips32r2",
            "-mabi=32",
            "-mno-abicalls",
            "-fno-pic",
            "-G0",
            "-ffreestanding",
            "-nostdlib",
            "-static",
            "-fuse-ld=lld",
            "-Wl,-e,_start",
            "-O1",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new(qemu).arg(&executable).output().unwrap();
    assert!(output.status.success(), "{little}: {code}\n{output:?}");
    std::fs::remove_file(source).unwrap();
    std::fs::remove_file(executable).unwrap();
}
