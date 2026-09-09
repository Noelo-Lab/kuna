//! **decodehalt** — a flow terminator kuna planted because it could not decode
//! the bytes says so, instead of printing as an ordinary `return;`.
//!
//! `FlowInfo` ends a path it cannot follow by planting an *artificial halt*:
//! a `CPUI_RETURN` carrying a constant `1` and one of the `getHaltType()` bits
//! (`artificial_halt`, C++ `flow.cc:483`).  Three of those bits mean the decode
//! failed rather than that the program returns —
//!
//! * `badinstruction` — the bytes at the address are not an instruction;
//! * `unimplemented` — SLEIGH decoded them but has no semantics for them;
//! * `missing` — the address was never decoded at all (a branch that left a
//!   caller-declared extent, `fillin_branch_stubs`).
//!
//! Upstream's `PrintC::opReturn` (printc.cc:774) renders those three as the
//! pseudo-calls `halt_baddata()` / `halt_unimplemented()` / `halt_missing()`,
//! and `FlowInfo::processInstruction` buffers a warning at the truncation point
//! plus a header warning on the function.  kuna ported the halt but neither the
//! rendering nor the warnings, so a function that ran off into bytes kuna could
//! not read ended with a bare `return;`, `error: null` in `--json`, and nothing
//! anywhere saying the body is incomplete — the reading an RE agent is least
//! able to recover from, because it is indistinguishable from a function that
//! really does return.
//!
//! The self-decrypting stub is the shape that motivated this: a packer XORs its
//! payload in place and jumps into it, so the bytes at the target are ciphertext
//! at load time and no decoder can read them.  kuna resolved the jump, decoded
//! ciphertext until it hit a byte that is not an instruction, and printed
//! `case 0x40190e: return;`.
//!
//! `noreturn` — the fourth halt bit, planted after a call that does not come
//! back — is deliberately NOT renamed.  It is not a decode failure, kuna already
//! reports it (`// no-return` on the call), and its rendering is pinned by the
//! whole-corpus measurements behind `noreturn_propagate` and friends.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;
use kuna_base::types::uint4;

/// Marshaling element `<decodehalt>` (kuna 4000+ range; 4159 = calleescratchbody).
pub const ELEM_DECODEHALT: ElementId = ElementId::new("decodehalt", 4160);

/// C++ `FlowInfo::processInstruction`'s bad-data truncation warning
/// (`flow.cc:466`), buffered at the address that failed to decode.
pub const BAD_INSTRUCTION_WARNING: &str = "Bad instruction - Truncating control flow here";

/// C++ `FlowInfo::processInstruction`'s bad-data header warning (`flow.cc:469`).
pub const BAD_INSTRUCTION_HEADER: &str = "Control flow encountered bad instruction data";

/// C++ `FlowInfo::processInstruction`'s unimplemented truncation warning
/// (`flow.cc:455`).
pub const UNIMPLEMENTED_WARNING: &str =
    "Unimplemented instruction - Truncating control flow here";

/// C++ `FlowInfo::processInstruction`'s unimplemented header warning
/// (`flow.cc:458`).
pub const UNIMPLEMENTED_HEADER: &str = "Control flow encountered unimplemented instructions";

/// C++ `FlowInfo::processInstruction`'s header warning for the `ignoreunimplemented`
/// policy, where the instruction is stepped over rather than halted (`flow.cc:447`).
pub const UNIMPLEMENTED_IGNORED_HEADER: &str = "Control flow ignored unimplemented instructions";

/// The pseudo-call a decode-failure halt renders as, or `None` when the halt
/// type is not a decode failure (`noreturn`, `halt`, or no halt at all).
///
/// Transcribes the three decode-failure arms of C++ `PrintC::opReturn`'s
/// `getHaltType()` switch (printc.cc:774-796).
pub fn halt_call_name(halt_type: uint4) -> Option<&'static str> {
    use crate::op::pcodeop_flags as f;
    if (halt_type & f::badinstruction) != 0 {
        Some("halt_baddata")
    } else if (halt_type & f::unimplemented) != 0 {
        Some("halt_unimplemented")
    } else if (halt_type & f::missing) != 0 {
        Some("halt_missing")
    } else {
        None
    }
}

/// (kuna) Choose whether a decode-failure halt reports itself: `decodehalt on|off`.
///
/// Following the kuna-option idiom
/// ([`OptionArrayNotation`](crate::kuna_arraynotation::OptionArrayNotation)),
/// this struct owns only the parse; the caller
/// (`Architecture::set_kuna_option`) writes the live field.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionDecodeHalt;

impl OptionDecodeHalt {
    /// The option name.
    pub const NAME: &'static str = "decodehalt";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = match p1 {
            "on" => true,
            "off" => false,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown decodehalt value: {other} (expected on|off)"
                )))
            }
        };
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Decode-failure halt reporting turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_decodehalt/tests.rs"]
mod tests;
