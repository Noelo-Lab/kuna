//! JVM code for one analysed method. Blocks are laid out in Dalvik address
//! order, except that a block reached only by an unconditional `goto` follows
//! its predecessor (as javac lays out a handler's code), so exception and
//! local ranges are built from the blocks they cover. Each `(register,
//! category)` pair is one JVM local (parameters keep their slots), and within a block a value defined once and read once is
//! left on the operand stack for its reader instead of stored: pending
//! definitions form a stack, and one folds into an instruction's operand
//! only when everything pending above it folds into the same instruction in
//! evaluation order; anything else is stored first, in Dalvik order.
//! Constants are emitted at every use they reach alone, and a join whose
//! incoming values are all constants read once at the join's start gets them
//! on the stack from each edge (`b = x > 3`, `c ? 5 : 7`), as javac writes it.

use std::collections::BTreeMap;

use crate::backport::Call;
use crate::classfile::{op, Asm, Body, Label, Pool};
use crate::code::{cat, is_wide, kind, size, Kind, Mask, Method, Op, D, F, I, J, L};
use crate::dex::{Dex, Value, NO_INDEX};
use crate::insn::{fill_array, packed_switch, sparse_switch};
use crate::{err, Result};

pub const THIS_NAME: u32 = NO_INDEX - 1;

/// Marks the second local of a reused parameter register in `slots`.
const REUSED: u32 = 1 << 31;

/// Class-level state a method's code adds to.
pub struct Shared<'p> {
    pub pool: &'p mut Pool,
    pub bootstrap: &'p mut Vec<(u16, Vec<u16>)>,
    /// The class needs at least this class-file version.
    pub version: u16,
    /// Whether a class (by descriptor) is an interface.
    pub interfaces: &'p dyn Fn(&[u8]) -> bool,
    pub verifiable: bool,
    /// The other DEX files of the input, whose d8 classes calls here may use.
    pub peers: &'p [crate::Dex<'p>],
}

struct Emitter<'m, 'd, 'a, 's, 'p> {
    m: &'m Method<'d, 'a>,
    sh: &'s mut Shared<'p>,
    asm: Asm,
    slots: BTreeMap<(u32, Mask), u16>,
    next_slot: u16,
    labels: Vec<Label>,
    stubs: BTreeMap<u32, Label>,
    deferred: Vec<bool>,
    pending: Vec<usize>,
    user: Vec<usize>,
    pc_at: Vec<u32>,
    pc_end: Vec<u32>,
    block_pc: Vec<(u32, u32)>,
    next_block: Vec<u32>,
    /// Joins that receive their value on the stack: block to its edges.
    joins: BTreeMap<u32, Join>,
    silent: Vec<bool>,
    on_stack: Vec<bool>,
    stmts: Vec<(u32, u32)>,
    min_addr: u32,
    regions: Vec<crate::region::Region>,
}

/// The `Code` attribute body for `m`.
pub fn code_attribute(m: &Method, sh: &mut Shared) -> Result<Vec<u8>> {
    for wide in [false, true] {
        let mut e = Emitter::new(m, sh, wide);
        e.run()?;
        if e.asm.resolve()? {
            return e.finish();
        }
    }
    Err(err("branch offsets overflow".to_string()))
}

/// A join block entered with its phi's value on the stack: the constant op
/// each predecessor brings, and a stub (push, then on to the join) for each
/// constant arriving by a branch.
struct Join {
    edges: Vec<(u32, usize)>,
    stubs: Vec<(usize, Label)>,
    /// The phi's register, category and value when the join stores it.
    store: Option<(u32, Mask, u32)>,
}

fn internal(desc: &[u8]) -> &[u8] {
    match desc {
        [b'L', rest @ .., b';'] => rest,
        d => d,
    }
}

impl<'m, 'd, 'a, 's, 'p> Emitter<'m, 'd, 'a, 's, 'p> {
    fn new(m: &'m Method<'d, 'a>, sh: &'s mut Shared<'p>, wide: bool) -> Self {
        let mut asm = Asm::new(wide);
        let labels = (0..m.blocks.len()).map(|_| asm.label()).collect();
        let mut slots = BTreeMap::new();
        let mut next_slot = 0u16;
        for &(r, c) in &m.params {
            slots.insert((r, c), next_slot);
            next_slot += size(c) as u16;
        }
        let mut user = vec![usize::MAX; m.direct_uses.len()];
        for (k, o) in m.ops.iter().enumerate() {
            for u in &o.uses {
                if (u.val as usize) < user.len() {
                    user[u.val as usize] = k;
                }
            }
        }
        Emitter {
            m,
            sh,
            asm,
            slots,
            next_slot,
            labels,
            stubs: BTreeMap::new(),
            deferred: vec![false; m.ops.len()],
            pending: Vec::new(),
            user,
            pc_at: vec![u32::MAX; m.ops.len() + 1],
            pc_end: vec![u32::MAX; m.ops.len()],
            block_pc: vec![(u32::MAX, u32::MAX); m.blocks.len()],
            next_block: vec![u32::MAX; m.blocks.len()],
            joins: BTreeMap::new(),
            silent: vec![false; m.ops.len()],
            on_stack: vec![false; m.ops.len()],
            stmts: Vec::new(),
            min_addr: u32::MAX,
            regions: Vec::new(),
        }
    }

    fn slot(&mut self, reg: u32, c: Mask) -> Result<u16> {
        if let Some(&s) = self.slots.get(&(reg, c)) {
            return Ok(s);
        }
        let s = self.next_slot;
        self.next_slot = s
            .checked_add(size(c) as u16)
            .ok_or_else(|| err("more than 65535 JVM locals".to_string()))?;
        self.slots.insert((reg, c), s);
        Ok(s)
    }

    /// The local holding value `v` of register `reg` in category `c`: a
    /// parameter's register d8 reuses for another value gets a second local,
    /// so the parameter's own keeps the parameter throughout.
    fn local(&mut self, reg: u32, c: Mask, v: u32) -> Result<u16> {
        let m = self.m;
        let reused = m.params.iter().enumerate().any(|(p, &pr)| pr == (reg, c) && m.root(p as u32) != m.root(v));
        self.slot(if reused { reg | REUSED } else { reg }, c)
    }

    fn candidate(&self, k: usize) -> bool {
        let m = self.m;
        let o = &m.ops[k];
        let Some(d) = &o.def else { return false };
        if o.kind == Kind::Absorbed || o.insn.op == 0x0d || m.inline_const(d.id) || m.named_def(k) {
            return false;
        }
        let (n, phi) = m.web_uses.get(&m.root(d.id)).copied().unwrap_or((0, false));
        let u = self.user[d.id as usize];
        n == 1 && !phi && m.direct_uses[d.id as usize] == 1 && u > k && u < m.ops.len() && m.ops[u].block == o.block
    }

    /// Finds the joins whose phi takes only constants on every edge: the value
    /// stays on the stack when it is read once, as the first operand of the
    /// join's first statement, and is stored at the join otherwise.
    fn find_joins(&mut self) -> Result<()> {
        let m = self.m;
        let mut members: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for v in 0..m.def_op.len() as u32 {
            members.entry(m.root(v)).or_default().push(v);
        }
        let mut preds: Vec<Vec<u32>> = vec![Vec::new(); m.blocks.len()];
        let mut handler = vec![false; m.blocks.len()];
        for (b, blk) in m.blocks.iter().enumerate().filter(|(_, b)| b.reachable) {
            for &s in &blk.succs {
                if !preds[s as usize].contains(&(b as u32)) {
                    preds[s as usize].push(b as u32);
                }
            }
            for h in &blk.handlers {
                handler[h.1 as usize] = true;
            }
        }
        for (i, (&j, &r)) in m.phi_block.iter().zip(&m.phi_reg).enumerate() {
            let phi = m.defs + i as u32;
            let root = m.root(phi);
            let ju = j as usize;
            let uses = m.web_uses.get(&root).map_or(0, |w| w.0);
            let caught = m.handler_live[r as usize / 64] >> (r % 64) & 1 != 0;
            if j == 0 || caught || handler[ju] || self.joins.contains_key(&j) || uses == 0 || m.direct_uses[phi as usize] != uses {
                continue;
            }
            let web = &members[&root];
            let consts = web.iter().all(|&v| v == phi || m.const_op(v).is_some());
            if !consts {
                continue;
            }
            let k = self.user[phi as usize];
            let blk = &m.blocks[ju];
            let direct = uses == 1 && k < m.ops.len() && m.ops[k].block == j && {
                let quiet = (blk.start..k).all(|q| {
                    let o = &m.ops[q];
                    o.kind == Kind::Absorbed || o.insn.op == 0x00 || o.def.as_ref().is_some_and(|d| m.inline_const(d.id))
                });
                quiet && m.ops[k].uses.first().map(|u| u.val) == Some(phi) && !self.candidate(k)
            };
            let mut edges = Vec::new();
            for &p in &preds[ju] {
                match m.edge_const.get(&(j, r, p)) {
                    Some(&v) if p != j && m.root(v) == root => edges.push((p, m.const_op(v).expect("constant"))),
                    _ => break,
                }
            }
            if edges.len() != preds[ju].len() || edges.len() < 2 {
                continue;
            }
            for &v in web {
                if v != phi {
                    self.silent[m.def_op[v as usize]] = true;
                }
            }
            let store = if direct {
                self.on_stack[k] = true;
                None
            } else {
                Some((r, m.cat[phi as usize], phi))
            };
            let mut stubs: Vec<(usize, Label)> = Vec::new();
            for &(p, c) in &edges {
                let last = &m.ops[m.blocks[p as usize].end - 1];
                let branches = matches!(last.insn.op, 0x2b..=0x3d)
                    && m.targets(last)?.iter().any(|&t| m.ops[m.ops.partition_point(|o| o.insn.addr < t)].block == j);
                let key = &m.ops[c].insn;
                if branches && !stubs.iter().any(|&(s, _)| m.ops[s].insn.lit == key.lit && m.ops[s].insn.idx == key.idx && m.ops[s].insn.op == key.op) {
                    let l = self.asm.label();
                    stubs.push((c, l));
                }
            }
            self.joins.insert(j, Join { edges, stubs, store });
        }
        Ok(())
    }

    /// The constant op predecessor `p` brings to join `j`.
    fn edge_value(&self, p: u32, j: u32) -> Option<usize> {
        self.joins.get(&j)?.edges.iter().find(|e| e.0 == p).map(|e| e.1)
    }

    /// Where a branch from block `p` to block `t` lands: `t`, or the stub
    /// pushing `p`'s constant for join `t`.
    fn land(&self, p: u32, t: u32) -> Label {
        let m = self.m;
        if let (Some(c), Some(join)) = (self.edge_value(p, t), self.joins.get(&t)) {
            let key = &m.ops[c].insn;
            if let Some(&(_, l)) = join.stubs.iter().find(|&&(s, _)| {
                let x = &m.ops[s].insn;
                x.op == key.op && x.lit == key.lit && x.idx == key.idx
            }) {
                return l;
            }
        }
        self.labels[t as usize]
    }

    fn join_cat(&self, j: u32) -> Mask {
        let m = self.m;
        if let Some((_, c, _)) = self.joins[&j].store {
            return c;
        }
        let k = (m.blocks[j as usize].start..m.ops.len()).find(|&k| self.on_stack[k]).expect("join use");
        m.use_cat(&m.ops[k], 0)
    }

    fn run(&mut self) -> Result<()> {
        let m = self.m;
        self.find_joins()?;
        let mut order = layout(m);
        self.regions = crate::region::arrange(m, &mut order);
        for w in order.windows(2) {
            self.next_block[w[0] as usize] = w[1];
        }
        for (i, &b) in order.iter().enumerate() {
            let b = b as usize;
            let blk = &m.blocks[b];
            let start = self.asm.pc();
            if let Some(join) = self.joins.get(&(b as u32)) {
                let c = self.join_cat(b as u32);
                let stubs = join.stubs.clone();
                for (i, &(k, l)) in stubs.iter().enumerate() {
                    self.asm.bind(l);
                    self.asm.depth = 0;
                    self.push_const(k, c)?;
                    if i + 1 < stubs.len() {
                        self.asm.branch(op::GOTO, self.labels[b], 0);
                    }
                }
            }
            self.asm.bind(self.labels[b]);
            let catches = m.ops[blk.start].insn.op == 0x0d;
            self.asm.depth = i32::from(catches) + i32::from(self.joins.contains_key(&(b as u32)));
            if let Some((r, c, phi)) = self.joins.get(&(b as u32)).and_then(|j| j.store) {
                let slot = self.local(r, c, phi)?;
                self.asm.local(op::ISTORE, kind(c), slot, -size(c));
            }
            for k in blk.start..blk.end {
                self.pc_at[k] = self.asm.pc();
                self.step(k, b)?;
            }
            self.flush()?;
            let next = order.get(i + 1).copied();
            if falls(&m.ops[blk.end - 1]) {
                let f = b as u32 + 1;
                let stubbed = match self.joins.get(&f) {
                    Some(join) => {
                        let stubbed = !join.stubs.is_empty();
                        let c = self.edge_value(b as u32, f).expect("join edge");
                        let cat = self.join_cat(f);
                        self.push_const(c, cat)?;
                        stubbed
                    }
                    None => false,
                };
                if stubbed || next != Some(f) {
                    self.asm.branch(op::GOTO, self.labels[f as usize], 0);
                }
            }
            for k in blk.start..blk.end - 1 {
                self.pc_end[k] = self.pc_at[k + 1];
            }
            self.pc_end[blk.end - 1] = self.asm.pc();
            self.block_pc[b] = (start, self.asm.pc());
        }
        let handlers: Vec<u32> = m
            .blocks
            .iter()
            .filter(|b| b.reachable)
            .flat_map(|b| b.handlers.iter().map(|h| h.1))
            .collect();
        for h in handlers {
            let hb = &m.blocks[h as usize];
            if !hb.reachable || m.ops[hb.start].insn.op == 0x0d || self.stubs.contains_key(&h) {
                continue;
            }
            let l = self.asm.label();
            self.asm.bind(l);
            self.asm.depth = 1;
            self.asm.op(op::POP, &[], -1);
            self.asm.branch(op::GOTO, self.labels[h as usize], 0);
            self.stubs.insert(h, l);
        }
        Ok(())
    }

    fn step(&mut self, k: usize, b: usize) -> Result<()> {
        let m = self.m;
        let o = &m.ops[k];
        if o.kind == Kind::Absorbed || o.insn.op == 0x00 || self.silent[k] {
            return Ok(());
        }
        if let Some(d) = &o.def {
            if m.inline_const(d.id) || m.param_copy(d.id).is_some() {
                return Ok(());
            }
        }
        if o.insn.op == 0x0d {
            if k != m.blocks[b].start {
                return Err(err(format!("move-exception at {:#x} is not first in its handler", o.insn.addr)));
            }
            let start = self.asm.pc();
            self.min_addr = o.insn.addr;
            self.store_or_pop(k)?;
            self.stmts.push((start, self.min_addr));
            return Ok(());
        }
        let mut cands: Vec<usize> = Vec::new();
        for u in &o.uses {
            let d = m.def_op[u.val as usize];
            if d < m.ops.len() && self.pending.contains(&d) {
                cands.push(d);
            }
        }
        let mut folded = 0;
        let mut above = usize::MAX;
        while folded < self.pending.len() {
            let p = self.pending[self.pending.len() - 1 - folded];
            match cands.iter().position(|&d| d == p) {
                Some(pos) if pos < above => {
                    above = pos;
                    folded += 1;
                }
                _ => break,
            }
        }
        let keep = self.pending.len() - folded;
        self.pending.truncate(keep);
        if self.candidate(k) {
            if folded < cands.len() {
                self.flush()?;
            }
            self.deferred[k] = true;
            self.pending.push(k);
            return Ok(());
        }
        self.flush()?;
        self.statement(k)
    }

    fn flush(&mut self) -> Result<()> {
        for d in std::mem::take(&mut self.pending) {
            self.deferred[d] = false;
            self.statement(d)?;
        }
        Ok(())
    }

    fn statement(&mut self, k: usize) -> Result<()> {
        let start = self.asm.pc();
        self.min_addr = u32::MAX;
        if !self.iinc(k)? {
            let c = self.core(k)?;
            if c != 0 {
                self.store_or_pop(k)?;
            }
        }
        if self.asm.pc() > start {
            self.stmts.push((start, self.min_addr));
        }
        Ok(())
    }

    fn store_or_pop(&mut self, k: usize) -> Result<()> {
        let m = self.m;
        let d = m.ops[k].def.as_ref().expect("def");
        let c = m.def_cat(d);
        let (uses, _) = m.web_uses.get(&m.root(d.id)).copied().unwrap_or((0, false));
        if uses == 0 && !m.named_def(k) {
            self.asm.op(if is_wide(c) { op::POP2 } else { op::POP }, &[], -size(c));
        } else {
            let s = self.local(d.reg, c, d.id)?;
            self.asm.local(op::ISTORE, kind(c), s, -size(c));
        }
        Ok(())
    }

    /// `add-int/lit` on one local becomes `iinc`.
    fn iinc(&mut self, k: usize) -> Result<bool> {
        let m = self.m;
        let o = &m.ops[k];
        if !matches!(o.insn.op, 0xd0 | 0xd8) {
            return Ok(false);
        }
        let (u, d) = (&o.uses[0], o.def.as_ref().expect("def"));
        let by = i16::try_from(o.insn.lit).ok();
        let src = m.def_op[u.val as usize];
        let folded = src < m.ops.len() && self.deferred[src];
        if u.reg != d.reg || by.is_none() || folded || m.inline_const(u.val) || m.def_cat(d) != I || m.use_cat(o, 0) != I {
            return Ok(false);
        }
        let s = self.local(d.reg, I, d.id)?;
        self.min_addr = self.min_addr.min(o.insn.addr);
        self.asm.iinc(s, by.expect("checked"));
        Ok(true)
    }

    fn push_int(&mut self, v: i32) {
        let p = self.sh.pool.int(v);
        match v {
            -1..=5 => self.asm.op(op::ICONST_0.wrapping_add(v as u8), &[], 1),
            -128..=127 => self.asm.op(op::BIPUSH, &[v as i8 as u8], 1),
            -32768..=32767 => self.asm.op_u16(op::SIPUSH, v as i16 as u16, 1),
            _ => self.ldc(p, 1),
        }
    }

    fn ldc(&mut self, idx: u16, delta: i32) {
        if idx < 256 {
            self.asm.op(op::LDC, &[idx as u8], delta);
        } else {
            self.asm.op_u16(op::LDC_W, idx, delta);
        }
    }

    /// A literal of category `c` (`bits` are the raw Dalvik bits).
    fn push_lit(&mut self, c: Mask, bits: i64) -> Result<()> {
        match c {
            I => self.push_int(bits as i32),
            F => {
                let b = bits as u32;
                match b {
                    0 | 0x3f80_0000 | 0x4000_0000 => {
                        let n = f32::from_bits(b) as u8;
                        self.asm.op(op::FCONST_0 + n, &[], 1)
                    }
                    _ => {
                        let p = self.sh.pool.float(b);
                        self.ldc(p, 1)
                    }
                }
            }
            J => match bits {
                0 | 1 => self.asm.op(op::LCONST_0 + bits as u8, &[], 2),
                _ => {
                    let p = self.sh.pool.long(bits);
                    self.asm.op_u16(op::LDC2_W, p, 2)
                }
            },
            D => match bits as u64 {
                0 | 0x3ff0_0000_0000_0000 => {
                    let n = u8::from(bits != 0);
                    self.asm.op(op::DCONST_0 + n, &[], 2)
                }
                b => {
                    let p = self.sh.pool.double(b);
                    self.asm.op_u16(op::LDC2_W, p, 2)
                }
            },
            _ => {
                if bits != 0 {
                    return Err(err(format!("non-zero constant {bits} used as a reference")));
                }
                self.asm.op(op::ACONST_NULL, &[], 1)
            }
        }
        Ok(())
    }

    fn push_const(&mut self, k: usize, c: Mask) -> Result<()> {
        let o = &self.m.ops[k];
        self.min_addr = self.min_addr.min(o.insn.addr);
        match o.insn.op {
            0x1a | 0x1b => {
                let s = self.m.dex.string(o.insn.idx)?;
                let p = self.sh.pool.string(s);
                self.ldc(p, 1);
                Ok(())
            }
            0x1c => {
                let p = self.class_ref(o.insn.idx)?;
                self.ldc(p, 1);
                Ok(())
            }
            _ => self.push_lit(c, o.insn.lit),
        }
    }

    fn push_use(&mut self, k: usize, u: usize) -> Result<()> {
        let m = self.m;
        let o = &m.ops[k];
        let v = o.uses[u].val;
        let c = m.use_cat(o, u);
        if u == 0 && self.on_stack[k] {
            return Ok(());
        }
        if m.inline_const(v) {
            return self.push_const(m.def_op[v as usize], c);
        }
        if let Some(p) = m.param_copy(v) {
            let (reg, pc) = m.params[p];
            let s = self.slot(reg, pc)?;
            self.asm.local(op::ILOAD, kind(pc), s, size(pc));
            return Ok(());
        }
        let d = m.def_op[v as usize];
        if d < m.ops.len() && self.deferred[d] {
            self.deferred[d] = false;
            let got = self.core(d)?;
            self.deferred[d] = true;
            if got == 0 {
                return Err(err(format!("folded op at {:#x} produced no value", m.ops[d].insn.addr)));
            }
            return Ok(());
        }
        let s = self.local(o.uses[u].reg, c, v)?;
        self.asm.local(op::ILOAD, kind(c), s, size(c));
        Ok(())
    }

    fn push_uses(&mut self, k: usize) -> Result<()> {
        for u in 0..self.m.ops[k].uses.len() {
            self.push_use(k, u)?;
        }
        Ok(())
    }

    fn class_ref(&mut self, ty: u32) -> Result<u16> {
        let name = self.class_name(ty)?;
        Ok(self.sh.pool.class(name))
    }

    fn field_ref(&mut self, idx: u32) -> Result<(u16, Mask)> {
        let dex = self.m.dex;
        let f = dex.field(idx)?;
        let class = internal(dex.type_desc(f.class)?);
        let desc = dex.type_desc(f.ty)?;
        Ok((self.sh.pool.field(class, dex.string(f.name)?, desc), cat(desc)))
    }

    fn proto_desc(&self, proto: u32) -> Result<Vec<u8>> {
        self.m.dex.proto_desc(proto)
    }

    /// A class's internal name, with d8's record superclass written back.
    fn class_name(&self, ty: u32) -> Result<&'a [u8]> {
        self.class_name_in(self.m.dex, ty)
    }

    fn class_name_in<'x>(&self, dex: &Dex<'x>, ty: u32) -> Result<&'x [u8]> {
        let name = internal(dex.type_desc(ty)?);
        Ok(if name == crate::record::RECORD_TAG && !self.sh.verifiable { b"java/lang/Record" } else { name })
    }

    fn method_ref(&mut self, idx: u32, interface: bool) -> Result<u16> {
        self.method_ref_in(self.m.dex, idx, interface)
    }

    fn method_ref_in(&mut self, dex: &Dex, idx: u32, interface: bool) -> Result<u16> {
        let mi = dex.method(idx)?;
        let class = self.class_name_in(dex, mi.class)?.to_vec();
        let desc = self.proto_desc(mi.proto)?;
        Ok(self.sh.pool.method(&class, dex.string(mi.name)?, &desc, interface))
    }

    fn method_handle(&mut self, idx: u32) -> Result<u16> {
        let dex = self.m.dex;
        let h = dex.method_handle(idx)?;
        let id = u32::from(h.member);
        let (kind, member) = match h.kind {
            0..=3 => {
                let kind = [4, 2, 3, 1][usize::from(h.kind)];
                (kind, self.field_ref(id)?.0)
            }
            4..=8 => {
                let kind = [6, 5, 8, 7, 9][usize::from(h.kind - 4)];
                let iface = h.kind == 8 || (self.sh.interfaces)(dex.type_desc(dex.method(id)?.class)?);
                (kind, self.method_ref(id, iface)?)
            }
            k => return Err(err(format!("method handle kind {k}"))),
        };
        self.sh.version = self.sh.version.max(51);
        Ok(self.sh.pool.method_handle(kind, member))
    }

    fn bootstrap_arg(&mut self, v: &Value) -> Result<u16> {
        let dex = self.m.dex;
        Ok(match *v {
            Value::Byte(x) => self.sh.pool.int(i32::from(x)),
            Value::Short(x) => self.sh.pool.int(i32::from(x)),
            Value::Char(x) => self.sh.pool.int(i32::from(x)),
            Value::Int(x) => self.sh.pool.int(x),
            Value::Boolean(x) => self.sh.pool.int(i32::from(x)),
            Value::Long(x) => self.sh.pool.long(x),
            Value::Float(x) => self.sh.pool.float(x),
            Value::Double(x) => self.sh.pool.double(x),
            Value::String(s) => self.sh.pool.string(dex.string(s)?),
            Value::Type(t) => self.class_ref(t)?,
            Value::MethodType(p) => {
                let d = self.proto_desc(p)?;
                self.sh.pool.method_type(&d)
            }
            Value::MethodHandle(h) => self.method_handle(h)?,
            _ => return Err(err("unsupported bootstrap argument".to_string())),
        })
    }

    /// The `invokedynamic` constant javac writes for a lambda or method reference.
    fn metafactory(&mut self, dex: &Dex, l: &crate::lambda::Lambda) -> Result<u16> {
        let mf = self.sh.pool.method(
            b"java/lang/invoke/LambdaMetafactory",
            b"metafactory",
            b"(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodType;Ljava/lang/invoke/MethodHandle;Ljava/lang/invoke/MethodType;)Ljava/lang/invoke/CallSite;",
            false,
        );
        let bsm = self.sh.pool.method_handle(6, mf);
        let sam = dex.proto_desc(l.sam_proto)?;
        let sam = self.sh.pool.method_type(&sam);
        let class = dex.type_desc(dex.method(l.method)?.class)?;
        let iface = l.kind == 9 || (l.kind != 8 && (self.sh.interfaces)(class));
        let target = self.method_ref_in(dex, l.method, iface)?;
        let handle = self.sh.pool.method_handle(l.kind, target);
        let inst = self.sh.pool.method_type(&l.instantiated);
        let entry = (bsm, vec![sam, handle, inst]);
        let index = match self.sh.bootstrap.iter().position(|e| *e == entry) {
            Some(i) => i,
            None => {
                self.sh.bootstrap.push(entry);
                self.sh.bootstrap.len() - 1
            }
        };
        let mut desc = vec![b'('];
        for &t in &l.captures {
            desc.extend_from_slice(dex.type_desc(t)?);
        }
        desc.push(b')');
        desc.extend_from_slice(dex.type_desc(l.iface)?);
        self.sh.version = self.sh.version.max(52);
        Ok(self.sh.pool.invoke_dynamic(index as u16, dex.string(l.sam_name)?, &desc))
    }

    fn invoke_dynamic(&mut self, site: u32) -> Result<u16> {
        let dex = self.m.dex;
        let values = dex.call_site(site)?;
        let (Some(Value::MethodHandle(h)), Some(Value::String(name)), Some(Value::MethodType(p))) =
            (values.first(), values.get(1), values.get(2))
        else {
            return Err(err(format!("malformed call site {site}")));
        };
        let bsm = self.method_handle(*h)?;
        let args = values[3..]
            .iter()
            .map(|v| self.bootstrap_arg(v))
            .collect::<Result<Vec<_>>>()?;
        let entry = (bsm, args);
        let index = match self.sh.bootstrap.iter().position(|e| *e == entry) {
            Some(i) => i,
            None => {
                self.sh.bootstrap.push(entry);
                self.sh.bootstrap.len() - 1
            }
        };
        let desc = self.proto_desc(*p)?;
        self.sh.version = self.sh.version.max(52);
        Ok(self.sh.pool.invoke_dynamic(index as u16, dex.string(*name)?, &desc))
    }

    fn uses_size(&self, k: usize) -> i32 {
        let o = &self.m.ops[k];
        (0..o.uses.len()).map(|u| size(self.m.use_cat(o, u))).sum()
    }

    fn branch_target(&self, o: &Op) -> Result<Label> {
        let t = self.m.targets(o)?[0];
        let k = self.m.ops.partition_point(|x| x.insn.addr < t);
        Ok(self.land(o.block, self.m.ops[k].block))
    }

    /// Emits op `k`'s operands and instruction; returns the category of the
    /// value it leaves on the stack (0 for none).
    fn core(&mut self, k: usize) -> Result<Mask> {
        let m = self.m;
        let o = &m.ops[k];
        let x = &o.insn;
        self.min_addr = self.min_addr.min(x.addr);
        let def_cat = o.def.as_ref().map(|d| m.def_cat(d)).unwrap_or(0);
        if let Kind::NewObject { ty } = o.kind {
            if let Some((owner, l)) = m.dex.lambda_in(ty, self.sh.peers).filter(|_| !self.sh.verifiable) {
                self.push_uses(k)?;
                let r = self.metafactory(owner, l)?;
                let pops = self.uses_size(k);
                self.asm.op(op::INVOKEDYNAMIC, &[(r >> 8) as u8, r as u8, 0, 0], 1 - pops);
                return Ok(L);
            }
            let c = self.class_ref(ty)?;
            self.asm.op_u16(op::NEW, c, 1);
            self.asm.op(op::DUP, &[], 1);
            self.push_uses(k)?;
            let r = self.method_ref(x.idx, false)?;
            let pops = self.uses_size(k) + 1;
            self.asm.op_u16(op::INVOKESPECIAL, r, -pops);
            return Ok(L);
        }
        match x.op {
            0x01..=0x09 => {
                self.push_use(k, 0)?;
                Ok(def_cat)
            }
            0x0e => {
                self.asm.op(op::RETURN, &[], 0);
                Ok(0)
            }
            0x0f..=0x11 => {
                self.push_use(k, 0)?;
                let c = m.use_cat(o, 0);
                self.asm.op(op::IRETURN + kind(c), &[], -size(c));
                Ok(0)
            }
            0x12..=0x1c => {
                self.push_const(k, def_cat)?;
                Ok(def_cat)
            }
            0x1d | 0x1e | 0x27 => {
                self.push_use(k, 0)?;
                let opc = match x.op {
                    0x1d => op::MONITORENTER,
                    0x1e => op::MONITOREXIT,
                    _ => op::ATHROW,
                };
                self.asm.op(opc, &[], -1);
                Ok(0)
            }
            0x1f | 0x20 => {
                self.push_use(k, 0)?;
                let c = self.class_ref(x.idx)?;
                self.asm.op_u16(if x.op == 0x1f { op::CHECKCAST } else { op::INSTANCEOF }, c, 0);
                Ok(def_cat)
            }
            0x21 => {
                self.push_use(k, 0)?;
                self.asm.op(op::ARRAYLENGTH, &[], 0);
                Ok(I)
            }
            0x22 => {
                let c = self.class_ref(x.idx)?;
                self.asm.op_u16(op::NEW, c, 1);
                Ok(L)
            }
            0x23 => {
                self.push_use(k, 0)?;
                let desc = m.dex.type_desc(x.idx)?;
                self.new_array(&desc[1..])?;
                Ok(L)
            }
            0x24 | 0x25 => {
                let desc = m.dex.type_desc(x.idx)?;
                let elem = &desc[1..];
                if !matches!(elem.first(), Some(b'I' | b'L' | b'[')) {
                    return Err(err(format!("filled-new-array of {}", String::from_utf8_lossy(desc))));
                }
                self.push_int(o.uses.len() as i32);
                self.new_array(elem)?;
                let store = if elem[0] == b'I' { op::IASTORE } else { op::IASTORE + 4 };
                for u in 0..o.uses.len() {
                    self.asm.op(op::DUP, &[], 1);
                    self.push_int(u as i32);
                    self.push_use(k, u)?;
                    self.asm.op(store, &[], -3);
                }
                if o.result {
                    Ok(L)
                } else {
                    self.asm.op(op::POP, &[], -1);
                    Ok(0)
                }
            }
            0x26 => {
                self.push_use(k, 0)?;
                let (width, elements) = fill_array(m.code.insns, x.target())?;
                let elem = m
                    .ref_desc(o.uses[0].val)
                    .and_then(|t| t.get(1).copied())
                    .unwrap_or(match width {
                        1 => b'B',
                        2 => b'S',
                        4 => b'I',
                        _ => b'J',
                    });
                let (c, store) = match elem {
                    b'Z' | b'B' => (I, 0x54),
                    b'C' => (I, 0x55),
                    b'S' => (I, 0x56),
                    b'I' => (I, op::IASTORE),
                    b'F' => (F, 0x51),
                    b'J' => (J, 0x50),
                    b'D' => (D, 0x52),
                    _ => return Err(err("fill-array-data on a non-primitive array".to_string())),
                };
                for (i, &e) in elements.iter().enumerate() {
                    let bits = match (width, elem) {
                        (1, _) => i64::from(e as u8 as i8),
                        (2, b'C') => e as i64,
                        (2, _) => i64::from(e as u16 as i16),
                        (4, _) => i64::from(e as u32 as i32),
                        _ => e as i64,
                    };
                    self.asm.op(op::DUP, &[], 1);
                    self.push_int(i as i32);
                    self.push_lit(c, bits)?;
                    self.asm.op(store, &[], -2 - size(c));
                }
                Ok(L)
            }
            0x28..=0x2a => {
                let addr = m.targets(o)?[0];
                let tb = m.ops[m.ops.partition_point(|x| x.insn.addr < addr)].block;
                let t = self.labels[tb as usize];
                if let Some(c) = self.edge_value(o.block, tb) {
                    let cat = self.join_cat(tb);
                    self.push_const(c, cat)?;
                }
                let stubbed = self.joins.get(&tb).is_some_and(|j| !j.stubs.is_empty());
                if self.next_block[o.block as usize] != tb || stubbed {
                    self.asm.branch(op::GOTO, t, 0);
                }
                Ok(0)
            }
            0x2b | 0x2c => {
                self.push_use(k, 0)?;
                let next = self.labels[o.block as usize + 1];
                let label_at = |e: &Self, rel: i32| -> Label {
                    let t = (i64::from(x.addr) + i64::from(rel)) as u32;
                    let k = e.m.ops.partition_point(|o| o.insn.addr < t);
                    e.land(o.block, e.m.ops[k].block)
                };
                if x.op == 0x2b {
                    let (first, rels) = packed_switch(m.code.insns, x.target())?;
                    let targets: Vec<Label> = rels.iter().map(|&r| label_at(self, r)).collect();
                    if targets.is_empty() {
                        self.asm.lookupswitch(next, &[]);
                    } else {
                        self.asm.tableswitch(next, first, &targets);
                    }
                } else {
                    let pairs: Vec<(i32, Label)> = sparse_switch(m.code.insns, x.target())?
                        .iter()
                        .map(|&(key, r)| (key, label_at(self, r)))
                        .collect();
                    if pairs.windows(2).any(|w| w[0].0 >= w[1].0) {
                        return Err(err("sparse-switch keys are not sorted".to_string()));
                    }
                    self.asm.lookupswitch(next, &pairs);
                }
                Ok(0)
            }
            0x2d..=0x31 => {
                self.push_uses(k)?;
                let opc = [0x95, 0x96, 0x97, 0x98, 0x94][usize::from(x.op - 0x2d)];
                let pops = self.uses_size(k);
                self.asm.op(opc, &[], 1 - pops);
                Ok(I)
            }
            0x32..=0x37 => {
                self.push_uses(k)?;
                let t = self.branch_target(o)?;
                let refs = m.use_cat(o, 0) == L || m.use_cat(o, 1) == L;
                let opc = if refs {
                    op::IF_ACMPEQ + (x.op - 0x32)
                } else {
                    op::IF_ICMPEQ + (x.op - 0x32)
                };
                self.asm.branch(opc, t, 2);
                Ok(0)
            }
            0x38..=0x3d => {
                self.push_use(k, 0)?;
                let t = self.branch_target(o)?;
                let opc = if m.use_cat(o, 0) == L {
                    if x.op == 0x38 {
                        op::IFNULL
                    } else {
                        op::IFNONNULL
                    }
                } else {
                    op::IFEQ + (x.op - 0x38)
                };
                self.asm.branch(opc, t, 1);
                Ok(0)
            }
            0x44..=0x4a => {
                self.push_uses(k)?;
                let opc = match x.op {
                    0x44 | 0x45 => op::IALOAD + kind(def_cat),
                    0x46 => op::IALOAD + 4,
                    0x47 | 0x48 => 0x33,
                    0x49 => 0x34,
                    _ => 0x35,
                };
                self.asm.op(opc, &[], size(def_cat) - 2);
                Ok(def_cat)
            }
            0x4b..=0x51 => {
                self.push_uses(k)?;
                let c = m.use_cat(o, 2);
                let opc = match x.op {
                    0x4b | 0x4c => op::IASTORE + kind(c),
                    0x4d => op::IASTORE + 4,
                    0x4e | 0x4f => 0x54,
                    0x50 => 0x55,
                    _ => 0x56,
                };
                self.asm.op(opc, &[], -2 - size(c));
                Ok(0)
            }
            0x52..=0x6d => {
                self.push_uses(k)?;
                let (f, c) = self.field_ref(x.idx)?;
                let (opc, delta) = match x.op {
                    0x52..=0x58 => (op::GETFIELD, size(c) - 1),
                    0x59..=0x5f => (op::PUTFIELD, -1 - size(c)),
                    0x60..=0x66 => (op::GETSTATIC, size(c)),
                    _ => (op::PUTSTATIC, -size(c)),
                };
                self.asm.op_u16(opc, f, delta);
                Ok(if matches!(x.op, 0x52..=0x58 | 0x60..=0x66) { c } else { 0 })
            }
            0x6e..=0x72 | 0x74..=0x78 | 0xfa..=0xfd => {
                self.push_uses(k)?;
                let pops = self.uses_size(k);
                let ret = m.result_mask(x)?;
                let push = if ret == 0 { 0 } else { size(ret) };
                match x.op {
                    0xfa | 0xfb => {
                        let dex = m.dex;
                        let mi = dex.method(x.idx)?;
                        let class = internal(dex.type_desc(mi.class)?).to_vec();
                        let desc = self.proto_desc(x.proto)?;
                        let r = self.sh.pool.method(&class, dex.string(mi.name)?, &desc, false);
                        self.asm.op_u16(op::INVOKEVIRTUAL, r, push - pops);
                    }
                    0xfc | 0xfd => {
                        let r = self.invoke_dynamic(x.idx)?;
                        self.asm.op(op::INVOKEDYNAMIC, &[(r >> 8) as u8, r as u8, 0, 0], push - pops);
                    }
                    0x71 | 0x77 if !self.sh.verifiable && m.dex.backport_in(x.idx, self.sh.peers).is_some() => {
                        let t = m.dex.backport_in(x.idx, self.sh.peers).expect("backport");
                        let iface = matches!(t.call, Call::Interface | Call::InterfaceStatic);
                        let r = self.sh.pool.method(t.owner.as_bytes(), t.name.as_bytes(), t.desc.as_bytes(), iface);
                        if iface {
                            self.sh.version = self.sh.version.max(52);
                        }
                        match t.call {
                            Call::Static | Call::InterfaceStatic => self.asm.op_u16(op::INVOKESTATIC, r, push - pops),
                            Call::Virtual => self.asm.op_u16(op::INVOKEVIRTUAL, r, push - pops),
                            Call::Interface => {
                                self.asm.op(op::INVOKEINTERFACE, &[(r >> 8) as u8, r as u8, pops as u8, 0], push - pops)
                            }
                        }
                    }
                    _ => {
                        let base = if x.op >= 0x74 { x.op - 0x74 } else { x.op - 0x6e };
                        let class = m.dex.type_desc(m.dex.method(x.idx)?.class)?;
                        let iface = base == 4 || (!self.sh.verifiable && (self.sh.interfaces)(class));
                        if iface && base != 4 {
                            self.sh.version = self.sh.version.max(52);
                        }
                        let r = self.method_ref(x.idx, iface)?;
                        match base {
                            0 => self.asm.op_u16(op::INVOKEVIRTUAL, r, push - pops),
                            1 | 2 => self.asm.op_u16(op::INVOKESPECIAL, r, push - pops),
                            3 => self.asm.op_u16(op::INVOKESTATIC, r, push - pops),
                            _ => self.asm.op(
                                op::INVOKEINTERFACE,
                                &[(r >> 8) as u8, r as u8, pops as u8, 0],
                                push - pops,
                            ),
                        }
                    }
                }
                if o.result {
                    Ok(ret)
                } else {
                    if ret != 0 {
                        self.asm.op(if is_wide(ret) { op::POP2 } else { op::POP }, &[], -size(ret));
                    }
                    Ok(0)
                }
            }
            0x7b..=0x8f => {
                self.push_use(k, 0)?;
                let c = m.use_cat(o, 0);
                match x.op {
                    0x7b | 0x7d | 0x7f | 0x80 => self.asm.op(0x74 + kind_index(c), &[], 0),
                    0x7c => {
                        self.push_int(-1);
                        self.asm.op(0x82, &[], -1);
                    }
                    0x7e => {
                        self.push_lit(J, -1)?;
                        self.asm.op(0x83, &[], -2);
                    }
                    _ => self.asm.op(x.op + 4, &[], size(def_cat) - size(c)),
                }
                Ok(def_cat)
            }
            0x90..=0xcf => {
                self.push_uses(k)?;
                let kk = if x.op >= 0xb0 { x.op - 0xb0 } else { x.op - 0x90 };
                let (t, j) = match kk {
                    0..=10 => (0, kk),
                    11..=21 => (1, kk - 11),
                    22..=26 => (2, kk - 22),
                    _ => (3, kk - 27),
                };
                let base = [0x60, 0x64, 0x68, 0x6c, 0x70, 0x7e, 0x80, 0x82, 0x78, 0x7a, 0x7c][usize::from(j)];
                let pops = self.uses_size(k);
                self.asm.op(base + t, &[], size(def_cat) - pops);
                Ok(def_cat)
            }
            0xd0..=0xe2 => {
                let j = if x.op >= 0xd8 { x.op - 0xd8 } else { x.op - 0xd0 };
                let lit = x.lit as i32;
                if j == 1 {
                    self.push_int(lit);
                    self.push_use(k, 0)?;
                } else {
                    self.push_use(k, 0)?;
                    self.push_int(lit);
                }
                let base = [0x60, 0x64, 0x68, 0x6c, 0x70, 0x7e, 0x80, 0x82, 0x78, 0x7a, 0x7c][usize::from(j)];
                self.asm.op(base, &[], -1);
                Ok(I)
            }
            0xfe => {
                let p = self.method_handle(x.idx)?;
                self.ldc(p, 1);
                Ok(L)
            }
            0xff => {
                let d = self.proto_desc(x.idx)?;
                let p = self.sh.pool.method_type(&d);
                self.sh.version = self.sh.version.max(51);
                self.ldc(p, 1);
                Ok(L)
            }
            _ => Err(err(format!("cannot translate opcode {:#04x} at {:#x}", x.op, x.addr))),
        }
    }

    fn new_array(&mut self, elem: &[u8]) -> Result<()> {
        let atype = match elem.first() {
            Some(b'Z') => 4,
            Some(b'C') => 5,
            Some(b'F') => 6,
            Some(b'D') => 7,
            Some(b'B') => 8,
            Some(b'S') => 9,
            Some(b'I') => 10,
            Some(b'J') => 11,
            _ => 0,
        };
        if atype != 0 {
            self.asm.op(op::NEWARRAY, &[atype], 0);
        } else {
            let c = self.sh.pool.class(internal(elem));
            self.asm.op_u16(op::ANEWARRAY, c, 0);
        }
        Ok(())
    }

    /// The JVM pc intervals a try item protects: every block it covers; and,
    /// as javac protects a `synchronized` block, its own catch-all handler's
    /// `move-exception` right before it, and the `monitor-exit` right after
    /// it when that handler releases the same monitor.
    fn coverage(&self, t: &crate::dex::Try) -> Vec<(u32, u32)> {
        let m = self.m;
        let end = t.start + t.count;
        let mut iv: Vec<(u32, u32)> = m
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| b.reachable && (t.start..end).contains(&m.ops[b.start].insn.addr))
            .map(|(i, _)| self.block_pc[i])
            .collect();
        let at = |addr: u32| m.ops.partition_point(|o| o.insn.addr < addr);
        if let Some(&(None, h)) = t.handlers.last() {
            let (hk, k) = (at(h), at(end));
            let catches = m.ops.get(hk).filter(|o| o.insn.op == 0x0d && o.insn.addr == h);
            let start = at(t.start);
            if let Some(prev) = start.checked_sub(1).map(|p| &m.ops[p]) {
                if prev.insn.op == 0x0d && prev.insn.addr == h && prev.end == t.start && m.blocks[prev.block as usize].reachable {
                    iv.push(self.block_pc[prev.block as usize]);
                }
            }
            let exit = m.ops.get(k).filter(|o| o.insn.op == 0x1e && o.insn.addr == end);
            let releases = catches.is_some()
                && m.ops.get(hk + 1).is_some_and(|o| o.insn.op == 0x1e && Some(o.insn.a) == exit.map(|e| e.insn.a));
            if let Some(e) = exit.filter(|e| releases && m.blocks[e.block as usize].reachable) {
                iv.push((self.block_pc[e.block as usize].0, self.pc_end[k]));
            }
        }
        merge(iv)
    }

    /// Whether every block that can throw meets, in the exception table, the
    /// handlers its try item lists, in that order, up to a catch-all.
    fn covers_as_dex(&self, table: &[(u32, u32, u32, Option<u32>)]) -> bool {
        let m = self.m;
        m.blocks.iter().enumerate().filter(|(b, blk)| blk.reachable && crate::region::throws(m, *b as u32)).all(|(b, blk)| {
            let (s, e) = self.block_pc[b];
            let mut seen = Vec::new();
            for &(start, end, h, ty) in table {
                if start < e && s < end {
                    if !(start <= s && e <= end) {
                        return false;
                    }
                    seen.push((ty, h));
                    if ty.is_none() {
                        break;
                    }
                }
            }
            seen == blk.handlers
        })
    }

    /// The JVM pc intervals of the ops whose addresses are in `[start, end)`.
    fn span(&self, start: u32, end: u32) -> Vec<(u32, u32)> {
        let m = self.m;
        let first = m.ops.partition_point(|o| o.insn.addr < start);
        let iv = (first..m.ops.len())
            .take_while(|&k| m.ops[k].insn.addr < end)
            .filter(|&k| self.pc_at[k] != u32::MAX)
            .map(|k| (self.pc_at[k], self.pc_end[k]))
            .collect();
        merge(iv)
    }

    fn finish(self) -> Result<Vec<u8>> {
        let m = self.m;
        let code = &self.asm.code;
        if code.len() > 0xffff {
            return Err(err(format!("method needs {} bytes of JVM code (limit 65535)", code.len())));
        }
        let mut table: Vec<(u32, u32, u32, Option<u32>)> = Vec::new();
        let per_item = |e: &Self, skip: &dyn Fn(u32, Option<u32>) -> bool, table: &mut Vec<(u32, u32, u32, Option<u32>)>| {
            for t in &m.code.tries {
                for (start, end) in e.coverage(t) {
                    for &(ty, addr) in &t.handlers {
                        let b = m.ops[m.ops.partition_point(|o| o.insn.addr < addr)].block;
                        if m.blocks[b as usize].reachable && !skip(b, ty) {
                            table.push((start, end, b, ty));
                        }
                    }
                }
            }
        };
        per_item(&self, &|b, ty| self.regions.iter().any(|r| r.handler == b && r.ty == ty), &mut table);
        for r in &self.regions {
            table.push((self.block_pc[r.first as usize].0, self.block_pc[r.last as usize].1, r.handler, r.ty));
            for (start, end) in merge(r.own.iter().map(|&b| self.block_pc[b as usize]).collect()) {
                table.push((start, end, r.handler, r.ty));
            }
        }
        if !self.covers_as_dex(&table) {
            table.clear();
            per_item(&self, &|_, _| false, &mut table);
        }
        let mut entries = Body::default();
        for &(start, end, b, ty) in &table {
            let label = self.stubs.get(&b).copied().unwrap_or(self.labels[b as usize]);
            let pc = self.asm.bound(label).ok_or_else(|| err("unbound handler".to_string()))?;
            let catch = match ty {
                Some(ty) => self.sh.pool.class(internal(m.dex.type_desc(ty)?)),
                None if crate::region::rethrows(m, b) => 0,
                None => self.sh.pool.class(b"java/lang/Throwable"),
            };
            entries.u16(start as u16).u16(end as u16).u16(pc as u16).u16(catch);
        }
        let mut attrs: Vec<(u16, Vec<u8>)> = Vec::new();
        let mut lines = Body::default();
        let mut count = 0u16;
        let mut last = None;
        for &(pc, addr) in &self.stmts {
            let i = m.debug.lines.partition_point(|&(a, _)| a <= addr);
            if i == 0 {
                continue;
            }
            let line = m.debug.lines[i - 1].1;
            if last != Some(line) && line <= 0xffff {
                lines.u16(pc as u16).u16(line as u16);
                count += 1;
                last = Some(line);
            }
        }
        if count > 0 {
            let mut b = Body::default();
            b.u16(count);
            b.0.extend_from_slice(&lines.0);
            attrs.push((self.sh.pool.utf8(b"LineNumberTable"), b.0));
        }
        let mut lvt = Body::default();
        let mut lvtt = Body::default();
        let (mut n, mut nt) = (0u16, 0u16);
        let mut seen = std::collections::BTreeSet::new();
        for l in &m.debug.locals {
            let (name, desc, c): (&[u8], Vec<u8>, Mask) = if l.name == THIS_NAME {
                (b"this", m.this_desc.clone(), L)
            } else if l.ty == NO_INDEX || l.name == NO_INDEX {
                continue;
            } else {
                let desc = m.dex.type_desc(l.ty)?;
                (m.dex.string(l.name)?, desc.to_vec(), cat(desc))
            };
            let own = m.params.iter().position(|&p| p == (l.reg, c)).is_some_and(|_| {
                l.start == 0 || m.debug.locals.iter().any(|p| p.reg == l.reg && p.start == 0 && p.name == l.name)
            });
            let key = if own || !self.slots.contains_key(&(l.reg | REUSED, c)) { l.reg } else { l.reg | REUSED };
            let Some(&slot) = self.slots.get(&(key, c)) else { continue };
            let whole = m.params.iter().zip(&m.clean_param).any(|(&(r, pc), &clean)| clean && r == l.reg && pc == c);
            let span = if l.start == 0 && whole {
                vec![(0, code.len() as u32)]
            } else if l.start == 0 {
                self.span(0, l.end).first().map(|&(_, e)| vec![(0, e)]).unwrap_or_default()
            } else {
                self.span(l.start, l.end)
            };
            for (start, end) in span {
                if start >= end || !seen.insert((start, end, name.to_vec(), slot)) {
                    continue;
                }
                let ni = self.sh.pool.utf8(name);
                let di = self.sh.pool.utf8(&desc);
                lvt.u16(start as u16).u16((end - start) as u16).u16(ni).u16(di).u16(slot);
                n += 1;
                if l.sig != NO_INDEX {
                    let si = self.sh.pool.utf8(m.dex.string(l.sig)?);
                    lvtt.u16(start as u16).u16((end - start) as u16).u16(ni).u16(si).u16(slot);
                    nt += 1;
                }
            }
        }
        for (name, count, body) in [(&b"LocalVariableTable"[..], n, lvt), (b"LocalVariableTypeTable", nt, lvtt)] {
            if count > 0 {
                let mut b = Body::default();
                b.u16(count);
                b.0.extend_from_slice(&body.0);
                attrs.push((self.sh.pool.utf8(name), b.0));
            }
        }
        let mut out = Body::default();
        out.u16(self.asm.max.max(0) as u16).u16(self.next_slot).u32(code.len() as u32);
        out.0.extend_from_slice(code);
        out.u16(table.len() as u16);
        out.0.extend_from_slice(&entries.0);
        out.u16(attrs.len() as u16);
        for (name, body) in attrs {
            out.u16(name).u32(body.len() as u32);
            out.0.extend_from_slice(&body);
        }
        Ok(out.0)
    }
}

fn falls(o: &Op) -> bool {
    !matches!(o.insn.op, 0x0e..=0x11 | 0x27 | 0x28..=0x2a)
}

/// Sorted intervals with touching ones joined.
fn merge(mut iv: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    iv.retain(|&(s, e)| s < e);
    iv.sort_unstable();
    let mut out: Vec<(u32, u32)> = Vec::new();
    for (s, e) in iv {
        match out.last_mut() {
            Some(last) if s <= last.1 => last.1 = last.1.max(e),
            _ => out.push((s, e)),
        }
    }
    out
}

/// Reachable blocks in emission order: address order, each fall-through
/// successor right after its block, and a block whose only predecessor
/// jumps to it unconditionally right after that predecessor.
fn layout(m: &Method) -> Vec<u32> {
    let n = m.blocks.len();
    let mut preds = vec![0u32; n];
    let mut handler = vec![false; n];
    for b in m.blocks.iter().filter(|b| b.reachable) {
        for &s in &b.succs {
            preds[s as usize] += 1;
        }
        for h in &b.handlers {
            handler[h.1 as usize] = true;
        }
    }
    let mut placed = vec![false; n];
    let mut order = Vec::new();
    for first in 0..n {
        if placed[first] || !m.blocks[first].reachable {
            continue;
        }
        let mut b = first;
        loop {
            placed[b] = true;
            order.push(b as u32);
            let blk = &m.blocks[b];
            let last = &m.ops[blk.end - 1];
            if falls(last) {
                b += 1;
                if placed[b] {
                    break;
                }
                continue;
            }
            let t = match last.insn.op {
                0x28..=0x2a => blk.succs[0] as usize,
                _ => break,
            };
            if placed[t] || preds[t] != 1 || handler[t] {
                break;
            }
            b = t;
        }
    }
    order
}

fn kind_index(c: Mask) -> u8 {
    match c {
        J => 1,
        F => 2,
        D => 3,
        _ => 0,
    }
}
