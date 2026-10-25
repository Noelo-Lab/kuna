// Tests for `kuna_fatstr` (included into `mod tests` in kuna_fatstr.rs).
//
// Covered:
//   - `FatStrMode`: parse vocabulary, u8 round-trip, default.
//   - `bytes_are_text`: text, UTF-8, control-byte and no-alnum refusals.
//   - `live`: off/auto/always against the ArchContext source-language fact.
//   - the W3 facet table: register + lookup through the `TypeFactory` trait
//     (the end-to-end rewrite is pinned by `tests/stages/kuna-fatstr.xml`).

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::context::ArchContext;
use crate::dtype::TypeFactoryImpl;
use crate::kuna_langfacet::LangFacet;
use crate::kuna_sourcelang::SourceLang;

fn build_fd(mode: FatStrMode, lang: SourceLang) -> Funcdata {
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
    ctx.fat_str = mode.as_u8();
    ctx.source_lang = lang;
    let glb = Rc::new(ctx);
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn mode_parses_and_round_trips() {
    for (tok, mode) in [
        ("off", FatStrMode::Off),
        ("auto", FatStrMode::Auto),
        ("always", FatStrMode::Always),
        ("on", FatStrMode::Always),
    ] {
        let (m, msg) = parse_fatstr_mode(tok).expect("parses");
        assert_eq!(m, mode, "{tok}");
        assert!(msg.contains(m.as_str()));
        assert_eq!(FatStrMode::from_u8(m.as_u8()), m);
    }
    assert!(parse_fatstr_mode("sometimes").is_err());
    assert_eq!(FatStrMode::default(), FatStrMode::Auto);
    assert_eq!(FatStrMode::from_u8(99), FatStrMode::Off);
}

#[test]
fn text_evidence() {
    assert!(bytes_are_text(b"hello multilang"));
    assert!(bytes_are_text(b"two\nlines\tand spaces"));
    assert!(bytes_are_text("héllo wörld".as_bytes()));
    // Interior NUL / control bytes are not text.
    assert!(!bytes_are_text(b"hel\x00lo"));
    assert!(!bytes_are_text(b"\x01\x02\x03\x04"));
    // Invalid UTF-8 is not text.
    assert!(!bytes_are_text(&[0xff, 0xfe, b'a', b'b']));
    // Punctuation-only padding never reads as a string.
    assert!(!bytes_are_text(b"----    ----"));
    assert!(!bytes_are_text(b""));
}

#[test]
fn live_follows_mode_and_language() {
    for (mode, lang, want) in [
        (FatStrMode::Off, SourceLang::Rust, false),
        (FatStrMode::Auto, SourceLang::Rust, true),
        (FatStrMode::Auto, SourceLang::Go, true),
        (FatStrMode::Auto, SourceLang::C, false),
        (FatStrMode::Auto, SourceLang::Unknown, false),
        (FatStrMode::Always, SourceLang::Unknown, true),
    ] {
        let fd = build_fd(mode, lang);
        assert_eq!(live(&fd), want, "{mode:?}/{lang:?}");
    }
}

#[test]
fn specs_register_the_analysis_clone() {
    let s = specs();
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].group, "analysis");
    let rule = (s[0].ctor)();
    assert_eq!(rule.get_op_list(), vec![OpCode::CPUI_CALL]);
}

#[test]
fn facet_table_registers_and_looks_up() {
    use crate::dtype::TypeFactory;
    let tf = TypeFactoryImpl::default();
    assert!(tf.kuna_lang_facet("alloc::string::String").is_none());
    tf.kuna_register_lang_facet("&str", LangFacet::RustStr).expect("registers");
    tf.kuna_register_lang_facet("&[u8]", LangFacet::RustSlice { elem: "u8".into() })
        .expect("registers");
    assert_eq!(tf.kuna_lang_facet("&str"), Some(LangFacet::RustStr));
    assert!(tf.kuna_lang_facet("&str").unwrap().is_text());
    assert!(!tf.kuna_lang_facet("&[u8]").unwrap().is_text());
    // Re-registration replaces (a provider refining its record).
    tf.kuna_register_lang_facet("&[u8]", LangFacet::RustSlice { elem: "u16".into() })
        .expect("re-registers");
    assert_eq!(tf.kuna_lang_facet("&[u8]"), Some(LangFacet::RustSlice { elem: "u16".into() }));
}
