//! Execute BOUND p-code over signed boundary cases.
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
        "synthetic-bound"
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

fn run_bound(bits: u32, value: i64, lower: i64, upper: i64) -> bool {
    use std::collections::HashMap;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let sla =
        std::fs::read(root.join("specs/Ghidra/Processors/x86/data/languages/x86.sla")).unwrap();
    let bytes = if bits == 16 {
        vec![0x66, 0x62, 0x03]
    } else {
        vec![0x62, 0x03]
    };
    let mut sleigh = Sleigh::new(
        Box::new(Bytes(bytes.clone())),
        Box::new(ContextInternal::new()),
    );
    sleigh.initialize_from_sla(&sla).unwrap();
    sleigh.set_context_default("addrsize", 1);
    sleigh.set_context_default("opsize", 1);
    let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
    let mut ops = Ops::default();
    assert_eq!(
        sleigh
            .one_instruction(&mut ops, &Address::new(ram, 0x1000))
            .unwrap(),
        bytes.len() as i32
    );
    assert!(!ops.0.is_empty(), "BOUND must not be a NOP");
    let loads: Vec<_> = ops
        .0
        .iter()
        .filter(|(op, _, _)| *op == OpCode::CPUI_LOAD)
        .collect();
    assert_eq!(loads.len(), 2, "{:?}", ops.0);
    assert!(loads
        .iter()
        .all(|(_, out, _)| out.as_ref().unwrap().size == bits / 8));
    let mut userops = Vec::new();
    sleigh.get_user_op_names(&mut userops);
    let fault = ops
        .0
        .iter()
        .find(|(op, _, _)| *op == OpCode::CPUI_CALLOTHER)
        .unwrap();
    assert_eq!(
        userops[fault.2[0].offset as usize],
        format!("x86_bound_check{bits}")
    );
    assert!(fault.1.is_none());
    assert_eq!(fault.2.len(), 4);
    assert!(fault.2[1..].iter().all(|v| v.size == bits / 8));
    assert!(!ops
        .0
        .iter()
        .any(|(op, _, _)| matches!(op, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)));
    let key = |v: &VarnodeData| (v.space.as_ref().unwrap().get_index(), v.offset);
    let mut state = HashMap::new();
    state.insert(
        key(&sleigh.get_register_varnode(b"EBX").unwrap()),
        0x2000u64,
    );
    state.insert(
        key(&sleigh
            .get_register_varnode(if bits == 16 { b"AX" } else { b"EAX" })
            .unwrap()),
        value as u64,
    );
    let read = |v: &VarnodeData, state: &HashMap<_, u64>| {
        let raw = if v.space.as_ref().unwrap().get_name() == "const" {
            v.offset
        } else {
            *state.get(&key(v)).unwrap_or(&0)
        };
        if v.size < 8 {
            raw & ((1u64 << (v.size * 8)) - 1)
        } else {
            raw
        }
    };
    let signed = |value: u64, size: u32| ((value << (64 - size * 8)) as i64) >> (64 - size * 8);
    let mut pc = 0i64;
    let mut steps = 0;
    while let Some((op, out, ins)) = ops.0.get(pc as usize) {
        steps += 1;
        assert!(steps < 100, "unexpected loop");
        let a = ins.first().map(|v| read(v, &state)).unwrap_or(0);
        let b = ins.get(1).map(|v| read(v, &state)).unwrap_or(0);
        let result = match op {
            OpCode::CPUI_COPY => a,
            OpCode::CPUI_INT_ADD => a.wrapping_add(b),
            OpCode::CPUI_LOAD => {
                if b == 0x2000 {
                    lower as u64
                } else {
                    assert_eq!(b, 0x2000 + u64::from(bits / 8));
                    upper as u64
                }
            }
            OpCode::CPUI_INT_SLESS => u64::from(signed(a, ins[0].size) < signed(b, ins[1].size)),
            OpCode::CPUI_INT_SLESSEQUAL => {
                u64::from(signed(a, ins[0].size) <= signed(b, ins[1].size))
            }
            OpCode::CPUI_BOOL_AND => a & b,
            OpCode::CPUI_BOOL_OR => a | b,
            OpCode::CPUI_BOOL_NEGATE => u64::from(a == 0),
            OpCode::CPUI_CBRANCH => {
                if b != 0 {
                    pc += a as i64;
                    continue;
                }
                0
            }
            OpCode::CPUI_CALLOTHER => {
                let index = signed(read(&ins[1], &state), ins[1].size);
                let low = signed(read(&ins[2], &state), ins[2].size);
                let high = signed(read(&ins[3], &state), ins[3].size);
                return index < low || index > high;
            }
            _ => panic!("unexpected p-code: {op:?} {ins:?}"),
        };
        if let Some(out) = out {
            state.insert(key(out), result);
        }
        pc += 1;
    }
    false
}

#[test]
fn bound_checks_signed_inclusive_limits_at_both_widths() {
    for bits in [16, 32] {
        let min = -(1i64 << (bits - 1));
        let max = (1i64 << (bits - 1)) - 1;
        for (value, low, high, traps) in [
            (min, min, max, false),
            (max, min, max, false),
            (min, min + 1, max, true),
            (max, min, max - 1, true),
            (-4, -3, 7, true),
            (-3, -3, 7, false),
            (0, -3, 7, false),
            (7, -3, 7, false),
            (8, -3, 7, true),
            (-1, 0, 0, true),
            (0, 0, 0, false),
            (3, 7, -3, true),
        ] {
            assert_eq!(
                run_bound(bits, value, low, high),
                traps,
                "{bits}: {value} in {low}..={high}"
            );
        }
    }
}
