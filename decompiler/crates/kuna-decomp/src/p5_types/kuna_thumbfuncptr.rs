//! Port of `decompiler/cpp/kuna_thumbfuncptr.{cc,hh}` — Thumb function-pointer
//! preservation (kuna GH-8471, S5 const-pointer).
//!
//! A pointer to a Thumb function carries the mode bit in its LSB
//! (`value = fn|1`).  `ActionConstantPtr` recovers `PTRSUB(fn) + 1`, but
//! `RulePtrsubUndo` then destroys it (the `+1` does not match the bounds of
//! `fn`'s code type), collapsing the constant back to a raw hex literal.  When
//! `option thumbfuncptr on`, `RulePtrsubUndo` preserves a PTRSUB whose base
//! resolves to a code-typed symbol when the leftover offset fits inside the
//! architecture's function-pointer alignment-encoding bits (`funcptr_align`).
//!
//! ## What this module ports
//!
//! [`kuna_preserve_thumb_funcptr`] is a faithful free-function transcription of
//! the C++ `kunaPreserveThumbFuncPtr(Varnode*, PcodeOp*, int8 val, int8 extra,
//! int8 multiplier, Architecture*)` predicate.  The `RulePtrsubUndo` integration
//! (preserving the PTRSUB when this returns `true`) is W5 (`ruleaction`); this
//! module owns only the *decision*.
//!
//! ## STUB(W4)/STUB(W6): Rule + ArchOption + type-system wrappers
//!
//! The C++ predicate threads through the W6 `Datatype`/`TypePointer` subsystem
//! (`getTypeReadFacing`, `getPtrTo`, `getWordSize`, `getSubType`) and two W4
//! `Architecture` fields (`preserve_thumb_funcptr`, `funcptr_align`).  The W3
//! `Datatype` skeleton (`dtype.rs`) carries only `getMetatype`/`getSize`, so the
//! type-system resolution is taken here as an already-resolved fact struct
//! [`ThumbPtrTypeFacts`] (each field `// STUB(W6)`), and the two arch fields are
//! taken as resolved inputs (the gate via [`ThumbFuncPtrOption`], `funcptr_align`
//! as a parameter).  W6 fills the type walk; W4 wires the arch fields.

use kuna_base::space::AddrSpace;
use kuna_base::types::int8;

use crate::dtype::type_metatype;

/// (kuna) Toggle preservation of mode-bit-encoded (Thumb) function pointers
/// (C++ `OptionThumbFuncPtr`, GH-8471).
///
/// STUB(W4): the C++ `OptionThumbFuncPtr::apply` flips
/// `glb->preserve_thumb_funcptr`; here the flag is carried as a plain `bool`
/// whose [`Default`] is the *shipped* default (`option thumbfuncptr on`, i.e.
/// \b true — DIV-2 default-on; `architecture.cc:1451`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThumbFuncPtrOption {
    /// True if mode-bit-encoded (Thumb) function pointers are kept symbolic
    /// (C++ `Architecture::preserve_thumb_funcptr`).
    pub enabled: bool,
}

impl Default for ThumbFuncPtrOption {
    /// Shipped default: `option thumbfuncptr on` (DIV-2 default-on;
    /// `architecture.cc:1451` sets `preserve_thumb_funcptr = true`).
    fn default() -> Self {
        ThumbFuncPtrOption { enabled: true }
    }
}

impl ThumbFuncPtrOption {
    /// (kuna) Set the gate (C++ `OptionThumbFuncPtr::apply`: `bool val =
    /// onOrOff(p1); glb->preserve_thumb_funcptr = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Thumb function-pointer preservation turned on"
        } else {
            "Thumb function-pointer preservation turned off"
        }
    }

    /// Read the gate (C++ `glb->preserve_thumb_funcptr`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Resolved W6 type-system facts about `basevn->getTypeReadFacing(op)` for
/// [`kuna_preserve_thumb_funcptr`].
///
/// STUB(W6): the C++ predicate reaches the `Datatype`/`TypePointer` subsystem
/// (`type.{hh,cc}`).  The W3 `Datatype` skeleton has no `TypePointer`,
/// `getPtrTo`, `getWordSize`, or `getSubType`, so the four query results the C++
/// extracts are carried here as resolved facts; W6 produces them from the real
/// type walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThumbPtrTypeFacts {
    /// `bt->getMetatype()` where `bt = basevn->getTypeReadFacing(op)`.
    /// // STUB(W6)
    pub base_metatype: type_metatype,
    /// `pt->getMetatype()` where `pt = ((TypePointer*)bt)->getPtrTo()` — valid
    /// only when `base_metatype == TYPE_PTR`.  // STUB(W6)
    pub ptr_to_metatype: type_metatype,
    /// `((TypePointer*)bt)->getWordSize()` — the pointer's word size, fed to
    /// `AddrSpace::addressToByteInt`.  // STUB(W6)
    pub word_size: u32,
    /// The result of `pt->getSubType(newoff, &newoff)` for the byte-offset
    /// `newoff = addressToByteInt(val, word_size)`: the looked-up sub-datatype's
    /// metatype and the residual `newoff`.  `None` transcribes a null `sub`
    /// (no sub-datatype at that offset).  Valid only when
    /// `ptr_to_metatype == TYPE_SPACEBASE`.  // STUB(W6)
    pub sub: Option<ThumbSubTypeFact>,
}

/// The resolved `pt->getSubType(newoff, &newoff)` result (STUB(W6)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThumbSubTypeFact {
    /// `sub->getMetatype()` — the symbol's data-type class (the C++ requires
    /// `TYPE_CODE`).  // STUB(W6)
    pub metatype: type_metatype,
    /// The residual offset `newoff` after the sub-type look-up (the C++ requires
    /// it be exactly 0 — `val` lands precisely on the symbol).  // STUB(W6)
    pub residual_offset: int8,
}

/// (kuna) Should `RulePtrsubUndo` preserve this PTRSUB as a mode-bit-encoded
/// function pointer? (C++ `kunaPreserveThumbFuncPtr`, GH-8471).
///
/// C++ `kuna_thumbfuncptr.cc:12-36`.
///
/// `gate` is the resolved `glb->preserve_thumb_funcptr` (see
/// [`ThumbFuncPtrOption`]); `funcptr_align` is `glb->funcptr_align`.  `facts`
/// carries the resolved W6 type-system queries (see [`ThumbPtrTypeFacts`]).
/// `val`, `extra`, `multiplier` are the PTRSUB constant decomposition exactly
/// as the C++ takes them.
pub fn kuna_preserve_thumb_funcptr(
    gate: bool,
    funcptr_align: i32,
    val: int8,
    extra: int8,
    multiplier: int8,
    facts: &ThumbPtrTypeFacts,
) -> bool {
    if !gate || funcptr_align == 0 {
        return false;
    }
    // Multiplier 1 = byte-grain PTRADD form of the same constant offset.
    if multiplier > 1 {
        return false;
    }
    if facts.base_metatype != type_metatype::TYPE_PTR {
        return false;
    }
    if facts.ptr_to_metatype != type_metatype::TYPE_SPACEBASE {
        return false;
    }
    // (newoff is recomputed inside the resolved getSubType fact; the conversion
    //  is recorded here to keep the byte-scaling visible against the C++.)
    let _newoff_in: int8 = AddrSpace::address_to_byte_int(val, facts.word_size);
    let sub = match facts.sub {
        Some(s) => s,
        None => return false, // sub == 0
    };
    if sub.residual_offset != 0 {
        // newoff != 0: val did not land exactly on the symbol.
        return false;
    }
    if sub.metatype != type_metatype::TYPE_CODE {
        return false;
    }
    let bextra: int8 = AddrSpace::address_to_byte_int(extra, facts.word_size);
    // The offset must be exactly the alignment-encoding bits (the Thumb LSB).
    // `funcptr_align` is an `int4` (the C++ shift count); `as u32` is the C++
    // implicit shift-count promotion — it is a small alignment bit-count
    // (set from `bits` in `architecture.cc:setFuncPtrAlign`), never negative
    // here (the `== 0` case returned above) nor >= 64.
    if bextra <= 0 || bextra >= (1i64 << (funcptr_align as u32)) {
        return false;
    }
    true
}

#[cfg(test)]
#[path = "kuna_thumbfuncptr/tests.rs"]
mod tests;
