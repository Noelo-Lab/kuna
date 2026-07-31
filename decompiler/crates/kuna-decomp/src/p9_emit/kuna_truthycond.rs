//! (kuna) The truthy-condition presentation sub-stage — no upstream Ghidra
//! equivalent (DIV-36).
//!
//! In boolean contexts — an if/while/for/ternary condition, or an operand of
//! `&&`/`||`/`!` — a comparison against zero carries no information beyond the
//! value's own truthiness, and idiomatic C writes `if (expr)` / `if (!expr)`.
//! Upstream Ghidra always renders the explicit comparison (`if (x != 0)`,
//! `if (p == (char *)0x0)`).  kuna parameterizes the rendering:
//! `option truthycond on` (the default) elides the zero comparison in boolean
//! contexts; `off` restores the upstream explicit form.
//!
//! The rendering itself lives in [`crate::printc`]: the
//! [`crate::printlanguage::modifiers::CONDITION_CONTEXT`] mod bit marks
//! boolean contexts (set at the CBRANCH condition push, `&&`/`||` operands,
//! and `!` operands; scoped off across any non-boolean-preserving operator by
//! `op_push_ir`), and `op_binary_ir` consumes it at an INT_EQUAL/INT_NOTEQUAL
//! whose one eligible zero operand is elided (`truthy_other_operand`).  Float
//! compares, enum-typed zeros, and equate-named zeros are excluded; value
//! contexts (`v = (x != 0)`, `return x != 0`, call arguments) never normalize.
//!
//! Following the kuna-option idiom ([`crate::kuna_arraynotation::OptionArrayNotation`]),
//! this module owns only the option struct that flips the
//! [`crate::printc::PrintCOptions::truthy_cond`] toggle; the caller
//! (`Architecture::set_kuna_option`) writes the live flag.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::options::on_or_off;

/// Marshaling element `<truthycond>` (kuna 4000+ range; 4113 = condfold,
/// 4114 retired — see kuna_condfold.rs).
pub const ELEM_TRUTHYCOND: ElementId = ElementId::new("truthycond", 4115);

/// (kuna) Toggle truthy condition rendering: `truthycond on|off` (DIV-36).
///
/// "off" keeps the upstream explicit comparison (`if (x != 0)` /
/// `if (p == NULL)`); "on" (the kuna default) renders the truthy form
/// (`if (x)` / `if (!p)`) in boolean contexts.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionTruthyCond;

impl OptionTruthyCond {
    /// The option name.
    pub const NAME: &'static str = "truthycond";

    /// Parse + validate the `on`/`off` value; the caller performs the printer
    /// write ([`crate::printc::PrintCOptions::set_truthy_cond`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Truthy condition rendering turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_truthycond/tests.rs"]
mod tests;
