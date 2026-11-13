//! (kuna `flirt`) FLIRT matching on a stripped binary.
//!
//! Fixture: `tests/fixtures/fid/callees/prog` and the signature
//! `tests/fixtures/flirt/callees.sig` (written by `make_callees_sig.py` beside
//! it). `scale_a` and `scale_b` are modules that fit inside the 32-byte trie
//! prefix; `wrap_a` and `wrap_b` share one pattern and differ only in the name
//! of the call site at offset 7.
//!
//! - `flirt off`: all four functions stay `sub_`.
//! - `flirt on`: the first round names `scale_a` / `scale_b`, and the second
//!   gives each wrapper the module whose call site agrees.
//! - `fid on` as well: with `lib.fid` FID also names `scale_b` and the name
//!   stands; with `lib_alt.fid` FID says `alt_scale`, the two disagree and
//!   `0x401020` keeps its placeholder.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

const ENTRIES: [u64; 4] = [0x401000, 0x401020, 0x401040, 0x401060];

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../tests/fixtures").canonicalize().unwrap()
}

fn name_at(prog: &ConsoleProgram, vma: u64) -> String {
    let space = prog.arch().manage().get_default_code_space().unwrap().clone();
    let addr = Address::new(Rc::clone(&space), vma);
    match prog.arch().symboltab.find_function_across_scopes(&addr) {
        Some((sid, _)) => prog.arch().symboltab.symbol(sid).get_name().to_string(),
        None => prog.arch().name_function(&addr),
    }
}

fn names(flirt: &str, fid_db: Option<&str>) -> Vec<String> {
    let dir = fixtures();
    let specs = dir.join("../../specs").canonicalize().unwrap();
    std::env::set_var("KUNA_FLIRT_SIGS", dir.join("flirt/callees.sig"));
    if let Some(db) = fid_db {
        std::env::set_var("kuna_fid_db", dir.join("fid/callees").join(db));
    }
    let bin = dir.join("fid/callees/prog");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &[specs.to_str().unwrap().to_string()])
        .expect("bootstrap fixture with built processor specs");
    let fid = if fid_db.is_some() { "on" } else { "off" };
    for (k, v) in [("listing", "on"), ("flirt", flirt), ("fid", fid)] {
        prog.arch_mut().set_kuna_option(k, v).expect("option flips");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    ENTRIES.iter().map(|&a| name_at(&prog, a)).collect()
}

/// One test: every case sets process environment variables.
#[test]
fn sites_name_prefix_modules_and_fid_disagreement_names_nothing() {
    let off = names("off", None);
    assert!(off.iter().all(|n| n.starts_with("sub_")), "off: nothing is named, got {off:?}");
    assert_eq!(names("on", None), ["scale_a", "scale_b", "wrap_a", "wrap_b"]);
    assert_eq!(names("on", Some("lib.fid")), ["scale_a", "scale_b", "wrap_a", "wrap_b"]);
    assert_eq!(names("on", Some("lib_alt.fid")), ["scale_a", "sub_401020", "wrap_a", "wrap_b"]);
}
