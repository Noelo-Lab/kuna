use crate::common;
use std::process::Command;

fn image(count: usize, scalar: bool, last: bool) -> Vec<u8> {
    let registers = [0u32, 2, 3, 4, 5, 6, 7, 8];
    let saved = registers[..count]
        .iter()
        .filter(|&&r| r >= 4)
        .fold(0, |mask, &r| mask | (1 << r));
    let mut words = vec![0xe92d4000 | saved, 0xe3001000, 0xe3451000];
    if scalar {
        words.extend(
            registers[..count]
                .iter()
                .enumerate()
                .map(|(i, &r)| 0xe5910000 | (r << 12) | (i as u32 * 4)),
        );
    } else {
        words.push(
            0xe8910000
                | registers[..count]
                    .iter()
                    .fold(0, |mask, &r| mask | (1 << r)),
        );
    }
    if last {
        words.push(0xe1a00000 | registers[count - 1]);
    }
    words.push(0xe8bd8000 | saved);
    words.into_iter().flat_map(u32::to_le_bytes).collect()
}

#[test]
fn unused_multiple_load_destinations_keep_their_volatile_reads() {
    for count in [4, 8] {
        for scalar in [false, true] {
            for last in [false, true] {
                let path = common::scratch_file("arm-volatile-reads", "bin");
                std::fs::write(&path, image(count, scalar, last)).unwrap();
                for marked in [0, 1, 2] {
                    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
                    cmd.args([
                        "decompile",
                        path.to_str().unwrap(),
                        "0x1000",
                        "--addr",
                        "--raw-image",
                        "--base",
                        "0x1000",
                        "--target",
                        "ARM:LE:32:v7",
                        "--isa",
                        "arm",
                        "--mode",
                        "aggressive",
                    ]);
                    if marked != 0 {
                        let (start, size) = if marked == 1 {
                            (0x50000000, count * 4)
                        } else {
                            (0x50000008, 4)
                        };
                        cmd.args([
                            "--assert",
                            &format!("volatile 0x{start:x}+{size}"),
                            "--assert-strict",
                        ]);
                    }
                    let result = cmd.output().unwrap();
                    let code = String::from_utf8_lossy(&result.stdout);
                    assert!(
                        result.status.success(),
                        "{code}\n{}",
                        String::from_utf8_lossy(&result.stderr)
                    );
                    let mut previous = None;
                    for i in 0..count {
                        let name = format!("dat_{:08x}", 0x50000000 + i * 4);
                        let should_read = marked == 1
                            || (marked == 2 && i == 2)
                            || i == if last { count - 1 } else { 0 };
                        assert_eq!(
                            code.matches(&name).count(),
                            usize::from(should_read),
                            "count={count} scalar={scalar} last={last} marked={marked}\n{code}"
                        );
                        if marked == 1 {
                            let at = code.find(&name).unwrap();
                            assert!(previous.is_none_or(|p| p < at), "read order: {code}");
                            previous = Some(at);
                        }
                    }
                }
            }
        }
    }
}

fn decompile_raw(hex: &str, target: &str, volatile: Option<&str>) -> String {
    let image: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    let path = common::scratch_file("volatile-reads", "bin");
    std::fs::write(&path, image).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args([
        "decompile",
        path.to_str().unwrap(),
        "0x1000",
        "--addr",
        "--raw-image",
        "--base",
        "0x1000",
        "--target",
        target,
        "--mode",
        "aggressive",
    ]);
    if let Some(range) = volatile {
        cmd.args(["--assert", &format!("volatile {range}"), "--assert-strict"]);
    }
    let result = cmd.output().unwrap();
    let code = String::from_utf8_lossy(&result.stdout).into_owned();
    assert!(
        result.status.success(),
        "{code}\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    code
}

/// `mov ecx,0x50000000; add rcx,4` three times, then four `mov eax,[rcx-N]`
/// reads that each overwrite the last, then `xor eax,eax; ret`.
#[test]
fn register_addressed_x86_volatile_reads_survive_into_a_dead_register() {
    let bytes = "b9000000504883c1044883c1044883c1048b018b41fc8b41f88b41f431c0c3";
    for marked in [false, true] {
        let code = decompile_raw(
            bytes,
            "x86:LE:64:default",
            marked.then_some("0x50000000+16"),
        );
        let mut previous = None;
        for offset in [0xc, 0x8, 0x4, 0x0] {
            let name = format!("dat_{:08x}", 0x50000000 + offset);
            assert_eq!(
                code.matches(&name).count(),
                usize::from(marked),
                "marked={marked}\n{code}"
            );
            if marked {
                let at = code.find(&name).unwrap();
                assert!(previous.is_none_or(|p| p < at), "read order: {code}");
                previous = Some(at);
            }
        }
    }
}

/// Flag macros re-load a memory operand: x86 `or dword [rcx+4],0x10` and
/// `add eax,[rcx+8]` behind a deep `rcx`, and MSP430 `add #1,8(r15)` into the
/// I/O range its pspec declares volatile. Each operand is one read.
#[test]
fn a_reloaded_operand_is_one_volatile_read() {
    let or = decompile_raw(
        "b9000000504883c1084883c1084883e9108349041031c0c3",
        "x86:LE:64:default",
        Some("0x50000000+16"),
    );
    assert!(or.contains("dat_50000004 = dat_50000004 | 0x10;"), "{or}");
    assert_eq!(or.matches("dat_50000004").count(), 2, "{or}");
    let add = decompile_raw(
        "b9000000504883c1044883c1044883c1044883e90c03410831c0c3",
        "x86:LE:64:default",
        Some("0x50000000+16"),
    );
    assert_eq!(add.matches("= dat_50000008;").count(), 1, "{add}");
    let msp = decompile_raw(
        "3f4018003f5004003f5004003f5004003f800c009f5308000c433041",
        "TI_MSP430:LE:16:default",
        None,
    );
    assert!(msp.contains("dat_20 = dat_20 + 1;"), "{msp}");
    assert_eq!(msp.matches("dat_20").count(), 2, "{msp}");
    let reads = decompile_raw(
        "3f4018003f5004003f5004003f5004003f8004002e4f1d4f02000c430f433041",
        "TI_MSP430:LE:16:default",
        None,
    );
    assert_eq!(reads.matches("= dat_20;").count(), 1, "{reads}");
    assert_eq!(reads.matches("= dat_22;").count(), 1, "{reads}");
}

#[test]
fn a_volatile_data_assertion_keeps_each_read_in_decompile_all() {
    let path = common::scratch_file("volatile-data-all", "bin");
    std::fs::write(
        &path,
        [0x8b, 0x04, 0x25, 0x00, 0x10, 0x60, 0x00, 0x03, 0x04, 0x25, 0x00, 0x10, 0x60, 0x00, 0xc3],
    )
    .unwrap();
    for declaration in ["data 0x601000 volatile int cursor", "data 0x601000 int volatile cursor"] {
        let (out, err, code) = common::run_kuna(&[
            "decompile-all",
            path.to_str().unwrap(),
            "--addr",
            "0x400000",
            "--raw-image",
            "--base",
            "0x400000",
            "--target",
            "x86:LE:64:default:gcc",
            "--assert",
            "function 0x400000=f",
            "--assert",
            declaration,
        ]);
        assert_eq!(code, 0, "{err}");
        assert!(out.contains("v1 = cursor;"), "{declaration}:\n{out}");
        assert!(!out.contains("cursor * 2"), "{declaration}:\n{out}");
    }
}

#[test]
fn volatile_typedef_data_preserves_scalar_and_array_reads() {
    let path = common::scratch_file("volatile-data-all", "bin");
    for (declaration, offset, volatile, access) in [
        ("volatile int cursor", 0, true, "cursor"),
        ("int volatile cursor", 0, true, "cursor"),
        ("VolWord cursor", 0, true, "cursor"),
        ("VolWord cursor[2]", 4, true, "cursor[1]"),
        ("VolWords cursor[3]", 0, true, "cursor[0][0]"),
        ("VolWords cursor[3]", 20, true, "cursor[2][1]"),
        ("volatile int cursor[3][2]", 20, true, "cursor[2][1]"),
        ("int cursor", 0, false, "cursor"),
        ("int cursor[2]", 4, false, "cursor[1]"),
        ("int cursor[3][2]", 20, false, "cursor[2][1]"),
    ] {
        std::fs::write(
            &path,
            [
                0x8b, 0x04, 0x25, offset, 0x10, 0x60, 0x00,
                0x8b, 0x14, 0x25, offset, 0x10, 0x60, 0x00,
                0x01, 0xd0, 0xc3,
            ],
        )
        .unwrap();
        for command in ["decompile", "decompile-all"] {
            let (out, err, code) = common::run_kuna(&[
                command,
                path.to_str().unwrap(),
                "--addr",
                "0x400000",
                "--raw-image",
                "--base",
                "0x400000",
                "--target",
                "x86:LE:64:default:gcc",
                "--assert-strict",
                "--assert",
                "function 0x400000=f",
                "--assert",
                "typedef typedef volatile int VolWord;",
                "--assert",
                "typedef typedef VolWord VolWords[2];",
                "--assert",
                &format!("data 0x601000 {declaration}"),
            ]);
            assert_eq!(code, 0, "{command} {declaration}: {err}");
            assert_eq!(
                out.matches(access).count(),
                if volatile { 2 } else { 1 },
                "{command} {declaration}:\n{out}",
            );
            assert_eq!(
                out.matches("cursor").count(),
                if volatile { 2 } else { 1 },
                "{command} {declaration}:\n{out}",
            );
        }
    }
}

#[test]
fn volatile_struct_data_reads_name_the_covered_field() {
    let path = common::scratch_file("volatile-struct-field", "bin");
    for (declaration, offset, access) in [
        ("volatile struct S s", 4, "s.b"),
        ("VolS s", 4, "s.b"),
        ("VolS cursor[3]", 20, "cursor[2].b"),
        ("struct S volatile cursor[3]", 16, "cursor[2].a"),
    ] {
        std::fs::write(
            &path,
            [
                0x8b, 0x04, 0x25, offset, 0x10, 0x60, 0x00,
                0x8b, 0x14, 0x25, offset, 0x10, 0x60, 0x00,
                0x01, 0xd0, 0xc3,
            ],
        )
        .unwrap();
        let (out, err, code) = common::run_kuna(&[
            "decompile",
            path.to_str().unwrap(),
            "--addr",
            "0x400000",
            "--raw-image",
            "--base",
            "0x400000",
            "--target",
            "x86:LE:64:default:gcc",
            "--assert-strict",
            "--assert",
            "function 0x400000=f",
            "--assert",
            "typedef struct S { int a; int b; };",
            "--assert",
            "typedef typedef volatile struct S VolS;",
            "--assert",
            &format!("data 0x601000 {declaration}"),
        ]);
        assert_eq!(code, 0, "{declaration}: {err}");
        assert_eq!(out.matches(access).count(), 2, "{declaration}:\n{out}");
    }
}

fn mapped_volatile_access(command: &str, hex: &str, declaration: &str) -> String {
    let image: Vec<u8> = (0..hex.len()).step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
    let path = common::scratch_file("volatile-access-width", "bin");
    std::fs::write(&path, image).unwrap();
    let (out, err, code) = common::run_kuna(&[
        command, path.to_str().unwrap(), "--addr", "0x400000", "--raw-image",
        "--base", "0x400000", "--target", "x86:LE:64:default:gcc", "--assert-strict",
        "--assert", "typedef struct One { int value; };",
        "--assert", "typedef typedef volatile struct One VolOne;",
        "--assert", "typedef union OneUnion { int value; unsigned int bits; };",
        "--assert", &format!("data 0x601000 {declaration}"),
    ]);
    std::fs::remove_file(path).unwrap();
    assert_eq!(code, 0, "{declaration}: {err}");
    out
}

#[test]
fn volatile_whole_aggregates_name_the_scalar_access() {
    let reads = "8b0425001060008b14250010600001d0c3";
    for command in ["decompile", "decompile-all"] {
        for (declaration, access) in [
            ("volatile int cursor[1]", "cursor[0]"),
            ("volatile int cursor[1][1]", "cursor[0][0]"),
            ("VolOne cursor", "cursor.value"),
            ("VolOne cursor[1]", "cursor[0].value"),
        ] {
            let out = mapped_volatile_access(command, reads, declaration);
            assert_eq!(out.matches(access).count(), 2, "{declaration}:\n{out}");
        }
    }
}

#[test]
fn volatile_array_accesses_keep_the_operation_width() {
    for command in ["decompile", "decompile-all"] {
        for (hex, declaration, access) in [
            ("8b0425001060008b14250010600001d0c3", "volatile char cursor[8]", "*(volatile unsigned int *)cursor"),
            ("8b0425041060008b14250410600001d0c3", "volatile char cursor[8]", "*(volatile unsigned int *)&cursor[4]"),
            ("8b0425001060008b14250010600001d0c3", "volatile long long cursor[2]", "*(volatile unsigned int *)&cursor[0]"),
            ("8b0425001060008b14250010600001d0c3", "volatile union OneUnion cursor", "*(volatile unsigned int *)&cursor"),
            ("c704250010600078563412c704250010600021436587c3", "volatile char cursor[8]", "*(volatile unsigned int *)cursor"),
            ("f30f10042500106000f30f10142500106000f30f58c2c3", "volatile char cursor[8]", "*(volatile unsigned int *)cursor"),
            ("660f6f042500106000660f6f142500106000660ffcc2660f7ec0c3", "volatile char cursor[32]", "*(volatile unsigned __int128 *)cursor"),
        ] {
            let out = mapped_volatile_access(command, hex, declaration);
            assert_eq!(out.matches(access).count(), 2, "{declaration}:\n{out}");
        }
    }
}

#[test]
fn inferred_simd_temporaries_have_scalar_declarations() {
    let hex = "660f6f042500106000660f6f142500106000660ffcc2660f7ec0c3";
    for command in ["decompile", "decompile-all"] {
        let out = mapped_volatile_access(command, hex, "volatile char cursor[32]");
        assert_eq!(out.matches("unsigned __int128 v").count(), 2, "{out}");
        assert_eq!(
            out.matches("*(volatile unsigned __int128 *)cursor").count(),
            2,
            "{out}"
        );
    }
    let path = common::scratch_file("simd-declaration-controls", "bin");
    let image: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    std::fs::write(&path, image).unwrap();
    for extra in [
        ["--option", "partialaccess", "off"].as_slice(),
        ["--assert", "type v1 char[16]"].as_slice(),
    ] {
        let mut args = vec![
            "decompile",
            path.to_str().unwrap(),
            "--addr",
            "0x400000",
            "--raw-image",
            "--base",
            "0x400000",
            "--target",
            "x86:LE:64:default:gcc",
            "--assert-strict",
            "--assert",
            "data 0x601000 volatile char cursor[32]",
        ];
        args.extend_from_slice(extra);
        let (out, err, code) = common::run_kuna(&args);
        assert_eq!(code, 0, "{err}");
        assert!(out.contains("char v1 [16]"), "{out}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn loaded_simd_temporaries_keep_array_declarations() {
    let hex = "f30f6f0766480f7ec0660f7ec1660f3a16c2014801c84801d0c3";
    let image: Vec<u8> = (0..hex.len()).step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect();
    let path = common::scratch_file("simd-loaded-declaration", "bin");
    std::fs::write(&path, image).unwrap();
    let (out, err, code) = common::run_kuna(&[
        "decompile", path.to_str().unwrap(), "--addr", "0x400000", "--raw-image",
        "--base", "0x400000", "--target", "x86:LE:64:default:gcc",
    ]);
    std::fs::remove_file(path).unwrap();
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("char v1 [16]"), "{out}");
    assert!(!out.contains("unsigned __int128 v1"), "{out}");
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn inferred_simd_temporaries_round_trip_through_native_c() {
    use common::process;
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("simd.c");
    for offset in [0, 4, 8, 12] {
        let shift = if offset == 0 {
            String::new()
        } else {
            format!("660f73d8{offset:02x}")
        };
        let hex = format!("660f6f042500106000660f6f142500106000660ffcc2{shift}660f7ec0c3");
        let out = mapped_volatile_access("decompile", &hex, "volatile char cursor[32]");
        let bytes = (0..hex.len())
            .step_by(2)
            .map(|i| format!("0x{}", &hex[i..i + 2]))
            .collect::<Vec<_>>()
            .join(",");
        let source = include_str!("fixtures/volatile_simd_native.c")
            .replace("GENERATED_FUNCTION", &out)
            .replace("ORIGINAL_BYTES", &bytes);
        std::fs::write(&file, source).unwrap();
        for cc in ["gcc", "clang"] {
            if process::optional_output(Command::new(cc).arg("--version")).is_none() {
                continue;
            }
            for optimization in ["-O0", "-O2"] {
                let binary = dir.path().join(format!("{cc}{optimization}"));
                let compiled = process::required_output(
                    Command::new(cc)
                        .args([
                            optimization,
                            "-fno-strict-aliasing",
                            "-Werror=implicit-function-declaration",
                        ])
                        .arg(&file)
                        .arg("-o")
                        .arg(&binary),
                );
                assert!(
                    compiled.status.success(),
                    "{cc} {optimization}: {}",
                    String::from_utf8_lossy(&compiled.stderr)
                );
                let result = process::required_output(&mut Command::new(&binary));
                assert!(
                    result.status.success(),
                    "{cc} {optimization}: {:?}: {}",
                    result.status,
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn discarded_pointer_reads_preserve_their_guard_and_access_width() {
    use common::process;
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "native read verification requires a C compiler");
    let dir = tempfile::tempdir().unwrap();
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/conditional_read_native.c");
    let original = dir.path().join("original.o");
    process::required_output(
        Command::new(compilers[0])
            .args(["-O2", "-fno-stack-protector", "-DORIGINAL_ONLY", "-c"])
            .arg(&fixture).arg("-o").arg(&original),
    );
    let mut targets = vec!["native"];
    if compilers.contains(&"clang") {
        targets.push("sparc");
    }
    for target in targets {
        let input = if target == "native" {
            original.clone()
        } else {
            let object = dir.path().join("sparc.o");
            process::required_output(
                Command::new("clang")
                    .args([
                        "--target=sparc-unknown-linux-gnu", "-integrated-as", "-O2",
                        "-fno-stack-protector", "-DORIGINAL_ONLY", "-c",
                    ])
                    .arg(&fixture).arg("-o").arg(&object),
            );
            object
        };
        let mut generated = String::new();
        for name in ["Probe8", "Probe16", "Probe32", "Probe64", "ProbeSpill32"] {
            let prototype = format!(
                "prototype {name} void {name}(unsigned int state,unsigned int mode,unsigned int left,unsigned int right,void *base)"
            );
            let result = process::required_output(
                Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .arg("decompile").arg(&input).arg(name)
                    .args(["--mode", "reliable", "--assert", &prototype, "--assert-strict"]),
            );
            let code = std::str::from_utf8(&result.stdout).unwrap();
            if target == "native" && name != "Probe8" {
                // Keep the inline type complete before its attributes. Besides
                // compiling and executing below, this form is consumed by C
                // frontends that reject attributes before an anonymous body.
                assert!(code.contains("} __attribute__((packed, may_alias))"), "{code}");
            }
            generated.push_str(code);
        }
        if target == "sparc" {
            let source = dir.path().join("sparc-width.c");
            let assembly = dir.path().join("sparc-width.s");
            std::fs::write(&source, format!(r#"
typedef unsigned char uint1; typedef unsigned short uint2;
typedef unsigned int uint4; typedef unsigned long long uint8;
typedef signed char int1; typedef short int2; typedef int int4; typedef long long int8;
typedef uint1 undefined1; typedef uint2 undefined2;
typedef uint4 undefined4; typedef uint8 undefined8;
{generated}
"#)).unwrap();
            process::required_output(
                Command::new("clang")
                    .args(["--target=sparc-unknown-linux-gnu", "-O2", "-S"])
                    .arg(&source).arg("-o").arg(&assembly),
            );
            let assembly = std::fs::read_to_string(assembly).unwrap();
            // Only Probe8 reads a byte. A packed multi-byte scalar here would
            // make Clang split the other native accesses into extra LDUBs.
            assert_eq!(
                assembly.lines().filter(|line| line.split_whitespace().next() == Some("ldub")).count(),
                1,
                "native SPARC scalar reads were split into byte accesses:\n{assembly}",
            );
        }
        let driver = dir.path().join(format!("{target}.c"));
        std::fs::write(
            &driver,
            include_str!("fixtures/conditional_read_native.c")
                .replace("GENERATED_FUNCTIONS", &generated),
        ).unwrap();
        for compiler in &compilers {
            for optimization in ["-O0", "-O2"] {
                let executable = dir.path().join(format!("{target}-{compiler}{optimization}"));
                process::required_output(
                    Command::new(compiler)
                        .args([
                            "-std=c11", optimization, "-fstrict-aliasing",
                            "-Werror=uninitialized", "-Werror=implicit-function-declaration",
                        ])
                        .arg(&driver).arg(&original).arg("-o").arg(&executable),
                );
                process::required_output(&mut Command::new(&executable));
            }
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn recovered_switch_dispatch_does_not_add_a_table_read() {
    use common::process;
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "native switch verification requires a C compiler");
    let mut storage = kuna_base::xml::DocumentStorage::new();
    let document = storage.parse_document(
        include_bytes!("../../../../tests/datatests/ifswitch.xml"),
    ).unwrap();
    let image = document.get_root().get_children().iter()
        .find(|element| element.get_name() == "binaryimage").unwrap();
    let chunk = image.get_children().iter()
        .find(|element| element.get_name() == "bytechunk").unwrap();
    let hex: Vec<_> = chunk.get_content().iter().copied()
        .filter(|byte| !byte.is_ascii_whitespace()).collect();
    let bytes: Vec<_> = hex.chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect();
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("switch.bin");
    std::fs::write(&input, &bytes).unwrap();
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&input)
            .args([
                "0x100000", "--addr", "--raw-image", "--base", "0x100000",
                "--target", "x86:LE:64:default:gcc", "--mode", "reliable",
                "--assert", "function 0x100000=table_probe",
                "--assert", "prototype table_probe int table_probe(int value)", "--assert-strict",
            ]),
    );
    let printed = std::str::from_utf8(&output.stdout).unwrap();
    let machine = bytes.iter().map(|byte| byte.to_string()).collect::<Vec<_>>().join(",");
    let driver = directory.path().join("switch.c");
    std::fs::write(&driver, format!(r#"
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <string.h>
#include <sys/mman.h>
typedef int32_t int4; typedef uint32_t uint4;
typedef int64_t int8; typedef uint64_t uint8;
{printed}
static const unsigned char machine[] = {{{machine}}};
static void bad_read(int signal) {{ (void)signal; _Exit(77); }}
int main(void) {{
    signal(SIGSEGV, bad_read);
    void *code = mmap(0,sizeof machine,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
    if (code == MAP_FAILED) return 90;
    memcpy(code,machine,sizeof machine);
    if (mprotect(code,sizeof machine,PROT_READ|PROT_EXEC)) return 91;
    int (*reference)(int) = (int (*)(int))code;
    for (int value=-32; value<=256; ++value) {{
        int expected=reference(value), actual=table_probe(value);
        if (expected != actual) {{
            fprintf(stderr,"selector=%d expected=%d actual=%d\n",value,expected,actual);
            return 1;
        }}
    }}
    return munmap(code,sizeof machine) ? 92 : 0;
}}
"#)).unwrap();
    for compiler in compilers {
        for level in ["-O0", "-O2"] {
            let executable = directory.path().join(format!("switch-{compiler}{level}"));
            process::required_output(
                Command::new(compiler).args(["-std=c11", level])
                    .arg(&driver).arg("-o").arg(&executable),
            );
            process::required_output(&mut Command::new(&executable));
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn retained_scalar_reads_do_not_require_natural_alignment() {
    use common::process;
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "native read verification requires a C compiler");
    let directory = tempfile::tempdir().unwrap();
    let mut c = String::new();
    let mut rust = String::new();
    let mut declarations = String::new();
    for (width, hex) in [
        (8, "85f674028a07c3"), (16, "85f67403668b07c3"),
        (32, "85f674028b07c3"), (64, "85f67403488b07c3"),
    ] {
        let name = format!("Probe{width}");
        let input = directory.path().join(format!("{name}.bin"));
        let bytes: Vec<_> = (0..hex.len()).step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap()).collect();
        std::fs::write(&input, bytes).unwrap();
        for language in ["c", "rust"] {
            let output = process::required_output(
                Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .arg("decompile").arg(&input)
                    .args([
                        "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                        "--target", "x86:LE:64:default:gcc", "--mode", "reliable",
                        "--assert", &format!("function 0x1000={name}"),
                        "--assert", &format!("prototype {name} void {name}(void *base,unsigned int active)"),
                        "--assert-strict", "--language", language,
                    ]),
            );
            let printed = std::str::from_utf8(&output.stdout).unwrap();
            if language == "c" {
                c.push_str(printed);
            } else {
                // A discarded machine read must not create an initialized
                // integer requirement. The compiled page oracle below still
                // checks that the opaque read occurs at the original width.
                assert!(printed.contains("MaybeUninit"), "{printed}");
                assert!(!printed.contains("assume_init"), "{printed}");
                rust.push_str(printed);
            }
        }
        declarations.push_str(&format!("void {name}(void *, unsigned);\n"));
        rust.push_str(&format!(r#"
#[export_name = "{name}"]
unsafe extern "C" fn Native{name}(base: *mut (), active: u32) {{
    {name}(base, active);
}}
"#));
    }
    let fixture = include_str!("fixtures/native_read_alignment.c");
    let c_source = directory.path().join("alignment.c");
    std::fs::write(&c_source, fixture.replace("GENERATED_FUNCTIONS", &c)).unwrap();
    for compiler in &compilers {
        for level in ["-O0", "-O2"] {
            let executable = directory.path().join(format!("{compiler}{level}"));
            process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", "-DC_MAIN", level, "-fsanitize=alignment", "-fsanitize-undefined-trap-on-error"])
                    .arg(&c_source).arg("-o").arg(&executable),
            );
            process::required_output(&mut Command::new(executable));
        }
    }
    let oracle_source = directory.path().join("oracle.c");
    let oracle = directory.path().join("oracle.o");
    std::fs::write(&oracle_source, fixture.replace("GENERATED_FUNCTIONS", &declarations)).unwrap();
    process::required_output(
        Command::new(compilers[0]).args(["-std=c11", "-O2", "-c"])
            .arg(&oracle_source).arg("-o").arg(&oracle),
    );
    rust.push_str(r#"
extern "C" { fn run_native_read_alignment() -> i32; }
fn main() { assert_eq!(unsafe { run_native_read_alignment() }, 0); }
"#);
    let rust_source = directory.path().join("alignment.rs");
    std::fs::write(&rust_source, &rust).unwrap();
    for level in ["0", "2"] {
        let executable = directory.path().join(format!("rust-{level}"));
        process::required_output(
            Command::new("rustc")
                .args(["--edition", "2021", "-C", &format!("opt-level={level}"), "-C"])
                .arg(format!("link-arg={}", oracle.display()))
                .arg(&rust_source).arg("-o").arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn a_retained_null_guard_is_a_rust_expression_not_a_sanitized_name() {
    use common::process;
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("null-guard.bin");
    // test rdi,rdi; je return; mov eax,[rdi]; ret.
    std::fs::write(&input, [0x48,0x85,0xff,0x74,0x02,0x8b,0x07,0xc3]).unwrap();
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&input)
            .args([
                "0x1000", "--addr", "--raw-image", "--base", "0x1000",
                "--target", "x86:LE:64:default:gcc", "--mode", "reliable",
                "--assert", "function 0x1000=probe",
                "--assert", "prototype probe void probe(void *base)",
                "--assert-strict", "--language", "rust",
            ]),
    );
    let printed = std::str::from_utf8(&output.stdout).unwrap();
    let source = directory.path().join("null_guard.rs");
    std::fs::write(&source, format!(r#"
{printed}
fn main() {{
    let data = [0u64; 4];
    unsafe {{
        probe(core::ptr::null_mut());
        probe((data.as_ptr() as *mut u8).add(1) as _);
    }}
}}
"#)).unwrap();
    for level in ["0", "2"] {
        let executable = directory.path().join(format!("null-guard-{level}"));
        process::required_output(
            Command::new("rustc").args(["--edition", "2021", "-C"])
                .arg(format!("opt-level={level}"))
                .arg(&source).arg("-o").arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn retained_rust_reads_do_not_capture_names_in_the_address_expression() {
    use common::process;
    let compiler = ["gcc", "clang"].into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .expect("native read verification requires a C compiler");
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("callee.c");
    let binary = directory.path().join("callee.so");
    std::fs::write(&original, r#"
static unsigned int word;
__attribute__((noinline)) void *NativeRead(void) { return &word; }
__attribute__((noinline)) void *address(void) { return &word; }
void probe(void) {
    (void)*(volatile unsigned int *)NativeRead();
    (void)*(volatile unsigned int *)address();
}
"#).unwrap();
    process::required_output(
        Command::new(compiler).args(["-O2", "-shared", "-fPIC"])
            .arg(&original).arg("-o").arg(&binary),
    );
    let output = process::required_output(
        Command::new(env!("CARGO_BIN_EXE_kuna"))
            .arg("decompile").arg(&binary).arg("probe")
            .args([
                "--mode", "reliable", "--language", "rust", "--assert-strict",
                "--assert", "prototype probe void probe(void)",
                "--assert", "prototype NativeRead void *NativeRead(void)",
                "--assert", "prototype address void *address(void)",
            ]),
    );
    let printed = std::str::from_utf8(&output.stdout).unwrap();
    let source = directory.path().join("callee.rs");
    std::fs::write(&source, format!(r#"
#![allow(non_snake_case)]
{printed}
use std::sync::atomic::{{AtomicUsize, Ordering}};
static CALLS: AtomicUsize = AtomicUsize::new(0);
static WORD: u32 = 0;
unsafe fn NativeRead() -> *mut () {{
    CALLS.fetch_add(1, Ordering::Relaxed);
    &WORD as *const u32 as *mut ()
}}
unsafe fn address() -> *mut () {{
    CALLS.fetch_add(1, Ordering::Relaxed);
    &WORD as *const u32 as *mut ()
}}
fn main() {{
    unsafe {{ probe(); }}
    assert_eq!(CALLS.load(Ordering::Relaxed), 2);
}}
"#)).unwrap();
    for level in ["0", "2"] {
        let executable = directory.path().join(format!("callee-{level}"));
        process::required_output(
            Command::new("rustc").args(["--edition", "2021", "-C"])
                .arg(format!("opt-level={level}"))
                .arg(&source).arg("-o").arg(&executable),
        );
        process::required_output(&mut Command::new(executable));
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn fusing_native_loads_preserves_the_wide_return_abi() {
    use common::process;
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "return ABI verification requires a C compiler");
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("pair.elf");
    std::fs::write(&input, include_bytes!("fixtures/native_read_pair.elf")).unwrap();
    let mut generated = String::new();
    for name in ["probe", "narrow"] {
        let output = process::required_output(
            Command::new(env!("CARGO_BIN_EXE_kuna"))
                .arg("decompile").arg(&input).arg(name).args(["--language", "c"]),
        );
        generated.push_str(std::str::from_utf8(&output.stdout).unwrap());
    }
    let source = directory.path().join("pair.c");
    std::fs::write(&source, format!(r#"
#include <stdlib.h>
typedef unsigned long undefined8;
typedef unsigned __int128 undefined16;
static unsigned long guard_word;
unsigned long *guard_address = &guard_word;
void fill(unsigned int ignored, void *value, ...) {{
    (void)ignored;
    ((unsigned long *)value)[0] = 0x1234;
    ((unsigned long *)value)[1] = 0x5678;
}}
void __stack_chk_fail(unsigned long ignored) {{ (void)ignored; abort(); }}
{generated}
int main(void) {{
    // A real typed consumer must accept both return registers. Do not execute
    // this opaque-callee frame reconstruction: that would also test its
    // independent stack-object layout recovery.
    unsigned __int128 (*wide)(void) = probe;
    unsigned long value = 0x123456789abcdef0UL;
    _Static_assert(sizeof(probe()) == 16, "lost second return register");
    if (sizeof(wide()) != 16 || sizeof(narrow(&value)) != 8) return 1;
    return narrow(&value) != value;
}}
"#)).unwrap();
    for compiler in compilers {
        for level in ["-O0", "-O2"] {
            let executable = directory.path().join(format!("{compiler}{level}"));
            process::required_output(
                Command::new(compiler)
                    .args(["-std=c11", level, "-Werror=incompatible-pointer-types"])
                    .arg(&source).arg("-o").arg(&executable),
            );
            process::required_output(&mut Command::new(executable));
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn opaque_record_reads_use_byte_addresses_without_invented_members() {
    use common::process;
    let compilers: Vec<_> = ["gcc", "clang"].into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    assert!(!compilers.is_empty(), "opaque record verification requires a C compiler");
    let directory = tempfile::tempdir().unwrap();
    let fixture = include_str!("fixtures/native_read_opaque.c");
    let original = directory.path().join("original.c");
    let image = directory.path().join("opaque.so");
    std::fs::write(&original, fixture).unwrap();
    process::required_output(
        Command::new(compilers[0]).args(["-shared", "-fPIC", "-O2", "-g0", "-DORIGINAL_ONLY"])
            .arg(&original).arg("-o").arg(&image),
    );
    let mut printed = String::new();
    for name in ["Probe", "ProbeRead", "ReadAlignment", "WriteAlignment"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.arg("decompile").arg(&image).arg(name);
        if name == "Probe" {
            command.args(["--assert", "prototype Probe void Probe(void **slot)", "--assert-strict"]);
        }
        let output = process::required_output(&mut command);
        printed.push_str(std::str::from_utf8(&output.stdout).unwrap());
    }
    let source = directory.path().join("recovered.c");
    std::fs::write(&source, fixture.replace("GENERATED_FUNCTIONS", &printed)).unwrap();
    for compiler in compilers {
        for level in ["-O0", "-O2"] {
            for (name, input, reference) in [("reference", &original, true), ("recovered", &source, false)] {
                let executable = directory.path().join(format!("{compiler}{level}-{name}"));
                let mut command = Command::new(compiler);
                command.args(["-std=gnu17", level, "-fsanitize=undefined", "-fno-sanitize-recover=all"]);
                if reference {
                    command.arg("-DNATIVE_REFERENCE");
                }
                process::required_output(command.arg(input).arg("-o").arg(&executable));
                process::required_output(&mut Command::new(executable));
            }
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn rust_opaque_reads_preserve_guard_assignments_and_pointer_types() {
    use common::process;
    let compiler = ["gcc", "clang"].into_iter()
        .find(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .expect("opaque Rust verification requires a C compiler");
    let directory = tempfile::tempdir().unwrap();
    let fixture = include_str!("fixtures/native_read_opaque.c");
    let original = directory.path().join("original.c");
    let image = directory.path().join("opaque.so");
    std::fs::write(&original, fixture).unwrap();
    process::required_output(Command::new(compiler)
        .args(["-shared", "-fPIC", "-O2", "-g0", "-DORIGINAL_ONLY"])
        .arg(&original).arg("-o").arg(&image));
    let mut rust = String::from(r#"
#![allow(non_camel_case_types, non_snake_case, unused_mut, unused_variables, unused_assignments, unused_parens)]
type obstack = core::ffi::c_void;
extern "C" {
    fn _obstack_newchunk(object: *mut obstack, length: i32);
    fn obstack_free(object: *mut (), position: *mut ());
    fn free(object: *mut ());
    fn run_native_opaque() -> i32;
}
"#);
    for name in ["Probe", "ProbeRead", "ReadAlignment", "WriteAlignment"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
        command.arg("decompile").arg(&image).arg(name).args(["--language", "rust"]);
        if name == "Probe" {
            command.args(["--assert", "prototype Probe void Probe(void **slot)", "--assert-strict"]);
        }
        let output = process::required_output(&mut command);
        rust.push_str(std::str::from_utf8(&output.stdout).unwrap());
    }
    rust.push_str(r#"
#[export_name = "Probe"]
unsafe extern "C" fn native_probe(slot: *mut *mut ()) { Probe(slot); }
#[export_name = "ProbeRead"]
unsafe extern "C" fn native_read(object: *mut obstack, active: u32) { ProbeRead(object, active as _); }
#[export_name = "ReadAlignment"]
unsafe extern "C" fn native_get(object: *mut obstack) -> i32 { ReadAlignment(object) as _ }
#[export_name = "WriteAlignment"]
unsafe extern "C" fn native_set(object: *mut obstack, value: i32) { WriteAlignment(object, value as _); }
fn main() { assert_eq!(unsafe { run_native_opaque() }, 0); }
"#);
    let oracle_source = directory.path().join("oracle.c");
    let oracle = directory.path().join("oracle.o");
    let declarations = "void Probe(void **); void ProbeRead(obstack *, unsigned); int ReadAlignment(obstack *); void WriteAlignment(obstack *, int);";
    std::fs::write(&oracle_source, fixture.replace("GENERATED_FUNCTIONS", declarations)).unwrap();
    process::required_output(Command::new(compiler)
        .args(["-std=gnu17", "-O2", "-Dmain=run_native_opaque", "-c"])
        .arg(&oracle_source).arg("-o").arg(&oracle));
    let source = directory.path().join("opaque.rs");
    std::fs::write(&source, rust).unwrap();
    for level in ["0", "2"] {
        let executable = directory.path().join(format!("rust-{level}"));
        process::required_output(Command::new("rustc")
            .args(["--edition=2021", "-C"]).arg(format!("opt-level={level}"))
            .arg("-C").arg(format!("link-arg={}", oracle.display()))
            .arg(&source).arg("-o").arg(&executable));
        process::required_output(&mut Command::new(executable));
    }
}
