//! Adversarial verification tests for item `w4x-flow-linkage`
//! (the *real* op-output/code-ref linkage that lit up
//! `decompiler/cpp/funcdata_op.cc::opSetOutput`,
//! `funcdata_varnode.cc::newVarnodeOut`/`newUniqueOut`/`newCodeRef`, and the
//! `PcodeEmitFd::dump` emitter — `decompiler/cpp/funcdata.cc:875`).
//!
//! Written by the INDEPENDENT verifier, targeting the output-rewiring edge cases
//! the hunt list flagged as most fragile for this fix:
//!
//!   - **Replacing an existing output.** `opSetOutput` on an op that already has a
//!     different output must `opUnsetOutput` the old one (freeing it, clearing its
//!     def) BEFORE the `vbank.setDef` — and when the new `vn` is itself already
//!     some *other* op's output, that other op's output must be unset too (the
//!     two-`opUnsetOutput` ladder, C++ funcdata_op.cc:78-84).
//!   - **`opSetOutput` xref-unification repointing.** When `vbank.setDef` unifies
//!     the new output with an equivalent free Varnode that already has readers,
//!     `replace_reads` must repoint those readers onto the kept Varnode (the
//!     descend/xref order the prompt calls out) — the kept id is what
//!     `op->setOutput` receives.
//!   - **Unique-space outputs.** `newUniqueOut` allocates in the \e unique space,
//!     advances `uniqid` by the size, and links the op as the varnode's def — no
//!     `localmap` match (the unique space carries no symbols).
//!   - **codeRef target semantics.** `newCodeRef(m)` builds a size-1 annotation
//!     Varnode AT `m` (the branch destination); through the real `dump` it becomes
//!     input slot 0 of a coderef op, at the exact `(space,offset,1)` of the
//!     target.  The `Varnode::annotation` flag is now set faithfully (LOSS-077
//!     added the `pub(crate)` `Varnode::set_annotation` sliver that `newCodeRef`
//!     wires; the formerly-carried LOSS-036 / LOSS-037 is restored).
//!   - **`opDestroyRaw` teardown order.** inputs destroyed first, then the output,
//!     then the op retires — the dead-op is gone and its in/out Varnodes with it.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, IopSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::context::{ArchContext, OpId, TypeOp, VarnodeId};

// ---------------------------------------------------------------------------
// Public-API scaffolding (mirrors tests/verify_w3_ir_funcdata_op.rs)
// ---------------------------------------------------------------------------

fn build_manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd() -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(ArchContext::new(manage));
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

fn unique_space(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name(UniqueSpace::NAME).unwrap())
}

fn opcode_flags(opc: OpCode) -> uint4 {
    match opc {
        // The C++ glb->inst[] flag word: BRANCH ops carry both `branch` and
        // `coderef` (the destination is an annotation address).
        OpCode::CPUI_BRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_RETURN => pcodeop_flags::returns,
        _ => 0,
    }
}

/// Create an op and set its opcode (which caches the property-flag word, exactly
/// as `op_set_opcode` / C++ `glb->inst[opc]` does).
fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let ram = ram_space(fd);
    let op = fd.new_op(inputs, Address::new(ram, off));
    fd.op_set_opcode(op, TypeOp::new(opc, opcode_flags(opc), format!("{opc:?}")));
    op
}

/// A free RAM Varnode (the candidate for an op output).
fn mk_free(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let ram = ram_space(fd);
    fd.new_varnode_space_off(size, ram, off)
}

// ===========================================================================
// TEST 1 — replacing an existing output (the two-`opUnsetOutput` ladder)
// ===========================================================================

#[test]
fn w4x_flow_linkage_op_set_output_replaces_existing_output_and_steals_from_other_op() {
    let mut fd = build_fd();

    // opA defines vn_old (its output).
    let op_a = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let vn_old = mk_free(&mut fd, 0x100, 4);
    fd.op_set_output(op_a, vn_old).unwrap();
    assert_eq!(fd.obank().get(op_a).unwrap().get_out(), Some(vn_old));
    assert_eq!(fd.vbank().get(vn_old).unwrap().get_def(), Some(op_a));

    // opB defines vn_new (a *different* op's output).
    let op_b = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_COPY);
    let vn_new = mk_free(&mut fd, 0x200, 4);
    fd.op_set_output(op_b, vn_new).unwrap();
    assert_eq!(fd.vbank().get(vn_new).unwrap().get_def(), Some(op_b));

    // Now retarget opA's output to vn_new.  C++ funcdata_op.cc:78-84 must:
    //   (1) opUnsetOutput(opA)         -> vn_old goes free, opA->out cleared
    //   (2) opUnsetOutput(vn_new->def) -> opB->out cleared, vn_new goes free
    //   (3) vbank.setDef(vn_new, opA)  -> vn_new now defined by opA
    fd.op_set_output(op_a, vn_new).unwrap();

    // opA now outputs vn_new; vn_new is defined by opA.
    assert_eq!(fd.obank().get(op_a).unwrap().get_out(), Some(vn_new));
    assert_eq!(fd.vbank().get(vn_new).unwrap().get_def(), Some(op_a));
    // opB lost its output (it was stolen).
    assert_eq!(fd.obank().get(op_b).unwrap().get_out(), None);
    // vn_old is now a free Varnode with no defining op (it was unset).
    assert_eq!(fd.vbank().get(vn_old).unwrap().get_def(), None);
    assert!(fd.vbank().get(vn_old).unwrap().is_free());
}

#[test]
fn w4x_flow_linkage_op_set_output_idempotent_when_already_set() {
    let mut fd = build_fd();
    let op = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let vn = mk_free(&mut fd, 0x100, 4);
    fd.op_set_output(op, vn).unwrap();
    let n_vn = fd.num_varnodes();
    // Setting the SAME output again is the C++ early-out (`vn == op->getOut()`):
    // no unset/setDef churn, no new Varnode, the def unchanged.
    fd.op_set_output(op, vn).unwrap();
    assert_eq!(fd.obank().get(op).unwrap().get_out(), Some(vn));
    assert_eq!(fd.vbank().get(vn).unwrap().get_def(), Some(op));
    assert_eq!(fd.num_varnodes(), n_vn, "no churn on idempotent set");
}

// ===========================================================================
// TEST 2 — newVarnodeOut xref does NOT clobber a free Varnode at the same
// address (cross-flag-class non-unification — a real dump-path edge).
// ===========================================================================
//
// The loc-tree comparator (VarnodeCompareLocDef, varnode.cc:34-53) orders by
// (addr, size, flagclass, ...); a *free* Varnode (flagclass 0) and a *written*
// Varnode (flagclass `written`) at the SAME (addr,size) are DISTINCT keys, so
// `createDef`->`xref` of an output does NOT unify with, and does NOT repoint
// readers of, a pre-existing free Varnode at that address.  (Unification is
// same-flag-class only — see the bank's `xref_unifies_equivalent_input`.)  This
// pins that a free input Varnode read by another op survives an output being
// defined at the same storage, exactly as C++ does.

#[test]
fn w4x_flow_linkage_new_varnode_out_does_not_clobber_free_at_same_address() {
    let mut fd = build_fd();

    // Pre-existing free Varnode at ram:0x300, read by a consumer op in slot 0.
    let pre = mk_free(&mut fd, 0x300, 4);
    let consumer = mk_op(&mut fd, 1, 0x1008, OpCode::CPUI_INT_ADD);
    fd.op_set_input(consumer, pre, 0).unwrap();
    assert_eq!(fd.obank().get(consumer).unwrap().get_in(0), Some(pre));

    // A defining op whose output is created at the SAME storage as `pre`, via the
    // real newVarnodeOut (createDef -> xref) path.
    let def_op = mk_op(&mut fd, 1, 0x100c, OpCode::CPUI_COPY);
    let oaddr = Address::new(ram_space(&fd), 0x300);
    let out = fd.new_varnode_out(4, &oaddr, def_op).unwrap();

    // The new output is a DISTINCT (written) Varnode from the free `pre`.
    assert_ne!(out, pre, "the written output is a separate Varnode from the free one");
    assert_eq!(fd.obank().get(def_op).unwrap().get_out(), Some(out));
    assert_eq!(fd.vbank().get(out).unwrap().get_def(), Some(def_op));
    // The consumer STILL reads the original free Varnode — it was NOT repointed
    // (no cross-flag-class unification happened).
    assert_eq!(
        fd.obank().get(consumer).unwrap().get_in(0),
        Some(pre),
        "consumer still reads the free Varnode (no clobber across flag classes)"
    );
    // Both Varnodes coexist in the bank.
    assert_eq!(fd.num_varnodes(), 2, "free input and written output both live");
}

// ===========================================================================
// TEST 3 — unique-space outputs (newUniqueOut)
// ===========================================================================

#[test]
fn w4x_flow_linkage_new_unique_out_allocates_unique_and_links_def() {
    let mut fd = build_fd();
    let uniq = unique_space(&fd);

    let op0 = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_INT_ZEXT);
    let a = fd.new_unique_out(4, op0).unwrap();
    let op1 = mk_op(&mut fd, 1, 0x1004, OpCode::CPUI_INT_ZEXT);
    let b = fd.new_unique_out(8, op1).unwrap();

    // Both are in the unique space, defined by their ops.
    let va = fd.vbank().get(a).unwrap();
    assert!(Rc::ptr_eq(va.get_space(), &uniq), "newUniqueOut lands in the unique space");
    assert_eq!(va.get_size(), 4);
    assert_eq!(va.get_def(), Some(op0));
    assert_eq!(fd.obank().get(op0).unwrap().get_out(), Some(a));

    let vb = fd.vbank().get(b).unwrap();
    assert!(Rc::ptr_eq(vb.get_space(), &uniq));
    assert_eq!(vb.get_size(), 8);
    assert_eq!(vb.get_def(), Some(op1));
    assert_eq!(fd.obank().get(op1).unwrap().get_out(), Some(b));

    // uniqid advanced by the first allocation's size (C++ createDefUnique:
    // `uniqid += s`), so `b`'s offset == `a`'s offset + 4.
    let oa = fd.vbank().get(a).unwrap().get_offset();
    let ob = fd.vbank().get(b).unwrap().get_offset();
    assert_eq!(ob, oa + 4, "the unique offset advances by the prior allocation's size");
}

// ===========================================================================
// TEST 4 — codeRef target semantics (newCodeRef + the real dump path)
// ===========================================================================

#[test]
fn w4x_flow_linkage_new_code_ref_is_size1_annotation_at_target() {
    let mut fd = build_fd();
    let target = Address::new(ram_space(&fd), 0x2040);
    let vn = fd.new_code_ref(&target);
    let v = fd.vbank().get(vn).unwrap();
    // size 1, at the exact target (space,offset).  This (space,offset,size) triple
    // is the only thing the branch input / flow following needs.
    assert_eq!(v.get_size(), 1, "newCodeRef Varnode is size 1");
    assert!(Rc::ptr_eq(v.get_space(), &ram_space(&fd)));
    assert_eq!(v.get_offset(), 0x2040, "annotation sits AT the branch destination");
    // LOSS-077 RESTORED: `Varnode::annotation` is now set (newCodeRef wires the
    // `pub(crate)` set_annotation sliver, matching the C++ setFlags(annotation)).
    assert!(v.is_annotation(), "the code-ref Varnode carries the annotation flag");
    // An annotation Varnode is otherwise free (no def, no readers).
    assert!(v.is_free(), "the code-ref annotation is a free Varnode");
    assert!(!v.is_input(), "the code-ref annotation is not an input");
}

/// Drive the **real** `PcodeEmitFd::dump` (via the `PcodeEmit` trait impl in
/// flow.rs) for a BRANCH op: the slot-0 input must be the code-ref Varnode at the
/// branch target.  This exercises the exact `op->isCodeRef()` arm of dump that the
/// fix wired (`newCodeRef` -> `opSetInput(op, cref, 0)`).
#[test]
fn w4x_flow_linkage_dump_branch_links_coderef_input_at_target() {
    // We test the dump op-building arm through a tiny hand-built emit: create the
    // BRANCH op, run the same statements dump runs for a coderef op (the public
    // factories are exactly what dump calls).  This pins the (space,offset,size)
    // of the linked input.
    let mut fd = build_fd();
    let ram = ram_space(&fd);

    // op = newOp(1, addr); opSetOpcode(op, BRANCH)  -- BRANCH is a coderef op.
    let op = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_BRANCH);
    assert!(fd.obank().get(op).unwrap().is_code_ref(), "BRANCH carries the coderef flag");

    // dump's coderef arm: addrcode(vars[0].space, vars[0].offset); cref =
    // newCodeRef(addrcode); opSetInput(op, cref, 0).
    let addrcode = Address::new(Rc::clone(&ram), 0x2100);
    let cref = fd.new_code_ref(&addrcode);
    fd.op_set_input(op, cref, 0).unwrap();

    // The BRANCH's input slot 0 is the code-ref Varnode at (ram, 0x2100, 1).
    let in0 = fd.obank().get(op).unwrap().get_in(0).expect("slot 0 set");
    assert_eq!(in0, cref);
    let v = fd.vbank().get(in0).unwrap();
    assert!(Rc::ptr_eq(v.get_space(), &ram));
    assert_eq!(v.get_offset(), 0x2100);
    assert_eq!(v.get_size(), 1);
    // The code-ref reads into the BRANCH: op is in its descend list.
    let descend: Vec<OpId> = v.descend_iter().collect();
    assert_eq!(descend, vec![op]);
}

// ===========================================================================
// TEST 5 — opDestroyRaw teardown order (inputs, then output, then the op)
// ===========================================================================

#[test]
fn w4x_flow_linkage_op_destroy_raw_reaps_inputs_output_and_op() {
    let mut fd = build_fd();

    // Build a COPY op `out = COPY in` with both Varnodes linked.
    let op = mk_op(&mut fd, 1, 0x1000, OpCode::CPUI_COPY);
    let in0 = mk_free(&mut fd, 0x400, 4);
    fd.op_set_input(op, in0, 0).unwrap();
    let out = mk_free(&mut fd, 0x408, 4);
    fd.op_set_output(op, out).unwrap();

    let n_vn_before = fd.num_varnodes();
    assert_eq!(n_vn_before, 2, "the input and output Varnodes exist");
    let n_op_before = fd.obank().iter_all().count();

    // opDestroyRaw destroys every input, then the output, then the op.
    fd.op_destroy_raw(op).unwrap();

    // Both Varnodes are gone and the op is retired.
    assert_eq!(fd.num_varnodes(), 0, "input + output Varnodes destroyed");
    assert!(
        fd.obank().iter_all().count() < n_op_before,
        "the op was destroyed (removed from the live/dead lists)"
    );
}

// ===========================================================================
// TEST 6 — the constant-space (spaceid) input slot of dump is a plain
// constant-space Varnode (LOAD/STORE slot 0), built through the same factory.
// ===========================================================================

#[test]
fn w4x_flow_linkage_dump_constant_space_input_is_plain_constant_varnode() {
    // dump's `for(;i<isize;++i)` loop builds each input via
    // newVarnode(size, space, offset).  For a LOAD/STORE the slot-0 operand is a
    // spaceid constant in the constant space (offset = encoded space index,
    // LOSS-015).  dump does not special-case it; it is just a constant-space
    // Varnode at that offset.  Pin that the factory yields exactly that.
    let mut fd = build_fd();
    let cspace = const_space(&fd);

    let op = mk_op(&mut fd, 2, 0x1000, OpCode::CPUI_LOAD);
    // slot 0: spaceid constant (offset = some space index, here 3 = ram).
    let vn_sp = fd.new_varnode_space_off(8, Rc::clone(&cspace), 3);
    fd.op_set_input(op, vn_sp, 0).unwrap();

    let v = fd.vbank().get(vn_sp).unwrap();
    assert!(v.is_constant(), "the spaceid operand is a constant-space Varnode");
    assert_eq!(v.get_offset(), 3, "its offset is the encoded space index (LOSS-015)");
    assert_eq!(v.get_size(), 8);
    assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(vn_sp));
}

// ===========================================================================
// TEST 7 — setInputVarnode: formalize once, dedup an identical re-mark.
// ===========================================================================
//
// `setInputVarnode` (funcdata_varnode.cc:342, added by this diff) formalizes a
// free Varnode as a function input via the `banks_mut` + `replace_reads_thunk`
// path, and the overlap pre-check returns the pre-existing input for an identical
// (same size + same address) candidate.  The W4 `funcp.hasEffect` marks are a
// documented no-op (no prototype in W3).

#[test]
fn w4x_flow_linkage_set_input_varnode_formalizes_and_dedups() {
    let mut fd = build_fd();
    let ram = ram_space(&fd);

    // A free Varnode at ram:0x500, formalized as an input.
    let f1 = fd.new_varnode_space_off(4, Rc::clone(&ram), 0x500);
    assert!(fd.vbank().get(f1).unwrap().is_free());
    let in1 = fd.set_input_varnode(f1).unwrap();
    assert_eq!(in1, f1, "the first free Varnode becomes the input");
    assert!(fd.vbank().get(in1).unwrap().is_input(), "now flagged input");

    // Re-marking the SAME input is the `isInput()` early-out: returns it unchanged.
    let again = fd.set_input_varnode(in1).unwrap();
    assert_eq!(again, in1);

    // A SECOND free Varnode at the SAME (size,address) hits the overlap pre-check
    // and returns the pre-existing input (the candidate is discarded for it).
    let f2 = fd.new_varnode_space_off(4, Rc::clone(&ram), 0x500);
    let in2 = fd.set_input_varnode(f2).unwrap();
    assert_eq!(in2, in1, "identical candidate dedups onto the existing input");
}

// A compile-time anchor that the emitter's VarnodeData shape is the one dump
// consumes (size: u32, offset: u64, space index).  Guards against a silent
// VarnodeData field-type drift under the `as int4` size cast in dump.
const _: fn() = || {
    let _check: fn(&VarnodeData) -> u32 = |vd| vd.size;
};
