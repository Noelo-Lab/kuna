//! Synthetic `lwarx` encoding, checked for the reservation that `stwcx.` tests.
use kuna_base::space::RegisterLookup;
use kuna_base::{address::Address, error::KunaResult};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    sleigh::Sleigh,
    translate::{AssemblyEmit, PcodeEmit},
};
use std::{path::PathBuf, rc::Rc};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-lwarx"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr
                .get_offset()
                .checked_sub(0x1000)
                .and_then(|offset| self.0.get(offset as usize + i))
                .copied()
                .unwrap_or(0);
        }
        Ok(())
    }
}

#[derive(Default)]
struct Ops(Vec<(OpCode, Option<VarnodeData>)>);
impl PcodeEmit for Ops {
    fn dump(&mut self, _: &Address, code: OpCode, out: Option<&VarnodeData>, _: &[VarnodeData]) {
        self.0.push((code, out.cloned()));
    }
}

#[derive(Default)]
struct Mnemonic(String);
impl AssemblyEmit for Mnemonic {
    fn dump(&mut self, _: &Address, mnemonic: &str, _: &str) {
        self.0 = mnemonic.to_string();
    }
}

#[test]
fn lwarx_sets_the_reservation_tested_by_stwcx() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for spec in [
        "ppc_32_be",
        "ppc_32_le",
        "ppc_32_e500_be",
        "ppc_64_be",
        "ppc_64_le",
        "ppc_64_isa_be",
        "ppc_64_isa_altivec_le",
    ] {
        let sla = std::fs::read(root.join(format!("specs/Ghidra/Processors/PowerPC/data/languages/{spec}.sla"))).unwrap();
        let word = 0x7d201828u32;
        let bytes = if spec.ends_with("_le") { word.to_le_bytes() } else { word.to_be_bytes() };
        let mut sleigh = Sleigh::new(Box::new(Bytes(Vec::new())), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).unwrap();
        sleigh.set_loader(Box::new(Bytes(bytes.to_vec())));
        let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        let mut asm = Mnemonic::default();
        assert_eq!(sleigh.print_assembly(&mut asm, &addr).unwrap(), 4, "{spec}");
        assert_eq!(asm.0, "lwarx", "{spec}");
        let mut ops = Ops::default();
        assert_eq!(sleigh.one_instruction(&mut ops, &addr).unwrap(), 4, "{spec}");
        let written = |name: &str| {
            let reg = sleigh.get_register(name).unwrap();
            ops.0.iter().any(|(_, out)| {
                out.as_ref().is_some_and(|v| {
                    v.offset == reg.offset
                        && v.size == reg.size
                        && matches!((&v.space, &reg.space), (Some(a), Some(b)) if Rc::ptr_eq(a, b))
                })
            })
        };
        for name in ["RESERVE", "RESERVE_LENGTH", "RESERVE_ADDRESS"] {
            assert!(written(name), "{spec}: {name} not written: {:?}", ops.0);
        }
        assert!(
            ops.0.iter().any(|(op, out)| *op == OpCode::CPUI_LOAD && out.as_ref().is_some_and(|v| v.size == 4)),
            "{spec}: {:?}",
            ops.0
        );
    }
}
