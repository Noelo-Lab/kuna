//! Independent verifier adversarial tests for item `w6-s5-type-1`.
//!
//! Targets the spots the hunt list flagged as most fragile for the
//! `Datatype` base + scalar/pointer/array bodies (type.cc lines ~1-1722):
//!
//!   * **Comparator totality / submeta ordering** — the single most
//!     load-bearing fact: `submeta < op.submeta` in C++ compares the *integer*
//!     enum value (`sub_metatype` 0..23).  The Rust `sub_metatype` derives `Ord`
//!     but declares variants in *descending* discriminant order, so we pin that
//!     the Rust `<` still matches the C++ integer order (derived `Ord` on a
//!     fieldless enum compares discriminants, not declaration position).
//!     (cpp: type.cc:216-222, 1074-1093, 1363-1375)
//!   * **`compare` size finisher width / wrapping** — `op.size - size` is a
//!     signed `int4` result returned raw (NOT clamped to -1/+1).  (cpp:1099,1108)
//!   * **pointer space tie-break ordering** — no-space-later, else by getIndex.
//!     (cpp:1082-1086)
//!   * **`compareDependency` total order over a mixed corpus** including the
//!     `op.size - size` finisher and ptr/array identity tie-break — a non-total
//!     order silently corrupts the TypeFactory tree.  (cpp:1095-1108, 1377-1384)
//!   * **`getSubType` array offset renormalization at off==0 / off==size-1 /
//!     off>=size** and the pointer truncate window with bigendian min.
//!     (cpp:1061-1072, 1386-1393)
//!   * **`isPtrsubMatching` array branch** off!=0 / multiplier boundary, and the
//!     union branch always-false.  (cpp:1260-1312)
//!
//! C++ oracle: `decompiler/cpp/type.cc` at blob 9014a307…, `type.hh` at 0bafaef2….

use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::dtype::{sub_metatype, type_metatype, Datatype, DatatypeKind};

fn space_with_index(idx: i32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        idx,
        0,
        0,
        0,
    ))
}

fn make_pointer(ptrto: Rc<Datatype>, spaceid: Option<Rc<AddrSpace>>, wordsize: u32) -> Datatype {
    let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    p.kind = DatatypeKind::Pointer {
        ptrto,
        spaceid,
        truncate: None,
        wordsize,
    };
    p
}

fn make_array(arrayof: Rc<Datatype>, arraysize: i32, size: i32) -> Datatype {
    let mut a = Datatype::new_with_align(size, -1, type_metatype::TYPE_ARRAY);
    a.kind = DatatypeKind::Array { arrayof, arraysize };
    a
}

/// THE load-bearing fact: the Rust `sub_metatype` `<` must match the C++ integer
/// comparison `submeta < op.submeta`.  The C++ enum runs SUB_PARTIALUNION=0 ...
/// SUB_VOID=23 (lower = more specific).  We assert the Rust ordering agrees with
/// the explicit discriminant for the full set, NOT the (reversed) declaration
/// order.  If derived `Ord` followed declaration order this would flip every
/// pointer/array/scalar comparison and corrupt the factory tree.
#[test]
fn w6_type1_submeta_ord_matches_cpp_integer_value() {
    let all = [
        (sub_metatype::SUB_PARTIALUNION, 0),
        (sub_metatype::SUB_UNION, 1),
        (sub_metatype::SUB_STRUCT, 2),
        (sub_metatype::SUB_ARRAY, 3),
        (sub_metatype::SUB_PTR_STRUCT, 4),
        (sub_metatype::SUB_PTRREL, 5),
        (sub_metatype::SUB_PTR, 6),
        (sub_metatype::SUB_PTRREL_UNK, 7),
        (sub_metatype::SUB_FLOAT, 8),
        (sub_metatype::SUB_CODE, 9),
        (sub_metatype::SUB_BOOL, 10),
        (sub_metatype::SUB_UINT_UNICODE, 11),
        (sub_metatype::SUB_INT_UNICODE, 12),
        (sub_metatype::SUB_UINT_ENUM, 13),
        (sub_metatype::SUB_UINT_PARTIALENUM, 14),
        (sub_metatype::SUB_INT_ENUM, 15),
        (sub_metatype::SUB_UINT_PLAIN, 16),
        (sub_metatype::SUB_INT_PLAIN, 17),
        (sub_metatype::SUB_UINT_CHAR, 18),
        (sub_metatype::SUB_INT_CHAR, 19),
        (sub_metatype::SUB_PARTIALSTRUCT, 20),
        (sub_metatype::SUB_UNKNOWN, 21),
        (sub_metatype::SUB_SPACEBASE, 22),
        (sub_metatype::SUB_VOID, 23),
    ];
    for &(a, ai) in &all {
        assert_eq!(a.as_i32(), ai, "discriminant transcription drift");
        for &(b, bi) in &all {
            // Rust `<` must agree with the C++ integer comparison for every pair.
            assert_eq!(
                a < b,
                ai < bi,
                "sub_metatype Ord disagrees with C++ integer order at ({ai},{bi})"
            );
        }
    }
}

/// `Datatype::compare` finisher returns the *raw* signed `op.size - size`, not a
/// clamped -1/+1 (the C++ `return (op.size - size);`).  This matters because the
/// factory tree relies on this exact magnitude transitively.  Bigger types come
/// earlier (op smaller => positive => self after op).
#[test]
fn w6_type1_compare_size_finisher_is_raw_signed_diff() {
    // Same submeta (both plain TYPE_INT -> SUB_INT_PLAIN), different size.
    let i1 = Datatype::new(1, type_metatype::TYPE_INT);
    let i8 = Datatype::new(8, type_metatype::TYPE_INT);
    // op.size - size: 1.compare(8) => op.size(8) - size(1) = 7.
    assert_eq!(i1.compare(&i8, 10).unwrap(), 8 - 1);
    assert_eq!(i8.compare(&i1, 10).unwrap(), 1 - 8); // -7
    // compareDependency base uses the same finisher.
    assert_eq!(i1.compare_dependency(&i8).unwrap(), 8 - 1);
}

/// `TypePointer::compare` space tie-break (type.cc:1082-1086): a pointer with a
/// space sorts BEFORE one without; two spaces order by getIndex; and the index
/// tie-break is strict and antisymmetric.
#[test]
fn w6_type1_pointer_space_tiebreak_total_and_antisymmetric() {
    let i4 = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let s2 = space_with_index(2);
    let s9 = space_with_index(9);

    let p_none = make_pointer(Rc::clone(&i4), None, 1);
    let p_s2 = make_pointer(Rc::clone(&i4), Some(Rc::clone(&s2)), 1);
    let p_s9 = make_pointer(Rc::clone(&i4), Some(Rc::clone(&s9)), 1);

    // with-space earlier than no-space.
    assert_eq!(p_s2.compare(&p_none, 10).unwrap(), -1);
    assert_eq!(p_none.compare(&p_s2, 10).unwrap(), 1);
    // index order, both directions.
    assert_eq!(p_s2.compare(&p_s9, 10).unwrap(), -1);
    assert_eq!(p_s9.compare(&p_s2, 10).unwrap(), 1);
    // same dirs in compareDependency (it reuses the same space helper).
    assert_eq!(p_s2.compare_dependency(&p_none).unwrap(), -1);
    assert_eq!(p_s9.compare_dependency(&p_s2).unwrap(), 1);
}

/// Strict-weak-order / total-order audit over a mixed corpus of in-scope kinds
/// for BOTH comparators.  Reflexive, antisymmetric, and transitive on the strict
/// part — the guard against a silent comparator inversion or grouping bug.
#[test]
fn w6_type1_compare_and_dependency_total_order_audit() {
    let i1 = Rc::new(Datatype::new(1, type_metatype::TYPE_INT));
    let i4 = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let u4 = Rc::new(Datatype::new(4, type_metatype::TYPE_UINT));
    let f4 = Rc::new(Datatype::new(4, type_metatype::TYPE_FLOAT));
    let s1 = space_with_index(1);

    let corpus: Vec<Rc<Datatype>> = vec![
        Rc::clone(&i1),
        Rc::clone(&i4),
        Rc::clone(&u4),
        Rc::clone(&f4),
        Rc::new(make_pointer(Rc::clone(&i4), None, 1)),
        Rc::new(make_pointer(Rc::clone(&i4), None, 4)),
        Rc::new(make_pointer(Rc::clone(&i4), Some(Rc::clone(&s1)), 1)),
        Rc::new(make_pointer(Rc::clone(&f4), None, 1)),
        Rc::new(make_array(Rc::clone(&i1), 4, 4)),
        Rc::new(make_array(Rc::clone(&i4), 1, 4)),
    ];
    let sgn = |x: i32| x.signum();

    for a in &corpus {
        assert_eq!(a.compare(a, 10).unwrap(), 0);
        assert_eq!(a.compare_dependency(a).unwrap(), 0);
        for b in &corpus {
            assert_eq!(
                sgn(a.compare(b, 10).unwrap()),
                -sgn(b.compare(a, 10).unwrap()),
                "compare not antisymmetric"
            );
            assert_eq!(
                sgn(a.compare_dependency(b).unwrap()),
                -sgn(b.compare_dependency(a).unwrap()),
                "compareDependency not antisymmetric"
            );
        }
    }
    // Transitivity of the strict part for both comparators.
    for a in &corpus {
        for b in &corpus {
            for c in &corpus {
                let cmp_ab = sgn(a.compare(b, 10).unwrap());
                let cmp_bc = sgn(b.compare(c, 10).unwrap());
                let cmp_ac = sgn(a.compare(c, 10).unwrap());
                if cmp_ab < 0 && cmp_bc < 0 {
                    assert!(cmp_ac < 0, "compare not transitive");
                }
                let d_ab = sgn(a.compare_dependency(b).unwrap());
                let d_bc = sgn(b.compare_dependency(c).unwrap());
                let d_ac = sgn(a.compare_dependency(c).unwrap());
                if d_ab < 0 && d_bc < 0 {
                    assert!(d_ac < 0, "compareDependency not transitive");
                }
            }
        }
    }
}

/// `TypeArray::getSubType` (type.cc:1386-1393): renormalize off modulo the
/// element align-size at the boundaries (off==0, off==size-1, off==size).
/// `TypePointer::getSubType` truncate window, little-endian (min==0) and the
/// big-endian min == size - trunc->getSize().
#[test]
fn w6_type1_get_sub_type_boundaries() {
    let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    elem.align_size = 4;
    let arr = make_array(Rc::new(elem), 3, 12);

    // off==0 -> element, newoff 0.
    let (sub, no) = arr.get_sub_type(0).unwrap();
    assert_eq!(sub.unwrap().get_size(), 4);
    assert_eq!(no, 0);
    // off==size-1 (11) -> element, newoff 11%4 = 3.
    let (sub, no) = arr.get_sub_type(11).unwrap();
    assert!(sub.is_some());
    assert_eq!(no, 3);
    // off==size (12) -> base (None, off).
    let (sub, no) = arr.get_sub_type(12).unwrap();
    assert!(sub.is_none());
    assert_eq!(no, 12);

    // Pointer with a big-endian truncate window: min = size - trunc.size = 8-4=4.
    let i4 = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let trunc = Rc::new(Datatype::new(4, type_metatype::TYPE_PTR));
    let mut p_be = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    // truncate_bigendian = 0x8000.
    p_be.flags |= 0x8000;
    p_be.kind = DatatypeKind::Pointer {
        ptrto: Rc::clone(&i4),
        spaceid: None,
        truncate: Some(Rc::clone(&trunc)),
        wordsize: 1,
    };
    // off == 3 < min(4) -> outside window -> base (None, 3).
    let (sub, no) = p_be.get_sub_type(3).unwrap();
    assert!(sub.is_none());
    assert_eq!(no, 3);
    // off == 4 (== min) -> inside [4, 8) -> truncated form, newoff = 4 - 4 = 0.
    let (sub, no) = p_be.get_sub_type(4).unwrap();
    assert_eq!(sub.unwrap().get_size(), 4);
    assert_eq!(no, 0);
    // off == 7 -> inside, newoff = 7 - 4 = 3.
    let (_sub, no) = p_be.get_sub_type(7).unwrap();
    assert_eq!(no, 3);
    // off == 8 -> outside upper bound (min + trunc.size = 8) -> base.
    let (sub, no) = p_be.get_sub_type(8).unwrap();
    assert!(sub.is_none());
    assert_eq!(no, 8);
}

/// `TypePointer::isPtrsubMatching` array branch (type.cc:1271-1278) and the union
/// always-false branch (1305-1310).  off must be 0 for array; multiplier must be
/// strictly below the *array's* align-size (C++ uses `ptrto->getAlignSize()`,
/// i.e. the array object, not the element type).
#[test]
fn w6_type1_is_ptrsub_matching_array_and_union_branches() {
    // ptr -> array of int4, array size 8 so the array's own align-size is 8.
    let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    elem.align_size = 4;
    let arr = make_array(Rc::new(elem), 2, 8);
    assert_eq!(arr.get_align_size(), 8, "array align-size precondition");
    let p_arr: Rc<Datatype> = Rc::new(make_pointer(Rc::new(arr), None, 1));

    // off == 0, multiplier 1 < array align 8 -> true.
    assert!(p_arr.is_ptrsub_matching(0, 0, 1).unwrap());
    // off != 0 -> false (we are not at the array head).
    assert!(!p_arr.is_ptrsub_matching(1, 0, 1).unwrap());
    // multiplier == array align-size (8) -> NOT strictly below -> false.
    assert!(!p_arr.is_ptrsub_matching(0, 0, 8).unwrap());
    // multiplier == 9 >= 8 -> false.
    assert!(!p_arr.is_ptrsub_matching(0, 0, 9).unwrap());
    // multiplier == 7 < 8 -> true.
    assert!(p_arr.is_ptrsub_matching(0, 0, 7).unwrap());
    // wordsize scaling: multiplier 4 with wordsize 2 -> 8 >= 8 -> false.
    let mut elem2 = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    elem2.align_size = 4;
    let arr2 = make_array(Rc::new(elem2), 2, 8);
    let p_ws2: Rc<Datatype> = Rc::new(make_pointer(Rc::new(arr2), None, 2));
    assert!(!p_ws2.is_ptrsub_matching(0, 0, 4).unwrap());

    // ptr -> union: always false (cannot be used for union field resolution).
    let mut u = Datatype::new_with_align(8, -1, type_metatype::TYPE_UNION);
    u.kind = DatatypeKind::Union { field: vec![] };
    let p_u: Rc<Datatype> = Rc::new(make_pointer(Rc::new(u), None, 1));
    assert!(!p_u.is_ptrsub_matching(0, 0, 1).unwrap());
}

/// `nearestArrayedComponentBackward` (type.cc:1404-1413): the off<=size vs off>size
/// branch and the off<0 skip; forward (1395-1402) off>0 skip and off<=0 distance.
#[test]
fn w6_type1_nearest_arrayed_backward_branch() {
    let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    elem.align_size = 4;
    let arr = make_array(Rc::new(elem), 3, 12); // size 12

    // backward, off == size (12): off <= size -> size - off = 0.
    let (dist, no, el) = arr.nearest_arrayed_component_backward(12, 128).unwrap();
    assert_eq!((dist, no, el), (0, 12, 4));
    // backward, off > size (13): off > size -> off - size = 1.
    let (dist, _, _) = arr.nearest_arrayed_component_backward(13, 128).unwrap();
    assert_eq!(dist, 1);
    // backward, off < 0 -> -1.
    let (dist, _, _) = arr.nearest_arrayed_component_backward(-5, 128).unwrap();
    assert_eq!(dist, -1);
    // forward, off < 0 (negative, before head) -> distance -off = 5.
    let (dist, no, el) = arr.nearest_arrayed_component_forward(-5, 128).unwrap();
    assert_eq!((dist, no, el), (5, -5, 4));
    // forward, off > 0 -> -1 (mid-array).
    let (dist, _, _) = arr.nearest_arrayed_component_forward(3, 128).unwrap();
    assert_eq!(dist, -1);
}
