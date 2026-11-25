//! RISC-V Zcmp/Zcmt (GH-1630): an ELF whose `Tag_RISCV_arch` lists `zcmp`
//! takes the `RISCV:LE:<size>:Zcmp` language and decodes the Zc stack and
//! jump-table instructions, where the default language reads `c.fsdsp`.
use crate::common;
use common::{repo_root, run_kuna};

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-cli/tests/fixtures/riscv_zcmp")
        .join(name)
        .to_str()
        .unwrap()
        .to_owned()
}

fn rows(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .map(|line| line.split_whitespace().skip(2).collect::<Vec<_>>().join(" "))
        .collect()
}

fn disassemble(args: &[&str]) -> Vec<String> {
    let (stdout, stderr, code) = run_kuna(&[&["disassemble"][..], args].concat());
    assert_eq!(code, 0, "kuna disassemble {args:?}: {stderr}");
    rows(&stdout)
}

const PROBE: [&str; 23] = [
    "cm.push {ra},-0x10",
    "cm.push {ra,s0},-0x20",
    "cm.push {ra,s0-s2},-0x10",
    "cm.push {ra,s0-s3},-0x20",
    "cm.push {ra,s0-s7},-0x30",
    "cm.push {ra,s0-s8},-0x60",
    "cm.push {ra,s0-s11},-0x40",
    "cm.push {ra,s0-s11},-0x70",
    "cm.pop {ra},0x10",
    "cm.pop {ra,s0-s4},0x20",
    "cm.pop {ra,s0-s11},0x50",
    "cm.popret {ra,s0-s5},0x20",
    "cm.popret {ra,s0-s11},0x70",
    "cm.popretz {ra,s0-s1},0x20",
    "cm.popretz {ra,s0-s9},0x40",
    "cm.mvsa01 s0,s1",
    "cm.mvsa01 s7,s2",
    "cm.mva01s s1,s0",
    "cm.mva01s s3,s7",
    "cm.jt 0x0",
    "cm.jt 0x1f",
    "cm.jalt 0x20",
    "cm.jalt 0xff",
];

#[test]
fn every_zc_encoding_shape_decodes_as_llvm_names_it() {
    let elf = fixture("zc_ops.elf");
    assert_eq!(disassemble(&[&elf, "probe"]), PROBE);
    let default = disassemble(&[&elf, "probe", "--target", "RISCV:LE:32:default:gcc"]);
    assert!(
        default.iter().all(|row| row.starts_with("c.fsdsp ")),
        "the default language keeps the c.fsdsp reading: {default:?}"
    );
}

#[test]
fn zcmp_prologue_and_epilogue_recover_an_ordinary_function() {
    let elf = fixture("keep.elf");
    let listing = disassemble(&[&elf, "keep"]);
    assert_eq!(listing.first().map(String::as_str), Some("cm.push {ra,s0-s1},-0x10"));
    assert_eq!(listing.last().map(String::as_str), Some("cm.popret {ra,s0-s1},0x10"));
    let (stdout, stderr, code) = run_kuna(&["decompile", &elf, "keep"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(!stdout.contains("jump-as-call"), "{stdout}");
    assert!(!stdout.contains("fs8"), "{stdout}");
    for line in ["v1 = ext(a0);", "ext(a1);", "ext(v1);"] {
        assert!(stdout.contains(line), "{line} missing from\n{stdout}");
    }
}

#[test]
fn rv64_zcmp_saves_eight_byte_registers() {
    let elf = fixture("zc_ops64.elf");
    let listing = disassemble(&[&elf, "swapcall"]);
    assert_eq!(listing.first().map(String::as_str), Some("cm.push {ra,s0-s1},-0x20"));
    for (function, body) in [
        ("swapcall", "return ext2(a1,a0) + 1;"),
        ("zret", "return 0;"),
        ("popsep", "return ext() + a0;"),
    ] {
        for file in [fixture("zc_ops.elf"), elf.clone()] {
            let (stdout, stderr, code) = run_kuna(&["decompile", &file, function]);
            assert_eq!(code, 0, "{stderr}");
            assert!(stdout.contains(body), "{function} in {file}:\n{stdout}");
        }
    }
}
