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
    /// Register offsets a callee changed against the calling convention.
    pub convention: Vec<(u64, u64)>,
}

struct Machine<'e, 'p, 'c> {
    env: &'e Env<'p>,
    calls: &'c Calls,
    state: State<'e, 'p>,
    records: Vec<Record>,
    indirect: Vec<u64>,
    followed: Vec<(u64, u64)>,
    steps: u64,
    budget: u64,
    /// Register offsets a callee run for real changed although the calling
    /// convention says the call preserves them, by call address.
    convention: Vec<(u64, u64)>,
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

    /// Record one of the function's own calls for the comparison, with kuna's
    /// arguments as the machine holds them.
    fn record(&mut self, at: u64, target: Option<u64>, indirect: bool) -> Result<Vec<u128>, Halt> {
        let site = self.calls.site(at).clone();
        let mut args = Vec::with_capacity(site.args.len());
        for a in &site.args {
            args.push(match a {
                Some(a) => self.arg(a)?,
                None => 0,
            });
        }
        let r = Record { at, target, indirect, args: args.clone(), writes: self.state.writes(), effect: None };
        crate::calls::record(&mut self.records, r)?;
        Ok(args)
    }

    /// Record a call and give it the stub's effect. `true` when it does not
    /// return.
    fn call(&mut self, at: u64, target: Option<u64>, indirect: bool) -> Result<bool, Halt> {
        let args = self.record(at, target, indirect)?;
        if self.calls.site(at).noreturn {
            return Ok(true);
        }
        self.stub(at, &args)?;
        Ok(false)
    }

    /// The stub's effect: kill, let the arguments escape, pop the stack.
    fn stub(&mut self, at: u64, args: &[u128]) -> Result<(), Halt> {
        let site = self.calls.site(at).clone();
        let pop = site.extrapop.ok_or_else(|| Halt::Unsupported(format!("call at {at:#x} with unknown stack cleanup")))?;
        self.state.call(&site.killed, args);
        let sp = self.sp();
        let v = self.state.read(&sp.space, sp.off, sp.size)?;
        self.state.write(&sp.space, sp.off, sp.size, v.wrapping_add(pop as i64 as u128) & mask(sp.size))?;
        Ok(())
    }

    /// The stack pointer's value now.
    fn sp_value(&mut self) -> Result<u64, Halt> {
        let sp = self.sp();
        Ok(self.state.read(&sp.space, sp.off, sp.size)? as u64)
    }

    /// The function's call at `at` returned from a callee run for real: keep
    /// what it did for the decompiled side to replay. Writes into the callee's
    /// own frame, below the stack pointer at the call, are dead and dropped.
    fn close_effect(&mut self, at: u64, call_sp: u64) {
        let Some(events) = self.state.log.take() else { return };
        let env = self.env;
        let ram = env.ram.get_index();
        let events = events
            .into_iter()
            .filter(|ev| match ev {
                crate::state::Event::Write(sp, off, _) => !(*sp == ram && env.in_stack(*off, 1) && *off < call_sp),
                crate::state::Event::Call(..) => true,
            })
            .collect();
        let killed = self.calls.site(at).killed.clone();
        let regs = self.state.register_bytes(&killed);
        if let Some(r) = self.records.last_mut() {
            r.effect = Some(crate::state::Effect { events, regs });
        }
    }

    /// Run the callee of the function's call at `at` for real and keep its
    /// effect for the decompiled side. `false`, with nothing changed, when the
    /// callee does something the model cannot run: the call then gets the stub.
    fn real(&mut self, at: u64, target: u64) -> Result<bool, Halt> {
        let saved = self.state.clone();
        let (records, steps, budget) = (self.records.len(), self.steps, self.budget);
        let call_sp = self.sp_value()?;
        self.state.log = Some(Vec::new());
        self.budget = budget.min(steps + CALLEE_STEPS);
        let ran = exec(self, target, Mode::Opaque, 1);
        self.budget = budget;
        match ran {
            Ok(Exit::Return) => {
                let site = self.calls.site(at);
                let mut keep = site.killed.clone();
                keep.push(self.env.sp);
                for o in self.state.preserve_registers(&saved, &keep) {
                    if !self.convention.contains(&(at, o)) {
                        self.convention.push((at, o));
                    }
                }
                self.close_effect(at, call_sp);
                Ok(true)
            }
            _ => {
                self.state = saved;
                self.records.truncate(records);
                Ok(false)
            }
        }
    }

    fn finish(self, noreturn: bool) -> Outcome<'e, 'p> {
        Outcome {
            state: self.state,
            indirect: self.indirect,
            calls: self.records,
            noreturn,
            followed: self.followed,
            convention: self.convention,
        }
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

/// Steps a callee run for real may take before the call falls back to a stub.
const CALLEE_STEPS: u64 = 50_000;

/// Calls nested inside a callee run for real.
const MAX_DEPTH: u32 = 32;

/// How an execution ended.
enum Exit {
    Return,
    NoReturn,
}

/// What is executing: the function itself, a callee kuna inlined or dropped
/// (its calls are kuna's too), or a callee run for real that kuna only calls.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Top,
    Inline,
    Opaque,
}

/// Execute from `entry` until a RETURN. `top` is the function itself: only its
/// calls are recorded and compared, and only it has tail calls and an exit
/// through the random return address. Anywhere else is a callee run for real.
fn exec(m: &mut Machine, entry: u64, mode: Mode, depth: u32) -> Result<Exit, Halt> {
    let top = mode == Mode::Top;
    if depth > MAX_DEPTH {
        return Err(Halt::Unsupported("calls nested too deep".into()));
    }
    let env = m.env;
    let calls = m.calls;
    let mut addr = entry;
    let mut insn = lift(env, addr)?;
    let mut i = 0usize;
    let mut mode_switch: Option<String> = None;
    let mut called_here = false;
    loop {
        if i >= insn.ops.len() {
            addr = env.code.wrap_offset(addr.wrapping_add(insn.len));
            insn = lift(env, addr)?;
            i = 0;
            m.state.end_instruction();
            continue;
        }
        m.steps += 1;
        if m.steps > m.budget {
            return Err(Halt::Budget);
        }
        if i == 0 {
            called_here = false;
        }
        let cur = Rc::clone(&insn);
        let op = &cur.ops[i];
        let arg = |k: usize| op.ins.get(k).ok_or_else(|| Halt::Harness(format!("{} lacks input {k}", get_opname(op.opc))));
        let tail = |at: u64, t: u64| top && !called_here && calls.tail(at, t);
        match op.opc {
            OpCode::CPUI_BRANCH => {
                let t = arg(0)?;
                let relative = space(t)?.get_type() == spacetype::IPTR_CONSTANT;
                if !relative && tail(addr, t.offset) {
                    return Ok(if m.call(addr, Some(t.offset), false)? { Exit::NoReturn } else { Exit::Return });
                }
                if let Some(name) = mode_switch.take().filter(|_| !relative) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
            }
            OpCode::CPUI_CBRANCH => {
                if read(&mut m.state, arg(1)?)? != 0 {
                    let t = arg(0)?;
                    if space(t)?.get_type() != spacetype::IPTR_CONSTANT && tail(addr, t.offset) {
                        return Ok(if m.call(addr, Some(t.offset), false)? { Exit::NoReturn } else { Exit::Return });
                    }
                    branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
                } else {
                    i += 1;
                }
            }
            OpCode::CPUI_BRANCHIND => {
                let target = read(&mut m.state, arg(0)?)? as u64;
                if tail(addr, target) {
                    return Ok(if m.call(addr, Some(target), true)? { Exit::NoReturn } else { Exit::Return });
                }
                if top && !env.code_mapped(target) {
                    return Ok(Exit::Return);
                }
                if top {
                    m.indirect.push(target);
                }
                if let Some(name) = mode_switch.take() {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                addr = target;
                insn = lift(env, addr)?;
                i = 0;
                m.state.end_instruction();
            }
            OpCode::CPUI_RETURN => return Ok(Exit::Return),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let target = if op.opc == OpCode::CPUI_CALL {
                    arg(0)?.offset
                } else {
                    read(&mut m.state, arg(0)?)? as u64
                };
                let indirect = op.opc == OpCode::CPUI_CALLIND;
                let own = mode != Mode::Opaque && calls.known(addr);
                let in_image = env.code_mapped(target);
                if own {
                    if calls.site(addr).noreturn {
                        m.record(addr, Some(target), indirect)?;
                        return Ok(Exit::NoReturn);
                    }
                    let args = m.record(addr, Some(target), indirect)?;
                    if !in_image || !m.real(addr, target)? {
                        m.stub(addr, &args)?;
                    }
                } else if in_image {
                    if top && !m.followed.contains(&(addr, target)) {
                        m.followed.push((addr, target));
                    }
                    let inner = if mode == Mode::Opaque { Mode::Opaque } else { Mode::Inline };
                    if let Exit::NoReturn = exec(m, target, inner, depth + 1)? {
                        return Ok(Exit::NoReturn);
                    }
                } else if mode != Mode::Opaque {
                    if m.call(addr, Some(target), indirect)? {
                        return Ok(Exit::NoReturn);
                    }
                } else {
                    m.stub(addr, &[])?;
                }
                called_here = true;
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

/// Run the function at `entry` from the run's initial state.
pub fn run<'e, 'p>(env: &'e Env<'p>, run: u32, entry: u64, budget: u64, calls: &Calls) -> Result<Outcome<'e, 'p>, Halt> {
    let mut m = Machine {
        env,
        calls,
        state: State::new(env, run),
        records: Vec::new(),
        indirect: Vec::new(),
        followed: Vec::new(),
        steps: 0,
        budget,
        convention: Vec::new(),
    };
    let exit = exec(&mut m, entry, Mode::Top, 0)?;
    Ok(m.finish(matches!(exit, Exit::NoReturn)))
}
