//! Independent raw storage execution and final-IR SSA execution.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    spacetype::{IPTR_CONSTANT, IPTR_SPACEBASE},
    AddrSpace,
};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::concolic::Expr;
use crate::program::{Arg, Block, Kind, Operand, Out, Program, Switch};
use crate::state::Env;
use crate::storage::Storage;

use super::byte_file::ByteFile;
use super::induction::Boundary;
use super::memory::{Memory, ReadMode};
use super::terms::{self, Term};

pub(super) const RAW_BRANCH_DEPTH: u8 = 32;
pub(super) const IR_BRANCH_DEPTH: usize = 64;

pub(super) struct Execution {
    pub value: Term,
    pub defined: Term,
    pub memory: Memory,
    pub completed: Term,
    pub boundary: Option<Boundary>,
}

impl Execution {
    pub fn cut(bytes: u32, defined: Term, memory: Memory) -> Result<Self, String> {
        let completed = Rc::new(Expr::Test("=", Rc::clone(&defined), constant(0, 1)?));
        Ok(Self {
            value: constant(0, bytes)?,
            defined,
            memory,
            completed,
            boundary: None,
        })
    }

    pub fn at_boundary(
        bytes: u32,
        defined: Term,
        memory: Memory,
        values: Vec<Term>,
    ) -> Result<Self, String> {
        Ok(Self {
            value: constant(0, bytes)?,
            defined,
            memory,
            completed: constant(0, 1)?,
            boundary: Some(Boundary {
                reached: constant(1, 1)?,
                values,
            }),
        })
    }

    pub fn join(self, other: Self, guard: Term) -> Result<Self, String> {
        Ok(Self {
            value: terms::select(Rc::clone(&guard), self.value, other.value)?,
            memory: self.memory.join(other.memory, &guard)?,
            completed: terms::select(Rc::clone(&guard), self.completed, other.completed)?,
            boundary: Boundary::merge(self.boundary, other.boundary, &guard)?,
            defined: terms::select(guard, self.defined, other.defined)?,
        })
    }
}

fn update_defined(defined: &mut Term, opcode: OpCode, args: &[Term]) {
    if let Some(guard) = terms::division_guard(opcode, args) {
        *defined = Rc::new(Expr::Op("bvand", vec![Rc::clone(defined), guard], 8));
    }
}

#[derive(Clone)]
pub(super) struct RawState {
    pub registers: ByteFile,
    temps: ByteFile,
    pub defined: Term,
    pub memory: Memory,
}

impl RawState {
    pub fn new(inputs: &ByteFile, memory: &Memory) -> Result<Self, String> {
        Ok(Self {
            registers: inputs.clone(),
            temps: ByteFile::default(),
            defined: constant(1, 1)?,
            memory: memory.clone(),
        })
    }

    pub fn observe(self, output: &Storage) -> Result<Execution, String> {
        Ok(Execution {
            value: self.registers.read(output.off, output.size)?,
            defined: self.defined,
            memory: self.memory,
            completed: constant(1, 1)?,
            boundary: None,
        })
    }

    pub fn finish_instruction(&mut self) {
        self.temps.clear();
    }
}

pub(super) struct Instruction<'a> {
    address: u64,
    next_address: u64,
    code: i32,
    ram: &'a AddrSpace,
    ops: &'a [crate::machine::RawOp],
    register: i32,
    unique: Option<i32>,
    steps: &'a mut u64,
    limit: u64,
}

impl Instruction<'_> {
    pub fn with_ops<'a>(
        env: &'a Env,
        address: u64,
        next_address: u64,
        ops: &'a [crate::machine::RawOp],
        steps: &'a mut u64,
        limit: u64,
    ) -> Instruction<'a> {
        Instruction {
            address,
            next_address,
            ops,
            steps,
            limit,
            code: env.code.get_index(),
            ram: &env.ram,
            register: env.register.get_index(),
            unique: env.unique.as_ref().map(|s| s.get_index()),
        }
    }

    pub fn run(
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
                let guard = self.read(condition, &mut state)?;
                let taken = self.run(state.clone(), target, remaining - 1)?;
                let fallthrough = self.run(state, index + 1, remaining - 1)?;
                return Ok(RawState {
                    registers: taken.registers.merge(fallthrough.registers, &guard)?,
                    temps: taken.temps.merge(fallthrough.temps, &guard)?,
                    memory: taken.memory.join(fallthrough.memory, &guard)?,
                    defined: terms::select(guard, taken.defined, fallthrough.defined)?,
                });
            }
            if op.opc == OpCode::CPUI_STORE {
                let [space, ptr, value] = op.ins.as_slice() else {
                    return Err("malformed raw STORE".into());
                };
                self.check_ram(space)?;
                let ptr = self.read(ptr, &mut state)?;
                let value = self.read(value, &mut state)?;
                state
                    .memory
                    .store(ptr, &value, self.ram.is_big_endian(), &state.defined)?;
                index += 1;
                continue;
            }
            let out = op.out.as_ref().ok_or_else(|| {
                format!("unmodeled raw effect at {:#x} ({:?})", self.address, op.opc)
            })?;
            let value = if op.opc == OpCode::CPUI_LOAD {
                let [space, ptr] = op.ins.as_slice() else {
                    return Err("malformed raw LOAD".into());
                };
                self.check_ram(space)?;
                let ptr = self.read(ptr, &mut state)?;
                state.memory.load(
                    ptr,
                    out.size,
                    self.ram.is_big_endian(),
                    &state.defined,
                    ReadMode::Machine,
                )?
            } else {
                let args = op
                    .ins
                    .iter()
                    .map(|arg| self.read(arg, &mut state))
                    .collect::<Result<Vec<_>, _>>()?;
                let value = terms::lower(op.opc, out.size, &args)?;
                update_defined(&mut state.defined, op.opc, &args);
                value
            };
            let space = out
                .space
                .as_ref()
                .ok_or("raw output has no storage space")?
                .get_index();
            if space == self.register {
                state.registers.write(out.offset, out.size, &value)?;
            } else if Some(space) == self.unique {
                state.temps.write(out.offset, out.size, &value)?;
            } else if space == self.ram.get_index() && self.ram.get_word_size() == 1 {
                state.memory.store(
                    constant(u128::from(out.offset), 8)?,
                    &value,
                    self.ram.is_big_endian(),
                    &state.defined,
                )?;
            } else {
                return Err(format!(
                    "unmodeled raw write at {:#x} in space {space}",
                    self.address
                ));
            }
            index += 1;
        }
        Ok(state)
    }

    fn check_ram(&self, space: &VarnodeData) -> Result<(), String> {
        if space.offset != self.ram.get_index() as u64
            || !space
                .space
                .as_ref()
                .is_some_and(|s| s.get_type() == IPTR_CONSTANT)
            || self.ram.get_word_size() != 1
        {
            return Err("raw access is outside byte-addressed RAM".into());
        }
        Ok(())
    }

    pub fn read(&self, arg: &VarnodeData, state: &mut RawState) -> Result<Term, String> {
        let space = arg.space.as_ref().ok_or("raw input has no storage space")?;
        let index = space.get_index();
        if space.get_type() == IPTR_CONSTANT {
            constant(u128::from(arg.offset), arg.size)
        } else if index == self.register {
            state.registers.read(arg.offset, arg.size)
        } else if Some(index) == self.unique {
            state.temps.read(arg.offset, arg.size)
        } else if index == self.ram.get_index() && self.ram.get_word_size() == 1 {
            state.memory.load(
                constant(u128::from(arg.offset), 8)?,
                arg.size,
                self.ram.is_big_endian(),
                &state.defined,
                ReadMode::Machine,
            )
        } else {
            Err(format!(
                "raw input uses unmodeled {} storage",
                space.get_name()
            ))
        }
    }
}

pub(super) fn constant(value: u128, size: u32) -> Result<Term, String> {
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
    memory: &Memory,
    entry: u64,
    stop: u64,
    output: &Storage,
    step_limit: u64,
) -> Result<Execution, String> {
    if stop <= entry {
        return Err("the symbolic block must end after its first instruction".to_owned());
    }
    let mut state = RawState::new(inputs, memory)?;
    let register = env.register.get_index();
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
        let mut encoded =
            Instruction::with_ops(env, pc, next, &instruction.ops, &mut steps, step_limit);
        state = encoded.run(state, 0, RAW_BRANCH_DEPTH)?;
        state.finish_instruction();
        pc = next;
    }
    let ret = Address::new(Rc::clone(&env.code), stop);
    if env.prog.read_bytes_at(&ret, 1).as_deref() != Some(&[0xc3]) {
        return Err("the block end is not a one-byte x86 near return".to_owned());
    }
    if output.space.get_index() != register {
        return Err("symbolic return projection is outside the integer register file".to_owned());
    }
    state.observe(output)
}

pub(super) fn ir_block(
    program: &Program,
    inputs: &ByteFile,
    memory: &Memory,
    env: &Env,
    size: u32,
    options: &super::Options<'_>,
    boundary: Option<IrBoundary<'_>>,
) -> Result<Execution, String> {
    if program.vararg.is_some() {
        return Err("symbolic IR vararg setup is outside the model".to_owned());
    }
    let mut encoder = IrEncoder {
        program,
        inputs,
        locations: IrLocations {
            register: env.register.get_index(),
            ram: env.ram.get_index(),
            sp0: env.sp0,
            ram_big_endian: env.ram.is_big_endian(),
        },
        size,
        stop: options.stop,
        steps: 0,
        step_limit: options.step_limit,
        loop_bound: options.loop_bound,
        boundary,
    };
    let mut state = IrState {
        slots: vec![None; program.slots],
        defined: constant(1, 1)?,
        memory: memory.clone(),
    };
    let mut start = 0;
    if let Some(boundary) = boundary {
        if let Some(seeds) = boundary.seeds {
            for (slot, value) in seeds {
                state.slots[*slot] = Some(Rc::clone(value));
            }
            start = boundary.block;
        }
    }
    encoder.block(start, None, state, &mut Vec::new())
}

#[derive(Clone, Copy)]
pub(super) struct IrBoundary<'a> {
    pub block: usize,
    pub carried: &'a [(usize, Storage)],
    pub seeds: Option<&'a [(usize, Term)]>,
}

#[derive(Clone)]
struct IrState {
    slots: Vec<Option<Term>>,
    defined: Term,
    memory: Memory,
}

impl IrState {
    fn assign(&mut self, out: &Out, value: Term) -> Result<(), String> {
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
    locations: IrLocations,
    size: u32,
    stop: Option<u64>,
    steps: u64,
    step_limit: u64,
    loop_bound: Option<u8>,
    boundary: Option<IrBoundary<'a>>,
}

struct IrLocations {
    register: i32,
    ram: i32,
    sp0: u64,
    ram_big_endian: bool,
}

impl IrLocations {
    fn fixed_address(&self, storage: &Storage) -> Result<Term, String> {
        let offset = if storage.space.get_type() == IPTR_SPACEBASE {
            self.sp0.wrapping_add(storage.off)
        } else if storage.space.get_index() == self.ram {
            storage.off
        } else {
            return Err("symbolic IR storage is outside RAM and the stack".to_owned());
        };
        constant(u128::from(offset), 8)
    }

    fn check_ram(&self, space: Option<&Rc<AddrSpace>>) -> Result<(), String> {
        if !space.is_some_and(|s| s.get_index() == self.ram && s.get_word_size() == 1) {
            return Err("symbolic IR access is outside byte-addressed RAM".to_owned());
        }
        Ok(())
    }
}

impl IrEncoder<'_> {
    fn block(
        &mut self,
        index: usize,
        from: Option<usize>,
        state: IrState,
        path: &mut Vec<usize>,
    ) -> Result<Execution, String> {
        let at_boundary = self.boundary.is_some_and(|b| b.block == index);
        let visits = path.iter().filter(|&&previous| previous == index).count();
        if visits != 0 && !at_boundary {
            let bound = self
                .loop_bound
                .ok_or("symbolic IR loop is outside the model")?;
            if visits >= usize::from(bound) {
                return Execution::cut(self.size, state.defined, state.memory);
            }
        }
        if path.len() == IR_BRANCH_DEPTH {
            return Err("symbolic IR branch depth exhausted".to_owned());
        }
        charge(&mut self.steps, self.step_limit)?;
        path.push(index);
        let result = self.run(index, from, state, path, visits != 0);
        path.pop();
        result
    }

    fn run(
        &mut self,
        index: usize,
        from: Option<usize>,
        mut state: IrState,
        path: &mut Vec<usize>,
        revisited: bool,
    ) -> Result<Execution, String> {
        let block = self
            .program
            .blocks
            .get(index)
            .ok_or("symbolic IR edge leaves the block list")?;
        let boundary = self.boundary.filter(|b| b.block == index);
        let seeded = boundary.is_some_and(|b| b.seeds.is_some() && from.is_none());
        let mut values = Vec::with_capacity(block.phis.len());
        if !seeded && !block.phis.is_empty() {
            let from = from.ok_or("symbolic entry block has a MULTIEQUAL")?;
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
                values.push((dest, self.read(arg, &mut state)?));
            }
        }
        if revisited && boundary.is_none() {
            for dest in values
                .iter()
                .map(|(dest, _)| *dest)
                .chain(block.ops.iter().filter_map(|op| op.out.as_ref()))
            {
                *state
                    .slots
                    .get_mut(dest.slot)
                    .ok_or("symbolic IR output slot is out of range")? = None;
            }
        }
        if let Some(capture) = boundary.filter(|_| !seeded) {
            for (out, value) in &values {
                self.write_backing(&mut state, out, value)?;
            }
            let carried = capture
                .carried
                .iter()
                .map(|(slot, _)| {
                    let value = values
                        .iter()
                        .find(|(out, _)| out.slot == *slot)
                        .ok_or("loop cutpoint lacks a carried phi value")?;
                    Ok(Rc::clone(&value.1))
                })
                .collect::<Result<_, String>>()?;
            return Execution::at_boundary(self.size, state.defined, state.memory, carried);
        }
        for (dest, value) in values {
            self.assign(&mut state, dest, value)?;
        }
        for op in &block.ops {
            charge(&mut self.steps, self.step_limit)?;
            match &op.kind {
                Kind::Eval(opcode) => {
                    let out = op.out.as_ref().ok_or("symbolic IR op has no output")?;
                    let args = op
                        .ins
                        .iter()
                        .map(|arg| self.read(arg, &mut state))
                        .collect::<Result<Vec<_>, _>>()?;
                    let value = terms::lower(*opcode, out.size, &args)?;
                    update_defined(&mut state.defined, *opcode, &args);
                    self.assign(&mut state, out, value)?;
                }
                Kind::Load(space) => {
                    self.locations.check_ram(space.as_ref())?;
                    let [ptr] = op.ins.as_slice() else {
                        return Err("malformed symbolic IR LOAD".into());
                    };
                    let out = op.out.as_ref().ok_or("symbolic IR LOAD lacks an output")?;
                    let ptr = self.read(ptr, &mut state)?;
                    let value = state.memory.load(
                        ptr,
                        out.size,
                        self.locations.ram_big_endian,
                        &state.defined,
                        ReadMode::IrCurrent,
                    )?;
                    self.assign(&mut state, out, value)?;
                }
                Kind::Store(space) => {
                    self.locations.check_ram(space.as_ref())?;
                    let [ptr, value] = op.ins.as_slice() else {
                        return Err("malformed symbolic IR STORE".into());
                    };
                    let ptr = self.read(ptr, &mut state)?;
                    let value = self.read(value, &mut state)?;
                    state.memory.store(
                        ptr,
                        &value,
                        self.locations.ram_big_endian,
                        &state.defined,
                    )?;
                }
                Kind::Branch => return self.edge(block, 0, state, path),
                Kind::CBranch { flip } => {
                    let condition = op.ins.get(1).ok_or("symbolic IR branch has no condition")?;
                    let guard = self.read(condition, &mut state)?;
                    let yes = self.edge(block, usize::from(!*flip), state.clone(), path)?;
                    let no = self.edge(block, usize::from(*flip), state, path)?;
                    return yes.join(no, guard);
                }
                Kind::BranchInd { switch, targets } => {
                    if self.stop.is_some() {
                        return Err("indirect IR edges require whole-function mode".into());
                    }
                    let arg = op
                        .ins
                        .first()
                        .ok_or("symbolic indirect edge lacks a selector")?;
                    let selector = self.read(arg, &mut state)?;
                    return self.indirect(block, switch.as_ref(), targets, selector, state, path);
                }
                Kind::Return { halt: false } => {
                    let [_address, arg] = op.ins.as_slice() else {
                        return Err("symbolic IR expects one return value".to_owned());
                    };
                    if arg.size != self.size || self.stop.is_some_and(|stop| op.at != stop) {
                        return Err(
                            "symbolic IR return differs from the chosen return site or width"
                                .to_owned(),
                        );
                    }
                    return Ok(Execution {
                        value: self.read(arg, &mut state)?,
                        defined: state.defined,
                        memory: state.memory,
                        completed: constant(1, 1)?,
                        boundary: None,
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

    fn indirect(
        &mut self,
        block: &Block,
        switch: Option<&Switch>,
        targets: &[u64],
        selector: Term,
        state: IrState,
        path: &mut Vec<usize>,
    ) -> Result<Execution, String> {
        let (cases, fallback) = match switch {
            Some(table) => {
                let rows = table
                    .rows
                    .iter()
                    .map(|&(label, edge)| {
                        let edge = edge
                            .and_then(|e| usize::try_from(e).ok())
                            .ok_or("symbolic switch row lacks a modeled edge")?;
                        Ok((label, edge))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                let default = table
                    .default
                    .map(usize::try_from)
                    .transpose()
                    .map_err(|_| "symbolic switch default edge is invalid")?;
                (rows, default)
            }
            None => (
                {
                    if selector.width() != 64 {
                        return Err("symbolic indirect address is outside the x86-64 width".into());
                    }
                    targets
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(edge, to)| (to, edge))
                        .collect()
                },
                None,
            ),
        };
        if cases.len() > 16 {
            return Err("symbolic IR indirect edge needs at most sixteen target rows".into());
        }
        let mut result = fallback
            .map(|edge| self.edge(block, edge, state.clone(), path))
            .transpose()?;
        let mut covered = constant(u128::from(fallback.is_some()), 1)?;
        for (label, edge) in cases.into_iter().rev() {
            let label = constant(u128::from(label), selector.width() / 8)?;
            let guard = Rc::new(Expr::Test("=", Rc::clone(&selector), label));
            covered = Rc::new(Expr::Op("bvor", vec![covered, Rc::clone(&guard)], 8));
            let selected = self.edge(block, edge, state.clone(), path)?;
            result = Some(match result {
                Some(other) => selected.join(other, guard)?,
                None => selected,
            });
        }
        let mut result = result.ok_or("symbolic indirect IR edge has no modeled target")?;
        result.memory.require(covered);
        Ok(result)
    }

    fn read(&self, arg: &Arg, state: &mut IrState) -> Result<Term, String> {
        let result = match &arg.op {
            Operand::Const(value) => constant(*value, arg.size)?,
            Operand::Input(storage) if storage.space.get_index() == self.locations.register => {
                self.inputs.read(storage.off, arg.size)?
            }
            Operand::Input(storage) | Operand::Memory(storage) | Operand::Free(storage) => {
                let address = self.locations.fixed_address(storage)?;
                let initial = matches!(arg.op, Operand::Input(_));
                state.memory.load(
                    address,
                    arg.size,
                    self.locations.ram_big_endian,
                    &state.defined,
                    if initial {
                        ReadMode::IrInitial
                    } else {
                        ReadMode::IrCurrent
                    },
                )?
            }
            Operand::Slot(slot, _) => state
                .slots
                .get(*slot)
                .and_then(Clone::clone)
                .ok_or_else(|| format!("read of unassigned symbolic IR slot {slot}"))?,
            _ => return Err(format!("unmodeled symbolic IR operand {:?}", arg.op)),
        };
        if result.width() != arg.size * 8 {
            return Err("symbolic IR operand width differs from its SSA value".into());
        }
        Ok(result)
    }

    fn assign(&self, state: &mut IrState, out: &Out, value: Term) -> Result<(), String> {
        self.write_backing(state, out, &value)?;
        state.assign(out, value)
    }

    fn write_backing(&self, state: &mut IrState, out: &Out, value: &Term) -> Result<(), String> {
        if let Some(storage) = &out.backing {
            let address = self.locations.fixed_address(storage)?;
            state.memory.store(
                address,
                value,
                self.locations.ram_big_endian,
                &state.defined,
            )?;
        }
        Ok(())
    }
}

pub(super) fn charge(steps: &mut u64, limit: u64) -> Result<(), String> {
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
        let registers = ByteFile::default();
        let mut encoder = IrEncoder {
            program: &p,
            inputs: &registers,
            locations: IrLocations {
                register: 0,
                ram: 1,
                sp0: 0,
                ram_big_endian: false,
            },
            size: 1,
            stop: Some(0x80),
            steps: 0,
            step_limit: 20,
            loop_bound: None,
            boundary: None,
        };
        let initial = IrState {
            slots: vec![None; 1],
            defined: constant(1, 1).unwrap(),
            memory: Memory::default(),
        };
        let encoded = encoder.block(0, None, initial, &mut Vec::new()).unwrap();
        let reference = Execution {
            value: constant(0x71, 1).unwrap(),
            defined: constant(1, 1).unwrap(),
            memory: Memory::default(),
            completed: constant(1, 1).unwrap(),
            boundary: None,
        };
        let mut emitter = Emitter::default();
        let observations = terms::observed_terms(&mut emitter, encoded, reference, false).unwrap();
        let obligation = Obligation {
            domain: format!("(set-logic QF_BV)\n{}", emitter.definitions()),
            mismatch: observations.mismatch,
            coverage_gap: observations.coverage_gap,
            completion_gap: observations.completion_gap,
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
