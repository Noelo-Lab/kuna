//! Port of `decompiler/cpp/cast.{cc,hh}` — the cast-insertion strategy
//! (`CastStrategy`, `CastStrategyC`, `CastStrategyJava`).
//!
//! ## What this module decides (S5/S9)
//!
//! A *cast* in C masks many low-level conversions (extension, truncation,
//! integer<->float, signed<->unsigned).  Many of these are *implied* by the
//! source language and need no explicit token.  [`CastStrategy`] is the API for
//! the four kinds of decision the printer asks of the type system:
//!
//!   - Does a given assignment need a cast operator? (`cast_standard`)
//!   - Does a given conversion op (ZEXT/SEXT/SUBPIECE) need to be a cast?
//!     (`is_sext_cast`/`is_zext_cast`/`is_subpiece_cast*`)
//!   - Does integer promotion match the expected extension/comparison level?
//!     (`int_promotion_type`/`check_int_promotion_for_*`/`is_extension_cast_implied`)
//!   - What data-type does an integer arithmetic op produce?
//!     (`arithmetic_output_standard`)
//!
//! plus the two constant-marking helpers `mark_explicit_unsigned` and
//! `mark_explicit_long_size` that flag a constant Varnode for `unsigned`/`long`
//! source-token printing.
//!
//! ## The two halves: pure-lattice vs. IR-walking
//!
//! cast.cc methods split cleanly:
//!
//!   * **Pure data-type lattice** (`cast_standard`, `arithmetic_output_standard`,
//!     `is_subpiece_cast`, `is_subpiece_cast_endian`, `is_sext_cast`,
//!     `is_zext_cast`) — depend only on the [`Datatype`] lattice and the
//!     [`TypeFactory`].  These are transcribed *directly* against the W6
//!     `dtype`/`typeop` surface and are exactly what `testtypes.cc`'s `cast_*`
//!     tests validate (through `TypeOp::getInputCast`, which calls
//!     `cast_standard`).
//!
//!   * **IR-walking promotion decisions** (`local_extension_type`,
//!     `int_promotion_type`, `check_int_promotion_for_compare`,
//!     `check_int_promotion_for_extension`, `is_extension_cast_implied`,
//!     `mark_explicit_unsigned`, `mark_explicit_long_size`) — recurse the
//!     data-flow graph through `vn->getDef()`, `op->getIn()`,
//!     `vn->loneDescend()`, `vn->getHighTypeReadFacing(op)`.  Per ADR 0001 the
//!     IR is `Funcdata`-owned, so (mirroring `variable::HighContext`) these
//!     take a [`CastContext`] trait abstracting the read-only IR queries
//!     cast.cc makes.  The control flow, iteration order, and tie-breakers are
//!     transcribed line-for-line from the C++; only the *accessor surface* is
//!     indirected so the decisions are unit-testable now and so the W9
//!     `TypeOp::getInputCast`/`PrintC` bodies can wire a real `Funcdata`-backed
//!     `CastContext` without touching this logic.
//!
//! ## W9 closure (the m1-bodies)
//!
//! `TypeOp::getInputCast`/`getOutputCast` per op-code (typeop.cc) and the
//! `PrintC` cast emission are the W9 callers of this module; they are
//! stub-noted in `typeop.rs`.  The in-module tests below port the
//! `cast_basic`/`cast_pointer`/`cast_enum`/`cast_compare`/`cast_integertoken`
//! decision matrices from `testtypes.cc` to pre-validate that closure: the
//! lattice tests call [`CastStrategyC::cast_standard`] directly with the same
//! data-type pairs `getInputCast` would feed it, and the promotion tests drive
//! the IR-walking methods through a mock [`CastContext`].

use std::rc::Rc;

use kuna_base::address::{mostsigbit_set, signbit_negative, uintb_negate};
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactory};

/// Types of integer promotion (C++ `CastStrategy::IntPromotionCode`,
/// cast.hh:48-54).  For many languages small integers are automatically
/// *promoted* to a standard size; these codes describe how an expression is or
/// will be affected by that promotion.  The discriminants are load-bearing:
/// `int_promotion_type` AND-combines them (`UNSIGNED_EXTENSION & SIGNED_EXTENSION
/// == 0`, `EITHER_EXTENSION == UNSIGNED|SIGNED`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum IntPromotionCode {
    /// There is no integer promotion.
    NO_PROMOTION = -1,
    /// The type of integer promotion cannot be determined.
    UNKNOWN_PROMOTION = 0,
    /// The value is promoted using unsigned extension.
    UNSIGNED_EXTENSION = 1,
    /// The value is promoted using signed extension.
    SIGNED_EXTENSION = 2,
    /// The value is promoted using either signed or unsigned extension.
    EITHER_EXTENSION = 3,
}

impl IntPromotionCode {
    /// The raw `int4` value C++ stores and bit-ANDs.  The IR-walking methods
    /// recombine these codes only with `&` against a fixed comparand and never
    /// reconstruct a new code from a combined value (each `&`-test feeds a
    /// `!= 0` predicate, exactly as cast.cc does), so only `as_i32` is needed.
    #[inline]
    pub fn as_i32(self) -> int4 {
        self as int4
    }
}

/// Opaque handle to a Varnode within a [`CastContext`] (a `Varnode *` in C++).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VnRef(pub usize);

/// Opaque handle to a PcodeOp within a [`CastContext`] (a `PcodeOp *` in C++).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpRef(pub usize);

/// A read-only view of the data-flow graph, supplying exactly the Varnode/PcodeOp
/// queries the IR-walking `CastStrategy` methods make.
///
/// Per ADR 0001 the IR lives in `Funcdata`-owned arenas; cast.cc reaches it
/// through `Varnode *`/`PcodeOp *`.  Mirroring `variable::HighContext`, this
/// trait names each C++ accessor cast.cc calls so the promotion logic stays a
/// line-for-line transcription while the IR plumbing (and the W7 HighVariable /
/// union-resolution `getHighTypeReadFacing`) is supplied by the caller.
///
/// Identity newtypes ([`VnRef`]/[`OpRef`]) are plain indices here so the trait
/// has no dependency on the slotmap key types; a `Funcdata`-backed implementor
/// maps them to/from its `VarnodeId`/`OpId`.
pub trait CastContext {
    /// `op->code()` — the op-code of the given op.
    fn op_code(&self, op: OpRef) -> OpCode;
    /// `op->numInput()`.
    fn op_num_input(&self, op: OpRef) -> int4;
    /// `op->getIn(slot)` — the Varnode in the given input slot.
    fn op_in(&self, op: OpRef, slot: int4) -> VnRef;
    /// `op->getOut()` — the output Varnode, or `None` if the op has none.
    fn op_out(&self, op: OpRef) -> Option<VnRef>;
    /// `op->getSlot(vn)` — the input slot of `vn` in `op`.
    fn op_slot(&self, op: OpRef, vn: VnRef) -> int4;

    /// `vn->isConstant()`.
    fn vn_is_constant(&self, vn: VnRef) -> bool;
    /// `vn->isExplicit()`.
    fn vn_is_explicit(&self, vn: VnRef) -> bool;
    /// `vn->isWritten()`.
    fn vn_is_written(&self, vn: VnRef) -> bool;
    /// `vn->getSize()`.
    fn vn_size(&self, vn: VnRef) -> int4;
    /// `vn->getOffset()` — only meaningful for constants.
    fn vn_offset(&self, vn: VnRef) -> uintb;
    /// `vn->getDef()` — the defining op, or `None` if `vn` is an input/free.
    fn vn_def(&self, vn: VnRef) -> Option<OpRef>;
    /// `vn->loneDescend()` — the unique reader op, or `None`.
    fn vn_lone_descend(&self, vn: VnRef) -> Option<OpRef>;
    /// `vn->getHigh()->getType()` — the HighVariable data-type (`getHigh`).
    fn vn_high_type(&self, vn: VnRef) -> Rc<Datatype>;
    /// `vn->getHighTypeReadFacing(op)` — the read-facing HighVariable data-type
    /// (`getHigh()->getType()` resolved against the union read by `op`).
    fn vn_high_type_read_facing(&self, vn: VnRef, op: OpRef) -> Rc<Datatype>;

    /// `op->getOpcode()->inheritsSign()` (the addlflags predicate of the op's
    /// op-code, `typeop::type_op_info(op->code()).inherits_sign()`).
    fn op_inherits_sign(&self, op: OpRef) -> bool;
    /// `op->getOpcode()->inheritsSignFirstParamOnly()`.
    fn op_inherits_sign_first_param_only(&self, op: OpRef) -> bool;
    /// `op->getOpcode()->isShiftOp()`.
    fn op_is_shift_op(&self, op: OpRef) -> bool;
    /// `op->isBoolOutput()` — output is a single-bit boolean value.
    fn op_is_bool_output(&self, op: OpRef) -> bool;
    /// `op->isCall()`.
    fn op_is_call(&self, op: OpRef) -> bool;

    /// `vn->setUnsignedPrint()` — mark a constant for explicit `unsigned`
    /// printing.  Called by `mark_explicit_unsigned`.
    fn vn_set_unsigned_print(&mut self, vn: VnRef);
    /// `vn->setLongPrint()` — mark a constant for explicit `long` printing.
    /// Called by `mark_explicit_long_size`.
    fn vn_set_long_print(&mut self, vn: VnRef);
}

/// The casting-strategy API (C++ `CastStrategy`, cast.hh:42-175).
///
/// `tlst`/`promote_size` are shared base state; the per-language decision methods
/// are the trait methods.  The two `mark_explicit_*` helpers and
/// `cares_about_char_representation` are non-virtual in C++ (defined on the base)
/// and are provided here as default methods.
pub trait CastStrategy {
    /// The size of the `int` data-type — the size integers get promoted to
    /// (C++ `CastStrategy::promoteSize`, set by `setTypeFactory`).
    fn promote_size(&self) -> int4;

    // -- Pure virtuals (per-language) ---------------------------------------

    /// C++ `localExtensionType` — decide integer promotion from just local
    /// properties of `vn` as read by `op`.
    fn local_extension_type(
        &self,
        ctx: &dyn CastContext,
        vn: VnRef,
        op: Option<OpRef>,
    ) -> IntPromotionCode;

    /// C++ `intPromotionType` — the integer-promotion code of `vn`, recursing
    /// through its defining expression as needed.
    fn int_promotion_type(&self, ctx: &dyn CastContext, vn: VnRef) -> IntPromotionCode;

    /// C++ `checkIntPromotionForCompare` — does promotion force a cast before the
    /// comparison `op` makes sense for input `slot`?
    fn check_int_promotion_for_compare(&self, ctx: &dyn CastContext, op: OpRef, slot: int4)
        -> bool;

    /// C++ `checkIntPromotionForExtension` — does promotion force a cast before
    /// the extension `op` (ZEXT/SEXT) makes sense?
    fn check_int_promotion_for_extension(&self, ctx: &dyn CastContext, op: OpRef) -> bool;

    /// C++ `isExtensionCastImplied` — is the ZEXT/SEXT `op` (viewed as a cast)
    /// implied by the expression it sits in (so it need not be printed)?
    fn is_extension_cast_implied(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
        read_op: Option<OpRef>,
    ) -> bool;

    /// C++ `castStandard` — does an assignment from `curtype` to `reqtype` need a
    /// visible cast?  `None` for no cast, else the data-type to cast to.
    fn cast_standard(
        &self,
        reqtype: &Rc<Datatype>,
        curtype: &Rc<Datatype>,
        care_uint_int: bool,
        care_ptr_uint: bool,
    ) -> Option<Rc<Datatype>>;

    /// C++ `arithmeticOutputStandard` — the output data-type produced by integer
    /// arithmetic `op`.
    fn arithmetic_output_standard(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
    ) -> KunaResult<Rc<Datatype>>;

    /// C++ `isSubpieceCast` — is truncating `intype` to `outtype` (SUBPIECE,
    /// byte `offset`) a cast?
    fn is_subpiece_cast(&self, outtype: &Datatype, intype: &Datatype, offset: uint4) -> bool;

    /// C++ `isSubpieceCastEndian` — `isSubpieceCast` adjusted for endianness when
    /// the truncation pulls bytes directly from memory.
    fn is_subpiece_cast_endian(
        &self,
        outtype: &Datatype,
        intype: &Datatype,
        offset: uint4,
        isbigend: bool,
    ) -> bool;

    /// C++ `isSextCast` — is sign-extending `intype` to `outtype` a cast?
    fn is_sext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool;

    /// C++ `isZextCast` — is zero-extending `intype` to `outtype` a cast?
    fn is_zext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool;

    // -- Non-virtual base helpers (default methods) -------------------------

    /// C++ `CastStrategy::markExplicitUnsigned` (cast.cc:38-71) — check if the
    /// constant in slot `slot` of `op` needs to be coerced (as a source token)
    /// into being `unsigned`, marking the Varnode for printing if so.
    fn mark_explicit_unsigned(&self, ctx: &mut dyn CastContext, op: OpRef, slot: int4) -> bool {
        if !ctx.op_inherits_sign(op) {
            return false;
        }
        let inherits_first_param_only = ctx.op_inherits_sign_first_param_only(op);
        if slot == 1 && inherits_first_param_only {
            return false;
        }
        let vn = ctx.op_in(op, slot);
        if !ctx.vn_is_constant(vn) {
            return false;
        }
        let dt = ctx.vn_high_type_read_facing(vn, op);
        let meta = dt.get_metatype();
        if meta != type_metatype::TYPE_UINT
            && meta != type_metatype::TYPE_UNKNOWN
            && meta != type_metatype::TYPE_PARTIALSTRUCT
            && meta != type_metatype::TYPE_PARTIALUNION
        {
            return false;
        }
        if dt.is_char_print() {
            return false;
        }
        if dt.is_enum_type() {
            return false;
        }
        if ctx.op_num_input(op) == 2 && !inherits_first_param_only {
            let firstvn = ctx.op_in(op, 1 - slot);
            let meta = ctx.vn_high_type_read_facing(firstvn, op).get_metatype();
            if meta == type_metatype::TYPE_UINT
                || meta == type_metatype::TYPE_UNKNOWN
                || meta == type_metatype::TYPE_PARTIALSTRUCT
                || meta == type_metatype::TYPE_PARTIALUNION
            {
                return false; // Other side of the operation will force the unsigned
            }
        }
        // Check if type is going to get forced anyway
        if let Some(outvn) = ctx.op_out(op) {
            if ctx.vn_is_explicit(outvn) {
                return false;
            }
            if let Some(lone) = ctx.vn_lone_descend(outvn) {
                if !ctx.op_inherits_sign(lone) {
                    return false;
                }
            }
        }

        ctx.vn_set_unsigned_print(vn);
        true
    }

    /// C++ `CastStrategy::markExplicitLongSize` (cast.cc:79-105) — check if the
    /// constant in slot `slot` of `op` needs to be coerced into a data-type
    /// larger than the base integer (typically `long`), marking it if so.
    fn mark_explicit_long_size(&self, ctx: &mut dyn CastContext, op: OpRef, slot: int4) -> bool {
        if !ctx.op_is_shift_op(op) {
            return false;
        }
        if slot != 0 {
            return false;
        }
        let vn = ctx.op_in(op, slot);
        if !ctx.vn_is_constant(vn) {
            return false;
        }
        if ctx.vn_size(vn) <= self.promote_size() {
            return false;
        }
        let dt = ctx.vn_high_type(vn);
        let meta = dt.get_metatype();
        if meta != type_metatype::TYPE_UINT
            && meta != type_metatype::TYPE_INT
            && meta != type_metatype::TYPE_UNKNOWN
            && meta != type_metatype::TYPE_PARTIALSTRUCT
            && meta != type_metatype::TYPE_PARTIALUNION
        {
            return false;
        }
        let size = ctx.vn_size(vn);
        let mut off: uintb = ctx.vn_offset(vn);
        if meta == type_metatype::TYPE_INT && signbit_negative(off, size) {
            off = uintb_negate(off, size);
            let bit = mostsigbit_set(off);
            if bit >= self.promote_size() * 8 - 1 {
                return false;
            }
        } else {
            let bit = mostsigbit_set(off);
            if bit >= self.promote_size() * 8 {
                return false; // If integer is big enough, it naturally becomes a long
            }
        }

        ctx.vn_set_long_print(vn);
        true
    }

    /// C++ `CastStrategy::caresAboutCharRepresentation` (cast.hh:174) — always
    /// `false` in the base class.
    fn cares_about_char_representation(&self, _vn: VnRef, _op: Option<OpRef>) -> bool {
        false
    }
}

/// Casting strategy specific to the C language (C++ `CastStrategyC`,
/// cast.hh:178-191).
pub struct CastStrategyC {
    tlst: Rc<dyn TypeFactory>,
    promote_size: int4,
}

impl CastStrategyC {
    /// Construct, establishing the data-type factory (C++ `setTypeFactory`,
    /// cast.cc:23-28): `promoteSize = tlst->getSizeOfInt()`.
    pub fn new(tlst: Rc<dyn TypeFactory>) -> CastStrategyC {
        let promote_size = tlst.get_size_of_int();
        CastStrategyC { tlst, promote_size }
    }
}

impl CastStrategy for CastStrategyC {
    fn promote_size(&self) -> int4 {
        self.promote_size
    }

    // C++ CastStrategyC::checkIntPromotionForCompare (cast.cc:107-124).
    fn check_int_promotion_for_compare(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
        slot: int4,
    ) -> bool {
        let vn = ctx.op_in(op, slot);
        let exttype1 = self.int_promotion_type(ctx, vn);
        if exttype1 == IntPromotionCode::NO_PROMOTION {
            return false;
        }
        if exttype1 == IntPromotionCode::UNKNOWN_PROMOTION {
            return true; // If there is promotion and we don't know type, we need a cast
        }

        let exttype2 = self.int_promotion_type(ctx, ctx.op_in(op, 1 - slot));
        if (exttype1.as_i32() & exttype2.as_i32()) != 0 {
            // If both sides share a common extension, then these bits aren't determining factor
            return false;
        }
        if exttype2 == IntPromotionCode::NO_PROMOTION {
            // other side would not have integer promotion, but our side is forcing it
            // but both sides get extended in the same way
            return false;
        }
        true
    }

    // C++ CastStrategyC::checkIntPromotionForExtension (cast.cc:126-138).
    fn check_int_promotion_for_extension(&self, ctx: &dyn CastContext, op: OpRef) -> bool {
        let vn = ctx.op_in(op, 0);
        let exttype = self.int_promotion_type(ctx, vn);
        if exttype == IntPromotionCode::NO_PROMOTION {
            return false;
        }
        if exttype == IntPromotionCode::UNKNOWN_PROMOTION {
            return true; // If there is an extension and we don't know type, we need a cast
        }

        // Test if the promotion extension matches the explicit extension
        if (exttype.as_i32() & IntPromotionCode::UNSIGNED_EXTENSION.as_i32()) != 0
            && ctx.op_code(op) == OpCode::CPUI_INT_ZEXT
        {
            return false;
        }
        if (exttype.as_i32() & IntPromotionCode::SIGNED_EXTENSION.as_i32()) != 0
            && ctx.op_code(op) == OpCode::CPUI_INT_SEXT
        {
            return false;
        }
        true // Otherwise we need a cast before we extend
    }

    // C++ CastStrategyC::localExtensionType (cast.cc:140-176).
    fn local_extension_type(
        &self,
        ctx: &dyn CastContext,
        vn: VnRef,
        op: Option<OpRef>,
    ) -> IntPromotionCode {
        use IntPromotionCode::*;
        // C++ calls vn->getHighTypeReadFacing(op); op is always supplied at the
        // call sites that produce a meaningful read-facing type.  When vn is a
        // constant reached via loneDescend (int_promotion_type), op may be null;
        // the read-facing query then falls back to the plain HighVariable type.
        let meta = match op {
            Some(o) => ctx.vn_high_type_read_facing(vn, o).get_metatype(),
            None => ctx.vn_high_type(vn).get_metatype(),
        };
        let natural: IntPromotionCode = if meta == type_metatype::TYPE_UINT
            || meta == type_metatype::TYPE_BOOL
            || meta == type_metatype::TYPE_UNKNOWN
            || meta == type_metatype::TYPE_PARTIALSTRUCT
            || meta == type_metatype::TYPE_PARTIALUNION
        {
            UNSIGNED_EXTENSION
        } else if meta == type_metatype::TYPE_INT {
            SIGNED_EXTENSION
        } else {
            return UNKNOWN_PROMOTION;
        };
        if ctx.vn_is_constant(vn) {
            if !signbit_negative(ctx.vn_offset(vn), ctx.vn_size(vn)) {
                // If the high-bit is zero
                return EITHER_EXTENSION; // Can be viewed as either extension
            }
            return natural;
        }
        if ctx.vn_is_explicit(vn) {
            return natural;
        }
        if !ctx.vn_is_written(vn) {
            return UNKNOWN_PROMOTION;
        }
        let def_op = ctx.vn_def(vn).expect("written varnode has a def");
        if ctx.op_is_bool_output(def_op) {
            return EITHER_EXTENSION;
        }
        let opc = ctx.op_code(def_op);
        if opc == OpCode::CPUI_CAST || opc == OpCode::CPUI_LOAD || ctx.op_is_call(def_op) {
            return natural;
        }
        if opc == OpCode::CPUI_INT_AND {
            // This is kind of recursing
            let tmpvn = ctx.op_in(def_op, 1);
            if ctx.vn_is_constant(tmpvn) {
                if !signbit_negative(ctx.vn_offset(tmpvn), ctx.vn_size(tmpvn)) {
                    return EITHER_EXTENSION;
                }
                return natural;
            }
        }
        UNKNOWN_PROMOTION
    }

    // C++ CastStrategyC::intPromotionType (cast.cc:178-247).
    fn int_promotion_type(&self, ctx: &dyn CastContext, vn: VnRef) -> IntPromotionCode {
        use IntPromotionCode::*;
        if ctx.vn_size(vn) >= self.promote_size {
            return NO_PROMOTION;
        }
        if ctx.vn_is_constant(vn) {
            return self.local_extension_type(ctx, vn, ctx.vn_lone_descend(vn));
        }
        if ctx.vn_is_explicit(vn) {
            return NO_PROMOTION;
        }
        if !ctx.vn_is_written(vn) {
            return UNKNOWN_PROMOTION;
        }
        let op = ctx.vn_def(vn).expect("written varnode has a def");
        match ctx.op_code(op) {
            OpCode::CPUI_INT_AND => {
                let othervn = ctx.op_in(op, 1);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & UNSIGNED_EXTENSION.as_i32())
                    != 0
                {
                    return UNSIGNED_EXTENSION;
                }
                let othervn = ctx.op_in(op, 0);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & UNSIGNED_EXTENSION.as_i32())
                    != 0
                {
                    return UNSIGNED_EXTENSION; // If either side has zero extension, result has zero extension
                }
            }
            OpCode::CPUI_INT_RIGHT => {
                let othervn = ctx.op_in(op, 0);
                let val = self.local_extension_type(ctx, othervn, Some(op));
                if (val.as_i32() & UNSIGNED_EXTENSION.as_i32()) != 0 {
                    // If the input provably zero extends
                    return val; // then the result is a zero extension (plus possibly a sign extension)
                }
            }
            OpCode::CPUI_INT_SRIGHT => {
                let othervn = ctx.op_in(op, 0);
                let val = self.local_extension_type(ctx, othervn, Some(op));
                if (val.as_i32() & SIGNED_EXTENSION.as_i32()) != 0 {
                    // If input can be construed as a sign-extension
                    return val; // then the result is a sign extension (plus possibly a zero extension)
                }
            }
            OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_REM => {
                let othervn = ctx.op_in(op, 0);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & UNSIGNED_EXTENSION.as_i32())
                    == 0
                {
                    return UNKNOWN_PROMOTION;
                }
                let othervn = ctx.op_in(op, 1);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & UNSIGNED_EXTENSION.as_i32())
                    == 0
                {
                    return UNKNOWN_PROMOTION;
                }
                return UNSIGNED_EXTENSION; // If both sides have zero extension, result has zero extension
            }
            OpCode::CPUI_INT_SDIV | OpCode::CPUI_INT_SREM => {
                let othervn = ctx.op_in(op, 0);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & SIGNED_EXTENSION.as_i32())
                    == 0
                {
                    return UNKNOWN_PROMOTION;
                }
                let othervn = ctx.op_in(op, 1);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & SIGNED_EXTENSION.as_i32())
                    == 0
                {
                    return UNKNOWN_PROMOTION;
                }
                return SIGNED_EXTENSION; // If both sides have sign extension, result has sign extension
            }
            OpCode::CPUI_INT_NEGATE | OpCode::CPUI_INT_2COMP => {
                let othervn = ctx.op_in(op, 0);
                if (self.local_extension_type(ctx, othervn, Some(op)).as_i32()
                    & SIGNED_EXTENSION.as_i32())
                    != 0
                {
                    return SIGNED_EXTENSION;
                }
            }
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_MULT => {}
            _ => return NO_PROMOTION, // No integer promotion at all
        }
        UNKNOWN_PROMOTION
    }

    // C++ CastStrategyC::isExtensionCastImplied (cast.cc:249-298).
    fn is_extension_cast_implied(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
        read_op: Option<OpRef>,
    ) -> bool {
        let out_vn = ctx.op_out(op).expect("extension op has an output");
        if ctx.vn_is_explicit(out_vn) {
            // C++: empty `if` body — falls through to `return false` at the end.
        } else {
            let read_op = match read_op {
                None => return false,
                Some(r) => r,
            };
            let metatype = ctx.vn_high_type_read_facing(out_vn, read_op).get_metatype();
            let slot;
            let other_vn;
            match ctx.op_code(read_op) {
                OpCode::CPUI_PTRADD => { /* break -> fall through to "return true" */ }
                OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_DIV
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL
                | OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL => {
                    slot = ctx.op_slot(read_op, out_vn);
                    other_vn = ctx.op_in(read_op, 1 - slot);
                    // Check if the expression involves an explicit variable of the right integer type
                    if ctx.vn_is_constant(other_vn) {
                        // Integer tokens do not naturally indicate their size, and
                        // integers that are bigger than the promotion size are NOT naturally extended.
                        if ctx.vn_size(other_vn) > self.promote_size {
                            // So if the integer is bigger than the promotion size
                            return false; // The extension cast on the other side must be explicit
                        }
                    } else if !ctx.vn_is_explicit(other_vn) {
                        return false;
                    }
                    if ctx.vn_high_type_read_facing(other_vn, read_op).get_metatype() != metatype {
                        return false;
                    }
                }
                _ => return false,
            }
            return true; // Everything is integer promotion
        }
        false
    }

    // C++ CastStrategyC::castStandard (cast.cc:300-392).
    fn cast_standard(
        &self,
        reqtype: &Rc<Datatype>,
        curtype: &Rc<Datatype>,
        care_uint_int: bool,
        care_ptr_uint: bool,
    ) -> Option<Rc<Datatype>> {
        // Generic casting rules that apply for most ops
        if Rc::ptr_eq(curtype, reqtype) {
            return None; // Types are equal, no cast required
        }
        let mut care_uint_int = care_uint_int;
        if curtype.get_metatype() == type_metatype::TYPE_VOID {
            return Some(Rc::clone(reqtype)); // If coming from "void" (a dereferenced pointer) we need a cast
        }
        let mut reqbase: Rc<Datatype> = Rc::clone(reqtype);
        let mut curbase: Rc<Datatype> = Rc::clone(curtype);
        let mut isptr = false;
        while reqbase.get_metatype() == type_metatype::TYPE_PTR
            && curbase.get_metatype() == type_metatype::TYPE_PTR
        {
            if reqbase.get_word_size() != curbase.get_word_size() {
                return Some(Rc::clone(reqtype));
            }
            let req_space = reqbase.get_pointer_space();
            let cur_space = curbase.get_pointer_space();
            // Faithful C++ nesting: the outer block's implicit else carries the
            // "one pointer doesn't have an address" comment.
            #[allow(clippy::collapsible_if)]
            if !addr_space_eq(&req_space, &cur_space) {
                if req_space.is_some() && cur_space.is_some() {
                    return Some(Rc::clone(reqtype)); // Pointers to different address spaces.  We must cast
                }
                // If one pointer doesn't have an address, assume a conversion to/from
                // sub-type and don't need a cast
            }
            let req_ptr_to = reqbase.get_ptr_to().expect("TYPE_PTR has ptrto");
            let cur_ptr_to = curbase.get_ptr_to().expect("TYPE_PTR has ptrto");
            reqbase = req_ptr_to;
            curbase = cur_ptr_to;
            care_uint_int = true;
            isptr = true;
        }
        // Strip typedefs.
        while let Some(td) = reqbase.get_typedef() {
            let td = Rc::clone(td);
            reqbase = td;
        }
        while let Some(td) = curbase.get_typedef() {
            let td = Rc::clone(td);
            curbase = td;
        }
        if Rc::ptr_eq(&curbase, &reqbase) {
            return None; // Different typedefs could point to the same type
        }
        if reqbase.get_metatype() == type_metatype::TYPE_VOID
            || curbase.get_metatype() == type_metatype::TYPE_VOID
        {
            return None; // Don't cast to or from a void pointer
        }
        if reqbase.get_size() != curbase.get_size() {
            if reqbase.is_variable_length()
                && isptr
                && reqbase.has_same_variable_base(&curbase).unwrap_or(false)
            {
                return None; // Don't need a cast
            }
            return Some(Rc::clone(reqtype)); // Otherwise, always cast change in size
        }
        match reqbase.get_metatype() {
            type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_PARTIALSTRUCT
            | type_metatype::TYPE_PARTIALUNION => {
                // As they are ultimately stripped, treat partials as undefined
                return None;
            }
            type_metatype::TYPE_UINT => {
                if !care_uint_int {
                    let meta = curbase.get_metatype();
                    // Note: meta can be TYPE_UINT if curbase is typedef/enumerated
                    if meta == type_metatype::TYPE_UNKNOWN
                        || meta == type_metatype::TYPE_INT
                        || meta == type_metatype::TYPE_UINT
                        || meta == type_metatype::TYPE_BOOL
                        || meta == type_metatype::TYPE_PARTIALSTRUCT
                        || meta == type_metatype::TYPE_PARTIALUNION
                    {
                        return None;
                    }
                } else {
                    let meta = curbase.get_metatype();
                    if meta == type_metatype::TYPE_UINT || meta == type_metatype::TYPE_BOOL {
                        // Can be TYPE_UINT for typedef/enumerated
                        return None;
                    }
                    if isptr
                        && (meta == type_metatype::TYPE_UNKNOWN
                            || meta == type_metatype::TYPE_PARTIALSTRUCT
                            || meta == type_metatype::TYPE_PARTIALUNION)
                    {
                        return None; // Don't cast pointers to unknown
                    }
                }
                if !care_ptr_uint && curbase.get_metatype() == type_metatype::TYPE_PTR {
                    return None;
                }
            }
            type_metatype::TYPE_INT => {
                if !care_uint_int {
                    let meta = curbase.get_metatype();
                    // Note: meta can be TYPE_INT if curbase is an enumerated type
                    if meta == type_metatype::TYPE_UNKNOWN
                        || meta == type_metatype::TYPE_INT
                        || meta == type_metatype::TYPE_UINT
                        || meta == type_metatype::TYPE_BOOL
                        || meta == type_metatype::TYPE_PARTIALSTRUCT
                        || meta == type_metatype::TYPE_PARTIALUNION
                    {
                        return None;
                    }
                } else {
                    let meta = curbase.get_metatype();
                    if meta == type_metatype::TYPE_INT || meta == type_metatype::TYPE_BOOL {
                        return None; // Can be TYPE_INT for typedef/enumerated/char
                    }
                    if isptr
                        && (meta == type_metatype::TYPE_UNKNOWN
                            || meta == type_metatype::TYPE_PARTIALSTRUCT
                            || meta == type_metatype::TYPE_PARTIALUNION)
                    {
                        return None; // Don't cast pointers to unknown
                    }
                }
            }
            type_metatype::TYPE_CODE => {
                if curbase.get_metatype() == type_metatype::TYPE_CODE {
                    // Don't cast between function pointer and generic code pointer
                    if code_prototype_is_null(&reqbase) {
                        return None;
                    }
                    if code_prototype_is_null(&curbase) {
                        return None;
                    }
                }
            }
            _ => {}
        }

        Some(Rc::clone(reqtype))
    }

    // C++ CastStrategyC::arithmeticOutputStandard (cast.cc:394-409).
    fn arithmetic_output_standard(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
    ) -> KunaResult<Rc<Datatype>> {
        let mut res1 = ctx.vn_high_type_read_facing(ctx.op_in(op, 0), op);
        if res1.get_metatype() == type_metatype::TYPE_BOOL {
            // Treat boolean as if it is cast to an integer
            res1 = self.tlst.get_base(res1.get_size(), type_metatype::TYPE_INT)?;
        }

        let num = ctx.op_num_input(op);
        let mut i = 1;
        while i < num {
            let res2 = ctx.vn_high_type_read_facing(ctx.op_in(op, i), op);
            if res2.get_metatype() == type_metatype::TYPE_BOOL {
                i += 1;
                continue;
            }
            if 0 > res2.type_order(&res1)? {
                res1 = res2;
            }
            i += 1;
        }
        Ok(res1)
    }

    // C++ CastStrategyC::isSubpieceCast (cast.cc:411-432).
    fn is_subpiece_cast(&self, outtype: &Datatype, intype: &Datatype, offset: uint4) -> bool {
        if offset != 0 {
            return false;
        }
        let inmeta = intype.get_metatype();
        if inmeta != type_metatype::TYPE_INT
            && inmeta != type_metatype::TYPE_UINT
            && inmeta != type_metatype::TYPE_UNKNOWN
            && inmeta != type_metatype::TYPE_PTR
            && inmeta != type_metatype::TYPE_PARTIALSTRUCT
            && inmeta != type_metatype::TYPE_PARTIALUNION
        {
            return false;
        }
        let outmeta = outtype.get_metatype();
        if outmeta != type_metatype::TYPE_INT
            && outmeta != type_metatype::TYPE_UINT
            && outmeta != type_metatype::TYPE_UNKNOWN
            && outmeta != type_metatype::TYPE_PTR
            && outmeta != type_metatype::TYPE_FLOAT
        {
            return false;
        }
        if inmeta == type_metatype::TYPE_PTR {
            if outmeta == type_metatype::TYPE_PTR && outtype.get_size() < intype.get_size() {
                return true; // Cast from far pointer to near pointer
            }
            if outmeta != type_metatype::TYPE_INT && outmeta != type_metatype::TYPE_UINT {
                return false; // other casts don't make sense for pointers
            }
        }
        true
    }

    // C++ CastStrategyC::isSubpieceCastEndian (cast.cc:434-441).
    fn is_subpiece_cast_endian(
        &self,
        outtype: &Datatype,
        intype: &Datatype,
        offset: uint4,
        isbigend: bool,
    ) -> bool {
        let mut tmpoff = offset;
        if isbigend {
            // intype->getSize()-1-offset: getSize() is int4, offset uint4.  The
            // C++ does the subtraction in uint4 (offset's type), so cast getSize.
            tmpoff = (intype.get_size() as uint4).wrapping_sub(1).wrapping_sub(offset);
        }
        self.is_subpiece_cast(outtype, intype, tmpoff)
    }

    // C++ CastStrategyC::isSextCast (cast.cc:443-455).
    fn is_sext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool {
        let metaout = outtype.get_metatype();
        if metaout != type_metatype::TYPE_UINT && metaout != type_metatype::TYPE_INT {
            return false;
        }
        let metain = intype.get_metatype();
        // Casting to larger storage always extends based on signedness of the input data-type
        // So the input must be SIGNED in order to treat SEXT as a cast
        if metain != type_metatype::TYPE_INT && metain != type_metatype::TYPE_BOOL {
            return false;
        }
        true
    }

    // C++ CastStrategyC::isZextCast (cast.cc:457-469).
    fn is_zext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool {
        let metaout = outtype.get_metatype();
        if metaout != type_metatype::TYPE_UINT && metaout != type_metatype::TYPE_INT {
            return false;
        }
        let metain = intype.get_metatype();
        // Casting to larger storage always extends based on signedness of the input data-type
        // So the input must be UNSIGNED in order to treat ZEXT as a cast
        if metain != type_metatype::TYPE_UINT && metain != type_metatype::TYPE_BOOL {
            return false;
        }
        true
    }
}

/// Casting strategy specific to the Java language (C++ `CastStrategyJava`,
/// cast.hh:198-202).  Nearly identical to C, but accounts for how object
/// references are encoded as pointer data-types.  Shared with the (deferred)
/// `printjava` back-end: `cast.cc` defines this class, and it is kept here so the
/// Java cast surface is complete even though `PrintJava` itself is deferred.
///
/// Only `cast_standard` and `is_zext_cast` differ from C; the rest inherit
/// `CastStrategyC` behavior (C++ `CastStrategyJava : public CastStrategyC`).
pub struct CastStrategyJava {
    inner: CastStrategyC,
}

impl CastStrategyJava {
    /// Construct, establishing the data-type factory (shares
    /// `CastStrategyC::setTypeFactory`).
    pub fn new(tlst: Rc<dyn TypeFactory>) -> CastStrategyJava {
        CastStrategyJava { inner: CastStrategyC::new(tlst) }
    }
}

impl CastStrategy for CastStrategyJava {
    fn promote_size(&self) -> int4 {
        self.inner.promote_size()
    }

    fn local_extension_type(
        &self,
        ctx: &dyn CastContext,
        vn: VnRef,
        op: Option<OpRef>,
    ) -> IntPromotionCode {
        self.inner.local_extension_type(ctx, vn, op)
    }

    fn int_promotion_type(&self, ctx: &dyn CastContext, vn: VnRef) -> IntPromotionCode {
        self.inner.int_promotion_type(ctx, vn)
    }

    fn check_int_promotion_for_compare(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
        slot: int4,
    ) -> bool {
        self.inner.check_int_promotion_for_compare(ctx, op, slot)
    }

    fn check_int_promotion_for_extension(&self, ctx: &dyn CastContext, op: OpRef) -> bool {
        self.inner.check_int_promotion_for_extension(ctx, op)
    }

    fn is_extension_cast_implied(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
        read_op: Option<OpRef>,
    ) -> bool {
        self.inner.is_extension_cast_implied(ctx, op, read_op)
    }

    fn arithmetic_output_standard(
        &self,
        ctx: &dyn CastContext,
        op: OpRef,
    ) -> KunaResult<Rc<Datatype>> {
        self.inner.arithmetic_output_standard(ctx, op)
    }

    fn is_subpiece_cast(&self, outtype: &Datatype, intype: &Datatype, offset: uint4) -> bool {
        self.inner.is_subpiece_cast(outtype, intype, offset)
    }

    fn is_subpiece_cast_endian(
        &self,
        outtype: &Datatype,
        intype: &Datatype,
        offset: uint4,
        isbigend: bool,
    ) -> bool {
        self.inner.is_subpiece_cast_endian(outtype, intype, offset, isbigend)
    }

    fn is_sext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool {
        self.inner.is_sext_cast(outtype, intype)
    }

    // C++ CastStrategyJava::castStandard (cast.cc:471-531).
    fn cast_standard(
        &self,
        reqtype: &Rc<Datatype>,
        curtype: &Rc<Datatype>,
        care_uint_int: bool,
        _care_ptr_uint: bool,
    ) -> Option<Rc<Datatype>> {
        if Rc::ptr_eq(curtype, reqtype) {
            return None; // Types are equal, no cast required
        }
        // Java has no typedef/pointer-chain stripping: reqbase == reqtype,
        // curbase == curtype throughout.
        let reqbase = reqtype;
        let curbase = curtype;
        if reqbase.get_metatype() == type_metatype::TYPE_PTR
            || curbase.get_metatype() == type_metatype::TYPE_PTR
        {
            return None; // There must be explicit cast op between objects, so assume no cast necessary
        }

        if reqbase.get_metatype() == type_metatype::TYPE_VOID
            || curtype.get_metatype() == type_metatype::TYPE_VOID
        {
            return None; // Don't cast from or to VOID
        }
        if reqbase.get_size() != curbase.get_size() {
            return Some(Rc::clone(reqtype)); // Always cast change in size
        }
        match reqbase.get_metatype() {
            type_metatype::TYPE_UNKNOWN
            | type_metatype::TYPE_PARTIALSTRUCT
            | type_metatype::TYPE_PARTIALUNION => {
                // As they are ultimately stripped, treat partials as undefined
                return None;
            }
            type_metatype::TYPE_UINT => {
                if !care_uint_int {
                    let meta = curbase.get_metatype();
                    // Note: meta can be TYPE_UINT if curbase is typedef/enumerated
                    if meta == type_metatype::TYPE_UNKNOWN
                        || meta == type_metatype::TYPE_INT
                        || meta == type_metatype::TYPE_UINT
                        || meta == type_metatype::TYPE_BOOL
                        || meta == type_metatype::TYPE_PARTIALSTRUCT
                        || meta == type_metatype::TYPE_PARTIALUNION
                    {
                        return None;
                    }
                } else {
                    let meta = curbase.get_metatype();
                    if meta == type_metatype::TYPE_UINT || meta == type_metatype::TYPE_BOOL {
                        // Can be TYPE_UINT for typedef/enumerated
                        return None;
                    }
                }
            }
            type_metatype::TYPE_INT => {
                if !care_uint_int {
                    let meta = curbase.get_metatype();
                    // Note: meta can be TYPE_INT if curbase is an enumerated type
                    if meta == type_metatype::TYPE_UNKNOWN
                        || meta == type_metatype::TYPE_INT
                        || meta == type_metatype::TYPE_UINT
                        || meta == type_metatype::TYPE_BOOL
                        || meta == type_metatype::TYPE_PARTIALSTRUCT
                        || meta == type_metatype::TYPE_PARTIALUNION
                    {
                        return None;
                    }
                } else {
                    let meta = curbase.get_metatype();
                    if meta == type_metatype::TYPE_INT || meta == type_metatype::TYPE_BOOL {
                        return None; // Can be TYPE_INT for typedef/enumerated/char
                    }
                }
            }
            type_metatype::TYPE_CODE => {
                if curbase.get_metatype() == type_metatype::TYPE_CODE {
                    // Don't cast between function pointer and generic code pointer
                    if code_prototype_is_null(reqbase) {
                        return None;
                    }
                    if code_prototype_is_null(curbase) {
                        return None;
                    }
                }
            }
            _ => {}
        }

        Some(Rc::clone(reqtype))
    }

    // C++ CastStrategyJava::isZextCast (cast.cc:533-544).
    fn is_zext_cast(&self, outtype: &Datatype, intype: &Datatype) -> bool {
        let outmeta = outtype.get_metatype();
        if outmeta != type_metatype::TYPE_INT
            && outmeta != type_metatype::TYPE_UINT
            && outmeta != type_metatype::TYPE_BOOL
        {
            return false;
        }
        let inmeta = intype.get_metatype();
        if inmeta != type_metatype::TYPE_INT
            && inmeta != type_metatype::TYPE_UINT
            && inmeta != type_metatype::TYPE_BOOL
        {
            return false; // Non-integer types, print functional ZEXT
        }
        if intype.get_size() == 2 && !intype.is_char_print() {
            return false; // cast is not zext for short
        }
        if intype.get_size() == 1 && inmeta == type_metatype::TYPE_INT {
            return false; // cast is not zext for byte
        }
        if intype.get_size() >= 4 {
            return false; // cast is not zext for int and long
        }
        true
    }
}

/// `reqptr->getSpace() != curptr->getSpace()` — pointer identity of the bound
/// AddrSpace.  Spaces are uniquely allocated by the space manager, so identity is
/// `Rc::ptr_eq`; two `None` spaces compare equal (both are the C++ null pointer).
fn addr_space_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        _ => false,
    }
}

/// `((TypeCode *)base)->getPrototype() == (const FuncProto *)0` — true if the
/// code data-type carries no prototype (a generic code pointer).  The W6 `Code`
/// kind stores `proto: Option<Rc<FuncProto>>`; the prototype model itself is a
/// W6 stub, but the *presence* test cast.cc makes is just the `Option`.
fn code_prototype_is_null(ct: &Datatype) -> bool {
    match &ct.kind {
        DatatypeKind::Code { proto } => proto.is_none(),
        // Caller only reaches here when metatype == TYPE_CODE; any non-Code kind
        // with that metatype would be a malformed type, so treat as "null proto"
        // (matches the C++ which would have a TypeCode here).
        _ => true,
    }
}

#[cfg(test)]
mod tests;
