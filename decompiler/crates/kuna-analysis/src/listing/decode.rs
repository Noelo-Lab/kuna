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
    /// Address-like constants written by a STORE's value input. On x86 this is
    /// the p-code provenance of `PUSH imm`; a LOAD's space/address inputs and a
    /// STORE's space/address inputs are deliberately not eligible.
    stored_scalar_values: Vec<u64>,
    /// Within-instruction COPY/extension provenance. SLEIGH lowers x86
    /// `PUSH imm` through a unique temporary before STORE slot 2.
    scalar_origins: Vec<(VarnodeData, u64)>,
    want_stored_scalars: bool,
}

impl OpCapture {
    fn scalar_origin(&self, vn: &VarnodeData) -> Option<u64> {
        let space = vn.space.as_ref()?;
        if space.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT {
            return Some(vn.offset);
        }
        self.scalar_origins
            .iter()
            .rev()
            .find_map(|(defined, value)| (defined == vn).then_some(*value))
    }

    fn remember_scalar_origin(
        &mut self,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        let Some(out) = outvar else { return };
        self.scalar_origins.retain(|(defined, _)| defined != out);
        if !matches!(opc, OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT) {
            return;
        }
        let Some(input) = vars.first() else { return };
        let Some(mut value) = self.scalar_origin(input) else { return };
        let bits = input.size.saturating_mul(8).min(64);
        let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
        value &= mask;
        if opc == OpCode::CPUI_INT_SEXT && bits != 0 && value & (1u64 << (bits - 1)) != 0 {
            value |= !mask;
        }
        self.scalar_origins.push((out.clone(), value));
    }
}

impl PcodeEmit for OpCapture {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push(RawOp { opcode: opc, in0: vars.first().cloned() });
        if !self.want_stored_scalars {
            return;
        }
        if opc != OpCode::CPUI_STORE {
            self.remember_scalar_origin(opc, outvar, vars);
            return;
        }
        // STORE(space, address, value): only the VALUE can be the immediate a
        // PUSH places on the stack. In particular, `push [absolute]` first
        // LOADs from that absolute address and STOREs a temporary; harvesting
        // LOAD slot 1 would falsely turn the memory address into a callback.
        let Some(vn) = vars.get(2) else { return };
        if let Some(value) = self.scalar_origin(vn) {
            if super::xrefs::looks_like_address(value) {
                self.stored_scalar_values.push(value);
            }
        }
    }
}

/// A capturing [`AssemblyEmit`] that records the mnemonic + operand body for one
/// instruction.
#[derive(Default)]
struct AsmCapture {
    mnemonic: String,
    operands: String,
}

impl AssemblyEmit for AsmCapture {
    fn dump(&mut self, _addr: &Address, mnem: &str, body: &str) {
        self.mnemonic = mnem.to_string();
        self.operands = body.to_string();
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
    /// The decoded operand body (the `body` half of `AssemblyEmit::dump`).
    pub operands: String,
    /// Address-like constants materialized by this instruction, excluding
    /// direct control-flow and memory-address operand slots.
    pub stored_scalar_values: Vec<u64>,
}

/// Decode the instruction at `vma` (in `code_space`) by driving `translate`.
///
/// Returns `Err` for an undecodable address (`KunaError::Unimpl` / `BadData`
/// from the engine); the caller's policy is to stop-this-path on an error
/// (design §3.4). The mnemonic capture is best-effort: if `print_assembly`
/// errs after `one_instruction` succeeded, the instruction is still returned
/// (with whatever mnemonic was captured, possibly empty) — the p-code (and thus
/// the flow classification) is the load-bearing output.
///
/// `want_assembly` selects whether the assembly TEXT is captured at all.
/// `print_assembly` is a **second, full SLEIGH parse** of the same bytes, so it
/// roughly doubles the per-instruction decode cost and adds two heap `String`s per
/// instruction. Only the text-reading consumers need it (`noreturn_propagate`,
/// `tailcallentry` and `poolentry` off [`super::Insn`], and the AIF prologue
/// fingerprinter); pass `false` wherever only `len`/`ops` are read and the two
/// text fields come back empty. `want_stored_scalars` selects the narrow x86
/// callback provenance described by [`Decoded::stored_scalar_values`]; it is
/// false for every caller except the x86 fast-discovery Listing walk.
pub fn decode_one(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    want_assembly: bool,
    want_stored_scalars: bool,
) -> KunaResult<Decoded> {
    let addr = Address::new(Rc::clone(code_space), vma);

    let mut cap = OpCapture { want_stored_scalars, ..OpCapture::default() };
    let len = translate.one_instruction(&mut cap, &addr)?;

    let mut asm = AsmCapture::default();
    if want_assembly {
        // The mnemonic is non-essential to flow; if the disassembly emit errs we
        // keep the (possibly empty) captured string rather than failing the decode.
        let _ = translate.print_assembly(&mut asm, &addr);
    }

    cap.stored_scalar_values.sort_unstable();
    cap.stored_scalar_values.dedup();
    Ok(Decoded {
        len: len.max(0) as u32,
        ops: cap.ops,
        mnemonic: asm.mnemonic,
        operands: asm.operands,
        stored_scalar_values: cap.stored_scalar_values,
    })
}

/// The mnemonic at `vma` alone, for a consumer that has already decoded the
/// instruction and only now needs its spelling. Empty when the disassembly
/// emit fails.
pub fn mnemonic_at(translate: &dyn Translate, vma: u64, code_space: &Rc<AddrSpace>) -> String {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut asm = AsmCapture::default();
    let _ = translate.print_assembly(&mut asm, &addr);
    asm.mnemonic
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use kuna_base::error::KunaError;
    use kuna_sleigh::globalcontext::ContextInternal;
    use kuna_sleigh::loadimage::LoadImage;
    use kuna_sleigh::sleigh::Sleigh;

    use super::*;

    struct MemImage {
        base: u64,
        bytes: Vec<u8>,
    }

    impl LoadImage for MemImage {
        fn get_file_name(&self) -> &str {
            "callback-decode"
        }

        fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
            for (i, byte) in ptr.iter_mut().enumerate() {
                let vma = addr.get_offset().wrapping_add(i as u64);
                *byte = vma
                    .checked_sub(self.base)
                    .and_then(|offset| self.bytes.get(offset as usize).copied())
                    .unwrap_or(0);
            }
            Ok(())
        }

        fn get_arch_type(&self) -> Vec<u8> {
            Vec::new()
        }

        fn adjust_vma(&mut self, _adjust: i64) {}
    }

    struct DummyImage;

    impl LoadImage for DummyImage {
        fn get_file_name(&self) -> &str {
            "dummy"
        }

        fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
            Err(KunaError::data_unavail("dummy"))
        }

        fn get_arch_type(&self) -> Vec<u8> {
            Vec::new()
        }

        fn adjust_vma(&mut self, _adjust: i64) {}
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
    }

    fn x86(bytes: &[u8]) -> Option<(Sleigh, Rc<AddrSpace>)> {
        let path = repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86.sla");
        let Ok(sla) = std::fs::read(&path) else {
            eprintln!("callback decode tests: skipping (no `{}`; run `make specs`)", path.display());
            return None;
        };
        let mut sleigh = Sleigh::new(Box::new(DummyImage), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).expect("initialize x86.sla");
        sleigh.set_context_default("addrsize", 1);
        sleigh.set_context_default("opsize", 1);
        sleigh.set_loader(Box::new(MemImage { base: 0x1000, bytes: bytes.to_vec() }));
        let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").expect("ram space"));
        Some((sleigh, ram))
    }

    #[test]
    fn decoded_x86_push_immediate_harvests_the_stored_value() {
        // `push 0x2000`: SUB ESP followed by STORE(space, ESP, 0x2000).
        let Some((sleigh, ram)) = x86(&[0x68, 0x00, 0x20, 0x00, 0x00]) else { return };
        let decoded = decode_one(&sleigh, 0x1000, &ram, true, true).unwrap();
        assert_eq!(decoded.mnemonic, "PUSH");
        assert_eq!(decoded.stored_scalar_values, vec![0x2000]);
    }

    #[test]
    fn decoded_x86_push_absolute_memory_does_not_harvest_the_load_address() {
        // `push dword ptr [0x2000]`: LOAD(space, 0x2000), then STORE the loaded
        // temporary. The memory address is not the value pushed and must not be
        // proposed as a callback.
        let Some((sleigh, ram)) = x86(&[0xff, 0x35, 0x00, 0x20, 0x00, 0x00]) else { return };
        let decoded = decode_one(&sleigh, 0x1000, &ram, true, true).unwrap();
        assert_eq!(decoded.mnemonic, "PUSH");
        assert!(decoded.stored_scalar_values.is_empty());
    }
}
