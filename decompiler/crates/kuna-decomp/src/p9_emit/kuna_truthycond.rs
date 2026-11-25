//! (kuna) The truthy-condition presentation sub-stage — no upstream Ghidra
//! equivalent (DIV-36).
//!
//! In boolean contexts — an if/while/for/ternary condition, or an operand of
//! `&&`/`||`/`!` — a comparison against zero carries no information beyond the
//! value's own truthiness, and idiomatic C writes `if (expr)` / `if (!expr)`.
//! Upstream Ghidra always renders the explicit comparison (`if (x != 0)`,
//! `if (p == (char *)0x0)`).  kuna parameterizes the rendering:
//! `option truthycond on` elides the zero comparison in boolean contexts;
//! `off` restores the upstream explicit form; `ptrchain` (the default) is `on`
//! except that a bare pointer variable that is itself an operand of `&&`/`||`
//! keeps its comparison (`p != NULL && n`, `p == NULL || n`), unless the chain
//! is a loop's condition.
//!
//! The rendering itself lives in [`crate::printc`]: the
//! [`crate::printlanguage::modifiers::CONDITION_CONTEXT`] mod bit marks
//! boolean contexts (set at the CBRANCH condition push, `&&`/`||` operands,
//! and `!` operands; scoped off across any non-boolean-preserving operator by
//! `op_push_ir`), and `op_binary_ir` consumes it at an INT_EQUAL/INT_NOTEQUAL
//! whose one eligible zero operand is elided (`truthy_other_operand`).  Float
//! compares, enum-typed zeros, and equate-named zeros are excluded; value
//! contexts (`v = (x != 0)`, `return x != 0`, call arguments) never normalize.
//! [`crate::printlanguage::modifiers::CHAIN_OPERAND`] marks the expression that
//! is a direct operand of a printed `&&`/`||` outside a loop's condition
//! ([`crate::printlanguage::modifiers::LOOP_CONDITION`]), and
//! [`PrintC::bare_pointer_variable`] decides the `ptrchain` exception.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;

use crate::block::BlockType;
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::type_metatype::{TYPE_PTR, TYPE_PTRREL};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;
use crate::printlanguage::modifiers;

/// Marshaling element `<truthycond>` (kuna 4000+ range; 4113 = condfold,
/// 4114 retired — see kuna_condfold.rs).
pub const ELEM_TRUTHYCOND: ElementId = ElementId::new("truthycond", 4115);

/// The `truthycond` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthyCond {
    /// The upstream explicit comparison everywhere.
    Off,
    /// The truthy form in every boolean context.
    On,
    /// The truthy form, except a bare pointer variable operand of `&&`/`||`.
    PtrChain,
}

/// (kuna) Truthy condition rendering: `truthycond on|off|ptrchain` (DIV-36).
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionTruthyCond;

impl OptionTruthyCond {
    /// The option name.
    pub const NAME: &'static str = "truthycond";

    /// Parse the value; the caller performs the printer write
    /// ([`crate::printc::PrintCOptions::set_truthy_mode`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(TruthyCond, String)> {
        let (mode, msg) = match p1 {
            "on" | "" => (TruthyCond::On, "turned on"),
            "off" => (TruthyCond::Off, "turned off"),
            "ptrchain" => (TruthyCond::PtrChain, "turned on, keeping pointer tests in && / || chains"),
            _ => return Err(KunaError::parse(format!("Unknown truthycond value: {p1} (on|off|ptrchain)"))),
        };
        Ok((mode, format!("Truthy condition rendering {msg}")))
    }
}

impl PrintC {
    /// Mark the clause `blk` of a printed `&&`/`||` chain as a chain operand: a
    /// basic block's branch or a nested chain (which marks its own clauses).  A
    /// loop's condition marks nothing.
    pub(crate) fn mark_chain_operand(&mut self, fd: &Funcdata, blk: BlockId) {
        if !self.context.is_set(modifiers::LOOP_CONDITION)
            && matches!(fd.sblocks_ref().block(blk).get_type(), BlockType::Copy | BlockType::Condition)
        {
            self.context.set_mod(modifiers::CHAIN_OPERAND);
        } else {
            self.context.unset_mod(modifiers::CHAIN_OPERAND);
        }
    }

    /// Whether `vn`, read by `op`, prints as a plain variable name whose
    /// declared type is a pointer: not an expression, constant, partial or
    /// field access, address-of, or a global.
    pub(crate) fn bare_pointer_variable(&self, fd: &Funcdata, vn: VarnodeId, op: OpId) -> bool {
        let vn = self.loop_tests.leaf(vn);
        let Some(v) = fd.vbank().get(vn) else { return false };
        if v.is_implied() || v.is_constant() || v.is_annotation() || v.is_persist() {
            return false;
        }
        let Some((_, sym_off, sym_type)) = v.get_high().and_then(|high| self.emitted_high_symbol(fd, high)) else {
            return false;
        };
        if self.is_value_indirect_input(fd, vn) {
            return false;
        }
        if let Some(st) = sym_type {
            if sym_off > 0 || st.get_size() != v.get_size() || !matches!(st.get_metatype(), TYPE_PTR | TYPE_PTRREL) {
                return false;
            }
        }
        let ty = self.printed_operand_type(fd, vn, op);
        ty.get_size() == v.get_size() && matches!(ty.get_metatype(), TYPE_PTR | TYPE_PTRREL)
    }
}

#[cfg(test)]
#[path = "kuna_truthycond/tests.rs"]
mod tests;
