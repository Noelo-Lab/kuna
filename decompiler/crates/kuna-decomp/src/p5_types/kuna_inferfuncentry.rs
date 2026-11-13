//! Port of `decompiler/cpp/kuna_inferfuncentry.{cc,hh}` — function-entry
//! constant-pointer inference (kuna GH-6930, S5 const-pointer).
//!
//! `ActionConstantPtr::isPointer` rejects any constant whose bit pattern "looks
//! like a single bit or mask" (`bit_transitions < 3`) before it consults the
//! symbol table, to protect integer flags.  A function placed exactly at a
//! power-of-two image base (e.g. `0x100000 == 1<<20`) is a single-bit value, so
//! its address is discarded.  The single-bit rejection is skipped for a
//! constant that resolves *exactly* to a known function entry — narrowly scoped
//! so legitimate single-bit flags are unaffected.
//!
//! ## What this module ports
//!
//! [`kuna_is_function_entry`] is a faithful free-function transcription of the
//! C++ `kunaIsFunctionEntry(Funcdata&, AddrSpace*, const Varnode*, const
//! PcodeOp*)` predicate.  The `isPointer` integration (skipping the
//! `bit_transitions` rejection when this returns `true`) is W4/W5
//! (`ruleaction`/`coreaction`); this module owns only the *decision*.
//!
//! ## STUB(W4): Action wrappers
//!
//! The two architecture-side resolutions the C++ reaches through `glb` are taken
//! here as already-resolved facts:
//!
//!   - **`glb->resolveConstant(spc, off, size, op->getAddr(), fullEncoding)`**
//!     (the loader/symbol resolution, W4): supplied as `rampoint`
//!     (`Option<Address>`, `None` == the C++ invalid `rampoint`).
//!   - **`scope->queryFunction(rampoint)->getAddress()`** (the symbol-table
//!     query, W4): supplied as `function_entry` — the queried function's entry
//!     address (`None` == no function at `rampoint`).

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

/// (kuna) Does `opcode` order its constant operand against something?
///
/// An ordering comparison pins a bound, and the strict/non-strict rewrite moves
/// the literal by one on the way here -- `MIN (n, BUFSIZ)` reaches this pass as
/// `INT_LESS(0x2000, n)` in coreutils `tail` and as `INT_LESS(0x2001, n)` in
/// coreutils `head`.  So a bound counts for its own value and for one below it.
pub fn orders_its_constant(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
    )
}

/// (kuna) Does `opcode` read its constant operand as a number?
///
/// An ordering comparison, a multiply, a divide, a remainder or a shift is
/// arithmetic on the value; C has none of them between a function pointer and
/// anything else.
pub fn reads_as_integer(opcode: OpCode) -> bool {
    orders_its_constant(opcode)
        || matches!(
        opcode,
        OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
    )
}

/// (kuna) May the single-bit escape be consulted for this constant?
///
/// The escape suspends the one guard (`bit_transitions < 3`) that keeps integer
/// flags, masks and round sizes out of the symbol table, so it may only be taken
/// where the operand means a code address.  `opcode` is the reader being decided;
/// `value_reads_as_integer` says whether the same numeric value is read as a
/// number ([`reads_as_integer`]) anywhere else in this function.
///
/// The second half is what a round buffer size looks like in a
/// position-independent executable, where `.init` sits at a low, round offset:
/// coreutils `tail` at `-O0` computes `MIN (n_remaining, BUFSIZ)`, `BUFSIZ` is
/// `0x2000` and `0x2000` is that image's `_DT_INIT`, so the comparison printed as
/// `if (_DT_INIT < v9)`, the other arm of the select as `v1 = _DT_INIT`, and the
/// `uintmax_t` byte counter they bounded became `void *` -- in the callee, and
/// from there through `protoorder` into its caller's stack variable.  The
/// comparison alone is not enough to refuse: the assignment is a plain COPY, the
/// shape a function-pointer table also has (coreutils `od` selects between
/// `sub_3f20`, `sub_3ff0`, `sub_40a0` and `sub_4150`, and only `sub_3ff0` has
/// fewer than three bit transitions).  What separates them is that the function
/// orders the one value and never orders the other.
pub fn entry_escape_applies(opcode: OpCode, value_reads_as_integer: bool) -> bool {
    !value_reads_as_integer && !reads_as_integer(opcode)
}

/// (kuna) Does the constant resolve exactly to a known function entry?
/// (C++ `kunaIsFunctionEntry`, GH-6930).
///
/// C++ `kuna_inferfuncentry.cc:12-26`.
///
/// `rampoint` is the resolved `glb->resolveConstant(...)`: `None` transcribes
/// the C++ invalid address (`rampoint.isInvalid()`).  // STUB(W4)
/// `function_entry` is the entry address of the function `queryFunction(rampoint)`
/// returns, or `None` if no function is at `rampoint`.  // STUB(W4)
///
/// Returns `true` only for an *exact* function-entry match — the C++
/// `fd->getAddress() == rampoint` — so an ordinary single-bit integer constant
/// (no function at its resolved address) is never reclassified.
pub fn kuna_is_function_entry(
    rampoint: Option<&Address>,
    function_entry: Option<&Address>,
) -> bool {
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
