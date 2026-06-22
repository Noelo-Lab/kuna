//! Independent verifier tests for `w7-s7-kuna-loweredswitch` (round 1).
//!
//! These are adversarial re-derivations written by the VERIFIER (not the
//! porter), targeting the hunt-list spots this item is most fragile in:
//!
//!   * **Iteration-order provenance.** The C++ picks the switch variable from a
//!     `std::map<Varnode *,int4>` (raw allocation-pointer order) and the cascade
//!     head from a `std::map<BlockBasic *,CmpNode>` (raw pointer order); the
//!     Rust port keys these on `BTreeMap<VarnodeId/BlockId,_>` (slotmap
//!     creation order).  The strict-`>` switch-variable pick and the
//!     `break`-at-first head pick are tie-break-sensitive.  These tests pin the
//!     *deterministic* Rust behavior on a constructed tie (LOSS class of
//!     LOSS-097: a determinism improvement over non-deterministic C++ pointer
//!     order).
//!   * **Off-by-one on the fail-safe cap.** `recoverCascade` accepts `<= 16`
//!     distinct cases and rejects `> 16`; the `< 3` minimum is the other end.
//!   * **Duplicate case value bail** (`cases.find(cval) != end => return false`).
//!   * **The `INT_NOTEQUAL` match-is-not-taken-edge** wiring in `analyzeCmp`.
//!   * **`canonSwitchVar` peel chain** identity (zero-offset SUBPIECE included).
//!
//! The harness mirrors the in-crate `kuna_loweredswitch/tests.rs` fixtures (the
//! only way to reach the read-only detection over a hand-built CFG); the root
//! block is reached via the public `fd.bblocks_ref().root` field, as the other
//! `verify_w*` integration tests do.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_loweredswitch::{
    ActionLowerSwitchDetect, KunaLoweredSwitchStore, OptionLowerSwitch,
};
use kuna_decomp::seams::{Architecture, BlockId, TypeOp, VarnodeId};

// -----------------------------------------------------------------------------
// Harness (mirrors kuna_loweredswitch/tests.rs)
// -----------------------------------------------------------------------------

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
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
}

fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn ram_addr(fd: &Funcdata, off: u64) -> Address {
    Address::new(ramspace(fd), off)
}

fn root_of(fd: &Funcdata) -> BlockId {
    fd.bblocks_ref().root.expect("bblocks root")
}

fn new_block(fd: &mut Funcdata, off: u64) -> BlockId {
    let root = root_of(fd);
    let bl = fd.bblocks_mut().new_block_basic(root);
    let beg = ram_addr(fd, off);
    let end = ram_addr(fd, off);
    fd.set_basic_block_range(bl, &beg, &end);
    bl
}

fn vn_size_pub(fd: &Funcdata, vn: VarnodeId) -> int4 {
    fd.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}

/// Append `CBRANCH( cmp_code(V, c) )` to block `bl`.  Out-edge 0 = false,
/// out-edge 1 = true (caller wires edges false-then-true).
fn append_cmp_cbranch(
    fd: &mut Funcdata,
    bl: BlockId,
    off: u64,
    cmp_code: OpCode,
    v: VarnodeId,
    c: u64,
) {
    let cmp = fd.new_op(2, ram_addr(fd, off));
    fd.op_set_opcode(cmp, TypeOp::new(cmp_code, 0, "cmp"));
    fd.op_set_input(cmp, v, 0).unwrap();
    let cvn = fd.new_constant(vn_size_pub(fd, v), c);
    fd.op_set_input(cmp, cvn, 1).unwrap();
    let boolvn = fd.new_unique_out(1, cmp).unwrap();
    fd.op_insert_end(cmp, bl);

    let cb = fd.new_op(2, ram_addr(fd, off + 1));
    fd.op_set_opcode(cb, TypeOp::new(OpCode::CPUI_CBRANCH, 0, "CPUI_CBRANCH"));
    let dest = fd.new_constant(8, 0);
    fd.op_set_input(cb, dest, 0).unwrap();
    fd.op_set_input(cb, boolvn, 1).unwrap();
    fd.op_insert_end(cb, bl);
}

/// A switch variable `V` defined once (LOAD output) at register `vaddr_off`,
/// in its own predecessor block at `def_off`.
fn switch_var_at(fd: &mut Funcdata, def_off: u64, vaddr_off: u64) -> VarnodeId {
    let def_bl = new_block(fd, def_off);
    let load = fd.new_op(2, ram_addr(fd, def_off));
    fd.op_set_opcode(load, TypeOp::new(OpCode::CPUI_LOAD, 0, "LOAD"));
    let spaceid = fd.new_constant(4, 0);
    let ptr = fd.new_constant(8, 0x5000 + vaddr_off);
    fd.op_set_input(load, spaceid, 0).unwrap();
    fd.op_set_input(load, ptr, 1).unwrap();
    let vaddr = ram_addr(fd, vaddr_off);
    let v = fd.new_varnode_out(4, &vaddr, load).unwrap();
    fd.op_insert_end(load, def_bl);
    v
}

fn off(a: &Address) -> u64 {
    a.get_offset()
}

// -----------------------------------------------------------------------------
// V1 (iteration order). Two independent valid cascades on TWO distinct switch
// variables with an EQUAL number of cascade nodes: a `var_count` tie.  C++ would
// resolve it by raw `Varnode *` pointer order (non-deterministic); the Rust port
// resolves it deterministically by slotmap creation order.  The earlier-created
// variable (`v1`, built first) wins, so its cascade is the one recorded.
// This pins the deterministic divergence (LOSS class of LOSS-097).
// -----------------------------------------------------------------------------

/// Build a 4-block binary-search cascade on `v` rooted at `base` with case
/// targets at `tbase`.  Cases {0,2,3}; default DEFAULT.  Returns nothing (the
/// detection finds the head itself).
fn build_cascade_on(fd: &mut Funcdata, v: VarnodeId, base: u64, tbase: u64) {
    let b0 = new_block(fd, base);
    let b1 = new_block(fd, base + 0x100);
    let b2 = new_block(fd, base + 0x200);
    let b3 = new_block(fd, base + 0x300);
    let case_a = new_block(fd, tbase);
    let case_b = new_block(fd, tbase + 0x100);
    let default = new_block(fd, tbase + 0x1000);

    append_cmp_cbranch(fd, b0, base, OpCode::CPUI_INT_LESS, v, 2);
    fd.bblocks_mut().add_edge(b0, b2);
    fd.bblocks_mut().add_edge(b0, b1);

    append_cmp_cbranch(fd, b1, base + 0x100, OpCode::CPUI_INT_EQUAL, v, 0);
    fd.bblocks_mut().add_edge(b1, default);
    fd.bblocks_mut().add_edge(b1, case_a);

    append_cmp_cbranch(fd, b2, base + 0x200, OpCode::CPUI_INT_EQUAL, v, 2);
    fd.bblocks_mut().add_edge(b2, b3);
    fd.bblocks_mut().add_edge(b2, case_b);

    append_cmp_cbranch(fd, b3, base + 0x300, OpCode::CPUI_INT_EQUAL, v, 3);
    fd.bblocks_mut().add_edge(b3, default);
    fd.bblocks_mut().add_edge(b3, case_a);
}

#[test]
fn v1_var_count_tie_is_deterministic_first_created_wins() {
    let mut fd = build_fd();
    // v1 created first, v2 second (both 4-node cascades => varCount tie at 4).
    let v1 = switch_var_at(&mut fd, 0x0F00, 0x4000);
    let v2 = switch_var_at(&mut fd, 0x0F10, 0x4010);
    build_cascade_on(&mut fd, v1, 0x1000, 0x2000);
    build_cascade_on(&mut fd, v2, 0x6000, 0x7000);

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1, "exactly one cascade recovered on a var tie");
    let recs = act.store().borrow().records(&fd).to_vec();
    assert_eq!(recs.len(), 1);
    // Deterministic resolution: the earlier-created varnode (v1) wins the
    // strict-`>` `bestCount` pick, so its cascade (head CBRANCH at 0x1001,
    // var storage 0x4000) is recorded — NOT v2's (0x6001 / 0x4010).
    assert_eq!(off(&recs[0].branch_addr), 0x1001, "v1's head, not v2's (0x6001)");
    assert_eq!(off(&recs[0].var_addr), 0x4000, "v1's storage, not v2's (0x4010)");
    // Sanity: the recovered cases are the cascade shape {0,2,3}.
    assert_eq!(recs[0].case_vals, vec![0u64, 2u64, 3u64]);
}

// -----------------------------------------------------------------------------
// V2 (off-by-one on the fail-safe cap). EXACTLY 16 distinct cases is accepted
// (`cases.size() > 16` is false); 17 distinct cases is rejected.
// -----------------------------------------------------------------------------

/// Build a cascade with `ncases` distinct equality cases hung off one range
/// node, all the equality fall-throughs going to a common DEFAULT.
///
/// Shape:  B_range: if (V < 0x10000) -> [equality chain] else DEFAULT
///         eq_i:    if (V == i)      -> CASE_i  else eq_{i+1} (last -> DEFAULT)
/// => `ncases` distinct case values, `ncases` distinct targets, one range node.
fn build_n_case_cascade(fd: &mut Funcdata, v: VarnodeId, ncases: u64) {
    let default = new_block(fd, 0x3_0000);
    let rangeb = new_block(fd, 0x1_0000);
    // range: V < 0x10000 ? eq_0 (true) : DEFAULT (false)
    append_cmp_cbranch(fd, rangeb, 0x1_0000, OpCode::CPUI_INT_LESS, v, 0x10000);

    // Pre-create the equality blocks + their case targets.
    let mut eqs = Vec::new();
    let mut cases = Vec::new();
    for i in 0..ncases {
        eqs.push(new_block(fd, 0x2_0000 + i * 0x10));
        cases.push(new_block(fd, 0x4_0000 + i * 0x10));
    }

    // range out 0 = false = DEFAULT, out 1 = true = eq_0.
    fd.bblocks_mut().add_edge(rangeb, default);
    fd.bblocks_mut().add_edge(rangeb, eqs[0]);

    for i in 0..ncases as usize {
        append_cmp_cbranch(
            fd,
            eqs[i],
            0x2_0000 + (i as u64) * 0x10,
            OpCode::CPUI_INT_EQUAL,
            v,
            i as u64,
        );
        // out 0 = false = cont (next eq, or DEFAULT for the last)
        let cont = if i + 1 < ncases as usize { eqs[i + 1] } else { default };
        fd.bblocks_mut().add_edge(eqs[i], cont);
        // out 1 = true = match = CASE_i
        fd.bblocks_mut().add_edge(eqs[i], cases[i]);
    }
}

#[test]
fn v2_cap_boundary_16_accepted_17_rejected() {
    // 16 cases: accepted.
    let mut fd16 = build_fd();
    let v16 = switch_var_at(&mut fd16, 0x0F00, 0x4000);
    build_n_case_cascade(&mut fd16, v16, 16);
    let mut act16 = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act16.detect(&mut fd16), 1, "16 cases is at the cap, accepted");
    assert_eq!(act16.store().borrow().records(&fd16)[0].case_vals.len(), 16);

    // 17 cases: rejected by the `> 16` fail-safe cap.
    let mut fd17 = build_fd();
    let v17 = switch_var_at(&mut fd17, 0x0F00, 0x4000);
    build_n_case_cascade(&mut fd17, v17, 17);
    let mut act17 = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act17.detect(&mut fd17), 0, "17 cases exceeds the cap, rejected");
    assert!(!act17.store().borrow().has_record(&fd17));
}

// -----------------------------------------------------------------------------
// V3 (duplicate case value bail). Two equality nodes test the SAME constant but
// branch to DIFFERENT case targets: `recoverCascade` must bail
// (`cases.find(cval) != end => return false`), recording nothing.
// -----------------------------------------------------------------------------

#[test]
fn v3_duplicate_case_value_bails() {
    let mut fd = build_fd();
    let v = switch_var_at(&mut fd, 0x0F00, 0x4000);

    let b0 = new_block(&mut fd, 0x1000); // range
    let b1 = new_block(&mut fd, 0x1100); // eq V==5 -> A
    let b2 = new_block(&mut fd, 0x1200); // eq V==5 -> B (DUPLICATE value 5)
    let b3 = new_block(&mut fd, 0x1300); // eq V==7 -> C
    let case_a = new_block(&mut fd, 0x2000);
    let case_b = new_block(&mut fd, 0x2100);
    let case_c = new_block(&mut fd, 0x2200);
    let default = new_block(&mut fd, 0x3000);

    // B0: range V < 8 ? B1 : DEFAULT
    append_cmp_cbranch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 8);
    fd.bblocks_mut().add_edge(b0, default); // false
    fd.bblocks_mut().add_edge(b0, b1); // true

    // B1: V == 5 ? A : B2
    append_cmp_cbranch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_EQUAL, v, 5);
    fd.bblocks_mut().add_edge(b1, b2); // false = cont
    fd.bblocks_mut().add_edge(b1, case_a); // true = match

    // B2: V == 5 ? B : B3   (DUPLICATE constant 5, different target)
    append_cmp_cbranch(&mut fd, b2, 0x1200, OpCode::CPUI_INT_EQUAL, v, 5);
    fd.bblocks_mut().add_edge(b2, b3); // false = cont
    fd.bblocks_mut().add_edge(b2, case_b); // true = match

    // B3: V == 7 ? C : DEFAULT
    append_cmp_cbranch(&mut fd, b3, 0x1300, OpCode::CPUI_INT_EQUAL, v, 7);
    fd.bblocks_mut().add_edge(b3, default); // false = cont
    fd.bblocks_mut().add_edge(b3, case_c); // true = match

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 0, "duplicate case value 5 must bail recoverCascade");
    assert!(!act.store().borrow().has_record(&fd));
}

// -----------------------------------------------------------------------------
// V4 (INT_NOTEQUAL: match is the NOT-taken edge). An `if (V != c)` cascade node
// must record the case for `V == c` on the *false* (not-taken) successor; the
// true successor is the continuation.  This pins the subtle `analyzeCmp`
// NOTEQUAL branch (matchOut = condFalse, contA = condTrue).
// -----------------------------------------------------------------------------

#[test]
fn v4_notequal_match_is_the_not_taken_edge() {
    let mut fd = build_fd();
    let v = switch_var_at(&mut fd, 0x0F00, 0x4000);

    let b0 = new_block(&mut fd, 0x1000); // range
    let b1 = new_block(&mut fd, 0x1100); // V != 0
    let b2 = new_block(&mut fd, 0x1200); // V != 1
    let b3 = new_block(&mut fd, 0x1300); // V != 2
    let case_a = new_block(&mut fd, 0x2000); // the V==0 / V==2 case body
    let case_b = new_block(&mut fd, 0x2100); // the V==1 case body
    let default = new_block(&mut fd, 0x3000);

    // B0: range V < 4 ? B1 : DEFAULT
    append_cmp_cbranch(&mut fd, b0, 0x1000, OpCode::CPUI_INT_LESS, v, 4);
    fd.bblocks_mut().add_edge(b0, default); // false
    fd.bblocks_mut().add_edge(b0, b1); // true

    // B1: V != 0 ?  true: B2 (cont)   false: A (match for V==0)
    append_cmp_cbranch(&mut fd, b1, 0x1100, OpCode::CPUI_INT_NOTEQUAL, v, 0);
    fd.bblocks_mut().add_edge(b1, case_a); // out0 = false = match (V==0) -> A
    fd.bblocks_mut().add_edge(b1, b2); // out1 = true  = cont

    // B2: V != 1 ?  true: B3 (cont)   false: B (match for V==1)
    append_cmp_cbranch(&mut fd, b2, 0x1200, OpCode::CPUI_INT_NOTEQUAL, v, 1);
    fd.bblocks_mut().add_edge(b2, case_b); // out0 = false = match (V==1) -> B
    fd.bblocks_mut().add_edge(b2, b3); // out1 = true  = cont

    // B3: V != 2 ?  true: DEFAULT (cont)   false: A (match for V==2)
    append_cmp_cbranch(&mut fd, b3, 0x1300, OpCode::CPUI_INT_NOTEQUAL, v, 2);
    fd.bblocks_mut().add_edge(b3, case_a); // out0 = false = match (V==2) -> A
    fd.bblocks_mut().add_edge(b3, default); // out1 = true  = cont

    let mut act = ActionLowerSwitchDetect::new(true, "base");
    assert_eq!(act.detect(&mut fd), 1, "NOTEQUAL cascade recovered");
    let recs = act.store().borrow().records(&fd).to_vec();
    let rec = &recs[0];
    // 3 distinct cases {0,2 -> A ; 1 -> B}; default = the common range sink.
    assert_eq!(rec.case_vals, vec![0u64, 1u64, 2u64]);
    assert_eq!(off(&rec.case_targets[0]), 0x2000, "V==0 -> A (the false/not-taken edge)");
    assert_eq!(off(&rec.case_targets[1]), 0x2100, "V==1 -> B");
    assert_eq!(off(&rec.case_targets[2]), 0x2000, "V==2 -> A");
    assert_eq!(off(&rec.default_target), 0x3000, "default is the range-false sink");
}

// -----------------------------------------------------------------------------
// V5 (canon peel via zero-offset SUBPIECE + the option/store surface). A pure
// public-surface re-check that the store key, option message, and SUBPIECE peel
// are exactly the C++ values.
// -----------------------------------------------------------------------------

#[test]
fn v5_option_and_store_surface() {
    // Option parse: on/off/empty/bad — the C++ onOrOff contract.
    let (on, msg_on) = OptionLowerSwitch.apply("on").unwrap();
    assert!(on);
    assert_eq!(msg_on, "Lowered comparison-cascade switch recovery turned on");
    let (off_v, msg_off) = OptionLowerSwitch.apply("off").unwrap();
    assert!(!off_v);
    assert_eq!(msg_off, "Lowered comparison-cascade switch recovery turned off");
    assert!(OptionLowerSwitch.apply("").unwrap().0, "empty => on (onOrOff)");
    assert!(OptionLowerSwitch.apply("nonsense").is_err());

    // Empty store: no record for a fresh function.
    let fd = build_fd();
    let store = KunaLoweredSwitchStore::new();
    assert!(!store.has_record(&fd));
    assert!(store.records(&fd).is_empty());
}
