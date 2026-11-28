//! Function execution over raw instruction addresses.

use std::collections::BTreeMap;
use std::ops::Range;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype::IPTR_CONSTANT;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::concolic::Expr;
use crate::state::Env;
use crate::storage::Storage;

use super::execute::{charge, constant, Execution, Instruction, RawState, RAW_BRANCH_DEPTH};
use super::terms::Term;

pub(super) const FUNCTION_BRANCH_DEPTH: u8 = 32;

pub(super) struct RawFunction<'a, 'p> {
    pub env: &'a Env<'p>,
    pub extent: Range<u64>,
    pub output: &'a Storage,
    pub stack_pointer: &'a Storage,
    pub indirect: &'a BTreeMap<u64, Vec<u64>>,
    pub step_limit: u64,
    pub loop_bound: Option<u8>,
}

impl RawFunction<'_, '_> {
    pub fn run(&self, state: RawState) -> Result<Execution, String> {
        self.follow(
            self.extent.start,
            state,
            BTreeMap::new(),
            FUNCTION_BRANCH_DEPTH,
            &mut 0,
        )
    }

    fn follow(
        &self,
        mut pc: u64,
        mut state: RawState,
        mut visits: BTreeMap<u64, u8>,
        remaining: u8,
        steps: &mut u64,
    ) -> Result<Execution, String> {
        loop {
            if !self.extent.contains(&pc) {
                return Err(format!(
                    "raw edge {pc:#x} leaves the selected ELF function extent"
                ));
            }
            let seen = visits.entry(pc).or_default();
            if *seen != 0 {
                let bound = self
                    .loop_bound
                    .ok_or_else(|| format!("raw loop at {pc:#x} is outside the acyclic model"))?;
                if *seen >= bound {
                    return Execution::cut(self.output.size, state.defined, state.memory);
                }
            }
            *seen += 1;
            charge(steps, self.step_limit)?;
            if pc != self.extent.start && self.env.is_entry(pc) {
                return Err(format!("raw path enters another function at {pc:#x}"));
            }
            let decoded = crate::machine::lift(self.env, pc).map_err(|halt| halt.render())?;
            let next = pc
                .checked_add(decoded.len)
                .filter(|&end| end <= self.extent.end)
                .ok_or_else(|| format!("raw instruction at {pc:#x} crosses the selected extent"))?;
            let address = Address::new(Rc::clone(&self.env.code), pc);
            if decoded.len == 1
                && self.env.prog.read_bytes_at(&address, 1).as_deref() == Some(&[0xc3])
            {
                let sp = state
                    .registers
                    .read(self.stack_pointer.off, self.stack_pointer.size)?;
                let initial = constant(u128::from(self.env.sp0), self.stack_pointer.size)?;
                let expected_slot = Rc::new(Expr::Test("=", sp, initial));
                let trapped = Rc::new(Expr::Test("=", Rc::clone(&state.defined), constant(0, 1)?));
                state
                    .memory
                    .require(Rc::new(Expr::Op("bvor", vec![trapped, expected_slot], 8)));
                return state.observe(self.output);
            }
            let branch = decoded.ops.last().filter(|op| {
                matches!(
                    op.opc,
                    OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND
                ) && (op.opc == OpCode::CPUI_BRANCHIND
                    || op
                        .ins
                        .first()
                        .is_some_and(|t| self.is_code_target(t) && t.offset != next))
            });
            let body = &decoded.ops[..decoded.ops.len() - usize::from(branch.is_some())];
            let mut instruction =
                Instruction::with_ops(self.env, pc, next, body, steps, self.step_limit);
            state = instruction.run(state, 0, RAW_BRANCH_DEPTH)?;

            if let Some(op) = branch {
                let target = op.ins.first().ok_or("raw machine branch has no target")?;
                let target_value = if op.opc == OpCode::CPUI_BRANCHIND {
                    Some(instruction.read(target, &mut state)?)
                } else {
                    None
                };
                let condition = if op.opc == OpCode::CPUI_CBRANCH {
                    Some(instruction.read(
                        op.ins.get(1).ok_or("raw machine branch has no condition")?,
                        &mut state,
                    )?)
                } else {
                    None
                };
                charge(steps, self.step_limit)?;
                state.finish_instruction();
                if let Some(guard) = condition {
                    if remaining == 0 {
                        return Err("raw function branch depth exhausted".into());
                    }
                    let yes = self.follow(
                        target.offset,
                        state.clone(),
                        visits.clone(),
                        remaining - 1,
                        steps,
                    )?;
                    let no = self.follow(next, state, visits, remaining - 1, steps)?;
                    return yes.join(no, guard);
                }
                if let Some(target_value) = target_value {
                    return self.indirect_edges(pc, target_value, state, visits, remaining, steps);
                }
                pc = target.offset;
            } else {
                state.finish_instruction();
                pc = next;
            }
        }
    }

    fn indirect_edges(
        &self,
        pc: u64,
        target: Term,
        state: RawState,
        visits: BTreeMap<u64, u8>,
        depth: u8,
        steps: &mut u64,
    ) -> Result<Execution, String> {
        if depth == 0 {
            return Err("raw function branch depth exhausted".into());
        }
        let targets = self
            .indirect
            .get(&pc)
            .ok_or_else(|| format!("raw indirect edge at {pc:#x} needs explicit targets"))?;
        if target.width() != 64 {
            return Err("raw indirect target is outside the x86-64 address width".into());
        }
        let mut result = None;
        let mut covered = constant(0, 1)?;
        for &destination in targets.iter().rev() {
            let to = constant(u128::from(destination), target.width() / 8)?;
            let is_target = Rc::new(Expr::Test("=", Rc::clone(&target), to));
            covered = Rc::new(Expr::Op("bvor", vec![covered, Rc::clone(&is_target)], 8));
            let path = self.follow(destination, state.clone(), visits.clone(), depth - 1, steps)?;
            result = Some(match result {
                Some(prior) => path.join(prior, is_target)?,
                None => path,
            });
        }
        let mut result = result.ok_or("raw indirect edge has no targets")?;
        result.memory.require(covered);
        Ok(result)
    }

    fn is_code_target(&self, target: &VarnodeData) -> bool {
        target.space.as_ref().is_some_and(|s| {
            s.get_type() != IPTR_CONSTANT && s.get_index() == self.env.code.get_index()
        })
    }
}
