//! (kuna) `switchflow` — the jump-table base and range check of a dispatch the
//! linear slice could not read, found over the function's own predecessor edges.
//!
//! # The gap
//!
//! [`super::kuna_switchtable`]'s slice steps back from the dispatch by address
//! and stops at the first instruction that does not fall through, so a base
//! hoisted above a `jmp` never reaches it. Three shapes every x86-64 compiler
//! emits sit behind one:
//!
//! ```text
//! lea  jt(%rip),%rbp        ; hoisted out of the loop
//! ...
//! call branch_b
//! jmp  merge                ; <- the slice stops here, seven instructions short
//! call branch_a
//! merge: cmp $7,%r15d; ja default; movslq 0(%rbp,%rax,4),%rax; add %rbp,%rax; jmp *%rax
//! ```
//!
//! the `if/else` merge above (clang `-O2`, MSVC `/O2`); two switches sharing
//! one `LEA RBP,[__ImageBase]`, where the first switch's case bodies end in
//! `jmp` or `ret` before the second dispatch (MSVC); and a range check more
//! than sixteen instructions above the jump, which the guard search never
//! reaches. Each lost the whole switch — every call its case bodies make —
//! and `kuna xrefs` said nothing about it.
//!
//! # The rule
//!
//! The value a register holds on entry to an instruction is what each
//! **predecessor** leaves in it, met: a predecessor is an instruction the walk
//! decoded in this function that falls through or branches to it, a switch
//! case edge included. The question is asked on demand, from the `BRANCHIND`'s
//! input back through the ops that define what it still wants, exactly the
//! slice's chain, and the fold is [`kuna_switchtable::fold`]: a constant, a
//! constant plus an index, a table entry, an entry composed with its base. A
//! call kills what the calling convention does not preserve. Predecessors that
//! agree keep the value, ones that disagree make it unknown, and a predecessor
//! the walk has not decoded — the back edge of a loop whose latch only the
//! case bodies reach — contributes nothing, as the linear slice never saw it
//! either. The range check is the nearest conditional branch up the single
//! chain of predecessors ending at the dispatch, falling through to it, read
//! by [`kuna_switchtable::guard_cases`] as before, and it must compare the
//! register the table is indexed by; a join, a taken side, any other
//! conditional branch or a comparison of another value yields no bound.
//!
//! The read runs only once the dispatching function's queue has drained, so
//! every edge of the body is known, and only for a dispatch the linear read
//! declined in the function whose walk it is. A function with no unread
//! dispatch pays nothing. One read decodes at most [`MAX_FLOW_INSNS`]
//! instructions, each once.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{
    self, delta_targets, feeders, fold, guard_cases, is_constant, key_of, keys_overlap,
    leaves_registers_unknown, leaving_branch, mask_limit, survives, targets, value_of, value_of_key,
    Key, Reach, TableRead, Val, MAX_GUARD_INSNS,
};
use super::xrefs::{decode, FullCapture, FullOp};

/// How many instructions one read decodes before it gives up: the whole of a
/// function above its dispatch on gcc's `diff`, with room over.
const MAX_FLOW_INSNS: usize = 512;

/// How many instructions the range-check search visits.
const MAX_GUARD_FLOW_INSNS: usize = 64;

/// The intra-function edges of one walked body, as `(to, from)`.
pub(super) struct Edges(Vec<(u64, u64)>);

impl Edges {
    pub(super) fn new(mut edges: Vec<(u64, u64)>) -> Self {
        edges.sort_unstable();
        edges.dedup();
        Self(edges)
    }

    fn preds(&self, to: u64) -> impl Iterator<Item = u64> + '_ {
        let lo = self.0.partition_point(|&(t, _)| t < to);
        self.0[lo..].iter().take_while(move |&&(t, _)| t == to).map(|&(_, from)| from)
    }
}

/// What a register holds along the edges into an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reached {
    /// No decoded predecessor says: the function's entry, or only a path
    /// through the question itself.
    Nothing,
    /// A predecessor leaves something the fold does not model, or two leave
    /// different values.
    Unknown,
    Known(Val),
}

impl Reached {
    fn meet(self, other: Reached) -> Reached {
        match (self, other) {
            (Reached::Nothing, r) | (r, Reached::Nothing) => r,
            (Reached::Known(a), Reached::Known(b)) if a == b => Reached::Known(a),
            _ => Reached::Unknown,
        }
    }
}

struct Flow<'a> {
    translate: &'a dyn Translate,
    code_space: &'a Rc<AddrSpace>,
    data: Option<&'a Rc<AddrSpace>>,
    decoded: &'a HashSet<u64>,
    pool: &'a PoolImage<'a>,
    edges: &'a Edges,
    preserved: &'a [(u64, u64)],
    big: bool,
    cap: FullCapture,
    /// Every instruction decoded by this read, once.
    insns: HashMap<u64, Option<Rc<(u32, Vec<FullOp>)>>>,
    /// `key`'s value at the end of the instruction at `vma`.
    memo: HashMap<(Key, u64), Reached>,
    active: HashSet<(Key, u64)>,
    budget: usize,
    /// The table-address sums whose unknown addend is the index, as
    /// `(instruction, op, key)`, for [`kuna_switchtable::mask_limit`].
    index: Vec<(u64, usize, Key)>,
}

impl Flow<'_> {
    fn insn(&mut self, vma: u64) -> Option<Rc<(u32, Vec<FullOp>)>> {
        if let Some(insn) = self.insns.get(&vma) {
            return insn.clone();
        }
        let insn = (self.budget > 0 && self.decoded.contains(&vma))
            .then(|| decode(self.translate, vma, self.code_space, &mut self.cap))
            .flatten()
            .map(|len| Rc::new((len, self.cap.ops().to_vec())));
        self.budget = self.budget.saturating_sub(1);
        self.insns.insert(vma, insn.clone());
        insn
    }

    /// `key` at the end of the instruction at `vma`.
    fn at_end(&mut self, key: Key, vma: u64) -> Reached {
        if let Some(&r) = self.memo.get(&(key, vma)) {
            return r;
        }
        if !self.active.insert((key, vma)) {
            return Reached::Nothing;
        }
        let r = match self.insn(vma) {
            Some(insn) => self.within(key, vma, &insn.1, insn.1.len()),
            None => Reached::Nothing,
        };
        self.active.remove(&(key, vma));
        self.memo.insert((key, vma), r);
        r
    }

    /// `key` just before op `upto` of the instruction at `vma`: the nearest
    /// op above that writes it, folded, else what the edges into `vma` carry.
    fn within(&mut self, key: Key, vma: u64, ops: &[FullOp], upto: usize) -> Reached {
        for (i, op) in ops[..upto].iter().enumerate().rev() {
            if leaves_registers_unknown(op.opcode) && !survives(key, self.preserved) {
                return Reached::Unknown;
            }
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, self.data)) else { continue };
            if !keys_overlap(out, key) {
                continue;
            }
            let vals = self.inputs(op, vma, ops, i);
            let Some(folded) = fold(op, &vals, self.data, self.pool) else { return Reached::Unknown };
            if op.opcode == OpCode::CPUI_INT_ADD && matches!(folded, Val::Indexed(_)) {
                let unknown = op.ins.iter().find(|vn| value_of(vn, &vals, self.data).is_none());
                if let Some(index) = unknown.and_then(|vn| key_of(vn, self.data)) {
                    self.index.push((vma, i, index));
                }
            }
            if out == key {
                return Reached::Known(folded);
            }
            return value_of_key(key, &[(out, folded)], self.big).map_or(Reached::Unknown, Reached::Known);
        }
        self.before(key, vma)
    }

    /// The values op `i` reads, for [`fold`].
    fn inputs(&mut self, op: &FullOp, vma: u64, ops: &[FullOp], i: usize) -> Vec<(Key, Val)> {
        let mut vals = Vec::new();
        for vn in op.ins.iter().filter(|vn| !is_constant(vn)) {
            let Some(key) = key_of(vn, self.data) else { continue };
            if let Reached::Known(v) = self.within(key, vma, ops, i) {
                vals.push((key, v));
            }
        }
        vals
    }

    /// `key` on entry to the instruction at `vma`: its decoded predecessors, met.
    fn before(&mut self, key: Key, vma: u64) -> Reached {
        let preds: Vec<u64> = self.edges.preds(vma).filter(|p| self.decoded.contains(p)).collect();
        let mut r = Reached::Nothing;
        for p in preds {
            r = r.meet(self.at_end(key, p));
            if r == Reached::Unknown {
                break;
            }
        }
        r
    }
}

/// The case bodies of the computed jump at `from` on `branch`, read over
/// `edges`: [`kuna_switchtable::register_targets`]'s answer for a dispatch
/// whose base or range check the linear read could not reach. Only a
/// range-checked table is read this way; a jump that is not a switch declines
/// at the first definition the fold does not model, as the slice does.
#[allow(clippy::too_many_arguments)]
pub(super) fn register_targets(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    data: Option<&Rc<AddrSpace>>,
    decoded: &HashSet<u64>,
    from: u64,
    branch: &VarnodeData,
    pool: &PoolImage,
    exec: &[(u64, u64)],
    reach: &Reach,
    edges: &Edges,
) -> TableRead {
    let Some(want) = key_of(branch, data) else { return TableRead::default() };
    let mut flow = Flow {
        translate,
        code_space,
        data,
        decoded,
        pool,
        edges,
        preserved: reach.preserved,
        big: data.is_some_and(|d| d.is_big_endian()),
        cap: FullCapture::default(),
        insns: HashMap::new(),
        memo: HashMap::new(),
        active: HashSet::new(),
        budget: MAX_FLOW_INSNS,
        index: Vec::new(),
    };
    let Some(insn) = flow.insn(from) else { return TableRead::default() };
    let Some(at) = insn.1.iter().position(|op| {
        matches!(op.opcode, OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND)
            && op.ins.first().is_some_and(|vn| key_of(vn, data) == Some(want))
    }) else {
        return TableRead::default();
    };
    let table = match flow.within(want, from, &insn.1, at) {
        Reached::Known(Val::Case { table, base }) => (table, Some(base)),
        Reached::Known(Val::Entry { table, width, .. }) if width == pool.ptr_width() => (table, None),
        _ => return TableRead::default(),
    };
    let index = std::mem::take(&mut flow.index);
    let Some(limit) = guard_limit(&mut flow, from, &index, reach.max_entries) else {
        return TableRead::default();
    };
    let masked = index
        .into_iter()
        .filter_map(|at| mask_limit(translate, code_space, data, &|at| decoded.contains(&at), from, at, reach.max_entries))
        .min();
    let limit = masked.map_or(limit, |n| n.min(limit));
    let cases = match table {
        (table, Some(base)) => delta_targets(table, base, from, pool, exec, Some(limit), reach.max_entries),
        (table, None) => targets(table, from, pool, exec, Some(limit), reach.max_entries),
    };
    TableRead { cases, stated: Some(limit), ..TableRead::default() }
}

/// The case count the range check above `from` states: the nearest
/// conditional branch up the single chain of predecessors that ends at the
/// dispatch, falling through to it, with the bound in the branch or in the
/// nearest instruction falling into it that writes what the condition reads
/// ([`kuna_switchtable::guard_limit`]'s rule), and comparing the selector: one
/// of the table-address `index` addends, followed up the same instructions,
/// still names a register the comparison reads ([`selects`]). A join, the
/// function's entry, a branch the chain leaves by its taken side and a
/// comparison of another value bound nothing.
fn guard_limit(flow: &mut Flow, from: u64, index: &[(u64, usize, Key)], max_entries: usize) -> Option<usize> {
    let mut path = vec![from];
    let mut cur = from;
    while path.len() <= MAX_GUARD_FLOW_INSNS {
        let preds: Vec<u64> = flow.edges.preds(cur).collect();
        let &[p] = preds.as_slice() else { return None };
        let insn = flow.insn(p)?;
        path.push(p);
        if let Some(at) = leaving_branch(&insn.1, p, insn.0) {
            if p.wrapping_add(u64::from(insn.0)) != cur {
                return None;
            }
            let (n, compared, above) = guard_at(flow, p, &insn.1, at, max_entries)?;
            path.extend(above);
            return index.iter().any(|&site| selects(flow, &path, site, &compared)).then_some(n);
        }
        cur = p;
    }
    None
}

/// The bound the conditional branch at op `at` of the instruction at `p`
/// states, from the comparison in the branch itself or in the nearest
/// instruction falling into it that writes what the condition reads, with the
/// registers that comparison reads and the instructions above `p` down to it.
fn guard_at(
    flow: &mut Flow,
    p: u64,
    ops: &[FullOp],
    at: usize,
    max_entries: usize,
) -> Option<(usize, Vec<Key>, Vec<u64>)> {
    let data = flow.data;
    let regs = |keys: Vec<Key>| -> Vec<Key> { keys.into_iter().filter(|k| matches!(k, Key::Reg(..))).collect() };
    let want: Vec<Key> = ops[at].ins.get(1).and_then(|vn| key_of(vn, data)).into_iter().collect();
    let (reads, feeding) = feeders(&ops[..at], want, data);
    let reads = regs(reads);
    if let Some(n) = guard_cases(&[], &feeding, ops, at, data, max_entries, false) {
        return Some((n, reads, Vec::new()));
    }
    let mut above = Vec::new();
    let mut cur = p;
    for _ in 0..MAX_GUARD_INSNS {
        let preds: Vec<u64> = flow.edges.preds(cur).collect();
        let &[w] = preds.as_slice() else { return None };
        let insn = flow.insn(w)?;
        if w.wrapping_add(u64::from(insn.0)) != cur || !kuna_switchtable::falls_through(&insn.1) {
            return None;
        }
        above.push(w);
        let wops = &insn.1;
        let writes = wops.iter().filter_map(|op| op.out.as_ref().and_then(|vn| key_of(vn, data)));
        let written: Vec<Key> = writes.filter(|w| reads.iter().any(|r| keys_overlap(*r, *w))).collect();
        if !written.is_empty() {
            let (compared, feeding) = feeders(wops, written, data);
            let n = guard_cases(wops, &feeding, ops, at, data, max_entries, false)?;
            return Some((n, regs(compared), above));
        }
        cur = w;
    }
    None
}

/// Does the table-address addend `key`, read by op `upto` of the instruction
/// at `vma` on `path` (the dispatch first, the comparison last), name a
/// register in `compared` once followed up `path` through copies, extensions
/// and arithmetic with a constant, as [`kuna_switchtable::mask_limit`] follows
/// an index? Any other definition on the way is a different value.
fn selects(flow: &mut Flow, path: &[u64], (vma, upto, mut key): (u64, usize, Key), compared: &[Key]) -> bool {
    let data = flow.data;
    let Some(start) = path.iter().position(|&at| at == vma) else { return false };
    let names = |key: Key| compared.iter().any(|c| keys_overlap(*c, key));
    let mut upto = Some(upto);
    for (i, &at) in path.iter().enumerate().skip(start) {
        if i + 1 == path.len() && names(key) {
            return true;
        }
        let Some(insn) = flow.insn(at) else { return false };
        let ops = &insn.1[..upto.take().unwrap_or(insn.1.len()).min(insn.1.len())];
        for op in ops.iter().rev() {
            let Some(out) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
            if !keys_overlap(out, key) {
                continue;
            }
            let input = |i: usize| op.ins.get(i).and_then(|vn| key_of(vn, data));
            let constant = |i: usize| op.ins.get(i).is_some_and(|vn| is_constant(vn));
            if out != key {
                if op.opcode == OpCode::CPUI_INT_ZEXT && input(0) == Some(key) {
                    continue;
                }
                return false;
            }
            let next = match op.opcode {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => input(0),
                OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                    if constant(1) =>
                {
                    input(0)
                }
                OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_ADD if constant(0) => input(1),
                _ => None,
            };
            let Some(next) = next else { return false };
            key = next;
        }
    }
    names(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Agreeing predecessors keep the value, a disagreeing one loses it, and
    /// one that says nothing (undecoded, or the question itself) is skipped.
    #[test]
    fn the_meet_keeps_agreement_and_skips_silence() {
        let a = Reached::Known(Val::Const(0x1000));
        let b = Reached::Known(Val::Const(0x2000));
        assert_eq!(a.meet(a), a);
        assert_eq!(a.meet(Reached::Nothing), a);
        assert_eq!(Reached::Nothing.meet(Reached::Nothing), Reached::Nothing);
        assert_eq!(a.meet(b), Reached::Unknown);
        assert_eq!(Reached::Unknown.meet(Reached::Nothing), Reached::Unknown);
    }

    #[test]
    fn edges_answer_the_predecessors_of_one_address() {
        let edges = Edges::new(vec![(0x30, 0x10), (0x20, 0x10), (0x30, 0x20), (0x30, 0x20)]);
        assert_eq!(edges.preds(0x30).collect::<Vec<_>>(), [0x10, 0x20]);
        assert_eq!(edges.preds(0x20).collect::<Vec<_>>(), [0x10]);
        assert_eq!(edges.preds(0x10).count(), 0);
    }
}
