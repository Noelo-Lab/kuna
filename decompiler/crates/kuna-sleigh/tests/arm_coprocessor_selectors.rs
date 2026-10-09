use std::{collections::HashMap, path::PathBuf, rc::Rc};

use kuna_base::{address::Address, error::KunaResult, space::RegisterLookup};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use kuna_sleigh::{
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    sleigh::Sleigh,
    translate::{PcodeEmit, Translate},
};

struct Bytes(Vec<u8>, u64);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-coprocessor"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = self
                .0
                .get(addr.get_offset().saturating_sub(self.1) as usize + i)
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

fn key(v: &VarnodeData) -> (i32, u64, u32) {
    (v.space.as_ref().unwrap().get_index(), v.offset, v.size)
}

fn constant(v: &VarnodeData, copies: &HashMap<(i32, u64, u32), u64>) -> Option<u64> {
    if v.space.as_ref().unwrap().get_name() == "const" {
        Some(v.offset)
    } else {
        copies.get(&key(v)).copied()
    }
}

#[test]
fn generic_transfers_use_encoded_selectors_and_live_cpu_registers() {
    let mut instructions = 0;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for profile in ["ARM4", "ARM4t", "ARM5", "ARM5t", "ARM6", "ARM7", "ARM8"] {
        for endian in ["le", "be"] {
            let sla = std::fs::read(root.join(format!(
                "specs/Ghidra/Processors/ARM/data/languages/{profile}_{endian}.sla"
            )))
            .unwrap();
            let mut offset = 0;
            let mut sleigh = Sleigh::new(
                Box::new(Bytes(Vec::new(), 0)),
                Box::new(ContextInternal::new()),
            );
            sleigh.initialize_from_sla(&sla).unwrap();
            let mut names = Vec::new();
            sleigh.get_user_op_names(&mut names);
            sleigh.set_context_default("TMode", 0);
            let control = if endian == "le" {
                0xee012f10u32.to_le_bytes()
            } else {
                0xee012f10u32.to_be_bytes()
            };
            sleigh.set_loader(Box::new(Bytes(control.to_vec(), 0)));
            let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
            let mut control_ops = Ops::default();
            assert_eq!(
                sleigh
                    .one_instruction(&mut control_ops, &Address::new(ram, 0))
                    .unwrap(),
                4
            );
            let control_calls: Vec<_> = control_ops
                .0
                .iter()
                .filter(|(op, _, _)| *op == OpCode::CPUI_CALLOTHER)
                .collect();
            assert_eq!(control_calls.len(), 1);
            let (_, _, inputs) = control_calls[0];
            assert_eq!(names[inputs[0].offset as usize], "coproc_moveto_Control");
            assert_eq!(inputs.len(), 2);
            let r2 = sleigh.get_register("r2").unwrap();
            assert_eq!(
                key(&inputs[1]),
                (r2.space.as_ref().unwrap().get_index(), r2.offset, r2.size)
            );
            for thumb in [false, true] {
                if thumb && !["ARM6", "ARM7", "ARM8"].contains(&profile) {
                    continue;
                }
                sleigh.set_context_default("TMode", u32::from(thumb));
                for (cp, op1, n, m, op2, rt) in [
                    (15, 0, 6, 0, 0, 0),
                    (15, 3, 6, 5, 2, 2),
                    (7, 5, 11, 4, 6, 3),
                    (7, 7, 15, 15, 7, 4),
                    (7, 0, 0, 0, 0, 5),
                    (7, 5, 11, 4, 6, 15),
                ] {
                    for read in [false, true] {
                        if rt == 15 && (!thumb || !read) {
                            continue;
                        }
                        for unconditional in [false, true] {
                            if unconditional && (profile.starts_with("ARM4") || rt == 15) {
                                continue;
                            }
                            for conditional in [false, true] {
                                if conditional && (thumb || unconditional) {
                                    continue;
                                }
                                let cond = if unconditional {
                                    15
                                } else if conditional {
                                    1
                                } else {
                                    14
                                };
                                let word: u32 = (cond << 28)
                                    | 0x0e000010
                                    | (op1 << 21)
                                    | (u32::from(read) << 20)
                                    | (n << 16)
                                    | (rt << 12)
                                    | (cp << 8)
                                    | (op2 << 5)
                                    | m;
                                let bytes = if thumb {
                                    let halves = [(word >> 16) as u16, word as u16];
                                    halves
                                        .into_iter()
                                        .flat_map(|h| {
                                            if endian == "le" {
                                                h.to_le_bytes()
                                            } else {
                                                h.to_be_bytes()
                                            }
                                        })
                                        .collect()
                                } else if endian == "le" {
                                    word.to_le_bytes().to_vec()
                                } else {
                                    word.to_be_bytes().to_vec()
                                };
                                offset += 4;
                                sleigh.set_loader(Box::new(Bytes(bytes, offset)));
                                let ram = Rc::clone(
                                    sleigh.base().manager().get_space_by_name("ram").unwrap(),
                                );
                                let mut ops = Ops::default();
                                assert_eq!(
                                    sleigh
                                        .one_instruction(&mut ops, &Address::new(ram, offset))
                                        .unwrap(),
                                    4
                                );
                                instructions += 1;
                                let label = format!(
                                    "{profile}_{endian}, TMode={}, {word:08x}",
                                    u8::from(thumb)
                                );
                                let mut copies = HashMap::new();
                                let mut calls = 0;
                                for (code, out, ins) in &ops.0 {
                                    if *code == OpCode::CPUI_COPY {
                                        if let Some(out) = out {
                                            copies.remove(&key(out));
                                            if let Some(value) = constant(&ins[0], &copies) {
                                                copies.insert(key(out), value);
                                            }
                                        }
                                    }
                                    if *code != OpCode::CPUI_CALLOTHER {
                                        continue;
                                    }
                                    let name = &names[ins[0].offset as usize];
                                    if !["coprocessor_moveto", "coprocessor_movefromRt"]
                                        .contains(&name.as_str())
                                    {
                                        continue;
                                    }
                                    calls += 1;
                                    let selectors = if read { &ins[4..6] } else { &ins[5..7] };
                                    for (v, expected) in
                                        ins[1..4].iter().chain(selectors).zip([cp, op1, op2, n, m])
                                    {
                                        assert_eq!(
                                            constant(v, &copies),
                                            Some(u64::from(expected)),
                                            "{label}: {name}, {v:?}"
                                        );
                                        assert_eq!(v.size, 4, "{label}");
                                    }
                                    if rt == 15 {
                                        for flag in ["NG", "ZR", "CY", "OV"] {
                                            let reg = sleigh.get_register(flag).unwrap();
                                            assert!(
                                                ops.0.iter().any(|(_, out, _)| out
                                                    .as_ref()
                                                    .is_some_and(|v| key(v)
                                                        == (
                                                            reg.space.as_ref().unwrap().get_index(),
                                                            reg.offset,
                                                            reg.size
                                                        ))),
                                                "{label}: {flag}"
                                            );
                                        }
                                        continue;
                                    }
                                    let reg = sleigh.get_register(&format!("r{rt}")).unwrap();
                                    let data = if read { out.as_ref().unwrap() } else { &ins[4] };
                                    assert_eq!(
                                        key(data),
                                        (
                                            reg.space.as_ref().unwrap().get_index(),
                                            reg.offset,
                                            reg.size
                                        ),
                                        "{label}: CPU data operand"
                                    );
                                }
                                assert_eq!(calls, 1, "{label}");
                                assert_eq!(
                                    ops.0
                                        .iter()
                                        .filter(|(op, _, _)| *op == OpCode::CPUI_CBRANCH)
                                        .count(),
                                    usize::from(conditional),
                                    "{label}: condition guard"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(instructions, 506);
}
