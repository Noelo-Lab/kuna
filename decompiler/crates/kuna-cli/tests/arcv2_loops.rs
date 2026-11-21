//! Synopsys ARCv2 (GH-1632): an `EM_ARC_COMPACT2` ELF selects the ARCv2 language
//! of its byte order, and a zero-overhead loop (`lp`) decompiles as a counted
//! `do .. while`. `fill`'s loop ends on a call, whose back-edge to the loop
//! start must not be listed as a second callee.
use crate::common;
use common::{repo_root, run_kuna};

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-cli/tests/fixtures/arcv2")
        .join(name)
        .to_str()
        .unwrap()
        .to_owned()
}

fn kuna(args: &[&str]) -> String {
    let (stdout, stderr, code) = run_kuna(args);
    assert_eq!(code, 0, "kuna {args:?}: {stderr}");
    stdout
}

#[test]
fn relocatable_object_decompiles_without_a_target() {
    let stdout = kuna(&["decompile", &fixture("add2.o"), "add2"]);
    assert!(stdout.contains("return a0 + a1;"), "{stdout}");
}

#[test]
fn zero_overhead_loops_are_counted_do_while_loops_in_both_byte_orders() {
    for elf in ["loops.elf", "loops_be.elf"] {
        let elf = fixture(elf);
        let names: Vec<String> = kuna(&["functions", &elf])
            .lines()
            .filter_map(|line| line.split('\t').nth(1).map(str::to_owned))
            .collect();
        assert_eq!(names, ["sum", "fill", "clamp", "sink"], "{elf}");

        let sum = kuna(&["decompile", &elf, "sum"]);
        for line in ["do {", "v1 += *v3;", "v2 -= 1;", "} while (v2);"] {
            assert!(sum.contains(line), "{elf}: {line} missing from\n{sum}");
        }
        let fill = kuna(&["decompile", &elf, "fill"]);
        for line in ["v5 = 8;", "} while (v5);", "sink(v4);", "return v4[a0 & 7];"] {
            assert!(fill.contains(line), "{elf}: {line} missing from\n{fill}");
        }
        let clamp = kuna(&["decompile", &elf, "clamp"]);
        for line in ["return a0 >> 2 | a1 & 0xff00ff;", "return a0 * 8 ^ a1;"] {
            assert!(clamp.contains(line), "{elf}: {line} missing from\n{clamp}");
        }
    }
}

#[test]
fn loop_end_call_has_one_callee() {
    let elf = fixture("loops.elf");
    let listing = kuna(&["disassemble", &elf, "fill"]);
    for row in ["lp 0x104c", "bl.d 0x1090", "st.ab r3, [r12, 0x4]", "ld.ab r13, [sp, 0x8]"] {
        assert!(listing.contains(row), "{row} missing from\n{listing}");
    }
    let (stdout, _, _) = run_kuna(&["xrefs", &elf, "--to", "0x1046"]);
    assert!(!stdout.contains("\tcall\t"), "the loop start is not a callee:\n{stdout}");
}
