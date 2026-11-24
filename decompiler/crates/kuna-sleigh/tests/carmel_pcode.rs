//! Carmel p-code regressions based on TASKING fixtures, independent of loader painting.
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use kuna_base::{address::Address, error::{KunaError, KunaResult}, space::AddrSpace};
use kuna_sleigh::{
    emulate::{BreakTableCallBack, Emulate, EmulatePcodeCache},
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    memstate::{MemoryHashOverlay, MemoryState},
    sleigh::Sleigh,
    translate::Translate,
};

const NOP: u32 = 0x2bec00;
const INC_A0: u32 = 0x21ac20;
const INC_A1: u32 = 0x21ac62;
const INC_A2: u32 = 0x21aca4;
const SEQUENTIAL: u32 = 0x800000;
const RETURN: u32 = 0x2bd800;
const CLIW: (u32, u32) = (0x8c6000, 0xdc61c2);
const TEMPLATE_NOP: [u32; 3] = [0x7ffffeff, 0xfff03fff, 0xff7ffff8];
const TEMPLATE_MAC: [u32; 3] = [0x8097e0ff, 0xfff03fff, 0xff7ffff8];

struct Bytes {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "carmel-pcode-fixture"
    }

    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }

    fn adjust_vma(&mut self, _: i64) {}

    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        let offset = addr.get_offset().checked_sub(self.base)
            .filter(|&offset| offset < self.bytes.len() as u64)
            .ok_or_else(|| KunaError::data_unavail("outside Carmel fixture CODE"))?;
        let bytes = &self.bytes[offset as usize..];
        let len = bytes.len().min(out.len());
        out[..len].copy_from_slice(&bytes[..len]);
        out[len..].fill(0);
        Ok(())
    }
}

struct Machine {
    emulator: EmulatePcodeCache,
    state: Rc<RefCell<MemoryState>>,
    code: Rc<AddrSpace>,
    data: Rc<AddrSpace>,
}

impl Machine {
    /// Paint independent fixture literals: (relative CODE word, issue mode, next issue bytes).
    fn new(base: u64, fetches: &[(u32, u32)], issues: &[(u64, u32, u32)], template: Option<[u32; 3]>) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let sla = std::fs::read(root.join("specs/Ghidra/Processors/Carmel/data/languages/carmel.sla"))
            .expect("compile the Carmel specification before running these tests");
        let bytes = fetches.iter().flat_map(|&(left, right)| {
            [left, right].into_iter().flat_map(|word| {
                let [_, a, b, c] = word.to_be_bytes();
                [a, b, c]
            })
        }).collect();
        let image = Bytes { base: 3 * base, bytes };
        let mut sleigh = Sleigh::new(Box::new(image), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).unwrap();
        let manager = sleigh.manager_rc();
        let code = Rc::clone(manager.get_space_by_name("code").unwrap());
        let data = Rc::clone(manager.get_space_by_name("data").unwrap());
        assert_eq!((code.get_word_size(), data.get_word_size()), (3, 2));
        let address = |word| Address::new(Rc::clone(&code), 3 * (base + word));
        sleigh.with_context_db_mut(|db| {
            for &(word, mode, next) in issues {
                if mode == 0 {
                    continue;
                }
                let (start, end) = (address(word), address(word + 1));
                db.set_variable_region(b"carmel_issue", &start, &end, mode).unwrap();
                db.set_variable_region(b"carmel_next_issue_size", &start, &end, next).unwrap();
            }
            if let Some([high, mid, low]) = template {
                let (start, end) = (address(0), address(1));
                for (field, value) in [
                    (b"carmel_cliw_known".as_slice(), 1),
                    (b"carmel_cliw_95_64".as_slice(), high),
                    (b"carmel_cliw_63_32".as_slice(), mid),
                    (b"carmel_cliw_31_0".as_slice(), low),
                ] {
                    db.set_variable_region(field, &start, &end, value).unwrap();
                }
            }
        });
        let trans: Rc<dyn Translate> = Rc::new(sleigh);
        let mut state = MemoryState::new(Rc::clone(&trans));
        // Bank alignment is independent of the target's address scaling (in particular CODE's 3).
        for (name, alignment) in [("code", 8), ("data", 2), ("register", 8), ("unique", 8)] {
            let space = Rc::clone(manager.get_space_by_name(name).unwrap());
            state.set_memory_bank(Rc::new(RefCell::new(MemoryHashOverlay::new(
                space, alignment, 64, 128, None,
            ))));
        }
        let state = Rc::new(RefCell::new(state));
        let breaks = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
        let emulator = EmulatePcodeCache::new(trans, manager, Rc::clone(&state), breaks);
        Self { emulator, state, code, data }
    }

    fn seed(&self, registers: &[(&str, u64)]) {
        let mut state = self.state.borrow_mut();
        for &(name, value) in registers {
            state.set_value_by_name(name, value).unwrap();
        }
    }

    fn reg(&self, name: &str) -> u64 {
        self.state.borrow().get_value_by_name(name).unwrap()
    }

    fn seed_data(&self, word: u64, value: u64) {
        self.state.borrow_mut().set_value(&self.data, 2 * word, 2, value).unwrap();
    }

    fn data(&self, word: u64) -> u64 {
        self.state.borrow().get_value(&self.data, 2 * word, 2).unwrap()
    }

    fn start(&mut self, word: u64) -> KunaResult<()> {
        self.emulator.set_execute_address(&Address::new(Rc::clone(&self.code), 3 * word))
    }

    /// The emulator PC is a byte offset even for word-addressed CODE.
    fn step(&mut self) -> u64 {
        self.emulator.execute_instruction().unwrap();
        let pc = self.emulator.get_execute_address();
        assert_eq!(pc.get_space().unwrap().get_name(), "code");
        pc.get_offset()
    }
}

fn goto(displacement: i32, delay: u32, call: bool) -> u32 {
    0x2b0000 | ((call as u32) << 13) | ((displacement as u32 & 0x3ff) << 3) | (delay << 1)
}

#[test]
fn parallel_move_reads_the_incoming_accumulator() {
    let mut m = Machine::new(0, &[(INC_A0, 0x31aa20), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("A0L", 7), ("A1L", 0x3412), ("ASTATUS", 0xa55a)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0L"), m.reg("A1L")), (8, 7));
    assert_eq!(m.reg("ASTATUS"), 0xa55a);
}

#[test]
fn inc_wrap_and_status_use_the_widened_alu_and_physical_lane() {
    // For 0x7fff + 1 the 40-bit temporary is positive but exceeds signed 32 bits.
    // EST2 is used by the physical parallel-right lane, not the sequential-right slot.
    let cases = [
        (0x0000, 0x0001, 0x00),
        (0xffff, 0x0000, 0x09),
        (0x7fff, 0x8000, 0x24),
        (0x8000, 0x8001, 0x10),
    ];
    for (input, result, flags) in cases {
        for (fetch, word, mode, shift) in [
            ((INC_A0, NOP), 0, 1, 0),
            ((NOP, INC_A0), 0, 1, 6),
            ((SEQUENTIAL | INC_A1, INC_A0), 1, 3, 0),
        ] {
            let mut m = Machine::new(0, &[fetch, (NOP, NOP)], &[(word, mode, 6), (2, 1, 0)], None);
            m.seed(&[("A0", 0x8765430000 | input), ("A1L", 0x1234), ("ASTATUS", 0x55aa)]);
            m.start(word).unwrap();
            assert_eq!(m.step(), 6);
            assert_eq!(m.reg("A0"), 0x8765430000 | result);
            assert_eq!(m.reg("A1L"), 0x1234);
            assert_eq!(m.reg("ASTATUS"), 0x55aa);
            assert_eq!(m.reg("EST"), flags << shift, "input {input:#06x}, mode {mode}, lane {shift}");
        }
    }
    let mut m = Machine::new(
        0, &[(INC_A0, NOP), (INC_A0, NOP), (NOP, NOP)],
        &[(0, 1, 6), (2, 1, 6), (4, 1, 0)], None,
    );
    m.seed(&[("A0L", 0x7fff)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.reg("EST"), 0x24);
    assert_eq!(m.step(), 12);
    assert_eq!(m.reg("EST"), 0x14, "L is sticky across a non-limiting INC");
}

#[test]
fn wide_data_store_uses_the_old_word_address_and_postincrements_r0() {
    for (low, expected_r0) in [(0xc64000, 0x4100), (0xc64040, 0x4101)] {
        let mut m = Machine::new(0, &[(0x808e4c, low), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x4100), ("ECFG", 0), ("ASTATUS", 0xa55a)]);
        for (word, value) in [(0x2080, 0xabcd), (0x40ff, 0x1111), (0x4100, 0x2222), (0x4101, 0x3333)] {
            m.seed_data(word, value);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("R0"), expected_r0);
        assert_eq!(m.reg("ASTATUS"), 0xa55a);
        assert_eq!(m.data(0x4100), 0x8e4c);
        assert_eq!((m.data(0x2080), m.data(0x40ff), m.data(0x4101)), (0xabcd, 0x1111, 0x3333));
    }
}

#[test]
fn goto_targets_an_odd_code_word_without_running_its_left_slot() {
    let mut m = Machine::new(
        0,
        &[(goto(7, 0, false), NOP), (INC_A0, NOP), (INC_A1, NOP), (SEQUENTIAL | INC_A0, INC_A2), (NOP, NOP)],
        &[(0, 1, 6), (2, 1, 6), (4, 1, 3), (6, 2, 3), (7, 3, 6), (8, 1, 0)],
        None,
    );
    m.start(0).unwrap();
    assert_eq!(m.step(), 21);
    assert_eq!(m.step(), 24);
    assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L")), (0, 0, 1));
}

#[test]
fn negative_goto_displacement_is_relative_to_the_odd_issue() {
    let base = 0x1000;
    let mut m = Machine::new(
        base, &[(INC_A2, NOP), (SEQUENTIAL | INC_A0, goto(-3, 0, false))],
        &[(0, 1, 3), (2, 2, 3), (3, 3, 0)], None,
    );
    m.start(base + 3).unwrap();
    assert_eq!(m.step(), 3 * base);
    assert_eq!(m.step(), 3 * (base + 2));
    assert_eq!((m.reg("A0L"), m.reg("A2L")), (0, 1));
}

#[test]
fn goto_delay_consumes_one_sequential_slot_or_one_wide_issue() {
    for (size, mode, delay, right_mode, expected_a0) in [
        (3, 2, (SEQUENTIAL | INC_A0, INC_A1), 3, 8),
        (6, 5, (0x801234, 0xc64240), 0, 0x1234),
    ] {
        let mut m = Machine::new(
            0, &[(goto(6, 1, false), NOP), delay, (INC_A1, NOP), (NOP, NOP)],
            &[(0, 1, size), (2, mode, size), (3, right_mode, 6), (4, 1, 6), (6, 1, 0)], None,
        );
        m.seed(&[("A0L", 7), ("A1L", 9)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 18);
        assert_eq!((m.reg("A0L"), m.reg("A1L")), (expected_a0, 9));
    }
}

#[test]
fn call_pushes_the_undelayed_next_pc_plus_two_words_per_delay() {
    let base = 0x120004;
    for (delay, next_size, next, next_right) in [
        (0, 3, (SEQUENTIAL | INC_A0, INC_A1), 3),
        (1, 3, (SEQUENTIAL | INC_A0, INC_A1), 3),
        (1, 6, (0x801234, 0xc64240), 0),
    ] {
        let next_mode = if next_size == 3 { 2 } else { 5 };
        let mut m = Machine::new(
            base,
            &[(goto(8, delay, true), NOP), next, (NOP, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 1, next_size), (2, next_mode, next_size), (3, next_right, 6), (4, 1, 6), (6, 1, 6), (8, 1, 0)],
            None,
        );
        m.seed(&[("SP", 0x4040), ("A0L", 7), ("A1L", 9)]);
        for (word, value) in [(0x403e, 0x1111), (0x403f, 0x2222), (0x4040, 0x3333), (0x4041, 0x4444)] {
            m.seed_data(word, value);
        }
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * (base + 8));
        assert_eq!(m.reg("SP"), 0x403e);
        assert_eq!(m.data(0x4040), 0x12);
        assert_eq!(m.data(0x403f), 6 + 2 * u64::from(delay));
        assert_eq!((m.data(0x403e), m.data(0x4041)), (0x1111, 0x4444));
        assert_eq!(m.reg("A0L"), match (delay, next_size) { (0, _) => 7, (_, 3) => 8, _ => 0x1234 });
        assert_eq!(m.reg("A1L"), 9, "the second sequential issue is not a delay slot");
    }
}

#[test]
fn return_reads_the_low_then_high_data_words_and_uses_a_code_word_pc() {
    for delay in [0, 1] {
        let mut fetches = [(NOP, NOP); 9];
        fetches[0].0 = RETURN | (delay << 1);
        fetches[1] = (INC_A0, NOP);
        let issues: Vec<_> = (0..=0x10).step_by(2).map(|word| (word, 1, 6)).collect();
        let mut m = Machine::new(0, &fetches, &issues, None);
        m.seed(&[("SP", 0x4000)]);
        m.seed_data(0x4000, 0xdead);
        m.seed_data(0x4001, 0x0010);
        m.seed_data(0x4002, 0);
        m.start(0).unwrap();
        assert_eq!(m.step(), 0x30);
        assert_eq!(m.reg("SP"), 0x4002);
        assert_eq!(m.reg("A0L"), u64::from(delay));
        assert_eq!((m.data(0x4000), m.data(0x4001), m.data(0x4002)), (0xdead, 0x0010, 0));
    }
}

#[test]
fn transfer_in_the_delay_slot_is_not_lifted() {
    let mut m = Machine::new(
        0, &[(goto(4, 1, false), NOP), (RETURN, NOP), (NOP, NOP)],
        &[(0, 1, 6), (2, 1, 6), (4, 1, 0)], None,
    );
    assert!(matches!(m.start(0), Err(KunaError::Unimpl { .. })), "a delayed transfer must not enter the p-code stream");
}

#[test]
fn cliw_nop_and_signed_mac_specialize_the_same_code_word() {
    for (template, ecfg, left, expected, flags) in [
        (TEMPLATE_NOP, 1, 0xfff9, 0x8765432100, 0),
        (TEMPLATE_MAC, 0, 7, 63, 0),
        (TEMPLATE_MAC, 1, 7, 126, 0),
        (TEMPLATE_MAC, 0, 0xfff9, 0xffffffffc1, 0x10),
        (TEMPLATE_MAC, 1, 0xfff9, 0xffffffff82, 0x10),
    ] {
        let mut m = Machine::new(0, &[CLIW, (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], Some(template));
        m.seed(&[("A0", 0x8765432100), ("A1L", left), ("A2L", 9), ("ECFG", ecfg),
                 ("ASTATUS", 0xa55a), ("PSTATUS", 0x40)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("A0"), expected);
        assert_eq!((m.reg("A1L"), m.reg("A2L"), m.reg("ECFG")), (left, 9, ecfg));
        assert_eq!((m.reg("ASTATUS"), m.reg("PSTATUS"), m.reg("EST")), (0xa55a, 0x40, flags));
    }
}

#[test]
fn tasking_compiled_addone_returns_the_c_result() {
    let code: String = include_str!("../../../../tests/fixtures/carmel/compiler-o2.hex")
        .split_whitespace()
        .collect();
    let base = 0x2e;
    let start = (6 * base) as usize;
    let left = u32::from_str_radix(&code[start..start + 6], 16).unwrap();
    let right = u32::from_str_radix(&code[start + 6..start + 12], 16).unwrap();
    let mut m = Machine::new(
        base, &[(left, right), (NOP, NOP)], &[(0, 2, 3), (1, 3, 6), (2, 1, 0)], None,
    );
    for (x, expected) in [(-2i16, -1i16), (0, 1), (42, 43)] {
        m.seed(&[("A0L", u64::from(x as u16)), ("SP", 0x4400), ("ECFG", 0)]);
        m.seed_data(0x4401, base + 2);
        m.seed_data(0x4402, 0);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * (base + 1));
        assert_eq!(m.step(), 3 * (base + 2));
        assert_eq!(m.reg("A0L"), u64::from(expected as u16));
        assert_eq!(m.reg("SP"), 0x4402);
    }
}
