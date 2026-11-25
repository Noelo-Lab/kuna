//! The exploration's limits on a counting loop (`rax += 1; jmp self`): the
//! work budget, the memory bound and the wall-clock backstop each end it with
//! the header caveat naming that limit.

use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace, ConstantSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::super::kuna_devirtualize::caveats_of;
use super::super::kuna_symemu::RawOp;
use super::*;

const LOOP: u64 = 0x1000;

struct Counter {
    cst: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
}

impl Counter {
    fn new() -> Self {
        let reg = AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 2, 0, 0, 0);
        let ram = AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 3, 0, 0, 0);
        Counter { cst: Rc::new(ConstantSpace::new()), reg: Rc::new(reg), ram: Rc::new(ram) }
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }
}

impl SymImage for Counter {
    fn read_bytes(&self, _addr: u64, _buf: &mut [u8]) -> bool {
        false
    }
    fn is_writable(&self, _addr: u64) -> bool {
        false
    }
}

impl SymTarget for Counter {
    fn lift(&self, addr: u64) -> Result<Rc<LiftedInsn>, String> {
        if addr != LOOP {
            return Err(format!("nothing at {addr:#x}"));
        }
        let rax = Self::vn(&self.reg, 0, 8);
        let ops = vec![
            RawOp { opc: OpCode::CPUI_INT_ADD, out: Some(rax.clone()), ins: vec![rax, Self::vn(&self.cst, 1, 8)] },
            RawOp { opc: OpCode::CPUI_BRANCH, out: None, ins: vec![Self::vn(&self.ram, LOOP, 8)] },
        ];
        Ok(Rc::new(LiftedInsn { addr, len: 4, ops, asm: "ADD RAX,1; JMP".into() }))
    }
    fn ptr_size(&self) -> u8 {
        8
    }
    fn space_by_index(&self, _index: i32) -> Option<Rc<AddrSpace>> {
        None
    }
    fn is_foreign_code(&self, _addr: u64) -> bool {
        false
    }
    fn is_executable(&self, addr: u64) -> bool {
        addr == LOOP
    }
    fn is_mapped(&self, addr: u64) -> bool {
        addr == LOOP
    }
    fn userop_name(&self, _index: u64) -> Option<String> {
        None
    }
    fn is_function_entry(&self, _addr: u64) -> bool {
        false
    }
    fn callee_cleans(&self) -> bool {
        false
    }
    fn declared_pop(&self, _addr: u64) -> Option<u32> {
        None
    }
    fn bytecode_like(&self, _addr: u64) -> bool {
        false
    }
    fn noreturn_interrupt(&self, _vector: u64) -> bool {
        false
    }
}

fn abi() -> Abi {
    Abi {
        ptr_size: 8,
        sp: 0x20,
        return_reg: 0,
        clobbers: Vec::new(),
        flag_regs: (0x200, 0x216),
        gprs: vec![0, 0x20],
        big_endian: false,
        df: None,
        stack_args: 0,
        call_home_words: 0,
        call_arg_words: 0,
        pc: None,
        fixed_flags: Vec::new(),
        callee_saved: Vec::new(),
        fixed_regs: Vec::new(),
        args: Vec::new(),
    }
}

/// Explore the loop with `limit` applied to the default configuration: the
/// statistics and the work done.
fn explore_counting(limit: impl FnOnce(&mut Config)) -> (Stats, u64) {
    let target = Counter::new();
    let mut config = Config { trace: None, ..Config::default() };
    limit(&mut config);
    let mut explorer = Explorer::new(&target, abi(), config, LOOP);
    explorer.probe = false;
    explorer.run();
    (explorer.stats, explorer.arena.work())
}

fn explore(limit: impl FnOnce(&mut Config)) -> Stats {
    explore_counting(limit).0
}

#[test]
fn unlimited_loop_reaches_its_fixpoint() {
    let stats = explore(|_| {});
    assert!(stats.converged);
    assert_eq!(stats.cut_short, None);
    assert_eq!(caveats_of(&stats), None);
}

#[test]
fn work_budget_ends_the_exploration_and_names_itself() {
    let stats = explore(|c| c.max_work = 1);
    assert_eq!(stats.cut_short, Some(Limit::Work));
    assert!(!stats.converged);
    let caveat = caveats_of(&stats).expect("a caveat");
    assert!(caveat.contains("exploration cut short by its work budget"), "{caveat}");
}

#[test]
fn memory_bound_ends_the_exploration_and_names_itself() {
    let stats = explore(|c| c.max_memory = 1);
    assert_eq!(stats.cut_short, Some(Limit::Memory));
    let caveat = caveats_of(&stats).expect("a caveat");
    assert!(caveat.contains("exploration cut short by its memory bound"), "{caveat}");
}

#[test]
fn wall_clock_backstop_ends_the_exploration_and_names_itself() {
    let stats = explore(|c| c.max_seconds = 0);
    assert_eq!(stats.cut_short, Some(Limit::Time));
    let caveat = caveats_of(&stats).expect("a caveat");
    assert!(caveat.contains("exploration cut short by its wall-clock limit"), "{caveat}");
}

#[test]
fn work_budget_cuts_at_the_same_point_every_run() {
    let (full, work) = explore_counting(|_| {});
    assert!(full.rounds > 1 && work > 1);
    let (a, wa) = explore_counting(|c| c.max_work = work / 2);
    let (b, wb) = explore_counting(|c| c.max_work = work / 2);
    assert_eq!(a.cut_short, Some(Limit::Work));
    assert_eq!((a.rounds, a.nodes, a.insns, a.halts, wa), (b.rounds, b.nodes, b.insns, b.halts, wb));
}
