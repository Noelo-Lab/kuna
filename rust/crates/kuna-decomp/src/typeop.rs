//! Port of `decompiler/cpp/typeop.{cc,hh}` — the data-type and behavior
//! information associated with each p-code op-code (S5 type-lattice edges).
//!
//! ## What this module is (and the seam it replaces)
//!
//! The C++ `TypeOp` is a per-op-code object: it bundles an [`OpCode`], a cached
//! property-flag word (`opflags`, the `PcodeOp::*` bits `setOpcode` ORs in), an
//! `addlflags` word (the `inherits_sign`/`shift_op`/... attributes used by the
//! rules), a display `name`, the input/output metatypes (for the
//! `TypeOpBinary`/`TypeOpUnary`/`TypeOpFunc` local-type calculators), and an
//! `OpBehavior` for emulation.  `TypeOp::registerInstructions` builds the
//! `glb->inst[]` vector indexed by op-code value; `PcodeOp::setOpcode` copies the
//! cached `opflags` into the op's `flags`, and the rules read the op-code
//! properties through `op->code()`-keyed lookups into `inst[]`.
//!
//! The W3 IR carried only a thin slice of this in [`crate::seams::TypeOp`]
//! (`opcode`/`flags`/`name`).  Each W6/W8 wave that needed to retype an op built
//! that triple from a small **local** helper transcribing the `typeop.cc`
//! `opflags` literals inline:
//!
//!   * `ruleaction_5::type_op_seam`
//!   * `funcdata_op::Funcdata::w6_type_op`
//!   * `ruleaction_2`/`ruleaction_7`/`ruleaction_8` `typeop_for`/local builders
//!   * `ruleaction_1`/`ruleaction_3`/`ruleaction_4` `opflags_for`/`opcode_seam_flags`
//!
//! This module is the **canonical** table those helpers approximate.
//! [`type_op_info`] is the single source for `(opflags, addlflags, name,
//! metaout, metain, class)` per op-code, transcribed field-for-field from the
//! per-op-code `TypeOp*` constructors in `typeop.cc`; [`type_op_for`] produces
//! the [`crate::seams::TypeOp`] triple `op_set_opcode` wants, and
//! [`register_instructions`] returns the `inst[]` vector the engine docs expect
//! (the W6 seam in `op.rs` notes `inst[opc]` is "filled by `typeop`/`type`").
//!
//! ### W8 cleanup (not done here, per the work order)
//!
//! The local helpers listed above SHOULD migrate to [`type_op_for`] in W8 — they
//! all build the same `(flags,name)` pair this table now owns.  They are **not**
//! deleted here (each lives in a file this porter does not own); the migration is
//! tracked for W8.
//!
//! ## Cross-wave seams (the per-op `propagateType`/`getInputCast`/`getOutputToken`)
//!
//! The full type-propagation surface of `typeop.cc` —
//! `propagateType`/`getInputCast`/`getOutputToken` per op-code and the
//! `propagateAddIn2Out`/`propagateAddPointer`/`propagateAcrossCompare`
//! helpers — is driven by `CastStrategy`, `Varnode::getHighTypeReadFacing`,
//! `TypePointer::downChain`, `TypePointerRel`, the `Architecture`, and the
//! `ConstantPool` — subsystems still unported at the W6-partial boundary (cast.cc
//! is W8; the full `TypeFactory`/`Architecture` are W6/W8).  Those methods are
//! **seam-noted** rather than transcribed against stubs; what this module ports
//! faithfully is the part whose result depends only on the op-code, the operand
//! **sizes**, and the [`TypeFactory`] `getBase` surface:
//!
//!   * the canonical `inst[]` table (`opflags`/`addlflags`/`name`/metatypes),
//!   * the `addlflags` predicates (`isCommutative`/`inheritsSign`/`isShiftOp`/
//!     `isArithmeticOp`/`isLogicalOp`/`isFloatingPointOp`),
//!   * the metatype-based `getOutputLocal`/`getInputLocal` for the generic
//!     binary/unary/func classes and the size-only per-op overrides
//!     (PTRADD/PTRSUB/CBRANCH/INT_LEFT/INT_RIGHT/INT_SRIGHT/INSERT/ZPULL/SPULL/
//!     CPOOLREF),
//!   * the self-contained static `floatSignManipulation`,
//!   * `selectJavaOperators` (the metatype/symbol toggle).
//!
//! The `propagateToPointer`/`propagateFromPointer` statics are deferred to W8
//! with the propagation bodies that call them (they need `TypePointerRel`/
//! `getTypePartialEnum` accessors not exposed here) — see the seam note at the
//! end of the file.

use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::op::pcodeop_flags as pf;
use crate::seams::TypeOp;

/// `TypeOp::addlflags` attribute bits (`typeop.hh:41-48`, the anonymous `enum`
/// in `class TypeOp`).  These are the per-op-code *display/typing* attributes
/// the rules query through `inheritsSign()`/`isShiftOp()`/... — distinct from
/// the `PcodeOp::*` `opflags` bits ([`crate::op::pcodeop_flags`]) that
/// `setOpcode` caches into the op's `flags`.
pub mod typeop_addlflags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Operator token inherits signedness from its inputs.
    pub const inherits_sign: uint4 = 1;
    /// Only inherits sign from first operand, not the second.
    pub const inherits_sign_zero: uint4 = 2;
    /// Shift operation.
    pub const shift_op: uint4 = 4;
    /// Operation involving addition, multiplication, or division.
    pub const arithmetic_op: uint4 = 8;
    /// Logical operation.
    pub const logical_op: uint4 = 0x10;
    /// Floating-point operation.
    pub const floatingpoint_op: uint4 = 0x20;
}

/// Which generic `TypeOp` shape an op-code's information object derives from.
///
/// In C++ this is the class hierarchy: `TypeOpBinary`/`TypeOpUnary`/`TypeOpFunc`
/// each override `getOutputLocal`/`getInputLocal` to return
/// `getBase(size, metaout/metain)`, while the remaining op-codes derive directly
/// from `TypeOp` and use the default `getBase(size, TYPE_UNKNOWN)` (unless they
/// further override with a size-only rule — see [`TypeOpInfo::get_output_local`]
/// / [`TypeOpInfo::get_input_local`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeOpClass {
    /// Derives directly from `TypeOp` (default `TYPE_UNKNOWN` local types).
    Base,
    /// `TypeOpBinary`: `getInputLocal`/`getOutputLocal` use `metain`/`metaout`.
    Binary,
    /// `TypeOpUnary`: `getInputLocal`/`getOutputLocal` use `metain`/`metaout`.
    Unary,
    /// `TypeOpFunc`: `getInputLocal`/`getOutputLocal` use `metain`/`metaout`.
    Func,
}

/// The per-op-code information object (C++ `TypeOp` + its per-op-code subclass
/// fields, minus the unported propagation/print surface).
///
/// Every field is transcribed from the matching `TypeOp*` constructor in
/// `typeop.cc`.  `name` is `&'static str` (the C++ `string name` is a fixed
/// literal per op-code), so the whole record is `Copy` and can be returned by
/// value from [`type_op_info`] without any interior mutability or statics.
#[derive(Debug, Clone, Copy)]
pub struct TypeOpInfo {
    /// The op-code value (C++ `TypeOp::opcode`).
    pub opcode: OpCode,
    /// Cached p-code-op properties (C++ `TypeOp::opflags`, the `PcodeOp::*`
    /// bits `setOpcode` ORs into the op's `flags`).
    pub opflags: uint4,
    /// Additional `TypeOp` attributes (C++ `TypeOp::addlflags`, the
    /// [`typeop_addlflags`] bits).
    pub addlflags: uint4,
    /// Symbol denoting this operation (C++ `TypeOp::name`).
    pub name: &'static str,
    /// Which generic shape governs the local-type calculators.
    pub class: TypeOpClass,
    /// Output metatype for `TypeOpBinary`/`Unary`/`Func` (C++ `metaout`).
    /// Meaningless for [`TypeOpClass::Base`].
    pub metaout: type_metatype,
    /// Input metatype for `TypeOpBinary`/`Unary`/`Func` (C++ `metain`).
    /// Meaningless for [`TypeOpClass::Base`].
    pub metain: type_metatype,
}

use type_metatype::*;
use typeop_addlflags as af;

impl TypeOpInfo {
    /// Build a [`TypeOpClass::Base`] record (no metatypes).
    const fn base(opcode: OpCode, opflags: uint4, addlflags: uint4, name: &'static str) -> Self {
        TypeOpInfo {
            opcode,
            opflags,
            addlflags,
            name,
            class: TypeOpClass::Base,
            // Unused for Base; the C++ default `getBase(size, TYPE_UNKNOWN)`.
            metaout: TYPE_UNKNOWN,
            metain: TYPE_UNKNOWN,
        }
    }

    /// Build a `TypeOpBinary`/`Unary`/`Func` record carrying input/output
    /// metatypes (mirrors the `TypeOpBinary(t,opc,n,mout,min)` constructor).
    const fn typed(
        opcode: OpCode,
        opflags: uint4,
        addlflags: uint4,
        name: &'static str,
        class: TypeOpClass,
        metaout: type_metatype,
        metain: type_metatype,
    ) -> Self {
        TypeOpInfo { opcode, opflags, addlflags, name, class, metaout, metain }
    }

    // -- addlflags predicates (typeop.hh:128-146) ---------------------------

    /// Return `true` if this op-code is commutative (C++ `isCommutative`:
    /// `(opflags & PcodeOp::commutative)!=0`).
    pub fn is_commutative(&self) -> bool {
        (self.opflags & pf::commutative) != 0
    }

    /// Return `true` if the op-code inherits signedness from its inputs
    /// (C++ `inheritsSign`).
    pub fn inherits_sign(&self) -> bool {
        (self.addlflags & af::inherits_sign) != 0
    }

    /// Return `true` if the op-code inherits signedness from only its first
    /// input (C++ `inheritsSignFirstParamOnly`).
    pub fn inherits_sign_first_param_only(&self) -> bool {
        (self.addlflags & af::inherits_sign_zero) != 0
    }

    /// Return `true` if the op-code is a shift (C++ `isShiftOp`).
    pub fn is_shift_op(&self) -> bool {
        (self.addlflags & af::shift_op) != 0
    }

    /// Return `true` if the op-code is arithmetic (C++ `isArithmeticOp`).
    pub fn is_arithmetic_op(&self) -> bool {
        (self.addlflags & af::arithmetic_op) != 0
    }

    /// Return `true` if the op-code is logical (C++ `isLogicalOp`).
    pub fn is_logical_op(&self) -> bool {
        (self.addlflags & af::logical_op) != 0
    }

    /// Return `true` if the op-code is floating-point (C++ `isFloatingPointOp`).
    pub fn is_floating_point_op(&self) -> bool {
        (self.addlflags & af::floatingpoint_op) != 0
    }

    /// Get the op-code value (C++ `TypeOp::getOpcode`).
    pub fn get_opcode(&self) -> OpCode {
        self.opcode
    }

    /// Get the cached op-code properties (C++ `TypeOp::getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.opflags
    }

    /// Get the display name (C++ `TypeOp::getName`).
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Build the [`crate::seams::TypeOp`] triple `PcodeOp::setOpcode`/
    /// `Funcdata::op_set_opcode` consume (the W3 seam carrying
    /// `opcode`/`flags`/`name`).  This is the canonical replacement for the
    /// per-wave `type_op_seam`/`w6_type_op`/`typeop_for` local helpers.
    pub fn to_seam(&self) -> TypeOp {
        TypeOp::new(self.opcode, self.opflags, self.name)
    }

    /// Find the minimal (suggested) data-type of an output to this op-code
    /// (C++ `getOutputLocal`).
    ///
    /// For [`TypeOpClass::Binary`]/[`TypeOpClass::Unary`]/[`TypeOpClass::Func`]
    /// this is `getBase(out_size, metaout)`; the per-op-code overrides whose
    /// result is size-only are handled here too.  The remaining `Base` op-codes
    /// fall to the default `getBase(out_size, TYPE_UNKNOWN)`.
    ///
    /// Op-codes whose `getOutputLocal` consults the prototype/cpool/userop
    /// (CALL/CALLIND/CALLOTHER/CPOOLREF) cannot be resolved from `out_size`
    /// alone; for those this returns the inherited default and the caller must
    /// route through the (unported, W8) prototype-aware path.  See the
    /// per-op-code notes in the source.
    pub fn get_output_local(
        &self,
        tlst: &dyn TypeFactory,
        out_size: int4,
    ) -> KunaResult<Rc<Datatype>> {
        match self.opcode {
            // TypeOpPtradd::getOutputLocal / TypeOpPtrsub::getOutputLocal:
            // getBase(out_size, TYPE_INT) (typeop.cc:2240-2244, 2310-2314).
            OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => tlst.get_base(out_size, TYPE_INT),
            _ => match self.class {
                // TypeOpBinary/Unary/Func::getOutputLocal: getBase(size, metaout).
                TypeOpClass::Binary | TypeOpClass::Unary | TypeOpClass::Func => {
                    tlst.get_base(out_size, self.metaout)
                }
                // TypeOp::getOutputLocal default: getBase(size, TYPE_UNKNOWN).
                TypeOpClass::Base => tlst.get_base(out_size, TYPE_UNKNOWN),
            },
        }
    }

    /// Find the minimal (suggested) data-type of an input to this op-code
    /// (C++ `getInputLocal`), for the size-only / metatype-only cases.
    ///
    /// `in_size` is the size of input `slot`.  Op-codes whose `getInputLocal`
    /// needs more than the input size and the slot (the code-pointer construction
    /// in CBRANCH/CALL/CALLIND/INDIRECT, the prototype/cpool/userop paths) are
    /// noted per-arm; for those the caller must use the (partly unported) richer
    /// path.  The faithfully size-resolvable arms are transcribed here.
    pub fn get_input_local(
        &self,
        tlst: &dyn TypeFactory,
        slot: int4,
        in_size: int4,
    ) -> KunaResult<Rc<Datatype>> {
        match self.opcode {
            // TypeOpIntLeft/IntRight/IntSright::getInputLocal: slot 1 is the
            // shift amount -> getBaseNoChar(size, TYPE_INT); else fall to the
            // TypeOpBinary default getBase(size, metain) (typeop.cc:1512-1518,
            // 1537-1543, 1602-1608).
            OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                if slot == 1 {
                    tlst.get_base_no_char(in_size, TYPE_INT)
                } else {
                    tlst.get_base(in_size, self.metain)
                }
            }
            // TypeOpPtradd::getInputLocal: getBase(size, TYPE_INT) for all slots
            // (typeop.cc:2234-2238).
            OpCode::CPUI_PTRADD => tlst.get_base(in_size, TYPE_INT),
            // TypeOpPtrsub::getInputLocal: getBase(size, TYPE_INT) (typeop.cc:2316-2320).
            OpCode::CPUI_PTRSUB => tlst.get_base(in_size, TYPE_INT),
            // TypeOpCpoolref::getInputLocal: getBase(size, TYPE_INT) (typeop.cc:2467-2471).
            OpCode::CPUI_CPOOLREF => tlst.get_base(in_size, TYPE_INT),
            // TypeOpCbranch::getInputLocal: slot 1 is a bool; slot 0 is a code
            // pointer needing getTypeCode + the in0 space wordsize -> NOT
            // size-only.  Only slot 1 is resolved here (typeop.cc:611-621).
            // SEAM(W8): slot-0 code-pointer path needs the op's address space.
            OpCode::CPUI_CBRANCH if slot == 1 => tlst.get_base(in_size, TYPE_BOOL),
            // TypeOpInsert::getInputLocal: slots <=1 -> getBase(size,TYPE_UNKNOWN);
            // else the TypeOpFunc default getBase(size, metain) (typeop.cc:2537-2543).
            OpCode::CPUI_INSERT => {
                if slot <= 1 {
                    tlst.get_base(in_size, TYPE_UNKNOWN)
                } else {
                    tlst.get_base(in_size, self.metain)
                }
            }
            // TypeOpZpull/Spull::getInputLocal: slot 0 -> getBase(size,TYPE_UNKNOWN);
            // else TypeOpFunc default getBase(size, metain) (typeop.cc:2564-2569, 2591-2596).
            OpCode::CPUI_ZPULL | OpCode::CPUI_SPULL => {
                if slot == 0 {
                    tlst.get_base(in_size, TYPE_UNKNOWN)
                } else {
                    tlst.get_base(in_size, self.metain)
                }
            }
            _ => match self.class {
                // TypeOpBinary/Unary/Func::getInputLocal: getBase(size, metain).
                TypeOpClass::Binary | TypeOpClass::Unary | TypeOpClass::Func => {
                    tlst.get_base(in_size, self.metain)
                }
                // TypeOp::getInputLocal default: getBase(size, TYPE_UNKNOWN).
                TypeOpClass::Base => tlst.get_base(in_size, TYPE_UNKNOWN),
            },
        }
    }
}

/// Look up the canonical per-op-code information record (C++: indexing
/// `glb->inst[opc]`).
///
/// The body is the union of `TypeOp::registerInstructions` (which op-code maps to
/// which subclass) and each `TypeOp*` constructor (the `opflags`/`addlflags`/
/// `name`/`metaout`/`metain` literals).  Every arm cites its `typeop.cc`
/// constructor.  `CPUI_MAX` and the unused discriminant `45` (no `CPUI_*`
/// between FLOAT_LESSEQUAL=44 and FLOAT_NAN=46) have no entry in C++ — the panic
/// arm marks that they must never be queried, matching the C++ `inst[CPUI_MAX]`
/// being a null pointer.
pub fn type_op_info(opc: OpCode) -> TypeOpInfo {
    use OpCode::*;
    use TypeOpClass::{Binary, Func, Unary};
    match opc {
        // -- Control / memory (derive from TypeOp) --------------------------
        // TypeOpCopy: unary|nocollapse (typeop.cc:391-396).
        CPUI_COPY => TypeOpInfo::base(opc, pf::unary | pf::nocollapse, 0, "copy"),
        // TypeOpLoad: special|nocollapse (typeop.cc:434-439).
        CPUI_LOAD => TypeOpInfo::base(opc, pf::special | pf::nocollapse, 0, "load"),
        // TypeOpStore: special|nocollapse (typeop.cc:514-519).
        CPUI_STORE => TypeOpInfo::base(opc, pf::special | pf::nocollapse, 0, "store"),
        // TypeOpBranch: special|branch|coderef|nocollapse (typeop.cc:585-590).
        CPUI_BRANCH => {
            TypeOpInfo::base(opc, pf::special | pf::branch | pf::coderef | pf::nocollapse, 0, "goto")
        }
        // TypeOpCbranch: special|branch|coderef|nocollapse (typeop.cc:604-609).
        CPUI_CBRANCH => {
            TypeOpInfo::base(opc, pf::special | pf::branch | pf::coderef | pf::nocollapse, 0, "goto")
        }
        // TypeOpBranchind: special|branch|nocollapse (typeop.cc:648-653).
        CPUI_BRANCHIND => {
            TypeOpInfo::base(opc, pf::special | pf::branch | pf::nocollapse, 0, "switch")
        }
        // TypeOpCall: special|call|has_callspec|coderef|nocollapse (typeop.cc:662-667).
        CPUI_CALL => TypeOpInfo::base(
            opc,
            pf::special | pf::call | pf::has_callspec | pf::coderef | pf::nocollapse,
            0,
            "call",
        ),
        // TypeOpCallind: special|call|has_callspec|nocollapse (typeop.cc:740-745).
        CPUI_CALLIND => TypeOpInfo::base(
            opc,
            pf::special | pf::call | pf::has_callspec | pf::nocollapse,
            0,
            "callind",
        ),
        // TypeOpCallother: special|call|nocollapse (typeop.cc:813-818).
        CPUI_CALLOTHER => {
            TypeOpInfo::base(opc, pf::special | pf::call | pf::nocollapse, 0, "syscall")
        }
        // TypeOpReturn: special|returns|nocollapse|return_copy (typeop.cc:877-882).
        CPUI_RETURN => TypeOpInfo::base(
            opc,
            pf::special | pf::returns | pf::nocollapse | pf::return_copy,
            0,
            "return",
        ),

        // -- Comparisons (TypeOpBinary, BOOL out) ---------------------------
        // TypeOpEqual: binary|booloutput|commutative; addl inherits_sign (typeop.cc:926-932).
        CPUI_INT_EQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput | pf::commutative,
            af::inherits_sign,
            "==",
            Binary,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpNotEqual: binary|booloutput|commutative; addl inherits_sign (typeop.cc:990-996).
        CPUI_INT_NOTEQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput | pf::commutative,
            af::inherits_sign,
            "!=",
            Binary,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpIntSless: binary|booloutput; addl inherits_sign (typeop.cc:1017-1023).
        CPUI_INT_SLESS => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpIntSlessEqual: binary|booloutput; addl inherits_sign (typeop.cc:1043-1049).
        CPUI_INT_SLESSEQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpIntLess: binary|booloutput; addl inherits_sign; metain UINT (typeop.cc:1069-1075).
        CPUI_INT_LESS => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_UINT,
        ),
        // TypeOpIntLessEqual: binary|booloutput; addl inherits_sign; metain UINT (typeop.cc:1093-1099).
        CPUI_INT_LESSEQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_UINT,
        ),

        // -- Extensions (TypeOpFunc) ----------------------------------------
        // TypeOpIntZext: unary; metaout/in UINT (typeop.cc:1117-1122).
        CPUI_INT_ZEXT => {
            TypeOpInfo::typed(opc, pf::unary, 0, "ZEXT", Func, TYPE_UINT, TYPE_UINT)
        }
        // TypeOpIntSext: unary; metaout/in INT (typeop.cc:1143-1148).
        CPUI_INT_SEXT => {
            TypeOpInfo::typed(opc, pf::unary, 0, "SEXT", Func, TYPE_INT, TYPE_INT)
        }

        // -- Integer arithmetic (TypeOpBinary/Unary) ------------------------
        // TypeOpIntAdd: binary|commutative; addl arithmetic_op|inherits_sign (typeop.cc:1169-1175).
        CPUI_INT_ADD => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::arithmetic_op | af::inherits_sign,
            "+",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntSub: binary; addl arithmetic_op|inherits_sign (typeop.cc:1320-1326).
        CPUI_INT_SUB => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "-",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntCarry (Func): binary|commutative|booloutput; addl arithmetic_op (typeop.cc:1334-1340).
        CPUI_INT_CARRY => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative | pf::booloutput,
            af::arithmetic_op,
            "CARRY",
            Func,
            TYPE_BOOL,
            TYPE_UINT,
        ),
        // TypeOpIntScarry (Func): binary|commutative|booloutput; addl arithmetic_op (typeop.cc:1350-1356).
        CPUI_INT_SCARRY => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative | pf::booloutput,
            af::arithmetic_op,
            "SCARRY",
            Func,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpIntSborrow (Func): binary|booloutput; addl arithmetic_op (typeop.cc:1366-1372).
        CPUI_INT_SBORROW => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::arithmetic_op,
            "SBORROW",
            Func,
            TYPE_BOOL,
            TYPE_INT,
        ),
        // TypeOpInt2Comp (Unary): unary; addl arithmetic_op|inherits_sign (typeop.cc:1382-1388).
        CPUI_INT_2COMP => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::arithmetic_op | af::inherits_sign,
            "-",
            Unary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntNegate (Unary): unary; addl logical_op|inherits_sign (typeop.cc:1396-1402).
        CPUI_INT_NEGATE => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::logical_op | af::inherits_sign,
            "~",
            Unary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntXor (Binary): binary|commutative; addl logical_op|inherits_sign (typeop.cc:1410-1416).
        CPUI_INT_XOR => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "^",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntAnd (Binary): binary|commutative; addl logical_op|inherits_sign (typeop.cc:1443-1449).
        CPUI_INT_AND => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "&",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntOr (Binary): binary|commutative; addl logical_op|inherits_sign (typeop.cc:1476-1482).
        CPUI_INT_OR => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "|",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntLeft (Binary): binary; addl inherits_sign|inherits_sign_zero|shift_op;
        // metaout/in INT (typeop.cc:1504-1510).
        CPUI_INT_LEFT => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            "<<",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntRight (Binary): binary; addl inherits_sign|inherits_sign_zero|shift_op;
        // metaout/in UINT (typeop.cc:1529-1535).
        CPUI_INT_RIGHT => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            ">>",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntSright (Binary): binary; addl inherits_sign|inherits_sign_zero|shift_op;
        // metaout/in INT (typeop.cc:1569-1575).
        CPUI_INT_SRIGHT => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            ">>",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntMult (Binary): binary|commutative; addl arithmetic_op|inherits_sign (typeop.cc:1619-1625).
        CPUI_INT_MULT => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::arithmetic_op | af::inherits_sign,
            "*",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntDiv (Binary): binary; addl arithmetic_op|inherits_sign; metaout/in UINT (typeop.cc:1633-1639).
        CPUI_INT_DIV => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "/",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntSdiv (Binary): binary; addl arithmetic_op|inherits_sign; metaout/in INT (typeop.cc:1653-1659).
        CPUI_INT_SDIV => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "/",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),
        // TypeOpIntRem (Binary): binary; addl arithmetic_op|inherits_sign|inherits_sign_zero;
        // metaout/in UINT (typeop.cc:1673-1679).
        CPUI_INT_REM => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::arithmetic_op | af::inherits_sign | af::inherits_sign_zero,
            "%",
            Binary,
            TYPE_UINT,
            TYPE_UINT,
        ),
        // TypeOpIntSrem (Binary): binary; addl arithmetic_op|inherits_sign|inherits_sign_zero;
        // metaout/in INT (typeop.cc:1693-1699).
        CPUI_INT_SREM => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::arithmetic_op | af::inherits_sign | af::inherits_sign_zero,
            "%",
            Binary,
            TYPE_INT,
            TYPE_INT,
        ),

        // -- Boolean ops (TypeOpUnary/Binary, BOOL) -------------------------
        // TypeOpBoolNegate (Unary): unary|booloutput; addl logical_op (typeop.cc:1713-1719).
        CPUI_BOOL_NEGATE => TypeOpInfo::typed(
            opc,
            pf::unary | pf::booloutput,
            af::logical_op,
            "!",
            Unary,
            TYPE_BOOL,
            TYPE_BOOL,
        ),
        // TypeOpBoolXor (Binary): binary|commutative|booloutput; addl logical_op (typeop.cc:1721-1727).
        CPUI_BOOL_XOR => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "^^",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL,
        ),
        // TypeOpBoolAnd (Binary): binary|commutative|booloutput; addl logical_op (typeop.cc:1729-1735).
        CPUI_BOOL_AND => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "&&",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL,
        ),
        // TypeOpBoolOr (Binary): binary|commutative|booloutput; addl logical_op (typeop.cc:1737-1743).
        CPUI_BOOL_OR => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "||",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL,
        ),

        // -- Floating-point comparisons (TypeOpBinary, BOOL out) ------------
        // TypeOpFloatEqual: binary|booloutput|commutative; addl floatingpoint_op (typeop.cc:1745-1751).
        CPUI_FLOAT_EQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput | pf::commutative,
            af::floatingpoint_op,
            "==",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT,
        ),
        // TypeOpFloatNotEqual: binary|booloutput|commutative; addl floatingpoint_op (typeop.cc:1753-1759).
        CPUI_FLOAT_NOTEQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput | pf::commutative,
            af::floatingpoint_op,
            "!=",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT,
        ),
        // TypeOpFloatLess: binary|booloutput; addl floatingpoint_op (typeop.cc:1761-1767).
        CPUI_FLOAT_LESS => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::floatingpoint_op,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT,
        ),
        // TypeOpFloatLessEqual: binary|booloutput; addl floatingpoint_op (typeop.cc:1769-1775).
        CPUI_FLOAT_LESSEQUAL => TypeOpInfo::typed(
            opc,
            pf::binary | pf::booloutput,
            af::floatingpoint_op,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT,
        ),
        // TypeOpFloatNan (Func): unary|booloutput; addl floatingpoint_op (typeop.cc:1777-1783).
        CPUI_FLOAT_NAN => TypeOpInfo::typed(
            opc,
            pf::unary | pf::booloutput,
            af::floatingpoint_op,
            "NAN",
            Func,
            TYPE_BOOL,
            TYPE_FLOAT,
        ),

        // -- Floating-point arithmetic (TypeOpBinary/Unary/Func) ------------
        // TypeOpFloatAdd: binary|commutative; addl floatingpoint_op (typeop.cc:1785-1791).
        CPUI_FLOAT_ADD => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::floatingpoint_op,
            "+",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatDiv: binary; addl floatingpoint_op (typeop.cc:1793-1799).
        CPUI_FLOAT_DIV => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::floatingpoint_op,
            "/",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatMult: binary|commutative; addl floatingpoint_op (typeop.cc:1801-1807).
        CPUI_FLOAT_MULT => TypeOpInfo::typed(
            opc,
            pf::binary | pf::commutative,
            af::floatingpoint_op,
            "*",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatSub: binary; addl floatingpoint_op (typeop.cc:1809-1815).
        CPUI_FLOAT_SUB => TypeOpInfo::typed(
            opc,
            pf::binary,
            af::floatingpoint_op,
            "-",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatNeg (Unary): unary; addl floatingpoint_op (typeop.cc:1817-1823).
        CPUI_FLOAT_NEG => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "-",
            Unary,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatAbs (Func): unary; addl floatingpoint_op (typeop.cc:1825-1831).
        CPUI_FLOAT_ABS => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "ABS",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatSqrt (Func): unary; addl floatingpoint_op (typeop.cc:1833-1839).
        CPUI_FLOAT_SQRT => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "SQRT",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatInt2Float (Func): unary; addl floatingpoint_op; metain INT (typeop.cc:1841-1847).
        CPUI_FLOAT_INT2FLOAT => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "INT2FLOAT",
            Func,
            TYPE_FLOAT,
            TYPE_INT,
        ),
        // TypeOpFloatFloat2Float (Func): unary; addl floatingpoint_op (typeop.cc:1906-1912).
        CPUI_FLOAT_FLOAT2FLOAT => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "FLOAT2FLOAT",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatTrunc (Func): unary; addl floatingpoint_op; metaout INT (typeop.cc:1914-1920).
        CPUI_FLOAT_TRUNC => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "TRUNC",
            Func,
            TYPE_INT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatCeil (Func): unary; addl floatingpoint_op (typeop.cc:1922-1928).
        CPUI_FLOAT_CEIL => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "CEIL",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatFloor (Func): unary; addl floatingpoint_op (typeop.cc:1930-1936).
        CPUI_FLOAT_FLOOR => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "FLOOR",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),
        // TypeOpFloatRound (Func): unary; addl floatingpoint_op (typeop.cc:1938-1944).
        CPUI_FLOAT_ROUND => TypeOpInfo::typed(
            opc,
            pf::unary,
            af::floatingpoint_op,
            "ROUND",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT,
        ),

        // -- Markers / structural (derive from TypeOp) ----------------------
        // TypeOpMulti: special|marker|nocollapse (typeop.cc:1946-1951).
        CPUI_MULTIEQUAL => {
            TypeOpInfo::base(opc, pf::special | pf::marker | pf::nocollapse, 0, "?")
        }
        // TypeOpIndirect: special|marker|nocollapse (typeop.cc:1987-1992).
        CPUI_INDIRECT => {
            TypeOpInfo::base(opc, pf::special | pf::marker | pf::nocollapse, 0, "[]")
        }
        // TypeOpPiece (Func): binary; metaout/in UNKNOWN (typeop.cc:2039-2048).
        CPUI_PIECE => {
            TypeOpInfo::typed(opc, pf::binary, 0, "CONCAT", Func, TYPE_UNKNOWN, TYPE_UNKNOWN)
        }
        // TypeOpSubpiece (Func): binary; metaout/in UNKNOWN (typeop.cc:2118-2127).
        CPUI_SUBPIECE => {
            TypeOpInfo::typed(opc, pf::binary, 0, "SUB", Func, TYPE_UNKNOWN, TYPE_UNKNOWN)
        }
        // TypeOpCast: unary|special|nocollapse (typeop.cc:2211-2216).
        CPUI_CAST => {
            TypeOpInfo::base(opc, pf::unary | pf::special | pf::nocollapse, 0, "(cast)")
        }
        // TypeOpPtradd: ternary|nocollapse; addl arithmetic_op (typeop.cc:2226-2232).
        CPUI_PTRADD => {
            TypeOpInfo::base(opc, pf::ternary | pf::nocollapse, af::arithmetic_op, "+")
        }
        // TypeOpPtrsub: binary|nocollapse; addl arithmetic_op (typeop.cc:2298-2308).
        CPUI_PTRSUB => {
            TypeOpInfo::base(opc, pf::binary | pf::nocollapse, af::arithmetic_op, "->")
        }
        // TypeOpSegment: special|nocollapse (typeop.cc:2392-2397).
        CPUI_SEGMENTOP => {
            TypeOpInfo::base(opc, pf::special | pf::nocollapse, 0, "segmentop")
        }
        // TypeOpCpoolref: special|nocollapse (typeop.cc:2445-2451).
        CPUI_CPOOLREF => TypeOpInfo::base(opc, pf::special | pf::nocollapse, 0, "cpoolref"),
        // TypeOpNew: special|call|nocollapse (typeop.cc:2496-2501).
        CPUI_NEW => TypeOpInfo::base(opc, pf::special | pf::call | pf::nocollapse, 0, "new"),
        // TypeOpInsert (Func): ternary; metaout UNKNOWN, metain INT (typeop.cc:2530-2535).
        CPUI_INSERT => {
            TypeOpInfo::typed(opc, pf::ternary, 0, "INSERT", Func, TYPE_UNKNOWN, TYPE_INT)
        }
        // TypeOpZpull (Func): ternary; metaout UINT, metain INT (typeop.cc:2557-2562).
        CPUI_ZPULL => TypeOpInfo::typed(opc, pf::ternary, 0, "ZPULL", Func, TYPE_UINT, TYPE_INT),
        // TypeOpPopcount (Func): unary; metaout INT, metain UNKNOWN (typeop.cc:2611-2616).
        CPUI_POPCOUNT => {
            TypeOpInfo::typed(opc, pf::unary, 0, "POPCOUNT", Func, TYPE_INT, TYPE_UNKNOWN)
        }
        // TypeOpLzcount (Func): unary; metaout INT, metain UNKNOWN (typeop.cc:2618-2623).
        CPUI_LZCOUNT => {
            TypeOpInfo::typed(opc, pf::unary, 0, "LZCOUNT", Func, TYPE_INT, TYPE_UNKNOWN)
        }
        // TypeOpSpull (Func): ternary; metaout/in INT (typeop.cc:2584-2589).
        CPUI_SPULL => TypeOpInfo::typed(opc, pf::ternary, 0, "SPULL", Func, TYPE_INT, TYPE_INT),

        // CPUI_MAX and the unused discriminant have no `inst[]` entry in C++
        // (`inst[CPUI_MAX]` is the trailing null `registerInstructions` inserts);
        // querying them is a porting bug, not a runtime path.
        CPUI_MAX => panic!("type_op_info: CPUI_MAX has no TypeOp entry (matches null inst[CPUI_MAX])"),
    }
}

/// Build the `inst[]` vector of [`TypeOpInfo`] records indexed by op-code value
/// (C++ `TypeOp::registerInstructions`).
///
/// The vector has length `CPUI_MAX` (`75`); `inst[0]` (no `CPUI_*` == 0) and the
/// unused discriminant `45` (between `FLOAT_LESSEQUAL`=44 and `FLOAT_NAN`=46) are
/// `None`, exactly as the C++ `inst.insert(inst.end(), CPUI_MAX, (TypeOp *)0)`
/// leaves those slots null.  Every other slot is the [`type_op_info`] record for
/// the op-code that indexes it.
pub fn register_instructions() -> Vec<Option<TypeOpInfo>> {
    let mut inst: Vec<Option<TypeOpInfo>> = vec![None; OpCode::CPUI_MAX as usize];
    // Iterate the op-codes registered by TypeOp::registerInstructions (every
    // CPUI_* except CPUI_MAX and the unused discriminant 45).
    for opc in REGISTERED_OPCODES {
        inst[*opc as usize] = Some(type_op_info(*opc));
    }
    inst
}

/// Build the `seams::TypeOp` triple for an op-code (canonical replacement for the
/// per-wave `type_op_seam`/`w6_type_op`/`typeop_for` local helpers).
pub fn type_op_for(opc: OpCode) -> TypeOp {
    type_op_info(opc).to_seam()
}

/// Every op-code that `TypeOp::registerInstructions` assigns an `inst[]` entry,
/// in the registration order of `typeop.cc:29-108`.  Excludes `CPUI_MAX` and the
/// unused discriminant `45` (which stay `None` in the table).
pub const REGISTERED_OPCODES: &[OpCode] = &[
    OpCode::CPUI_COPY,
    OpCode::CPUI_LOAD,
    OpCode::CPUI_STORE,
    OpCode::CPUI_BRANCH,
    OpCode::CPUI_CBRANCH,
    OpCode::CPUI_BRANCHIND,
    OpCode::CPUI_CALL,
    OpCode::CPUI_CALLIND,
    OpCode::CPUI_CALLOTHER,
    OpCode::CPUI_RETURN,
    OpCode::CPUI_MULTIEQUAL,
    OpCode::CPUI_INDIRECT,
    OpCode::CPUI_PIECE,
    OpCode::CPUI_SUBPIECE,
    OpCode::CPUI_INT_EQUAL,
    OpCode::CPUI_INT_NOTEQUAL,
    OpCode::CPUI_INT_SLESS,
    OpCode::CPUI_INT_SLESSEQUAL,
    OpCode::CPUI_INT_LESS,
    OpCode::CPUI_INT_LESSEQUAL,
    OpCode::CPUI_INT_ZEXT,
    OpCode::CPUI_INT_SEXT,
    OpCode::CPUI_INT_ADD,
    OpCode::CPUI_INT_SUB,
    OpCode::CPUI_INT_CARRY,
    OpCode::CPUI_INT_SCARRY,
    OpCode::CPUI_INT_SBORROW,
    OpCode::CPUI_INT_2COMP,
    OpCode::CPUI_INT_NEGATE,
    OpCode::CPUI_INT_XOR,
    OpCode::CPUI_INT_AND,
    OpCode::CPUI_INT_OR,
    OpCode::CPUI_INT_LEFT,
    OpCode::CPUI_INT_RIGHT,
    OpCode::CPUI_INT_SRIGHT,
    OpCode::CPUI_INT_MULT,
    OpCode::CPUI_INT_DIV,
    OpCode::CPUI_INT_SDIV,
    OpCode::CPUI_INT_REM,
    OpCode::CPUI_INT_SREM,
    OpCode::CPUI_BOOL_NEGATE,
    OpCode::CPUI_BOOL_XOR,
    OpCode::CPUI_BOOL_AND,
    OpCode::CPUI_BOOL_OR,
    OpCode::CPUI_CAST,
    OpCode::CPUI_PTRADD,
    OpCode::CPUI_PTRSUB,
    OpCode::CPUI_FLOAT_EQUAL,
    OpCode::CPUI_FLOAT_NOTEQUAL,
    OpCode::CPUI_FLOAT_LESS,
    OpCode::CPUI_FLOAT_LESSEQUAL,
    OpCode::CPUI_FLOAT_NAN,
    OpCode::CPUI_FLOAT_ADD,
    OpCode::CPUI_FLOAT_DIV,
    OpCode::CPUI_FLOAT_MULT,
    OpCode::CPUI_FLOAT_SUB,
    OpCode::CPUI_FLOAT_NEG,
    OpCode::CPUI_FLOAT_ABS,
    OpCode::CPUI_FLOAT_SQRT,
    OpCode::CPUI_FLOAT_INT2FLOAT,
    OpCode::CPUI_FLOAT_FLOAT2FLOAT,
    OpCode::CPUI_FLOAT_TRUNC,
    OpCode::CPUI_FLOAT_CEIL,
    OpCode::CPUI_FLOAT_FLOOR,
    OpCode::CPUI_FLOAT_ROUND,
    OpCode::CPUI_SEGMENTOP,
    OpCode::CPUI_CPOOLREF,
    OpCode::CPUI_NEW,
    OpCode::CPUI_INSERT,
    OpCode::CPUI_ZPULL,
    OpCode::CPUI_POPCOUNT,
    OpCode::CPUI_LZCOUNT,
    OpCode::CPUI_SPULL,
];

/// Toggle Java- vs C-specific aspects of the op-code information
/// (C++ `TypeOp::selectJavaOperators`, typeop.cc:115-148).
///
/// For Java, the unsigned ops are re-typed as signed (`TYPE_INT`) and `INT_RIGHT`
/// prints as `>>>`; for C they are `TYPE_UINT` and `>>`.  This mutates the table
/// records in place, exactly as the C++ calls `setMetatypeIn`/`setMetatypeOut`/
/// `setSymbol` on the live `inst[]` objects.  Only the op-codes the C++ touches
/// (`INT_ZEXT`, `INT_NEGATE`, `INT_XOR`, `INT_OR`, `INT_AND`, `INT_RIGHT`) are
/// modified; the rest are left as registered.
pub fn select_java_operators(inst: &mut [Option<TypeOpInfo>], java: bool) {
    fn set_meta_in(inst: &mut [Option<TypeOpInfo>], opc: OpCode, m: type_metatype) {
        if let Some(rec) = inst.get_mut(opc as usize).and_then(|s| s.as_mut()) {
            rec.metain = m;
        }
    }
    fn set_meta_out(inst: &mut [Option<TypeOpInfo>], opc: OpCode, m: type_metatype) {
        if let Some(rec) = inst.get_mut(opc as usize).and_then(|s| s.as_mut()) {
            rec.metaout = m;
        }
    }
    if java {
        set_meta_in(inst, OpCode::CPUI_INT_ZEXT, TYPE_UNKNOWN);
        set_meta_out(inst, OpCode::CPUI_INT_ZEXT, TYPE_INT);
        set_meta_in(inst, OpCode::CPUI_INT_NEGATE, TYPE_INT);
        set_meta_out(inst, OpCode::CPUI_INT_NEGATE, TYPE_INT);
        set_meta_in(inst, OpCode::CPUI_INT_XOR, TYPE_INT);
        set_meta_out(inst, OpCode::CPUI_INT_XOR, TYPE_INT);
        set_meta_in(inst, OpCode::CPUI_INT_OR, TYPE_INT);
        set_meta_out(inst, OpCode::CPUI_INT_OR, TYPE_INT);
        set_meta_in(inst, OpCode::CPUI_INT_AND, TYPE_INT);
        set_meta_out(inst, OpCode::CPUI_INT_AND, TYPE_INT);
        set_meta_in(inst, OpCode::CPUI_INT_RIGHT, TYPE_INT);
        set_meta_out(inst, OpCode::CPUI_INT_RIGHT, TYPE_INT);
        set_symbol(inst, OpCode::CPUI_INT_RIGHT, ">>>");
    } else {
        set_meta_in(inst, OpCode::CPUI_INT_ZEXT, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_ZEXT, TYPE_UINT);
        set_meta_in(inst, OpCode::CPUI_INT_NEGATE, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_NEGATE, TYPE_UINT);
        set_meta_in(inst, OpCode::CPUI_INT_XOR, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_XOR, TYPE_UINT);
        set_meta_in(inst, OpCode::CPUI_INT_OR, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_OR, TYPE_UINT);
        set_meta_in(inst, OpCode::CPUI_INT_AND, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_AND, TYPE_UINT);
        set_meta_in(inst, OpCode::CPUI_INT_RIGHT, TYPE_UINT);
        set_meta_out(inst, OpCode::CPUI_INT_RIGHT, TYPE_UINT);
        set_symbol(inst, OpCode::CPUI_INT_RIGHT, ">>");
    }
}

/// Set the display symbol of an op-code's record (C++ `TypeOp::setSymbol`).
fn set_symbol(inst: &mut [Option<TypeOpInfo>], opc: OpCode, sym: &'static str) {
    if let Some(rec) = inst.get_mut(opc as usize).and_then(|s| s.as_mut()) {
        rec.name = sym;
    }
}

/// Return `CPUI_FLOAT_NEG` if the \e sign bit is flipped, `CPUI_FLOAT_ABS` if the
/// \e sign bit is zeroed out, otherwise `CPUI_MAX`
/// (C++ `TypeOp::floatSignManipulation`, typeop.cc:154-177).
///
/// `opc` is the op-code, and `in1` is input slot 1 if it is constant
/// (`(size, offset)`), or `None` if slot 1 is not a constant.  The size/offset of
/// the constant are passed explicitly (the W3 IR holds them on the Varnode); this
/// keeps the helper free of the not-yet-ported Varnode accessor surface.
pub fn float_sign_manipulation(opc: OpCode, in1: Option<(int4, uintb)>) -> OpCode {
    match opc {
        OpCode::CPUI_INT_AND => {
            if let Some((size, offset)) = in1 {
                let mut val = calc_mask(size);
                val >>= 1;
                if val == offset {
                    return OpCode::CPUI_FLOAT_ABS;
                }
            }
        }
        OpCode::CPUI_INT_XOR => {
            if let Some((size, offset)) = in1 {
                let val = calc_mask(size);
                let val = val ^ (val >> 1);
                if val == offset {
                    return OpCode::CPUI_FLOAT_NEG;
                }
            }
        }
        _ => {}
    }
    OpCode::CPUI_MAX
}

// SEAM(W8): TypeOp::propagateToPointer / propagateFromPointer (typeop.cc:187-229)
// are the pointer-up/pointer-down helpers used only by the per-op-code
// propagateType bodies (LOAD/STORE).  They depend on `TypePointerRel::getParent`,
// `TypeFactory::getTypePartialEnum`, and `TypePartialStruct::getComponentForPtr`,
// none of which are exposed at the W6-partial boundary.  Since their only callers
// are the (unported, W8) propagateType paths, they are deferred to the W8 wave
// that ports those bodies rather than half-implemented against missing accessors.

#[cfg(test)]
mod tests;
