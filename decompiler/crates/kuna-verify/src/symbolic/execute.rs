//! Independent raw storage execution and final-IR SSA execution.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype::IPTR_CONSTANT;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::concolic::Expr;
use crate::program::{Arg, Block, Kind, Operand, Out, Program};
use crate::state::Env;
use crate::storage::Storage;

use super::byte_file::ByteFile;
use super::terms::{self, Term};

pub(super) const RAW_BRANCH_DEPTH: u8 = 32;
pub(super) const IR_BRANCH_DEPTH: usize = 64;

pub(super) struct Execution {
    pub value: Term,
    pub defined: Term,
}

fn update_defined(defined: &mut Term, opcode: OpCode, args: &[Term]) {
    if let Some(guard) = terms::division_guard(opcode, args) {
        *defined = Rc::new(Expr::Op("bvand", vec![Rc::clone(defined), guard], 8));
    }
}

#[derive(Clone)]
struct RawState {
    registers: ByteFile,
    temps: ByteFile,
    defined: Term,
}

struct Instruction<'a> {
    address: u64,
    next_address: u64,
    code: i32,
    ops: &'a [crate::machine::RawOp],
    register: i32,
    unique: Option<i32>,
    steps: &'a mut u64,
    limit: u64,
}

impl Instruction<'_> {
    fn run(
        &mut self,
        mut state: RawState,
        mut index: usize,
        remaining: u8,
    ) -> Result<RawState, String> {
        while let Some(op) = self.ops.get(index) {
            charge(self.steps, self.limit)?;
            if matches!(op.opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) {
                let destination = op.ins.first().ok_or("raw branch has no target")?;
                let space = destination
                    .space
                    .as_ref()
                    .ok_or("raw branch target has no space")?;
                let target = if space.get_type() == IPTR_CONSTANT {
                    let delta = destination.offset as u32 as i32;
                    usize::try_from(delta)
                        .ok()
                        .and_then(|d| index.checked_add(d))
                        .filter(|&t| t > index && t <= self.ops.len())
                        .ok_or("unsupported backward or out-of-range intra-instruction branch")?
                } else if space.get_index() == self.code && destination.offset == self.next_address
                {
                    self.ops.len()
                } else {
                    return Err(format!(
                        "machine control-flow edge at {:#x} is outside the interval model",
                        self.address,
                    ));
                };
                if op.opc == OpCode::CPUI_BRANCH {
                    index = target;
                    continue;
                }
                if remaining == 0 {
                    return Err("symbolic intra-instruction branch depth exhausted".to_owned());
                }
                let condition = op
                    .ins
                    .get(1)
                    .ok_or("raw conditional branch has no condition")?;
                let guard = self.read(condition, &state)?;
                let taken = self.run(state.clone(), target, remaining - 1)?;
                let fallthrough = self.run(state, index + 1, remaining - 1)?;
                return Ok(RawState {
                    registers: taken.registers.merge(fallthrough.registers, &guard)?,
                    temps: taken.temps.merge(fallthrough.temps, &guard)?,
                    defined: terms::select(guard, taken.defined, fallthrough.defined)?,
                });
            }
            let out = op.out.as_ref().ok_or_else(|| {
                format!("unmodeled raw effect at {:#x} ({:?})", self.address, op.opc)
            })?;
            let args = op
                .ins
                .iter()
                .map(|arg| self.read(arg, &state))
                .collect::<Result<Vec<_>, _>>()?;
            let value = terms::lower(op.opc, out.size, &args)?;
            update_defined(&mut state.defined, op.opc, &args);
            let space = out
                .space
                .as_ref()
                .ok_or("raw output has no storage space")?
                .get_index();
            let dest = if space == self.register {
                &mut state.registers
            } else if Some(space) == self.unique {
                &mut state.temps
            } else {
                return Err(format!(
                    "unmodeled raw write at {:#x} in space {space}",
                    self.address
                ));
            };
            dest.write(out.offset, out.size, &value)?;
            index += 1;
        }
        Ok(state)
    }

    fn read(&self, arg: &VarnodeData, state: &RawState) -> Result<Term, String> {
        let space = arg.space.as_ref().ok_or("raw input has no storage space")?;
        let index = space.get_index();
        if space.get_type() == IPTR_CONSTANT {
            constant(u128::from(arg.offset), arg.size)
        } else if index == self.register {
            state.registers.read(arg.offset, arg.size)
        } else if Some(index) == self.unique {
            state.temps.read(arg.offset, arg.size)
        } else {
            Err(format!(
                "raw input uses unmodeled {} storage",
                space.get_name()
            ))
        }
    }
}

fn constant(value: u128, size: u32) -> Result<Term, String> {
    if !(1..=16).contains(&size) {
        return Err(format!(
            "unsupported symbolic constant width of {size} bytes"
        ));
    }
    Ok(Rc::new(Expr::Const(
        value & crate::value::mask(size),
        size * 8,
    )))
}

pub(super) fn raw_block(
    env: &Env,
    inputs: &ByteFile,
    entry: u64,
    stop: u64,
    output: &Storage,
    step_limit: u64,
) -> Result<Execution, String> {
    if stop <= entry {
        return Err("the symbolic block must end after its first instruction".to_owned());
    }
    let mut state = RawState {
        registers: inputs.clone(),
        temps: ByteFile::default(),
        defined: constant(1, 1)?,
    };
    let register = env.register.get_index();
    let unique = env.unique.as_ref().map(|s| s.get_index());
    let mut pc = entry;
    let mut steps = 0;
    while pc < stop {
        charge(&mut steps, step_limit)?;
        if pc != entry && env.is_entry(pc) {
            return Err(format!(
                "straight-line block crosses another function at {pc:#x}"
            ));
        }
        let instruction = crate::machine::lift(env, pc).map_err(|halt| halt.render())?;
        let next = pc
            .checked_add(instruction.len)
            .filter(|next| *next <= stop)
            .ok_or_else(|| format!("the block end cuts an instruction at {pc:#x}"))?;
        let mut encoded = Instruction {
            address: pc,
            next_address: next,
            code: env.code.get_index(),
            ops: &instruction.ops,
            register,
            unique,
            steps: &mut steps,
            limit: step_limit,
        };
        state = encoded.run(state, 0, RAW_BRANCH_DEPTH)?;
        state.temps.clear();
        pc = next;
    }
    let ret = Address::new(Rc::clone(&env.code), stop);
    if env.prog.read_bytes_at(&ret, 1).as_deref() != Some(&[0xc3]) {
        return Err("the block end is not a one-byte x86 near return".to_owned());
    }
    if output.space.get_index() != register {
        return Err("symbolic return projection is outside the integer register file".to_owned());
    }
    Ok(Execution {
        value: state.registers.read(output.off, output.size)?,
        defined: state.defined,
    })
}

pub(super) fn ir_block(
    program: &Program,
    inputs: &ByteFile,
    register: i32,
    size: u32,
    stop: u64,
    step_limit: u64,
) -> Result<Execution, String> {
    if program.vararg.is_some() {
        return Err("symbolic IR vararg setup is outside the model".to_owned());
    }
    let mut encoder = IrEncoder {
        program,
        inputs,
        register,
        size,
        stop,
        steps: 0,
        step_limit,
    };
    let state = IrState {
        slots: vec![None; program.slots],
        defined: constant(1, 1)?,
    };
    encoder.block(0, None, state, &mut Vec::new())
}

#[derive(Clone)]
struct IrState {
    slots: Vec<Option<Term>>,
    defined: Term,
}

impl IrState {
    fn assign(&mut self, out: &Out, value: Term) -> Result<(), String> {
        if out.backing.is_some() {
            return Err("unmodeled symbolic IR memory write".to_owned());
        }
        if value.width() != out.size * 8 {
            return Err("symbolic SSA assignment has the wrong width".to_owned());
        }
        let slot = self
            .slots
            .get_mut(out.slot)
            .ok_or("symbolic IR output slot is out of range")?;
        if slot.replace(value).is_some() {
            return Err("symbolic IR assigns an SSA slot more than once".to_owned());
        }
        Ok(())
    }
}

struct IrEncoder<'a> {
    program: &'a Program,
    inputs: &'a ByteFile,
    register: i32,
    size: u32,
    stop: u64,
    steps: u64,
    step_limit: u64,
}

impl IrEncoder<'_> {
    fn block(
        &mut self,
        index: usize,
        from: Option<usize>,
        state: IrState,
        path: &mut Vec<usize>,
    ) -> Result<Execution, String> {
        if path.contains(&index) {
            return Err("symbolic IR loop is outside the model".to_owned());
        }
        if path.len() == IR_BRANCH_DEPTH {
            return Err("symbolic IR branch depth exhausted".to_owned());
        }
        charge(&mut self.steps, self.step_limit)?;
        path.push(index);
        let result = self.run(index, from, state, path);
        path.pop();
        result
    }

    fn run(
        &mut self,
        index: usize,
        from: Option<usize>,
        mut state: IrState,
        path: &mut Vec<usize>,
    ) -> Result<Execution, String> {
        let block = self
            .program
            .blocks
            .get(index)
            .ok_or("symbolic IR edge leaves the block list")?;
        if !block.phis.is_empty() {
            let from = from.ok_or("symbolic entry block has a MULTIEQUAL")?;
            let mut values = Vec::with_capacity(block.phis.len());
            for phi in &block.phis {
                charge(&mut self.steps, self.step_limit)?;
                let arg = phi
                    .ins
                    .get(from)
                    .ok_or("symbolic MULTIEQUAL lacks the predecessor input")?;
                let dest = phi
                    .out
                    .as_ref()
                    .ok_or("symbolic MULTIEQUAL has no output")?;
                values.push((dest, self.read(arg, &state)?));
            }
            for (dest, value) in values {
                state.assign(dest, value)?;
            }
        }
        for op in &block.ops {
            charge(&mut self.steps, self.step_limit)?;
            match op.kind {
                Kind::Eval(opcode) => {
                    let out = op.out.as_ref().ok_or("symbolic IR op has no output")?;
                    let args = op
                        .ins
                        .iter()
                        .map(|arg| self.read(arg, &state))
                        .collect::<Result<Vec<_>, _>>()?;
                    let value = terms::lower(opcode, out.size, &args)?;
                    update_defined(&mut state.defined, opcode, &args);
                    state.assign(out, value)?;
                }
                Kind::Branch => return self.edge(block, 0, state, path),
                Kind::CBranch { flip } => {
                    let condition = op.ins.get(1).ok_or("symbolic IR branch has no condition")?;
                    let guard = self.read(condition, &state)?;
                    let yes = self.edge(block, usize::from(!flip), state.clone(), path)?;
                    let no = self.edge(block, usize::from(flip), state, path)?;
                    return Ok(Execution {
                        value: terms::select(Rc::clone(&guard), yes.value, no.value)?,
                        defined: terms::select(guard, yes.defined, no.defined)?,
                    });
                }
                Kind::Return { halt: false } => {
                    let [_address, arg] = op.ins.as_slice() else {
                        return Err("symbolic IR expects one return value".to_owned());
                    };
                    if arg.size != self.size || op.at != self.stop {
                        return Err(
                            "symbolic IR return differs from the chosen return site or width"
                                .to_owned(),
                        );
                    }
                    return Ok(Execution {
                        value: self.read(arg, &state)?,
                        defined: state.defined,
                    });
                }
                _ => {
                    return Err(format!(
                        "unmodeled IR effect at {:#x} ({:?})",
                        op.at, op.kind
                    ))
                }
            }
        }
        self.edge(block, 0, state, path)
    }

    fn edge(
        &mut self,
        block: &Block,
        edge: usize,
        state: IrState,
        path: &mut Vec<usize>,
    ) -> Result<Execution, String> {
        let &(target, from) = block
            .outs
            .get(edge)
            .ok_or("symbolic IR block has no expected successor")?;
        let from =
            usize::try_from(from).map_err(|_| "symbolic IR edge has an invalid predecessor")?;
        self.block(target, Some(from), state, path)
    }

    fn read(&self, arg: &Arg, state: &IrState) -> Result<Term, String> {
        read_ir(arg, self.inputs, self.register, &state.slots)
    }
}

fn read_ir(
    arg: &Arg,
    inputs: &ByteFile,
    register: i32,
    slots: &[Option<Term>],
) -> Result<Term, String> {
    match &arg.op {
        Operand::Const(value) => constant(*value, arg.size),
        Operand::Input(storage) if storage.space.get_index() == register => {
            inputs.read(storage.off, arg.size)
        }
        Operand::Slot(slot, _) => slots
            .get(*slot)
            .and_then(Clone::clone)
            .ok_or_else(|| format!("read of unassigned symbolic IR slot {slot}")),
        _ => Err(format!("unmodeled symbolic IR operand {:?}", arg.op)),
    }
}

fn charge(steps: &mut u64, limit: u64) -> Result<(), String> {
    if *steps >= limit {
        return Err("symbolic block operation budget exhausted".to_owned());
    }
    *steps += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concolic::Emitter;
    use crate::program::{Op, Phi};
    use crate::symbolic::obligation::{Obligation, Verdict};
    use std::process::Command;
    use std::time::{Duration, Instant};

    #[test]
    fn multiequal_uses_the_incoming_edge_order() {
        if Command::new("z3").arg("-version").output().is_err() {
            return;
        }
        let byte = |value| Arg {
            op: Operand::Const(value),
            size: 1,
            storage: None,
        };
        let block = |index, phis, ops, outs| Block {
            index,
            start: 0,
            phis,
            ops,
            outs,
        };
        let control = Op {
            kind: Kind::CBranch { flip: false },
            ins: vec![byte(0), byte(1)],
            out: None,
            at: 0,
        };
        let join = Phi {
            out: Some(Out {
                slot: 0,
                size: 1,
                storage: None,
                backing: None,
            }),
            ins: vec![byte(0x34), byte(0x71)],
        };
        let terminal = Op {
            kind: Kind::Return { halt: false },
            ins: vec![
                byte(0),
                Arg {
                    op: Operand::Slot(0, 0),
                    size: 1,
                    storage: None,
                },
            ],
            out: None,
            at: 0x80,
        };
        let p = Program {
            blocks: vec![
                block(0, vec![], vec![control], vec![(2, 0), (1, 0)]),
                block(1, vec![], vec![], vec![(3, 1)]),
                block(2, vec![], vec![], vec![(3, 0)]),
                block(3, vec![join], vec![terminal], vec![]),
            ],
            slots: 1,
            starts: Default::default(),
            watch: None,
            reached: Default::default(),
            vararg: None,
        };
        let encoded = ir_block(&p, &ByteFile::default(), 0, 1, 0x80, 20).unwrap();
        let reference = Execution {
            value: constant(0x71, 1).unwrap(),
            defined: constant(1, 1).unwrap(),
        };
        let mut emitter = Emitter::default();
        let mismatch = terms::observed_terms(&mut emitter, encoded, reference).unwrap();
        let obligation = Obligation {
            domain: format!("(set-logic QF_BV)\n{}", emitter.definitions()),
            mismatch,
            input_bytes: vec![],
        };
        let mut z3 = Command::new("z3");
        z3.args(["-in", "-smt2"]);
        assert_eq!(
            obligation.solve(&mut z3, Instant::now() + Duration::from_secs(5)),
            Verdict::Equivalent,
        );
    }
}
