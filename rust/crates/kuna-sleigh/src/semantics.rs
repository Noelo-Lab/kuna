//! Port of `decompiler/cpp/semantics.{hh,cc}` (item `w2-sleigh-semantics`):
//! the p-code construct templates decoded from a compiled `.sla` file.
//!
//! ## What is ported
//!
//! - [`ConstTpl`]: a constant template with all `const_type` variants
//!   (real constants, handle references with `v_field` selection including
//!   `v_offset_plus` truncation, space ids, relative jump offsets, the
//!   `j_*` placeholders).  `fix`/`fixSpace` resolve against the
//!   `ParserWalker` seam ([`SymbolWalker`], defined by the symbol wave).
//! - [`VarnodeTpl`] / [`OpTpl`] / [`HandleTpl`] / [`ConstructTpl`] with
//!   their encode/decode paths (`.sla` FORMAT_SCOPE ids, see [`sla`]).
//! - [`PcodeBuilder`]: the SLEIGH-specific p-code generator dispatch
//!   (`build`), as a trait whose required methods are the C++ virtuals.
//!
//! ## Representation notes
//!
//! - The C++ `value` union (`AddrSpace *spaceid` / `int4 handle_index`) is
//!   two struct fields; only the field matching `type` is meaningful, as in
//!   C++ (the inactive arm is dead data).
//! - C++ encodes an `AddrSpace *` **pointer** as a `uintb`
//!   (`(uintb)(uintp)spc`, `ConstTpl::fix`).  The port follows the
//!   `kuna-num` `pcoderaw.rs` convention and encodes the space's *manager
//!   index* instead (`VarnodeData::get_space_from_const` recovers it
//!   through the `AddrSpaceManager`); a C++ null pointer encodes as 0.
//!   This also makes `ConstTpl::operator<` deterministic where C++
//!   compared heap pointers.
//! - C++ nullable owning pointers (`OpTpl::output`, `ConstructTpl::result`)
//!   are `Option<...>` by value; `vector<VarnodeTpl *>` /
//!   `vector<OpTpl *>` are `Vec<...>` by value.
//! - `ConstTpl::fixSpace` returns `KunaResult<Option<Rc<AddrSpace>>>`: the
//!   `Option` mirrors the C++ nullable `AddrSpace *` result and the `Err`
//!   mirrors the C++ `LowlevelError` throw.  Call sites that C++
//!   dereferences unconditionally `expect()` (UB in C++ = internal
//!   invariant violation, ADR 0004).
//! - The C++ `PcodeBuilder` private fields `labelbase`/`labelcount` become
//!   required accessors on the trait (Rust traits cannot hold state); the
//!   protected `ParserWalker *walker` member and `getCurrentWalker()` are
//!   left to the implementor (the sleigh decode-engine wave), which holds
//!   its own walker.
//! - The compiler-only protected mutators `ConstructTpl::setOpvec` /
//!   `setNumLabels` (friend `SleighCompile`) are not ported (LOSS-001
//!   scope); `get_opvec_mut` is a port-added accessor standing in for the
//!   C++ pattern of mutating template ops through `vector<OpTpl *>`
//!   pointees (used by `pcodecompile.rs` size propagation).

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::Wrap;
use kuna_num::opcodes::{OpCode, OpcodeDecoder, OpcodeEncoder};

use crate::context::FixedHandle;
use crate::slghsymbol::SymbolWalker;

/// `.sla`-format ElementIds/AttributeIds used by the template system
/// (slaformat.cc, `FORMAT_SCOPE`).  Extends the set already defined by the
/// pattern/symbol waves, which is re-exported here so template code uses a
/// single `sla::` namespace.
pub mod sla {
    use kuna_base::marshal::{AttributeId, ElementId};

    pub use crate::slghsymbol::sla::*;

    pub const ATTRIB_S: AttributeId = AttributeId::new("s", 5);
    pub const ATTRIB_PLUS: AttributeId = AttributeId::new("plus", 28);
    pub const ATTRIB_DELAY: AttributeId = AttributeId::new("delay", 42);
    pub const ATTRIB_SECTION: AttributeId = AttributeId::new("section", 54);
    pub const ATTRIB_LABELS: AttributeId = AttributeId::new("labels", 55);

    pub const ELEM_CONST_REAL: ElementId = ElementId::new("const_real", 1);
    pub const ELEM_VARNODE_TPL: ElementId = ElementId::new("varnode_tpl", 2);
    pub const ELEM_CONST_SPACEID: ElementId = ElementId::new("const_spaceid", 3);
    pub const ELEM_CONST_HANDLE: ElementId = ElementId::new("const_handle", 4);
    pub const ELEM_OP_TPL: ElementId = ElementId::new("op_tpl", 5);
    pub const ELEM_CONSTRUCT_TPL: ElementId = ElementId::new("construct_tpl", 21);
    pub const ELEM_HANDLE_TPL: ElementId = ElementId::new("handle_tpl", 30);
    pub const ELEM_CONST_RELATIVE: ElementId = ElementId::new("const_relative", 31);
    pub const ELEM_CONST_START: ElementId = ElementId::new("const_start", 80);
    pub const ELEM_CONST_NEXT: ElementId = ElementId::new("const_next", 81);
    pub const ELEM_CONST_NEXT2: ElementId = ElementId::new("const_next2", 82);
    pub const ELEM_CONST_CURSPACE: ElementId = ElementId::new("const_curspace", 83);
    pub const ELEM_CONST_CURSPACE_SIZE: ElementId = ElementId::new("const_curspace_size", 84);
    pub const ELEM_CONST_FLOWREF: ElementId = ElementId::new("const_flowref", 85);
    pub const ELEM_CONST_FLOWREF_SIZE: ElementId = ElementId::new("const_flowref_size", 86);
    pub const ELEM_CONST_FLOWDEST: ElementId = ElementId::new("const_flowdest", 87);
    pub const ELEM_CONST_FLOWDEST_SIZE: ElementId = ElementId::new("const_flowdest_size", 88);
}

// We remap these opcodes for internal use during pcode generation
// (semantics.hh `#define`s).

/// C++ `BUILD` (remapped `CPUI_MULTIEQUAL`)
pub const BUILD: OpCode = OpCode::CPUI_MULTIEQUAL;
/// C++ `DELAY_SLOT` (remapped `CPUI_INDIRECT`)
pub const DELAY_SLOT: OpCode = OpCode::CPUI_INDIRECT;
/// C++ `CROSSBUILD` (remapped `CPUI_PTRSUB`)
pub const CROSSBUILD: OpCode = OpCode::CPUI_PTRSUB;
/// C++ `MACROBUILD` (remapped `CPUI_CAST`)
pub const MACROBUILD: OpCode = OpCode::CPUI_CAST;
/// C++ `LABELBUILD` (remapped `CPUI_PTRADD`)
pub const LABELBUILD: OpCode = OpCode::CPUI_PTRADD;

/// C++ `ConstTpl::const_type` (discriminants match the C++ enum values;
/// the derived order is the C++ `type < op2.type` integer order).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstType {
    /// C++ `real`: an actual constant
    #[default]
    Real = 0,
    /// C++ `handle`: reference into the fixed-handle array
    Handle = 1,
    /// C++ `j_start`
    JStart = 2,
    /// C++ `j_next`
    JNext = 3,
    /// C++ `j_next2`
    JNext2 = 4,
    /// C++ `j_curspace`
    JCurspace = 5,
    /// C++ `j_curspace_size`
    JCurspaceSize = 6,
    /// C++ `spaceid`
    Spaceid = 7,
    /// C++ `j_relative`
    JRelative = 8,
    /// C++ `j_flowref`
    JFlowref = 9,
    /// C++ `j_flowref_size`
    JFlowrefSize = 10,
    /// C++ `j_flowdest`
    JFlowdest = 11,
    /// C++ `j_flowdest_size`
    JFlowdestSize = 12,
}

/// C++ `ConstTpl::v_field`: which part of a handle to use as the constant
/// (discriminants match the C++ enum values).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum VField {
    /// C++ `v_space`
    #[default]
    VSpace = 0,
    /// C++ `v_offset`
    VOffset = 1,
    /// C++ `v_size`
    VSize = 2,
    /// C++ `v_offset_plus`
    VOffsetPlus = 3,
}

/// C++ `(uintb)(uintp)spc`: encode a space "pointer" as a `u64`.  The port
/// stores the space's manager index (kuna-num `pcoderaw.rs` convention,
/// `VarnodeData::get_space_from_const`).
fn space_to_const(spc: &Rc<AddrSpace>) -> u64 {
    spc.get_index() as u64 // cast: manager indices are small and non-negative
}

/// [`space_to_const`] over a nullable pointer (C++ null casts to 0).
fn opt_space_to_const(spc: &Option<Rc<AddrSpace>>) -> u64 {
    spc.as_ref().map_or(0, space_to_const)
}

/// C++ raw-pointer equality over nullable space pointers (null == null).
fn opt_space_ptr_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// ConstTpl
// ---------------------------------------------------------------------------

/// C++ `ConstTpl`: a constant value template, instantiated at
/// instruction-decode time against a `ParserWalker`.
#[derive(Debug, Clone, Default)]
pub struct ConstTpl {
    /// C++ `type`
    type_: ConstType,
    /// C++ union arm `value.spaceid` (meaningful when `type_ == Spaceid`)
    value_spaceid: Option<Rc<AddrSpace>>,
    /// C++ union arm `value.handle_index` (meaningful when `type_ == Handle`)
    value_handle_index: i32,
    /// C++ `value_real`
    value_real: u64,
    /// C++ `select`: which part of the handle to use as the constant
    select: VField,
}

impl ConstTpl {
    /// C++ `ConstTpl(void)`: a real constant 0.
    pub fn new() -> ConstTpl {
        ConstTpl::default()
    }

    /// C++ `ConstTpl(const_type tp)`: constructor for relative jump
    /// constants and uniques.  (C++ leaves the other fields uninitialized;
    /// the port zeroes them.)
    pub fn new_type(tp: ConstType) -> ConstTpl {
        ConstTpl {
            type_: tp,
            ..ConstTpl::default()
        }
    }

    /// C++ `ConstTpl(const_type tp,uintb val)`: constructor for real
    /// constants.
    pub fn new_real(tp: ConstType, val: u64) -> ConstTpl {
        ConstTpl {
            type_: tp,
            value_spaceid: None,
            value_handle_index: 0,
            value_real: val,
            select: VField::VSpace,
        }
    }

    /// C++ `ConstTpl(const_type tp,int4 ht,v_field vf)`: constructor for a
    /// handle constant.  (The C++ signature takes a `const_type` parameter
    /// but unconditionally assigns `type = handle`; the parameter is
    /// dropped.)
    pub fn new_handle(ht: i32, vf: VField) -> ConstTpl {
        ConstTpl {
            type_: ConstType::Handle,
            value_spaceid: None,
            value_handle_index: ht,
            value_real: 0,
            select: vf,
        }
    }

    /// C++ `ConstTpl(const_type tp,int4 ht,v_field vf,uintb plus)` (same
    /// dropped-parameter note as [`ConstTpl::new_handle`]).
    pub fn new_handle_plus(ht: i32, vf: VField, plus: u64) -> ConstTpl {
        ConstTpl {
            type_: ConstType::Handle,
            value_spaceid: None,
            value_handle_index: ht,
            value_real: plus,
            select: vf,
        }
    }

    /// C++ `ConstTpl(AddrSpace *sid)`.
    pub fn new_space(sid: Rc<AddrSpace>) -> ConstTpl {
        ConstTpl {
            type_: ConstType::Spaceid,
            value_spaceid: Some(sid),
            value_handle_index: 0,
            value_real: 0,
            select: VField::VSpace,
        }
    }

    /// C++ `isConstSpace`.
    pub fn is_const_space(&self) -> bool {
        if self.type_ == ConstType::Spaceid {
            return self.get_space().get_type() == spacetype::IPTR_CONSTANT;
        }
        false
    }

    /// C++ `isUniqueSpace`.
    pub fn is_unique_space(&self) -> bool {
        if self.type_ == ConstType::Spaceid {
            return self.get_space().get_type() == spacetype::IPTR_INTERNAL;
        }
        false
    }

    /// C++ `getReal`.
    pub fn get_real(&self) -> u64 {
        self.value_real
    }

    /// C++ `getSpace` (the union spaceid arm; C++ reads it unchecked, so a
    /// non-spaceid `ConstTpl` is an internal invariant violation here).
    pub fn get_space(&self) -> Rc<AddrSpace> {
        Rc::clone(
            self.value_spaceid
                .as_ref()
                .expect("ConstTpl::get_space on a ConstTpl without a space"),
        )
    }

    /// C++ `getHandleIndex`.
    pub fn get_handle_index(&self) -> i32 {
        self.value_handle_index
    }

    /// C++ `getType`.
    pub fn get_type(&self) -> ConstType {
        self.type_
    }

    /// C++ `getSelect`.
    pub fn get_select(&self) -> VField {
        self.select
    }

    /// C++ `isZero`.
    pub fn is_zero(&self) -> bool {
        self.type_ == ConstType::Real && self.value_real == 0
    }

    /// C++ `operator<`.  Where C++ ordered the `spaceid` case by raw heap
    /// pointer (nondeterministic), the port orders by manager index, with a
    /// null pointer ordering before every space.
    pub fn less_than(&self, op2: &ConstTpl) -> bool {
        if self.type_ != op2.type_ {
            return self.type_ < op2.type_;
        }
        match self.type_ {
            ConstType::Real => self.value_real < op2.value_real,
            ConstType::Handle => {
                if self.value_handle_index != op2.value_handle_index {
                    return self.value_handle_index < op2.value_handle_index;
                }
                if self.select != op2.select {
                    return self.select < op2.select;
                }
                false
            }
            ConstType::Spaceid => {
                // C++ `value.spaceid < op2.value.spaceid` (pointer order);
                // deterministic substitute: manager-index order, None first.
                let i1 = self.value_spaceid.as_ref().map(|s| s.get_index());
                let i2 = op2.value_spaceid.as_ref().map(|s| s.get_index());
                i1 < i2
            }
            _ => false, // Nothing additional to compare
        }
    }

    /// C++ `fix(const ParserWalker &)`: get the value of the ConstTpl in
    /// context.  NOTE: if the property is dynamic this returns the property
    /// of the temporary storage.
    pub fn fix(&self, walker: &dyn SymbolWalker) -> KunaResult<u64> {
        match self.type_ {
            // Fill in starting address placeholder with real address
            ConstType::JStart => Ok(walker.get_addr().get_offset()),
            // Fill in next address placeholder with real address
            ConstType::JNext => Ok(walker.get_naddr().get_offset()),
            // Fill in next2 address placeholder with real address
            ConstType::JNext2 => Ok(walker.get_n2addr()?.get_offset()),
            ConstType::JFlowref => Ok(walker.get_ref_addr()?.get_offset()),
            ConstType::JFlowrefSize => {
                // cast: Address::getAddrSize is int4, non-negative
                Ok(walker.get_ref_addr()?.get_addr_size() as u64)
            }
            ConstType::JFlowdest => Ok(walker.get_dest_addr()?.get_offset()),
            ConstType::JFlowdestSize => {
                // cast: Address::getAddrSize is int4, non-negative
                Ok(walker.get_dest_addr()?.get_addr_size() as u64)
            }
            ConstType::JCurspaceSize => Ok(u64::from(walker.get_cur_space().get_addr_size())),
            ConstType::JCurspace => Ok(space_to_const(&walker.get_cur_space())),
            ConstType::Handle => {
                let hand = walker.get_fixed_handle(self.value_handle_index)?;
                match self.select {
                    VField::VSpace => {
                        if hand.offset_space.is_none() {
                            Ok(opt_space_to_const(&hand.space))
                        } else {
                            Ok(opt_space_to_const(&hand.temp_space))
                        }
                    }
                    VField::VOffset => {
                        if hand.offset_space.is_none() {
                            Ok(hand.offset_offset)
                        } else {
                            Ok(hand.temp_offset)
                        }
                    }
                    VField::VSize => Ok(u64::from(hand.size)),
                    VField::VOffsetPlus => {
                        let const_space = walker.get_const_space();
                        let is_const =
                            matches!(&hand.space, Some(s) if Rc::ptr_eq(s, &const_space));
                        if !is_const {
                            // If we are not a constant:
                            // adjust offset by truncation amount
                            if hand.offset_space.is_none() {
                                Ok(hand.offset_offset.wadd(self.value_real & 0xffff))
                            } else {
                                Ok(hand.temp_offset.wadd(self.value_real & 0xffff))
                            }
                        } else {
                            // If we are a constant, we want to return a
                            // shifted value
                            let val = if hand.offset_space.is_none() {
                                hand.offset_offset
                            } else {
                                hand.temp_offset
                            };
                            // cast: shift count `8*(value_real>>16)`; C++
                            // `>>=` is UB for counts >= 64, wshr wraps
                            Ok(val.wshr(8u64.wmul(self.value_real >> 16) as u32))
                        }
                    }
                }
            }
            ConstType::JRelative | ConstType::Real => Ok(self.value_real),
            ConstType::Spaceid => Ok(opt_space_to_const(&self.value_spaceid)),
        }
    }

    /// C++ `fixSpace`: get the value of the ConstTpl in context when we
    /// know it is a space.  `Option` mirrors the nullable C++ result
    /// pointer; `Err` mirrors the C++ `LowlevelError`.
    pub fn fix_space(&self, walker: &dyn SymbolWalker) -> KunaResult<Option<Rc<AddrSpace>>> {
        match self.type_ {
            ConstType::JCurspace => return Ok(Some(walker.get_cur_space())),
            ConstType::Handle => {
                let hand = walker.get_fixed_handle(self.value_handle_index)?;
                if self.select == VField::VSpace {
                    if hand.offset_space.is_none() {
                        return Ok(hand.space);
                    }
                    return Ok(hand.temp_space);
                }
                // other selects fall through to the throw
            }
            ConstType::Spaceid => return Ok(self.value_spaceid.clone()),
            ConstType::JFlowref => return Ok(walker.get_ref_addr()?.get_space().cloned()),
            _ => {}
        }
        Err(KunaError::lowlevel("ConstTpl is not a spaceid as expected"))
    }

    /// C++ `fillinSpace`: fill in the space portion of a FixedHandle, based
    /// on this ConstTpl.
    pub fn fillin_space(
        &self,
        hand: &mut FixedHandle,
        walker: &dyn SymbolWalker,
    ) -> KunaResult<()> {
        match self.type_ {
            ConstType::JCurspace => {
                hand.space = Some(walker.get_cur_space());
                return Ok(());
            }
            ConstType::Handle => {
                let otherhand = walker.get_fixed_handle(self.value_handle_index)?;
                if self.select == VField::VSpace {
                    hand.space = otherhand.space;
                    return Ok(());
                }
                // other selects fall through to the throw
            }
            ConstType::Spaceid => {
                hand.space = self.value_spaceid.clone();
                return Ok(());
            }
            _ => {}
        }
        Err(KunaError::lowlevel("ConstTpl is not a spaceid as expected"))
    }

    /// C++ `fillinOffset`: fill in the offset portion of a FixedHandle,
    /// based on this ConstTpl.  If the offset value is dynamic, indicate
    /// this in the handle; we don't just fill in the temporary variable
    /// offset.  We assume hand.space is already filled in.
    pub fn fillin_offset(
        &self,
        hand: &mut FixedHandle,
        walker: &dyn SymbolWalker,
    ) -> KunaResult<()> {
        if self.type_ == ConstType::Handle {
            let otherhand = walker.get_fixed_handle(self.value_handle_index)?;
            hand.offset_space = otherhand.offset_space;
            hand.offset_offset = otherhand.offset_offset;
            hand.offset_size = otherhand.offset_size;
            hand.temp_space = otherhand.temp_space;
            hand.temp_offset = otherhand.temp_offset;
        } else {
            hand.offset_space = None;
            let fixed = self.fix(walker)?;
            let space = hand.space.as_ref().expect(
                "FixedHandle.space must be filled in before fillinOffset (C++ dereferences unconditionally)",
            );
            hand.offset_offset = space.wrap_offset(fixed);
        }
        Ok(())
    }

    /// C++ `transfer`: replace old handles with new handles.
    pub fn transfer(&mut self, params: &[HandleTpl]) -> KunaResult<()> {
        if self.type_ != ConstType::Handle {
            return Ok(());
        }
        // cast: C++ indexes vector<HandleTpl *> with int4
        let newhandle = &params[self.value_handle_index as usize];

        match self.select {
            VField::VSpace => {
                *self = newhandle.get_space().clone();
            }
            VField::VOffset => {
                *self = newhandle.get_ptr_offset().clone();
            }
            VField::VOffsetPlus => {
                let tmp = self.value_real;
                *self = newhandle.get_ptr_offset().clone();
                if self.type_ == ConstType::Real {
                    self.value_real = self.value_real.wadd(tmp & 0xffff);
                } else if self.type_ == ConstType::Handle && self.select == VField::VOffset {
                    self.select = VField::VOffsetPlus;
                    self.value_real = tmp;
                } else {
                    return Err(KunaError::lowlevel(
                        "Cannot truncate macro input in this way",
                    ));
                }
            }
            VField::VSize => {
                *self = newhandle.get_size().clone();
            }
        }
        Ok(())
    }

    /// C++ `changeHandleIndex`.
    pub fn change_handle_index(&mut self, handmap: &[i32]) {
        if self.type_ == ConstType::Handle {
            // cast: C++ indexes vector<int4> with int4
            self.value_handle_index = handmap[self.value_handle_index as usize];
        }
    }

    /// C++ `encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        match self.type_ {
            ConstType::Real => {
                encoder.open_element(&sla::ELEM_CONST_REAL);
                encoder.write_unsigned_integer(&sla::ATTRIB_VAL, self.value_real);
                encoder.close_element(&sla::ELEM_CONST_REAL);
            }
            ConstType::Handle => {
                encoder.open_element(&sla::ELEM_CONST_HANDLE);
                encoder.write_signed_integer(&sla::ATTRIB_VAL, i64::from(self.value_handle_index));
                encoder.write_signed_integer(&sla::ATTRIB_S, self.select as i64);
                if self.select == VField::VOffsetPlus {
                    encoder.write_unsigned_integer(&sla::ATTRIB_PLUS, self.value_real);
                }
                encoder.close_element(&sla::ELEM_CONST_HANDLE);
            }
            ConstType::JStart => {
                encoder.open_element(&sla::ELEM_CONST_START);
                encoder.close_element(&sla::ELEM_CONST_START);
            }
            ConstType::JNext => {
                encoder.open_element(&sla::ELEM_CONST_NEXT);
                encoder.close_element(&sla::ELEM_CONST_NEXT);
            }
            ConstType::JNext2 => {
                encoder.open_element(&sla::ELEM_CONST_NEXT2);
                encoder.close_element(&sla::ELEM_CONST_NEXT2);
            }
            ConstType::JCurspace => {
                encoder.open_element(&sla::ELEM_CONST_CURSPACE);
                encoder.close_element(&sla::ELEM_CONST_CURSPACE);
            }
            ConstType::JCurspaceSize => {
                encoder.open_element(&sla::ELEM_CONST_CURSPACE_SIZE);
                encoder.close_element(&sla::ELEM_CONST_CURSPACE_SIZE);
            }
            ConstType::Spaceid => {
                encoder.open_element(&sla::ELEM_CONST_SPACEID);
                encoder.write_space(&sla::ATTRIB_SPACE, &self.get_space());
                encoder.close_element(&sla::ELEM_CONST_SPACEID);
            }
            ConstType::JRelative => {
                encoder.open_element(&sla::ELEM_CONST_RELATIVE);
                encoder.write_unsigned_integer(&sla::ATTRIB_VAL, self.value_real);
                encoder.close_element(&sla::ELEM_CONST_RELATIVE);
            }
            ConstType::JFlowref => {
                encoder.open_element(&sla::ELEM_CONST_FLOWREF);
                encoder.close_element(&sla::ELEM_CONST_FLOWREF);
            }
            ConstType::JFlowrefSize => {
                encoder.open_element(&sla::ELEM_CONST_FLOWREF_SIZE);
                encoder.close_element(&sla::ELEM_CONST_FLOWREF_SIZE);
            }
            ConstType::JFlowdest => {
                encoder.open_element(&sla::ELEM_CONST_FLOWDEST);
                encoder.close_element(&sla::ELEM_CONST_FLOWDEST);
            }
            ConstType::JFlowdestSize => {
                encoder.open_element(&sla::ELEM_CONST_FLOWDEST_SIZE);
                encoder.close_element(&sla::ELEM_CONST_FLOWDEST_SIZE);
            }
        }
    }

    /// C++ `decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.open_element()?;
        if el == sla::ELEM_CONST_REAL {
            self.type_ = ConstType::Real;
            self.value_real = decoder.read_unsigned_integer_id(&sla::ATTRIB_VAL)?;
        } else if el == sla::ELEM_CONST_HANDLE {
            self.type_ = ConstType::Handle;
            // cast: C++ implicit intb -> int4
            self.value_handle_index = decoder.read_signed_integer_id(&sla::ATTRIB_VAL)? as i32;
            // cast: C++ `uint4 selectInt = readSignedInteger(...)` (intb ->
            // uint4 wrap; a negative value becomes huge and is rejected)
            let select_int = decoder.read_signed_integer_id(&sla::ATTRIB_S)? as u32;
            self.select = match select_int {
                0 => VField::VSpace,
                1 => VField::VOffset,
                2 => VField::VSize,
                3 => VField::VOffsetPlus,
                _ => return Err(KunaError::decoder("Bad handle selector encoding")),
            };
            if self.select == VField::VOffsetPlus {
                self.value_real = decoder.read_unsigned_integer_id(&sla::ATTRIB_PLUS)?;
            }
        } else if el == sla::ELEM_CONST_START {
            self.type_ = ConstType::JStart;
        } else if el == sla::ELEM_CONST_NEXT {
            self.type_ = ConstType::JNext;
        } else if el == sla::ELEM_CONST_NEXT2 {
            self.type_ = ConstType::JNext2;
        } else if el == sla::ELEM_CONST_CURSPACE {
            self.type_ = ConstType::JCurspace;
        } else if el == sla::ELEM_CONST_CURSPACE_SIZE {
            self.type_ = ConstType::JCurspaceSize;
        } else if el == sla::ELEM_CONST_SPACEID {
            self.type_ = ConstType::Spaceid;
            self.value_spaceid = Some(decoder.read_space_id(&sla::ATTRIB_SPACE)?);
        } else if el == sla::ELEM_CONST_RELATIVE {
            self.type_ = ConstType::JRelative;
            self.value_real = decoder.read_unsigned_integer_id(&sla::ATTRIB_VAL)?;
        } else if el == sla::ELEM_CONST_FLOWREF {
            self.type_ = ConstType::JFlowref;
        } else if el == sla::ELEM_CONST_FLOWREF_SIZE {
            self.type_ = ConstType::JFlowrefSize;
        } else if el == sla::ELEM_CONST_FLOWDEST {
            self.type_ = ConstType::JFlowdest;
        } else if el == sla::ELEM_CONST_FLOWDEST_SIZE {
            self.type_ = ConstType::JFlowdestSize;
        } else {
            // C++ throws LowlevelError (not DecoderError) here
            return Err(KunaError::lowlevel("Bad constant type"));
        }
        decoder.close_element(el)
    }
}

impl PartialEq for ConstTpl {
    /// C++ `operator==` (the `spaceid` case is raw pointer equality).
    fn eq(&self, op2: &ConstTpl) -> bool {
        if self.type_ != op2.type_ {
            return false;
        }
        match self.type_ {
            ConstType::Real => self.value_real == op2.value_real,
            ConstType::Handle => {
                if self.value_handle_index != op2.value_handle_index {
                    return false;
                }
                if self.select != op2.select {
                    return false;
                }
                true
            }
            ConstType::Spaceid => opt_space_ptr_eq(&self.value_spaceid, &op2.value_spaceid),
            _ => true, // Nothing additional to compare
        }
    }
}
impl Eq for ConstTpl {}

// ---------------------------------------------------------------------------
// VarnodeTpl
// ---------------------------------------------------------------------------

/// C++ `VarnodeTpl`: a varnode template (space/offset/size ConstTpl
/// triple).
#[derive(Debug, Clone, Default)]
pub struct VarnodeTpl {
    space: ConstTpl,
    offset: ConstTpl,
    size: ConstTpl,
    unnamed_flag: bool,
}

impl VarnodeTpl {
    /// C++ `VarnodeTpl(int4 hand,bool zerosize)`: varnode built from a
    /// handle; if zerosize is true, set the size constant to zero.
    pub fn new_from_handle(hand: i32, zerosize: bool) -> VarnodeTpl {
        let mut res = VarnodeTpl {
            space: ConstTpl::new_handle(hand, VField::VSpace),
            offset: ConstTpl::new_handle(hand, VField::VOffset),
            size: ConstTpl::new_handle(hand, VField::VSize),
            unnamed_flag: false,
        };
        if zerosize {
            res.size = ConstTpl::new_real(ConstType::Real, 0);
        }
        res
    }

    /// C++ `VarnodeTpl(const ConstTpl &sp,const ConstTpl &off,const
    /// ConstTpl &sz)`.
    pub fn new(sp: ConstTpl, off: ConstTpl, sz: ConstTpl) -> VarnodeTpl {
        VarnodeTpl {
            space: sp,
            offset: off,
            size: sz,
            unnamed_flag: false,
        }
    }

    /// C++ `getSpace`.
    pub fn get_space(&self) -> &ConstTpl {
        &self.space
    }

    /// C++ `getOffset`.
    pub fn get_offset(&self) -> &ConstTpl {
        &self.offset
    }

    /// C++ `getSize`.
    pub fn get_size(&self) -> &ConstTpl {
        &self.size
    }

    /// C++ `isDynamic`.
    pub fn is_dynamic(&self, walker: &dyn SymbolWalker) -> KunaResult<bool> {
        if self.offset.get_type() != ConstType::Handle {
            return Ok(false);
        }
        // Technically we should probably check all three
        // ConstTpls for dynamic handles, but in all cases
        // if there is any dynamic piece then the offset is
        let hand = walker.get_fixed_handle(self.offset.get_handle_index())?;
        Ok(hand.offset_space.is_some())
    }

    /// C++ `transfer`: a positive return indicates truncation of a local
    /// temp or a zero-size object; -1 otherwise.
    pub fn transfer(&mut self, params: &[HandleTpl]) -> KunaResult<i32> {
        let mut does_offset_plus = false;
        let mut handle_index: i32 = 0;
        let mut plus: i32 = 0;
        if self.offset.get_type() == ConstType::Handle
            && self.offset.get_select() == VField::VOffsetPlus
        {
            handle_index = self.offset.get_handle_index();
            plus = self.offset.get_real() as i32; // cast: C++ explicit (int4)
            does_offset_plus = true;
        }
        self.space.transfer(params)?;
        self.offset.transfer(params)?;
        self.size.transfer(params)?;
        if does_offset_plus {
            if self.is_local_temp() {
                return Ok(plus); // A positive number indicates truncation of a local temp
            }
            // cast: C++ indexes vector<HandleTpl *> with int4
            if params[handle_index as usize].get_size().is_zero() {
                return Ok(plus); //    or a zerosize object
            }
        }
        Ok(-1)
    }

    /// C++ `isZeroSize`.
    pub fn is_zero_size(&self) -> bool {
        self.size.is_zero()
    }

    /// C++ `operator<`.
    pub fn less_than(&self, op2: &VarnodeTpl) -> bool {
        if self.space != op2.space {
            return self.space.less_than(&op2.space);
        }
        if self.offset != op2.offset {
            return self.offset.less_than(&op2.offset);
        }
        if self.size != op2.size {
            return self.size.less_than(&op2.size);
        }
        false
    }

    /// C++ `setOffset(uintb constVal)`.
    pub fn set_offset(&mut self, const_val: u64) {
        self.offset = ConstTpl::new_real(ConstType::Real, const_val);
    }

    /// C++ `setRelative(uintb constVal)`.
    pub fn set_relative(&mut self, const_val: u64) {
        self.offset = ConstTpl::new_real(ConstType::JRelative, const_val);
    }

    /// C++ `setSize`.
    pub fn set_size(&mut self, sz: ConstTpl) {
        self.size = sz;
    }

    /// C++ `isUnnamed`.
    pub fn is_unnamed(&self) -> bool {
        self.unnamed_flag
    }

    /// C++ `setUnnamed`.
    pub fn set_unnamed(&mut self, val: bool) {
        self.unnamed_flag = val;
    }

    /// C++ `isLocalTemp`.
    pub fn is_local_temp(&self) -> bool {
        if self.space.get_type() != ConstType::Spaceid {
            return false;
        }
        if self.space.get_space().get_type() != spacetype::IPTR_INTERNAL {
            return false;
        }
        true
    }

    /// C++ `isRelative`.
    pub fn is_relative(&self) -> bool {
        self.offset.get_type() == ConstType::JRelative
    }

    /// C++ `changeHandleIndex`.
    pub fn change_handle_index(&mut self, handmap: &[i32]) {
        self.space.change_handle_index(handmap);
        self.offset.change_handle_index(handmap);
        self.size.change_handle_index(handmap);
    }

    /// C++ `adjustTruncation`: we know `this->offset` is an offset_plus,
    /// check that the truncation is in bounds (given `sz`), adjust plus for
    /// endianness if necessary; return true if truncation is in bounds.
    pub fn adjust_truncation(&mut self, sz: i32, isbigendian: bool) -> bool {
        if self.size.get_type() != ConstType::Real {
            return false;
        }
        let numbytes = self.size.get_real() as i32; // cast: C++ explicit (int4)
        let byteoffset = self.offset.get_real() as i32; // cast: C++ explicit (int4)
        if numbytes + byteoffset > sz {
            return false;
        }

        // Encode the original truncation amount with the plus value
        // (cast: C++ implicit int4 -> uintb sign extension)
        let mut val = i64::from(byteoffset) as u64;
        val = val.wshl(16);
        if isbigendian {
            // cast: C++ explicit (uintb) of a non-negative int4 expression
            val |= i64::from(sz - (numbytes + byteoffset)) as u64;
        } else {
            // cast: C++ explicit (uintb) of a non-negative int4
            val |= i64::from(byteoffset) as u64;
        }

        self.offset =
            ConstTpl::new_handle_plus(self.offset.get_handle_index(), VField::VOffsetPlus, val);
        true
    }

    /// C++ `encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_VARNODE_TPL);
        self.space.encode(encoder);
        self.offset.encode(encoder);
        self.size.encode(encoder);
        encoder.close_element(&sla::ELEM_VARNODE_TPL);
    }

    /// C++ `decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.open_element_id(&sla::ELEM_VARNODE_TPL)?;
        self.space.decode(decoder)?;
        self.offset.decode(decoder)?;
        self.size.decode(decoder)?;
        decoder.close_element(el)
    }
}

impl PartialEq for VarnodeTpl {
    /// C++ `operator==` (compares space/offset/size; `unnamed_flag` is not
    /// part of equality).
    fn eq(&self, op2: &VarnodeTpl) -> bool {
        self.space == op2.space && self.offset == op2.offset && self.size == op2.size
    }
}
impl Eq for VarnodeTpl {}

// ---------------------------------------------------------------------------
// HandleTpl
// ---------------------------------------------------------------------------

/// C++ `HandleTpl`: template for a FixedHandle (what a constructor
/// exports).
#[derive(Debug, Clone, Default)]
pub struct HandleTpl {
    space: ConstTpl,
    size: ConstTpl,
    ptrspace: ConstTpl,
    ptroffset: ConstTpl,
    ptrsize: ConstTpl,
    temp_space: ConstTpl,
    temp_offset: ConstTpl,
}

impl HandleTpl {
    /// C++ `HandleTpl(void)`.
    pub fn new() -> HandleTpl {
        HandleTpl::default()
    }

    /// C++ `HandleTpl(const VarnodeTpl *vn)`: build handle which indicates
    /// the given varnode.
    pub fn new_from_varnode(vn: &VarnodeTpl) -> HandleTpl {
        HandleTpl {
            space: vn.get_space().clone(),
            size: vn.get_size().clone(),
            ptrspace: ConstTpl::new_real(ConstType::Real, 0),
            ptroffset: vn.get_offset().clone(),
            ..HandleTpl::default()
        }
    }

    /// C++ `HandleTpl(const ConstTpl &spc,const ConstTpl &sz,const
    /// VarnodeTpl *vn,AddrSpace *t_space,uintb t_offset)`: build handle to
    /// the thing being pointed at by `vn`.
    pub fn new_ptr(
        spc: &ConstTpl,
        sz: &ConstTpl,
        vn: &VarnodeTpl,
        t_space: Rc<AddrSpace>,
        t_offset: u64,
    ) -> HandleTpl {
        HandleTpl {
            space: spc.clone(),
            size: sz.clone(),
            ptrspace: vn.get_space().clone(),
            ptroffset: vn.get_offset().clone(),
            ptrsize: vn.get_size().clone(),
            temp_space: ConstTpl::new_space(t_space),
            temp_offset: ConstTpl::new_real(ConstType::Real, t_offset),
        }
    }

    /// C++ `getSpace`.
    pub fn get_space(&self) -> &ConstTpl {
        &self.space
    }

    /// C++ `getPtrSpace`.
    pub fn get_ptr_space(&self) -> &ConstTpl {
        &self.ptrspace
    }

    /// C++ `getPtrOffset`.
    pub fn get_ptr_offset(&self) -> &ConstTpl {
        &self.ptroffset
    }

    /// C++ `getPtrSize`.
    pub fn get_ptr_size(&self) -> &ConstTpl {
        &self.ptrsize
    }

    /// C++ `getSize`.
    pub fn get_size(&self) -> &ConstTpl {
        &self.size
    }

    /// C++ `getTempSpace`.
    pub fn get_temp_space(&self) -> &ConstTpl {
        &self.temp_space
    }

    /// C++ `getTempOffset`.
    pub fn get_temp_offset(&self) -> &ConstTpl {
        &self.temp_offset
    }

    /// C++ `setSize`.
    pub fn set_size(&mut self, sz: ConstTpl) {
        self.size = sz;
    }

    /// C++ `setPtrSize`.
    pub fn set_ptr_size(&mut self, sz: ConstTpl) {
        self.ptrsize = sz;
    }

    /// C++ `setPtrOffset(uintb val)`.
    pub fn set_ptr_offset(&mut self, val: u64) {
        self.ptroffset = ConstTpl::new_real(ConstType::Real, val);
    }

    /// C++ `setTempOffset(uintb val)`.
    pub fn set_temp_offset(&mut self, val: u64) {
        self.temp_offset = ConstTpl::new_real(ConstType::Real, val);
    }

    /// C++ `fix`: resolve this template into `hand` against the walker.
    pub fn fix(&self, hand: &mut FixedHandle, walker: &dyn SymbolWalker) -> KunaResult<()> {
        if self.ptrspace.get_type() == ConstType::Real {
            // The export is unstarred, but this doesn't mean the varnode
            // being exported isn't dynamic
            self.space.fillin_space(hand, walker)?;
            hand.size = self.size.fix(walker)? as u32; // cast: C++ implicit uintb -> uint4
            self.ptroffset.fillin_offset(hand, walker)?;
        } else {
            hand.space = self.space.fix_space(walker)?;
            hand.size = self.size.fix(walker)? as u32; // cast: C++ implicit uintb -> uint4
            hand.offset_offset = self.ptroffset.fix(walker)?;
            hand.offset_space = self.ptrspace.fix_space(walker)?;
            let offset_space_type = hand
                .offset_space
                .as_ref()
                .expect(
                    "HandleTpl::fix: ptrspace resolved to null (C++ dereferences unconditionally)",
                )
                .get_type();
            if offset_space_type == spacetype::IPTR_CONSTANT {
                // Handle could have been dynamic but wasn't
                hand.offset_space = None;
                let space = hand.space.as_ref().expect(
                    "HandleTpl::fix: space resolved to null (C++ dereferences unconditionally)",
                );
                hand.offset_offset =
                    AddrSpace::address_to_byte(hand.offset_offset, space.get_word_size());
                hand.offset_offset = space.wrap_offset(hand.offset_offset);
            } else {
                hand.offset_size = self.ptrsize.fix(walker)? as u32; // cast: C++ implicit uintb -> uint4
                hand.temp_space = self.temp_space.fix_space(walker)?;
                hand.temp_offset = self.temp_offset.fix(walker)?;
            }
        }
        Ok(())
    }

    /// C++ `changeHandleIndex`.
    pub fn change_handle_index(&mut self, handmap: &[i32]) {
        self.space.change_handle_index(handmap);
        self.size.change_handle_index(handmap);
        self.ptrspace.change_handle_index(handmap);
        self.ptroffset.change_handle_index(handmap);
        self.ptrsize.change_handle_index(handmap);
        self.temp_space.change_handle_index(handmap);
        self.temp_offset.change_handle_index(handmap);
    }

    /// C++ `encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_HANDLE_TPL);
        self.space.encode(encoder);
        self.size.encode(encoder);
        self.ptrspace.encode(encoder);
        self.ptroffset.encode(encoder);
        self.ptrsize.encode(encoder);
        self.temp_space.encode(encoder);
        self.temp_offset.encode(encoder);
        encoder.close_element(&sla::ELEM_HANDLE_TPL);
    }

    /// C++ `decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.open_element_id(&sla::ELEM_HANDLE_TPL)?;
        self.space.decode(decoder)?;
        self.size.decode(decoder)?;
        self.ptrspace.decode(decoder)?;
        self.ptroffset.decode(decoder)?;
        self.ptrsize.decode(decoder)?;
        self.temp_space.decode(decoder)?;
        self.temp_offset.decode(decoder)?;
        decoder.close_element(el)
    }
}

// ---------------------------------------------------------------------------
// OpTpl
// ---------------------------------------------------------------------------

/// C++ `OpTpl`: a p-code op template.
#[derive(Debug, Clone)]
pub struct OpTpl {
    output: Option<VarnodeTpl>,
    opc: OpCode,
    input: Vec<VarnodeTpl>,
}

impl OpTpl {
    /// C++ `OpTpl(OpCode oc)`.
    pub fn new(oc: OpCode) -> OpTpl {
        OpTpl {
            output: None,
            opc: oc,
            input: Vec::new(),
        }
    }

    /// C++ `OpTpl(void)` (decode shell; C++ leaves `opc` uninitialized
    /// until `decode` runs, the port uses `CPUI_COPY` as the placeholder).
    pub fn new_for_decode() -> OpTpl {
        OpTpl::new(OpCode::CPUI_COPY)
    }

    /// C++ `getOut` (nullable).
    pub fn get_out(&self) -> Option<&VarnodeTpl> {
        self.output.as_ref()
    }

    /// Mutable access to the output (port-added; C++ mutates through the
    /// `VarnodeTpl *` returned by `getOut`).
    pub fn get_out_mut(&mut self) -> Option<&mut VarnodeTpl> {
        self.output.as_mut()
    }

    /// C++ `numInput`.
    pub fn num_input(&self) -> i32 {
        self.input.len() as i32 // cast: C++ returns vector::size() as int4
    }

    /// C++ `getIn(int4 i)`.
    pub fn get_in(&self, i: i32) -> &VarnodeTpl {
        &self.input[i as usize] // cast: C++ int4 vector index
    }

    /// Mutable access to an input (port-added; C++ mutates through the
    /// `VarnodeTpl *` returned by `getIn`).
    pub fn get_in_mut(&mut self, i: i32) -> &mut VarnodeTpl {
        &mut self.input[i as usize] // cast: C++ int4 vector index
    }

    /// C++ `getOpcode`.
    pub fn get_opcode(&self) -> OpCode {
        self.opc
    }

    /// C++ `isZeroSize`: return true if any input or output has zero size.
    pub fn is_zero_size(&self) -> bool {
        if let Some(output) = &self.output {
            if output.is_zero_size() {
                return true;
            }
        }
        for vn in &self.input {
            if vn.is_zero_size() {
                return true;
            }
        }
        false
    }

    /// C++ `setOpcode`.
    pub fn set_opcode(&mut self, o: OpCode) {
        self.opc = o;
    }

    /// C++ `setOutput` (takes ownership; callers free any previous output
    /// via `clearOutput` in C++, here it is simply replaced).
    pub fn set_output(&mut self, vt: VarnodeTpl) {
        self.output = Some(vt);
    }

    /// C++ `clearOutput`.
    pub fn clear_output(&mut self) {
        self.output = None;
    }

    /// C++ `addInput`.
    pub fn add_input(&mut self, vt: VarnodeTpl) {
        self.input.push(vt);
    }

    /// C++ `setInput(VarnodeTpl *vt,int4 slot)` (the old varnode is freed
    /// by `ConstructTpl::setInput` in C++; here it is simply replaced).
    pub fn set_input(&mut self, vt: VarnodeTpl, slot: i32) {
        self.input[slot as usize] = vt; // cast: C++ int4 vector index
    }

    /// C++ `removeInput(int4 index)` (the C++ shift-down loop + pop_back is
    /// `Vec::remove`).
    pub fn remove_input(&mut self, index: i32) {
        self.input.remove(index as usize); // cast: C++ int4 vector index
    }

    /// C++ `changeHandleIndex`.
    pub fn change_handle_index(&mut self, handmap: &[i32]) {
        if let Some(output) = &mut self.output {
            output.change_handle_index(handmap);
        }
        for vn in &mut self.input {
            vn.change_handle_index(handmap);
        }
    }

    /// C++ `encode`.
    pub fn encode(&self, encoder: &mut dyn OpcodeEncoder) {
        encoder.open_element(&sla::ELEM_OP_TPL);
        encoder.write_opcode(&sla::ATTRIB_CODE, self.opc);
        match &self.output {
            None => {
                encoder.open_element(&sla::ELEM_NULL);
                encoder.close_element(&sla::ELEM_NULL);
            }
            Some(output) => output.encode(encoder),
        }
        for vn in &self.input {
            vn.encode(encoder);
        }
        encoder.close_element(&sla::ELEM_OP_TPL);
    }

    /// C++ `decode`.
    pub fn decode(&mut self, decoder: &mut dyn OpcodeDecoder) -> KunaResult<()> {
        let el = decoder.open_element_id(&sla::ELEM_OP_TPL)?;
        self.opc = decoder.read_opcode_id(&sla::ATTRIB_CODE)?;
        let subel = decoder.peek_element()?;
        if subel == sla::ELEM_NULL {
            decoder.open_element()?;
            decoder.close_element(subel)?;
            self.output = None;
        } else {
            let mut output = VarnodeTpl::default();
            output.decode(decoder)?;
            self.output = Some(output);
        }
        while decoder.peek_element()? != 0 {
            let mut vn = VarnodeTpl::default();
            vn.decode(decoder)?;
            self.input.push(vn);
        }
        decoder.close_element(el)
    }
}

// ---------------------------------------------------------------------------
// ConstructTpl
// ---------------------------------------------------------------------------

/// C++ `ConstructTpl`: the full p-code template of one constructor section.
#[derive(Debug, Clone, Default)]
pub struct ConstructTpl {
    /// C++ `delayslot` (uint4)
    delayslot: u32,
    /// C++ `numlabels`: number of label templates (uint4)
    numlabels: u32,
    /// C++ `vec` (owned op templates)
    vec: Vec<OpTpl>,
    /// C++ `result` (nullable owned handle template)
    result: Option<HandleTpl>,
}

impl ConstructTpl {
    /// C++ `ConstructTpl(void)`.
    pub fn new() -> ConstructTpl {
        ConstructTpl::default()
    }

    /// C++ `delaySlot`.
    pub fn delay_slot(&self) -> u32 {
        self.delayslot
    }

    /// C++ `numLabels`.
    pub fn num_labels(&self) -> u32 {
        self.numlabels
    }

    /// C++ `getOpvec`.
    pub fn get_opvec(&self) -> &[OpTpl] {
        &self.vec
    }

    /// Mutable op access (port-added; C++ mutates ops through the
    /// `vector<OpTpl *>` pointees, e.g. pcodecompile.cc size propagation).
    pub fn get_opvec_mut(&mut self) -> &mut Vec<OpTpl> {
        &mut self.vec
    }

    /// C++ `getResult` (nullable).
    pub fn get_result(&self) -> Option<&HandleTpl> {
        self.result.as_ref()
    }

    /// C++ `addOp`: returns false on a duplicate delay-slot op.
    pub fn add_op(&mut self, ot: OpTpl) -> bool {
        if ot.get_opcode() == DELAY_SLOT {
            if self.delayslot != 0 {
                return false; // Cannot have multiple delay slots
            }
            // cast: C++ implicit uintb -> uint4
            self.delayslot = ot.get_in(0).get_offset().get_real() as u32;
        } else if ot.get_opcode() == LABELBUILD {
            self.numlabels = self.numlabels.wadd(1); // Count labels
        }
        self.vec.push(ot);
        true
    }

    /// C++ `addOpList`.
    pub fn add_op_list(&mut self, oplist: Vec<OpTpl>) -> bool {
        for ot in oplist {
            if !self.add_op(ot) {
                return false;
            }
        }
        true
    }

    /// C++ `setResult`.
    pub fn set_result(&mut self, t: Option<HandleTpl>) {
        self.result = t;
    }

    /// C++ `fillinBuild`: make sure there is a build statement for all
    /// subtable params.  Return 0 upon success, 1 if there is a duplicate
    /// BUILD, 2 if there is a build for a non-subtable.
    pub fn fillin_build(&mut self, check: &mut [i32], const_space: &Rc<AddrSpace>) -> i32 {
        for op in &self.vec {
            if op.get_opcode() == BUILD {
                // cast: C++ implicit uintb -> int4, then vector index
                let index = op.get_in(0).get_offset().get_real() as i32;
                if check[index as usize] != 0 {
                    // Duplicate BUILD statement or non-subtable
                    return check[index as usize];
                }
                check[index as usize] = 1; // Mark to avoid future duplicate build
            }
        }
        for (i, chk) in check.iter().enumerate() {
            if *chk == 0 {
                // Didn't see a BUILD statement
                let mut op = OpTpl::new(BUILD);
                let indvn = VarnodeTpl::new(
                    ConstTpl::new_space(Rc::clone(const_space)),
                    // cast: C++ int4 loop index -> uintb (non-negative)
                    ConstTpl::new_real(ConstType::Real, i as u64),
                    ConstTpl::new_real(ConstType::Real, 4),
                );
                op.add_input(indvn);
                self.vec.insert(0, op);
            }
        }
        0
    }

    /// C++ `buildOnly`.
    pub fn build_only(&self) -> bool {
        for op in &self.vec {
            if op.get_opcode() != BUILD {
                return false;
            }
        }
        true
    }

    /// C++ `changeHandleIndex`.
    pub fn change_handle_index(&mut self, handmap: &[i32]) {
        for op in &mut self.vec {
            if op.get_opcode() == BUILD {
                // cast: C++ implicit uintb -> int4, then vector index
                let mut index = op.get_in(0).get_offset().get_real() as i32;
                index = handmap[index as usize];
                // cast: C++ implicit int4 -> uintb sign extension
                op.get_in_mut(0).set_offset(i64::from(index) as u64);
            } else {
                op.change_handle_index(handmap);
            }
        }
        if let Some(result) = &mut self.result {
            result.change_handle_index(handmap);
        }
    }

    /// C++ `setInput`: set the VarnodeTpl input for a particular op (for
    /// use with optimization routines).
    pub fn set_input(&mut self, vn: VarnodeTpl, index: i32, slot: i32) {
        // cast: C++ int4 vector index; the old varnode is dropped (C++ delete)
        self.vec[index as usize].set_input(vn, slot);
    }

    /// C++ `setOutput`: set the VarnodeTpl output for a particular op (for
    /// use with optimization routines).
    pub fn set_output(&mut self, vn: VarnodeTpl, index: i32) {
        // cast: C++ int4 vector index; the old varnode is dropped (C++ delete)
        self.vec[index as usize].set_output(vn);
    }

    /// C++ `deleteOps`: delete a particular set of ops (C++ nulls the slots
    /// then compacts preserving order).
    pub fn delete_ops(&mut self, indices: &[i32]) {
        let mut removed = vec![false; self.vec.len()];
        for &i in indices {
            removed[i as usize] = true; // cast: C++ int4 vector index
        }
        let mut poscur: usize = 0;
        for (i, rem) in removed.iter().enumerate() {
            if !*rem {
                self.vec.swap(poscur, i);
                poscur += 1;
            }
        }
        self.vec.truncate(poscur);
    }

    /// C++ `encode(Encoder &,int4 sectionid)`.
    pub fn encode(&self, encoder: &mut dyn OpcodeEncoder, sectionid: i32) {
        encoder.open_element(&sla::ELEM_CONSTRUCT_TPL);
        if sectionid >= 0 {
            encoder.write_signed_integer(&sla::ATTRIB_SECTION, i64::from(sectionid));
        }
        if self.delayslot != 0 {
            encoder.write_signed_integer(&sla::ATTRIB_DELAY, i64::from(self.delayslot));
        }
        if self.numlabels != 0 {
            encoder.write_signed_integer(&sla::ATTRIB_LABELS, i64::from(self.numlabels));
        }
        match &self.result {
            Some(result) => result.encode(encoder),
            None => {
                encoder.open_element(&sla::ELEM_NULL);
                encoder.close_element(&sla::ELEM_NULL);
            }
        }
        for op in &self.vec {
            op.encode(encoder);
        }
        encoder.close_element(&sla::ELEM_CONSTRUCT_TPL);
    }

    /// C++ `decode`: returns the section id (-1 for the main section).
    pub fn decode(&mut self, decoder: &mut dyn OpcodeDecoder) -> KunaResult<i32> {
        let el = decoder.open_element_id(&sla::ELEM_CONSTRUCT_TPL)?;
        let mut sectionid: i32 = -1;
        let mut attrib = decoder.get_next_attribute_id()?;
        while attrib != 0 {
            if attrib == sla::ATTRIB_DELAY {
                // cast: C++ implicit intb -> uint4
                self.delayslot = decoder.read_signed_integer()? as u32;
            } else if attrib == sla::ATTRIB_LABELS {
                // cast: C++ implicit intb -> uint4
                self.numlabels = decoder.read_signed_integer()? as u32;
            } else if attrib == sla::ATTRIB_SECTION {
                // cast: C++ implicit intb -> int4
                sectionid = decoder.read_signed_integer()? as i32;
            }
            attrib = decoder.get_next_attribute_id()?;
        }
        let subel = decoder.peek_element()?;
        if subel == sla::ELEM_NULL {
            decoder.open_element()?;
            decoder.close_element(subel)?;
            self.result = None;
        } else {
            let mut result = HandleTpl::default();
            result.decode(decoder)?;
            self.result = Some(result);
        }
        while decoder.peek_element()? != 0 {
            let mut op = OpTpl::new_for_decode();
            op.decode(decoder)?;
            self.vec.push(op);
        }
        decoder.close_element(el)?;
        Ok(sectionid)
    }
}

// ---------------------------------------------------------------------------
// PcodeBuilder
// ---------------------------------------------------------------------------

/// C++ `PcodeBuilder`: SLEIGH-specific p-code generator.  The C++ abstract
/// class becomes a trait: the pure virtuals (`dump`, `appendBuild`,
/// `delaySlot`, `setLabel`, `appendCrossBuild`) and accessors for the C++
/// private `labelbase`/`labelcount` fields are required methods backed by
/// implementor state (initialize both to the C++ constructor's `lbcnt`);
/// `build` is the C++ concrete dispatch loop.  The C++ protected
/// `ParserWalker *walker` member and `getCurrentWalker()` live with the
/// implementor (the sleigh decode-engine wave).
pub trait PcodeBuilder {
    /// C++ `getLabelBase`.
    fn get_label_base(&self) -> u32;
    /// Setter for the C++ private `labelbase` field (used by `build`).
    fn set_label_base(&mut self, val: u32);
    /// Accessor for the C++ private `labelcount` field (used by `build`).
    fn get_label_count(&self) -> u32;
    /// Setter for the C++ private `labelcount` field (used by `build`).
    fn set_label_count(&mut self, val: u32);

    /// C++ pure virtual `dump`.
    fn dump(&mut self, op: &OpTpl) -> KunaResult<()>;
    /// C++ pure virtual `appendBuild`.
    fn append_build(&mut self, bld: &OpTpl, secnum: i32) -> KunaResult<()>;
    /// C++ pure virtual `delaySlot`.
    fn delay_slot(&mut self, op: &OpTpl) -> KunaResult<()>;
    /// C++ pure virtual `setLabel`.
    fn set_label(&mut self, op: &OpTpl) -> KunaResult<()>;
    /// C++ pure virtual `appendCrossBuild`.
    fn append_cross_build(&mut self, bld: &OpTpl, secnum: i32) -> KunaResult<()>;

    /// C++ `PcodeBuilder::build` (semantics.cc): `None` mirrors the C++
    /// null `ConstructTpl *` and produces `UnimplError("",0)`.
    fn build(&mut self, construct: Option<&ConstructTpl>, secnum: i32) -> KunaResult<()> {
        let construct = match construct {
            // Pcode is not implemented for this constructor
            None => return Err(KunaError::unimpl("", 0)),
            Some(c) => c,
        };

        let oldbase = self.get_label_base(); // Recursively store old labelbase
        self.set_label_base(self.get_label_count()); // Set the newbase
                                                     // Add labels from this template
        self.set_label_count(self.get_label_count().wadd(construct.num_labels()));

        for op in construct.get_opvec() {
            match op.get_opcode() {
                // BUILD
                OpCode::CPUI_MULTIEQUAL => self.append_build(op, secnum)?,
                // DELAY_SLOT
                OpCode::CPUI_INDIRECT => self.delay_slot(op)?,
                // LABELBUILD
                OpCode::CPUI_PTRADD => self.set_label(op)?,
                // CROSSBUILD
                OpCode::CPUI_PTRSUB => self.append_cross_build(op, secnum)?,
                _ => self.dump(op)?,
            }
        }
        self.set_label_base(oldbase); // Restore old labelbase
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use kuna_base::address::Address;
    use kuna_base::marshal::{PackedDecode, PackedEncode};
    use kuna_base::space::{addrspace_flags, AddrSpaceManager, ConstantSpace, UniqueSpace};

    use super::*;
    use crate::slghpatexpress::PatternExpressionContext;
    use crate::slghsymbol::ConstructorRef;

    // -- fixtures ------------------------------------------------------------

    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            2, // wordsize 2 to exercise addressToByte
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(ram)).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(2, 0, false)))
            .unwrap();
        manager
    }

    fn cspace(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_constant_space().unwrap())
    }

    fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("ram").unwrap())
    }

    fn uniq(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("unique").unwrap())
    }

    /// Synthetic ParserWalker seam for the fix() resolution matrix.
    struct TestWalker {
        addr: Address,
        naddr: Address,
        n2addr: Option<Address>,
        ref_addr: Option<Address>,
        dest_addr: Option<Address>,
        const_space: Rc<AddrSpace>,
        cur_space: Rc<AddrSpace>,
        handles: Vec<FixedHandle>,
    }

    impl TestWalker {
        fn new(manager: &AddrSpaceManager) -> TestWalker {
            TestWalker {
                addr: Address::new(ram(manager), 0x1000),
                naddr: Address::new(ram(manager), 0x1004),
                n2addr: None,
                ref_addr: None,
                dest_addr: None,
                const_space: cspace(manager),
                cur_space: ram(manager),
                handles: Vec::new(),
            }
        }
    }

    impl PatternExpressionContext for TestWalker {
        fn get_instruction_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
            Err(KunaError::sleigh("no instruction bytes in semantics tests"))
        }
        fn get_context_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
            Err(KunaError::sleigh("no context bytes in semantics tests"))
        }
        fn get_addr(&self) -> Address {
            self.addr.clone()
        }
        fn get_naddr(&self) -> Address {
            self.naddr.clone()
        }
        fn get_n2addr(&self) -> KunaResult<Address> {
            self.n2addr
                .clone()
                .ok_or_else(|| KunaError::lowlevel("inst_next2 not available in this context"))
        }
        fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
            Err(KunaError::sleigh("no operand values in semantics tests"))
        }
    }

    impl SymbolWalker for TestWalker {
        fn push_operand(&mut self, _i: i32) -> KunaResult<()> {
            Ok(())
        }
        fn pop_operand(&mut self) -> KunaResult<()> {
            Ok(())
        }
        fn get_constructor(&self) -> KunaResult<ConstructorRef> {
            Err(KunaError::sleigh("no constructor in semantics tests"))
        }
        fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
            self.handles
                .get(i as usize)
                .cloned()
                .ok_or_else(|| KunaError::sleigh("no fixed handle for operand"))
        }
        fn get_const_space(&self) -> Rc<AddrSpace> {
            Rc::clone(&self.const_space)
        }
        fn get_cur_space(&self) -> Rc<AddrSpace> {
            Rc::clone(&self.cur_space)
        }
        fn get_dest_addr(&self) -> KunaResult<Address> {
            self.dest_addr
                .clone()
                .ok_or_else(|| KunaError::lowlevel("no flowdest"))
        }
        fn get_ref_addr(&self) -> KunaResult<Address> {
            self.ref_addr
                .clone()
                .ok_or_else(|| KunaError::lowlevel("no flowref"))
        }
        fn get_instruction_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
            Err(KunaError::sleigh("no instruction bits in semantics tests"))
        }
        fn get_context_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
            Err(KunaError::sleigh("no context bits in semantics tests"))
        }
    }

    /// Static handle: ram:0x80, size 4.
    fn static_handle(manager: &AddrSpaceManager) -> FixedHandle {
        FixedHandle {
            space: Some(ram(manager)),
            size: 4,
            offset_space: None,
            offset_offset: 0x80,
            offset_size: 0,
            temp_space: None,
            temp_offset: 0,
        }
    }

    /// Dynamic handle: value lives at unique:0x300, pointer in ram.
    fn dynamic_handle(manager: &AddrSpaceManager) -> FixedHandle {
        FixedHandle {
            space: Some(ram(manager)),
            size: 4,
            offset_space: Some(ram(manager)),
            offset_offset: 0x40,
            offset_size: 4,
            temp_space: Some(uniq(manager)),
            temp_offset: 0x300,
        }
    }

    /// Constant handle: const-space value.
    fn const_handle(manager: &AddrSpaceManager) -> FixedHandle {
        FixedHandle {
            space: Some(cspace(manager)),
            size: 8,
            offset_space: None,
            offset_offset: 0xdead_beef_1234_5678,
            offset_size: 0,
            temp_space: None,
            temp_offset: 0,
        }
    }

    // -- ConstTpl::fix resolution matrix ---------------------------------------

    #[test]
    fn semantics_consttpl_fix_addresses_and_spaces() {
        let manager = test_manager();
        let mut walker = TestWalker::new(&manager);
        walker.ref_addr = Some(Address::new(ram(&manager), 0x2000));
        walker.dest_addr = Some(Address::new(ram(&manager), 0x3000));

        assert_eq!(
            ConstTpl::new_real(ConstType::Real, 0x55)
                .fix(&walker)
                .unwrap(),
            0x55
        );
        assert_eq!(
            ConstTpl::new_real(ConstType::JRelative, 7)
                .fix(&walker)
                .unwrap(),
            7
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JStart).fix(&walker).unwrap(),
            0x1000
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JNext).fix(&walker).unwrap(),
            0x1004
        );
        // j_next2 errors through the walker when unavailable
        assert!(ConstTpl::new_type(ConstType::JNext2).fix(&walker).is_err());
        assert_eq!(
            ConstTpl::new_type(ConstType::JFlowref)
                .fix(&walker)
                .unwrap(),
            0x2000
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JFlowrefSize)
                .fix(&walker)
                .unwrap(),
            4
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JFlowdest)
                .fix(&walker)
                .unwrap(),
            0x3000
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JFlowdestSize)
                .fix(&walker)
                .unwrap(),
            4
        );
        assert_eq!(
            ConstTpl::new_type(ConstType::JCurspaceSize)
                .fix(&walker)
                .unwrap(),
            4
        );
        // spaces resolve to their manager index (the C++ pointer-as-uintb)
        assert_eq!(
            ConstTpl::new_type(ConstType::JCurspace)
                .fix(&walker)
                .unwrap(),
            u64::try_from(ram(&manager).get_index()).unwrap()
        );
        assert_eq!(
            ConstTpl::new_space(uniq(&manager)).fix(&walker).unwrap(),
            u64::try_from(uniq(&manager).get_index()).unwrap()
        );
    }

    #[test]
    fn semantics_consttpl_fix_handle_static_and_dynamic() {
        let manager = test_manager();
        let mut walker = TestWalker::new(&manager);
        walker.handles.push(static_handle(&manager));
        walker.handles.push(dynamic_handle(&manager));

        // static handle: space, offset, size
        assert_eq!(
            ConstTpl::new_handle(0, VField::VSpace)
                .fix(&walker)
                .unwrap(),
            u64::try_from(ram(&manager).get_index()).unwrap()
        );
        assert_eq!(
            ConstTpl::new_handle(0, VField::VOffset)
                .fix(&walker)
                .unwrap(),
            0x80
        );
        assert_eq!(
            ConstTpl::new_handle(0, VField::VSize).fix(&walker).unwrap(),
            4
        );
        // dynamic handle reads the temporary storage
        assert_eq!(
            ConstTpl::new_handle(1, VField::VSpace)
                .fix(&walker)
                .unwrap(),
            u64::try_from(uniq(&manager).get_index()).unwrap()
        );
        assert_eq!(
            ConstTpl::new_handle(1, VField::VOffset)
                .fix(&walker)
                .unwrap(),
            0x300
        );
    }

    #[test]
    fn semantics_consttpl_fix_offset_plus() {
        let manager = test_manager();
        let mut walker = TestWalker::new(&manager);
        walker.handles.push(static_handle(&manager));
        walker.handles.push(dynamic_handle(&manager));
        walker.handles.push(const_handle(&manager));

        // non-constant static: offset adjusted by the low-16 truncation amount
        assert_eq!(
            ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0003_0002)
                .fix(&walker)
                .unwrap(),
            0x80 + 2
        );
        // non-constant dynamic: temp offset adjusted
        assert_eq!(
            ConstTpl::new_handle_plus(1, VField::VOffsetPlus, 0x0003_0002)
                .fix(&walker)
                .unwrap(),
            0x300 + 2
        );
        // constant: value shifted down by 8*(plus>>16) bits
        assert_eq!(
            ConstTpl::new_handle_plus(2, VField::VOffsetPlus, 0x0004_0004)
                .fix(&walker)
                .unwrap(),
            0xdead_beef_1234_5678u64 >> 32
        );
    }

    #[test]
    fn semantics_consttpl_fixspace_matrix() {
        let manager = test_manager();
        let mut walker = TestWalker::new(&manager);
        walker.ref_addr = Some(Address::new(ram(&manager), 0x2000));
        walker.handles.push(static_handle(&manager));
        walker.handles.push(dynamic_handle(&manager));

        let cur = ConstTpl::new_type(ConstType::JCurspace)
            .fix_space(&walker)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&cur, &ram(&manager)));
        let sid = ConstTpl::new_space(uniq(&manager))
            .fix_space(&walker)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&sid, &uniq(&manager)));
        let h0 = ConstTpl::new_handle(0, VField::VSpace)
            .fix_space(&walker)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&h0, &ram(&manager)));
        let h1 = ConstTpl::new_handle(1, VField::VSpace)
            .fix_space(&walker)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&h1, &uniq(&manager)));
        let fr = ConstTpl::new_type(ConstType::JFlowref)
            .fix_space(&walker)
            .unwrap()
            .unwrap();
        assert!(Rc::ptr_eq(&fr, &ram(&manager)));
        // not a space: the C++ LowlevelError
        let err = ConstTpl::new_real(ConstType::Real, 4)
            .fix_space(&walker)
            .unwrap_err();
        assert_eq!(err.explain(), "ConstTpl is not a spaceid as expected");
        // handle with a non-space select also throws
        assert!(ConstTpl::new_handle(0, VField::VOffset)
            .fix_space(&walker)
            .is_err());
    }

    #[test]
    fn semantics_handletpl_fix_unstarred() {
        let manager = test_manager();
        let walker = TestWalker::new(&manager);

        // HandleTpl(vn) for a static varnode ram:0x42:4
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(ram(&manager)),
            ConstTpl::new_real(ConstType::Real, 0x42),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let tpl = HandleTpl::new_from_varnode(&vn);
        let mut hand = FixedHandle::default();
        tpl.fix(&mut hand, &walker).unwrap();
        assert!(Rc::ptr_eq(hand.space.as_ref().unwrap(), &ram(&manager)));
        assert_eq!(hand.size, 4);
        assert!(hand.offset_space.is_none());
        assert_eq!(hand.offset_offset, 0x42);
    }

    #[test]
    fn semantics_handletpl_fix_starred_constant_pointer() {
        let manager = test_manager();
        let walker = TestWalker::new(&manager);

        // *[ram]:4 of the constant 0x21 (a star that wasn't dynamic):
        // ptrspace is the constant space, so fix folds the pointer into a
        // static offset, converting address units to bytes (wordsize 2).
        let ptr_vn = VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 0x21),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let tpl = HandleTpl::new_ptr(
            &ConstTpl::new_space(ram(&manager)),
            &ConstTpl::new_real(ConstType::Real, 4),
            &ptr_vn,
            uniq(&manager),
            0x100,
        );
        let mut hand = FixedHandle::default();
        tpl.fix(&mut hand, &walker).unwrap();
        assert!(Rc::ptr_eq(hand.space.as_ref().unwrap(), &ram(&manager)));
        assert_eq!(hand.size, 4);
        assert!(hand.offset_space.is_none());
        // addressToByte multiplies by the wordsize
        assert_eq!(hand.offset_offset, 0x42);
    }

    #[test]
    fn semantics_handletpl_fix_starred_dynamic_pointer() {
        let manager = test_manager();
        let mut walker = TestWalker::new(&manager);
        walker.handles.push(static_handle(&manager));

        // *[ram]:4 handle0 — a dynamic export: pointer pieces + temp location
        let ptr_vn = VarnodeTpl::new_from_handle(0, false);
        let tpl = HandleTpl::new_ptr(
            &ConstTpl::new_space(ram(&manager)),
            &ConstTpl::new_real(ConstType::Real, 4),
            &ptr_vn,
            uniq(&manager),
            0x100,
        );
        let mut hand = FixedHandle::default();
        tpl.fix(&mut hand, &walker).unwrap();
        assert!(Rc::ptr_eq(hand.space.as_ref().unwrap(), &ram(&manager)));
        assert_eq!(hand.size, 4);
        assert!(Rc::ptr_eq(
            hand.offset_space.as_ref().unwrap(),
            &ram(&manager)
        ));
        assert_eq!(hand.offset_offset, 0x80);
        assert_eq!(hand.offset_size, 4);
        assert!(Rc::ptr_eq(
            hand.temp_space.as_ref().unwrap(),
            &uniq(&manager)
        ));
        assert_eq!(hand.temp_offset, 0x100);
    }

    // -- equality / ordering ----------------------------------------------------

    #[test]
    fn semantics_consttpl_equality_and_order() {
        let manager = test_manager();
        let r5 = ConstTpl::new_real(ConstType::Real, 5);
        let r9 = ConstTpl::new_real(ConstType::Real, 9);
        assert_eq!(r5, ConstTpl::new_real(ConstType::Real, 5));
        assert_ne!(r5, r9);
        assert!(r5.less_than(&r9));
        assert!(!r9.less_than(&r5));
        // type order dominates
        assert!(r9.less_than(&ConstTpl::new_handle(0, VField::VSpace)));
        // handle order: index then select
        let h0s = ConstTpl::new_handle(0, VField::VSpace);
        let h0o = ConstTpl::new_handle(0, VField::VOffset);
        let h1s = ConstTpl::new_handle(1, VField::VSpace);
        assert!(h0s.less_than(&h0o));
        assert!(h0o.less_than(&h1s));
        assert_ne!(h0s, h0o);
        // spaceid: pointer equality
        let s1 = ConstTpl::new_space(ram(&manager));
        let s2 = ConstTpl::new_space(ram(&manager));
        assert_eq!(s1, s2);
        assert!(!s1.less_than(&s2));
        assert_ne!(s1, ConstTpl::new_space(uniq(&manager)));
        // j_* types compare equal beyond the type
        assert_eq!(
            ConstTpl::new_type(ConstType::JStart),
            ConstTpl::new_type(ConstType::JStart)
        );
        assert!(ConstTpl::new_real(ConstType::Real, 0).is_zero());
        assert!(!ConstTpl::new_real(ConstType::JRelative, 0).is_zero());
    }

    // -- transfer ---------------------------------------------------------------

    #[test]
    fn semantics_consttpl_transfer_through_handles() {
        let manager = test_manager();
        // params[0]: handle exporting ram:0x42:4
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(ram(&manager)),
            ConstTpl::new_real(ConstType::Real, 0x42),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let params = vec![HandleTpl::new_from_varnode(&vn)];

        let mut space = ConstTpl::new_handle(0, VField::VSpace);
        space.transfer(&params).unwrap();
        assert_eq!(space.get_type(), ConstType::Spaceid);
        assert!(Rc::ptr_eq(&space.get_space(), &ram(&manager)));

        let mut off = ConstTpl::new_handle(0, VField::VOffset);
        off.transfer(&params).unwrap();
        assert_eq!(off.get_type(), ConstType::Real);
        assert_eq!(off.get_real(), 0x42);

        let mut size = ConstTpl::new_handle(0, VField::VSize);
        size.transfer(&params).unwrap();
        assert_eq!(size.get_real(), 4);

        // offset_plus onto a real ptroffset adds the truncation amount
        let mut plus = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0001_0002);
        plus.transfer(&params).unwrap();
        assert_eq!(plus.get_type(), ConstType::Real);
        assert_eq!(plus.get_real(), 0x42 + 2);

        // offset_plus onto a handle ptroffset re-targets the truncation
        let hvn = VarnodeTpl::new_from_handle(3, false);
        let hparams = vec![HandleTpl::new_from_varnode(&hvn)];
        let mut plus2 = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0001_0002);
        plus2.transfer(&hparams).unwrap();
        assert_eq!(plus2.get_type(), ConstType::Handle);
        assert_eq!(plus2.get_handle_index(), 3);
        assert_eq!(plus2.get_select(), VField::VOffsetPlus);
        assert_eq!(plus2.get_real(), 0x0001_0002);

        // offset_plus onto anything else is the C++ LowlevelError
        let jvn = VarnodeTpl::new(
            ConstTpl::new_space(ram(&manager)),
            ConstTpl::new_type(ConstType::JStart),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let jparams = vec![HandleTpl::new_from_varnode(&jvn)];
        let mut plus3 = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 2);
        let err = plus3.transfer(&jparams).unwrap_err();
        assert_eq!(err.explain(), "Cannot truncate macro input in this way");
    }

    #[test]
    fn semantics_varnodetpl_transfer_truncation_returns() {
        let manager = test_manager();
        // local-temp varnode with an offset_plus offset
        let mut vn = VarnodeTpl::new(
            ConstTpl::new_space(uniq(&manager)),
            ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0000_0002),
            ConstTpl::new_real(ConstType::Real, 2),
        );
        // params[0] export: unique:0x10:4 (so vn stays a local temp)
        let exp = VarnodeTpl::new(
            ConstTpl::new_space(uniq(&manager)),
            ConstTpl::new_real(ConstType::Real, 0x10),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let params = vec![HandleTpl::new_from_varnode(&exp)];
        assert_eq!(vn.transfer(&params).unwrap(), 2);

        // non-temp varnode without offset_plus returns -1
        let mut plain = VarnodeTpl::new(
            ConstTpl::new_space(ram(&manager)),
            ConstTpl::new_real(ConstType::Real, 0x42),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        assert_eq!(plain.transfer(&params).unwrap(), -1);
    }

    #[test]
    fn semantics_varnodetpl_adjust_truncation() {
        // little endian: encoded as (byteoffset<<16) | byteoffset
        let mut vn = VarnodeTpl::new(
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_handle_plus(2, VField::VOffsetPlus, 2),
            ConstTpl::new_real(ConstType::Real, 2),
        );
        assert!(vn.adjust_truncation(8, false));
        assert_eq!(vn.get_offset().get_type(), ConstType::Handle);
        assert_eq!(vn.get_offset().get_handle_index(), 2);
        assert_eq!(vn.get_offset().get_real(), (2u64 << 16) | 2);

        // big endian flips the in-byte offset: size 2 at byteoffset 2 of 8
        let mut vn2 = VarnodeTpl::new(
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_handle_plus(2, VField::VOffsetPlus, 2),
            ConstTpl::new_real(ConstType::Real, 2),
        );
        assert!(vn2.adjust_truncation(8, true));
        assert_eq!(vn2.get_offset().get_real(), (2u64 << 16) | 4);

        // out of bounds
        let mut vn3 = VarnodeTpl::new(
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_handle_plus(2, VField::VOffsetPlus, 7),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        assert!(!vn3.adjust_truncation(8, false));
    }

    // -- ConstructTpl bookkeeping -------------------------------------------

    fn copy_op(manager: &AddrSpaceManager) -> OpTpl {
        let mut op = OpTpl::new(OpCode::CPUI_COPY);
        op.add_input(VarnodeTpl::new(
            ConstTpl::new_space(ram(manager)),
            ConstTpl::new_real(ConstType::Real, 0x42),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        op.set_output(VarnodeTpl::new(
            ConstTpl::new_space(uniq(manager)),
            ConstTpl::new_real(ConstType::Real, 0x10),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        op
    }

    fn build_op(manager: &AddrSpaceManager, index: u64) -> OpTpl {
        let mut op = OpTpl::new(BUILD);
        op.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(manager)),
            ConstTpl::new_real(ConstType::Real, index),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        op
    }

    #[test]
    fn semantics_constructtpl_addop_counts_labels_and_delayslots() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();

        let mut label = OpTpl::new(LABELBUILD);
        label.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        assert!(ct.add_op(label));
        assert_eq!(ct.num_labels(), 1);

        let mut ds = OpTpl::new(DELAY_SLOT);
        ds.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 1),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        assert!(ct.add_op(ds));
        assert_eq!(ct.delay_slot(), 1);

        // second delay slot refused
        let mut ds2 = OpTpl::new(DELAY_SLOT);
        ds2.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 2),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        assert!(!ct.add_op(ds2));
    }

    #[test]
    fn semantics_constructtpl_fillin_build_and_buildonly() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        ct.add_op(build_op(&manager, 1));
        // operands 0 and 2 have no BUILD: fillinBuild inserts them at the front
        let mut check = vec![0i32; 3];
        assert_eq!(ct.fillin_build(&mut check, &cspace(&manager)), 0);
        assert_eq!(ct.get_opvec().len(), 3);
        assert!(ct.build_only());
        // inserted BUILDs are prepended (op for index 2 inserted last, ends
        // up first)
        let offsets: Vec<u64> = ct
            .get_opvec()
            .iter()
            .map(|op| op.get_in(0).get_offset().get_real())
            .collect();
        assert_eq!(offsets, vec![2, 0, 1]);

        // duplicate BUILD reports the stored check value
        let mut ct2 = ConstructTpl::new();
        ct2.add_op(build_op(&manager, 0));
        ct2.add_op(build_op(&manager, 0));
        let mut check2 = vec![0i32; 1];
        assert_eq!(ct2.fillin_build(&mut check2, &cspace(&manager)), 1);

        // non-subtable marker 2 is propagated
        let mut ct3 = ConstructTpl::new();
        ct3.add_op(build_op(&manager, 0));
        let mut check3 = vec![2i32; 1];
        assert_eq!(ct3.fillin_build(&mut check3, &cspace(&manager)), 2);

        let mut ct4 = ConstructTpl::new();
        ct4.add_op(copy_op(&manager));
        assert!(!ct4.build_only());
    }

    #[test]
    fn semantics_constructtpl_delete_ops_compacts_in_order() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        for i in 0..5 {
            ct.add_op(build_op(&manager, i));
        }
        ct.delete_ops(&[1, 3]);
        let offsets: Vec<u64> = ct
            .get_opvec()
            .iter()
            .map(|op| op.get_in(0).get_offset().get_real())
            .collect();
        assert_eq!(offsets, vec![0, 2, 4]);
    }

    #[test]
    fn semantics_change_handle_index_remaps_builds_and_handles() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        ct.add_op(build_op(&manager, 0));
        let mut op = OpTpl::new(OpCode::CPUI_COPY);
        op.add_input(VarnodeTpl::new_from_handle(1, false));
        op.set_output(VarnodeTpl::new_from_handle(0, false));
        ct.add_op(op);
        ct.set_result(Some(HandleTpl::new_from_varnode(
            &VarnodeTpl::new_from_handle(1, false),
        )));

        let handmap = vec![5, 7];
        ct.change_handle_index(&handmap);
        // BUILD input offset remapped through the map
        assert_eq!(ct.get_opvec()[0].get_in(0).get_offset().get_real(), 5);
        // ordinary op handles remapped
        assert_eq!(
            ct.get_opvec()[1].get_in(0).get_offset().get_handle_index(),
            7
        );
        assert_eq!(
            ct.get_opvec()[1]
                .get_out()
                .unwrap()
                .get_offset()
                .get_handle_index(),
            5
        );
        // result handle remapped
        assert_eq!(
            ct.get_result().unwrap().get_ptr_offset().get_handle_index(),
            7
        );
    }

    // -- decode round-trips -----------------------------------------------------

    /// Encode with PackedEncode, decode, re-encode, compare byte streams.
    fn roundtrip_consttpl(manager: &AddrSpaceManager, ct: &ConstTpl) -> ConstTpl {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            ct.encode(&mut enc);
        }
        let mut dec = PackedDecode::new(manager);
        dec.ingest_stream(&buf).unwrap();
        let mut out = ConstTpl::new();
        out.decode(&mut dec).unwrap();
        let mut buf2 = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf2);
            out.encode(&mut enc);
        }
        assert_eq!(buf, buf2);
        out
    }

    #[test]
    fn semantics_consttpl_decode_roundtrip_all_variants() {
        let manager = test_manager();
        let variants = vec![
            ConstTpl::new_real(ConstType::Real, 0x1234),
            ConstTpl::new_real(ConstType::JRelative, 9),
            ConstTpl::new_handle(2, VField::VOffset),
            ConstTpl::new_handle_plus(1, VField::VOffsetPlus, 0x0002_0001),
            ConstTpl::new_type(ConstType::JStart),
            ConstTpl::new_type(ConstType::JNext),
            ConstTpl::new_type(ConstType::JNext2),
            ConstTpl::new_type(ConstType::JCurspace),
            ConstTpl::new_type(ConstType::JCurspaceSize),
            ConstTpl::new_type(ConstType::JFlowref),
            ConstTpl::new_type(ConstType::JFlowrefSize),
            ConstTpl::new_type(ConstType::JFlowdest),
            ConstTpl::new_type(ConstType::JFlowdestSize),
            ConstTpl::new_space(ram(&manager)),
        ];
        for v in &variants {
            let out = roundtrip_consttpl(&manager, v);
            assert_eq!(out.get_type(), v.get_type());
            assert_eq!(*v, out);
            match v.get_type() {
                ConstType::Real | ConstType::JRelative => {
                    assert_eq!(out.get_real(), v.get_real());
                }
                ConstType::Handle => {
                    assert_eq!(out.get_handle_index(), v.get_handle_index());
                    assert_eq!(out.get_select(), v.get_select());
                    if v.get_select() == VField::VOffsetPlus {
                        assert_eq!(out.get_real(), v.get_real());
                    }
                }
                ConstType::Spaceid => {
                    assert!(Rc::ptr_eq(&out.get_space(), &v.get_space()));
                }
                _ => {}
            }
        }
    }

    #[test]
    fn semantics_consttpl_decode_bad_selector() {
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_CONST_HANDLE);
            enc.write_signed_integer(&sla::ATTRIB_VAL, 0);
            enc.write_signed_integer(&sla::ATTRIB_S, 9);
            enc.close_element(&sla::ELEM_CONST_HANDLE);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let mut out = ConstTpl::new();
        let err = out.decode(&mut dec).unwrap_err();
        assert_eq!(err.explain(), "Bad handle selector encoding");
    }

    #[test]
    fn semantics_optpl_and_constructtpl_decode_roundtrip() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        ct.add_op(copy_op(&manager));
        let mut noout = OpTpl::new(OpCode::CPUI_STORE);
        noout.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 1),
            ConstTpl::new_real(ConstType::Real, 8),
        ));
        noout.add_input(VarnodeTpl::new_from_handle(0, false));
        noout.add_input(VarnodeTpl::new_from_handle(1, true));
        ct.add_op(noout);
        ct.set_result(Some(HandleTpl::new_from_varnode(
            &VarnodeTpl::new_from_handle(0, false),
        )));

        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            ct.encode(&mut enc, 3);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let mut out = ConstructTpl::new();
        let sectionid = out.decode(&mut dec).unwrap();
        assert_eq!(sectionid, 3);
        assert_eq!(out.get_opvec().len(), 2);
        assert_eq!(out.get_opvec()[0].get_opcode(), OpCode::CPUI_COPY);
        assert_eq!(out.get_opvec()[0].num_input(), 1);
        assert!(out.get_opvec()[0].get_out().is_some());
        assert_eq!(out.get_opvec()[1].get_opcode(), OpCode::CPUI_STORE);
        assert_eq!(out.get_opvec()[1].num_input(), 3);
        assert!(out.get_opvec()[1].get_out().is_none());
        assert!(out.get_result().is_some());

        // re-encode is byte-identical
        let mut buf2 = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf2);
            out.encode(&mut enc, 3);
        }
        assert_eq!(buf, buf2);
    }

    #[test]
    fn semantics_constructtpl_decode_null_result_and_attributes() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        // a labeled template with a delay slot, no result export
        let mut label = OpTpl::new(LABELBUILD);
        label.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        ct.add_op(label);
        let mut ds = OpTpl::new(DELAY_SLOT);
        ds.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, 1),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        ct.add_op(ds);

        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            ct.encode(&mut enc, -1);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let mut out = ConstructTpl::new();
        assert_eq!(out.decode(&mut dec).unwrap(), -1);
        assert_eq!(out.delay_slot(), 1);
        assert_eq!(out.num_labels(), 1);
        assert!(out.get_result().is_none());
    }

    // -- PcodeBuilder dispatch ----------------------------------------------------

    struct TestBuilder {
        labelbase: u32,
        labelcount: u32,
        calls: Vec<String>,
        /// (labelbase, labelcount) observed while inside build()
        inside: Vec<(u32, u32)>,
    }

    impl TestBuilder {
        fn new(lbcnt: u32) -> TestBuilder {
            TestBuilder {
                labelbase: lbcnt,
                labelcount: lbcnt,
                calls: Vec::new(),
                inside: Vec::new(),
            }
        }
    }

    impl PcodeBuilder for TestBuilder {
        fn get_label_base(&self) -> u32 {
            self.labelbase
        }
        fn set_label_base(&mut self, val: u32) {
            self.labelbase = val;
        }
        fn get_label_count(&self) -> u32 {
            self.labelcount
        }
        fn set_label_count(&mut self, val: u32) {
            self.labelcount = val;
        }
        fn dump(&mut self, op: &OpTpl) -> KunaResult<()> {
            self.calls.push(format!("dump:{:?}", op.get_opcode()));
            self.inside.push((self.labelbase, self.labelcount));
            Ok(())
        }
        fn append_build(&mut self, _bld: &OpTpl, secnum: i32) -> KunaResult<()> {
            self.calls.push(format!("build:{secnum}"));
            Ok(())
        }
        fn delay_slot(&mut self, _op: &OpTpl) -> KunaResult<()> {
            self.calls.push("delayslot".to_string());
            Ok(())
        }
        fn set_label(&mut self, _op: &OpTpl) -> KunaResult<()> {
            self.calls.push("setlabel".to_string());
            Ok(())
        }
        fn append_cross_build(&mut self, _bld: &OpTpl, secnum: i32) -> KunaResult<()> {
            self.calls.push(format!("crossbuild:{secnum}"));
            Ok(())
        }
    }

    #[test]
    fn semantics_pcodebuilder_build_dispatch_and_labels() {
        let manager = test_manager();
        let mut ct = ConstructTpl::new();
        let mkin = |val: u64| {
            VarnodeTpl::new(
                ConstTpl::new_space(cspace(&manager)),
                ConstTpl::new_real(ConstType::Real, val),
                ConstTpl::new_real(ConstType::Real, 4),
            )
        };
        let mut bld = OpTpl::new(BUILD);
        bld.add_input(mkin(0));
        ct.add_op(bld);
        let mut ds = OpTpl::new(DELAY_SLOT);
        ds.add_input(mkin(1));
        ct.add_op(ds);
        let mut lab = OpTpl::new(LABELBUILD);
        lab.add_input(mkin(0));
        ct.add_op(lab);
        let mut cb = OpTpl::new(CROSSBUILD);
        cb.add_input(mkin(0));
        ct.add_op(cb);
        ct.add_op(copy_op(&manager));

        let mut builder = TestBuilder::new(5);
        builder.build(Some(&ct), 2).unwrap();
        assert_eq!(
            builder.calls,
            vec![
                "build:2".to_string(),
                "delayslot".to_string(),
                "setlabel".to_string(),
                "crossbuild:2".to_string(),
                "dump:CPUI_COPY".to_string(),
            ]
        );
        // during the build: labelbase moved to old labelcount and labelcount
        // grew by the template's label count (1)
        assert_eq!(builder.inside, vec![(5, 6)]);
        // labelbase restored afterwards
        assert_eq!(builder.get_label_base(), 5);
        assert_eq!(builder.get_label_count(), 6);

        // null template is UnimplError
        let err = builder.build(None, 0).unwrap_err();
        assert!(matches!(err, KunaError::Unimpl { .. }));
    }
}
