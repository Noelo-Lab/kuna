//! x86-64 VMX decode and flag lift (kuna #1628): VMREAD/VMWRITE decode without
//! REX.W, every VMX instruction writes CF and ZF from its status, and VMPTRST
//! stores the VMCS pointer.

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
        "synthetic-x86-vmx"
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

type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

#[derive(Default)]
struct Ops(Vec<Op>);

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
    ops: Vec<Op>,
    userops: Vec<String>,
    cf: VarnodeData,
    zf: VarnodeData,
}

fn decode(longmode: bool, bytes: &[u8]) -> Decoded {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sla = if longmode { "x86-64.sla" } else { "x86.sla" };
    let sla = std::fs::read(root.join("specs/Ghidra/Processors/x86/data/languages").join(sla)).unwrap();
    let mut sleigh = Sleigh::new(Box::new(Bytes(bytes.to_vec())), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&sla).unwrap();
    sleigh.set_context_default("addrsize", if longmode { 2 } else { 1 });
    sleigh.set_context_default("opsize", 1);
    if longmode {
        sleigh.set_context_default("longMode", 1);
        sleigh.set_context_default("bit64", 1);
    }
    let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, BASE);
    let mut ops = Ops::default();
    let mut text = Text::default();
    let length = sleigh.one_instruction_with_assembly(&mut ops, &mut text, &addr).unwrap();
    let mut userops = Vec::new();
    sleigh.get_user_op_names(&mut userops);
    let cf = sleigh.get_register_varnode(b"CF").unwrap();
    let zf = sleigh.get_register_varnode(b"ZF").unwrap();
    Decoded { length, text: text.0, ops: ops.0, userops, cf, zf }
}

fn callothers(d: &Decoded) -> Vec<(&str, Option<u32>)> {
    d.ops
        .iter()
        .filter(|(op, _, _)| *op == OpCode::CPUI_CALLOTHER)
        .map(|(_, out, ins)| (d.userops[ins[0].offset as usize].as_str(), out.as_ref().map(|v| v.size)))
        .collect()
}

/// CF and ZF are each written exactly once, after the last user-op, and the
/// status that feeds them is the output of `userop`.
fn assert_status_flags(d: &Decoded, userop: &str) {
    let last_call = d.ops.iter().rposition(|(op, _, _)| *op == OpCode::CPUI_CALLOTHER).unwrap();
    let status = d
        .ops
        .iter()
        .find(|(op, _, ins)| *op == OpCode::CPUI_CALLOTHER && d.userops[ins[0].offset as usize] == userop)
        .and_then(|(_, out, _)| out.clone())
        .unwrap_or_else(|| panic!("{userop} returns a status: {:?}", d.ops));
    assert_eq!(status.size, 4, "{userop} status is a 4-byte value");
    for (flag, name, value) in [(&d.cf, "CF", 2u64), (&d.zf, "ZF", 1u64)] {
        let writes: Vec<usize> = d
            .ops
            .iter()
            .enumerate()
            .filter(|(_, (_, out, _))| out.as_ref() == Some(flag))
            .map(|(i, _)| i)
            .collect();
        assert_eq!(writes.len(), 1, "{name} written once by {userop}: {:?}", d.ops);
        assert!(writes[0] > last_call, "{name} is written after the user-op");
        let (op, _, ins) = &d.ops[writes[0]];
        assert_eq!(*op, OpCode::CPUI_INT_EQUAL, "{name} is a status compare: {:?}", d.ops);
        assert_eq!(ins[0], status, "{name} compares the {userop} status");
        assert_eq!(ins[1].offset, value, "{name} is status == {value}");
    }
}

#[test]
fn vmread_and_vmwrite_decode_without_rex_w_in_64_bit_mode() {
    for (bytes, text) in [
        (&[0x0f, 0x78, 0xf9][..], "VMREAD RCX, RDI"),
        (&[0x48, 0x0f, 0x78, 0xf9][..], "VMREAD RCX, RDI"),
        (&[0x0f, 0x78, 0x07][..], "VMREAD qword ptr [RDI], RAX"),
        (&[0x0f, 0x79, 0xfe][..], "VMWRITE RDI, RSI"),
        (&[0x48, 0x0f, 0x79, 0xfe][..], "VMWRITE RDI, RSI"),
        (&[0x0f, 0x79, 0x07][..], "VMWRITE RAX, qword ptr [RDI]"),
    ] {
        let d = decode(true, bytes);
        assert_eq!(d.length as usize, bytes.len(), "{bytes:02x?}");
        assert_eq!(d.text, text, "{bytes:02x?}");
    }
    let d = decode(false, &[0x0f, 0x78, 0xf9]);
    assert_eq!(d.text, "VMREAD ECX, EDI");
    assert_eq!(decode(false, &[0x0f, 0x79, 0xfe]).text, "VMWRITE EDI, ESI");
}

#[test]
fn vmread_keeps_its_value_and_takes_the_status_from_vmx_status() {
    for longmode in [true, false] {
        let d = decode(longmode, &[0x0f, 0x78, 0xf9]);
        assert_eq!(callothers(&d), [("vmread", Some(if longmode { 8 } else { 4 })), ("vmx_status", Some(4))]);
        assert_status_flags(&d, "vmx_status");
    }
}

#[test]
fn vmx_instructions_set_cf_and_zf_from_their_status() {
    let cases: [(&[u8], &str, &str); 9] = [
        (&[0x0f, 0x01, 0xc2], "VMLAUNCH", "vmlaunch"),
        (&[0x0f, 0x01, 0xc3], "VMRESUME", "vmresume"),
        (&[0x0f, 0x01, 0xc4], "VMXOFF", "vmxoff"),
        (&[0x0f, 0x79, 0xfe], "VMWRITE RDI, RSI", "vmwrite"),
        (&[0x66, 0x0f, 0xc7, 0x37], "VMCLEAR qword ptr [RDI]", "vmclear"),
        (&[0x0f, 0xc7, 0x37], "VMPTRLD qword ptr [RDI]", "vmptrld"),
        (&[0xf3, 0x0f, 0xc7, 0x37], "VMXON qword ptr [RDI]", "vmxon"),
        (&[0x66, 0x0f, 0x38, 0x80, 0x07], "INVEPT RAX, xmmword ptr [RDI]", "invept"),
        (&[0x66, 0x0f, 0x38, 0x81, 0x07], "INVVPID RAX, xmmword ptr [RDI]", "invvpid"),
    ];
    for (bytes, text, userop) in cases {
        for longmode in [true, false] {
            if !longmode && userop.starts_with("inv") {
                continue;
            }
            let d = decode(longmode, bytes);
            assert_eq!(d.length as usize, bytes.len(), "{bytes:02x?}");
            if longmode {
                assert_eq!(d.text, text);
            }
            assert_eq!(callothers(&d).len(), 1, "{userop}: {:?}", d.ops);
            assert_status_flags(&d, userop);
        }
    }
}

#[test]
fn vmptrst_stores_the_vmcs_pointer_and_succeeds() {
    let d = decode(true, &[0x0f, 0xc7, 0x3f]);
    assert_eq!(d.text, "VMPTRST qword ptr [RDI]");
    assert_eq!(callothers(&d), [("vmptrst", Some(8))]);
    let store = d.ops.iter().find(|(op, _, _)| *op == OpCode::CPUI_STORE).expect("VMPTRST stores");
    let (_, pointer, _) = &d.ops[0];
    assert_eq!(store.2[2], pointer.clone().unwrap(), "the stored value is the user-op result: {:?}", d.ops);
    assert!(d.ops.iter().all(|(op, _, _)| *op != OpCode::CPUI_LOAD), "VMPTRST loads nothing: {:?}", d.ops);
    for flag in [&d.cf, &d.zf] {
        let (op, _, ins) = d.ops.iter().find(|(_, out, _)| out.as_ref() == Some(flag)).unwrap();
        assert_eq!(*op, OpCode::CPUI_COPY);
        assert_eq!(ins[0].offset, 0, "VMPTRST always succeeds");
    }
}
