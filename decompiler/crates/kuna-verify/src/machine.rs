//! The machine side: the function's own instructions, lifted by the program's
//! SLEIGH translator one at a time and executed op by op from its entry until
//! it returns.
//!
//! A `RETURN`, or an indirect branch to an address the image does not map (the
//! random return address), ends the run. A call runs a stub instead of the
//! callee (`calls.rs`): it is recorded with its arguments and the memory the
//! callee could read, then has its effect. A branch out of the function from an
//! instruction kuna treats as a call is a tail call: the stub runs, and the
//! function returns. Where kuna has no call, a callee in the image is run for
//! real, and its RETURN comes back to the caller.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::calls::{Calls, Record};
use crate::state::{Env, State};
use crate::storage::Storage;
use crate::value::{eval, mask, Halt};

pub struct RawOp {
    pub opc: OpCode,
    pub out: Option<VarnodeData>,
    pub ins: Vec<VarnodeData>,
}

pub struct Insn {
    pub len: u64,
    pub ops: Vec<RawOp>,
}

struct Collect(Vec<RawOp>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.0.push(RawOp { opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

fn lift(env: &Env, addr: u64) -> Result<Rc<Insn>, Halt> {
    if let Some(i) = env.insns.borrow().get(&addr) {
        return Ok(Rc::clone(i));
    }
    let mut emit = Collect(Vec::new());
    let len = env
        .prog
        .arch()
        .translate()
        .one_instruction(&mut emit, &Address::new(Rc::clone(&env.code), addr))
        .map_err(|e| Halt::Unsupported(format!("lift at {addr:#x}: {}", e.explain())))?;
    let insn = Rc::new(Insn { len: len.max(1) as u64, ops: emit.0 });
    env.insns.borrow_mut().insert(addr, Rc::clone(&insn));
    Ok(insn)
}

fn space(vn: &VarnodeData) -> Result<&Rc<AddrSpace>, Halt> {
    vn.space.as_ref().ok_or_else(|| Halt::Harness("varnode without a space".into()))
}

fn read(state: &mut State, vn: &VarnodeData) -> Result<u128, Halt> {
    let spc = space(vn)?;
    if spc.get_type() == spacetype::IPTR_CONSTANT {
        return Ok(u128::from(vn.offset) & mask(vn.size));
    }
    state.read(spc, vn.offset, vn.size)
}

fn write(state: &mut State, vn: &VarnodeData, val: u128) -> Result<(), Halt> {
    state.write(space(vn)?, vn.offset, vn.size, val)
}

fn mem_space(env: &Env, vn: &VarnodeData) -> Result<Rc<AddrSpace>, Halt> {
    vn.get_space_from_const(env.prog.arch().manage())
        .ok_or_else(|| Halt::Harness("bad LOAD/STORE space id".into()))
}

/// User ops that only switch the instruction set for the next instruction. Before
/// the function's exit they change nothing either side can observe; before a
/// branch inside the function they would change the decode, which is refused.
pub const EXIT_ONLY_USEROPS: &[&str] = &["setISAMode"];

/// How the machine run ended.
pub struct Outcome<'e, 'p> {
    pub state: State<'e, 'p>,
    /// Every indirect branch target taken inside the image.
    pub indirect: Vec<u64>,
    pub calls: Vec<Record>,
    /// The run ended in a call kuna says does not return.
    pub noreturn: bool,
    /// Calls kuna does not make that the machine ran for real: call address
    /// and callee.
    pub followed: Vec<(u64, u64)>,
}

struct Machine<'e, 'p, 'c> {
    env: &'e Env<'p>,
    calls: &'c Calls,
    state: State<'e, 'p>,
    records: Vec<Record>,
    indirect: Vec<u64>,
    followed: Vec<(u64, u64)>,
}

impl<'e, 'p> Machine<'e, 'p, '_> {
    fn sp(&self) -> Storage {
        Storage::new(&self.env.register, self.env.sp.0, self.env.sp.1)
    }

    /// An argument's value at the call.
    fn arg(&mut self, a: &crate::calls::Arg) -> Result<u128, Halt> {
        let st = &a.storage;
        if a.callee_frame && st.space.get_type() == spacetype::IPTR_SPACEBASE {
            let sp = self.sp();
            let base = self.state.read(&sp.space, sp.off, sp.size)? as u64;
            let ram = Rc::clone(&self.env.ram);
            return self.state.read(&ram, ram.wrap_offset(base.wrapping_add(st.off)), st.size);
        }
        self.state.read_storage(&st.space, st.off, st.size)
    }

    /// Record a call and give it its effect. `true` when it does not return.
    fn call(&mut self, at: u64, target: Option<u64>, indirect: bool) -> Result<bool, Halt> {
        let site = self.calls.site(at).clone();
        let mut args = Vec::with_capacity(site.args.len());
        for a in &site.args {
            args.push(match a {
                Some(a) => self.arg(a)?,
                None => 0,
            });
        }
        crate::calls::record(&mut self.records, Record { at, target, indirect, args: args.clone(), writes: self.state.writes() })?;
        if site.noreturn {
            return Ok(true);
        }
        let pop = site.extrapop.ok_or_else(|| Halt::Unsupported(format!("call at {at:#x} with unknown stack cleanup")))?;
        self.state.call(&site.killed, &args);
        let sp = self.sp();
        let v = self.state.read(&sp.space, sp.off, sp.size)?;
        self.state.write(&sp.space, sp.off, sp.size, v.wrapping_add(pop as i64 as u128) & mask(sp.size))?;
        Ok(false)
    }

    fn finish(self, noreturn: bool) -> Outcome<'e, 'p> {
        Outcome { state: self.state, indirect: self.indirect, calls: self.records, noreturn, followed: self.followed }
    }
}

fn branch(
    env: &Env,
    state: &mut State,
    target: &VarnodeData,
    i: &mut usize,
    addr: &mut u64,
    insn: &mut Rc<Insn>,
) -> Result<(), Halt> {
    if space(target)?.get_type() == spacetype::IPTR_CONSTANT {
        *i = (target.offset as u32).wrapping_add(*i as u32) as i32 as usize;
        if *i > insn.ops.len() {
            return Err(Halt::Harness("bad intra-instruction branch".into()));
        }
    } else {
        *addr = target.offset;
        *insn = lift(env, *addr)?;
        *i = 0;
        state.end_instruction();
    }
    Ok(())
}

/// Run the function at `entry` from the run's initial state.
pub fn run<'e, 'p>(env: &'e Env<'p>, run: u32, entry: u64, budget: u64, calls: &Calls) -> Result<Outcome<'e, 'p>, Halt> {
    let mut m = Machine {
        env,
        calls,
        state: State::new(env, run),
        records: Vec::new(),
        indirect: Vec::new(),
        followed: Vec::new(),
    };
    let mut addr = entry;
    let mut insn = lift(env, addr)?;
    let mut i = 0usize;
    let mut steps = 0u64;
    let mut mode_switch: Option<String> = None;
    let mut depth = 0u32;
    loop {
        if i >= insn.ops.len() {
            addr = env.code.wrap_offset(addr.wrapping_add(insn.len));
            insn = lift(env, addr)?;
            i = 0;
            m.state.end_instruction();
            continue;
        }
        steps += 1;
        if steps > budget {
            return Err(Halt::Budget);
        }
        let cur = Rc::clone(&insn);
        let op = &cur.ops[i];
        let arg = |k: usize| op.ins.get(k).ok_or_else(|| Halt::Harness(format!("{} lacks input {k}", get_opname(op.opc))));
        match op.opc {
            OpCode::CPUI_BRANCH => {
                let t = arg(0)?;
                let relative = space(t)?.get_type() == spacetype::IPTR_CONSTANT;
                if !relative && depth == 0 && calls.tail(addr, t.offset) {
                    let noreturn = m.call(addr, Some(t.offset), false)?;
                    return Ok(m.finish(noreturn));
                }
                if let Some(name) = mode_switch.take().filter(|_| !relative) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
            }
            OpCode::CPUI_CBRANCH => {
                if read(&mut m.state, arg(1)?)? != 0 {
                    let t = arg(0)?;
                    if space(t)?.get_type() != spacetype::IPTR_CONSTANT && depth == 0 && calls.tail(addr, t.offset) {
                        let noreturn = m.call(addr, Some(t.offset), false)?;
                        return Ok(m.finish(noreturn));
                    }
                    branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
                } else {
                    i += 1;
                }
            }
            OpCode::CPUI_BRANCHIND => {
                let target = read(&mut m.state, arg(0)?)? as u64;
                if depth == 0 && calls.tail(addr, target) {
                    let noreturn = m.call(addr, Some(target), true)?;
                    return Ok(m.finish(noreturn));
                }
                if !env.code_mapped(target) && depth == 0 {
                    return Ok(m.finish(false));
                }
                m.indirect.push(target);
                if let Some(name) = mode_switch.take() {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                addr = target;
                insn = lift(env, addr)?;
                i = 0;
                m.state.end_instruction();
            }
            OpCode::CPUI_RETURN if depth == 0 => return Ok(m.finish(false)),
            OpCode::CPUI_RETURN => {
                depth -= 1;
                addr = read(&mut m.state, arg(0)?)? as u64;
                insn = lift(env, addr)?;
                i = 0;
                m.state.end_instruction();
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let target = if op.opc == OpCode::CPUI_CALL {
                    arg(0)?.offset
                } else {
                    read(&mut m.state, arg(0)?)? as u64
                };
                if !calls.known(addr) && env.code_mapped(target) {
                    if depth == 0 && !m.followed.contains(&(addr, target)) {
                        m.followed.push((addr, target));
                    }
                    depth += 1;
                    addr = target;
                    insn = lift(env, addr)?;
                    i = 0;
                    m.state.end_instruction();
                    continue;
                }
                if m.call(addr, Some(target), op.opc == OpCode::CPUI_CALLIND)? {
                    return Ok(m.finish(true));
                }
                i += 1;
            }
            OpCode::CPUI_CALLOTHER => {
                let name = crate::program::userop_name(env.prog, arg(0)?.offset);
                if !(op.out.is_none() && EXIT_ONLY_USEROPS.contains(&name.as_str())) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} at {addr:#x}")));
                }
                mode_switch = Some(name);
                i += 1;
            }
            OpCode::CPUI_LOAD => {
                let spc = mem_space(env, arg(0)?)?;
                let ptr = read(&mut m.state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let out = op.out.as_ref().ok_or_else(|| Halt::Harness("LOAD without output".into()))?;
                let v = m.state.read(&spc, at, out.size)?;
                write(&mut m.state, out, v)?;
                i += 1;
            }
            OpCode::CPUI_STORE => {
                let spc = mem_space(env, arg(0)?)?;
                let ptr = read(&mut m.state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let val = arg(2)?;
                let v = read(&mut m.state, val)?;
                m.state.write(&spc, at, val.size, v)?;
                i += 1;
            }
            opc => {
                let out = op.out.as_ref().ok_or_else(|| Halt::Unsupported(get_opname(opc).to_string()))?;
                let mut ins = Vec::with_capacity(op.ins.len());
                for vn in &op.ins {
                    ins.push((read(&mut m.state, vn)?, vn.size));
                }
                let v = eval(opc, out.size, &ins)?;
                write(&mut m.state, out, v)?;
                i += 1;
            }
        }
    }
}
