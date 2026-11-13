use std::rc::Rc;

use kuna_base::space::{ConstantSpace, OtherSpace, UniqueSpace};

use super::*;

type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

struct Fx {
    cst: Rc<AddrSpace>,
    reg: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
    tmp: Rc<AddrSpace>,
    regs: Regs,
}

fn fx() -> Fx {
    let reg: Rc<AddrSpace> = Rc::new(OtherSpace::new(1));
    let regs = Regs { space: reg.get_index(), offs: [0, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c] };
    Fx {
        cst: Rc::new(ConstantSpace::new()),
        reg,
        ram: Rc::new(OtherSpace::new(3)),
        tmp: Rc::new(UniqueSpace::new(2, 0, false)),
        regs,
    }
}

fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
    VarnodeData { space: Some(space.clone()), offset, size }
}

impl Fx {
    fn r(&self, i: usize) -> VarnodeData {
        vn(&self.reg, self.regs.offs[i], 4)
    }
    fn c(&self, k: i64) -> VarnodeData {
        vn(&self.cst, k as u32 as u64, 4)
    }
    fn add(&self, dst: usize, src: usize, k: i64) -> Op {
        (OpCode::CPUI_INT_ADD, Some(self.r(dst)), vec![self.r(src), self.c(k)])
    }
    fn sub_reg(&self, dst: usize, other: usize) -> Op {
        (OpCode::CPUI_INT_SUB, Some(self.r(dst)), vec![self.r(dst), self.r(other)])
    }
    fn copy(&self, dst: usize, src: usize) -> Op {
        (OpCode::CPUI_COPY, Some(self.r(dst)), vec![self.r(src)])
    }
    fn push(&self, src: usize) -> Vec<Op> {
        vec![
            self.add(SP, SP, -4),
            (OpCode::CPUI_STORE, None, vec![vn(&self.cst, 3, 8), self.r(SP), self.r(src)]),
        ]
    }
    fn pop(&self, dst: usize) -> Vec<Op> {
        vec![
            (OpCode::CPUI_LOAD, Some(self.r(dst)), vec![vn(&self.cst, 3, 8), self.r(SP)]),
            self.add(SP, SP, 4),
        ]
    }
    fn ret(&self, imm: i64) -> Vec<Op> {
        let eip = vn(&self.reg, 0x284, 4);
        let mut v = vec![(OpCode::CPUI_LOAD, Some(eip.clone()), vec![vn(&self.cst, 3, 8), self.r(SP)]), self.add(SP, SP, 4)];
        if imm != 0 {
            v.push(self.add(SP, SP, imm));
        }
        v.push((OpCode::CPUI_RETURN, None, vec![eip]));
        v
    }
    fn call(&self, target: u64) -> Vec<Op> {
        let mut v = self.push(0);
        v.push((OpCode::CPUI_CALL, None, vec![vn(&self.ram, target, 4)]));
        v
    }
    fn run(&self, insns: &[Vec<Op>]) -> Vec<Step> {
        let mut state = State::at_entry();
        let mut out = Vec::new();
        for ops in insns {
            let s = step(ops, &self.regs, state);
            state = s.state;
            out.push(s);
        }
        out
    }
}

fn sp(s: &Step) -> Val {
    s.state.0[SP].unwrap()
}

#[test]
fn ret_imm16_from_a_balanced_stack_states_its_extrapop() {
    let f = fx();
    let steps = f.run(&[f.ret(0x14)]);
    assert_eq!(steps[0].ret, Some((Val::exact(0), Val::exact(0x18))));
    assert!(!steps[0].falls_through);
}

#[test]
fn a_frame_restores_the_stack_pointer() {
    let f = fx();
    let steps = f.run(&[
        f.push(BP),
        vec![f.copy(BP, SP)],
        vec![f.add(SP, SP, -0x20)],
        vec![f.copy(SP, BP)],
        f.pop(BP),
        f.ret(8),
    ]);
    assert_eq!(steps[1].state.0[BP], Some(Val::exact(-4)));
    assert_eq!(steps[5].ret, Some((Val::exact(0), Val::exact(0xc))));
}

#[test]
fn a_runtime_stack_adjustment_is_the_callees_own() {
    let f = fx();
    let steps = f.run(&[vec![f.sub_reg(SP, 0)], f.ret(0)]);
    let (pre, _) = steps[1].ret.unwrap();
    assert!(pre.as_exact().is_none() && pre.own(), "alloca-style `sub esp,eax` is the callee's own move: {pre:?}");
}

#[test]
fn restoring_from_the_callers_frame_pointer_is_the_callees_own() {
    let f = fx();
    let steps = f.run(&[vec![f.copy(SP, BP)], f.pop(BP), f.push(1), f.ret(0)]);
    let (pre, _) = steps[3].ret.unwrap();
    assert!(pre.own(), "an epilogue helper's `mov esp,ebp` on the caller's EBP: {pre:?}");
    assert_ne!(steps[1].writes, 0, "pop ebp writes a callee-saved register");
}

#[test]
fn a_push_and_ret_is_unbalanced_by_an_exact_amount() {
    let f = fx();
    let steps = f.run(&[f.push(0), f.ret(0)]);
    assert_eq!(steps[1].ret.unwrap().0, Val::exact(-4));
}

#[test]
fn a_call_is_reported_with_the_return_address_pushed() {
    let f = fx();
    let steps = f.run(&[f.call(0x401000)]);
    assert_eq!(steps[0].call, Some(Call::Direct(0x401000)));
    assert_eq!(sp(&steps[0]), Val::exact(-4));
    assert!(steps[0].falls_through);
}

#[test]
fn an_unread_call_bounds_the_stack_pointer_by_the_return_address() {
    let after = after_unread_call(Val::exact(-0x14));
    assert_eq!(after, Val::Stack { lo: -INF, hi: 0, own: false });
    assert!(after.as_exact().is_none());
}

#[test]
fn direct_jump_is_followed_and_ends_the_path() {
    let f = fx();
    let s = step(&[(OpCode::CPUI_BRANCH, None, vec![vn(&f.ram, 0x5fa1c0, 4)])], &f.regs, State::at_entry());
    assert_eq!(s.targets, vec![0x5fa1c0]);
    assert!(!s.falls_through);
    assert_eq!(s.ret, None);
}

#[test]
fn conditional_branch_keeps_the_fall_through() {
    let f = fx();
    let cond = vn(&f.reg, 0x206, 1);
    let s = step(&[(OpCode::CPUI_CBRANCH, None, vec![vn(&f.ram, 0x401020, 4), cond])], &f.regs, State::at_entry());
    assert_eq!(s.targets, vec![0x401020]);
    assert!(s.falls_through);
}

#[test]
fn intra_instruction_branch_and_indirect_jump() {
    let f = fx();
    let rel = step(&[(OpCode::CPUI_BRANCH, None, vec![vn(&f.cst, 2, 4)])], &f.regs, State::at_entry());
    assert!(rel.targets.is_empty() && rel.falls_through);
    let ind = step(&[(OpCode::CPUI_BRANCHIND, None, vec![f.r(0)])], &f.regs, State::at_entry());
    assert!(!ind.falls_through && ind.unknown);
}

#[test]
fn int3_is_neither_a_return_nor_a_call() {
    let f = fx();
    let intloc = vn(&f.tmp, 0x80, 4);
    let ops = vec![
        (OpCode::CPUI_CALLOTHER, Some(intloc.clone()), vec![f.c(5), vn(&f.cst, 3, 1)]),
        (OpCode::CPUI_CALLIND, None, vec![intloc]),
        (OpCode::CPUI_RETURN, None, vec![vn(&f.cst, 0, 1)]),
    ];
    let s = step(&ops, &f.regs, State::at_entry());
    assert_eq!((s.ret, s.call), (None, None));
    assert!(s.falls_through);
}

#[test]
fn an_import_call_names_its_slot() {
    let f = fx();
    let t = vn(&f.tmp, 0x100, 4);
    let mut ops = vec![(OpCode::CPUI_LOAD, Some(t.clone()), vec![vn(&f.cst, 3, 8), f.c(0x40e0f0)])];
    ops.extend(f.push(0));
    ops.push((OpCode::CPUI_CALLIND, None, vec![t]));
    assert_eq!(step(&ops, &f.regs, State::at_entry()).call, Some(Call::Indirect(Some(0x40e0f0))));
}

#[test]
fn a_path_through_a_stack_moving_call_meeting_a_clean_one_stays_the_calls() {
    let after_helper = Some(Val::Opaque { own: false });
    assert_eq!(join(Some(Val::exact(0)), after_helper), Some(Val::Opaque { own: false }));
    assert_eq!(join(after_helper, Some(Val::exact(-4))), Some(Val::Opaque { own: false }));
    assert_eq!(join(Some(Val::CallerFrame), after_helper), Some(Val::Opaque { own: true }));
    assert_eq!(join(Some(Val::Stack { lo: -8, hi: 0, own: true }), after_helper), Some(Val::Opaque { own: true }));
}

#[test]
fn a_walked_local_pointer_stays_below_the_return_address() {
    let old = Some(Val::exact(-0x30));
    let moved = join(old, Some(Val::exact(-0x2c)));
    assert_eq!(widen(old, moved, false), Some(Val::Stack { lo: -0x30, hi: -1, own: true }));
    assert_eq!(widen(old, moved, true), Some(Val::Stack { lo: -0x30, hi: INF, own: true }));
}

impl Fx {
    fn load(&self, base: usize, off: i64) -> Vec<Op> {
        let t = vn(&self.tmp, 0x100, 4);
        vec![
            (OpCode::CPUI_INT_ADD, Some(t.clone()), vec![self.r(base), self.c(off)]),
            (OpCode::CPUI_LOAD, Some(self.r(0)), vec![vn(&self.cst, 3, 8), t]),
        ]
    }
    fn extent(&self, insns: &[Vec<Op>]) -> (i64, bool) {
        let steps = self.run(insns);
        (steps.iter().map(|s| s.arg_end).max().unwrap_or(4), steps.iter().any(|s| s.unplaced))
    }
}

#[test]
fn reading_the_first_argument_touches_it() {
    let f = fx();
    assert_eq!(f.extent(&[f.load(SP, 4)]), (8, false));
    assert_eq!(f.extent(&[f.load(SP, 0)]), (4, false), "the return address is not an argument");
    assert_eq!(f.extent(&[f.load(SP, 0xc)]), (0x10, false), "the third argument ends at sixteen");
}

#[test]
fn a_push_moves_the_frame_and_a_frame_pointer_follows_it() {
    let f = fx();
    assert_eq!(f.extent(&[f.push(BP), f.load(SP, 8)]), (8, false));
    assert_eq!(f.extent(&[f.push(BP), vec![f.copy(BP, SP)], f.load(BP, 8)]), (8, false), "[ebp+8] is the first argument");
}

#[test]
fn the_callers_frame_pointer_is_unplaced() {
    let f = fx();
    assert!(f.extent(&[f.load(BP, 8)]).1);
}

#[test]
fn a_copy_of_the_stack_pointer_is_followed() {
    let f = fx();
    let esi = 6;
    assert_eq!(f.extent(&[f.push(esi), vec![f.copy(esi, SP)], f.load(esi, 0xc)]), (0xc, false));
}

#[test]
fn an_argument_address_handed_on_is_unplaced_and_a_locals_is_not() {
    let f = fx();
    let lea = |off| vec![f.add(0, SP, off)];
    assert!(f.extent(&[lea(8), f.push(0)]).1, "va_start's address stored");
    assert!(!f.extent(&[vec![f.add(SP, SP, -0x10)], lea(4), f.push(0)]).1, "a local's address stored");
}

#[test]
fn a_local_pointer_indexed_at_run_time_stays_a_local() {
    let f = fx();
    let ecx = 1;
    let index = vec![(OpCode::CPUI_INT_ADD, Some(f.r(0)), vec![f.r(0), f.r(ecx)])];
    assert_eq!(f.extent(&[vec![f.add(SP, SP, -0x20)], vec![f.add(0, SP, 0)], index.clone(), f.load(0, 0)]), (4, false));
    assert!(f.extent(&[vec![f.add(0, SP, 4)], index, f.load(0, 0)]).1, "an argument indexed at run time");
}
