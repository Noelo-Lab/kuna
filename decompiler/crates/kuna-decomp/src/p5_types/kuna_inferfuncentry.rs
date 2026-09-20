//! Port of `decompiler/cpp/kuna_inferfuncentry.{cc,hh}` — function-entry
//! constant-pointer inference (kuna GH-6930, S5 const-pointer).
//!
//! `ActionConstantPtr::isPointer` rejects any constant whose bit pattern "looks
//! like a single bit or mask" (`bit_transitions < 3`) before it consults the
//! symbol table, to protect integer flags.  A function placed exactly at a
//! power-of-two image base (e.g. `0x100000 == 1<<20`) is a single-bit value, so
//! its address is discarded.  When `option inferfuncentry on`, the single-bit
//! rejection is skipped for a constant that resolves *exactly* to a known
//! function entry — narrowly scoped so legitimate single-bit flags are
//! unaffected.
//!
//! ## What this module ports
//!
//! [`kuna_is_function_entry`] is a faithful free-function transcription of the
//! C++ `kunaIsFunctionEntry(Funcdata&, AddrSpace*, const Varnode*, const
//! PcodeOp*)` predicate.  The `isPointer` integration (skipping the
//! `bit_transitions` rejection when this returns `true`) is W4/W5
//! (`ruleaction`/`coreaction`); this module owns only the *decision*.
//!
//! ## STUB(W4): Action + ArchOption wrappers
//!
//! The two architecture-side resolutions the C++ reaches through `glb` are taken
//! here as already-resolved facts:
//!
//!   - the **gate** `glb->infer_funcentry` (default \b true, shipped
//!     `option inferfuncentry on` — `architecture.cc:1430`,
//!     `kuna_stages.cc` settableTable): modelled by [`InferFuncEntryOption`],
//!     whose `default()` is the shipped on-by-default flag.
//!   - **`glb->resolveConstant(spc, off, size, op->getAddr(), fullEncoding)`**
//!     (the loader/symbol resolution, W4): supplied as `rampoint`
//!     (`Option<Address>`, `None` == the C++ invalid `rampoint`).
//!   - **`scope->queryFunction(rampoint)->getAddress()`** (the symbol-table
//!     query, W4): supplied as `function_entry` — the queried function's entry
//!     address (`None` == no function at `rampoint`).

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

/// (kuna) Toggle inference of function entries at single-bit image bases
/// (C++ `OptionInferFuncEntry`, GH-6930).
///
/// STUB(W4): the C++ `OptionInferFuncEntry::apply` flips `glb->infer_funcentry`;
/// here the flag is carried as a plain `bool` whose [`Default`] is the *shipped*
/// default (`option inferfuncentry on`, i.e. \b true — DIV-2 default-on;
/// `architecture.cc:1430`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InferFuncEntryOption {
    /// True if single-bit constants matching an exact function entry are
    /// inferred as pointers (C++ `Architecture::infer_funcentry`).
    pub enabled: bool,
}

impl Default for InferFuncEntryOption {
    /// Shipped default: `option inferfuncentry on` (DIV-2 default-on;
    /// `architecture.cc:1430` sets `infer_funcentry = true`).
    fn default() -> Self {
        InferFuncEntryOption { enabled: true }
    }
}

impl InferFuncEntryOption {
    /// (kuna) Set the gate (C++ `OptionInferFuncEntry::apply`: `bool val =
    /// onOrOff(p1); glb->infer_funcentry = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Function-entry constant-pointer inference turned on"
        } else {
            "Function-entry constant-pointer inference turned off"
        }
    }

    /// Read the gate (C++ `glb->infer_funcentry`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// (kuna) May the single-bit escape be consulted for a constant read by `opcode`?
///
/// The escape suspends the one guard (`bit_transitions < 3`) that keeps integer
/// flags, masks and round sizes out of the symbol table, so it is only safe where
/// a raw code address is what the operand means: handed to a callee (GH-6930's
/// `bufferevent_setcb(..., evhttp_write_cb, ...)`), written into a function-pointer
/// slot, compared for identity with one, or added to a value already typed as a
/// pointer.
///
/// Everywhere else the constant is being used as a number and the entry it lands
/// on is a coincidence, which is what a round buffer size looks like in a PIE:
/// coreutils `tail -O0` computes `MIN (n_remaining, BUFSIZ)`, `BUFSIZ` is `0x2000`
/// and `0x2000` is `_DT_INIT`, so the bound printed as `if (_DT_INIT < v9)` and
/// dragged the `uintmax_t` byte counter it was compared with to `void *` — in the
/// callee, and from there through `protoorder` into the caller's stack variable.
pub fn entry_escape_applies(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_STORE
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
    )
}

/// (kuna) Does the constant resolve exactly to a known function entry?
/// (C++ `kunaIsFunctionEntry`, GH-6930).
///
/// C++ `kuna_inferfuncentry.cc:12-26`.
///
/// `gate` is the resolved `glb->infer_funcentry` (see [`InferFuncEntryOption`]).
/// `rampoint` is the resolved `glb->resolveConstant(...)`: `None` transcribes
/// the C++ invalid address (`rampoint.isInvalid()`).  // STUB(W4)
/// `function_entry` is the entry address of the function `queryFunction(rampoint)`
/// returns, or `None` if no function is at `rampoint`.  // STUB(W4)
///
/// Returns `true` only for an *exact* function-entry match — the C++
/// `fd->getAddress() == rampoint` — so an ordinary single-bit integer constant
/// (no function at its resolved address) is never reclassified.
pub fn kuna_is_function_entry(
    gate: bool,
    rampoint: Option<&Address>,
    function_entry: Option<&Address>,
) -> bool {
    if !gate {
        return false;
    }
    let rampoint = match rampoint {
        // C++ `rampoint.isInvalid()` is true for the null/sentinel address; a
        // `Some` carrying an invalid Address must also be rejected.
        Some(addr) if !addr.is_invalid() => addr,
        _ => return false,
    };
    match function_entry {
        Some(entry) => entry == rampoint,
        None => false,
    }
}

#[cfg(test)]
#[path = "kuna_inferfuncentry/tests.rs"]
mod tests;
