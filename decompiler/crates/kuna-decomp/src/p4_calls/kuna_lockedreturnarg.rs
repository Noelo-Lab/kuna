//! (kuna) A call whose return storage is stated while its inputs are still
//! recovered keeps its input trial at that storage.
//!
//! Upstream `Heritage::guardCalls` skips a call whose own output covers the
//! heritaged range exactly, input trial included. That is harmless when the
//! output is recovered, because the trial is planted before any output exists,
//! and when the whole prototype is locked, because no input is recovered. An
//! output-only declaration (`return f::x0 T`) locks the output and leaves the
//! inputs open, so on a target whose return register is also an argument
//! register (AArch64 `x0`, ARM `r0`, PowerPC `r3`) the trial was never planted:
//! the copy into the register before the call had no reader and was removed,
//! and the trial a later pass added read an older value, so `f(a1,a0)` printed
//! as `f(0,a0)`. The trial is planted as it is without the declaration, only
//! where the model takes the range as an input. A variadic call keeps the rules
//! of [`crate::kuna_varargretreg::argument_in_own_output`].

use crate::fspec::FuncCallSpecs;

/// Does the non-variadic call `fc` state its output while recovering its inputs?
pub fn argument_in_locked_output(fc: &FuncCallSpecs) -> bool {
    !fc.is_dotdotdot() && fc.proto().is_output_locked()
}
