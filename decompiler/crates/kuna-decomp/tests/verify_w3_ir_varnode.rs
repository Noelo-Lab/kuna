//! Verifier adversarial tests for item `w3-ir-varnode`.
//!
//! These are INDEPENDENT of the porter's in-module unit tests. They drive only
//! the public `VarnodeBank` API so they exercise the real comparator + BTreeMap
//! insertion path (not the comparator helpers in isolation). Targets are the
//! hunt-list spots the review flagged as most fragile for this IR item:
//!
//!  - def-tree ordering is SEQNUM-dominated, not address-dominated
//!    (`VarnodeCompareDefLoc`, varnode.cc:60-79) — the def tree must reorder a
//!    written varnode away from its loc-tree position;
//!  - the `(f1-1)<(f2-1)` flag-class wrap: free sorts AFTER input/written even
//!    though raw `flags&(input|written)==0` is the smallest value;
//!  - `find_covered_input`'s address-wrap branch (`end == highest`,
//!    varnode.cc:1513) — the branch the porter's `input_finders` test does NOT
//!    cover (it only hits the `loc+s` branch);
//!  - `create_unique` offset accumulation + `clear` resetting `uniqid`;
//!  - xref dedup: a *true-equal* re-insert unifies, but two writtens that share
//!    (addr,size) yet differ in def seqnum stay DISTINCT (set acts like a
//!    multiset keyed on seqnum for writtens).

use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::context::{OpId, VarnodeId};
use kuna_decomp::varnode::{varnode_flags, DefOpInfo, VarnodeBank};

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

/// const(0), unique(1), ram(2, 8-byte), small(3, 1-byte so highest == 0xFF).
fn build_manager() -> AddrSpaceManager {
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
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    // 1-byte address size => highest offset == 0xFF (exercises wrap branch)
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "small",
        false,
        1,
        1,
        3,
        addrspace_flags::hasphysical,
        0,
        0,
    )))
    .unwrap();
    m.set_default_code_space(2).unwrap();
    m
}

fn space(m: &AddrSpaceManager, idx: i32) -> Rc<AddrSpace> {
    Rc::clone(m.get_space(idx).unwrap())
}

fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> kuna_base::error::KunaResult<()>
{
    |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> kuna_base::error::KunaResult<()> { Ok(()) }
}

/// Fabricate a distinct OpId + a SeqNum at (pc, uniq).
fn def_info(idc: &mut u32, pc: Address, uniq: u32) -> DefOpInfo {
    let mut sm: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
    for _ in 0..*idc {
        sm.insert(());
    }
    let id = sm.insert(());
    *idc += 1;
    DefOpInfo { id, seqnum: SeqNum::new(pc, uniq) }
}

/// def-tree order is dominated by the def SeqNum, NOT the storage address.
/// Two written varnodes: A at the HIGHER address but EARLIER seqnum, B at the
/// LOWER address but LATER seqnum. The loc tree must order them by address
/// (B before A); the def tree must order them by seqnum (A before B). This is
/// exactly the VarnodeCompareLocDef vs VarnodeCompareDefLoc split.
#[test]
fn def_tree_is_seqnum_dominated_not_address() {
    let m = build_manager();
    let mut bank = VarnodeBank::new(&m, 0).unwrap();
    let hi_addr = Address::new(space(&m, 2), 0x2000);
    let lo_addr = Address::new(space(&m, 2), 0x1000);

    let fa = bank.create(4, hi_addr.clone(), dt(4));
    let fb = bank.create(4, lo_addr.clone(), dt(4));
    let mut idc = 0u32;
    // A: higher address, earlier seqnum (uniq 1)
    let a = bank
        .set_def(fa, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 1), &mut no_replace())
        .unwrap();
    // B: lower address, later seqnum (uniq 9)
    let b = bank
        .set_def(fb, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 9), &mut no_replace())
        .unwrap();

    let loc_order: Vec<VarnodeId> = bank.iter_loc().collect();
    assert_eq!(loc_order, vec![b, a], "loc tree orders by address: lo(B) before hi(A)");

    let def_order: Vec<VarnodeId> = bank.iter_def().collect();
    assert_eq!(def_order, vec![a, b], "def tree orders by seqnum: uniq1(A) before uniq9(B)");

    // And the written-class def iterator agrees.
    let wr: Vec<VarnodeId> = bank.iter_def_flag(varnode_flags::written).collect();
    assert_eq!(wr, vec![a, b]);
}

/// The `(f1-1)<(f2-1)` wrap: input < written < free, at one address.  Insert in
/// a scrambled order and confirm the loc tree pulls them back into that order
/// (free LAST despite its raw flag class being numerically smallest).
#[test]
fn flag_class_wrap_free_sorts_last() {
    let m = build_manager();
    let mut bank = VarnodeBank::new(&m, 0).unwrap();
    let addr = Address::new(space(&m, 2), 0x3000);

    // create the free first, then written, then input — scrambled vs final order
    let free = bank.create(4, addr.clone(), dt(4));
    let wf = bank.create(4, addr.clone(), dt(4));
    let mut idc = 0u32;
    let written = bank
        .set_def(wf, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 2), &mut no_replace())
        .unwrap();
    let inf = bank.create(4, addr.clone(), dt(4));
    let input = bank.set_input(inf, &mut no_replace()).unwrap();

    let order: Vec<VarnodeId> = bank.iter_loc().collect();
    assert_eq!(order, vec![input, written, free], "input < written < free (wrap)");
}

/// `find_covered_input` exercising the ADDRESS-WRAP branch: a 1-byte space has
/// highest offset 0xFF; querying size 1 at offset 0xFF makes `end == highest`,
/// so the C++ takes the `endDef(input, Address(space, highest))` branch rather
/// than `beginDef(input, loc+s)` (which would wrap to 0x00). The covered input
/// at 0xFF must still be found.
#[test]
fn find_covered_input_address_wrap_branch() {
    let m = build_manager();
    let mut bank = VarnodeBank::new(&m, 0).unwrap();
    let small = space(&m, 3);
    assert_eq!(small.get_highest(), 0xFF, "1-byte space tops out at 0xFF");

    // input occupying the very last byte 0xFF
    let top_addr = Address::new(Rc::clone(&small), 0xFF);
    let top_free = bank.create(1, top_addr.clone(), dt(1));
    let top = bank.set_input(top_free, &mut no_replace()).unwrap();
    // a second, lower input so the def tree has a real predecessor to bound on
    let lo_addr = Address::new(Rc::clone(&small), 0x10);
    let lo_free = bank.create(1, lo_addr.clone(), dt(1));
    let _lo = bank.set_input(lo_free, &mut no_replace()).unwrap();

    // size-1 query at 0xFF: end = 0xFF + 1 - 1 = 0xFF == highest -> wrap branch
    let found = bank.find_covered_input(1, &top_addr).unwrap();
    assert_eq!(found, Some(top), "covered input at the wrap boundary is found");

    // sanity: a query that does NOT touch the boundary still works (non-wrap)
    let found_lo = bank.find_covered_input(1, &lo_addr).unwrap();
    assert_eq!(found_lo, Some(_lo));
}

/// `create_unique` accumulates offsets by size, and `clear` resets `uniqid` to
/// the base so a fresh round restarts at the analysis base. Also confirms the
/// create_index counter resets.
#[test]
fn create_unique_accumulates_and_clear_resets() {
    let m = build_manager();
    let base: u32 = 0x4000;
    let mut bank = VarnodeBank::new(&m, base).unwrap();
    let u0 = bank.create_unique(8, dt(8));
    let u1 = bank.create_unique(4, dt(4));
    assert_eq!(bank.get(u0).unwrap().get_offset(), base as u64);
    assert_eq!(bank.get(u1).unwrap().get_offset(), base as u64 + 8);
    assert_eq!(bank.get(u1).unwrap().get_create_index(), 1);

    bank.clear();
    assert_eq!(bank.num_varnodes(), 0);
    assert_eq!(bank.get_create_index(), 0, "create_index resets on clear");
    let u2 = bank.create_unique(2, dt(2));
    assert_eq!(bank.get(u2).unwrap().get_offset(), base as u64, "uniqid resets to base on clear");
    assert_eq!(bank.get(u2).unwrap().get_create_index(), 0);
}

/// xref dedup is keyed on the FULL comparator: a true-equal re-insert unifies,
/// but two writtens sharing (addr,size) with DIFFERENT def seqnums are distinct
/// (the def-seqnum tie-break keeps them apart — multiset-like for writtens).
#[test]
fn xref_writtens_distinct_by_seqnum_but_equal_inputs_unify() {
    let m = build_manager();
    let mut bank = VarnodeBank::new(&m, 0).unwrap();
    let addr = Address::new(space(&m, 2), 0x5000);

    // two writtens, same (addr,size), DIFFERENT seqnum -> both survive
    let f1 = bank.create(4, addr.clone(), dt(4));
    let f2 = bank.create(4, addr.clone(), dt(4));
    let mut idc = 0u32;
    let w1 = bank
        .set_def(f1, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 1), &mut no_replace())
        .unwrap();
    let w2 = bank
        .set_def(f2, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 2), &mut no_replace())
        .unwrap();
    assert_ne!(w1, w2);
    assert_eq!(bank.num_varnodes(), 2, "writtens with distinct seqnum are distinct entries");

    // two inputs, same (addr,size) -> the second unifies onto the first and the
    // replace callback fires once with (old=second, new=first).
    let g1 = bank.create(2, Address::new(space(&m, 2), 0x6000), dt(2));
    let i1 = bank.set_input(g1, &mut no_replace()).unwrap();
    let g2 = bank.create(2, Address::new(space(&m, 2), 0x6000), dt(2));
    let mut seen: Vec<(VarnodeId, VarnodeId)> = Vec::new();
    let mut rec = |_: &mut VarnodeBank, old: VarnodeId, new: VarnodeId| -> kuna_base::error::KunaResult<()> {
        seen.push((old, new));
        Ok(())
    };
    let i2 = bank.set_input(g2, &mut rec).unwrap();
    assert_eq!(i2, i1, "equal inputs unify");
    assert_eq!(seen, vec![(g2, i1)]);
    assert!(bank.get(g2).is_none(), "the unified-away varnode is removed from the arena");
}

/// descend list is order-preserving push_back and erase-by-value mid-list,
/// matching the C++ `list<PcodeOp*>` + `eraseDescend` find-then-erase. Verifies
/// erase of the FIRST and LAST elements too (boundary positions).  Uses a
/// WRITTEN varnode (not free) so multiple descendants are allowed without the
/// spacebase exemption (which has no public setter).
#[test]
fn descend_push_back_and_boundary_erase() {
    let m = build_manager();
    let mut bank = VarnodeBank::new(&m, 0).unwrap();
    let free = bank.create(4, Address::new(space(&m, 2), 0x7000), dt(4));
    let mut idc = 0u32;
    let v = bank
        .set_def(free, def_info(&mut idc, Address::new(space(&m, 2), 0x10), 1), &mut no_replace())
        .unwrap();

    let mut sm: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
    let o0 = sm.insert(());
    let o1 = sm.insert(());
    let o2 = sm.insert(());
    bank.add_descend(v, o0).unwrap();
    bank.add_descend(v, o1).unwrap();
    bank.add_descend(v, o2).unwrap();
    let got: Vec<OpId> = bank.get(v).unwrap().descend_iter().collect();
    assert_eq!(got, vec![o0, o1, o2]);

    // erase first
    bank.erase_descend(v, o0);
    assert_eq!(bank.get(v).unwrap().descend_iter().collect::<Vec<_>>(), vec![o1, o2]);
    // erase last
    bank.erase_descend(v, o2);
    assert_eq!(bank.get(v).unwrap().descend_iter().collect::<Vec<_>>(), vec![o1]);
}
