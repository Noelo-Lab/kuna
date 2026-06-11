//! Adversarial verifier tests for item `w5-dtype-expand`.
//!
//! These target the spots the hunt list flagged as most fragile for the
//! `type.hh` `Datatype` interface port:
//!
//!   * comparator routing / totality (`Datatype::compare` /
//!     `compareDependency`), where a wrong subclass grouping silently corrupts
//!     ordering — the riskiest surface in this item;
//!   * the base `compare` body's signed `op.size - size` subtraction (the C++
//!     wrapping vs. Rust panic boundary);
//!   * `string2metatype` boundary inputs (empty string, known-first-char with a
//!     non-matching tail), which the C++ resolves via `metastring[0]` + fallthrough.
//!
//! C++ oracle: `decompiler/cpp/type.cc` (`Datatype::compare` 216-222,
//! `Datatype::compareDependency` 231-237, `TypeSpacebase::compare` 3498-3501,
//! `TypeSpacebase::compareDependency` 3504-3514, `string2metatype` 307-369) and
//! `decompiler/cpp/type.hh` (`TypeSpacebase` 799-824, declaring `compare`/
//! `compareDependency` as overrides).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{AddrSpace, ConstantSpace};
use kuna_decomp::dtype::{
    metatype2string, string2metatype, sub_metatype, type_metatype, Datatype, DatatypeKind,
};

fn const_space() -> Rc<AddrSpace> {
    Rc::new(ConstantSpace::new())
}

/// Build a `TYPE_SPACEBASE` Datatype with the given spaceid/localframe, mirroring
/// the C++ `TypeSpacebase(int4=0, align=1, TYPE_SPACEBASE)` shape (size 0).
fn spacebase(spaceid: Option<Rc<AddrSpace>>, localframe: Address) -> Datatype {
    let mut d = Datatype::new_with_align(0, 1, type_metatype::TYPE_SPACEBASE);
    d.kind = DatatypeKind::Spacebase {
        spaceid,
        localframe,
    };
    d
}

/// **The bug.** `TypeSpacebase` overrides `compare`/`compareDependency`
/// (type.hh:821-822): after the base submeta/size comparison returns 0 (which it
/// always does for two spacebases — both size 0, both SUB_SPACEBASE), the C++
/// `TypeSpacebase::compareDependency` (type.cc:3504-3514) tie-breaks on
/// `spaceid` then `localframe`, and `TypeSpacebase::compare` (3498-3501) just
/// delegates to it. The port routes `DatatypeKind::Spacebase` to the *base*
/// `compare` body, so two distinct spacebases (same submeta+size, different
/// localframe) wrongly compare **equal** (`Ok(0)`).
///
/// Since the porter's own rule seams every other subclass `compare` override to
/// `Err(SEAM(W6))`, the faithful behavior here is *either* a SEAM error *or* the
/// real tie-break — never a silent `Ok(0)`. This asserts that contract; under the
/// current routing it FAILS (the port returns `Ok(0)`), which is the divergence
/// trace backing the REJECT.
#[test]
fn w5_dtype_spacebase_compare_must_not_silently_equal_distinct_frames() {
    let spc = const_space();
    let frame_a = Address::new(Rc::clone(&spc), 0x1000);
    let frame_b = Address::new(Rc::clone(&spc), 0x2000);

    let a = spacebase(Some(Rc::clone(&spc)), frame_a);
    let b = spacebase(Some(Rc::clone(&spc)), frame_b);

    // C++ `TypeSpacebase::compareDependency`: base part is 0 (equal submeta+size),
    // spaceid equal, localframe valid and a.localframe != b.localframe ->
    // returns +/-1, NEVER 0. `compare` delegates to it.
    match a.compare(&b, 10) {
        Ok(0) => panic!(
            "Datatype::compare routed TYPE_SPACEBASE to the base body: two \
             distinct-localframe spacebases compared EQUAL (Ok(0)). C++ \
             TypeSpacebase::compare (type.cc:3498) tie-breaks on localframe and \
             cannot return 0 here. The override must be SEAM(W6)'d like every \
             other subclass compare."
        ),
        Ok(_n) => { /* a real tie-break would also be acceptable */ }
        Err(_seam) => { /* SEAM(W6) is the faithful interface-stage answer */ }
    }
}

/// Same divergence via `compareDependency` directly (type.cc:3504-3514): the
/// localframe tie-break is in `compareDependency`, so this is the primary site.
#[test]
fn w5_dtype_spacebase_compare_dependency_distinguishes_localframe() {
    let spc = const_space();
    let a = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x10));
    let b = spacebase(Some(Rc::clone(&spc)), Address::new(Rc::clone(&spc), 0x20));

    match a.compare_dependency(&b) {
        Ok(0) => panic!(
            "Datatype::compareDependency routed TYPE_SPACEBASE to the base body: \
             distinct-localframe spacebases compared EQUAL. C++ \
             TypeSpacebase::compareDependency (type.cc:3504) tie-breaks on \
             spaceid/localframe."
        ),
        Ok(_) | Err(_) => {}
    }
}

/// The kinds that genuinely use the base `compare` body (Base/Void/Unknown — no
/// C++ override) must reproduce type.cc:216-222 exactly: submeta drives the
/// sign, then `op.size - size` (signed `int4`, wrapping). This confirms the
/// *correct* path is faithful and brackets the Spacebase mis-routing.
#[test]
fn w5_dtype_base_compare_submeta_then_size_subtraction() {
    // INT (SUB_INT_PLAIN=17) vs UINT (SUB_UINT_PLAIN=16): uint is more specific,
    // so int.compare(uint): submeta 17 != 16, 17 < 16 is false -> +1.
    let int4_t = Datatype::new(4, type_metatype::TYPE_INT);
    let uint4_t = Datatype::new(4, type_metatype::TYPE_UINT);
    assert_eq!(int4_t.get_sub_meta(), sub_metatype::SUB_INT_PLAIN);
    assert_eq!(uint4_t.get_sub_meta(), sub_metatype::SUB_UINT_PLAIN);
    assert_eq!(int4_t.compare(&uint4_t, 10).unwrap(), 1);
    assert_eq!(uint4_t.compare(&int4_t, 10).unwrap(), -1);

    // Same submeta, different size: returns op.size - size (bigger types earlier).
    let int1_t = Datatype::new(1, type_metatype::TYPE_INT);
    let int9_t = Datatype::new(9, type_metatype::TYPE_INT);
    // int1.compare(int9): op.size(9) - size(1) = 8
    assert_eq!(int1_t.compare(&int9_t, 10).unwrap(), 8);
    // int9.compare(int1): op.size(1) - size(9) = -8
    assert_eq!(int9_t.compare(&int1_t, 10).unwrap(), -8);
    // Equal: 0.
    assert_eq!(int4_t.compare(&int4_t, 10).unwrap(), 0);

    // The unknown / void kinds also use the base body (no C++ override).
    let unk = Datatype::new(4, type_metatype::TYPE_UNKNOWN);
    let void = Datatype::new(0, type_metatype::TYPE_VOID);
    // SUB_UNKNOWN(21) < SUB_VOID(23) -> unknown.compare(void) == -1.
    assert_eq!(unk.compare(&void, 10).unwrap(), -1);
}

/// `string2metatype` boundary inputs. C++ switches on `metastring[0]`
/// (type.cc:308); an empty string in C++11 reads the NUL terminator -> `default`
/// -> throw. The Rust port uses `.first()` -> `None` -> error. A known first
/// char with a non-matching full string falls through the inner ifs and still
/// throws (the C++ `break` then final `throw`).
#[test]
fn w5_dtype_string2metatype_boundary_and_fallthrough() {
    // Empty string: both C++ (NUL -> default -> throw) and Rust error.
    assert!(string2metatype("").is_err());
    // Known first char 'p', unknown tail -> falls through inner ifs -> error.
    assert!(string2metatype("ptrz").is_err());
    assert!(string2metatype("partbogus").is_err());
    // 's' has two distinct matches (struct/spacebase) via separate ifs in C++.
    assert_eq!(string2metatype("struct").unwrap(), type_metatype::TYPE_STRUCT);
    assert_eq!(
        string2metatype("spacebase").unwrap(),
        type_metatype::TYPE_SPACEBASE
    );
    // 'u' three-way (unknown/uint/union).
    assert_eq!(string2metatype("union").unwrap(), type_metatype::TYPE_UNION);
    // The documented upstream asymmetry: "partenum" is emitted but not decodable.
    assert_eq!(
        metatype2string(type_metatype::TYPE_PARTIALENUM).unwrap(),
        "partenum"
    );
    assert!(string2metatype("partenum").is_err());
}
