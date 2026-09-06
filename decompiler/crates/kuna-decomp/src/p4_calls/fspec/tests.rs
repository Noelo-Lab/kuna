//! Tests for the parameter-recovery foundation (`fspec.rs`, item
//! `w6-s4-fspec-1`): ParamEntry containment matrices, ParamTrial sorting
//! parity, and ParamListStandard assignment/fillin walks for synthetic
//! prototypes.
//!
//! Address spaces are built directly with `AddrSpace::new` (no manager wiring
//! needed for the non-float-extension paths), and parameter entries with the
//! `ParamEntry::seed` builder hook that runs the real post-decode resolution
//! chain.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::dtype::{type_class, type_metatype, Datatype, TypeFactory};

use super::*;

/// A little-endian "register" space at index 3.
fn reg_space_le() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false, // little endian
        4,
        1,
        3,
        0,
        0,
        0,
    ))
}

/// A big-endian "register" space at index 4.
fn reg_space_be() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "registerBE",
        true, // big endian
        4,
        1,
        4,
        0,
        0,
        0,
    ))
}

/// A little-endian stack (spacebase) space at index 5.
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        4,
        1,
        5,
        0,
        0,
        0,
    ))
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

/// Build an exclusion ParamEntry (alignment 0) at the given offset/size.
fn excl_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        size,
        1, // minsize
        0, // alignment == size signals exclusion via the seed adjust below; pass 0 here
        0, // flags
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed exclusion entry")
}

/// Build a stack resource ParamEntry with the given alignment.
fn stack_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    align: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        size,
        1,
        align,
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed stack entry")
}

// =========================================================================
// ParamEntry containment matrices
// =========================================================================

#[test]
fn exclusion_entry_basic_properties() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    assert!(e.is_exclusion());
    assert!(!e.is_reverse_stack());
    assert_eq!(e.get_group(), 0);
    assert_eq!(e.get_size(), 4);
    assert_eq!(e.get_base(), 0x10);
    // First entry in its (only) storage class.
    assert!(e.is_first_in_class());
}

#[test]
fn contained_by_matrix() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);

    // Exactly the entry's range -> contained.
    assert!(e.contained_by(&addr(&reg, 0x10), 4));
    // A larger covering range -> contained.
    assert!(e.contained_by(&addr(&reg, 0x10), 8));
    assert!(e.contained_by(&addr(&reg, 0x0c), 8)); // 0x0c..0x14 covers 0x10..0x14
    // A smaller range -> NOT contained (entry extends past it).
    assert!(!e.contained_by(&addr(&reg, 0x10), 2));
    // A range starting after the entry base -> NOT contained.
    assert!(!e.contained_by(&addr(&reg, 0x11), 4));
    // Different space -> NOT contained.
    let other = reg_space_be();
    assert!(!e.contained_by(&addr(&other, 0x10), 4));
}

#[test]
fn justified_contain_little_endian() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    // LE: least significant byte is at the lowest address (the base).
    // A range covering the LS bytes (offset 0x10) is justified -> 0.
    assert_eq!(e.justified_contain(&addr(&reg, 0x10), 2), 0);
    // A range higher up is contained but not justified -> 2.
    assert_eq!(e.justified_contain(&addr(&reg, 0x12), 2), 2);
    // A range not contained -> -1.
    assert_eq!(e.justified_contain(&addr(&reg, 0x20), 2), -1);
}

#[test]
fn justified_contain_big_endian() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_be();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    // BE: least significant byte is at the highest address.
    // A 2-byte range at the top of the entry (offset 0x12) is justified -> 0.
    assert_eq!(e.justified_contain(&addr(&reg, 0x12), 2), 0);
    // A 2-byte range at the base (offset 0x10) is the most-significant -> 2.
    assert_eq!(e.justified_contain(&addr(&reg, 0x10), 2), 2);
    // The full range is justified -> 0.
    assert_eq!(e.justified_contain(&addr(&reg, 0x10), 4), 0);
}

#[test]
fn get_container_exclusion_passes_back_whole_entry() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    let mut res = VarnodeData::default();
    assert!(e.get_container(&addr(&reg, 0x10), 2, &mut res));
    assert_eq!(res.offset, 0x10);
    assert_eq!(res.size, 4);
    assert!(Rc::ptr_eq(res.space.as_ref().unwrap(), &reg));
}

#[test]
fn assumed_extension_zext_le() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    // An exclusion entry that zero-extends small values.
    let e = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&reg),
        0x10,
        4,
        1,
        0,
        param_entry_flags::SMALLSIZE_ZEXT,
        true,
        false,
        &[],
        &mgr,
    )
    .unwrap();
    let mut res = VarnodeData::default();
    // A 2-byte justified value (LE, at base) gets zero-extended to the whole 4.
    assert_eq!(e.assumed_extension(&addr(&reg, 0x10), 2, &mut res), OpCode::CPUI_INT_ZEXT);
    assert_eq!(res.offset, 0x10);
    assert_eq!(res.size, 4);
    // A full-size value needs no extension.
    let mut res2 = VarnodeData::default();
    assert_eq!(e.assumed_extension(&addr(&reg, 0x10), 4, &mut res2), OpCode::CPUI_COPY);
    // A non-justified value cannot be extended.
    let mut res3 = VarnodeData::default();
    assert_eq!(e.assumed_extension(&addr(&reg, 0x12), 2, &mut res3), OpCode::CPUI_COPY);
}

#[test]
fn stack_entry_slots_and_addrs() {
    let mgr = AddrSpaceManager::new();
    let stk = stack_space();
    // A stack resource: 0x00..0x20, alignment 4 -> 8 slots, groups start at 7.
    let e = stack_entry(7, &stk, 0x0, 0x20, 4, &[], &mgr);
    assert!(!e.is_exclusion());
    assert_eq!(e.get_align(), 4);
    // getSlot: byte 0 is slot group 7, byte 4 is group 8, etc.
    assert_eq!(e.get_slot(&addr(&stk, 0x0), 0), 7);
    assert_eq!(e.get_slot(&addr(&stk, 0x4), 0), 8);
    assert_eq!(e.get_slot(&addr(&stk, 0x8), 0), 9);
    // getAddrBySlot: a 4-byte param from slot 0 lands at offset 0, consuming 1 slot.
    let mut slot = 0;
    let a = e.get_addr_by_slot(&mut slot, 4, 1, &mgr).unwrap();
    assert_eq!(a.get_offset(), 0x0);
    assert_eq!(slot, 1);
    // The next 4-byte param lands at offset 4.
    let a2 = e.get_addr_by_slot(&mut slot, 4, 1, &mgr).unwrap();
    assert_eq!(a2.get_offset(), 0x4);
    assert_eq!(slot, 2);
    // An 8-byte param consumes 2 slots.
    let mut slot2 = 0;
    let a3 = e.get_addr_by_slot(&mut slot2, 8, 1, &mgr).unwrap();
    assert_eq!(a3.get_offset(), 0x0);
    assert_eq!(slot2, 2);
}

#[test]
fn get_addr_by_slot_rejects_too_small() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    // minsize 4 entry: a 2-byte request returns invalid.
    let e = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&reg),
        0x10,
        4,
        4, // minsize 4
        0,
        0,
        true,
        false,
        &[],
        &mgr,
    )
    .unwrap();
    let mut slot = 0;
    let a = e.get_addr_by_slot(&mut slot, 2, 1, &mgr).unwrap();
    assert!(a.is_invalid());
}

#[test]
fn intersects_and_contains() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let big = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    let small = excl_entry(1, &reg, 0x10, 2, &[], &mgr);
    assert!(big.intersects(&addr(&reg, 0x10), 4));
    assert!(big.contains(&small));
    assert!(!small.contains(&big));
    // Non-overlapping ranges do not intersect.
    assert!(!big.intersects(&addr(&reg, 0x20), 4));
}

#[test]
fn group_overlap_via_resolve_overlap() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    // First a 2-byte entry, then a 4-byte entry that contains it: resolveOverlap
    // on the later (containing) entry reassigns its group set to the contained
    // earlier entry's group and marks it overlapping.  (A later entry that does
    // NOT contain the earlier one is an illegal overlap in C++.)
    let mut list: Vec<ParamEntry> = Vec::new();
    let e0 = excl_entry(0, &reg, 0x10, 2, &list, &mgr);
    list.push(e0);
    let e1 = excl_entry(1, &reg, 0x10, 4, &list, &mgr);
    // The overlapping entry inherits group 0 and is marked overlapping.
    assert!(e1.is_overlap());
    assert_eq!(e1.get_group(), 0);
    assert!(list[0].group_overlap(&e1));
}

// =========================================================================
// ParamTrial sorting parity
// =========================================================================

/// Build a 2-entry exclusion model (groups 0 and 1) for sorting tests.
fn two_excl_model() -> (ParamListStandard, Rc<AddrSpace>) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    model.push_entry(e0);
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();
    (model, reg)
}

#[test]
fn trial_sort_by_group_order() {
    let (model, reg) = two_excl_model();
    let entries = model.get_entry();
    // Two trials: one in group 1's entry (idx 1), one in group 0's (idx 0).
    let mut t_hi = ParamTrial::new(addr(&reg, 0x20), 4, 2);
    t_hi.set_entry(Some(1), 0);
    let mut t_lo = ParamTrial::new(addr(&reg, 0x10), 4, 1);
    t_lo.set_entry(Some(0), 0);
    // t_lo (group 0) must sort before t_hi (group 1).
    assert_eq!(t_lo.cmp(&t_hi, entries), std::cmp::Ordering::Less);
    assert_eq!(t_hi.cmp(&t_lo, entries), std::cmp::Ordering::Greater);
}

#[test]
fn trial_null_entry_sorts_last() {
    let (model, reg) = two_excl_model();
    let entries = model.get_entry();
    let mut t_real = ParamTrial::new(addr(&reg, 0x10), 4, 1);
    t_real.set_entry(Some(0), 0);
    let t_null = ParamTrial::new(addr(&reg, 0x30), 4, 2); // entry None
    // A trial with an entry sorts before a trial without one.
    assert_eq!(t_real.cmp(&t_null, entries), std::cmp::Ordering::Less);
    assert_eq!(t_null.cmp(&t_real, entries), std::cmp::Ordering::Greater);
}

#[test]
fn fixed_position_compare_orders_fixed_first() {
    let (model, reg) = two_excl_model();
    let entries = model.get_entry();
    let mut a = ParamTrial::new(addr(&reg, 0x10), 4, 1);
    a.set_entry(Some(0), 0);
    a.set_fixed_position(1);
    let mut b = ParamTrial::new(addr(&reg, 0x20), 4, 2);
    b.set_entry(Some(1), 0);
    // b has no fixed position (-1) -> a (fixed) comes first.
    assert_eq!(
        ParamTrial::fixed_position_compare(&a, &b, entries),
        std::cmp::Ordering::Less
    );
    // Both fixed: order by fixed position.
    b.set_fixed_position(0);
    assert_eq!(
        ParamTrial::fixed_position_compare(&a, &b, entries),
        std::cmp::Ordering::Greater
    ); // a.fixed=1 > b.fixed=0
}

#[test]
fn param_active_sort_reorders_into_group_order() {
    let (model, reg) = two_excl_model();
    let mut active = ParamActive::new(false);
    // Register trials out of group order: group 1 first, then group 0.
    active.register_trial(&addr(&reg, 0x20), 4); // slot 1
    active.register_trial(&addr(&reg, 0x10), 4); // slot 2
    active.get_trial_mut(0).set_entry(Some(1), 0);
    active.get_trial_mut(1).set_entry(Some(0), 0);
    active.sort_trials(model.get_entry());
    // After sorting, group 0's trial (offset 0x10) is first.
    assert_eq!(active.get_trial(0).get_address().get_offset(), 0x10);
    assert_eq!(active.get_trial(1).get_address().get_offset(), 0x20);
}

// =========================================================================
// ParamActive trial bookkeeping
// =========================================================================

#[test]
fn register_trial_marks_killed_by_call_for_registers() {
    let reg = reg_space_le();
    let stk = stack_space();
    let mut active = ParamActive::new(true);
    active.register_trial(&addr(&reg, 0x10), 4); // register -> killed by call
    active.register_trial(&addr(&stk, 0x0), 4); // stack -> not killed
    assert!(active.get_trial(0).is_killed_by_call());
    assert!(!active.get_trial(1).is_killed_by_call());
    assert_eq!(active.get_num_trials(), 2);
    // Slots are assigned starting at 1.
    assert_eq!(active.get_trial(0).get_slot(), 1);
    assert_eq!(active.get_trial(1).get_slot(), 2);
}

#[test]
fn split_and_join_trial() {
    let reg = reg_space_le();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 8); // one 8-byte trial at slot 1
    active.split_trial(0, 4).unwrap();
    assert_eq!(active.get_num_trials(), 2);
    assert_eq!(active.get_trial(0).get_size(), 4);
    assert_eq!(active.get_trial(0).get_address().get_offset(), 0x10);
    assert_eq!(active.get_trial(1).get_size(), 4);
    assert_eq!(active.get_trial(1).get_address().get_offset(), 0x14); // LE split lo
    // Now join them back.
    active.join_trial(1, &addr(&reg, 0x10), 8).unwrap();
    assert_eq!(active.get_num_trials(), 1);
    assert_eq!(active.get_trial(0).get_size(), 8);
    assert!(active.get_trial(0).is_used());
}

#[test]
fn delete_unused_trials_reorders_slots() {
    let reg = reg_space_le();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4);
    active.register_trial(&addr(&reg, 0x20), 4);
    active.register_trial(&addr(&reg, 0x30), 4);
    active.get_trial_mut(0).mark_used();
    active.get_trial_mut(2).mark_used();
    active.delete_unused_trials();
    assert_eq!(active.get_num_trials(), 2);
    assert_eq!(active.get_trial(0).get_slot(), 1);
    assert_eq!(active.get_trial(1).get_slot(), 2);
    assert_eq!(active.get_trial(1).get_address().get_offset(), 0x30);
}

#[test]
fn which_trial_finds_overlap() {
    let reg = reg_space_le();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4);
    active.register_trial(&addr(&reg, 0x20), 4);
    // A direct overlap with the first trial is found immediately.
    assert_eq!(active.which_trial(&addr(&reg, 0x12), 1), 0);
    // For sz > 1, the endpoint scan reaches the second trial.  (With sz <= 1
    // the C++ loop returns after probing only the first trial.)
    assert_eq!(active.which_trial(&addr(&reg, 0x20), 4), 1);
    assert_eq!(active.which_trial(&addr(&reg, 0x40), 4), -1);
}

// =========================================================================
// ParamListStandard find / characterize / fillin walks
// =========================================================================

/// A 3-entry register model: groups 0,1,2 at 0x10,0x20,0x30 (4 bytes each).
fn three_reg_model() -> (ParamListStandard, Rc<AddrSpace>) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    model.push_entry(e0);
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    let e2 = excl_entry(2, &reg, 0x30, 4, model.get_entry(), &mgr);
    model.push_entry(e2);
    model.finish_decode();
    (model, reg)
}

#[test]
fn find_entry_resolves_by_offset() {
    let (model, reg) = three_reg_model();
    assert_eq!(model.find_entry(&addr(&reg, 0x10), 4, true), Some(0));
    assert_eq!(model.find_entry(&addr(&reg, 0x20), 4, true), Some(1));
    assert_eq!(model.find_entry(&addr(&reg, 0x30), 4, true), Some(2));
    // An unmapped offset has no entry.
    assert_eq!(model.find_entry(&addr(&reg, 0x40), 4, true), None);
    // A justified 2-byte sub-range of group 0 still resolves to entry 0.
    assert_eq!(model.find_entry(&addr(&reg, 0x10), 2, true), Some(0));
    // A non-justified sub-range fails the `just` check.
    assert_eq!(model.find_entry(&addr(&reg, 0x12), 2, true), None);
    // ...but matches when justification is not enforced.
    assert_eq!(model.find_entry(&addr(&reg, 0x12), 2, false), Some(0));
}

#[test]
fn characterize_as_param_codes() {
    let (model, reg) = three_reg_model();
    // Exactly an entry, justified.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x10), 4),
        Containment::ContainsJustified
    );
    // A justified sub-range.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x10), 2),
        Containment::ContainsJustified
    );
    // A contained-but-unjustified sub-range.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x12), 2),
        Containment::ContainsUnjustified
    );
    // A range covering an entry (contained_by).
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x10), 8),
        Containment::ContainedBy
    );
    // No overlap at all.
    assert_eq!(
        model.characterize_as_param(&addr(&reg, 0x40), 4),
        Containment::NoContainment
    );
}

#[test]
fn possible_param_and_with_slot() {
    let (model, reg) = three_reg_model();
    assert!(model.possible_param(&addr(&reg, 0x20), 4));
    assert!(!model.possible_param(&addr(&reg, 0x40), 4));
    let mut slot = 0;
    let mut slotsize = 0;
    assert!(model.possible_param_with_slot(&addr(&reg, 0x20), 4, &mut slot, &mut slotsize));
    assert_eq!(slot, 1); // group of entry at 0x20
    assert_eq!(slotsize, 1); // exclusion entry, one group
}

#[test]
fn biggest_contained_param() {
    let (model, reg) = three_reg_model();
    let mut res = VarnodeData::default();
    // A range covering group 0's 4-byte entry -> passes it back.
    assert!(model.get_biggest_contained_param(&addr(&reg, 0x10), 8, &mut res));
    assert_eq!(res.offset, 0x10);
    assert_eq!(res.size, 4);
    // A range over an unmapped area -> false.
    assert!(!model.get_biggest_contained_param(&addr(&reg, 0x40), 8, &mut res));
}

#[test]
fn fillin_map_marks_active_used() {
    let mgr = AddrSpaceManager::new();
    let (model, reg) = three_reg_model();
    let mut active = ParamActive::new(false);
    // Two active trials matching groups 0 and 1.
    active.register_trial(&addr(&reg, 0x10), 4);
    active.register_trial(&addr(&reg, 0x20), 4);
    active.get_trial_mut(0).mark_active();
    active.get_trial_mut(1).mark_active();
    model.fillin_map(&mut active, &mgr).unwrap();
    // Both should be marked used (active in consecutive groups, no holes).
    let used = (0..active.get_num_trials())
        .filter(|&i| active.get_trial(i).is_used())
        .count();
    assert!(used >= 2);
}

#[test]
fn fillin_map_register_kind() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Register);
    let e0 = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    model.push_entry(e0);
    let e1 = excl_entry(1, &reg, 0x20, 4, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();

    let mut active = ParamActive::new(false);
    // Register model: any subset can be used, even with a "hole".
    active.register_trial(&addr(&reg, 0x20), 4); // only group 1 active
    active.get_trial_mut(0).mark_active();
    model.fillin_map(&mut active, &mgr).unwrap();
    // The single active trial in group 1 is marked used.
    let trial = active.get_trial(0);
    assert!(trial.is_used());
    assert_eq!(trial.get_entry(), Some(1));
}

// =========================================================================
// ParamListStandard assignMap (assignment walk for synthetic prototypes)
// =========================================================================

/// A `TypeFactory` stub that panics on any reach — the input-list `assignMap`
/// path with no model rules never calls it.
struct PanicTypeFactory;

macro_rules! unreached {
    () => {
        panic!("TypeFactory should not be reached in this test")
    };
}

#[allow(unused_variables)]
impl TypeFactory for PanicTypeFactory {
    fn get_size_of_int(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_long(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_char(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_wchar(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_pointer(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_alt_pointer(&self) -> int4 {
        unreached!()
    }
    fn get_alignment(&self, _size: u32) -> KunaResult<int4> {
        unreached!()
    }
    fn get_primitive_align_size(&self, _size: u32) -> KunaResult<int4> {
        unreached!()
    }
    fn get_type_void(&self) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_base_no_char(&self, _s: int4, _m: type_metatype) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_base(&self, _s: int4, _m: type_metatype) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_base_named(
        &self,
        _s: int4,
        _m: type_metatype,
        _n: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_char(&self, _s: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_code(&self) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_strip_array(
        &self,
        _s: int4,
        _pt: Rc<Datatype>,
        _ws: u32,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer(&self, _s: int4, _pt: Rc<Datatype>, _ws: u32) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_named(
        &self,
        _s: int4,
        _pt: Rc<Datatype>,
        _ws: u32,
        _n: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn resize_pointer(&self, _ptr: Rc<Datatype>, _new_size: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_rel(
        &self,
        _parent_ptr: Rc<Datatype>,
        _ptr_to: Rc<Datatype>,
        _off: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_rel_full(
        &self,
        _sz: int4,
        _parent: Rc<Datatype>,
        _ptr_to: Rc<Datatype>,
        _ws: int4,
        _off: int4,
        _nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_with_space(
        &self,
        _ptr_to: Rc<Datatype>,
        _spc: Rc<AddrSpace>,
        _nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_array(&self, _as_: int4, _ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_struct(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_struct(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_union(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_union(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_enum(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_enum(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_spacebase(&self, _id: Rc<AddrSpace>, _addr: &Address) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn resize_integer(&self, _ct: Rc<Datatype>, _new_size: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_exact_piece(
        &self,
        _ct: Rc<Datatype>,
        _offset: int4,
        _size: int4,
    ) -> KunaResult<Option<Rc<Datatype>>> {
        unreached!()
    }
    fn find_by_name(&self, _n: &str) -> KunaResult<Option<Rc<Datatype>>> {
        unreached!()
    }
    fn concretize(&self, _ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
}

/// A 4-byte general-purpose integer data-type.
fn int4_type() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_INT))
}

#[test]
fn assign_map_standard_input_walk() {
    let mgr = AddrSpaceManager::new();
    let (model, _reg) = three_reg_model();
    let tf = PanicTypeFactory;
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![int4_type(), int4_type()],
        innames: vec!["a".into(), "b".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 2);
    // First int -> group 0 (offset 0x10), second -> group 1 (offset 0x20).
    assert_eq!(res[0].addr.get_offset(), 0x10);
    assert_eq!(res[1].addr.get_offset(), 0x20);
    assert!(res[0].type_.is_some());
}

/// A `register` space (addr_size 8) usable as the default data space, plus the
/// constant/unique spaces a `TypeFactoryImpl`-backed pointer build needs.
fn manager_with_default_data_space() -> (AddrSpaceManager, Rc<AddrSpace>) {
    use kuna_base::space::{ConstantSpace, UniqueSpace};
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false, // little endian
        8,     // addr_size — the hidden-return pointer size
        1,     // word_size
        2,     // index
        0,
        0,
        0,
    ));
    m.insert_space(Rc::clone(&reg)).unwrap();
    m.set_default_code_space(2).unwrap();
    m.set_default_data_space(2).unwrap();
    (m, reg)
}

/// A `ParamListStandardOut` whose lone 8-byte register entry cannot hold a
/// 24-byte return value (but CAN hold the 8-byte hidden pointer), so `assignMap`
/// falls through to the hidden-return path.
fn single_small_out_model(reg: &Rc<AddrSpace>, mgr: &AddrSpaceManager) -> ParamListStandard {
    let mut model = ParamListStandard::new(ParamListKind::StandardOut);
    let e0 = excl_entry(0, reg, 0x10, 8, &[], mgr);
    model.push_entry(e0);
    model.finish_decode();
    model
}

#[test]
fn assign_map_standard_out_hidden_return_emits_indirect_output_plus_pointer_param() {
    // w10-struct-return de-stub: a too-big (8-byte) return value that does not
    // fit the model's 4-byte output register is returned through a hidden
    // pointer parameter (C++ `ParamListStandardOut::assignMap` fspec.cc:1584+).
    use crate::dtype::TypeFactoryImpl;
    let (mgr, reg) = manager_with_default_data_space();
    let model = single_small_out_model(&reg, &mgr);
    let tf = TypeFactoryImpl::new();
    tf.setup_sizes(Some(4), 8, 8); // initialize the size/alignment map
    // A 24-byte struct: too big for the 8-byte output register, so it must be
    // returned through a hidden pointer (which DOES fit the register).
    let outtype = Rc::new(Datatype::new_with_align(24, 8, type_metatype::TYPE_STRUCT));
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: Some(outtype),
        intypes: Vec::new(),
        innames: Vec::new(),
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    // Two pieces: the indirect-storage output, then the hidden pointer param.
    assert_eq!(res.len(), 2);
    assert_ne!(res[0].flags & parameter_pieces_flags::INDIRECTSTORAGE, 0);
    // The output's recovered type is the pointer to the struct (size = addr_size).
    assert_eq!(res[0].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_PTR);
    assert_eq!(res[0].type_.as_ref().unwrap().get_size(), 8);
    // The hidden return parameter carries the pointer type and an invalid
    // address (filled in by the input-list assignMap), and — with no model
    // rules — is the plain (non-special) hidden return (flags 0).
    assert_eq!(res[1].type_.as_ref().unwrap().get_metatype(), type_metatype::TYPE_PTR);
    assert!(res[1].addr.is_invalid());
    assert_eq!(res[1].flags, 0);
}

#[test]
fn assign_address_fallback_exhausts_groups() {
    let mgr = AddrSpaceManager::new();
    let (model, _reg) = three_reg_model();
    let tp = int4_type();
    let mut status = vec![0i32; 3];
    let mut p0 = ParameterPieces::default();
    // First assignment grabs group 0.
    let r0 = model
        .assign_address_fallback(type_class::TYPECLASS_GENERAL, &tp, false, &mut status, &mut p0, &mgr)
        .unwrap();
    assert_eq!(r0, AssignActionResponse::success);
    assert_eq!(p0.addr.get_offset(), 0x10);
    assert_eq!(status[0], -1); // group 0 consumed
    // Second assignment grabs group 1.
    let mut p1 = ParameterPieces::default();
    let r1 = model
        .assign_address_fallback(type_class::TYPECLASS_GENERAL, &tp, false, &mut status, &mut p1, &mgr)
        .unwrap();
    assert_eq!(r1, AssignActionResponse::success);
    assert_eq!(p1.addr.get_offset(), 0x20);
}

// =========================================================================
// ParamListMerged fold-in
// =========================================================================

#[test]
fn merged_fold_in_dedups() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut a = ParamListStandard::new(ParamListKind::Standard);
    a.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
    a.finish_decode();

    let mut b = ParamListStandard::new(ParamListKind::Standard);
    b.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr)); // same as a's entry
    b.push_entry(excl_entry(1, &reg, 0x20, 4, b.get_entry(), &mgr)); // new
    b.finish_decode();

    let mut merged = ParamListStandard::new(ParamListKind::Merged);
    merged.fold_in(&a).unwrap();
    merged.fold_in(&b).unwrap();
    merged.finalize();
    // The duplicate 0x10 entry is folded; 0x20 is added -> 2 entries total.
    assert_eq!(merged.get_entry().len(), 2);
}

// =========================================================================
// EffectRecord
// =========================================================================

#[test]
fn effect_record_from_param_entry() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let e = excl_entry(0, &reg, 0x10, 4, &[], &mgr);
    let er = EffectRecord::from_param_entry(&e, effect_type::KILLEDBYCALL);
    assert_eq!(er.get_type(), effect_type::KILLEDBYCALL);
    assert_eq!(er.get_address().get_offset(), 0x10);
    assert_eq!(er.get_size(), 4);
    // Equality.
    let er2 = EffectRecord::from_param_entry(&e, effect_type::KILLEDBYCALL);
    assert_eq!(er, er2);
    let er3 = EffectRecord::from_param_entry(&e, effect_type::UNAFFECTED);
    assert_ne!(er, er3);
}

// =========================================================================
// fspec-2: ProtoModel / ScoreProtoModel / ProtoModelMerged
// =========================================================================

fn vdata(spc: &Rc<AddrSpace>, off: u64, sz: u32) -> VarnodeData {
    VarnodeData { space: Some(Rc::clone(spc)), offset: off, size: sz }
}

/// Build a standard ProtoModel with two int register entries (groups 0/1 at
/// 0x10/0x20) for input and one int register at 0x10 for output.
fn three_reg_proto_model(mgr: &AddrSpaceManager, reg: &Rc<AddrSpace>) -> ProtoModel {
    let mut model = ProtoModel::new(mgr);
    model.build_param_list("standard").unwrap();
    model.set_name("__cdecl");
    {
        let input = model.input_mut();
        input.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
        let e1 = excl_entry(1, reg, 0x20, 4, input.get_entry(), mgr);
        input.push_entry(e1);
        input.finish_decode();
    }
    {
        let output = model.output_mut();
        output.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
        output.finish_decode();
    }
    model
}

#[test]
fn proto_model_build_param_list_strategies() {
    let mgr = AddrSpaceManager::new();
    let mut model = ProtoModel::new(&mgr);
    model.build_param_list("standard").unwrap();
    assert_eq!(model.input().get_type(), ParamListType::Standard);
    assert_eq!(model.output().get_type(), ParamListType::StandardOut);
    let mut model2 = ProtoModel::new(&mgr);
    model2.build_param_list("register").unwrap();
    assert_eq!(model2.input().get_type(), ParamListType::Register);
    assert_eq!(model2.output().get_type(), ParamListType::RegisterOut);
    let mut model3 = ProtoModel::new(&mgr);
    assert!(model3.build_param_list("nonsense").is_err());
}

#[test]
fn proto_model_thiscall_name_forces_has_this() {
    let mgr = AddrSpaceManager::new();
    let mut model = ProtoModel::new(&mgr);
    assert!(!model.has_this_pointer());
    model.set_name("__thiscall");
    assert!(model.has_this_pointer());
}

#[test]
fn proto_model_copy_named_is_compatible() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let base = three_reg_proto_model(&mgr, &reg);
    let copy = ProtoModel::copy_named("__stdcall", &base);
    // The named copy is compatible with its parent (in both directions).
    assert!(copy.is_compatible(&base));
    assert!(base.is_compatible(&copy));
    assert_eq!(copy.get_name(), "__stdcall");
    // A model is compatible with itself.
    assert!(base.is_compatible(&base));
    // Two unrelated models are not compatible.
    let other = three_reg_proto_model(&mgr, &reg);
    assert!(!base.is_compatible(&other));
}

#[test]
fn proto_model_unknown_flag() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let base = three_reg_proto_model(&mgr, &reg);
    assert!(!base.is_unknown());
    let unk = ProtoModel::new_unknown("weird", &base);
    assert!(unk.is_unknown());
    assert_eq!(unk.get_name(), "weird");
    assert!(unk.is_compatible(&base));
}

#[test]
fn proto_model_lookup_effect_and_record() {
    let reg = reg_space_le();
    // Two effect records, sorted by address.
    let e_low = EffectRecord::from_varnode(vdata(&reg, 0x10, 4), effect_type::UNAFFECTED);
    let e_high = EffectRecord::from_varnode(vdata(&reg, 0x20, 4), effect_type::KILLEDBYCALL);
    let mut efflist = vec![e_high, e_low];
    efflist.sort_by(EffectRecord::compare_by_address);

    // Exact hit on the low record.
    assert_eq!(
        ProtoModel::lookup_effect(&efflist, &addr(&reg, 0x10), 4),
        effect_type::UNAFFECTED
    );
    // Exact hit on the high record.
    assert_eq!(
        ProtoModel::lookup_effect(&efflist, &addr(&reg, 0x20), 4),
        effect_type::KILLEDBYCALL
    );
    // A range below the first record -> unknown.
    assert_eq!(
        ProtoModel::lookup_effect(&efflist, &addr(&reg, 0x00), 4),
        effect_type::UNKNOWN_EFFECT
    );

    // lookupRecord: exact-match index, and overlap classification.
    assert_eq!(
        ProtoModel::lookup_record(&efflist, efflist.len() as i32, &addr(&reg, 0x10), 4),
        0
    );
    assert_eq!(
        ProtoModel::lookup_record(&efflist, efflist.len() as i32, &addr(&reg, 0x20), 4),
        1
    );
    // No overlap below the first record -> -1.
    assert_eq!(
        ProtoModel::lookup_record(&efflist, efflist.len() as i32, &addr(&reg, 0x00), 4),
        -1
    );
    // Partial overlap with the low record (offset 0x11 within 0x10..0x14) -> -2.
    assert_eq!(
        ProtoModel::lookup_record(&efflist, efflist.len() as i32, &addr(&reg, 0x12), 4),
        -2
    );
}

#[test]
fn proto_model_has_effect_internal_space_is_unaffected() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = three_reg_proto_model(&mgr, &reg);
    // unique/internal space is always unaffected (early return).
    let unique = Rc::new(AddrSpace::new(
        spacetype::IPTR_INTERNAL,
        "unique",
        false,
        4,
        1,
        9,
        0,
        0,
        0,
    ));
    assert_eq!(
        model.has_effect(&addr(&unique, 0x0), 4),
        effect_type::UNAFFECTED
    );
}

#[test]
fn proto_model_assign_parameter_storage_orders_output_first() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = three_reg_proto_model(&mgr, &reg);
    let tf = PanicTypeFactory; // input/output assign with no model rules never reaches it
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: Some(int4_type()),
        intypes: vec![int4_type(), int4_type()],
        innames: vec!["a".into(), "b".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model
        .assign_parameter_storage(&proto, &mut res, false, &tf, &mgr)
        .unwrap();
    // res[0] is the output (0x10), res[1..] are inputs (0x10, 0x20).
    assert_eq!(res.len(), 3);
    assert_eq!(res[0].addr.get_offset(), 0x10); // output
    assert_eq!(res[1].addr.get_offset(), 0x10); // first input
    assert_eq!(res[2].addr.get_offset(), 0x20); // second input
}

#[test]
fn score_proto_model_perfect_fit_is_zero() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = three_reg_proto_model(&mgr, &reg);
    // Two trials matching the two input slots exactly -> score 0.
    let mut score = ScoreProtoModel::new(true, 2);
    score.add_parameter(&model, &addr(&reg, 0x10), 4);
    score.add_parameter(&model, &addr(&reg, 0x20), 4);
    score.do_score();
    assert_eq!(score.get_num_mismatch(), 0);
    assert_eq!(score.get_score(), 0);
}

#[test]
fn score_proto_model_hole_and_mismatch_penalties() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = three_reg_proto_model(&mgr, &reg);
    // One trial in slot 1 (0x20) only: slot 0 is a hole -> penalty[0] == 16.
    let mut score = ScoreProtoModel::new(true, 1);
    score.add_parameter(&model, &addr(&reg, 0x20), 4);
    score.do_score();
    assert_eq!(score.get_num_mismatch(), 0);
    assert_eq!(score.get_score(), 16);

    // A trial in an address that is not a parameter -> mismatch (penalty 20).
    let mut score2 = ScoreProtoModel::new(true, 1);
    score2.add_parameter(&model, &addr(&reg, 0x100), 4);
    score2.do_score();
    assert_eq!(score2.get_num_mismatch(), 1);
    assert_eq!(score2.get_score(), 20);
}

#[test]
fn proto_model_intersect_registers_keeps_common() {
    let reg = reg_space_le();
    let mut a = vec![vdata(&reg, 0x10, 4), vdata(&reg, 0x20, 4), vdata(&reg, 0x30, 4)];
    let mut b = vec![vdata(&reg, 0x20, 4), vdata(&reg, 0x30, 4), vdata(&reg, 0x40, 4)];
    a.sort_unstable();
    b.sort_unstable();
    ProtoModel::intersect_registers(&mut a, &b);
    // Intersection is {0x20, 0x30}.
    assert_eq!(a.len(), 2);
    assert_eq!(a[0].offset, 0x20);
    assert_eq!(a[1].offset, 0x30);
}

#[test]
fn proto_model_merged_fold_in_and_select() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    // Two constituent models: m0 uses 0x10/0x20, m1 uses 0x20/0x30.
    let mut m0 = ProtoModel::new(&mgr);
    m0.build_param_list("standard").unwrap();
    m0.set_name("m0");
    {
        let input = m0.input_mut();
        input.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
        let e1 = excl_entry(1, &reg, 0x20, 4, input.get_entry(), &mgr);
        input.push_entry(e1);
        input.finish_decode();
        let output = m0.output_mut();
        output.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
        output.finish_decode();
    }
    let mut m1 = ProtoModel::new(&mgr);
    m1.build_param_list("standard").unwrap();
    m1.set_name("m1");
    {
        let input = m1.input_mut();
        input.push_entry(excl_entry(0, &reg, 0x20, 4, &[], &mgr));
        let e1 = excl_entry(1, &reg, 0x30, 4, input.get_entry(), &mgr);
        input.push_entry(e1);
        input.finish_decode();
        let output = m1.output_mut();
        output.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
        output.finish_decode();
    }
    let m0 = Rc::new(m0);
    let m1 = Rc::new(m1);

    let mut merged = ProtoModel::new_merged(&mgr);
    merged.merged_push(Rc::clone(&m0)).unwrap();
    merged.merged_push(Rc::clone(&m1)).unwrap();
    merged.merged_finalize();
    assert!(merged.is_merged());
    assert_eq!(merged.num_models(), 2);

    // A single trial at 0x10 fits m0 (slot 0) but not m1 -> m0 scores better.
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4);
    active.get_trial_mut(0).mark_active();
    let selected = merged.select_model(&active).unwrap();
    assert_eq!(selected.get_name(), "m0");
}

// =========================================================================
// fspec-2: ParameterBasic / ProtoStoreInternal
// =========================================================================

#[test]
fn parameter_basic_lock_semantics() {
    let reg = reg_space_le();
    let mut p = ParameterBasic::new("a", addr(&reg, 0x10), int4_type(), 0);
    assert!(!p.is_type_locked());
    p.set_type_lock(true);
    assert!(p.is_type_locked());
    // Locking a non-unknown type does NOT set the size lock.
    assert!(!p.is_size_type_locked());
    p.set_type_lock(false);
    assert!(!p.is_type_locked());

    // Locking a TYPE_UNKNOWN also sets the size lock.
    let unk = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UNKNOWN));
    let mut q = ParameterBasic::new("b", addr(&reg, 0x10), unk, 0);
    q.set_type_lock(true);
    assert!(q.is_type_locked());
    assert!(q.is_size_type_locked());
}

#[test]
fn parameter_basic_override_size_lock_type() {
    let reg = reg_space_le();
    let unk = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UNKNOWN));
    let mut p = ParameterBasic::new("a", addr(&reg, 0x10), unk, 0);
    p.set_type_lock(true); // sets size lock too (unknown type)
                           // Override with a same-size int succeeds.
    assert!(p.override_size_lock_type(int4_type()).is_ok());
    assert_eq!(
        p.get_type().unwrap().get_metatype(),
        type_metatype::TYPE_INT
    );
    // Override with a different size fails.
    let int8 = Rc::new(Datatype::new_with_align(8, 8, type_metatype::TYPE_INT));
    assert!(p.override_size_lock_type(int8).is_err());

    // Overriding a parameter that is not size-locked fails.
    let mut q = ParameterBasic::new("b", addr(&reg, 0x10), int4_type(), 0);
    let another = int4_type();
    assert!(q.override_size_lock_type(another).is_err());
}

#[test]
fn proto_store_internal_round_trip() {
    let reg = reg_space_le();
    let voidt = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let mut store = ProtoStoreInternal::new(Rc::clone(&voidt));
    // A fresh store has a void output and no inputs.
    assert_eq!(store.get_num_inputs(), 0);
    assert_eq!(
        store.get_output().get_type().unwrap().get_metatype(),
        type_metatype::TYPE_VOID
    );

    // Set two inputs and an output.
    let p0 = ParameterPieces { addr: addr(&reg, 0x10), type_: Some(int4_type()), flags: 0 };
    let p1 = ParameterPieces { addr: addr(&reg, 0x20), type_: Some(int4_type()), flags: 0 };
    store.set_input(0, "a", &p0);
    store.set_input(1, "b", &p1);
    let out = ParameterPieces { addr: addr(&reg, 0x10), type_: Some(int4_type()), flags: 0 };
    store.set_output(&out);
    assert_eq!(store.get_num_inputs(), 2);
    assert_eq!(store.get_input(0).unwrap().get_name(), "a");
    assert_eq!(store.get_input(1).unwrap().get_address().get_offset(), 0x20);
    assert_eq!(
        store.get_output().get_type().unwrap().get_metatype(),
        type_metatype::TYPE_INT
    );

    // clearInput shifts following parameters down.
    store.clear_input(0);
    assert_eq!(store.get_num_inputs(), 1);
    assert_eq!(store.get_input(0).unwrap().get_name(), "b");

    // clone is independent.
    let cloned = store.clone_box();
    assert_eq!(cloned.get_num_inputs(), 1);

    // clearOutput restores void.
    store.clear_output();
    assert_eq!(
        store.get_output().get_type().unwrap().get_metatype(),
        type_metatype::TYPE_VOID
    );

    // clearAllInputs empties.
    store.clear_all_inputs();
    assert_eq!(store.get_num_inputs(), 0);
}

// =========================================================================
// fspec-2: FuncProto
// =========================================================================

#[test]
fn func_proto_flag_matrix() {
    let mut fp = FuncProto::new();
    // Default: nothing set.
    assert!(!fp.is_inline());
    assert!(!fp.is_no_return());
    assert!(!fp.is_dotdotdot());
    assert!(!fp.is_constructor());
    assert!(!fp.is_destructor());
    assert!(!fp.is_override());

    fp.set_inline(true);
    assert!(fp.is_inline());
    fp.set_inline(false);
    assert!(!fp.is_inline());

    fp.set_no_return(true);
    assert!(fp.is_no_return());
    fp.set_dotdotdot(true);
    assert!(fp.is_dotdotdot());
    fp.set_constructor(true);
    assert!(fp.is_constructor());
    fp.set_destructor(true);
    assert!(fp.is_destructor());
    fp.set_override(true);
    assert!(fp.is_override());
    fp.set_input_errors(true);
    assert!(fp.has_input_errors());
    fp.set_output_errors(true);
    assert!(fp.has_output_errors());

    // Comparable flags exclude inline/no_return/error/override.
    let cmp = fp.get_comparable_flags();
    assert_ne!(cmp & func_proto_flags::DOTDOTDOT, 0);
    assert_ne!(cmp & func_proto_flags::IS_CONSTRUCTOR, 0);
    assert_eq!(cmp & func_proto_flags::IS_INLINE, 0);
    assert_eq!(cmp & func_proto_flags::IS_OVERRIDE, 0);
}

#[test]
fn func_proto_set_model_inherits_properties() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = three_reg_proto_model(&mgr, &reg);
    model.set_has_this(true);
    model.set_constructor(true);
    model.set_extra_pop(8);
    let model = Rc::new(model);

    let mut fp = FuncProto::new();
    fp.set_model(Some(Rc::clone(&model)));
    assert!(fp.has_this_pointer());
    assert!(fp.is_constructor());
    assert_eq!(fp.get_extra_pop(), 8);
    assert_eq!(fp.get_model_name(), "__cdecl");

    // Clearing the model sets extrapop to unknown.
    fp.set_model(None);
    assert_eq!(fp.get_extra_pop(), EXTRAPOP_UNKNOWN);
}

#[test]
fn func_proto_inject_id_toggles_inline() {
    let mut fp = FuncProto::new();
    fp.set_inject_id(7);
    assert_eq!(fp.get_inject_id(), 7);
    assert!(fp.is_inline());
    // A negative id cancels.
    fp.set_inject_id(-1);
    assert_eq!(fp.get_inject_id(), -1);
    assert!(!fp.is_inline());
    fp.set_inject_id(3);
    fp.cancel_inject_id();
    assert_eq!(fp.get_inject_id(), -1);
    assert!(!fp.is_inline());
}

#[test]
fn func_proto_input_lock_void_and_params() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let voidt = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let model = Rc::new(three_reg_proto_model(&mgr, &reg));

    let mut fp = FuncProto::new();
    fp.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    // No params: locking sets the void-input lock and the model lock.
    assert!(!fp.is_input_locked());
    fp.set_input_lock(true);
    assert!(fp.is_input_locked());
    assert!(fp.is_model_locked());
    fp.set_input_lock(false);
    assert!(!fp.is_input_locked());

    // With a param: input lock type-locks the parameter.
    let p0 = ParameterPieces { addr: addr(&reg, 0x10), type_: Some(int4_type()), flags: 0 };
    fp.set_param(0, "a", &p0);
    assert!(!fp.is_input_locked());
    fp.set_input_lock(true);
    assert!(fp.is_input_locked());
    assert!(fp.get_param(0).unwrap().is_type_locked());
}

#[test]
fn func_proto_resolve_extra_pop_x86() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stack = stack_space();
    let voidt = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let model = Rc::new(three_reg_proto_model(&mgr, &reg));

    let mut fp = FuncProto::new();
    fp.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    // A single stack parameter at offset 0 of size 4 -> cur = (0+4+3)&~3 = 4,
    // extrapop = max(4 (retaddr), 4) = 4.
    let p0 = ParameterPieces { addr: addr(&stack, 0x0), type_: Some(int4_type()), flags: 0 };
    fp.set_param(0, "a", &p0);
    fp.set_input_lock(true); // resolveExtraPop only runs when input is locked
    fp.resolve_extra_pop();
    assert_eq!(fp.get_extra_pop(), 4);

    // A stack parameter at offset 4 size 4 -> cur = (4+4+3)&~3 = 8 -> extrapop 8.
    let mut fp2 = FuncProto::new();
    fp2.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    let p1 = ParameterPieces { addr: addr(&stack, 0x4), type_: Some(int4_type()), flags: 0 };
    fp2.set_param(0, "a", &p1);
    fp2.set_input_lock(true);
    fp2.resolve_extra_pop();
    assert_eq!(fp2.get_extra_pop(), 8);
}

#[test]
fn func_proto_copy_and_compatible() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let voidt = Rc::new(Datatype::new(0, type_metatype::TYPE_VOID));
    let model = Rc::new(three_reg_proto_model(&mgr, &reg));

    let mut fp = FuncProto::new();
    fp.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    fp.set_inline(true);
    fp.set_no_return(true);
    fp.set_inject_id(5);

    let mut fp2 = FuncProto::new();
    fp2.copy(&fp);
    assert!(fp2.is_inline());
    assert!(fp2.is_no_return());
    assert_eq!(fp2.get_inject_id(), 5);
    assert!(fp2.has_model());
    // Same model + same flags -> compatible.
    assert!(fp.is_compatible(&fp2));

    // copyFlowEffects copies only inline/no_return/injectid.
    let mut fp3 = FuncProto::new();
    fp3.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    fp3.copy_flow_effects(&fp);
    assert!(fp3.is_inline());
    assert!(fp3.is_no_return());
    assert_eq!(fp3.get_inject_id(), 5);
}

#[test]
fn func_proto_return_bytes_consumed_takes_smallest() {
    let mut fp = FuncProto::new();
    assert!(!fp.set_return_bytes_consumed(0)); // 0 is a no-op
    assert!(fp.set_return_bytes_consumed(8)); // first non-zero
    assert_eq!(fp.get_return_bytes_consumed(), 8);
    assert!(fp.set_return_bytes_consumed(4)); // smaller -> update
    assert_eq!(fp.get_return_bytes_consumed(), 4);
    assert!(!fp.set_return_bytes_consumed(6)); // larger -> no change
    assert_eq!(fp.get_return_bytes_consumed(), 4);
}

#[test]
fn func_proto_stub_methods_error() {
    let mut fp = FuncProto::new();
    assert!(fp.set_scope().is_err());
    assert!(fp.update_input_types().is_err());
    assert!(fp.update_output_types().is_err());
    assert!(fp.decode().is_err());
    let mgr = AddrSpaceManager::new();
    let mut model = ProtoModel::new(&mgr);
    assert!(model.decode().is_err());
}

// =========================================================================
// fspec-3: FuncCallSpecs
// =========================================================================

use crate::funcdata::Funcdata;
use crate::kuna_restartlog::RestartLog;
use crate::op::pcodeop_flags;
use crate::context::{ArchContext, OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

/// A manager carrying const / unique / ram (code) / stack (spacebase) spaces,
/// like the real architecture wiring the call-site logic walks.
fn fcs_manager() -> AddrSpaceManager {
    use kuna_base::space::{ConstantSpace, UniqueSpace};
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    // A spacebase (stack) space at index 3.
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        8,
        1,
        3,
        0,
        1,
        1,
    )))
    .unwrap();
    // The fspec space, so deindirect's CALLIND->CALL rewrite can build the
    // call-spec annotation Varnode (newVarnodeCallSpecs).
    m.insert_space(Rc::new(kuna_base::space::FspecSpace::new(4))).unwrap();
    m.set_default_code_space(2).unwrap();
    m
}

fn fcs_fd() -> Funcdata {
    let glb = Rc::new(ArchContext::new(fcs_manager()));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let entry = Address::new(ram, 0x1000);
    Funcdata::new("caller", "caller", glb, entry, 0x10000000, 0x40).unwrap()
}

fn fcs_ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}
fn fcs_stack(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("stack").unwrap())
}
fn fcs_const(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space(0).unwrap())
}

fn fcs_new_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = fcs_ram(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    let flags = if opc == OpCode::CPUI_CALL || opc == OpCode::CPUI_CALLIND {
        pcodeop_flags::call
    } else {
        0
    };
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, flags, format!("{opc:?}")));
    op
}

/// A varnode in `space` at `off`/`size` defined by `op` (sets `written`).
fn fcs_out(fd: &mut Funcdata, op: OpId, space: Rc<AddrSpace>, off: u64, size: int4) -> VarnodeId {
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum: seq };
    let vn = {
        let mut noop = |_b: &mut crate::varnode::VarnodeBank,
                        _o: VarnodeId,
                        _n: VarnodeId|
         -> KunaResult<()> { panic!("collision") };
        fd.vbank_mut()
            .create_def(size, Address::new(space, off), Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN)), def, &mut noop)
            .unwrap()
    };
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

fn fcs_const_vn(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    let c = fcs_const(fd);
    fd.vbank_mut().create(size, Address::new(c, val), Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN)))
}

/// A `FuncProto` with a standard model and an internal (void-output) store —
/// the post-construction state a real `FuncCallSpecs` has before parameter
/// recovery (model from the ProtoModel registry, internal store).
fn proto_with_std_model(mgr: &AddrSpaceManager) -> FuncProto {
    let mut model = ProtoModel::new(mgr);
    model.build_param_list("standard").unwrap();
    let voidt = Rc::new(Datatype::new(1, type_metatype::TYPE_VOID));
    let mut fp = FuncProto::new();
    fp.set_internal(Rc::new(model), voidt);
    fp
}

/// A `FuncCallSpecs` on `call` with a standard model + internal store (FuncProto
/// `copy` carries both the model and a `clone_box` of the store).
fn fcs_with_std_model(mgr: &AddrSpaceManager, call: OpId) -> FuncCallSpecs {
    let mut fc = FuncCallSpecs::new(call, Address::default());
    let fp = proto_with_std_model(mgr);
    fc.proto_mut().copy(&fp);
    fc
}

#[test]
fn func_call_specs_constructor_defaults() {
    let mut fd = fcs_fd();
    let ram = fcs_ram(&fd);
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let entry = Address::new(ram, 0x2000);
    let fc = FuncCallSpecs::new(call, entry.clone());

    assert_eq!(fc.get_op(), call);
    assert_eq!(*fc.get_entry_address(), entry);
    assert_eq!(fc.get_effective_extra_pop(), EXTRAPOP_UNKNOWN);
    assert_eq!(fc.get_spacebase_offset(), OFFSET_UNKNOWN);
    assert_eq!(fc.get_stack_placeholder_slot(), -1);
    assert_eq!(fc.get_paramshift(), 0);
    assert!(!fc.is_input_active());
    assert!(!fc.is_output_active());
    assert!(!fc.is_bad_jump_table());
    assert!(!fc.is_stack_output_lock());
    assert!(!fc.has_funcdata());
    assert!(fc.get_name().is_empty());
}

#[test]
fn func_call_specs_state_toggles_and_set_funcdata() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let ram = fcs_ram(&fd);
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = fcs_with_std_model(&mgr, call);

    fc.set_paramshift(2);
    assert_eq!(fc.get_paramshift(), 2);
    fc.set_effective_extra_pop(8);
    assert_eq!(fc.get_effective_extra_pop(), 8);
    fc.set_bad_jump_table(true);
    assert!(fc.is_bad_jump_table());
    fc.set_stack_output_lock(true);
    assert!(fc.is_stack_output_lock());

    fc.init_active_output();
    assert!(fc.is_output_active());
    fc.clear_active_output();
    assert!(!fc.is_output_active());

    // init_active_input: standard model max input delay 0 => maxpass 0.
    fc.init_active_input();
    assert!(fc.is_input_active());
    assert_eq!(fc.get_active_input().get_max_pass(), 0);
    fc.clear_active_input();
    assert!(!fc.is_input_active());

    // set_funcdata records presence + name; double set errs.
    let entry = Address::new(ram, 0x2000);
    assert!(fc.set_funcdata(entry.clone(), "callee").is_ok());
    assert!(fc.has_funcdata());
    assert_eq!(fc.get_name(), "callee");
    assert_eq!(*fc.get_entry_address(), entry);
    assert!(fc.set_funcdata(entry, "callee2").is_err());
}

#[test]
fn func_call_specs_input_bytes_consumed_monotone() {
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());

    // Unset slot reads 0.
    assert_eq!(fc.get_input_bytes_consumed(3), 0);
    // First non-zero set takes (grows the vector with zeros).
    assert!(fc.set_input_bytes_consumed(2, 4));
    assert_eq!(fc.get_input_bytes_consumed(2), 4);
    assert_eq!(fc.get_input_bytes_consumed(0), 0);
    // Smaller -> update.
    assert!(fc.set_input_bytes_consumed(2, 2));
    assert_eq!(fc.get_input_bytes_consumed(2), 2);
    // Larger -> no change.
    assert!(!fc.set_input_bytes_consumed(2, 6));
    assert_eq!(fc.get_input_bytes_consumed(2), 2);
}

/// `get_trial_for_input_varnode` index math: subtract 1 (call address) plus 1
/// more when past the stack placeholder.
#[test]
fn param_active_trial_for_input_varnode_index_math() {
    let stk = stack_space();
    let mut pa = ParamActive::new(true);
    // Three trials.
    pa.register_trial(&addr(&stk, 0x0), 4); // slot 1
    pa.register_trial(&addr(&stk, 0x4), 4); // slot 2
    pa.register_trial(&addr(&stk, 0x8), 4); // slot 3

    // No placeholder: input slot s maps to trial[s-1].
    assert_eq!(pa.get_trial_for_input_varnode(1).get_address().get_offset(), 0x0);
    assert_eq!(pa.get_trial_for_input_varnode(2).get_address().get_offset(), 0x4);

    // After establishing a placeholder (slot 4), input slots past it subtract 2.
    pa.set_placeholder_slot(); // stackplaceholder = slotbase (4)
    // slot 2 < placeholder(4): subtract 1 -> trial[1] (offset 0x4)
    assert_eq!(pa.get_trial_for_input_varnode(2).get_address().get_offset(), 0x4);
    // slot 5 >= placeholder(4): subtract 2 -> trial[3]... but only 3 trials;
    // use slot 4 -> trial[2] (offset 0x8).
    assert_eq!(pa.get_trial_for_input_varnode(4).get_address().get_offset(), 0x8);
}

#[test]
fn func_call_specs_check_input_join_gates() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = fcs_with_std_model(&mgr, call);
    let stk = stack_space();

    // Two adjacent 4-byte stack trials.
    fc.get_active_input().register_trial(&addr(&stk, 0x0), 4);
    fc.get_active_input().register_trial(&addr(&stk, 0x4), 4);

    // While input recovery is active, never join.
    fc.init_active_input();
    assert!(!fc.check_input_join(1, true, 4, 4));
    fc.clear_active_input();

    // slot1 past the trial count => false.
    assert!(!fc.check_input_join(5, true, 4, 4));

    // Size mismatch on the high/low slot => false (returns at the size check
    // before delegating to FuncProto::checkInputJoin).
    assert!(!fc.check_input_join(1, true, 8, 4)); // hislot size 4 != vn1 8
    assert!(!fc.check_input_join(1, false, 4, 8)); // hislot size 4 != vn2 8
}

#[test]
fn func_call_specs_do_input_join_locked_errs() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = fcs_with_std_model(&mgr, call);
    // Lock the input prototype (0 params => VOIDINPUTLOCK).
    fc.proto_mut().set_input_lock(true);
    // A trivial RegisterLookup: the default-impl trait object is reached via
    // the manager's installed lookup; here joins on a locked proto err before
    // any join-address construction, so a dummy lookup is fine.
    struct NoLookup;
    impl kuna_base::space::RegisterLookup for NoLookup {
        fn get_register(
            &self,
            _nm: &str,
        ) -> KunaResult<kuna_base::space::VarnodeStorage> {
            Err(kuna_base::error::KunaError::lowlevel("no register"))
        }
        fn get_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: int4) -> String {
            String::new()
        }
        fn get_exact_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: int4) -> String {
            String::new()
        }
    }
    let res = fc.do_input_join(1, true, &mgr, &NoLookup);
    assert!(res.is_err());
}

#[test]
fn func_call_specs_late_restriction_no_model_copies() {
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());
    // No model on `fc`: lateRestriction copies the restricted proto wholesale.
    assert!(!fc.proto().has_model());

    let mut restricted = FuncProto::new();
    restricted.set_extra_pop(12);

    let mut newinput: Vec<Option<VarnodeId>> = Vec::new();
    let mut newoutput: Vec<VarnodeId> = Vec::new();
    let ok = fc.late_restriction(&fd, &restricted, &mut newinput, &mut newoutput).unwrap();
    assert!(ok);
    assert_eq!(fc.proto().get_extra_pop(), 12);
}

#[test]
fn func_call_specs_late_restriction_incompatible_models_false() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());

    // Give `fc` a model so hasModel() is true.
    let mut model_a = ProtoModel::new(&mgr);
    model_a.build_param_list("standard").unwrap();
    fc.proto_mut().set_model(Some(Rc::new(model_a)));

    // A restricted proto with a *different-named* model => isCompatible false.
    let mut model_b = ProtoModel::new(&mgr);
    model_b.build_param_list("register").unwrap();
    let mut restricted = FuncProto::new();
    restricted.set_model(Some(Rc::new(model_b)));

    let mut ni: Vec<Option<VarnodeId>> = Vec::new();
    let mut no: Vec<VarnodeId> = Vec::new();
    let ok = fc.late_restriction(&fd, &restricted, &mut ni, &mut no).unwrap();
    assert!(!ok);
}

#[test]
fn func_call_specs_deindirect_restart_when_incompatible() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let ram = fcs_ram(&fd);
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());

    // Model on fc, different model on newproto => lateRestriction fails =>
    // restart is recorded and restart-pending is set.
    let mut m_a = ProtoModel::new(&mgr);
    m_a.build_param_list("standard").unwrap();
    fc.proto_mut().set_model(Some(Rc::new(m_a)));

    let mut m_b = ProtoModel::new(&mgr);
    m_b.build_param_list("register").unwrap();
    let mut newproto = FuncProto::new();
    newproto.set_model(Some(Rc::new(m_b)));

    let mut log = RestartLog::new();
    let new_entry = Address::new(ram, 0x2000);
    fc.deindirect(&mut fd, new_entry.clone(), "callee", &newproto, false, false, &mut log)
        .unwrap();

    assert!(fd.has_restart_pending());
    assert!(!log.is_empty_for(&fd));
    let dump = log.render(&fd);
    assert!(dump.contains("prototype discovered late at indirect call"), "dump={dump}");
    // The de-indirect updated the entry/name even though it restarted.
    assert_eq!(*fc.get_entry_address(), new_entry);
    assert_eq!(fc.get_name(), "callee");
}

#[test]
fn func_call_specs_deindirect_override_short_circuits() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let ram = fcs_ram(&fd);
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());

    let mut m_a = ProtoModel::new(&mgr);
    m_a.build_param_list("standard").unwrap();
    fc.proto_mut().set_model(Some(Rc::new(m_a)));
    // Mark the call-site as overridden: deindirect must bail before any restart.
    fc.proto_mut().set_override(true);

    let mut m_b = ProtoModel::new(&mgr);
    m_b.build_param_list("register").unwrap();
    let mut newproto = FuncProto::new();
    newproto.set_model(Some(Rc::new(m_b)));

    let mut log = RestartLog::new();
    let new_entry = Address::new(ram, 0x2000);
    fc.deindirect(&mut fd, new_entry, "callee", &newproto, false, false, &mut log)
        .unwrap();

    assert!(!fd.has_restart_pending());
    assert!(log.is_empty_for(&fd));
}

#[test]
fn func_call_specs_force_set_restart_and_locks() {
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = fcs_with_std_model(&mgr, call);

    // Incompatible restrictive prototype (different model name) => lateRestriction
    // fails => restart.
    let mut m_b = ProtoModel::new(&mgr);
    m_b.build_param_list("register").unwrap();
    let voidt = Rc::new(Datatype::new(1, type_metatype::TYPE_VOID));
    let mut fp = FuncProto::new();
    fp.set_internal(Rc::new(m_b), voidt);
    fp.set_input_errors(true);
    fp.set_output_errors(true);

    let mut log = RestartLog::new();
    fc.force_set(&mut fd, &fp, &mut log).unwrap();

    assert!(fd.has_restart_pending());
    let dump = log.render(&fd);
    assert!(dump.contains("prototype forced late at call site"), "dump={dump}");
    // Regardless of restart, the input is locked (0 params => VOIDINPUTLOCK) and
    // the error flags are transferred from fp.
    assert!(fc.proto().is_input_locked());
    assert!(fc.proto().has_input_errors());
    assert!(fc.proto().has_output_errors());
}

/// Build a CALLIND with a stack-pointer placeholder input:
///   refvn   = (stack, 0x10)   [the spacebase reference]
///   phvn    = LOAD(refvn, _)   marked spacebase placeholder
///   CALLIND(target, phvn@slot1)
/// Returns (fd, fc, phvn).  `fc` carries a standard model + internal store.
fn build_call_with_placeholder(mgr: &AddrSpaceManager) -> (Funcdata, FuncCallSpecs, VarnodeId) {
    let mut fd = fcs_fd();
    let stk = fcs_stack(&fd);

    // The CALLIND op, 2 inputs (the target + the placeholder).
    let call = fcs_new_op(&mut fd, 2, 0x1010, OpCode::CPUI_CALLIND);
    let target = fcs_const_vn(&mut fd, 0x2000, 8);
    fd.op_set_input(call, target, 0).unwrap();

    // The LOAD op producing the placeholder.  Its input 0 is the spacebase
    // reference varnode (a stack-space varnode whose offset is the spacebase
    // relative value).
    let load = fcs_new_op(&mut fd, 1, 0x1008, OpCode::CPUI_LOAD);
    // refvn: a free varnode in the stack space at offset 0x10.
    let refvn = fd.vbank_mut().create(
        8,
        Address::new(Rc::clone(&stk), 0x10),
        Rc::new(Datatype::new(8, type_metatype::TYPE_UNKNOWN)),
    );
    fd.op_set_input(load, refvn, 0).unwrap();
    // phvn: the LOAD output (written), placed in unique space, marked placeholder.
    let uniq = Rc::clone(fd.get_arch().manage().get_space(1).unwrap());
    let phvn = fcs_out(&mut fd, load, uniq, 0x0, 8);
    fd.vbank_mut().get_mut(phvn).unwrap().set_spacebase_placeholder();

    // Wire phvn as the CALLIND placeholder input at slot 1.
    fd.op_set_input(call, phvn, 1).unwrap();

    let mut fc = fcs_with_std_model(mgr, call);
    fc.set_paramshift(0);
    (fd, fc, phvn)
}

#[test]
fn func_call_specs_get_spacebase_relative_no_slot_is_none() {
    let mgr = fcs_manager();
    let (fd, fc, _phvn) = build_call_with_placeholder(&mgr);
    // No placeholder slot recorded (stack_placeholder_slot == -1) => None even
    // though the CALLIND has a placeholder input wired.
    assert_eq!(fc.get_stack_placeholder_slot(), -1);
    assert!(fc.get_spacebase_relative(&fd).is_none());
}

#[test]
fn func_call_specs_resolve_and_abort_spacebase_relative() {
    let mgr = fcs_manager();
    let (mut fd, mut fc, phvn) = build_call_with_placeholder(&mgr);

    // stack_placeholder_slot < 0 and the prototype is not input-locked: resolve
    // reads the offset off the placeholder's def input, then throws "Unresolved
    // stack placeholder" (the C++ end-of-function throw).
    let r = fc.resolve_spacebase_relative(&mut fd, phvn);
    assert!(r.is_err());
    // The offset was read off refvn (stack, 0x10) before the error.
    assert_eq!(fc.get_spacebase_offset(), 0x10);

    // abort with no placeholder slot is a no-op (slot stays -1).
    fc.abort_spacebase_relative(&mut fd);
    assert_eq!(fc.get_stack_placeholder_slot(), -1);
}

#[test]
fn func_call_specs_fspec_registry_roundtrip() {
    let mut fd = fcs_fd();
    let ram = fcs_ram(&fd);
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);

    // Named call spec: printed name is the name verbatim, regardless of angr.
    let mut fc = FuncCallSpecs::new(call, Address::new(Rc::clone(&ram), 0x2000));
    fc.set_funcdata(Address::new(Rc::clone(&ram), 0x2000), "my_func").unwrap();
    assert_eq!(fc.fspec_printed_name(crate::database::KunaNameStyle::Func), "my_func");
    assert_eq!(fc.fspec_printed_name(crate::database::KunaNameStyle::Angr), "my_func");

    // Unnamed: angr => sub_<addr>, non-angr => func_<addr>.
    let fc2 = FuncCallSpecs::new(call, Address::new(Rc::clone(&ram), 0x3000));
    assert!(fc2.fspec_printed_name(crate::database::KunaNameStyle::Angr).starts_with("sub_"));
    assert!(fc2.fspec_printed_name(crate::database::KunaNameStyle::Func).starts_with("func_"));

    // Register and look up via the kuna-base FspecSpace registry.
    let handle: u64 = 0xCAFE;
    fc.register_in_fspec_space(handle, crate::database::KunaNameStyle::Func);
    let info = kuna_base::space::fspec_lookup(handle).unwrap();
    assert_eq!(info.printed_name, "my_func");
    assert_eq!(info.entry.get_offset(), 0x2000);

    // The FspecSpace itself prints the registered name through printRaw.
    let fspec = kuna_base::space::FspecSpace::new(7);
    let mut s = String::new();
    fspec.print_raw(&mut s, handle).unwrap();
    assert_eq!(s, "my_func");

    kuna_base::space::fspec_unregister(handle);
    assert!(kuna_base::space::fspec_lookup(handle).is_none());
    // After unregister, printRaw errs on the unknown handle.
    let mut s2 = String::new();
    assert!(fspec.print_raw(&mut s2, handle).is_err());
}

#[test]
fn func_call_specs_transfer_locked_output_void_ok() {
    // transfer_locked_output is private; exercise it through late_restriction
    // with an output-locked, *void*-output restricted proto.  A void output
    // returns Ok(true) (no Varnode transfer needed), so lateRestriction
    // succeeds.  Compatibility is by ProtoModel identity, so both protos share
    // the same model Rc.
    let mgr = fcs_manager();
    let mut fd = fcs_fd();
    let call = fcs_new_op(&mut fd, 1, 0x1010, OpCode::CPUI_CALLIND);
    let mut fc = FuncCallSpecs::new(call, Address::default());

    let mut model = ProtoModel::new(&mgr);
    model.build_param_list("standard").unwrap();
    let model = Rc::new(model);
    let voidt = Rc::new(Datatype::new(1, type_metatype::TYPE_VOID));

    // fc and restricted share the same model => isCompatible true.
    fc.proto_mut().set_internal(Rc::clone(&model), Rc::clone(&voidt));

    let mut restricted = FuncProto::new();
    restricted.set_internal(Rc::clone(&model), Rc::clone(&voidt));
    restricted.set_output_lock(true); // output is void by default => transfer Ok(true)

    let mut ni: Vec<Option<VarnodeId>> = Vec::new();
    let mut no: Vec<VarnodeId> = Vec::new();
    // Not input-locked, output-locked-void => lateRestriction succeeds.
    let ok = fc.late_restriction(&fd, &restricted, &mut ni, &mut no).unwrap();
    assert!(ok);
    assert!(no.is_empty());
}

// =========================================================================
// ModelRule wiring (float-typeclass wave): the cspec `<rule>` actions are now
// plumbed into `ParamListStandard::assign_address` (the `goto_stack` float10
// model + the `pointermax` ConvertToPointer rule).  These exercise the
// wired chain end-to-end: ModelRule precedence over the metatype fallback,
// the goto_stack stack footprint for an oversized type, and the pointermax
// conversion.
// =========================================================================

/// A 10-byte float (x87 long double / `float10`), aligned to 8 (the gcc
/// alignment map gives `getAlignment(10)==8`; `getAlignSize` then rounds the
/// 10-byte size up to the 16-byte 2-slot footprint).
fn float10_type() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(10, 8, type_metatype::TYPE_FLOAT))
}

/// A model with one 8-byte general register (group 0) and an 8-byte-aligned,
/// multi-slot stack resource (group 1), plus a single trailing `goto_stack`
/// ModelRule.  Mirrors (in miniature) the SysV `<input>` list: registers for
/// the small ints, the stack for everything the rule routes there.
fn reg_plus_stack_gotostack_model(mgr: &AddrSpaceManager) -> (ParamListStandard, Rc<AddrSpace>, Rc<AddrSpace>) {
    let reg = reg_space_le();
    let stk = stack_space();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    // group 0: one 8-byte general register at reg:0x10.
    let e0 = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&reg),
        0x10,
        8,
        1,
        0,
        0,
        true,
        false,
        &[],
        mgr,
    )
    .expect("seed reg entry");
    model.push_entry(e0);
    // group 1: an 8-byte-aligned multi-slot stack resource (size 0x100, align 8).
    let e1 = stack_entry(1, &stk, 0x0, 0x100, 8, model.get_entry(), mgr);
    model.push_entry(e1);
    model.finish_decode();
    // The trailing `<rule><datatype name="any"/><goto_stack/></rule>`: a
    // (true-filter) GotoStack action whose stack_entry resolves to the model's
    // stack resource.
    let action = crate::modelrules::AssignAction::GotoStack {
        stack_entry: model.get_stack_entry(),
    };
    let filter = crate::modelrules::DatatypeFilter::SizeRestricted(
        crate::modelrules::SizeRestriction::new(0, 0),
    );
    model.push_model_rule(crate::modelrules::ModelRule::from_components(filter, action));
    (model, reg, stk)
}

#[test]
fn modelrule_gotostack_routes_oversized_float_to_aligned_stack_footprint() {
    // float-typeclass wave: a 10-byte float10 (x87 long double) does not fit any
    // register pentry (maxsize 8); the wired `goto_stack` ModelRule routes it to
    // the stack, reserving a 16-byte (2 x align-8 slots) footprint so a following
    // parameter lands at offset 0x10, not inside the float10.  This is the
    // longdouble `passmany` stack layout: `x`(float10)@0x0..0x10, next@0x10.
    let mgr = AddrSpaceManager::new();
    let (model, _reg, stk) = reg_plus_stack_gotostack_model(&mgr);
    let tf = PanicTypeFactory; // goto_stack never touches the TypeFactory
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![float10_type(), int4_type()],
        innames: vec!["x".into(), "z".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 2);
    // float10 -> stack at 0x0 (the rule wins over the register fallback; the
    // 10-byte float can't fit the 8-byte register anyway).
    assert!(Rc::ptr_eq(res[0].addr.get_space().unwrap(), &stk));
    assert_eq!(res[0].addr.get_offset(), 0x0);
    assert_eq!(res[0].type_.as_ref().unwrap().get_size(), 10);
    // The next int4 lands AFTER the float10's 16-byte (2-slot) footprint, at
    // 0x10 — never overlapping the float10 (the dropped-`z` failure mode is
    // exactly an overlapping next-param).
    assert!(Rc::ptr_eq(res[1].addr.get_space().unwrap(), &stk));
    assert_eq!(res[1].addr.get_offset(), 0x10);
}

#[test]
fn modelrule_precedes_metatype_fallback() {
    // The `goto_stack` (true-filter) rule fires for an int4 too, taking
    // precedence over the metatype-keyed register fallback: the int4 is routed
    // to the stack, NOT the general register, because the rule returns a
    // non-fail response first (C++ `assignAddress` iterates rules before the
    // fallback, fspec.cc:783-792).
    let mgr = AddrSpaceManager::new();
    let (model, reg, stk) = reg_plus_stack_gotostack_model(&mgr);
    let tf = PanicTypeFactory;
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![int4_type()],
        innames: vec!["a".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 1);
    assert!(Rc::ptr_eq(res[0].addr.get_space().unwrap(), &stk));
    assert!(!Rc::ptr_eq(res[0].addr.get_space().unwrap(), &reg));
}

#[test]
fn no_modelrule_falls_through_to_metatype_fallback() {
    // Control: WITHOUT any ModelRule, the same int4 takes the register fallback
    // (the pre-wiring behavior the 327-assertion baseline depends on stays
    // intact for rule-less models).
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = ParamEntry::seed(
        0,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(&reg),
        0x10,
        8,
        1,
        0,
        0,
        true,
        false,
        &[],
        &mgr,
    )
    .expect("seed reg entry");
    model.push_entry(e0);
    model.finish_decode();
    assert_eq!(model.num_model_rules(), 0);
    let tf = PanicTypeFactory;
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![int4_type()],
        innames: vec!["a".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 1);
    assert!(Rc::ptr_eq(res[0].addr.get_space().unwrap(), &reg));
    assert_eq!(res[0].addr.get_offset(), 0x10);
}

#[test]
fn push_pointermax_rule_appends_convert_to_pointer() {
    // The synthetic `pointermax` rule (C++ fspec.cc:1507-1512): a
    // SizeRestrictedFilter(pointermax+1, 0) feeding a ConvertToPointer action,
    // appended to model_rules.  Verifies the count bump (the action's pointer
    // build is exercised by the modelrules ConvertToPointer tests, which need a
    // real TypeFactory).
    let mgr = AddrSpaceManager::new();
    let (mut model, _reg) = three_reg_model();
    assert_eq!(model.num_model_rules(), 0);
    model.push_pointermax_rule(8);
    assert_eq!(model.num_model_rules(), 1);
}

// =========================================================================
// Adversarial tests (verifier, w10-float-typeclass): the rule-iteration
// boundary in `ParamListStandard::assign_address` (first non-fail wins, a
// FAILING rule does not short-circuit the chain) and the `pointermax` rule
// append-at-end ordering.  These target the most fragile spots of the
// ModelRule wiring: the loop's continue-on-fail semantics (C++
// fspec.cc:778-784) and the C++ "pointermax rule planted at the END of
// modelRules" ordering (fspec.cc:1507-1512, after the decoded rules).
// =========================================================================

/// Build a model with a non-matching (size >= 20) filter on a FIRST goto_stack
/// rule and a matching (true) filter on a SECOND goto_stack rule.  An int4 (4
/// bytes) fails the first rule's size gate, so the chain must continue to the
/// second rule — proving the loop does not stop at the first `fail`.
#[test]
fn w10_float_typeclass_failing_rule_does_not_short_circuit_chain() {
    let mgr = AddrSpaceManager::new();
    // Build a fresh reg+stack model and push a TWO-rule chain in order:
    //   rule 1: size>=20 filter (an int4 fails its size gate -> returns `fail`),
    //   rule 2: true-filter goto_stack (matches everything).
    let reg = reg_space_le();
    let stk = stack_space();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    let e0 = ParamEntry::seed(
        0, type_class::TYPECLASS_GENERAL, Rc::clone(&reg), 0x10, 8, 1, 0, 0, true, false, &[], &mgr,
    )
    .expect("seed reg entry");
    model.push_entry(e0);
    let e1 = stack_entry(1, &stk, 0x0, 0x100, 8, model.get_entry(), &mgr);
    model.push_entry(e1);
    model.finish_decode();
    // Rule 1: size>=20 filter -> an int4 never matches, the action is never
    // consulted, the rule returns `fail`.
    let fail_filter = crate::modelrules::DatatypeFilter::SizeRestricted(
        crate::modelrules::SizeRestriction::new(20, 0),
    );
    let fail_action = crate::modelrules::AssignAction::GotoStack {
        stack_entry: model.get_stack_entry(),
    };
    model.push_model_rule(crate::modelrules::ModelRule::from_components(fail_filter, fail_action));
    // Rule 2: true filter goto_stack (matches everything).
    let ok_filter = crate::modelrules::DatatypeFilter::SizeRestricted(
        crate::modelrules::SizeRestriction::new(0, 0),
    );
    let ok_action = crate::modelrules::AssignAction::GotoStack {
        stack_entry: model.get_stack_entry(),
    };
    model.push_model_rule(crate::modelrules::ModelRule::from_components(ok_filter, ok_action));
    assert_eq!(model.num_model_rules(), 2);
    let tf = PanicTypeFactory;
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![int4_type()],
        innames: vec!["a".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 1);
    // The SECOND rule won (stack), proving the first `fail` did not abort the
    // chain AND did not fall straight to the metatype fallback (which would have
    // routed the int4 to the general register at 0x10).
    assert!(Rc::ptr_eq(res[0].addr.get_space().unwrap(), &stk));
    assert!(!Rc::ptr_eq(res[0].addr.get_space().unwrap(), &reg));
}

/// The synthetic `pointermax` rule is appended to the END of `model_rules`
/// (C++ fspec.cc:1507-1512: `modelRules.emplace_back(...)` after the decoded
/// rules).  A preceding `goto_stack` rule must therefore still win for a type
/// that ALSO exceeds pointermax — the pointermax rule never preempts an earlier
/// matching rule.
#[test]
fn w10_float_typeclass_pointermax_rule_appends_after_existing_rules() {
    let mgr = AddrSpaceManager::new();
    let (mut model, _reg, stk) = reg_plus_stack_gotostack_model(&mgr);
    // model has one true-filter goto_stack rule.  Append pointermax(8): any type
    // > 8 bytes would (in isolation) convert to a pointer.
    assert_eq!(model.num_model_rules(), 1);
    model.push_pointermax_rule(8);
    assert_eq!(model.num_model_rules(), 2);
    // A 10-byte float10 exceeds pointermax(8); but the goto_stack rule precedes
    // the pointermax rule, so it wins -> the float10 lands on the stack at its
    // full 10-byte size, NOT converted to a pointer (which would shrink it to
    // the pointer width).
    let tf = PanicTypeFactory;
    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: None,
        intypes: vec![float10_type()],
        innames: vec!["x".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 1);
    assert!(Rc::ptr_eq(res[0].addr.get_space().unwrap(), &stk));
    // Full float10 size preserved (the earlier goto_stack rule won; the
    // pointermax ConvertToPointer never fired and never shrank it to ptr size).
    assert_eq!(res[0].type_.as_ref().unwrap().get_size(), 10);
}
