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
