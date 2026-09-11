//! Unit tests for the `declhightype` type-representative ordering.
//!
//! The shapes are the witness's: one merged RAX carrying a `char *` from a call
//! return, an `unsigned long long *` from a pointer computation, and an integer.

use super::*;
use crate::dtype::{type_metatype, DatatypeKind};
use slotmap::KeyData;

fn vid(n: u64) -> VarnodeId {
    VarnodeId::from(KeyData::from_ffi(n))
}

fn ty(size: i32, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, m))
}

/// `types->getTypePointer(8, elem, 1)` — an 8-byte pointer.
fn ptr_to(elem: Rc<Datatype>) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    t.kind = DatatypeKind::Pointer { ptrto: elem, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(t)
}

fn member(n: u64, type_lock: bool, ty: Rc<Datatype>) -> Option<Member> {
    Some(Member { vn: vid(n), type_lock, ty })
}

/// No members at all: no representative, so the caller keeps its own type.
#[test]
fn empty_instance_list_has_no_representative() {
    assert_eq!(pick_representative(std::iter::empty()), None);
}

/// A lone member is the representative whatever its type.
#[test]
fn lone_member_is_the_representative() {
    let m = vec![member(3, false, ty(8, type_metatype::TYPE_UINT))];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(3)));
}

/// A pointer is more specialized than an integer of the same width, so the
/// merged RAX of the witness (a `char *` from the call, an integer from the
/// state-machine write-back) declares as the pointer.
#[test]
fn a_pointer_beats_an_integer_of_the_same_width() {
    let m = vec![
        member(1, false, ty(8, type_metatype::TYPE_UINT)),
        member(2, false, ptr_to(ty(1, type_metatype::TYPE_INT))),
    ];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(2)));
    // and the order of the members does not decide it
    let m = vec![
        member(2, false, ptr_to(ty(1, type_metatype::TYPE_INT))),
        member(1, false, ty(8, type_metatype::TYPE_UINT)),
    ];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(2)));
}

/// A type-locked member outranks an unlocked one that would otherwise win on
/// specialization (C++ `getTypeRepresentative`'s first arm).
#[test]
fn a_type_locked_member_outranks_a_more_specialized_unlocked_one() {
    let m = vec![
        member(1, true, ty(8, type_metatype::TYPE_UINT)),
        member(2, false, ptr_to(ty(1, type_metatype::TYPE_INT))),
    ];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(1)));
}

/// Two members that tie keep the earlier one — member zero is the seed, so the
/// ordering never reshuffles a high whose members agree.
#[test]
fn a_tie_keeps_the_earlier_member() {
    let t = ty(4, type_metatype::TYPE_INT);
    let m = vec![member(7, false, Rc::clone(&t)), member(9, false, t)];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(7)));
}

/// An unresolvable member is skipped, not treated as a candidate.
#[test]
fn an_unresolvable_member_is_skipped() {
    let m = vec![None, member(5, false, ty(4, type_metatype::TYPE_INT)), None];
    assert_eq!(pick_representative(m.into_iter()), Some(vid(5)));
}

/// The option parses `on`/`off` and refuses anything else.
#[test]
fn the_option_parses_on_and_off_only() {
    assert_eq!(OptionDeclHighType.apply("on").unwrap().0, true);
    assert_eq!(OptionDeclHighType.apply("off").unwrap().0, false);
    assert!(OptionDeclHighType.apply("maybe").is_err());
}
