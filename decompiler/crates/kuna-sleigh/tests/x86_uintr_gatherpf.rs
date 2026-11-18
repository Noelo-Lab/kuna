//! x86-64 decode of the UINTR group, PREFETCHIT0/1 and the AVX-512PF gather
//! prefetches (kuna #1627): `f3 0f c7 /6` is SENDUIPI, not RDRAND; the
//! RIP-relative `0f 18 /7`, `/6` are prefetches, not reserved NOPs; and
//! VGATHERPF* index with a ZMM register and load nothing.

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

const BASE: u64 = 0x1000;

struct Bytes(Vec<u8>);

impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-x86-1627"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr
                .get_offset()
                .checked_sub(BASE)
                .and_then(|offset| self.0.get(offset as usize + i))
                .copied()
                .unwrap_or(0);
        }
        Ok(())
    }
}

#[derive(Default)]
struct Ops(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Ops {
    fn dump(&mut self, _: &Address, code: OpCode, out: Option<&VarnodeData>, ins: &[VarnodeData]) {
        self.0.push((code, out.cloned(), ins.to_vec()));
    }
}

#[derive(Default)]
struct Text(String);

impl AssemblyEmit for Text {
    fn dump(&mut self, _: &Address, mnemonic: &str, body: &str) {
        self.0 = format!("{mnemonic} {}", body.trim()).trim_end().to_string();
    }
}

struct Decoded {
    length: i32,
    text: String,
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
    userops: Vec<String>,
}

fn decode(bytes: &[u8]) -> Decoded {
    decode_in(true, bytes)
}

fn decode_in(longmode: bool, bytes: &[u8]) -> Decoded {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sla = if longmode { "x86-64.sla" } else { "x86.sla" };
    let sla = std::fs::read(root.join("specs/Ghidra/Processors/x86/data/languages").join(sla)).unwrap();
    let mut sleigh = Sleigh::new(Box::new(Bytes(bytes.to_vec())), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&sla).unwrap();
    sleigh.set_context_default("addrsize", if longmode { 2 } else { 1 });
    sleigh.set_context_default("opsize", 1);
    if longmode {
        sleigh.set_context_default("longMode", 1);
    }
    let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, BASE);
    let mut ops = Ops::default();
    let mut text = Text::default();
    let length = sleigh.one_instruction_with_assembly(&mut ops, &mut text, &addr).unwrap();
    let mut userops = Vec::new();
    sleigh.get_user_op_names(&mut userops);
    Decoded { length, text: text.0, ops: ops.0, userops }
}

fn callother_names(d: &Decoded) -> Vec<&str> {
    d.ops
        .iter()
        .filter(|(op, _, _)| *op == OpCode::CPUI_CALLOTHER)
        .map(|(_, _, ins)| d.userops[ins[0].offset as usize].as_str())
        .collect()
}

fn loads(d: &Decoded) -> usize {
    d.ops.iter().filter(|(op, _, _)| *op == OpCode::CPUI_LOAD).count()
}

#[test]
fn senduipi_is_not_rdrand() {
    for bytes in [
        &[0xf3, 0x0f, 0xc7, 0xf7][..],
        &[0xf3, 0x48, 0x0f, 0xc7, 0xf7][..],
        &[0x66, 0xf3, 0x0f, 0xc7, 0xf7][..],
    ] {
        let d = decode(bytes);
        assert_eq!(d.length as usize, bytes.len());
        assert_eq!(d.text, "SENDUIPI RDI", "{bytes:02x?}");
        assert_eq!(callother_names(&d), ["senduipi"], "{bytes:02x?}");
    }
    let d = decode(&[0xf3, 0x41, 0x0f, 0xc7, 0xf7]);
    assert_eq!(d.text, "SENDUIPI R15");
    let d = decode(&[0x0f, 0xc7, 0xf7]);
    assert_eq!(d.text, "RDRAND EDI");
    assert_eq!(callother_names(&d), ["rdrand", "rdrandIsValid"]);
    let d = decode(&[0x48, 0x0f, 0xc7, 0xf7]);
    assert_eq!(d.text, "RDRAND RDI");
    assert_eq!(decode(&[0x66, 0x0f, 0xc7, 0xf7]).text, "RDRAND DI");
}

#[test]
fn uintr_control_instructions_decode() {
    let d = decode(&[0xf3, 0x0f, 0x01, 0xec]);
    assert_eq!(d.text, "UIRET");
    assert_eq!(callother_names(&d), ["uiret"]);
    assert_eq!(d.ops.last().unwrap().0, OpCode::CPUI_RETURN);
    let d = decode(&[0xf3, 0x0f, 0x01, 0xed]);
    assert_eq!(d.text, "TESTUI");
    assert_eq!(callother_names(&d), ["testui"]);
    let d = decode(&[0xf3, 0x0f, 0x01, 0xee]);
    assert_eq!(d.text, "CLUI");
    assert_eq!(callother_names(&d), ["clui"]);
    let d = decode(&[0xf3, 0x0f, 0x01, 0xef]);
    assert_eq!(d.text, "STUI");
    assert_eq!(callother_names(&d), ["stui"]);
    assert_eq!(decode(&[0x0f, 0x01, 0xee]).text, "RDPKRU");
    assert_eq!(decode(&[0x0f, 0x01, 0xef]).text, "WRPKRU");
}

#[test]
fn prefetchit_is_the_rip_relative_form_only() {
    let d = decode(&[0x0f, 0x18, 0x3d, 0x40, 0x00, 0x00, 0x00]);
    assert_eq!(d.length, 7);
    assert_eq!(d.text, "PREFETCHIT0 byte ptr [0x1047]");
    assert!(d.ops.is_empty(), "{:?}", d.ops);
    let d = decode(&[0x0f, 0x18, 0x35, 0x40, 0x00, 0x00, 0x00]);
    assert_eq!(d.text, "PREFETCHIT1 byte ptr [0x1047]");
    assert_eq!(decode(&[0x0f, 0x18, 0x38]).text, "NOP/reserved dword ptr [RAX]");
    assert_eq!(decode(&[0x0f, 0x18, 0x30]).text, "NOP/reserved dword ptr [RAX]");
}

#[test]
fn gather_prefetches_index_with_a_zmm_register_and_load_nothing() {
    let cases: [(&[u8], &str, &str); 4] = [
        (
            &[0x62, 0xf2, 0x7d, 0x49, 0xc6, 0x0c, 0x88],
            "VGATHERPF0DPS zmmword ptr [RAX + ZMM1*0x4]{K1}",
            "vgatherpf0dps_avx512pf",
        ),
        (
            &[0x62, 0xf2, 0x7d, 0x49, 0xc7, 0x0c, 0x88],
            "VGATHERPF0QPS zmmword ptr [RAX + ZMM1*0x4]{K1}",
            "vgatherpf0qps_avx512pf",
        ),
        (
            &[0x62, 0xf2, 0xfd, 0x49, 0xc6, 0x14, 0x88],
            "VGATHERPF1DPD ymmword ptr [RAX + YMM1*0x4]{K1}",
            "vgatherpf1dpd_avx512pf",
        ),
        (
            &[0x62, 0xf2, 0xfd, 0x49, 0xc7, 0x14, 0x88],
            "VGATHERPF1QPD zmmword ptr [RAX + ZMM1*0x4]{K1}",
            "vgatherpf1qpd_avx512pf",
        ),
    ];
    for (bytes, text, userop) in cases {
        let d = decode(bytes);
        assert_eq!(d.length as usize, bytes.len(), "{bytes:02x?}");
        assert_eq!(d.text, text);
        assert_eq!(callother_names(&d), [userop]);
        assert_eq!(loads(&d), 0, "a prefetch loads nothing: {:?}", d.ops);
        assert!(
            d.ops.iter().any(|(op, _, ins)| *op == OpCode::CPUI_INT_MULT && ins[1].offset == 4),
            "the ZMM index is scaled into the address: {:?}",
            d.ops
        );
    }
}

#[test]
fn gather_prefetches_keep_their_vector_index_in_32_bit_mode() {
    for (bytes, text) in [
        (&[0x62, 0xf2, 0x7d, 0x49, 0xc6, 0x0c, 0x88][..], "VGATHERPF0DPS zmmword ptr [EAX + ZMM1*0x4]{K1}"),
        (&[0x62, 0xf2, 0x7d, 0x49, 0xc7, 0x0c, 0x88][..], "VGATHERPF0QPS zmmword ptr [EAX + ZMM1*0x4]{K1}"),
        (&[0x62, 0xf2, 0xfd, 0x4f, 0xc7, 0x54, 0xf9, 0x08][..], "VGATHERPF1QPD zmmword ptr [ECX + ZMM7*0x8 + 0x40]{K7}"),
    ] {
        let d = decode_in(false, bytes);
        assert_eq!(d.length as usize, bytes.len(), "{bytes:02x?}");
        assert_eq!(d.text, text);
        assert_eq!(loads(&d), 0, "a prefetch loads nothing: {:?}", d.ops);
    }
}
