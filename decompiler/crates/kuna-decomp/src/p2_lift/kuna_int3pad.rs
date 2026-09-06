//! **int3pad** — control that runs into an `int3` pad is named, instead of
//! printing as a call through a pseudo-op.
//!
//! ```text
//!   v1 = (void *)swi(3);                      v1 = (void *)swi(3); // int3-pad x4
//!   return (*v1)();                     -->   return (*v1)();
//! ```
//!
//! # The defect
//!
//! x86 SLEIGH lowers `INT3` to a black-box userop feeding an indirect call
//! (`ia.sinc:3670`):
//!
//! ```text
//!   tmp:1 = 0x3;
//!   intloc:$(SIZE) = swi(tmp);
//!   call [intloc];
//!   return [0:1];
//! ```
//!
//! That is honest lifting — SLEIGH cannot know what sits behind the vector — but
//! the printer renders it as `(*(void *)swi(3))()`, which reads as an ordinary
//! call through a function pointer. Nothing in the C says that the byte at that
//! address is `0xcc`, that it is one of a run of them, or that control got there
//! by falling off the end of the decoded code. On an anti-debug binary the
//! clean-run path frequently ends in exactly that pad, and the only way to see it
//! today is to open the disassembly.
//!
//! # The fix
//!
//! [`is_int3_callind`] matches the lowered shape — a `CALLIND` that reads the
//! storage a `swi` `CALLOTHER` wrote in the same instruction, from the one-byte
//! constant vector `3` — and [`pad_run_length`] counts the `0xcc` bytes at the
//! site. `flow.rs::setup_callind_specs` buffers the
//! [`warning_text`] the P9 slug table renders as `// int3-pad xN`
//! ([`warning_slug`](crate::printc::warning_slug)).
//!
//! Under `option int3pad halt` the call spec is additionally marked no-return and
//! the artificial halt `check_for_flow_modification` plants for a named no-return
//! callee is planted here too, so the pad ends the flow rather than contributing
//! the fabricated return value of `return (*v1)();`. That is a judgement about the
//! pad rather than a fact about the encoding — a debugger attached to the process
//! *can* step past an `int3` — so it is not the default.
//!
//! # What it will not do
//!
//! * **Only vector 3.** `int 0x29` is `__fastfail` and belongs to
//!   [`fastfailnoreturn`](crate::kuna_fastfailnoreturn); `int 0x80` is a Linux
//!   syscall and belongs to [`linuxsyscall`](crate::kuna_linuxsyscall).
//! * **Only the exact lifted shape**, shared with `fastfailnoreturn`: the
//!   `CALLIND` must read the storage the `swi` `CALLOTHER` in the same instruction
//!   wrote. Hexagon is the only other vendored processor with a `swi` user-op,
//!   and its takes a register and produces no output, so it can feed no `CALLIND`
//!   and no language gate is needed.
//! * **No claim about the run when the image cannot be read.** The pad length is
//!   a loader read; when it fails, or when the byte is not `0xcc` (the two-byte
//!   `INT 0x03` encoding), the warning is emitted without a count rather than
//!   with a guessed one.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::ElementId;
use kuna_base::types::uintb;

/// Marshaling element `<int3pad>` (kuna).  ElementIds live in the 4000+ range;
/// 4149 is the last one taken, so this is 4150.
pub const ELEM_INT3PAD: ElementId = ElementId::new("int3pad", 4150);

/// The breakpoint interrupt vector `INT3` lowers through.
pub const INT3_VECTOR: uintb = 3;

/// The `INT3` encoding, and so the byte a pad is made of.
pub const INT3_BYTE: u64 = 0xcc;

/// Ceiling on the pad walk: the count is presentation, and a real pad is a
/// handful of bytes.
pub const MAX_PAD_RUN: u32 = 256;

/// (kuna) `int3pad off|warn|halt` — how a decoded `int3` is handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Int3PadMode {
    /// Upstream: the pad prints as an indirect call and nothing says so.
    Off,
    /// Buffer the `// int3-pad` warning, leaving the rendering alone (the default).
    #[default]
    Warn,
    /// Warn, and additionally end the flow at the pad.
    Halt,
}

impl Int3PadMode {
    /// Is a warning buffered at a matched site?
    pub fn warns(self) -> bool {
        !matches!(self, Int3PadMode::Off)
    }

    /// Is the flow ended at a matched site?
    pub fn halts(self) -> bool {
        matches!(self, Int3PadMode::Halt)
    }
}

/// A matched `int3` site, as the flow classifier needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Int3PadSite {
    /// The number of `0xcc` bytes at the site, or 0 when the image cannot say.
    pub run: u32,
    /// Whether the flow ends here (`option int3pad halt`).
    pub halt: bool,
}

/// (kuna) Choose the `int3` pad handling: `int3pad off|warn|halt`.
///
/// Following the kuna-option idiom
/// ([`OptionArrayNotation`](crate::kuna_arraynotation::OptionArrayNotation)),
/// this struct owns only the parse; the caller
/// (`Architecture::set_kuna_option`) writes the live field.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionInt3Pad;

impl OptionInt3Pad {
    /// The option name.
    pub const NAME: &'static str = "int3pad";

    /// Parse + validate the `off`/`warn`/`halt` value.
    pub fn apply(&self, p1: &str) -> KunaResult<(Int3PadMode, String)> {
        let mode = match p1 {
            "off" => Int3PadMode::Off,
            "warn" => Int3PadMode::Warn,
            "halt" => Int3PadMode::Halt,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown int3pad value: {other} (expected off|warn|halt)"
                )))
            }
        };
        Ok((mode, format!("int3 pad handling set to {p1}")))
    }
}

/// Is `op` the `CALLIND` half of a decoded `int3`?
///
/// The lowered shape is `fastfailnoreturn`'s — one `swi` `CALLOTHER` feeding a
/// `CALLIND` in the same instruction — read for the vector rather than tested
/// against `0x29`; see
/// [`swi_vector_of_callind`](crate::kuna_fastfailnoreturn::swi_vector_of_callind).
pub fn is_int3_callind<F>(data: &Funcdata, op: OpId, userop_name: F) -> bool
where
    F: Fn(u32) -> Option<String>,
{
    crate::kuna_fastfailnoreturn::swi_vector_of_callind(data, op, userop_name)
        == Some(INT3_VECTOR)
}

/// How many consecutive `0xcc` bytes the image holds at `addr`, capped at
/// [`MAX_PAD_RUN`].
///
/// Zero means the question could not be answered: either the byte is not `0xcc`
/// (the two-byte `INT 0x03` encoding lowers through the same vector) or the read
/// left the loaded image.  The reads are one byte at a time because the loader
/// fills a full word and refuses a short one at a section end, which is exactly
/// where a pad tends to sit.
pub fn pad_run_length(data: &Funcdata, addr: &Address) -> u32 {
    let Some(space) = addr.get_space() else { return 0 };
    let mut run = 0u32;
    while run < MAX_PAD_RUN {
        let at = Address::new(
            std::rc::Rc::clone(space),
            addr.get_offset().wrapping_add(u64::from(run)),
        );
        match data.get_arch().get_load_image_value(&at, 1) {
            Ok(b) if b == INT3_BYTE => run += 1,
            _ => break,
        }
    }
    run
}

/// The warning text buffered at a matched site, with `run` from
/// [`pad_run_length`].
///
/// The body deliberately carries no digit before the count: the P9 slug table
/// reads the first integer in a producer-tagged warning as its ` xN` suffix.
pub fn warning_text(run: u32) -> String {
    if run >= 2 {
        format!("int3pad: flow enters a debug-trap pad of {run} bytes")
    } else {
        "int3pad: flow enters a debug trap".to_string()
    }
}

#[cfg(test)]
#[path = "kuna_int3pad/tests.rs"]
mod tests;
