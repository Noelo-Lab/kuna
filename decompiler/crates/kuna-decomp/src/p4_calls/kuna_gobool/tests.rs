// Tests for `kuna_gobool` (included into `mod tests` in kuna_gobool.rs).
//
// Covered:
//   - `GoBoolMode`: parse vocabulary, u8 round-trip, default.
//   - `produces_boolean`: the genuine 0/1 producers, and the near-misses.
//   - `live`: off/auto/always against the ArchContext source-language fact.
//   - `RuleGoBoolReturn::apply_op` is inert on a function with no matching op
//     (the end-to-end rewrite is pinned by `tests/stages/kuna-gobool.xml` and
//     the corpus measurement).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::ArchContext;
use crate::kuna_sourcelang::SourceLang;

fn build_fd(mode: GoBoolMode, lang: SourceLang) -> Funcdata {
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
    ctx.go_bool = mode.as_u8();
    ctx.source_lang = lang;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn mode_parses_and_round_trips() {
    for (tok, mode) in [
        ("off", GoBoolMode::Off),
        ("auto", GoBoolMode::Auto),
        ("always", GoBoolMode::Always),
        ("on", GoBoolMode::Always),
    ] {
        let (m, msg) = parse_gobool_mode(tok).expect("parses");
        assert_eq!(m, mode, "{tok}");
        assert!(msg.contains(m.as_str()));
        assert_eq!(GoBoolMode::from_u8(m.as_u8()), m);
    }
    assert!(parse_gobool_mode("sometimes").is_err());
    assert_eq!(GoBoolMode::default(), GoBoolMode::Auto);
    assert_eq!(GoBoolMode::from_u8(77), GoBoolMode::Off);
}

#[test]
fn boolean_producers_are_the_predicate_opcodes() {
    for c in [
        OpCode::CPUI_INT_EQUAL,
        OpCode::CPUI_INT_NOTEQUAL,
        OpCode::CPUI_INT_SLESS,
        OpCode::CPUI_BOOL_NEGATE,
        OpCode::CPUI_FLOAT_NAN,
    ] {
        assert!(produces_boolean(c), "{c:?}");
    }
    for c in [
        OpCode::CPUI_COPY,
        OpCode::CPUI_INT_ADD,
        OpCode::CPUI_PIECE,
        OpCode::CPUI_SUBPIECE,
        OpCode::CPUI_INT_ZEXT,
        OpCode::CPUI_LOAD,
    ] {
        assert!(!produces_boolean(c), "{c:?}");
    }
}

#[test]
fn live_follows_mode_and_language() {
    for (mode, lang, want) in [
        (GoBoolMode::Off, SourceLang::Go, false),
        (GoBoolMode::Auto, SourceLang::Go, true),
        (GoBoolMode::Auto, SourceLang::Unknown, false),
        (GoBoolMode::Auto, SourceLang::Rust, false),
        (GoBoolMode::Always, SourceLang::Unknown, true),
    ] {
        let fd = build_fd(mode, lang);
        assert_eq!(live(&fd), want, "{mode:?}/{lang:?}");
    }
}

#[test]
fn apply_is_inert_without_a_matching_return() {
    let mut fd = build_fd(GoBoolMode::Always, SourceLang::Unknown);
    let mut rule = RuleGoBoolReturn::with_group(true, "analysis");
    // No ops exist; the per-op driver would never call us, and a direct call
    // on a dangling id must refuse cleanly.
    assert_eq!(rule.apply_op(crate::context::OpId::default(), &mut fd), 0);
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_RETURN]);
}
