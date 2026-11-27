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
        Self::with_root_context(base, fetches, issues, template, &[])
    }

    fn with_root_context(base: u64, fetches: &[(u32, u32)], issues: &[(u64, u32, u32)],
                         template: Option<[u32; 3]>, root_fields: &[(&[u8], u32)]) -> Self {
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
            let (start, end) = (address(0), address(1));
            for &(field, value) in root_fields {
                db.set_variable_region(field, &start, &end, value).unwrap();
            }
            if let Some([high, mid, low]) = template {
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
            let capacity = if name == "unique" { 512 } else { 128 };
            state.set_memory_bank(Rc::new(RefCell::new(MemoryHashOverlay::new(
                space, alignment, 64, capacity, None,
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
fn compact_half_publication_samples_both_ce_rails_before_lane_writeback() {
    for (fetch, mode, first, flags, next_rails) in [
        ((INC_A0, NOP), 1, 0, 0x0009, 0x3000),
        ((NOP, INC_A0), 1, 0, 0x0240, 0),
        ((INC_A0, 0x21ac62), 1, 0, 0x0249, 0x1000),
        ((INC_A0, 0x31aa20), 1, 0, 0x0409, 0x3000),
        ((SEQUENTIAL | INC_A0, NOP), 2, 0, 0x0009, 0x3000),
        ((SEQUENTIAL | NOP, INC_A0), 2, 1, 0x0009, 0x3000),
    ] {
        let (next, right_mode) = if mode == 1 { (6, 0) } else { (3, 3) };
        let mut m = Machine::new(
            0, &[fetch, (NOP, NOP), (NOP, NOP)], &[(0, mode, next), (1, right_mode, 6), (2, 1, 6), (4, 1, 0)], None,
        );
        m.seed(&[("A0L", 0xffff), ("A1L", 0xffff), ("CECFG1", 0x0888), ("CECFG2", 0x0999), ("CETBL", 0x0180)]);
        m.start(first).unwrap();
        m.step();
        assert_eq!(m.reg("EST"), 0x2000 | flags, "{fetch:x?}");
        m.step();
        assert_eq!(m.reg("EST"), next_rails | flags, "{fetch:x?}");
    }
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
fn signed_compact_compare_preserves_flags_and_samples_incoming_feedback() {
    for (fetch, a, b, ce, old, after, next) in [
        ((0x209a02, NOP), 7, 7, 0x0888, 0x4fe7, 0x4fed, 0x7fed),
        ((NOP, 0x209a02), 0x8000, 1, 0x0999, 0x4fff, 0x7d7f, 0x4d7f),
    ] {
        let mut m = Machine::new(0, &[fetch, (NOP, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 6), (4, 1, 0)], None);
        m.seed(&[("A0L", a), ("A1L", b), ("CETBL", 0x8080), ("CECFG1", ce), ("CECFG2", ce), ("EST", old)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("EST"), after);
        assert_eq!(m.step(), 12);
        assert_eq!(m.reg("EST"), next);
    }
}

#[test]
fn compact_status_move_uses_agu_and_restores_the_saved_loop_frame() {
    for (placement, raw, value, ce1, ce2, table, status, pointer) in [
        (0, 0x31b541, 0x000d, 0x1000, 0, 0, 0xe009, 0x100),
        (0, 0x31b541, 0x000c, 0x1000, 0, 0, 0xe00c, 0x100),
        (0, 0x31b541, 0xffff, 0x1000, 0, 0, 0xe01b, 0x100),
        (0, 0x31b541, 0x0004, 0x1000, 0, 0, 0xe000, 0x100),
        (0, 0x31b541, 0x0009, 0, 0x1000, 0, 0xe010, 0x100),
        (1, 0x31b541, 0x0009, 0, 0x1000, 0, 0xe009, 0x100),
        (1, 0x31b541, 0x0009, 0x1000, 0, 0, 0xe010, 0x100),
        (2, 0x31b541, 0x0009, 0x1000, 0, 0, 0xe009, 0x100),
        (2, 0x31b541, 0x0009, 0, 0x1000, 0, 0xe010, 0x100),
        (0, 0x31b540, 0x0009, 0, 0, 0, 0xe009, 0x100),
        (0, 0x31b551, 0x0009, 0x1000, 0, 0, 0xe009, 0x103),
        (0, 0x31b561, 0x0009, 0x1000, 0, 0, 0xe009, 0x101),
        (0, 0x31b571, 0x0009, 0x1000, 0, 0, 0xe009, 0xff),
    ] {
        let fetch = match placement {
            0 => (raw, NOP),
            1 => (NOP, raw),
            _ => (NOP | 0x800000, raw),
        };
        let parallel = [(0, 1, 6), (2, 1, 0)];
        let sequential = [(0, 2, 3), (1, 3, 3), (2, 1, 0)];
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)],
                                 if placement == 2 { &sequential[..] } else { &parallel[..] }, None);
        m.seed(&[("PSTATUS", 0xe010), ("LC", 7), ("LSAH", 2), ("LSAL", 0x1010),
                 ("LEAH", 2), ("LEAL", 0x1018), ("LC1", 0xabcd), ("LSA1W", 0x442238),
                 ("LEA1W", 0x55323a), ("R0", 0x100), ("RN0", 3),
                 ("CECFG1", ce1), ("CECFG2", ce2), ("CETBL", table)]);
        m.seed_data(0x100, value);
        m.seed_data(0x101, 0x1b);
        m.seed_data(0x103, 0x1b);
        m.seed_data(0xff, 0x1b);
        m.start(0).unwrap();
        if placement == 2 {
            assert_eq!(m.step(), 3);
        }
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("PSTATUS"), m.reg("R0")), (status, pointer));
        let frame = (m.reg("LC"), m.reg("LSAH"), m.reg("LSAL"), m.reg("LEAH"), m.reg("LEAL"));
        assert_eq!(frame, if status == 0xe010 { (7, 2, 0x1010, 2, 0x1018) } else { (0xabcd, 0x44, 0x2238, 0x55, 0x323a) });
        assert_eq!(m.data(0x100), value);
    }
}

#[test]
fn explicit_status_loads_drive_the_next_conditional_transfer() {
    let mut pop = Machine::new(0, &[(0xb02642, 0x281c0c), (INC_A2, NOP), (0x21ace6, NOP), (NOP, NOP)],
                               &[(0, 2, 3), (1, 3, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None);
    pop.seed(&[("SP", 0xffff), ("EST", 0x10)]);
    pop.seed_data(0, 0x8009);
    pop.seed_data(1, 0x4567);
    pop.start(0).unwrap();
    assert_eq!(pop.step(), 3);
    assert_eq!((pop.reg("SP"), pop.reg("EST"), pop.data(1)), (0, 9, 0x4567));
    assert_eq!(pop.step(), 12);

    let mut load = Machine::new(0, &[(0xbf1234, 0xc36304), (0x281c10, NOP), (INC_A2, NOP), (0x21ace6, NOP)],
                                &[(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None);
    load.seed(&[("SP", 0x4000), ("EST", 0x10)]);
    load.seed_data(0x1234, 0x8009);
    load.start(0).unwrap();
    assert_eq!(load.step(), 6);
    assert_eq!((load.reg("SP"), load.reg("EST"), load.data(0x1234)), (0x4000, 9, 0x8009));
    assert_eq!(load.step(), 18);
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
fn wide_signed_move_data_mux() {
    // Seed nonzero arithmetic flags as if an earlier ALU issue retired.
    for (fetch, cfg, table, ecfg, dst, expected, pointers, status) in [
        ((0xc6b222, 0xc3611c), (0, 0), 0, 0, 0xc048, (0x8123, 0xffff), (0x8048, 0xc048), 0x24),
        ((0x960040, 0xc3615c), (0, 0), 0, 0x20, 0xc048, (0xc091, 0xffff), (0x8049, 0xc04a), 0x24),
        ((0x960040, 0xc3615d), (0, 0), 0, 0, 0xc048, (0x8123, 0xa456), (0x804a, 0xc04a), 0x24),
        ((0x960040, 0xc3605d), (0, 0), 0, 0, 0xc048, (0xa456, 0x5678), (0x804a, 0xc049), 0x24),
        ((0x160040, 0xc3615c), (0, 0), 0, 0, 0xc048, (0x1234, 0x5678), (0x8048, 0xc048), 0x24),
        ((0x160040, 0xc3615c), (0x1000, 0), 0, 0, 0xc048, (0x1234, 0x5678), (0x8049, 0xc04a), 0x24),
        ((0x160040, 0xc3615c), (0, 0), 0xffff, 0, 0xc048, (0x8123, 0xffff), (0x8049, 0xc04a), 0x3024),
        ((0x560040, 0xc3615c), (0x1000, 0), 0xff00, 0, 0xc048, (0x8123, 0xffff), (0x8049, 0xc04a), 0x2024),
        ((0x924030, 0xc3615f), (0, 0), 0, 0, 0xc048, (0x2345, 0x81), (0x8048, 0xc04a), 0x24),
        ((0x93a55a, 0xc3615f), (0, 0), 0, 0x20, 0xc048, (0xa291, 0x49), (0x8048, 0xc04a), 0x24),
        ((0x96a55a, 0xc3615d), (0, 0), 0, 0, 0xffff, (0x8123, 0xa456), (0x804a, 1), 0x24),
    ] {
        let mode = if fetch.0 & 0x800000 == 0 { 4 } else { 5 };
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None);
        m.seed(&[("R6", 0x8048), ("R7", dst), ("RN6", 3), ("RN7", 4),
                 ("CECFG1", cfg.0), ("CECFG2", cfg.1), ("CETBL", table),
                 ("ECFG", ecfg), ("EST", 0x24), ("LSA1", 0x812345), ("LEA1", 0x934522)]);
        let upper = (dst + 1) & 0xffff;
        let sentinel = (dst + 2) & 0xffff;
        for (address, word) in [(0x8048, 0x8123), (0x8049, 0xa456), (dst, 0x1234),
                                (upper, 0x5678), (sentinel, 0xdead)] {
            m.seed_data(address, word);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(dst), m.data(upper)), expected);
        assert_eq!((m.reg("R6"), m.reg("R7")), pointers);
        assert_eq!((m.reg("EST"), m.reg("LSA1"), m.reg("LEA1")), (status, 0x812345, 0x934522));
        assert_eq!((m.data(0x8048), m.data(0x8049), m.data(sentinel)), (0x8123, 0xa456, 0xdead));
    }
    for (fetch, mode) in [((0x0f0040, 0xc3615e), 4), ((0x3d0040, 0xc3615e), 4), ((0x9655aa, 0xc36159), 5)] {
        let mut m = Machine::new(0, &[fetch], &[(0, mode, 0)], None);
        assert!(matches!(m.start(0), Err(KunaError::Unimpl { .. })));
    }
}

#[test]
fn wide_signed_move_register_mux() {
    for (acc, ecfg, initial, result, status) in [
        (0x000000abcd, 0, 0, 0x0000000000, 0x0008),
        (0x0081234321, 0, 0, 0x0081238123, 0x0024),
        (0x0081234321, 4, 0, 0x0081238123, 0x0020),
        (0xff00004321, 0, 0, 0xff00000000, 0x0034),
        (0xff00004321, 8, 0x7fff, 0xff00000000, 0x7ff4),
    ] {
        let mut m = Machine::new(0, &[(0x882983, 0xc36642), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0", acc), ("ECFG", ecfg), ("EST", initial),
                 ("CECFG1", 0x888), ("CECFG2", 0x888), ("CETBL", 0x8080)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST")), (result, status));
    }

    for (left, right, table, cfg, dest, result, r0, r1, status) in [
        (0x112983, 0xc3667a, 0, 0, "X1", 0x1357, 0x3300, 0x7300, 0x4fff),
        (0x112983, 0xc3667a, 0, 0x1000, "X1", 0x7654, 0x3300, 0x7300, 0x4fff),
        (0x192983, 0xc3665a, 0xff00, 0x1000, "FF1", 0x8b98, 0x3300, 0x7300, 0x6fff),
        (0x412983, 0xc3667c, 0, 0, "X2", 0x2468, 0x3300, 0x7300, 0x4fff),
        (0x412983, 0xc3667c, 0, 0x1000, "X2", 0xd256, 0x3300, 0x7300, 0x4fff),
        (0x902983, 0xc3665c, 0, 0, "FF2", 0xa123, 0x3301, 0x7300, 0x4fff),
        (0x912983, 0xc36658, 0, 0, "FF1", 0xd256, 0x3300, 0x7301, 0x4fff),
        (0x992983, 0xc3667e, 0, 0, "X2", 0x8b98, 0x3300, 0x7300, 0x4fff),
    ] {
        let mode = if left & 0x800000 == 0 { 4 } else { 5 };
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x007ffe4321), ("A1", 0x7e8b987654), ("R0", 0x3300), ("R1", 0x7300),
                 ("X1", 0x1357), ("X2", 0x2468), ("FF1", 0x3579), ("FF2", 0xcafe),
                 ("ECFG", 0x20), ("EST", 0x7fff), ("CECFG1", cfg), ("CECFG2", cfg), ("CETBL", table)]);
        m.seed_data(0x3300, 0xa123);
        m.seed_data(0x7300, 0xd256);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(dest), m.reg("R0"), m.reg("R1"), m.reg("EST")), (result, r0, r1, status));
        assert_eq!((m.data(0x3300), m.data(0x7300)), (0xa123, 0xd256));
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
fn direct_repeat_zero_skips_only_the_first_of_the_selected_issues() {
    for (delay, count, expected, literal) in [
        (2u32, 3u32, [1, 1, 1], false), (3, 3, [3, 1, 1], false), (4, 3, [3, 3, 1], false),
        (5, 0, [0, 1, 1], false), (5, 3, [3, 3, 3], false), (7, 3, [3, 3, 3], false),
        (3, 3, [3, 1, 1], true), (5, 0, [0, 1, 1], true), (5, 3, [3, 3, 3], true),
    ] {
        let body = delay.saturating_sub(2);
        let (left, right) = if literal { (0xb80000 | count, 0xcf0400) } else { (0xb84100, 0xcf0000) };
        let (address, data) = if literal { (u64::from(count), 7) } else { (0x4100, u64::from(count)) };
        let mut m = Machine::with_root_context(
            0,
            &[(left, right | delay << 11), (INC_A0, NOP), (INC_A1, NOP), (INC_A2, NOP),
              (0x21ace6, NOP), (0x21ad28, NOP), (0x21ad6a, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP)],
            &[(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6),
              (10, 1, 6), (12, 1, 6), (14, 1, 6), (16, 1, 6), (18, 1, 0)],
            None,
            &[(b"carmel_direct_repeat", body + 1)],
        );
        m.seed_data(address, data);
        m.start(0).unwrap();
        let end = 2 + 2 * (3 + body.max(1));
        assert_eq!(m.step(), u64::from(end) * 3);
        for _ in (end..14).step_by(2) {
            m.step();
        }
        assert_eq!([m.reg("A0L"), m.reg("A1L"), m.reg("A2L")], [1, 1, 1]);
        assert_eq!([m.reg("A3L"), m.reg("A4L"), m.reg("A5L")], expected);
        assert_eq!(m.data(address), data);
    }
}

#[test]
fn direct_repeat_zero_publishes_ce_without_running_the_annulled_issue() {
    for (delay, body, empty, once) in [(0u32, 0, 0x1000, 0x1009), (3, 1, 0x1000, 0x1009), (5, 3, 0x2000, 0x3009)] {
        for (literal, count) in [(false, 0u32), (false, 1), (true, 0), (true, 1)] {
            let branch = if literal { (0xb80000 | count, 0xcf0400) } else { (0xb84100, 0xcf0000) };
            let mut fetches = [(NOP, NOP); 9];
            fetches[0] = (branch.0, branch.1 | delay << 11);
            fetches[4] = (INC_A0, NOP);
            let mut m = Machine::with_root_context(
                0, &fetches,
                &[(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6),
                  (10, 1, 6), (12, 1, 6), (14, 1, 6), (16, 1, 0)],
                None, &[(b"carmel_direct_repeat", body + 1)],
            );
            m.seed(&[("A0L", 0xffff), ("CECFG1", 0x08ed), ("CECFG2", 0x08cd), ("CETBL", 0xe132)]);
            m.seed_data(0x4100, u64::from(count));
            m.start(0).unwrap();
            assert_eq!(m.step(), 6 * u64::from(4 + body.max(1)));
            let expected = if count == 0 { (empty, 0xffff) } else { (once, 0) };
            assert_eq!((m.reg("EST"), m.reg("A0L")), expected, "{delay}, {count}, {literal}");
        }
    }
}

#[test]
fn immediate_repeat_publishes_ce_at_setup_and_annul_without_an_extra_iteration() {
    for (mode, right) in [(1, false), (1, true), (2, false), (3, true), (5, false)] {
        for (count, est) in [(0u32, 0x1000), (1, 0x1009), (2, 0x3000)] {
            let repeat = 0x2bc000 | count;
            let root = match mode {
                2 => (SEQUENTIAL | repeat, INC_A0),
                3 => (SEQUENTIAL | NOP, repeat),
                5 => (0x800000 | count, 0xcef800),
                _ if right => (NOP, repeat),
                _ => (repeat, NOP),
            };
            let next = if mode == 2 { (NOP, NOP) } else { (INC_A0, NOP) };
            let mode0 = if mode == 3 { 2 } else { mode };
            let (next_size, mode1) = if mode0 == 2 { (3, 3) } else { (6, 0) };
            let mut m = Machine::new(
                0, &[root, next, (NOP, NOP), (NOP, NOP)],
                &[(0, mode0, next_size), (1, mode1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None,
            );
            m.seed(&[("A0L", 0xffff), ("CECFG1", 0x08ed), ("CECFG2", 0x08cd), ("CETBL", 0xe132)]);
            m.start(u64::from(mode == 3)).unwrap();
            assert_eq!(m.step(), if mode == 2 { 6 } else { 12 });
            assert_eq!((m.reg("EST"), m.reg("A0L")), (est, u64::from(count.wrapping_sub(1) & 0xffff)),
                       "{mode}, {right}, {count}");
        }
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
fn direct_goto_publishes_ce_on_idle_refill_clocks() {
    for (wide, delay, sequential, est) in [
        (false, 0, false, 0x2000),
        (false, 1, false, 0x3009),
        (false, 1, true, 0x2000),
        (false, 2, true, 0x1009),
        (false, 3, true, 0x1009),
        (true, 0, false, 0x2000),
        (true, 1, false, 0x3009),
        (true, 1, true, 0x2000),
        (true, 2, true, 0x1009),
        (true, 7, true, 0x1009),
    ] {
        let branch = if wide { (0x80000c, 0xcf8000 | delay << 11) } else { (goto(12, delay, false), NOP) };
        let (slot, mode, size, right_mode) = if sequential { ((SEQUENTIAL | NOP, INC_A0), 2, 3, 3) }
            else { ((INC_A0, NOP), 1, 6, 0) };
        let mut fetches = [(NOP, NOP); 8];
        fetches[0] = branch;
        fetches[1] = slot;
        let mut m = Machine::new(0, &fetches,
            &[(0, if wide { 5 } else { 1 }, size), (2, mode, size), (3, right_mode, 6),
              (4, 1, 6), (6, 1, 6), (12, 1, 6), (14, 1, 0)], None);
        m.seed(&[("A0L", 0xffff), ("CECFG1", 0x08ed), ("CECFG2", 0x08cd), ("CETBL", 0xe132)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 36);
        let a0 = if delay > u32::from(sequential) { 0 } else { 0xffff };
        assert_eq!((m.reg("EST"), m.reg("A0L")), (est, a0), "{wide}, {delay}, {sequential}");
    }
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
    for (rhs, target, sp) in [(1, 10, 0x4002), (2, 4, 0x4004)] {
        let mut m = Machine::new(
            0,
            &[(0x209a02, NOP), (0x289c20, NOP), (INC_A0, NOP), (INC_A1, NOP),
              (INC_A2, NOP), (0x21ace6, NOP), (NOP, NOP)],
            &[(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 6), (12, 1, 0)],
            None,
        );
        m.seed(&[("A0L", 1), ("A1L", rhs), ("SP", 0x4004)]);
        m.seed_data(0x4003, 10);
        m.seed_data(0x4004, 0x9876);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.step(), 3 * target);
        assert_eq!((m.reg("SP"), m.data(0x4003), m.data(0x4004)), (sp, 4, 0));
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
fn disconnected_wide_link_uses_selected_ce_and_stores_at_old_sp() {
    let old_sp = 0x8030;
    let new_sp = 0xd24e;
    for (left, selected, unselected, bit) in [
        (0x2d521e, "CECFG1", "CECFG2", 0x0001),
        (0x6d521e, "CECFG2", "CECFG1", 0x0100),
    ] {
        for (cfg, table, sp, saved, flags) in [
            (0, 0x0101 ^ bit, old_sp, 0xbe27, 0),
            (0x1000, 0x0101 ^ bit, new_sp, 0xbe27, 0),
            (0, bit, new_sp, 0xffff, 0x4000),
        ] {
            let mut m = Machine::new(0, &[(left, 0xc62fc0), (NOP, NOP)],
                                     &[(0, 4, 6), (2, 1, 0)], None);
            m.seed(&[("SP", old_sp), (selected, cfg), (unselected, 0x1000), ("CETBL", table),
                     ("ECFG", 0x10), ("R0", 0x3456), ("A0", 0x123456789a)]);
            m.seed_data(old_sp, 0xbe27);
            m.seed_data(new_sp, 0x932d);
            m.start(0).unwrap();
            assert_eq!(m.step(), 6);
            assert_eq!((m.reg("SP"), m.data(old_sp), m.data(new_sp)), (sp, saved, 0x932d));
            assert_eq!((m.reg("R0"), m.reg("A0")), (0x3456, 0x123456789a));
            assert_eq!((m.reg(selected), m.reg(unselected), m.reg("CETBL")), (cfg, 0x1000, table));
            assert_eq!(m.reg("EST") & 0xcfff, flags);
        }
    }
}

#[test]
fn link_rechecks_the_store_predicate_after_writing_ce_controls() {
    for (left, right, reg, sp, table, saved, flags) in [
        (0x3ffffc, 0xc6030b, "CECFG1", 0x6011, 0x0101, 0x5aa5, 0x2000),
        (0x7ffffc, 0xc6030b, "CECFG1", 0x6011, 0x0101, 0x1000, 0x6000),
        (0x3ffffc, 0xc6030f, "CECFG2", 0x6011, 0x0101, 0x1000, 0x5000),
        (0x7ffffc, 0xc6030f, "CECFG2", 0x6011, 0x0101, 0x5aa5, 0x1000),
        (0x3ffffc, 0xc60313, "CETBL", 0x8030, 0, 0, 0x1000),
        (0x7ffffc, 0xc60313, "CETBL", 0x8030, 0, 0x5aa5, 0x1000),
        (0x3ffffc, 0xc60313, "CETBL", 0x8030, 0xffff, 0xffff, 0x5000),
        (0x7ffffc, 0xc60313, "CETBL", 0x8030, 0xffff, 0x5aa5, 0x1000),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP), (NOP, NOP)],
                                 &[(0, 4, 6), (2, 1, 0), (4, 1, 0)], None);
        m.seed(&[("SP", sp), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", table), ("ECFG", 0x10)]);
        m.seed_data(sp, 0x5aa5);
        m.seed_data(sp + 1, 0xabcd);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("SP"), m.reg(reg), m.data(sp)), (sp - 4, sp - 1, saved));
        m.step();
        assert_eq!((m.reg("EST"), m.data(sp + 1)), (flags, 0xabcd));
    }
}

#[test]
fn link_ce_control_write_cannot_create_its_own_address_enable() {
    let mut m = Machine::new(0, &[(0x3ffffc, 0xc60313), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
    m.seed(&[("SP", 0x6011), ("CETBL", 0), ("CECFG1", 0), ("CECFG2", 0)]);
    m.seed_data(0x6011, 0x5aa5);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("SP"), m.reg("CETBL"), m.data(0x6011)), (0x6011, 0, 0x5aa5));
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
fn wide_binary_unbound_ports() {
    let machine = |left, right, mode| {
        Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None)
    };
    for (table, ce) in [(0, 0), (0xffff, 0x3000)] {
        let mut m = machine(0xa977e8, 0x87a6db, 5);
        m.seed(&[("CETBL", table), ("EST", 0x29), ("R5", 0x4048), ("RN5", 2)]);
        m.seed_data(0x404a, 0x5dda);
        m.seed_data(0x404b, 0xcbfe);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("R5"), m.data(0x404a), m.data(0x404b)), (ce | 0x29, 0x4048, 0x5dda, 0xcbfe));

        let mut m = machine(0x1c2609, 0xe26360, 4);
        m.seed(&[("CETBL", table), ("CECFG1", 0x1000), ("EST", 0x24), ("R4", 0x48), ("A0", 0xea433809a4)]);
        m.seed_data(0x47, 0xffff);
        m.seed_data(0x48, 0x8001);
        m.seed_data(0x49, 0x1234);
        m.seed_data(0xffff, 0xcafe);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        let (acc, est) = if table != 0 { (0xea800109a4, ce | 4) } else { (0xea433809a4, 0x24) };
        assert_eq!((m.reg("A0"), m.reg("EST"), m.reg("R4")), (acc, est, 0x47));

        let mut m = machine(0xa42532, 0xe761d6, 5);
        m.seed(&[("CETBL", table), ("EST", 0x24), ("R4", 0x48), ("R5", 0x4048)]);
        m.seed_data(0x4048, 0x71bb);
        m.seed_data(0x4049, 0x03a3);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("R4"), m.reg("R5"), m.data(0x4048)), (ce | 4, 0x48, 0x407a, 0x71bb));

        let mut m = machine(0x9177e8, 0x87b121, 5);
        m.seed(&[("CETBL", table), ("A0", 0xea12347fff), ("A1", 0x6a56788000)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("A0"), m.reg("A1")), (ce | 0x35, 0xea12347fff, 0x6a56788000));

        let mut m = machine(0x73861d, 0xba7539, 4);
        m.seed(&[("CETBL", table), ("CECFG2", 0x1000), ("EST", 9), ("ECFG", 0x20)]);
        m.seed_data(0x861d, 0x8bad);
        m.seed_data(0x861e, 0x1234);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        let (word, est) = if table != 0 { (0x7452, ce | 0x11) } else { (0x8bad, 9) };
        assert_eq!((m.data(0x861d), m.data(0x861e), m.reg("EST")), (word, 0x1234, est));
    }

    for (left, right, data, expected) in [
        (0xa52501, 0xe761d6, (1, 0), 0x3008), // RHS samples the pointer before the AGU.
        (0xa42501, 0xe361d6, (0, 0x8000), 0x3020), // OR sees the upper DATA word.
    ] {
        let mut m = machine(left, right, 5);
        m.seed(&[("CETBL", 0xffff), ("R5", 0x4048)]);
        m.seed_data(0x4048, data.0);
        m.seed_data(0x4049, data.1);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("R5")), (expected, 0x4049));
    }
}

#[test]
fn compact_accumulator_stack_round_trips_words_and_extends_the_guard() {
    let mut m = Machine::new(0, &[(0x303780, NOP), (NOP, 0x303380), (NOP, NOP)],
                             &[(0, 1, 6), (2, 1, 6), (4, 1, 0)], None);
    m.seed(&[("A0", 0xaa8001abcd), ("SP", 0), ("ECFG", 0x10), ("EST", 0x17)]);
    m.seed_data(1, 0x3ab1);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("SP"), m.data(0), m.data(0xffff), m.data(1)), (0xfffe, 0x8001, 0xabcd, 0x3ab1));
    assert_eq!((m.reg("A0"), m.reg("EST")), (0xaa8001abcd, 0x4017));
    assert_eq!(m.step(), 12);
    assert_eq!((m.reg("SP"), m.reg("A0"), m.reg("EST")), (0, 0xff8001abcd, 0x4417));
}

#[test]
fn paired_pop_updates_distinct_halves_and_both_flag_lanes() {
    let mut m = Machine::new(0, &[(0x809891, 0xca0000), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A0", 0xaa1234abcd), ("A1", 0x9876543210), ("SP", 0x4002), ("EST", 0x4fff)]);
    m.seed_data(0x4003, 0x8001);
    m.seed_data(0x4004, 0);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("SP"), m.reg("A0"), m.reg("A1"), m.reg("EST")),
               (0x4004, 0xaa0000abcd, 0x9876548001, 0x450c));
    assert_eq!((m.data(0x4003), m.data(0x4004)), (0x8001, 0));

    let mut collision = Machine::new(0, &[(0x809098, 0xca0000), (NOP, NOP)],
                                     &[(0, 5, 6), (2, 1, 0)], None);
    let error = collision.start(0).expect_err("a split POP must not write two halves of the same accumulator");
    assert!(matches!(error, KunaError::BadData { .. }), "{error:?}");
}

#[test]
fn wide_guard_bit_ports() {
    let a1 = 0xa4000621a4;
    let a5 = 0xa4000a21a4;
    for (left, right, mode, table, ce2, expected_a1, expected_a5, est) in [
        (0x850345, 0xcd3604, 5, 0xffff, 0, 0x20000621a4, a5, 0x3000),
        (0x8566e2, 0xcd0a14, 5, 0xffff, 0, a1, 0x20000a21a4, 0x3000),
        (0x8566e2, 0xcd0616, 5, 0xffff, 0, a1, 0x00000a21a4, 0x3000),
        (0x8566e2, 0xcd0216, 5, 0xffff, 0, a1, a5, 0x3008),
        (0x4166e2, 0xcd0204, 4, 0x00ff, 0x1000, a1, a5, 0x1000),
        (0x4166e2, 0xcd0204, 4, 0xff00, 0, a1, a5, 0x2008),
        (0x4166e2, 0xcd0604, 4, 0x00ff, 0x1000, a1, a5, 0x1000),
        (0x4166e2, 0xcd0604, 4, 0xff00, 0, 0x02000621a4, a5, 0x2000),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None);
        m.seed(&[("A1", a1), ("A5", a5), ("A2", 0x6612345678), ("CETBL", table), ("CECFG2", ce2)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(
            (m.reg("A1"), m.reg("A5"), m.reg("A2"), m.reg("EST")),
            (expected_a1, expected_a5, 0x6612345678, est),
            "{left:06x}{right:06x}",
        );
    }

    let mut m = Machine::new(
        0, &[(0x8166e2, 0xcd0204), (NOP, NOP), (NOP, NOP)], &[(0, 5, 6), (2, 1, 6), (4, 1, 0)], None,
    );
    m.seed(&[("A1", a1), ("CETBL", 0xaaaa), ("CECFG1", 8), ("CECFG2", 8)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A1"), m.reg("EST")), (a1, 8));
    assert_eq!(m.step(), 12);
    assert_eq!(m.reg("EST"), 0x3008);

    for right in [0xcd0218, 0xcd061c] {
        let mut m = Machine::new(0, &[(0x8566e2, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        assert!(matches!(m.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })), "{right:06x}");
    }
}

#[test]
fn wide_bit_index_uses_the_entire_immediate_and_target_width() {
    for (index, port, operation, a0, rn1, zero) in [
        (39, 0x88, 2, 0x8020002001, 1, false),
        (40, 0x88, 1, 0x0020002001, 1, false),
        (64, 0x88, 2, 0x0020002001, 1, false),
        (15, 0xa9, 2, 0x0020002001, 0x8001, false),
        (16, 0xa9, 2, 0x0020002001, 1, false),
        (255, 0xa9, 2, 0x0020002001, 1, false),
        (128, 0xa9, 0, 0x0020002001, 1, true),
    ] {
        let left = 0x807f38 | (index & 63) << 16;
        let right = 0xcd0000 | operation << 10 | port << 2 | index >> 6;
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x0020002001), ("RN1", 1), ("CETBL", 0xffff)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("RN1"), m.reg("EST")), (a0, rn1, 0x3000 | u64::from(zero) << 3),
                   "{left:06x}{right:06x}");
    }
}

#[test]
fn wide_bit_index_does_not_wrap_the_long_data_bit() {
    for (index, high) in [(16, 1), (31, 0x8000), (32, 0), (64, 0), (255, 0)] {
        let left = 0x807f38 | (index & 63) << 16;
        let right = 0xcd1900 | index >> 6;
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0xffff), ("CETBL", 0xffff)]);
        m.seed_data(0xffff, 0x403);
        m.seed_data(1, 0xabcd);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("R0"), m.data(0xffff), m.data(0), m.data(1)),
                   (0x3000, 0xffff, 0x403, high, 0xabcd), "bit {index}");
    }
}

#[test]
fn wide_bit_index_separates_alu_flags_and_data_writeback_width() {
    for (index, est) in [(32, 0x3000), (39, 0x3000), (40, 0x3008), (255, 0x3008)] {
        let left = 0x807f38 | (index & 63) << 16;
        let right = 0xcd1900 | index >> 6;
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x40), ("CETBL", 0xffff)]);
        m.seed_data(0x42, 0xabcd);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.data(0x40), m.data(0x41), m.data(0x42)),
                   (est, 0, 0, 0xabcd), "bit {index}");
    }
}

#[test]
fn wide_bit_index_tests_sign_extended_inputs() {
    for (right, index, est) in [
        (0xcd12a4, 16, 0x3000), (0xcd12a4, 39, 0x3000), (0xcd12a4, 40, 0x3008),
        (0xcd1100, 32, 0x3000), (0xcd1100, 39, 0x3000), (0xcd1100, 40, 0x3008),
    ] {
        let left = 0x807f38 | index << 16;
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("RN1", 0x8001), ("R0", 0x40), ("CETBL", 0xffff)]);
        m.seed_data(0x40, 1);
        m.seed_data(0x41, 0x8000);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("RN1"), m.data(0x40), m.data(0x41)),
                   (est, 0x8001, 1, 0x8000), "{right:06x}: bit {index}");
    }
}

#[test]
fn wide_bit_index_qualifiers_sample_incoming_feedback() {
    for (left, tested_est) in [(0x287f38, 0x1008), (0x687f38, 0x1000)] {
        let mut m = Machine::new(
            0, &[(left, 0xcd12a4), (NOP, NOP), (NOP, NOP)], &[(0, 4, 6), (2, 1, 6), (4, 1, 0)], None,
        );
        m.seed(&[("RN1", 1), ("CETBL", 0xaa55), ("CECFG1", 8), ("CECFG2", 8)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("RN1"), m.reg("EST")), (1, tested_est));
        assert_eq!(m.step(), 12);
        assert_eq!(m.reg("EST"), if tested_est & 8 == 0 { 0x1000 } else { 0x2008 });
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
fn cliw_mac1_register_compute_ports() {
    let run = |mac1: u32, extra: &[(&str, u64)], expected| {
        let template = [(mac1 << 8) | 0xff, 0xfff03fff, 0xff7ffff8];
        let mut m = Machine::new(0, &[CLIW, (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], Some(template));
        m.seed(&[("A0", 0x12342744), ("A1", 0xff9000abaa), ("A2", 0xa234cdef),
                 ("A3", 0x12340055), ("X1", 5), ("X2", 7), ("FF1", 0xff05), ("FF2", 0x15)]);
        m.seed(extra);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6, "{mac1:06x}");
        assert_eq!((m.reg("A0"), m.reg("EST"), m.reg("X1"), m.reg("X2")),
                   expected, "{mac1:06x}");
    };
    for (word, result, flags) in [
        (0x8797e0, 0xffffff05ab, 0x10), // signed, first input already in X1
        (0x9097e0, 0xffde6d65b6, 0x10), // unsigned first
        (0x8897e0, 0xffbc2865b6, 0x10), // unsigned second
        (0x9897e0, 0x008a1765b6, 0x20), // both unsigned
        (0xc09420, 0x0022b28cfa, 0),
        (0xe09420, 0x0001b5c18e, 1),
        (0xc094e0, 0x0022b2660b, 0),
        (0x0094e0, 0x0022b20000, 0),
        (0xa897e0, 0x00107e0000, 0),
        (0x8097d0, 0x00107e2744, 0),
    ] {
        run(word, &[], (result, flags, 5, 7));
    }
    // The first FF1 input latches; the FF1 second input does not.
    run(0x8617e0, &[("A2L", 0xffff)], (0xfb, 0, 0xff05, 7));
    run(0x87e7e0, &[("X1", 0xfffe)], (0x1f6, 0, 0xfffe, 7));
    let factors = [("A1L", 0x8000), ("A2L", 0x8000), ("ECFG", 5)];
    run(0x8097d0, &factors, (0x007fff2744, 0x24, 5, 7));
    run(0x8097d0, &[("A0", 0x815678abcd), ("A1L", 5), ("A2L", 1)], (0x810000abcd, 8, 5, 7));
    run(0x9897e0, &[("A1L", 0xffff), ("A2L", 0xffff), ("ECFG", 5)], (0x01fffc0002, 0x20, 5, 7));
    run(0xe09420, &[("A0", 15), ("A1L", 5), ("A2L", 3)], (0, 9, 5, 7));
    for mac1 in [0x0094e0, 0xa897e0] {
        run(mac1, &[("A1L", 0x8000), ("A2L", 0xffff), ("A3", 0), ("ECFG", 2)], (0x10000, 0, 5, 7));
        run(mac1, &[("A1L", 0x8000), ("A2L", 1), ("A3", 0), ("ECFG", 2)], (0, 9, 5, 7));
    }
    run(0x9097e0, &[("A1L", 0), ("EST", 0x249), ("CECFG1", 0x222), ("CECFG2", 0x888),
                   ("CETBL", 0x8080)], (0, 0x3248, 5, 7));
    run(0x8797e0, &[("A2L", 0x8000), ("ECFG", 1), ("EST", 0x24)], (0xfffffb0000, 0x14, 5, 7));
}

#[test]
fn cliw_extended_address_destination_ports() {
    const INPUT: &[(&str, u64)] = &[
        ("A0", 0x12342744), ("A1", 0xff9000abaa), ("A2L", 1),
        ("R0", 0x100), ("R1", 0x110), ("R4", 0x140), ("FF1", 0xff05), ("FF2", 0x15),
    ];
    const MEMORY: &[(u64, u64)] = &[
        (0x100, 0x8123), (0x101, 0xabcd), (0x110, 0xfedc), (0x111, 0x9876),
        (0x140, 0x4567), (0x141, 0x1234),
    ];
    let rows: &[((u32, u32), [u32; 3], &[(&str, u64)], &[(u64, u64)])] = &[
        ((0x8c6000, 0xdf2048), [0x7fffff26, 0x3f003fff, 0xff933f88],
            &[("R0", 0x101), ("R1", 0x111), ("A0", 0xffffff8123), ("A1", 0xfffffffedc), ("EST", 0x410)], &[]),
        ((0x8dec00, 0xdc6dda), [0x7fffff24, 0x7f003fff, 0xff7ffff8],
            &[("A0", 0xff9000abaa), ("EST", 0x10)], &[]),
        ((0x846000, 0xdf61ea), [0x7fffff26, 0x7f003fff, 0xff7ffff8],
            &[("A0", 0xff9876fedc), ("EST", 0x10)], &[]),
        ((0x8e2000, 0xdd61c2), [0x7ffffeff, 0xfff03fff, 0xff935f88],
            &[("R4", 0x142), ("A1", 0x12344567)], &[]),
        ((0x8e2000, 0xdd61c2), [0x7ffffeff, 0xfff03fff, 0xff7ffffe],
            &[("R4", 0x142), ("FF1", 0x1234), ("FF2", 0x4567)], &[]),
        ((0x8c6000, 0xdc02c2), [0x8097f8ff, 0xfff03fff, 0xff7ffff8],
            &[("A0", 0xffff2744), ("EST", 0x10)], &[(0x100, 0xffff)]),
        ((0x846000, 0xdf61ec), [0x7ffffeff, 0xfff0404b, 0xfd7ffff8],
            &[("A1", 0xffffffabaa), ("EST", 0x400)], &[(0x110, 0xabaa), (0x111, 0xffff)]),
        ((0x8c6000, 0xdd7204), [0x7fffff24, 0x7fc03fff, 0xff7ffff8],
            &[("A0", 0xff9000abaa), ("EST", 0x10)], &[(0x100, 0xabaa), (0x101, 0x9000)]),
        ((0x8c6000, 0xdd7142), [0x7fffff25, 0xff083fff, 0xff7ffffd],
            &[("R0", 0x102), ("A0", 0xffff050015), ("FF1", 0xabcd), ("FF2", 0x8123), ("EST", 0x10)], &[]),
    ];
    let run = |fetch, template, extra: &[(&str, u64)], output: &[(&str, u64)], writes: &[(u64, u64)]| {
        let mut m = Machine::new(0, &[fetch, (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], Some(template));
        m.seed(INPUT);
        m.seed(extra);
        for &(address, value) in MEMORY {
            m.seed_data(address, value);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        for &(name, value) in INPUT.iter().chain(&[("EST", 0)]).chain(extra) {
            let expected = output.iter().rev().chain(extra.iter().rev())
                .find(|&&(reg, _)| reg == name).map_or(value, |&(_, result)| result);
            assert_eq!(m.reg(name), expected, "{fetch:06x?} {name}");
        }
        for &(reg, expected) in output {
            assert_eq!(m.reg(reg), expected, "{fetch:06x?} {reg}");
        }
        for &(address, before) in MEMORY {
            let expected = writes.iter().find(|&&(a, _)| a == address).map_or(before, |&(_, v)| v);
            assert_eq!(m.data(address), expected, "{fetch:06x?} DATA[{address:x}]");
        }
    };
    for &(fetch, template, output, writes) in rows {
        run(fetch, template, &[], output, writes);
    }
    // The same long product drives different bits to DATA and the accumulator.
    run(rows[6].0, rows[6].1, &[("A1", 0x8000), ("A2L", 0x8000), ("ECFG", 5)],
        &[("A1", 0x7fffffff), ("EST", 0x900)], &[(0x110, 0), (0x111, 0x8000)]);
    run(rows[6].0, rows[6].1, &[("A1", 0x8000), ("A2L", 0x8000), ("ECFG", 9)],
        &[("A1", 0x80000000), ("EST", 0x900)], &[(0x110, 0xffff), (0x111, 0x7fff)]);
    // CE1 inhibits the ALU, not the two-word load or one postincrement.
    run((0x8c6000, 0xdd7143), rows[8].1, &[],
        &[("R0", 0x102), ("FF1", 0xabcd), ("FF2", 0x8123)], &[]);
    run((0x8c6000, 0xdd7205), rows[7].1, &[], &[], &[]);
    // An alternate local selector uses the bank-13 R5 AGU.
    run((0x8ea000, 0xdd61c2), rows[4].1, &[("R5", 0x140)],
        &[("R5", 0x142), ("FF1", 0x1234), ("FF2", 0x4567)], &[]);
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

fn wide_product_machine(packet: u64) -> Machine {
    let mode = if packet >> 47 == 0 { 4 } else { 5 };
    Machine::new(0, &[((packet >> 24) as u32, (packet & 0xffffff) as u32), (NOP, NOP)],
                 &[(0, mode, 6), (2, 1, 0)], None)
}

#[test]
fn wide_saved_loop_addc2_uses_full_loop_address_and_incoming_flags() {
    for (packet, saved, word, ecfg, incoming, config, table, a0, data, est) in [
        (0x9500009b21a7, 0, 0, 0, 0, 0x888, 0x0180, 0xea43380000, (0x1234, 0x5678), 0x2008),
        (0x9582699b21a7, 0xffff, 0, 0, 0x240, 0x333, 0x0180, 0xea43380001, (0x1234, 0x5678), 0x1240),
        (0xaf82699a4fa4, 0xfe2345, 0x8000, 0, 0, 0, 0, 0xea433809a4, (0xfd, 0x5678), 1),
        (0x9582699acfa7, 0xffffff, 0, 0x20, 0, 0, 0, 0xea433809a4, (0xffff, 0x7f), 0),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("LSA1", saved), ("LEA1", saved), ("A5L", word), ("A0", 0xea433809a4),
                 ("R7", 0xc048), ("RN7", 4), ("EST", incoming), ("ECFG", ecfg),
                 ("CECFG1", config), ("CECFG2", config), ("CETBL", table)]);
        m.seed_data(0xc048, 0x22b4);
        m.seed_data(0xc04c, word);
        m.seed_data(0xc0b1, 0x1234);
        m.seed_data(0xc0b2, 0x5678);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST")), (a0, est), "{packet:x}");
        assert_eq!((m.data(0xc0b1), m.data(0xc0b2)), data, "{packet:x}");
        assert_eq!((m.reg("R7"), m.reg("RN7"), m.data(0xc048), m.data(0xc04c)),
                   (0xc048, 4, 0x22b4, word), "{packet:x}");
    }
}

#[test]
fn wide_saved_loop_addc2_samples_rn7_literal_after_move() {
    for rn7 in [4u16, 0x8000, 0xffff] {
        let mut m = Machine::new(
            0, &[(0x800000 | u32::from(rn7), 0xc642bc), (0xaf8269, 0x9a4fa4), (NOP, NOP)],
            &[(0, 5, 6), (2, 5, 6), (4, 1, 0)], None,
        );
        m.seed(&[("LSA1", 0xfe2345), ("R7", 0xc048), ("RN7", 0x6666), ("RN6", 0x4444),
                 ("CECFG1", 0x888), ("CECFG2", 0x888), ("CETBL", 0x0180)]);
        m.seed_data(u64::from(0xc048u16.wrapping_add(rn7)), 0x8000);
        m.seed_data(0xc048, 0x1234);
        m.seed_data(0xc0b1, 0xcccc);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("RN7"), m.reg("RN6"), m.reg("R7"), m.reg("EST")),
                   (u64::from(rn7), 0x4444, 0xc048, 0x2000));
        assert_eq!(m.step(), 12);
        assert_eq!((m.data(0xc0b1), m.reg("EST")), (0xfd, 0x2001));
    }
}

#[test]
fn wide_register_square_selects_one_input_and_preserves_incoming_ce_feedback() {
    for (packet, input, ecfg, incoming, table, result, latch, est) in [
        (0x910000b0888d, 0x9005, 0, 0, 0,      0x30fba019,  0xf234, 0),
        (0x910000b3888d, 0x9005, 0, 0, 0,      0x5105a019,  0xf234, 0),
        (0x910000b0888d, 0,      0, 0, 0x0180, 0,           0xf234, 0x2008),
        (0x910000b0888d, 0x8000, 5, 8, 0x0180, 0x7fffffff,  0xf234, 0x1024),
        (0xbe0000b3889d, 0xffff, 1, 0, 0x8001, 0x1fffc0002, 0xffff, 0x1020),
    ] {
        let mut m = wide_product_machine(packet);
        let (a1, ff1) = if packet == 0xbe0000b3889d { (0xbeef, input) } else { (input, 0xbeef) };
        m.seed(&[("A0", 7), ("A1", a1), ("FF1", ff1), ("X1", 0xf234), ("X2", 0xa67b),
                 ("ECFG", ecfg), ("EST", incoming), ("CETBL", table),
                 ("CECFG1", 0x888), ("CECFG2", 0x888)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("X1"), m.reg("EST")), (result, latch, est), "{packet:x}");
        assert_eq!((m.reg("A1"), m.reg("FF1"), m.reg("X2")), (a1, ff1, 0xa67b), "{packet:x}");
    }
}

#[test]
fn wide_data_square_reads_the_old_pointer_once_and_rounds_mac_ties_upward() {
    for (packet, mode, address, modulo, bound, word, addend, ecfg, next, result, est) in [
        (0x900000b0888e, 0, 0x0100, 0, 0,      101,    0,            0, 0x0101, 0x27d9,       0),
        (0x900000b0888e, 1, 0xffff, 7, 0xfff8, 0xffff, 0,            1, 0,      2,            0),
        (0x900000b0888e, 2, 0x4047, 6, 0,      0x8003, 0,            0, 0x4041, 0x3ffd0009,   0),
        (0x900000b0888e, 3, 0xfffd, 7, 0xfffc, 13,     0,            0, 0xfff6, 169,          0),
        (0xa3108acb8088, 0, 0x4040, 0, 0,      0,      0x8000,       2, 0x4041, 0x10000,      0),
        (0xa3108acb8088, 2, 0x4047, 7, 0,      0xffff, 0xfffffe7fff, 2, 0x4040, 0xffffff0000, 0x10),
        (0xa3108acb8088, 3, 0x4040, 3, 0x4040, 0x8000, 0x11223344,   5, 0x4041, 0x7fff0000,   0x24),
    ] {
        let mut m = wide_product_machine(packet);
        let adjacent = (address + 1) & 0xffff;
        m.seed_data(address, word);
        m.seed_data(adjacent, word ^ 0x9635);
        m.seed(&[("PACFG", mode), ("R0", address), ("RN0", 0x8123), ("RM0", modulo),
                 ("RL0", bound), ("A0", 0x8d6543210a), ("A2", addend), ("ECFG", ecfg),
                 ("X1", 0x7654), ("X2", 0x4321), ("CETBL", 0)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R0"), m.reg("X1"), m.reg("A0"), m.reg("EST")),
                   (next, word, result, est), "{packet:x}, mode {mode}");
        assert_eq!((m.reg("RN0"), m.reg("X2"), m.reg("A2")), (0x8123, 0x4321, addend));
        assert_eq!((m.data(address), m.data(adjacent)), (word, word ^ 0x9635));
    }
}

#[test]
fn wide_insert_keeps_the_agu_gate_and_result_ports_separate() {
    let words = [(0x4036, 0xdda4), (0x4037, 0x69bd), (0x4044, 0x25de), (0x4045, 0xd3e5),
                 (0x8040, 0x7c4b), (0x8041, 0x20fa), (0x8043, 0x8be1)];
    for (packet, table, ff1, ff2, old_a2, r2, a2, high, low, est) in [
        (0x23d72cf8952d, 0, 0x1234, 0xf678, 0x87b0084b74, 0x8043, 0x87b0084b74, 0x20fa, 0x69bd, 0),
        (0x23d72cf8952d, 0xffff, 0x1234, 0xf678, 0x87b0084b74, 0x8043, 0x87b0084b74, 0x234a, 0x69bd, 0x3000),
        (0x23c204f904c3, 0, 0x1234, 0xf678, 0x87b0084b74, 0x8040, 0x87b0084b74, 0x20fa, 0x69bd, 0),
        (0x23c204f904c3, 0xffff, 0x1234, 0xf678, 0x87b0084b74, 0x8040, 0xe0b0084b74, 0x20fa, 0x69bd, 0x3000),
        (0xa3f4f6f8c225, 0, 0x1234, 0xf678, 0x87b0084b74, 0x8041, 0x87b0084b74, 0x20fa, 0xc4bd, 0),
        (0xa3f4f6f8c225, 0xffff, 0x1234, 0xf678, 0x87b0084b74, 0x8041, 0x87b0084b74, 0x20fa, 0xc4bd, 0x3000),
        (0x23c204f904c3, 0xffff, 15, 3, 0x87b0084b74, 0x8040, 0x03b0084b74, 0x20fa, 0x69bd, 0x3000),
        (0x23c204f904c3, 0xffff, 16, 3, 0xb0084b74, 0x8040, 0x06b0084b74, 0x20fa, 0x69bd, 0x3008),
        (0x23c204f904c3, 0xffff, 31, 24, 0x87b0084b74, 0x8040, 0xb0084b74, 0x20fa, 0x69bd, 0x3000),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("R1", 0x4040), ("R2", 0x8040), ("RN2", 3), ("A2", old_a2),
                 ("FF1", ff1), ("FF2", ff2), ("CETBL", table), ("CECFG1", 0x1000), ("CECFG2", 0x1000)]);
        for (address, native) in words {
            m.seed_data(address, native);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R2"), m.reg("A2"), m.reg("EST")), (r2, a2, est), "{packet:x}");
        assert_eq!((m.reg("R1"), m.reg("FF1"), m.reg("FF2")), (0x4040, ff1, ff2));
        for (address, incoming) in words {
            let expected = match address {
                0x8041 => high,
                0x4037 => low,
                _ => incoming,
            };
            assert_eq!(m.data(address), expected, "{packet:x}, DATA[{address:x}]");
        }
    }
}

#[test]
fn wide_insert_checks_three_data_ports_after_postmodify() {
    for (address, source, pacfg, retires) in [
        (0x4036u16, 0x0030u16, 0, false),
        (0x8004, 0xf003, 0, false),
        (0x7fff, 0x7ffd, 0, false),
        (0x7fff, 0x8002, 0, false),
        (0x7fff, 0x7ffc, 0, true),
        (0x7fff, 0x8003, 0, true),
        (0xffff, 0x0002, 0, true),
        (0xffff, 0xfffd, 0, true),
        (0x4036, 0x8041, 0x10, true),
    ] {
        let mut m = wide_product_machine(0xa3f4f6f8c225);
        let next = address.wrapping_add(1);
        let incremented = source.wrapping_add(1);
        m.seed(&[("R1", u64::from(address.wrapping_add(10))), ("R2", u64::from(source)),
                 ("FF1", 0), ("FF2", 16), ("PACFG", pacfg)]);
        m.seed_data(u64::from(address), 0x8d00);
        m.seed_data(u64::from(next), 0xb600);
        m.seed_data(u64::from(source), 0xa500);
        m.start(0).unwrap();
        if retires {
            assert_eq!(m.step(), 6);
        } else {
            assert_eq!(m.emulator.execute_instruction().unwrap_err().explain(), "Userop not hooked");
        }
        assert_eq!(m.reg("R2"), u64::from(incremented));
        assert_eq!(m.data(u64::from(address)), if retires { 0xa500 } else { 0x8d00 });
        assert_eq!(m.data(u64::from(next)), 0xb600);
        assert_eq!(m.data(u64::from(source)), 0xa500);
    }
}

#[test]
fn wide_abs_uses_the_connected_result_bank() {
    for (packet, input, config, expected, flags) in [
        (0xe0269bc22a23, 0xea433809a4, 0, 0x15bcc7f65c, 0x20),
        (0xe0269bc22a23, 0, 0, 0, 0x08),
        (0xe0269bc22a23, 0x8000000000, 0, 0x8000000000, 0x36),
        (0xe0269bc22a23, 0x8000000000, 8, 0x8000000000, 0x32),
        (0xe0269bc22a23, 0xff80000000, 4, 0x7fffffff, 0x24),
        (0xe6269bc22a23, 0xea433809a4, 4, 1, 0),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("A0", input), ("ECFG", config), ("CETBL", 0xffff), ("FF2", 0xf678)]);
        m.seed_data(0xffff, 0x9876);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST")), (expected, 0x3000 | flags));
        assert_eq!((m.reg("FF2"), m.data(0xffff)), (0xf678, 0x9876));
    }

    for (word, config, expected, flags) in [
        (0xf678, 0, 0x0988, 0),
        (0, 0, 0, 8),
        (0x8000, 0, 0x8000, 0x24),
        (0x8000, 4, 0x7fff, 0x24),
    ] {
        let mut m = wide_product_machine(0xe7269bc22a5f);
        m.seed(&[("FF2", word), ("A0", 0x123456789a), ("ECFG", config), ("CETBL", 0xffff)]);
        m.seed_data(0xffff, 0x9876);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("FF2"), m.reg("EST")), (expected, 0x3000 | flags));
        assert_eq!((m.reg("A0"), m.data(0xffff)), (0x123456789a, 0x9876));
    }

    for packet in [0xfd269bc229d7, 0xff269bc229d7, 0xfd269bc228d7, 0xe7269bc22a23] {
        let mut m = wide_product_machine(packet);
        assert!(matches!(m.start(0), Err(KunaError::Unimpl { .. })));
    }
}

#[test]
fn wide_mul_disconnected_and_control_outputs_keep_their_own_flag_lanes() {
    for (packet, flags, ff1, lc1) in [
        (0x910000b385a1, 0x3000, 0x1234, 0x5555),
        (0x910000b38ea1, 0x3020, 0x1234, 0x5555),
        (0x910000b38fa1, 0x3000, 0x1234, 0x5555),
        (0x910000b3bea1, 0x3024, 0xfffe, 0x5555),
        (0x910000b3d4a1, 0x3024, 0x1234, 0xfffe),
        (0x910000b3d8a1, 0x3024, 0x1234, 0x5555),
    ] {
        let mut m = wide_product_machine(packet);
        let guard = 0xadaabbccdd;
        m.seed(&[("A0", 0xffff), ("A1", 0xffff), ("A5", guard), ("X1", 0xba98),
                 ("FF1", 0x1234), ("LC1", 0x5555), ("CETBL", 0xffff)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("FF1"), m.reg("LC1")), (flags, ff1, lc1), "{packet:012x}");
        assert_eq!((m.reg("A0"), m.reg("A5"), m.reg("LC"), m.reg("X1")), (0xffff, guard, 0, 0xba98));
    }
    for (packet, x1) in [(0xa20000b38601, 0x8000), (0xe606ffb3d34f, 0x1aec), (0x79b71db28f5b, 0x5dda)] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("R0", 0x40), ("R5", 0x4048), ("RN5", 2), ("R7", 0xc048),
                 ("LEA1", 0x78bbbb), ("X1", 0xba98), ("CETBL", 0xffff)]);
        m.seed_data(0x40, 0x8000);
        m.seed_data(0xc047, 0x1aec);
        m.seed_data(0x404a, 0x5dda);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("X1"), m.reg("LEA1"), m.reg("EST")), (x1, 0x78bbbb, 0x3000));
        assert_eq!((m.reg("R0"), m.reg("R5"), m.reg("R7")), (0x40, 0x4048, 0xc048));
    }
    for (dst, config, word, flags) in [
        (0x8e, 0, 0x3344, 0x3000), (0xbe, 0, 0, 0x3008), (0xbe, 4, 0xfffe, 0x3020),
    ] {
        let mut m = wide_product_machine(0xd1a701b300a1 | (dst << 8));
        let input = if config == 4 { 0xffff } else { 1 };
        m.seed(&[("A0L", input), ("A1L", input), ("FF1", 0x3344), ("CETBL", 0xffff), ("ECFG", config)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("FF1"), m.reg("EST")), (word, flags));
    }
}

#[test]
fn wide_rounded_product_uses_same_result_and_literal_ports_as_multiply() {
    for (packet, a0, x1, ff1, lc1, flags) in [
        (0xf3a701b585e7, 0x4123, 0, 0x3210, 0x5151, 0x3000),
        (0xd1a701b58ea1, 0x4123, 0xba98, 0x3210, 0x5151, 0x3010),
        (0xd1a701b5bea1, 0x4123, 0xba98, 0xf706, 0x5151, 0x3010),
        (0xd1a701b5d4a1, 0x4123, 0xba98, 0x3210, 0xf706, 0x3010),
        (0xd1a701b5d3a1, 0x4123, 0xba98, 0x3210, 0x5151, 0x3000),
        (0xd1a701b5d8a1, 0x4123, 0xba98, 0x3210, 0x5151, 0x3010),
        (0xd0a701b5889f, 0x160000, 0x55, 0x3210, 0x5151, 0x3000),
        (0xcfa701b588a1, 0xffe95b0000, 0xba98, 0x3210, 0x5151, 0x3010),
        (0xd0a701b5489f, 0x4123, 0x55, 0x3210, 0x5151, 0x3000),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("A0", 0x4123), ("A1", 0xdcba), ("A5", 0xadaabbccdd), ("X1", 0xba98),
                 ("R0", 0x40), ("LC1", 0x5151), ("LEA1", 0x2566), ("FF1", 0x3210), ("CETBL", 0xffff)]);
        m.seed_data(0xffff, 0x55);
        m.seed_data(0x40, 0x8000);
        m.seed_data(0x41, 0x4321);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("X1"), m.reg("FF1"), m.reg("LC1"), m.reg("EST")),
                   (a0, x1, ff1, lc1, flags), "{packet:012x}");
        assert_eq!((m.reg("LEA1"), m.reg("A5"), m.data(0xffff)), (0x2566, 0xadaabbccdd, 0x55));
        let expected_store = if packet == 0xd0a701b5489f { (0x41, 0, 0x16) } else { (0x40, 0x8000, 0x4321) };
        assert_eq!((m.reg("R0"), m.data(0x40), m.data(0x41)), expected_store);
    }
    for (packet, cfg1, cfg2, table, pointer, latch, word, flags) in [
        (0x10a701b5489f, 0, 0, 0, 0x40, 0x7777, 0x4321, 0),
        (0x10a701b5489f, 0x1000, 0, 0, 0x41, 0x55, 0x4321, 0),
        (0x50a701b5489f, 0, 0x1000, 0, 0x41, 0x55, 0x4321, 0),
        (0x10a701b5489f, 0, 0, 0xffff, 0x41, 0x55, 0x15, 0x3000),
        (0xd0a701b5489f, 0, 0, 0, 0x41, 0x55, 0x15, 0),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("A0L", 0x4000), ("R0", 0x40), ("X1", 0x7777), ("CETBL", table),
                 ("CECFG1", cfg1), ("CECFG2", cfg2)]);
        m.seed_data(0xffff, 0x55);
        m.seed_data(0x41, 0x4321);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R0"), m.reg("X1"), m.data(0x41), m.reg("EST")), (pointer, latch, word, flags));
    }
    for (packet, config, left, right, flags, high) in [
        (0xd1a701b5bea1, 0x22, 0x4000, 2, 0x3000, 1),
        (0xd1a701b5bea1, 0x25, 0x8000, 0x8000, 0x3024, 0x7fff),
        (0xd1a701b548a1, 0x22, 0x4000, 2, 0x3008, 0),
        (0xd1a701b548a1, 0x25, 0x8000, 0x8000, 0x3024, 0x3fff),
    ] {
        let mut m = wide_product_machine(packet);
        m.seed(&[("A0L", left), ("A1L", right), ("R0", 0x40), ("CETBL", 0xffff), ("ECFG", config)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        let result = if (packet >> 8) & 0xff == 0x48 { m.data(0x41) } else { m.reg("FF1") };
        assert_eq!((m.reg("EST"), result), (flags, high));
    }
    let mut m = wide_product_machine(0xd0a701b59e9f);
    m.seed(&[("A0L", 0x4000), ("X1", 0x7777), ("CETBL", 0xffff)]);
    m.seed_data(0xffff, 0x55);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("X1"), m.reg("EST")), (0x15, 0x3000));
}

#[test]
fn wide_multiply_result_ports_drive_the_following_eq_branch() {
    for (port, input, low, high, zero, a0, ff1) in [
        (0x3a, 7, 0, 0xbbbb, true, 0x12340007, 0x55aa),
        (0x3a, 0x4000, 1, 0xbbbb, false, 0x12340007, 0x55aa),
        (0x3a, 0xffff, 0xffff, 0xbbbb, false, 0x12340007, 0x55aa),
        (0x7a, 7, 35, 0, false, 0x12340007, 0x55aa),
        (0x88, 7, 0xaaaa, 0xbbbb, false, 35, 0x55aa),
        (0x96, 7, 0xaaaa, 0xbbbb, true, 0x12340007, 0),
    ] {
        let mut m = Machine::new(0, &[
            (0x9c6651, 0xb000da | (port << 8)), (0x281c10, NOP),
            (INC_A2, NOP), (0x21ace6, NOP), (NOP, NOP),
        ], &[(0, 5, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 0)], None);
        m.seed(&[("CETBL", 0xffff), ("A0", 0x12340007), ("FF1", 0x55aa), ("R4", 0x4040),
                 ("RN5", 5), ("X1", 0x1111), ("X2", 0x2222)]);
        m.seed_data(0x4040, input);
        m.seed_data(0x6651, 0xaaaa);
        m.seed_data(0x6652, 0xbbbb);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R4"), m.reg("X1"), m.reg("X2")), (0x403f, 5, 0x2222));
        assert_eq!((m.data(0x6651), m.data(0x6652), m.reg("A0"), m.reg("FF1")), (low, high, a0, ff1));
        assert_eq!(m.step(), if zero { 18 } else { 12 });
        if !zero { assert_eq!(m.step(), 18); }
        assert_eq!(m.step(), 24);
        assert_eq!((m.reg("A2"), m.reg("A3")), (u64::from(!zero), 1));
    }
    for (port, input, low, high, flags) in [
        (0x3a, 0x4000, 0, 0xbbbb, 0x3000),
        (0x3a, 0xffff, 0xffff, 0xbbbb, 0x3010),
        (0x7a, 7, 17, 0, 0x3000),
        (0x7a, 0xffff, 0xfffd, 0xffff, 0x3010),
    ] {
        let mut m = wide_product_machine(0x9c6651b000da | (port << 8));
        m.seed(&[("ECFG", 0x20), ("CETBL", 0xffff), ("R4", 0x4040), ("RN5", 5)]);
        m.seed_data(0x4040, input);
        m.seed_data(0x6651, 0xaaaa);
        m.seed_data(0x6652, 0xbbbb);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(0x6651), m.data(0x6652), m.reg("EST")), (low, high, flags));
    }
    for (a1, flags) in [(0xb505, 0x3024), (0, 0x3008)] {
        let mut m = wide_product_machine(0x926651b33aa3);
        m.seed(&[("A1", a1), ("A2", 0xb505), ("X1", 0x1111), ("ECFG", 1), ("CETBL", 0xffff)]);
        m.seed_data(0x6651, 0xaaaa);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(0x6651), m.reg("EST"), m.reg("X1")), (0, flags, 0x1111));
    }
}

#[test]
fn wide_multiply_latch_is_visible_to_the_following_signed_move() {
    for (left, right, table, cfg, word, a0, a1, x1) in [
        (0x32bf6c, 0xb0999f, 0xffff, 0x1000, 0, 0x10007, 3, 0),
        (0x32bf6c, 0xb0999f, 0xffff, 0x1000, 0x4000, 0x10007, 0x10003, 0x4000),
        (0x32bf6c, 0xb0999f, 0, 0x1000, 0x4000, 0x10007, 0x23450003, 0x4000),
        (0x32bf6c, 0xb0999f, 0, 0, 0x4000, 0x10007, 0x23450003, 17),
        (0xb2bf6c, 0xb0e09f, 0xffff, 0x1000, 0x4000, 0x10007, 0x23450003, 0x4000),
        (0xb2bf6c, 0xb088ad, 0xffff, 0x1000, 0x4000, 55, 0x23450003, 11),
        (0xb2bf6c, 0xb088a1, 0xffff, 0x1000, 0x4000, 35, 0x23450003, 17),
        (0x8fbf6c, 0xb088a5, 0xffff, 0x1000, 0x4000, 0xfffffe3bf4, 0x23450003, 17),
        (0x8fbf6c, 0xb0888d, 0xffff, 0x1000, 0x4000, 0x104a5590, 0x23450003, 17),
    ] {
        let mode = if left & SEQUENTIAL != 0 { 5 } else { 4 };
        let mut m = Machine::new(0, &[
            (left, right), (NOP, NOP), (NOP, NOP), (NOP, NOP), (NOP, NOP),
            (0x9e0000, 0xc3622e), (NOP, NOP),
        ], &[(0, mode, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 5, 6), (12, 1, 0)], None);
        m.seed(&[("CECFG1", cfg), ("CETBL", table), ("A0", 0x10007), ("A1", 0x23450003),
                 ("A2", 7), ("A3", 9), ("FF1", 11), ("RM0", 5), ("X1", 17), ("X2", 19)]);
        m.seed_data(0xffff, word);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("A1"), m.reg("A3"), m.reg("X1")), (a0, a1, 9, x1));
        for pc in [12, 18, 24, 30, 36] {
            assert_eq!(m.step(), pc);
        }
        assert_eq!((m.reg("A3"), m.reg("X1"), m.reg("X2")), (x1, x1, 19));
    }
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
fn wide_short_callabs_zero_extends_and_gates_stack_publish() {
    let base = 0xffe0;
    let start = 0x10000;
    let target = 0xfff0;
    let count = 5;
    let link = start + 16;
    let mut issues = vec![(16, 1, 6)];
    issues.extend((17..23).map(|i| (i * 2, 1, 6)));
    for (left, right, mode, cetbl, ce1, ce2, taken, stored) in [
        (0xe2e956, 0xcf7c2b, 5, 0, 0, 0, true, true),
        (0x06e956, 0xcf7c2a, 4, 0, 0, 0, false, false),
        (0x06e956, 0xcf7c2a, 4, 0, 0x1000, 0, true, false),
        (0x62e956, 0xcf7c2b, 4, 0xff00, 0, 0, true, true),
    ] {
        let mut fetches = [(NOP, NOP); 24];
        fetches[16] = (left, right);
        fetches[17..23].fill((INC_A0, NOP));
        let mut context = issues.clone();
        context.push((32, mode, 6));
        let mut m = Machine::new(base, &fetches, &context, None);
        m.seed(&[("A0L", 0x8110), ("SP", 0x4040), ("CETBL", cetbl), ("CECFG1", ce1), ("CECFG2", ce2)]);
        for (word, value) in [(0xffff, target), (0, 0x7654), (0xe956, 0x1018), (0x4040, 0x1234), (0x403f, 0x5678)] {
            m.seed_data(word, value);
        }
        m.start(start).unwrap();
        assert_eq!(m.step(), 3 * if taken { target } else { start + 2 * (count + 1) });
        assert_eq!((m.reg("A0L"), m.reg("SP")), (0x8110 + count, if taken { 0x403e } else { 0x4040 }));
        assert_eq!((m.data(0x4040), m.data(0x403f)), if stored { (1, link & 0xffff) } else { (0x1234, 0x5678) });
        assert_eq!((m.data(0xffff), m.data(0)), (target, 0x7654));
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

#[test]
fn compact_aux_add_distinguishes_pair_width_address_values_and_word_flags() {
    for (raw, expected, flags) in [
        (0x12b380, 0x81236756, 0x3020),
        (0x12b3c0, 0x81236756, 0x3020),
        (0x12b390, 0x81233455, 0x3021),
    ] {
        let mut m = Machine::new(0, &[(raw, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x3300), ("RN0", 0xffff), ("FF1", 0x3456), ("FF2", 0x8123), ("CETBL", 0xffff)]);
        m.seed_data(0x3300, 0x7abc);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A1"), m.reg("EST")), (expected, flags));
    }
    for raw in [0x12bc00, 0x12be00] {
        let mut m = Machine::new(0, &[(raw, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x3300), ("A0", 0x23456789), ("CETBL", 0xffff)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("FF1"), m.reg("FF2"), m.reg("A0"), m.reg("EST")),
                   (0x9a89, 0x2345, 0x23456789, 0x3000));
    }
    for (left, right, reg, a0, a1, ecfg, table, expected, flags) in [
        (0x132b02, NOP, "FF1", 0, 1, 0, 0xffff, 0, 0x3008),
        (0x132b82, NOP, "FF2", 0x80000000, 0x7fffffff, 4, 0xffff, 0x7fff, 0x3024),
        (0x132f02, NOP, "X1", 0x80000000, 0x7fffffff, 8, 0xffff, 0xffff, 0x3020),
        (NOP, 0x132f02, "X2", 0x80000000, 0x7fffffff, 8, 0xffff, 0xffff, 0x3800),
        (0x132f82, NOP, "X2", 0x23456789, 0x03654321, 0x20, 0xffff, 0x26aa, 0x3000),
        (0x132f83, NOP, "X2", 0x23456789, 0x03654321, 0, 0, 0x5678, 0),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A0", a0), ("A1", a1), ("ECFG", ecfg), ("CETBL", table),
                 ("CECFG1", 0x1000), ("X2", 0x5678)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(reg), m.reg("EST"), m.reg("A0"), m.reg("A1")), (expected, flags, a0, a1));
    }
}

#[test]
fn compact_arithmetic_packet_routes() {
    for table in [0, 0xffff] {
        let on = table != 0;
        let mut m = Machine::new(0, &[(0x3b5d27, 0x1a031c), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("CETBL", table), ("A0", 0xea433809a4), ("A3L", 38), ("RN1", 2),
                 ("R7", 0xc048), ("X2", 0x5678)]);
        m.seed_data(0xc048, 0xa25e);
        m.seed_data(0xc049, 0x8a5a);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("A0"), 0xea433899b0);
        assert_eq!(m.reg("EST"), if on { 0x3930 } else { 0x0900 });
        assert_eq!((m.data(0xc048), m.data(0xc049)), if on { (0, 0) } else { (0xa25e, 0x8a5a) });

        let mut m = Machine::new(0, &[(0x08b756, 0x4a2001), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("CETBL", table), ("R0", 0x40), ("R3", 0xc040), ("RN3", 4),
                 ("A4", 0x80f4c5af94), ("A5", 0x89d1dd24b5), ("X2", 0x5678)]);
        m.seed(&[("CECFG2", 0x1000)]);
        m.seed_data(0x40, 0xa258);
        m.seed_data(0xc040, 0x0786);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R3"), m.reg("X2"), m.reg("A5")), (0xc044, 0xa258, 0x89d1dd24b5));
        assert_eq!((m.reg("A4"), m.reg("EST")), if on { (0x80f4c50028, 0x3011) } else { (0x80f4c5af94, 0x11) });

        let mut m = Machine::new(0, &[(0x183c2f, 0x0336a7), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("CETBL", table), ("R5", 0x4048), ("RN5", 2), ("R7", 0xc048),
                 ("A0L", 0x09a4), ("A2", 0x87b0084b74), ("A3", 0x8d0def9f26)]);
        m.seed(&[("CECFG1", 0x1000), ("CECFG2", 0x1000)]);
        for (addr, value) in [(0xc048, 0xa25e), (0xc049, 0x8a5a), (0x4048, 0x71bb), (0x4049, 0x03a3)] {
            m.seed_data(addr, value);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R5"), m.reg("R7")), (0x404a, 0xc049));
        assert_eq!(m.reg("EST"), if on { 0x3d00 } else { 0 });
        assert_eq!(m.data(0xc048), if on { 0x6746 } else { 0xa25e });
        assert_eq!((m.data(0x4048), m.data(0x4049)), if on { (0xac4e, 0xa218) } else { (0x71bb, 0x03a3) });

        let mut m = Machine::new(0, &[(0x1b332e, 0x41964f), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("CETBL", table), ("R1", 0x4040), ("R6", 0x8048), ("R7", 0xc048),
                 ("A2", 0x87b0084b74), ("FF1", 0x1234), ("FF2", 0xf678)]);
        m.seed(&[("CECFG2", 0x1000)]);
        m.seed_data(0x4040, 0xc460);
        m.seed_data(0xc048, 0xa25e);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R1"), m.reg("X2")), (0x403f, 0xc460));
        assert_eq!((m.reg("A2"), m.reg("EST")), if on { (0x15cedb40, 0x3034) } else { (0x87b0084b74, 0x34) });
        assert_eq!((m.data(0x8048), m.data(0x8049)), (0x5da8, 0xa680));
    }
}

#[test]
fn compact_arithmetic_indexed_rmw_uses_the_encoded_register_and_pointer() {
    let mut m = Machine::new(0, &[(0x936c9a, NOP)], &[(0, 2, 3), (1, 3, 0)], None);
    m.seed(&[("RN1", 2), ("R5", 0x4048), ("RN5", 2)]);
    m.seed_data(0x404a, 0x5dda);
    m.start(0).unwrap();
    assert_eq!(m.step(), 3);
    assert_eq!((m.reg("RN1"), m.reg("R5"), m.reg("EST")), (0x5ddc, 0x4048, 0));
    assert_eq!(m.data(0x404a), 0x5dda);
}

#[test]
fn compact_arithmetic_alternate_shift_and_addabs_selectors() {
    // R6 and A1L are selected instead of R7 and A0L; selector 15 still uses X2.
    let mut m = Machine::new(0, &[(0x3b5927, 0x1a071e), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("CETBL", 0xffff), ("A0H", 0x4338), ("A3L", 2),
             ("RN1", 2), ("R6", 0x8048), ("R7", 0xc048), ("X2", 0x5678)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A1L"), m.reg("EST")), (0x99b0, 0x3900));
    assert_eq!((m.data(0x8048), m.data(0x8049), m.data(0xc048)), (8, 0, 0));
}

#[test]
fn compact_arithmetic_operand_fetch_requires_address_qualification() {
    let mut m = Machine::new(0, &[(0x1b332e, 0x41964f), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("R1", 0x4040), ("R6", 0x8048), ("R7", 0xc048), ("X2", 0x5678),
             ("A2", 0x87b0084b74), ("FF1", 0x1234), ("FF2", 0xf678)]);
    m.seed_data(0x4040, 0xc460);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("R1"), m.reg("X2"), m.reg("A2")), (0x4040, 0x5678, 0x87b0084b74));
    assert_eq!((m.data(0x8048), m.data(0x8049)), (0x5da8, 0xa680));
}

#[test]
fn compact_arithmetic_fft_shift_uses_signed_data_bits_not_the_guard() {
    let mut m = Machine::new(0, &[(0x3b5d27, 0x1a031c), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    for (mode, threshold, shift, table, high, fft) in [
        (0,    31, 31, 0xff, 0x8000, 0),
        (0x10, 31, 31, 0xff, 0x8000, 0x4000),
        (0x10, 1,  27, 0xff, 0x0800, 0x4000),
        (0x10, 7,  27, 0xff, 0x0800, 0),
        (0x10, 0,  39, 0xff, 0,      0),
        (0x10, 0,  31, 0,    0x5555, 0),
    ] {
        m.seed(&[("EST", 0), ("ECFG", mode), ("FFTV", threshold), ("CETBL", table),
                 ("RN1", 1), ("A3L", shift), ("R7", 0xc048), ("A0H", 0x6000), ("X2", 0)]);
        m.seed_data(0xc049, 0x5555);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg("EST") & 0x4000, fft);
        assert_eq!((m.reg("A0L"), m.data(0xc049)), (0x6000, high));
    }
}

#[test]
fn compact_arithmetic_fft_dual_stores_test_after_alim_but_before_saturation() {
    let mut m = Machine::new(0, &[(0x183c2f, 0x0336a7), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    for (mode, table, threshold, short, right, est) in [
        (0x10, 0xff,   31, 0x8000, 0x5555, 0x5024),
        (0x14, 0xff,   31, 0x7fff, 0x5555, 0x1024),
        (0x18, 0xff,   31, 0x7fff, 0x5555, 0x5024),
        (0x1c, 0xff,   31, 0x7fff, 0x5555, 0x1024),
        (0x30, 0xff,   31, 0x4000, 0x5555, 0x1024),
        (0x10, 0xff00, 27, 1,      0xa218, 0x2d00),
        (0x14, 0xff00, 27, 1,      0x8000, 0x6d00),
        (0x18, 0xff00, 27, 1,      0x8000, 0x2d00),
    ] {
        m.seed(&[("EST", 0), ("ECFG", mode), ("FFTV", threshold), ("CETBL", table),
                 ("R5", 0x4048), ("RN5", 2), ("R7", 0xc048),
                 ("A0L", 0x7fff), ("A2", 0x87b0084b74), ("A3", 0x8d0def9f26)]);
        m.seed_data(0xc048, 1);
        m.seed_data(0x4049, 0x5555);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.data(0xc048), m.data(0x4049)), (est, short, right));
    }
}

#[test]
fn compact_arithmetic_fft_samples_the_long_store_not_the_multiply() {
    let mut m = Machine::new(0, &[(0x1b332e, 0x41964f), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    for (mode, high, fft) in [(0x10, 0xa680, 0x4000), (0x30, 0x5340, 0)] {
        m.seed(&[("EST", 0), ("ECFG", mode), ("FFTV", 21), ("CETBL", 0),
                 ("R1", 0x4040), ("R6", 0x8048), ("R7", 0xc048),
                 ("A2", 0x87b0084b74), ("FF1", 0x1234), ("FF2", 0xf678)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST") & 0x4000, m.data(0x8049)), (fft, high));
    }
    m.seed(&[("EST", 0), ("ECFG", 0x11), ("FFTV", 0), ("CETBL", 0xff00),
             ("R1", 0x4040), ("A2", 0), ("FF1", 0), ("FF2", 0)]);
    m.seed_data(0x4040, 0x8000);
    m.seed_data(0xc048, 0x8000);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("EST") & 0x4000, m.reg("A2"), m.data(0x8049)), (0, 0x80000000, 0));
}

#[test]
fn compact_arithmetic_fft_survives_a_later_below_threshold_issue() {
    let fetches = [
        (0x183c2f, 0x0336a7), (NOP, NOP), (NOP, NOP), (NOP, NOP),
        (NOP, NOP), (0x3b5d27, 0x1a031c), (NOP, NOP),
    ];
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 1, 6), (12, 1, 0)];
    let mut m = Machine::new(0, &fetches, &issues, None);
    m.seed(&[("ECFG", 0x10), ("CETBL", 0xffff), ("R5", 0x4048), ("RN5", 2),
             ("R7", 0xc048), ("RN1", 2), ("A0", 0xea433809a4), ("A2", 0x87b0084b74),
             ("A3", 0x8d0def9f26), ("X2", 0x5678)]);
    m.seed_data(0xc048, 0xa25e);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.reg("EST"), 0x7d00);
    for next in [12, 18, 24, 30, 36] {
        assert_eq!(m.step(), next);
    }
    assert_eq!(m.reg("EST"), 0x7930);
}

#[test]
fn wide_extrema_trace_is_independent_of_the_word_result_lane() {
    let seed = [("CETBL", 0xffff), ("R1", 0x1234), ("A0", 0xa58f61c253),
                ("A1", 0x2243217654), ("USER_IO", 0x9234), ("BTR1", 3), ("BTR2", 9)];
    for (left, right, reg, value, btr1, btr2, est) in [
        (0x88e52d, 0xc2be86, "R1",  0x8f61, 3,      0x13, 0x3000),
        (0x88e52d, 0xc28686, "R1",  0,      6,      9,    0x3000),
        (0x88e52d, 0xc2be66, "A1H", 0x8f61, 3,      0x13, 0x3001),
        (0x97e52d, 0xc28243, "A0L", 0xffff, 7,      9,    0x3010),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&seed);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(reg), m.reg("BTR1"), m.reg("BTR2"), m.reg("EST")),
                   (value, btr1, btr2, est), "{left:06x}{right:06x}");
        assert_eq!(m.reg("GB1"), 0x22);
    }
}

#[test]
fn wide_extrema_trace_load_and_minmax_address_use_different_ports() {
    for (left, right, value, reference, btr2) in [
        (0x82e52d, 0xc28e80, 0x0039, 0x0040, 9),
        (0x81e52d, 0xc2be81, 0x1000, 0x1234, 0x13),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("R0", 0x6000), ("R1", 0x20), ("R2", 0x40), ("BTR2", 9),
                 ("MINMAX", 0x1234), ("CETBL", 0xffff)]);
        m.seed_data(0x20, 0x27);
        m.seed_data(0x21, 0x1000);
        m.seed_data(0x40, 0x39);
        m.seed_data(0x41, 0x2000);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R0"), m.reg("MINMAX"), m.reg("BTR2"), m.reg("EST")),
                   (value, reference, btr2, 0x3000));
    }
}

#[test]
fn wide_extrema_stored_inputs_keep_guard_and_literal_widths() {
    for (left, right, words, est) in [
        (0xc18743, 0xc84400, (0x17, 0), 0x10),
        (0xef8743, 0xc84402, (0x8743, 0xffff), 0),
        (0xce8743, 0xc84408, (0xffff, 0xffff), 0),
        (0xce8743, 0xc85408, (0x1234, 0), 0),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x1712341234), ("A1", 0x2209871234), ("GB5", 0xf5), ("R0", 0x100)]);
        m.seed_data(0xffff, 0x718d);
        m.seed_data(0x100, 0x1111);
        m.seed_data(0x101, 0x2222);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(0x100), m.data(0x101)), words);
        assert_eq!(m.data(0xffff), 0x718d);
        assert_eq!(m.reg("EST"), est);
    }
}

#[test]
fn wide_extrema_stored_postdisplacement_has_independent_ce_address_gate() {
    let mut m = Machine::new(0, &[(0x258011, 0xc84742), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
    for (ce, table, pointer, words, est) in [
        (0,      0,      0x48, (0x1111, 0x2222), 0),
        (0x1000, 0,      0x59, (0x1111, 0x2222), 0),
        (0,      0xffff, 0x59, (0x89, 0),        0x3008),
    ] {
        m.seed(&[("R4", 0x48), ("GB5", 0x89), ("CECFG1", ce), ("CETBL", table), ("EST", 0)]);
        m.seed_data(0x48, 0x1111);
        m.seed_data(0x49, 0x2222);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(0x48), m.data(0x49)), words);
        assert_eq!((m.reg("R4"), m.reg("EST")), (pointer, est));
    }
}

#[test]
fn wide_extrema_minmax_reg_and_guard_results_use_reference_width() {
    for (left, right, a1, acc, reference, est) in [
        (0xae5ed6, 0xc82908, 0xfffe, 0x331234ffff, 0xffff, 0x3010),
        (0xae5ed6, 0xc83908, 0x2000, 0x3312340000, 0xbaba, 0x3001),
        (0xb25ed6, 0xc82808, 0xaaaa, 0xff12341000, 0xdada, 0x3000),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x3312341000), ("A1L", a1), ("A2L", 0xdada), ("MINMAX", 0xbaba), ("CETBL", 0xffff)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("MINMAX"), m.reg("EST")), (acc, reference, est));
    }
}

#[test]
fn compact_minmu_records_the_incoming_address_without_reading_data() {
    for (op, table, ce, mode, pointer, mask, initial, next, half, record, est) in [
        (0x24d96a, 0xffff, 0, 0, 0x403f, 0x003f, 0xaaaa, 0x403f, 0x1111, 0x403f, 0x3000),
        (0x24d97a, 0xffff, 0, 1, 0x403f, 0x003f, 0xaaaa, 0x4038, 0x1111, 0x403f, 0x3000),
        (0x24d97a, 0xffff, 0, 2, 0x4020, 0x003f, 0xaaaa, 0x3fe4, 0x1111, 0x4020, 0x3000),
        (0x24d97a, 0xffff, 0, 3, 0x503f, 0x103f, 0xaaaa, 0x4003, 0x1111, 0x503f, 0x3000),
        (0x24d97a, 0xffff, 0, 0, 0x403f, 0x003f, 0x1111, 0x4043, 0x1111, 0x1234, 0x3009),
        (0x24d97b, 0,      0, 2, 0x403f, 0x003f, 0xaaaa, 0x403f, 0xaaaa, 0x1234, 0),
        (0x24d97b, 0, 0x1000, 2, 0x403f, 0x003f, 0xaaaa, 0x4003, 0xaaaa, 0x1234, 0),
    ] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A4", 0x807777af94), ("A4H", initial), ("A3H", 0x1111), ("R5", pointer),
                 ("RN5", 4), ("PACFG", mode << 10), ("RL4", 0x4000), ("RM4", mask),
                 ("MINMAX", 0x1234), ("CECFG1", ce), ("CETBL", table)]);
        m.seed_data(pointer, 0xfedc);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R5"), m.reg("A4"), m.reg("MINMAX"), m.reg("EST"), m.data(pointer)),
                   (next, 0x800000af94 | (half << 16), record, est, 0xfedc));
    }
    let mut m = Machine::new(0, &[(0x24d53c | SEQUENTIAL, NOP)], &[(0, 2, 3), (1, 3, 0)], None);
    m.seed(&[("A2H", 0x8001), ("A1H", 0x1111), ("R6", 0x52), ("RN6", 0xfffc)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 3);
    assert_eq!((m.reg("R6"), m.reg("A2H"), m.reg("MINMAX")), (0x4e, 0x1111, 0x52));
}

#[test]
fn wide_minmax_pacfg_reference_separates_data_literal_and_all_ones_ports() {
    for (left, right, table, input, word, a0l, record, est) in [
        (0x9abe5e, 0xcaf743, 0xffff, 0x4321, 0x4321, 0x09a4, 0x9876, 0x3000),
        (0x9abe5e, 0xcad743, 0xffff, 0x4321, 0x4321, 0x09a4, 0x1357, 0x3000),
        (0x9abe5e, 0xcad73b, 0xffff, 0x4321, 0x4321, 0x09a4, 0x9876, 0x3009),
        (0x9a9234, 0xcaf7c3, 0xffff, 0x4321, 0x4321, 0x09a4, 0x9876, 0x3000),
        (0x9a9234, 0xcaf7bf, 0xffff, 0x4321, 0x9234, 0x09a4, 0x1357, 0x3001),
        (0x9abe5e, 0xcaf7bf, 0xffff, 0xfedc, 0xfedc, 0x09a4, 0x9876, 0x3000),
        (0x9abe5e, 0xcac7bf, 0xffff, 0xfedc, 0xfedc, 0x09a4, 0x9876, 0x3008),
        (0x9a9234, 0xcae83f, 0xffff, 0x7f01, 0x7f01, 0x7f01, 0x1357, 0x3000),
        (0x9abe5e, 0xcaf83f, 0xffff, 0x4321, 0x4321, 0xbe5e, 0x1357, 0x3001),
        (0x1a9234, 0xcaf7bf,      0, 0x4321, 0x4321, 0x09a4, 0x9876, 0),
        (0x5a9234, 0xcaf7bf, 0xff00, 0x4321, 0x9234, 0x09a4, 0x1357, 0x2001),
    ] {
        let mode = if left & SEQUENTIAL == 0 { 4 } else { 5 };
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x6f000009a4), ("PACFG", 0x1357), ("MINMAX", 0x9876), ("CETBL", table)]);
        m.seed_data(0xffff, input);
        m.seed_data(0, 0x2222);
        m.seed_data((left & 0xffff) as u64, 0x7777);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("MINMAX"), m.reg("EST"), m.data(0xffff), m.data(0)),
                   (0x6f00000000 | a0l, record, est, word, 0x2222), "{left:06x}{right:06x}");
        assert_eq!(m.data((left & 0xffff) as u64), 0x7777);
    }
}

#[test]
fn wide_extrema_control_result_does_not_feed_its_own_ce() {
    let mut m = Machine::new(0, &[(0xee5ed6, 0xc83c3d), (NOP, NOP), (NOP, NOP)],
                             &[(0, 5, 6), (2, 1, 6), (4, 1, 0)], None);
    m.seed(&[("A3H", 0x1234), ("CECFG2", 0x2323), ("MINMAX", 0xbaba), ("CETBL", 0xffff)]);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("CECFG2"), m.reg("MINMAX"), m.reg("EST")), (0, 0xffff, 0x1000));
    assert_eq!(m.step(), 12);
    assert_eq!(m.reg("EST"), 0x3000);
}

#[test]
fn wide_extrema_reserved_result_still_rejects_lift() {
    let mut m = Machine::new(0, &[(0xc58743, 0xc84ca2), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    assert!(matches!(m.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })));
}

#[test]
fn compact_unbound_token_input_ports() {
    for (op, result, r0, r1, x1, est) in [
        (0x1203c2, 0xab1234fffe, 0x4126, 0x8127, 0x2b, 0x914),
        (0x1203e2, 0xab1234fffe, 0x4126, 0x8127, 0x2b, 0x914),
        (0x12c47c, 0xffffff8833, 0x4126, 0x8128, 0x2b, 0x914),
        (0x12c47e, 0xffffff8833, 0x4126, 0x8128, 0x2b, 0x914),
        (0x4161e4, 0x14d10, 0x4126, 0x8127, 0x2b, 0x904),
        (0x4161f4, 0x14d10, 0x4126, 0x8127, 0x2b, 0x904),
        (0x41043c, 0, 0x4127, 0x8127, 0xff80, 0x90c),
        (0x41043e, 0, 0x4127, 0x8127, 0xff80, 0x90c),
    ] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0xab12344321), ("A1L", 0xfffe), ("A2L", 0xfedc),
                 ("X1", 0x2b), ("X2", 0x13), ("EST", 0x924), ("R0", 0x4126), ("R1", 0x8127)]);
        m.seed_data(0x4126, 0xff80);
        m.seed_data(0x8127, 0x8833);
        m.start(0).unwrap_or_else(|error| panic!("{op:06x}: {error}"));
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("R0"), m.reg("R1"), m.reg("X1"), m.reg("X2"), m.reg("EST")),
                   (result, r0, r1, x1, 0x13, est), "{op:06x}");
        assert_eq!((m.data(0x4126), m.data(0x8127)), (0xff80, 0x8833));
    }

    for (cfg, table, a0, r0, x2, est) in [
        (0, 0, 0x54321, 0x127, 0x13, 0),
        (0x1000, 0, 0x54321, 0x124, 0x8833, 0),
        (0x1000, 0xff00, 0, 0x124, 0x8833, 0x2200),
    ] {
        let mut m = Machine::new(0, &[(NOP, 0x41043d), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0x54321), ("R0", 0x127), ("X1", 0x2b), ("X2", 0x13),
                 ("PACFG", 3), ("RM0", 3), ("RL0", 0x120), ("CECFG2", cfg), ("CETBL", table)]);
        m.seed_data(0x127, 0x8833);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("R0"), m.reg("X1"), m.reg("X2"), m.reg("EST")),
                   (a0, r0, 0x2b, x2, est));
    }

    let mut m = Machine::new(0, &[(NOP | SEQUENTIAL, 0x4161ec), (NOP, NOP)],
                             &[(0, 2, 3), (1, 3, 6), (2, 1, 0)], None);
    m.seed(&[("A2H", 0x8000), ("ECFG", 5), ("X1", 0x2b), ("X2", 0x13)]);
    m.start(1).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0"), m.reg("X1"), m.reg("X2"), m.reg("EST")), (0x7fffffff, 0x2b, 0x13, 0x24));

    for (op, source) in [(0x416024, 7), (0x4160a4, 11), (0x416164, 13)] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R2", 7), ("RN2", 11), ("FF1", 13), ("A2L", 9), ("X1", 0x2b)]);
        m.start(0).unwrap_or_else(|error| panic!("{op:06x}: {error}"));
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("X1")), (source * 9, source));
    }

    let mut m = Machine::new(0, &[(0x1203c2, INC_A1)], &[(0, 1, 0)], None);
    assert!(matches!(m.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })));
}

#[test]
fn compact_word_multiply_narrows_each_signedness_variant() {
    for (op, a, b, ecfg, word, flags) in [
        (0x424c9a, 3, 4, 0, 0, 0x08),
        (0x464c9a, 3, 4, 0, 0, 0x08),
        (0x4a4c9a, 3, 4, 0, 0, 0x08),
        (0x4e4c9a, 3, 4, 0, 0, 0x08),
        (0x424c9a, 0xfffe, 3, 0, 0xffff, 0x10),
        (0x4e4c9a, 0x1234, 0x2468, 0, 0x0296, 0),
        (0x424c9a, 0xfffe, 0xfffd, 0, 0, 0x08),
        (0x464c9a, 0xfffe, 0xfffd, 0, 0xfffe, 0x10),
        (0x4a4c9a, 0xfffe, 0xfffd, 0, 0xfffd, 0x10),
        (0x4e4c9a, 0xfffe, 0xfffd, 0, 0xfffb, 0x24),
        (0x424c9a, 0x8000, 0x8000, 5, 0x7fff, 0x24),
        (0x4e4c9a, 0x8000, 0x8000, 5, 0x8000, 0x20),
        (0x4e4c9a, 0x8000, 0x8000, 9, 0x8000, 0x20),
    ] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("A1", 0x6a834cecd0), ("R2", 0x8040), ("RN5", b), ("X1", 0xba98),
                 ("X2", 0x5678), ("CETBL", 0xffff), ("ECFG", ecfg)]);
        m.seed_data(0x8040, a);
        m.seed_data(0x8041, 0x7777);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A1"), m.reg("R2"), m.reg("X1"), m.reg("X2"), m.reg("EST")),
                   (0x6a0000ecd0 | (word << 16), 0x8041, a, 0x5678, 0x3000 | flags));
        assert_eq!((m.data(0x8040), m.data(0x8041)), (a, 0x7777));
    }
}

#[test]
fn compact_word_multiply_selects_destination_source_and_address_update() {
    for (op, dst, ptr, offset, address, input, factor, product, next) in [
        (0x42551c, "A2H", "R4", "RN6", 0x48, 0x3456, 0x4321, 0x0db9, 0x49),
        (0x425a56, "A3H", "R1", "RN3", 0x4040, 0x789a, 4, 1, 0x4045),
        (0x424150, "A0H", "R5", "RN0", 0x4048, 0x4567, 0x2345, 0x098f, 0x4048),
        (0x424e9a, "A1H", "R2", "RN5", 0x8040, 0x2222, 0x3333, 0x06d3, 0x803f),
        (0x420c9a, "A1L", "R2", "RN5", 0x8040, 0x2222, 0x3333, 0x06d3, 0x8041),
    ] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[(dst, 0x1234), (ptr, address), (offset, factor), ("RN1", 5), ("CETBL", 0xffff)]);
        m.seed_data(address, input);
        if address != next {
            m.seed_data(next, 0x1111);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(dst), m.reg(ptr), m.reg("X1"), m.reg("EST")),
                   (product, next, input, 0x3000));
    }

    let mut m = Machine::new(0, &[(NOP, 0x4e62da), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    m.seed(&[("R3", 0xc040), ("RN3", 4), ("RN5", 4), ("A4", 0x80f4c5af94), ("EST", 0x24),
             ("X1", 0xba98), ("CETBL", 0xffff)]);
    m.seed_data(0xc040, 0x0786);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A4"), m.reg("R3"), m.reg("X1"), m.reg("X2"), m.reg("EST")),
               (0x800000af94, 0xc044, 0xba98, 0x0786, 0x3224));
}

#[test]
fn compact_word_multiply_samples_the_qualified_round_peer() {
    for (left, first, second, ecfg, table, ce1, old, x1, a5, flags) in [
        (0x57680f, 0xa258, 0xa25e, 0, 0xffff, 0x1000, 0, 0xa258, 0x2241, 0x3200),
        (0x57680f, 0x0001, 0xffff, 0, 0xffff, 0x1000, 0, 0x0001, 0, 0x3209),
        (0x57680f, 0x4000, 0x0002, 2, 0xffff, 0x1000, 0, 0x4000, 1, 0x3200),
        (0x57680f, 0x8000, 0x8000, 5, 0xffff, 0x1000, 0, 0x8000, 0x7fff, 0x3224),
        (0x57680f, 0xa258, 0xa25e, 0, 0, 0, 4, 0xba98, 0xd1dd, 0x0204),
        (0x57680f, 0xa258, 0xa25e, 0, 0xff00, 0x1000, 4, 0xa258, 0xd1dd, 0x2204),
        (0x57404d, 0x0123, 0x4567, 0, 0xffff, 0x1000, 0, 0x0123, 0x004f, 0x3200),
        (0x57680f, 0x6000, 6, 0, 0x0202, 8, 8, 0x6000, 2, 0x0200),
    ] {
        let mut m = Machine::new(0, &[(left, 0x4e62da), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        let (l1, l2, dst) = if left == 0x57404d { (0x4040, 0x8048, "A0") } else { (0x40, 0xc048, "A5") };
        m.seed(&[(dst, 0x89d1dd24b5), ("A4", 0x80f4c5af94), ("X1", 0xba98), ("X2", 0x5678),
                 ("R0", 0x40), ("R1", 0x4040), ("R6", 0x8048), ("R7", 0xc048),
                 ("R3", 0xc040), ("RN3", 4), ("RN5", 4), ("ECFG", ecfg), ("CETBL", table),
                 ("CECFG1", ce1), ("EST", old)]);
        m.seed_data(l1, first);
        m.seed_data(l2, second);
        m.seed_data(0xc040, 0x0786);
        m.seed_data(0xc044, 0xfedc);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg(dst), m.reg("A4"), m.reg("R3"), m.reg("X1"), m.reg("X2"), m.reg("EST")),
                   (0x89000024b5 | (a5 << 16), 0x800000af94, 0xc044, x1, 0x0786, flags),
                   "{left:06x}: {first:04x} * {second:04x}, CE={ce1:04x}/{table:04x}");
        assert_eq!((m.data(l1), m.data(l2), m.data(0xc040), m.data(0xc044)),
                   (first, second, 0x0786, 0xfedc));
    }
}

#[test]
fn wide_intbase_add_control_ports() {
    for (left, right, a0l, est, table, config, expected, flags) in [
        (0x0daa23, 0x91add1, 0xffff,     0, 0xffff, 0, 0xd1dd, 0x3034),
        (0x0daa23, 0x91adf1, 0xffff,     0, 0xffff, 4, 0xd1dd, 0x3030),
        (0x104250, 0x91adc5, 0xffff, 0x264, 0xffff, 0, 0x0080, 0x3245),
        (0x10aa23, 0x91ade7, 0xffff,     0, 0xffff, 0, 0x0020, 0x3001),
        (0x10aa23, 0x91adc9, 0x1234,     0, 0xffff, 0, 0x6385, 0x3000),
        (0x10aa23, 0x91adcb, 0xffff,     0, 0xffff, 0, 0x0000, 0x3009),
        (0x10aa23, 0x91adcf, 0xffff,     0, 0xffff, 0, 0x0000, 0x3009),
        (0x10aa23, 0x91adf1, 0xffff,     0, 0xffff, 0, 0xffff, 0x3010),
        (0x11aa23, 0x91ade9, 0xffff,     0, 0xffff, 0, 0x431f, 0x3001),
        (0x11aa23, 0x91aded, 0xffff,     0, 0xffff, 0, 0x8133, 0x3010),
        (0x11aa23, 0x91adf5, 0xffff,     0, 0xffff, 0, 0x5190, 0x3000),
        (0x10aa23, 0x91adc5, 0xffff,     0, 0xff00, 0, 0x8123, 0x2000),
        (0x50aa23, 0x91adc5, 0xffff,     0, 0x00ff, 0, 0x8123, 0x1000),
        (0x90aa23, 0x91adc5, 0xffff,     0,      0, 0, 0x0080, 0x0001),
    ] {
        let mode = if left & SEQUENTIAL == 0 { 4 } else { 5 };
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, mode, 6), (2, 1, 0)], None);
        let count = if right == 0x91ade9 { 0xfffe } else { 0x5151 };
        let a1l = if right == 0x91ade9 { 0x4321 } else { 0x10 };
        m.seed(&[("A0L", a0l), ("A1L", a1l), ("A5", 0x89d1dd24b5),
                 ("LSA1", 0x807726), ("LEA1", 0x20df32), ("LC1", count),
                 ("PSTATUS", 0x10), ("USER_IO", 7), ("PACFG", 0x5180), ("INTBASE", 0x8123),
                 ("ECFG", config), ("EST", est), ("CETBL", table)]);
        m.seed_data(0, 0xd35e);
        m.seed_data(0xffff, 0xa5b3);
        m.start(0).unwrap_or_else(|err| panic!("{left:06x}{right:06x}: {err:?}"));
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("INTBASE"), m.reg("EST")), (expected, flags), "{left:06x}{right:06x}");
        assert_eq!((m.reg("A0L"), m.reg("A1L"), m.reg("LC1")), (a0l, a1l, count));
        assert_eq!((m.data(0), m.data(0xffff)), (0xd35e, 0xa5b3));
    }

    let mut hole = Machine::new(0, &[(0x10aa23, 0x91adc1), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
    assert!(matches!(hole.start(0), Err(KunaError::BadData { .. })));
    let mut data = Machine::new(0, &[(0x10aa23, 0x91ad1f), (NOP, NOP)], &[(0, 4, 6), (2, 1, 0)], None);
    assert!(matches!(data.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })));
}

#[test]
fn wide_lshift_control_aliases() {
    for (left, right, count, input, expected, flags) in [
        (0x9a097e, 0xf1c28b, 8, 0x01000000, None, 0x3001),
        (0x9a097e, 0xf1c28b, 0x7f, 0x80000001, None, 0x3001),
        (0x9a097e, 0xf1c28b, 0x60, 0x80000000, None, 0x3001),
        (0x9a097e, 0xf1c28b, 0x58, 0x80000000, None, 0x3000),
        (0xd0097e, 0xf1b08b, 0, 1, None, 0x3008),
        (0xd0097e, 0xf1b08b, 0x7f, 0x10000, None, 0x3009),
        (0xd0097e, 0xf1b08b, 0x7f, 0x80000001, None, 0x3000),
        (0xd0097e, 0xf1b08b, 1, 0x80000000, None, 0x3021),
        (0xd0097e, 0xf1b08b, 33, 1, None, 0x3020),
        (0xd0097e, 0xf1108b, 0, 1, Some(1), 0x3000),
        (0xd0097e, 0xf1108b, 0x7f, 0x80000001, Some(0x40000000), 0x3001),
        (0xd0097e, 0xf1108b, 1, 0x80000000, Some(0x100000000), 0x3020),
        (0xd0097e, 0xf1108b, 0x3f, 0x80010001, Some(0), 0x3009),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        let a0 = 0xe9ab000000 | count;
        m.seed(&[("A0", a0), ("A2H", count), ("R5", 0x4048), ("ECFG", 4), ("CETBL", 0xffff)]);
        m.seed_data(0x4048, input & 0xffff);
        m.seed_data(0x4049, input >> 16);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), m.reg("EST")), (expected.unwrap_or(a0), flags));
        assert_eq!(m.reg("R5"), 0x4048);
        assert_eq!((m.data(0x4048), m.data(0x4049)), (input & 0xffff, input >> 16));
    }

    for (left, address, user_io, count, table, old, flags) in [
        (0xe42e2d, 0x75, 0x1111, 1, 0xffff, 0, 0x3008),
        (0xe42e2d, 0x75, 0x1111, 8, 0xffff, 0, 0x3000),
        (0xe42e2d, 0x75, 0, 8, 0xffff, 0, 0x3008),
        (0xe42e2d, 0x75, 0x8000, 1, 0, 0, 0),
        (0xe42e2d, 0x75, 0x1111, 0x80, 0xffff, 1, 0x3009),
        (0xe42e2d, 0x75, 1, 33, 0xffff, 0, 0x3021),
        (0xe42e2d, 0x75, 1, 63, 0xffff, 0, 0x3009),
        (0xe48081, 0xffc9, 0x1111, 8, 0xffff, 0, 0x3000),
    ] {
        let mut m = Machine::new(0, &[(left, 0xf1b1ee), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("R4", 0x48), ("USER_IO", user_io), ("A0L", 7), ("CETBL", table), ("EST", old)]);
        m.seed_data(address, count);
        m.seed_data(0xffff, 0x4780);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("USER_IO"), m.reg("R4"), m.reg("A0L")), (flags, user_io, 0x48, 7));
        assert_eq!((m.data(address), m.data(0xffff)), (count, 0x4780));
    }

    let mut m = Machine::new(0, &[(0xd0097e, 0xf1108b), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A0L", 1), ("R5", 0xffff), ("CETBL", 0)]);
    m.seed_data(0xffff, 1);
    m.seed_data(0, 0x8001);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!((m.reg("A0"), m.reg("R5"), m.reg("EST")), (0x100020002, 0xffff, 0x20));

    for (right, source, input, count, result, flags) in [
        (0xf1ade9, "LC1", 0x5566, 1, 0xaacc, 0x3020),
        (0xf1ade9, "LC1", 0x8001, 0x7f, 0x4000, 0x3001),
        (0xf1ade9, "LC1", 0x7fff, 16, 0, 0x3031),
        (0xf1add5, "PACFG", 0x4499, 0x7f, 0x224c, 0x3001),
        (0xf1add5, "PACFG", 0xffff, 1, 0xfffe, 0x3021),
        (0xf1aded, "INTBASE", 0xa561, 0x7f, 0x52b0, 0x3001),
    ] {
        let mut m = Machine::new(0, &[(0xd0097e, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0L", count), ("INTBASE", 0x3421), ("CETBL", 0xffff), ("ECFG", 4), (source, input)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("INTBASE"), m.reg("EST")), (result, flags), "{source}");
        assert_eq!(m.reg("A0L"), count);
        if source != "INTBASE" {
            assert_eq!(m.reg(source), input);
        }
    }

    let mut m = Machine::new(0, &[(0x9a097e, 0xf1c28b), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    m.seed(&[("A2H", 0), ("R5", 0x4048), ("EST", 9), ("CETBL", 0xffff)]);
    m.seed_data(0x4048, 1);
    m.start(0).unwrap();
    assert_eq!(m.step(), 6);
    assert_eq!(m.reg("EST"), 0x3009);
}

#[test]
fn wide_arithmetic_result_port_lanes() {
    let cases = [
        (1, 0x911103, 0xea433809a4, 0xf100000000, 0, 0xf2, 0x3000),
        (0x8000, 0x911103, 0xea433809a4, 0x8000000000, 0, 0xffffff8080, 0x3010),
        (1, 0x912111, 0x1234, 0xf100000000, 0, 0, 0x3008),
        (1, 0x912111, 0x007fffffff, 0xf100000000, 4, 0x7fff7fff, 0x3024),
        (0, 0x918311, 0x0180000001, 0xf100000000, 0, 0x0180000001, 0x3024),
        (0xffff, 0x918311, 0x8000000000, 0xf100000000, 0, 0x8000000000, 0x3027),
    ];
    let mut packets = [(NOP, NOP); 7];
    packets[..6].copy_from_slice(&cases.map(|(imm, right, ..)| (0x8f0000 | imm, right)));
    let issues = [(0, 5, 6), (2, 5, 6), (4, 5, 6), (6, 5, 6), (8, 5, 6), (10, 5, 6), (12, 1, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (n, (_, _, a0, a1, config, result, flags)) in cases.into_iter().enumerate() {
        let pc = 2 * n as u64;
        m.seed(&[("A0", a0), ("A1", a1), ("CETBL", 0xffff), ("ECFG", config), ("EST", 0)]);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("A0"), m.reg("A1"), m.reg("EST")), (result, a1, flags));
    }

    let packets = [(0x842345, 0x810f20), (NOP, NOP), (0x842345, 0x8085ca), (0x842345, 0x810dc6)];
    let issues = [(0, 5, 6), (2, 1, 6), (4, 5, 6), (6, 5, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (address, input, a0l, config, result, flags) in [
        (0x48, 0x5df9, 0x09a4, 0, 0xabab, 0x3010),
        (0x1240, 0x71c7, 0x1240, 0, 0xa079, 0x3010),
        (0x48, 0xee50, 0x1240, 0x20, 0x11f8, 0x3000),
        (0x48, 0x8000, 0x7fff, 8, 0x7fff, 0x3024),
    ] {
        m.seed(&[("R4", address), ("A0L", a0l), ("CETBL", 0xffff), ("ECFG", config), ("EST", 0)]);
        m.seed_data(address, input);
        m.seed_data(address + 1, 0x6c19);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0L"), m.reg("R4"), m.reg("EST")), (a0l, address, flags));
        assert_eq!((m.data(address), m.data(address + 1)), (result, 0x6c19));
    }
    for pc in [4, 6] {
        assert!(matches!(m.start(pc), Err(KunaError::Unimpl { instruction_length: 6, .. })));
    }
}

#[test]
fn wide_addl_routes_the_full_accumulator_input() {
    let packets = [(0x8aaa3f, 0xc21222), (NOP, NOP), (0x8aaa3f, 0xc2103e), (0x8aaa3f, 0xc2113e),
                   (0x8aaa3f, 0xc2123e)];
    let issues = [(0, 5, 6), (2, 1, 6), (4, 5, 6), (6, 5, 6), (8, 5, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (a0, a2, ecfg, result, flags) in [
        (0xea433809a4, 0x10, 0, 0xea433809b4, 0x30),
        (0xea433809a4, 0xf123456789, 0, 0xdb667d712d, 0x37),
        (0x8000000000, 1, 0, 0x8000000001, 0x30),
        (0x100000, 0x00123456, 0, 0x223456, 0),
        (0x100000, 0xff00123456, 0, 0xff00223456, 0x36),
        (0, 0x0080000000, 4, 0x7fffffff, 0x24),
        (0, 0xff7fffffff, 4, 0xff80000000, 0x36),
        (0, 0xff7fffffff, 8, 0xff7fffffff, 0x32),
        (0, 0xff7fffffff, 0x28, 0xff7fffffff, 0x36),
    ] {
        m.seed(&[("A0", a0), ("A2", a2), ("R7", 0xc060), ("RN7", 4), ("ECFG", ecfg), ("EST", 0)]);
        m.seed_data(0xc060, 0x7777);
        m.seed_data(0xc061, 0x5678);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R7"), m.reg("A0"), m.reg("EST")), (0xc060, result, flags));
        assert_eq!((m.reg("A2"), m.data(0xc060), m.data(0xc061)), (a2, 0x7777, 0x5678));
    }
    for pc in [4, 6, 8] {
        assert!(matches!(m.start(pc), Err(KunaError::Unimpl { instruction_length: 6, .. })));
    }
}

#[test]
fn wide_lshift_loop_input_feedback() {
    let mut issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 6), (8, 1, 6), (10, 5, 6), (12, 1, 0)];
    for (right, count, a1l, expected_a0, ff1, est) in [
        (0xf17d22, 1, 0, 0xea43380000, 0, 0x3008),
        (0xf17d22, 0, 0x8001, 0xea43380000, 0x8001, 0x3021),
        (0xf17d22, 0x7f, 0x8001, 0xea43380000, 0x4000, 0x3001),
        (0xf17d22, 5, 0x0800, 0xea43380000, 0, 0x3021),
        (0xf111a6, 1, 0, 0, 0x1234, 0x3008),
        (0xf111a6, 0, 0, 0, 0x1234, 0x3009),
    ] {
        let mut packets = [(NOP, NOP); 7];
        packets[0] = (INC_A0, NOP);
        packets[5] = (0xc10000, right);
        let mut m = Machine::new(0x1000, &packets, &issues, None);
        m.seed(&[("A0", 0xea4338ffff), ("A1L", a1l), ("R1", 0x4040),
                 ("FF1", 0x1234), ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff)]);
        m.seed_data(0x4040, count);
        m.start(0x1000).unwrap();
        for word in (0x1002..=0x100a).step_by(2) {
            assert_eq!(m.step(), 3 * word);
        }
        assert_eq!(m.reg("EST"), 0x3009);
        assert_eq!(m.step(), 3 * 0x100c);
        assert_eq!((m.reg("A0"), m.reg("FF1"), m.reg("EST")), (expected_a0, ff1, est));
        assert_eq!((m.reg("A1L"), m.reg("R1"), m.data(0x4040)), (a1l, 0x4040, count));
    }

    // An ordinary LOAD populates the selected saved bank before the original producer and shift.
    let mut packets = [(NOP, NOP); 12];
    packets[0] = (0x01fa02, 0xc27b4d);
    packets[5] = (INC_A0, NOP);
    packets[10] = (0xc10000, 0xf111a6);
    let mut with_bound = [(0, 1, 6); 12];
    for (i, context) in with_bound.iter_mut().enumerate() {
        *context = (2 * i as u64, 1, 6);
    }
    with_bound[0].1 = 4;
    with_bound[10].1 = 5;
    with_bound[11].2 = 0;
    for (depth, saved) in [(0, "LEA1"), (9, "LEA1"), (10, "LEA2"), (11, "LEA3"), (12, "LEA4")] {
        let mut m = Machine::new(0x0ff6, &packets, &with_bound, None);
        m.seed(&[("A0", 0xea4338ffff), ("R1", 0x4040), ("PSTATUS", depth),
                 ("CECFG1", 0x1000), ("CECFG2", 0x1000), ("CETBL", 0xffff)]);
        m.seed_data(0x4040, 0x7f);
        m.seed_data(0x4041, 0x80);
        m.start(0x0ff6).unwrap();
        for word in (0x0ff8..=0x100a).step_by(2) {
            assert_eq!(m.step(), 3 * word);
        }
        assert_eq!((m.reg(saved), m.reg("EST")), (0x80007e, 0x3009));
        assert_eq!(m.step(), 3 * 0x100c);
        assert_eq!((m.reg("A0"), m.reg("EST")), (0x40003f, 0x3000));
    }

    issues[0].1 = 5;
    for right in [0xf17da4, 0xf17da6, 0xf17db0, 0xf1b1a6, 0xf1b1c4] {
        let mut m = Machine::new(0, &[(0xc10000, right), (NOP, NOP)], &issues[..2], None);
        assert!(matches!(m.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })), "{right:06x}");
    }
}

#[test]
fn packed_min2_literal_writes_only_the_upper_minimum_and_both_traces() {
    for (left, right, source, destination, result, trace) in [
        (0x8fe26b, 0xc5d395, 0x0080007fff, "A3", 0x6f55668000, (0x4add, 0x4c6a)),
        (0x8fffff, 0xc5d295, 0x80ffff8000, "A2", 0x80ffffffff, (0x4adc, 0x4c6b)),
        (0x8f7fff, 0xc5d395, 0xff00008000, "A3", 0x6f55660000, (0x4adc, 0x4c6b)),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A2", source), ("A3", 0x6f55662222), ("EST", 0x83a5), ("CETBL", 0xffff),
                 ("BTR1", 0xa56e), ("BTR2", 0xa635)]);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!(m.reg(destination), result);
        let unchanged = if destination == "A3" { ("A2", source) } else { ("A3", 0x6f55662222) };
        assert_eq!(m.reg(unchanged.0), unchanged.1);
        assert_eq!((m.reg("BTR1"), m.reg("BTR2")), trace);
        assert_eq!(m.reg("EST"), 0xb3a5);
    }
}

#[test]
fn packed_banks_displacements_compact() {
    for (left, right, seed, stored, flags) in [
        (0x696624, NOP, 0, 0x24, 0x400),
        (NOP, 0x695624, 0, 0x42, 0x10),
        (0x697622, NOP, 0, 0x20, 0x400),
        (NOP, 0x697622, 0, 0x20, 0x10),
        (0x697622, NOP, 1, 0, 0x408),
        (NOP, 0x697622, 1, 0, 0x210),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R1", 0x20), ("R2", 0x30), ("R3", 0x40), ("R5", 0x50), ("R6", 0x60),
                 ("R7", 0x70), ("EST", 0x410)]);
        for (address, word) in [(0x20, 2), (0x30, 4), (0x40, 8), (0x50, 0x20), (0x60, 0x40), (0x70, 0x80)] {
            m.seed_data(address, if seed == 0 { word } else { 0 });
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        let dst = if left == 0x696624 { 0x70 } else { 0x40 };
        assert_eq!((m.data(dst), m.reg("EST")), (stored, flags));
        assert_eq!((m.reg("R3"), m.reg("R7")), (0x40, 0x70));
    }

    for (op, ptr, desc, r3, r7) in [
        (0x36ae0e, 0x40, 0x70, 0x44, 0x70),
        (0x36ae06, 0x70, 0x40, 0x40, 0x78),
    ] {
        let mut m = Machine::new(0, &[(op, NOP), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
        m.seed(&[("R3", 0x40), ("R7", 0x70), ("RN3", 4), ("RN7", 8), ("A0", 0x880055)]);
        m.seed_data(ptr, 0x1234);
        m.seed_data(desc, 12);
        m.seed_data(desc + 1, 1);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.data(ptr), m.data(desc), m.data(desc + 1)), (0x2a, 12, 1));
        assert_eq!((m.reg("R3"), m.reg("R7"), m.reg("A0"), m.reg("EST")), (r3, r7, 0x880055, 0));
    }
}

#[test]
fn packed_banks_displacements_wide() {
    for (left, right, a0, ptrs, word0, word1) in [
        (0xa20701, 0xe22084, 0x410, (0x100, 0x200, 0x300), 1, 2),
        (0x900701, 0xe0c48a, 0x0a0, (0x100, 0x201, 0x300), 1, 0x4a0),
        (0xa20701, 0xe0c084, 0x0a0, (0x101, 0x200, 0x300), 0x84, 2),
        (0xb20701, 0xe220c4, 6, (0x100, 0x201, 0x307), 1, 2),
        (0xa27f80, 0xe22084, 0x8040, (0x100, 0x200, 0x300), 1, 2),
        (0xb27f80, 0xe220c4, 6, (0x100, 0x180, 0x37f), 1, 2),
        (0xa20701, 0xe0c484, 0x0a0, (0x100, 0x201, 0x300), 1, 0x84),
    ] {
        let mut m = Machine::new(0, &[(left, right), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
        m.seed(&[("A0", 0xa0), ("R0", 0x100), ("R1", 0x200), ("R2", 0x300), ("EST", 0x410)]);
        for (address, word) in [
            (0x100, 1), (0x180, 0x8000), (0x200, 2), (0x201, 0x10), (0x207, 0x80),
            (0x300, 4), (0x301, 0x200), (0x307, 0x400), (0x37f, 0x40),
        ] {
            m.seed_data(address, word);
        }
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("A0"), (m.reg("R0"), m.reg("R1"), m.reg("R2")), m.reg("EST")), (a0, ptrs, 0x400));
        assert_eq!((m.data(0x100), m.data(0x200), m.data(0x300)), (word0, word1, 4));
    }

    let mut m = Machine::new(0, &[(0xb20701, 0xe220c8), (NOP, NOP)], &[(0, 5, 6), (2, 1, 0)], None);
    assert!(matches!(m.start(0), Err(KunaError::Unimpl { instruction_length: 6, .. })));
}

#[test]
fn compact_frames_publish_ce_feedback() {
    let packets = [(0x2c41b0, NOP), (0x2c5800, NOP), (0x2c5801, NOP), (NOP, NOP)];
    let issues = [(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (pc, table, ce1, est, sp, frame, saved) in [
        (0, 0x0001, 0, 0x5000, 0x8008, 0x802f, 0xffff),
        (0, 0x0100, 0, 0x6000, 0x8008, 0x802f, 0xffff),
        (2, 0x0101, 0, 0x3000, 0, 0xaa00, 0xbe27),
        (4, 0x0001, 0, 0x1000, 0, 0xaa00, 0xbe27),
        (4, 0x0100, 0, 0x2000, 0x8030, 0xffff, 0xbe27),
        (4, 0x0100, 0x1000, 0x2000, 0, 0xaa00, 0xbe27),
    ] {
        m.seed(&[("R0", 0xffff), ("SP", 0x8030), ("EST", 0), ("ECFG", 0x10),
                 ("CECFG1", ce1), ("CECFG2", 0), ("CETBL", table)]);
        m.seed_data(0, 0xaa00);
        m.seed_data(0x8030, 0xbe27);
        m.seed_data(0x8008, 0x9820);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("EST"), m.reg("SP"), m.reg("R0")), (est, sp, frame),
                   "CODE[{pc}], CETBL={table:#x}, CECFG1={ce1:#x}");
        assert_eq!((m.data(0), m.data(0x8030), m.data(0x8008)), (0xaa00, saved, 0x9820));
    }
}

#[test]
fn wide_link_publishes_ce_after_frame_address_phase() {
    let packets = [
        (0xbffffc, 0xc60283), (NOP, NOP), (0x3ffffc, 0xc60283), (NOP, NOP),
        (0x2d521e, 0xc62fc0), (NOP, NOP), (0x3ffffc, 0xc60313), (NOP, NOP),
        (0x7ffffc, 0xc60313), (NOP, NOP),
    ];
    let issues = [
        (0, 5, 6), (2, 1, 6), (4, 4, 6), (6, 1, 6), (8, 4, 6),
        (10, 1, 6), (12, 4, 6), (14, 1, 6), (16, 4, 6), (18, 1, 0),
    ];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (pc, table, ce1, expected) in [
        (0, 0x0100, 0, (0x6000, 0x802c, 0x802f, 0x0100, 0x0400)),
        (4, 0x0100, 0, (0x2000, 0x8030, 0x0400, 0x0100, 0x5aa5)),
        (4, 0x0100, 0x1000, (0x2000, 0x802c, 0x802f, 0x0100, 0x5aa5)),
        (8, 0x0001, 0, (0x5000, 0xd24e, 0x0400, 0x0001, 0xffff)),
        (12, 0x0100, 0x1000, (0x1000, 0x802c, 0x0400, 0x802f, 0x0100)),
        (16, 0x0101, 0, (0x1000, 0x802c, 0x0400, 0x802f, 0x5aa5)),
    ] {
        m.seed(&[("SP", 0x8030), ("R0", 0x0400), ("EST", 0), ("ECFG", 0x10),
                 ("CECFG1", ce1), ("CECFG2", 0), ("CETBL", table)]);
        m.seed_data(0x8030, 0x5aa5);
        m.seed_data(0x802f, 0x1234);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("EST"), m.reg("SP"), m.reg("R0"), m.reg("CETBL"), m.data(0x8030)),
                   expected, "CODE[{pc}], CETBL={table:#x}, CECFG1={ce1:#x}");
        assert_eq!(m.data(0x802f), 0x1234);
    }
}

#[test]
fn wide_single_stack_uses_address_enable_and_publishes_both_latches() {
    let packets = [
        (0xa0b91c, 0xcb1456), (0x60b91c, 0xcb1456), (0x20b91c, 0xcb1452),
        (0x39b91c, 0xcb1452), (0x988b47, 0xcb3aa7), (NOP, NOP),
    ];
    let issues = [(0, 5, 6), (2, 4, 6), (4, 4, 6), (6, 4, 6), (8, 5, 6), (10, 1, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (pc, table, address_enable, est, ecfg, expected) in [
        (0, 0x0100, 0, 0x0000, 0x10, (0x6000, 0x802f, 0x0400, 3, 0x0400)),
        (0, 0x0001, 0, 0x1024, 0x10, (0x5024, 0x802f, 0x0400, 3, 0x0400)),
        (0, 0x0001, 0, 0x0000, 0x00, (0x1000, 0x802f, 0x0400, 3, 0x0400)),
        (2, 0x0001, 0, 0x0000, 0x10, (0x1000, 0x8030, 0x0400, 3, 0x5aa5)),
        (2, 0x0001, 0x1000, 0x1024, 0x10, (0x1024, 0x802f, 0x0400, 3, 0x5aa5)),
        (2, 0x0100, 0, 0x0000, 0x10, (0x6000, 0x802f, 0x0400, 3, 0x0400)),
        (4, 0x0100, 0, 0x0000, 0x10, (0x2000, 0x8030, 0x0400, 3, 0x5aa5)),
        (4, 0x0100, 0x1000, 0x2024, 0x10, (0x2024, 0x8031, 0x8abc, 3, 0x5aa5)),
        (6, 0x0100, 0x1000, 0x0000, 0x10, (0x2000, 0x8031, 0x0400, 0, 0x5aa5)),
        (8, 0x0101, 0, 0x4024, 0x10, (0x7024, 0x802f, 0x0400, 3, 0)),
    ] {
        m.seed(&[("SP", 0x8030), ("R0", 0x0400), ("ASTATUS", 3), ("EST", est),
                 ("ECFG", ecfg), ("CECFG1", address_enable), ("CECFG2", address_enable),
                 ("CETBL", table)]);
        m.seed_data(0x802f, 0x1234);
        m.seed_data(0x8030, 0x5aa5);
        m.seed_data(0x8031, 0x8abc);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("EST"), m.reg("SP"), m.reg("R0"), m.reg("ASTATUS"), m.data(0x8030)),
                   expected, "CODE[{pc}], CETBL={table:#x}, AEN={address_enable:#x}");
        assert_eq!((m.data(0x802f), m.data(0x8031)), (0x1234, 0x8abc));
    }
}

#[test]
fn compact_shift_and_pop_use_independent_predicates() {
    let mut m = Machine::new(0, &[(0x3966a3, 0x303123), (NOP, NOP)], &[(0, 1, 6), (2, 1, 0)], None);
    for (table, ce2, incoming, rn5, sp, expected) in [
        (0x0000, 0,      0x0000,  1, 0x8030, (0x0000, 0x8030, 0xbeef, 0x5252, 0x2323)),
        (0x0001, 0,      0x1009,  1, 0x8030, (0x1020, 0x8030,      0, 0x5252, 0x2323)),
        (0x0001, 0x1000, 0x1024, 64, 0xffff, (0x1034,      1,      0, 0x84bd, 0x9365)),
        (0x0100, 0,      0x2024, 64, 0x8030, (0x2024, 0x8032, 0xbeef, 0x84bd, 0x9365)),
        (0x0101, 0,      0x3009,  0, 0xfffe, (0x3008,      0,      0, 0x84bd, 0x9365)),
    ] {
        m.seed(&[("CECFG1", 0), ("CECFG2", ce2), ("CETBL", table), ("SP", sp),
                 ("EST", incoming), ("RN5", rn5), ("FF1", 0xbeef), ("FF2", 0xface),
                 ("R5", 0x5252), ("RL2", 0x2323)]);
        let top = (sp + 1) & 0xffff;
        let next = (sp + 2) & 0xffff;
        m.seed_data(top, 0x84bd);
        m.seed_data(next, 0x9365);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("EST"), m.reg("SP"), m.reg("FF1"), m.reg("R5"), m.reg("RL2")), expected);
        assert_eq!((m.reg("FF2"), m.data(top), m.data(next)), (0xface, 0x84bd, 0x9365));
    }
}

#[test]
fn compact_inactive_shifter_updates_only_its_address_port() {
    let packets = [
        (0x575092, 0x3b982b), (0x575092, 0x3b9b2b), (0x1605e7, 0x3b933f), (NOP, NOP),
    ];
    let mut m = Machine::new(0, &packets, &[(0, 1, 6), (2, 1, 6), (4, 1, 6), (6, 1, 0)], None);
    for (pc, table, right_aen, ecfg, r1, expected) in [
        (0, 0, 0x1000, 0, 0x4040, (0x4042, 0x0001, 0x8700004b74, 0x0008)),
        (0, 0xffff, 0, 1, 0x4040, (0x4042, 0x0001, 0x8700014b74, 0x3000)),
        (2, 0, 0, 0, 0x4040, (0x4042, 0x0001, 0x8700004b74, 0x0008)),
    ] {
        m.seed(&[("EST", 0), ("ECFG", ecfg), ("CETBL", table), ("CECFG2", right_aen),
                 ("PACFG", 0), ("R1", r1), ("RN1", 2), ("R2", 0x8040), ("A2", 0x87b0084b74),
                 ("X1", 0xba98)]);
        m.seed_data(0x4040, 0x4000);
        m.seed_data(0x8040, 1);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("R1"), m.reg("X1"), m.reg("A2"), m.reg("EST")), expected);
    }
    for (table, right_aen, ecfg, carry, expected) in [
        (0, 0, 0, 0, (0x4047, 0x6a834cecd0, 0x0000)),
        (0, 0x1000, 0, 0, (0x404e, 0x6a834cecd0, 0x0000)),
        (0xffff, 0, 0, 0, (0x404e, 0x6a834cfffe, 0x3024)),
        (0xffff, 0, 0, 1, (0x404e, 0x6a834cffff, 0x3024)),
        (0xffff, 0, 4, 0, (0x404e, 0x6a834c7fff, 0x3024)),
    ] {
        m.seed(&[("EST", carry), ("ECFG", ecfg), ("CETBL", table), ("CECFG1", 0x1000),
                 ("CECFG2", right_aen), ("R1", 0x4047), ("PACFG", 8), ("RM0", 7),
                 ("RN7", 0x7fff), ("A3L", 0x7fff), ("A1", 0x6a834cecd0), ("A4", 0x8765432100)]);
        m.start(4).unwrap();
        assert_eq!(m.step(), 18);
        assert_eq!((m.reg("R1"), m.reg("A1"), m.reg("EST")), expected);
        assert_eq!(m.reg("A4"), 0x8765432100);
    }
}

#[test]
fn compact_direct_stack_checks_the_connected_high_word_and_publishes_ce() {
    let mut m = Machine::new(
        0, &[(0x303400, NOP), (0xb0378c, NOP), (NOP, 0x303000), (NOP, NOP)],
        &[(0, 1, 6), (2, 2, 3), (3, 3, 6), (4, 1, 6), (6, 1, 0)], None,
    );
    for (pc, high, low, expected) in [
        (0, 0x8001, 0x0123, 0x7000),
        (0, 0x0123, 0x8001, 0x3000),
        (2, 0x8001, 0x0123, 0x3000),
    ] {
        m.seed(&[("SP", 0x8030), ("EST", 0), ("ECFG", 0x10), ("CETBL", 0x0101),
                 ("R0", high), ("R4", low), ("FF1", high), ("FF2", low)]);
        m.start(pc).unwrap();
        let end = if pc == 0 { 6 } else { 9 };
        assert_eq!(m.step(), end);
        assert_eq!((m.reg("SP"), m.data(0x8030), m.data(0x802f), m.reg("EST")),
                   (0x802e, high, low, expected));
    }
    m.seed(&[("SP", 0x8030), ("EST", 0x4024), ("CETBL", 0x0100)]);
    m.seed_data(0x8031, 0x8000);
    m.seed_data(0x8032, 0xffff);
    m.start(4).unwrap();
    assert_eq!(m.step(), 18);
    assert_eq!((m.reg("SP"), m.reg("R0"), m.reg("R4"), m.reg("EST")),
               (0x8032, 0xffff, 0x8000, 0x6024));
}

#[test]
fn wide_extract_cetbl_publishes_feedback_before_table_writeback() {
    let packets = [
        (0x8f0e00, 0xf589c1), (NOP, NOP),
        (0x8f0e00, 0xf589c5), (NOP, NOP),
        (0x8f0e00, 0xf589cb), (NOP, NOP),
        (0x8f0000, 0xf589c1), (NOP, NOP), (NOP, NOP),
    ];
    let issues = [(0, 5, 6), (2, 1, 6), (4, 5, 6), (6, 1, 6), (8, 5, 6), (10, 1, 6),
                  (12, 5, 6), (14, 1, 6), (16, 1, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (pc, table, cecfg1, cecfg2, est, expected) in [
        (0, 0x0001, 0x0666, 0x0666, 0, (0xe000, 0x1010, 0x2010)),
        (0, 0xffff, 0x0225, 0x1000, 0x3009, (0xe000, 0x3011, 0x0011)),
        (4, 0x0000, 0x0225, 0x1000, 0, (0x0225, 0x0000, 0x1000)),
        (8, 0xffff, 0x0225, 0x1000, 0, (0x0264, 0x3000, 0x0000)),
        (12, 0x0000, 0x0225, 0x1000, 0, (0x0000, 0x0008, 0x0008)),
    ] {
        m.seed(&[("ECFG", 0x2000), ("FFTV", 0x0264), ("CETBL", table), ("CECFG1", cecfg1),
                 ("CECFG2", cecfg2), ("EST", est)]);
        m.start(pc).unwrap();
        assert_eq!(m.step(), 3 * (pc + 2));
        assert_eq!((m.reg("CETBL"), m.reg("EST")), (expected.0, expected.1));
        assert_eq!(m.step(), 3 * (pc + 4));
        assert_eq!(m.reg("EST"), expected.2);
    }
}

#[test]
fn wide_data_extract_and_shift_use_incoming_feedback() {
    let packets = [(0xdcc85e, 0xf6a224), (NOP, NOP), (0xa602ca, 0xf3cef2), (NOP, NOP), (NOP, NOP)];
    let issues = [(0, 5, 6), (2, 1, 6), (4, 5, 6), (6, 1, 6), (8, 1, 0)];
    let mut m = Machine::new(0, &packets, &issues, None);
    for (word, count, want) in [(0, 0, (0, 0, 0x1008, 0x2008)),
                                (0x8000, 31, (0x8000, 0x7fff, 0x5000, 0x5000))] {
        m.seed(&[("R1", 0x4040), ("R2", 0x8040), ("R4", 0x48), ("PACFG", 0),
                 ("EST", 0), ("ECFG", 0x10), ("FFTV", 1), ("CETBL", 0x8001),
                 ("CECFG1", 0x888), ("CECFG2", 0x888)]);
        m.seed_data(0x4040, 0xaaaa);
        m.seed_data(0x4041, 0xbbbb);
        m.seed_data(0x8040, word);
        m.seed_data(0x48, count);
        m.seed_data(0x49, 0);
        m.start(0).unwrap();
        assert_eq!(m.step(), 6);
        assert_eq!((m.reg("R1"), m.reg("R2"), m.reg("R4")), (0x4042, 0x8041, 0x46));
        assert_eq!((m.data(0x4040), m.data(0x4041), m.reg("EST")), (want.0, want.1, want.2));
        assert_eq!(m.step(), 12);
        assert_eq!(m.reg("EST"), want.3);
    }
    m.seed(&[("R6", 0x8048), ("ECFG", 0), ("EST", 0), ("CETBL", 0x8001),
             ("CECFG1", 0x222), ("CECFG2", 0x222)]);
    for (at, value) in [(0x2ca, 0xe1b2), (0x2cb, 0x8003), (0x8012, 1)] {
        m.seed_data(at, value);
    }
    m.start(4).unwrap();
    assert_eq!(m.step(), 18);
    assert_eq!(m.reg("EST"), 0x1001);
    assert_eq!(m.step(), 24);
    assert_eq!(m.reg("EST"), 0x2001);
}
