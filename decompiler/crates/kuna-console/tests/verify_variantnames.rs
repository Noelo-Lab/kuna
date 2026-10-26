//! End-to-end gate for discriminant-value naming (`--option variantnames`,
//! `docs/multilang.md` — the first `VariantLayout` consumer).
//!
//! `dwarfvariants` recovers the geometry; this gate proves the VOCABULARY: with
//! the option on (the default) a tag store/test renders the variant name the
//! compiler stated, with it off the bare integer returns, and the layout —
//! fields, offsets, payload stores — is byte-identical either way.
//!
//! Own `[[test]]` target: the gate is a process env var flipped around
//! `load file` (the `verify_dwarfvariants` arrangement), serialized by a mutex
//! against this binary's own tests. The committed `dwarfvariants_x86_64`
//! fixture (source vendored beside it) is reused — no new fixture.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV;
use kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV;
use kuna_decomp::kuna_variantnames::VARIANTNAMES_ENV;

/// Serializes the env-var flip + bootstrap across the concurrently-run tests.
static GATE: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the committed fixture with `variantnames` on/off (its parents
/// `dwarfstructs`/`dwarfvariants` pinned ON either way, so what is measured is
/// the naming arm alone), decompile each of `funcs`, return the combined C.
fn decompile(funcs: &[&str], names: bool) -> String {
    let _guard = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let path = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/dwarfvariants_x86_64")
        .to_str()
        .expect("UTF-8 fixture path")
        .to_string();
    std::env::set_var(DWARFSTRUCTS_ENV, "on");
    std::env::set_var(DWARFVARIANTS_ENV, "on");
    std::env::set_var(VARIANTNAMES_ENV, if names { "on" } else { "off" });
    let mut prog = bootstrap_from_object(&path, "", &spec_roots).expect("bootstrap fixture");
    std::env::remove_var(VARIANTNAMES_ENV);
    std::env::remove_var(DWARFVARIANTS_ENV);
    std::env::remove_var(DWARFSTRUCTS_ENV);
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

/// The happy path: a three-variant tagged enum's stores name A/B/C under the
/// default and the bare 0/1/2 with the option off; the payload stores do not
/// move, so only the tag's vocabulary changed.
#[test]
fn tag_constants_take_variant_names_and_the_layout_stands_still() {
    let on = decompile(&["ret_three"], true);
    let off = decompile(&["ret_three"], false);

    assert!(on.contains("rethidden->tag = A;"), "on names the store:\n{on}");
    assert!(on.contains("Three::Discriminant"), "on declares the synthesized enum:\n{on}");
    assert!(!on.contains("rethidden->tag = 0;"), "on leaves no bare store:\n{on}");

    assert!(off.contains("rethidden->tag = 0;"), "off restores the bare store:\n{off}");
    assert!(!off.contains("rethidden->tag = A;"), "off names nothing:\n{off}");
    assert!(!off.contains("Three::Discriminant"), "off synthesizes no enum:\n{off}");

    for both in ["(rethidden->payload).A.__0 = x;"] {
        assert!(on.contains(both) && off.contains(both), "layout unchanged: {both}");
    }
}

/// A tag READ names the variant too (the recursive List walker), and a niche
/// enum — which has no tag field to retype — renders identically either way.
#[test]
fn tag_reads_are_named_and_niche_layouts_are_untouched() {
    let on = decompile(&["list_len", "ret_niche"], true);
    let off = decompile(&["list_len", "ret_niche"], false);

    assert!(on.contains("Cons") && on.contains("Nil"), "on names the walker's test:\n{on}");
    assert!(!off.contains("== Cons"), "off keeps integers:\n{off}");

    let niche_on = &on[on.find("ret_niche").expect("niche on")..];
    let niche_off = &off[off.find("ret_niche").expect("niche off")..];
    assert_eq!(niche_on, niche_off, "a niche layout has no tag field to retype");
}
