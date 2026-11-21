//! ARCv2 decode vectors: every instruction class of the base EM/HS set, as
//! `arc-zephyr-elf-as -mcpu=em4_fpuda` assembled it and as that toolchain's
//! disassembler reads it back (operands compared by value, branch targets as
//! absolute addresses). `arcv2_decode.tsv` holds `address, bytes, text`.
//!
//! The zero-overhead loop is pinned at the p-code level: `lp` marks its end
//! address in the decode context, and the instruction there branches back.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};

struct MemImg {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImg {
    fn get_file_name(&self) -> &str {
        "arc"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let start = addr.get_offset();
        for (i, b) in ptr.iter_mut().enumerate() {
            *b = start
                .wrapping_add(i as u64)
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

#[derive(Default)]
struct Text(String);

impl AssemblyEmit for Text {
    fn dump(&mut self, _addr: &Address, mnemonic: &str, body: &str) {
        self.0 = if body.is_empty() { mnemonic.to_string() } else { format!("{mnemonic} {body}") };
    }
}

#[derive(Default)]
struct Ops(Vec<(OpCode, Vec<VarnodeData>)>);

impl PcodeEmit for Ops {
    fn dump(&mut self, _addr: &Address, opc: OpCode, _out: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.0.push((opc, vars.to_vec()));
    }
}

fn arc(sla: &str, base: u64, bytes: Vec<u8>) -> Sleigh {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs/Ghidra/Processors/ARC/data/languages")
        .join(sla);
    let sla = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut sleigh = Sleigh::new(Box::new(MemImg { base, bytes }), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&sla).expect("initialize the ARCv2 language");
    sleigh
}

fn at(sleigh: &Sleigh, offset: u64) -> Address {
    Address::new(Rc::clone(sleigh.base().manager().get_space_by_name("ram").expect("ram")), offset)
}

fn hex(text: &str) -> Vec<u8> {
    (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap()).collect()
}

fn decodes(sla: &str, address: u64, bytes: &str, want: &str) {
    let bytes = hex(bytes);
    let len = bytes.len() as i32;
    let mut sleigh = arc(sla, address, bytes);
    let mut text = Text::default();
    let got = sleigh.print_assembly(&mut text, &at(&sleigh, address));
    assert_eq!((got.ok(), text.0.as_str()), (Some(len), want), "at {address:#x}");
}

#[test]
fn little_endian_vectors_decode_as_assembled() {
    let vectors = include_str!("arcv2_decode.tsv");
    let mut count = 0;
    for line in vectors.lines() {
        let mut fields = line.split('\t');
        let (Some(address), Some(bytes), Some(want)) = (fields.next(), fields.next(), fields.next()) else {
            panic!("malformed vector {line:?}");
        };
        let address = u64::from_str_radix(address.trim_start_matches("0x"), 16).unwrap();
        decodes("arcv2_le.sla", address, bytes, want);
        count += 1;
    }
    assert_eq!(count, 508);
}

#[test]
fn big_endian_words_share_the_patterns() {
    for (address, bytes, want) in [
    (0x10014, "21001f8812345678", "add r8, r9, 0x12345678"),
    (0x10d58, "16040205", "ld.aw r5, [r6, 0x4]"),
    (0x10ef0, "19000004", "sth r0, [r1]"),
    (0x11296, "41c312345678", "mov_s r1, 0x12345678"),
    ] {
        decodes("arcv2_be.sla", address, bytes, want);
    }
}

/// `mov lp_count,r2 ; lp 0x102a ; ld.aw r12,[r3,4] ; add_s r0,r0,r12 ; j_s [blink]`
#[test]
fn loop_end_instruction_branches_back_to_the_loop_start() {
    let base = 0x101c;
    let mut sleigh = arc("arcv2_le.sla", base, hex("0a248070a820400104130c029860e07e"));
    let mut lp = Ops::default();
    assert_eq!(sleigh.one_instruction(&mut lp, &at(&sleigh, 0x1020)).unwrap(), 4);
    assert!(lp.0.iter().all(|(opc, _)| *opc == OpCode::CPUI_COPY), "lp only records the loop bounds");

    let mut end = Ops::default();
    assert_eq!(sleigh.one_instruction(&mut end, &at(&sleigh, 0x102a)).unwrap(), 2);
    let opcodes: Vec<OpCode> = end.0.iter().map(|(opc, _)| *opc).collect();
    assert_eq!(
        opcodes,
        [OpCode::CPUI_INT_SUB, OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_CBRANCH, OpCode::CPUI_RETURN]
    );
    assert_eq!(end.0[2].1[0].offset, 0x1024, "the back-edge targets the first body instruction");

    let mut body = Ops::default();
    assert_eq!(sleigh.one_instruction(&mut body, &at(&sleigh, 0x1028)).unwrap(), 2);
    assert_eq!(body.0.len(), 1, "an instruction inside the body carries no loop test");
}
