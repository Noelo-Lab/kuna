//! Floating-to-integer conversions use integer destination widths and encodings.
use std::{path::PathBuf, rc::Rc};
use kuna_base::{address::Address, error::KunaResult, space::RegisterLookup};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{globalcontext::ContextInternal, loadimage::LoadImage, sleigh::Sleigh, translate::PcodeEmit};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str { "synthetic-sparc-conversion" }
    fn get_arch_type(&self) -> Vec<u8> { Vec::new() }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr.get_offset().checked_sub(0x1000)
                .and_then(|offset| self.0.get(offset as usize + i)).copied().unwrap_or(0);
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

fn check_conversions(cases: &[(&str, u32, u32, u32, &str)]) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for bits in [32, 64] {
        let sla = std::fs::read(root.join(format!("specs/Ghidra/Processors/Sparc/data/languages/SparcV9_{bits}.sla"))).unwrap();
        for &(assembly, word, input_size, output_size, destination) in cases {
            let mut sleigh = Sleigh::new(Box::new(Bytes(word.to_be_bytes().to_vec())), Box::new(ContextInternal::new()));
            sleigh.initialize_from_sla(&sla).unwrap();
            let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
            let mut ops = Ops::default();
            assert_eq!(sleigh.one_instruction(&mut ops, &Address::new(ram, 0x1000)).unwrap(), 4);
            let conversions: Vec<_> = ops.0.iter().filter(|(op, _, _)| *op == OpCode::CPUI_FLOAT_TRUNC).collect();
            assert_eq!(conversions.len(), 1, "{assembly}, SPARC{bits}");
            let (_, output, inputs) = conversions[0];
            let output = output.as_ref().unwrap();
            let register = sleigh.get_register(destination).unwrap();
            assert_eq!(inputs[0].size, input_size, "{assembly}, SPARC{bits}");
            assert_eq!(output.size, output_size, "{assembly}, SPARC{bits}");
            assert_eq!(output.offset, register.offset, "{assembly}, SPARC{bits}");
            assert_eq!(output.space.as_ref().unwrap().get_index(), register.space.as_ref().unwrap().get_index());
        }
    }
}

#[test]
fn fdtoi_preserves_the_scalar_destination() {
    check_conversions(&[("fdtoi %f2, %f3", 0x87a01a42, 8, 4, "fs3")]);
}

#[test]
fn fqtoi_preserves_the_scalar_destination() {
    check_conversions(&[
        ("fqtoi %f0, %f4", 0x89a01a60, 16, 4, "fs4"),
        ("fqtoi %f0, %f3", 0x87a01a60, 16, 4, "fs3"),
    ]);
}

#[test]
fn fstox_uses_a_double_destination_and_the_upper_bank_encoding() {
    check_conversions(&[
        ("fstox %f2, %f4", 0x89a01022, 4, 8, "fd4"),
        ("fstox %f2, %f32", 0x83a01022, 4, 8, "fd32"),
    ]);
}
