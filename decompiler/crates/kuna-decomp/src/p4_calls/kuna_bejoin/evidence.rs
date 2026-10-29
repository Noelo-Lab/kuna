//! (kuna) The per-function proof `option bejoin evidence` joins a
//! two-register value by, read off the function's own instructions.
//!
//! The answer has to be a function of the callee alone: a call reads its
//! callee's pair in whatever order the callee's own RETURN joins it, and the
//! two are decided in different decompiles, in any order, in any process
//! (`--jobs`, a narrowed run, one `kuna decompile`, a front-end session). So
//! the driver walks a function's raw p-code at most once per image, before any
//! pipeline runs, and both seams read the same cached verdict ([`seed`]): the
//! function's own and each direct callee's. A call the pipeline resolves only
//! mid-way (a constant-propagated function pointer, a `-mlong-calls` call
//! through a register) asks about a callee with no verdict at hand and reads
//! it as not proven; after the pass the driver takes that verdict, and if it
//! is proven runs the pass again with it ([`missed_proven`]). A caller that
//! reads only the first register of a proven callee reads the value's high
//! word ([`build_high_read`]).
//!
//! The walk follows every path from the entry to a RETURN or a call that
//! does not return, executing the p-code over symbolic values ([`Term`]): a
//! register's value at the entry, a literal, an operation on values, or an
//! opaque value (a load, a call's result or clobber, a partly written
//! register). At each RETURN the model's pair must hold one of three shapes
//! an `int` function has no reason to compute, both registers written by the
//! function and neither read again before the RETURN:
//!
//! * a sign extension: the first register is the second register's value
//!   shifted right arithmetically by all but one of its bits, the shift (and
//!   the carry its instruction computes for nothing) the value's only reader;
//! * a zero extension: the first register is a literal zero, and nothing reads
//!   the value;
//! * the two halves of one product of two extended words, the low half made
//!   by an instruction of its own -- a separate low product of the same two
//!   words (PowerPC's `mullw` beside `mulhw`), or a move out of the register
//!   the multiply left it in (MIPS's `mflo`) -- and read by nothing.
//!
//! "Nothing reads" counts every path, a path ending in a call that does not
//! return included: gcc -Os hoists an error path's computation into a branch
//! delay slot (`beqz a0; addiu v1,a1,1`), so the RETURN of `return 0` finds a
//! value in `$3` that only the path calling `exit` reads. An extended value
//! must be made on purpose: computed by the function, or another register's
//! argument; not a literal, not the second register's own entry value, not a
//! load, a call's result or a piece of a wider value. A call reads every
//! argument register holding a value the function put there. A loop, an
//! indirect jump, a tail branch to another function, an operation given no
//! register inputs, or a walk over its budget proves nothing, and neither does
//! an "entry" whose first instruction calls itself (the placeholder an
//! unrelocated call in an object file leaves).

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::architecture::Architecture;
use crate::context::VarnodeId;
use crate::fspec::{effect_type, FuncCallSpecs, ParamActive, ProtoModel};
use crate::funcdata::Funcdata;

/// How many paths one walk follows before it gives up.
const MAX_PATHS: usize = 64;

/// How many p-code operations one walk executes before it gives up.
const MAX_OPS: usize = 20_000;

/// How many machine instructions one path runs through before it gives up.
const MAX_PATH_INSNS: usize = 512;

/// A storage location: space index, offset, size.
type Key = (int4, u64, u32);

/// The model's two-register output pair, high word's register first, with
/// the space each lies in.
#[derive(Clone, Debug)]
pub struct WidePair {
    pub hi: (Address, int4),
    pub lo: (Address, int4),
}

impl WidePair {
    fn key(loc: &(Address, int4)) -> Option<Key> {
        Some((loc.0.get_space()?.get_index(), loc.0.get_offset(), loc.1 as u32))
    }
}

/// A function, by `(space index, entry offset)`.
type Entry = (int4, uintb);

/// What `option bejoin evidence` keeps per image: the model's pair (once
/// looked up; `Some(None)` when the model joins first register low) and
/// every verdict taken.
#[derive(Default)]
pub struct WideCache {
    pair: Option<Option<WidePair>>,
    map: HashMap<Entry, bool>,
}

/// One decompile's view: the model's pair, the verdicts of the function and
/// of each callee it calls directly (or that an earlier pass of the same
/// decompile asked about), and each callee a call asked about without one.
pub struct WideView {
    pub pair: WidePair,
    verdicts: HashMap<Entry, bool>,
    missed: std::cell::RefCell<Vec<Entry>>,
}

impl WideView {
    fn get(&self, key: Entry) -> bool {
        self.verdicts.get(&key).copied().unwrap_or_else(|| {
            self.missed.borrow_mut().push(key);
            false
        })
    }
}

/// The verdict for `key`, taken once per image.
fn verdict(arch: &Architecture, map: &mut HashMap<Entry, bool>, model: &Rc<ProtoModel>, pair: &WidePair, key: Entry) -> bool {
    if let Some(&v) = map.get(&key) {
        return v;
    }
    let v = arch
        .manage()
        .get_space(key.0)
        .is_some_and(|sp| proves_wide(arch, model, pair, &Address::new(Rc::clone(sp), key.1)));
    map.insert(key, v);
    v
}

/// A symbolic value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Term {
    /// A location's value at the function's entry.
    Input(Key),
    /// A literal of the given size.
    Const(u64, u32),
    /// An operation on values, with its output size.
    Op(OpCode, Args, u32),
    /// A value nothing here can name: a load, a call's result or clobber, a
    /// partly written register.
    Opaque(u32),
}

/// An operation's inputs: at most three values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Args {
    n: u8,
    v: [usize; 3],
}

impl std::ops::Deref for Args {
    type Target = [usize];
    fn deref(&self) -> &[usize] {
        &self.v[..self.n as usize]
    }
}

/// The values one walk has made, each once.
#[derive(Default)]
struct Terms {
    list: Vec<Term>,
    index: HashMap<Term, usize>,
    opaque: u32,
}

impl Terms {
    fn intern(&mut self, t: Term) -> usize {
        if let Some(&id) = self.index.get(&t) {
            return id;
        }
        self.list.push(t.clone());
        self.index.insert(t, self.list.len() - 1);
        self.list.len() - 1
    }

    fn opaque(&mut self) -> usize {
        self.opaque += 1;
        let t = Term::Opaque(self.opaque);
        self.intern(t)
    }

    fn get(&self, id: usize) -> &Term {
        &self.list[id]
    }

    fn literal(&self, id: usize) -> Option<u64> {
        match self.get(id) {
            Term::Const(v, _) => Some(*v),
            _ => None,
        }
    }
}

/// One operand an operation read on a path.
#[derive(Clone, Debug)]
struct Read {
    term: usize,
    insn: u64,
    opc: OpCode,
    out: Option<usize>,
}

/// A path's state.
#[derive(Clone)]
struct Path {
    at: Address,
    /// The most recent write of each location, with the instruction that made
    /// the value the location holds.
    regs: Vec<(Key, usize, u64)>,
    /// A call has run: a register nothing wrote since holds the call's clobber
    /// unless the model says calls leave it alone.
    after_call: bool,
    reads: Vec<Read>,
    /// The first instruction on the path that computed each value.
    made_at: HashMap<usize, u64>,
    /// Was the high or low register read after its last write, by another
    /// instruction than the one that wrote it (which reads it only for the
    /// flags it sets)?
    hi_read: bool,
    lo_read: bool,
    hi_at: Option<u64>,
    lo_at: Option<u64>,
    insns: usize,
    visited: Vec<u64>,
}

/// Where a branch goes.
enum Dest {
    Op(usize),
    Insn(Address),
}

/// What one RETURN reached holds in the pair, with the instruction that
/// last wrote the low register and the one that made its value.
struct Site {
    hi: usize,
    lo: usize,
    lo_writer: Option<u64>,
    lo_made: Option<u64>,
}

/// What one decoded instruction is.
struct Insn {
    len: u64,
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
    /// Does it copy three or more registers into others at once (a register
    /// window's `save` or `restore`)?
    window: bool,
}

#[derive(Default)]
struct Collect {
    ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
}

impl kuna_sleigh::translate::PcodeEmit for Collect {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

/// The model's two-register output pair in the ABI's order, when its output
/// rule joins the two first general registers high word first.
pub fn wide_pair(model: &ProtoModel, manager: &kuna_base::space::AddrSpaceManager) -> Option<WidePair> {
    let list = model.output_list()?;
    let regs: Vec<(Address, int4)> = list
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == crate::dtype::type_class::TYPECLASS_GENERAL && e.get_join_record().is_none())
        .filter(|e| e.get_space().get_type() == spacetype::IPTR_PROCESSOR)
        .map(|e| (Address::new(Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .take(2)
        .collect();
    let [first, second] = &regs[..] else { return None };
    if first.1 != second.1 {
        return None;
    }
    let mut probe = ParamActive::new(false);
    for (a, s) in [first, second] {
        probe.register_trial(a, *s);
    }
    for i in 0..2 {
        probe.get_trial_mut(i).mark_active();
    }
    if model.derive_output_map(&mut probe, manager).is_err() || !(0..2).all(|i| probe.get_trial(i).is_used()) {
        return None;
    }
    let (lo, hi) = probe.join_pair_order();
    if (lo, hi) == (0, 1) {
        return None;
    }
    let piece = |i: int4| (probe.get_trial(i).get_address().clone(), probe.get_trial(i).get_size());
    Some(WidePair { hi: piece(hi), lo: piece(lo) })
}

/// Take (or reuse, per image) the verdict for the function `data` is, for
/// every direct callee it calls and for every callee an earlier pass of this
/// decompile asked about, and its pair, for the RETURN and CALL seams to read.
/// Inert unless `option bejoin` is `evidence` and the image's model joins its
/// pair high word first.
pub fn seed(arch: &mut Architecture, data: &mut Funcdata) {
    if arch.be_join != super::BeJoinMode::Evidence {
        return;
    }
    let Some(model) = arch.default_fp().cloned() else { return };
    let mut cache = std::mem::take(&mut arch.kuna_wide);
    if cache.pair.is_none() {
        cache.pair = Some(wide_pair(&model, arch.manage()));
    }
    let Some(Some(pair)) = cache.pair.clone() else {
        arch.kuna_wide = cache;
        return;
    };
    let mut entries: Vec<Entry> = Vec::new();
    let key = |a: &Address| a.get_space().map(|sp| (sp.get_index(), a.get_offset()));
    entries.extend(key(data.get_address()));
    for i in 0..data.num_calls() {
        let entry = data.get_call_specs(i).get_entry_address();
        if !entry.is_invalid() {
            entries.extend(key(entry));
        }
    }
    if let Some(earlier) = data.kuna_wide() {
        entries.extend(earlier.verdicts.keys().copied());
        entries.extend(earlier.missed.borrow().iter().copied());
    }
    let mut verdicts = HashMap::new();
    for k in entries {
        if !verdicts.contains_key(&k) {
            verdicts.insert(k, verdict(arch, &mut cache.map, &model, &pair, k));
        }
    }
    data.kuna_set_wide(WideView { pair, verdicts, missed: Default::default() });
    arch.kuna_wide = cache;
}

/// After a pass of the pipeline: did a call ask about a callee no verdict was
/// taken for (one the pipeline resolved an indirect call to), and is any such
/// callee proven? Then the driver runs the pass again, and [`seed`] takes
/// those callees' verdicts too.
pub fn missed_proven(arch: &mut Architecture, data: &Funcdata) -> bool {
    let Some(view) = data.kuna_wide() else { return false };
    let missed = view.missed.borrow().clone();
    if missed.is_empty() {
        return false;
    }
    let Some(model) = arch.default_fp().cloned() else { return false };
    let mut cache = std::mem::take(&mut arch.kuna_wide);
    let mut any = false;
    for k in missed {
        any |= verdict(arch, &mut cache.map, &model, &view.pair, k);
    }
    arch.kuna_wide = cache;
    any
}

/// Is the function at `entry` proven to return its pair as one wide value,
/// by the verdict the driver seeded for this decompile ([`seed`])? A callee
/// with none is not, and is noted for [`missed_proven`].
pub fn proven(data: &Funcdata, entry: &Address) -> bool {
    let Some(sp) = entry.get_space() else { return false };
    crate::p4_calls::kuna_callwide::proven(data, entry)
        || data.kuna_wide().is_some_and(|w| w.get((sp.get_index(), entry.get_offset())))
}

/// The walk: does every RETURN of the function at `entry` hand back one
/// value twice a register's width in `pair` (see the module header)?
fn proves_wide(arch: &Architecture, model: &Rc<ProtoModel>, pair: &WidePair, entry: &Address) -> bool {
    let (Some(hi_key), Some(lo_key)) = (WidePair::key(&pair.hi), WidePair::key(&pair.lo)) else { return false };
    if entry.get_space().is_none_or(|s| s.get_type() != spacetype::IPTR_PROCESSOR) {
        return false;
    }
    let mut w = Walk {
        arch,
        model,
        hi: hi_key,
        lo: lo_key,
        terms: Terms::default(),
        insns: HashMap::new(),
        reads: Vec::new(),
        sites: Vec::new(),
        spaces: HashMap::new(),
        args: model
            .input_opt()
            .map(|l| {
                l.get_entry()
                    .iter()
                    .filter(|e| e.get_space().get_type() == spacetype::IPTR_PROCESSOR && e.get_join_record().is_none())
                    .map(|e| (e.get_space().get_index(), e.get_base(), e.get_size() as u32))
                    .collect()
            })
            .unwrap_or_default(),
        entry: entry.clone(),
        ops: 0,
    };
    let tracked = arch.with_context_db(|db| db.get_tracked_set(entry).clone());
    let mut start = Path {
        at: entry.clone(),
        regs: Vec::new(),
        after_call: false,
        reads: Vec::new(),
        made_at: HashMap::new(),
        hi_read: false,
        lo_read: false,
        hi_at: None,
        lo_at: None,
        insns: 0,
        visited: Vec::new(),
    };
    for t in &tracked {
        let Some(sp) = t.loc.space.as_ref() else { continue };
        let c = w.terms.intern(Term::Const(t.val & ones(t.loc.size), t.loc.size));
        start.regs.push(((sp.get_index(), t.loc.offset, t.loc.size), c, entry.get_offset()));
    }
    let mut todo = vec![(start, 0usize)];
    let mut paths = 1usize;
    while let Some((path, op)) = todo.pop() {
        if w.run(path, op, &mut todo, &mut paths).is_none() || paths > MAX_PATHS {
            return false;
        }
    }
    let sites = std::mem::take(&mut w.sites);
    !sites.is_empty() && sites.iter().all(|site| w.proves(site))
}

struct Walk<'a> {
    arch: &'a Architecture,
    model: &'a Rc<ProtoModel>,
    hi: Key,
    lo: Key,
    terms: Terms,
    insns: HashMap<u64, Rc<Insn>>,
    /// Every operand read on every path walked, a no-return path's included.
    reads: Vec<Read>,
    /// What each RETURN reached holds.
    sites: Vec<Site>,
    spaces: HashMap<int4, Rc<AddrSpace>>,
    args: Vec<Key>,
    entry: Address,
    ops: usize,
}

fn ones(size: u32) -> u64 {
    if size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

fn overlaps(a: &Key, b: &Key) -> bool {
    a.0 == b.0 && a.1 < b.1 + b.2 as u64 && b.1 < a.1 + a.2 as u64
}

impl<'a> Walk<'a> {
    /// Decode (once per walk) the instruction at `at`.
    fn insn(&mut self, at: &Address) -> Option<Rc<Insn>> {
        if let Some(i) = self.insns.get(&at.get_offset()) {
            return Some(Rc::clone(i));
        }
        let mut c = Collect::default();
        let len = self.arch.translate().one_instruction(&mut c, at).ok().filter(|&n| n > 0)?;
        let mut copies = 0;
        for (opc, out, ins) in &c.ops {
            for v in out.iter().chain(ins.iter()) {
                if let Some(sp) = v.space.as_ref() {
                    self.spaces.entry(sp.get_index()).or_insert_with(|| Rc::clone(sp));
                }
            }
            let reg = |v: &VarnodeData| v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
            if *opc == OpCode::CPUI_COPY && out.as_ref().is_some_and(reg) && ins.first().is_some_and(reg) {
                copies += 1;
            }
        }
        let i = Rc::new(Insn { len: len as u64, ops: c.ops, window: copies >= 3 });
        self.insns.insert(at.get_offset(), Rc::clone(&i));
        Some(i)
    }

    fn key(v: &VarnodeData) -> Option<Key> {
        Some((v.space.as_ref()?.get_index(), v.offset, v.size))
    }

    fn is_const(v: &VarnodeData) -> bool {
        v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
    }

    /// Does a call leave the location alone?
    fn preserved(&self, k: &Key) -> bool {
        let Some(sp) = self.spaces.get(&k.0) else { return false };
        if sp.get_type() != spacetype::IPTR_PROCESSOR && sp.get_type() != spacetype::IPTR_INTERNAL {
            return true;
        }
        self.model.has_effect(&Address::new(Rc::clone(sp), k.1), k.2 as int4) == effect_type::UNAFFECTED
    }

    /// The value `v` holds on `path`, marking a read of the pair's registers.
    fn value(&mut self, path: &mut Path, v: &VarnodeData) -> usize {
        if Self::is_const(v) {
            return self.terms.intern(Term::Const(v.offset & ones(v.size), v.size));
        }
        let Some(k) = Self::key(v) else { return self.terms.opaque() };
        let here = path.at.get_offset();
        if overlaps(&k, &self.hi) && path.hi_at != Some(here) {
            path.hi_read = true;
        }
        if overlaps(&k, &self.lo) && path.lo_at != Some(here) {
            path.lo_read = true;
        }
        let t = self.read(path, &k);
        if let Some(&(r, _, by)) = path.regs.iter().rev().find(|(r, _, _)| overlaps(r, &k)) {
            if r != k {
                path.made_at.entry(t).or_insert(by);
            }
        }
        t
    }

    /// The value `k` holds on `path`: the latest write of exactly it, a piece
    /// of the latest write that contains it, or its entry value (a call's
    /// clobber once a call ran, unless the model says calls leave it alone).
    fn read(&mut self, path: &Path, k: &Key) -> usize {
        if let Some(&(r, t, _)) = path.regs.iter().rev().find(|(r, _, _)| overlaps(r, k)) {
            if r == *k {
                return t;
            }
            if r.1 <= k.1 && k.1 + k.2 as u64 <= r.1 + r.2 as u64 {
                let big = self.spaces.get(&k.0).is_some_and(|s| s.is_big_endian());
                let shift = if big { (r.1 + r.2 as u64) - (k.1 + k.2 as u64) } else { k.1 - r.1 };
                let c = self.terms.intern(Term::Const(shift, 4));
                return self.fold(OpCode::CPUI_SUBPIECE, vec![t, c], k.2);
            }
            return self.terms.opaque();
        }
        if path.after_call && !self.preserved(k) {
            return self.terms.opaque();
        }
        self.terms.intern(Term::Input(*k))
    }

    fn write(&mut self, path: &mut Path, v: &VarnodeData, t: usize, by: u64) {
        let Some(k) = Self::key(v) else { return };
        path.regs.retain(|(r, _, _)| !overlaps(r, &k));
        path.regs.push((k, t, by));
        if overlaps(&k, &self.hi) {
            path.hi_read = false;
            path.hi_at = Some(path.at.get_offset());
        }
        if overlaps(&k, &self.lo) {
            path.lo_read = false;
            path.lo_at = Some(path.at.get_offset());
        }
    }

    /// The value of `opc` over `ins`, folded when every input is a literal
    /// and reduced when the operation only moves its input.
    fn fold(&mut self, opc: OpCode, ins: Vec<usize>, size: u32) -> usize {
        let lit: Vec<Option<u64>> = ins.iter().map(|&i| self.terms.literal(i)).collect();
        let mask = ones(size);
        let moved = match (opc, &lit[..]) {
            (OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD, [None, Some(0)]) => Some(ins[0]),
            (OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_ADD, [Some(0), None]) => Some(ins[1]),
            (OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND, [None, None]) if ins[0] == ins[1] => Some(ins[0]),
            _ => None,
        };
        if let Some(m) = moved {
            return m;
        }
        if lit.iter().all(|l| l.is_some()) && !lit.is_empty() {
            let a = lit[0].unwrap_or(0);
            let b = lit.get(1).copied().flatten().unwrap_or(0);
            let r = match opc {
                OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => Some(a),
                OpCode::CPUI_INT_ADD => Some(a.wrapping_add(b)),
                OpCode::CPUI_INT_SUB => Some(a.wrapping_sub(b)),
                OpCode::CPUI_INT_AND => Some(a & b),
                OpCode::CPUI_INT_OR => Some(a | b),
                OpCode::CPUI_INT_XOR => Some(a ^ b),
                OpCode::CPUI_INT_EQUAL => Some((a == b) as u64),
                OpCode::CPUI_INT_NOTEQUAL => Some((a != b) as u64),
                OpCode::CPUI_BOOL_NEGATE => Some((a == 0) as u64),
                OpCode::CPUI_BOOL_AND => Some(((a != 0) && (b != 0)) as u64),
                OpCode::CPUI_BOOL_OR => Some(((a != 0) || (b != 0)) as u64),
                OpCode::CPUI_INT_NEGATE => Some(!a),
                OpCode::CPUI_INT_2COMP => Some(a.wrapping_neg()),
                OpCode::CPUI_INT_LEFT => Some(a.checked_shl(b as u32).unwrap_or(0)),
                OpCode::CPUI_INT_RIGHT => Some(a.checked_shr(b as u32).unwrap_or(0)),
                OpCode::CPUI_SUBPIECE => Some(a.checked_shr(8 * b as u32).unwrap_or(0)),
                _ => None,
            };
            if let Some(r) = r {
                return self.terms.intern(Term::Const(r & mask, size));
            }
        }
        if ins.len() > 3 {
            return self.terms.opaque();
        }
        let mut v = [0usize; 3];
        v[..ins.len()].copy_from_slice(&ins);
        self.terms.intern(Term::Op(opc, Args { n: ins.len() as u8, v }, size))
    }

    /// Run one path from op `op` of the instruction at `path.at`: `Some(true)`
    /// at a RETURN holding a wide value, `Some(false)` at a path end that is no
    /// RETURN (a no-return call), `None` when the function proves nothing.
    fn run(&mut self, mut path: Path, mut op: usize, todo: &mut Vec<(Path, usize)>, paths: &mut usize) -> Option<bool> {
        loop {
            let at = path.at.clone();
            if op == 0 {
                if path.visited.contains(&at.get_offset()) || path.insns >= MAX_PATH_INSNS {
                    return None;
                }
                path.visited.push(at.get_offset());
                path.insns += 1;
            }
            let insn = self.insn(&at)?;
            let mut next: Option<Address> = Some(&at + insn.len as i64);
            while op < insn.ops.len() {
                self.ops += 1;
                if self.ops > MAX_OPS {
                    return None;
                }
                let (opc, out, ins) = &insn.ops[op];
                let here = at.get_offset();
                op += 1;
                match opc {
                    OpCode::CPUI_COPY => {
                        let t = self.value(&mut path, &ins[0]);
                        let by = if insn.window {
                            Self::key(&ins[0])
                                .and_then(|k| path.regs.iter().rev().find(|(r, _, _)| *r == k).map(|r| r.2))
                                .unwrap_or(here)
                        } else {
                            here
                        };
                        if let Some(o) = out {
                            self.write(&mut path, o, t, by);
                        }
                    }
                    OpCode::CPUI_LOAD => {
                        let a = self.value(&mut path, &ins[1]);
                        path.reads.push(Read { term: a, insn: here, opc: *opc, out: None });
                        let t = self.terms.opaque();
                        if let Some(o) = out {
                            self.write(&mut path, o, t, here);
                        }
                    }
                    OpCode::CPUI_STORE => {
                        for v in &ins[1..] {
                            let t = self.value(&mut path, v);
                            path.reads.push(Read { term: t, insn: here, opc: *opc, out: None });
                        }
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        if *opc == OpCode::CPUI_CALLIND {
                            let t = self.value(&mut path, &ins[0]);
                            path.reads.push(Read { term: t, insn: here, opc: *opc, out: None });
                        }
                        for (k, t, _) in path.regs.clone() {
                            let arg = self.args.iter().any(|a| overlaps(a, &k));
                            if arg && self.terms.get(t) != &Term::Input(k) {
                                path.reads.push(Read { term: t, insn: here, opc: *opc, out: None });
                            }
                        }
                        let target = (*opc == OpCode::CPUI_CALL)
                            .then(|| ins[0].space.as_ref().map(|sp| Address::new(Rc::clone(sp), ins[0].offset)))
                            .flatten();
                        if target.as_ref().is_some_and(|t| *t == self.entry && here == self.entry.get_offset()) {
                            return None;
                        }
                        if target.is_some_and(|t| crate::p4_calls::kuna_calleenoreturn::is_no_return(self.arch, &t)) {
                            self.reads.append(&mut path.reads);
                            return Some(false);
                        }
                        for i in 0..path.regs.len() {
                            if !self.preserved(&path.regs[i].0) {
                                path.regs[i].1 = self.terms.opaque();
                                path.regs[i].2 = here;
                            }
                        }
                        path.after_call = true;
                        path.hi_read = false;
                        path.lo_read = false;
                    }
                    OpCode::CPUI_CALLOTHER => {
                        if ins[1..].iter().all(Self::is_const) {
                            return None;
                        }
                        for v in &ins[1..] {
                            let t = self.value(&mut path, v);
                            path.reads.push(Read { term: t, insn: here, opc: *opc, out: None });
                        }
                        if let Some(o) = out {
                            let t = self.terms.opaque();
                            self.write(&mut path, o, t, here);
                        }
                    }
                    OpCode::CPUI_BRANCHIND => return None,
                    OpCode::CPUI_RETURN => {
                        let site = self.site(&path)?;
                        self.sites.push(site);
                        self.reads.append(&mut path.reads);
                        return Some(true);
                    }
                    OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                        let cond = if *opc == OpCode::CPUI_CBRANCH {
                            let c = self.value(&mut path, &ins[1]);
                            path.reads.push(Read { term: c, insn: here, opc: *opc, out: None });
                            self.terms.literal(c).map(|v| v != 0)
                        } else {
                            Some(true)
                        };
                        if cond == Some(false) {
                            continue;
                        }
                        let dest = self.destination(&insn, op - 1, &ins[0], &next)?;
                        if cond == Some(true) {
                            match dest {
                                Dest::Op(i) => op = i,
                                Dest::Insn(a) => {
                                    next = Some(a);
                                    op = insn.ops.len();
                                }
                            }
                            continue;
                        }
                        *paths += 1;
                        let mut fork = path.clone();
                        match dest {
                            Dest::Op(i) => todo.push((fork, i)),
                            Dest::Insn(a) => {
                                fork.at = a;
                                todo.push((fork, 0));
                            }
                        }
                    }
                    _ => {
                        let mut ts = Vec::with_capacity(ins.len());
                        for v in ins {
                            ts.push(self.value(&mut path, v));
                        }
                        let size = out.as_ref().map_or(0, |o| o.size);
                        let t = self.fold(*opc, ts.clone(), size);
                        if !ts.contains(&t) {
                            for &i in &ts {
                                if !matches!(self.terms.get(i), Term::Const(..)) {
                                    path.reads.push(Read { term: i, insn: here, opc: *opc, out: Some(t) });
                                }
                            }
                        }
                        if *opc == OpCode::CPUI_SUBPIECE {
                            path.made_at.entry(t).or_insert(here);
                        }
                        if let Some(o) = out {
                            self.write(&mut path, o, t, here);
                        }
                    }
                }
            }
            let to = next?;
            let spaces = &self.spaces;
            path.regs.retain(|(k, _, _)| spaces.get(&k.0).is_none_or(|s| s.get_type() != spacetype::IPTR_INTERNAL));
            path.at = to;
            op = 0;
        }
    }

    /// Where a branch at op `index` of `insn` goes: another op of the same
    /// instruction (a p-code relative target), or an instruction; `None` for a
    /// tail branch into another function, or a relative target out of range.
    fn destination(&self, insn: &Insn, index: usize, dest: &VarnodeData, next: &Option<Address>) -> Option<Dest> {
        if Self::is_const(dest) {
            let bits = 8 * dest.size.clamp(1, 8);
            let off = ((dest.offset << (64 - bits)) as i64) >> (64 - bits);
            let to = index as i64 + off;
            if to == insn.ops.len() as i64 {
                return next.clone().map(Dest::Insn);
            }
            if to < 0 || to > insn.ops.len() as i64 {
                return None;
            }
            return Some(Dest::Op(to as usize));
        }
        let sp = dest.space.as_ref()?;
        let a = Address::new(Rc::clone(sp), dest.offset);
        if a.get_offset() < self.entry.get_offset()
            || (a != self.entry
                && (self.arch.symboltab.find_function_across_scopes(&a).is_some()
                    || self.arch.is_lazy_entry(a.get_offset())))
        {
            return None;
        }
        Some(Dest::Insn(a))
    }

    /// What the RETURN `path` reached holds, when both pair registers were
    /// written on the path and neither was read again by another instruction.
    fn site(&mut self, path: &Path) -> Option<Site> {
        let written = |k: &Key| path.regs.iter().any(|(r, _, _)| r == k);
        if path.hi_read || path.lo_read || !written(&self.hi) || !written(&self.lo) {
            return None;
        }
        let (hi, lo) = (self.read(path, &self.hi.clone()), self.read(path, &self.lo.clone()));
        let lo_writer = path.regs.iter().rev().find(|(r, _, _)| *r == self.lo).map(|r| r.2);
        Some(Site { hi, lo, lo_writer, lo_made: path.made_at.get(&lo).copied() })
    }

    /// Does `site` hold one of the module header's shapes, judged against the
    /// reads of every path the walk took?
    fn proves(&self, site: &Site) -> bool {
        let size = self.lo.2;
        self.sign_extended(site.hi, site.lo, size) || self.zero_extended(site.lo, site.hi) || self.product(site, size)
    }

    fn sign_extended(&self, hi: usize, lo: usize, size: u32) -> bool {
        let Term::Op(OpCode::CPUI_INT_SRIGHT, ins, _) = self.terms.get(hi) else { return false };
        if ins.len() != 2 || ins[0] != lo || self.terms.literal(ins[1]) != Some(8 * size as u64 - 1) || !self.on_purpose(lo) {
            return false;
        }
        let shifts: Vec<u64> = self
            .reads
            .iter()
            .filter(|r| r.term == lo && r.opc == OpCode::CPUI_INT_SRIGHT && r.out == Some(hi))
            .map(|r| r.insn)
            .collect();
        if shifts.is_empty() {
            return false;
        }
        let mut flags: Vec<usize> = self
            .reads
            .iter()
            .filter(|r| r.term == lo && shifts.contains(&r.insn) && r.out != Some(hi))
            .filter_map(|r| r.out)
            .collect();
        let mut k = 0;
        while k < flags.len() {
            let f = flags[k];
            for r in self.reads.iter().filter(|r| r.term == f && shifts.contains(&r.insn)) {
                if let Some(o) = r.out.filter(|o| !flags.contains(o) && *o != hi) {
                    flags.push(o);
                }
            }
            k += 1;
        }
        self.reads.iter().all(|r| {
            if r.term == lo {
                shifts.contains(&r.insn) && r.opc != OpCode::CPUI_CALL && r.opc != OpCode::CPUI_CALLIND
            } else if flags.contains(&r.term) {
                shifts.contains(&r.insn)
            } else {
                true
            }
        })
    }

    fn zero_extended(&self, lo: usize, hi: usize) -> bool {
        self.terms.literal(hi) == Some(0) && self.on_purpose(lo) && !self.reads.iter().any(|r| r.term == lo)
    }

    fn product(&self, site: &Site, size: u32) -> bool {
        let (hi, lo) = (site.hi, site.lo);
        let Some(wide) = self.high_half_of(hi, size) else { return false };
        let Term::Op(OpCode::CPUI_INT_MULT, ins, _) = self.terms.get(wide) else { return false };
        let factor = |t: usize| match self.terms.get(t) {
            Term::Op(OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, x, _) if x.len() == 1 => Some(x[0]),
            _ => None,
        };
        let (Some(a), Some(b)) = (factor(ins[0]), factor(ins[1])) else { return false };
        let own = match self.terms.get(lo) {
            Term::Op(OpCode::CPUI_INT_MULT, l, s) => *s == size && l.len() == 2 && ((l[0] == a && l[1] == b) || (l[0] == b && l[1] == a)),
            Term::Op(OpCode::CPUI_SUBPIECE, l, _) => {
                l.len() == 2
                    && l[0] == wide
                    && self.terms.literal(l[1]) == Some(0)
                    && site.lo_writer.is_some()
                    && site.lo_made.is_some()
                    && site.lo_writer != site.lo_made
            }
            _ => false,
        };
        own && !self.reads.iter().any(|r| r.term == lo)
    }

    /// The wide value `t` is the high `size` bytes of.
    fn high_half_of(&self, t: usize, size: u32) -> Option<usize> {
        let Term::Op(OpCode::CPUI_SUBPIECE, ins, _) = self.terms.get(t) else { return None };
        let (w, k) = (ins[0], self.terms.literal(ins[1])?);
        let wsize = match self.terms.get(w) {
            Term::Op(_, _, s) => *s,
            _ => return None,
        };
        if wsize != 2 * size {
            return None;
        }
        if k == size as u64 {
            return Some(w);
        }
        if k != 0 {
            return None;
        }
        let Term::Op(OpCode::CPUI_INT_RIGHT, sh, _) = self.terms.get(w) else { return None };
        (self.terms.literal(sh[1])? == 8 * size as u64).then_some(sh[0])
    }

    /// Is `t` made on purpose: computed, or the argument another register
    /// passed in?
    fn on_purpose(&self, t: usize) -> bool {
        match self.terms.get(t) {
            Term::Input(k) => !overlaps(k, &self.lo) && self.args.iter().any(|a| overlaps(a, k)),
            Term::Const(..) | Term::Opaque(_) => false,
            Term::Op(OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, x, _) => x.len() == 1 && self.on_purpose(x[0]),
            Term::Op(OpCode::CPUI_SUBPIECE | OpCode::CPUI_PIECE, _, _) => false,
            Term::Op(..) => true,
        }
    }
}

/// Give a call whose callee is proven to return its pair as one wide value
/// ([`seed`]) that pair as its output when the caller reads only the first
/// register: the read becomes the pair's high piece, `(int)(g() >> 32)`,
/// rather than the whole of a narrower result, which C's truncation would
/// read as the low word.
///
/// Runs in `buildOutputFromTrials` while the unused trials are still present;
/// `trialvn` holds each trial's INDIRECT creation, by original slot. Returns
/// `false`, changing nothing, unless the pair is built.
pub(crate) fn build_high_read(fc: &mut FuncCallSpecs, data: &mut Funcdata, trialvn: &[Option<VarnodeId>]) -> bool {
    if super::mode(data) != super::BeJoinMode::Evidence {
        return false;
    }
    let reads: Vec<(Address, int4, VarnodeId)> = {
        let active = fc.get_active_output();
        (0..active.get_num_trials())
            .map(|i| active.get_trial(i))
            .filter(|t| t.is_used())
            .filter_map(|t| trialvn.get((t.get_slot() - 1) as usize).copied().flatten().map(|vn| (t.get_address().clone(), t.get_size(), vn)))
            .collect()
    };
    let [(ref raddr, rsize, vn)] = reads[..] else { return false };
    let entry = fc.get_entry_address().clone();
    if !proven(data, &entry) {
        return false;
    }
    let Some(pair) = data.kuna_wide().map(|w| w.pair.clone()) else { return false };
    let ((hi_addr, hi_size), (lo_addr, lo_size)) = (pair.hi, pair.lo);
    let within = hi_addr.justified_contain(hi_size, raddr, rsize, false);
    if within < 0 {
        return false;
    }
    let op = fc.get_op();
    let Some(call_addr) = data.obank().get(op).map(|o| o.get_addr().clone()) else { return false };
    let Some(joinaddr) = crate::kuna_rustabi::pair_join_address(data, &hi_addr, hi_size, &lo_addr, lo_size, &call_addr)
    else {
        return false;
    };
    let Ok(whole) = data.new_varnode_out(lo_size + hi_size, &joinaddr, op) else { return false };
    if let Some(v) = data.vbank_mut().get_mut(whole) {
        v.set_write_mask();
    }
    let indop = data.vbank().get(vn).and_then(|v| v.get_def());
    let sub = data.new_op(2, call_addr);
    data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
    let c = data.new_constant(4, (lo_size + within) as u64);
    let _ = data.op_set_input(sub, whole, 0);
    let _ = data.op_set_input(sub, c, 1);
    let _ = data.op_set_output(sub, vn);
    data.op_insert_after(sub, op);
    if let Some(indop) = indop {
        data.op_destroy(indop);
    }
    true
}
