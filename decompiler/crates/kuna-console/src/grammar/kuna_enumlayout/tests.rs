//! Tests for C's enum width and constants ([`super`]).

use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype as meta, Datatype, TypeFactory, TypeFactoryImpl};

use super::super::{parse_c, DataOrg};

/// A factory with `int`/`long` widths as a cspec's `<data_organization>` sets
/// them and `setup_sizes` fills the rest (the enum default is the address size).
fn factory(int: i32, long: i32, addr: i32) -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("int4", 4, meta::TYPE_INT, false).unwrap();
    f.set_core_type("float4", 4, meta::TYPE_FLOAT, false).unwrap();
    f.set_core_type("char", 1, meta::TYPE_INT, true).unwrap();
    f.set_core_type("uint1", 1, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("int2", 2, meta::TYPE_INT, false).unwrap();
    f.cache_core_types().unwrap();
    f.set_size_of_int(int);
    f.set_size_of_long(long);
    f.setup_sizes(Some(addr), addr, addr);
    f
}

fn declare(f: &TypeFactoryImpl, decl: &str) -> Rc<Datatype> {
    let org = DataOrg { addr_size: 8, word_size: 1 };
    parse_c(decl, f, org, &[], |_, _| Ok(())).unwrap_or_else(|e| panic!("{decl}: {}", e.explain()));
    let name = decl.split_whitespace().nth(1).unwrap();
    f.find_by_name(name).unwrap().unwrap()
}

/// `enum Mode { FIRST = 1, SECOND = 2 }` is `int`-wide on LP64 and ILP32
/// alike, not as wide as a pointer, so a following member lands at offset 4.
#[test]
fn an_enum_is_as_wide_as_int() {
    for (int, long, addr) in [(4, 8, 8), (4, 4, 4), (4, 4, 8), (2, 4, 2)] {
        let f = factory(int, long, addr);
        let mode = declare(&f, "enum Mode { FIRST = 1, SECOND = 2 };");
        assert_eq!(mode.get_size(), int, "int {int} addr {addr}");
        assert!(mode.is_enum_type());
        let packet = declare(&f, "struct Packet { Mode mode; float4 gain; int4 value; };");
        assert_eq!(packet.get_field(1).unwrap().offset, int.max(4), "int {int} addr {addr}");
    }
}

/// A constant too wide for `int` widens the enum to `long`, then `long long`.
#[test]
fn a_constant_too_wide_for_int_widens_the_enum() {
    let f = factory(4, 8, 8);
    assert_eq!(declare(&f, "enum Big { LOW = 1, HIGH = 0x100000000 };").get_size(), 8);
    assert_eq!(declare(&f, "enum Top { TOP = 0xffffffff };").get_size(), 4);
    let f = factory(4, 4, 4);
    assert_eq!(declare(&f, "enum Big { HIGH = 0x100000000 };").get_size(), 8);
    let f = factory(2, 4, 2);
    assert_eq!(declare(&f, "enum Mid { MID = 0x10000 };").get_size(), 4);
}

/// An enumerator with no `=` is one past the one before it, the first zero.
#[test]
fn an_unvalued_enumerator_follows_the_one_before_it() {
    let f = factory(4, 8, 8);
    let color = declare(&f, "enum Color { RED, GREEN, BLUE };");
    for (v, name) in [(0, "RED"), (1, "GREEN"), (2, "BLUE")] {
        assert!(color.has_named_value(v), "{name} = {v}");
    }
    assert!(!color.has_named_value(3));
    let gap = declare(&f, "enum Gap { A = 5, B, C = 1, D };");
    for v in [5, 6, 1, 2] {
        assert!(gap.has_named_value(v), "{v}");
    }
    let org = DataOrg { addr_size: 8, word_size: 1 };
    let err = parse_c("enum Clash { X = 1, Y = 0, Z };", &f, org, &[], |_, _| Ok(())).unwrap_err();
    assert!(err.explain().contains("duplicate value"), "{}", err.explain());
}

/// A C23 underlying type sets the width and signedness; a non-integer one, or
/// one without a body, is refused.
#[test]
fn a_c23_underlying_type_sets_the_width() {
    let f = factory(4, 8, 8);
    let small = declare(&f, "enum Small : uint1 { S0, S1 };");
    assert_eq!(small.get_size(), 1);
    assert_eq!(small.get_metatype(), meta::TYPE_UINT);
    let signed = declare(&f, "enum Signed : int2 { N0 };");
    assert_eq!(signed.get_size(), 2);
    assert_eq!(signed.get_metatype(), meta::TYPE_INT);
    let org = DataOrg { addr_size: 8, word_size: 1 };
    for bad in [
        "enum F : float4 { F0 };",
        "enum G : uint1;",
        "enum H : uint1 { H0 = 0x100 };",
        "enum I : int2 { I0 = 0x8000 };",
    ] {
        assert!(parse_c(bad, &f, org, &[], |_, _| Ok(())).is_err(), "{bad}");
    }
}
