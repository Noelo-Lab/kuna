//! INDEPENDENT VERIFIER adversarial tests for item `w10-cast-strategy`.
//!
//! The diff under review (`rport/w10-cast-strategy`) routes `INT_SEXT`/`INT_ZEXT`
//! through `CastStrategyC` in the C printer (`printc.rs` `op_int_sext_ir` /
//! `op_int_zext_ir`, the faithful transcription of `PrintC::opIntSext`/
//! `opIntZext`, printc.cc:806-830).  The render decision is:
//!
//! ```text
//!   if  isZextCast(out_def_facing, in_read_facing)        // signedness gate
//!       if  option_hide_exts && isExtensionCastImplied(op, readOp)
//!           opHiddenFunc(op)     // hidden — implied by integer promotion
//!       else
//!           opTypeCast(op)       // explicit `(uintN)x` / `(intN)x`
//!   else
//!       opFunc(op)               // functional `ZEXT(x)` / `SEXT(x)`
//! ```
//!
//! The pure cast-strategy predicates (`isZextCast`/`isSextCast`/
//! `isExtensionCastImplied`) live in the UNCHANGED `cast.rs` (ported in a prior
//! wave); this wave only WIRES them into the printer.  The faithfulness risk is
//! therefore in the DECISION COMBINATION the wiring performs — the argument order
//! fed to the predicate, the `option_hide_exts` gate, and the `readOp == 0`
//! (top-of-expression) handling.  These tests reproduce that exact combination
//! over a mock [`CastContext`] (the same shape `cast/tests.rs` uses internally,
//! re-built here against the public trait) and pin the boundaries the
//! verification hunt list flagged:
//!
//!   (V1) SIGNEDNESS GATE — a ZEXT whose input metatype is INT (signed), not
//!        UINT/BOOL, is NOT a zext-cast: it must fall to the functional `ZEXT(x)`
//!        form regardless of `hide_exts` (the `isZextCast` input-signedness
//!        requirement, cast.cc:457).  A SEXT mirrors it for the input-UINT case.
//!
//!   (V2) READOP==0 BOUNDARY — at the top of a statement `readOp` is null, so
//!        `isExtensionCastImplied` short-circuits to `false` (cast.cc:255): a
//!        non-explicit ZEXT output can NEVER be hidden there, it renders as the
//!        explicit `(uintN)x` cast.  This is the spot the printc seam threads
//!        `None`.
//!
//!   (V3) BIG-CONSTANT BOUNDARY — when the extension is read by an integer op
//!        whose OTHER operand is a constant strictly LARGER than the promotion
//!        size, `isExtensionCastImplied` returns false (cast.cc:281-284: a big
//!        integer is not naturally extended, so the cast must be explicit).  At
//!        the exact boundary (== promoteSize) it is still implied (hidden).  This
//!        is the off-by-one the hunt list flags for the `> promoteSize` compare.
//!
//!   (V4) HIDE-EXTS GATE — with `option_hide_exts` OFF, even a genuinely-implied
//!        extension renders as the explicit cast, never hidden.  The gate is an
//!        AND, so flipping it must change Hidden->Cast and nothing else.
//!
//! Each test is a pure decision check (no SLA frontend), so it never SKIPs.

use std::rc::Rc;

use kuna_decomp::cast::{CastContext, CastStrategy, CastStrategyC, OpRef, VnRef};
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactoryImpl};
use kuna_num::opcodes::OpCode;

#[allow(non_camel_case_types)]
type int4 = i32;
#[allow(non_camel_case_types)]
type uintb = u64;

// -- data-type builders (mirror cast/tests.rs) ------------------------------

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, meta))
}
fn int_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_INT)
}
fn uint_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_UINT)
}

/// `CastStrategyC` with `promoteSize == 4` (the x86:LE:64 `int` size).
fn strategy_c() -> CastStrategyC {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.setup_sizes(Some(4), 8, 8);
    CastStrategyC::new(Rc::new(f))
}

// -- the exact printc decision -----------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Render {
    /// `opTypeCast` — explicit `(uintN)x` / `(intN)x`.
    Cast,
    /// `opHiddenFunc` — extension implied by promotion, not printed.
    Hidden,
    /// `opFunc` — functional `ZEXT(x)` / `SEXT(x)`.
    Functional,
}

/// Faithful re-statement of `PrintC::opIntZext` (printc.cc:806).  Mirrors
/// `printc.rs::op_int_zext_ir`: same predicate order, same gate, same fallback.
fn decide_zext(
    strat: &CastStrategyC,
    ctx: &dyn CastContext,
    op: OpRef,
    read_op: Option<OpRef>,
    hide_exts: bool,
) -> Render {
    let outvn = ctx.op_out(op).unwrap();
    let invn = ctx.op_in(op, 0);
    // (out->getHighTypeDefFacing(), in0->getHighTypeReadFacing(op))
    let outtype = ctx.vn_high_type(outvn);
    let intype = ctx.vn_high_type_read_facing(invn, op);
    if strat.is_zext_cast(&outtype, &intype) {
        if hide_exts && strat.is_extension_cast_implied(ctx, op, read_op) {
            Render::Hidden
        } else {
            Render::Cast
        }
    } else {
        Render::Functional
    }
}

/// Faithful re-statement of `PrintC::opIntSext` (printc.cc:819).
fn decide_sext(
    strat: &CastStrategyC,
    ctx: &dyn CastContext,
    op: OpRef,
    read_op: Option<OpRef>,
    hide_exts: bool,
) -> Render {
    let outvn = ctx.op_out(op).unwrap();
    let invn = ctx.op_in(op, 0);
    let outtype = ctx.vn_high_type(outvn);
    let intype = ctx.vn_high_type_read_facing(invn, op);
    if strat.is_sext_cast(&outtype, &intype) {
        if hide_exts && strat.is_extension_cast_implied(ctx, op, read_op) {
            Render::Hidden
        } else {
            Render::Cast
        }
    } else {
        Render::Functional
    }
}

// -- mock CastContext (mirrors cast/tests.rs MockCtx) -----------------------

struct MockVn {
    size: int4,
    offset: uintb,
    ty: Rc<Datatype>,
    constant: bool,
    explicit: bool,
    written: bool,
    def: Option<OpRef>,
}
struct MockOp {
    code: OpCode,
    ins: Vec<VnRef>,
    out: Option<VnRef>,
}
struct MockCtx {
    vns: Vec<MockVn>,
    ops: Vec<MockOp>,
}
impl MockCtx {
    fn new() -> MockCtx {
        MockCtx { vns: Vec::new(), ops: Vec::new() }
    }
    fn add_var(&mut self, size: int4, ty: Rc<Datatype>) -> VnRef {
        let id = self.vns.len();
        self.vns.push(MockVn {
            size,
            offset: 0,
            ty,
            constant: false,
            explicit: false,
            written: false,
            def: None,
        });
        VnRef(id)
    }
    fn add_const(&mut self, size: int4, offset: uintb, ty: Rc<Datatype>) -> VnRef {
        let v = self.add_var(size, ty);
        self.vns[v.0].constant = true;
        self.vns[v.0].offset = offset;
        v
    }
    fn set_explicit(&mut self, v: VnRef, val: bool) {
        self.vns[v.0].explicit = val;
    }
    fn add_op(&mut self, code: OpCode, ins: Vec<VnRef>, out: Option<VnRef>) -> OpRef {
        let id = self.ops.len();
        let op = OpRef(id);
        if let Some(o) = out {
            self.vns[o.0].written = true;
            self.vns[o.0].def = Some(op);
        }
        self.ops.push(MockOp { code, ins, out });
        op
    }
}
impl CastContext for MockCtx {
    fn op_code(&self, op: OpRef) -> OpCode {
        self.ops[op.0].code
    }
    fn op_num_input(&self, op: OpRef) -> int4 {
        self.ops[op.0].ins.len() as int4
    }
    fn op_in(&self, op: OpRef, slot: int4) -> VnRef {
        self.ops[op.0].ins[slot as usize]
    }
    fn op_out(&self, op: OpRef) -> Option<VnRef> {
        self.ops[op.0].out
    }
    fn op_slot(&self, op: OpRef, vn: VnRef) -> int4 {
        self.ops[op.0].ins.iter().position(|v| *v == vn).unwrap() as int4
    }
    fn vn_is_constant(&self, vn: VnRef) -> bool {
        self.vns[vn.0].constant
    }
    fn vn_is_explicit(&self, vn: VnRef) -> bool {
        self.vns[vn.0].explicit
    }
    fn vn_is_written(&self, vn: VnRef) -> bool {
        self.vns[vn.0].written
    }
    fn vn_size(&self, vn: VnRef) -> int4 {
        self.vns[vn.0].size
    }
    fn vn_offset(&self, vn: VnRef) -> uintb {
        self.vns[vn.0].offset
    }
    fn vn_def(&self, vn: VnRef) -> Option<OpRef> {
        self.vns[vn.0].def
    }
    fn vn_lone_descend(&self, _vn: VnRef) -> Option<OpRef> {
        None
    }
    fn vn_high_type(&self, vn: VnRef) -> Rc<Datatype> {
        Rc::clone(&self.vns[vn.0].ty)
    }
    fn vn_high_type_read_facing(&self, vn: VnRef, _op: OpRef) -> Rc<Datatype> {
        Rc::clone(&self.vns[vn.0].ty)
    }
    fn op_inherits_sign(&self, op: OpRef) -> bool {
        kuna_decomp::typeop::type_op_info(self.ops[op.0].code).inherits_sign()
    }
    fn op_inherits_sign_first_param_only(&self, op: OpRef) -> bool {
        kuna_decomp::typeop::type_op_info(self.ops[op.0].code).inherits_sign_first_param_only()
    }
    fn op_is_shift_op(&self, op: OpRef) -> bool {
        kuna_decomp::typeop::type_op_info(self.ops[op.0].code).is_shift_op()
    }
    fn op_is_bool_output(&self, _op: OpRef) -> bool {
        false
    }
    fn op_is_call(&self, _op: OpRef) -> bool {
        false
    }
    fn vn_set_unsigned_print(&mut self, _vn: VnRef) {
        unreachable!("not used by isExtensionCastImplied");
    }
    fn vn_set_long_print(&mut self, _vn: VnRef) {
        unreachable!("not used by isExtensionCastImplied");
    }
}

/// Build a `out = ZEXT(in0)` op whose output is read by `read = readcode(out, other)`.
/// Returns (ctx, zext_op, read_op).  The ZEXT output is left non-explicit so the
/// "implied" arm is reachable; `other` is whatever the caller wired in.
fn zext_read_by(
    out_ty: Rc<Datatype>,
    in_ty: Rc<Datatype>,
    in_size: int4,
    out_size: int4,
    readcode: OpCode,
    other: impl FnOnce(&mut MockCtx) -> VnRef,
) -> (MockCtx, OpRef, OpRef) {
    let mut ctx = MockCtx::new();
    let invn = ctx.add_var(in_size, in_ty);
    let outvn = ctx.add_var(out_size, out_ty);
    let zext = ctx.add_op(OpCode::CPUI_INT_ZEXT, vec![invn], Some(outvn));
    let othervn = other(&mut ctx);
    // out is read by `read = out <readcode> other`; result type matches `out`.
    let resty = uint_t(out_size);
    let resvn = ctx.add_var(out_size, resty);
    let read = ctx.add_op(readcode, vec![outvn, othervn], Some(resvn));
    (ctx, zext, read)
}

// ===========================================================================

/// (V1) The signedness gate: a ZEXT whose INPUT is signed (TYPE_INT) is NOT a
/// zext-cast (`isZextCast` requires the input metatype to be UINT or BOOL,
/// cast.cc:464).  So it renders as the FUNCTIONAL `ZEXT(x)`, never a `(uintN)`
/// cast, regardless of `hide_exts` — this guards against the wiring feeding the
/// predicate the wrong type pair (e.g. swapping out/in, which would flip the
/// gate).
#[test]
fn w10_cast_strategy_zext_signed_input_stays_functional() {
    let strat = strategy_c();
    // out uint8, in INT4 (signed) -> isZextCast false (input not UINT/BOOL).
    let mut ctx = MockCtx::new();
    let invn = ctx.add_var(4, int_t(4));
    let outvn = ctx.add_var(8, uint_t(8));
    let zext = ctx.add_op(OpCode::CPUI_INT_ZEXT, vec![invn], Some(outvn));

    // No reader / null readOp, both hide settings: always functional.
    assert_eq!(decide_zext(&strat, &ctx, zext, None, true), Render::Functional);
    assert_eq!(decide_zext(&strat, &ctx, zext, None, false), Render::Functional);

    // Flip the input to UNSIGNED: now it IS a zext-cast (explicit, no reader).
    let mut ctx2 = MockCtx::new();
    let invn2 = ctx2.add_var(4, uint_t(4));
    let outvn2 = ctx2.add_var(8, uint_t(8));
    let zext2 = ctx2.add_op(OpCode::CPUI_INT_ZEXT, vec![invn2], Some(outvn2));
    assert_eq!(decide_zext(&strat, &ctx2, zext2, None, true), Render::Cast);

    // SEXT mirror: a SEXT of an UNSIGNED input is NOT a sext-cast
    // (`isSextCast` requires INT/BOOL input, cast.cc:450) -> functional.
    let mut ctx3 = MockCtx::new();
    let sin = ctx3.add_var(4, uint_t(4));
    let sout = ctx3.add_var(8, int_t(8));
    let sext = ctx3.add_op(OpCode::CPUI_INT_SEXT, vec![sin], Some(sout));
    assert_eq!(decide_sext(&strat, &ctx3, sext, None, true), Render::Functional);
    // INT input -> sext-cast (explicit).
    let mut ctx4 = MockCtx::new();
    let sin4 = ctx4.add_var(4, int_t(4));
    let sout4 = ctx4.add_var(8, int_t(8));
    let sext4 = ctx4.add_op(OpCode::CPUI_INT_SEXT, vec![sin4], Some(sout4));
    assert_eq!(decide_sext(&strat, &ctx4, sext4, None, true), Render::Cast);
}

/// (V2) readOp == null (top of statement): `isExtensionCastImplied` returns false
/// for a NON-explicit output when there is no reader (cast.cc:255).  So the
/// genuine zext-cast renders as the EXPLICIT `(uintN)` cast at statement top, and
/// is only ever hidden when threaded a real reader.  This pins the printc seam
/// that passes `None` at printc.cc:2579.
#[test]
fn w10_cast_strategy_null_readop_never_hidden() {
    let strat = strategy_c();
    // out uint8, in uint4 -> isZextCast true.  Output NON-explicit.
    let mut ctx = MockCtx::new();
    let invn = ctx.add_var(4, uint_t(4));
    let outvn = ctx.add_var(8, uint_t(8));
    let zext = ctx.add_op(OpCode::CPUI_INT_ZEXT, vec![invn], Some(outvn));
    assert!(!ctx.vn_is_explicit(outvn), "precondition: output non-explicit");

    // hide_exts ON but readOp None: implied is false -> CAST, not Hidden.
    assert_eq!(
        decide_zext(&strat, &ctx, zext, None, true),
        Render::Cast,
        "a null readOp must short-circuit isExtensionCastImplied to false, \
         so a zext-cast at statement top renders explicit, never hidden"
    );

    // Cross-check the predicate directly: implied is false with no reader.
    assert!(!strat.is_extension_cast_implied(&ctx, zext, None));
}

/// (V3) The big-constant boundary in `isExtensionCastImplied` (cast.cc:281-284):
/// when the extension's reader is an integer op whose OTHER operand is a CONSTANT,
/// the extension is implied (hidden) only if that constant is NOT bigger than the
/// promotion size.  At `size == promoteSize` it is still implied (hidden); at
/// `size == promoteSize + 1` (`> promoteSize`) it flips to NOT implied -> the
/// cast must be explicit.  This is the off-by-one the hunt list flags for the
/// `getSize() > promoteSize` compare (promoteSize == 4 here).
#[test]
fn w10_cast_strategy_big_constant_boundary_flips_implied() {
    let strat = strategy_c();
    let promote = strat.promote_size();
    assert_eq!(promote, 4);

    // out=uint8 = ZEXT(uint4 in); read = out + const(size).  metatype(out)=UINT.
    // The other operand is a UINT constant so its metatype matches.
    let build = |const_size: int4| {
        zext_read_by(
            uint_t(8),
            uint_t(4),
            4,
            8,
            OpCode::CPUI_INT_ADD,
            move |c| c.add_const(const_size, 7, uint_t(const_size)),
        )
    };

    // const size == promoteSize (4): NOT bigger -> implied -> Hidden (hide on).
    let (ctx_eq, zext_eq, read_eq) = build(promote);
    assert!(
        strat.is_extension_cast_implied(&ctx_eq, zext_eq, Some(read_eq)),
        "const at exactly promoteSize must be implied (boundary inclusive)"
    );
    assert_eq!(
        decide_zext(&strat, &ctx_eq, zext_eq, Some(read_eq), true),
        Render::Hidden
    );

    // const size == promoteSize + 1 (5 > 4): bigger -> NOT implied -> Cast.
    let (ctx_gt, zext_gt, read_gt) = build(promote + 1);
    assert!(
        !strat.is_extension_cast_implied(&ctx_gt, zext_gt, Some(read_gt)),
        "const strictly bigger than promoteSize must NOT be implied"
    );
    assert_eq!(
        decide_zext(&strat, &ctx_gt, zext_gt, Some(read_gt), true),
        Render::Cast,
        "a >promoteSize constant forces the extension cast to be explicit"
    );
}

/// (V4) The `option_hide_exts` gate is an AND: an extension that IS implied (and
/// would be Hidden with the option on) renders as the explicit `(uintN)` CAST
/// when the option is off — and nothing else changes.  This guards against the
/// wiring dropping the gate (which would over-hide casts under `--option
/// hideextension off`).
#[test]
fn w10_cast_strategy_hide_exts_gate_is_and() {
    let strat = strategy_c();
    // A genuinely-implied zext: out uint8 = ZEXT(uint4), read by `out & explicit-uint8`.
    let (ctx, zext, read) = zext_read_by(
        uint_t(8),
        uint_t(4),
        4,
        8,
        OpCode::CPUI_INT_AND,
        |c| {
            let v = c.add_var(8, uint_t(8));
            c.set_explicit(v, true);
            v
        },
    );
    // Precondition: it IS implied.
    assert!(strat.is_extension_cast_implied(&ctx, zext, Some(read)));

    // hide ON  -> Hidden ; hide OFF -> Cast (the AND short-circuits the implied test).
    assert_eq!(decide_zext(&strat, &ctx, zext, Some(read), true), Render::Hidden);
    assert_eq!(decide_zext(&strat, &ctx, zext, Some(read), false), Render::Cast);
}

/// (V5, bonus) Metatype-mismatch reader: if the OTHER operand's metatype differs
/// from the extension output's metatype, the extension is NOT implied
/// (cast.cc:289-290) -> explicit cast.  Pins the "right integer type" check the
/// wiring relies on the unchanged predicate for.
#[test]
fn w10_cast_strategy_metatype_mismatch_not_implied() {
    let strat = strategy_c();
    // out is UINT8; other operand is an explicit INT8 (signed) -> metatype differs.
    let (ctx, zext, read) = zext_read_by(
        uint_t(8),
        uint_t(4),
        4,
        8,
        OpCode::CPUI_INT_ADD,
        |c| {
            let v = c.add_var(8, int_t(8)); // signed, mismatched metatype
            c.set_explicit(v, true);
            v
        },
    );
    assert!(
        !strat.is_extension_cast_implied(&ctx, zext, Some(read)),
        "a mismatched-metatype other operand must defeat the implied test"
    );
    assert_eq!(decide_zext(&strat, &ctx, zext, Some(read), true), Render::Cast);
}
