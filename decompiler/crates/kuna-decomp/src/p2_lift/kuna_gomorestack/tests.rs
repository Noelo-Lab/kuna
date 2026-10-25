// Tests for `kuna_gomorestack` (included into `mod tests` in
// kuna_gomorestack.rs).
//
// Covered:
//   - `GoMorestackMode`: parse vocabulary, u8 round-trip, default.
//   - `is_morestack_name`: the runtime spellings, and the near-misses that
//     must NOT match.
//   - `live`: off/auto/always against the ArchContext source-language fact.
//   - `ActionGoMorestack::apply` and the `match_entry_guard_loop` shape
//     matcher are inert on a function with no calls (the end-to-end fold is
//     pinned by `tests/stages/kuna-gomorestack.xml` and the kuna-console
//     `verify_go_morestack` integration test over a real Go build).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::action::ActionContext;
use crate::context::ArchContext;
use crate::kuna_sourcelang::SourceLang;

fn build_fd(mode: GoMorestackMode, lang: SourceLang) -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let mut ctx = ArchContext::new(m);
    ctx.go_morestack = mode.as_u8();
    ctx.source_lang = lang;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn mode_parses_and_round_trips() {
    for (tok, mode) in [
        ("off", GoMorestackMode::Off),
        ("auto", GoMorestackMode::Auto),
        ("always", GoMorestackMode::Always),
        ("on", GoMorestackMode::Always),
    ] {
        let (m, msg) = parse_gomorestack_mode(tok).expect("parses");
        assert_eq!(m, mode, "{tok}");
        assert!(msg.contains(m.as_str()));
        assert_eq!(GoMorestackMode::from_u8(m.as_u8()), m);
    }
    assert!(parse_gomorestack_mode("sometimes").is_err());
    assert_eq!(GoMorestackMode::default(), GoMorestackMode::Auto);
    assert_eq!(GoMorestackMode::from_u8(99), GoMorestackMode::Off);
}

#[test]
fn morestack_name_recognition() {
    for name in [
        "runtime.morestack",
        "runtime.morestack_noctxt",
        "runtime.morestack_noctxt.abi0",
        "runtime.morestack.abi0",
        "runtime.morestackc",
    ] {
        assert!(is_morestack_name(name), "{name}");
    }
    for name in [
        "runtime.morestacked",   // not a runtime spelling
        "morestack",             // no package path
        "runtime.gopanic",
        "my.runtime.morestack",  // prefix test is anchored
        "",
    ] {
        assert!(!is_morestack_name(name), "{name}");
    }
}

#[test]
fn live_follows_mode_and_language() {
    for (mode, lang, want) in [
        (GoMorestackMode::Off, SourceLang::Go, false),
        (GoMorestackMode::Auto, SourceLang::Go, true),
        (GoMorestackMode::Auto, SourceLang::Unknown, false),
        (GoMorestackMode::Auto, SourceLang::Rust, false),
        (GoMorestackMode::Always, SourceLang::Unknown, true),
    ] {
        let fd = build_fd(mode, lang);
        assert_eq!(live(&fd), want, "{mode:?}/{lang:?}");
    }
}

#[test]
fn apply_and_matcher_are_inert_without_calls() {
    let mut fd = build_fd(GoMorestackMode::Always, SourceLang::Unknown);
    assert!(match_entry_guard_loop(&fd, &is_morestack_name).is_none());
    let mut action = ActionGoMorestack { base: ActionBase::new(0, "gomorestack", "deadcode") };
    let mut ctx = ActionContext::default();
    assert_eq!(action.apply(&mut fd, &mut ctx), 0);
}
