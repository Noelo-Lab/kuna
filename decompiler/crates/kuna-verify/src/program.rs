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
use kuna_decomp::context::{OpId, VarnodeId};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::jumptable::NO_LABEL;
use kuna_num::opcodes::OpCode;

use crate::storage::Storage;
use crate::value::{mask, MAX_SIZE};

#[derive(Clone, Debug)]
pub enum Operand {
    Const(u128),
    /// A written varnode's slot, and the address offset its error names.
    Slot(usize, u64),
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
}

/// Where a written varnode lives: its slot, and the memory a later LOAD of the
/// same bytes reads when the varnode is a stack or memory location.
#[derive(Clone, Debug)]
pub struct Out {
    pub slot: usize,
    pub size: u32,
    pub backing: Option<Storage>,
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
    Store,
    Branch,
    CBranch { flip: bool },
    Return { halt: bool },
    /// A switch's rows, or `None` when the input is a code address; and the
    /// start address of each out-edge's block.
    BranchInd { switch: Option<Switch>, targets: Vec<u64> },
    Call,
    CallOther { name: String, builtin: Option<&'static str> },
    Indirect,
    StrayPhi,
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

pub struct Program {
    pub blocks: Vec<Block>,
    pub slots: usize,
    pub starts: BTreeSet<u64>,
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
        (BUILTIN_STRLEN, "strlen"),
    ]
    .into_iter()
    .find(|&(id, _)| id == index)
    .map(|(_, name)| name)
}

pub fn userop_name(prog: &ConsoleProgram, index: u64) -> String {
    let mut names = Vec::new();
    prog.arch().translate().get_user_op_names(&mut names);
    names.get(index as usize).cloned().unwrap_or_else(|| format!("#{index}"))
}

struct Compiler<'a> {
    prog: &'a ConsoleProgram,
    fd: &'a Funcdata,
    slots: BTreeMap<VarnodeId, usize>,
}

impl Compiler<'_> {
    fn slot(&mut self, vn: VarnodeId) -> usize {
        let n = self.slots.len();
        *self.slots.entry(vn).or_insert(n)
    }

    fn arg(&mut self, vn: Option<VarnodeId>) -> Arg {
        let Some(id) = vn else { return Arg { op: Operand::Missing, size: 0 } };
        let Some(v) = self.fd.vbank().get(id) else { return Arg { op: Operand::Missing, size: 0 } };
        let size = v.get_size() as u32;
        let op = if size > MAX_SIZE {
            Operand::TooWide
        } else if v.is_constant() {
            Operand::Const(u128::from(v.get_offset()) & mask(size))
        } else if v.is_written() {
            Operand::Slot(self.slot(id), v.get_addr().get_offset())
        } else {
            match Storage::at(v.get_addr(), size) {
                Some(st) if v.is_input() => Operand::Input(st),
                Some(st) => Operand::Free(st),
                None => Operand::Missing,
            }
        };
        Arg { op, size }
    }

    fn out(&mut self, vn: Option<VarnodeId>) -> Option<Out> {
        let id = vn?;
        let v = self.fd.vbank().get(id)?;
        let size = v.get_size() as u32;
        let backing = Storage::at(v.get_addr(), size).filter(|st| {
            st.space.get_type() == spacetype::IPTR_SPACEBASE
                || (st.space.get_type() == spacetype::IPTR_PROCESSOR
                    && st.space.get_index()
                        != self.prog.arch().manage().get_space_by_name("register").map_or(-1, |r| r.get_index()))
        });
        Some(Out { slot: self.slot(id), size, backing })
    }

    fn switch(&self, op: OpId, starts: &[u64]) -> Option<Switch> {
        let jt = self.fd.find_jump_table(op).filter(|jt| jt.is_labelled())?;
        let edge_to = |addr: u64| starts.iter().position(|&s| s == addr).map(|e| e as i32);
        let rows = (0..jt.num_entries())
            .filter_map(|i| {
                let label = jt.get_label_by_index(i);
                (label != NO_LABEL).then(|| (label, edge_to(jt.get_address_by_index(i).get_offset())))
            })
            .collect();
        let default = (jt.get_default_block() >= 0).then(|| jt.get_default_block());
        Some(Switch { rows, default })
    }

    fn op(&mut self, id: OpId, out_starts: &[u64]) -> Option<Op> {
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
                Kind::BranchInd { switch: self.switch(id, out_starts), targets: out_starts.to_vec() }
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => Kind::Call,
            OpCode::CPUI_STORE => Kind::Store,
            OpCode::CPUI_INDIRECT => Kind::Indirect,
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

/// Compile the final `Funcdata`.
pub fn compile(prog: &ConsoleProgram, fd: &Funcdata) -> Program {
    let graph = fd.bblocks_ref();
    let n = fd.bblocks_get_size();
    let ids: Vec<_> = (0..n).map(|i| fd.bblocks_get_block(i)).collect();
    let position: BTreeMap<_, usize> = ids.iter().enumerate().map(|(i, &b)| (b, i)).collect();
    let start_of = |b| kuna_decomp::block::block_get_start(&graph.arena, b).get_offset();
    let mut c = Compiler { prog, fd, slots: BTreeMap::new() };
    let mut blocks = Vec::with_capacity(ids.len());
    for &bl in &ids {
        let block = graph.block(bl);
        let succ: Vec<_> = (0..block.size_out()).map(|e| block.get_out(e)).collect();
        let out_starts: Vec<u64> = succ.iter().map(|&b| start_of(b)).collect();
        let outs = (0..block.size_out())
            .map(|e| (position.get(&succ[e as usize]).copied().unwrap_or(usize::MAX), block.get_out_rev_index(e)))
            .collect();
        let mut phis = Vec::new();
        let mut ops = Vec::new();
        for op in fd.bb_ops(bl) {
            let Some(o) = fd.obank().get(op) else { continue };
            if o.code() == OpCode::CPUI_MULTIEQUAL && ops.is_empty() {
                let ins = (0..o.num_input()).map(|k| c.arg(o.get_in(k))).collect();
                let out = c.out(o.get_out());
                phis.push(Phi { out, ins });
            } else if let Some(compiled) = c.op(op, &out_starts) {
                ops.push(compiled);
            }
        }
        blocks.push(Block { index: block.get_index(), start: start_of(bl), phis, ops, outs });
    }
    let starts = blocks.iter().map(|b| b.start).collect();
    Program { blocks, slots: c.slots.len(), starts }
}
