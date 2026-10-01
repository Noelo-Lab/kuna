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
    f.set_core_type("uint2", 2, meta::TYPE_UINT, false).unwrap();
    f.set_core_type("int8", 8, meta::TYPE_INT, false).unwrap();
    f.set_core_type("uint8", 8, meta::TYPE_UINT, false).unwrap();
    f.cache_core_types().unwrap();
    f.set_size_of_int(int);
    f.set_size_of_long(long);
    f.setup_sizes(Some(addr), addr, addr);
    f
}

fn declare(f: &TypeFactoryImpl, decl: &str) -> Rc<Datatype> {
    parse(f, decl).unwrap_or_else(|e| panic!("{decl}: {e}"));
    let name = decl.split_whitespace().nth(1).unwrap();
    f.find_by_name(name).unwrap().unwrap()
}

fn parse(f: &TypeFactoryImpl, decl: &str) -> Result<(), String> {
    let org = DataOrg { addr_size: 8, word_size: 1 };
    parse_c(decl, f, org, &[], |_, _| Ok(())).map_err(|e| e.explain().to_string())
}

/// The names `ct` prints for `val`.
fn names(ct: &Datatype, val: u64) -> Vec<String> {
    ct.get_matches(val).unwrap().match_name
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
    let neg = declare(&f, "enum Neg { A = -2, B, C };");
    for (v, name) in [(0xffff_fffe, "A"), (0xffff_ffff, "B"), (0, "C")] {
        assert_eq!(names(&neg, v), [name], "{name}");
    }
}

/// A negative constant makes the enum `int`, as gcc and clang make it: as wide
/// as `int` while every constant fits, and signed.
#[test]
fn a_negative_constant_makes_the_enum_signed() {
    for (int, long, addr) in [(4, 8, 8), (4, 4, 4)] {
        let f = factory(int, long, addr);
        for decl in [
            "enum Sign { NEG = -1, ZERO, POS };",
            "enum Neg2 { NA = -5, NB, NC, ND, NE, NF };",
            "enum Lo { LMIN = -2147483648, LMAX = 2147483647 };",
        ] {
            let ct = declare(&f, decl);
            assert_eq!((ct.get_size(), ct.get_metatype()), (4, meta::TYPE_INT), "{decl} on int {int} addr {addr}");
        }
        let sign = declare(&f, "enum S2 { M1 = -1, Z0 };");
        assert_eq!(names(&sign, 0xffff_ffff), ["M1"]);
        let rec = declare(&f, "struct Rec { S2 s; char c; int4 v; };");
        assert_eq!(rec.get_field(2).unwrap().offset, 8);
        for decl in ["enum Below { B = -2147483649 };", "enum Mixed { MN = -1, MX = 0x80000000 };"] {
            let ct = declare(&f, decl);
            assert_eq!((ct.get_size(), ct.get_metatype()), (8, meta::TYPE_INT), "{decl}");
        }
    }
    let f = factory(4, 8, 8);
    let hi = declare(&f, "enum Hi { HI = 0x80000000 };");
    assert_eq!((hi.get_size(), hi.get_metatype()), (4, meta::TYPE_UINT), "unsigned int holds it");
}

/// C lets two enumerators share a constant; the first keeps the value and the
/// alias is dropped, so the declaration is not refused.
#[test]
fn an_alias_enumerator_is_accepted_and_the_first_name_kept() {
    let f = factory(4, 8, 8);
    let op = declare(&f, "enum Op { OP_NOP, OP_ADD, OP_SUB, OP_LAST = 2 };");
    assert_eq!(names(&op, 2), ["OP_SUB"]);
    let gap = declare(&f, "enum Clash { X = 1, Y = 0, Z };");
    assert_eq!(names(&gap, 1), ["X"]);
    assert_eq!(names(&gap, 0), ["Y"]);
    let packet = declare(&f, "struct Opcode { Op op; int4 arg; };");
    assert_eq!(packet.get_field(1).unwrap().offset, 4);
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
    for bad in ["enum F : float4 { F0 };", "enum G : uint1;"] {
        assert!(parse_c(bad, &f, org, &[], |_, _| Ok(())).is_err(), "{bad}");
    }
}

/// A constant fits a C23 underlying type exactly when that type can hold it:
/// both ends of every width are accepted, one past either end is refused.
#[test]
fn a_c23_underlying_type_holds_its_own_range() {
    let f = factory(4, 8, 8);
    let cases: [(&str, &[&str], &[&str]); 9] = [
        ("signed char", &["-128", "127", "-1"], &["-129", "128"]),
        ("unsigned char", &["0", "255"], &["256", "-1"]),
        ("short", &["-32768", "32767"], &["-32769", "32768"]),
        ("unsigned short", &["0", "65535"], &["65536", "-1"]),
        ("int", &["-2147483648", "2147483647", "-1"], &["-2147483649", "2147483648"]),
        ("unsigned int", &["0", "4294967295"], &["4294967296", "-1"]),
        ("long long", &["-9223372036854775808", "9223372036854775807", "-1"], &[]),
        ("unsigned long long", &["0", "9223372036854775807"], &["-1"]),
        ("_Bool", &["0", "1"], &["2", "-1"]),
    ];
    for (n, (ty, ok, bad)) in cases.iter().enumerate() {
        for v in *ok {
            let decl = format!("enum Ok{n}_{} : {ty} {{ K = {v} }};", v.replace('-', "m"));
            let ct = declare(&f, &decl);
            let signed = !ty.starts_with("unsigned") && *ty != "_Bool";
            let want = if signed { meta::TYPE_INT } else { meta::TYPE_UINT };
            assert_eq!(ct.get_metatype(), want, "{decl}");
        }
        for v in *bad {
            let decl = format!("enum Bad{n}_{} : {ty} {{ K = {v} }};", v.replace('-', "m"));
            let err = parse(&f, &decl).unwrap_err();
            assert!(err.contains("does not fit the underlying type"), "{decl}: {err}");
        }
    }
    let flag = declare(&f, "enum Flag : _Bool { OFF, ON };");
    assert_eq!(flag.get_size(), 1);
    let sc = declare(&f, "enum SC : signed char { SM = -1, S0 };");
    assert_eq!((sc.get_size(), sc.get_metatype()), (1, meta::TYPE_INT));
    assert_eq!(names(&sc, 0xff), ["SM"]);
}
