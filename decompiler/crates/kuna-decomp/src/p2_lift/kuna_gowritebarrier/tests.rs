// Tests for `kuna_gowritebarrier` (included into `mod tests` in
// kuna_gowritebarrier.rs).
//
// Covered:
//   - `GoWriteBarrierMode`: parse vocabulary, u8 round-trip, default.
//   - `is_write_barrier_name`: the thunk spellings (numbered, bare assembly
//     symbol, `.abi0`), and the near-misses that must NOT match (`wbBufFlush`
//     stays out on purpose).
//   - `live`: off/auto/always against the ArchContext source-language fact.
//   - `ActionGoWriteBarrier::apply` and `match_guarded_cold_call` are inert on
//     a function with no calls (the end-to-end fold is pinned by
//     `tests/stages/kuna-gowritebarrier.xml` and the kuna-console
//     `verify_go_writebarrier` gate over a real Go build).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::action::ActionContext;
use crate::context::ArchContext;
use crate::kuna_sourcelang::SourceLang;

fn build_fd(mode: GoWriteBarrierMode, lang: SourceLang) -> Funcdata {
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
    ctx.go_write_barrier = mode.as_u8();
    ctx.source_lang = lang;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn mode_parses_and_round_trips() {
    for (tok, mode) in [
        ("off", GoWriteBarrierMode::Off),
        ("auto", GoWriteBarrierMode::Auto),
        ("always", GoWriteBarrierMode::Always),
        ("on", GoWriteBarrierMode::Always),
    ] {
        let (m, msg) = parse_gowritebarrier_mode(tok).expect("parses");
        assert_eq!(m, mode, "{tok}");
        assert!(msg.contains(m.as_str()));
        assert_eq!(GoWriteBarrierMode::from_u8(m.as_u8()), m);
    }
    assert!(parse_gowritebarrier_mode("sometimes").is_err());
    assert_eq!(GoWriteBarrierMode::default(), GoWriteBarrierMode::Auto);
    assert_eq!(GoWriteBarrierMode::from_u8(99), GoWriteBarrierMode::Off);
}

#[test]
fn write_barrier_name_recognition() {
    for name in [
        "runtime.gcWriteBarrier",
        "runtime.gcWriteBarrier1",
        "runtime.gcWriteBarrier4",
        "runtime.gcWriteBarrier8",
        "runtime.gcWriteBarrier.abi0",
        "runtime.gcWriteBarrier2.abi0",
        "gcWriteBarrier", // the bare assembly thunk (no package path in the pclntab)
    ] {
        assert!(is_write_barrier_name(name), "{name}");
    }
    for name in [
        "runtime.gcWriteBarrier12",    // no such thunk; two digits is not the family
        "runtime.gcWriteBarriers",     // not a runtime spelling
        "runtime.wbBufFlush",          // the thunks' own slow path - never inline
        "runtime.wbBufFlush1",
        "my.gcWriteBarrier",           // anchored prefixes only
        "writeBarrier",
        "",
    ] {
        assert!(!is_write_barrier_name(name), "{name}");
    }
}

#[test]
fn live_follows_mode_and_language() {
    for (mode, lang, want) in [
        (GoWriteBarrierMode::Off, SourceLang::Go, false),
        (GoWriteBarrierMode::Auto, SourceLang::Go, true),
        (GoWriteBarrierMode::Auto, SourceLang::Unknown, false),
        (GoWriteBarrierMode::Auto, SourceLang::Rust, false),
        (GoWriteBarrierMode::Always, SourceLang::Unknown, true),
    ] {
        let fd = build_fd(mode, lang);
        assert_eq!(live(&fd), want, "{mode:?}/{lang:?}");
    }
}

#[test]
fn apply_and_matcher_are_inert_without_calls() {
    let mut fd = build_fd(GoWriteBarrierMode::Always, SourceLang::Unknown);
    assert!(match_guarded_cold_call(&fd, &is_write_barrier_name).is_none());
    let mut action = ActionGoWriteBarrier { base: ActionBase::new(0, "gowritebarrier", "deadcode") };
    let mut ctx = ActionContext::default();
    assert_eq!(action.apply(&mut fd, &mut ctx), 0);
}
