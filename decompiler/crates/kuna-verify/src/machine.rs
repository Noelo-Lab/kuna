//! The machine side: the function's own instructions, lifted by the program's
//! SLEIGH translator one at a time and executed op by op from its entry until
//! it returns.
//!
//! A `RETURN`, or an indirect branch to an address the image does not map (the
//! random return address), ends the run. Calls end it as not-leaf, which is
//! outside slice C1.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::state::{Env, State};
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

/// How the machine run ended: its final state, and every indirect branch target
/// it took inside the image.
pub struct Outcome<'e, 'p> {
    pub state: State<'e, 'p>,
    pub indirect: Vec<u64>,
}

/// Run the function at `entry` from the run's initial state.
pub fn run<'e, 'p>(env: &'e Env<'p>, run: u32, entry: u64, budget: u64) -> Result<Outcome<'e, 'p>, Halt> {
    let mut state = State::new(env, run);
    let mut indirect = Vec::new();
    let mut addr = entry;
    let mut insn = lift(env, addr)?;
    let mut i = 0usize;
    let mut steps = 0u64;
    let mut mode_switch: Option<String> = None;
    loop {
        if i >= insn.ops.len() {
            addr = env.code.wrap_offset(addr.wrapping_add(insn.len));
            insn = lift(env, addr)?;
            i = 0;
            state.end_instruction();
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
                if let Some(name) = mode_switch.take().filter(|_| space(t).is_ok_and(|s| s.get_type() != spacetype::IPTR_CONSTANT)) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                branch(env, &mut state, t, &mut i, &mut addr, &mut insn)?;
            }
            OpCode::CPUI_CBRANCH => {
                if read(&mut state, arg(1)?)? != 0 {
                    let t = arg(0)?;
                    branch(env, &mut state, t, &mut i, &mut addr, &mut insn)?;
                } else {
                    i += 1;
                }
            }
            OpCode::CPUI_BRANCHIND => {
                let target = read(&mut state, arg(0)?)? as u64;
                if !env.code_mapped(target) {
                    return Ok(Outcome { state, indirect });
                }
                indirect.push(target);
                if let Some(name) = mode_switch.take() {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                addr = target;
                insn = lift(env, addr)?;
                i = 0;
                state.end_instruction();
            }
            OpCode::CPUI_RETURN => return Ok(Outcome { state, indirect }),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                return Err(Halt::NotLeaf(format!("call at {addr:#x}")));
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
                let ptr = read(&mut state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let out = op.out.as_ref().ok_or_else(|| Halt::Harness("LOAD without output".into()))?;
                env.check_load(&spc, at, out.size)?;
                let v = state.read(&spc, at, out.size)?;
                write(&mut state, out, v)?;
                i += 1;
            }
            OpCode::CPUI_STORE => {
                let spc = mem_space(env, arg(0)?)?;
                let ptr = read(&mut state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let val = arg(2)?;
                env.check_store(&spc, at, val.size)?;
                let v = read(&mut state, val)?;
                state.write(&spc, at, val.size, v)?;
                i += 1;
            }
            opc => {
                let out = op.out.as_ref().ok_or_else(|| Halt::Unsupported(get_opname(opc).to_string()))?;
                let mut ins = Vec::with_capacity(op.ins.len());
                for vn in &op.ins {
                    ins.push((read(&mut state, vn)?, vn.size));
                }
                let v = eval(opc, out.size, &ins)?;
                write(&mut state, out, v)?;
                i += 1;
            }
        }
    }
}
