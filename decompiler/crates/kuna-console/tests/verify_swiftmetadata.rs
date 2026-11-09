//! swiftmetadata end-to-end gate: name the functions of a fully stripped Swift
//! binary from its `swift5_*` reflection sections.
//!
//! Fixture: `tests/fixtures/swift_metadata_elf` — an x86-64 ELF executable
//! built from `swift_metadata_elf.swift` with swiftc 6.3 (`-O`) and then
//! `llvm-strip --strip-all`, so it carries NO symbol for any of its own
//! functions; only the metadata can recover a name. The pinned VMAs below come
//! from the unstripped build's symbol table (see the fixtures README), which is
//! the oracle: each name the pass installs stands at the address the real
//! symbol had.
//!
//! The two-state proof (the `verify_objc` posture): with `swiftmetadata` off
//! (default) every pinned address carries an engine placeholder; on, it carries
//! the metadata-derived name.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, bootstrap_from_object};

const FIXTURE: &str = "swift_metadata_elf";

/// `(VMA, name the pass installs, the unstripped symbol it stands for)`.
const PINNED: &[(u64, &str, &str)] = &[
    (
        0x30b0,
        "metafix.Point.type_metadata_accessor",
        "$s7metafix5PointVMa",
    ),
    (
        0x30d0,
        "metafix.Animal.type_metadata_accessor",
        "$s7metafix6AnimalCMa",
    ),
    (
        0x2fe0,
        "metafix.Animal.vtable#4.method",
        "$s7metafix6AnimalC5soundSSyF",
    ),
    (
        0x2f60,
        "metafix.Animal.vtable#0.getter",
        "$s7metafix6AnimalC4legsSivg",
    ),
    (
        0x3100,
        "metafix.Animal.vtable#3.init",
        "$s7metafix6AnimalCACycfC",
    ),
    (
        0x3000,
        "metafix.Dog.vtable#4.method",
        "$s7metafix3DogC5soundSSyF",
    ),
    (
        0x2ab0,
        "<metafix.Square:Swift.Hashable>.hash(into:)",
        "$s7metafix6SquareVSHAASH4hash4intoys6HasherVz_tFTW",
    ),
    (
        0x2b00,
        "<metafix.Square:Swift.Equatable>.==(_:_:)",
        "$s7metafix6SquareVSQAASQ2eeoiySbx_xtFZTW",
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

fn run(on: bool) -> Vec<String> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(FIXTURE);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    if on {
        prog.arch_mut()
            .set_kuna_option("swiftmetadata", "on")
            .expect("swiftmetadata flips on");
    }
    prog.commit_pending_analysis()
        .expect("analysis commit succeeds");
    PINNED
        .iter()
        .map(|&(vma, _, _)| name_at(&prog, vma))
        .collect()
}

/// The pass's names agree with the unstripped symbol table at every pinned
/// address: the demangled symbol name has the same path for an accessor, and
/// a vtable slot / witness name stands where the method / witness symbol did.
#[test]
fn stripped_swift_functions_are_named_from_metadata() {
    let off = run(false);
    let on = run(true);
    for (i, &(vma, want, symbol)) in PINNED.iter().enumerate() {
        assert!(
            off[i].starts_with("sub_") || off[i].starts_with("FUN_"),
            "default leaves 0x{vma:x} a placeholder, got `{}`",
            off[i]
        );
        assert_eq!(on[i], want, "swiftmetadata on names 0x{vma:x} ({symbol})");
        // The demangled symbol's path is a prefix of (or equal to) the
        // installed name's type path, so the two agree on which type owns it.
        let demangled = kuna_analysis::demangle::kuna_swift::demangle_swift_name(symbol)
            .expect("symbol demangles");
        let type_path = |n: &str| {
            let n = n.trim_start_matches('<');
            n.split([':', '.']).take(2).collect::<Vec<_>>().join(".")
        };
        assert_eq!(
            type_path(&demangled),
            type_path(want),
            "same owning type for 0x{vma:x}"
        );
    }
}
