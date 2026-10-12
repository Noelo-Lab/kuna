#![cfg(all(target_arch = "x86_64", target_os = "linux"))]

use crate::common;

use common::process;
use std::path::Path;
use std::process::Command;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/mapped_array_c")
        .join(name)
}

fn kuna(args: &[String]) -> serde_json::Value {
    let runner =
        std::env::var_os("KUNA_TEST_RUNNER").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let output = process::required_output(
        Command::new(runner)
            .env(
                "KUNA_BINARY",
                std::env::var_os("KUNA_BINARY")
                    .unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into()),
            )
            .env("KUNA_SPECS", common::repo_root().join("specs"))
            .env("KUNA_DECODE_JOBS", "1")
            .args(args),
    );
    serde_json::from_slice(&output.stdout).expect("CLI JSON")
}

fn compile(cc: &str, level: &str, source: &Path, driver: &Path, executable: &Path) {
    process::required_output(
        Command::new(cc)
            .args([
                "-std=c11",
                level,
                "-fstrict-aliasing",
                "-fsanitize=undefined",
                "-fno-sanitize-recover=all",
                "-Werror",
                "-no-pie",
            ])
            .arg(source)
            .arg(driver)
            .arg("-o")
            .arg(executable),
    );
}

#[test]
fn mapped_byte_accesses_compile_and_preserve_bytes() {
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping mapped-array round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("mapped-array-c", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    let driver = fixture("driver.c");
    let native = dir.join("native");
    compile(
        compilers[0],
        "-O0",
        &fixture("access_x86_64.S"),
        &driver,
        &native,
    );
    process::required_output(&mut Command::new(&native));
    let symbols = process::required_output(Command::new("nm").arg(&native));
    let symbols = String::from_utf8(symbols.stdout).unwrap();
    let address = |name: &str| {
        symbols
            .lines()
            .find(|line| line.ends_with(&format!(" {name}")))
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned()
    };
    let prototypes = [
        "void write1(unsigned char x)",
        "void write2(unsigned short x)",
        "void write4(unsigned int x)",
        "void write8(unsigned long long x)",
        "unsigned char read1(void)",
        "unsigned short read2(void)",
        "unsigned int read4(void)",
        "unsigned long long read8(void)",
        "int read_signed4(void)",
        "void whole_write(unsigned long long x)",
        "unsigned long long whole_read(void)",
        "void overlap_copy(unsigned short x)",
        "unsigned int read_const4(void)",
        "void whole_write1(unsigned char x)",
        "unsigned char whole_read1(void)",
        "void whole_write2(unsigned short x)",
        "unsigned short whole_read2(void)",
        "void whole_write4(unsigned int x)",
        "unsigned int whole_read4(void)",
    ];
    let names = "write1,write2,write4,write8,read1,read2,read4,read8,read_signed4,whole_write,whole_read,overlap_copy,read_const4,whole_write1,whole_read1,whole_write2,whole_read2,whole_write4,whole_read4";
    let mut args: Vec<String> = [
        "decompile-all",
        native.to_str().unwrap(),
        "--functions",
        names,
        "--mode",
        "reliable",
        "--language",
        "c",
        "--jobs",
        "1",
        "--target",
        "x86:LE:64:default:gcc",
        "--option",
        "readonly",
        "off",
        "--assert-strict",
        "--json",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    for (name, size) in [
        ("bytes", 32),
        ("whole", 8),
        ("readonly_bytes", 8),
        ("whole1", 1),
        ("whole2", 2),
        ("whole4", 4),
    ] {
        args.extend([
            "--assert".into(),
            format!("data 0x{} unsigned char {name}[{size}]", address(name)),
        ]);
    }
    for proto in prototypes {
        let name = proto
            .split('(')
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap();
        args.extend(["--assert".into(), format!("prototype {name} {proto}")]);
    }
    let code = |result: serde_json::Value| {
        let functions = result["functions"].as_array().expect("functions");
        assert_eq!(functions.len(), 19);
        functions
            .iter()
            .map(|f| f["code"].as_str().expect("emitted C"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let original = code(kuna(&args));
    assert!(original.contains("bytes._3_4_"), "{original}");
    assert!(original.contains("whole._0_8_"), "{original}");
    args.extend(["--option".into(), "bytearrayc".into(), "off".into()]);
    assert_eq!(
        original,
        code(kuna(&args)),
        "explicit off must match default"
    );
    *args.last_mut().unwrap() = "on".into();
    let printed = code(kuna(&args));
    assert!(!printed.contains("._"), "{printed}");
    assert!(
        printed.contains("&readonly_bytes[1]"),
        "const source was not exercised: {printed}"
    );
    let source = dir.join("printed.c");
    std::fs::write(&source, format!(
        "#include <stdint.h>\n#include \"{}\"\ntypedef unsigned char uint1;\ntypedef unsigned short uint2;\ntypedef unsigned int uint4;\ntypedef unsigned long long uint8;\ntypedef int int4;\nunsigned char bytes[32], whole[8], whole1[1], whole2[2], whole4[4];\nconst unsigned char readonly_bytes[8] = {{0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88}};\n{printed}", fixture("bits.h").display()
    )).unwrap();
    for cc in compilers {
        for level in ["-O0", "-O2"] {
            let executable = dir.join(format!("printed-{cc}-{level}"));
            compile(cc, level, &source, &driver, &executable);
            process::required_output(&mut Command::new(executable));
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn arm_copy_uses_target_endian_and_preserves_struct_and_volatile_controls() {
    if process::optional_output(Command::new("arm-none-eabi-as").arg("--version")).is_none() {
        eprintln!("skipping ARM mapped-array round trip: no ARM binutils");
        return;
    }
    let compilers: Vec<_> = ["gcc", "clang"]
        .into_iter()
        .filter(|cc| process::optional_output(Command::new(cc).arg("--version")).is_some())
        .collect();
    if compilers.is_empty() {
        eprintln!("skipping ARM mapped-array round trip: no GCC or Clang");
        return;
    }
    let dir = common::scratch_file("mapped-array-arm", "dir");
    std::fs::create_dir_all(&dir).unwrap();
    for big in [false, true] {
        let endian = if big { "BE" } else { "LE" };
        let object = dir.join(format!("copy-{endian}.o"));
        let elf = dir.join(format!("copy-{endian}.elf"));
        process::required_output(
            Command::new("arm-none-eabi-as")
                .args(["-march=armv5t", if big { "-EB" } else { "-EL" }, "-o"])
                .arg(&object)
                .arg(fixture("copy_arm.s")),
        );
        process::required_output(
            Command::new("arm-none-eabi-ld")
                .args([
                    if big { "-EB" } else { "-EL" },
                    "-Ttext=0x10000",
                    "-Tdata=0x11000",
                    "-e",
                    "CopyFiveWords",
                    "-o",
                ])
                .arg(&elf)
                .arg(&object),
        );
        let mut args: Vec<String> = [
            "decompile-all",
            elf.to_str().unwrap(),
            "--mode",
            "reliable",
            "--language",
            "c",
            "--jobs",
            "1",
            "--isa",
            "arm",
            "--assert-strict",
            "--addr",
            "0x10000",
            "--json",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        args.extend([
            "--target".into(),
            format!("ARM:{endian}:32:v5t:default"),
            "--assert".into(),
            format!("@{}", fixture("copy_array.kuna").display()),
        ]);
        let code = |v: serde_json::Value| v["functions"][0]["code"].as_str().unwrap().to_owned();
        let original = code(kuna(&args));
        assert_eq!(
            original.matches("mutable_tile_fallback._").count(),
            5,
            "{original}"
        );
        args.extend(["--option".into(), "bytearrayc".into(), "on".into()]);
        let printed = code(kuna(&args));
        assert!(!printed.contains("._"), "{printed}");
        let source = dir.join(format!("printed-{endian}.c"));
        std::fs::write(&source,format!(
            "typedef unsigned int uint4;\nextern unsigned int source_words[5];\nextern unsigned char mutable_tile_fallback[20];\n{printed}"
        )).unwrap();
        for cc in &compilers {
            for level in ["-O0", "-O2"] {
                let executable = dir.join(format!("printed-{endian}-{cc}-{level}"));
                process::required_output(
                    Command::new(cc)
                        .args([
                            "-std=c11",
                            level,
                            "-fstrict-aliasing",
                            "-fsanitize=undefined",
                            "-fno-sanitize-recover=all",
                        ])
                        .arg(format!("-DBIG_ENDIAN_TARGET={}", u8::from(big)))
                        .arg(&source)
                        .arg(fixture("copy_driver.c"))
                        .arg("-o")
                        .arg(&executable),
                );
                process::required_output(&mut Command::new(executable));
            }
        }
        args.truncate(args.len() - 3);
        *args.last_mut().unwrap() = format!("@{}", fixture("copy_struct.kuna").display());
        let structure = code(kuna(&args));
        args.extend(["--option".into(), "bytearrayc".into(), "on".into()]);
        assert_eq!(
            structure,
            code(kuna(&args)),
            "healthy struct output changed"
        );
        let struct_source = dir.join("struct.c");
        std::fs::write(&struct_source,format!(
            "typedef unsigned int uint4;\nextern unsigned int source_words[5];\nstruct MutableTileFallbackWords {{ unsigned int word_00, word_04, word_08, word_0c, word_10; }};\nextern struct MutableTileFallbackWords mutable_tile_fallback;\n{structure}"
        )).unwrap();
        for cc in &compilers {
            process::required_output(
                Command::new(cc)
                    .args(["-std=c11", "-fsyntax-only"])
                    .arg(&struct_source),
            );
        }
        args.truncate(args.len() - 3);
        *args.last_mut().unwrap() = format!("@{}", fixture("copy_array.kuna").display());
        args.extend(["--assert".into(), "volatile 0x11014+20".into()]);
        let volatile = code(kuna(&args));
        args.extend(["--option".into(), "bytearrayc".into(), "on".into()]);
        assert_eq!(
            volatile,
            code(kuna(&args)),
            "wide volatile effects must not become byte stores"
        );
    }
    std::fs::remove_dir_all(dir).unwrap();
}
