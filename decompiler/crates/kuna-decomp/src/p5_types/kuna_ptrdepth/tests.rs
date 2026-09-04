//! Tests for the inferred-pointer-nesting cap (kuna `ptrdepthcap`):
//! [`cap_pointer_depth`] is upstream `TypeFactory::getTypePointerNoDepth`'s rule
//! applied to an already-built candidate type, so the properties that matter are
//! (a) it leaves the shapes real programs have alone, (b) it collapses the
//! escalation, and (c) its output is a FIXED POINT, which is what actually makes
//! the propagation settle instead of running to the seven-pass ceiling.

use super::*;

use crate::dtype::TypeFactoryImpl;

/// A minimally-configured factory with 8-byte pointers and the core types the
/// pointer constructors need.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("undefined8", 8, type_metatype::TYPE_UNKNOWN, false).unwrap();
    f.set_core_type("char", 1, type_metatype::TYPE_INT, true).unwrap();
    f.cache_core_types().unwrap();
    f
}

fn ptr(f: &TypeFactoryImpl, to: Rc<Datatype>) -> Rc<Datatype> {
    f.get_type_pointer(8, to, 1).unwrap()
}

fn char_t(f: &TypeFactoryImpl) -> Rc<Datatype> {
    f.get_type_char(1).unwrap()
}

#[test]
fn a_non_pointer_is_never_touched() {
    let f = factory();
    let c = char_t(&f);
    assert!(cap_pointer_depth(&f, &c).is_none());
}

#[test]
fn depth_one_and_two_over_a_concrete_base_are_left_alone() {
    // `char *` and `char **` are what argv/envp and every honest
    // pointer-to-pointer parameter look like; capping them would be the
    // regression, not the fix.
    let f = factory();
    let p1 = ptr(&f, char_t(&f));
    let p2 = ptr(&f, Rc::clone(&p1));
    assert_eq!(pointer_depth(&p1, 8), 1);
    assert_eq!(pointer_depth(&p2, 8), 2);
    assert!(cap_pointer_depth(&f, &p1).is_none());
    assert!(cap_pointer_depth(&f, &p2).is_none());
}

#[test]
fn depth_three_collapses_to_one_pointer_to_an_opaque_word() {
    let f = factory();
    let p3 = ptr(&f, ptr(&f, ptr(&f, char_t(&f))));
    assert_eq!(pointer_depth(&p3, 8), 3);
    let capped = cap_pointer_depth(&f, &p3).expect("depth 3 must be capped");
    assert_eq!(pointer_depth(&capped, 8), 1);
    let target = capped.get_ptr_to().unwrap();
    assert_eq!(target.get_metatype(), type_metatype::TYPE_UNKNOWN);
    // The collapsed target is the width of the pointer level it replaced.
    assert_eq!(target.get_size(), 8);
}

#[test]
fn every_deeper_chain_collapses_to_the_same_type() {
    // The escalation walks 3, 4, 5, ... levels one pass at a time; each of them
    // must land on the identical capped type or the lattice never settles.
    let f = factory();
    let mut cur = ptr(&f, ptr(&f, ptr(&f, char_t(&f))));
    let expected = cap_pointer_depth(&f, &cur).unwrap();
    for _ in 0..5 {
        cur = ptr(&f, cur);
        let capped = cap_pointer_depth(&f, &cur).expect("a deeper chain must still be capped");
        assert!(Rc::ptr_eq(&capped, &expected) || capped.get_name() == expected.get_name());
        assert_eq!(pointer_depth(&capped, 8), 1);
    }
}

#[test]
fn the_capped_type_is_a_fixed_point() {
    // Applying the cap to its own output must change nothing -- this is the
    // property that stops `propagate_type_edge` from deepening forever.
    let f = factory();
    let p3 = ptr(&f, ptr(&f, ptr(&f, char_t(&f))));
    let once = cap_pointer_depth(&f, &p3).unwrap();
    assert!(cap_pointer_depth(&f, &once).is_none(), "the cap must be idempotent");
}

#[test]
fn a_pointer_to_the_capped_type_collapses_back_one_level() {
    // The next propagation pass builds `ptr(ptr(undefined8))` off the already
    // capped type; upstream `getTypePointerNoDepth` returns the inner pointer
    // unchanged in exactly this case, so the chain cannot grow past the cap.
    let f = factory();
    let capped = cap_pointer_depth(&f, &ptr(&f, ptr(&f, ptr(&f, char_t(&f))))).unwrap();
    let grown = ptr(&f, Rc::clone(&capped));
    assert_eq!(pointer_depth(&grown, 8), 2);
    let recapped = cap_pointer_depth(&f, &grown).expect("the regrown chain must be capped again");
    assert_eq!(pointer_depth(&recapped, 8), 1);
    assert_eq!(recapped.get_size(), capped.get_size());
    assert_eq!(
        recapped.get_ptr_to().unwrap().get_metatype(),
        type_metatype::TYPE_UNKNOWN
    );
}

#[test]
fn a_pointer_to_a_narrow_unknown_is_a_real_type_and_is_kept() {
    // `ptr(ptr(undefined1))` is NOT the escalation shape: the inner unknown is
    // one byte, not the pointer width, so upstream builds the pointer normally.
    let f = factory();
    let u1 = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let p2 = ptr(&f, ptr(&f, u1));
    assert!(cap_pointer_depth(&f, &p2).is_none());
}

#[test]
fn max_inferred_ptr_depth_matches_what_the_rule_admits() {
    let f = factory();
    let deepest_kept = ptr(&f, ptr(&f, char_t(&f)));
    assert_eq!(pointer_depth(&deepest_kept, 8), MAX_INFERRED_PTR_DEPTH);
    assert!(cap_pointer_depth(&f, &deepest_kept).is_none());
    let one_deeper = ptr(&f, deepest_kept);
    assert!(pointer_depth(&one_deeper, 8) > MAX_INFERRED_PTR_DEPTH);
    assert!(cap_pointer_depth(&f, &one_deeper).is_some());
}
