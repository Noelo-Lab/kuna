//! Inductive obligations for a pair of loop cutpoints.

use std::fmt::Write as _;
use std::ops::Range;
use std::process::Command;
use std::rc::Rc;
use std::time::Instant;

use serde_json::{json, Value};

use crate::concolic::{Emitter, Expr};
use crate::program::Program;
use crate::storage::Storage;
use crate::{oracle, state::Env};

use super::byte_file::ByteFile;
use super::execute::{constant, ir_block, Execution, IrBoundary, RawState};
use super::memory::Memory;
use super::raw_function::{RawBoundary, RawFunction};
use super::solver::{self, Status};
use super::terms::{self, Term};
use super::Options;

pub(super) struct Cutpoint {
    pub raw_address: u64,
    pub ir_block: usize,
    pub carried: Vec<(usize, Storage)>,
}

impl Cutpoint {
    pub fn infer(program: &Program, register_space: i32) -> Result<Self, String> {
        // Reduce the IR to a DAG by removing candidate latch edges. The
        // encoders later require every remaining region to be acyclic.
        let mut color = vec![0u8; program.blocks.len()];
        let mut headers = std::collections::BTreeSet::new();
        let mut stack = vec![(0usize, 0usize)];
        while let Some((index, next)) = stack.last_mut() {
            let block = program
                .blocks
                .get(*index)
                .ok_or("induction encountered an invalid IR edge")?;
            color[*index] = 1;
            if let Some(&(to, _)) = block.outs.get(*next) {
                *next += 1;
                match color.get(to).copied() {
                    Some(0) => stack.push((to, 0)),
                    Some(1) => {
                        headers.insert(to);
                    }
                    Some(2) => {}
                    _ => return Err("induction encountered an invalid IR edge".into()),
                }
            } else {
                color[*index] = 2;
                stack.pop();
            }
        }
        let [header] = headers.into_iter().collect::<Vec<_>>()[..] else {
            return Err("induction requires one reachable IR loop header".into());
        };
        let block = &program.blocks[header];
        let carried = block
            .phis
            .iter()
            .map(|phi| {
                let out = phi.out.as_ref().ok_or("header phi has no output")?;
                let storage = out
                    .storage
                    .as_ref()
                    .filter(|storage| {
                        storage.space.get_index() == register_space
                            && matches!(storage.size, 1 | 2 | 4 | 8)
                            && storage.size == out.size
                            && out.backing.is_none()
                    })
                    .ok_or("induction requires register-backed header phi values")?;
                Ok((out.slot, storage.clone()))
            })
            .collect::<Result<Vec<_>, String>>()?;
        if carried.is_empty() || carried.len() > 8 {
            return Err("induction needs one to eight register-backed loop-carried values".into());
        }
        for (i, (_, left)) in carried.iter().enumerate() {
            if carried[..i].iter().any(|(_, right)| overlap(left, right)) {
                return Err("loop-carried register projections overlap".into());
            }
        }
        Ok(Self {
            raw_address: block.start,
            ir_block: header,
            carried,
        })
    }
}

pub(super) fn overlap(left: &Storage, right: &Storage) -> bool {
    left.space.get_index() == right.space.get_index()
        && u128::from(left.off) < u128::from(right.off) + u128::from(right.size)
        && u128::from(right.off) < u128::from(left.off) + u128::from(left.size)
}

pub(super) struct Boundary {
    pub reached: Term,
    pub values: Vec<Term>,
}

impl Boundary {
    pub fn merge(
        left: Option<Self>,
        right: Option<Self>,
        guard: &Term,
    ) -> Result<Option<Self>, String> {
        let zero = || constant(0, 1);
        let select = |yes, no| terms::select(Rc::clone(guard), yes, no);
        Ok(match (left, right) {
            (Some(left), Some(right)) if left.values.len() == right.values.len() => Some(Self {
                reached: select(left.reached, right.reached)?,
                values: left
                    .values
                    .into_iter()
                    .zip(right.values)
                    .map(|(left, right)| select(left, right))
                    .collect::<Result<_, _>>()?,
            }),
            (Some(left), None) => Some(Self {
                reached: select(left.reached, zero()?)?,
                values: left.values,
            }),
            (None, Some(right)) => Some(Self {
                reached: select(zero()?, right.reached)?,
                values: right.values,
            }),
            (None, None) => None,
            _ => return Err("symbolic cutpoint states have different projections".into()),
        })
    }
}

pub(super) struct Induction {
    pub header: Value,
    pub checks: Vec<(&'static str, String, Status)>,
    pub ranks: Vec<(usize, u8, String)>,
}

impl Induction {
    pub fn solve(self, solver: &mut Command, deadline: Instant) -> Value {
        let mut report =
            json!({"classification":"INCONCLUSIVE","cutpoint":self.header,"queries":[]});
        let record = |report: &mut Value, kind: &str, text: &str, expected: &str| {
            report["queries"].as_array_mut().expect("queries is an array").push(json!({
                "kind":kind,"expected":expected,"smt2":text,"sha256":crate::oracle::sha256(text.as_bytes())
            }));
        };
        for (kind, text, expected) in self.checks {
            record(
                &mut report,
                kind,
                &text,
                if expected == Status::Sat {
                    "sat"
                } else {
                    "unsat"
                },
            );
            let status = solver::run(solver, &text, deadline);
            record_status(&mut report, &status);
            match status {
                Ok((status, _)) if status == expected => {}
                other => {
                    report["phase"] = json!(kind);
                    report["reason"] = json!(solver_reason(other));
                    return report;
                }
            }
        }
        let mut reason =
            "no inferred unsigned ranking term strictly decreases at the cutpoint".to_owned();
        for (index, bias, text) in self.ranks {
            record(&mut report, "rank-decrease", &text, "unsat");
            let rank = json!({"carried_index":index,"subtract":bias});
            last_query(&mut report)["rank"] = rank.clone();
            let status = solver::run(solver, &text, deadline);
            record_status(&mut report, &status);
            match status {
                Ok((Status::Unsat, _)) => {
                    report["ranking"] = rank;
                    report["classification"] = json!("MODEL_EQUIVALENCE");
                    return report;
                }
                Ok((Status::Sat, _)) => {}
                other => {
                    reason = solver_reason(other);
                    break;
                }
            }
        }
        report["phase"] = json!("rank-decrease");
        report["reason"] = json!(reason);
        report
    }
}

fn last_query(report: &mut Value) -> &mut Value {
    report["queries"]
        .as_array_mut()
        .expect("queries is an array")
        .last_mut()
        .expect("recorded query")
}

fn record_status(report: &mut Value, answer: &Result<(Status, String), solver::Error>) {
    let result = match answer {
        Ok((Status::Sat, _)) => "sat",
        Ok((Status::Unsat, _)) => "unsat",
        Ok((Status::Unknown(_), _)) => "unknown",
        Err(solver::Error::Timeout) => "timeout",
        Err(_) => "solver-error",
    };
    last_query(report)["result"] = json!(result);
}

pub(super) struct Function<'a, 'p> {
    pub env: &'a Env<'p>,
    pub extent: Range<u64>,
    pub program: &'a Program,
    pub inputs: &'a ByteFile,
    pub output: &'a Storage,
    pub stack_pointer: &'a Storage,
    pub options: &'a Options<'a>,
}

impl Function<'_, '_> {
    pub fn infer(self, domain: String) -> Result<Induction, String> {
        let cut = Cutpoint::infer(self.program, self.env.register.get_index())?;
        let preserved = self
            .options
            .inputs
            .iter()
            .map(|reg| oracle::register(self.env, reg))
            .chain(std::iter::once(Ok(self.stack_pointer.clone())))
            .map(|register| {
                let register = register?;
                match cut
                    .carried
                    .iter()
                    .find(|(_, state)| overlap(&register, state))
                {
                    None => Ok(Some(register)),
                    Some((_, state))
                        if state.covers(&register) && !overlap(&register, self.stack_pointer) =>
                    {
                        Ok(None)
                    }
                    _ => Err(
                        "carried state partly overlaps an original input or the stack pointer"
                            .to_owned(),
                    ),
                }
            })
            .filter_map(Result::transpose)
            .collect::<Result<Vec<_>, _>>()?;
        let registers = cut
            .carried
            .iter()
            .map(|(_, reg)| reg.clone())
            .chain(preserved.iter().cloned())
            .collect::<Vec<_>>();
        let memory = Memory::default();
        let original = preserved
            .iter()
            .map(|r| self.inputs.read(r.off, r.size))
            .collect::<Result<Vec<_>, _>>()?;
        let raw = |registers_at_entry: &ByteFile, resume| {
            RawFunction {
                env: self.env,
                extent: self.extent.clone(),
                output: self.output,
                stack_pointer: self.stack_pointer,
                indirect: self.options.indirect,
                step_limit: self.options.step_limit,
                loop_bound: None,
                boundary: Some(RawBoundary {
                    address: cut.raw_address,
                    registers: &registers,
                    resume,
                }),
            }
            .run(RawState::new(registers_at_entry, &memory)?)
        };
        let ir = |seeds: Option<&[(usize, Term)]>| {
            ir_block(
                self.program,
                self.inputs,
                &memory,
                self.env,
                self.output.size,
                self.options,
                Some(IrBoundary {
                    block: cut.ir_block,
                    carried: &cut.carried,
                    seeds,
                }),
            )
        };
        let prefix_raw = raw(self.inputs, false)?;
        let prefix_ir = ir(None)?;
        let mut prefix = Emitter::default();
        let prefix_defect =
            synchronization_defect(&mut prefix, &prefix_raw, &prefix_ir, &original)?;
        let cut_reachable = reaches(&mut prefix, &prefix_raw);

        let mut arbitrary = self.inputs.clone();
        let mut step_domain = domain.clone();
        let mut seed_slots = Vec::with_capacity(cut.carried.len());
        for (index, (slot, reg)) in cut.carried.iter().enumerate() {
            let value = fresh(&mut step_domain, &format!("carry{index}"), reg.size);
            arbitrary.write(reg.off, reg.size, &value)?;
            seed_slots.push((*slot, value));
        }
        for (index, (flag, _)) in oracle::SYMBOLIC_FLAGS.iter().enumerate() {
            let reg = oracle::register(self.env, flag)?;
            let name = format!("flag{index}");
            let value = fresh(&mut step_domain, &name, 1);
            let _ = writeln!(step_domain, "(assert (bvule {name}_0 #x01))");
            arbitrary.write(reg.off, reg.size, &value)?;
        }
        let step_raw = raw(&arbitrary, true)?;
        let step_ir = ir(Some(&seed_slots))?;
        let mut step = Emitter::default();
        let step_defect = synchronization_defect(&mut step, &step_raw, &step_ir, &original)?;
        let checks = vec![
            (
                "initial-domain",
                query(&domain, &prefix, "true"),
                Status::Sat,
            ),
            (
                "loop-initiation-and-prefix-exits",
                query(&domain, &prefix, &prefix_defect),
                Status::Unsat,
            ),
            (
                "reachable-loop-cutpoint",
                query(&domain, &prefix, &cut_reachable),
                Status::Sat,
            ),
            (
                "loop-preservation-and-suffix-exits",
                query(&step_domain, &step, &step_defect),
                Status::Unsat,
            ),
        ];
        let mut ranks = Vec::new();
        let continue_raw = reaches(&mut step, &step_raw);
        if let Some(next) = &step_raw.boundary {
            for (index, ((_, now), then)) in seed_slots.iter().zip(&next.values).enumerate() {
                let width = now.width();
                let from = step.name(now);
                let to = step.name(then);
                for bias in [0, 1] {
                    let less = if bias == 0 {
                        format!("(bvult {to} {from})")
                    } else {
                        format!("(bvult (bvsub {to} (_ bv{bias} {width})) (bvsub {from} (_ bv{bias} {width})))")
                    };
                    let condition = format!("(and {continue_raw} (not {less}))");
                    ranks.push((index, bias, query(&step_domain, &step, &condition)));
                }
            }
        }
        Ok(Induction {
            header: json!({
                "raw_address": format!("{:#x}", cut.raw_address), "ir_block": cut.ir_block,
                "carried": cut.carried.iter().map(|(slot, reg)| json!({
                    "slot":slot, "raw_register_offset":format!("{:#x}", reg.off), "bytes":reg.size
                })).collect::<Vec<_>>(),
                "preserved_register_offsets":preserved.iter().map(|reg| json!({
                    "offset":format!("{:#x}", reg.off), "bytes":reg.size
                })).collect::<Vec<_>>(),
            }),
            checks,
            ranks,
        })
    }
}

fn synchronization_defect(
    emitter: &mut Emitter,
    raw: &Execution,
    ir: &Execution,
    preserved: &[Term],
) -> Result<String, String> {
    let base = synchronize(emitter, raw, ir);
    let (Some(raw_cut), Some(ir_cut)) = (&raw.boundary, &ir.boundary) else {
        return Ok(base);
    };
    if raw_cut.values.len() != ir_cut.values.len() + preserved.len() {
        return Err("raw and IR cutpoint state lengths do not match".into());
    }
    let differences = raw_cut
        .values
        .iter()
        .zip(ir_cut.values.iter().chain(preserved))
        .map(|(left, right)| format!("(distinct {} {})", emitter.name(left), emitter.name(right)))
        .collect::<Vec<_>>()
        .join(" ");
    let cut = reaches(emitter, raw);
    Ok(format!("(or {base} (and {cut} (or {differences})))"))
}

fn solver_reason(other: Result<(Status, String), solver::Error>) -> String {
    match other {
        Ok((Status::Sat, _)) => "solver found a state violating the induction condition".into(),
        Ok((Status::Unsat, _)) => "induction condition has an empty domain".into(),
        Ok((Status::Unknown(reason), _)) => reason,
        Err(solver::Error::Timeout) => "solver wall-clock deadline".into(),
        Err(solver::Error::Unavailable(reason) | solver::Error::Failed(reason)) => reason,
    }
}

pub(super) fn query(declarations: &str, emitter: &Emitter, assertion: &str) -> String {
    format!(
        "{declarations}{}\n(assert {assertion})\n(check-sat)\n(get-info :reason-unknown)\n",
        emitter.definitions()
    )
}

/// Both sides must reach the same kind of terminal point. Every symbolic
/// read and return before it must be covered by the machine model.
pub(super) fn synchronize(emitter: &mut Emitter, left: &Execution, right: &Execution) -> String {
    let mut problems = Vec::new();
    for side in [left, right] {
        problems.push(format!("(= {} #x00)", emitter.name(&side.defined)));
        problems.push(format!(
            "(= {} #x00)",
            emitter.name(&side.memory.coverage())
        ));
    }
    let raw = reaches(emitter, left);
    let ir = reaches(emitter, right);
    problems.push(format!("(distinct {raw} {ir})"));
    let raw_value = emitter.name(&left.value);
    let ir_value = emitter.name(&right.value);
    problems.push(format!(
        "(and (not {raw}) (not {ir}) (distinct {raw_value} {ir_value}))"
    ));
    format!("(or {})", problems.join(" "))
}

pub(super) fn reaches(emitter: &mut Emitter, execution: &Execution) -> String {
    execution.boundary.as_ref().map_or_else(
        || "false".to_owned(),
        |state| format!("(distinct {} #x00)", emitter.name(&state.reached)),
    )
}

pub(super) fn fresh(declarations: &mut String, name: &str, bytes: u32) -> Term {
    let mut parts = Vec::new();
    for index in 0..bytes {
        let byte = format!("{name}_{index}");
        let _ = writeln!(declarations, "(declare-const {byte} (_ BitVec 8))");
        parts.push(Rc::new(Expr::Var(Rc::from(byte))));
    }
    if parts.len() == 1 {
        parts.pop().expect("one byte")
    } else {
        Rc::new(Expr::Concat(parts))
    }
}
