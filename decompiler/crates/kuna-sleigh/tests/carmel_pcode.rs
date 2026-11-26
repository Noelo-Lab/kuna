//! Carmel p-code regressions based on TASKING fixtures, independent of loader painting.
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use kuna_base::{address::Address, error::{KunaError, KunaResult}, space::AddrSpace};
use kuna_num::opcodes::OpCode;
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
    io: Rc<AddrSpace>,
    ext: Rc<AddrSpace>,
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
        let io = Rc::clone(manager.get_space_by_name("io").unwrap());
        let ext = Rc::clone(manager.get_space_by_name("ext").unwrap());
        assert_eq!((io.get_word_size(), ext.get_word_size()), (2, 2));
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
        for (name, alignment) in [("code", 8), ("data", 2), ("io", 2), ("ext", 2), ("register", 8), ("unique", 8)] {
            let space = Rc::clone(manager.get_space_by_name(name).unwrap());
            state.set_memory_bank(Rc::new(RefCell::new(MemoryHashOverlay::new(
                space, alignment, 64, 128, None,
            ))));
        }
        let state = Rc::new(RefCell::new(state));
        let breaks = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
        let emulator = EmulatePcodeCache::new(trans, manager, Rc::clone(&state), breaks);
        Self { emulator, state, code, data, io, ext }
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
fn compact_half_compare_uses_signed_order_and_the_physical_lane() {
    for (word, lhs, rhs, flags) in [
        (0x209a02, 0x8000, 1, 0x10),
        (0x209a02, 0x8000, 0x8000, 8),
        (0x209a02, 1, 0x8000, 0),
        (0x20ba02, 0x8000, 1, 1),
        (0x20ba02, 0x8000, 0x8000, 9),
        (0x20ba02, 1, 0x8000, 0),
    ] {
        for (fetch, metadata, ticks, shift) in [
            ((word, NOP), &[(0, 1, 6), (2, 1, 0)][..], 1, 0),
            ((NOP, word), &[(0, 1, 6), (2, 1, 0)][..], 1, 6),
            ((SEQUENTIAL | NOP, word), &[(0, 2, 3), (1, 3, 3), (2, 1, 0)][..], 2, 0),
        ] {
            let mut m = Machine::new(0, &[fetch, (NOP, NOP)], metadata, None);
            let a0 = 0x8765430000 | lhs;
            let a1 = 0x1234560000 | rhs;
            m.seed(&[("A0", a0), ("A1", a1)]);
            m.start(0).unwrap();
            for _ in 0..ticks {
                m.step();
            }
            assert_eq!((m.reg("A0"), m.reg("A1"), m.reg("EST")), (a0, a1, flags << shift));
        }
    }
}

#[test]
fn wide_signed_literal_to_whole_accumulator_replaces_the_guard() {
    for (literal, expected, est) in [
        (0, 0, 8),
        (0x7fff, 0x7fff, 0),
        (0x8000, 0xffffff8000, 0x10),
        (0xffff, 0xffffffffff, 0x10),
    ] {
        let mut m = Machine::new(
            0, &[(0x800000 | literal, 0xc64220), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None,
        );
        m.seed(&[("A0", 0x8765432100), ("R0", 0xbeef)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST"), m.reg("R0")), (expected, est, 0xbeef));
    }
}

#[test]
fn wide_literal_to_address_register_does_not_shift_or_set_alu_status() {
    for (right, register) in [(0xc64280, "R0"), (0xc64290, "R4"), (0xc6429c, "R7")] {
        let mut m = Machine::new(
            0, &[(0x808001, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None,
        );
        m.seed(&[(register, 0xbeef), ("A0", 0x8765432100), ("ECFG", 0x20)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(register), m.reg("A0"), m.reg("EST")), (0x8001, 0x8765432100, 0));
    }
}

#[test]
fn signed_compact_address_move_uses_the_issue_lane_and_preserves_arithmetic_flags() {
    for (fetch, mode, word, register, cfg, table, expected) in [
        ((0x310101, NOP), 1, 0, "R0", (0, 0), 0xff00, 0x2345),
        ((0x310101, NOP), 1, 0, "R0", (0x1000, 0), 0xff00, 0xff80),
        ((0xb10fd1, NOP), 2, 0, "R7", (0, 0), 0x00ff, 0xffe8),
        ((SEQUENTIAL | NOP, 0x3116ff), 3, 1, "RN3", (0, 0), 0x00ff, 0x007f),
        ((NOP, 0x3117d1), 1, 0, "RN3", (0, 0), 0x00ff, 0x2345),
        ((NOP, 0x3117d1), 1, 0, "RN3", (0, 0), 0xff00, 0xffe8),
    ] {
        let issues = if mode == 1 { [(0, 1, 6), (1, 0, 0), (2, 1, 0)] }
                     else { [(0, 2, 3), (1, 3, 6), (2, 1, 0)] };
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)], &issues, None);
        m.seed(&[(register, 0x2345), ("CECFG1", cfg.0), ("CECFG2", cfg.1), ("CETBL", table), ("EST", 2)]);
        m.start(word).unwrap();
        assert_eq!(m.step(), if mode == 2 { 3 } else { 6 });
        assert_eq!(m.reg(register), expected);
        assert_eq!(m.reg("EST"), 2 | (if table == 0xff00 { 0x2000 } else { 0x1000 }));
    }
}

#[test]
fn compact_data_move_reads_the_old_address_and_uses_the_new_signed_increment() {
    for (cfg, value, rn, pointer, accumulator, flags) in [
        (0, 0, 3, 0x414b, 0, 8),
        (0x1000, 0x8333, 0xffe8, 0x4130, 0xffffff8333, 0x10),
    ] {
        let mut m = Machine::new(
            0, &[(0x3113d1, NOP), (0x31dc92, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 6), (4, 1, 0)], None,
        );
        m.seed(&[("R1", 0x4148), ("RN1", 3), ("CECFG1", cfg), ("A2", 0x23567890), ("EST", 0xfff)]);
        m.seed_data(0x4148, value);
        m.seed_data(pointer, 0x7777);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("RN1"), rn);
        assert_eq!(m.step(), 12);
        assert_eq!((m.reg("R1"), m.reg("A2"), m.reg("EST")), (pointer, accumulator, flags));
        assert_eq!((m.data(0x4148), m.data(pointer)), (value, 0x7777));
    }
}

#[test]
fn signed_mac_and_msu_preserve_native_wrap_limit_and_no_borrow() {
    for (word, lhs, rhs, accumulator, ecfg, result, est) in [
        (0x58d114, 2, 3, 0x7fffffffff, 0, 0x8000000005, 0x36),
        (0x58d114, 2, 3, 0x7fffffff, 4, 0x7fffffff, 0x24),
        (0x58d114, 0xfffe, 3, 0xff80000000, 4, 0xff80000000, 0x35),
        (0x5cd114, 2, 3, 6, 0, 0, 9),
        (0x5cd114, 0x8000, 0x8000, 0, 5, 0xff80000000, 0x10),
    ] {
        let mut m = Machine::new(0, &[(word, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A0", accumulator), ("A1L", lhs), ("A2L", rhs), ("ECFG", ecfg), ("X1", 0x1234)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST"), m.reg("X1")), (result, est, 0x1234));
    }
}

#[test]
fn immediate_repeat_skips_or_replays_only_its_next_issue() {
    for count in [0, 1, 5] {
        let mut m = Machine::new(
            0, &[(0x2bc000 | count, NOP), (INC_A0, NOP), (INC_A1, NOP), (NOP, NOP)],
            &[(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None,
        );
        m.seed(&[("A0L", 7), ("A1L", 12)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 12, "the repeated issue is not followed twice");
        assert_eq!((m.reg("A0L"), m.reg("A1L")), (7 + u64::from(count), 12));
        assert_eq!(m.step(), 18);
        assert_eq!(m.reg("A1L"), 13);
    }
}

#[test]
fn ce1_repeat_executes_once_on_veto_or_uses_the_literal_count() {
    for (count, cfg, table, expected) in [
        (0, 0, 0, 1), (3, 0, 0, 1), (0, 0x1000, 0, 0), (3, 0x1000, 0, 3), (3, 0, 0xffff, 3),
    ] {
        let mut m = Machine::new(
            0, &[(count, 0xcec000), (INC_A0, NOP), (INC_A1, NOP), (NOP, NOP)],
            &[(0, 4, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None,
        );
        m.seed(&[("A0L", 7), ("A1L", 12), ("CECFG1", cfg), ("CETBL", table)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 12);
        assert_eq!((m.reg("A0L"), m.reg("A1L")), (7 + expected, 12));
        assert_eq!(m.step(), 18);
        assert_eq!(m.reg("A1L"), 13);
    }
}

#[test]
fn dynamic_register_repeat_samples_before_the_setup_and_modifies_r1_once() {
    for (root, mode) in [((0x2bc400, NOP), 1), ((0x890000, 0xcf1c00), 5)] {
        for count in [0, 3] {
            let mut m = Machine::new(
                0,
                &[root, (SEQUENTIAL | INC_A0, INC_A1), (INC_A2, NOP), (0x21ace6, NOP), (NOP, NOP)],
                &[(0, mode, 3), (2, 2, 3), (3, 3, 6), (4, 1, 6), (6, 1, 6), (8, 1, 0)],
                None,
            );
            m.seed(&[("A0L", count), ("R1", count), ("RN1", 2), ("PSTATUS", 0x10)]);
            m.start(0).unwrap();
            assert_eq!(m.step(), 24);
            assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L")), (count + 1, 1, 1));
            assert_eq!(m.reg("A3L"), count);
            assert_eq!(m.reg("R1"), count + if mode == 5 { 2 } else { 0 });
            assert_eq!(m.reg("PSTATUS"), 0x10);
        }
    }
}

#[test]
fn dynamic_data_repeat_zero_skips_only_the_first_of_the_selected_issues() {
    for (delay, count, expected) in [
        (2, 3, [1, 1, 1]), (3, 3, [3, 1, 1]), (4, 3, [3, 3, 1]),
        (5, 0, [0, 1, 1]), (5, 3, [3, 3, 3]), (7, 3, [3, 3, 3]),
    ] {
        let mut m = Machine::new(
            0,
            &[(0xb84100, 0xcf0000 | delay << 11), (INC_A0, NOP), (INC_A1, NOP), (INC_A2, NOP),
              (0x21ace6, NOP), (0x21ad28, NOP), (0x21ad6a, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6),
              (10, 1, 6), (12, 1, 6), (14, 1, 6), (16, 1, 6), (18, 1, 0)],
            None,
        );
        m.seed_data(0x4100, count);
        m.start(0).unwrap();
        let end = 2 + 2 * (3 + delay.saturating_sub(2).max(1));
        assert_eq!(m.step(), u64::from(end) * 3);
        for _ in (end..14).step_by(2) {
            m.step();
        }
        assert_eq!([m.reg("A0L"), m.reg("A1L"), m.reg("A2L")], [1, 1, 1]);
        assert_eq!([m.reg("A3L"), m.reg("A4L"), m.reg("A5L")], expected);
        assert_eq!(m.data(0x4100), count);
    }
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
fn adjacent_data_load_reads_before_store_but_remains_an_independent_target() {
    let mut m = Machine::new(
        0,
        &[(0x805678, 0xc64040), (0xb84000, 0xc36280), (NOP, NOP)],
        &[(0, 5, 6), (2, 5, 6), (4, 1, 0)],
        None,
    );
    m.seed(&[("R0", 0x4000)]);
    m.seed_data(0x4000, 0xabcd);
    m.start(0).unwrap();
    assert_eq!(m.step(), 12);
    assert_eq!((m.reg("R0"), m.data(0x4000)), (0xabcd, 0x5678));

    m.start(2).unwrap();
    assert_eq!(m.step(), 12);
    assert_eq!(m.reg("R0"), 0x5678);
}

#[test]
fn delayed_transfer_does_not_execute_the_following_data_load() {
    let mut m = Machine::new(
        0,
        &[(goto(6, 1, false), NOP), (0x805678, 0xc64040), (0xb84000, 0xc36280), (NOP, NOP)],
        &[(0, 1, 6), (2, 5, 6), (4, 5, 6), (6, 1, 0)],
        None,
    );
    m.seed(&[("R0", 0x4000)]);
    m.seed_data(0x4000, 0xabcd);
    m.start(0).unwrap();
    assert_eq!(m.step(), 18);
    assert_eq!((m.reg("R0"), m.data(0x4000)), (0x4001, 0x5678));
}

#[test]
fn wide_goto_saturates_delay_and_counts_sequential_slots_separately() {
    let mut m = Machine::new(
        0,
        &[(0x800008, 0xcfb800), (SEQUENTIAL | INC_A0, INC_A1), (0x801234, 0xc64280),
          (INC_A2, NOP), (NOP, NOP)],
        &[(0, 5, 3), (2, 2, 3), (3, 3, 6), (4, 5, 6), (6, 1, 6), (8, 1, 0)],
        None,
    );
    m.seed(&[("A0L", 7), ("A1L", 9), ("A2L", 13), ("R0", 0)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 24);
    assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L"), m.reg("R0")), (8, 10, 13, 0x1234));
}

#[test]
fn qualified_wide_goto_samples_address_enable_before_the_delay() {
    for (config, next) in [(0, 18), (0x1000, 24)] {
        let mut m = Machine::new(
            0,
            &[(0x000008, 0xcf9000), (INC_A0, NOP), (INC_A1, NOP), (INC_A2, NOP), (NOP, NOP)],
            &[(0, 4, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 0)],
            None,
        );
        m.seed(&[("CECFG1", config), ("CETBL", 0), ("EST", 0)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), next);
        assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L")), (1, 1, 0));
    }
}

#[test]
fn wide_relative_call_limits_the_delay_but_returns_to_the_encoded_word() {
    let base = 0x11000;
    for delay in 0..=7 {
        let mut fetches = [(NOP, NOP); 14];
        fetches[0] = (0x800018, 0xcf8200 | delay << 11);
        fetches[1..=8].fill((INC_A0, NOP));
        fetches[12] = (RETURN, NOP);
        let issues: [(u64, u32, u32); 14] =
            std::array::from_fn(|i| (2 * i as u64, if i == 0 { 5 } else { 1 }, 6));
        let mut m = Machine::new(base, &fetches, &issues, None);
        m.seed(&[("A0L", 7), ("SP", 0x4040)]);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * (base + 24));
        assert_eq!(m.reg("A0L"), 7 + u64::from(delay.min(3)));
        assert_eq!(m.reg("SP"), 0x403e);
        let link = base + 2 + 2 * u64::from(delay);
        assert_eq!((m.data(0x403f), m.data(0x4040)), (link & 0xffff, link >> 16));
        assert_eq!(m.step(), 3 * link);
        assert_eq!(m.reg("SP"), 0x4040);
    }
}

#[test]
fn compact_call_counts_a_sequential_right_issue_but_keeps_the_odd_link() {
    let mut m = Machine::new(
        0, &[(0xab2066, INC_A0), (INC_A0, NOP), (INC_A1, NOP), (SEQUENTIAL | NOP, INC_A2),
             (NOP, NOP), (NOP, NOP), (RETURN, NOP), (NOP, NOP)],
        &[(0, 2, 3), (1, 3, 6), (2, 1, 6), (4, 1, 3), (6, 2, 3), (7, 3, 6),
          (8, 1, 6), (10, 1, 6), (12, 1, 6), (14, 1, 0)],
        None,
    );
    m.seed(&[("SP", 0x4040)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 36);
    assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L")), (2, 1, 0));
    assert_eq!((m.reg("SP"), m.data(0x403f), m.data(0x4040)), (0x403e, 7, 0));
    assert_eq!(m.step(), 21);
    assert_eq!(m.reg("SP"), 0x4040);
    assert_eq!(m.step(), 24);
    assert_eq!(m.reg("A2L"), 1);
}

#[test]
fn conditional_call_stores_its_link_on_both_paths_but_only_a_call_decrements_sp() {
    for (a0, target, sp) in [(0, 12, 0x403e), (0xffff, 8, 0x4040)] {
        let mut m = Machine::new(
            0,
            &[(INC_A0, NOP), (0x28822b, NOP), (INC_A1, NOP), (INC_A2, NOP),
              (INC_A0, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 6), (12, 1, 0)],
            None,
        );
        m.seed(&[("A0L", a0), ("A1L", 0xffff), ("SP", 0x4040)]);
        m.seed_data(0x403f, 0x1188);
        m.seed_data(0x4040, 0x2299);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.step(), target * 3);
        assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("A2L")), ((a0 + 1) & 0xffff, 0, 1));
        assert_eq!((m.reg("SP"), m.data(0x403f), m.data(0x4040)), (sp, 10, 0));
    }
}

#[test]
fn wide_nc1_goto_uses_carry_from_the_older_issue() {
    for (a0, target) in [(0, 8), (0xffff, 4)] {
        let mut m = Machine::new(
            0, &[(INC_A0, NOP), (0xc00006, 0xce4004), (INC_A1, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 1, 6), (2, 5, 6), (4, 1, 6), (6, 1, 6), (8, 1, 0)], None,
        );
        m.seed(&[("A0L", a0)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.step(), target * 3);
        assert_eq!(m.reg("A1L"), 0);
    }
}

#[test]
fn delayed_compact_eq_samples_est_before_its_two_counted_issues() {
    for (est, a0, a1, next) in [(8, 7, 9, 36), (0, 0xffff, 0xffff, 18)] {
        let mut m = Machine::new(
            0,
            &[(0x281c32, NOP), (INC_A0, NOP), (INC_A1, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 6), (12, 1, 0)],
            None,
        );
        m.seed(&[("A0L", a0), ("A1L", a1), ("EST", est)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), next);
        assert_eq!(m.reg("A0L"), (a0 + 1) & 0xffff);
        assert_eq!(m.reg("A1L"), (a1 + 1) & 0xffff);
        assert_ne!(m.reg("EST") & 8, est);
    }
}

#[test]
fn absolute_peripheral_moves_keep_equal_data_io_and_ext_word_addresses_distinct() {
    let mut m = Machine::new(
        0,
        &[(0x800102, 0xc64640), (0xb83456, 0xcc0409), (NOP, NOP)],
        &[(0, 5, 6), (2, 5, 6), (4, 1, 0)],
        None,
    );
    m.seed(&[("A0", 0xab87650000), ("EST", 0x1222)]);
    m.seed_data(0x102, 0x1afe);
    m.state.borrow_mut().set_value(&m.io, 0x204, 2, 0xb5d9).unwrap();
    m.state.borrow_mut().set_value(&m.ext, 0x204, 2, 0xa61d).unwrap();
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.step(), 12);
    assert_eq!((m.reg("A0"), m.reg("EST"), m.data(0x102)), (0xab8765b5d9, 0x1222, 0x1afe));
    let state = m.state.borrow();
    assert_eq!(state.get_value(&m.io, 0x204, 2).unwrap(), 0xb5d9);
    assert_eq!(state.get_value(&m.ext, 0x204, 2).unwrap(), 0x3456);
}

#[test]
fn wide_swap_exchanges_only_the_word_at_its_effective_data_address() {
    let mut m = Machine::new(
        0,
        &[(NOP, INC_A1), (0x8023dd, 0xc35618), (0x9f1010, 0xc340e2), (NOP, NOP)],
        &[(0, 1, 6), (2, 5, 6), (4, 5, 6), (6, 1, 0)],
        None,
    );
    m.seed(&[("A0", 0x12ab436010), ("A1L", 0xffff), ("X2", 0x4d6f), ("CETBL", 0xffff)]);
    m.seed_data(0xffff, 0x8000);
    m.seed_data(0x23dd, 0xbcde);
    m.seed_data(0, 0x4221);
    m.seed_data(0x1010, 0x4e5d);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.step(), 12);
    assert_eq!((m.reg("A0"), m.reg("EST")), (0x12ab438000, 0x3250));
    assert_eq!((m.data(0xffff), m.data(0), m.data(0x23dd)), (0x6010, 0x4221, 0xbcde));
    assert_eq!(m.step(), 18);
    assert_eq!((m.reg("X2"), m.reg("EST"), m.data(0x1010)), (0x4e5d, 0x3240, 0x4d6f));
}

#[test]
fn wide_swap_uses_the_old_pointer_and_a_long_stride_for_word_data() {
    let mut m = Machine::new(
        0,
        &[(0x900000, 0xc34142), (NOP, NOP)],
        &[(0, 5, 6), (2, 1, 0)],
        None,
    );
    m.seed(&[("A0", 0x12ab436010), ("R0", 0x4045), ("RN0", 3), ("RM0", 7), ("PACFG", 2)]);
    m.seed_data(0x4045, 0x8123);
    m.seed_data(0x4046, 0xbc31);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0"), m.reg("R0")), (0x12ab438123, 0x403f));
    assert_eq!((m.data(0x4045), m.data(0x4046)), (0x6010, 0xbc31));
}

#[test]
fn wide_swap_peer_data_selector_updates_its_pointer_without_reading_that_cell() {
    let mut m = Machine::new(0, &[(0x914040, 0xc34021), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A1", 0x234567789a), ("R0", 0x4040), ("RN0", 3), ("R1", 0x4041)]);
    m.seed_data(0x4040, 0x8123);
    m.seed_data(0x4041, 0xbc31);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("R0"), m.reg("R1"), m.reg("A1")), (0x4043, 0x4043, 0x2345678123));
    assert_eq!((m.data(0x4040), m.data(0x4041)), (0x789a, 0xbc31));

    let mut conflict =
        Machine::new(0, &[(0x904040, 0xc34020), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    assert!(matches!(conflict.start(0), Err(KunaError::BadData { .. })), "both AGU lanes update R0");
}

#[test]
fn link_uses_address_enable_before_its_independently_predicated_store() {
    for (config, table, sp, frame, saved) in [
        (0, 0, 0x4040, 0xc048, 0x2299),
        (0x1000, 0, 0x5aa5, 0x403f, 0x2299),
        (0x1000, 0xffff, 0x5aa5, 0x403f, 0xc048),
    ] {
        let mut m = Machine::new(0, &[(0x741a65, 0xc60a9e), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
        m.seed(&[("SP", 0x4040), ("R7", 0xc048), ("CECFG2", config), ("CETBL", table)]);
        m.seed_data(0x4040, 0x2299);
        m.seed_data(0x4041, 0x67ea);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("SP"), m.reg("R7"), m.data(0x4040), m.data(0x4041)), (sp, frame, saved, 0x67ea));
    }
}

#[test]
fn static_ce_banks_both_sample_the_incoming_feedback_bit() {
    for (incoming, after) in [(4, 0x3004), (0x1004, 4)] {
        let mut m = Machine::new(0, &[(0x8c0001, 0xcde4b3), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xaabb), ("EST", incoming)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("CECFG1"), m.reg("CECFG2"), m.reg("CETBL"), m.reg("EST")), (0x2ccc, 0x2ccc, 0x0101, after));
    }
}

#[test]
fn bank_exchange_swaps_only_selected_pointer_configuration_bits() {
    let mut m = Machine::new(0, &[(0x800001, 0xcb4000), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[
        ("R0", 0x1234), ("R0S", 0xabcd), ("RN0", 3), ("RN0S", 5),
        ("PACFG", 0x5156), ("PACFGS", 0xfff9), ("R1", 0x1111), ("R1S", 0x2222),
    ]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("R0"), m.reg("R0S"), m.reg("RN0"), m.reg("RN0S")), (0xabcd, 0x1234, 5, 3));
    assert_eq!((m.reg("PACFG"), m.reg("PACFGS"), m.reg("R1"), m.reg("R1S")), (0x5155, 0xfffa, 0x1111, 0x2222));
}

fn step_bank_without_data(m: &mut Machine, next: u64) {
    assert!(m.emulator.num_current_ops() > 0);
    for i in 0..m.emulator.num_current_ops() {
        let op = m.emulator.get_op_by_index(i).get_opcode();
        assert!(!matches!(op, OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER));
    }
    assert_eq!(m.step(), 3 * next);
}

#[test]
fn compact_bank_exchange_routes_all_address_selects() {
    for (select, pairs) in [
        (0, &[("R0", "R0S"), ("RN0", "RN0S")][..]),
        (1, &[("R1", "R1S"), ("RN1", "RN1S")][..]),
        (2, &[("R2", "R2S"), ("RN2", "RN2S")][..]),
        (3, &[("R3", "R3S"), ("RN3", "RN3S")][..]),
        (4, &[("R4", "R4S"), ("RN4", "RN4S")][..]),
        (5, &[("R5", "R5S"), ("RN5", "RN5S")][..]),
        (6, &[("R6", "R6S"), ("RN6", "RN6S")][..]),
        (7, &[("R7", "R7S"), ("RN7", "RN7S")][..]),
        (8, &[("RM0", "RM0S"), ("RL0", "RL0S")][..]),
        (9, &[("RM2", "RM2S"), ("RL2", "RL2S")][..]),
        (10, &[("RM4", "RM4S"), ("RL4", "RL4S")][..]),
        (11, &[("RM6", "RM6S"), ("RL6", "RL6S")][..]),
    ] {
        let slot = SEQUENTIAL | 0x2b8000 | 1 << (2 + select);
        let mut m = Machine::new(0, &[(slot, NOP)], &[(0, 2, 3), (1, 3, 0)], None);
        m.seed(&[("PACFG", 0x9999), ("PACFGS", 0x6666), ("ESTS", 0x2240), ("CETBL", 0x00ff)]);
        for (i, &(a, s)) in pairs.iter().enumerate() {
            m.seed(&[(a, 0x4100 + i as u64), (s, 0x5100 + i as u64)]);
        }
        m.start(0).unwrap();
        step_bank_without_data(&mut m, 1);
        for (i, &(a, s)) in pairs.iter().enumerate() {
            assert_eq!((m.reg(a), m.reg(s)), (0x5100 + i as u64, 0x4100 + i as u64));
        }
        let mask = if select < 8 { 3 << (2 * select) } else { 0 };
        assert_eq!((m.reg("PACFG"), m.reg("PACFGS")), (0x9999 ^ mask, 0x6666 ^ mask));
        assert_eq!((m.reg("EST"), m.reg("ESTS")), (0x1000, 0x2240));
    }
}

#[test]
fn compact_bank_exchange_orders_compare_and_restores_full_accumulators() {
    const BANK: u32 = 0x2b8006;
    const CMPU: u32 = 0x20ba02;
    for (fetch, alu, primary, prior, first, second) in [
        ((BANK, CMPU), true, true, 0, 0x2240, 0x0249),
        ((CMPU, BANK), true, false, 0, 0x2240, 0x0249),
        ((NOP, BANK), true, false, 0, 0x2240, 0x0249),
        ((SEQUENTIAL | NOP, BANK), true, true, 1, 0x2240, 0x0249),
        ((SEQUENTIAL | BANK, NOP), true, true, 0, 0x2240, 0x0249),
        ((BANK ^ 2, CMPU), false, true, 0, 0x0240, 0x0249),
        ((CMPU, BANK ^ 2), false, false, 0, 0x0009, 0x0009),
    ] {
        let sequential = fetch.0 & SEQUENTIAL != 0;
        let metadata = if sequential {
            &[(0, 2, 3), (1, 3, 6), (2, 1, 6), (4, 1, 0)][..]
        } else {
            &[(0, 1, 6), (2, 1, 6), (4, 1, 0)][..]
        };
        let mut m = Machine::new(0, &[fetch, (CMPU, NOP), (NOP, NOP)], metadata, None);
        let a2 = (0x1245623789, 0x9878765432);
        let a3 = (0x975312468a, 0x2468ace0ff);
        m.seed(&[("A0L", 5), ("A1L", 5), ("A2", a2.0), ("A2S", a2.1),
                 ("A3", a3.0), ("A3S", a3.1), ("ESTS", 0x2240), ("R0", 0x1234), ("R0S", 0x4567)]);
        m.start(0).unwrap();
        if prior != 0 {
            assert_eq!(m.step(), 3);
        }
        step_bank_without_data(&mut m, if sequential && prior == 0 { 1 } else { 2 });
        assert_eq!((m.reg("R0"), m.reg("R0S")), (0x4567, 0x1234));
        assert_eq!((m.reg("A2"), m.reg("A2S")), if alu && primary { (a2.1, a2.0) } else { a2 });
        assert_eq!((m.reg("A3"), m.reg("A3S")), if alu && !primary { (a3.1, a3.0) } else { a3 });
        assert_eq!((m.reg("EST"), m.reg("ESTS")), (first, if alu { 0 } else { 0x2240 }));
        if sequential && prior == 0 {
            assert_eq!(m.step(), 6);
        }
        assert_eq!(m.step(), 12);
        assert_eq!(m.reg("EST"), second);
    }
}

#[test]
fn compact_bank_exchange_leaves_qualified_and_unverified_peers_unlifted() {
    for packet in [(0x2b8007, NOP), (NOP, 0x2b8007), (0x2b8006, 0x315270)] {
        let mut m = Machine::new(0, &[packet], &[(0, 1, 0)], None);
        assert!(matches!(m.start(0), Err(KunaError::Unimpl { .. })));
    }
}

#[test]
fn compact_cmpu_samples_ce_before_publishing_its_flags() {
    const CMPU: u32 = 0x20ba02;
    for (fetch, metadata, steps, flags, refresh) in [
        ((CMPU, NOP), &[(0, 1, 6), (2, 1, 6), (4, 1, 0)][..], 1, 0x0009, 0x1009),
        ((NOP, CMPU), &[(0, 1, 6), (2, 1, 6), (4, 1, 0)][..], 1, 0x0240, 0x2240),
        ((SEQUENTIAL | CMPU, NOP), &[(0, 2, 3), (1, 3, 6), (2, 1, 6), (4, 1, 0)][..], 1, 0x0009, 0x1009),
        ((SEQUENTIAL | NOP, CMPU), &[(0, 2, 3), (1, 3, 6), (2, 1, 6), (4, 1, 0)][..], 2, 0x0009, 0x1009),
    ] {
        for (cfg1, cfg2, table, prior_ce) in [(0x222, 0x333, 0x8080, 0), (0, 0, 0xff00, 0x2000)] {
            let mut m = Machine::new(0, &[fetch, (NOP, NOP), (NOP, NOP)], metadata, None);
            m.seed(&[("A0L", 5), ("A1L", 5), ("CECFG1", cfg1), ("CECFG2", cfg2), ("CETBL", table)]);
            m.start(0).unwrap();
            for _ in 0..steps {
                m.step();
            }
            assert_eq!(m.reg("EST"), flags | prior_ce);
            m.step();
            assert_eq!(m.reg("EST"), if prior_ce == 0 { refresh } else { flags | prior_ce });
        }
    }
}

#[test]
fn wide_logic_flags_include_the_guard_even_when_the_word_result_is_zero() {
    let mut m = Machine::new(0, &[(0xc9c1a7, 0xea8222), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A0", 0xcc00000000), ("A1", 0), ("R0", 64), ("EST", 0x3000)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("R0"), m.reg("EST")), (0, 0x3030));
}

#[test]
fn compact_neg_of_ff12_uses_the_unsigned_double_word_input() {
    let mut m = Machine::new(0, &[(0x20c20c, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("FF1", 0x8001), ("FF2", 0xffff), ("EST", 0), ("ECFG", 0)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0"), m.reg("EST")), (0xff00007fff, 0x30));
}

#[test]
fn compact_address_update_uses_implicit_and_explicit_modulo_windows() {
    for (mode, incoming, increment, expected) in [
        (2, 0x0003, 1, 0x0004),
        (2, 0x4003, 1, 0x3ffc),
        (2, 0xffff, 1, 0),
        (3, 0x4003, 1, 0x4004),
        (3, 0x4007, 1, 0x4000),
        (3, 0x4000, 0xffff, 0x4007),
    ] {
        let mut m = Machine::new(0, &[(0x2c8080, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R0", incoming), ("RN0", increment), ("PACFG", mode), ("RM0", 7), ("RL0", 0x4000)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("R0"), expected, "mode {mode}, R0 {incoming:#x}, RN0 {increment:#x}");
        assert_eq!(m.reg("PACFG"), mode);
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

#[test]
fn tasking_wide_relative_call_reaches_the_linked_callee_and_restores_sp() {
    let code: String = include_str!("../../../../tests/fixtures/carmel/compiler-o2.hex")
        .split_whitespace()
        .collect();
    let callee = 0x2e;
    let mut fetches = vec![(NOP, NOP); (callee / 2 + 2) as usize];
    for pc in [0, 2, callee] {
        let at = (6 * pc) as usize;
        fetches[(pc / 2) as usize] = (
            u32::from_str_radix(&code[at..at + 6], 16).unwrap(),
            u32::from_str_radix(&code[at + 6..at + 12], 16).unwrap(),
        );
    }
    let caller_return = callee + 2;
    let issues = [(0, 5, 6), (2, 1, 6), (callee, 2, 3), (callee + 1, 3, 6), (caller_return, 1, 0)];
    let mut m = Machine::new(0, &fetches, &issues, None);
    for (x, expected) in [(-2i16, -1i16), (0, 1), (42, 43)] {
        m.seed(&[("A0L", u64::from(x as u16)), ("SP", 0x4400), ("ECFG", 0)]);
        m.seed_data(0x4401, caller_return);
        m.seed_data(0x4402, 0);
        m.start(0).unwrap();
        assert_eq!(m.step(), 3 * callee);
        assert_eq!((m.reg("SP"), m.data(0x43ff), m.data(0x4400)), (0x43fe, 2, 0));
        assert_eq!(m.step(), 3 * (callee + 1));
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("SP"), 0x4400);
        assert_eq!(m.step(), 3 * caller_return);
        assert_eq!(m.reg("SP"), 0x4402);
        assert_eq!(m.reg("A0L"), u64::from(expected as u16));
    }
}

#[test]
fn backtrace_clear_keeps_the_other_port_and_obeys_the_predicate() {
    const LEFT: u32 = 0x2da022;
    const RIGHT: u32 = 0x2dbffe;
    for (fetch, issues, expected) in [
        ((LEFT, NOP), &[(0, 1, 6), (2, 1, 0)][..], (0, 0xbf32)),
        ((NOP, RIGHT), &[(0, 1, 6), (2, 1, 0)][..], (0xacdb, 0)),
        ((SEQUENTIAL | NOP, LEFT), &[(0, 2, 3), (1, 3, 6), (2, 1, 0)][..], (0, 0xbf32)),
        ((0x2da003, NOP), &[(0, 1, 6), (2, 1, 0)][..], (0xacdb, 0xbf32)),
    ] {
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)], issues, None);
        m.seed(&[("BTR1", 0xacdb), ("BTR2", 0xbf32), ("CETBL", 0)]);
        m.start(0).unwrap();
        if issues[0].1 == 2 {
            assert_eq!(m.step(), 3);
            assert_eq!((m.reg("BTR1"), m.reg("BTR2")), (0xacdb, 0xbf32));
        }
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("BTR1"), m.reg("BTR2")), expected);
    }
}

#[test]
fn wide_load_initializes_the_pointer_without_overwriting_its_lower_bound() {
    let mut m = Machine::new(0, &[(0x8f0a01, 0xc276c2), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("R0", 0x40), ("RL0", 0x106), ("PACFG", 0), ("CETBL", 0xffff)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("R0"), m.reg("RL0"), m.reg("PACFG")), (0x0a01, 0x106, 1));
    assert_eq!(m.reg("EST") & 0x3f, 0);
}

#[test]
fn cliw_half_move_sign_extends_into_a_full_result_and_clears_carry() {
    let template = [0x7fffff22, 0x7f003fff, 0xff7ffff8];
    let mut m = Machine::new(0, &[CLIW, (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], Some(template));
    m.seed(&[("A0", 0x7612342744), ("A1", 0xff90001000), ("EST", 1)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.reg("A0"), 0xffffff9000);
    assert_eq!(m.reg("EST") & 0x3f, 0x10);
}

#[test]
fn compact_long_store_wraps_its_second_data_word_and_records_loss() {
    let mut m = Machine::new(0, &[(0x133321, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("A2", 0x40000000), ("A0", 0x40000003), ("R6", 0xffff), ("CETBL", 0xffff)]);
    for word in [0xfffe, 0xffff, 0, 1] {
        m.seed_data(word, 0xa55a);
    }
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.data(0xffff), m.data(0)), (3, 0x8000));
    assert_eq!((m.data(0xfffe), m.data(1), m.reg("EST") & 0x3f), (0xa55a, 0xa55a, 0x24));
}

#[test]
fn wide_transfer_adds_to_the_saved_code_low_word_without_rewriting_the_high_byte() {
    for (fetch, expected) in [
        ((0x80002b, 0xc04748), 0x10806b), // Saved LSA1 low word from R3 + 43.
        ((0xf46726, 0xc0f797), 0x103412), // Both transfer ports are disconnected.
    ] {
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("LSA1", 0x103412), ("LEA1", 0x20c452), ("R3", 0x8040), ("PACFG", 0)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("LSA1"), m.reg("LEA1"), m.reg("R3")), (expected, 0x20c452, 0x8040));
    }
}

#[test]
fn ce2_word_shift_publishes_carry_even_when_the_result_word_is_zero() {
    for (table, expected) in [(0, (0x8035, 0)), (0xffff, (0, 9))] {
        let mut m = Machine::new(0, &[(0x4ead81, 0xf181ed), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
        m.seed(&[("BTR1", 1), ("ECFG", 0x8035), ("CECFG2", 0x5605), ("EST", 0), ("CETBL", table)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("ECFG"), m.reg("EST") & 0x39), expected);
    }
}

#[test]
fn wide_long_io_separates_address_enable_from_bus_publication() {
    for (ce, config, expected) in [(0, 0, (0x4047, 0x96b5)), (0, 0x1000, (0x4041, 0x96b5)), (0xff00, 0, (0x4041, 0))] {
        let mut m = Machine::new(0, &[(0x400102, 0xc68140), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x4047), ("RM0", 7), ("PACFG", 2), ("CETBL", ce), ("CECFG2", config)]);
        m.state.borrow_mut().set_value(&m.io, 0x204, 2, 0x96b5).unwrap();
        m.state.borrow_mut().set_value(&m.ext, 0x204, 2, 0x43a6).unwrap();
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        let state = m.state.borrow();
        assert_eq!((state.get_value_by_name("R0").unwrap(), state.get_value(&m.io, 0x204, 2).unwrap()), expected);
        assert_eq!(state.get_value(&m.ext, 0x204, 2).unwrap(), 0x43a6);
    }
}

#[test]
fn wide_signed_unsigned_mac_publishes_the_latch_without_clamping_the_result() {
    let mut m = Machine::new(0, &[(0xf86df3, 0xcb8d88), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A3", 0x007fffffff), ("RL0", 0xffff), ("RN5", 1), ("ECFG", 0x44), ("EST", 0)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0"), m.reg("X1"), m.reg("EST") & 0x3f), (0x0080000000, 0xffff, 0x20));
}

#[test]
fn wide_mac_reads_all_24_saved_code_address_bits_as_unsigned() {
    let mut m = Machine::new(0, &[(0xf86df2, 0xcb8dd4), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("LSA1", 0xfef876), ("RL0", 3), ("RN5", 7), ("ECFG", 0x40), ("EST", 0)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("LC1"), m.reg("LSA1"), m.reg("X1"), m.reg("EST") & 0x3f), (0xfe, 0xfef876, 3, 1));
}

#[test]
fn compact_return_retires_only_its_counted_issues_and_pops_the_link() {
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 0)];
    for delay in [2, 3] {
        let mut m = Machine::new(0, &[
            (RETURN | (delay << 1), NOP), (INC_A0, NOP), (INC_A0, NOP),
            (INC_A0, NOP), (INC_A1, NOP), (NOP, NOP),
        ], &issues, None);
        m.seed(&[("A0L", 0x1234), ("A1L", 0x5678), ("SP", 0x4100)]);
        m.seed_data(0x4101, 10);
        m.seed_data(0x4102, 0);
        m.start(0).unwrap();
        assert_eq!(m.step(), 30);
        assert_eq!((m.reg("SP"), m.reg("A0L"), m.reg("A1L")),
                   (0x4102, 0x1234 + u64::from(delay), 0x5678));
    }
}

#[test]
fn compact_ce1_return_pops_for_address_enable_even_with_execution_vetoed() {
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 0)];
    for (table, ce_cfg, target, sp) in [(0, 0, 6, 0x4100), (0, 0x1000, 12, 0x4102), (0xff, 0, 12, 0x4102)] {
        let mut m = Machine::new(0, &[(RETURN | 1, NOP), (NOP, NOP), (NOP, NOP)], &issues, None);
        m.seed(&[("CECFG1", ce_cfg), ("CETBL", table), ("EST", 0), ("SP", 0x4100)]);
        m.seed_data(0x4101, 4);
        m.seed_data(0x4102, 0);
        m.start(0).unwrap();
        assert_eq!((m.step(), m.reg("SP")), (target, sp));
    }
}

#[test]
fn wide_goabs_samples_the_payload_before_its_five_delay_issues() {
    let base = 0x11000;
    let mut fetches = [(INC_A1, NOP); 14];
    fetches[0] = (0x880000, 0xcf7802);
    fetches[1] = (INC_A0, NOP);
    let issues = [(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6),
                  (10, 1, 6), (12, 1, 6), (14, 1, 6), (24, 1, 6), (26, 1, 0)];
    for target in [base + 24, base + 26] {
        let mut m = Machine::new(base, &fetches, &issues, None);
        m.seed(&[("A0", (0x81 << 32) | target), ("A1L", 0x1234)]);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * target);
        assert_eq!((m.reg("A0L"), m.reg("A1L")), ((target + 1) & 0xffff, 0x1238));
    }
}

#[test]
fn wide_goabs_does_not_wrap_an_out_of_range_payload_into_code() {
    let base = 0x11000;
    let mut m = Machine::new(base, &[(0x880000, 0xcf4002), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    let target = 0x1000000 | (base + 2);
    m.seed(&[("A0", target)]);
    m.start(base).unwrap();
    assert!(m.emulator.execute_instruction().is_err(), "the wrapped address would execute a mapped NOP");
    assert_eq!(m.emulator.get_execute_address().get_offset(), 3 * target);
}

#[test]
fn wide_callabs_reads_both_data_words_and_returns_to_the_encoded_link() {
    let mut fetches = [(INC_A0, NOP); 16];
    fetches[0] = (0x800000, 0xcf7c01);
    fetches[12] = (RETURN, NOP);
    fetches[15] = (RETURN, NOP);
    let issues = [(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 6),
                  (12, 1, 6), (14, 1, 6), (16, 1, 6), (24, 1, 6), (30, 1, 0)];
    for (base, offset) in [(0x11000, 24), (0x21000, 30)] {
        let mut m = Machine::new(base, &fetches, &issues, None);
        let target = base + offset;
        let link = base + 16;
        m.seed(&[("R0", 0x4200), ("SP", 0x4100), ("A0L", 0x1234)]);
        m.seed_data(0x4200, target & 0xffff);
        m.seed_data(0x4201, target >> 16);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * target);
        assert_eq!((m.reg("R0"), m.reg("SP"), m.reg("A0L")), (0x4200, 0x40fe, 0x1239));
        assert_eq!((m.data(0x40ff), m.data(0x4100)), (link & 0xffff, link >> 16));
        assert_eq!(m.step(), 3 * link);
        assert_eq!(m.reg("SP"), 0x4100);
    }
}

#[test]
fn compact_callabs_selects_its_encoded_source_before_delay_updates_a0() {
    let base = 0x11000;
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6),
                  (8, 1, 6), (24, 1, 6), (30, 1, 0)];
    for (op, target) in [(0x2bca06, base + 24), (0x2bcb06, base + 30)] {
        let mut fetches = [(INC_A0, NOP); 16];
        fetches[0] = (op, NOP);
        fetches[12] = (RETURN, NOP);
        fetches[15] = (RETURN, NOP);
        let mut m = Machine::new(base, &fetches, &issues, None);
        m.seed(&[("R0", 0x4200), ("SP", 0x4100), ("A0", (0x81 << 32) | (base + 30))]);
        m.seed_data(0x4200, 0x1018);
        m.seed_data(0x4201, 1);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * target);
        assert_eq!((m.reg("A0L"), m.reg("SP")), (0x1021, 0x40fe));
        assert_eq!(m.step(), 3 * (base + 8));
        assert_eq!(m.reg("SP"), 0x4100);
    }
}

#[test]
fn compact_goabs_combines_the_two_data_words_without_pushing_a_link() {
    let base = 0x21000;
    let target = base + 24;
    let mut fetches = [(INC_A0, NOP); 13];
    fetches[0] = (0x2bc806, NOP);
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (24, 1, 0)];
    let mut m = Machine::new(base, &fetches, &issues, None);
    m.seed(&[("R0", 0x4200), ("SP", 0x4100), ("A0L", 0x207)]);
    m.seed_data(0x4200, target & 0xffff);
    m.seed_data(0x4201, target >> 16);
    m.seed_data(0x40ff, 0x1415);
    m.seed_data(0x4100, 0x2618);
    m.start(base).unwrap();
    assert_eq!(m.step(), 3 * target);
    assert_eq!((m.reg("R0"), m.reg("SP"), m.reg("A0L")), (0x4200, 0x4100, 0x20a));
    assert_eq!((m.data(0x40ff), m.data(0x4100)), (0x1415, 0x2618));
}

#[test]
fn compact_ce1_callabs_only_stores_a_link_when_execution_is_enabled() {
    let base = 0x11000;
    let target = base + 6;
    let fetches = [(0x2bcb01, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP)];
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)];
    for (table, cfg, take, write) in [(0, 0, false, false), (0xffff, 0, true, true),
                                      (0, 0x1000, true, false), (0xffff, 0x1000, true, true)] {
        let mut m = Machine::new(base, &fetches, &issues, None);
        m.seed(&[("A0", (0x81 << 32) | target), ("EST", 0), ("CETBL", table), ("CECFG1", cfg), ("SP", 0x4100)]);
        m.seed_data(0x40ff, 0x1415);
        m.seed_data(0x4100, 0x2618);
        m.start(base).unwrap();
        assert_eq!(m.step(), 3 * if take { target } else { base + 2 });
        assert_eq!(m.reg("SP"), if take { 0x40fe } else { 0x4100 });
        assert_eq!((m.data(0x40ff), m.data(0x4100)), if write { (0x1002, 1) } else { (0x1415, 0x2618) });
    }
}

#[test]
fn reti_consumes_nested_traps_before_clearing_unstacked_status() {
    let mut fetches = [(NOP, NOP); 17];
    fetches[0] = (0x2bd001, NOP);
    fetches[1] = (0x2bd810, NOP);
    fetches[4] = (0x2bd002, NOP);
    fetches[5] = (INC_A1, NOP);
    fetches[6] = (0x2bd810, NOP);
    let issues = [(0, 1, 6), (2, 1, 6), (8, 1, 6), (10, 1, 6), (12, 1, 6), (32, 1, 0)];
    let mut m = Machine::new(0, &fetches, &issues, None);
    m.seed(&[("INTBASE", 0), ("TRAPCTX", 0), ("SP", 0x4040), ("PSTATUS", 0x10)]);
    m.seed_data(0x4041, 32);
    m.seed_data(0x4042, 0);
    m.start(0).unwrap();
    assert_eq!(m.step(), 3 * 8);
    assert_eq!((m.reg("TRAPCTX"), m.reg("SP"), m.data(0x403f)), (1, 0x403e, 2));
    assert_eq!(m.step(), 3 * 12);
    assert_eq!((m.reg("TRAPCTX"), m.reg("SP"), m.data(0x403d)), (6, 0x403c, 10));
    assert_eq!(m.step(), 3 * 10);
    assert_eq!((m.reg("TRAPCTX"), m.reg("SP"), m.reg("PSTATUS")), (1, 0x403e, 0x10));
    assert_eq!(m.step(), 3 * 12);
    assert_eq!(m.step(), 3 * 2);
    assert_eq!((m.reg("TRAPCTX"), m.reg("SP"), m.reg("PSTATUS"), m.reg("A1L")), (0, 0x4040, 0x10, 1));
    assert_eq!(m.step(), 3 * 32);
    assert_eq!((m.reg("SP"), m.reg("PSTATUS")), (0x4042, 0));
}

#[test]
fn reti_ce1_veto_preserves_the_active_frame_and_address_enable_consumes_it() {
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 0)];
    for (mode, dst, sp, context) in [(0, 2, 0x4040, 6), (0x1000, 4, 0x4042, 1)] {
        let mut m = Machine::new(0, &[(0x2bd811, NOP), (NOP, NOP), (NOP, NOP)], &issues, None);
        m.seed(&[("PSTATUS", 0x10), ("SP", 0x4040), ("TRAPCTX", 6), ("CECFG1", mode), ("CETBL", 0)]);
        m.seed_data(0x4041, 4);
        m.seed_data(0x4042, 0);
        m.start(0).unwrap();
        assert_eq!(m.step(), 3 * dst);
        assert_eq!((m.reg("SP"), m.reg("TRAPCTX"), m.reg("PSTATUS")), (sp, context, 0x10));
    }
}
