//! swiftobjcthunk end-to-end gate: a stripped Swift class's `@objc` members
//! are named from their Objective-C entry points instead of their vtable
//! position.
//!
//! Fixture: `tests/fixtures/macho_swift_objcthunk_arm64`, an arm64 iOS dylib
//! linked with `-x` from `macho_swift_objcthunk.swift` (`-Onone`) and
//! `macho_swift_objcthunk_probe.swift` (`-O`), so none of its Swift functions
//! keeps a symbol. The VMAs below come from the same link without `-x` (see
//! the fixtures README). `Probe.refresh()` is not `@objc`; the `-O` thunk of
//! `Probe`'s `doesNotRecognizeSelector:` override sends `super` and then calls
//! it directly, so it must keep its positional name.
//!
//! Fixture: `tests/fixtures/macho_swift_objcthunk_inlined_arm64` (`-O -wmo`),
//! whose `@objc` thunks hold their member's inlined body: the slot such a
//! thunk calls is another method, which keeps its positional name, and a
//! Swift symbol keeps the name it gives.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, bootstrap_from_object};

/// `(VMA, name with the gate on, name with it off, the unlinked symbol)`.
const PINNED: &[(u64, &str, &str, &str)] = &[
    (
        0x4060,
        "objcfix.Replay.scrollView.getter",
        "objcfix.Replay.vtable#0.getter",
        "$s7objcfix6ReplayC10scrollViewSo8NSObjectCSgvg",
    ),
    (
        0x4130,
        "objcfix.Replay.scrollView.setter",
        "objcfix.Replay.vtable#1.setter",
        "$s7objcfix6ReplayC10scrollViewSo8NSObjectCSgvs",
    ),
    (
        0x428c,
        "objcfix.Replay.count.getter",
        "objcfix.Replay.vtable#3.getter",
        "$s7objcfix6ReplayC5countSivg",
    ),
    (
        0x4340,
        "objcfix.Replay.count.setter",
        "objcfix.Replay.vtable#4.setter",
        "$s7objcfix6ReplayC5countSivs",
    ),
    (
        0x4410,
        "objcfix.Replay.addFrameWithTimestamp:label:",
        "objcfix.Replay.vtable#6.method",
        "$s7objcfix6ReplayC8addFrame9timestamp5labelSiSd_So8NSObjectCSgtF",
    ),
    (
        0x4708,
        "objcfix.Replay.make",
        "objcfix.Replay.vtable#7.method",
        "$s7objcfix6ReplayC4makeACyFZ",
    ),
    (
        0x49bc,
        "objcfix.Sub.addFrameWithTimestamp:label:",
        "objcfix.Sub.vtable#6.method",
        "$s7objcfix3SubC8addFrame9timestamp5labelSiSd_So8NSObjectCSgtF",
    ),
    (
        0x4c10,
        "objcfix.Probe.vtable#3.method",
        "objcfix.Probe.vtable#3.method",
        "$s7objcfix5Probe33_7E6203954F19F560930F971E824A5ED5LLC7refreshyyF",
    ),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let code_space = prog
        .arch()
        .manage()
        .get_default_code_space()
        .expect("a default code space")
        .clone();
    let addr = Address::new(Rc::clone(&code_space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

/// `(VMA, reported name, the unlinked symbol)` in the `-O -wmo` fixture.
const INLINED: &[(u64, &str, &str)] = &[
    (0x4000, "objt.Gone.vtable#3.method", "$s4objt4GoneC6handleyyF"),
    (0x4054, "objt.Kept.vtable#3.method", "$s4objt4KeptC6handleyyF"),
    (0x4118, "objt.Pure.run", "$s4objt4PureC3runyyF"),
    (0x4280, "objt.Sym.act(_:)", "$s4objt3SymC3actyySo8NSObjectCF"),
];

fn load(fixture: &str, on: bool) -> ConsoleProgram {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut()
        .set_kuna_option("swiftmetadata", "on")
        .expect("swiftmetadata flips");
    prog.arch_mut()
        .set_kuna_option("swiftobjcthunk", if on { "on" } else { "off" })
        .expect("swiftobjcthunk flips");
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    prog
}

fn run(on: bool) -> Vec<String> {
    let prog = load("macho_swift_objcthunk_arm64", on);
    PINNED
        .iter()
        .map(|&(vma, ..)| name_at(&prog, vma))
        .collect()
}

#[test]
fn objc_thunks_name_swift_vtable_slots() {
    let on = run(true);
    let off = run(false);
    for (i, &(vma, want_on, want_off, symbol)) in PINNED.iter().enumerate() {
        assert_eq!(
            on[i], want_on,
            "swiftobjcthunk on names 0x{vma:x} ({symbol})"
        );
        assert_eq!(
            off[i], want_off,
            "swiftobjcthunk off keeps 0x{vma:x} positional ({symbol})"
        );
    }
}

#[test]
fn an_inlined_thunk_body_names_no_other_slot() {
    let prog = load("macho_swift_objcthunk_inlined_arm64", true);
    let entries = prog.function_entries_canonical();
    for &(vma, want, symbol) in INLINED {
        let got = entries
            .iter()
            .find(|e| e.addr.get_offset() == vma)
            .map(|e| e.name.as_str());
        assert_eq!(got, Some(want), "0x{vma:x} ({symbol})");
    }
}
