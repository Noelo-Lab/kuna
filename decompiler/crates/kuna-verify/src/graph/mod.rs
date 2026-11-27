//! The decompiled side: an interpreter over the compiled `Funcdata`.
//!
//! It walks the basic blocks from the entry, evaluates each block's MULTIEQUALs
//! together on the incoming edge, then the block's ops in order. A CBRANCH takes
//! out-edge 1 when its condition, flipped by `boolean_flip`, holds. A varnode
//! that is neither constant nor written reads its storage: an input from the
//! run's initial state, any other from the current one. LOAD and STORE use the
//! same memory model as the machine side, and a call has the same stub.

pub mod program;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::get_opname;

use crate::calls::{Calls, Record};
use crate::machine::{Checkpoint, CopyOperands};
use crate::program::{Arg, Effect, Kind, Op, Operand, Out, Program};
use crate::state::{Env, State, Writes};
use crate::value::{eval, mask, Halt};

/// What the decompiled function returned: `None` when its RETURN carries no
/// value.
pub type Returned = Option<u128>;

#[derive(Clone, Copy)]
pub(crate) struct CopyOverlap {
    pub(crate) checkpoint: Checkpoint,
    pub(crate) operands: CopyOperands,
}

/// How the decompiled run ended.
pub struct Outcome {
    pub ret: Returned,
    /// A switch met a selector no row of its table carries.
    pub unmatched_switch: bool,
    /// The run stopped before a forward overlapping copy.
    pub(crate) overlapping_copy: Option<CopyOverlap>,
    pub calls: Vec<Record>,
    /// The run ended in a call kuna says does not return.
    pub noreturn: bool,
    /// Memory written outside the stack since the last call.
    pub writes: Writes,
    /// It read a byte of its own frame that nothing wrote.
    pub read_uninitialized: bool,
    /// The calls, up to [`CALL_INPUTS`] of them, where a callee the machine
    /// ran for real read argument bytes kuna's call does not pass the same
    /// values in.
    pub call_inputs: Vec<CallInput>,
    /// Calls past [`CALL_INPUTS`] whose argument reads differ.
    pub unkept_inputs: usize,
    /// System calls and traps, in order.
    pub events: Vec<crate::calls::UserEvent>,
}

/// Calls per run whose differing argument reads are kept for a rerun; more
/// are only counted ([`Outcome::unkept_inputs`]).
pub const CALL_INPUTS: usize = 64;

/// A call whose callee reads argument bytes kuna passes otherwise:
/// the report, and kuna's bytes (a stand-in where it passes none) for a
/// machine run that gives them to the callee.
#[derive(Clone, Debug)]
pub struct CallInput {
    pub msg: String,
    pub with: crate::machine::WithArgs,
    /// Omitted bytes that can be tried with zero if the first stand-in fails.
    pub(crate) omitted_register_bytes: Vec<u64>,
    /// The callee's prototype is declared: kuna's call follows it.
    pub locked: bool,
}

/// Storage kuna's C never assigns, given other values for a second run: the
/// registers it reads at entry without a parameter for them, and registers a
/// call kills that it reads afterwards without a call output for them.
#[derive(Clone, Debug)]
pub struct Perturb {
    pub salt: u64,
    pub inputs: Vec<(u64, u32)>,
    /// Also give registers a call kills, read after it, other values: the
    /// output registers (`Some(true)`) or the others (`Some(false)`).
    pub killed: Option<bool>,
}

/// The largest copy, fill or compare a builtin runs; a longer one spends the
/// budget, as the loop it replaces would.
const MAX_BUILTIN_BYTES: u64 = 1 << 20;

struct Run<'e, 'p, 'c> {
    calls: &'c Calls,
    perturb: Option<&'c Perturb>,
    call_inputs: Vec<CallInput>,
    unkept_inputs: usize,
    events: Vec<crate::calls::UserEvent>,
    /// The machine's calls, whose recorded effects this side replays.
    replay: &'c [Record],
    state: State<'e, 'p>,
    initial: State<'e, 'p>,
    values: Vec<Option<u128>>,
    unmatched_switch: bool,
    records: Vec<Record>,
    /// Steps a memory builtin took, charged to the budget like the loop it
    /// replaces.
    charged: u64,
    copies: std::collections::BTreeMap<u64, usize>,
}

enum Flow {
    Next,
    Edge(i32),
    Overlap(CopyOverlap),
    Return(Returned),
    NoReturn,
}

impl Run<'_, '_, '_> {
    /// Whether the machine's callee is kuna's, so its recorded effect may be
    /// replayed: the same address, the same code through thunks, a call kuna
    /// bound past the PLT stub the machine calls, an injection, or a target of
    /// kuna's that is not code in the image (an import, a PIC GOT slot), which
    /// the call check leaves alone.
    fn same_callee(&self, machine: Option<u64>, kuna: Option<u64>, site: &crate::calls::Site) -> bool {
        let env = self.state.env;
        if let Some(stub) = site.plt_stub {
            return crate::compare::observe::reaches_stub(env, machine, stub);
        }
        let (m, k) = (crate::compare::observe::code_target(env, machine), crate::compare::observe::code_target(env, kuna));
        site.resolved
            || m == k
            || k.is_none_or(|k| !env.is_code(k))
            || m.zip(k).is_some_and(|(m, k)| crate::machine::transparent_target(env, m) == crate::machine::transparent_target(env, k))
    }

    /// Fill the register save area as `va_start` does: the stores kuna's
    /// `varargsave` folded, the vector ones only when the incoming `al` is set.
    fn va_start(&mut self, env: &Env, f: &kuna_decomp::kuna_varargsave::Folded) -> Result<(), Halt> {
        let al = self.initial.read_storage(&env.register, f.al.0, f.al.1)?;
        for &(reg, size, frame, vector) in &f.stores {
            if vector && al == 0 {
                continue;
            }
            let v = self.initial.read_storage(&env.register, reg, size)?;
            self.state.write_storage(&env.ram, env.sp0.wrapping_add(frame as u64), size, v)?;
        }
        Ok(())
    }

    fn value(&mut self, a: &Arg) -> Result<u128, Halt> {
        match &a.op {
            Operand::Const(v) => Ok(*v),
            Operand::Slot(slot, at) => self.values[*slot]
                .ok_or_else(|| Halt::Harness(format!("read of {at} before its definition"))),
            Operand::Memory(st) => self.state.read_storage(&st.space, st.off, a.size),
            Operand::Input(st) => match self.perturb.filter(|p| self.unassigned(p, st)) {
                Some(p) => Ok(scramble(p.salt, self.state.run, st.off) & mask(a.size)),
                None => self.initial.read_storage(&st.space, st.off, a.size),
            },
            Operand::Free(st) => self.state.read_storage(&st.space, st.off, a.size),
            Operand::TooWide => Err(Halt::Unsupported(format!("{}-byte varnode", a.size))),
            Operand::Missing => Err(Halt::Harness("op lacks an input".into())),
        }
    }

    /// Whether an op writes a register the convention returns values in.
    fn is_output(&self, op: &Op) -> bool {
        op.out.as_ref().and_then(|o| o.storage.as_ref()).is_some_and(|st| {
            st.space.get_index() == self.state.env.register.get_index()
                && self.calls.output_regs.iter().any(|&(o, n)| st.off < o + u64::from(n) && o < st.off + u64::from(st.size))
        })
    }

    /// Whether input storage `st` is one kuna's C never assigns.
    fn unassigned(&self, p: &Perturb, st: &crate::storage::Storage) -> bool {
        st.space.get_index() == self.state.env.register.get_index()
            && p.inputs.iter().any(|&(o, n)| st.off < o + u64::from(n) && o < st.off + u64::from(st.size))
    }

    /// Compare what a callee the machine ran for real read with what kuna's
    /// call passes there.
    fn check_inputs(&mut self, op: &Op, k: usize) -> Result<(), Halt> {
        let Some(live) = self.replay.get(k).filter(|r| r.at == op.at).and_then(|r| r.live_in.clone()) else { return Ok(()) };
        let env = self.state.env;
        let reg = env.register.get_index();
        let be = env.register.is_big_endian();
        let params = &self.calls.site(op.at).input_storage;
        let int_regs = &self.calls.int_regs;
        let mut next = 0usize;
        let mut placed: Vec<Option<crate::storage::Storage>> = Vec::with_capacity(op.ins.len());
        for (k, a) in op.ins.iter().enumerate().skip(1) {
            let entry_reg = match &a.op {
                crate::program::Operand::Input(st) => int_regs.iter().position(|r| r.covers(st)).map(|i| (i, st.clone())),
                _ => None,
            };
            let float_reg = a.storage.clone().filter(|st| {
                st.space.get_index() == reg
                    && !int_regs.iter().any(|r| r.overlaps(st))
                    && self.calls.input_regs.iter().any(|&(o, n)| st.off >= o && st.off + u64::from(st.size) <= o + u64::from(n))
            });
            let st = match (params.get(k - 1).cloned().flatten(), entry_reg) {
                (Some(p), _) => Some(p),
                (None, Some((i, st))) => {
                    next = next.max(i + 1);
                    Some(st)
                }
                (None, None) if float_reg.is_some() => float_reg,
                (None, None) => {
                    let r = int_regs.get(next).cloned();
                    next += 1;
                    r
                }
            };
            if let Some(i) = st.as_ref().and_then(|s| int_regs.iter().position(|r| r.covers(s))) {
                next = next.max(i + 1);
            }
            placed.push(st);
        }
        let mut bytes = Vec::new();
        let mut omitted_register_bytes = Vec::new();
        let mut first = None;
        for (off, m) in live {
            let (o, n) = self.calls.input_regs.iter().copied().find(|&(o, n)| off >= o && off < o + u64::from(n)).unwrap_or((off, 1));
            let mut kuna = None;
            let mut passes_part = false;
            'inputs: for (a, st) in op.ins.iter().skip(1).zip(&placed) {
                let Some(st) = st.as_ref() else { continue };
                let pieces = st.pieces(env.prog);
                let mut low = 0u64;
                for p in pieces.iter().rev() {
                    let here = low;
                    low += u64::from(p.size);
                    if p.space.get_index() != reg {
                        continue;
                    }
                    passes_part |= p.off < o + u64::from(n) && o < p.off + u64::from(p.size);
                    if off >= p.off && off < p.off + u64::from(p.size) {
                        let v = self.value(a)?;
                        let pos = if be { p.off + u64::from(p.size) - 1 - off } else { off - p.off };
                        kuna = Some((v >> (8 * (here + pos))) as u8);
                        break 'inputs;
                    }
                }
            }
            if kuna == Some(m) || (kuna.is_none() && passes_part) {
                continue;
            }
            bytes.push((off, kuna.unwrap_or_else(|| !m)));
            if kuna.is_none() {
                omitted_register_bytes.push(off);
            }
            if first.is_none() {
                let name = crate::storage::Storage::new(&env.register, o, n).name(env.prog);
                let target = self.calls.site(op.at).target.map_or_else(|| "?".to_string(), |t| format!("{t:#x}"));
                let what = match kuna {
                    Some(kb) => format!("kuna passes 0x{kb:02x} there"),
                    None => "kuna's call passes nothing there".to_string(),
                };
                first = Some(format!(
                    "call at {:#x} to {target}: the callee reads {name} (byte {:#x}) = 0x{m:02x}; {what}",
                    op.at,
                    off - o
                ));
            }
        }
        let (stack_bytes, stack_msg) = self.stack_arguments(op.at, k);
        if let Some(msg) = first.or(stack_msg) {
            if self.call_inputs.len() < CALL_INPUTS {
                let locked = self.calls.site(op.at).locked;
                self.call_inputs.push(CallInput {
                    msg, with: crate::machine::WithArgs { call: Some(k), bytes, stack_bytes, kill_salt: 0 },
                    omitted_register_bytes, locked,
                });
            } else {
                self.unkept_inputs += 1;
            }
        }
        Ok(())
    }

    fn stack_arguments(&self, at: u64, k: usize) -> (Vec<(u64, u8)>, Option<String>) {
        let site = self.calls.site(at);
        let machine = &self.replay[k];
        let Some(stack) = machine.stack_in.as_ref().filter(|_| self.same_callee(machine.target, self.records[k].target, site)) else {
            return (Vec::new(), None);
        };
        let args = &self.records[k].args;
        let mut bytes = Vec::new();
        let mut first = None;
        for (off, read) in stack.bytes() {
            let kuna = args.iter().enumerate().find_map(|(i, &v)| {
                let st = site.stack_input(i)?;
                let byte = off.checked_sub(st.off).filter(|&byte| byte < u64::from(st.size))?;
                Some((v >> (8 * byte)) as u8)
            });
            if let Some(m) = read.filter(|&m| kuna != Some(m)) {
                bytes.push((off, kuna.unwrap_or(!m)));
                if first.is_none() {
                    let what = kuna.map_or_else(|| "kuna's call passes nothing there".to_string(), |b| format!("kuna passes 0x{b:02x} there"));
                    first = Some(format!("call at {at:#x}: the callee reads stack[{off:#x}] = 0x{m:02x}; {what}"));
                }
            } else if let Some(b) = kuna {
                bytes.push((off, b));
            }
        }
        if first.is_none() {
            bytes.clear();
        }
        (bytes, first)
    }

    fn input(&mut self, op: &Op, k: usize) -> Result<u128, Halt> {
        match op.ins.get(k) {
            Some(a) => self.value(a),
            None => Err(Halt::Harness(format!("op lacks input {k}"))),
        }
    }

    fn set(&mut self, out: &Out, val: u128) -> Result<(), Halt> {
        self.values[out.slot] = Some(val);
        if let Some(st) = &out.backing {
            self.state.write_storage(&st.space, st.off, out.size, val)?;
        }
        Ok(())
    }

    fn set_out(&mut self, op: &Op, val: u128) -> Result<(), Halt> {
        let out = op.out.as_ref().ok_or_else(|| Halt::Harness("op without output".into()))?;
        self.set(out, val)
    }

    /// The value an output's storage holds now.
    fn stored(&mut self, op: &Op) -> Result<u128, Halt> {
        let out = op.out.as_ref().ok_or_else(|| Halt::Harness("op without output".into()))?;
        let st = out.storage.clone().ok_or_else(|| Halt::Harness("output without storage".into()))?;
        self.state.read_storage(&st.space, st.off, out.size)
    }

    fn address(&mut self, space: &Option<std::rc::Rc<AddrSpace>>, op: &Op) -> Result<(std::rc::Rc<AddrSpace>, u64), Halt> {
        let spc = space.clone().ok_or_else(|| Halt::Harness("bad LOAD space id".into()))?;
        let ptr = self.input(op, 0)? as u64;
        let addr = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
        Ok((spc, addr))
    }

    /// kuna's memory builtins, byte by byte: `memcpy(dst, src, bytes)` (as
    /// `memmove`, so an overlap kuna's `memcpy` does not allow shows),
    /// `memset(dst, byte, bytes)`, `memset16`/`32`/`64(dst, value, count)` and
    /// `r = memcmp(a, b, bytes)`, whose sign says which first differing byte
    /// is smaller.
    fn memory_builtin(&mut self, op: &Op, name: &str, at: u64) -> Result<Flow, Halt> {
        let ram = std::rc::Rc::clone(&self.state.env.ram);
        let ptr = |v: u128| ram.wrap_offset(v as u64);
        let width: u64 = match name {
            "memset16" => 2,
            "memset32" => 4,
            "memset64" => 8,
            "memcpy" | "memset" | "memcmp" => 1,
            _ => return Err(Halt::Unsupported(format!("{name} at {at:#x}"))),
        };
        let (a, b, n) = (self.input(op, 1)?, self.input(op, 2)?, self.input(op, 3)?);
        let bytes = u64::try_from(n).ok().and_then(|n| n.checked_mul(width)).filter(|&n| n <= MAX_BUILTIN_BYTES).ok_or(Halt::Budget)?;
        self.charged = bytes;
        let dst = ptr(a);
        match name {
            "memcpy" => {
                let src = ptr(b);
                let visits = self.copies.entry(at).or_default();
                *visits += 1;
                if dst > src && dst - src < bytes {
                    self.charged = 0;
                    return Ok(Flow::Overlap(CopyOverlap {
                        checkpoint: Checkpoint { at, visit: *visits },
                        operands: CopyOperands { dest: dst, src, bytes: u128::from(bytes), backwards: false },
                    }));
                }
                let mut data = Vec::with_capacity(bytes as usize);
                for i in 0..bytes {
                    data.push(self.state.read(&ram, ram.wrap_offset(src.wrapping_add(i)), 1)?);
                }
                for (i, v) in data.into_iter().enumerate() {
                    self.state.write(&ram, ram.wrap_offset(dst.wrapping_add(i as u64)), 1, v)?;
                }
            }
            "memcmp" => {
                let other = ptr(b);
                let mut r = 0i64;
                for i in 0..bytes {
                    let x = self.state.read(&ram, ram.wrap_offset(dst.wrapping_add(i)), 1)?;
                    let y = self.state.read(&ram, ram.wrap_offset(other.wrapping_add(i)), 1)?;
                    if x != y {
                        r = if x < y { -1 } else { 1 };
                        break;
                    }
                }
                let size = op.out.as_ref().map_or(4, |o| o.size);
                self.set_out(op, (r as i128 as u128) & mask(size))?;
            }
            _ => {
                let value = if name == "memset" { b & 0xff } else { b };
                for i in 0..bytes / width {
                    self.state.write(&ram, ram.wrap_offset(dst.wrapping_add(i * width)), width as u32, value & mask(width as u32))?;
                }
            }
        }
        Ok(Flow::Next)
    }

    fn exec(&mut self, op: &Op) -> Result<Flow, Halt> {
        let at = op.at;
        match &op.kind {
            Kind::Branch => Ok(Flow::Edge(0)),
            Kind::CBranch { flip } => {
                let cond = self.input(op, 1)?;
                Ok(Flow::Edge(if (cond != 0) != *flip { 1 } else { 0 }))
            }
            Kind::Return { halt } => {
                if *halt {
                    return Err(Halt::Unsupported(format!("the decompiled flow halts at {at:#x}")));
                }
                if op.ins.len() > 1 {
                    Ok(Flow::Return(Some(self.input(op, 1)?)))
                } else {
                    Ok(Flow::Return(None))
                }
            }
            Kind::BranchInd { switch, targets } => {
                let v = self.input(op, 0)?;
                let vsize = op.ins[0].size;
                let edge = match switch {
                    Some(sw) => {
                        match sw.rows.iter().find(|(label, _)| u128::from(*label) & mask(vsize) == v) {
                            Some((_, Some(e))) => Some(*e),
                            Some((_, None)) => {
                                return Err(Halt::Unsupported(format!("BRANCHIND at {at:#x}: the row for {v:#x} reaches no out-edge")));
                            }
                            None => {
                                self.unmatched_switch = true;
                                sw.default
                            }
                        }
                    }
                    None => targets.iter().position(|&t| u128::from(t) == v).map(|e| e as i32),
                };
                if edge.is_none() && self.unmatched_switch {
                    return Err(Halt::NoRow(format!("BRANCHIND at {at:#x} on {v:#x}")));
                }
                edge.map(Flow::Edge)
                    .ok_or_else(|| Halt::Unsupported(format!("BRANCHIND at {at:#x} on {v:#x}, which no out-edge takes")))
            }
            Kind::Call { direct } => {
                let target = match direct {
                    Some(_) => self.calls.site(at).target,
                    None => Some(self.input(op, 0)? as u64),
                };
                let mut args = Vec::with_capacity(op.ins.len().saturating_sub(1));
                for k in 1..op.ins.len() {
                    args.push(self.input(op, k)?);
                }
                let indirect = direct.is_none();
                let wide = target.is_some_and(|t| self.state.env.code_mapped(t));
                let writes = self.state.writes_at_call(&args, wide);
                crate::calls::record(&mut self.records, Record { at, target, indirect, args, writes, effect: None, frame_reads: None, live_in: None, stack_in: None })?;
                if target.is_some_and(|t| self.state.env.explicit.as_ref().is_some_and(|case| case.external_entries.contains_key(&t))) {
                    self.events.push(crate::calls::UserEvent {
                        at, name: "external-call".to_owned(), ins: Vec::new(), file: Vec::new(),
                    });
                    return Ok(Flow::NoReturn);
                }
                let site = self.calls.site(at);
                if site.noreturn {
                    return Ok(Flow::NoReturn);
                }
                let k = self.records.len() - 1;
                self.check_inputs(op, k)?;
                let real = self.replay.get(k).filter(|r| r.at == at && self.same_callee(r.target, target, site)).and_then(|r| r.effect.clone());
                match real {
                    Some(effect) => self.state.replay(&effect),
                    None => {
                        let killed = site.killed.clone();
                        let args: Vec<u128> = self.records[k]
                            .args
                            .iter()
                            .zip(&site.args)
                            .filter(|(_, a)| a.is_some())
                            .map(|(&v, _)| v)
                            .collect();
                        self.state.call(&killed, &args, site.ret_ext.as_ref());
                    }
                }
                if let Some(out) = op.out.as_ref() {
                    let physical = out.storage.as_ref().is_some_and(|st| {
                        matches!(
                            st.space.get_type(),
                            kuna_base::space::spacetype::IPTR_PROCESSOR
                                | kuna_base::space::spacetype::IPTR_SPACEBASE
                                | kuna_base::space::spacetype::IPTR_JOIN
                        )
                    });
                    let v = match (physical, self.calls.site(at).output.clone()) {
                        (false, Some(st)) => self.state.read_storage(&st.space, st.off, out.size.min(st.size))?,
                        _ => self.stored(op)?,
                    };
                    self.set_out(op, v)?;
                }
                Ok(Flow::Next)
            }
            Kind::CallOther { name, builtin } => {
                if let Some(b) = builtin {
                    return self.memory_builtin(op, b, at);
                }
                let system = crate::calls::SYSTEM_USEROPS.contains(&name.as_str());
                let trap = crate::calls::TRAP_USEROPS.contains(&name.as_str());
                if system || trap {
                    let mut ins = Vec::with_capacity(op.ins.len().saturating_sub(1));
                    for k in 1..op.ins.len() {
                        ins.push(self.input(op, k)?);
                    }
                    let k = self.events.len();
                    self.events.push(crate::calls::UserEvent { at, name: name.clone(), ins, file: Vec::new() });
                    if trap || self.state.env.explicit.is_some() {
                        return Ok(Flow::NoReturn);
                    }
                    if let Some(out) = op.out.as_ref() {
                        self.set(out, crate::calls::userop_value(self.state.run, k, out.size))?;
                    }
                    return Ok(Flow::Next);
                }
                if name == crate::program::SYSCALL_ERROR {
                    if let (Some(out), Some(k)) = (op.out.as_ref(), self.events.len().checked_sub(1)) {
                        self.set(out, crate::calls::syscall_error_value(self.state.run, k, out.size))?;
                        return Ok(Flow::Next);
                    }
                }
                if let (Some(out), Some(v)) = (op.out.as_ref(), crate::machine::constant_userop(name)) {
                    self.set(out, v)?;
                    return Ok(Flow::Next);
                }
                let quiet = crate::machine::EXIT_ONLY_USEROPS.contains(&name.as_str()) || crate::machine::NO_EFFECT_USEROPS.contains(&name.as_str());
                if op.out.is_some() || !quiet {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} at {at:#x}")));
                }
                Ok(Flow::Next)
            }
            Kind::Store(space) => {
                let (spc, addr) = self.address(space, op)?;
                let size = op.ins.get(1).map(|a| a.size).ok_or_else(|| Halt::Harness("STORE lacks a value".into()))?;
                let v = self.input(op, 1)?;
                self.state.write(&spc, addr, size, v)?;
                Ok(Flow::Next)
            }
            Kind::Load(space) => {
                let (spc, addr) = self.address(space, op)?;
                let size = op.out.as_ref().map(|o| o.size).ok_or_else(|| Halt::Unsupported("op without output".into()))?;
                let v = if spc.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT {
                    u128::from(addr) & mask(size)
                } else {
                    self.state.read(&spc, addr, size)?
                };
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
            Kind::Indirect(effect) => {
                let v = match effect {
                    Effect::Keep => self.input(op, 0)?,
                    Effect::Killed if self.perturb.and_then(|p| p.killed).is_some_and(|outputs| outputs == self.is_output(op)) => {
                        let (size, slot) = op.out.as_ref().map_or((0, 0), |o| (o.size, o.slot as u64));
                        scramble(self.perturb.map_or(0, |p| p.salt), self.state.run, op.at ^ (slot << 40) ^ 0x5bd1_e995) & mask(size)
                    }
                    Effect::Killed | Effect::Memory => self.stored(op)?,
                };
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
            Kind::IndirectBefore => {
                let v = self.input(op, 0)?;
                let out = op.out.as_ref().ok_or_else(|| Halt::Harness("op without output".into()))?;
                if let Some(st) = &out.storage {
                    self.state.write_storage(&st.space, st.off, out.size, v)?;
                }
                Ok(Flow::Next)
            }
            Kind::StrayPhi => Err(Halt::Harness("MULTIEQUAL after a block's head".into())),
            Kind::Nop => Ok(Flow::Next),
            Kind::Eval(opc) => {
                let size = op.out.as_ref().map(|o| o.size).ok_or_else(|| Halt::Unsupported("op without output".into()))?;
                let mut ins = crate::value::Inputs::default();
                for a in &op.ins {
                    ins.push((self.value(a)?, a.size));
                }
                let v = eval(*opc, size, ins.as_slice()).map_err(|h| match h {
                    Halt::Unsupported(s) if s == get_opname(*opc) => Halt::Unsupported(format!("{s} at {at:#x}")),
                    other => other,
                })?;
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
        }
    }

    fn finish(self, ret: Returned, noreturn: bool, overlapping_copy: Option<CopyOverlap>) -> Outcome {
        Outcome {
            ret,
            unmatched_switch: self.unmatched_switch,
            overlapping_copy,
            writes: self.state.writes(),
            read_uninitialized: self.state.read_uninitialized,
            call_inputs: self.call_inputs,
            unkept_inputs: self.unkept_inputs,
            events: self.events,
            calls: self.records,
            noreturn,
        }
    }
}

/// Run the decompiled function from its entry block on the run's initial state.
/// A value for storage kuna never assigns, other than the run's own.
pub(crate) fn scramble(salt: u64, run: u32, key: u64) -> u128 {
    let mut z = salt ^ u64::from(run).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ key.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    u128::from(z) | (u128::from(z.rotate_left(17)) << 64)
}

pub fn run(
    env: &Env,
    program: &Program,
    run: u32,
    budget: u64,
    calls: &Calls,
    replay: &[Record],
    perturb: Option<&Perturb>,
) -> Result<Outcome, Halt> {
    if program.blocks.is_empty() {
        return Err(Halt::Harness("no basic blocks".into()));
    }
    let mut r = Run {
        calls,
        perturb,
        call_inputs: Vec::new(),
        unkept_inputs: 0,
        events: Vec::new(),
        replay,
        state: State::new(env, run),
        initial: State::new(env, run),
        values: vec![None; program.slots],
        unmatched_switch: false,
        records: Vec::new(),
        charged: 0,
        copies: std::collections::BTreeMap::new(),
    };
    if let Some(f) = &program.vararg {
        r.va_start(env, f)?;
    }
    let mut bl = 0usize;
    let mut slot: Option<i32> = None;
    let mut steps = 0u64;
    loop {
        let block = &program.blocks[bl];
        steps += 1;
        if steps > budget {
            return Err(Halt::Budget);
        }
        if !block.phis.is_empty() {
            let s = slot.ok_or_else(|| Halt::Harness("MULTIEQUAL in the entry block".into()))?;
            let mut vals = Vec::with_capacity(block.phis.len());
            for phi in &block.phis {
                let a = phi.ins.get(s as usize).ok_or_else(|| Halt::Harness(format!("op lacks input {s}")))?;
                vals.push(r.value(a)?);
            }
            for (phi, v) in block.phis.iter().zip(vals) {
                let out = phi.out.as_ref().ok_or_else(|| Halt::Harness("op without output".into()))?;
                r.set(out, v)?;
            }
        }
        let mut edge = 0;
        for (k, op) in block.ops.iter().enumerate() {
            if program.watch == Some((bl, k)) {
                program.reached.set(true);
            }
            steps += 1;
            if steps > budget {
                return Err(Halt::Budget);
            }
            let flow = r.exec(op)?;
            steps += std::mem::take(&mut r.charged);
            if steps > budget {
                return Err(Halt::Budget);
            }
            match flow {
                Flow::Next => {}
                Flow::Edge(e) => {
                    edge = e;
                    break;
                }
                Flow::Overlap(at) => return Ok(r.finish(None, false, Some(at))),
                Flow::Return(ret) => return Ok(r.finish(ret, false, None)),
                Flow::NoReturn => return Ok(r.finish(None, true, None)),
            }
        }
        let Some(&(next, rev)) = block.outs.get(edge as usize).filter(|_| edge >= 0) else {
            return Err(Halt::Harness(format!("block {} has no out-edge {edge}", block.index)));
        };
        if next == usize::MAX {
            return Err(Halt::Harness(format!("block {} leaves the block list", block.index)));
        }
        slot = Some(rev);
        bl = next;
    }
}
