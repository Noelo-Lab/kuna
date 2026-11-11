//! A PPC64 switch whose index a call writes keeps every case of its table.

use crate::common;

use object::write::{Object, Symbol, SymbolSection};
use object::{
    Architecture, BinaryFormat, Endianness, FileFlags, SectionKind, SymbolFlags, SymbolKind,
    SymbolScope,
};
use std::process::Command;

/// `select` stores 0 to a stack slot, passes its address to `get_state`, which
/// writes it, then reads the slot back, guards it and dispatches through a
/// five-entry relative table (`base + sext(table[i])`, entries 0 and 4 sharing
/// a target). The base is `bcl`/`mflr` plus `addis`/`addi`, a constant expression
/// several ops deep. Each case returns its own constant.
const SELECT: [u32; 45] = [
    0x7c0802a6, 0xf8010010, 0xf821ff71, 0x39200000, 0x38a10070, 0x91210070, 0x48000091,
    0x2f83000b, 0x419e0074, 0x81210070, 0x2b890004, 0x419d0060, 0x7d2a4b78, 0x429f0005,
    0x7d0802a6, 0x3d080000, 0x39080020, 0x794a1764, 0x7d2852aa, 0x7d294214, 0x7d2903a6,
    0x4e800420, 0x00000014, 0x0000001c, 0x00000024, 0x0000002c, 0x00000014, 0x38600028,
    0x48000028, 0x38600029, 0x48000020, 0x3860002a, 0x48000018, 0x3860002b, 0x48000010,
    0x38600063, 0x48000008, 0x38600001, 0x38210090, 0xe8010010, 0x7c0803a6, 0x4e800020,
    0x90850000, 0x38600000, 0x4e800020,
];

/// [`SELECT`] with the slot stored as 6 and the guard `5 <= i && i <= 9` over a
/// ten-entry table whose rows 0 to 4 also hold mapped targets (returning 40 to
/// 44). Those rows never reach the switch, so they must not become cases.
const LOWER_BOUND: [u32; 64] = [
    0x7c0802a6, 0xf8010010, 0xf821ff71, 0x39200006, 0x38a10070, 0x91210070, 0x480000dd,
    0x2f83000b, 0x419e00c0, 0x81210070, 0x2b890005, 0x419c00ac, 0x2b890009, 0x419d00a4,
    0x7d2a4b78, 0x429f0005, 0x7d0802a6, 0x3d080000, 0x39080020, 0x794a1764, 0x7d2852aa,
    0x7d294214, 0x7d2903a6, 0x4e800420, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60, 0x68,
    0x70, 0x38600028, 0x48000058, 0x38600029, 0x48000050, 0x3860002a, 0x48000048, 0x3860002b,
    0x48000040, 0x3860002c, 0x48000038, 0x3860002d, 0x48000030, 0x3860002e, 0x48000028,
    0x3860002f, 0x48000020, 0x38600030, 0x48000018, 0x38600031, 0x48000010, 0x38600063,
    0x48000008, 0x38600001, 0x38210090, 0xe8010010, 0x7c0803a6, 0x4e800020, 0x90850000,
    0x38600000, 0x4e800020,
];

/// `n` copies of [`SELECT`]'s dispatch in a chain: case 1 of stage `k` runs
/// stage `k + 1`, so each restart reveals one more switch. Stage `k`'s cases
/// return `100 + 10k`, `+ 2`, `+ 3`, and its default `+ 9`.
fn nested(n: usize) -> Vec<u32> {
    const STAGE: usize = 31;
    let out = 3 + n * STAGE + 1;
    let get_state = out + 4;
    let branch = |op: u32, from: usize, to: usize| op | ((to as i64 - from as i64) as u32 * 4 & 0x03ff_fffc);
    let mut code = vec![0x7c0802a6, 0xf8010010, 0xf821ff71];
    for k in 0..n {
        let s = code.len();
        let value = |v: usize| 0x38600000 | (100 + 10 * k + v) as u32;
        code.extend([0x39200000, 0x38a10070, 0x91210070, branch(0x48000001, s + 3, get_state)]);
        code.extend([0x81210070, 0x2b890004, 0x419d0058, 0x7d2a4b78, 0x429f0005, 0x7d0802a6]);
        code.extend([0x3d080000, 0x39080020, 0x794a1764, 0x7d2852aa, 0x7d294214, 0x7d2903a6]);
        code.extend([0x4e800420, 0x14, 0x34, 0x1c, 0x24, 0x14]);
        for (slot, v) in [(22, 0), (24, 2), (26, 3), (28, 9)] {
            code.extend([value(v), branch(0x48000000, s + slot + 1, out)]);
        }
        if k + 1 < n {
            code.push(branch(0x48000000, s + 30, s + STAGE));
        } else {
            code.extend([0x38600007, branch(0x48000000, s + 31, out)]);
        }
    }
    assert_eq!(code.len(), out);
    code.extend([0x38210090, 0xe8010010, 0x7c0803a6, 0x4e800020]);
    code.extend([0x90850000, 0x38600000, 0x4e800020]);
    code
}

fn image(endian: Endianness, words: &[u32]) -> Vec<u8> {
    let mut object = Object::new(BinaryFormat::Elf, Architecture::PowerPc64, endian);
    object.flags = FileFlags::Elf { os_abi: 0, abi_version: 0, e_flags: 2 };
    let code: Vec<u8> = words
        .iter()
        .flat_map(|&n| match endian {
            Endianness::Big => n.to_be_bytes(),
            Endianness::Little => n.to_le_bytes(),
        })
        .collect();
    let text = object.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
    object.append_section_data(text, &code, 4);
    let helper = code.len() as u64 - 12;
    for (name, value, size) in [("select", 0, helper), ("get_state", helper, 12)] {
        object.add_symbol(Symbol {
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
    object.write().unwrap()
}

/// Decompile `select` from `words` in both byte orders, flattening whitespace.
fn decompile_both(name: &str, words: &[u32]) -> Vec<(Endianness, String, String)> {
    let path = common::scratch_file(name, "o");
    let mut results = Vec::new();
    for endian in [Endianness::Big, Endianness::Little] {
        std::fs::write(&path, image(endian, words)).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(["decompile", path.to_str().unwrap(), "select"])
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let code = String::from_utf8(output.stdout).unwrap();
        let flat = code.split_whitespace().collect::<Vec<_>>().join(" ");
        results.push((endian, code, flat));
    }
    std::fs::remove_file(path).unwrap();
    results
}

#[test]
fn a_call_written_switch_index_keeps_every_table_case() {
    for (endian, code, flat) in decompile_both("ppc64-delayed-constant-switch", &SELECT) {
        assert!(!code.contains("does not reproduce the recovered table"), "{endian:?}: {code}");
        for body in [
            "case 0: case 4: return 0x28;",
            "case 1: return 0x29;",
            "case 2: return 0x2a;",
            "case 3: return 0x2b;",
            "default: return 99;",
        ] {
            assert!(flat.contains(body), "{endian:?}: lost `{body}`: {code}");
        }
    }
}

#[test]
fn rows_a_lower_bound_guard_excludes_do_not_become_cases() {
    for (endian, code, flat) in decompile_both("ppc64-delayed-lower-bound", &LOWER_BOUND) {
        assert!(flat.contains("5 <="), "{endian:?}: dropped the lower bound: {code}");
        for value in 0x28..=0x2c {
            assert!(!flat.contains(&format!(": return {value:#x};")), "{endian:?}: row of {value:#x} became a case: {code}");
        }
    }
}

#[test]
fn an_exhausted_restart_budget_keeps_the_analyzed_body() {
    for (endian, code, _) in decompile_both("ppc64-delayed-nested-switch", &nested(9)) {
        assert!(!code.contains("structuring declined"), "{endian:?}: lost the body: {code}");
        assert!(code.contains("Exceeded maximum restarts with more pending"), "{endian:?}: {code}");
        assert!(code.matches("switch(").count() >= 8, "{endian:?}: {code}");
    }
}
