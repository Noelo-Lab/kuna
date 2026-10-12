//! What the whole-program float-global scan (`floatglobals`) finds, pinned on
//! linked images of `floatglobal.c` built for x86-64, AArch64 and hard-float
//! ARM.
//!
//! `gd`, `gf`, `gd2` and `gf2` are only ever loaded into or stored from a float
//! register; `gi` is an `int`; `gpun` takes a float's bits and is added to as an
//! integer; `gz` is a `double` zeroed by an integer store at -O2 and through
//! `xmm0` at clang -O0.

use std::collections::BTreeMap;
use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use object::{Object, ObjectSymbol};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The float globals the scan finds in `fixture`, by symbol name and width.
fn float_globals(fixture: &str) -> BTreeMap<String, u8> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let input = prog.arch().kuna_float_scan.clone().expect("a loaded image stashes the scan input");
    let found = kuna_analysis::listing::kuna_floatglobals::scan(prog.arch(), &input);

    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let names: BTreeMap<u64, String> = file
        .symbols()
        .filter_map(|s| Some((s.address(), s.name().ok()?.to_string())))
        .filter(|(_, n)| n.starts_with('g'))
        .collect();
    found.iter().map(|(addr, width)| (names.get(addr).cloned().unwrap_or(format!("{addr:#x}")), *width)).collect()
}

fn expect(fixture: &str, want: &[(&str, u8)]) {
    let want: BTreeMap<String, u8> = want.iter().map(|(n, w)| (n.to_string(), *w)).collect();
    assert_eq!(float_globals(fixture), want, "{fixture}");
}

#[test]
fn x86_64_gcc_o2_floats_are_found_and_integer_moves_refuse() {
    expect("floatglobal_x86_64_gcc_O2", &[("gd", 8), ("gd2", 8), ("gf", 4), ("gf2", 4)]);
}

#[test]
fn x86_64_clang_o0_zeroes_its_double_through_xmm0() {
    expect("floatglobal_x86_64_clang_O0", &[("gd", 8), ("gd2", 8), ("gf", 4), ("gf2", 4), ("gz", 8)]);
}

#[test]
fn aarch64_adrp_addressed_floats_are_found() {
    expect("floatglobal_a64_O2", &[("gd", 8), ("gd2", 8), ("gf", 4), ("gf2", 4)]);
}

#[test]
fn arm_hard_float_literal_pool_addressed_floats_are_found() {
    expect("floatglobal_armhf_O2", &[("gd", 8), ("gd2", 8), ("gf", 4), ("gf2", 4)]);
}

/// `floatglobal_pun.c`: `fy` reads `gd`'s bits as an integer in a case of a
/// jump table bounded only by a mask, and through a pointer spilled among more
/// stack slots than the walk keeps. The scan reads the masked table and records
/// the spilled address as an escape, so `gd` is not a float.
#[test]
fn integer_reads_behind_a_masked_switch_or_a_spill_refuse() {
    expect("floatglobal_pun_sw_x86_64_gcc_O2", &[]);
    expect("floatglobal_pun_sw_x86_64_clang_O2", &[]);
    expect("floatglobal_pun_spill_x86_64_gcc_O0", &[]);
}
