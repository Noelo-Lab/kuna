//! The decompiled side: an interpreter over the final `Funcdata`.
//!
//! It walks the basic blocks from the entry, evaluates each block's MULTIEQUALs
//! together on the incoming edge, then the block's ops in order. A CBRANCH takes
//! out-edge 1 when its condition, flipped by `boolean_flip`, holds. A varnode
//! that is neither constant nor written reads its storage from the run's
//! initial state, declared in the prototype or not.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::context::{BlockId, OpId, VarnodeId};
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::{get_opname, OpCode};

use crate::state::{Env, State};
use crate::value::{eval, mask, Halt, MAX_SIZE};

pub struct GraphRun<'e, 'p, 'f> {
    fd: &'f Funcdata,
    state: State<'e, 'p>,
    initial: State<'e, 'p>,
    values: BTreeMap<VarnodeId, u128>,
    unmatched_switch: bool,
}

/// How the decompiled run ended.
pub struct Outcome {
    pub ret: Returned,
    /// A switch met a selector no row of its table carries.
    pub unmatched_switch: bool,
}

/// What the decompiled function returned: `None` when its RETURN carries no
/// value.
pub type Returned = Option<u128>;

impl<'e, 'p, 'f> GraphRun<'e, 'p, 'f> {
    fn value(&self, vn: VarnodeId) -> Result<(u128, u32), Halt> {
        let v = self
            .fd
            .vbank()
            .get(vn)
            .ok_or_else(|| Halt::Harness("stale varnode".into()))?;
        let size = v.get_size() as u32;
        if size > MAX_SIZE {
            return Err(Halt::Unsupported(format!("{size}-byte varnode")));
        }
        if v.is_constant() {
            return Ok((u128::from(v.get_offset()) & mask(size), size));
        }
        if v.is_written() {
            return self
                .values
                .get(&vn)
                .map(|&x| (x, size))
                .ok_or_else(|| Halt::Harness(format!("read of {} before its definition", v.get_addr().get_offset())));
        }
        let addr = v.get_addr();
        let spc = addr
            .get_space()
            .ok_or_else(|| Halt::Harness("varnode without a space".into()))?;
        let from = if v.is_input() { &self.initial } else { &self.state };
        Ok((from.read_storage(spc, addr.get_offset(), size)?, size))
    }

    fn input(&self, op: OpId, slot: i32) -> Result<VarnodeId, Halt> {
        self.fd
            .obank()
            .get(op)
            .and_then(|o| o.get_in(slot))
            .ok_or_else(|| Halt::Harness(format!("op lacks input {slot}")))
    }

    fn mem_space(&self, op: OpId) -> Result<Rc<AddrSpace>, Halt> {
        let id = self.value(self.input(op, 0)?)?.0;
        self.fd
            .get_arch()
            .manage()
            .get_space(id as i32)
            .cloned()
            .ok_or_else(|| Halt::Harness("bad LOAD space id".into()))
    }

    fn set_out(&mut self, op: OpId, val: u128) -> Result<(), Halt> {
        let out = self
            .fd
            .obank()
            .get(op)
            .and_then(|o| o.get_out())
            .ok_or_else(|| Halt::Harness("op without output".into()))?;
        self.values.insert(out, val);
        let Some(v) = self.fd.vbank().get(out) else { return Ok(()) };
        let addr = v.get_addr();
        if let Some(spc) = addr.get_space() {
            if Self::memory_backed(self.state.env, spc) {
                let spc = Rc::clone(spc);
                self.state.write_storage(&spc, addr.get_offset(), v.get_size() as u32, val)?;
            }
        }
        Ok(())
    }

    /// A written varnode in the stack space or in memory updates the memory a
    /// later LOAD through a pointer reads.
    fn memory_backed(env: &Env, spc: &AddrSpace) -> bool {
        spc.get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE
            || (spc.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR
                && spc.get_index() != env.register.get_index())
    }

    fn out_size(&self, op: OpId) -> Result<u32, Halt> {
        let out = self
            .fd
            .obank()
            .get(op)
            .and_then(|o| o.get_out())
            .ok_or_else(|| Halt::Unsupported("op without output".into()))?;
        Ok(self.fd.vbank().get(out).map(|v| v.get_size() as u32).unwrap_or(0))
    }
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

enum Flow {
    Next,
    Edge(i32),
    Return(Returned),
}

fn exec<'e, 'p, 'f>(g: &mut GraphRun<'e, 'p, 'f>, op: OpId) -> Result<Flow, Halt> {
    let o = g.fd.obank().get(op).ok_or_else(|| Halt::Harness("stale op".into()))?;
    let opc = o.code();
    let at = o.get_addr().get_offset();
    let nin = o.num_input();
    let flip = o.is_boolean_flip();
    match opc {
        OpCode::CPUI_BRANCH => Ok(Flow::Edge(0)),
        OpCode::CPUI_CBRANCH => {
            let cond = g.value(g.input(op, 1)?)?.0;
            Ok(Flow::Edge(if (cond != 0) != flip { 1 } else { 0 }))
        }
        OpCode::CPUI_RETURN => {
            if o.get_halt_type() != 0 {
                return Err(Halt::Unsupported(format!("the decompiled flow halts at {at:#x}")));
            }
            if nin > 1 {
                Ok(Flow::Return(Some(g.value(g.input(op, 1)?)?.0)))
            } else {
                Ok(Flow::Return(None))
            }
        }
        OpCode::CPUI_BRANCHIND => {
            let (v, vsize) = g.value(g.input(op, 0)?)?;
            let graph = g.fd.bblocks_ref();
            let parent = o.get_parent().ok_or_else(|| Halt::Harness("BRANCHIND outside a block".into()))?;
            let block = graph.block(parent);
            let edge_to = |addr: u64| {
                (0..block.size_out())
                    .find(|&e| kuna_decomp::block::block_get_start(&graph.arena, block.get_out(e)).get_offset() == addr)
            };
            let jt = g.fd.find_jump_table(op).filter(|jt| jt.is_labelled());
            let edge = match jt {
                Some(jt) => {
                    let row = (0..jt.num_entries()).find(|&i| {
                        let label = jt.get_label_by_index(i);
                        label != kuna_decomp::jumptable::NO_LABEL && u128::from(label) & mask(vsize) == v
                    });
                    if row.is_none() {
                        g.unmatched_switch = true;
                    }
                    row.and_then(|i| edge_to(jt.get_address_by_index(i).get_offset()))
                        .or_else(|| (jt.get_default_block() >= 0).then(|| jt.get_default_block()))
                }
                None => edge_to(v as u64),
            };
            if edge.is_none() && g.unmatched_switch {
                return Err(Halt::NoRow(format!("BRANCHIND at {at:#x} on {v:#x}")));
            }
            edge.map(Flow::Edge).ok_or_else(|| {
                Halt::Unsupported(format!("BRANCHIND at {at:#x} on {v:#x}, which no out-edge takes"))
            })
        }
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => Err(Halt::NotLeaf(format!("call at {at:#x}"))),
        OpCode::CPUI_CALLOTHER => {
            let index = g.value(g.input(op, 0)?)?.0 as u64;
            if let Some(name) = memory_builtin(index) {
                return Err(Halt::Memory(format!("{name} at {at:#x}")));
            }
            let name = crate::machine::userop_name(g.state.env, index);
            let has_out = g.fd.obank().get(op).is_some_and(|o| o.get_out().is_some());
            if has_out || !crate::machine::EXIT_ONLY_USEROPS.contains(&name.as_str()) {
                return Err(Halt::Unsupported(format!("CALLOTHER {name} at {at:#x}")));
            }
            Ok(Flow::Next)
        }
        OpCode::CPUI_STORE => Err(Halt::Memory(format!("store at {at:#x}"))),
        OpCode::CPUI_LOAD => {
            let spc = g.mem_space(op)?;
            let ptr = g.value(g.input(op, 1)?)?.0 as u64;
            let size = g.out_size(op)?;
            let addr = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
            g.state.env.check_load(&spc, addr, size)?;
            let v = g.state.read(&spc, addr, size)?;
            g.set_out(op, v)?;
            Ok(Flow::Next)
        }
        OpCode::CPUI_INDIRECT => {
            let v = g.value(g.input(op, 0)?)?.0;
            g.set_out(op, v)?;
            Ok(Flow::Next)
        }
        OpCode::CPUI_MULTIEQUAL => Err(Halt::Harness("MULTIEQUAL after a block's head".into())),
        _ => {
            let size = g.out_size(op)?;
            let mut ins = Vec::with_capacity(nin as usize);
            for k in 0..nin {
                ins.push(g.value(g.input(op, k)?)?);
            }
            let v = eval(opc, size, &ins).map_err(|h| match h {
                Halt::Unsupported(s) if s == get_opname(opc) => Halt::Unsupported(format!("{s} at {at:#x}")),
                other => other,
            })?;
            g.set_out(op, v)?;
            Ok(Flow::Next)
        }
    }
}

/// The start address of every basic block: the code the decompiled function
/// can reach.
pub fn block_starts(fd: &Funcdata) -> std::collections::BTreeSet<u64> {
    let graph = fd.bblocks_ref();
    (0..fd.bblocks_get_size())
        .map(|i| kuna_decomp::block::block_get_start(&graph.arena, fd.bblocks_get_block(i)).get_offset())
        .collect()
}

/// Run the decompiled function from its entry block on the run's initial state.
pub fn run(env: &Env, fd: &Funcdata, run: u32, budget: u64) -> Result<Outcome, Halt> {
    let graph = fd.bblocks_ref();
    if fd.bblocks_get_size() == 0 {
        return Err(Halt::Harness("no basic blocks".into()));
    }
    let mut g = GraphRun {
        fd,
        state: State::new(env, run),
        initial: State::new(env, run),
        values: BTreeMap::new(),
        unmatched_switch: false,
    };
    let mut bl: BlockId = fd.bblocks_get_block(0);
    let mut slot: Option<i32> = None;
    let mut steps = 0u64;
    loop {
        let ops = fd.bb_ops(bl);
        let mut phis = Vec::new();
        let mut body = Vec::new();
        for &op in &ops {
            let is_phi = fd.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_MULTIEQUAL);
            if is_phi && body.is_empty() {
                phis.push(op);
            } else {
                body.push(op);
            }
        }
        if !phis.is_empty() {
            let s = slot.ok_or_else(|| Halt::Harness("MULTIEQUAL in the entry block".into()))?;
            let vals = phis
                .iter()
                .map(|&op| g.value(g.input(op, s)?).map(|v| (op, v.0)))
                .collect::<Result<Vec<_>, _>>()?;
            for (op, v) in vals {
                g.set_out(op, v)?;
            }
        }
        let mut edge = 0;
        for op in body {
            steps += 1;
            if steps > budget {
                return Err(Halt::Budget);
            }
            match exec(&mut g, op)? {
                Flow::Next => {}
                Flow::Edge(e) => {
                    edge = e;
                    break;
                }
                Flow::Return(r) => return Ok(Outcome { ret: r, unmatched_switch: g.unmatched_switch }),
            }
        }
        let block = graph.block(bl);
        if edge >= block.size_out() {
            return Err(Halt::Harness(format!("block {} has no out-edge {edge}", block.get_index())));
        }
        slot = Some(block.get_out_rev_index(edge));
        bl = block.get_out(edge);
    }
}
