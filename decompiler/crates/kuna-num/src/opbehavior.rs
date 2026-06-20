//! Port of `decompiler/cpp/opbehavior.hh` + `opbehavior.cc` (W1, item
//! `w1-num-pcode-semantics`): classes describing the behavior of individual
//! p-code operations.
//!
//! Paradigm mapping:
//!
//! - The C++ virtual hierarchy becomes the [`OpBehavior`] trait whose default
//!   methods are the base-class "emulation unimplemented" /
//!   "Cannot recover input" throws; each `OpBehaviorXxx` subclass becomes a
//!   struct implementing the trait with the C++ bodies transcribed exactly.
//!   The plain instances C++ creates directly from the concrete base class
//!   (`new OpBehavior(CPUI_LOAD,false,true)` etc.) are [`OpBehaviorDefault`].
//! - **Translate substitution (noted per the item spec):** the C++ float
//!   behaviors hold a `const Translate *` purely to call
//!   `Translate::getFloatFormat(int4 size)`.  `Translate` arrives with the
//!   sleigh wave, so the registration is parameterized by the
//!   [`FloatFormatProvider`] trait — exactly that one lookup — and the float
//!   behaviors store an `Rc<dyn FloatFormatProvider>` (the C++ raw pointer's
//!   shared, non-owning role).  `Translate` will implement the trait when it
//!   lands.
//! - Exceptions become `Result` (ADR 0004): `EvaluationError` ->
//!   `KunaError::Evaluation`, the base-class `LowlevelError` throws ->
//!   `KunaError::Lowlevel`, same explain strings.
//! - UB-2 (`docs/rust-port/upstream-bugs.md`): the C++ `INT_SDIV`/`INT_SREM`
//!   evaluation SIGFPEs on `INT64_MIN / -1`; the port returns an evaluation
//!   error for that cell (never panics, never wraps) — the golden vectors pin
//!   those cells as `TRAP` rows, with TRAP == error.

use std::rc::Rc;

use kuna_base::address::{
    calc_mask, count_leading_zeros, popcount, sign_extend, sign_extend_sized, signbit_negative,
    uintb_negate, zero_extend,
};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::Wrap;

use crate::float::FloatFormat;
use crate::opcodes::{get_opname, OpCode};

// ---------------------------------------------------------------------------
// FloatFormatProvider (the Translate substitution)
// ---------------------------------------------------------------------------

/// The one slice of C++ `Translate` the float behaviors use:
/// `getFloatFormat(int4 size)` — the FloatFormat for encodings of a given
/// size, or `None` if none is registered (C++ returns a null pointer).
pub trait FloatFormatProvider {
    /// Get the floating-point format for encodings of the given size in
    /// bytes.
    fn get_float_format(&self, size: i32) -> Option<&FloatFormat>;
}

// ---------------------------------------------------------------------------
// The OpBehavior base class
// ---------------------------------------------------------------------------

/// The base-class `OpBehavior::evaluateUnary` throw.
fn unary_unimplemented(opcode: OpCode) -> KunaError {
    let name = get_opname(opcode);
    KunaError::lowlevel(format!("Unary emulation unimplemented for {name}"))
}

/// The base-class `OpBehavior::evaluateBinary` throw.
fn binary_unimplemented(opcode: OpCode) -> KunaError {
    let name = get_opname(opcode);
    KunaError::lowlevel(format!("Binary emulation unimplemented for {name}"))
}

/// The base-class `OpBehavior::evaluateTernary` throw.
fn ternary_unimplemented(opcode: OpCode) -> KunaError {
    let name = get_opname(opcode);
    KunaError::lowlevel(format!("Ternary emulation unimplemented for {name}"))
}

/// The base-class `OpBehavior::recoverInputUnary` / `recoverInputBinary`
/// throw.
fn recover_unimplemented() -> KunaError {
    KunaError::lowlevel("Cannot recover input parameter without loss of information")
}

/// \brief Class encapsulating the action/behavior of specific pcode opcodes
///
/// At the lowest level, a pcode op is one of a small set of opcodes that
/// operate on varnodes (address space, offset, size). Classes derived from
/// this base class encapsulate this basic behavior for each possible opcode.
/// These classes describe the most basic behaviors and include:
///    * evaluate_binary(sizeout, sizein, in1, in2)
///    * evaluate_unary(sizeout, sizein, in1)
///    * recover_input_binary(slot, sizeout, out, sizein, in)
///    * recover_input_unary(sizeout, out, sizein)
///
/// The default method bodies are the concrete C++ base-class implementations
/// (the "unimplemented" / "cannot recover" throws).
pub trait OpBehavior {
    /// \brief Get the opcode for this pcode operation
    fn get_opcode(&self) -> OpCode;

    /// \brief Check if this is a special operator
    ///
    /// If this function returns false, the operation is a normal unary or
    /// binary operation which can be evaluated calling evaluate_binary() or
    /// evaluate_unary(). Otherwise, the operation requires special handling
    /// to emulate properly.
    fn is_special(&self) -> bool {
        false
    }

    /// \brief Check if operator is unary
    ///
    /// The operator can either be evaluated as unary or binary.
    fn is_unary(&self) -> bool;

    /// \brief Emulate the unary op-code on an input value
    ///
    /// \param sizeout is the size of the output in bytes
    /// \param sizein is the size of the input in bytes
    /// \param in1 is the input value
    /// \return the output value
    fn evaluate_unary(&self, _sizeout: i32, _sizein: i32, _in1: u64) -> KunaResult<u64> {
        Err(unary_unimplemented(self.get_opcode()))
    }

    /// \brief Emulate the binary op-code on input values
    ///
    /// \param sizeout is the size of the output in bytes
    /// \param sizein is the size of the inputs in bytes
    /// \param in1 is the first input value
    /// \param in2 is the second input value
    /// \return the output value
    fn evaluate_binary(
        &self,
        _sizeout: i32,
        _sizein: i32,
        _in1: u64,
        _in2: u64,
    ) -> KunaResult<u64> {
        Err(binary_unimplemented(self.get_opcode()))
    }

    /// \brief Emulate the ternary op-code on input values
    ///
    /// \param sizeout is the size of the output in bytes
    /// \param sizein is the size of the inputs in bytes
    /// \param in1 is the first input value
    /// \param in2 is the second input value
    /// \param in3 is the third input value
    /// \return the output value
    fn evaluate_ternary(
        &self,
        _sizeout: i32,
        _sizein: i32,
        _in1: u64,
        _in2: u64,
        _in3: u64,
    ) -> KunaResult<u64> {
        Err(ternary_unimplemented(self.get_opcode()))
    }

    /// \brief Reverse the binary op-code operation, recovering an input value
    ///
    /// If the output value and one of the input values is known, recover the
    /// value of the other input.
    /// \param slot is the input slot to recover
    /// \param sizeout is the size of the output in bytes
    /// \param out is the output value
    /// \param sizein is the size of the inputs in bytes
    /// \param in is the known input value
    /// \return the input value corresponding to the \b slot
    fn recover_input_binary(
        &self,
        _slot: i32,
        _sizeout: i32,
        _out: u64,
        _sizein: i32,
        _in: u64,
    ) -> KunaResult<u64> {
        Err(recover_unimplemented())
    }

    /// \brief Reverse the unary op-code operation, recovering the input value
    ///
    /// If the output value is known, recover the input value.
    /// \param sizeout is the size of the output in bytes
    /// \param out is the output value
    /// \param sizein is the size of the input in bytes
    /// \return the input value
    fn recover_input_unary(&self, _sizeout: i32, _out: u64, _sizein: i32) -> KunaResult<u64> {
        Err(recover_unimplemented())
    }
}

/// The concrete C++ base class: the behaviors `registerInstructions` creates
/// directly via `new OpBehavior(opc,isun)` / `new OpBehavior(opc,isun,isspec)`
/// (special ops and the generic INSERT/ZPULL/SPULL/PTRADD/PTRSUB entries).
/// Every evaluate/recover call lands in the default (throwing) trait bodies.
pub struct OpBehaviorDefault {
    /// the internal enumeration for pcode types
    opcode: OpCode,
    /// true = use unary interfaces, false = use binary
    isunary: bool,
    /// Is op not a normal unary or binary op
    isspecial: bool,
}

impl OpBehaviorDefault {
    /// A behavior constructor (C++ `OpBehavior(OpCode,bool)`).
    ///
    /// This kind of OpBehavior is associated with a particular opcode and is
    /// either unary or binary.
    /// \param opc is the opcode of the behavior
    /// \param isun is \b true if the behavior is unary, \b false if binary
    pub fn new(opc: OpCode, isun: bool) -> Self {
        OpBehaviorDefault { opcode: opc, isunary: isun, isspecial: false }
    }

    /// A special behavior constructor (C++ `OpBehavior(OpCode,bool,bool)`).
    ///
    /// This kind of OpBehavior can be set to \b special, if it is neither
    /// unary or binary.
    /// \param opc is the opcode of the behavior
    /// \param isun is \b true if the behavior is unary
    /// \param isspec is \b true if the behavior is neither unary or binary
    pub fn new_special(opc: OpCode, isun: bool, isspec: bool) -> Self {
        OpBehaviorDefault { opcode: opc, isunary: isun, isspecial: isspec }
    }
}

impl OpBehavior for OpBehaviorDefault {
    fn get_opcode(&self) -> OpCode {
        self.opcode
    }
    fn is_special(&self) -> bool {
        self.isspecial
    }
    fn is_unary(&self) -> bool {
        self.isunary
    }
}

// ---------------------------------------------------------------------------
// A class for each opcode
// ---------------------------------------------------------------------------

/// CPUI_COPY behavior
pub struct OpBehaviorCopy;

impl OpBehavior for OpBehaviorCopy {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_COPY
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, _sizein: i32, in1: u64) -> KunaResult<u64> {
        Ok(in1)
    }

    fn recover_input_unary(&self, _sizeout: i32, out: u64, _sizein: i32) -> KunaResult<u64> {
        Ok(out)
    }
}

/// CPUI_INT_EQUAL behavior
pub struct OpBehaviorEqual;

impl OpBehavior for OpBehaviorEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_EQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64 = if in1 == in2 { 1 } else { 0 };
        Ok(res)
    }
}

/// CPUI_INT_NOTEQUAL behavior
pub struct OpBehaviorNotEqual;

impl OpBehavior for OpBehaviorNotEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_NOTEQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64 = if in1 != in2 { 1 } else { 0 };
        Ok(res)
    }
}

/// CPUI_INT_SLESS behavior
pub struct OpBehaviorIntSless;

impl OpBehavior for OpBehaviorIntSless {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SLESS
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64;
        if sizein <= 0 {
            res = 0;
        } else {
            // mask = 0x80 << 8*(sizein-1): a uintb shift; counts >= 64
            // (sizein > 8) are C++ UB, wshl matches the oracle's x86 masking
            let mut mask: u64 = 0x80;
            mask = mask.wshl((8 * (sizein - 1)) as u32); // cast: shift count
            let bit1 = in1 & mask; // Get the sign bits
            let bit2 = in2 & mask;
            if bit1 != bit2 {
                res = if bit1 != 0 { 1 } else { 0 };
            } else {
                res = if in1 < in2 { 1 } else { 0 };
            }
        }
        Ok(res)
    }
}

/// CPUI_INT_SLESSEQUAL behavior
pub struct OpBehaviorIntSlessEqual;

impl OpBehavior for OpBehaviorIntSlessEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SLESSEQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64;
        if sizein <= 0 {
            res = 0;
        } else {
            let mut mask: u64 = 0x80;
            mask = mask.wshl((8 * (sizein - 1)) as u32); // cast: shift count
            let bit1 = in1 & mask; // Get the sign bits
            let bit2 = in2 & mask;
            if bit1 != bit2 {
                res = if bit1 != 0 { 1 } else { 0 };
            } else {
                res = if in1 <= in2 { 1 } else { 0 };
            }
        }
        Ok(res)
    }
}

/// CPUI_INT_LESS behavior
pub struct OpBehaviorIntLess;

impl OpBehavior for OpBehaviorIntLess {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_LESS
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64 = if in1 < in2 { 1 } else { 0 };
        Ok(res)
    }
}

/// CPUI_INT_LESSEQUAL behavior
pub struct OpBehaviorIntLessEqual;

impl OpBehavior for OpBehaviorIntLessEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_LESSEQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64 = if in1 <= in2 { 1 } else { 0 };
        Ok(res)
    }
}

/// CPUI_INT_ZEXT behavior
pub struct OpBehaviorIntZext;

impl OpBehavior for OpBehaviorIntZext {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_ZEXT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, _sizein: i32, in1: u64) -> KunaResult<u64> {
        Ok(in1)
    }

    fn recover_input_unary(&self, _sizeout: i32, out: u64, sizein: i32) -> KunaResult<u64> {
        let mask = calc_mask(sizein);
        if (mask & out) != out {
            return Err(KunaError::evaluation("Output is not in range of zext operation"));
        }
        Ok(out)
    }
}

/// CPUI_INT_SEXT behavior
pub struct OpBehaviorIntSext;

impl OpBehavior for OpBehaviorIntSext {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SEXT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        // C++ calls the uintb sign_extend(in,sizein,sizeout) overload
        let res = sign_extend_sized(in1, sizein, sizeout);
        Ok(res)
    }

    fn recover_input_unary(&self, sizeout: i32, out: u64, sizein: i32) -> KunaResult<u64> {
        let masklong = calc_mask(sizeout);
        let maskshort = calc_mask(sizein);

        if (out & (maskshort ^ (maskshort >> 1))) == 0 {
            // Positive input
            if (out & maskshort) != out {
                return Err(KunaError::evaluation("Output is not in range of sext operation"));
            }
        } else {
            // Negative input
            if (out & (masklong ^ maskshort)) != (masklong ^ maskshort) {
                return Err(KunaError::evaluation("Output is not in range of sext operation"));
            }
        }
        Ok(out & maskshort)
    }
}

/// CPUI_INT_ADD behavior
pub struct OpBehaviorIntAdd;

impl OpBehavior for OpBehaviorIntAdd {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_ADD
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wadd(in2) & calc_mask(sizeout);
        Ok(res)
    }

    fn recover_input_binary(
        &self,
        _slot: i32,
        sizeout: i32,
        out: u64,
        _sizein: i32,
        in_: u64,
    ) -> KunaResult<u64> {
        let res = out.wsub(in_) & calc_mask(sizeout);
        Ok(res)
    }
}

/// CPUI_INT_SUB behavior
pub struct OpBehaviorIntSub;

impl OpBehavior for OpBehaviorIntSub {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SUB
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wsub(in2) & calc_mask(sizeout);
        Ok(res)
    }

    fn recover_input_binary(
        &self,
        slot: i32,
        sizeout: i32,
        out: u64,
        _sizein: i32,
        in_: u64,
    ) -> KunaResult<u64> {
        let mut res: u64;
        if slot == 0 {
            res = in_.wadd(out);
        } else {
            res = in_.wsub(out);
        }
        res &= calc_mask(sizeout);
        Ok(res)
    }
}

/// CPUI_INT_CARRY behavior
pub struct OpBehaviorIntCarry;

impl OpBehavior for OpBehaviorIntCarry {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_CARRY
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res: u64 = if in1 > (in1.wadd(in2) & calc_mask(sizein)) { 1 } else { 0 };
        Ok(res)
    }
}

/// CPUI_INT_SCARRY behavior
pub struct OpBehaviorIntScarry;

impl OpBehavior for OpBehaviorIntScarry {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SCARRY
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wadd(in2);

        // sign-bit extraction: shift count sizein*8-1 (sizein <= 0 is C++
        // UB; wshr matches the oracle's x86 count masking)
        let mut a = (in1.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit
        let b = (in2.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit
        let mut r = (res.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit

        r ^= a;
        a ^= b;
        a ^= 1;
        r &= a;
        Ok(u64::from(r))
    }
}

/// CPUI_INT_SBORROW behavior
pub struct OpBehaviorIntSborrow;

impl OpBehavior for OpBehaviorIntSborrow {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SBORROW
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wsub(in2);

        let mut a = (in1.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit
        let b = (in2.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit
        let mut r = (res.wshr((sizein * 8 - 1) as u32) & 1) as u32; // cast: uintb -> uint4, value is 0/1; Grab sign bit

        a ^= r;
        r ^= b;
        r ^= 1;
        a &= r;
        Ok(u64::from(a))
    }
}

/// CPUI_INT_2COMP behavior
pub struct OpBehaviorInt2Comp;

impl OpBehavior for OpBehaviorInt2Comp {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_2COMP
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let res = uintb_negate(in1.wsub(1), sizein);
        Ok(res)
    }

    fn recover_input_unary(&self, _sizeout: i32, out: u64, sizein: i32) -> KunaResult<u64> {
        let res = uintb_negate(out.wsub(1), sizein);
        Ok(res)
    }
}

/// CPUI_INT_NEGATE behavior
pub struct OpBehaviorIntNegate;

impl OpBehavior for OpBehaviorIntNegate {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_NEGATE
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let res = uintb_negate(in1, sizein);
        Ok(res)
    }

    fn recover_input_unary(&self, _sizeout: i32, out: u64, sizein: i32) -> KunaResult<u64> {
        let res = uintb_negate(out, sizein);
        Ok(res)
    }
}

/// CPUI_INT_XOR behavior
pub struct OpBehaviorIntXor;

impl OpBehavior for OpBehaviorIntXor {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_XOR
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 ^ in2)
    }
}

/// CPUI_INT_AND behavior
pub struct OpBehaviorIntAnd;

impl OpBehavior for OpBehaviorIntAnd {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_AND
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 & in2)
    }
}

/// CPUI_INT_OR behavior
pub struct OpBehaviorIntOr;

impl OpBehavior for OpBehaviorIntOr {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_OR
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 | in2)
    }
}

/// CPUI_INT_LEFT behavior
pub struct OpBehaviorIntLeft;

impl OpBehavior for OpBehaviorIntLeft {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_LEFT
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        // mixed comparison: uintb in2 vs int4 sizeout*8 — the int converts
        // to uintb by sign-extension (a negative product compares huge)
        if in2 >= sizeout.wmul(8) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Ok(0);
        }
        // in2 < sizeout*8 here; counts >= 64 only for sizeout > 8 (C++ UB,
        // x86 masks the count — wshl matches)
        let res = in1.wshl(in2 as u32) & calc_mask(sizeout); // cast: shift count < 64 for real sizes
        Ok(res)
    }

    fn recover_input_binary(
        &self,
        slot: i32,
        sizeout: i32,
        out: u64,
        _sizein: i32,
        in_: u64,
    ) -> KunaResult<u64> {
        // mixed comparison: uintb in vs int4 sizeout*8 (sign-extends)
        if slot != 0 || in_ >= sizeout.wmul(8) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Err(recover_unimplemented());
        }
        let sa = in_ as i32; // cast: C++ `int4 sa = in` truncation (in < sizeout*8 here)
        // 8*sizeout-sa can hit 64 (sa=0, sizeout=8): C++ UB; the oracle's
        // x86 shift masks the count, which wshl reproduces
        if (out.wshl((8 * sizeout - sa) as u32) & calc_mask(sizeout)) != 0 {
            // cast above: shift count
            return Err(KunaError::evaluation("Output is not in range of left shift operation"));
        }
        Ok(out.wshr(sa as u32)) // cast: shift count < 64
    }
}

/// CPUI_INT_RIGHT behavior
pub struct OpBehaviorIntRight;

impl OpBehavior for OpBehaviorIntRight {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_RIGHT
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        // mixed comparison: uintb in2 vs int4 sizeout*8 (sign-extends)
        if in2 >= sizeout.wmul(8) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Ok(0);
        }
        let res = (in1 & calc_mask(sizeout)).wshr(in2 as u32); // cast: shift count < 64 for real sizes
        Ok(res)
    }

    fn recover_input_binary(
        &self,
        slot: i32,
        sizeout: i32,
        out: u64,
        sizein: i32,
        in_: u64,
    ) -> KunaResult<u64> {
        // mixed comparison: uintb in vs int4 sizeout*8 (sign-extends)
        if slot != 0 || in_ >= sizeout.wmul(8) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Err(recover_unimplemented());
        }

        let sa = in_ as i32; // cast: C++ `int4 sa = in` truncation (in < sizeout*8 here)
        // 8*sizein-sa can hit 64 (sa=0, sizein=8): C++ UB, x86-masked count
        if out.wshr((8 * sizein - sa) as u32) != 0 {
            // cast above: shift count
            return Err(KunaError::evaluation("Output is not in range of right shift operation"));
        }
        Ok(out.wshl(sa as u32)) // cast: shift count < 64
    }
}

/// CPUI_INT_SRIGHT behavior
pub struct OpBehaviorIntSright;

impl OpBehavior for OpBehaviorIntSright {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SRIGHT
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        // mixed comparison: uintb in2 vs int4 8*sizeout (sign-extends)
        if in2 >= 8i32.wmul(sizeout) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Ok(if signbit_negative(in1, sizein) { calc_mask(sizeout) } else { 0 });
        }

        let res: u64;
        if signbit_negative(in1, sizein) {
            let r = in1.wshr(in2 as u32); // cast: shift count < 8*sizeout here
            let mut mask = calc_mask(sizein);
            mask = mask.wshr(in2 as u32) ^ mask; // cast: shift count < 8*sizeout here
            res = r | mask;
        } else {
            res = in1.wshr(in2 as u32); // cast: shift count < 8*sizeout here
        }
        Ok(res)
    }

    fn recover_input_binary(
        &self,
        slot: i32,
        sizeout: i32,
        out: u64,
        sizein: i32,
        in_: u64,
    ) -> KunaResult<u64> {
        // mixed comparison: uintb in vs int4 sizeout*8 (sign-extends)
        if slot != 0 || in_ >= sizeout.wmul(8) as i64 as u64 {
            // cast: int4 -> uintb sign-extension as in C++
            return Err(recover_unimplemented());
        }

        let sa = in_ as i32; // cast: C++ `int4 sa = in` truncation (in < sizeout*8 here)
        // sizein*8-sa-1 can go negative when sizein < sizeout: C++ UB, the
        // oracle's x86 shift masks the (two's-complement) count — wshr matches
        let mut testval = out.wshr((sizein * 8 - sa - 1) as u32); // cast: shift count, see above
        let mut count: i32 = 0;
        let mut i: i32 = 0;
        while i <= sa {
            if (testval & 1) != 0 {
                count += 1;
            }
            testval >>= 1;
            i += 1;
        }
        if count != sa + 1 {
            return Err(KunaError::evaluation("Output is not in range of right shift operation"));
        }
        Ok(out.wshl(sa as u32)) // cast: shift count < 64
    }
}

/// CPUI_INT_MULT behavior
pub struct OpBehaviorIntMult;

impl OpBehavior for OpBehaviorIntMult {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_MULT
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wmul(in2) & calc_mask(sizeout);
        Ok(res)
    }
}

/// CPUI_INT_DIV behavior
pub struct OpBehaviorIntDiv;

impl OpBehavior for OpBehaviorIntDiv {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_DIV
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        if in2 == 0 {
            return Err(KunaError::evaluation("Divide by 0"));
        }
        Ok(in1 / in2)
    }
}

/// CPUI_INT_SDIV behavior
pub struct OpBehaviorIntSdiv;

impl OpBehavior for OpBehaviorIntSdiv {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SDIV
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        if in2 == 0 {
            return Err(KunaError::evaluation("Divide by 0"));
        }
        let num = sign_extend(in1 as i64, 8 * sizein - 1); // cast: uintb -> intb reinterpret (C++ implicit); Convert to signed
        let denom = sign_extend(in2 as i64, 8 * sizein - 1); // cast: uintb -> intb reinterpret (C++ implicit)
        // UB-2 (docs/rust-port/upstream-bugs.md): the C++ host division
        // SIGFPEs on INT64_MIN / -1; the port returns an evaluation error
        // instead of trapping (golden vectors pin these cells as TRAP rows,
        // TRAP == error).  It must NOT panic and must NOT wrap.
        if num == i64::MIN && denom == -1 {
            return Err(KunaError::evaluation("Signed division overflow"));
        }
        let sres = num / denom; // Do the signed division
        let sres = zero_extend(sres, 8 * sizeout - 1); // Cut to appropriate size
        Ok(sres as u64) // cast: intb -> uintb reinterpret; Recast as unsigned
    }
}

/// CPUI_INT_REM behavior
pub struct OpBehaviorIntRem;

impl OpBehavior for OpBehaviorIntRem {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_REM
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        if in2 == 0 {
            return Err(KunaError::evaluation("Remainder by 0"));
        }
        let res = in1 % in2;
        Ok(res)
    }
}

/// CPUI_INT_SREM behavior
pub struct OpBehaviorIntSrem;

impl OpBehavior for OpBehaviorIntSrem {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_INT_SREM
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        if in2 == 0 {
            return Err(KunaError::evaluation("Remainder by 0"));
        }
        let val = sign_extend(in1 as i64, 8 * sizein - 1); // cast: uintb -> intb reinterpret (C++ implicit); Convert inputs to signed values
        let mod_ = sign_extend(in2 as i64, 8 * sizein - 1); // cast: uintb -> intb reinterpret (C++ implicit)
        // UB-2 (docs/rust-port/upstream-bugs.md): INT64_MIN % -1 raises the
        // same x86 #DE trap as the division; return an error, never panic.
        if val == i64::MIN && mod_ == -1 {
            return Err(KunaError::evaluation("Signed division overflow"));
        }
        let sres = val % mod_; // Do the remainder
        let sres = zero_extend(sres, 8 * sizeout - 1); // Convert back to unsigned
        Ok(sres as u64) // cast: intb -> uintb reinterpret
    }
}

/// CPUI_BOOL_NEGATE behavior
pub struct OpBehaviorBoolNegate;

impl OpBehavior for OpBehaviorBoolNegate {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_BOOL_NEGATE
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, _sizein: i32, in1: u64) -> KunaResult<u64> {
        Ok(in1 ^ 1)
    }
}

/// CPUI_BOOL_XOR behavior
pub struct OpBehaviorBoolXor;

impl OpBehavior for OpBehaviorBoolXor {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_BOOL_XOR
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 ^ in2)
    }
}

/// CPUI_BOOL_AND behavior
pub struct OpBehaviorBoolAnd;

impl OpBehavior for OpBehaviorBoolAnd {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_BOOL_AND
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 & in2)
    }
}

/// CPUI_BOOL_OR behavior
pub struct OpBehaviorBoolOr;

impl OpBehavior for OpBehaviorBoolOr {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_BOOL_OR
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        Ok(in1 | in2)
    }
}

/// CPUI_FLOAT_EQUAL behavior
pub struct OpBehaviorFloatEqual {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatEqual {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatEqual { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_EQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_equal(in1, in2))
    }
}

/// CPUI_FLOAT_NOTEQUAL behavior
pub struct OpBehaviorFloatNotEqual {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatNotEqual {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatNotEqual { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatNotEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_NOTEQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_not_equal(in1, in2))
    }
}

/// CPUI_FLOAT_LESS behavior
pub struct OpBehaviorFloatLess {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatLess {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatLess { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatLess {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_LESS
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_less(in1, in2))
    }
}

/// CPUI_FLOAT_LESSEQUAL behavior
pub struct OpBehaviorFloatLessEqual {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatLessEqual {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatLessEqual { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatLessEqual {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_LESSEQUAL
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_less_equal(in1, in2))
    }
}

/// CPUI_FLOAT_NAN behavior
pub struct OpBehaviorFloatNan {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatNan {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatNan { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatNan {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_NAN
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_nan(in1))
    }
}

/// CPUI_FLOAT_ADD behavior
pub struct OpBehaviorFloatAdd {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatAdd {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatAdd { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatAdd {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_ADD
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_add(in1, in2))
    }
}

/// CPUI_FLOAT_DIV behavior
pub struct OpBehaviorFloatDiv {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatDiv {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatDiv { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatDiv {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_DIV
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_div(in1, in2))
    }
}

/// CPUI_FLOAT_MULT behavior
pub struct OpBehaviorFloatMult {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatMult {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatMult { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatMult {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_MULT
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_mult(in1, in2))
    }
}

/// CPUI_FLOAT_SUB behavior
pub struct OpBehaviorFloatSub {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatSub {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatSub { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatSub {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_SUB
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, _sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(binary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_sub(in1, in2))
    }
}

/// CPUI_FLOAT_NEG behavior
pub struct OpBehaviorFloatNeg {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatNeg {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatNeg { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatNeg {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_NEG
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_neg(in1))
    }
}

/// CPUI_FLOAT_ABS behavior
pub struct OpBehaviorFloatAbs {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatAbs {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatAbs { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatAbs {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_ABS
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_abs(in1))
    }
}

/// CPUI_FLOAT_SQRT behavior
pub struct OpBehaviorFloatSqrt {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatSqrt {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatSqrt { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatSqrt {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_SQRT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_sqrt(in1))
    }
}

/// CPUI_FLOAT_INT2FLOAT behavior
pub struct OpBehaviorFloatInt2Float {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatInt2Float {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatInt2Float { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatInt2Float {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_INT2FLOAT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        // NOTE: the format is looked up by the OUTPUT size
        let format = match self.translate.get_float_format(sizeout) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_int2float(in1, sizein))
    }
}

/// CPUI_FLOAT_FLOAT2FLOAT behavior
pub struct OpBehaviorFloatFloat2Float {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatFloat2Float {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatFloat2Float { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatFloat2Float {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_FLOAT2FLOAT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let formatout = match self.translate.get_float_format(sizeout) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        let formatin = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(formatin.op_float2float(in1, formatout))
    }
}

/// CPUI_FLOAT_TRUNC behavior
pub struct OpBehaviorFloatTrunc {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatTrunc {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatTrunc { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatTrunc {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_TRUNC
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_trunc(in1, sizeout))
    }
}

/// CPUI_FLOAT_CEIL behavior
pub struct OpBehaviorFloatCeil {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatCeil {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatCeil { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatCeil {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_CEIL
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_ceil(in1))
    }
}

/// CPUI_FLOAT_FLOOR behavior
pub struct OpBehaviorFloatFloor {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatFloor {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatFloor { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatFloor {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_FLOOR
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_floor(in1))
    }
}

/// CPUI_FLOAT_ROUND behavior
pub struct OpBehaviorFloatRound {
    /// Float-format provider standing in for the C++ `const Translate *`
    translate: Rc<dyn FloatFormatProvider>,
}

impl OpBehaviorFloatRound {
    /// Constructor
    pub fn new(trans: Rc<dyn FloatFormatProvider>) -> Self {
        OpBehaviorFloatRound { translate: trans }
    }
}

impl OpBehavior for OpBehaviorFloatRound {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_FLOAT_ROUND
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        let format = match self.translate.get_float_format(sizein) {
            Some(f) => f,
            None => return Err(unary_unimplemented(self.get_opcode())),
        };
        Ok(format.op_round(in1))
    }
}

/// CPUI_PIECE behavior
pub struct OpBehaviorPiece;

impl OpBehavior for OpBehaviorPiece {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_PIECE
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        // shift count (sizeout-sizein)*8 goes negative for the golden
        // sizeout==1 rows: C++ UB resolved by the oracle's x86 `shl` masking
        // the (two's-complement) count to 6 bits — wshl reproduces exactly
        // that (e.g. sizein 2, sizeout 1 shifts by -8 & 63 == 56)
        let res = in1.wshl((sizeout - sizein).wmul(8) as u32) | in2; // cast: i32 -> u32 two's-complement count
        Ok(res)
    }
}

/// CPUI_SUBPIECE behavior
pub struct OpBehaviorSubpiece;

impl OpBehavior for OpBehaviorSubpiece {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_SUBPIECE
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        // C++ `in2 >= sizeof(uintb)`: uintb vs size_t, both unsigned
        if in2 >= 8 {
            return Ok(0);
        }
        let res = in1.wshr((in2 * 8) as u32) & calc_mask(sizeout); // cast: in2 < 8 here, count < 64
        Ok(res)
    }
}

/// CPUI_PTRADD behavior
pub struct OpBehaviorPtradd;

impl OpBehavior for OpBehaviorPtradd {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_PTRADD
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_ternary(
        &self,
        sizeout: i32,
        _sizein: i32,
        in1: u64,
        in2: u64,
        in3: u64,
    ) -> KunaResult<u64> {
        let res = in1.wadd(in2.wmul(in3)) & calc_mask(sizeout);
        Ok(res)
    }
}

/// CPUI_PTRSUB behavior
pub struct OpBehaviorPtrsub;

impl OpBehavior for OpBehaviorPtrsub {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_PTRSUB
    }
    fn is_unary(&self) -> bool {
        false
    }

    fn evaluate_binary(&self, sizeout: i32, _sizein: i32, in1: u64, in2: u64) -> KunaResult<u64> {
        let res = in1.wadd(in2) & calc_mask(sizeout);
        Ok(res)
    }
}

/// CPUI_POPCOUNT behavior
pub struct OpBehaviorPopcount;

impl OpBehavior for OpBehaviorPopcount {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_POPCOUNT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, _sizein: i32, in1: u64) -> KunaResult<u64> {
        Ok(popcount(in1) as u64) // cast: int4 -> uintb, popcount is in 0..=64
    }
}

/// CPUI_LZCOUNT behavior
pub struct OpBehaviorLzcount;

impl OpBehavior for OpBehaviorLzcount {
    fn get_opcode(&self) -> OpCode {
        OpCode::CPUI_LZCOUNT
    }
    fn is_unary(&self) -> bool {
        true
    }

    fn evaluate_unary(&self, _sizeout: i32, sizein: i32, in1: u64) -> KunaResult<u64> {
        // C++ `count_leading_zeros(in1) - 8*(sizeof(uintb) - sizein)`:
        // sizeof(uintb) is size_t, so the whole expression computes in
        // unsigned 64-bit arithmetic and wraps if in1 has bits above
        // sizein bytes (callers pass size-masked values)
        let sub = 8u64.wmul(8u64.wsub(sizein as i64 as u64)); // cast: int4 -> size_t sign-extension as in C++
        Ok((count_leading_zeros(in1) as u64).wsub(sub)) // cast: int4 -> size_t, clz is in 0..=64
    }
}

// ---------------------------------------------------------------------------
// registerInstructions
// ---------------------------------------------------------------------------

/// Build all pcode behaviors (C++ `OpBehavior::registerInstructions`).
///
/// This routine generates a vector of OpBehavior objects indexed by opcode.
/// \param inst is the vector of behaviors to be filled
/// \param trans is the float-format provider needed by the floating point
///        behaviors (the C++ `const Translate *`; see module docs for the
///        substitution)
///
/// Entries are `Rc` rather than the C++ raw pointer so a `PcodeOpRaw` can
/// share its behavior with the table (`pcoderaw.rs`).
pub fn register_instructions(
    inst: &mut Vec<Option<Rc<dyn OpBehavior>>>,
    trans: &Rc<dyn FloatFormatProvider>,
) {
    // inst.insert(inst.end(), CPUI_MAX, (OpBehavior *)0)
    for _ in 0..OpCode::CPUI_MAX as usize {
        inst.push(None);
    }

    inst[OpCode::CPUI_COPY as usize] = Some(Rc::new(OpBehaviorCopy));
    inst[OpCode::CPUI_LOAD as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_LOAD, false, true)));
    inst[OpCode::CPUI_STORE as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_STORE, false, true)));
    inst[OpCode::CPUI_BRANCH as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_BRANCH, false, true)));
    inst[OpCode::CPUI_CBRANCH as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CBRANCH, false, true)));
    inst[OpCode::CPUI_BRANCHIND as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_BRANCHIND, false, true)));
    inst[OpCode::CPUI_CALL as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CALL, false, true)));
    inst[OpCode::CPUI_CALLIND as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CALLIND, false, true)));
    inst[OpCode::CPUI_CALLOTHER as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CALLOTHER, false, true)));
    inst[OpCode::CPUI_RETURN as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_RETURN, false, true)));

    inst[OpCode::CPUI_MULTIEQUAL as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_MULTIEQUAL, false, true)));
    inst[OpCode::CPUI_INDIRECT as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_INDIRECT, false, true)));

    inst[OpCode::CPUI_PIECE as usize] = Some(Rc::new(OpBehaviorPiece));
    inst[OpCode::CPUI_SUBPIECE as usize] = Some(Rc::new(OpBehaviorSubpiece));
    inst[OpCode::CPUI_INT_EQUAL as usize] = Some(Rc::new(OpBehaviorEqual));
    inst[OpCode::CPUI_INT_NOTEQUAL as usize] = Some(Rc::new(OpBehaviorNotEqual));
    inst[OpCode::CPUI_INT_SLESS as usize] = Some(Rc::new(OpBehaviorIntSless));
    inst[OpCode::CPUI_INT_SLESSEQUAL as usize] = Some(Rc::new(OpBehaviorIntSlessEqual));
    inst[OpCode::CPUI_INT_LESS as usize] = Some(Rc::new(OpBehaviorIntLess));
    inst[OpCode::CPUI_INT_LESSEQUAL as usize] = Some(Rc::new(OpBehaviorIntLessEqual));
    inst[OpCode::CPUI_INT_ZEXT as usize] = Some(Rc::new(OpBehaviorIntZext));
    inst[OpCode::CPUI_INT_SEXT as usize] = Some(Rc::new(OpBehaviorIntSext));
    inst[OpCode::CPUI_INT_ADD as usize] = Some(Rc::new(OpBehaviorIntAdd));
    inst[OpCode::CPUI_INT_SUB as usize] = Some(Rc::new(OpBehaviorIntSub));
    inst[OpCode::CPUI_INT_CARRY as usize] = Some(Rc::new(OpBehaviorIntCarry));
    inst[OpCode::CPUI_INT_SCARRY as usize] = Some(Rc::new(OpBehaviorIntScarry));
    inst[OpCode::CPUI_INT_SBORROW as usize] = Some(Rc::new(OpBehaviorIntSborrow));
    inst[OpCode::CPUI_INT_2COMP as usize] = Some(Rc::new(OpBehaviorInt2Comp));
    inst[OpCode::CPUI_INT_NEGATE as usize] = Some(Rc::new(OpBehaviorIntNegate));
    inst[OpCode::CPUI_INT_XOR as usize] = Some(Rc::new(OpBehaviorIntXor));
    inst[OpCode::CPUI_INT_AND as usize] = Some(Rc::new(OpBehaviorIntAnd));
    inst[OpCode::CPUI_INT_OR as usize] = Some(Rc::new(OpBehaviorIntOr));
    inst[OpCode::CPUI_INT_LEFT as usize] = Some(Rc::new(OpBehaviorIntLeft));
    inst[OpCode::CPUI_INT_RIGHT as usize] = Some(Rc::new(OpBehaviorIntRight));
    inst[OpCode::CPUI_INT_SRIGHT as usize] = Some(Rc::new(OpBehaviorIntSright));
    inst[OpCode::CPUI_INT_MULT as usize] = Some(Rc::new(OpBehaviorIntMult));
    inst[OpCode::CPUI_INT_DIV as usize] = Some(Rc::new(OpBehaviorIntDiv));
    inst[OpCode::CPUI_INT_SDIV as usize] = Some(Rc::new(OpBehaviorIntSdiv));
    inst[OpCode::CPUI_INT_REM as usize] = Some(Rc::new(OpBehaviorIntRem));
    inst[OpCode::CPUI_INT_SREM as usize] = Some(Rc::new(OpBehaviorIntSrem));

    inst[OpCode::CPUI_BOOL_NEGATE as usize] = Some(Rc::new(OpBehaviorBoolNegate));
    inst[OpCode::CPUI_BOOL_XOR as usize] = Some(Rc::new(OpBehaviorBoolXor));
    inst[OpCode::CPUI_BOOL_AND as usize] = Some(Rc::new(OpBehaviorBoolAnd));
    inst[OpCode::CPUI_BOOL_OR as usize] = Some(Rc::new(OpBehaviorBoolOr));

    inst[OpCode::CPUI_CAST as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CAST, false, true)));
    // NOTE: upstream registers the plain base behavior for PTRADD/PTRSUB
    // (not OpBehaviorPtradd/OpBehaviorPtrsub) — transcribed as-is
    inst[OpCode::CPUI_PTRADD as usize] =
        Some(Rc::new(OpBehaviorDefault::new(OpCode::CPUI_PTRADD, false)));
    inst[OpCode::CPUI_PTRSUB as usize] =
        Some(Rc::new(OpBehaviorDefault::new(OpCode::CPUI_PTRSUB, false)));

    inst[OpCode::CPUI_FLOAT_EQUAL as usize] =
        Some(Rc::new(OpBehaviorFloatEqual::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_NOTEQUAL as usize] =
        Some(Rc::new(OpBehaviorFloatNotEqual::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_LESS as usize] =
        Some(Rc::new(OpBehaviorFloatLess::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_LESSEQUAL as usize] =
        Some(Rc::new(OpBehaviorFloatLessEqual::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_NAN as usize] =
        Some(Rc::new(OpBehaviorFloatNan::new(Rc::clone(trans))));

    inst[OpCode::CPUI_FLOAT_ADD as usize] =
        Some(Rc::new(OpBehaviorFloatAdd::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_DIV as usize] =
        Some(Rc::new(OpBehaviorFloatDiv::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_MULT as usize] =
        Some(Rc::new(OpBehaviorFloatMult::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_SUB as usize] =
        Some(Rc::new(OpBehaviorFloatSub::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_NEG as usize] =
        Some(Rc::new(OpBehaviorFloatNeg::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_ABS as usize] =
        Some(Rc::new(OpBehaviorFloatAbs::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_SQRT as usize] =
        Some(Rc::new(OpBehaviorFloatSqrt::new(Rc::clone(trans))));

    inst[OpCode::CPUI_FLOAT_INT2FLOAT as usize] =
        Some(Rc::new(OpBehaviorFloatInt2Float::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_FLOAT2FLOAT as usize] =
        Some(Rc::new(OpBehaviorFloatFloat2Float::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_TRUNC as usize] =
        Some(Rc::new(OpBehaviorFloatTrunc::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_CEIL as usize] =
        Some(Rc::new(OpBehaviorFloatCeil::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_FLOOR as usize] =
        Some(Rc::new(OpBehaviorFloatFloor::new(Rc::clone(trans))));
    inst[OpCode::CPUI_FLOAT_ROUND as usize] =
        Some(Rc::new(OpBehaviorFloatRound::new(Rc::clone(trans))));
    inst[OpCode::CPUI_SEGMENTOP as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_SEGMENTOP, false, true)));
    inst[OpCode::CPUI_CPOOLREF as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_CPOOLREF, false, true)));
    inst[OpCode::CPUI_NEW as usize] =
        Some(Rc::new(OpBehaviorDefault::new_special(OpCode::CPUI_NEW, false, true)));
    inst[OpCode::CPUI_INSERT as usize] =
        Some(Rc::new(OpBehaviorDefault::new(OpCode::CPUI_INSERT, false)));
    inst[OpCode::CPUI_ZPULL as usize] =
        Some(Rc::new(OpBehaviorDefault::new(OpCode::CPUI_ZPULL, false)));
    inst[OpCode::CPUI_POPCOUNT as usize] = Some(Rc::new(OpBehaviorPopcount));
    inst[OpCode::CPUI_LZCOUNT as usize] = Some(Rc::new(OpBehaviorLzcount));
    inst[OpCode::CPUI_SPULL as usize] =
        Some(Rc::new(OpBehaviorDefault::new(OpCode::CPUI_SPULL, false)));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A provider with no formats registered (all float ops fall back).
    struct NoFormats;
    impl FloatFormatProvider for NoFormats {
        fn get_float_format(&self, _size: i32) -> Option<&FloatFormat> {
            None
        }
    }

    fn table() -> Vec<Option<Rc<dyn OpBehavior>>> {
        let mut inst: Vec<Option<Rc<dyn OpBehavior>>> = Vec::new();
        let trans: Rc<dyn FloatFormatProvider> = Rc::new(NoFormats);
        register_instructions(&mut inst, &trans);
        inst
    }

    #[test]
    fn test_opbehavior_registration_shape() {
        let inst = table();
        assert_eq!(inst.len(), OpCode::CPUI_MAX as usize);
        // Unregistered slots: 0 (no op) and 45 (unused enum slot).
        for (i, slot) in inst.iter().enumerate() {
            match slot {
                Some(beh) => {
                    assert_eq!(beh.get_opcode() as usize, i, "behavior registered at wrong slot");
                }
                None => assert!(i == 0 || i == 45, "missing behavior for opcode {i}"),
            }
        }
        // C++ flag spot-checks: LOAD special, COPY unary, INT_ADD binary,
        // PTRADD registered as the plain (non-special, binary) base class.
        let load = inst[OpCode::CPUI_LOAD as usize].as_ref().unwrap();
        assert!(load.is_special() && !load.is_unary());
        let copy = inst[OpCode::CPUI_COPY as usize].as_ref().unwrap();
        assert!(!copy.is_special() && copy.is_unary());
        let add = inst[OpCode::CPUI_INT_ADD as usize].as_ref().unwrap();
        assert!(!add.is_special() && !add.is_unary());
        let ptradd = inst[OpCode::CPUI_PTRADD as usize].as_ref().unwrap();
        assert!(!ptradd.is_special() && !ptradd.is_unary());
        assert!(ptradd.evaluate_binary(8, 8, 1, 2).is_err());
    }

    #[test]
    fn test_opbehavior_ub2_sdiv_srem_error_not_panic() {
        // UB-2: INT64_MIN / -1 (and % -1) SIGFPE in C++; here they must
        // return Err and must not panic or wrap.
        let sdiv = OpBehaviorIntSdiv;
        let srem = OpBehaviorIntSrem;
        for sizeout in [8, 1] {
            assert!(matches!(
                sdiv.evaluate_binary(sizeout, 8, 0x8000000000000000, 0xffffffffffffffff),
                Err(KunaError::Evaluation { .. })
            ));
            assert!(matches!(
                srem.evaluate_binary(sizeout, 8, 0x8000000000000000, 0xffffffffffffffff),
                Err(KunaError::Evaluation { .. })
            ));
        }
        // The same bit pattern at a smaller size is an ordinary division:
        // sizein 4 sign-extends 0x80000000 to INT32_MIN and -1 stays -1,
        // INT32_MIN / -1 == 0x80000000 in 64-bit arithmetic.
        assert_eq!(sdiv.evaluate_binary(4, 4, 0x80000000, 0xffffffff).unwrap(), 0x80000000);
        // Divide by zero is the EvaluationError path, not a panic.
        assert!(matches!(
            sdiv.evaluate_binary(8, 8, 1, 0),
            Err(KunaError::Evaluation { explain }) if explain == "Divide by 0"
        ));
        assert!(matches!(
            srem.evaluate_binary(8, 8, 1, 0),
            Err(KunaError::Evaluation { explain }) if explain == "Remainder by 0"
        ));
    }

    #[test]
    fn test_opbehavior_base_throw_messages() {
        let beh = OpBehaviorDefault::new(OpCode::CPUI_SPULL, false);
        // UB-1: in C++ this very error path reads past the name table for
        // SPULL; here it formats the canonical name.
        match beh.evaluate_binary(8, 8, 1, 2) {
            Err(KunaError::Lowlevel { explain }) => {
                assert_eq!(explain, "Binary emulation unimplemented for SPULL");
            }
            other => panic!("expected Lowlevel error, got {other:?}"),
        }
        match beh.evaluate_unary(8, 8, 1) {
            Err(KunaError::Lowlevel { explain }) => {
                assert_eq!(explain, "Unary emulation unimplemented for SPULL");
            }
            other => panic!("expected Lowlevel error, got {other:?}"),
        }
        match beh.evaluate_ternary(8, 8, 1, 2, 3) {
            Err(KunaError::Lowlevel { explain }) => {
                assert_eq!(explain, "Ternary emulation unimplemented for SPULL");
            }
            other => panic!("expected Lowlevel error, got {other:?}"),
        }
        match beh.recover_input_binary(0, 8, 1, 8, 2) {
            Err(KunaError::Lowlevel { explain }) => {
                assert_eq!(explain, "Cannot recover input parameter without loss of information");
            }
            other => panic!("expected Lowlevel error, got {other:?}"),
        }
    }

    #[test]
    fn test_opbehavior_recover_paths() {
        // INT_ADD: out = in1 + in2 -> recover the other input.
        assert_eq!(OpBehaviorIntAdd.recover_input_binary(0, 4, 5, 4, 3).unwrap(), 2);
        // wrap: out 0, in 1 -> 0xffffffff at size 4
        assert_eq!(OpBehaviorIntAdd.recover_input_binary(0, 4, 0, 4, 1).unwrap(), 0xffffffff);
        // INT_SUB: slot 0 -> in + out; slot 1 -> in - out.
        assert_eq!(OpBehaviorIntSub.recover_input_binary(0, 4, 2, 4, 3).unwrap(), 5);
        assert_eq!(OpBehaviorIntSub.recover_input_binary(1, 4, 2, 4, 7).unwrap(), 5);
        // COPY / ZEXT pass through; ZEXT rejects out-of-range outputs.
        assert_eq!(OpBehaviorCopy.recover_input_unary(4, 0x1234, 4).unwrap(), 0x1234);
        assert_eq!(OpBehaviorIntZext.recover_input_unary(4, 0xff, 1).unwrap(), 0xff);
        assert!(matches!(
            OpBehaviorIntZext.recover_input_unary(4, 0x100, 1),
            Err(KunaError::Evaluation { .. })
        ));
        // SEXT: negative path recovers the truncated input, positive path
        // rejects values with bits above sizein.
        assert_eq!(OpBehaviorIntSext.recover_input_unary(2, 0xffff, 1).unwrap(), 0xff);
        assert_eq!(OpBehaviorIntSext.recover_input_unary(2, 0x007f, 1).unwrap(), 0x7f);
        assert!(OpBehaviorIntSext.recover_input_unary(2, 0x0100, 1).is_err());
        assert!(OpBehaviorIntSext.recover_input_unary(2, 0x8000, 1).is_err());
        // INT_2COMP / INT_NEGATE involutions.
        assert_eq!(OpBehaviorInt2Comp.recover_input_unary(1, 0xff, 1).unwrap(), 1);
        assert_eq!(OpBehaviorIntNegate.recover_input_unary(1, 0xfe, 1).unwrap(), 1);
    }

    #[test]
    fn test_opbehavior_shift_recover_edges() {
        // INT_LEFT recover: slot != 0 and oversized shifts take the base
        // throw; in-range shifts validate the recovered bits.
        assert!(matches!(
            OpBehaviorIntLeft.recover_input_binary(1, 4, 8, 4, 1),
            Err(KunaError::Lowlevel { .. })
        ));
        assert!(matches!(
            OpBehaviorIntLeft.recover_input_binary(0, 4, 8, 4, 32),
            Err(KunaError::Lowlevel { .. })
        ));
        assert_eq!(OpBehaviorIntLeft.recover_input_binary(0, 4, 8, 4, 3).unwrap(), 1);
        assert!(matches!(
            OpBehaviorIntLeft.recover_input_binary(0, 4, 9, 4, 3),
            Err(KunaError::Evaluation { .. })
        ));
        // INT_RIGHT recover.
        assert_eq!(OpBehaviorIntRight.recover_input_binary(0, 4, 1, 4, 3).unwrap(), 8);
        assert!(matches!(
            OpBehaviorIntRight.recover_input_binary(0, 4, 0x20000000, 4, 3),
            Err(KunaError::Evaluation { .. })
        ));
        // INT_SRIGHT recover: sign bits must replicate across sa+1 places.
        // out 0xc0 at size 1, sa 1: top two bits set -> input 0x80 << 1.
        assert_eq!(OpBehaviorIntSright.recover_input_binary(0, 1, 0xc0, 1, 1).unwrap(), 0x180);
        assert!(matches!(
            OpBehaviorIntSright.recover_input_binary(0, 1, 0x40, 1, 1),
            Err(KunaError::Evaluation { .. })
        ));
    }

    #[test]
    fn test_opbehavior_ptradd_ternary() {
        // The (unregistered upstream) OpBehaviorPtradd class itself.
        assert_eq!(OpBehaviorPtradd.evaluate_ternary(4, 4, 0x10, 3, 4).unwrap(), 0x1c);
        assert_eq!(
            OpBehaviorPtradd.evaluate_ternary(2, 2, 0xffff, 1, 1).unwrap(),
            0 // wraps then masks to sizeout
        );
        assert_eq!(OpBehaviorPtrsub.evaluate_binary(2, 2, 0xffff, 2).unwrap(), 1);
    }
}
