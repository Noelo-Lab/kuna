//! Bounded i386 table successors proved from a local unsigned guard and the
//! computed jump's own pointer load. Missing or ambiguous evidence declines.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;
use object::Object;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{is_constant, key_of, Key, DEFAULT_MAX_ENTRIES, MIN_ENTRIES};
use super::model::FlowKind;
use super::walk::WalkState;
use super::xrefs::{decode, FullCapture, FullOp};

const WORD_LIMIT: u64 = 1u64 << 32;
const MAX_PATH_INSNS: usize = 16;
const MAX_PROOF_OPS: usize = 256;
const MAX_ADDRESS_STEPS: usize = 32;
const MAX_INSN_BYTES: u64 = 15;

const MAX_RECONCILE_ROUNDS: usize = 16;
const MAX_CANDIDATE_BYTES: u64 = 1024 * 1024 * 1024;

/// Expand shared snapshots only; late proof invalidation discards the entire
/// candidate, including independent switches accepted in an earlier round.
pub(super) fn reconcile(
    tables: &X86Tables<'_>,
    ctx: &super::walk::StepCtx<'_>,
    ordinary: WalkState,
) -> WalkState {
    let Some(budget) = CandidateBudget::new(&ordinary) else {
        return ordinary;
    };
    let mut admitted = BTreeMap::new();
    let Some(mut proofs) = tables.proofs(
        ctx.translate,
        ctx.code_space,
        &ordinary,
        &admitted,
        budget.records(&ordinary, 0),
    ) else {
        return ordinary;
    };
    if proofs.is_empty() {
        return ordinary;
    }
    let mut candidate = ordinary.clone();
    for round in 0..=MAX_RECONCILE_ROUNDS {
        let retained = edge_records(&admitted);
        if round != 0 {
            let Some(revalidated) = tables.proofs(
                ctx.translate,
                ctx.code_space,
                &candidate,
                &admitted,
                budget.records(&candidate, retained),
            ) else {
                return ordinary;
            };
            proofs = revalidated;
        }
        if !budget.allows(&candidate, retained)
            || admitted
                .iter()
                .any(|(at, targets)| proofs.get(at) != Some(targets))
        {
            return ordinary;
        }
        let added: BTreeMap<_, _> = proofs
            .iter()
            .filter(|(at, _)| !admitted.contains_key(at))
            .map(|(&at, targets)| (at, targets.clone()))
            .collect();
        if added.is_empty() {
            return candidate;
        }
        if round == MAX_RECONCILE_ROUNDS {
            return ordinary;
        }
        admitted.extend(added.iter().map(|(&at, targets)| (at, targets.clone())));
        let retained = edge_records(&admitted);
        if !budget.allows(&candidate, retained)
            || !super::walk::extend_x86(ctx, &mut candidate, &added, |state| {
                budget.allows(state, retained)
            })
        {
            return ordinary;
        }
    }
    ordinary
}

fn edge_records(edges: &BTreeMap<u64, Vec<u64>>) -> usize {
    edges.values().fold(edges.len(), |count, targets| {
        count.saturating_add(targets.len())
    })
}

struct CandidateBudget {
    original_insns: usize,
    initial_bytes: u64,
    resident: Option<u64>,
    limit: u64,
}

impl CandidateBudget {
    fn new(state: &WalkState) -> Option<Self> {
        let limit = kuna_decomp::kuna_hostmem::available()
            .map_or(128 * 1024 * 1024, |available| available.bytes / 4)
            .min(MAX_CANDIDATE_BYTES);
        let initial_bytes = (state.insns.len() as u64)
            .saturating_mul(192)
            .saturating_add(
                (state.refs.len() as u64)
                    .saturating_mul(std::mem::size_of::<super::model::Reference>() as u64),
            )
            .saturating_add((state.funcs.len() as u64).saturating_mul(256))
            .saturating_add((state.stack_callback_refs.len() as u64).saturating_mul(96));
        (initial_bytes <= limit).then(|| Self {
            original_insns: state.insns.len(),
            initial_bytes,
            resident: kuna_decomp::kuna_hostmem::resident_bytes(),
            limit,
        })
    }

    fn allows(&self, state: &WalkState, retained: usize) -> bool {
        let added = state.insns.len().saturating_sub(self.original_insns) as u64;
        let used = self
            .initial_bytes
            .saturating_add(added.saturating_mul(512))
            .saturating_add((retained as u64).saturating_mul(256));
        if used > self.limit {
            return false;
        }
        match (self.resident, kuna_decomp::kuna_hostmem::resident_bytes()) {
            (Some(before), Some(now)) => now.saturating_sub(before) <= self.limit,
            _ => true,
        }
    }

    fn records(&self, state: &WalkState, retained: usize) -> usize {
        let added = state.insns.len().saturating_sub(self.original_insns) as u64;
        let used = self
            .initial_bytes
            .saturating_add(added.saturating_mul(512))
            .saturating_add((retained as u64).saturating_mul(256));
        usize::try_from(self.limit.saturating_sub(used) / 256).unwrap_or(usize::MAX)
    }
}

struct ProofBudget(usize);

impl ProofBudget {
    fn take(&mut self, count: usize) -> Option<()> {
        self.0 = self.0.checked_sub(count)?;
        Some(())
    }
}

pub(super) struct X86Tables<'a> {
    pool: PoolImage<'a>,
    data_space: Rc<AddrSpace>,
    exec: Vec<(u64, u64)>,
    max_entries: usize,
}

impl<'a> X86Tables<'a> {
    pub(super) fn new(file: &'a object::File<'a>, arch: &Architecture) -> Option<Self> {
        if file.architecture() != object::Architecture::I386
            || !matches!(
                file.kind(),
                object::ObjectKind::Executable | object::ObjectKind::Dynamic
            )
            || !matches!(
                file.format(),
                object::BinaryFormat::Elf | object::BinaryFormat::Pe
            )
            || !file.is_little_endian()
        {
            return None;
        }
        let code_space = arch.manage().get_default_code_space()?;
        let data_space = arch.manage().get_default_data_space()?;
        if !arch.archid.starts_with("x86:LE:32:")
            || code_space.get_addr_size() != 4
            || data_space.get_addr_size() != 4
            || code_space.get_word_size() != 1
            || data_space.get_word_size() != 1
        {
            return None;
        }
        let pool = PoolImage::new(file)?;
        if pool.ptr_width() != 4 {
            return None;
        }
        let mut exec: Vec<_> = crate::entry::executable_sections(file)
            .into_iter()
            .map(|(lo, hi, _)| (lo, hi))
            .collect();
        exec.sort_unstable();
        Some(Self {
            pool,
            data_space: data_space.clone(),
            exec,
            max_entries: match arch.max_jumptable_size as usize {
                0 => DEFAULT_MAX_ENTRIES,
                n => n,
            },
        })
    }

    /// Recomputing against the completed candidate also checks newly exposed
    /// incoming edges and function entries before any coverage is published.
    pub(super) fn proofs(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        state: &WalkState,
        admitted: &BTreeMap<u64, Vec<u64>>,
        max_records: usize,
    ) -> Option<BTreeMap<u64, Vec<u64>>> {
        let eligible = |insn: &super::model::Insn| {
            insn.flow.kind == FlowKind::ComputedJump
                && insn.flows.is_empty()
                && insn.fall_through.is_none()
        };
        if !state.insns.iter().any(|(_, insn)| eligible(insn)) {
            return Some(BTreeMap::new());
        }
        let mut budget = ProofBudget(max_records);
        let incoming = incoming_edges(state, admitted, &mut budget)?;
        let mut out = BTreeMap::new();
        let mut capture = FullCapture::default();
        for (&at, insn) in state.insns.iter() {
            if !eligible(insn) {
                continue;
            }
            budget.take(1)?;
            let Some(targets) = self.prove(
                translate,
                code_space,
                state,
                &incoming,
                at,
                &mut capture,
                budget.0,
            ) else {
                continue;
            };
            let targets = targets.ok()?;
            budget.take(targets.len())?;
            out.insert(at, targets);
        }
        Some(out)
    }

    fn prove(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        state: &WalkState,
        incoming: &BTreeMap<u64, BTreeSet<u64>>,
        dispatch: u64,
        capture: &mut FullCapture,
        max_targets: usize,
    ) -> Option<Result<Vec<u64>, ()>> {
        let mut backwards = Vec::new();
        let mut at = dispatch;
        let mut guard = None;
        let mut op_count = 0;
        for _ in 0..MAX_PATH_INSNS {
            let insn = state.insns.get(&at)?;
            if decode(translate, at, code_space, capture) != Some(insn.len) {
                return None;
            }
            op_count += capture.ops().len();
            if op_count > MAX_PROOF_OPS {
                return None;
            }
            backwards.push(Decoded {
                at,
                len: insn.len,
                ops: capture.ops().to_vec(),
            });
            if guard.is_some() && protected_path(&backwards, state, incoming) {
                if let Some((address, lo, hi)) =
                    prove_path(&backwards, guard?, code_space, &self.data_space)
                {
                    return self.read_bounded(dispatch, address, lo, hi, state, max_targets);
                }
            }
            let (&previous, pred) = state.insns.range(..at).next_back()?;
            if previous.checked_add(u64::from(pred.len)) != Some(at)
                || pred.fall_through != Some(at)
            {
                return None;
            }
            match pred.flow.kind {
                FlowKind::Fallthrough if pred.flows.is_empty() => {}
                FlowKind::ConditionalBranch if guard.is_none() => guard = Some(previous),
                _ => return None,
            }
            at = previous;
        }
        None
    }

    fn read_bounded(
        &self,
        dispatch: u64,
        address: Affine,
        lo: u64,
        hi: u64,
        state: &WalkState,
        max_targets: usize,
    ) -> Option<Result<Vec<u64>, ()>> {
        if address.value.scale != 4 || !address.without_wrap(lo, hi) {
            return None;
        }
        let base = u64::try_from(address.value.at(lo)?).ok()?;
        let count = usize::try_from(hi.checked_sub(lo)?.checked_add(1)?).ok()?;
        let read_count = count.min(self.max_entries);
        if read_count < MIN_ENTRIES {
            return None;
        }
        if read_count > max_targets {
            return Some(Err(()));
        }
        if !self
            .pool
            .contains_bytes(base, (read_count as u64).checked_mul(4)?)
        {
            return None;
        }
        let cases = super::kuna_switchtable::targets(
            base,
            dispatch,
            &self.pool,
            &self.exec,
            Some(read_count),
            self.max_entries,
        );
        if cases.len() != read_count {
            return None;
        }
        for target in &cases {
            if state.funcs.contains_key(target) {
                return None;
            }
            if state
                .insns
                .range(target.saturating_sub(MAX_INSN_BYTES)..*target)
                .any(|(&start, insn)| start.saturating_add(u64::from(insn.len)) > *target)
            {
                return None;
            }
        }
        Some(Ok(cases))
    }

    #[cfg(test)]
    fn read(
        &self,
        dispatch: u64,
        address: Affine,
        lo: u64,
        hi: u64,
        state: &WalkState,
    ) -> Option<Vec<u64>> {
        self.read_bounded(dispatch, address, lo, hi, state, usize::MAX)?
            .ok()
    }
}

fn incoming_edges(
    state: &WalkState,
    admitted: &BTreeMap<u64, Vec<u64>>,
    budget: &mut ProofBudget,
) -> Option<BTreeMap<u64, BTreeSet<u64>>> {
    let mut incoming: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    for (&at, insn) in state.insns.iter() {
        for target in insn.flows.iter().copied().chain(insn.fall_through) {
            if !incoming
                .get(&target)
                .is_some_and(|sources| sources.contains(&at))
            {
                budget.take(1)?;
            }
            incoming.entry(target).or_default().insert(at);
        }
    }
    for (&at, targets) in admitted {
        for &target in targets {
            if !incoming
                .get(&target)
                .is_some_and(|sources| sources.contains(&at))
            {
                budget.take(1)?;
            }
            incoming.entry(target).or_default().insert(at);
        }
    }
    Some(incoming)
}

struct Decoded {
    at: u64,
    len: u32,
    ops: Vec<FullOp>,
}

fn protected_path(
    backwards: &[Decoded],
    state: &WalkState,
    incoming: &BTreeMap<u64, BTreeSet<u64>>,
) -> bool {
    for pair in backwards.windows(2) {
        let (current, previous) = (&pair[0], &pair[1]);
        let Some(end) = current.at.checked_add(u64::from(current.len)) else {
            return false;
        };
        if state.funcs.range(current.at..end).next().is_some() {
            return false;
        }
        if incoming.range(current.at..end).any(|(&target, sources)| {
            target != current.at || sources.iter().any(|&source| source != previous.at)
        }) {
            return false;
        }
    }
    true
}

#[derive(Clone, Copy)]
struct Linear {
    root: Key,
    scale: i128,
    offset: i128,
}

impl Linear {
    fn at(self, index: u64) -> Option<i128> {
        self.scale
            .checked_mul(i128::from(index))?
            .checked_add(self.offset)
    }

    fn wrapped(self, index: u64) -> Option<u64> {
        u64::try_from(self.at(index)?.rem_euclid(i128::from(WORD_LIMIT))).ok()
    }
}

#[derive(Clone)]
struct Affine {
    value: Linear,
    steps: Vec<Linear>,
}

impl Affine {
    fn root(key: Key) -> Self {
        Self {
            value: Linear {
                root: key,
                scale: 1,
                offset: 0,
            },
            steps: Vec::new(),
        }
    }

    fn shifted(mut self, offset: i128) -> Option<Self> {
        self.value.offset = self.value.offset.checked_add(offset)?;
        self.record()
    }

    fn scaled(mut self, scale: i128) -> Option<Self> {
        self.value.scale = self.value.scale.checked_mul(scale)?;
        self.value.offset = self.value.offset.checked_mul(scale)?;
        self.record()
    }

    fn record(mut self) -> Option<Self> {
        if self.steps.len() >= MAX_ADDRESS_STEPS {
            return None;
        }
        self.steps.push(self.value);
        Some(self)
    }

    fn without_wrap(&self, lo: u64, hi: u64) -> bool {
        self.steps
            .iter()
            .chain(std::iter::once(&self.value))
            .all(|step| {
                [lo, hi].into_iter().all(|at| {
                    step.at(at)
                        .is_some_and(|v| (0..i128::from(WORD_LIMIT)).contains(&v))
                })
            })
    }
}

#[derive(Clone)]
enum Value {
    Constant(u64),
    Index(Affine),
    Condition(usize),
    Loaded(Affine),
}

#[derive(Clone, Copy)]
enum Comparison {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

impl Comparison {
    fn test(self, a: u64, b: u64) -> bool {
        match self {
            Self::Less => a < b,
            Self::LessEqual => a <= b,
            Self::Greater => a > b,
            Self::GreaterEqual => a >= b,
            Self::Equal => a == b,
            Self::NotEqual => a != b,
        }
    }

    fn reversed(self) -> Self {
        match self {
            Self::Less => Self::Greater,
            Self::LessEqual => Self::GreaterEqual,
            Self::Greater => Self::Less,
            Self::GreaterEqual => Self::LessEqual,
            v => v,
        }
    }
}

enum Condition {
    Constant(bool),
    Compare(Linear, u64, Comparison),
    Not(usize),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

struct Proof {
    values: Vec<(Key, Option<Value>)>,
    conditions: Vec<Condition>,
}

impl Proof {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            conditions: Vec::new(),
        }
    }

    fn get(&self, vn: &VarnodeData, data: &Rc<AddrSpace>) -> Option<Value> {
        if is_constant(vn) {
            return Some(Value::Constant(vn.offset & mask(vn.size)?));
        }
        let key = key_of(vn, Some(data))?;
        if let Some((_, value)) = self.values.iter().rev().find(|(k, _)| *k == key) {
            return value.clone();
        }
        if matches!(key, Key::Reg(_, 4)) && !self.values.iter().any(|(k, _)| overlap(*k, key)) {
            return Some(Value::Index(Affine::root(key)));
        }
        None
    }

    fn write(&mut self, key: Key, value: Option<Value>) {
        self.values.retain(|(old, _)| !overlap(*old, key));
        self.values.push((key, value));
    }

    fn condition(&mut self, value: Value) -> Option<usize> {
        match value {
            Value::Condition(at) => Some(at),
            Value::Constant(v) if v <= 1 => Some(self.push(Condition::Constant(v != 0))),
            _ => None,
        }
    }

    fn push(&mut self, condition: Condition) -> usize {
        let at = self.conditions.len();
        self.conditions.push(condition);
        at
    }

    fn fold(&mut self, op: &FullOp, data: &Rc<AddrSpace>) -> Option<Value> {
        let out = op.out.as_ref()?;
        let a = self.get(op.ins.first()?, data);
        let b = op.ins.get(1).and_then(|v| self.get(v, data));
        match op.opcode {
            OpCode::CPUI_COPY if out.size == op.ins[0].size => a,
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_MULT
                if out.size == 4 && op.ins.iter().all(|v| v.size == 4) =>
            {
                match (a?, b?) {
                    (Value::Index(v), Value::Constant(c)) => Some(Value::Index(match op.opcode {
                        OpCode::CPUI_INT_ADD => v.shifted(i128::from(c))?,
                        OpCode::CPUI_INT_SUB => v.shifted(-i128::from(c))?,
                        OpCode::CPUI_INT_MULT => v.scaled(i128::from(c))?,
                        _ => return None,
                    })),
                    (Value::Constant(c), Value::Index(v)) => Some(Value::Index(match op.opcode {
                        OpCode::CPUI_INT_ADD => v.shifted(i128::from(c))?,
                        OpCode::CPUI_INT_MULT => v.scaled(i128::from(c))?,
                        _ => return None,
                    })),
                    (Value::Constant(a), Value::Constant(b)) => {
                        let value = match op.opcode {
                            OpCode::CPUI_INT_ADD => a.checked_add(b)?,
                            OpCode::CPUI_INT_SUB => a.checked_sub(b)?,
                            OpCode::CPUI_INT_MULT => a.checked_mul(b)?,
                            _ => return None,
                        };
                        (value < WORD_LIMIT).then_some(Value::Constant(value))
                    }
                    _ => None,
                }
            }
            OpCode::CPUI_INT_LEFT if out.size == 4 && op.ins[0].size == 4 => {
                let (Value::Index(index), Value::Constant(shift)) = (a?, b?) else {
                    return None;
                };
                Some(Value::Index(
                    index.scaled(1i128.checked_shl(u32::try_from(shift).ok()?)?)?,
                ))
            }
            OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
                if out.size == 1 && op.ins.iter().all(|v| v.size == 4) =>
            {
                let comparison = match op.opcode {
                    OpCode::CPUI_INT_LESS => Comparison::Less,
                    OpCode::CPUI_INT_LESSEQUAL => Comparison::LessEqual,
                    OpCode::CPUI_INT_EQUAL => Comparison::Equal,
                    OpCode::CPUI_INT_NOTEQUAL => Comparison::NotEqual,
                    _ => return None,
                };
                let condition = match (a?, b?) {
                    (Value::Index(index), Value::Constant(c)) if index.value.scale == 1 => {
                        Condition::Compare(index.value, c, comparison)
                    }
                    (Value::Constant(c), Value::Index(index)) if index.value.scale == 1 => {
                        Condition::Compare(index.value, c, comparison.reversed())
                    }
                    (Value::Constant(a), Value::Constant(b)) => {
                        Condition::Constant(comparison.test(a, b))
                    }
                    _ => return None,
                };
                Some(Value::Condition(self.push(condition)))
            }
            OpCode::CPUI_BOOL_NEGATE if out.size == 1 => {
                let at = self.condition(a?)?;
                Some(Value::Condition(self.push(Condition::Not(at))))
            }
            OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
                if out.size == 1 && op.ins.iter().all(|v| v.size == 1) =>
            {
                let a = self.condition(a?)?;
                let b = self.condition(b?)?;
                let condition = match op.opcode {
                    OpCode::CPUI_BOOL_AND | OpCode::CPUI_INT_AND => Condition::And(a, b),
                    OpCode::CPUI_BOOL_OR | OpCode::CPUI_INT_OR => Condition::Or(a, b),
                    OpCode::CPUI_BOOL_XOR | OpCode::CPUI_INT_XOR => Condition::Xor(a, b),
                    _ => return None,
                };
                Some(Value::Condition(self.push(condition)))
            }
            OpCode::CPUI_LOAD if out.size == 4 => {
                if !is_constant(&op.ins[0])
                    || op.ins[0].offset != u64::try_from(data.get_index()).ok()?
                    || op.ins.get(1)?.size != 4
                {
                    return None;
                }
                let Value::Index(address) = b? else {
                    return None;
                };
                Some(Value::Loaded(address))
            }
            _ => None,
        }
    }

    fn interval(&self, condition: usize, root: Key) -> Option<(u64, u64)> {
        let mut cuts = BTreeSet::from([0, WORD_LIMIT]);
        let mut visited = vec![false; self.conditions.len()];
        self.cuts(condition, root, &mut cuts, &mut visited)?;
        let cuts: Vec<_> = cuts.into_iter().collect();
        let mut admitted: Vec<(u64, u64)> = Vec::new();
        let mut evaluated = vec![None; self.conditions.len()];
        for pair in cuts.windows(2) {
            evaluated.fill(None);
            if self.evaluate(condition, pair[0], &mut evaluated)? {
                continue;
            }
            match admitted.last_mut() {
                Some((_, end)) if *end == pair[0] => *end = pair[1],
                _ => admitted.push((pair[0], pair[1])),
            }
        }
        let [(lo, end)] = admitted.as_slice() else {
            return None;
        };
        Some((*lo, end.checked_sub(1)?))
    }

    fn cuts(
        &self,
        at: usize,
        root: Key,
        cuts: &mut BTreeSet<u64>,
        visited: &mut [bool],
    ) -> Option<()> {
        if *visited.get(at)? {
            return Some(());
        }
        visited[at] = true;
        match *self.conditions.get(at)? {
            Condition::Constant(_) => {}
            Condition::Compare(value, constant, _) => {
                if value.root != root || value.scale != 1 || constant >= WORD_LIMIT {
                    return None;
                }
                for value_at in [0, constant, (constant + 1) % WORD_LIMIT] {
                    let cut =
                        (i128::from(value_at) - value.offset).rem_euclid(i128::from(WORD_LIMIT));
                    cuts.insert(u64::try_from(cut).ok()?);
                }
            }
            Condition::Not(a) => self.cuts(a, root, cuts, visited)?,
            Condition::And(a, b) | Condition::Or(a, b) | Condition::Xor(a, b) => {
                self.cuts(a, root, cuts, visited)?;
                self.cuts(b, root, cuts, visited)?;
            }
        }
        Some(())
    }

    fn evaluate(&self, at: usize, index: u64, evaluated: &mut [Option<bool>]) -> Option<bool> {
        if let Some(value) = *evaluated.get(at)? {
            return Some(value);
        }
        let value = match *self.conditions.get(at)? {
            Condition::Constant(value) => value,
            Condition::Compare(value, constant, comparison) => {
                comparison.test(value.wrapped(index)?, constant)
            }
            Condition::Not(a) => !self.evaluate(a, index, evaluated)?,
            Condition::And(a, b) => {
                self.evaluate(a, index, evaluated)? && self.evaluate(b, index, evaluated)?
            }
            Condition::Or(a, b) => {
                self.evaluate(a, index, evaluated)? || self.evaluate(b, index, evaluated)?
            }
            Condition::Xor(a, b) => {
                self.evaluate(a, index, evaluated)? != self.evaluate(b, index, evaluated)?
            }
        };
        evaluated[at] = Some(value);
        Some(value)
    }
}

fn prove_path(
    backwards: &[Decoded],
    guard: u64,
    code_space: &Rc<AddrSpace>,
    data: &Rc<AddrSpace>,
) -> Option<(Affine, u64, u64)> {
    let mut proof = Proof::new();
    let mut condition = None;
    let dispatch = backwards.first()?;
    let mut loaded = None;
    for insn in backwards.iter().rev() {
        proof.values.retain(|(key, _)| matches!(key, Key::Reg(..)));
        for (position, op) in insn.ops.iter().enumerate() {
            match op.opcode {
                OpCode::CPUI_CBRANCH => {
                    if insn.at != guard || condition.is_some() || position + 1 != insn.ops.len() {
                        return None;
                    }
                    let target = op.ins.first()?;
                    let end = dispatch.at.checked_add(u64::from(dispatch.len))?;
                    if target.size != 4
                        || !target
                            .space
                            .as_ref()
                            .is_some_and(|s| Rc::ptr_eq(s, code_space))
                        || (backwards.last()?.at..end).contains(&target.offset)
                    {
                        return None;
                    }
                    let value = proof.get(op.ins.get(1)?, data)?;
                    condition = Some(proof.condition(value)?);
                }
                OpCode::CPUI_BRANCHIND => {
                    if insn.at != dispatch.at || position + 1 != insn.ops.len() || loaded.is_some()
                    {
                        return None;
                    }
                    let input = op.ins.first()?;
                    if input.size != 4 {
                        return None;
                    }
                    let Value::Loaded(address) = proof.get(input, data)? else {
                        return None;
                    };
                    loaded = Some(address);
                }
                OpCode::CPUI_BRANCH
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER => return None,
                _ => {
                    if let Some(key) = op.out.as_ref().and_then(|out| key_of(out, Some(data))) {
                        let value = proof.fold(op, data);
                        proof.write(key, value);
                    }
                }
            }
        }
    }
    let address = loaded?;
    let (lo, hi) = proof.interval(condition?, address.value.root)?;
    Some((address, lo, hi))
}

fn overlap(a: Key, b: Key) -> bool {
    let (a_class, a_offset, a_size) = match a {
        Key::Reg(offset, size) => (0, offset, size),
        Key::Tmp(offset, size) => (1, offset, size),
    };
    let (b_class, b_offset, b_size) = match b {
        Key::Reg(offset, size) => (0, offset, size),
        Key::Tmp(offset, size) => (1, offset, size),
    };
    a_class == b_class
        && a_offset < b_offset.saturating_add(u64::from(b_size))
        && b_offset < a_offset.saturating_add(u64::from(a_size))
}

fn mask(size: u32) -> Option<u64> {
    match size {
        1..=7 => Some((1u64 << (size * 8)) - 1),
        8 => Some(u64::MAX),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use kuna_base::space::spacetype;

    use super::*;

    struct Spaces {
        reg: Rc<AddrSpace>,
        tmp: Rc<AddrSpace>,
        constant: Rc<AddrSpace>,
        ram: Rc<AddrSpace>,
    }

    impl Spaces {
        fn new() -> Self {
            Self {
                reg: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
                tmp: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
                constant: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
                ram: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            }
        }

        fn vn(&self, space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
            VarnodeData {
                space: Some(space.clone()),
                offset,
                size,
            }
        }

        fn path(&self, compared_register: u64, width: u32, signed: bool) -> Vec<Decoded> {
            let reg = |off, size| self.vn(&self.reg, off, size);
            let tmp = |off, size| self.vn(&self.tmp, off, size);
            let c = |value, size| self.vn(&self.constant, value, size);
            vec![
                Decoded {
                    at: 0x1009,
                    len: 7,
                    ops: vec![
                        op(
                            OpCode::CPUI_INT_MULT,
                            Some(tmp(0x100, 4)),
                            vec![reg(0, 4), c(4, 4)],
                        ),
                        op(
                            OpCode::CPUI_INT_ADD,
                            Some(tmp(0x104, 4)),
                            vec![c(0x2000, 4), tmp(0x100, 4)],
                        ),
                        op(
                            OpCode::CPUI_LOAD,
                            Some(tmp(0x108, 4)),
                            vec![c(self.ram.get_index() as u64, 8), tmp(0x104, 4)],
                        ),
                        op(OpCode::CPUI_BRANCHIND, None, vec![tmp(0x108, 4)]),
                    ],
                },
                Decoded {
                    at: 0x1007,
                    len: 2,
                    ops: vec![
                        op(
                            OpCode::CPUI_BOOL_OR,
                            Some(tmp(0x100, 1)),
                            vec![reg(0x200, 1), reg(0x206, 1)],
                        ),
                        op(
                            OpCode::CPUI_BOOL_NEGATE,
                            Some(tmp(0x101, 1)),
                            vec![tmp(0x100, 1)],
                        ),
                        op(
                            OpCode::CPUI_CBRANCH,
                            None,
                            vec![self.vn(&self.ram, 0x1100, 4), tmp(0x101, 1)],
                        ),
                    ],
                },
                Decoded {
                    at: 0x1004,
                    len: 3,
                    ops: vec![
                        op(
                            if signed {
                                OpCode::CPUI_INT_SLESS
                            } else {
                                OpCode::CPUI_INT_LESS
                            },
                            Some(reg(0x200, 1)),
                            vec![reg(compared_register, width), c(3, width)],
                        ),
                        op(
                            OpCode::CPUI_INT_SUB,
                            Some(tmp(0x100, width)),
                            vec![reg(compared_register, width), c(3, width)],
                        ),
                        op(
                            OpCode::CPUI_INT_EQUAL,
                            Some(reg(0x206, 1)),
                            vec![tmp(0x100, width), c(0, width)],
                        ),
                    ],
                },
            ]
        }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    fn empty_state() -> WalkState {
        WalkState {
            insns: Default::default(),
            refs: Vec::new(),
            funcs: BTreeMap::new(),
            stack_callback_refs: BTreeMap::new(),
            mode_runs: Vec::new(),
        }
    }

    #[test]
    fn cmp_ja_proves_the_loaded_index_despite_wrapped_flag_subtraction() {
        let spaces = Spaces::new();
        let (address, lo, hi) =
            prove_path(&spaces.path(0, 4, false), 0x1007, &spaces.ram, &spaces.ram).unwrap();
        assert_eq!((lo, hi), (0, 3));
        assert_eq!(address.value.at(lo), Some(0x2000));
        assert!(address.without_wrap(lo, hi));
        for (register, width, signed) in [(4, 4, false), (0, 1, false), (0, 4, true)] {
            assert!(prove_path(
                &spaces.path(register, width, signed),
                0x1007,
                &spaces.ram,
                &spaces.ram
            )
            .is_none());
        }
    }

    #[test]
    fn partial_register_writes_and_instruction_local_temporaries_do_not_carry_a_proof() {
        let spaces = Spaces::new();
        for (offset, width) in [(0, 1), (1, 1), (0, 2)] {
            let mut path = spaces.path(0, 4, false);
            path.insert(
                1,
                Decoded {
                    at: 0x1008,
                    len: 1,
                    ops: vec![op(
                        OpCode::CPUI_COPY,
                        Some(spaces.vn(&spaces.reg, offset, width)),
                        vec![spaces.vn(&spaces.constant, 0, width)],
                    )],
                },
            );
            assert!(prove_path(&path, 0x1007, &spaces.ram, &spaces.ram).is_none());
        }
        let mut path = spaces.path(0, 4, false);
        path[2].ops[0].out = Some(spaces.vn(&spaces.tmp, 0x200, 1));
        path[1].ops[0].ins[0] = spaces.vn(&spaces.tmp, 0x200, 1);
        assert!(prove_path(&path, 0x1007, &spaces.ram, &spaces.ram).is_none());
    }

    #[test]
    fn normalized_bounds_keep_machine_width_and_address_wrap_obligations() {
        let root = Key::Reg(0, 4);
        let mut proof = Proof::new();
        let normalized = Affine::root(root).shifted(-5).unwrap();
        let condition = proof.push(Condition::Compare(normalized.value, 3, Comparison::Greater));
        assert_eq!(proof.interval(condition, root), Some((5, 8)));
        let address = normalized.scaled(4).unwrap().shifted(0x2000).unwrap();
        assert!(address.without_wrap(5, 8));
        assert!(!address.without_wrap(0, 8));
        let wrapped = Affine::root(root)
            .scaled(4)
            .unwrap()
            .shifted(i128::from(u32::MAX) - 3)
            .unwrap();
        assert!(!wrapped.without_wrap(0, 3));
        let split = proof.push(Condition::Compare(
            Affine::root(root).shifted(3).unwrap().value,
            3,
            Comparison::Greater,
        ));
        assert_eq!(proof.interval(split, root), None);
    }

    #[test]
    fn repeated_boolean_inputs_share_bounded_proof_work() {
        let root = Key::Reg(0, 4);
        let mut proof = Proof::new();
        let mut condition = proof.push(Condition::Compare(
            Affine::root(root).value,
            3,
            Comparison::Greater,
        ));
        for _ in 0..128 {
            condition = proof.push(Condition::And(condition, condition));
        }
        assert_eq!(proof.interval(condition, root), Some((0, 3)));
    }

    #[test]
    fn an_invalid_in_bound_word_cannot_shorten_the_proved_table() {
        let spaces = Spaces::new();
        let bytes: Vec<_> = [0x1010u32, 0x1020, 0xdead, 0x1040, 0x1050]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect();
        let mut tables = X86Tables {
            pool: PoolImage::from_ranges(vec![(0x2000, 0x2014, &bytes)], 4, true).unwrap(),
            data_space: spaces.ram,
            exec: vec![(0x1000, 0x1100)],
            max_entries: 4,
        };
        let address = Affine::root(Key::Reg(0, 4))
            .scaled(4)
            .unwrap()
            .shifted(0x2000)
            .unwrap();
        assert!(tables
            .read(0x1009, address.clone(), 0, 3, &empty_state())
            .is_none());
        tables.max_entries = 2;
        assert_eq!(
            tables.read(0x1009, address, 0, 3, &empty_state()),
            Some(vec![0x1010, 0x1020])
        );
    }

    #[test]
    fn repeated_cases_preserve_the_bound_and_do_not_consume_the_adjacent_callback() {
        let spaces = Spaces::new();
        let bytes: Vec<_> = [0x1010u32, 0x1020, 0x1010, 0x1040, 0x1050]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect();
        let tables = X86Tables {
            pool: PoolImage::from_ranges(vec![(0x2000, 0x2014, &bytes)], 4, true).unwrap(),
            data_space: spaces.ram,
            exec: vec![(0x1000, 0x1100)],
            max_entries: 1024,
        };
        let address = Affine::root(Key::Reg(0, 4))
            .scaled(4)
            .unwrap()
            .shifted(0x2000)
            .unwrap();
        let mut state = empty_state();
        state.funcs.insert(
            0x1050,
            super::super::model::DiscoveredFunction {
                entry: 0x1050,
                name: Some("independent".into()),
                from_symbol: true,
                has_no_return: false,
                call_fixup: None,
            },
        );
        assert_eq!(
            tables.read(0x1009, address.clone(), 0, 3, &state),
            Some(vec![0x1010, 0x1020, 0x1010, 0x1040])
        );
        state.funcs.insert(
            0x1020,
            super::super::model::DiscoveredFunction {
                entry: 0x1020,
                name: None,
                from_symbol: false,
                has_no_return: false,
                call_fixup: None,
            },
        );
        assert!(tables.read(0x1009, address, 0, 3, &state).is_none());
    }

    #[test]
    fn the_declared_pool_range_must_contain_every_byte_of_the_table() {
        let spaces = Spaces::new();
        let bytes: Vec<_> = [0x1010u32, 0x1020, 0x1030, 0x1040]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect();
        let tables = X86Tables {
            pool: PoolImage::from_ranges(vec![(0x2000, 0x200f, &bytes)], 4, true).unwrap(),
            data_space: spaces.ram,
            exec: vec![(0x1000, 0x1100)],
            max_entries: 1024,
        };
        let address = Affine::root(Key::Reg(0, 4))
            .scaled(4)
            .unwrap()
            .shifted(0x2000)
            .unwrap();
        assert!(tables.read(0x1009, address, 0, 3, &empty_state()).is_none());
    }

    #[test]
    fn incoming_and_table_rows_refuse_before_exhausting_the_aggregate_budget() {
        let state = empty_state();
        let admitted = BTreeMap::from([(0x1009, vec![0x1010, 0x1020, 0x1030, 0x1040])]);
        assert!(incoming_edges(&state, &admitted, &mut ProofBudget(3)).is_none());
        assert_eq!(
            incoming_edges(&state, &admitted, &mut ProofBudget(4))
                .unwrap()
                .len(),
            4
        );
        let spaces = Spaces::new();
        let bytes: Vec<_> = [0x1010u32, 0x1020, 0x1030, 0x1040]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect();
        let tables = X86Tables {
            pool: PoolImage::from_ranges(vec![(0x2000, 0x2010, &bytes)], 4, true).unwrap(),
            data_space: spaces.ram,
            exec: vec![(0x1000, 0x1100)],
            max_entries: 1024,
        };
        let address = Affine::root(Key::Reg(0, 4))
            .scaled(4)
            .unwrap()
            .shifted(0x2000)
            .unwrap();
        assert_eq!(
            tables.read_bounded(0x1009, address, 0, 3, &state, 3),
            Some(Err(()))
        );
    }

    #[test]
    fn retained_edges_remain_budgeted_without_host_resident_measurements() {
        let state = empty_state();
        let budget = CandidateBudget {
            original_insns: 0,
            initial_bytes: 0,
            resident: None,
            limit: 1024,
        };
        assert!(budget.allows(&state, 4));
        assert!(!budget.allows(&state, 5));
        assert_eq!(budget.records(&state, 3), 1);
    }
}
