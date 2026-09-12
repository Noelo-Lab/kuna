use std::{path::PathBuf, rc::Rc};
use kuna_base::{address::Address, error::KunaResult};
use kuna_base::space::RegisterLookup;
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{globalcontext::ContextInternal, loadimage::LoadImage, sleigh::Sleigh, translate::{PcodeEmit, Translate}};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str { "synthetic-pop" }
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

#[test]
fn only_the_single_register_stack_pop_is_a_return() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for profile in ["ARM4", "ARM4t", "ARM5", "ARM5t", "ARM6", "ARM7", "ARM8"] {
        for endian in ["le", "be"] {
            let sla = std::fs::read(root.join(format!("specs/Ghidra/Processors/ARM/data/languages/{profile}_{endian}.sla"))).unwrap();
            for (word, lrset, returns) in [
                (0xe49df004u32, 0, true),
                (0x149df004, 0, true),
                (0xe490f004, 0, false),
                (0xe59df004, 0, false),
                (0xe49df008, 0, false),
                (0xe49df004, 1, false),
            ] {
                let bytes = if endian == "le" { word.to_le_bytes() } else { word.to_be_bytes() };
                let mut sleigh = Sleigh::new(Box::new(Bytes(Vec::new())), Box::new(ContextInternal::new()));
                sleigh.initialize_from_sla(&sla).unwrap();
                sleigh.set_context_default("TMode", 0);
                sleigh.set_context_default("LRset", lrset);
                sleigh.set_loader(Box::new(Bytes(bytes.to_vec())));
                let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
                let mut ops = Ops::default();
                assert_eq!(sleigh.one_instruction(&mut ops, &Address::new(ram, 0x1000)).unwrap(), 4);
                assert_eq!(ops.0.iter().any(|(op, _, _)| *op == OpCode::CPUI_RETURN), returns, "{word:x}, {profile}_{endian}, LRset={lrset}");
                if returns {
                    let sp = sleigh.get_register("sp").unwrap();
                    assert!(ops.0.iter().any(|(op, out, ins)| *op == OpCode::CPUI_INT_ADD
                        && out.as_ref().is_some_and(|v| v.offset == sp.offset && v.size == sp.size
                            && v.space.as_ref().map(|s| s.get_index()) == sp.space.as_ref().map(|s| s.get_index()))
                        && ins.iter().any(|v| v.space.as_ref().is_some_and(|s| s.get_name() == "const") && v.offset == 4)));
                    assert!(!ops.0.iter().any(|(op, _, _)| matches!(op, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)));
                }
            }
        }
    }
}
