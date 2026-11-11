//! SPARC branches sample their condition before a delay slot that rewrites it.
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use kuna_base::{address::Address, error::KunaResult, space::AddrSpace};
use kuna_sleigh::{
    emulate::{BreakTableCallBack, Emulate, EmulatePcodeCache},
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    memstate::{MemoryHashOverlay, MemoryState},
    sleigh::Sleigh,
    translate::Translate,
};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-sparc-delay-condition"
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

struct Machine {
    emulator: EmulatePcodeCache,
    state: Rc<RefCell<MemoryState>>,
    ram: Rc<AddrSpace>,
}

impl Machine {
    fn new(bits: u32, branch: u32, slot: u32) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let sla = std::fs::read(root.join(format!(
            "specs/Ghidra/Processors/Sparc/data/languages/SparcV9_{bits}.sla"
        )))
        .unwrap();
        let bytes: Vec<_> = [branch, slot, 0x01000000, 0x01000000, 0x01000000]
            .into_iter()
            .flat_map(u32::to_be_bytes)
            .collect();
        let mut sleigh = Sleigh::new(Box::new(Bytes(bytes)), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).unwrap();
        let manager = sleigh.base().manager_rc();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let trans: Rc<dyn Translate> = Rc::new(sleigh);
        let mut state = MemoryState::new(Rc::clone(&trans));
        for name in ["ram", "register", "unique"] {
            let space = Rc::clone(manager.get_space_by_name(name).unwrap());
            state.set_memory_bank(Rc::new(RefCell::new(MemoryHashOverlay::new(
                space, 8, 64, 64, None,
            ))));
        }
        let state = Rc::new(RefCell::new(state));
        let breaks = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
        let emulator = EmulatePcodeCache::new(trans, manager, Rc::clone(&state), breaks);
        Machine {
            emulator,
            state,
            ram,
        }
    }

    fn step(&mut self, registers: &[(&str, u64)]) -> u64 {
        {
            let mut state = self.state.borrow_mut();
            for (name, value) in registers {
                state.set_value_by_name(name, *value).unwrap();
            }
        }
        self.emulator
            .set_execute_address(&Address::new(Rc::clone(&self.ram), 0x1000))
            .unwrap();
        self.emulator.execute_instruction().unwrap();
        self.emulator.get_execute_address().get_offset()
    }

    fn read(&self, name: &str) -> u64 {
        self.state.borrow().get_value_by_name(name).unwrap()
    }
}

fn check_branch(bits: u32, predicted: bool, extended: bool, annul: bool, cond: u32, flag: &str) {
    let word = (u32::from(annul) << 29)
        | (cond << 25)
        | (if predicted { 1 << 22 } else { 2 << 22 })
        | (if extended { 2 << 20 } else { 0 })
        | 4;
    let mut machine = Machine::new(bits, word, 0x80a04002);
    for incoming in [0, 1] {
        for slot_flag in [0, 1] {
            let (left, right) = if slot_flag == 0 {
                (0, 0)
            } else if cond == 7 {
                (if extended { 1u64 << 63 } else { 1u64 << 31 }, 1)
            } else {
                (0, 1)
            };
            let mut registers: Vec<(&str, u64)> = ["i_cf", "i_nf", "i_vf", "x_cf", "x_nf", "x_vf"]
                .into_iter()
                .map(|name| (name, 0))
                .collect();
            registers.extend([(flag, incoming), ("g1", left), ("g2", right)]);
            let next = machine.step(&registers);
            let context = format!("SPARC{bits} predicted={predicted} extended={extended} annul={annul} {flag}={incoming} slot={slot_flag}");
            assert_eq!(
                next,
                if incoming == 1 { 0x1010 } else { 0x1008 },
                "{context}"
            );
            assert_eq!(
                machine.read(flag),
                if annul && incoming == 0 {
                    incoming
                } else {
                    slot_flag
                },
                "{context}"
            );
        }
    }
}

#[test]
fn integer_branches_use_incoming_flags_and_publish_delay_slot_flags() {
    for bits in [32, 64] {
        for annul in [false, true] {
            for (cond, flag) in [(5, "cf"), (6, "nf"), (7, "vf")] {
                check_branch(bits, false, false, annul, cond, &format!("i_{flag}"));
                check_branch(bits, true, false, annul, cond, &format!("i_{flag}"));
                if bits == 64 {
                    check_branch(bits, true, true, annul, cond, &format!("x_{flag}"));
                }
            }
        }
    }
}

#[test]
fn register_branches_test_the_register_before_the_delay_slot_rewrites_it() {
    let rconds: [(u32, fn(i64) -> bool); 6] = [
        (1, |v| v == 0),
        (2, |v| v <= 0),
        (3, |v| v < 0),
        (5, |v| v != 0),
        (6, |v| v > 0),
        (7, |v| v >= 0),
    ];
    for bits in [32u32, 64] {
        let mask = if bits == 32 {
            u64::from(u32::MAX)
        } else {
            u64::MAX
        };
        for annul in [false, true] {
            for (rcond, holds) in rconds {
                for slot in [-1i64, 0, 1] {
                    let branch =
                        (u32::from(annul) << 29) | (rcond << 25) | (3 << 22) | (1 << 14) | 4;
                    let mov_g1 = 0x82102000 | (slot as u32 & 0x1fff);
                    let mut machine = Machine::new(bits, branch, mov_g1);
                    for incoming in [-1i64, 0, 1] {
                        let taken = holds(incoming);
                        let next = machine.step(&[("g1", incoming as u64 & mask)]);
                        let context = format!(
                            "SPARC{bits} rcond={rcond} annul={annul} g1={incoming} slot={slot}"
                        );
                        assert_eq!(next, if taken { 0x1010 } else { 0x1008 }, "{context}");
                        let g1 = if annul && !taken { incoming } else { slot };
                        assert_eq!(machine.read("g1"), g1 as u64 & mask, "{context}");
                    }
                }
            }
        }
    }
}
