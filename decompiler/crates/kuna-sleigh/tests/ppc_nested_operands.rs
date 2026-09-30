//! Synthetic MD- and MDS-form encodings, checked against rotation and architectural bit numbering.
use kuna_base::{address::Address, error::KunaResult};
use kuna_sleigh::{
    emulate::{BreakTableCallBack, Emulate, EmulatePcodeCache},
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    memstate::{MemoryBank, MemoryHashOverlay, MemoryState},
    sleigh::Sleigh,
    translate::Translate,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

struct Bytes(Vec<u8>);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "synthetic-rotate"
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

fn mask(begin: u32, end: u32) -> u64 {
    (0..64)
        .filter(|&bit| {
            if begin <= end {
                bit >= begin && bit <= end
            } else {
                bit >= begin || bit <= end
            }
        })
        .fold(0, |value, bit| value | (1u64 << (63 - bit)))
}

/// One rotate instruction: its word, rotate amount, selected bits, and whether
/// the unselected bits of rA are kept (rldimi).
struct Case {
    word: u32,
    shift: u32,
    selected: u64,
    insert: bool,
    record: bool,
}

/// MD forms rldicl/rldicr/rldic/rldimi (XO 0-3) and MDS form rldcr (XO 9,
/// rotate by r5): rldic, rldimi and rldcr reach their masks through nested
/// operand expressions.
fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (shift, bound) in [
        (0, 0),
        (0, 63),
        (1, 0),
        (2, 30),
        (31, 32),
        (32, 31),
        (63, 0),
        (63, 63),
    ] {
        for record in [false, true] {
            let fields = (30u32 << 26)
                | (4 << 21)
                | (3 << 16)
                | ((bound & 31) << 6)
                | (bound & 32)
                | u32::from(record);
            for form in 0..4u32 {
                cases.push(Case {
                    word: fields | ((shift & 31) << 11) | (form << 2) | ((shift >> 5) << 1),
                    shift,
                    selected: match form {
                        0 => mask(bound, 63),
                        1 => mask(0, bound),
                        _ => mask(bound, 63 - shift),
                    },
                    insert: form == 3,
                    record,
                });
            }
            cases.push(Case {
                word: fields | (5 << 11) | (9 << 1),
                shift,
                selected: mask(0, bound),
                insert: false,
                record,
            });
        }
    }
    cases
}

#[test]
fn nested_rotate_operands_preserve_values_and_record_flags() {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs/Ghidra/Processors/PowerPC/data/languages");
    let cases = cases();
    for model in ["ppc_64", "ppc_64_isa_altivec"] {
        for endian in ["be", "le"] {
            let sla = std::fs::read(specs.join(format!("{model}_{endian}.sla"))).unwrap();
            let bytes: Vec<u8> = cases
                .iter()
                .map(|case| case.word)
                .chain([0x60000000])
                .flat_map(|w| if endian == "be" { w.to_be_bytes() } else { w.to_le_bytes() })
                .collect();
            let mut sleigh = Sleigh::new(Box::new(Bytes(bytes)), Box::new(ContextInternal::new()));
            sleigh.initialize_from_sla(&sla).unwrap();
            let manager = sleigh.manager_rc();
            let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
            let translate: Rc<dyn Translate> = Rc::new(sleigh);
            let mut memory = MemoryState::new(Rc::clone(&translate));
            for name in ["ram", "register", "unique"] {
                let bank: Rc<RefCell<dyn MemoryBank>> =
                    Rc::new(RefCell::new(MemoryHashOverlay::new(
                        Rc::clone(manager.get_space_by_name(name).unwrap()),
                        8,
                        64,
                        4096,
                        None,
                    )));
                memory.set_memory_bank(bank);
            }
            let memory = Rc::new(RefCell::new(memory));
            let breaks = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&translate))));
            let mut emulator = EmulatePcodeCache::new(translate, manager, Rc::clone(&memory), breaks);
            for (i, case) in cases.iter().enumerate() {
                let word = case.word;
                for value in [0u64, u64::MAX, 0x8000000000000001, 0x123456789abcdef0] {
                    let old = 0xa55ac33c69969669;
                    let expected = (value.rotate_left(case.shift) & case.selected)
                        | if case.insert { old & !case.selected } else { 0 };
                    {
                        let mut state = memory.borrow_mut();
                        state.set_value_by_name("r3", old).unwrap();
                        state.set_value_by_name("r4", value).unwrap();
                        state.set_value_by_name("r5", 0xffc0 | u64::from(case.shift)).unwrap();
                        state.set_value_by_name("cr0", 0xa).unwrap();
                        state.set_value_by_name("xer_so", 1).unwrap();
                    }
                    emulator
                        .set_execute_address(&Address::new(Rc::clone(&ram), 0x1000 + 4 * i as u64))
                        .unwrap();
                    emulator
                        .execute_instruction()
                        .unwrap_or_else(|e| panic!("{model}_{endian} {word:08x}: {e}"));
                    let state = memory.borrow();
                    assert_eq!(
                        state.get_value_by_name("r3").unwrap(),
                        expected,
                        "{model}_{endian} {word:08x} {value:016x}"
                    );
                    let flags = if !case.record {
                        0xa
                    } else {
                        1 | if expected == 0 {
                            2
                        } else if (expected as i64) < 0 {
                            8
                        } else {
                            4
                        }
                    };
                    assert_eq!(
                        state.get_value_by_name("cr0").unwrap(),
                        flags,
                        "record flag {word:08x}"
                    );
                }
            }
        }
    }
}
