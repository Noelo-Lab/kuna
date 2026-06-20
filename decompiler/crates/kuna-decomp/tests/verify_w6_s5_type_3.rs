//! Adversarial verifier tests for item `w6-s5-type-3` (TypeFactory container +
//! the type-3 `TypePointerRel` overrides in `decompiler/cpp/type.cc`,
//! lines ~3072-3147 and the factory body 3565-end).
//!
//! The hunt list flagged these as the most fragile spots for this item:
//!
//!   * **Integer widths.** `TypePointerRel::isPtrsubMatching` (type.cc:3138-3147)
//!     stores its running offset in an `int4 iOff`, so the C++ truncates the
//!     `addressToByteInt`/`offset`/`extra` sum to 32 bits before the bound check.
//!     The port keeps the whole computation in `int8` and never truncates — a real
//!     (if only-at-pathological-inputs) divergence. F1 below pins the boundary.
//!   * **Comparator totality / field order.** `TypePointerRel::compare`
//!     (3072-3090) and `compareDependency` (3092-3103) drive the factory's `tree`
//!     ordering (`DatatypeCompare`); a wrong tie-break or field order silently
//!     corrupts the interning set.  These exercise the formal-vs-ephemeral
//!     `stripped` tie-break and the exact field precedence
//!     (ptrto, offset, parent, wordsize, size).
//!   * **Wrapping / signedness.** `hashName`/`hashSize` (type.cc:693-720) are
//!     bit-exact u64 hashes; the `(uint8)nm[i]` of a *signed* `char` sign-extends
//!     in C++.  The port masks to a byte (ASCII-only) — documented divergence.
//!
//! C++ oracle anchors are cited inline.

use std::rc::Rc;

use kuna_decomp::dtype::{Datatype, DatatypeKind, sub_metatype, type_metatype};

fn int4_t() -> Rc<Datatype> {
    Rc::new(Datatype::new(4, type_metatype::TYPE_INT))
}

/// Build a plain `TypePointer(8, ptrto, ws)` value (the `stripped` payload a
/// markEphemeral relative pointer would carry).
fn plain_pointer(ptrto: Rc<Datatype>, ws: u32) -> Datatype {
    let mut p = Datatype::new(8, type_metatype::TYPE_PTR);
    p.submeta = sub_metatype::SUB_PTR;
    p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: ws };
    p
}

/// Build a `TypePointerRel` Datatype of pointer-size `size` with the given fields,
/// mirroring `TypePointerRel(size,ptrTo,ws,parent,off)` (submeta SUB_PTRREL for a
/// formal relative pointer; markEphemeral lowers it and sets `stripped`).
fn ptrrel(
    size: int4_alias,
    ptrto: Rc<Datatype>,
    wordsize: u32,
    stripped: Option<Rc<Datatype>>,
    parent: Rc<Datatype>,
    offset: int4_alias,
) -> Datatype {
    let mut pr = Datatype::new_with_align(size, -1, type_metatype::TYPE_PTRREL);
    pr.submeta = sub_metatype::SUB_PTRREL;
    pr.kind = DatatypeKind::PointerRel { ptrto, wordsize, stripped, parent, offset };
    pr
}

#[allow(non_camel_case_types)]
type int4_alias = i32;

/// **F1 (the width divergence).** `TypePointerRel::isPtrsubMatching`
/// (type.cc:3143-3146) computes `int4 iOff = addressToByteInt(off,ws); extra =
/// addressToByteInt(extra,ws); iOff += offset + extra;` and returns
/// `0 <= iOff <= parent->getSize()`.  Because `iOff` is a **32-bit** `int4` the
/// C++ sum truncates to 32 bits before the comparison; the Rust port keeps the
/// computation in `int8`.
///
/// This test pins the *agreeing* domain (all realistic small offsets) so a future
/// regression in the bound arithmetic is caught, and documents the boundary
/// where the int4-truncation would diverge.  With `wordsize == 1`,
/// `addressToByteInt` is the identity, so `iOff = off + offset + extra`.
#[test]
fn f1_ptrrel_isptrsubmatching_bounds_realistic_domain() {
    let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT)); // getSize()==16
    // formal relative pointer (stripped == None) at offset 4 into the parent.
    let pr = ptrrel(8, int4_t(), 1, None, Rc::clone(&parent), 4);

    // iOff = off(0) + offset(4) + extra(0) = 4 ; 0 <= 4 <= 16 -> true
    assert!(pr.is_ptrsub_matching(0, 0, 1).unwrap());
    // Upper boundary inclusive: iOff = 0 + 4 + 12 = 16 ; 16 <= 16 -> true
    assert!(pr.is_ptrsub_matching(0, 12, 1).unwrap());
    // One past the upper boundary: iOff = 17 -> false
    assert!(!pr.is_ptrsub_matching(0, 13, 1).unwrap());
    // Lower boundary: iOff = off(-5) + 4 + 0 = -1 -> false (negative)
    assert!(!pr.is_ptrsub_matching(-5, 0, 1).unwrap());
    // Exactly zero is in-bounds: iOff = off(-4) + 4 + 0 = 0 -> true
    assert!(pr.is_ptrsub_matching(-4, 0, 1).unwrap());

    // The stripped (ephemeral) form defers to the inherited TypePointer body
    // (type.cc:3141-3142); ptrto is a plain int (not STRUCT/ARRAY/UNION/SPACEBASE)
    // so TypePointer::isPtrsubMatching falls to the `else return false` arm.
    let strip: Rc<Datatype> = Rc::new(plain_pointer(int4_t(), 1));
    let pr_eph = ptrrel(8, int4_t(), 1, Some(strip), Rc::clone(&parent), 4);
    assert!(!pr_eph.is_ptrsub_matching(0, 0, 1).unwrap());
}

/// **F1 continued — the truncation boundary the port does NOT reproduce.**
/// In C++ `iOff` is `int4`; a sum of `0x1_0000_0004` truncates to `4`, so the
/// C++ would return `true` (4 in [0,16]) while the Rust int8 path sees
/// `0x1_0000_0004 = 4294967300 > 16` and returns `false`.  We assert the *Rust*
/// behaviour here and flag the divergence in the verdict (F1, minor — the input
/// is a 4 GiB pointer offset, outside the realistic PTRSUB domain where
/// `parent->getSize()` is an int4).
#[test]
fn f1_ptrrel_isptrsubmatching_int4_truncation_divergence_documented() {
    let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
    let pr = ptrrel(8, int4_t(), 1, None, Rc::clone(&parent), 4);
    // off = 0x1_0000_0000 (2^32): C++ int4 iOff would wrap this contribution to 0,
    // giving iOff == 4 (in-bounds, true).  The port computes in int8: 0x1_0000_0004,
    // which exceeds 16 -> false.  Documented as F1.
    let off: i64 = 0x1_0000_0000;
    assert!(
        !pr.is_ptrsub_matching(off, 0, 1).unwrap(),
        "Rust int8 path: a 2^32 offset is out of bounds (C++ int4 would truncate to in-bounds)"
    );
}

/// **F2 — `TypePointerRel::compare` formal-vs-ephemeral tie-break + totality**
/// (type.cc:3081-3089).  After comparing equal as plain pointers, the *formal*
/// version (`stripped == null`) sorts before the *ephemeral* one
/// (`stripped != null`); two same-kind relative pointers tie at 0.  We also check
/// antisymmetry (a<b implies b>a) — a non-total tie-break would corrupt the
/// `DatatypeCompare` tree.
#[test]
fn f2_ptrrel_compare_formal_ephemeral_totality() {
    let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
    let strip: Rc<Datatype> = Rc::new(plain_pointer(int4_t(), 1));
    let formal = ptrrel(8, int4_t(), 1, None, Rc::clone(&parent), 4);
    let ephem = ptrrel(8, int4_t(), 1, Some(Rc::clone(&strip)), Rc::clone(&parent), 4);

    // formal (None) < ephemeral (Some) -> -1, and antisymmetric.
    assert_eq!(formal.compare(&ephem, 10).unwrap(), -1);
    assert_eq!(ephem.compare(&formal, 10).unwrap(), 1);
    // Reflexive-ish: identical formals tie.
    let formal2 = ptrrel(8, int4_t(), 1, None, Rc::clone(&parent), 4);
    assert_eq!(formal.compare(&formal2, 10).unwrap(), 0);
    // Two ephemerals also tie (both Some) — the tie-break only fires on a None/Some
    // mismatch (C++: the else-branch returns 1 only when tp->stripped == null).
    let ephem2 = ptrrel(8, int4_t(), 1, Some(strip), Rc::clone(&parent), 4);
    assert_eq!(ephem.compare(&ephem2, 10).unwrap(), 0);
}

/// **F3 — `TypePointerRel::compareDependency` exact field precedence**
/// (type.cc:3095-3102): submeta, then ptrto-identity, **offset**, parent-identity,
/// wordsize, then `op.getSize() - size`.  The offset comparison precedes the
/// parent comparison; a transposition would reorder the interning tree.  We drive
/// each tier independently and assert strict antisymmetry / the size tail.
#[test]
fn f3_ptrrel_compare_dependency_field_precedence() {
    // The factory interns each sub-type once; `compareDependency` then compares
    // `ptrto`/`parent` by raw-pointer identity (type.cc:3097/3099).  Faithful
    // fixtures therefore SHARE one `Rc` per logical sub-type — two `Rc::new`
    // allocations of the "same" int are distinct objects and (correctly) do not
    // tie, exactly as two un-interned C++ `Datatype*` would not.
    let ptrto = int4_t(); // one shared interned-like ptrto object
    let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
    let parent2 = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT)); // distinct object
    let a = ptrrel(8, Rc::clone(&ptrto), 1, None, Rc::clone(&parent), 4);

    // Identical fields and SHARED identity -> 0 (the interning-equal case).
    let a2 = ptrrel(8, Rc::clone(&ptrto), 1, None, Rc::clone(&parent), 4);
    assert_eq!(a.compare_dependency(&a2).unwrap(), 0);

    // Offset is the discriminator after ptrto-identity (equal here): different
    // offset orders by offset BEFORE parent identity is even consulted.
    let b = ptrrel(8, Rc::clone(&ptrto), 1, None, Rc::clone(&parent), 8);
    assert_eq!(a.compare_dependency(&b).unwrap(), -1);
    assert_eq!(b.compare_dependency(&a).unwrap(), 1);

    // Same ptrto + offset, *different parent object* -> ordered by parent pointer
    // identity (raw-address order in C++; Rc::as_ptr in the port).  Antisymmetric,
    // non-zero.
    let c = ptrrel(8, Rc::clone(&ptrto), 1, None, Rc::clone(&parent2), 4);
    let ac = a.compare_dependency(&c).unwrap();
    let ca = c.compare_dependency(&a).unwrap();
    assert_ne!(ac, 0, "distinct parent objects must not tie");
    assert_eq!(ac, -ca, "parent-identity order must be antisymmetric");

    // ptrto-identity is the FIRST sub-tier after submeta: a distinct ptrto object
    // discriminates before offset/parent are looked at.
    let e = ptrrel(8, int4_t(), 1, None, Rc::clone(&parent), 4); // distinct ptrto
    let ae = a.compare_dependency(&e).unwrap();
    assert_ne!(ae, 0, "distinct ptrto objects must not tie");
    assert_eq!(ae, -e.compare_dependency(&a).unwrap(), "ptrto order antisymmetric");

    // Same ptrto/offset/parent/wordsize, different total size -> tail
    // `op.getSize() - size` (wrapping i32). a.size==8; make d.size==6 so
    // a.compare_dependency(d) returns d.size - a.size = 6 - 8 = -2.
    let d = ptrrel(6, Rc::clone(&ptrto), 1, None, Rc::clone(&parent), 4);
    assert_eq!(a.compare_dependency(&d).unwrap(), -2);
    assert_eq!(d.compare_dependency(&a).unwrap(), 2);
}

/// **F4 — `hashName`/`hashSize` bit-exactness and the documented signedness
/// scope** (type.cc:693-720).  `hashName` sets the two top header bits; `hashSize`
/// is an involution for a fixed size.  The ASCII-only `(uint8)nm[i]` masking is a
/// documented divergence from the C++ signed-`char` sign-extension — we assert the
/// header bits and reversibility, which hold regardless.
#[test]
fn f4_hash_name_size_properties() {
    // Two top header bits always set (C++ `res |= 0xC000000000000000`).
    let h = Datatype::hash_name("int");
    assert_eq!(h & 0xC000_0000_0000_0000, 0xC000_0000_0000_0000);
    // Deterministic and name-sensitive.
    assert_eq!(h, Datatype::hash_name("int"));
    assert_ne!(h, Datatype::hash_name("uint"));
    assert_ne!(Datatype::hash_name(""), Datatype::hash_name("a"));
    // The empty name hashes to just the header bits OR'd onto the seed 123.
    assert_eq!(Datatype::hash_name(""), 123u64 | 0xC000_0000_0000_0000);

    // hashSize is reversible (an involution for a fixed size): applying twice with
    // the same size restores the input id (C++ doc: "reversible by feeding the
    // output ID back ... with the same size").
    for size in [1, 4, 7, 16, 255] {
        let id = 0x1234_5678_9abc_def0u64;
        let sized = Datatype::hash_size(id, size);
        assert_eq!(Datatype::hash_size(sized, size), id, "hashSize involution at size {size}");
    }
    // A negative size sign-extends to u64 before the multiply (C++
    // `uint8 sizeHash = size;`): size = -1 hashes differently from size = 1 and is
    // still reversible.
    let id = 0xdead_beef_0000_0001u64;
    let neg = Datatype::hash_size(id, -1);
    assert_ne!(neg, Datatype::hash_size(id, 1));
    assert_eq!(Datatype::hash_size(neg, -1), id);
}
