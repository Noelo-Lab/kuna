//! x87 integer conversions preserve width, extended precision, and control-word input.
use kuna_base::{address::Address, error::KunaResult};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    sleigh::Sleigh,
    translate::{PcodeEmit, Translate},
};
use std::{path::PathBuf, rc::Rc};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-x87"
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
struct Ops(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);
impl PcodeEmit for Ops {
    fn dump(&mut self, _: &Address, code: OpCode, out: Option<&VarnodeData>, ins: &[VarnodeData]) {
        self.0.push((code, out.cloned(), ins.to_vec()));
    }
}

#[test]
fn fist_conversions_retain_control_word_and_pop_only_when_requested() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for (spec, mode) in [("x86", 1), ("x86-64", 2)] {
        let sla = std::fs::read(root.join(format!(
            "specs/Ghidra/Processors/x86/data/languages/{spec}.sla"
        )))
        .unwrap();
        for (bytes, width, pop) in [
            ([0xdf, 0x10], 2, false),
            ([0xdb, 0x10], 4, false),
            ([0xdf, 0x08], 2, true),
            ([0xdb, 0x08], 4, true),
            ([0xdd, 0x08], 8, true),
            ([0xdf, 0x18], 2, true),
            ([0xdb, 0x18], 4, true),
            ([0xdf, 0x38], 8, true),
        ] {
            let mut sleigh = Sleigh::new(
                Box::new(Bytes(bytes.to_vec())),
                Box::new(ContextInternal::new()),
            );
            sleigh.initialize_from_sla(&sla).unwrap();
            sleigh.set_context_default("addrsize", mode);
            sleigh.set_context_default("opsize", 1);
            if mode == 2 {
                sleigh.set_context_default("longMode", 1);
            }
            let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
            let mut ops = Ops::default();
            sleigh
                .one_instruction(&mut ops, &Address::new(ram, 0x1000))
                .unwrap();
            let conversions: Vec<_> = ops
                .0
                .iter()
                .enumerate()
                .filter(|(_, (op, _, _))| *op == OpCode::CPUI_CALLOTHER)
                .collect();
            if bytes[1] == 0x08 {
                assert!(conversions.is_empty());
                assert!(ops
                    .0
                    .iter()
                    .any(|(op, _, _)| *op == OpCode::CPUI_FLOAT_TRUNC));
                assert!(!ops
                    .0
                    .iter()
                    .any(|(op, _, _)| *op == OpCode::CPUI_FLOAT_ROUND));
                continue;
            }
            assert_eq!(conversions.len(), 1);
            let (index, (_, out, inputs)) = conversions[0];
            let mut names = Vec::new();
            sleigh.get_user_op_names(&mut names);
            assert_eq!(
                names[inputs[0].offset as usize],
                format!("x87_float_to_int{}", width * 8)
            );
            assert_eq!(out.as_ref().unwrap().size, width);
            assert_eq!(inputs[1], sleigh.get_register_varnode(b"ST0").unwrap());
            assert_eq!(inputs[1].size, 10);
            assert_eq!(
                inputs[2],
                sleigh.get_register_varnode(b"FPUControlWord").unwrap()
            );
            let st0 = sleigh.get_register_varnode(b"ST0").unwrap();
            let pop_index = ops
                .0
                .iter()
                .position(|(_, out, _)| out.as_ref() == Some(&st0));
            assert_eq!(pop_index.is_some(), pop);
            if let Some(pop_index) = pop_index {
                assert!(index < pop_index);
            }
            assert!(!ops.0.iter().any(|(op, _, _)| matches!(
                op,
                OpCode::CPUI_FLOAT_ROUND | OpCode::CPUI_FLOAT_TRUNC
            )));
            let store = ops
                .0
                .iter()
                .find(|(op, _, _)| *op == OpCode::CPUI_STORE)
                .unwrap();
            assert_eq!(store.2[2].size, width);
            let mut value = store.2[2].clone();
            for (_, next, inputs) in ops.0
                [..ops.0.iter().position(|op| std::ptr::eq(op, store)).unwrap()]
                .iter()
                .rev()
            {
                if next.as_ref() == Some(&value) && inputs.len() == 1 {
                    value = inputs[0].clone();
                }
            }
            assert_eq!(value, *out.as_ref().unwrap());
        }
    }
}
