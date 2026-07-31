//! (kuna) The single-statement if-body brace-elision presentation sub-stage —
//! no upstream Ghidra equivalent (DIV-37).
//!
//! Idiomatic C often writes a single-statement if-body without braces:
//!
//! ```c
//! if (expr)
//!   stmt;
//! ```
//!
//! Upstream Ghidra always braces (`if (expr) { stmt; }`).  kuna parameterizes
//! the rendering: `option braceelide on` (the default) drops the braces when
//! the then-body is a plain single-statement `BlockCopy` leaf; `off` restores
//! the upstream braced form.
//!
//! The rendering lives in [`crate::printc`]: `emit_block_if` consults
//! `PrintC::if_body_elides` — the body must be a Copy leaf (so it can never be
//! a nested `if`, ruling out the dangling-else hazard), with exactly one op
//! that `emit_basic_block_ops` would print under NO_BRANCH, no label line, and
//! no comment positioned in the block.  The `if (cond) goto L;` one-liner and
//! the `else if` collapse are separate, pre-existing forms and stay untouched;
//! else arms and loop/switch bodies always keep their braces.
//!
//! Following the kuna-option idiom ([`crate::kuna_arraynotation::OptionArrayNotation`]),
//! this module owns only the option struct that flips
//! [`crate::printc::PrintCOptions::brace_elide`]; the caller
//! (`Architecture::set_kuna_option`) writes the live flag.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::options::on_or_off;

/// Marshaling element `<braceelide>` (kuna 4000+ range; 4115 = truthycond).
pub const ELEM_BRACEELIDE: ElementId = ElementId::new("braceelide", 4116);

/// (kuna) Toggle single-statement if-body brace elision: `braceelide on|off`
/// (DIV-37).
///
/// "off" keeps the upstream braced form (`if (expr) { stmt; }`); "on" (the
/// kuna default) renders the statement indented on the next line, braceless.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionBraceElide;

impl OptionBraceElide {
    /// The option name.
    pub const NAME: &'static str = "braceelide";

    /// Parse + validate the `on`/`off` value; the caller performs the printer
    /// write ([`crate::printc::PrintCOptions::set_brace_elide`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Single-statement if-body brace elision turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_braceelide/tests.rs"]
mod tests;
