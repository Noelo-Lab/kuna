//! Decode one machine instruction by driving the ported SLEIGH engine (design §4).
//!
//! The decompiler returns **length + a flat list of p-code ops** via a
//! [`PcodeEmit`] sink (there is no `getFlow()` outside the decompiler), and the
//! mnemonic via a parallel [`AssemblyEmit`] sink. [`decode_one`] builds an
//! [`Address`] in the code space, drives [`Translate::one_instruction`] with a
//! capturing `PcodeEmit`, and [`Translate::print_assembly`] with a capturing
//! `AssemblyEmit`, returning `(len, ops, mnemonic)`.
//!
//! The decode reads bytes through the loader the engine already has attached
//! (the same loader `bootstrap_from_object` installed), so no extra setup is needed
//! beyond a live [`Translate`].

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};

use super::model::RawOp;

/// A capturing [`PcodeEmit`] that records the emitted ops (opcode + first input)
/// for one instruction. Only `in0` is kept — enough for the classifier and a
/// future `skipNOPS` consumer (design §2.1).
#[derive(Default)]
struct OpCapture {
    ops: Vec<RawOp>,
}

impl PcodeEmit for OpCapture {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        _outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push(RawOp { opcode: opc, in0: vars.first().cloned() });
    }
}

/// A capturing [`AssemblyEmit`] that records the mnemonic for one instruction.
#[derive(Default)]
struct AsmCapture {
    mnemonic: String,
}

impl AssemblyEmit for AsmCapture {
    fn dump(&mut self, _addr: &Address, mnem: &str, _body: &str) {
        self.mnemonic = mnem.to_string();
    }
}

/// One decoded instruction's raw output.
pub struct Decoded {
    /// The fall-through byte length (the `one_instruction` return; folds in
    /// delay slots — design §4.3 gotcha 3).
    pub len: u32,
    /// The flat p-code op list the decoder emitted.
    pub ops: Vec<RawOp>,
    /// The decoded mnemonic.
    pub mnemonic: String,
}

/// Decode the instruction at `vma` (in `code_space`) by driving `translate`.
///
/// Returns `Err` for an undecodable address (`KunaError::Unimpl` / `BadData`
/// from the engine); the caller's policy is to stop-this-path on an error
/// (design §3.4). The mnemonic capture is best-effort: if `print_assembly`
/// errs after `one_instruction` succeeded, the instruction is still returned
/// (with whatever mnemonic was captured, possibly empty) — the p-code (and thus
/// the flow classification) is the load-bearing output.
pub fn decode_one(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
) -> KunaResult<Decoded> {
    let addr = Address::new(Rc::clone(code_space), vma);

    let mut cap = OpCapture::default();
    let len = translate.one_instruction(&mut cap, &addr)?;

    let mut asm = AsmCapture::default();
    // The mnemonic is non-essential to flow; if the disassembly emit errs we
    // keep the (possibly empty) captured string rather than failing the decode.
    let _ = translate.print_assembly(&mut asm, &addr);

    Ok(Decoded { len: len.max(0) as u32, ops: cap.ops, mnemonic: asm.mnemonic })
}
