//! Regression gate for the instruction-buffer tail read in
//! `ParserContext::getInstructionBytes` / `getInstructionBits` (kuna GH-271).
//!
//! `ParserContext` buffers `MAX_INSTRUCTION_LEN` (16) bytes, and both accessors
//! guard only the *start* byte of a read against that bound before pulling a
//! fixed-width run from it. C++ `PatternBlock::isInstructionMatch` walks a
//! pattern in `sizeof(uintm)` chunks whatever the pattern's real width, so
//! matching a 15-byte instruction issues a 4-byte read at offset 13 — one byte
//! past `buf`. Upstream reads adjacent struct memory there (UB) and the caller's
//! mask discards it; kuna raised `BadData` instead, which aborted the decode of
//! a valid instruction.
//!
//! The witness is clang's 15-byte loop-alignment NOP
//! (`66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00`, the x86 encoding-length
//! maximum), common in `-O2` output ahead of a loop head. `FlowInfo` turned the
//! failed decode into an artificial halt, truncating every function that
//! contained one. This test decodes it against the built `x86-64.sla`, and pins
//! that the start-byte guard still rejects a read genuinely past 16 bytes. It is
//! skipped when the `.sla` is absent (`make specs`).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// In-memory image: the instruction bytes at `base`, zero-filled elsewhere so
/// the decoder's read-ahead never errors.
struct MemImg {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImg {
    fn get_file_name(&self) -> &str {
        "mem"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let start = addr.get_offset();
        for (i, b) in ptr.iter_mut().enumerate() {
            let a = start.wrapping_add(i as u64);
            *b = a
                .checked_sub(self.base)
                .and_then(|off| self.bytes.get(off as usize).copied())
                .unwrap_or(0);
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

struct OpcodeEmit {
    ops: Vec<OpCode>,
}
impl PcodeEmit for OpcodeEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        _outvar: Option<&VarnodeData>,
        _vars: &[VarnodeData],
    ) {
        self.ops.push(opc);
    }
}

#[derive(Default)]
struct TextEmit {
    text: Option<(String, String)>,
}
impl AssemblyEmit for TextEmit {
    fn dump(&mut self, _addr: &Address, mnemonic: &str, body: &str) {
        self.text = Some((mnemonic.to_string(), body.to_string()));
    }
}

/// A stub loader used only until the real image is installed.
struct DummyImg;
impl LoadImage for DummyImg {
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

/// Build an x86-64 SLEIGH over `bytes` at `base`, or `None` when the `.sla` is
/// not built.
fn x86_64_sleigh(base: u64, bytes: Vec<u8>) -> Option<(Sleigh, Address)> {
    let sla_path =
        repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla");
    let Ok(sla) = std::fs::read(&sla_path) else {
        eprintln!(
            "x86_maxlen_nop_regression: skipping (no `{}`; run `make specs`)",
            sla_path.display()
        );
        return None;
    };

    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh.initialize_from_sla(&sla).expect("initialize x86-64.sla");
    // x86-64.pspec `<context_data>`: 64-bit long mode, 32-bit default operand.
    sleigh.set_context_default("addrsize", 2);
    sleigh.set_context_default("opsize", 1);
    sleigh.set_context_default("rexprefix", 0);
    sleigh.set_context_default("longMode", 1);

    sleigh.set_loader(Box::new(MemImg { base, bytes }));

    let ram =
        Rc::clone(sleigh.base().manager().get_space_by_name("ram").expect("ram space"));
    Some((sleigh, Address::new(ram, base)))
}

#[test]
fn x86_64_fifteen_byte_alignment_nop_decodes() {
    let base: u64 = 0x1000;
    // clang loop-alignment padding: six 0x66 operand-size prefixes, a 0x2e CS
    // segment override, then `0f 1f /4` with SIB + disp32 -- 15 bytes, the x86
    // maximum. `c3` (RET) anchors the far side so we can see the stream realign.
    let mut bytes = vec![
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];
    bytes.push(0xc3);

    let Some((sleigh, addr)) = x86_64_sleigh(base, bytes) else { return };

    let mut text = TextEmit::default();
    // Before the fix this failed with "Instruction byte read past buffer".
    let len = sleigh
        .print_assembly(&mut text, &addr)
        .expect("the 15-byte alignment NOP must disassemble");
    assert_eq!(len, 15, "the alignment NOP is 15 bytes (got {len})");
    let (mnem, _) = text.text.expect("assembly emitted");
    assert_eq!(mnem, "NOP", "the 15-byte form must decode as a NOP");

    let mut emit = OpcodeEmit { ops: Vec::new() };
    assert_eq!(sleigh.one_instruction(&mut emit, &addr).unwrap(), 15);

    // The stream realigns: the anchor RET decodes at base + 15.
    let ret_addr = Address::new(Rc::clone(addr.get_space().unwrap()), base + 15);
    let mut ret = TextEmit::default();
    assert_eq!(sleigh.print_assembly(&mut ret, &ret_addr).unwrap(), 1);
    assert_eq!(ret.text.map(|(m, _)| m).as_deref(), Some("RET"));
}

#[test]
fn x86_64_over_length_encoding_still_rejected() {
    let base: u64 = 0x1000;
    // Sixteen 0x66 prefixes push the opcode byte itself to offset 16, so the
    // read's *start* is past the buffer -- still a hard decode error.
    let mut bytes = vec![0x66; 16];
    bytes.push(0x90);

    let Some((sleigh, addr)) = x86_64_sleigh(base, bytes) else { return };

    let mut text = TextEmit::default();
    let err = sleigh
        .print_assembly(&mut text, &addr)
        .expect_err("an encoding longer than 16 bytes must not decode");
    assert!(
        err.explain().contains("more than 16 bytes"),
        "expected the instruction-length guard, got: {}",
        err.explain()
    );
}
