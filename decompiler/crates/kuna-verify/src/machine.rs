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

pub(crate) fn lift(env: &Env, addr: u64) -> Result<Rc<Insn>, Halt> {
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

fn follow_thunks(env: &Env, target: u64) -> u64 {
    follow(env, target, false)
}

fn follow(env: &Env, target: u64, strict: bool) -> u64 {
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
fn writes_argument(env: &Env, out: &VarnodeData) -> bool {
    space(out).is_ok_and(|s| s.get_index() == env.register.get_index())
        && env.arg_regs.iter().any(|&(o, n)| out.offset < o + u64::from(n) && o < out.offset + u64::from(out.size))
}

/// Whether a `strict` hop may write `out`: a temporary or a veneer register.
fn hidden(env: &Env, out: &VarnodeData) -> bool {
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
fn thunk_hop(env: &Env, at: u64, run: u32, strict: bool) -> Option<u64> {
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

fn mem_space(env: &Env, vn: &VarnodeData) -> Result<Rc<AddrSpace>, Halt> {
    vn.get_space_from_const(env.prog.arch().manage())
        .ok_or_else(|| Halt::Harness("bad LOAD/STORE space id".into()))
}

/// User ops that only switch the instruction set for the next instruction. Before
/// the function's exit they change nothing either side can observe; before a
/// branch inside the function they would change the decode, which is refused
/// unless the target already decodes in the mode switched to.
pub const EXIT_ONLY_USEROPS: &[&str] = &["setISAMode"];

/// User ops with no effect on one thread's values: memory barriers, x86's
/// `lock` prefix, ARM's exclusive monitor, which one thread always holds, and
/// Cortex-M's interrupt masks and base priority, which no read sees changed.
pub const NO_EFFECT_USEROPS: &[&str] = &[
    "LOCK",
    "UNLOCK",
    "DataMemoryBarrier",
    "DataSynchronizationBarrier",
    "InstructionSynchronizationBarrier",
    "ExclusiveAccess",
    "ClearExclusiveLocal",
    "setBasePriority",
    "enableIRQinterrupts",
    "disableIRQinterrupts",
    "enableFIQinterrupts",
    "disableFIQinterrupts",
    "setThreadModePrivileged",
];

/// User ops that answer a question about the processor's state with a value
/// both sides read alike: ARM's `strex` asks whether this thread still holds
/// the exclusive monitor (one thread always does), and Cortex-M's `mrs` reads
/// a system register. The code runs privileged; the masks, base priority and
/// exception number read 0, and writes to them (`NO_EFFECT_USEROPS`) change
/// nothing either side reads back.
pub const CONSTANT_USEROPS: &[(&str, u128)] = &[
    ("hasExclusiveAccess", 1),
    ("isCurrentModePrivileged", 1),
    ("isThreadModePrivileged", 1),
    ("getBasePriority", 0),
    ("getCurrentExceptionNumber", 0),
    ("isIRQinterruptsEnabled", 0),
    ("isFIQinterruptsEnabled", 0),
];

pub fn constant_userop(name: &str) -> Option<u128> {
    CONSTANT_USEROPS.iter().find(|(n, _)| *n == name).map(|&(_, v)| v)
}

/// Whether code at `target` decodes in instruction-set mode `mode` (ARM's
/// `TMode`, 1 for Thumb): a switch to the mode the target is in anyway, as
/// every `bx` on a Thumb-only core is, changes nothing.
fn decodes_in(env: &Env, target: u64, mode: Option<u128>) -> bool {
    let Some(mode) = mode else { return false };
    let at = Address::new(Rc::clone(&env.code), target & !1);
    env.prog.arch().with_context_db(|db| {
        let Ok(var) = db.get_variable(b"TMode") else { return false };
        let word = var.get_word() as usize;
        let mask = var.get_mask() << var.get_shift();
        db.get_context(&at).get(word).is_some_and(|w| u128::from((w & mask) >> var.get_shift()) == mode & 1)
    })
}

/// C5's view of one run: what expression of the inputs each written byte
/// holds, with the byte's concrete value when it was written, so a later
/// overwrite by code the view does not follow (a callee, a stub) drops it.
#[derive(Default)]
struct Shadow {
    bytes: std::collections::HashMap<(i32, u64), (Rc<crate::concolic::Expr>, u32, u8)>,
    path: crate::concolic::Path,
}

/// Branches a recorded path keeps.
const PATH_BRANCHES: usize = 256;

impl Machine<'_, '_, '_> {
    /// C5: the expression `vn` holds, given its concrete value; `None` when it
    /// depends on no input.
    fn sym_read(&mut self, vn: &VarnodeData, value: u128) -> Option<Rc<crate::concolic::Expr>> {
        use crate::concolic::Expr;
        self.shadow.as_ref()?;
        let spc = Rc::clone(vn.space.as_ref()?);
        if spc.get_type() == spacetype::IPTR_CONSTANT {
            return None;
        }
        let n = vn.size;
        let big = spc.is_big_endian();
        let mut pieces = Vec::with_capacity(n as usize);
        let mut symbolic = false;
        for i in 0..u64::from(n) {
            let at = spc.wrap_offset(vn.offset.wrapping_add(i));
            let shift = if big { u64::from(n) - 1 - i } else { i };
            let cb = (value >> (8 * shift)) as u8;
            let known = self.shadow.as_ref()?.bytes.get(&(spc.get_index(), at)).filter(|(_, _, conc)| *conc == cb).cloned();
            let piece = match known {
                Some((e, idx, _)) => Some(crate::concolic::byte(&e, idx)),
                None if self.state.untouched(&spc, at) => self.env.input_place(&spc, at).map(|place| {
                    let name: Rc<str> = format!("v{}_{at:x}", place.space).into();
                    if let Some(s) = self.shadow.as_mut() {
                        s.path.inputs.insert(Rc::clone(&name), place);
                    }
                    Rc::new(Expr::Var(name))
                }),
                None => None,
            };
            symbolic |= piece.is_some();
            pieces.push(piece.unwrap_or_else(|| Rc::new(Expr::Const(u128::from(cb), 8))));
        }
        if !symbolic {
            return None;
        }
        if !big {
            pieces.reverse();
        }
        Some(if pieces.len() == 1 { pieces.pop()? } else { Rc::new(Expr::Concat(pieces)) })
    }

    /// C5: `vn` now holds `value`, and the expression `e` when it has one.
    fn sym_write(&mut self, vn: &VarnodeData, value: u128, e: Option<Rc<crate::concolic::Expr>>) {
        let Some(shadow) = self.shadow.as_mut() else { return };
        let Some(spc) = vn.space.as_ref() else { return };
        let n = u64::from(vn.size);
        for i in 0..n {
            let at = spc.wrap_offset(vn.offset.wrapping_add(i));
            let shift = if spc.is_big_endian() { n - 1 - i } else { i };
            let key = (spc.get_index(), at);
            match &e {
                Some(e) => {
                    shadow.bytes.insert(key, (Rc::clone(e), shift as u32, (value >> (8 * shift)) as u8));
                }
                None => {
                    shadow.bytes.remove(&key);
                }
            }
        }
    }

    fn sym_branch(&mut self, at: u64, op: usize, cond: &VarnodeData, value: u128) {
        if self.shadow.as_ref().is_none_or(|s| s.path.branches.len() >= PATH_BRANCHES) {
            return;
        }
        if let Some(e) = self.sym_read(cond, value) {
            if let Some(s) = self.shadow.as_mut() {
                s.path.branches.push(crate::concolic::Branch { at, op: op as u32, cond: e, taken: value != 0 });
            }
        }
    }
}

/// How far past a call a callee may return: a delay slot (MIPS, SPARC) puts
/// the return address one instruction further.
const RETURN_SLACK: u64 = 8;

/// How far from the call's a callee's stack pointer may be when it returns
/// (`_chkstk`-style probes move it); further, it switched stacks.
const STACK_SWITCH: u64 = 1 << 20;

/// How the machine run ended.
pub struct Outcome<'e, 'p> {
    pub state: State<'e, 'p>,
    /// Every indirect branch target taken inside the image.
    pub indirect: Vec<u64>,
    pub calls: Vec<Record>,
    /// The run ended in a call kuna says does not return.
    pub noreturn: bool,
    /// It ended in one only kuna says does not return: no known name, no
    /// layout evidence, and no in-image callee that ran to its return.
    pub unverified_noreturn: bool,
    /// Calls kuna does not make that the machine ran for real: call address
    /// and callee.
    pub followed: Vec<(u64, u64)>,
    /// Register offsets a callee changed against the calling convention.
    pub convention: Vec<(u64, u64)>,
    /// An in-image callee given a frame object got the stub's guessed effect.
    pub guessed: bool,
    /// System calls and traps, in order.
    pub events: Vec<crate::calls::UserEvent>,
    /// C5: the run's path, when `Env::record_path` was on.
    pub path: Option<crate::concolic::Path>,
}

struct Machine<'e, 'p, 'c> {
    env: &'e Env<'p>,
    calls: &'c Calls,
    /// Stubs given so far: a callee that ran to its return through one may
    /// only have returned because the stub did.
    stubs: u32,
    unverified_noreturn: bool,
    /// A callee run for real reached a call that does not return by its
    /// definition.
    proved_noreturn: bool,
    /// A callee runs to test kuna's no-return claim: a call it makes through
    /// a PLT stub that leads out of the code is a call out of the image.
    checking_noreturn: bool,
    state: State<'e, 'p>,
    records: Vec<Record>,
    indirect: Vec<u64>,
    followed: Vec<(u64, u64)>,
    steps: u64,
    budget: u64,
    /// Steps left for callees run for real; theirs do not count against the
    /// function's budget, which the decompiled side never spends on them.
    callee_pool: u64,
    in_callee: bool,
    /// Where each enclosing call returns, innermost last.
    returns: Vec<u64>,
    /// System calls and traps, in order.
    events: Vec<crate::calls::UserEvent>,
    /// Argument-register bytes to give one call's callee instead of the
    /// machine's own: kuna's, to see whether the callee cares.
    with_args: Option<&'c WithArgs>,
    /// An in-image callee given a frame object fell back to the stub.
    guessed: bool,
    /// Temporaries this instruction copied a callee-saved entry value to:
    /// (temporary offset, register offset, size).
    copies: Vec<(u64, u64, u32)>,
    /// Temporaries this instruction loaded a spilled entry value into, with
    /// the register it came from, until a copy restores it.
    restoring: Vec<(u64, u64)>,
    /// Register offsets a callee run for real changed although the calling
    /// convention says the call preserves them, by call address.
    convention: Vec<(u64, u64)>,
    /// C5: each byte's expression over the inputs, and the path so far.
    shadow: Option<Shadow>,
}

impl<'e, 'p> Machine<'e, 'p, '_> {
    fn sp(&self) -> Storage {
        Storage::new(&self.env.register, self.env.sp.0, self.env.sp.1)
    }

    /// An argument's value at the call.
    fn arg(&mut self, a: &crate::calls::Arg) -> Result<u128, Halt> {
        if !a.callee_frame {
            return self.state.read_storage(&a.storage.space, a.storage.off, a.storage.size);
        }
        let mut acc = 0u128;
        for p in a.storage.pieces(self.env.prog) {
            let v = if p.space.get_type() == spacetype::IPTR_SPACEBASE {
                let base = self.sp_value()?;
                let ram = Rc::clone(&self.env.ram);
                self.state.read(&ram, ram.wrap_offset(base.wrapping_add(p.off)), p.size)?
            } else {
                self.state.read_storage(&p.space, p.off, p.size)?
            };
            acc = if p.size >= 16 { v } else { (acc << (p.size * 8)) | v };
        }
        Ok(acc & mask(a.storage.size))
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
        let mut given: Vec<u128> = args.iter().zip(&site.args).filter(|(_, a)| a.is_some()).map(|(&v, _)| v).collect();
        for st in &self.calls.pointer_regs {
            given.push(self.state.read_storage(&st.space, st.off, st.size)?);
        }
        let r = Record { at, target, indirect, args: args.clone(), writes: self.state.writes_at_call(&given, target.is_some_and(|t| self.env.code_mapped(t))), effect: None, frame_reads: None, live_in: None };
        crate::calls::record(&mut self.records, r)?;
        if let Some(o) = self.with_args.filter(|o| o.call == Some(self.records.len() - 1)) {
            let register = std::rc::Rc::clone(&self.env.register);
            for &(off, b) in &o.bytes {
                self.state.write(&register, off, 1, u128::from(b))?;
            }
        }
        Ok(args)
    }

    /// One of the function's tail calls: record it, then run an in-image
    /// callee for real or give it the stub. The function then returns.
    /// Whether `target` does not return by its definition (`known_noreturn`),
    /// reading `error`'s `status` from the convention's first argument register.
    fn known_noreturn(&mut self, target: u64) -> Result<bool, Halt> {
        let prog = self.env.prog;
        let Some(name) = prog.function_named_at(target).or_else(|| prog.function_named_at(through_thunks(self.env, target))) else {
            return Ok(false);
        };
        let plain = name.split('@').next().unwrap_or(&name).trim_start_matches('_');
        let first = match self.calls.int_regs.first().filter(|_| matches!(plain, "error" | "error_at_line")) {
            Some(r) => {
                let live = self.state.live.take();
                let v = self.state.read_storage(&r.space, r.off, r.size);
                self.state.live = live;
                Some(v?)
            }
            None => None,
        };
        Ok(crate::calls::known_noreturn(&name, first))
    }

    /// For a call kuna says does not return: `None` when the machine stops
    /// there too, or the callee's exit when it ran for real and returned
    /// without a stub on the way, so kuna's claim is false. The machine stops
    /// on evidence of its own (a known name, the layout) or, with none, on
    /// kuna's word, which the run records.
    fn stops_at(&mut self, at: u64, target: u64, args: &[u128]) -> Result<Option<Exit>, Halt> {
        let site = self.calls.site(at);
        if site.laid_out_noreturn {
            return Ok(None);
        }
        let name = self
            .env
            .prog
            .function_named_at(target)
            .or_else(|| self.env.prog.function_named_at(through_thunks(self.env, target)))
            .unwrap_or_default();
        if crate::calls::known_noreturn(&name, args.first().copied()) {
            return Ok(None);
        }
        if self.env.code_mapped(target & !1) && !leads_out(self.env, target) && !self.in_callee {
            let saved = self.state.clone();
            let (records, stubs) = (self.records.len(), self.stubs);
            self.proved_noreturn = false;
            self.checking_noreturn = true;
            let ran = self.real(at, target);
            self.checking_noreturn = false;
            let ran = ran?;
            if let Some(e @ Exit::Return) = ran.filter(|_| self.stubs == stubs) {
                return Ok(Some(e));
            }
            self.state = saved;
            self.records.truncate(records);
            if let Some(r) = self.records.last_mut() {
                r.effect = None;
            }
            if std::mem::take(&mut self.proved_noreturn) {
                return Ok(None);
            }
        }
        self.unverified_noreturn = true;
        Ok(None)
    }

    fn tail_call(&mut self, at: u64, target: u64, indirect: bool) -> Result<Exit, Halt> {
        let args = self.record(at, Some(target), indirect)?;
        if self.calls.site(at).noreturn {
            return Ok(match self.stops_at(at, target, &args)? {
                Some(_) => Exit::Return,
                None => Exit::NoReturn,
            });
        }
        let in_image = self.env.code_mapped(target);
        if in_image {
            match self.real(at, target)? {
                Some(e @ Exit::Unwind(_)) => return Ok(e),
                Some(_) => return Ok(Exit::Return),
                None => {}
            }
        }
        self.guess_if_frame(in_image, &args);
        self.stub(at, &args)?;
        Ok(Exit::Return)
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

    /// An in-image callee the model could not run gets the stub, whose effect
    /// on a frame object it was given is a guess where kuna may know better
    /// (a callee that only reads it): differences after it are doubtful.
    fn guess_if_frame(&mut self, in_image: bool, args: &[u128]) {
        if in_image && args.iter().any(|&a| self.env.in_stack(a as u64, 1) && self.env.stack_object(a as u64).is_some()) {
            self.guessed = true;
        }
    }

    /// The stub's effect: kill, let the arguments escape, pop the stack.
    fn stub(&mut self, at: u64, args: &[u128]) -> Result<(), Halt> {
        self.stubs += 1;
        let site = self.calls.site(at).clone();
        let pop = site.extrapop.ok_or_else(|| Halt::Unsupported(format!("call at {at:#x} with unknown stack cleanup")))?;
        self.state.call(&site.killed, args, site.ret_ext.as_ref());
        let sp = self.sp();
        let v = self.state.read(&sp.space, sp.off, sp.size)?;
        let after = v.wrapping_add(pop as i64 as u128) & mask(sp.size);
        self.state.write(&sp.space, sp.off, sp.size, after)?;
        self.state.forget_below(after as u64);
        Ok(())
    }

    /// The callee-saved register whose entry value `vn` holds: the register
    /// itself, or a temporary this instruction copied it to.
    fn entry_value_of(&self, vn: &VarnodeData) -> Option<u64> {
        let env = self.env;
        let sp = space(vn).ok()?;
        if sp.get_index() != env.register.get_index() {
            return self.copies.iter().find(|&&(u, _, n)| u == vn.offset && n == vn.size).map(|&(_, r, _)| r);
        }
        let saved = vn.offset != env.sp.0
            && !self.calls.default.killed.iter().any(|&(o, n)| vn.offset < o + u64::from(n) && o < vn.offset + u64::from(vn.size))
            && !self.state.wrote(&env.register, vn.offset, vn.size);
        saved.then_some(vn.offset)
    }

    /// A load of spilled entry values reads them as a value unless it restores
    /// the register they came from, directly or through a temporary this
    /// instruction then copies there.
    fn note_load(&mut self, at: u64, out: &VarnodeData) {
        let regs = self.state.saved_in(at, out.size);
        let [reg] = regs[..] else {
            self.state.read_saved |= !regs.is_empty();
            return;
        };
        match space(out).map(|s| s.get_type()) {
            Ok(spacetype::IPTR_INTERNAL) => self.restoring.push((out.offset, reg)),
            _ => self.state.read_saved |= out.offset != reg,
        }
    }

    /// A stack store of a callee-saved register's entry value: the register.
    fn spill_of_entry(&self, spc: &AddrSpace, at: u64, val: &VarnodeData) -> Option<u64> {
        let env = self.env;
        if spc.get_index() != env.ram.get_index() || !env.in_stack(at, u64::from(val.size)) {
            return None;
        }
        self.entry_value_of(val)
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
    /// effect for the decompiled side. `None`, with nothing changed, when the
    /// callee does something the model cannot run or outruns its budget: the
    /// call then gets the stub.
    fn real(&mut self, at: u64, target: u64) -> Result<Option<Exit>, Halt> {
        let outer = !self.in_callee;
        let limit = if outer { CALLEE_STEPS.min(self.callee_pool) } else { CALLEE_STEPS };
        if limit == 0 {
            return Ok(None);
        }
        let saved = self.state.clone();
        let (records, steps, budget) = (self.records.len(), self.steps, self.budget);
        let call_sp = self.sp_value()?;
        self.state.log = Some(Vec::new());
        if outer {
            self.state.frame_reads = Some(crate::state::FrameReads::default());
            self.state.live = Some(crate::state::LiveIn::new(self.calls.input_regs.clone()));
        }
        self.budget = if outer { steps + limit } else { budget.min(steps + limit) };
        self.in_callee = true;
        let back = lift(self.env, at).ok().map(|i| (at.wrapping_add(i.len), call_sp));
        let ran = exec(self, target, Mode::Opaque, 1, back);
        self.in_callee = !outer;
        let reads = if outer { self.state.frame_reads.take() } else { None };
        let live = if outer {
            self.state.live.take().map(|mut l| {
                l.instruction();
                l.read.into_iter().collect::<Vec<_>>()
            })
        } else {
            None
        };
        self.budget = budget;
        if outer {
            self.callee_pool -= (self.steps - steps).min(self.callee_pool);
            self.steps = steps;
        }
        match ran {
            Err(Halt::Budget) if !outer && self.steps > budget => Err(Halt::Budget),
            Ok(exit @ (Exit::Return | Exit::Leave | Exit::Unwind(_))) => {
                let site = self.calls.site(at);
                let mut keep = site.abi_killed.clone();
                keep.push(self.env.sp);
                for o in self.state.preserve_registers(&saved, &keep) {
                    if !self.convention.contains(&(at, o)) {
                        self.convention.push((at, o));
                    }
                }
                self.close_effect(at, call_sp);
                if let Some(r) = self.records.last_mut() {
                    r.frame_reads = reads;
                    r.live_in = live;
                }
                let sp = self.sp_value()?;
                self.state.forget_below(sp.min(call_sp.wrapping_add(u64::from(self.env.ret_slot))));
                Ok(Some(exit))
            }
            _ => {
                self.state = saved;
                self.records.truncate(records);
                Ok(None)
            }
        }
    }

    fn finish(self, noreturn: bool) -> Outcome<'e, 'p> {
        Outcome {
            state: self.state,
            indirect: self.indirect,
            calls: self.records,
            noreturn,
            unverified_noreturn: self.unverified_noreturn,
            followed: self.followed,
            guessed: self.guessed,
            events: self.events,
            convention: self.convention,
            path: self.shadow.map(|s| s.path),
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
#[derive(Clone, Copy)]
enum Exit {
    Return,
    NoReturn,
    /// A callee returned to where an enclosing call returns (libgcc's float
    /// routines reach a shared tail with `bl` and pop their caller's frame):
    /// every frame up to that call's ends.
    Unwind(u64),
    /// A callee returned straight to the function's caller (a trampoline).
    Leave,
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
/// `back` is where a callee should return, just past the call that entered
/// it, and the stack pointer at that call. A callee that returns elsewhere,
/// into other code of the image or on another stack (an RTOS's context switch
/// to another thread, a `longjmp`), did not return to its caller, and the
/// model cannot follow it. A return elsewhere on the same stack outside the
/// image (a pointer authentication the model does not run, eBPF's `exit`) is
/// taken as a return, as before.
fn exec(m: &mut Machine, entry: u64, mode: Mode, depth: u32, back: Option<(u64, u64)>) -> Result<Exit, Halt> {
    let outer = m.state.in_callee_code;
    m.state.in_callee_code = mode != Mode::Top;
    if let Some((b, _)) = back {
        m.returns.push(b & !1);
    }
    let exit = exec_body(m, entry, mode, depth, back);
    if back.is_some() {
        m.returns.pop();
    }
    m.state.in_callee_code = outer;
    exit.map(|e| match (e, back) {
        (Exit::Unwind(t), Some((b, _))) if returns_to(t, b) => Exit::Return,
        (e, _) => e,
    })
}

/// Whether a return to `target` is a return to `back`, a call's fallthrough.
fn returns_to(target: u64, back: u64) -> bool {
    (target & !1).wrapping_sub(back & !1) <= RETURN_SLACK
}

fn exec_body(m: &mut Machine, entry: u64, mode: Mode, depth: u32, back: Option<(u64, u64)>) -> Result<Exit, Halt> {
    let top = mode == Mode::Top;
    if depth > MAX_DEPTH {
        return Err(Halt::Unsupported("calls nested too deep".into()));
    }
    let env = m.env;
    let calls = m.calls;
    let mut addr = entry;
    let mut insn = lift(env, addr)?;
    let mut i = 0usize;
    let mut mode_switch: Option<(String, Option<u128>)> = None;
    let mut called_here = false;
    loop {
        if i >= insn.ops.len() {
            addr = env.code.wrap_offset(addr.wrapping_add(insn.len));
            insn = lift(env, addr)?;
            i = 0;
            m.state.end_instruction();
            m.steps += 1;
            if m.steps > m.budget {
                return Err(Halt::Budget);
            }
            continue;
        }
        m.steps += 1;
        if m.steps > m.budget {
            return Err(Halt::Budget);
        }
        if i == 0 {
            called_here = false;
            m.copies.clear();
            m.state.read_saved |= !m.restoring.is_empty();
            m.restoring.clear();
            if let Some(l) = m.state.live.as_mut() {
                l.instruction();
            }
        }
        if let Some(l) = m.state.live.as_mut() {
            l.op();
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
                    return m.tail_call(addr, t.offset, false);
                }
                if let Some((name, _)) = mode_switch.take().filter(|(_, v)| !relative && !decodes_in(env, t.offset, *v)) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
            }
            OpCode::CPUI_CBRANCH => {
                let c = read(&mut m.state, arg(1)?)?;
                if top && m.shadow.is_some() {
                    m.sym_branch(addr, i, arg(1)?, c);
                }
                if c != 0 {
                    let t = arg(0)?;
                    if space(t)?.get_type() != spacetype::IPTR_CONSTANT && tail(addr, t.offset) {
                        return m.tail_call(addr, t.offset, false);
                    }
                    branch(env, &mut m.state, t, &mut i, &mut addr, &mut insn)?;
                } else {
                    i += 1;
                }
            }
            OpCode::CPUI_BRANCHIND => {
                let target = read(&mut m.state, arg(0)?)? as u64;
                if tail(addr, target) {
                    return m.tail_call(addr, target, true);
                }
                if top && !env.code_mapped(target) {
                    return Ok(Exit::Return);
                }
                if top {
                    m.indirect.push(target);
                }
                if let Some((name, _)) = mode_switch.take().filter(|(_, v)| !decodes_in(env, target, *v)) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} before a branch at {addr:#x}")));
                }
                addr = target;
                insn = lift(env, addr)?;
                i = 0;
                m.state.end_instruction();
            }
            OpCode::CPUI_RETURN => {
                let target = read(&mut m.state, arg(0)?)? as u64;
                if env.ret_addr == Some(target) {
                    return Ok(if top { Exit::Return } else { Exit::Leave });
                }
                if !top {
                    if let Some((b, call_sp)) = back {
                        let elsewhere = !returns_to(target, b);
                        let sp = m.sp_value()?;
                        let switched = sp.wrapping_sub(call_sp).wrapping_add(STACK_SWITCH) > 2 * STACK_SWITCH;
                        if elsewhere && !switched && m.returns.iter().rev().skip(1).any(|&r| returns_to(target, r)) {
                            return Ok(Exit::Unwind(target));
                        }
                        if elsewhere && (switched || env.code_mapped(target & !1)) {
                            return Err(Halt::Unsupported(format!("a callee returned to {target:#x}, not to {b:#x} after its call")));
                        }
                    }
                    return Ok(Exit::Return);
                }
                if tail(addr, target) {
                    return m.tail_call(addr, target, true);
                }
                if !calls.branches.contains(&addr) || !env.code_mapped(target) {
                    return Ok(Exit::Return);
                }
                addr = target;
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
                let indirect = op.opc == OpCode::CPUI_CALLIND;
                let own = mode != Mode::Opaque && calls.known(addr);
                if indirect && mode != Mode::Opaque && !own && calls.jumps.contains(&addr) {
                    addr = target;
                    insn = lift(env, addr)?;
                    i = 0;
                    m.state.end_instruction();
                    continue;
                }
                let in_image = env.code_mapped(target) && !(m.checking_noreturn && leads_out(env, target));
                if own {
                    let args = m.record(addr, Some(target), indirect)?;
                    if calls.site(addr).noreturn {
                        match m.stops_at(addr, target, &args)? {
                            None => return Ok(Exit::NoReturn),
                            Some(_) => {
                                called_here = true;
                                i += 1;
                                continue;
                            }
                        }
                    }
                    match if in_image { m.real(addr, target)? } else { None } {
                        Some(e @ (Exit::Leave | Exit::Unwind(_))) => return Ok(e),
                        Some(_) => {}
                        None => {
                            m.guess_if_frame(in_image, &args);
                            m.stub(addr, &args)?
                        }
                    }
                } else if mode == Mode::Opaque && m.known_noreturn(target)? {
                    m.proved_noreturn = true;
                    return Ok(Exit::NoReturn);
                } else if in_image {
                    if top && !m.followed.contains(&(addr, target)) {
                        m.followed.push((addr, target));
                    }
                    let inner = if mode == Mode::Opaque { Mode::Opaque } else { Mode::Inline };
                    let back = Some((addr.wrapping_add(insn.len), m.sp_value()?));
                    match exec(m, target, inner, depth + 1, back)? {
                        Exit::NoReturn => return Ok(Exit::NoReturn),
                        e @ (Exit::Leave | Exit::Unwind(_)) => return Ok(e),
                        Exit::Return => {}
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
                let system = crate::calls::SYSTEM_USEROPS.contains(&name.as_str());
                let trap = crate::calls::TRAP_USEROPS.contains(&name.as_str());
                if (system && mode != Mode::Opaque) || (trap && top) {
                    let mut ins = Vec::with_capacity(op.ins.len().saturating_sub(1));
                    for vn in op.ins.iter().skip(1) {
                        ins.push(read(&mut m.state, vn)?);
                    }
                    let k = m.events.len();
                    let file = m.state.register_file();
                    m.events.push(crate::calls::UserEvent { at: addr, name, ins, file });
                    if trap {
                        return Ok(Exit::NoReturn);
                    }
                    let run = m.state.run;
                    if let Some(out) = op.out.as_ref() {
                        write(&mut m.state, out, crate::calls::userop_value(run, k, out.size))?;
                    } else if env.kernel.get() {
                        let abi = crate::calls::SyscallAbi::of(env.prog);
                        if let Some(r) = abi.result.as_ref().filter(|_| !abi.inputs.is_empty()) {
                            m.state.write_storage(&r.space, r.off, r.size, crate::calls::userop_value(run, k, r.size))?;
                        }
                        if let Some(e) = abi.error.as_ref() {
                            m.state.write_storage(&e.space, e.off, e.size, crate::calls::syscall_error_value(run, k, e.size))?;
                        }
                    }
                    i += 1;
                    continue;
                }
                if op.out.is_none() && NO_EFFECT_USEROPS.contains(&name.as_str()) {
                    i += 1;
                    continue;
                }
                if let (Some(out), Some(v)) = (op.out.as_ref(), constant_userop(&name)) {
                    write(&mut m.state, out, v)?;
                    i += 1;
                    continue;
                }
                if !(op.out.is_none() && EXIT_ONLY_USEROPS.contains(&name.as_str())) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} at {addr:#x}")));
                }
                let to = match op.ins.get(1) {
                    Some(vn) => Some(read(&mut m.state, vn)?),
                    None => None,
                };
                mode_switch = Some((name, to));
                i += 1;
            }
            OpCode::CPUI_LOAD => {
                let spc = mem_space(env, arg(0)?)?;
                let ptr = read(&mut m.state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let out = op.out.as_ref().ok_or_else(|| Halt::Harness("LOAD without output".into()))?;
                let v = if spc.get_type() == spacetype::IPTR_CONSTANT {
                    u128::from(ptr) & mask(out.size)
                } else {
                    if spc.get_index() == env.ram.get_index() {
                        m.note_load(at, out);
                    }
                    m.state.read(&spc, at, out.size)?
                };
                write(&mut m.state, out, v)?;
                if top && m.shadow.is_some() && spc.get_type() != spacetype::IPTR_CONSTANT {
                    let e = m.sym_read(&VarnodeData { space: Some(Rc::clone(&spc)), offset: at, size: out.size }, v);
                    m.sym_write(out, v, e);
                }
                i += 1;
            }
            OpCode::CPUI_STORE => {
                let spc = mem_space(env, arg(0)?)?;
                let ptr = read(&mut m.state, arg(1)?)? as u64;
                let at = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                let val = arg(2)?;
                if let Some(l) = m.state.live.as_mut() {
                    l.op();
                }
                let v = read(&mut m.state, val)?;
                if spc.get_index() == env.ram.get_index() && env.in_stack(at, u64::from(val.size)) {
                    let unique = space(val).is_ok_and(|s| s.get_type() == spacetype::IPTR_INTERNAL).then_some(val.offset);
                    if let Some(l) = m.state.live.as_mut() {
                        l.spilled(unique);
                    }
                }
                m.state.write(&spc, at, val.size, v)?;
                if m.shadow.is_some() {
                    let e = if top { m.sym_read(val, v) } else { None };
                    m.sym_write(&VarnodeData { space: Some(Rc::clone(&spc)), offset: at, size: val.size }, v, e);
                }
                if let Some(reg) = m.spill_of_entry(&spc, at, val) {
                    m.state.mark_saved(at, val.size, reg);
                }
                i += 1;
            }
            opc => {
                let out = op.out.as_ref().ok_or_else(|| Halt::Unsupported(get_opname(opc).to_string()))?;
                let mut ins = crate::value::Inputs::default();
                let track = top && m.shadow.is_some();
                let mut syms = Vec::new();
                for vn in &op.ins {
                    let v = read(&mut m.state, vn)?;
                    if track {
                        syms.push((m.sym_read(vn, v), v, vn.size));
                    }
                    ins.push((v, vn.size));
                }
                let by_zero = matches!(opc, OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_SDIV)
                    && env.divide_by_zero_is_zero
                    && ins.as_slice().get(1).is_some_and(|&(d, _)| d == 0);
                let v = if by_zero { 0 } else { eval(opc, out.size, ins.as_slice())? };
                if opc == OpCode::CPUI_COPY {
                    let src = &op.ins[0];
                    if space(out).is_ok_and(|s| s.get_type() == spacetype::IPTR_INTERNAL) {
                        if let Some(l) = m.state.live.as_mut() {
                            l.copied(out.offset);
                        }
                        if let Some(reg) = m.entry_value_of(src) {
                            m.copies.push((out.offset, reg, out.size));
                        }
                    } else if space(src).is_ok_and(|s| s.get_type() == spacetype::IPTR_INTERNAL) {
                        m.restoring.retain(|&(u, r)| !(u == src.offset && r == out.offset));
                    }
                }
                write(&mut m.state, out, v)?;
                if track {
                    let e = crate::concolic::op(opc, out.size, &syms);
                    m.sym_write(out, v, e);
                }
                i += 1;
            }
        }
    }
}

/// Run the function at `entry` from the run's initial state.
/// The argument-register bytes the callee of the `call`-th call gets.
#[derive(Clone, Debug)]
pub struct WithArgs {
    /// The call by index, or `None` for the function's own entry.
    pub call: Option<usize>,
    pub bytes: Vec<(u64, u8)>,
    /// Nonzero: other values in the registers stubs kill.
    pub kill_salt: u32,
}

pub fn run<'e, 'p>(
    env: &'e Env<'p>,
    run: u32,
    entry: u64,
    budget: u64,
    calls: &Calls,
    with_args: Option<&WithArgs>,
) -> Result<Outcome<'e, 'p>, Halt> {
    let mut m = Machine {
        with_args,
        env,
        calls,
        state: State::new(env, run),
        records: Vec::new(),
        indirect: Vec::new(),
        followed: Vec::new(),
        steps: 0,
        budget,
        callee_pool: budget,
        in_callee: false,
        returns: Vec::new(),
        events: Vec::new(),
        guessed: false,
        stubs: 0,
        unverified_noreturn: false,
        proved_noreturn: false,
        checking_noreturn: false,
        copies: Vec::new(),
        restoring: Vec::new(),
        convention: Vec::new(),
        shadow: env.record_path.get().then(Shadow::default),
    };
    m.state.kill_salt = with_args.map_or(0, |o| o.kill_salt);
    m.state.kill_salt_skip = calls.output_regs.clone();
    if let Some(o) = with_args.filter(|o| o.call.is_none()) {
        let register = std::rc::Rc::clone(&env.register);
        for &(off, b) in &o.bytes {
            m.state.write(&register, off, 1, u128::from(b))?;
        }
    }
    let exit = exec(&mut m, entry, Mode::Top, 0, None)?;
    Ok(m.finish(matches!(exit, Exit::NoReturn)))
}
