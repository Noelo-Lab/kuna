//! Verifier adversarial tests for `rport/w10-console-family` (round 1).
//!
//! Focus of this wave: the console map/print/retype/rename command family plus
//! the supporting transcriptions — `Funcdata::printRaw` (SeqNum hex), the
//! `Datatype::setDisplayFormat` bit math, and `ScopeInternal::retypeSymbol`.
//! These tests pin the *load-bearing, format-sensitive* details the hunt list
//! flagged as most fragile:
//!
//!  * the SeqNum `:uniq` token is emitted in **hex** (the stream-base carry-over
//!    from `Address::printRaw`) — a behavior change to an existing caller
//!    (`IopSpace::printRaw`), so it must be hex for *every* value, not just the
//!    handful in the corpus.
//!  * `Datatype::setDisplayFormat` clears the `force_format` bit-field and ORs in
//!    `format << 12`, round-tripping through `getDisplayFormat`, for every
//!    `encodeIntegerFormat` code 0..=5 (and replacing a prior format, not
//!    accumulating bits).
//!
//! Built with public APIs only (no porter notes).

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, ConstantSpace};

use kuna_decomp::dtype::{flags as dtype_flags, type_metatype, Datatype};

fn ram() -> Rc<AddrSpace> {
    // wordsize 1 so AddrSpace::printRaw never appends a `+cut` suffix.
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        0,
        0,
    ))
}

/// C++ `operator<<(ostream&,const SeqNum&)`: `sq.pc.printRaw(s); s << ':' <<
/// sq.uniq;`.  `Address::printRaw` leaves the stream base in `hex`, so `uniq`
/// is hexadecimal.  This must hold for a `uniq` that actually *differs* between
/// decimal and hex (>= 10), and for the boundary value 0.
#[test]
fn seqnum_uniq_is_hex_not_decimal() {
    let ram = ram();

    // uniq = 0x2a = 42: hex "2a" must NOT render as decimal "42".
    let sq = SeqNum::new(Address::new(Rc::clone(&ram), 0x100017), 0x2a);
    let mut s = String::new();
    sq.print_raw(&mut s).expect("SeqNum::print_raw");
    assert!(
        s.ends_with(":2a"),
        "uniq must be hex (`:2a`), got {s:?} — a decimal `:42` is the LOSS-bug this wave fixed"
    );
    assert!(!s.ends_with(":42"), "uniq rendered in decimal: {s:?}");

    // uniq = 0xff = 255.
    let mut s2 = String::new();
    SeqNum::new(Address::new(Rc::clone(&ram), 0x100020), 0xff)
        .print_raw(&mut s2)
        .expect("SeqNum::print_raw");
    assert!(s2.ends_with(":ff"), "got {s2:?}");

    // uniq = 0: hex and dec agree, but the `:0` separator must still be present.
    let mut s3 = String::new();
    SeqNum::new(Address::new(Rc::clone(&ram), 0x100000), 0)
        .print_raw(&mut s3)
        .expect("SeqNum::print_raw");
    assert!(s3.ends_with(":0"), "got {s3:?}");
    // The address portion itself is always hex with a `0x` prefix.
    assert!(s3.starts_with("0x"), "address not hex-prefixed: {s3:?}");
}

/// A constant-space address (the `#`-shortcut operand) renders `0x<hex>` — the
/// base for the `#0xNN` constant tokens the printRaw listing emits.  Pin that a
/// value >= 10 is hex, never decimal.
#[test]
fn constant_space_address_is_hex() {
    let cst = Rc::new(ConstantSpace::new());
    let mut s = String::new();
    Address::new(cst, 0x64).print_raw(&mut s).expect("const Address::print_raw");
    assert_eq!(s, "0x64", "constant offset 0x64 must be hex, got {s:?}");
}

/// C++ `Datatype::setDisplayFormat(format)`: `flags &= ~force_format; flags |=
/// (format << 12)`.  Round-trips through `getDisplayFormat` for every
/// `encodeIntegerFormat` code, and *replaces* (does not accumulate) a prior
/// format.
#[test]
fn display_format_bit_math_round_trips_and_replaces() {
    // encodeIntegerFormat codes: hex=1 dec=2 oct=3 bin=4 char=5; 0 clears.
    for code in 0u32..=5 {
        let mut dt = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
        dt.set_display_format(code);
        assert_eq!(
            dt.get_display_format(),
            code,
            "display-format code {code} did not round-trip"
        );
        // The format occupies exactly the force_format bit-field (`<< 12`).
        assert_eq!(
            dt.flags & dtype_flags::force_format,
            code << 12,
            "force_format bits wrong for code {code}"
        );
        // No bits leak outside force_format (besides what we set).
        assert_eq!(
            dt.flags & !dtype_flags::force_format,
            0,
            "set_display_format polluted bits outside force_format for code {code}"
        );
    }

    // Setting a second format REPLACES the first (clear-then-or), not ORs into it.
    let mut dt = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    dt.set_display_format(1); // hex
    dt.set_display_format(5); // char
    assert_eq!(dt.get_display_format(), 5, "second set_display_format must replace, not accumulate");
    assert_eq!(dt.flags & dtype_flags::force_format, 5 << 12);
}

/// The `force_format` mask is the C++ `Datatype::force_format = 0x7000` (three
/// bits at offset 12), so codes 1..=5 (which need 3 bits, max value 5) all fit.
#[test]
fn force_format_mask_holds_all_codes() {
    assert_eq!(dtype_flags::force_format, 0x7000);
    // Max code 5 shifted left 12 stays inside the mask.
    assert_eq!((5u32 << 12) & dtype_flags::force_format, 5u32 << 12);
    // A hypothetical code 7 (all three bits) still fits; 8 would overflow — proving
    // the field is exactly 3 bits and the encodeIntegerFormat range (1..5) is safe.
    assert_eq!((7u32 << 12) & dtype_flags::force_format, 7u32 << 12);
    assert_ne!((8u32 << 12) & dtype_flags::force_format, 8u32 << 12);
}
