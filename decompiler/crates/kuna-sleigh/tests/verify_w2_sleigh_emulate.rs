//! Verifier adversarial tests for item `w2-sleigh-emulate` (round 1).
//!
//! Probes A-F below were pinned against a standalone C++ oracle harness:
//! verbatim transcriptions of `MemoryBank::setValue/getValue` (memstate.cc:
//! 182-294), the default `getPage`/`setPage` word loops (93-171),
//! `set/getChunk` (302-359), `MemoryHashOverlay::insert/find` (551-614),
//! `EmulatePcodeCache::executeBranch`'s uintm arithmetic (emulate.cc:404-421)
//! and `EmulateSnippet::executeBranch`'s `(int4)offset` truncation
//! (emulateutil.cc:204-219), compiled `g++ -O0 -Wno-sign-compare` on the
//! little-endian oracle host.  Oracle outputs reproduced in
//! docs/rust-port/reviews/w2-sleigh-emulate.md.
//!
//! Targets: the wordsize<8 hash-bank quirks (unmasked full-uintb values
//! through the `size == wordsize` early paths — *defined* C++ behavior),
//! big-endian unaligned/spill setValue arithmetic, chunk traffic through the
//! DEFAULT page word-loops (the porter's tests only exercise aligned LE
//! cases there), the upstream getPage first-word anomaly (C++ buffer
//! overrun -> ADR 0004 panic), the backward intra-instruction branch (uintm
//! wraparound), the relative-branch-at-instruction-start `instruction_start`
//! quirk of executeInstruction, and the snippet backward CBRANCH.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, OtherSpace,
    RegisterLookup, UniqueSpace, VarnodeStorage,
};
use kuna_base::xml::DocumentStorage;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::emulate::{BreakTableCallBack, Emulate, EmulatePcodeCache};
use kuna_sleigh::emulateutil::EmulateSnippet;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::memstate::{MemoryBank, MemoryHashOverlay, MemoryState};
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate, TranslateBase};

// ---------------------------------------------------------------------------
// fixtures
// ---------------------------------------------------------------------------

/// const(0)/OTHER(1)/unique(2)/ram(3)/register(4), as in the porter's
/// test_support fixture (little-endian unless `big_end`).
fn manager(big_end: bool) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(OtherSpace::new(1))).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(2, 0, big_end))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big_end,
        8,
        1,
        3,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        big_end,
        4,
        1,
        4,
        0,
        0,
        0,
    )))
    .unwrap();
    m.set_default_code_space(3).unwrap();
    m
}

fn ram(m: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(m.get_space_by_name("ram").unwrap())
}

/// A loader that has no data anywhere (DataUnavail on every request); the
/// emulator fixtures here never LOAD.
struct EmptyLoader;

impl LoadImage for EmptyLoader {
    fn get_file_name(&self) -> &str {
        "empty"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("EmptyLoader"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        b"test:LE:64:default".to_vec()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// One canned p-code op of a canned instruction.
struct Op {
    opc: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// Canned-program translator: instruction offset -> (length, ops), with
/// registers r0..r7 (8 bytes at register:8*N).
struct CannedTranslator {
    base: TranslateBase,
    register_space: Rc<AddrSpace>,
    program: BTreeMap<u64, (i32, Vec<Op>)>,
}

impl CannedTranslator {
    fn new(m: &AddrSpaceManager, program: BTreeMap<u64, (i32, Vec<Op>)>) -> Self {
        CannedTranslator {
            base: TranslateBase::new(),
            register_space: Rc::clone(m.get_space_by_name("register").unwrap()),
            program,
        }
    }
}

impl RegisterLookup for CannedTranslator {
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        if let Some(num) = nm.strip_prefix('r') {
            if let Ok(index) = num.parse::<u64>() {
                if index < 8 {
                    return Ok(VarnodeStorage {
                        space: Some(Rc::clone(&self.register_space)),
                        offset: index * 8,
                        size: 8,
                    });
                }
            }
        }
        Err(KunaError::sleigh(format!("Unknown register name: {nm}")))
    }
    fn get_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: i32) -> String {
        String::new()
    }
    fn get_exact_register_name(&self, _base: &Rc<AddrSpace>, _off: u64, _size: i32) -> String {
        String::new()
    }
}

impl Translate for CannedTranslator {
    fn translate_base(&self) -> &TranslateBase {
        &self.base
    }
    fn translate_base_mut(&mut self) -> &mut TranslateBase {
        &mut self.base
    }
    fn initialize(&mut self, _store: &mut DocumentStorage) -> KunaResult<()> {
        Ok(())
    }
    // mutable_key_type: see the trait declaration (translate.rs)
    #[allow(clippy::mutable_key_type)]
    fn get_all_registers(&self, _reglist: &mut BTreeMap<VarnodeData, String>) {}
    fn get_user_op_names(&self, _res: &mut Vec<String>) {}
    fn instruction_length(&self, baseaddr: &Address) -> KunaResult<i32> {
        match self.program.get(&baseaddr.get_offset()) {
            None => Err(KunaError::bad_data("no instruction")),
            Some((len, _)) => Ok(*len),
        }
    }
    fn one_instruction(&self, emit: &mut dyn PcodeEmit, baseaddr: &Address) -> KunaResult<i32> {
        match self.program.get(&baseaddr.get_offset()) {
            None => Err(KunaError::bad_data("no instruction")),
            Some((len, ops)) => {
                for op in ops {
                    emit.dump(baseaddr, op.opc, op.out.as_ref(), &op.ins);
                }
                Ok(*len)
            }
        }
    }
    fn print_assembly(&self, _emit: &mut dyn AssemblyEmit, _baseaddr: &Address) -> KunaResult<i32> {
        Err(KunaError::lowlevel("no assembly"))
    }
}

fn cst(m: &AddrSpaceManager, val: u64, size: u32) -> VarnodeData {
    VarnodeData {
        space: Some(Rc::clone(m.get_constant_space().unwrap())),
        offset: val,
        size,
    }
}

fn reg(m: &AddrSpaceManager, n: u64) -> VarnodeData {
    VarnodeData {
        space: Some(Rc::clone(m.get_space_by_name("register").unwrap())),
        offset: n * 8,
        size: 8,
    }
}

fn uvn(m: &AddrSpaceManager, offset: u64, size: u32) -> VarnodeData {
    VarnodeData {
        space: Some(Rc::clone(m.get_space_by_name("unique").unwrap())),
        offset,
        size,
    }
}

/// Emulator over a canned program: hash overlays (ws=8, ps=64, hashsize=64)
/// for ram, register and unique, as in the emulate.hh documentation example.
fn build_emulator(
    m: &Rc<AddrSpaceManager>,
    program: BTreeMap<u64, (i32, Vec<Op>)>,
) -> (EmulatePcodeCache, Rc<RefCell<MemoryState>>, Rc<AddrSpace>) {
    let trans: Rc<dyn Translate> = Rc::new(CannedTranslator::new(m, program));
    let r = ram(m);
    let register = Rc::clone(m.get_space_by_name("register").unwrap());
    let unique = Rc::clone(m.get_space_by_name("unique").unwrap());
    let mut state = MemoryState::new(Rc::clone(&trans));
    let ram_bank: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(MemoryHashOverlay::new(
        Rc::clone(&r),
        8,
        64,
        64,
        None,
    )));
    let reg_bank: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(MemoryHashOverlay::new(
        register, 8, 64, 64, None,
    )));
    let uniq_bank: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(MemoryHashOverlay::new(
        unique, 8, 64, 64, None,
    )));
    state.set_memory_bank(ram_bank);
    state.set_memory_bank(reg_bank);
    state.set_memory_bank(uniq_bank);
    let memstate = Rc::new(RefCell::new(state));
    let breaktable = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
    let emulator = EmulatePcodeCache::new(trans, Rc::clone(m), Rc::clone(&memstate), breaktable);
    (emulator, memstate, r)
}

// ---------------------------------------------------------------------------
// memstate: oracle probes A-D
// ---------------------------------------------------------------------------

/// Probe A: a wordsize-4 hash bank stores FULL uintb values through the
/// `size == wordsize` early `insert` in setValue (memstate.cc:200-202) and
/// returns them UNMASKED through the early `return val1` in getValue
/// (memstate.cc:273-274) — defined C++ behavior the port must transcribe,
/// not repair.  Oracle: A1=1122334455667788, A2=5566, A3=0000aabbccdd0000,
/// A4=aabb, A5=aabbccdd.
#[test]
fn verify_w2emulate_hash_bank_wordsize4_unmasked_value_oracle() {
    let m = manager(false);
    let mut bank = MemoryHashOverlay::new(ram(&m), 4, 16, 16, None);

    bank.set_value(0x100, 4, 0x1122334455667788).unwrap();
    // A1: all 64 bits come back from a 4-byte read
    assert_eq!(bank.get_value(0x100, 4).unwrap(), 0x1122334455667788);
    // A2: a sub-word read masks normally
    assert_eq!(bank.get_value(0x102, 2).unwrap(), 0x5566);

    // spill across two 4-byte words: read-modify-write through find
    bank.set_value(0x106, 4, 0xAABBCCDD).unwrap();
    // A3/A4: the stored word values (find is word-aligned access)
    assert_eq!(bank.find(0x104).unwrap(), 0x0000AABBCCDD0000);
    assert_eq!(bank.find(0x108).unwrap(), 0xAABB);
    // A5: reassembled value
    assert_eq!(bank.get_value(0x106, 4).unwrap(), 0xAABBCCDD);
}

/// Probe B: big-endian unaligned setValue, non-spill (gap shift) and spill
/// (8*size1 / 8*size2 shifts) paths.  Oracle: B1=000000aabbccdd00,
/// B2=aabbccdd, B3=1122, B4=3344000000000000, B5=11223344.
#[test]
fn verify_w2emulate_setvalue_bigendian_unaligned_spill_oracle() {
    let m = manager(true);
    let mut bank = MemoryHashOverlay::new(ram(&m), 8, 16, 16, None);

    // non-spill: offset 0x103 size 4 within word 0x100
    bank.set_value(0x103, 4, 0xAABBCCDD).unwrap();
    assert_eq!(bank.find(0x100).unwrap(), 0x000000AABBCCDD00);
    assert_eq!(bank.get_value(0x103, 4).unwrap(), 0xAABBCCDD);

    // spill: offset 0x10e size 4 across words 0x108 / 0x110
    bank.set_value(0x10e, 4, 0x11223344).unwrap();
    assert_eq!(bank.find(0x108).unwrap(), 0x1122);
    assert_eq!(bank.find(0x110).unwrap(), 0x3344000000000000);
    assert_eq!(bank.get_value(0x10e, 4).unwrap(), 0x11223344);
}

/// Probes C+D: chunk traffic routed through the DEFAULT getPage/setPage
/// word loops (MemoryHashOverlay overrides neither), word-aligned: a
/// page-crossing LE write (ps=16), a 1.5-word read, then the big-endian
/// full-word byte_swap path and the partial-word read-modify-write path.
/// Oracle: C1=0807060504030201, C2=100f0e0d0c0b0a09, C3=1817161514131211,
/// C4=09..14; D1=1122334455667788, D2 same, D3=11 22 .. 88,
/// D4=deadbf0000000000, D5=deadbf.
#[test]
fn verify_w2emulate_default_page_word_loop_chunk_oracle() {
    // little-endian, page-crossing setChunk + intra-page getChunk
    let m = manager(false);
    let mut bank = MemoryHashOverlay::new(ram(&m), 8, 16, 32, None);
    let buf: Vec<u8> = (1..=24).collect();
    bank.set_chunk(0x108, &buf).unwrap();
    assert_eq!(bank.find(0x108).unwrap(), 0x0807060504030201);
    assert_eq!(bank.find(0x110).unwrap(), 0x100F0E0D0C0B0A09);
    assert_eq!(bank.find(0x118).unwrap(), 0x1817161514131211);
    let mut out = [0xEEu8; 12];
    bank.get_chunk(0x110, &mut out).unwrap();
    assert_eq!(
        out,
        [0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14]
    );

    // big-endian: full-word path (byte_swap of the raw host word) and the
    // partial-word find/patch path of the default setPage
    let mbe = manager(true);
    let mut bank = MemoryHashOverlay::new(ram(&mbe), 8, 16, 32, None);
    bank.set_chunk(0x100, &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88])
        .unwrap();
    assert_eq!(bank.find(0x100).unwrap(), 0x1122334455667788);
    assert_eq!(bank.get_value(0x100, 8).unwrap(), 0x1122334455667788);
    let mut out = [0xEEu8; 8];
    bank.get_chunk(0x100, &mut out).unwrap();
    assert_eq!(out, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    bank.set_chunk(0x108, &[0xDE, 0xAD, 0xBF]).unwrap();
    assert_eq!(bank.find(0x108).unwrap(), 0xDEADBF0000000000);
    assert_eq!(bank.get_value(0x108, 3).unwrap(), 0xDEADBF);
}

/// Upstream anomaly: the default getPage compares the first partial word
/// against `addr` (the page start) instead of `addr + skip`
/// (memstate.cc:113), so a chunk read at a non-word-aligned offset through
/// a default-page bank overruns the caller's buffer in C++ (UB).  ADR 0004
/// maps that UB state to a panic; pin it so a silent "fix" (which would
/// diverge from any C++ execution that survives the overrun) is caught.
#[test]
#[should_panic]
fn verify_w2emulate_unaligned_chunk_default_getpage_cpp_overrun_panics() {
    let m = manager(false);
    let bank = MemoryHashOverlay::new(ram(&m), 8, 16, 16, None);
    let mut out = [0u8; 4];
    // offset 0x103: skip=3 within the page, not word-aligned -> C++ copies
    // wordsize bytes from the word start and overruns `out` by 3
    let _ = bank.get_chunk(0x103, &mut out);
}

// ---------------------------------------------------------------------------
// emulate: backward intra-instruction branch + instruction_start quirk
// ---------------------------------------------------------------------------

/// Probe E: a backward relative CBRANCH.  destaddr offset (uintb)-2
/// truncates to uintm 0xFFFFFFFE (emulate.cc:409); id + (uintm)current_op
/// wraps around 2^32 back to op index 0 — the loop converges and falls
/// through to the next instruction.  Oracle: current_op = 0, in-bounds.
#[test]
fn verify_w2emulate_backward_intra_instruction_branch_uintm_wrap() {
    let m = Rc::new(manager(false));
    let mut program = BTreeMap::new();
    // 0x1000: { r0 += 1; u100 = r0 < 3; CBRANCH -2, u100 }
    program.insert(
        0x1000,
        (
            4,
            vec![
                Op {
                    opc: OpCode::CPUI_INT_ADD,
                    out: Some(reg(&m, 0)),
                    ins: vec![reg(&m, 0), cst(&m, 1, 8)],
                },
                Op {
                    opc: OpCode::CPUI_INT_LESS,
                    out: Some(uvn(&m, 0x100, 1)),
                    ins: vec![reg(&m, 0), cst(&m, 3, 8)],
                },
                Op {
                    opc: OpCode::CPUI_CBRANCH,
                    out: None,
                    // (uintb)-2: truncates to uintm 0xFFFFFFFE in
                    // executeBranch, wrapping back to index 0
                    ins: vec![cst(&m, u64::MAX - 1, 8), uvn(&m, 0x100, 1)],
                },
            ],
        ),
    );
    // 0x1004: r1 = 0x77 (proves the fallthru landed here)
    program.insert(
        0x1004,
        (
            4,
            vec![Op {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(&m, 1)),
                ins: vec![cst(&m, 0x77, 8)],
            }],
        ),
    );

    let (mut emulator, memstate, r) = build_emulator(&m, program);
    // r0 starts 0 (hash overlay over nothing reads zero)
    emulator
        .set_execute_address(&Address::new(Rc::clone(&r), 0x1000))
        .unwrap();
    // The instruction loops internally: r0 = 1, 2 take the backward branch,
    // r0 = 3 falls through; one executeInstruction call does all of it
    // (instruction_start stays false during the backward branches).
    emulator.execute_instruction().unwrap();
    assert_eq!(emulator.get_execute_address().get_offset(), 0x1004);
    assert_eq!(memstate.borrow().get_value_by_name("r0").unwrap(), 3);
    // Executing the last mapped instruction errs out of the eager fallthru
    // translation of 0x1008 — but only AFTER the COPY ran and the address
    // advanced (C++ partial-state parity: oneInstruction throws after
    // current_address was already updated, emulate.cc:396-398).
    assert!(emulator.execute_instruction().is_err());
    assert_eq!(memstate.borrow().get_value_by_name("r1").unwrap(), 0x77);
    assert_eq!(emulator.get_execute_address().get_offset(), 0x1008);
}

/// The instruction_start quirk (emulate.cc:391-401 + 450-460): a relative
/// BRANCH does not clear instruction_start, so when the FIRST op of an
/// instruction is an intra-instruction branch, executeInstruction's
/// do-while exits after executing only that branch (instruction_start is
/// still true), the execute address still names the same instruction, and
/// the next executeInstruction call re-checks address breakpoints and
/// finishes the instruction.
#[test]
fn verify_w2emulate_relative_branch_first_op_instruction_start_quirk() {
    let m = Rc::new(manager(false));
    let mut program = BTreeMap::new();
    // 0x1000: { BRANCH +2; r0 = 0xBAD (skipped); r1 = 0x22 }
    program.insert(
        0x1000,
        (
            4,
            vec![
                Op {
                    opc: OpCode::CPUI_BRANCH,
                    out: None,
                    ins: vec![cst(&m, 2, 8)],
                },
                Op {
                    opc: OpCode::CPUI_COPY,
                    out: Some(reg(&m, 0)),
                    ins: vec![cst(&m, 0xBAD, 8)],
                },
                Op {
                    opc: OpCode::CPUI_COPY,
                    out: Some(reg(&m, 1)),
                    ins: vec![cst(&m, 0x22, 8)],
                },
            ],
        ),
    );
    // 0x1004: r2 = 0x33
    program.insert(
        0x1004,
        (
            4,
            vec![Op {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(&m, 2)),
                ins: vec![cst(&m, 0x33, 8)],
            }],
        ),
    );

    let (mut emulator, memstate, r) = build_emulator(&m, program);
    emulator
        .set_execute_address(&Address::new(Rc::clone(&r), 0x1000))
        .unwrap();

    // Call 1: executes ONLY the relative branch (instruction_start stays
    // true), landing on op index 2 of the SAME instruction.
    emulator.execute_instruction().unwrap();
    assert!(emulator.is_instruction_start());
    assert_eq!(emulator.get_current_op_index(), 2);
    assert_eq!(emulator.get_execute_address().get_offset(), 0x1000);
    assert_eq!(memstate.borrow().get_value_by_name("r1").unwrap(), 0);

    // Call 2: executes r1 = 0x22 and falls through to 0x1004.
    emulator.execute_instruction().unwrap();
    assert_eq!(emulator.get_execute_address().get_offset(), 0x1004);
    assert_eq!(memstate.borrow().get_value_by_name("r1").unwrap(), 0x22);
    // the skipped op never ran
    assert_eq!(memstate.borrow().get_value_by_name("r0").unwrap(), 0);

    // Call 3: the next instruction runs (then errs translating the
    // unmapped 0x1008 fallthru, exactly as C++ would throw — the COPY has
    // already executed).
    assert!(emulator.execute_instruction().is_err());
    assert_eq!(memstate.borrow().get_value_by_name("r2").unwrap(), 0x33);
}

// ---------------------------------------------------------------------------
// emulateutil: snippet backward branch
// ---------------------------------------------------------------------------

/// Probe F: EmulateSnippet backward CBRANCH.  `(int4)vn->offset` of
/// (uintb)-2 truncates to rel = -2 (emulateutil.cc:210); pos walks back
/// from 3 to 1 and the loop converges.  Oracle: pos = 1 after the branch
/// at index 3 (3 + (-2)); final u98 = 6.
#[test]
fn verify_w2emulate_snippet_backward_cbranch_int4_truncation() {
    let m = Rc::new(manager(false));
    let loader: Rc<RefCell<dyn LoadImage>> = Rc::new(RefCell::new(EmptyLoader));
    let trans: Rc<dyn Translate> = Rc::new(CannedTranslator::new(&m, BTreeMap::new()));
    let mut inst: Vec<Option<Rc<dyn kuna_num::opbehavior::OpBehavior>>> = Vec::new();
    let provider: Rc<dyn kuna_num::opbehavior::FloatFormatProvider> =
        Rc::new(kuna_sleigh::emulate::TranslateFloatFormats(trans));
    kuna_num::opbehavior::register_instructions(&mut inst, &provider);

    let mut snippet = EmulateSnippet::new(loader, Rc::clone(&m));
    let addr = Address::new(ram(&m), 0x1000);
    {
        let mut emit = snippet.build_emitter(&inst, 0x200);
        // 0: u80 = COPY 0
        emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(&m, 0x80, 8)), &[cst(&m, 0, 8)]);
        // 1: u80 = INT_ADD u80, 1
        emit.dump(
            &addr,
            OpCode::CPUI_INT_ADD,
            Some(&uvn(&m, 0x80, 8)),
            &[uvn(&m, 0x80, 8), cst(&m, 1, 8)],
        );
        // 2: u90 = INT_LESS u80, 3
        emit.dump(
            &addr,
            OpCode::CPUI_INT_LESS,
            Some(&uvn(&m, 0x90, 1)),
            &[uvn(&m, 0x80, 8), cst(&m, 3, 8)],
        );
        // 3: CBRANCH -2, u90   ((uintb)-2 truncates to int4 -2 -> index 1)
        emit.dump(
            &addr,
            OpCode::CPUI_CBRANCH,
            None,
            &[cst(&m, u64::MAX - 1, 8), uvn(&m, 0x90, 1)],
        );
        // 4: u98 = INT_MULT u80, 2
        emit.dump(
            &addr,
            OpCode::CPUI_INT_MULT,
            Some(&uvn(&m, 0x98, 8)),
            &[uvn(&m, 0x80, 8), cst(&m, 2, 8)],
        );
    }
    assert!(snippet.check_for_legal_code());

    snippet.reset_memory();
    let mut steps = 0;
    while !snippet.get_halt() {
        snippet.execute_current_op().unwrap();
        steps += 1;
        assert!(steps < 100, "runaway snippet");
    }
    assert_eq!(snippet.get_temp_value(0x80), 3);
    assert_eq!(snippet.get_temp_value(0x98), 6);
}
