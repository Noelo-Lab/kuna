//! Thunks: where a call really goes once jumps, PLT slots and veneers are followed.

use super::*;

/// Where a call to `target` really goes: through a thunk (a jump, a PLT
/// slot's `jmp [got]`, an `endbr64` before it) to what it jumps to, when two
/// runs from different states reach the same place without storing or
/// calling. A target that is no thunk is its own answer.
pub fn through_thunks(env: &Env, target: u64) -> u64 {
    if let Some(&t) = env.thunks.borrow().get(&target) {
        return t;
    }
    let t = follow_thunks(env, target);
    env.thunks.borrow_mut().insert(target, t);
    t
}

/// Whether `target` is a thunk that leads out of the image's code: a PLT
/// stub whose GOT slot the file leaves zero or points outside. Running it
/// would jump to no code, so its call is a call out of the image.
pub fn leads_out(env: &Env, target: u64) -> bool {
    let t = through_thunks(env, target);
    t != target && !env.is_code(t)
}

/// Where a call to `target` goes through thunks that change nothing a callee
/// can see: each hop writes only temporaries and the registers the ABI keeps
/// for veneers ([`Env::scratch_regs`]), reads no register it did not write
/// itself, stores nothing and leaves the stack pointer and flags alone. So the
/// destination depends only on the code and the image's bytes, never on the
/// state at the call. A PLT slot's `jmp [got]` and a
/// plain `jmp f` are such thunks; one that sets or pushes an argument, or
/// moves the stack pointer, is not. Two calls are the same call when they
/// reach the same code this way.
pub fn transparent_target(env: &Env, target: u64) -> u64 {
    if let Some(&t) = env.transparent.borrow().get(&target) {
        return t;
    }
    let t = follow(env, target, true);
    env.transparent.borrow_mut().insert(target, t);
    t
}

pub(super) fn follow_thunks(env: &Env, target: u64) -> u64 {
    follow(env, target, false)
}

pub(super) fn follow(env: &Env, target: u64, strict: bool) -> u64 {
    let mut t = target;
    for _ in 0..4 {
        let a = thunk_hop(env, t, 0, strict);
        let b = thunk_hop(env, t, 1, strict);
        match (a, b) {
            (Some(x), Some(y)) if x == y && x != t => t = x,
            _ => break,
        }
    }
    t
}

/// Whether `out` is an argument register the calling convention reads.
pub(super) fn writes_argument(env: &Env, out: &VarnodeData) -> bool {
    space(out).is_ok_and(|s| s.get_index() == env.register.get_index())
        && env.arg_regs.iter().any(|&(o, n)| out.offset < o + u64::from(n) && o < out.offset + u64::from(out.size))
}

/// Whether a `strict` hop may write `out`: a temporary or a veneer register.
pub(super) fn hidden(env: &Env, out: &VarnodeData) -> bool {
    let Ok(s) = space(out) else { return false };
    s.get_type() == spacetype::IPTR_INTERNAL
        || (s.get_index() == env.register.get_index()
            && env.scratch_regs.iter().any(|&(o, n)| out.offset >= o && out.offset + u64::from(out.size) <= o + u64::from(n)))
}

/// Where the thunk at `at` jumps, run from state `run`; `None` when the code
/// there stores anywhere but the stack (a lazy PLT entry pushes its index),
/// writes an argument register, calls, tests or does not jump within a few
/// instructions. `strict` also refuses every store and any write but to a
/// temporary or a scratch register ([`transparent_target`]).
pub(super) fn thunk_hop(env: &Env, at: u64, run: u32, strict: bool) -> Option<u64> {
    let mut state = State::new(env, 0x7fff_0000 + run);
    let mut addr = at;
    let mut written: Vec<(u64, u32)> = Vec::new();
    let is_reg = |vn: &VarnodeData| space(vn).is_ok_and(|s| s.get_index() == env.register.get_index());
    for _ in 0..3 {
        let insn = lift(env, addr).ok()?;
        for op in &insn.ops {
            if strict {
                let unset = op.ins.iter().filter(|vn| is_reg(vn)).any(|vn| {
                    !written.iter().any(|&(o, n)| vn.offset >= o && vn.offset + u64::from(vn.size) <= o + u64::from(n))
                });
                if unset {
                    return None;
                }
                if let Some(out) = op.out.as_ref().filter(|o| is_reg(o)) {
                    written.push((out.offset, out.size));
                }
            }
            match op.opc {
                OpCode::CPUI_BRANCH => {
                    let t = op.ins.first()?;
                    if space(t).ok()?.get_type() == spacetype::IPTR_CONSTANT {
                        return None;
                    }
                    return Some(t.offset);
                }
                OpCode::CPUI_BRANCHIND => return Some(read(&mut state, op.ins.first()?).ok()? as u64),
                OpCode::CPUI_LOAD => {
                    let spc = mem_space(env, op.ins.first()?).ok()?;
                    let ptr = read(&mut state, op.ins.get(1)?).ok()? as u64;
                    let out = op.out.as_ref().filter(|o| !writes_argument(env, o) && (!strict || hidden(env, o)))?;
                    let v = state.read(&spc, spc.wrap_offset(ptr), out.size).ok()?;
                    write(&mut state, out, v).ok()?;
                }
                OpCode::CPUI_STORE if strict => return None,
                OpCode::CPUI_STORE => {
                    let spc = mem_space(env, op.ins.first()?).ok()?;
                    let ptr = read(&mut state, op.ins.get(1)?).ok()? as u64;
                    let val = op.ins.get(2)?;
                    if spc.get_index() != env.ram.get_index() || !env.in_stack(ptr, u64::from(val.size)) {
                        return None;
                    }
                    let v = read(&mut state, val).ok()?;
                    state.write(&spc, spc.wrap_offset(ptr), val.size, v).ok()?;
                }
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN => return None,
                opc => {
                    let out = op.out.as_ref().filter(|o| !writes_argument(env, o) && (!strict || hidden(env, o)))?;
                    let mut ins = Vec::with_capacity(op.ins.len());
                    for vn in &op.ins {
                        ins.push((read(&mut state, vn).ok()?, vn.size));
                    }
                    let v = eval(opc, out.size, &ins).ok()?;
                    write(&mut state, out, v).ok()?;
                }
            }
        }
        addr = env.code.wrap_offset(addr.wrapping_add(insn.len));
    }
    None
}
