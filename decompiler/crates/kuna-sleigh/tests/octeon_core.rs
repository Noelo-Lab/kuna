//! Original encoding and executable p-code checks for Octeon integer extensions.
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
        "synthetic-octeon-core"
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
    fn new(spec: &str, branch: u32, slot: u32) -> Self {
        let sla = read_sla(spec);
        let bytes: Vec<_> = [branch, slot, 0, 0, 0]
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

fn read_sla(spec: &str) -> Vec<u8> {
    let directory = std::env::var_os("OCTEON_TEST_SLA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../../specs/Ghidra/Processors/MIPS/data/languages")
        });
    std::fs::read(directory.join(format!("{spec}.sla"))).unwrap()
}

const SPECS: [&str; 4] = ["octeonbe", "octeonplusbe", "octeon2be", "octeon3be"];

#[test]
fn unsupported_atomics_never_decode_as_generic_save_restore() {
    use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
    use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit};
    #[derive(Default)]
    struct Output {
        mnemonic: String,
        ops: usize,
    }
    impl AssemblyEmit for Output {
        fn dump(&mut self, _: &Address, mnemonic: &str, _: &str) {
            self.mnemonic = mnemonic.into();
        }
    }
    impl PcodeEmit for Output {
        fn dump(&mut self, _: &Address, _: OpCode, _: Option<&VarnodeData>, _: &[VarnodeData]) {
            self.ops += 1;
        }
    }
    // GNU as -64 -march=octeon2: lai/laid/lad/ladd/las/lasd/lac/lacd,
    // laa/laad/law/lawd $8,($4)[,$5], followed by saa/saad $5,($4).
    let words = [
        0x7080409fu32,
        0x708040df,
        0x7080419f,
        0x708041df,
        0x7080429f,
        0x708042df,
        0x7080439f,
        0x708043df,
        0x7085449f,
        0x708544df,
        0x7085459f,
        0x708545df,
        0x70850018,
        0x70850019,
        0x7080609f,
        0x7085649f,
    ];
    for spec in SPECS {
        let bytes = words.iter().flat_map(|w| w.to_be_bytes()).collect();
        let mut sleigh = Sleigh::new(Box::new(Bytes(bytes)), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&read_sla(spec)).unwrap();
        let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
        for (i, word) in words.iter().enumerate() {
            let addr = Address::new(Rc::clone(&ram), 0x1000 + i as u64 * 4);
            let mut out = Output::default();
            assert!(
                sleigh.print_assembly(&mut out, &addr).is_err(),
                "{spec} {word:08x}: {}",
                out.mnemonic
            );
            assert!(
                sleigh.one_instruction(&mut out, &addr).is_err(),
                "{spec} {word:08x} lifted"
            );
            assert_eq!(
                out.ops, 0,
                "a rejected atomic must emit no stack side effects"
            );
        }
    }
    for (word, expected) in [(0x7080409fu32, "restore"), (0x7080609f, "save")] {
        let mut sleigh = Sleigh::new(
            Box::new(Bytes(word.to_be_bytes().to_vec())),
            Box::new(ContextInternal::new()),
        );
        sleigh.initialize_from_sla(&read_sla("mips64be")).unwrap();
        let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
        let mut out = Output::default();
        sleigh
            .print_assembly(&mut out, &Address::new(ram, 0x1000))
            .unwrap();
        assert_eq!(
            out.mnemonic, expected,
            "generic MIPS must retain its constructors"
        );
    }
}

#[test]
fn generic_mips_and_generation_boundaries_remain_distinct() {
    use kuna_sleigh::translate::AssemblyEmit;
    #[derive(Default)]
    struct Mnemonic(String);
    impl AssemblyEmit for Mnemonic {
        fn dump(&mut self, _: &Address, mnemonic: &str, _: &str) {
            self.0 = mnemonic.into();
        }
    }
    for (opcode, generic, octeon) in [
        (0x32, "lwc2", "bbit0"),
        (0x36, "ldc2", "bbit032"),
        (0x3a, "swc2", "bbit1"),
        (0x3e, "sdc2", "bbit132"),
    ] {
        for (spec, expected) in [("mips64be", generic), ("octeonbe", octeon)] {
            let word: u32 = (opcode << 26) | (4 << 21) | 4;
            let mut sleigh = Sleigh::new(
                Box::new(Bytes(word.to_be_bytes().to_vec())),
                Box::new(ContextInternal::new()),
            );
            sleigh.initialize_from_sla(&read_sla(spec)).unwrap();
            let ram = Rc::clone(sleigh.base().manager().get_space_by_name("ram").unwrap());
            let mut out = Mnemonic::default();
            sleigh
                .print_assembly(&mut out, &Address::new(ram, 0x1000))
                .unwrap();
            assert_eq!(out.0, expected);
        }
    }
    for spec in ["octeonbe", "octeonplusbe"] {
        let mut m = Machine::new(spec, 0x7c85_400a, 0);
        let address = Address::new(Rc::clone(&m.ram), 0x1000);
        let result = m
            .emulator
            .set_execute_address(&address)
            .and_then(|_| m.emulator.execute_instruction());
        assert!(
            result.is_err(),
            "{spec} must not accept an Octeon II indexed load"
        );
    }
}

#[test]
fn arithmetic_and_zero_sources_execute() {
    let values = [
        0,
        1,
        255,
        256,
        u64::MAX,
        0x8000_0000_0000_0000,
        0x1234_5678_9abc_def0,
    ];
    for spec in SPECS {
        for function in [3, 0x28, 0x2a, 0x2b] {
            for a in values {
                for b in values {
                    for zero in [false, true] {
                        let b = if zero { 0 } else { b };
                        let word = 0x7000_0000
                            | (4 << 21)
                            | (if zero { 0 } else { 5 << 16 })
                            | (8 << 11)
                            | function;
                        let mut m = Machine::new(spec, word, 0);
                        m.step(&[("a0", a), ("a1", b), ("t0", 0x55)]);
                        let expected = match function {
                            3 => a.wrapping_mul(b),
                            0x28 => (a.wrapping_add(b)) & 255,
                            0x2a => u64::from(a == b),
                            _ => u64::from(a != b),
                        };
                        assert_eq!(
                            m.read("t0"),
                            expected,
                            "{spec} f={function:x} a={a:x} b={b:x}"
                        );
                    }
                }
            }
        }
        for a in values {
            for (f, expected) in [(0x2c, (a as u32).count_ones()), (0x2d, a.count_ones())] {
                let mut m = Machine::new(spec, 0x7080_4000 | f, 0);
                m.step(&[("a0", a)]);
                assert_eq!(m.read("t0"), u64::from(expected));
            }
        }
        for imm in [-512i64, -1, 0, 511] {
            for a in [imm as u64, (imm as u64).wrapping_add(1)] {
                for f in [0x2e, 0x2f] {
                    let word = 0x7088_0000 | (((imm as u32) & 1023) << 6) | f;
                    let mut m = Machine::new(spec, word, 0);
                    m.step(&[("a0", a)]);
                    assert_eq!(m.read("t0"), u64::from((a == imm as u64) == (f == 0x2e)));
                }
            }
        }
    }
}

#[test]
fn bit_branches_sample_before_delay_slot() {
    for spec in SPECS {
        for (opcode, high, sense) in [
            (0x32, 0, false),
            (0x36, 32, false),
            (0x3a, 0, true),
            (0x3e, 32, true),
        ] {
            for bit in [0, 3, 31] {
                for set in [false, true] {
                    let value = if set { 1u64 << (bit + high) } else { 0 };
                    let word = (opcode << 26) | (4 << 21) | (bit << 16) | 4;
                    // The slot clears the tested register. Branch outcome must
                    // still use the incoming bit, and slot effects must survive.
                    let mut m = Machine::new(spec, word, 0x3404_0000);
                    assert_eq!(
                        m.step(&[("a0", value)]),
                        if set == sense { 0x1014 } else { 0x1008 }
                    );
                    assert_eq!(m.read("a0"), 0);
                }
            }
        }
    }
}

#[test]
fn bitfields_preserve_width_and_sign() {
    // GNU as: cins $8,$4,8,7 and exts $8,$4,8,7. Unequal fields
    // independently catch swapping the position and length token slices.
    for (word, input, expected) in [
        (0x7088_3a32, u64::MAX, 0xff00),
        (0x7088_3a3a, 0x8000, (-128i64) as u64),
    ] {
        let mut m = Machine::new("octeonbe", word, 0);
        m.step(&[("a0", input)]);
        assert_eq!(m.read("t0"), expected);
    }
    for spec in SPECS {
        for p in [0u32, 8, 31] {
            for len in [0u32, 7, 31] {
                for high in [0u32, 32] {
                    if p + high + len >= 64 {
                        continue;
                    }
                    for a in [0u64, u64::MAX, 0x8000, 0x8000_0000_0000] {
                        for extract in [false, true] {
                            let f = (if extract { 0x3a } else { 0x32 }) + u32::from(high != 0);
                            let mut m =
                                Machine::new(spec, 0x7088_0000 | (len << 11) | (p << 6) | f, 0);
                            m.step(&[("a0", a), ("t0", u64::MAX)]);
                            let modulus = 1u128 << (len + 1);
                            let expected = if extract {
                                let field = ((a as u128) >> (p + high)) % modulus;
                                if field >= modulus / 2 {
                                    (field as i128 - modulus as i128) as u64
                                } else {
                                    field as u64
                                }
                            } else {
                                (((a as u128) % modulus) << (p + high)) as u64
                            };
                            assert_eq!(
                                m.read("t0"),
                                expected,
                                "{spec} p={p} len={len} high={high} extract={extract}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn indexed_loads_have_correct_width_and_extension() {
    for spec in ["octeon2be", "octeon3be"] {
        for (sa, offset, expected) in [
            (0x16, 0, -128i64 as u64),
            (6, 0, 128),
            (4, 2, -32768i64 as u64),
            (0x14, 2, 32768),
            (0, 4, u64::MAX),
            (0x10, 4, 0xffff_ffff),
            (8, 0, 0x80ff_8000_ffff_ffff),
        ] {
            let mut m = Machine::new(spec, 0x7c85_400a | (sa << 6), 0);
            m.state
                .borrow_mut()
                .set_value(&m.ram, 0x2000, 8, 0x80ff_8000_ffff_ffff)
                .unwrap();
            m.step(&[("a0", 0x2000), ("a1", offset)]);
            assert_eq!(m.read("t0"), expected, "{spec} sa={sa}");
        }
    }
}
