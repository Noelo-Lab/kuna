//! nanoMIPS decode and executable p-code checks.
//!
//! The workload images under `fixtures/nanomips` are a `nanomips-elf-gcc` build of
//! `workload.c` (see the README there). Each case maps a seed to a value with
//! fixed-width arithmetic only, so the expected values come from a native build
//! of the same source, not from this lifter.
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use kuna_base::{address::Address, error::KunaResult, space::AddrSpace};
use kuna_sleigh::{
    emulate::{BreakTableCallBack, Emulate, EmulatePcodeCache},
    globalcontext::ContextInternal,
    loadimage::LoadImage,
    memstate::{MemoryHashOverlay, MemoryState},
    sleigh::Sleigh,
    translate::{AssemblyEmit, Translate},
};

const BASE: u64 = 0x8000_1000;
const GP: u64 = 0x8000_8000;
const STACK: u64 = 0x7fff_0000;
const RETURN: u64 = 0x7ff0_0000;

struct Bytes(Vec<u8>, u64);
impl LoadImage for Bytes {
    fn get_file_name(&self) -> &str {
        "nanomips-workload"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], addr: &Address) -> KunaResult<()> {
        for (i, byte) in out.iter_mut().enumerate() {
            *byte = addr
                .get_offset()
                .checked_sub(self.1)
                .and_then(|offset| self.0.get(offset as usize + i))
                .copied()
                .unwrap_or(0);
        }
        Ok(())
    }
}

fn read_sla(spec: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs/Ghidra/Processors/nanoMIPS/data/languages")
        .join(format!("{spec}.sla"));
    std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn sleigh(spec: &str, image: Vec<u8>, base: u64) -> Sleigh {
    let mut sleigh = Sleigh::new(Box::new(Bytes(image, base)), Box::new(ContextInternal::new()));
    sleigh.initialize_from_sla(&read_sla(spec)).unwrap();
    sleigh
}

struct Machine {
    emulator: EmulatePcodeCache,
    state: Rc<RefCell<MemoryState>>,
    ram: Rc<AddrSpace>,
}

impl Machine {
    fn new(spec: &str, image: &[u8]) -> Self {
        let sleigh = sleigh(spec, image.to_vec(), BASE);
        let manager = sleigh.base().manager_rc();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let trans: Rc<dyn Translate> = Rc::new(sleigh);
        let mut state = MemoryState::new(Rc::clone(&trans));
        for name in ["ram", "register", "unique"] {
            let space = Rc::clone(manager.get_space_by_name(name).unwrap());
            state.set_memory_bank(Rc::new(RefCell::new(MemoryHashOverlay::new(
                space, 8, 4096, 4096, None,
            ))));
        }
        state.set_chunk(image, &ram, BASE).unwrap();
        let state = Rc::new(RefCell::new(state));
        let breaks = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
        let emulator = EmulatePcodeCache::new(trans, manager, Rc::clone(&state), breaks);
        Machine { emulator, state, ram }
    }

    fn set(&self, name: &str, value: u64) {
        self.state.borrow_mut().set_value_by_name(name, value).unwrap();
    }

    fn get(&self, name: &str) -> u64 {
        self.state.borrow().get_value_by_name(name).unwrap()
    }

    /// Calls `entry(argument)` under the p32 convention and returns `a0`.
    fn call(&mut self, entry: u64, argument: u32) -> u32 {
        self.set("a0", u64::from(argument));
        self.set("sp", STACK);
        self.set("gp", GP);
        self.set("ra", RETURN);
        self.emulator
            .set_execute_address(&Address::new(Rc::clone(&self.ram), entry))
            .unwrap();
        for _ in 0..20_000_000u32 {
            if self.emulator.get_execute_address().get_offset() == RETURN {
                assert_eq!(self.get("sp"), STACK, "the frame must be closed on return");
                assert_eq!(self.get("gp"), GP, "gp is preserved across the call");
                return self.get("a0") as u32;
            }
            self.emulator.execute_instruction().unwrap();
        }
        panic!("function at {entry:#x} did not return");
    }
}

const SEEDS: [u32; 8] = [0, 1, 0x12345678, 0xdeadbeef, 0x80000000, 0x7fffffff, 0xffffffff, 42];

/// `table[case](seed)` from a native build of `fixtures/nanomips/workload.c`.
const EXPECTED: [[u32; 8]; 8] = [
    [0x8775a3bc, 0x464503c6, 0x305d74cf, 0x36977c13, 0x32c948c9, 0xed3ed944, 0xffad05d7, 0xfab1524a],
    [0x906bbc67, 0x1ae001ac, 0x2af0f90e, 0xb4b52418, 0x98a8f3af, 0xf5b9027b, 0x43971892, 0x5186be23],
    [0x7ea21be5, 0x2415b624, 0x9174be7b, 0x095ebe9e, 0x2f2df4c2, 0x1ae9ab5d, 0x2e0a6438, 0xf039e492],
    [0x9f2b7135, 0x9123304f, 0x7069c64b, 0xb5f7644d, 0xec884424, 0x98ee19ff, 0xafff152c, 0x83299b54],
    [0x7bb0d9c3, 0xecf61c01, 0xd9836b0f, 0x17434c95, 0x5b1e2ad8, 0xb92d7e46, 0xe58a9f18, 0x85edfbf1],
    [0x4a339184, 0x13f32f09, 0xcd459db3, 0x0861e17b, 0xdabdad78, 0x330dc632, 0xa97b8ffb, 0xc26c0a64],
    [0xfef0b6f3, 0x4c5e264c, 0xe76f4a7c, 0x7128d554, 0xace1ad53, 0x21ea7d3d, 0xafb78c9a, 0xc2e93966],
    [0x21c7bb3b, 0x80b1490e, 0xc8ee2506, 0x6c54f9b0, 0x6837b745, 0x9c8e560b, 0x14a32870, 0xf63a70c4],
];

#[test]
fn compiled_workload_computes_the_native_results() {
    for (image, spec, little) in [
        ("workload_O2EL.bin", "nanomips32le", true),
        ("workload_OsEL.bin", "nanomips32le", true),
        ("workload_O2EB.bin", "nanomips32be", false),
        ("workload_OsEB.bin", "nanomips32be", false),
    ] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nanomips").join(image);
        let bytes = std::fs::read(&path).unwrap();
        let mut machine = Machine::new(spec, &bytes);
        for (case, expected) in EXPECTED.iter().enumerate() {
            let word: [u8; 4] = bytes[case * 4..case * 4 + 4].try_into().unwrap();
            let entry = if little { u32::from_le_bytes(word) } else { u32::from_be_bytes(word) };
            for (seed, want) in SEEDS.iter().zip(expected) {
                let got = machine.call(u64::from(entry), *seed);
                assert_eq!(got, *want, "{image} case {case} seed {seed:#x}");
            }
        }
    }
}

const WORDS: [u32; 4] = [0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00];

fn fold(h: &mut u32, value: u32) {
    *h = h.rotate_left(5) ^ value;
}

/// The value `fixtures/nanomips/extras.S` computes, restated from the
/// instruction-set manual's definitions.
fn extras_model(x: u32, little: bool) -> u32 {
    let mut h = 0u32;
    let a1 = 0x0f0f0f0fu32;
    let mask = ((1u32 << 9) - 1) << 5;
    fold(&mut h, (a1 & !mask) | ((x << 5) & mask));
    fold(&mut h, (x >> 7) & 0x1fff);
    fold(&mut h, (a1 << 21) | (x >> 11));
    fold(&mut h, x.reverse_bits());
    fold(&mut h, x.reverse_bits().rotate_left(16));
    fold(&mut h, x.reverse_bits().swap_bytes());
    fold(&mut h, x.swap_bytes());
    fold(&mut h, x.swap_bytes().rotate_left(16));
    fold(&mut h, x.rotate_right(9));
    fold(&mut h, u32::from((x as i32).checked_add(a1 as i32).is_none()));
    fold(&mut h, u32::from(x == 42));
    fold(&mut h, x.leading_ones());
    fold(&mut h, x);
    fold(&mut h, 0x89abcdefu32.wrapping_add(0x76543211));
    fold(&mut h, WORDS[0]);
    fold(&mut h, x);
    fold(&mut h, WORDS[1]);
    fold(&mut h, WORDS[1]);
    fold(&mut h, WORDS[2]);
    fold(&mut h, WORDS[3]);
    let image: Vec<u8> = WORDS
        .iter()
        .flat_map(|word| if little { word.to_le_bytes() } else { word.to_be_bytes() })
        .collect();
    for offset in [2, 6] {
        let bytes: [u8; 4] = image[offset..offset + 4].try_into().unwrap();
        fold(&mut h, if little { u32::from_le_bytes(bytes) } else { u32::from_be_bytes(bytes) });
    }
    let stored = h;
    fold(&mut h, x);
    fold(&mut h, stored);
    fold(&mut h, 0);
    fold(&mut h, x);
    fold(&mut h, x);
    let moved = h;
    fold(&mut h, x);
    fold(&mut h, moved);
    h
}

#[test]
fn rarely_compiled_instructions_compute_the_manual_results() {
    for (image, spec, little) in [("extras_EL.bin", "nanomips32le", true), ("extras_EB.bin", "nanomips32be", false)] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nanomips").join(image);
        let bytes = std::fs::read(&path).unwrap();
        let word: [u8; 4] = bytes[..4].try_into().unwrap();
        let entry = if little { u32::from_le_bytes(word) } else { u32::from_be_bytes(word) };
        for seed in SEEDS {
            let mut machine = Machine::new(spec, &bytes);
            assert_eq!(machine.call(u64::from(entry), seed), extras_model(seed, little), "{image} seed {seed:#x}");
        }
    }
}

struct Text(String);
impl AssemblyEmit for Text {
    fn dump(&mut self, _: &Address, mnemonic: &str, body: &str) {
        self.0 = format!("{mnemonic} {body}").trim_end().to_string();
    }
}

/// Address, 16-bit units in instruction order, and listing text. The units are
/// GNU assembler output, except the MCU and virtualization rows at the end,
/// which follow the instruction-set manual.
#[test]
fn instruction_units_decode_in_both_byte_orders() {
    let start = GOLDEN[0].0;
    for (spec, little) in [("nanomips32le", true), ("nanomips32be", false)] {
        let image: Vec<u8> = GOLDEN
            .iter()
            .flat_map(|(_, units, _)| units.iter())
            .flat_map(|unit| if little { unit.to_le_bytes() } else { unit.to_be_bytes() })
            .collect();
        let sleigh = sleigh(spec, image, start);
        let ram = Rc::clone(sleigh.base().manager_rc().get_space_by_name("ram").unwrap());
        let mut next = start;
        for (address, units, text) in GOLDEN {
            assert_eq!(*address, next, "rows must be contiguous");
            let mut got = Text(String::new());
            let length = sleigh
                .print_assembly(&mut got, &Address::new(Rc::clone(&ram), *address))
                .unwrap();
            assert_eq!(length as usize, units.len() * 2, "{spec} {address:#x}");
            assert_eq!(&got.0, text, "{spec} {address:#x}");
            next += length as u64;
        }
    }
}

const GOLDEN: &[(u64, &[u16], &str)] = &[
    (0x80200094, &[0x0000, 0x0005], "sigrie 0x5"),
    (0x80200098, &[0x0008, 0x0009], "syscall 0x9"),
    (0x8020009c, &[0x1013], "break 0x3"),
    (0x8020009e, &[0x101c], "sdbbp 0x4"),
    (0x802000a0, &[0x1009], "syscall 0x1"),
    (0x802000a2, &[0x1012], "break 0x2"),
    (0x802000a4, &[0x101b], "sdbbp 0x3"),
    (0x802000a6, &[0x20a4, 0x0000], "teq a0, a1"),
    (0x802000aa, &[0x20e6, 0x0400], "tne a2, a3"),
    (0x802000ae, &[0x218d, 0x0048], "seh t0, t1"),
    (0x802000b2, &[0x2211, 0x0008], "seb s0, s1"),
    (0x802000b6, &[0x20c5, 0x20d0], "rotrv a0, a1, a2"),
    (0x802000ba, &[0x20c5, 0x2098], "mulu a0, a1, a2"),
    (0x802000be, &[0x20c5, 0x2058], "muh a0, a1, a2"),
    (0x802000c2, &[0x20c5, 0x2110], "add a0, a1, a2"),
    (0x802000c6, &[0x20c5, 0x2190], "sub a0, a1, a2"),
    (0x802000ca, &[0x20c5, 0x23d0], "sov a0, a1, a2"),
    (0x802000ce, &[0x2080, 0x0390], "dvp a0"),
    (0x802000d2, &[0x20a0, 0x0790], "evp a1"),
    (0x802000d6, &[0x208c, 0x0030], "mfc0 a0, 0xc, 0x0"),
    (0x802000da, &[0x208c, 0x0870], "mtc0 a0, 0xc, 0x1"),
    (0x802000de, &[0x2085, 0x1038], "mfhc0 a0, 0x5, 0x2"),
    (0x802000e2, &[0x2085, 0x1078], "mthc0 a0, 0x5, 0x2"),
    (0x802000e6, &[0x209d, 0x01c0], "rdhwr a0, 0x1d, 0x0"),
    (0x802000ea, &[0x20c5, 0x2207], "lhx a0, a1(a2)"),
    (0x802000ee, &[0x2062, 0x8407], "lwx s0, t4(t5)"),
    (0x802000f2, &[0x20c5, 0x2087], "sbx a0, a1(a2)"),
    (0x802000f6, &[0x2085, 0x4b3f], "clo a0, a1"),
    (0x802000fa, &[0x2085, 0x5b3f], "clz a0, a1"),
    (0x802000fe, &[0x2080, 0x477f], "di a0"),
    (0x80200102, &[0x20a0, 0x577f], "ei a1"),
    (0x80200106, &[0x2000, 0x477f], "di zero"),
    (0x8020010a, &[0x2000, 0x577f], "ei zero"),
    (0x8020010e, &[0x2000, 0xf37f], "eret"),
    (0x80200112, &[0x2001, 0xf37f], "eretnc"),
    (0x80200116, &[0x2000, 0xe37f], "deret"),
    (0x8020011a, &[0x2000, 0xc37f], "wait"),
    (0x8020011e, &[0x2085, 0xe17f], "rdpgpr a0, a1"),
    (0x80200122, &[0x2085, 0xf17f], "wrpgpr a0, a1"),
    (0x80200126, &[0x2000, 0x037f], "tlbp"),
    (0x8020012a, &[0x2000, 0x137f], "tlbr"),
    (0x8020012e, &[0x2000, 0x237f], "tlbwi"),
    (0x80200132, &[0x2000, 0x337f], "tlbwr"),
    (0x80200136, &[0x2000, 0x077f], "tlbinv"),
    (0x8020013a, &[0x2000, 0x177f], "tlbinvf"),
    (0x8020013e, &[0x4be4, 0x1000], "jalrc.hb ra, a0"),
    (0x80200142, &[0x4984, 0x0000], "jalrc t0, a0"),
    (0x80200146, &[0xd890], "jalrc a0"),
    (0x80200148, &[0xd8a0], "jrc a1"),
    (0x8020014a, &[0x4be4, 0x8000], "balrsc ra, a0"),
    (0x8020014e, &[0x4805, 0x8000], "brsc a1"),
    (0x80200152, &[0x8085, 0x1abc], "xori a0, a1, 0xabc"),
    (0x80200156, &[0x8085, 0x0fff], "ori a0, a1, 0xfff"),
    (0x8020015a, &[0x8085, 0x504d], "sltiu a0, a1, 0x4d"),
    (0x8020015e, &[0x8085, 0x404d], "slti a0, a1, 0x4d"),
    (0x80200162, &[0x8085, 0x604d], "seqi a0, a1, 0x4d"),
    (0x80200166, &[0x8085, 0xc0cd], "rotr a0, a1, 0xd"),
    (0x8020016a, &[0x8085, 0xc08d], "sra a0, a1, 0xd"),
    (0x8020016e, &[0x82f9, 0xc01f], "sll s7, t9, 0x1f"),
    (0x80200172, &[0x8000, 0xc003], "ehb"),
    (0x80200176, &[0x8000, 0xc005], "pause"),
    (0x8020017a, &[0x8000, 0xc006], "sync 0x0"),
    (0x8020017e, &[0x8005, 0xc006], "sync 0x5"),
    (0x80200182, &[0x9008], "nop"),
    (0x80200184, &[0x8000, 0xc000], "nop"),
    (0x80200188, &[0x8085, 0xd1c5], "rotx a0, a1, 0x5, 0x6, 0x1"),
    (0x8020018c, &[0x8085, 0xd185], "rotx a0, a1, 0x5, 0x6, 0x0"),
    (0x80200190, &[0x8085, 0xd608], "byterevh a0, a1"),
    (0x80200194, &[0x8085, 0xd218], "byterevw a0, a1"),
    (0x80200198, &[0x8085, 0xd01f], "bitrevw a0, a1"),
    (0x8020019c, &[0x8085, 0xd40f], "bitrevh a0, a1"),
    (0x802001a0, &[0x8085, 0xd247], "bitrevb a0, a1"),
    (0x802001a4, &[0x8085, 0xd247], "bitrevb a0, a1"),
    (0x802001a8, &[0x8085, 0xe2c3], "ins a0, a1, 0x3, 0x9"),
    (0x802001ac, &[0x8085, 0xf203], "ext a0, a1, 0x3, 0x9"),
    (0x802001b0, &[0x83c7, 0x3044], "save 0x40,fp,ra,s0,s1,s2,s3,gp"),
    (0x802001b4, &[0x83c7, 0x3046], "restore 0x40,fp,ra,s0,s1,s2,s3,gp"),
    (0x802001b8, &[0x83c7, 0x3047], "restore.jrc 0x40,fp,ra,s0,s1,s2,s3,gp"),
    (0x802001bc, &[0x8084, 0x3ff8], "save 0xff8,a0,a1,a2,a3"),
    (0x802001c0, &[0x1c80], "save 0x80"),
    (0x802001c2, &[0x8381, 0x3024], "save 0x20,gp"),
    (0x802001c6, &[0x8305, 0x3023], "restore.jrc 0x20,t8,t9,k0,k1,gp"),
    (0x802001ca, &[0x83ee, 0x3100], "save 0x100,ra,s0,s1,s2,s3,s4,s5,s6,s7,t8,t9,k0,k1,gp"),
    (0x802001ce, &[0x1c1a], "save 0x10,fp,ra,s0,s1,s2,s3,s4,s5,s6,s7"),
    (0x802001d0, &[0x1ff1], "restore.jrc 0xf0,ra"),
    (0x802001d2, &[0x8485, 0x07ff], "lb a0, 0x7ff(a1)"),
    (0x802001d6, &[0x8485, 0x1fff], "sb a0, 0xfff(a1)"),
    (0x802001da, &[0x8485, 0x212c], "lbu a0, 0x12c(a1)"),
    (0x802001de, &[0x8485, 0x412c], "lh a0, 0x12c(a1)"),
    (0x802001e2, &[0x8485, 0x512c], "sh a0, 0x12c(a1)"),
    (0x802001e6, &[0x8485, 0x612c], "lhu a0, 0x12c(a1)"),
    (0x802001ea, &[0x848c, 0x812c], "lw a0, 0x12c(t0)"),
    (0x802001ee, &[0x848c, 0x912c], "sw a0, 0x12c(t0)"),
    (0x802001f2, &[0x8465, 0x312c], "pref 0x3, 0x12c(a1)"),
    (0x802001f6, &[0xa485, 0x80fd], "lb a0, -0x3(a1)"),
    (0x802001fa, &[0xa485, 0x8800], "sb a0, -0x100(a1)"),
    (0x802001fe, &[0xa485, 0x90ff], "lbu a0, -0x1(a1)"),
    (0x80200202, &[0xa485, 0xa0fe], "lh a0, -0x2(a1)"),
    (0x80200206, &[0xa485, 0xa8fe], "sh a0, -0x2(a1)"),
    (0x8020020a, &[0xa485, 0xb0fe], "lhu a0, -0x2(a1)"),
    (0x8020020e, &[0xa485, 0xc0fc], "lw a0, -0x4(a1)"),
    (0x80200212, &[0xa485, 0xc8fc], "sw a0, -0x4(a1)"),
    (0x80200216, &[0xa465, 0x98fc], "pref 0x3, -0x4(a1)"),
    (0x8020021a, &[0xa7e5, 0x98fc], "synci -0x4(a1)"),
    (0x8020021e, &[0x87e5, 0x3008], "synci 0x8(a1)"),
    (0x80200222, &[0xa485, 0xa1fd], "ualh a0, -0x3(a1)"),
    (0x80200226, &[0xa485, 0x2903], "uash a0, 0x3(a1)"),
    (0x8020022a, &[0xa4a5, 0xb9fc], "cache 0x5, -0x4(a1)"),
    (0x8020022e, &[0xa485, 0xd1fc], "ll a0, -0x4(a1)"),
    (0x80200232, &[0xa485, 0xd9f8], "sc a0, -0x8(a1)"),
    (0x80200236, &[0xa486, 0x5129], "llwp a0, a1, (a2)"),
    (0x8020023a, &[0xa486, 0x5929], "scwp a0, a1, (a2)"),
    (0x8020023e, &[0xa485, 0x82fd], "lbe a0, -0x3(a1)"),
    (0x80200242, &[0xa485, 0x8afd], "sbe a0, -0x3(a1)"),
    (0x80200246, &[0xa485, 0x92fd], "lbue a0, -0x3(a1)"),
    (0x8020024a, &[0xa485, 0xa2fe], "lhe a0, -0x2(a1)"),
    (0x8020024e, &[0xa485, 0xaafe], "she a0, -0x2(a1)"),
    (0x80200252, &[0xa485, 0xb2fe], "lhue a0, -0x2(a1)"),
    (0x80200256, &[0xa485, 0xc2fc], "lwe a0, -0x4(a1)"),
    (0x8020025a, &[0xa485, 0xcafc], "swe a0, -0x4(a1)"),
    (0x8020025e, &[0xa465, 0x9afc], "prefe 0x3, -0x4(a1)"),
    (0x80200262, &[0xa7e5, 0x9afc], "syncie -0x4(a1)"),
    (0x80200266, &[0xa4a5, 0xbafc], "cachee 0x5, -0x4(a1)"),
    (0x8020026a, &[0xa485, 0xd2fc], "lle a0, -0x4(a1)"),
    (0x8020026e, &[0xa485, 0xdafc], "sce a0, -0x4(a1)"),
    (0x80200272, &[0xa61d, 0xb4f8], "lwm s0, -0x8(sp), 0x3"),
    (0x80200276, &[0xa61d, 0x0c08], "swm s0, 0x8(sp), 8"),
    (0x8020027a, &[0xa504, 0x5504], "ualwm a4, 0x4(a0), 0x5"),
    (0x8020027e, &[0xa504, 0x7d04], "uaswm a4, 0x4(a0), 0x7"),
    (0x80200282, &[0xa404, 0x7c04], "swm zero, 0x4(a0), 0x7"),
    (0x80200286, &[0xa6c4, 0x0404], "lwm s6, 0x4(a0), 8"),
    (0x8020028a, &[0x6080, 0x5678, 0x1234], "li a0, 0x12345678"),
    (0x80200290, &[0x6081, 0x5678, 0x1234], "addiu a0, a0, 0x12345678"),
    (0x80200296, &[0x6082, 0x5678, 0x1234], "addiu a0, gp, 0x12345678"),
    (0x8020029c, &[0x448c, 0x0190], "addiu a0, gp, 0x190"),
    (0x802002a0, &[0x448c, 0x0191], "addiu a0, gp, 0x191"),
    (0x802002a4, &[0x5664], "lw a0, 0x190(gp)"),
    (0x802002a6, &[0xd664], "sw a0, 0x190(gp)"),
    (0x802002a8, &[0x4180, 0x0192], "lw t0, 0x190(gp)"),
    (0x802002ac, &[0x419e, 0x8483], "sw t0, 0x1e8480(gp)"),
    (0x802002b0, &[0x4480, 0x0191], "lb a0, 0x191(gp)"),
    (0x802002b4, &[0x4484, 0x0191], "sb a0, 0x191(gp)"),
    (0x802002b8, &[0x4488, 0x0191], "lbu a0, 0x191(gp)"),
    (0x802002bc, &[0x4490, 0x0192], "lh a0, 0x192(gp)"),
    (0x802002c0, &[0x4490, 0x0193], "lhu a0, 0x192(gp)"),
    (0x802002c4, &[0x4494, 0x0192], "sh a0, 0x192(gp)"),
    (0x802002c8, &[0x7590], "lw a0, 0x4(s0)"),
    (0x802002ca, &[0xf498], "sw a0, 0x8(s0)"),
    (0x802002cc, &[0xf578], "sw zero, 0xc(s0)"),
    (0x802002ce, &[0x7795], "lw s4, 0x4(s5)"),
    (0x802002d0, &[0x7260], "addiu a0, sp, 0x80"),
    (0x802002d2, &[0x3590], "lw t0, 0x40(sp)"),
    (0x802002d4, &[0xb590], "sw t0, 0x40(sp)"),
    (0x802002d6, &[0xe094, 0x5244], "lui a0, 0x12345000"),
    (0x802002da, &[0xe09f, 0xfffd], "lui a0, 0xfffff000"),
    (0x802002de, &[0xd27f], "li a0, -1"),
    (0x802002e0, &[0xd27e], "li a0, 0x7e"),
    (0x802002e2, &[0x0080, 0x007f], "li a0, 0x7f"),
    (0x802002e6, &[0x0080, 0xffff], "li a0, 0xffff"),
    (0x802002ea, &[0xf25c], "andi a0, a1, 0xff"),
    (0x802002ec, &[0xf25d], "andi a0, a1, 0xffff"),
    (0x802002ee, &[0xf25b], "andi a0, a1, 0xb"),
    (0x802002f0, &[0xf25e], "andi a0, a1, 0xe"),
    (0x802002f2, &[0x3250], "sll a0, a1, 8"),
    (0x802002f4, &[0x3259], "srl a0, a1, 0x1"),
    (0x802002f6, &[0x5e53], "lb a0, 0x3(a1)"),
    (0x802002f8, &[0x5c57], "sb zero, 0x3(a1)"),
    (0x802002fa, &[0x5e5b], "lbu a0, 0x3(a1)"),
    (0x802002fc, &[0x7e56], "lh a0, 0x6(a1)"),
    (0x802002fe, &[0x7c57], "sh zero, 0x6(a1)"),
    (0x80200300, &[0x7e5e], "lhu a0, 0x6(a1)"),
    (0x80200302, &[0x945f], "sw zero, 0x3c(a1)"),
    (0x80200304, &[0xd402], "sw zero, 0x8(gp)"),
    (0x80200306, &[0xbe30], "movep a0, a1, s0, s1"),
    (0x80200308, &[0xbf0b], "movep a3, a4, zero, s0"),
    (0x8020030a, &[0xfe30], "movep s0, s1, a0, a1"),
    (0x8020030c, &[0xffea], "movep a6, s7, a3, a4"),
    (0x8020030e, &[0x0a1f, 0xfd83], "move.balc a0, s0, 0x80200094"),
    (0x80200312, &[0x097f, 0xfd7f], "move.balc a1, zero, 0x80200094"),
    (0x80200316, &[0x9251], "addiu a0, a1, 0x4"),
    (0x80200318, &[0x9098], "addiu a0, -0x8"),
    (0x8020031a, &[0x918f], "addiu t0, 0x7"),
    (0x8020031c, &[0xb359], "subu a0, a1, a2"),
    (0x8020031e, &[0xb358], "addu a0, a1, a2"),
    (0x80200320, &[0x5250], "not a0, a1"),
    (0x80200322, &[0xdac2], "beqc a0, a1, 0x80200328"),
    (0x80200324, &[0xa885, 0x0000], "bnec a1, a0, 0x80200328"),
    (0x80200328, &[0x9a7f], "beqzc a0, 0x80200328"),
    (0x8020032a, &[0xba7d], "bnezc a0, 0x80200328"),
    (0x8020032c, &[0x1bfb], "bc 0x80200328"),
    (0x8020032e, &[0x3bf9], "balc 0x80200328"),
    (0x80200330, &[0xc883, 0xfff5], "beqic a0, 0x7f, 0x80200328"),
    (0x80200334, &[0xc890, 0x07f1], "bneic a0, 0x0, 0x80200328"),
    (0x80200338, &[0xc888, 0x2fed], "bgeic a0, 0x5, 0x80200328"),
    (0x8020033c, &[0xc898, 0x2fe9], "bltic a0, 0x5, 0x80200328"),
    (0x80200340, &[0xc88c, 0x2fe5], "bgeiuc a0, 0x5, 0x80200328"),
    (0x80200344, &[0xc89c, 0x2fe1], "bltiuc a0, 0x5, 0x80200328"),
    (0x80200348, &[0xc884, 0xffdd], "bbeqzc a0, 0x1f, 0x80200328"),
    (0x8020034c, &[0xc894, 0x07d9], "bbnezc a0, 0x0, 0x80200328"),
    (0x80200350, &[0x8984, 0xffd5], "bgeuc a0, t0, 0x80200328"),
    (0x80200354, &[0xa984, 0xffd1], "bltuc a0, t0, 0x80200328"),
    (0x80200358, &[0x049f, 0xfd39], "lapc a0, 0x80200094"),
    (0x8020035c, &[0x6083, 0xfd32, 0xffff], "lapc a0, 0x80200094"),
    (0x80200362, &[0x608b, 0xfd2c, 0xffff], "lwpc a0, 0x80200094"),
    (0x80200368, &[0x608f, 0xfd26, 0xffff], "swpc a0, 0x80200094"),
    (0x8020036e, &[0xe094, 0x5246], "aluipc a0, 0x92545000"),
    (0x80200372, &[0x20c5, 0x21df], "extw a0, a1, a2, 0x7"),
    (0x80200376, &[0x20c5, 0x260f], "lsa a0, a1, a2, 0x3"),
    (0x8020037a, &[0x20c5, 0x2210], "movz a0, a1, a2"),
    (0x8020037e, &[0x20c5, 0x2610], "movn a0, a1, a2"),
    (0x80200382, &[0xa465, 0x91fc], "aset 0x3, -0x4(a1)"),
    (0x80200386, &[0xa6e5, 0x1105], "aclr 0x7, 0x5(a1)"),
    (0x8020038a, &[0x2000, 0xd37f], "iret"),
    (0x8020038e, &[0x000c, 0x0003], "hypcall 0x3"),
    (0x80200392, &[0x100d], "hypcall 0x1"),
];
