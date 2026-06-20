// Verifier-authored adversarial tests for item `w7-s6-dynamic-pack`
// (DynamicHash / ToOpEdge / kuna_dynamichashmax / kuna_stackguard).
//
// These exercise the spots the hunt list flagged as most fragile for this item,
// against the public crate surface only:
//
//   T1  slot == -1 (output-attached root) packs as 0x1f and decodes back to -1
//       through a *real* calcHash (not just a hand-packed hash), exercising the
//       `(slot & 0x1f)` field and getSlotFromHash on a genuine output edge.
//   T2  the OP-FORM uniqueHash *always* returns hash 0 — the faithfully
//       transcribed upstream quirk (dynamic.cc:519 `oplist.clear()` inside the
//       method loop empties the collision scan), so champion stays empty.  A
//       "fixed" port would return a nonzero op hash here; this pins bug-for-bug
//       parity.
//   T3  `x = a + a` (one op reads the root in TWO slots) — buildVnDown sorts the
//       same-op/different-slot edges; the hash is deterministic and matches a
//       re-hash, proving the stable-vs-unstable sort substitution is safe here
//       (distinct slot keys, no ties).
//   T4  a CAST interposed on the *input* path to an op-form root is invisible to
//       the op hash (moveOffSkip / the calcHash(op) skip-peel) — calcHash with
//       method 5 on the same logical input through a CAST yields the same
//       comparable hash as without it.
//   T5  ActionStripStackGuard is inert (returns 0, CFG untouched) with the gate
//       OFF and ALSO with the gate ON over a real canary epilogue (the
//       removeBranch/removeUnreachableBlocks CFG surgery is the W4/W8 seam);
//       plus the option decision body + the budget ceiling of the wide option.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use kuna_decomp::action::{Action, ActionContext};
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::dynamic::DynamicHash;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_dynamichashmax::DynamicHashMaxOption;
use kuna_decomp::kuna_stackguard::{ActionStripStackGuard, StackGuardOption};
use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};

// --- fd builder (mirrors the in-crate test harness) ---------------------------

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
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn unk(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(inputs, Address::new(r, off));
    fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    op
}

fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    fd.vbank_mut().create(size, Address::new(r, off), unk(size))
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn copy(fd: &mut Funcdata, bl: BlockId, off: u64, a: VarnodeId, out_off: u64, size: int4) -> VarnodeId {
    let op = mk_op(fd, 1, off, OpCode::CPUI_COPY);
    fd.op_set_input(op, a, 0).unwrap();
    let out = mk_vn(fd, out_off, size);
    fd.op_set_output(op, out).unwrap();
    fd.op_insert_end(op, bl);
    out
}

// =============================================================================
// T1 — slot == -1 output edge round-trips through a real calcHash
// =============================================================================

#[test]
fn t1_output_attached_root_encodes_slot_minus_one() {
    // Root = the OUTPUT of an INT_ADD.  calcHash finds the op directly attached
    // to the root via its output (slot -1 in pieceTogetherHash), so the packed
    // slot field is 0x1f and getSlotFromHash decodes it back to -1.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let ca = fd.new_constant(8, 0x11);
    let a = copy(&mut fd, bl, 0x10, ca, 0x100, 8);
    let cb = fd.new_constant(8, 0x22);
    let b = copy(&mut fd, bl, 0x14, cb, 0x108, 8);
    let add = mk_op(&mut fd, 2, 0x18, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, a, 0).unwrap();
    fd.op_set_input(add, b, 1).unwrap();
    let c = mk_vn(&mut fd, 0x110, 8);
    fd.op_set_output(add, c).unwrap();
    fd.op_insert_end(add, bl);

    let mut dh = DynamicHash::new();
    dh.calc_hash(c, 0, &mut fd);
    let h = dh.get_hash();
    assert_ne!(h, 0, "well-formed output root hashes nonzero");
    // The root is attached as the OUTPUT of `add` => slot field encodes -1.
    assert_eq!(
        DynamicHash::get_slot_from_hash(h),
        -1,
        "output-attached root decodes to slot -1 (0x1f field)"
    );
    // And it's a directly-attached op, so the not-attached bit is clear.
    assert!(!DynamicHash::get_is_not_attached(h), "root directly attached => not-attached bit clear");
}

// =============================================================================
// T2 — op-form uniqueHash always returns hash 0 (faithful upstream quirk)
// =============================================================================

#[test]
fn t2_op_form_unique_hash_is_always_zero() {
    // dynamic.cc:519 clears `oplist` at the top of the method loop, BEFORE the
    // inner `for(i<oplist.size())` collision scan, so the scan never runs and
    // `champion` stays empty for every method => uniqueHashOp returns hash 0.
    // This is a bug-for-bug parity pin: any "fix" that filled the collision list
    // would make this nonzero and silently diverge from upstream.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let ca = fd.new_constant(8, 0x11);
    let a = copy(&mut fd, bl, 0x10, ca, 0x100, 8);
    let cb = fd.new_constant(8, 0x22);
    let b = copy(&mut fd, bl, 0x14, cb, 0x108, 8);
    let add = mk_op(&mut fd, 2, 0x18, OpCode::CPUI_INT_ADD);
    fd.op_set_input(add, a, 0).unwrap();
    fd.op_set_input(add, b, 1).unwrap();
    let c = mk_vn(&mut fd, 0x110, 8);
    fd.op_set_output(add, c).unwrap();
    fd.op_insert_end(add, bl);

    // Both slots (an input slot and the output slot -1) hit the same quirk.
    for slot in [-1, 0, 1] {
        let mut dh = DynamicHash::new();
        dh.unique_hash_op(add, slot, 8, &mut fd);
        assert_eq!(
            dh.get_hash(),
            0,
            "op-form uniqueHash(slot={slot}) returns 0 (upstream oplist.clear() quirk)"
        );
        // Wide budget changes nothing — the scan still never executes.
        let mut dw = DynamicHash::new();
        dw.unique_hash_op(add, slot, 16, &mut fd);
        assert_eq!(dw.get_hash(), 0, "wide budget cannot rescue the empty scan");
    }
}

// =============================================================================
// T3 — `x = a + a` (root read in two slots) hashes deterministically
// =============================================================================

#[test]
fn t3_root_read_in_two_slots_hashes_deterministically() {
    // `out = INT_ADD(a, a)` — a single descendant op reads `a` in BOTH slots, so
    // buildVnDown emits two edges (same op, slot 0 and slot 1) and sorts that
    // sub-range.  The two keys differ only by slot (distinct => no tie), so the
    // stable sort the port substitutes for std::sort is order-equivalent; the
    // hash must be reproducible across repeated computations.
    let mut fd = build_fd();
    let bl = mk_block(&mut fd);
    let ca = fd.new_constant(8, 0x11);
    let a = copy(&mut fd, bl, 0x10, ca, 0x100, 8);
    let dbl = mk_op(&mut fd, 2, 0x18, OpCode::CPUI_INT_ADD);
    fd.op_set_input(dbl, a, 0).unwrap();
    fd.op_set_input(dbl, a, 1).unwrap(); // same varnode in slot 1
    let out = mk_vn(&mut fd, 0x110, 8);
    fd.op_set_output(dbl, out).unwrap();
    fd.op_insert_end(dbl, bl);

    // Hash the doubled input `a` via method 0 (which runs buildVnDown over `a`).
    let mut dh1 = DynamicHash::new();
    dh1.calc_hash(a, 0, &mut fd);
    let h1 = dh1.get_hash();
    let mut dh2 = DynamicHash::new();
    dh2.calc_hash(a, 0, &mut fd);
    let h2 = dh2.get_hash();
    assert_eq!(h1, h2, "two-slot-read root hashes deterministically");
    assert_ne!(h1, 0, "the doubled input still yields a nonzero hash");
    // Method 2 (one more level of outputs) must also be stable.
    let mut dh3 = DynamicHash::new();
    dh3.calc_hash(a, 2, &mut fd);
    let mut dh4 = DynamicHash::new();
    dh4.calc_hash(a, 2, &mut fd);
    assert_eq!(dh3.get_hash(), dh4.get_hash(), "method-2 over a two-slot read is stable");
}

// =============================================================================
// T4 — a CAST on the input path is invisible to the op-form (method 5) hash
// =============================================================================

#[test]
fn t4_input_cast_is_invisible_to_op_hash() {
    // Two ops identical except one feeds its input through a CAST.  Method 5
    // (calcHash(op,slot,5) -> buildOpUp/buildVnUp) peels the CAST (TRANSTABLE
    // skips it), so the comparable (neighborhood) hash is identical.
    // -- graph A: out = NEGATE(a)
    let mut fda = build_fd();
    let bla = mk_block(&mut fda);
    let ca = fda.new_constant(8, 0x55);
    let a = copy(&mut fda, bla, 0x10, ca, 0x100, 8);
    let nega = mk_op(&mut fda, 1, 0x20, OpCode::CPUI_INT_NEGATE);
    fda.op_set_input(nega, a, 0).unwrap();
    let outa = mk_vn(&mut fda, 0x110, 8);
    fda.op_set_output(nega, outa).unwrap();
    fda.op_insert_end(nega, bla);
    let mut dha = DynamicHash::new();
    dha.calc_hash_op(nega, 0, 5, &mut fda);
    let ha = DynamicHash::get_comparable(dha.get_hash());

    // -- graph B: cast = CAST(a); out = NEGATE(cast)
    let mut fdb = build_fd();
    let blb = mk_block(&mut fdb);
    let cb = fdb.new_constant(8, 0x55);
    let b = copy(&mut fdb, blb, 0x10, cb, 0x100, 8);
    let cast = mk_op(&mut fdb, 1, 0x18, OpCode::CPUI_CAST);
    fdb.op_set_input(cast, b, 0).unwrap();
    let castout = mk_vn(&mut fdb, 0x108, 8);
    fdb.op_set_output(cast, castout).unwrap();
    fdb.op_insert_end(cast, blb);
    let negb = mk_op(&mut fdb, 1, 0x20, OpCode::CPUI_INT_NEGATE);
    fdb.op_set_input(negb, castout, 0).unwrap();
    let outb = mk_vn(&mut fdb, 0x110, 8);
    fdb.op_set_output(negb, outb).unwrap();
    fdb.op_insert_end(negb, blb);
    let mut dhb = DynamicHash::new();
    dhb.calc_hash_op(negb, 0, 5, &mut fdb);
    let hb = DynamicHash::get_comparable(dhb.get_hash());

    assert_eq!(ha, hb, "an interposed CAST on the input is invisible to the neighborhood hash");
}

// =============================================================================
// T5 — kuna option bodies + the stackguard Action seam boundary
// =============================================================================

#[test]
fn t5_dynamichashmax_budget_and_default() {
    let mut opt = DynamicHashMaxOption::default();
    assert!(!opt.is_enabled(), "option-class neutral default is off");
    assert_eq!(opt.max_duplicates(), 8, "off => budget 8 (upstream)");
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert_eq!(opt.max_duplicates(), 16, "on => budget 16 (wide)");
    assert_eq!(msg, "DynamicHash wide collision budget turned on");
    // 16 is the exact ceiling of the 3-bit (+1) total field; never above.
    assert!(opt.max_duplicates() <= 16);
}

/// Build a real glibc-style canary epilogue: header CBRANCH on an INT_NOTEQUAL
/// of two values both derived from a LOAD of `<base>+0x28`, true edge -> a fail
/// block that CALLs a handler.  Returns the header block.
fn build_canary_epilogue(fd: &mut Funcdata) -> BlockId {
    let h = mk_block(fd);
    let ok = mk_block(fd);
    let fail = mk_block(fd);

    // base = COPY(#0) so it can have multiple readers.
    let base = {
        let op = mk_op(fd, 1, 0x08, OpCode::CPUI_COPY);
        let c = fd.new_constant(8, 0);
        fd.op_set_input(op, c, 0).unwrap();
        let out = mk_vn(fd, 0x80, 8);
        fd.op_set_output(op, out).unwrap();
        fd.op_insert_end(op, h);
        out
    };
    let load_slot = |fd: &mut Funcdata, off: u64, ptr_off: u64, out_off: u64| -> VarnodeId {
        let add = mk_op(fd, 2, off, OpCode::CPUI_INT_ADD);
        fd.op_set_input(add, base, 0).unwrap();
        let c = fd.new_constant(8, 0x28);
        fd.op_set_input(add, c, 1).unwrap();
        let ptr = mk_vn(fd, ptr_off, 8);
        fd.op_set_output(add, ptr).unwrap();
        fd.op_insert_end(add, h);
        let ld = mk_op(fd, 2, off + 2, OpCode::CPUI_LOAD);
        let r = ram(fd);
        let spaceid = fd.new_constant(8, r.get_index() as u64);
        fd.op_set_input(ld, spaceid, 0).unwrap();
        fd.op_set_input(ld, ptr, 1).unwrap();
        let out = mk_vn(fd, out_off, 8);
        fd.op_set_output(ld, out).unwrap();
        fd.op_insert_end(ld, h);
        out
    };
    let saved = load_slot(fd, 0x10, 0x100, 0x108);
    let reload = load_slot(fd, 0x18, 0x110, 0x118);

    let cmp = mk_op(fd, 2, 0x24, OpCode::CPUI_INT_NOTEQUAL);
    fd.op_set_input(cmp, saved, 0).unwrap();
    fd.op_set_input(cmp, reload, 1).unwrap();
    let boolvn = mk_vn(fd, 0x128, 1);
    fd.op_set_output(cmp, boolvn).unwrap();
    fd.op_insert_end(cmp, h);

    let cbr = mk_op(fd, 2, 0x28, OpCode::CPUI_CBRANCH);
    let dest = mk_vn(fd, 0x200, 8);
    fd.op_set_input(cbr, dest, 0).unwrap();
    fd.op_set_input(cbr, boolvn, 1).unwrap();
    fd.op_insert_end(cbr, h);

    fd.bblocks_mut().add_edge(h, ok);
    fd.bblocks_mut().add_edge(h, fail);

    let call = mk_op(fd, 1, 0x30, OpCode::CPUI_CALL);
    let target = mk_vn(fd, 0x300, 8);
    fd.op_set_input(call, target, 0).unwrap();
    fd.op_insert_end(call, fail);

    h
}

#[test]
fn t5_stackguard_action_inert_both_gates() {
    // Gate OFF: inert, CFG untouched.
    let mut fd = build_fd();
    let _h = build_canary_epilogue(&mut fd);
    let before = fd.bblocks_get_size();
    let mut off = ActionStripStackGuard::new(false, "stackptrflow");
    let mut ctx = ActionContext::default();
    assert_eq!(off.apply(&mut fd, &mut ctx), 0, "gate off => inert");
    assert_eq!(fd.bblocks_get_size(), before, "gate off leaves the CFG untouched");

    // Gate ON over a *real* canary epilogue: the detection runs, but the
    // removeBranch/removeUnreachableBlocks surgery is the W4/W8 seam, so the
    // Action still reports no change and the CFG is intact (LOSS pin: when the
    // primitive lands, this flips to 1 + one fewer edge).
    let mut on = ActionStripStackGuard::new(true, "stackptrflow");
    assert_eq!(
        on.apply(&mut fd, &mut ctx),
        0,
        "detection-only until the removeBranch seam lands"
    );
    assert_eq!(fd.bblocks_get_size(), before, "gate on (seam) leaves the CFG untouched");
    assert_eq!(on.get_name(), "stripstackguard");
}

#[test]
fn t5_stackguard_option_body() {
    let mut opt = StackGuardOption::default();
    assert!(!opt.is_enabled(), "default off => upstream byte-identical");
    let on = opt.apply(true);
    assert!(opt.is_enabled());
    assert_eq!(on, "Stack-protector canary epilogue stripping turned on");
    let off = opt.apply(false);
    assert!(!opt.is_enabled());
    assert_eq!(off, "Stack-protector canary epilogue stripping turned off");
}
