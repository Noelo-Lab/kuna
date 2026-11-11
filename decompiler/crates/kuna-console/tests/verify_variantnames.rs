//! End-to-end gate for discriminant-value naming (part of `--option dwarf`,
//! `docs/multilang.md` — the first `VariantLayout` consumer).
//!
//! The variant-part import recovers the geometry; this gate proves the
//! VOCABULARY: a tag store/test renders the variant name the compiler stated,
//! and the layout — fields, offsets, payload stores — is what the import gives.
//! The committed `dwarfvariants_x86_64` fixture (source vendored beside it) is
//! reused — no new fixture.

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the committed fixture, decompile each of `funcs`, return the combined C.
fn decompile(funcs: &[&str]) -> String {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()
        .expect("UTF-8 fixture path")
        .to_string();
    let mut prog = bootstrap_from_object(&path, "", &spec_roots).expect("bootstrap fixture");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let mut cmds: Vec<String> = Vec::new();
    for f in funcs {
        cmds.push(format!("load function {f}"));
        cmds.push("decompile".to_string());
        cmds.push("print C".to_string());
    }
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The happy path: a three-variant tagged enum's stores name A/B/C, and the
/// payload stores keep their named field paths.
#[test]
fn tag_constants_take_variant_names_and_the_layout_stands_still() {
    let on = decompile(&["ret_three"]);

    assert!(on.contains("rethidden->tag = A;"), "the store is named:\n{on}");
    assert!(on.contains("Three::Discriminant"), "the synthesized enum is declared:\n{on}");
    assert!(!on.contains("rethidden->tag = 0;"), "no bare store is left:\n{on}");
    assert!(on.contains("(rethidden->payload).A.__0 = x;"), "the payload store is named:\n{on}");
}

/// A tag READ names the variant too (the recursive List walker), and a niche
/// enum has no tag field to retype.
#[test]
fn tag_reads_are_named_and_niche_layouts_are_untouched() {
    let on = decompile(&["list_len", "ret_niche"]);

    assert!(on.contains("Cons") && on.contains("Nil"), "the walker's test is named:\n{on}");

    let niche = &on[on.find("ret_niche").expect("niche")..];
    assert!(!niche.contains("Discriminant"), "a niche layout has no tag field to retype:\n{niche}");
}
