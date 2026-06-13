//! Tests for the parameter-recovery foundation (`fspec.rs`, item
//! `w6-s4-fspec-1`): ParamEntry containment matrices, ParamTrial sorting
//! parity, and ParamListStandard assignment/fillin walks for synthetic
//! prototypes.
//!
//! Address spaces are built directly with `AddrSpace::new` (no manager wiring
//! needed for the non-float-extension paths), and parameter entries with the
//! `ParamEntry::seed` builder seam that runs the real post-decode resolution
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
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    model.assign_map(&proto, &tf, &mut res, &mgr).unwrap();
    assert_eq!(res.len(), 2);
    // First int -> group 0 (offset 0x10), second -> group 1 (offset 0x20).
    assert_eq!(res[0].addr.get_offset(), 0x10);
    assert_eq!(res[1].addr.get_offset(), 0x20);
    assert!(res[0].type_.is_some());
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
fn func_proto_seam_methods_error() {
    let mut fp = FuncProto::new();
    assert!(fp.set_scope().is_err());
    assert!(fp.update_input_types().is_err());
    assert!(fp.update_output_types().is_err());
    assert!(fp.decode().is_err());
    let mgr = AddrSpaceManager::new();
    let mut model = ProtoModel::new(&mgr);
    assert!(model.decode().is_err());
}
