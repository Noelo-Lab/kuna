//! The decompiled side: an interpreter over the compiled `Funcdata`.
//!
//! It walks the basic blocks from the entry, evaluates each block's MULTIEQUALs
//! together on the incoming edge, then the block's ops in order. A CBRANCH takes
//! out-edge 1 when its condition, flipped by `boolean_flip`, holds. A varnode
//! that is neither constant nor written reads its storage: an input from the
//! run's initial state, any other from the current one.

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::get_opname;

use crate::program::{Arg, Kind, Op, Operand, Out, Program};
use crate::state::{Env, State};
use crate::value::{eval, mask, Halt};

/// What the decompiled function returned: `None` when its RETURN carries no
/// value.
pub type Returned = Option<u128>;

/// How the decompiled run ended.
pub struct Outcome {
    pub ret: Returned,
    /// A switch met a selector no row of its table carries.
    pub unmatched_switch: bool,
}

struct Run<'e, 'p> {
    state: State<'e, 'p>,
    initial: State<'e, 'p>,
    values: Vec<Option<u128>>,
    unmatched_switch: bool,
}

enum Flow {
    Next,
    Edge(i32),
    Return(Returned),
}

impl Run<'_, '_> {
    fn value(&mut self, a: &Arg) -> Result<u128, Halt> {
        match &a.op {
            Operand::Const(v) => Ok(*v),
            Operand::Slot(slot, at) => self.values[*slot]
                .ok_or_else(|| Halt::Harness(format!("read of {at} before its definition"))),
            Operand::Input(st) => self.initial.read_storage(&st.space, st.off, a.size),
            Operand::Free(st) => self.state.read_storage(&st.space, st.off, a.size),
            Operand::TooWide => Err(Halt::Unsupported(format!("{}-byte varnode", a.size))),
            Operand::Missing => Err(Halt::Harness("op lacks an input".into())),
        }
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
                        let row = sw.rows.iter().find(|(label, _)| u128::from(*label) & mask(vsize) == v);
                        if row.is_none() {
                            self.unmatched_switch = true;
                        }
                        row.and_then(|(_, e)| *e).or(sw.default)
                    }
                    None => targets.iter().position(|&t| u128::from(t) == v).map(|e| e as i32),
                };
                if edge.is_none() && self.unmatched_switch {
                    return Err(Halt::NoRow(format!("BRANCHIND at {at:#x} on {v:#x}")));
                }
                edge.map(Flow::Edge)
                    .ok_or_else(|| Halt::Unsupported(format!("BRANCHIND at {at:#x} on {v:#x}, which no out-edge takes")))
            }
            Kind::Call => Err(Halt::NotLeaf(format!("call at {at:#x}"))),
            Kind::CallOther { name, builtin } => {
                if let Some(b) = builtin {
                    return Err(Halt::Memory(format!("{b} at {at:#x}")));
                }
                if op.out.is_some() || !crate::machine::EXIT_ONLY_USEROPS.contains(&name.as_str()) {
                    return Err(Halt::Unsupported(format!("CALLOTHER {name} at {at:#x}")));
                }
                Ok(Flow::Next)
            }
            Kind::Store => Err(Halt::Memory(format!("store at {at:#x}"))),
            Kind::Load(space) => {
                let spc = space.clone().ok_or_else(|| Halt::Harness("bad LOAD space id".into()))?;
                let ptr = self.input(op, 0)? as u64;
                let size = op.out.as_ref().map(|o| o.size).ok_or_else(|| Halt::Unsupported("op without output".into()))?;
                let addr = spc.wrap_offset(AddrSpace::address_to_byte(ptr, spc.get_word_size()));
                self.state.env.check_load(&spc, addr, size)?;
                let v = self.state.read(&spc, addr, size)?;
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
            Kind::Indirect => {
                let v = self.input(op, 0)?;
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
            Kind::StrayPhi => Err(Halt::Harness("MULTIEQUAL after a block's head".into())),
            Kind::Eval(opc) => {
                let size = op.out.as_ref().map(|o| o.size).ok_or_else(|| Halt::Unsupported("op without output".into()))?;
                let mut ins = Vec::with_capacity(op.ins.len());
                for a in &op.ins {
                    ins.push((self.value(a)?, a.size));
                }
                let v = eval(*opc, size, &ins).map_err(|h| match h {
                    Halt::Unsupported(s) if s == get_opname(*opc) => Halt::Unsupported(format!("{s} at {at:#x}")),
                    other => other,
                })?;
                self.set_out(op, v)?;
                Ok(Flow::Next)
            }
        }
    }
}

/// Run the decompiled function from its entry block on the run's initial state.
pub fn run(env: &Env, program: &Program, run: u32, budget: u64) -> Result<Outcome, Halt> {
    if program.blocks.is_empty() {
        return Err(Halt::Harness("no basic blocks".into()));
    }
    let mut r = Run {
        state: State::new(env, run),
        initial: State::new(env, run),
        values: vec![None; program.slots],
        unmatched_switch: false,
    };
    let mut bl = 0usize;
    let mut slot: Option<i32> = None;
    let mut steps = 0u64;
    loop {
        let block = &program.blocks[bl];
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
        for op in &block.ops {
            steps += 1;
            if steps > budget {
                return Err(Halt::Budget);
            }
            match r.exec(op)? {
                Flow::Next => {}
                Flow::Edge(e) => {
                    edge = e;
                    break;
                }
                Flow::Return(ret) => return Ok(Outcome { ret, unmatched_switch: r.unmatched_switch }),
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
