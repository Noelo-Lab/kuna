//! Unit tests for the `endptrbound` frame-hint coalescence.

use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeFactoryImpl};
use crate::varmap::{MapState, RangeType, COPY_CONSTANT, TYPELOCK};

use super::{coalesce_hints, OptionEndPtrBound, PointerWalk, ELEM_ENDPTRBOUND};

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn base(size: i32, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

fn state(space: &Rc<AddrSpace>, types: &TypeFactoryImpl) -> MapState {
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(space), 0, 0x100);
    let dflt = types.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    MapState::new(Rc::clone(space), &rn, &RangeList::new(), dflt)
}

fn walk(start: i64, end: i64, step: i64) -> PointerWalk {
    PointerWalk { cmp: OpId::default(), slot: 1, start, end, step }
}

fn layout(state: &mut MapState) -> Vec<(i64, i32, RangeType)> {
    let mut v: Vec<_> = state.hints_mut().iter().map(|h| (h.sstart, h.size, h.range_type)).collect();
    v.sort();
    v
}

#[test]
fn constant_initializers_and_pointer_hints_become_one_buffer() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    let char1 = base(1, type_metatype::TYPE_INT);
    st.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x24, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x20, Some(Rc::clone(&char1)), 0, RangeType::Open, -1);
    st.add_range_pub(0x28, Some(base(8, type_metatype::TYPE_UINT)), 0, RangeType::Fixed, -1);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 8, RangeType::Fixed), (0x28, 8, RangeType::Fixed)]);
    let buf = st.hints_mut().iter().find(|h| h.sstart == 0x20).unwrap().type_.clone();
    assert!(Rc::ptr_eq(&buf, &char1), "the element type comes from the walk's own byte hint");
}

#[test]
fn a_store_straddling_the_end_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(4, type_metatype::TYPE_UNKNOWN)), COPY_CONSTANT, RangeType::Fixed, -1);
    st.add_range_pub(0x24, Some(base(8, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Fixed, -1);
    let before = layout(&mut st);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), before);
}

#[test]
fn a_typed_value_or_locked_symbol_inside_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    for (flags, meta) in [(0, type_metatype::TYPE_INT), (TYPELOCK, type_metatype::TYPE_UNKNOWN)] {
        let mut st = state(&space, &types);
        st.add_range_pub(0x24, Some(base(4, meta)), flags, RangeType::Fixed, -1);
        let before = layout(&mut st);
        coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
        assert_eq!(layout(&mut st), before);
    }
}

#[test]
fn index_evidence_past_the_end_vetoes_the_buffer() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    st.add_range_pub(0x20, Some(base(1, type_metatype::TYPE_UNKNOWN)), 0, RangeType::Open, 15);
    let before = layout(&mut st);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1)], &space, &types);
    assert_eq!(layout(&mut st), before);
}

#[test]
fn the_widest_walk_from_a_start_wins_and_the_window_bounds_it() {
    let (space, types) = (stack_space(), factory());
    let mut st = state(&space, &types);
    coalesce_hints(&mut st, &[walk(0x20, 0x28, 1), walk(0x20, 0x30, 1), walk(0xf8, 0x108, 4)], &space, &types);
    assert_eq!(layout(&mut st), vec![(0x20, 16, RangeType::Fixed)]);
}

#[test]
fn option_identity() {
    assert_eq!(ELEM_ENDPTRBOUND.get_id(), 4166);
    assert_eq!(OptionEndPtrBound::NAME, "endptrbound");
    assert!(OptionEndPtrBound.apply("on").unwrap().0);
    assert!(!OptionEndPtrBound.apply("off").unwrap().0);
    assert!(OptionEndPtrBound.apply("maybe").is_err());
}
