//! (kuna, rustc) Keep the two-register value a Rust function returns, and connect
//! it at the call.
//!
//! # The symptom
//!
//! A `Result<u32,u32>` producer and the `match` that consumes it, `rustc -C
//! opt-level=2`, x86-64:
//!
//! ```text
//! prod:  xor %eax,%eax; cmp $0xb,%edi; setb %al; lea 0x7(%rdi),%edx; cmovae %ecx,%edx; ret
//! cons:  call prod; test $0x1,%al; lea 0x1(%rdx),%esi; lea 0x64(%rdx),%ecx; cmove %esi,%ecx
//! ```
//!
//! recovered as
//!
//! ```text
//! bool prod(uint4 a0) { return a0 < 0xb; }        // the EDX payload is GONE
//!
//! int4 cons(void) {
//!   uint8 v2; // rax
//!   int4 v3;  // edx                              // declared, read twice, NEVER ASSIGNED
//!   prod();
//!   v1 = v3 + 100;
//!   if (!(v2 & 1)) v1 = v3 + 1;
//!   return v1;
//! }
//! ```
//!
//! The payload the whole function exists to produce is not merely mis-typed: on
//! the producer side it is deleted, and on the consumer side it is a variable
//! with no definition anywhere in the program. Every `Result`/`Option` guard
//! downstream reads a phantom.
//!
//! # What was actually missing
//!
//! Not the ABI decision. rustc's `ScalarPair` return -- discriminant in the first
//! return register, payload in the second -- lands on exactly the storage the
//! x86-64 cspec's `<join_dual_class/>` output rule already describes, and kuna
//! already recovers it: two active output trials in `RAX`/`RDX`, the rule matches,
//! `buildReturnOutput` builds the `join` concatenation. Two *later* seams throw it
//! away, and both of them are invisible to a C corpus because a C function whose
//! first returned register holds a one-bit value is rare and a Rust `Result` is
//! nothing else.
//!
//! **Producer.** `SubvariableFlow::tryReturnPull` truncates a RETURN to the
//! logical width of the value being traced. rustc materializes the discriminant
//! with `xor %eax,%eax; setb %al`, so `RAX` is a one-bit logical value and the
//! subvariable rules truncate the whole RETURN to it -- which silently discards
//! the *other register* in the concatenation. The recovered return goes from a
//! 16-byte pair to `bool`.
//!
//! **Consumer.** `FuncCallSpecs::buildOutputFromTrials` handles one used output
//! trial and returns early on two or more ("STUB(W4 translate-on-handle)"), so a
//! call whose model asks for a register pair gets **no output at all**. The
//! INDIRECT creations that stood for "the callee wrote something here" survive,
//! and every read of the payload register after the call renders as a local the
//! function never assigns. The stub's stated blocker -- no `constructJoinAddress`
//! on the merged arch handle -- is stale: the sibling
//! `ActionReturnRecovery::buildReturnOutput` calls it today.
//!
//! # The classification
//!
//! Both seams need the same question answered: *is this register pair one
//! logical value?* Asked of the observed writes at the return sites, never of a
//! size -- rustc's choice is a function of the variant layout, and size does not
//! predict it (`Result<u32,u32>` is 8 bytes and a pair; `Result<Box<u64>,u32>` is
//! 16 bytes and is returned through memory).
//!
//! [`classify_return_pair`] reads the concatenation the ABI already built and
//! reports one of three verdicts:
//!
//! * **`ScalarPair`** -- the least-significant half is *discriminant-shaped*: its
//!   value at the RETURN is confined to a byte (a small constant, a `setCC`, a
//!   masked flag, or a phi over those). That is the discriminant, and the other
//!   half is its payload. The recognition is deliberately on the value's known
//!   non-zero bits rather than on "a constant per path", because rustc emits both
//!   forms for the same source -- `mov $0/$1` on the branchy shape and `setb %al`
//!   on the branchless one, and the branchless shape is the common one at
//!   `-C opt-level=2`.
//! * **`Memory`** -- the half traces back to the function's own incoming pointer
//!   argument. That is the `sret` epilogue (`mov %rbx,%rax` where `rbx` came from
//!   `rdi`), where `RAX` carries the hidden return pointer and is not a
//!   discriminant. A veto, not an action: the pair must not form.
//! * **`Scalar`** -- anything else. Today's answer, unchanged.
//!
//! # What this does NOT do
//!
//! It does not name anything `Result` or `Option`, does not synthesize a union,
//! struct or enum type, and does not touch emission. Its entire deliverable is
//! that the payload **exists as a variable and is connected to its producer**;
//! spelling that value as a Rust enum is a later, separate decision that cannot
//! be made until the value survives to be spelled.
//!
//! # The gate
//!
//! `option rustabi off|auto|always`. `auto` acts only when the loader's
//! source-language detection reported rustc (`Compiler::Rustc`, from the
//! `.comment` `rustc version` record, a `.rodata` signature, or a Rust-mangled
//! symbol), which the XML datatest bootstrap never runs -- so the C corpus is
//! untouched by construction, not by luck. `always` drops the language test and
//! is what the stage testcase uses, since a `<bytechunk>` carries no
//! `.comment`.

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How far back the discriminant/sret walks chase move-only operations. The
/// shapes that matter are one or two copies deep; the bound only stops a cycle.
const MAX_DEPTH: u32 = 16;

/// The widest non-zero mask a *discriminant* half may carry. A tag is an index
/// into a variant list, so one byte is already generous; a payload register that
/// happens to be byte-wide is indistinguishable from a tag and is left alone.
const DISCRIMINANT_MASK: u64 = 0xff;

/// The three values of `option rustabi`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RustAbiMode {
    /// Never act (the shipped default): byte-identical to the pre-fix engine.
    Off,
    /// Act only when the loader detected a rustc-produced image.
    Auto,
    /// Act regardless of the detected source language.
    Always,
}

impl RustAbiMode {
    /// The `option rustabi <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            RustAbiMode::Off => "off",
            RustAbiMode::Auto => "auto",
            RustAbiMode::Always => "always",
        }
    }

    /// The wire encoding carried on `Architecture`/`ArchContext` (0/1/2).
    pub fn as_u8(self) -> u8 {
        match self {
            RustAbiMode::Off => 0,
            RustAbiMode::Auto => 1,
            RustAbiMode::Always => 2,
        }
    }

    /// Decode the wire encoding; anything unrecognized is [`RustAbiMode::Off`].
    pub fn from_u8(v: u8) -> RustAbiMode {
        match v {
            1 => RustAbiMode::Auto,
            2 => RustAbiMode::Always,
            _ => RustAbiMode::Off,
        }
    }
}

/// Parse `option rustabi off|auto|always`, returning the mode and the
/// confirmation message.
pub fn parse_rust_abi_mode(p1: &str) -> KunaResult<(RustAbiMode, String)> {
    let mode = match p1.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => RustAbiMode::Off,
        "auto" | "on" | "1" | "true" => RustAbiMode::Auto,
        "always" => RustAbiMode::Always,
        _ => return Err(KunaError::parse("Must specify off, auto or always")),
    };
    Ok((mode, format!("Rust return-ABI pair recovery set to {} form", mode.as_str())))
}

/// Is the Rust return-ABI rule live for this function?
pub fn live(data: &Funcdata) -> bool {
    match RustAbiMode::from_u8(data.get_arch().rust_abi) {
        RustAbiMode::Off => false,
        RustAbiMode::Auto => data.get_arch().source_is_rust,
        RustAbiMode::Always => true,
    }
}

/// What one recovered register pair represents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnRepr {
    /// Discriminant + payload: two registers holding one logical value.
    ScalarPair,
    /// A hidden return pointer handed back in the first return register.
    Memory,
    /// Neither shape applies; keep today's answer.
    Scalar,
}

/// Classify a recovered two-register return concatenation from its halves.
///
/// `lo` is the least-significant register (the first return register, where
/// rustc puts the tag) and `hi` the second. Three conditions, in the order they
/// can be decided: the low half is not the sret pointer, the low half is a tag,
/// and the high half is a value this function put there rather than leftover.
pub fn classify_return_pair(data: &Funcdata, hi: VarnodeId, lo: VarnodeId) -> ReturnRepr {
    if traces_to_incoming_pointer(data, lo, 0) {
        return ReturnRepr::Memory;
    }
    if is_discriminant_shaped(data, lo) && carries_a_payload(data, hi) {
        return ReturnRepr::ScalarPair;
    }
    ReturnRepr::Scalar
}

/// Is `vn` a value the function put in the payload register?
///
/// The narrow half alone does not make a pair: the same shape appears when a
/// function returns a boolean and the second return register merely still holds
/// whatever it held. Two terminals say leftover — a Varnode the function never
/// wrote, and a callee's INDIRECT creation — and they are the same two
/// [`crate::kuna_returnuncomputed`] exists to drop. Asking here as well keeps
/// this rule from holding a pair alive for the phantom-killer to have to undo.
fn carries_a_payload(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(def) = v.get_def() else { return false };
    match data.obank().get(def) {
        Some(op) => !(op.code() == OpCode::CPUI_INDIRECT && op.is_indirect_creation()),
        None => false,
    }
}

/// Is `vn` confined to the bits a variant tag occupies?
///
/// A constant that fits in a byte, or a value whose dataflow-computed non-zero
/// mask fits in a byte. The second arm is what catches `xor %eax,%eax; setb %al`
/// and `and $0x1,%eax`, which is how rustc materializes a two-variant tag at
/// `-C opt-level=2`; the first catches the branchy `mov $0` / `mov $1` form.
fn is_discriminant_shaped(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return v.get_offset() <= DISCRIMINANT_MASK;
    }
    // A zero non-zero-mask means "provably zero", which is a tag value too, but
    // it is also what an unanalyzed Varnode reports before dataflow has run; the
    // rule only ever runs after heritage, where zero is a real answer.
    v.get_nz_mask() <= DISCRIMINANT_MASK
}

/// Does `vn` trace back, through move-only operations, to an incoming pointer
/// argument of this function?
///
/// The `sret` shape: the callee is handed the result buffer in the first
/// argument register and returns that same pointer, so the first return register
/// carries an address, not a tag.
fn traces_to_incoming_pointer(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth >= MAX_DEPTH {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    let Some(def) = v.get_def() else {
        // An unwritten Varnode in input-parameter storage, pointer-sized.
        if !v.is_input() {
            return false;
        }
        let (addr, size) = (v.get_addr().clone(), v.get_size());
        if Some(size) != pointer_size(data) {
            return false;
        }
        return data.get_func_proto().possible_input_param(&addr, size);
    };
    let Some(op) = data.obank().get(def) else { return false };
    let inputs: Vec<VarnodeId> = match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT => op.get_in(0).into_iter().collect(),
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).filter_map(|i| op.get_in(i)).collect(),
        _ => return false,
    };
    inputs.into_iter().any(|i| traces_to_incoming_pointer(data, i, depth + 1))
}

/// Does `vn` hold a recovered register pair that must survive intact?
///
/// The predicate `SubvariableFlow::tryReturnPull` consults before truncating a
/// RETURN. `vn` is the RETURN's value; the pair shape is the `join`-space
/// concatenation that `ActionReturnRecovery::buildReturnOutput` builds, and
/// nothing else in the engine produces one. Truncating it would keep one half's
/// logical bits and drop the other register entirely -- which is not a narrower
/// rendering of the same value, it is a different value.
///
/// The later uncomputed-half repair ([`crate::kuna_returnuncomputed`]) still runs
/// on the pair this keeps alive, so a half that is genuine leftover is still
/// dropped -- just by the rule that can tell, instead of by a width heuristic.
pub fn holds_scalar_pair(data: &Funcdata, vn: VarnodeId) -> bool {
    if !live(data) {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if !v.get_addr().is_join() {
        return false;
    }
    let Some((hi, lo)) = pair_pieces(data, vn, 0) else { return false };
    classify_return_pair(data, hi, lo) == ReturnRepr::ScalarPair
}

/// The two halves of the concatenation behind `vn`, looking through the
/// width-reshaping the rule pool applies to it.
///
/// `buildReturnOutput` leaves a bare `PIECE(hi, lo)`, but the pool rewrites that
/// shape as soon as a half is itself an extension: `RuleConcatZext` turns
/// `PIECE(ZEXT(V), W)` into `ZEXT(PIECE(V, W))`, which is exactly what happens
/// when the payload register is written 32-bit (`lea 0x7(%rdi),%edx`) -- the
/// overwhelmingly common rustc case. Matching only the bare PIECE would miss it.
fn pair_pieces(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<(VarnodeId, VarnodeId)> {
    if depth >= MAX_DEPTH {
        return None;
    }
    let def = data.vbank().get(vn)?.get_def()?;
    let op = data.obank().get(def)?;
    match op.code() {
        OpCode::CPUI_PIECE if op.num_input() == 2 => Some((op.get_in(0)?, op.get_in(1)?)),
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            pair_pieces(data, op.get_in(0)?, depth + 1)
        }
        _ => None,
    }
}

/// Build the call's two-piece output the model asked for (C++
/// `FuncCallSpecs::buildOutputFromTrials`, the `numTrials > 1` arm kuna shipped
/// as a stub).
///
/// `finalvn` are the trial Varnodes in trial order -- least-significant first --
/// each currently the output of an INDIRECT creation sitting just before the
/// CALL. On success the CALL gains a `join`-space output covering both
/// registers, each half becomes a SUBPIECE of it inserted after the CALL, and
/// the INDIRECT creations are destroyed. Returns `false` (changing nothing) when
/// the gate is off, the shape is not a plain register pair, or the join address
/// cannot be constructed.
pub fn build_call_output_pair(
    callop: OpId,
    data: &mut Funcdata,
    finalvn: &[VarnodeId],
) -> bool {
    if !live(data) || finalvn.len() != 2 {
        return false;
    }
    let (lo, hi) = (finalvn[0], finalvn[1]);
    let Some((lo_addr, lo_size)) = register_piece(data, lo) else { return false };
    let Some((hi_addr, hi_size)) = register_piece(data, hi) else { return false };
    let Some(call_addr) = data.obank().get(callop).map(|o| o.get_addr().clone()) else {
        return false;
    };
    let manage = data.get_arch().manage.clone();
    let Some(joinaddr) = manage.register_lookup().and_then(|rl| {
        manage
            .construct_join_address(rl.as_ref(), &hi_addr, hi_size, &lo_addr, lo_size)
            .ok()
    }) else {
        return false;
    };
    let Ok(whole) = data.new_varnode_out(hi_size + lo_size, &joinaddr, callop) else {
        return false;
    };
    // The concatenation is a rendering of storage the CALL already wrote; it must
    // not become a heritage root of its own (same reason `buildReturnOutput` masks
    // the whole it builds).
    if let Some(v) = data.vbank_mut().get_mut(whole) {
        v.set_write_mask();
    }
    for (piece, off) in [(lo, 0), (hi, lo_size)] {
        let indop = data.vbank().get(piece).and_then(|v| v.get_def());
        let sub = data.new_op(2, call_addr.clone());
        data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
        let c = data.new_constant(4, off as u64);
        let _ = data.op_set_input(sub, whole, 0);
        let _ = data.op_set_input(sub, c, 1);
        // Steals `piece` from the INDIRECT creation that defined it.
        let _ = data.op_set_output(sub, piece);
        data.op_insert_after(sub, callop);
        if let Some(indop) = indop {
            data.op_destroy(indop);
        }
    }
    true
}

/// The pointer width of this architecture (the default code space's address
/// size), or `None` when the manager has no default code space.
fn pointer_size(data: &Funcdata) -> Option<int4> {
    data.get_arch().manage.get_default_code_space().map(|s| s.get_addr_size() as int4)
}

/// The (address, size) of a Varnode that is a plain register piece defined by an
/// INDIRECT creation at a call, or `None` for any other shape.
fn register_piece(data: &Funcdata, vn: VarnodeId) -> Option<(Address, int4)> {
    let v = data.vbank().get(vn)?;
    if v.get_addr().get_space()?.get_type() != kuna_base::space::spacetype::IPTR_PROCESSOR {
        return None;
    }
    let def = v.get_def()?;
    let op = data.obank().get(def)?;
    if op.code() != OpCode::CPUI_INDIRECT || !op.is_indirect_creation() {
        return None;
    }
    Some((v.get_addr().clone(), v.get_size()))
}

#[cfg(test)]
#[path = "kuna_rustabi/tests.rs"]
mod tests;
