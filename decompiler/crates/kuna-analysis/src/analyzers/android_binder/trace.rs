//! Trace the class returned by the Binder factory, and the transaction-code argument in its callback.
use super::image::{Api, Image};
use crate::analyzers::nativeflow::{
    decode_scope, discard_temporaries, Machine, Slot, State, Value,
};
use crate::listing::xrefs::{FullCapture, FullOp};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use object::{Architecture, Object};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

const CODE: u64 = u64::MAX;

/// A call site seen with different arguments has no unique static binding.
fn observe<T: Eq>(map: &mut BTreeMap<u64, Option<T>>, pc: u64, value: Option<T>) {
    map.entry(pc)
        .and_modify(|seen| {
            if *seen != value {
                *seen = None;
            }
        })
        .or_insert(value);
}

#[derive(Default)]
pub(super) struct Facts {
    pub(super) classes: BTreeMap<u64, Option<[u64; 4]>>,
    pub(super) maps: BTreeMap<u64, BTreeSet<Option<(u64, u64)>>>,
    pub(super) exposed: bool,
    pub(super) callees: BTreeSet<u64>,
    pub(super) candidates: BTreeSet<u32>,
    pub(super) indirect: BTreeMap<u64, Option<u64>>,
    pub(super) cases: BTreeMap<u64, Option<u32>>,
}

impl Facts {
    fn observe_bound(&mut self, m: &Machine, st: &State, op: &FullOp) {
        if !matches!(
            op.opcode,
            OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL
        ) {
            return;
        }
        let value = |i| m.value(st, op.ins.get(i)?);
        let (delta, n) = match (value(0), value(1)) {
            (Some(Value::Tagged(CODE, delta)), Some(Value::Constant(n)))
            | (Some(Value::Constant(n)), Some(Value::Tagged(CODE, delta))) => (delta, n),
            _ => return,
        };
        if n < 256 && self.candidates.len() <= 256 {
            let first = (delta as u32).wrapping_neg();
            self.candidates.extend(
                (0..=n as u32)
                    .filter_map(|i| first.checked_add(i))
                    .filter(|&c| c > 0),
            );
        }
    }

    fn call(
        &mut self,
        m: &Machine,
        image: &Image,
        st: &mut State,
        api: Option<Api>,
        pc: u64,
        watch: &std::ops::Range<u64>,
    ) {
        let args: [_; 4] = std::array::from_fn(|i| m.argument(st, i).and_then(|s| st.read(s)));
        for (i, arg) in args.iter().enumerate() {
            if !(api == Some(Api::Map) && i == 1)
                && arg
                    .and_then(Value::constant)
                    .is_some_and(|v| watch.contains(&v))
            {
                self.exposed = true;
            }
        }
        let mut result = None;
        match api {
            Some(Api::Define) => {
                let definition = (|| {
                    Some([
                        args[0]?.constant()?,
                        args[1]?.constant()?,
                        args[2]?.constant()?,
                        args[3]?.constant()?,
                    ])
                })();
                let valid = definition.filter(|a| {
                    image.text(a[0]).is_some() && a[1..].iter().all(|&v| image.code(v))
                });
                observe(&mut self.classes, pc, valid);
                if valid.is_some() {
                    result = Some(Value::Tagged(pc, 0));
                }
            }
            Some(Api::Map) => {
                if let Some(Value::Tagged(owner, 0)) = args[0] {
                    self.maps.entry(owner).or_default().insert(
                        args[1]
                            .and_then(Value::constant)
                            .zip(args[2].and_then(Value::constant)),
                    );
                }
            }
            None => {
                // Only an address passed to the callee can expose the caller's stack slots.
                if args.into_iter().any(|a| matches!(a, Some(Value::Stack(_)))) {
                    st.forget_memory();
                } else {
                    st.0.retain(|slot, _| {
                        !matches!(slot.0, crate::analyzers::nativeflow::Place::Data)
                    });
                }
            }
        }
        m.after_call(st);
        st.write(m.result, result);
    }
}

fn exposes(machine: &Machine, st: &State, op: &FullOp, range: &std::ops::Range<u64>) -> bool {
    use OpCode::*;
    let operand = |i| machine.value(st, op.ins.get(i)?);
    let inside = |value: Option<Value>| {
        value
            .and_then(Value::constant)
            .is_some_and(|n| range.contains(&n))
    };
    let overlaps =
        |at: u64, size: u32| at < range.end && at.saturating_add(u64::from(size)) > range.start;
    if let Some(out) = op.out.as_ref().filter(|v| {
        v.space
            .as_ref()
            .is_some_and(|s| s.get_index() == machine.code_space)
    }) {
        if overlaps(out.offset, out.size)
            || matches!(op.opcode, CPUI_COPY | CPUI_CAST) && inside(operand(0))
        {
            return true;
        }
    }
    match op.opcode {
        CPUI_STORE => {
            operand(1)
                .and_then(Value::constant)
                .is_some_and(|n| overlaps(n, op.ins[2].size))
                || inside(operand(2)) && !matches!(operand(1), Some(Value::Stack(_)))
        }
        CPUI_INT_ADD | CPUI_INT_SUB => [(0, 1), (1, 0)]
            .into_iter()
            .any(|(a, b)| inside(operand(a)) && operand(b).and_then(Value::constant).is_none()),
        CPUI_RETURN => inside(st.read(machine.result)),
        _ => false,
    }
}

/// A linker or C forwarding thunk that transfers control without modifying any argument.
pub(super) fn forwarding_target(image: &Image, m: &Machine, raw: u64) -> Option<Api> {
    let arch = m.arch;
    let _scope = decode_scope(arch, raw, image.file.architecture() == Architecture::Arm)?;
    let code = arch.manage().get_default_code_space()?;
    let tr = arch.translate();
    let mapped = tr.loader_rc().try_borrow().ok()?.shared_bytes()?;
    let mut capture = FullCapture::default();
    let mut pc = image.pc(raw);
    for _ in 0..4 {
        capture.begin();
        let len = tr
            .one_instruction_checked(
                &mut capture,
                &Address::new(Rc::clone(code), pc),
                mapped.as_ref(),
            )
            .ok()?;
        match capture.ops() {
            [op] if op.opcode == OpCode::CPUI_BRANCH => {
                let target = op.ins.first()?;
                if target.space.as_ref()?.get_index() != m.code_space {
                    return None;
                }
                return image.stubs.get(&target.offset).copied();
            }
            [] if len > 0 => pc = pc.checked_add(len as u64)?,
            _ => return None,
        }
    }
    None
}

pub(super) enum Mode {
    Factory,
    Watch(std::ops::Range<u64>),
    Dispatch(Option<u32>),
}

pub(super) fn function(
    image: &Image,
    m: &Machine,
    raw: u64,
    end: u64,
    mode: Mode,
    budget: &mut usize,
) -> Option<Facts> {
    let arch = m.arch;
    let _scope = decode_scope(arch, raw, image.file.architecture() == Architecture::Arm)?;
    let code = arch.manage().get_default_code_space()?;
    let tr = arch.translate();
    let mapped = tr.loader_rc().try_borrow().ok()?.shared_bytes()?;
    let start = image.pc(raw);
    let mut first = State::default();
    first.write(m.sp, Some(Value::Stack(0)));
    let watch = match &mode {
        Mode::Watch(range) => range.clone(),
        _ => 0..0,
    };
    let dispatch = matches!(mode, Mode::Dispatch(_));
    if let Mode::Dispatch(concrete) = mode {
        let Slot(place, off, _) = m.argument(&first, 1)?;
        first.write(
            Slot(place, off, 4),
            Some(concrete.map_or(Value::Tagged(CODE, 0), |n| Value::Constant(n.into()))),
        );
    }
    let mut queue = VecDeque::from([(start, first)]);
    let mut visited = BTreeMap::<u64, Vec<State>>::new();
    let mut capture = FullCapture::default();
    let mut facts = Facts::default();
    let mut local = 4096_usize;
    while let Some((pc, mut st)) = queue.pop_front() {
        if !(start..end).contains(&pc) {
            continue;
        }
        let states = visited.entry(pc).or_default();
        if states.contains(&st) {
            continue;
        }
        if states.len() >= 12 {
            return None;
        }
        local = local.checked_sub(1)?;
        *budget = budget.checked_sub(1)?;
        states.push(st.clone());
        capture.begin();
        let addr = Address::new(Rc::clone(code), pc);
        let len = tr
            .one_instruction_checked(&mut capture, &addr, mapped.as_ref())
            .ok()?;
        let next = pc.checked_add(u64::try_from(len).ok()?)?;
        if next <= pc || next > end {
            return None;
        }
        let mut falls = true;
        for op in capture.ops() {
            use OpCode::*;
            let first = op.ins.first();
            let direct = first
                .filter(|v| {
                    v.space
                        .as_ref()
                        .is_some_and(|s| s.get_index() == m.code_space)
                })
                .map(|v| v.offset);
            let ref_target = || first.and_then(|v| m.value(&st, v));
            let api = |addr: Option<Value>| match addr? {
                Value::Constant(va) => image.stubs.get(&va).copied(),
                Value::Function(slot) => image.got.get(&slot).copied(),
                _ => None,
            };
            if matches!(mode, Mode::Dispatch(None)) {
                facts.observe_bound(m, &st, op);
            }
            if !watch.is_empty() && exposes(m, &st, op, &watch) {
                facts.exposed = true;
                return Some(facts);
            }
            match op.opcode {
                CPUI_CALL | CPUI_CALLIND => {
                    let to = if op.opcode == CPUI_CALL {
                        direct.map(Value::Constant)
                    } else {
                        ref_target()
                    };
                    if !(m.stack_return && to == Some(Value::Constant(next))) {
                        if !watch.is_empty() && op.opcode == CPUI_CALL {
                            if let Some(callee) =
                                direct.and_then(|va| image.direct_target(raw, pc, va, true))
                            {
                                facts.callees.insert(callee);
                            }
                        }
                        facts.call(m, image, &mut st, api(to), pc, &watch);
                    }
                }
                CPUI_BRANCH | CPUI_BRANCHIND | CPUI_CBRANCH => {
                    let cond = if op.opcode == CPUI_CBRANCH {
                        op.ins.get(1).and_then(|v| m.value(&st, v))
                    } else {
                        Some(Value::Constant(1))
                    };
                    if cond.and_then(Value::constant) == Some(0) {
                        continue;
                    }
                    let to = if op.opcode == CPUI_BRANCHIND {
                        ref_target()
                    } else {
                        direct.map(Value::Constant)
                    };
                    if dispatch && op.opcode == CPUI_BRANCHIND {
                        let target = to
                            .and_then(Value::constant)
                            .filter(|&t| (start..end).contains(&t));
                        observe(&mut facts.indirect, pc, target);
                        if matches!(mode, Mode::Dispatch(Some(_))) {
                            falls = false;
                            break;
                        }
                    }
                    if dispatch && op.opcode == CPUI_CBRANCH && to.is_some() {
                        let case = match cond {
                            Some(Value::Predicate {
                                tag: CODE, equals, ..
                            }) => u32::try_from(equals).ok().filter(|&c| c != 0),
                            _ => None,
                        };
                        observe(&mut facts.cases, pc, case);
                    }
                    if let Some(target) = to {
                        if let Some(a) = api(to) {
                            facts.call(m, image, &mut st, Some(a), pc, &watch);
                        } else if let Some(target) =
                            target.constant().filter(|v| (start..end).contains(v))
                        {
                            let mut branch = st.clone();
                            discard_temporaries(&mut branch, arch);
                            queue.push_back((target, branch));
                        } else if !watch.is_empty() && op.opcode == CPUI_BRANCH {
                            if let Some(callee) =
                                direct.and_then(|va| image.direct_target(raw, pc, va, false))
                            {
                                facts.callees.insert(callee);
                            }
                            facts.call(m, image, &mut st, None, pc, &watch);
                        }
                    } else if op.opcode == CPUI_CBRANCH || op.opcode == CPUI_BRANCH {
                        // Relative p-code targets stay inside this instruction.
                        return None;
                    }
                    if cond.and_then(Value::constant).is_some() {
                        falls = false;
                        break;
                    }
                }
                CPUI_RETURN => {
                    falls = false;
                    break;
                }
                _ => m.transfer(&mut st, op, &|addr, size| {
                    if size != m.width {
                        return None;
                    }
                    let slot = addr.constant()?;
                    image
                        .got
                        .contains_key(&slot)
                        .then_some(Value::Function(slot))
                }),
            }
        }
        if falls {
            discard_temporaries(&mut st, arch);
            queue.push_back((next, st));
        }
    }
    Some(facts)
}
