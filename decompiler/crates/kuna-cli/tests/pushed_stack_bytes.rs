//! Pushed scratch storage, partial reads, snapshots and overlapping byte copies (#810).
#[path = "common/arm_images.rs"]
mod arm_images;
mod common;

use std::process::Command;
use std::time::Duration;

const FUNCTIONS: &[(u64, u64, &str, &str)] = &[
    (
        0x10000,
        0x38,
        "PushedByteSum",
        "const unsigned char *source",
    ),
    (
        0x10038,
        0x38,
        "StackByteCopyViews",
        "const unsigned char *source",
    ),
    (
        0x10070,
        0x5c,
        "PushedByteBranches",
        "const unsigned char *source, unsigned int selector",
    ),
    (
        0x100cc,
        0x2c,
        "PushedByteSeed",
        "const unsigned char *source, unsigned int seed",
    ),
    (
        0x100f8,
        0x34,
        "PushedByteSnapshot",
        "const unsigned char *source, unsigned int seed",
    ),
    (0x1012c, 0x2c, "PushedByteOverlap", "unsigned int seed"),
    (
        0x10158,
        0x60,
        "PushedByteBranchesSeed",
        "const unsigned char *source, unsigned int selector, unsigned int seed",
    ),
    (
        0x101b8,
        0x38,
        "LargeStackByteCopy",
        "const unsigned char *source",
    ),
    (
        0x101f0,
        0x38,
        "DynamicStackByteCopy",
        "const unsigned char *source, unsigned int count",
    ),
];

fn fixture() -> (std::path::PathBuf, std::path::PathBuf) {
    let xml = include_str!("../../../../tests/stages/kuna-stack-byte-copy.xml");
    let chunk = xml
        .split_once("<bytechunk ")
        .unwrap()
        .1
        .split_once('>')
        .unwrap()
        .1
        .split_once("</bytechunk>")
        .unwrap()
        .0;
    let code: Vec<_> = chunk
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    let symbols: Vec<_> = FUNCTIONS
        .iter()
        .map(|&(address, size, name, _)| (address - 0x10000, name, size))
        .collect();
    let image = common::scratch_file("pushed-stack-bytes", "elf");
    std::fs::write(&image, arm_images::elf(&code, &[(0, "$a")], &symbols)).unwrap();
    let assertions = image.with_extension("kuna");
    let directives: String = FUNCTIONS.iter().map(|&(address, size, name, args)| {
        format!("function {address:#x}-{:#x}={name}\nprototype {address:#x} unsigned int {name}({args})\n", address + size)
    }).collect();
    std::fs::write(&assertions, directives).unwrap();
    (image, assertions)
}

fn decompile(mode: &str) -> Vec<(String, String)> {
    let (image, assertions) = fixture();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .arg("decompile-all")
        .arg(&image)
        .args([
            "--mode",
            "reliable",
            "--jobs",
            "1",
            "--json",
            "--isa",
            "arm",
            "--target",
            "ARM:LE:32:v5t:default",
            "--assert-strict",
            "--assert",
        ])
        .arg(format!("@{}", assertions.display()))
        .arg("--sleighpath")
        .arg(common::repo_root().join("specs"));
    for &(address, _, _, _) in FUNCTIONS {
        command.args(["--addr", &format!("{address:#x}")]);
    }
    match mode {
        "views" => {
            command.args([
                "--option",
                "stackviews",
                "on",
                "--option",
                "structdefs",
                "on",
            ]);
        }
        "alias" => {
            command.args(["--option", "stackalias", "on"]);
        }
        "off" => {
            command.args(["--option", "stackstoreguard", "off"]);
        }
        _ => (),
    }
    let output = common::process::output_with_timeout(
        &mut command,
        Duration::from_secs(30),
        Duration::from_millis(25),
    )
    .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(payload["error"].is_null(), "{payload}");
    for assertion in payload["assertions"].as_array().unwrap() {
        assert_eq!(assertion["status"], "applied", "{assertion}");
    }
    payload["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|function| {
            assert!(function["error"].is_null(), "{function}");
            let name = function["name"].as_str().unwrap();
            let &(address, size, _, _) = FUNCTIONS
                .iter()
                .find(|&&(_, _, candidate, _)| candidate == name)
                .unwrap();
            assert_eq!(function["address"].as_u64().unwrap(), address);
            assert_eq!(function["size"].as_u64().unwrap(), size);
            (
                name.to_owned(),
                function["code"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

const PRELUDE: &str = r#"
typedef unsigned char uint1; typedef unsigned short uint2;
typedef unsigned int uint4; typedef int int4; typedef unsigned long long uint8;
typedef unsigned char undefined1; typedef unsigned short undefined2;
typedef unsigned int undefined4; typedef unsigned long long undefined8;
#define CONCAT11(x,y) ((uint2)(((uint2)(uint1)(x)<<8)|(uint1)(y)))
#define CONCAT12(x,y) (((uint4)(uint1)(x)<<16)|(uint2)(y))
#define CONCAT13(x,y) (((uint4)(uint1)(x)<<24)|((uint4)(y)&0xffffff))
#define CONCAT21(x,y) (((uint4)(uint2)(x)<<8)|(uint1)(y))
#define CONCAT22(x,y) (((uint4)(uint2)(x)<<16)|(uint2)(y))
#define CONCAT31(x,y) ((((uint4)(x)&0xffffff)<<8)|(uint1)(y))
"#;

fn compile(name: &str, mode: &str, code: &str) {
    let source = common::scratch_file(&format!("{name}-{mode}"), "c");
    std::fs::write(&source, format!("{PRELUDE}\n{code}\n")).unwrap();
    for compiler in ["arm-none-eabi-gcc", "clang"] {
        if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            eprintln!("{compiler} unavailable; skipping ARM diagnostic compilation");
            continue;
        }
        let mut command = Command::new(compiler);
        if compiler == "clang" {
            command.arg("--target=arm-none-eabi");
        } else {
            command.arg("-Werror=maybe-uninitialized");
        }
        command
            .args([
                "-march=armv5te",
                "-marm",
                "-O2",
                "-ffreestanding",
                "-Werror=uninitialized",
                "-Werror=implicit-function-declaration",
                "-c",
            ])
            .arg(&source)
            .arg("-o")
            .arg(source.with_extension(format!("{compiler}.o")));
        common::process::required_output(&mut command);
    }
}

fn execute(name: &str, mode: &str, code: &str) {
    if cfg!(target_endian = "big") {
        return;
    }
    let (arguments, expected) = match name {
        "PushedByteSum" => ("bytes", "bytes[0]+2*bytes[1]+4*bytes[2]"),
        "StackByteCopyViews" => ("bytes", "(word&0xffffff)|((uint4)bytes[1]<<24)"),
        "PushedByteBranches" => (
            "bytes, selector",
            "bytes[0]+(selector?4:2)*bytes[1]+(selector?2:4)*bytes[2]",
        ),
        "PushedByteBranchesSeed" => (
            "bytes, selector, seed",
            "bytes[0]+(selector?4:2)*bytes[1]+(selector?2:4)*bytes[2]",
        ),
        "PushedByteSeed" => ("bytes, seed", "(seed&0xff000000)|(word&0xffffff)"),
        "PushedByteSnapshot" => ("bytes, seed", "word^seed"),
        "PushedByteOverlap" => ("seed", "(seed&255)*0x01010101"),
        _ => unreachable!(),
    };
    let source = common::scratch_file(&format!("{name}-{mode}-execute"), "c");
    std::fs::write(
        &source,
        format!(
            r#"{PRELUDE}
{code}
int main(void) {{
    uint4 seeds[]={{0,1,0x78563412,0xffffffff}};
    for (unsigned i=0;i<256;i++) {{
        uint4 word=i|((i^0x5a)<<8)|((255-i)<<16)|0xa5000000;
        uint1 storage[7];
        for (unsigned alignment=0;alignment<4;alignment++) {{
        uint1 *bytes=storage+alignment;
        for (unsigned byte=0;byte<4;byte++) bytes[byte]=(uint1)(word>>(8*byte));
        for (unsigned j=0;j<4;j++) for (unsigned selector=0;selector<3;selector++) {{
            uint4 seed=seeds[j];
            uint4 expected={expected};
            if ({name}({arguments})!=expected) return 1;
        }}
        }}
    }}
    return 0;
}}
"#
        ),
    )
    .unwrap();
    for compiler in ["gcc", "clang"] {
        if common::process::optional_output(Command::new(compiler).arg("--version")).is_none() {
            eprintln!("{compiler} unavailable; skipping native emitted-C execution");
            continue;
        }
        let binary = source.with_extension(compiler);
        let mut command = Command::new(compiler);
        command.args([
            "-std=c11",
            "-O2",
            "-fstrict-aliasing",
            "-Werror=uninitialized",
            "-Werror=implicit-function-declaration",
        ]);
        if compiler == "gcc" {
            command.arg("-Werror=maybe-uninitialized");
        }
        common::process::required_output(command.arg(&source).arg("-o").arg(&binary));
        common::process::required_output(&mut Command::new(binary));
    }
}

#[test]
fn short_copies_keep_their_bytes_in_each_storage_mode() {
    for mode in ["default", "views", "alias"] {
        for (name, code) in decompile(mode) {
            if matches!(name.as_str(), "LargeStackByteCopy" | "DynamicStackByteCopy") {
                assert!(code.contains("while ("), "unproved loop expanded: {code}");
                continue;
            }
            assert!(
                !code.contains("while ("),
                "short copy stayed indirect: {code}"
            );
            assert!(
                !code.contains("._1_1_") && !code.contains("._2_1_"),
                "{code}"
            );
            if name == "PushedByteSnapshot" {
                assert!(code.contains("^ seed"), "snapshot lost: {code}");
            }
            compile(&name, mode, &code);
            execute(&name, mode, &code);
        }
    }
}

#[test]
fn disabling_the_guard_reproduces_the_stale_seed_and_lost_snapshot() {
    let functions = decompile("off");
    let code = &functions
        .iter()
        .find(|(name, _)| name == "PushedByteBranchesSeed")
        .unwrap()
        .1;
    assert!(code.contains("while ("), "{code}");
    assert!(code.contains("return (seed & 0xff)"), "{code}");
    let code = &functions
        .iter()
        .find(|(name, _)| name == "PushedByteSnapshot")
        .unwrap()
        .1;
    assert!(code.contains("return 0;"), "{code}");
}
