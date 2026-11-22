//! A FID table needs an upper bound on the index the jump actually uses.
//! Listing's discovery heuristics deliberately make no such guarantee.

use std::collections::HashSet;

use super::{
    decode, delta_targets, is_constant, key_of, keys_overlap, mask, previous, AddrSpace,
    FullCapture, FullOp, Key, OpCode, PoolImage, Rc, Translate, VarnodeData,
    MAX_GUARD_INSNS,
};

/// Prove the straight-line x86-64 compare, guard and table load. The caller
/// must also rule out entries to `(start, from]` after discovering every edge.
#[allow(clippy::too_many_arguments)]
pub(in crate::listing) fn targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: &Rc<AddrSpace>,
    decoded: &HashSet<u64>,
    from: u64,
    pool: &PoolImage,
    exec: &[(u64, u64)],
    max_entries: usize,
) -> Option<(u64, Vec<u64>)> {
    let is_decoded = |vma| decoded.contains(&vma);
    let mut cap = FullCapture::default();
    let mut suffix = [0; MAX_GUARD_INSNS];
    let mut at = from;
    let mut end = None;
    let mut suffix_len = 0;
    let guard_at = loop {
        let len = decode(translate, at, code_space, &mut cap)?;
        if end.is_some_and(|end| at.checked_add(u64::from(len)) != Some(end)) {
            return None;
        }
        if at != from && cap.ops().last().is_some_and(|op| op.opcode == OpCode::CPUI_CBRANCH) {
            break at;
        }
        *suffix.get_mut(suffix_len)? = at;
        suffix_len += 1;
        end = Some(at);
        at = previous(&is_decoded, at)?;
    };
    let (branch, guard) = cap.ops().split_last()?;
    let [dest, condition] = branch.ins.as_slice() else { return None };
    if !dest.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, code_space)) {
        return None;
    }
    let start = previous(&is_decoded, guard_at)?;
    let mut writer_cap = FullCapture::default();
    let len = decode(translate, start, code_space, &mut writer_cap)?;
    if start.checked_add(u64::from(len)) != Some(guard_at) {
        return None;
    }
    let writer = writer_cap.ops();
    let mut replay = FullCapture::default();
    let mut machine = Machine { data, values: Vec::new() };
    for (i, op) in writer.iter().enumerate().filter(|(_, op)| op.opcode == OpCode::CPUI_INT_LESS) {
        let (Some(sel), Some(limit)) = (op.ins.first(), op.ins.get(1)) else { continue };
        if sel.size != limit.size || !matches!(sel.size, 1 | 2 | 4 | 8) || !is_constant(limit) {
            continue;
        }
        let Some(key) = machine.selector(sel, &writer[..i]) else { continue };
        let last = limit.offset & mask(limit.size);
        if last == 0 || last >= max_entries as u64 || last == mask(sel.size) {
            continue;
        }
        // With all values above the candidate upper bound, the guard must
        // always leave. Neither unmodeled flags nor signed tests are facts.
        machine.start(key, last + 1, mask(sel.size));
        machine.run(writer)?;
        machine.run(guard)?;
        if machine.get(condition) != Some(Value::constant(1)) {
            continue;
        }
        // Re-evaluate on fall-through with the *same input* now bounded. The
        // exact jump target must come from a load with a proven stride/count.
        machine.start(key, 0, last);
        machine.run(writer)?;
        machine.run(guard)?;
        for &at in suffix[1..suffix_len].iter().rev() {
            decode(translate, at, code_space, &mut replay)?;
            machine.run(replay.ops())?;
        }
        decode(translate, from, code_space, &mut replay)?;
        let (dispatch, prefix) = replay.ops().split_last()?;
        if dispatch.opcode != OpCode::CPUI_BRANCHIND || dispatch.ins.first()?.size != 8 {
            return None;
        }
        machine.run(prefix)?;
        let Some(Value::Cases { table, count, base }) = machine.get(dispatch.ins.first()?) else { continue };
        if count > max_entries {
            continue;
        }
        let cases = match base {
            Some(base) => delta_targets(table, base, from, pool, exec, Some(count), max_entries),
            None => super::targets(table, from, pool, exec, Some(count), max_entries),
        };
        if count == cases.len() {
            return Some((start, cases));
        }
    }
    None
}

/// A strided unsigned interval, or the result of its precisely modeled LOAD.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Value {
    Span { lo: u64, hi: u64, step: u64 },
    Delta32 { table: u64, count: usize, signed: bool },
    Cases { table: u64, count: usize, base: Option<u64> },
}

impl Value {
    fn constant(n: u64) -> Self {
        Self::Span { lo: n, hi: n, step: 0 }
    }

    fn exact(self) -> Option<u64> {
        let Self::Span { lo, hi, .. } = self else { return None };
        (lo == hi).then_some(lo)
    }

    fn endpoints(self) -> Option<(u64, u64)> {
        let Self::Span { lo, hi, .. } = self else { return None };
        Some((lo, hi))
    }

    fn map(self, f: impl Fn(u64) -> Option<u64>, scale: u64, max: u64) -> Option<Self> {
        let Self::Span { lo, hi, step } = self else { return None };
        let (lo, hi) = (f(lo)?, f(hi)?);
        (hi <= max).then_some(Self::Span { lo, hi, step: step.checked_mul(scale)? })
    }
}

struct Machine<'a> {
    data: &'a Rc<AddrSpace>,
    values: Vec<(Key, Value)>,
}

impl Machine<'_> {
    fn key(&self, vn: &VarnodeData) -> Option<Key> {
        key_of(vn, Some(self.data))
    }

    fn selector(&self, vn: &VarnodeData, ops: &[FullOp]) -> Option<Key> {
        let mut key = self.key(vn)?;
        // The x86 CMP constructor can copy its register operand into a local.
        for op in ops.iter().rev() {
            let Some(out) = op.out.as_ref().and_then(|vn| self.key(vn)) else { continue };
            if keys_overlap(key, out) && matches!(key, Key::Tmp(..)) {
                let src = op.ins.first()?;
                if key != out || op.opcode != OpCode::CPUI_COPY || src.size != vn.size {
                    return None;
                }
                key = self.key(src)?;
            }
        }
        matches!(key, Key::Reg(..)).then_some(key)
    }

    fn start(&mut self, key: Key, lo: u64, hi: u64) {
        self.values.clear();
        let step = u64::from(lo != hi);
        self.values.push((key, Value::Span { lo, hi, step }));
    }

    fn get(&self, vn: &VarnodeData) -> Option<Value> {
        if !(1..=8).contains(&vn.size) {
            return None;
        }
        if is_constant(vn) {
            return Some(Value::constant(vn.offset & mask(vn.size)));
        }
        let key = self.key(vn)?;
        self.values.iter().find_map(|&(k, v)| (k == key).then_some(v))
    }

    fn run(&mut self, ops: &[FullOp]) -> Option<()> {
        self.values.retain(|(k, _)| matches!(k, Key::Reg(..)));
        for op in ops {
            if matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN
                | OpCode::CPUI_STORE)
            {
                return None;
            }
            let out = op.out.as_ref()?;
            let key = self.key(out)?;
            let value = (1..=8).contains(&out.size).then(|| self.fold(op, out.size)).flatten();
            self.values.retain(|(k, _)| !keys_overlap(key, *k));
            if let Some(value) = value {
                self.values.push((key, value));
            }
        }
        Some(())
    }

    fn fold(&self, op: &FullOp, size: u32) -> Option<Value> {
        use OpCode::*;
        let input = |i: usize| op.ins.get(i);
        let get = |i| self.get(input(i)?);
        let exact = |i| get(i)?.exact();
        let boolean = |v: bool| Some(Value::constant(u64::from(v)));
        let max = mask(size);
        match op.opcode {
            CPUI_COPY if input(0)?.size == size => get(0),
            CPUI_INT_ZEXT if input(0)?.size < size && size <= 8 => get(0)?.map(Some, 1, max),
            CPUI_INT_SEXT if input(0)?.size == 4 && size == 8 => match get(0)? {
                Value::Delta32 { table, count, signed: false } => Some(Value::Delta32 { table, count, signed: true }),
                _ => None,
            },
            CPUI_INT_ADD | CPUI_INT_MULT => {
                let (a, b) = (get(0)?, get(1)?);
                let (span, c) = if let Some(n) = a.exact() { (b, n) } else { (a, b.exact()?) };
                if op.opcode == CPUI_INT_MULT {
                    span.map(|x| x.checked_mul(c), c, max)
                } else if let Value::Delta32 { table, count, signed: true } = span {
                    (size == 8).then_some(Value::Cases { table, count, base: Some(c) })
                } else {
                    span.map(|x| x.checked_add(c), 1, max)
                }
            }
            CPUI_INT_LEFT => {
                let scale = 1_u64.checked_shl(u32::try_from(exact(1)?).ok()?)?;
                get(0)?.map(|x| x.checked_mul(scale), scale, max)
            }
            CPUI_INT_SUB => {
                let c = exact(1)?;
                get(0)?.map(|x| x.checked_sub(c), 1, max)
            }
            CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL => {
                let ((a, b), (c, d)) = (get(0)?.endpoints()?, get(1)?.endpoints()?);
                match op.opcode {
                    CPUI_INT_LESS if b < c => boolean(true),
                    CPUI_INT_LESS if a >= d => boolean(false),
                    CPUI_INT_LESSEQUAL if b <= c => boolean(true),
                    CPUI_INT_LESSEQUAL if a > d => boolean(false),
                    CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL if b < c || a > d => boolean(op.opcode == CPUI_INT_NOTEQUAL),
                    CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL if a == b && a == c && c == d => boolean(op.opcode == CPUI_INT_EQUAL),
                    _ => None,
                }
            }
            CPUI_BOOL_NEGATE => boolean(exact(0)? == 0),
            CPUI_BOOL_AND => boolean(exact(0)? != 0 && exact(1)? != 0),
            CPUI_BOOL_OR => boolean(exact(0)? != 0 || exact(1)? != 0),
            CPUI_LOAD if is_constant(input(0)?) && input(0)?.offset == self.data.get_index() as u64
                && input(1)?.size == 8 => {
                let Value::Span { lo, hi, step } = get(1)? else { return None };
                if !matches!(size, 4 | 8) || step != u64::from(size) {
                    return None;
                }
                let count = usize::try_from((hi - lo) / step + 1).ok()?;
                if size == 4 {
                    Some(Value::Delta32 { table: lo, count, signed: false })
                } else {
                    Some(Value::Cases { table: lo, count, base: None })
                }
            }
            _ => None,
        }
    }
}
