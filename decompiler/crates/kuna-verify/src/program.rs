//! The decompiled function compiled once for repeated runs.
//!
//! Every written varnode gets a slot in a flat value array, operands are
//! resolved to a constant, a slot, or a storage location to read, and each
//! switch's rows are mapped to out-edges ahead of time. A run then touches
//! neither the op bank nor the varnode bank. Anything that cannot be modeled is
//! kept as data and reported only when a run reaches it, so a path no run takes
//! never decides the verdict.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::context::{BlockId, OpId, VarnodeId};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::jumptable::NO_LABEL;
use kuna_num::opcodes::OpCode;

use crate::calls::Calls;
use crate::storage::Storage;
use crate::value::{mask, MAX_SIZE};

#[derive(Clone, Debug)]
pub enum Operand {
    Const(u128),
    /// A written varnode's slot, and the address offset its error names.
    Slot(usize, u64),
    /// A varnode tied to a stack or memory location: the location now, as the
    /// printed C reads the variable. A STORE through a pointer the IR did not
    /// guard still shows.
    Memory(Storage),
    /// An input varnode: its storage at entry.
    Input(Storage),
    /// Neither written nor an input: its storage now.
    Free(Storage),
    /// A varnode the value model cannot hold.
    TooWide,
    Missing,
}

#[derive(Clone, Debug)]
pub struct Arg {
    pub op: Operand,
    pub size: u32,
    /// Where the varnode lives, for a CALL's inputs: the storage a callee
    /// reads the argument from.
    pub storage: Option<Storage>,
}

/// Where a written varnode lives: its slot, its storage, and the memory a later
/// LOAD of the same bytes reads when the varnode is a stack or memory location.
#[derive(Clone, Debug)]
pub struct Out {
    pub slot: usize,
    pub size: u32,
    pub storage: Option<Storage>,
    pub backing: Option<Storage>,
}

/// What an INDIRECT's output holds once the op it guards has run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// The op leaves the storage alone: the output is the input.
    Keep,
    /// A register the guarded call kills.
    Killed,
    /// Memory: whatever the store or the call left there.
    Memory,
}

#[derive(Clone, Debug)]
pub struct Switch {
    /// Labelled rows: label, and the out-edge their address starts.
    pub rows: Vec<(u64, Option<i32>)>,
    pub default: Option<i32>,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Eval(OpCode),
    Load(Option<Rc<AddrSpace>>),
    Store(Option<Rc<AddrSpace>>),
    Branch,
    CBranch { flip: bool },
    Return { halt: bool },
    /// A switch's rows, or `None` when the input is a code address; and the
    /// start address of each out-edge's block.
    BranchInd { switch: Option<Switch>, targets: Vec<u64> },
    /// `direct`: the target of a CALL; a CALLIND reads its input.
    Call { direct: Option<u64> },
    CallOther { name: String, builtin: Option<&'static str> },
    Indirect(Effect),
    /// A memory INDIRECT at its own place, before the op it guards: its input
    /// is what the storage holds when that op runs.
    IndirectBefore,
    StrayPhi,
    /// Does nothing: what a mutant leaves of a dropped store.
    Nop,
}

#[derive(Clone, Debug)]
pub struct Op {
    pub kind: Kind,
    pub ins: Vec<Arg>,
    pub out: Option<Out>,
    pub at: u64,
}

#[derive(Clone, Debug)]
pub struct Phi {
    pub out: Option<Out>,
    pub ins: Vec<Arg>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub index: i32,
    pub start: u64,
    pub phis: Vec<Phi>,
    pub ops: Vec<Op>,
    /// Successor block position, and this edge's input slot in it.
    pub outs: Vec<(usize, i32)>,
}

#[derive(Clone)]
pub struct Program {
    pub blocks: Vec<Block>,
    pub slots: usize,
    pub starts: BTreeSet<u64>,
    /// A mutant's op, `(block, op)`, and whether a run reached it.
    pub watch: Option<(usize, usize)>,
    pub reached: std::rc::Rc<std::cell::Cell<bool>>,
    /// The register-save stores kuna's `varargsave` folded away: what the
    /// printed `va_list` setup means by `va_start`, run at entry.
    pub vararg: Option<kuna_decomp::kuna_varargsave::Folded>,
    /// CSIC: the op whose output an intervention sets, `(block, op)`, and the
    /// value for each of its executions, in order (`None` leaves one alone).
    pub force: Option<(usize, usize, Vec<Option<u128>>)>,
    /// CSIC: each executed op and its output value, in order.
    pub record: Option<std::rc::Rc<std::cell::RefCell<Vec<Exec>>>>,
}

/// One op of kuna's side as a run executed it.
#[derive(Clone, Copy, Debug)]
pub struct Exec {
    pub block: usize,
    pub op: usize,
    pub value: Option<u128>,
}

/// The decompiler's built-in user ops that read or write memory.
pub fn memory_builtin(index: u64) -> Option<&'static str> {
    use kuna_decomp::userop::*;
    let index = u32::try_from(index).ok()?;
    [
        (BUILTIN_STRINGDATA, "stringdata"),
        (BUILTIN_VOLATILE_READ, "volatile read"),
        (BUILTIN_VOLATILE_WRITE, "volatile write"),
        (BUILTIN_MEMCPY, "memcpy"),
        (BUILTIN_STRNCPY, "strncpy"),
        (BUILTIN_WCSNCPY, "wcsncpy"),
        (BUILTIN_MEMSET, "memset"),
        (BUILTIN_MEMSET16, "memset16"),
        (BUILTIN_MEMSET32, "memset32"),
        (BUILTIN_MEMSET64, "memset64"),
        (BUILTIN_MEMCMP, "memcmp"),
        (BUILTIN_STRLEN, "strlen"),
    ]
    .into_iter()
    .find(|&(id, _)| id == index)
    .map(|(_, name)| name)
}

/// The name the decompiler's `syscall_error()` builtin is checked under.
pub const SYSCALL_ERROR: &str = "syscall_error";

pub fn userop_name(prog: &ConsoleProgram, index: u64) -> String {
    if index == u64::from(kuna_decomp::userop::BUILTIN_SYSCALL_ERROR) {
        return SYSCALL_ERROR.to_string();
    }
    let mut names = Vec::new();
    prog.arch().translate().get_user_op_names(&mut names);
    names.get(index as usize).cloned().unwrap_or_else(|| format!("#{index}"))
}

struct Compiler<'a> {
    prog: &'a ConsoleProgram,
    fd: &'a Funcdata,
    calls: &'a Calls,
    register: i32,
    slots: BTreeMap<VarnodeId, usize>,
}

/// The op an INDIRECT guards, from the iop constant that names it.
fn guarded_op(fd: &Funcdata, indirect: OpId) -> Option<OpId> {
    let o = fd.obank().get(indirect)?;
    let v = fd.vbank().get(o.get_in(1)?)?;
    let space = v.get_addr().get_space()?;
    (space.get_type() == spacetype::IPTR_IOP).then(|| OpId::from(slotmap::KeyData::from_ffi(v.get_offset())))
}

impl Compiler<'_> {
    fn slot(&mut self, vn: VarnodeId) -> usize {
        let n = self.slots.len();
        *self.slots.entry(vn).or_insert(n)
    }

    fn arg(&mut self, vn: Option<VarnodeId>) -> Arg {
        let Some(id) = vn else { return Arg { op: Operand::Missing, size: 0, storage: None } };
        let Some(v) = self.fd.vbank().get(id) else { return Arg { op: Operand::Missing, size: 0, storage: None } };
        let size = v.get_size() as u32;
        let op = if size > MAX_SIZE {
            Operand::TooWide
        } else if v.is_constant() {
            Operand::Const(u128::from(v.get_offset()) & mask(size))
        } else if let Some(st) =
            Storage::at(v.get_addr(), size).filter(|st| v.is_addr_tied() && self.memory(st))
        {
            if v.is_written() {
                self.slot(id);
            }
            Operand::Memory(st)
        } else if v.is_written() {
            Operand::Slot(self.slot(id), v.get_addr().get_offset())
        } else {
            match Storage::at(v.get_addr(), size) {
                Some(st) if v.is_input() => Operand::Input(st),
                Some(st) => Operand::Free(st),
                None => Operand::Missing,
            }
        };
        let storage = (!v.is_constant()).then(|| Storage::at(v.get_addr(), size)).flatten();
        Arg { op, size, storage }
    }

    fn out(&mut self, vn: Option<VarnodeId>) -> Option<Out> {
        let id = vn?;
        let v = self.fd.vbank().get(id)?;
        let size = v.get_size() as u32;
        let storage = Storage::at(v.get_addr(), size);
        let backing = storage.clone().filter(|st| self.memory(st));
        Some(Out { slot: self.slot(id), size, storage, backing })
    }

    fn memory(&self, st: &Storage) -> bool {
        st.space.get_type() == spacetype::IPTR_SPACEBASE
            || (st.space.get_type() == spacetype::IPTR_PROCESSOR && st.space.get_index() != self.register)
    }

    fn effect(&self, indirect: OpId, out: Option<VarnodeId>) -> Effect {
        let Some(st) = out.and_then(|o| self.fd.vbank().get(o)).and_then(|v| Storage::at(v.get_addr(), v.get_size() as u32))
        else {
            return Effect::Keep;
        };
        if self.memory(&st) {
            return Effect::Memory;
        }
        let call = guarded_op(self.fd, indirect)
            .and_then(|g| self.fd.obank().get(g))
            .filter(|g| matches!(g.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND));
        let killed = call.is_some_and(|c| {
            st.space.get_index() == self.register
                && self.calls.site(c.get_addr().get_offset()).killed.iter().any(|&(off, size)| {
                    st.off < off + u64::from(size) && off < st.off + u64::from(st.size)
                })
        });
        if killed {
            Effect::Killed
        } else {
            Effect::Keep
        }
    }

    /// A labelled switch's rows, each mapped to the out-edge whose block the
    /// table sends it to.
    fn switch(&self, op: OpId, succ: &[BlockId]) -> Option<Switch> {
        let jt = self.fd.find_jump_table(op).filter(|jt| jt.is_labelled())?;
        let mut edge_of: BTreeMap<i32, i32> = BTreeMap::new();
        for (e, &bl) in succ.iter().enumerate() {
            let n = jt.num_indices_by_block(self.fd, bl).unwrap_or(0);
            for i in 0..n {
                if let Ok(idx) = jt.get_index_by_block(self.fd, bl, i) {
                    edge_of.entry(idx).or_insert(e as i32);
                }
            }
        }
        let rows = (0..jt.num_entries())
            .filter_map(|i| {
                let label = jt.get_label_by_index(i);
                (label != NO_LABEL).then(|| (label, edge_of.get(&i).copied()))
            })
            .collect();
        let default = (jt.get_default_block() >= 0).then(|| jt.get_default_block());
        Some(Switch { rows, default })
    }

    fn op(&mut self, id: OpId, out_starts: &[u64], succ: &[BlockId]) -> Option<Op> {
        let o = self.fd.obank().get(id)?;
        let opc = o.code();
        let at = o.get_addr().get_offset();
        let ins_ids: Vec<Option<VarnodeId>> = (0..o.num_input()).map(|k| o.get_in(k)).collect();
        let out_id = o.get_out();
        let flip = o.is_boolean_flip();
        let halt = o.get_halt_type() != 0;
        let mut ins: Vec<Arg> = ins_ids.iter().map(|&v| self.arg(v)).collect();
        let kind = match opc {
            OpCode::CPUI_BRANCH => Kind::Branch,
            OpCode::CPUI_CBRANCH => Kind::CBranch { flip },
            OpCode::CPUI_RETURN => Kind::Return { halt },
            OpCode::CPUI_BRANCHIND => {
                Kind::BranchInd { switch: self.switch(id, succ), targets: out_starts.to_vec() }
            }
            OpCode::CPUI_CALL => {
                let direct = ins_ids.first().copied().flatten().and_then(|v| self.fd.vbank().get(v)).map(|v| v.get_offset());
                Kind::Call { direct }
            }
            OpCode::CPUI_CALLIND => Kind::Call { direct: None },
            OpCode::CPUI_STORE => {
                let space = match ins.first().map(|a| &a.op) {
                    Some(Operand::Const(idx)) => self.prog.arch().manage().get_space(*idx as i32).cloned(),
                    _ => None,
                };
                ins.remove(0);
                Kind::Store(space)
            }
            OpCode::CPUI_INDIRECT => Kind::Indirect(self.effect(id, out_id)),
            OpCode::CPUI_MULTIEQUAL => Kind::StrayPhi,
            OpCode::CPUI_LOAD => {
                let space = match ins.first().map(|a| &a.op) {
                    Some(Operand::Const(idx)) => self.prog.arch().manage().get_space(*idx as i32).cloned(),
                    _ => None,
                };
                ins.remove(0);
                Kind::Load(space)
            }
            OpCode::CPUI_CALLOTHER => {
                let index = match ins.first().map(|a| &a.op) {
                    Some(Operand::Const(i)) => *i as u64,
                    _ => u64::MAX,
                };
                Kind::CallOther { name: userop_name(self.prog, index), builtin: memory_builtin(index) }
            }
            other => Kind::Eval(other),
        };
        let out = self.out(out_id);
        Some(Op { kind, ins, out, at })
    }
}

/// Where a body op runs: as itself, or as the two halves of an INDIRECT that
/// was moved.
#[derive(Clone, Copy)]
enum Place {
    Op(OpId),
    Before(OpId),
}

/// An INDIRECT stands just before the op whose effect it describes, and its
/// output is the value after that op: its input, with whatever the op wrote on
/// top. Each one is moved just after its op, and leaves a `Before` half in its
/// own place that puts the input in memory for the op to overwrite.
fn after_their_effects(fd: &Funcdata, body: &[OpId]) -> Vec<Place> {
    let guarded: BTreeMap<OpId, OpId> = body
        .iter()
        .filter(|&&op| fd.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_INDIRECT))
        .filter_map(|&op| guarded_op(fd, op).filter(|g| body.contains(g)).map(|g| (op, g)))
        .collect();
    let mut out = Vec::with_capacity(body.len() + guarded.len());
    for &op in body {
        if guarded.contains_key(&op) {
            out.push(Place::Before(op));
            continue;
        }
        out.push(Place::Op(op));
        out.extend(body.iter().filter(|&&i| guarded.get(&i) == Some(&op)).map(|&i| Place::Op(i)));
    }
    out
}

/// Whether an INDIRECT's input was computed by an op. An entry value, or
/// another guard's output on the same storage, only passes the location's
/// value along, so writing it back before the guarded op would undo a write
/// the IR did not guard.
fn computed_input(fd: &Funcdata, indirect: OpId) -> bool {
    let Some(v) = fd.obank().get(indirect).and_then(|o| o.get_in(0)).and_then(|v| fd.vbank().get(v)) else {
        return false;
    };
    let def = v.get_def().and_then(|d| fd.obank().get(d));
    def.is_some_and(|d| d.code() != OpCode::CPUI_INDIRECT)
}

/// Compile the final `Funcdata`.
pub fn compile(prog: &ConsoleProgram, fd: &Funcdata, calls: &Calls) -> Program {
    let graph = fd.bblocks_ref();
    let n = fd.bblocks_get_size();
    let ids: Vec<_> = (0..n).map(|i| fd.bblocks_get_block(i)).collect();
    let position: BTreeMap<_, usize> = ids.iter().enumerate().map(|(i, &b)| (b, i)).collect();
    let start_of = |b| kuna_decomp::block::block_get_start(&graph.arena, b).get_offset();
    let register = prog.arch().manage().get_space_by_name("register").map_or(-1, |r| r.get_index());
    let mut c = Compiler { prog, fd, calls, register, slots: BTreeMap::new() };
    let mut blocks = Vec::with_capacity(ids.len());
    for &bl in &ids {
        let block = graph.block(bl);
        let succ: Vec<_> = (0..block.size_out()).map(|e| block.get_out(e)).collect();
        let out_starts: Vec<u64> = succ.iter().map(|&b| start_of(b)).collect();
        let outs = (0..block.size_out())
            .map(|e| (position.get(&succ[e as usize]).copied().unwrap_or(usize::MAX), block.get_out_rev_index(e)))
            .collect();
        let mut phis = Vec::new();
        let mut body = Vec::new();
        for op in fd.bb_ops(bl) {
            let Some(o) = fd.obank().get(op) else { continue };
            if o.code() == OpCode::CPUI_MULTIEQUAL && body.is_empty() {
                let ins = (0..o.num_input()).map(|k| c.arg(o.get_in(k))).collect();
                let out = c.out(o.get_out());
                phis.push(Phi { out, ins });
            } else {
                body.push(op);
            }
        }
        let ops = after_their_effects(fd, &body)
            .into_iter()
            .filter_map(|place| match place {
                Place::Op(op) => c.op(op, &out_starts, &succ),
                Place::Before(op) => c
                    .op(op, &out_starts, &succ)
                    .filter(|o| matches!(o.kind, Kind::Indirect(Effect::Memory)) && computed_input(fd, op))
                    .map(|mut o| {
                        o.kind = Kind::IndirectBefore;
                        o
                    }),
            })
            .collect();
        blocks.push(Block { index: block.get_index(), start: start_of(bl), phis, ops, outs });
    }
    let starts = blocks.iter().map(|b| b.start).collect();
    Program { blocks, slots: c.slots.len(), starts, watch: None, reached: Default::default(), vararg: fd.vararg_folded().cloned(), force: None, record: None }
}
