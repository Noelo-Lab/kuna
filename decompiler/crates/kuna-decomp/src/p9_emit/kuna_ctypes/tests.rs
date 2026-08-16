//! Unit tests for the C type speller. The interesting content is entirely in
//! the *disagreements* between data models — a table that is right for x86-64
//! Linux and wrong for x86-64 Windows would pass any single-model test.

use super::*;
use crate::dtype::type_metatype::*;

/// x86-64 System V: LP64, x87 long double.
const LP64: CDataModel = CDataModel::LP64;

/// i386 / ARM32 / MIPS32: ILP32. `long` is 4, so 8 bytes must reach `long long`.
const ILP32: CDataModel = CDataModel {
    char_size: 1,
    short_size: 2,
    int_size: 4,
    long_size: 4,
    long_long_size: 8,
    wchar_size: 4,
    float_size: 4,
    double_size: 8,
    long_double_size: 10,
};

/// x86-64 Windows: LLP64. 8-byte pointers but a 4-byte `long`, and `long double`
/// aliases `double`.
const LLP64: CDataModel = CDataModel {
    char_size: 1,
    short_size: 2,
    int_size: 4,
    long_size: 4,
    long_long_size: 8,
    wchar_size: 2,
    float_size: 4,
    double_size: 8,
    long_double_size: 8,
};

/// The whole point of the feature: the same size spells differently per target.
/// An 8-byte integer is `long` under LP64 and `long long` under ILP32/LLP64,
/// because `long` is 4 there — the case a hard-coded table gets wrong.
#[test]
fn eight_byte_integers_follow_the_target_long() {
    assert_eq!(LP64.integer_spelling(8, true), Some("long"));
    assert_eq!(LP64.integer_spelling(8, false), Some("unsigned long"));
    assert_eq!(ILP32.integer_spelling(8, true), Some("long long"));
    assert_eq!(ILP32.integer_spelling(8, false), Some("unsigned long long"));
    assert_eq!(LLP64.integer_spelling(8, true), Some("long long"));
}

/// Declaration order decides the LP64 tie: `long` and `long long` are both 8
/// bytes there, and `long` must win. Reversing the rank order would silently
/// produce `long long` on every x86-64 Linux binary.
#[test]
fn lp64_prefers_long_over_long_long() {
    assert_eq!(LP64.long_size, LP64.long_long_size);
    assert_eq!(LP64.integer_spelling(8, true), Some("long"));
}

/// The small widths, and the signedness split. A 1-byte integer is `signed char`
/// / `unsigned char`, never bare `char` — whose signedness is
/// implementation-defined and which kuna reserves for text.
#[test]
fn small_integers_are_explicitly_signed() {
    assert_eq!(LP64.integer_spelling(1, true), Some("signed char"));
    assert_eq!(LP64.integer_spelling(1, false), Some("unsigned char"));
    assert_eq!(LP64.integer_spelling(2, true), Some("short"));
    assert_eq!(LP64.integer_spelling(2, false), Some("unsigned short"));
    assert_eq!(LP64.integer_spelling(4, true), Some("int"));
    assert_eq!(LP64.integer_spelling(4, false), Some("unsigned int"));
}

/// Sizes no C type has must stay unspelled rather than be widened: `(undefined3)x`
/// is a 24-bit truncation and `(unsigned int)x` is not.
#[test]
fn residual_integer_widths_have_no_spelling() {
    for size in [3, 5, 6, 7, 16] {
        assert_eq!(LP64.integer_spelling(size, false), None, "size {size}");
        assert_eq!(ILP32.integer_spelling(size, true), None, "size {size}");
    }
}

/// Floats: exact matches, then the widest-type approximation above `double`.
/// The x87 `float10` has no exact match on any target, so it reaches
/// `long double` through the width rule, not through a size equality.
#[test]
fn floats_match_exactly_then_widen_to_long_double() {
    assert_eq!(LP64.float_spelling(4), Some("float"));
    assert_eq!(LP64.float_spelling(8), Some("double"));
    assert_eq!(LP64.float_spelling(10), Some("long double"));
    assert_eq!(LP64.float_spelling(16), Some("long double"));
    // MSVC aliases long double to double, so an 8-byte float is `double` (the
    // exact match wins over the long-double rank) and a 10-byte x87 value still
    // has to reach `long double`, the only wider name C offers.
    assert_eq!(LLP64.float_spelling(8), Some("double"));
    assert_eq!(LLP64.float_spelling(10), Some("long double"));
}

/// A width below `float` names nothing (there is no `_Float16` in the emitted
/// dialect), so it keeps its Ghidra name rather than being rounded up.
#[test]
fn sub_float_widths_have_no_spelling() {
    assert_eq!(LP64.float_spelling(2), None);
}

/// The metatype dispatch: signedness comes from INT vs UINT, `code` becomes
/// `void` so `code *` reads `void *`, and everything else keeps its own name.
#[test]
fn metatype_dispatch() {
    assert_eq!(core_type_spelling(&LP64, TYPE_INT, 4, false), Some("int"));
    assert_eq!(core_type_spelling(&LP64, TYPE_UINT, 4, false), Some("unsigned int"));
    assert_eq!(core_type_spelling(&LP64, TYPE_FLOAT, 8, false), Some("double"));
    assert_eq!(core_type_spelling(&LP64, TYPE_CODE, 1, false), Some("void"));
    // TYPE_BOOL already spells `bool`; TYPE_UNKNOWN belongs to the older
    // `realtypes` relabel, which answers a different question and keeps its own
    // table so `option realtypes off` still restores upstream exactly.
    assert_eq!(core_type_spelling(&LP64, TYPE_BOOL, 1, false), None);
    assert_eq!(core_type_spelling(&LP64, TYPE_UNKNOWN, 4, false), None);
    assert_eq!(core_type_spelling(&LP64, TYPE_VOID, 1, false), None);
}

/// The text types: `char` stays `char`, and a wide character type becomes
/// `wchar_t` only where its width actually matches the target's declaration —
/// on x86-64 Linux that is `wchar4`, on Windows `wchar2`. The other one falls
/// back to a plain integer rather than claiming a width `wchar_t` lacks there.
#[test]
fn character_types_track_the_target_wchar() {
    assert_eq!(core_type_spelling(&LP64, TYPE_INT, 1, true), Some("char"));
    assert_eq!(core_type_spelling(&LP64, TYPE_INT, 4, true), Some("wchar_t"));
    assert_eq!(core_type_spelling(&LP64, TYPE_INT, 2, true), Some("short"));
    assert_eq!(core_type_spelling(&LLP64, TYPE_INT, 2, true), Some("wchar_t"));
    assert_eq!(core_type_spelling(&LLP64, TYPE_INT, 4, true), Some("int"));
}

/// Every spelling the table can produce must be a C type name — no Ghidra
/// vocabulary can leak through, on any model. This is the property the feature
/// exists to guarantee, asserted directly rather than inferred from the cases
/// above.
#[test]
fn no_spelling_is_ever_a_ghidra_name() {
    const VALID: &[&str] = &[
        "char", "signed char", "unsigned char", "short", "unsigned short", "int", "unsigned int",
        "long", "unsigned long", "long long", "unsigned long long", "wchar_t", "float", "double",
        "long double", "void",
    ];
    for m in [&LP64, &ILP32, &LLP64] {
        for meta in [TYPE_INT, TYPE_UINT, TYPE_FLOAT, TYPE_CODE, TYPE_BOOL, TYPE_UNKNOWN] {
            for size in 1..=16 {
                for is_char in [false, true] {
                    if let Some(s) = core_type_spelling(m, meta, size, is_char) {
                        assert!(VALID.contains(&s), "{meta:?} size {size} spelled {s:?}");
                    }
                }
            }
        }
    }
}
