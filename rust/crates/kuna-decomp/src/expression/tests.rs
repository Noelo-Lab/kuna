//! Unit tests for `expression.rs`.
//!
//! These build small hand-wired Varnode/PcodeOp graphs on a real [`Funcdata`]
//! and pin the equality / term-order / boolean-match machinery against the C++
//! semantics (`expression.cc`).  The term-order comparator and the
//! functional-equality contingent-pair protocol are the load-bearing surfaces
//! (`RuleTermOrder` and the constant-collapse rules consume them), so they get
//! exhaustive coverage.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{Architecture, OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

use super::{
    boolean_match, functional_difference, functional_equality, functional_equality_level,
    pointer_equality, root_pointer, term_order, AdditiveEdge, BooleanExpressionMatch, BooleanMatch,
    PcodeOpNode, TermOrder,
};

// -----------------------------------------------------------------------------
// Fixture
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

fn ram_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn const_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space(0).unwrap())
}

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn opcode_flags(opc: OpCode) -> kuna_base::types::uint4 {
    // A pragmatic stand-in for the W6 op-info table: the property bits these tests
    // actually read (`branch`, `returns`, `booloutput`, `commutative`).
    match opc {
        OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        // Boolean-output comparison ops (also commutative for EQUAL/NOTEQUAL).
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => {
            pcodeop_flags::booloutput | pcodeop_flags::commutative
        }
        OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS => {
            pcodeop_flags::booloutput
        }
        // Commutative arithmetic/logical ops.
        OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR => pcodeop_flags::commutative,
        _ => 0,
    }
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// A free (non-written) varnode at a ram offset.  A free non-spacebase varnode
/// may have at most ONE descendant (the C++ `addDescend` invariant), so this is
/// only for single-reader / leaf terms; use [`mk_input`] for shared operands.
fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.vbank_mut().create(size, Address::new(ram, off), dt(size))
}

/// A formal *input* varnode (allowed multiple descendants), for operands read by
/// more than one op.  Mirrors `setInputVarnode`; with a distinct address the
/// `xref` collision callback never fires, so a no-op thunk suffices.
fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let vn = fd.vbank_mut().create(size, Address::new(ram, off), dt(size));
    let mut noop_thunk = |_b: &mut crate::varnode::VarnodeBank,
                          _o: VarnodeId,
                          _n: VarnodeId|
     -> kuna_base::error::KunaResult<()> {
        panic!("mk_input: distinct addresses must not collide")
    };
    fd.vbank_mut().set_input(vn, &mut noop_thunk).unwrap()
}

/// A constant varnode.
fn mk_const(fd: &mut Funcdata, val: u64, size: int4) -> VarnodeId {
    let c = const_space(fd);
    fd.vbank_mut().create(size, Address::new(c, val), dt(size))
}

/// Mark `op` as the definition of a fresh written varnode at `off`/`size` and
/// return it.  Mirrors `newVarnodeOut` without the W4 property seeding: create a
/// defined varnode in the bank (`create_def`, which sets the `written` flag) then
/// set it as the op's output.
///
/// `create_def` runs `xref`, which only invokes the `replace_reads` callback on a
/// *location collision*.  Every test output here has a distinct address, so no
/// collision occurs and the callback never fires — a no-op thunk that ignores the
/// op bank is sufficient (it sidesteps the missing `banks_mut()` split accessor).
fn mk_out(fd: &mut Funcdata, op: OpId, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum: seq };
    let vn = {
        let mut noop_thunk = |_b: &mut crate::varnode::VarnodeBank,
                              _o: VarnodeId,
                              _n: VarnodeId|
         -> kuna_base::error::KunaResult<()> {
            panic!("mk_out: distinct addresses must not collide (xref replace fired)")
        };
        fd.vbank_mut()
            .create_def(size, Address::new(ram, off), dt(size), def, &mut noop_thunk)
            .unwrap()
    };
    fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
    vn
}

/// Wire `reader` to read `vn` in `slot` (op input + descend link).
fn wire_read(fd: &mut Funcdata, reader: OpId, vn: VarnodeId, slot: int4) {
    fd.obank_mut().get_mut(reader).unwrap().set_input(Some(vn), slot);
    fd.vbank_mut().add_descend(vn, reader).unwrap();
}

// =============================================================================
// term_order (Varnode::termOrder) — the comparator behind RuleTermOrder
// =============================================================================

#[test]
fn term_order_constant_sorts_after_nonconstant() {
    let mut fd = build_fd();
    let k = mk_const(&mut fd, 5, 4);
    let v = mk_vn(&mut fd, 0x100, 4);
    // isConstant() && !op->isConstant() => 1 (constant sorts AFTER).
    assert_eq!(term_order(k, v, &fd), 1);
    // Symmetric: non-constant vs constant => -1.
    assert_eq!(term_order(v, k, &fd), -1);
}

#[test]
fn term_order_two_constants_equal() {
    let mut fd = build_fd();
    let k1 = mk_const(&mut fd, 5, 4);
    let k2 = mk_const(&mut fd, 9, 4);
    // Both constant: the `isConstant` branch returns 0 (no further ordering).
    assert_eq!(term_order(k1, k2, &fd), 0);
    assert_eq!(term_order(k2, k1, &fd), 0);
}

#[test]
fn term_order_nonconstants_by_address() {
    let mut fd = build_fd();
    let lo = mk_vn(&mut fd, 0x100, 4);
    let hi = mk_vn(&mut fd, 0x200, 4);
    // Both non-constant, neither INT_MULT: compare by storage address.
    assert_eq!(term_order(lo, hi, &fd), -1);
    assert_eq!(term_order(hi, lo, &fd), 1);
    // Same varnode => addresses equal => 0.
    assert_eq!(term_order(lo, lo, &fd), 0);
}

#[test]
fn term_order_peels_int_mult_by_constant() {
    // term = base * 3  vs  base : after peeling the INT_MULT-by-const coefficient
    // both reduce to `base`, so termOrder == 0 (same storage address).
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x100, 4); // shared underlying term
    let coeff = mk_const(&mut fd, 3, 4);
    let mult = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_MULT);
    let mult_out = mk_out(&mut fd, mult, 0x300, 4);
    wire_read(&mut fd, mult, base, 0);
    wire_read(&mut fd, mult, coeff, 1);

    // mult_out (= base*3) vs base => peel to base vs base => 0.
    assert_eq!(term_order(mult_out, base, &fd), 0);
    assert_eq!(term_order(base, mult_out, &fd), 0);
}

#[test]
fn term_order_no_peel_when_mult_coeff_nonconstant() {
    // base * other (other non-constant): the peel does NOT apply, so the term is
    // the INT_MULT output itself, compared by ITS address.
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x100, 4);
    let other = mk_vn(&mut fd, 0x108, 4);
    let mult = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_MULT);
    let mult_out = mk_out(&mut fd, mult, 0x300, 4); // address 0x300
    wire_read(&mut fd, mult, base, 0);
    wire_read(&mut fd, mult, other, 1);

    // mult_out(0x300) vs base(0x100): no peel (coeff non-const) => by address.
    assert_eq!(term_order(mult_out, base, &fd), 1); // 0x300 > 0x100
    assert_eq!(term_order(base, mult_out, &fd), -1);
}

// =============================================================================
// TermOrder::collect + sortTerms (the additive expression collector)
// =============================================================================

#[test]
fn termorder_collects_and_sorts_simple_sum() {
    // root = (a + b) where a@0x200, b@0x100 — collect yields two leaf terms; the
    // sort must order by termOrder (address), i.e. b before a.
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x200, 4);
    let b = mk_vn(&mut fd, 0x100, 4);
    let add = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let _add_out = mk_out(&mut fd, add, 0x300, 4);
    wire_read(&mut fd, add, a, 0);
    wire_read(&mut fd, add, b, 1);

    let mut to = TermOrder::new(add);
    to.collect(&fd);
    assert_eq!(to.get_size(), 2, "two leaf terms (a, b)");
    to.sort_terms(&fd);
    let sort = to.get_sort().clone();
    assert_eq!(sort.len(), 2);
    // The first sorted term must be b (lower address 0x100), then a (0x200).
    let first = to.term(sort[0]).get_varnode();
    let second = to.term(sort[1]).get_varnode();
    assert_eq!(first, b, "lower-address term sorts first");
    assert_eq!(second, a);
}

#[test]
fn termorder_descends_nested_add_chain() {
    // root = (a + (b + c)) where the inner add has a single use => collect
    // descends into it, yielding three terms a,b,c.
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x200, 4);
    let c = mk_vn(&mut fd, 0x300, 4);
    let inner = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let inner_out = mk_out(&mut fd, inner, 0x400, 4);
    wire_read(&mut fd, inner, b, 0);
    wire_read(&mut fd, inner, c, 1);
    let root = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let _root_out = mk_out(&mut fd, root, 0x500, 4);
    wire_read(&mut fd, root, a, 0);
    wire_read(&mut fd, root, inner_out, 1); // single use of inner_out

    let mut to = TermOrder::new(root);
    to.collect(&fd);
    assert_eq!(to.get_size(), 3, "descended the single-use inner add: a, b, c");
}

#[test]
fn termorder_stops_at_multiuse_subexpression() {
    // Same as above but inner_out has TWO uses => collect must NOT descend; it
    // becomes a single leaf term, giving 2 terms (a, inner_out).
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x200, 4);
    let c = mk_vn(&mut fd, 0x300, 4);
    let inner = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let inner_out = mk_out(&mut fd, inner, 0x400, 4);
    wire_read(&mut fd, inner, b, 0);
    wire_read(&mut fd, inner, c, 1);
    let root = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let _root_out = mk_out(&mut fd, root, 0x500, 4);
    wire_read(&mut fd, root, a, 0);
    wire_read(&mut fd, root, inner_out, 1);
    // Second use of inner_out (some other reader) defeats loneDescend.
    let other = mk_op(&mut fd, 1, 0x1008, OpCode::CPUI_COPY);
    wire_read(&mut fd, other, inner_out, 0);

    let mut to = TermOrder::new(root);
    to.collect(&fd);
    assert_eq!(to.get_size(), 2, "multi-use subexpression stays a single leaf term");
}

#[test]
fn additive_edge_carries_slot_and_multiplier() {
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x200, 4);
    let add = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let _o = mk_out(&mut fd, add, 0x300, 4);
    wire_read(&mut fd, add, a, 0);
    wire_read(&mut fd, add, b, 1);
    let edge = AdditiveEdge::new(add, 1, None, fd.obank());
    assert_eq!(edge.get_op(), add);
    assert_eq!(edge.get_slot(), 1);
    assert_eq!(edge.get_varnode(), b);
    assert!(edge.get_multiplier().is_none());
}

// =============================================================================
// functionalEqualityLevel0 / Level / functionalEquality
// =============================================================================

#[test]
fn functional_equality_identical_varnode() {
    let mut fd = build_fd();
    let v = mk_vn(&mut fd, 0x100, 4);
    // vn1 == vn2 => level0 returns 0 => equal.
    assert!(functional_equality(v, v, fd.vbank(), fd.obank()));
}

#[test]
fn functional_equality_equal_constants() {
    let mut fd = build_fd();
    let k1 = mk_const(&mut fd, 7, 4);
    let k2 = mk_const(&mut fd, 7, 4);
    assert!(functional_equality(k1, k2, fd.vbank(), fd.obank()));
    let k3 = mk_const(&mut fd, 8, 4);
    assert!(!functional_equality(k1, k3, fd.vbank(), fd.obank()));
}

#[test]
fn functional_equality_different_sizes_not_equal() {
    let mut fd = build_fd();
    let k1 = mk_const(&mut fd, 7, 4);
    let k2 = mk_const(&mut fd, 7, 8);
    // size mismatch => -1 in level0 => not equal.
    assert!(!functional_equality(k1, k2, fd.vbank(), fd.obank()));
}

#[test]
fn functional_equality_free_varnodes_not_equal() {
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4); // free
    let b = mk_vn(&mut fd, 0x200, 4); // free
    // Both free, distinct => level0 returns -1.
    assert!(!functional_equality(a, b, fd.vbank(), fd.obank()));
}

#[test]
fn functional_equality_level_contingent_single_input() {
    // Two COPY ops reading distinct (free) inputs: same opcode, 1 input.
    // level0(inputs) == 1 (contingent), so functionalEqualityLevel returns 1 and
    // res1[0]/res2[0] are the two input varnodes.
    let mut fd = build_fd();
    let in_a = mk_input(&mut fd, 0x100, 4); // non-free so level0 is contingent (1), not -1
    let in_b = mk_input(&mut fd, 0x200, 4);
    let cp1 = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let out1 = mk_out(&mut fd, cp1, 0x300, 4);
    wire_read(&mut fd, cp1, in_a, 0);
    let cp2 = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_COPY);
    let out2 = mk_out(&mut fd, cp2, 0x400, 4);
    wire_read(&mut fd, cp2, in_b, 0);

    let mut r1 = [out1, out1];
    let mut r2 = [out2, out2];
    let res = functional_equality_level(out1, out2, &mut r1, &mut r2, fd.vbank(), fd.obank());
    assert_eq!(res, 1, "single-input contingent match returns 1 pair");
    assert_eq!(r1[0], in_a);
    assert_eq!(r2[0], in_b);
}

#[test]
fn functional_equality_level_commutative_flip() {
    // op1 = a + b ; op2 = b + a (commutative INT_ADD with same operands swapped).
    // The straight pairing (a,b)/(b,a) is contingent both slots; the flip detects
    // a==a, b==b and returns 0 (provably equal).  Uses the SAME varnodes so the
    // == identity test in level0 fires.
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by both adds -> must allow 2 descendants
    let b = mk_input(&mut fd, 0x200, 4);
    let add1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let out1 = mk_out(&mut fd, add1, 0x300, 4);
    wire_read(&mut fd, add1, a, 0);
    wire_read(&mut fd, add1, b, 1);
    let add2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let out2 = mk_out(&mut fd, add2, 0x400, 4);
    wire_read(&mut fd, add2, b, 0); // swapped
    wire_read(&mut fd, add2, a, 1);

    assert!(
        functional_equality(out1, out2, fd.vbank(), fd.obank()),
        "commutative INT_ADD with swapped identical operands is functionally equal"
    );
}

#[test]
fn functional_equality_marker_op_not_equal() {
    // Two MULTIEQUALs: isMarker() => level returns -1 even with identical inputs.
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by both MULTIEQUALs
    let me1 = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_MULTIEQUAL);
    // give MULTIEQUAL the marker flag (setOpcode would; we set it explicitly).
    fd.obank_mut().get_mut(me1).unwrap().set_flag(pcodeop_flags::marker);
    let out1 = mk_out(&mut fd, me1, 0x300, 4);
    wire_read(&mut fd, me1, a, 0);
    let me2 = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_MULTIEQUAL);
    fd.obank_mut().get_mut(me2).unwrap().set_flag(pcodeop_flags::marker);
    let out2 = mk_out(&mut fd, me2, 0x400, 4);
    wire_read(&mut fd, me2, a, 0);

    assert!(!functional_equality(out1, out2, fd.vbank(), fd.obank()));
}

// =============================================================================
// functionalDifference
// =============================================================================

#[test]
fn functional_difference_distinct_constants_differ() {
    let mut fd = build_fd();
    let k1 = mk_const(&mut fd, 1, 4);
    let k2 = mk_const(&mut fd, 2, 4);
    // Both constant, different addr => true (different).
    assert!(functional_difference(k1, k2, 3, fd.vbank(), fd.obank()));
}

#[test]
fn functional_difference_same_varnode_not_different() {
    let mut fd = build_fd();
    let v = mk_vn(&mut fd, 0x100, 4);
    assert!(!functional_difference(v, v, 3, fd.vbank(), fd.obank()));
}

#[test]
fn functional_difference_two_free_inputs_maybe_same() {
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4); // free
    let b = mk_vn(&mut fd, 0x200, 4); // free
    // Not both written; not both constant; not both input; isFree => false (might be same).
    assert!(!functional_difference(a, b, 3, fd.vbank(), fd.obank()));
}

#[test]
fn functional_difference_different_opcodes_differ() {
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by cp and neg
    let cp = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let cp_out = mk_out(&mut fd, cp, 0x300, 4);
    wire_read(&mut fd, cp, a, 0);
    let neg = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_INT_2COMP);
    let neg_out = mk_out(&mut fd, neg, 0x400, 4);
    wire_read(&mut fd, neg, a, 0);
    // Different op codes => true.
    assert!(functional_difference(cp_out, neg_out, 3, fd.vbank(), fd.obank()));
}

#[test]
fn functional_difference_depth_exhausted_returns_true() {
    // Same opcode/arity but depth 0 => "different as far as we can tell" (true).
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by cp1 and cp2
    let cp1 = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let out1 = mk_out(&mut fd, cp1, 0x300, 4);
    wire_read(&mut fd, cp1, a, 0);
    let cp2 = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_COPY);
    let out2 = mk_out(&mut fd, cp2, 0x400, 4);
    wire_read(&mut fd, cp2, a, 0);
    assert!(functional_difference(out1, out2, 0, fd.vbank(), fd.obank()));
    // With depth 1 it recurses one level: inputs identical (a==a) => not different.
    assert!(!functional_difference(out1, out2, 1, fd.vbank(), fd.obank()));
}

// =============================================================================
// BooleanMatch::evaluate / BooleanExpressionMatch
// =============================================================================

#[test]
fn boolean_match_identical_varnode_same() {
    let mut fd = build_fd();
    let v = mk_vn(&mut fd, 0x100, 4);
    assert_eq!(BooleanMatch::evaluate(v, v, 1, fd.vbank(), fd.obank()), boolean_match::same);
}

#[test]
fn boolean_match_negate_is_complementary() {
    // vn2 = BOOL_NEGATE(vn1) => complementary.
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x100, 1);
    let neg = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_BOOL_NEGATE);
    let neg_out = mk_out(&mut fd, neg, 0x300, 1);
    wire_read(&mut fd, neg, base, 0);
    // base vs !base => complementary.
    assert_eq!(
        BooleanMatch::evaluate(base, neg_out, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary
    );
    // !base vs base => complementary (symmetric).
    assert_eq!(
        BooleanMatch::evaluate(neg_out, base, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary
    );
}

#[test]
fn boolean_match_unwritten_second_is_uncorrelated() {
    // vn1 written boolean, vn2 free non-boolean: the `!vn2->isWritten()` early
    // return yields uncorrelated.
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x108, 4);
    let less = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let less_out = mk_out(&mut fd, less, 0x300, 1);
    wire_read(&mut fd, less, a, 0);
    wire_read(&mut fd, less, b, 1);
    let free = mk_vn(&mut fd, 0x400, 1);
    assert_eq!(
        BooleanMatch::evaluate(less_out, free, 1, fd.vbank(), fd.obank()),
        boolean_match::uncorrelated
    );
}

#[test]
fn boolean_match_same_compare_op_same() {
    // Two identical INT_LESS(a,b) ops => same.
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by both INT_LESS
    let b = mk_input(&mut fd, 0x108, 4);
    let l1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let o1 = mk_out(&mut fd, l1, 0x300, 1);
    wire_read(&mut fd, l1, a, 0);
    wire_read(&mut fd, l1, b, 1);
    let l2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_LESS);
    let o2 = mk_out(&mut fd, l2, 0x400, 1);
    wire_read(&mut fd, l2, a, 0);
    wire_read(&mut fd, l2, b, 1);
    assert_eq!(BooleanMatch::evaluate(o1, o2, 1, fd.vbank(), fd.obank()), boolean_match::same);
}

#[test]
fn boolean_match_booleanflip_pair_complementary() {
    // INT_EQUAL(a,b) vs INT_NOTEQUAL(a,b): get_booleanflip(NOTEQUAL)=EQUAL, no
    // reorder, varnodes match in both slots => complementary.
    let mut fd = build_fd();
    let a = mk_input(&mut fd, 0x100, 4); // shared by INT_EQUAL and INT_NOTEQUAL
    let b = mk_input(&mut fd, 0x108, 4);
    let eq = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_EQUAL);
    let oeq = mk_out(&mut fd, eq, 0x300, 1);
    wire_read(&mut fd, eq, a, 0);
    wire_read(&mut fd, eq, b, 1);
    let ne = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_NOTEQUAL);
    let one = mk_out(&mut fd, ne, 0x400, 1);
    wire_read(&mut fd, ne, a, 0);
    wire_read(&mut fd, ne, b, 1);
    assert_eq!(
        BooleanMatch::evaluate(oeq, one, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary
    );
}

#[test]
fn boolean_match_sameop_complement_less_constants() {
    // INT_LESS(x, #9) vs INT_LESS(#8, x): the sameOpComplement corner — x<9 and
    // 8<x are complementary (val1+1==val2, 9 != 0).
    let mut fd = build_fd();
    let x = mk_input(&mut fd, 0x100, 4); // shared by both INT_LESS
    let nine = mk_const(&mut fd, 9, 4);
    let eight = mk_const(&mut fd, 8, 4);
    let l1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let o1 = mk_out(&mut fd, l1, 0x300, 1);
    wire_read(&mut fd, l1, x, 0); // x < 9
    wire_read(&mut fd, l1, nine, 1);
    let l2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_LESS);
    let o2 = mk_out(&mut fd, l2, 0x400, 1);
    wire_read(&mut fd, l2, eight, 0); // 8 < x
    wire_read(&mut fd, l2, x, 1);
    assert_eq!(
        BooleanMatch::evaluate(o1, o2, 1, fd.vbank(), fd.obank()),
        boolean_match::complementary
    );
}

#[test]
fn boolean_expression_match_verify_condition_same() {
    // Two CBRANCHs keying on the same INT_LESS(a,b) condition => verifyCondition
    // returns true with no flip.
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x108, 4);
    let less = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let cond = mk_out(&mut fd, less, 0x300, 1);
    wire_read(&mut fd, less, a, 0);
    wire_read(&mut fd, less, b, 1);
    // cbranch1: in(0)=target (unused), in(1)=cond.
    let cb1 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_CBRANCH);
    let tgt1 = mk_vn(&mut fd, 0x500, 8);
    wire_read(&mut fd, cb1, tgt1, 0);
    wire_read(&mut fd, cb1, cond, 1);
    let cb2 = mk_op(&mut fd, 2, 0x1008, OpCode::CPUI_CBRANCH);
    let tgt2 = mk_vn(&mut fd, 0x508, 8);
    wire_read(&mut fd, cb2, tgt2, 0);
    wire_read(&mut fd, cb2, cond, 1);

    let mut m = BooleanExpressionMatch::new();
    assert!(m.verify_condition(cb1, cb2, fd.vbank(), fd.obank()));
    assert!(!m.get_flip(), "same condition, no boolean flip");
    assert_eq!(m.get_multi_slot(), -1);
}

#[test]
fn boolean_expression_match_one_cbranch_flipped() {
    // Same condition but cb2 has the boolean_flip flag => verifyCondition true,
    // getFlip() true.
    let mut fd = build_fd();
    let a = mk_vn(&mut fd, 0x100, 4);
    let b = mk_vn(&mut fd, 0x108, 4);
    let less = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_LESS);
    let cond = mk_out(&mut fd, less, 0x300, 1);
    wire_read(&mut fd, less, a, 0);
    wire_read(&mut fd, less, b, 1);
    let cb1 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_CBRANCH);
    let tgt1 = mk_vn(&mut fd, 0x500, 8);
    wire_read(&mut fd, cb1, tgt1, 0);
    wire_read(&mut fd, cb1, cond, 1);
    let cb2 = mk_op(&mut fd, 2, 0x1008, OpCode::CPUI_CBRANCH);
    fd.obank_mut().get_mut(cb2).unwrap().set_flag(pcodeop_flags::boolean_flip);
    let tgt2 = mk_vn(&mut fd, 0x508, 8);
    wire_read(&mut fd, cb2, tgt2, 0);
    wire_read(&mut fd, cb2, cond, 1);

    let mut m = BooleanExpressionMatch::new();
    assert!(m.verify_condition(cb1, cb2, fd.vbank(), fd.obank()));
    assert!(m.get_flip(), "one CBRANCH boolean-flipped => anti-correlated");
}

// =============================================================================
// rootPointer / pointerEquality
// =============================================================================

#[test]
fn root_pointer_backtracks_ptrsub_and_intadd() {
    // ptr2 = INT_ADD(PTRSUB(base, #0x10), #0x8) => rootPointer reaches base with
    // accumulated offset 0x18.
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x100, 8);
    let c10 = mk_const(&mut fd, 0x10, 8);
    let ptrsub = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_PTRSUB);
    let ps_out = mk_out(&mut fd, ptrsub, 0x300, 8);
    wire_read(&mut fd, ptrsub, base, 0);
    wire_read(&mut fd, ptrsub, c10, 1);
    let c8 = mk_const(&mut fd, 0x8, 8);
    let add = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let add_out = mk_out(&mut fd, add, 0x400, 8);
    wire_read(&mut fd, add, ps_out, 0);
    wire_read(&mut fd, add, c8, 1);

    let mut off = 999;
    let root = root_pointer(add_out, &mut off, fd.vbank(), fd.obank());
    assert_eq!(root, base);
    assert_eq!(off, 0x18);
}

#[test]
fn root_pointer_stops_at_nonconstant_intadd() {
    // INT_ADD(base, var) with var non-constant: rootPointer breaks at the add,
    // returning the add output and offset 0.
    let mut fd = build_fd();
    let base = mk_vn(&mut fd, 0x100, 8);
    let var = mk_vn(&mut fd, 0x108, 8);
    let add = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let add_out = mk_out(&mut fd, add, 0x300, 8);
    wire_read(&mut fd, add, base, 0);
    wire_read(&mut fd, add, var, 1);
    let mut off = 999;
    let root = root_pointer(add_out, &mut off, fd.vbank(), fd.obank());
    assert_eq!(root, add_out, "non-constant addend stops the backtrack");
    assert_eq!(off, 0);
}

#[test]
fn pointer_equality_same_root_same_offset() {
    // p1 = PTRSUB(base,#0x10); p2 = PTRSUB(base,#0x10): same root, same offset.
    let mut fd = build_fd();
    let base = mk_input(&mut fd, 0x100, 8); // shared by both PTRSUBs
    let c10a = mk_const(&mut fd, 0x10, 8);
    let ps1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_PTRSUB);
    let ps1_out = mk_out(&mut fd, ps1, 0x300, 8);
    wire_read(&mut fd, ps1, base, 0);
    wire_read(&mut fd, ps1, c10a, 1);
    let c10b = mk_const(&mut fd, 0x10, 8);
    let ps2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_PTRSUB);
    let ps2_out = mk_out(&mut fd, ps2, 0x400, 8);
    wire_read(&mut fd, ps2, base, 0);
    wire_read(&mut fd, ps2, c10b, 1);
    assert!(pointer_equality(ps1_out, ps2_out, fd.vbank(), fd.obank()));
}

#[test]
fn pointer_equality_different_offset_not_equal() {
    let mut fd = build_fd();
    let base = mk_input(&mut fd, 0x100, 8); // shared by both PTRSUBs
    let c10 = mk_const(&mut fd, 0x10, 8);
    let ps1 = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_PTRSUB);
    let ps1_out = mk_out(&mut fd, ps1, 0x300, 8);
    wire_read(&mut fd, ps1, base, 0);
    wire_read(&mut fd, ps1, c10, 1);
    let c20 = mk_const(&mut fd, 0x20, 8);
    let ps2 = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_PTRSUB);
    let ps2_out = mk_out(&mut fd, ps2, 0x400, 8);
    wire_read(&mut fd, ps2, base, 0);
    wire_read(&mut fd, ps2, c20, 1);
    assert!(!pointer_equality(ps1_out, ps2_out, fd.vbank(), fd.obank()));
}

// =============================================================================
// PcodeOpNode comparator
// =============================================================================

#[test]
fn pcodeopnode_orders_by_seqtime_then_slot() {
    let mut fd = build_fd();
    // Two ops with distinct creation times (new_op bumps uniqid => SeqNum time).
    let op_a = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_INT_ADD);
    let op_b = mk_op(&mut fd, 2, 0x1004, OpCode::CPUI_INT_ADD);
    let na0 = PcodeOpNode::new(op_a, 0);
    let nb0 = PcodeOpNode::new(op_b, 0);
    // op_a created first => earlier SeqNum time => na0 < nb0.
    assert!(na0.less(&nb0, fd.obank()));
    assert!(!nb0.less(&na0, fd.obank()));
    // Same op, slot tiebreak.
    let na1 = PcodeOpNode::new(op_a, 1);
    assert!(na0.less(&na1, fd.obank()));
    assert!(!na1.less(&na0, fd.obank()));
    // Equal node: not less than itself.
    assert!(!na0.less(&na0, fd.obank()));
}
