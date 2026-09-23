//! Tests for the value-preservation predicate of `castimplied`.
//!
//! Which cast a function prints needs a decompile and is covered end to end by
//! `tests/stages/kuna-castimplied.xml` (pass 1 off, pass 2 on) and by the
//! compiled round trip in `kuna-cli/tests/decompile_all_cli.rs`
//! (`an_implied_cast_round_trips_through_the_printed_c`).  What is pinned here
//! is the arithmetic every removal rests on: a conversion is left out only when
//! C's conversion to its target keeps every value of its source.

use super::*;

use crate::dtype::{TypeFactory, TypeFactoryImpl};

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("bool", 1, type_metatype::TYPE_BOOL, false).unwrap();
    for (size, u, i) in [(1, "uint1", "int1"), (2, "uint2", "int2"), (4, "uint4", "int4"), (8, "uint8", "int8")] {
        f.set_core_type(u, size, type_metatype::TYPE_UINT, false).unwrap();
        f.set_core_type(i, size, type_metatype::TYPE_INT, false).unwrap();
    }
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.set_core_type("float4", 4, type_metatype::TYPE_FLOAT, false).unwrap();
    f.set_core_type("float8", 8, type_metatype::TYPE_FLOAT, false).unwrap();
    f.set_core_type("xunknown4", 4, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn int(f: &TypeFactoryImpl, size: i32) -> Rc<Datatype> {
    f.get_base(size, type_metatype::TYPE_INT).unwrap()
}

fn uint(f: &TypeFactoryImpl, size: i32) -> Rc<Datatype> {
    f.get_base(size, type_metatype::TYPE_UINT).unwrap()
}

/// C's conversion of the value `v` to the integer type `t` (C11 6.3.1.3): kept
/// when representable, else reduced modulo 2^N -- the unsigned rule, and what gcc
/// and clang define for the signed case.
fn c_convert(v: i128, t: &Datatype) -> i128 {
    let bits = 8 * t.get_size() as u32;
    let m = v.rem_euclid(1i128 << bits);
    if t.get_metatype() == type_metatype::TYPE_INT && m >= 1i128 << (bits - 1) {
        m - (1i128 << bits)
    } else {
        m
    }
}

#[test]
fn the_ranges_are_the_c_ranges() {
    let f = factory();
    assert_eq!(int_range(&int(&f, 1)), Some((-128, 127)));
    assert_eq!(int_range(&uint(&f, 1)), Some((0, 255)));
    assert_eq!(int_range(&int(&f, 2)), Some((-32768, 32767)));
    assert_eq!(int_range(&uint(&f, 4)), Some((0, 0xffff_ffff)));
    assert_eq!(int_range(&int(&f, 8)), Some((i64::MIN as i128, i64::MAX as i128)));
    assert_eq!(int_range(&uint(&f, 8)), Some((0, u64::MAX as i128)));
    assert_eq!(int_range(&f.get_base(1, type_metatype::TYPE_BOOL).unwrap()), Some((0, 1)));
    assert_eq!(int_range(&f.get_base(1, type_metatype::TYPE_INT).unwrap()), Some((-128, 127)));
}

#[test]
fn a_float_a_pointer_or_an_undecided_type_is_never_an_integer() {
    let f = factory();
    let float4 = f.get_base(4, type_metatype::TYPE_FLOAT).unwrap();
    let unknown = f.get_base(4, type_metatype::TYPE_UNKNOWN).unwrap();
    let ptr = f.get_type_pointer(8, int(&f, 4), 1).unwrap();
    for t in [&float4, &unknown, &ptr] {
        assert_eq!(int_range(t), None, "{}", t.get_name());
        assert!(!preserves(t, &int(&f, 8)), "{}", t.get_name());
        assert!(!preserves(&int(&f, 4), t), "{}", t.get_name());
    }
}

#[test]
fn a_conversion_to_bool_is_a_test_not_a_widening() {
    let f = factory();
    let b = f.get_base(1, type_metatype::TYPE_BOOL).unwrap();
    assert!(!preserves(&b, &b));
    assert!(!preserves(&uint(&f, 1), &b));
    assert!(preserves(&b, &int(&f, 4)));
}

#[test]
fn widenings_that_keep_the_value_are_accepted() {
    let f = factory();
    for (src, dst) in [
        (int(&f, 1), int(&f, 4)),
        (int(&f, 2), int(&f, 8)),
        (int(&f, 4), int(&f, 8)),
        (uint(&f, 1), int(&f, 4)),
        (uint(&f, 1), uint(&f, 4)),
        (uint(&f, 2), int(&f, 4)),
        (uint(&f, 4), int(&f, 8)),
        (uint(&f, 4), uint(&f, 8)),
        (int(&f, 4), int(&f, 4)),
    ] {
        assert!(preserves(&src, &dst), "{} -> {}", src.get_name(), dst.get_name());
    }
}

#[test]
fn sign_changes_and_narrowings_are_refused() {
    let f = factory();
    for (src, dst) in [
        (int(&f, 1), uint(&f, 4)),
        (int(&f, 4), uint(&f, 8)),
        (int(&f, 4), uint(&f, 4)),
        (uint(&f, 4), int(&f, 4)),
        (uint(&f, 8), int(&f, 8)),
        (int(&f, 8), int(&f, 4)),
        (uint(&f, 4), uint(&f, 2)),
        (uint(&f, 1), int(&f, 1)),
    ] {
        assert!(!preserves(&src, &dst), "{} -> {}", src.get_name(), dst.get_name());
    }
}

/// The predicate agrees with C's own conversion over every pair of integer
/// types: accepted exactly when no value of the source changes.
#[test]
fn preserves_is_exactly_every_value_surviving_c_conversion() {
    let f = factory();
    let mut all = vec![f.get_base(1, type_metatype::TYPE_BOOL).unwrap()];
    for size in [1, 2, 4, 8] {
        all.push(int(&f, size));
        all.push(uint(&f, size));
    }
    for src in &all {
        let (lo, hi) = int_range(src).unwrap();
        let samples = [lo, lo + 1, -1, 0, 1, hi - 1, hi, 127, 128, 255, 256, 65535, 65536];
        for dst in &all {
            if dst.get_metatype() == type_metatype::TYPE_BOOL {
                continue;
            }
            let survives = samples
                .iter()
                .filter(|&&v| lo <= v && v <= hi)
                .all(|&v| c_convert(v, dst) == v);
            assert_eq!(preserves(src, dst), survives, "{} -> {}", src.get_name(), dst.get_name());
        }
    }
}
