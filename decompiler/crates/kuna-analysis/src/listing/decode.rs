//! Decode one machine instruction by driving the ported SLEIGH engine (design §4).
//!
//! The decompiler returns **length + a flat list of p-code ops** via a
//! [`PcodeEmit`] sink (there is no `getFlow()` outside the decompiler), and the
//! mnemonic via a parallel [`AssemblyEmit`] sink. [`decode_one`] builds an
//! [`Address`] in the code space and drives [`Translate::one_instruction_with_assembly`]
//! with capturing `PcodeEmit` and `AssemblyEmit` sinks when text is requested,
//! returning `(len, ops, mnemonic)`.
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

use super::model::{Mnemonic, RawOp};

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
        self.mnemonic.clear();
        self.mnemonic.push_str(mnem);
        self.operands.clear();
        self.operands.push_str(body);
    }
}

/// One decoded instruction's raw output.
#[derive(Default)]
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

/// The Listing walk's decode product. It interns the mnemonic directly from
/// the assembly callback while retaining the operand body as owned text.
pub(super) struct DecodedListing {
    pub(super) len: u32,
    pub(super) ops: Vec<RawOp>,
    pub(super) mnemonic: Mnemonic,
    pub(super) operands: String,
    pub(super) stored_scalar_values: Vec<u64>,
}

struct CapturedInstruction {
    len: u32,
    ops: Vec<RawOp>,
    stored_scalar_values: Vec<u64>,
}

/// Decode the instruction at `vma` (in `code_space`) by driving `translate`.
///
/// Returns `Err` for an undecodable address (`KunaError::Unimpl` / `BadData`
/// from the engine); the caller's policy is to stop-this-path on an error
/// (design §3.4). Assembly capture is best-effort: if rendering errs after the
/// p-code lift succeeds, the instruction is still returned (with whatever
/// mnemonic was captured, possibly empty) — the p-code (and thus the flow
/// classification) is the load-bearing output.
///
/// `want_assembly` selects whether assembly text is captured. When requested,
/// this uses [`Translate::one_instruction_with_assembly`]: SLEIGH can render
/// from the p-code parse when safe, while context-commit and delay-slot cases
/// retain the post-lift assembly fallback. Other translators inherit the trait's
/// sequential best-effort fallback. Pass `false` wherever only `len`/`ops` are
/// read and the text fields should remain empty. This public helper returns
/// both text fields as owned `String`s; the Listing walk uses
/// [`decode_one_listing`] to intern its repeated mnemonic text directly from the
/// assembly callback while retaining operands as a `String`. `want_stored_scalars`
/// selects the narrow x86 callback provenance described by
/// [`Decoded::stored_scalar_values`]; it is false for every caller except the x86
/// fast-discovery Listing walk.
pub fn decode_one(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    want_assembly: bool,
    want_stored_scalars: bool,
) -> KunaResult<Decoded> {
    let mut asm = AsmCapture::default();
    let captured = capture_instruction(
        translate,
        vma,
        code_space,
        want_assembly,
        want_stored_scalars,
        &mut asm,
        None,
    )?;
    Ok(Decoded {
        len: captured.len,
        ops: captured.ops,
        mnemonic: asm.mnemonic,
        operands: asm.operands,
        stored_scalar_values: captured.stored_scalar_values,
    })
}

/// Decode for the Listing walker, interning mnemonic text in the serial store
/// or current parallel lane as soon as the assembly callback provides it.
///
/// `key`, when given, receives the decode context that certifies a
/// context-free decode (see [`super::kuna_decodememo`]).
pub(super) fn decode_one_listing(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    want_assembly: bool,
    want_stored_scalars: bool,
    intern: &mut impl FnMut(&str) -> Mnemonic,
    key: Option<&mut Option<Vec<u32>>>,
) -> KunaResult<DecodedListing> {
    let mut asm = InterningAsmCapture::new(intern);
    let captured = capture_instruction(
        translate,
        vma,
        code_space,
        want_assembly,
        want_stored_scalars,
        &mut asm,
        key,
    )?;
    Ok(DecodedListing {
        len: captured.len,
        ops: captured.ops,
        mnemonic: asm.mnemonic,
        operands: asm.operands,
        stored_scalar_values: captured.stored_scalar_values,
    })
}

fn capture_instruction(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    want_assembly: bool,
    want_stored_scalars: bool,
    asm: &mut dyn AssemblyEmit,
    key: Option<&mut Option<Vec<u32>>>,
) -> KunaResult<CapturedInstruction> {
    let addr = Address::new(Rc::clone(code_space), vma);

    let mut cap = OpCapture { want_stored_scalars, ..OpCapture::default() };
    let len = if let Some(key) = key.filter(|_| want_assembly) {
        translate.one_instruction_reusable(&mut cap, asm, &addr, key)?
    } else if want_assembly {
        translate.one_instruction_with_assembly(&mut cap, asm, &addr)?
    } else {
        translate.one_instruction(&mut cap, &addr)?
    };

    cap.stored_scalar_values.sort_unstable();
    cap.stored_scalar_values.dedup();
    Ok(CapturedInstruction {
        len: len.max(0) as u32,
        ops: cap.ops,
        stored_scalar_values: cap.stored_scalar_values,
    })
}

struct InterningAsmCapture<'a, F> {
    intern: &'a mut F,
    mnemonic: Mnemonic,
    operands: String,
}

impl<'a, F> InterningAsmCapture<'a, F>
where
    F: FnMut(&str) -> Mnemonic,
{
    fn new(intern: &'a mut F) -> Self {
        Self {
            intern,
            mnemonic: Mnemonic::default(),
            operands: String::new(),
        }
    }
}

impl<F> AssemblyEmit for InterningAsmCapture<'_, F>
where
    F: FnMut(&str) -> Mnemonic,
{
    fn dump(&mut self, _addr: &Address, mnemonic: &str, body: &str) {
        self.mnemonic = (self.intern)(mnemonic);
        self.operands.clear();
        self.operands.push_str(body);
    }
}

/// AIF needs both flow and text for each new speculative decode. The combined
/// translator operation can reuse its parse without changing either result.
pub(crate) fn decode_one_with_assembly(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    decoded: &mut Decoded,
) -> KunaResult<()> {
    let addr = Address::new(Rc::clone(code_space), vma);
    decoded.ops.clear();
    decoded.mnemonic.clear();
    decoded.operands.clear();
    let mut cap = OpCapture { ops: std::mem::take(&mut decoded.ops), ..Default::default() };
    let mut asm = AsmCapture {
        mnemonic: std::mem::take(&mut decoded.mnemonic),
        operands: std::mem::take(&mut decoded.operands),
    };
    let length = translate.one_instruction_with_assembly(&mut cap, &mut asm, &addr);
    decoded.ops = cap.ops;
    decoded.mnemonic = asm.mnemonic;
    decoded.operands = asm.operands;
    decoded.len = length?.max(0) as u32;
    Ok(())
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
    use std::cell::Cell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use kuna_base::error::KunaError;
    use kuna_base::space::{RegisterLookup, VarnodeStorage};
    use kuna_sleigh::globalcontext::ContextInternal;
    use kuna_sleigh::loadimage::LoadImage;
    use kuna_sleigh::sleigh::Sleigh;
    use kuna_sleigh::translate::TranslateBase;

    use super::*;

    struct MemImage {
        base: u64,
        bytes: Vec<u8>,
        reads: Rc<Cell<usize>>,
        fail: bool,
    }

    impl LoadImage for MemImage {
        fn get_file_name(&self) -> &str {
            "callback-decode"
        }

        fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
            self.reads.set(self.reads.get() + 1);
            if self.fail {
                return Err(KunaError::data_unavail("counted test image failure"));
            }
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

    fn x86_with(bytes: &[u8], fail: bool) -> (Sleigh, Rc<AddrSpace>, Rc<Cell<usize>>) {
        let path = repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86.sla");
        let sla = std::fs::read(&path)
            .unwrap_or_else(|error| panic!("required compiled spec {}: {error}", path.display()));
        let mut sleigh = Sleigh::new(Box::new(DummyImage), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).expect("initialize x86.sla");
        sleigh.set_context_default("addrsize", 1);
        sleigh.set_context_default("opsize", 1);
        let reads = Rc::new(Cell::new(0));
        sleigh.set_loader(Box::new(MemImage {
            base: 0x1000,
            bytes: bytes.to_vec(),
            reads: Rc::clone(&reads),
            fail,
        }));
        let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").expect("ram space"));
        (sleigh, ram, reads)
    }

    struct AssemblyFailure<'a> {
        inner: &'a Sleigh,
        base: TranslateBase,
        calls: Rc<Cell<usize>>,
    }

    impl RegisterLookup for AssemblyFailure<'_> {
        fn get_register(&self, name: &str) -> KunaResult<VarnodeStorage> {
            self.inner.get_register(name)
        }

        fn get_register_name(&self, base: &Rc<AddrSpace>, offset: u64, size: i32) -> String {
            self.inner.get_register_name(base, offset, size)
        }

        fn get_exact_register_name(&self, base: &Rc<AddrSpace>, offset: u64, size: i32) -> String {
            self.inner.get_exact_register_name(base, offset, size)
        }
    }

    impl Translate for AssemblyFailure<'_> {
        fn translate_base(&self) -> &TranslateBase {
            &self.base
        }

        fn translate_base_mut(&mut self) -> &mut TranslateBase {
            &mut self.base
        }

        fn initialize(&mut self, _store: &mut kuna_base::xml::DocumentStorage) -> KunaResult<()> {
            Ok(())
        }

        fn get_all_registers(
            &self,
            registers: &mut std::collections::BTreeMap<VarnodeData, String>,
        ) {
            self.inner.get_all_registers(registers);
        }

        fn get_user_op_names(&self, names: &mut Vec<String>) {
            self.inner.get_user_op_names(names);
        }

        fn instruction_length(&self, addr: &Address) -> KunaResult<i32> {
            self.inner.instruction_length(addr)
        }

        fn one_instruction(&self, emit: &mut dyn PcodeEmit, addr: &Address) -> KunaResult<i32> {
            self.inner.one_instruction(emit, addr)
        }

        fn print_assembly(&self, _emit: &mut dyn AssemblyEmit, _addr: &Address) -> KunaResult<i32> {
            self.calls.set(self.calls.get() + 1);
            Err(KunaError::lowlevel("intentional test assembly failure"))
        }
    }

    #[test]
    fn decoded_x86_push_immediate_harvests_the_stored_value() {
        // `push 0x2000`: SUB ESP followed by STORE(space, ESP, 0x2000).
        let (sleigh, ram, _) = x86_with(&[0x68, 0x00, 0x20, 0x00, 0x00], false);
        let decoded = decode_one(&sleigh, 0x1000, &ram, true, true).unwrap();
        assert_eq!(decoded.mnemonic, "PUSH");
        assert_eq!(decoded.stored_scalar_values, vec![0x2000]);

        let mut pool = super::super::model::MnemonicPool::default();
        let listed = decode_one_listing(&sleigh, 0x1000, &ram, true, true, &mut |text| {
            pool.intern(text)
        }, None)
        .unwrap();
        assert_eq!(listed.len, decoded.len);
        assert_eq!(listed.mnemonic.as_str(), decoded.mnemonic.as_str());
        assert_eq!(listed.operands, decoded.operands);
        assert_eq!(listed.ops.len(), decoded.ops.len());
        assert_eq!(listed.stored_scalar_values, decoded.stored_scalar_values);
    }

    #[test]
    fn decoded_x86_push_absolute_memory_does_not_harvest_the_load_address() {
        // `push dword ptr [0x2000]`: LOAD(space, 0x2000), then STORE the loaded
        // temporary. The memory address is not the value pushed and must not be
        // proposed as a callback.
        let (sleigh, ram, _) = x86_with(&[0xff, 0x35, 0x00, 0x20, 0x00, 0x00], false);
        let decoded = decode_one(&sleigh, 0x1000, &ram, true, true).unwrap();
        assert_eq!(decoded.mnemonic, "PUSH");
        assert!(decoded.stored_scalar_values.is_empty());
    }

    #[test]
    fn combined_listing_capture_uses_one_read_and_skips_assembly_after_lift_failure() {
        let (sleigh, ram, reads) = x86_with(&[0x68, 0x00, 0x20, 0x00, 0x00], false);
        let decoded = decode_one(&sleigh, 0x1000, &ram, true, true).unwrap();
        assert_eq!(decoded.mnemonic, "PUSH");
        assert_eq!(reads.get(), 1, "SLEIGH should share the p-code parse with text rendering");

        let (sleigh, ram, reads) = x86_with(&[0x68, 0x00, 0x20, 0x00, 0x00], false);
        let decoded = decode_one(&sleigh, 0x1000, &ram, false, true).unwrap();
        assert!(decoded.mnemonic.is_empty());
        assert!(decoded.operands.is_empty());
        assert_eq!(reads.get(), 1, "assembly-disabled capture should only lift once");

        let (sleigh, ram, reads) = x86_with(&[0x68, 0x00, 0x20, 0x00, 0x00], true);
        assert!(decode_one(&sleigh, 0x1000, &ram, true, true).is_err());
        assert_eq!(reads.get(), 1, "a failed lift must not attempt assembly capture");
    }

    #[test]
    fn generic_combined_fallback_ignores_assembly_failure() {
        let (sleigh, ram, reads) = x86_with(&[0x90], false);
        let calls = Rc::new(Cell::new(0));
        let translate = AssemblyFailure {
            inner: &sleigh,
            base: TranslateBase::default(),
            calls: Rc::clone(&calls),
        };
        let decoded = decode_one(&translate, 0x1000, &ram, true, false)
            .expect("assembly failure is best-effort after a successful p-code lift");
        assert!(decoded.len > 0);
        assert!(decoded.mnemonic.is_empty());
        assert!(decoded.operands.is_empty());
        assert_eq!(calls.get(), 1);
        assert_eq!(reads.get(), 1, "the generic fallback must not reparse after failed rendering");
    }
}
