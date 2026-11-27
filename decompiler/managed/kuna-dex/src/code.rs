//! Method analysis for translation: Dalvik instructions become [`Op`]s with
//! typed register uses and defs; basic blocks and exception edges; liveness;
//! value numbering with phis at joins (a use names the single definition or
//! phi reaching it); webs of values joined through phis; and a category
//! (int-like, float, long, double, reference) for every web, inferred from
//! the typed opcodes, member and method types, array element types and the
//! method's own signature. A constant that reaches a use alone is typed by
//! that use, so one register can carry one literal to differently-typed uses.

use std::collections::{BTreeMap, BTreeSet};

use crate::debug::DebugInfo;
use crate::dex::{CodeItem, Dex, Proto};
use crate::insn::{decode, Insn};
use crate::{err, Result};

pub type Mask = u8;
pub const I: Mask = 1;
pub const F: Mask = 2;
pub const J: Mask = 4;
pub const D: Mask = 8;
pub const L: Mask = 16;
pub const N: Mask = I | F;
pub const W: Mask = J | D;
pub const UNDEF: u32 = u32::MAX;

pub fn is_wide(m: Mask) -> bool {
    m & W != 0 && m & !W == 0
}

pub fn size(m: Mask) -> i32 {
    if is_wide(m) {
        2
    } else {
        1
    }
}

/// The category of a field, parameter or return descriptor (0 for `V`).
pub fn cat(desc: &[u8]) -> Mask {
    match desc.first() {
        Some(b'Z' | b'B' | b'S' | b'C' | b'I') => I,
        Some(b'F') => F,
        Some(b'J') => J,
        Some(b'D') => D,
        Some(b'L' | b'[') => L,
        _ => 0,
    }
}

/// The JVM kind index of a resolved category (`iload` + kind).
pub fn kind(m: Mask) -> u8 {
    match m {
        J => 1,
        F => 2,
        D => 3,
        L => 4,
        _ => 0,
    }
}

#[derive(Clone, Debug)]
pub struct Use {
    pub reg: u32,
    pub mask: Mask,
    pub val: u32,
}

#[derive(Clone, Debug)]
pub struct Def {
    pub reg: u32,
    pub mask: Mask,
    pub id: u32,
}

/// How an op's operand types relate beyond their own masks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rel {
    None,
    /// The def and use 0 have one category (moves, `check-cast`, fills).
    Same,
    /// Uses 0 and 1 have one category (`if-eq`, `if-ne`).
    SameUses,
    /// An element read into the def from the array at use 0.
    ArrayGet,
    /// An element (use 2) written into the array at use 0.
    ArrayPut,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Plain,
    /// `new-instance` and its `<init>` in one op: `insn` is the invoke, `ty` the class.
    NewObject { ty: u32 },
    /// A `new-instance` folded into its constructor call.
    Absorbed,
}

#[derive(Clone, Debug)]
pub struct Op {
    pub insn: Insn,
    pub end: u32,
    pub kind: Kind,
    pub uses: Vec<Use>,
    pub def: Option<Def>,
    pub rel: Rel,
    pub throws: bool,
    /// An invoke's result is used (`move-result` followed).
    pub result: bool,
    pub block: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub start: usize,
    pub end: usize,
    pub succs: Vec<u32>,
    /// Handlers of the try item covering the block: `(catch type, block)`.
    pub handlers: Vec<(Option<u32>, u32)>,
    pub try_item: Option<usize>,
    pub reachable: bool,
    pub falls_off: bool,
}

pub struct Method<'d, 'a> {
    pub dex: &'d Dex<'a>,
    pub code: &'d CodeItem<'a>,
    pub debug: &'d DebugInfo,
    pub is_static: bool,
    pub proto: Proto,
    pub ops: Vec<Op>,
    pub blocks: Vec<Block>,
    /// `(register, category)` of each parameter slot, `this` first.
    pub params: Vec<(u32, Mask)>,
    /// Union-find parent over value ids (defs, then phis).
    parent: Vec<u32>,
    pub value_wide: Vec<bool>,
    /// Number of def values; ids at or above are phis.
    pub defs: u32,
    pub phi_block: Vec<u32>,
    pub phi_reg: Vec<u32>,
    /// Per value: its category (0 for a constant inlined at its uses).
    pub cat: Vec<Mask>,
    /// Per value: its type variable (webs joined by moves and compares).
    pub tvar: Vec<u32>,
    /// Per web root: number of uses, and whether a phi is in it.
    pub web_uses: BTreeMap<u32, (u32, bool)>,
    /// Per value: the op defining it (`usize::MAX` for a parameter).
    pub def_op: Vec<usize>,
    /// Per value: the use count where the use names that exact value.
    pub direct_uses: Vec<u32>,
    pub ref_type: BTreeMap<u32, Vec<u8>>,
    /// The declaring class's descriptor, for `this` in the local table.
    pub this_desc: Vec<u8>,
    /// The declared parameters' `MethodParameters` names: a parameter that
    /// debug info does not name (a release build) gets a local-variable entry
    /// with it over the whole method, as javac's `-g` writes one.
    pub param_names: Vec<u32>,
    /// `(block, register, predecessor)` to the constant or `move` def arriving on that edge.
    pub edge_const: BTreeMap<(u32, u32, u32), u32>,
    /// Registers live into some exception handler (a bit per register).
    pub handler_live: Vec<u64>,
    /// Per parameter: nothing writes its local (no def in its web, and none
    /// in its register that debug info names as the parameter).
    pub clean_param: Vec<bool>,
    /// Webs of values in a parameter's register that are another variable
    /// (not the parameter updated in place, nor named as it by debug info):
    /// they get a local of their own.
    pub reused: BTreeSet<u32>,
}

/// An instruction's register uses (push order) and def, with their masks.
type Shape = (Vec<(u32, Mask)>, Option<(u32, Mask)>, Rel);

fn is_invoke(op: u8) -> bool {
    matches!(op, 0x6e..=0x72 | 0x74..=0x78 | 0xfa..=0xfd)
}

pub fn throws(op: u8) -> bool {
    matches!(op,
        0x1a..=0x1f | 0x20..=0x27 | 0x44..=0x6d | 0x6e..=0x72 | 0x74..=0x78
        | 0x93 | 0x94 | 0x9e | 0x9f | 0xb3 | 0xb4 | 0xbe | 0xbf
        | 0xd3 | 0xd4 | 0xdb | 0xdc | 0xfa..=0xff)
}

/// Whether an op ends its block: branches, returns, throws, and the monitor
/// ops (a `synchronized` region starts right after `monitor-enter` and ends
/// right after `monitor-exit`).
fn ends_block(op: u8) -> bool {
    matches!(op, 0x0e..=0x11 | 0x1d | 0x1e | 0x27 | 0x28..=0x2c | 0x32..=0x3d)
}

fn unop_masks(op: u8) -> (Mask, Mask) {
    match op {
        0x7b | 0x7c | 0x8d..=0x8f => (I, I),
        0x7d | 0x7e => (J, J),
        0x7f => (F, F),
        0x80 => (D, D),
        0x81 => (I, J),
        0x82 => (I, F),
        0x83 => (I, D),
        0x84 => (J, I),
        0x85 => (J, F),
        0x86 => (J, D),
        0x87 => (F, I),
        0x88 => (F, J),
        0x89 => (F, D),
        0x8a => (D, I),
        0x8b => (D, J),
        _ => (D, F),
    }
}

/// `(left, right, result)` masks of a binary op by its index in the
/// add..ushr block (0x90-based).
pub fn binop_masks(k: u8) -> (Mask, Mask, Mask) {
    match k {
        0..=10 => (I, I, I),
        11..=18 => (J, J, J),
        19..=21 => (J, I, J),
        22..=26 => (F, F, F),
        _ => (D, D, D),
    }
}

impl<'d, 'a> Method<'d, 'a> {
    /// `undo` writes d8's nest accessors back as the accesses they make.
    pub fn new(
        dex: &'d Dex<'a>,
        code: &'d CodeItem<'a>,
        debug: &'d DebugInfo,
        is_static: bool,
        proto: Proto,
        undo: bool,
    ) -> Result<Method<'d, 'a>> {
        let mut m = Method {
            dex,
            code,
            debug,
            is_static,
            proto,
            ops: Vec::new(),
            blocks: Vec::new(),
            params: Vec::new(),
            parent: Vec::new(),
            value_wide: Vec::new(),
            defs: 0,
            phi_block: Vec::new(),
            phi_reg: Vec::new(),
            cat: Vec::new(),
            tvar: Vec::new(),
            web_uses: BTreeMap::new(),
            def_op: Vec::new(),
            direct_uses: Vec::new(),
            ref_type: BTreeMap::new(),
            this_desc: Vec::new(),
            param_names: Vec::new(),
            edge_const: BTreeMap::new(),
            handler_live: Vec::new(),
            clean_param: Vec::new(),
            reused: BTreeSet::new(),
        };
        m.build_ops(undo)?;
        m.build_blocks()?;
        m.fuse_constructors()?;
        m.number_values()?;
        m.infer_types()?;
        m.classify_params();
        m.forward_moves();
        Ok(m)
    }

    fn desc(&self, ty: u32) -> Result<&'a [u8]> {
        self.dex.type_desc(ty)
    }

    fn proto_masks(&self, proto: u32) -> Result<(Vec<Mask>, Mask)> {
        let p = self.dex.proto(proto)?;
        let params = p
            .params
            .iter()
            .map(|&t| Ok(cat(self.desc(t)?)))
            .collect::<Result<Vec<_>>>()?;
        Ok((params, cat(self.desc(p.ret)?)))
    }

    /// The method's parameters: `this` (unless static), then each declared one.
    fn param_layout(&mut self) -> Result<()> {
        let regs = u32::from(self.code.registers);
        let mut reg = regs - u32::from(self.code.ins);
        if !self.is_static {
            self.params.push((reg, L));
            reg += 1;
        }
        for &t in &self.proto.params.clone() {
            let m = cat(self.desc(t)?);
            self.params.push((reg, m));
            reg += size(m) as u32;
        }
        if reg != regs {
            return Err(err(format!(
                "ins_size {} does not match the method's parameters",
                self.code.ins
            )));
        }
        Ok(())
    }

    fn shape(&self, x: &Insn) -> Result<Shape> {
        let (a, b, c) = (x.a, x.b, x.c);
        let none = Rel::None;
        Ok(match x.op {
            0x00 | 0x0e | 0x28..=0x2a => (vec![], None, none),
            0x01..=0x03 => (vec![(b, N)], Some((a, N)), Rel::Same),
            0x04..=0x06 => (vec![(b, W)], Some((a, W)), Rel::Same),
            0x07..=0x09 => (vec![(b, L)], Some((a, L)), Rel::Same),
            0x0a..=0x0c => return Err(err(format!("move-result without an invoke at {:#x}", x.addr))),
            0x0d => (vec![], Some((a, L)), none),
            0x0f => (vec![(a, cat(self.desc(self.proto.ret)?) & N)], None, none),
            0x10 => (vec![(a, cat(self.desc(self.proto.ret)?) & W)], None, none),
            0x11 => (vec![(a, L)], None, none),
            0x12..=0x15 => (vec![], Some((a, if x.lit == 0 { N | L } else { N })), none),
            0x16..=0x19 => (vec![], Some((a, W)), none),
            0x1a..=0x1c | 0x22 | 0xfe | 0xff => (vec![], Some((a, L)), none),
            0x1d | 0x1e | 0x27 => (vec![(a, L)], None, none),
            0x1f => (vec![(a, L)], Some((a, L)), none),
            0x20 => (vec![(b, L)], Some((a, I)), none),
            0x21 => (vec![(b, L)], Some((a, I)), none),
            0x23 => (vec![(b, I)], Some((a, L)), none),
            0x24 | 0x25 => {
                let desc = self.desc(x.idx)?;
                let m = cat(&desc[1..]);
                (x.args().into_iter().map(|r| (r, m)).collect(), None, none)
            }
            0x26 => (vec![(a, L)], Some((a, L)), Rel::Same),
            0x2b | 0x2c => (vec![(a, I)], None, none),
            0x2d | 0x2e => (vec![(b, F), (c, F)], Some((a, I)), none),
            0x2f | 0x30 => (vec![(b, D), (c, D)], Some((a, I)), none),
            0x31 => (vec![(b, J), (c, J)], Some((a, I)), none),
            0x32 | 0x33 => (vec![(a, I | L), (b, I | L)], None, Rel::SameUses),
            0x34..=0x37 => (vec![(a, I), (b, I)], None, none),
            0x38 | 0x39 => (vec![(a, I | L)], None, none),
            0x3a..=0x3d => (vec![(a, I)], None, none),
            0x44..=0x4a => {
                let m = match x.op {
                    0x44 => N,
                    0x45 => W,
                    0x46 => L,
                    _ => I,
                };
                (vec![(b, L), (c, I)], Some((a, m)), Rel::ArrayGet)
            }
            0x4b..=0x51 => {
                let m = match x.op {
                    0x4b => N,
                    0x4c => W,
                    0x4d => L,
                    _ => I,
                };
                (vec![(b, L), (c, I), (a, m)], None, Rel::ArrayPut)
            }
            0x52..=0x58 => {
                let f = self.dex.field(x.idx)?;
                (vec![(b, L)], Some((a, cat(self.desc(f.ty)?))), none)
            }
            0x59..=0x5f => {
                let f = self.dex.field(x.idx)?;
                (vec![(b, L), (a, cat(self.desc(f.ty)?))], None, none)
            }
            0x60..=0x66 => {
                let f = self.dex.field(x.idx)?;
                (vec![], Some((a, cat(self.desc(f.ty)?))), none)
            }
            0x67..=0x6d => {
                let f = self.dex.field(x.idx)?;
                (vec![(a, cat(self.desc(f.ty)?))], None, none)
            }
            0x6e..=0x72 | 0x74..=0x78 | 0xfa..=0xfd => {
                let (params, _) = match x.op {
                    0xfa | 0xfb => self.proto_masks(x.proto)?,
                    0xfc | 0xfd => (self.call_site_masks(x.idx)?.0, 0),
                    _ => self.proto_masks(self.dex.method(x.idx)?.proto)?,
                };
                let receiver = !matches!(x.op, 0x71 | 0x77 | 0xfc | 0xfd);
                let mut masks = Vec::new();
                if receiver {
                    masks.push(L);
                }
                masks.extend(params);
                let args = x.args();
                let mut uses = Vec::new();
                let mut k = 0usize;
                for m in masks {
                    let r = *args
                        .get(k)
                        .ok_or_else(|| err(format!("too few arguments at {:#x}", x.addr)))?;
                    if is_wide(m) && args.get(k + 1) != Some(&(r + 1)) {
                        return Err(err(format!("wide argument not in a register pair at {:#x}", x.addr)));
                    }
                    uses.push((r, m));
                    k += size(m) as usize;
                }
                if k != args.len() {
                    return Err(err(format!("argument count mismatch at {:#x}", x.addr)));
                }
                (uses, None, none)
            }
            0x7b..=0x8f => {
                let (u, d) = unop_masks(x.op);
                (vec![(b, u)], Some((a, d)), none)
            }
            0x90..=0xaf => {
                let (l, r, d) = binop_masks(x.op - 0x90);
                (vec![(b, l), (c, r)], Some((a, d)), none)
            }
            0xb0..=0xcf => {
                let (l, r, d) = binop_masks(x.op - 0xb0);
                (vec![(a, l), (b, r)], Some((a, d)), none)
            }
            0xd0..=0xe2 => (vec![(b, I)], Some((a, I)), none),
            _ => return Err(err(format!("unsupported opcode {:#04x} at {:#x}", x.op, x.addr))),
        })
    }

    /// Parameter and return categories of an `invoke-custom` call site.
    pub fn call_site_masks(&self, idx: u32) -> Result<(Vec<Mask>, Mask)> {
        let site = self.dex.call_site(idx)?;
        match site.get(2) {
            Some(crate::dex::Value::MethodType(p)) => self.proto_masks(*p),
            _ => Err(err(format!("call site {idx} has no method type"))),
        }
    }

    fn build_ops(&mut self, undo: bool) -> Result<()> {
        self.param_layout()?;
        let mut insns = decode(self.code.insns)?;
        if undo {
            crate::nest::rewrite(self.dex, &mut insns);
        }
        let regs = u32::from(self.code.registers);
        let mut k = 0;
        while k < insns.len() {
            let x = &insns[k];
            let (uses, def, rel) = self.shape(x)?;
            for &(r, m) in uses.iter().chain(def.iter()) {
                if r + size(m) as u32 > regs {
                    return Err(err(format!("register v{r} out of range at {:#x}", x.addr)));
                }
            }
            let mut op = Op {
                insn: x.clone(),
                end: x.next(),
                kind: Kind::Plain,
                uses: uses
                    .into_iter()
                    .map(|(reg, mask)| Use {
                        reg,
                        mask,
                        val: UNDEF,
                    })
                    .collect(),
                def: def.map(|(reg, mask)| Def { reg, mask, id: 0 }),
                rel,
                throws: throws(x.op),
                result: false,
                block: 0,
            };
            if (is_invoke(x.op) || matches!(x.op, 0x24 | 0x25)) && k + 1 < insns.len() {
                let next = &insns[k + 1];
                if matches!(next.op, 0x0a..=0x0c) {
                    let m = if matches!(x.op, 0x24 | 0x25) {
                        L
                    } else {
                        self.result_mask(x)?
                    };
                    let want = match next.op {
                        0x0a => N,
                        0x0b => W,
                        _ => L,
                    };
                    if m & want == 0 {
                        return Err(err(format!("move-result kind does not match the call at {:#x}", next.addr)));
                    }
                    if next.a + size(m) as u32 > regs {
                        return Err(err(format!("register v{} out of range", next.a)));
                    }
                    op.def = Some(Def {
                        reg: next.a,
                        mask: m,
                        id: 0,
                    });
                    op.result = true;
                    op.end = next.next();
                    k += 1;
                }
            }
            self.ops.push(op);
            k += 1;
        }
        Ok(())
    }

    pub fn result_mask(&self, x: &Insn) -> Result<Mask> {
        Ok(match x.op {
            0xfa | 0xfb => self.proto_masks(x.proto)?.1,
            0xfc | 0xfd => self.call_site_masks(x.idx)?.1,
            _ => self.proto_masks(self.dex.method(x.idx)?.proto)?.1,
        })
    }

    /// Pairs each `new-instance` with the `<init>` call that every path from
    /// it reaches first, within a few blocks and without a loop back, on the
    /// register holding the new reference: its own, or a copy d8 made with
    /// `move-object` to line up a range call. Nothing else may read the
    /// reference before the call, and at most one of its registers is read
    /// after it; the pair becomes one op defining that register at the call,
    /// and the copies vanish.
    fn fuse_constructors(&mut self) -> Result<()> {
        let touches = |r: u32, m: Mask, reg: u32| r == reg || (is_wide(m) && r + 1 == reg);
        let mut live: Option<Vec<Vec<u64>>> = None;
        for k in 0..self.ops.len() {
            if self.ops[k].insn.op != 0x22 || !self.blocks[self.ops[k].block as usize].reachable {
                continue;
            }
            let (ty, home) = (self.ops[k].insn.idx, self.ops[k].block);
            let mut init: Option<(usize, u32)> = None;
            let mut seen: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
            let mut moves: Vec<usize> = Vec::new();
            let mut work = vec![(k + 1, vec![self.ops[k].insn.a])];
            let mut ok = true;
            while let Some((start, mut aliases)) = work.pop() {
                if start >= self.ops.len() || seen.len() > 64 {
                    ok = false;
                    break;
                }
                let b = self.ops[start].block as usize;
                let mut reached = false;
                for j in start..self.blocks[b].end {
                    let o = &self.ops[j];
                    let hits = |r: u32, m: Mask| aliases.iter().any(|&a| touches(r, m, a));
                    let is_init = matches!(o.insn.op, 0x70 | 0x76)
                        && o.uses.first().is_some_and(|u| aliases.contains(&u.reg))
                        && self.dex.method(o.insn.idx)?.class == ty
                        && self.dex.string(self.dex.method(o.insn.idx)?.name)? == b"<init>";
                    if is_init && !o.uses[1..].iter().any(|u| hits(u.reg, u.mask)) {
                        let mut holder = o.uses[0].reg;
                        if aliases.len() > 1 {
                            let live = live.get_or_insert_with(|| self.liveness());
                            let after: Vec<u32> = aliases.iter().copied().filter(|&a| !self.dead_after(j, a, live)).collect();
                            ok &= after.len() <= 1;
                            holder = after.first().copied().unwrap_or(holder);
                        }
                        ok &= init.is_none_or(|i| i == (j, holder));
                        init = Some((j, holder));
                        reached = true;
                        break;
                    }
                    if matches!(o.insn.op, 0x07..=0x09) && o.uses.first().is_some_and(|u| aliases.contains(&u.reg)) {
                        let d = o.def.as_ref().expect("move").reg;
                        aliases.retain(|&a| a != d);
                        aliases.push(d);
                        aliases.sort_unstable();
                        if !moves.contains(&j) {
                            moves.push(j);
                        }
                        continue;
                    }
                    if o.uses.iter().any(|u| hits(u.reg, u.mask)) {
                        ok = false;
                        break;
                    }
                    if let Some(d) = &o.def {
                        aliases.retain(|&a| !touches(d.reg, d.mask, a));
                    }
                    if aliases.is_empty() {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
                if reached {
                    continue;
                }
                let succs = &self.blocks[b].succs;
                if succs.is_empty() || succs.contains(&home) {
                    ok = false;
                    break;
                }
                for &s in succs {
                    match seen.get(&s) {
                        Some(prev) if *prev != aliases => ok = false,
                        Some(_) => {}
                        None => {
                            seen.insert(s, aliases.clone());
                            work.push((self.blocks[s as usize].start, aliases.clone()));
                        }
                    }
                }
            }
            if let (true, Some((j, reg))) = (ok, init) {
                self.ops[k].kind = Kind::Absorbed;
                self.ops[k].def = None;
                for &m in &moves {
                    let o = &mut self.ops[m];
                    o.kind = Kind::Absorbed;
                    o.def = None;
                    o.uses.clear();
                    o.rel = Rel::None;
                }
                let o = &mut self.ops[j];
                o.kind = Kind::NewObject { ty };
                o.uses.remove(0);
                o.def = Some(Def { reg, mask: L, id: 0 });
            }
        }
        Ok(())
    }

    /// Whether register `r` is written before it is read after op `j`.
    fn dead_after(&self, j: usize, r: u32, live: &[Vec<u64>]) -> bool {
        let b = self.ops[j].block as usize;
        for o in &self.ops[j + 1..self.blocks[b].end] {
            if o.uses.iter().any(|u| u.reg == r || (is_wide(u.mask) && u.reg + 1 == r)) {
                return false;
            }
            if o.def.as_ref().is_some_and(|d| d.reg == r || (is_wide(d.mask) && d.reg + 1 == r)) {
                return true;
            }
        }
        let blk = &self.blocks[b];
        let bit = |s: u32| live[s as usize].get(r as usize / 64).is_some_and(|w| w >> (r % 64) & 1 != 0);
        !blk.succs.iter().chain(blk.handlers.iter().map(|h| &h.1)).any(|&s| bit(s))
    }

    fn op_at(&self, addr: u32) -> Option<usize> {
        self.ops.binary_search_by_key(&addr, |o| o.insn.addr).ok()
    }

    /// The first op starting at or after `addr`.
    fn op_from(&self, addr: u32) -> usize {
        self.ops.partition_point(|o| o.insn.addr < addr)
    }

    pub fn targets(&self, o: &Op) -> Result<Vec<u32>> {
        let x = &o.insn;
        let at = |t: i64| -> Result<u32> {
            u32::try_from(t)
                .ok()
                .filter(|&t| self.op_at(t).is_some())
                .ok_or_else(|| err(format!("branch at {:#x} to {t:#x} is not an instruction", x.addr)))
        };
        Ok(match x.op {
            0x28..=0x2a | 0x32..=0x3d => vec![at(x.target())?],
            0x2b => crate::insn::packed_switch(self.code.insns, x.target())?
                .1
                .iter()
                .map(|&t| at(i64::from(x.addr) + i64::from(t)))
                .collect::<Result<_>>()?,
            0x2c => crate::insn::sparse_switch(self.code.insns, x.target())?
                .iter()
                .map(|&(_, t)| at(i64::from(x.addr) + i64::from(t)))
                .collect::<Result<_>>()?,
            _ => vec![],
        })
    }

    fn build_blocks(&mut self) -> Result<()> {
        if self.ops.is_empty() {
            return Err(err("method has no instructions".to_string()));
        }
        let mut leader = vec![false; self.ops.len() + 1];
        leader[0] = true;
        leader[self.ops.len()] = true;
        for k in 0..self.ops.len() {
            let o = &self.ops[k];
            if ends_block(o.insn.op) {
                leader[k + 1] = true;
            }
            for t in self.targets(o)? {
                leader[self.op_at(t).expect("checked")] = true;
            }
        }
        for t in &self.code.tries {
            leader[self.op_from(t.start)] = true;
            leader[self.op_from(t.start + t.count)] = true;
            for &(_, h) in &t.handlers {
                let i = self
                    .op_at(h)
                    .ok_or_else(|| err(format!("handler {h:#x} is not an instruction")))?;
                leader[i] = true;
            }
        }
        let mut block_of = vec![0u32; self.ops.len() + 1];
        let mut start = 0;
        for (k, _) in leader.iter().enumerate().skip(1).filter(|p| *p.1) {
            self.blocks.push(Block {
                start,
                end: k,
                ..Block::default()
            });
            start = k;
        }
        for (b, blk) in self.blocks.iter().enumerate() {
            block_of[blk.start..blk.end].fill(b as u32);
        }
        for b in 0..self.blocks.len() {
            let (s, e) = (self.blocks[b].start, self.blocks[b].end);
            for o in &mut self.ops[s..e] {
                o.block = b as u32;
            }
            let last = &self.ops[e - 1];
            let mut succs: Vec<u32> = self
                .targets(last)?
                .iter()
                .map(|&t| block_of[self.op_at(t).expect("checked")])
                .collect();
            let falls = !matches!(last.insn.op, 0x0e..=0x11 | 0x27 | 0x28..=0x2a);
            if falls && e < self.ops.len() {
                succs.push(block_of[e]);
            }
            let falls_off = falls && e >= self.ops.len();
            let addr = self.ops[s].insn.addr;
            let mut handlers = Vec::new();
            let mut try_item = None;
            for (ti, t) in self.code.tries.iter().enumerate() {
                if addr >= t.start && addr < t.start + t.count {
                    try_item = Some(ti);
                    for &(ty, h) in &t.handlers {
                        handlers.push((ty, block_of[self.op_at(h).expect("checked")]));
                    }
                    break;
                }
            }
            let blk = &mut self.blocks[b];
            blk.succs = succs;
            blk.handlers = handlers;
            blk.try_item = try_item;
            blk.falls_off = falls_off;
        }
        let mut stack = vec![0u32];
        self.blocks[0].reachable = true;
        while let Some(b) = stack.pop() {
            let blk = &self.blocks[b as usize];
            if blk.falls_off {
                return Err(err("code falls off its end".to_string()));
            }
            let throws = self.ops[blk.start..blk.end].iter().any(|o| o.throws);
            let mut next: Vec<u32> = blk.succs.clone();
            if throws {
                next.extend(blk.handlers.iter().map(|h| h.1));
            }
            for s in next {
                if !self.blocks[s as usize].reachable {
                    self.blocks[s as usize].reachable = true;
                    stack.push(s);
                }
            }
        }
        Ok(())
    }

    fn handlers_of(&self, b: usize) -> impl Iterator<Item = u32> + '_ {
        self.blocks[b].handlers.iter().map(|h| h.1)
    }

    fn liveness(&self) -> Vec<Vec<u64>> {
        let regs = usize::from(self.code.registers);
        let words = regs.div_ceil(64).max(1);
        let n = self.blocks.len();
        let mut live_in = vec![vec![0u64; words]; n];
        let set = |v: &mut [u64], r: u32, on: bool| {
            let (w, bit) = ((r / 64) as usize, 1u64 << (r % 64));
            if on {
                v[w] |= bit;
            } else {
                v[w] &= !bit;
            }
        };
        let mut changed = true;
        while changed {
            changed = false;
            for b in (0..n).rev() {
                if !self.blocks[b].reachable {
                    continue;
                }
                let mut live = vec![0u64; words];
                for &s in &self.blocks[b].succs {
                    for (x, y) in live.iter_mut().zip(&live_in[s as usize]) {
                        *x |= y;
                    }
                }
                let blk = &self.blocks[b];
                for o in self.ops[blk.start..blk.end].iter().rev() {
                    if let Some(d) = &o.def {
                        set(&mut live, d.reg, false);
                        if is_wide(d.mask) {
                            set(&mut live, d.reg + 1, false);
                        }
                    }
                    if o.throws {
                        for h in self.handlers_of(b) {
                            for (x, y) in live.iter_mut().zip(&live_in[h as usize]) {
                                *x |= y;
                            }
                        }
                    }
                    for u in &o.uses {
                        set(&mut live, u.reg, true);
                        if is_wide(u.mask) {
                            set(&mut live, u.reg + 1, true);
                        }
                    }
                }
                if live != live_in[b] {
                    live_in[b] = live;
                    changed = true;
                }
            }
        }
        live_in
    }

    pub fn find(&mut self, mut v: u32) -> u32 {
        while self.parent[v as usize] != v {
            let p = self.parent[v as usize];
            self.parent[v as usize] = self.parent[p as usize];
            v = p;
        }
        v
    }

    pub fn root(&self, mut v: u32) -> u32 {
        while self.parent[v as usize] != v {
            v = self.parent[v as usize];
        }
        v
    }

    fn union(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            let (lo, hi) = if ra < rb { (ra, rb) } else { (rb, ra) };
            self.parent[hi as usize] = lo;
        }
    }

    fn new_value(&mut self, wide: bool, op: usize) -> u32 {
        let id = self.parent.len() as u32;
        self.parent.push(id);
        self.value_wide.push(wide);
        self.def_op.push(op);
        id
    }

    fn number_values(&mut self) -> Result<()> {
        let regs = usize::from(self.code.registers);
        let n = self.blocks.len();
        if regs.saturating_mul(n) > 8 << 20 {
            return Err(err(format!("method too large to analyse ({regs} registers, {n} blocks)")));
        }
        let mut entry0 = vec![UNDEF; regs];
        for p in 0..self.params.len() {
            let (r, m) = self.params[p];
            let id = self.new_value(is_wide(m), usize::MAX);
            entry0[r as usize] = id;
        }
        for k in 0..self.ops.len() {
            if self.ops[k].def.is_some() {
                let wide = is_wide(self.ops[k].def.as_ref().expect("def").mask);
                let id = self.new_value(wide, k);
                self.ops[k].def.as_mut().expect("def").id = id;
            }
        }
        self.defs = self.parent.len() as u32;
        let live = self.liveness();
        self.handler_live = vec![0; live.first().map_or(1, Vec::len)];
        for blk in self.blocks.iter().filter(|b| b.reachable) {
            for h in &blk.handlers {
                for (x, y) in self.handler_live.iter_mut().zip(&live[h.1 as usize]) {
                    *x |= y;
                }
            }
        }
        let mut rpo = vec![u32::MAX; n];
        let mut post = Vec::new();
        let mut seen = vec![false; n];
        let mut stack: Vec<(u32, usize)> = vec![(0, 0)];
        seen[0] = true;
        while let Some(&mut (b, ref mut i)) = stack.last_mut() {
            let blk = &self.blocks[b as usize];
            let next = blk.succs.iter().chain(blk.handlers.iter().map(|h| &h.1)).nth(*i).copied();
            *i += 1;
            match next {
                Some(s) if !seen[s as usize] => {
                    seen[s as usize] = true;
                    stack.push((s, 0));
                }
                Some(_) => {}
                None => {
                    post.push(b);
                    stack.pop();
                }
            }
        }
        for (i, &b) in post.iter().rev().enumerate() {
            rpo[b as usize] = i as u32;
        }
        let mut flow = Flow {
            entry: vec![None; n],
            phis: BTreeMap::new(),
            from: BTreeMap::new(),
            live,
            rpo,
            work: BTreeSet::new(),
        };
        flow.entry[0] = Some(entry0);
        flow.work.insert((0, 0));
        let mut final_pass = false;
        loop {
            while let Some((_, b)) = flow.work.pop_first() {
                let mut state = flow.entry[b as usize].clone().expect("entered");
                let blk = self.blocks[b as usize].clone();
                for k in blk.start..blk.end {
                    if self.ops[k].throws && !final_pass {
                        for &(_, h) in &blk.handlers {
                            flow.merge(self, b, h, &state);
                        }
                    }
                    if final_pass {
                        for ui in 0..self.ops[k].uses.len() {
                            let r = self.ops[k].uses[ui].reg as usize;
                            let v = state[r];
                            if v == UNDEF {
                                return Err(err(format!(
                                    "register v{r} is read at {:#x} before it is written",
                                    self.ops[k].insn.addr
                                )));
                            }
                            self.ops[k].uses[ui].val = v;
                        }
                    }
                    if let Some(d) = &self.ops[k].def {
                        let r = d.reg as usize;
                        if r > 0 && state[r - 1] != UNDEF && self.value_wide[state[r - 1] as usize] {
                            state[r - 1] = UNDEF;
                        }
                        state[r] = d.id;
                        if is_wide(d.mask) {
                            state[r + 1] = UNDEF;
                        }
                    }
                }
                if !final_pass {
                    for &s in &blk.succs {
                        flow.merge(self, b, s, &state);
                    }
                }
            }
            if final_pass {
                break;
            }
            final_pass = true;
            flow.work = (0..n as u32)
                .filter(|&b| flow.entry[b as usize].is_some())
                .map(|b| (flow.rpo[b as usize], b))
                .collect();
        }
        let values = self.parent.len();
        self.direct_uses = vec![0; values];
        for k in 0..self.ops.len() {
            for u in 0..self.ops[k].uses.len() {
                let v = self.ops[k].uses[u].val;
                if v == UNDEF {
                    continue;
                }
                self.direct_uses[v as usize] += 1;
                let r = self.find(v);
                let e = self.web_uses.entry(r).or_insert((0, false));
                e.0 += 1;
            }
        }
        for v in self.defs..values as u32 {
            let r = self.find(v);
            self.web_uses.entry(r).or_insert((0, false)).1 = true;
        }
        Ok(())
    }

    /// Which values in a parameter's register are the parameter (its web,
    /// an update of it in place of its own type: arithmetic, or another op
    /// but a cast whose value is read more than once; or a value debug info
    /// names as it) and
    /// which are another variable d8 put there, from the instructions as
    /// d8 wrote them (before copies are forwarded).
    fn classify_params(&mut self) {
        let name_at = |m: &Self, r: u32, pc: u32| {
            m.debug.locals.iter().find(|l| l.reg == r && l.start <= pc && pc < l.end.max(l.start + 1)).map(|l| l.name)
        };
        let (mut named, mut reused, mut clean_param) = (BTreeSet::new(), BTreeSet::new(), Vec::new());
        for p in 0..self.params.len() {
            let (r, root) = (self.params[p].0, self.root(p as u32));
            let name = name_at(self, r, 0);
            let mut clean = true;
            let mut own = BTreeSet::from([root]);
            let ty = self.ref_desc(p as u32).map(<[u8]>::to_vec);
            for o in &self.ops {
                let Some(d) = o.def.as_ref().filter(|d| d.reg == r) else { continue };
                let web = self.root(d.id);
                let same = self.cat[d.id as usize] == self.cat[p] && self.ref_desc(d.id).map(<[u8]>::to_vec) == ty;
                let reads = o.uses.iter().any(|u| u.reg == r && u.val != UNDEF && own.contains(&self.root(u.val)));
                let arith = matches!(o.insn.op, 0x7b..=0xe2);
                let kept = self.web_uses.get(&web).is_some_and(|w| w.0 > 1 || w.1);
                let in_place = o.insn.op != 0x1f && same && reads && (arith || kept);
                if web == root || in_place || (name.is_some() && name_at(self, r, o.end) == name) {
                    clean = false;
                    named.insert(web);
                    own.insert(web);
                } else {
                    reused.insert(web);
                }
            }
            clean_param.push(clean);
        }
        reused.retain(|w| !named.contains(w));
        self.reused = reused;
        self.clean_param = clean_param;
    }

    /// Removes the register copies d8 makes to line up a range call: a
    /// `move` of a parameter or of a value read only by it, whose copy is
    /// read once later in the same block, hands that reader the value itself,
    /// when no other op writes the value's local in between. The value can
    /// then fold into its reader as an expression, and a constructor's
    /// `super(...)` reads its parameters where javac's does.
    fn forward_moves(&mut self) {
        let single = |m: &Self, v: u32| {
            m.direct_uses.get(v as usize) == Some(&1) && m.web_uses.get(&m.root(v)).is_some_and(|w| w.0 == 1 && !w.1)
        };
        let mut user = vec![usize::MAX; self.def_op.len()];
        for (k, o) in self.ops.iter().enumerate() {
            for u in &o.uses {
                user[u.val as usize] = k;
            }
        }
        let mut moved: Vec<(usize, usize, u32)> = Vec::new();
        for k in 0..self.ops.len() {
            let o = &self.ops[k];
            let (Some(d), Some(src)) = (&o.def, o.uses.first()) else { continue };
            let u = user[d.id as usize];
            let from = self.def_op[src.val as usize];
            let param = from == usize::MAX && (src.val as usize) < self.params.len();
            let plain = matches!(o.insn.op, 0x01..=0x09) && o.kind == Kind::Plain && (!self.named_def(k) || param && self.moves_local(k));
            let source = param || from < self.ops.len() && single(self, src.val);
            if plain && source && single(self, d.id) && u > k && u < self.ops.len() && self.ops[u].block == o.block {
                moved.push((k, u, d.id));
            }
        }
        loop {
            let absorbed: BTreeSet<usize> = moved.iter().map(|m| m.0).collect();
            let source = |m: &Self, mut v: u32| {
                while let Some(&(k, _, _)) = moved.iter().find(|x| x.2 == v) {
                    v = m.ops[k].uses[0].val;
                }
                v
            };
            let bad = moved.iter().position(|&(k, u, d)| {
                let v = source(self, d);
                let (reg, start) = match self.def_op[v as usize] {
                    usize::MAX => (self.params[v as usize].0, k),
                    at => (self.ops[at].def.as_ref().expect("def").reg, at.max(k)),
                };
                let c = self.cat[v as usize];
                (start + 1..u).any(|j| !absorbed.contains(&j) && self.ops[j].def.as_ref().is_some_and(|x| x.reg == reg && self.cat[x.id as usize] == c))
            });
            match bad {
                Some(i) => {
                    moved.remove(i);
                }
                None => break,
            }
        }
        for &(k, u, d) in &moved {
            let src = self.ops[k].uses[0].clone();
            for x in &mut self.ops[u].uses {
                if x.val == d {
                    x.val = src.val;
                    x.reg = src.reg;
                }
            }
            let o = &mut self.ops[k];
            o.kind = Kind::Absorbed;
            o.def = None;
            o.uses.clear();
            o.rel = Rel::None;
        }
    }

    /// Whether the value is a constant read only where it reaches alone.
    pub fn inline_const(&self, v: u32) -> bool {
        let k = self.def_op[v as usize];
        if k >= self.ops.len() {
            return false;
        }
        let o = &self.ops[k];
        if !matches!(o.insn.op, 0x12..=0x1c) || self.named_def(k) {
            return false;
        }
        let (_, phi) = self.web_uses.get(&self.root(v)).copied().unwrap_or((0, false));
        !phi
    }

    /// The constant op a value holds: its def, or the def a `move` copies.
    pub fn const_op(&self, v: u32) -> Option<usize> {
        let k = *self.def_op.get(v as usize)?;
        let o = self.ops.get(k)?;
        match o.insn.op {
            0x12..=0x1c => Some(k),
            0x01..=0x09 if o.kind == Kind::Plain => {
                let src = *self.def_op.get(o.uses[0].val as usize)?;
                self.ops.get(src).filter(|s| matches!(s.insn.op, 0x12..=0x1c)).map(|_| src)
            }
            _ => None,
        }
    }

    /// The parameter a value copies when it is a `move` of a parameter (or
    /// of such a copy) whose JVM local nothing else writes: each use of the
    /// copy can then read the parameter's own local.
    pub fn param_copy(&self, v: u32) -> Option<usize> {
        let mut v = v;
        for _ in 0..4 {
            let k = *self.def_op.get(v as usize)?;
            if (v as usize) < self.params.len() && k == usize::MAX {
                return self.clean_param[v as usize].then_some(v as usize);
            }
            let o = self.ops.get(k)?;
            if !matches!(o.insn.op, 0x01..=0x09) || o.kind != Kind::Plain || (self.named_def(k) && !self.moves_local(k)) {
                return None;
            }
            if self.web_uses.get(&self.root(v)).is_none_or(|w| w.1) {
                return None;
            }
            v = o.uses[0].val;
        }
        None
    }

    /// Whether a `move` only carries a source variable to another register:
    /// debug info gives its destination, right after it, the name and type
    /// its source has at it.
    fn moves_local(&self, k: usize) -> bool {
        let o = &self.ops[k];
        let (Some(d), Some(u)) = (&o.def, o.uses.first()) else { return false };
        let at = |reg: u32, pc: u32| {
            self.debug
                .locals
                .iter()
                .find(|l| l.reg == reg && l.start <= pc && pc < l.end.max(l.start + 1))
                .map(|l| (l.name, l.ty))
        };
        at(d.reg, o.end).is_some_and(|n| at(u.reg, o.insn.addr) == Some(n))
    }

    /// Whether debug info names the register an op defines right after it.
    pub fn named_def(&self, k: usize) -> bool {
        let o = &self.ops[k];
        let Some(d) = &o.def else { return false };
        self.debug
            .locals
            .iter()
            .any(|l| l.reg == d.reg && l.start <= o.end && o.end < l.end.max(l.start + 1))
    }

    fn infer_types(&mut self) -> Result<()> {
        let values = self.parent.len();
        for v in 0..values as u32 {
            self.find(v);
        }
        let mut tp: Vec<u32> = (0..values as u32).map(|v| self.root(v)).collect();
        fn tfind(p: &mut [u32], mut v: u32) -> u32 {
            while p[v as usize] != v {
                p[v as usize] = p[p[v as usize] as usize];
                v = p[v as usize];
            }
            v
        }
        let mut mask: Vec<Mask> = vec![N | W | L; values];
        let tunion = |p: &mut Vec<u32>, mask: &mut Vec<Mask>, a: u32, b: u32| {
            let (ra, rb) = (tfind(p, a), tfind(p, b));
            if ra != rb {
                p[rb as usize] = ra;
                mask[ra as usize] &= mask[rb as usize];
            }
        };
        let inline: Vec<bool> = (0..values as u32).map(|v| self.inline_const(v)).collect();
        for (k, &(_, m)) in self.params.iter().enumerate() {
            let t = tfind(&mut tp, k as u32);
            mask[t as usize] &= m;
        }
        let live: Vec<usize> = (0..self.ops.len())
            .filter(|&k| self.blocks[self.ops[k].block as usize].reachable)
            .collect();
        for &k in &live {
            let o = &self.ops[k];
            for u in &o.uses {
                if !inline[u.val as usize] {
                    let t = tfind(&mut tp, u.val);
                    mask[t as usize] &= u.mask;
                }
            }
            if let Some(d) = &o.def {
                let t = tfind(&mut tp, d.id);
                mask[t as usize] &= d.mask;
            }
            match o.rel {
                Rel::Same => {
                    let (u, d) = (&o.uses[0], o.def.as_ref().expect("def"));
                    if !inline[u.val as usize] {
                        tunion(&mut tp, &mut mask, d.id, u.val);
                    }
                }
                Rel::SameUses => {
                    let (a, b) = (&o.uses[0], &o.uses[1]);
                    if !inline[a.val as usize] && !inline[b.val as usize] {
                        tunion(&mut tp, &mut mask, a.val, b.val);
                    }
                }
                _ => {}
            }
        }
        self.tvar = (0..values as u32).map(|v| tfind(&mut tp, v)).collect();
        self.infer_ref_types(&live);
        for &k in &live {
            let o = &self.ops[k];
            if !matches!(o.rel, Rel::ArrayGet | Rel::ArrayPut) {
                continue;
            }
            let Some(e) = self.elem_cat(o.uses[0].val) else { continue };
            let target = if o.rel == Rel::ArrayGet {
                o.def.as_ref().map(|d| d.id)
            } else {
                Some(o.uses[2].val).filter(|&v| !inline[v as usize])
            };
            if let Some(t) = target {
                mask[self.tvar[t as usize] as usize] &= e;
            }
        }
        self.cat = vec![0; values];
        for v in 0..values {
            if inline[v] {
                continue;
            }
            let m = mask[self.tvar[v] as usize];
            self.cat[v] = resolve(m, self.value_wide[v]).ok_or_else(|| {
                err(format!(
                    "conflicting types for a register value{}",
                    match self.def_op.get(v) {
                        Some(&k) if k < self.ops.len() => format!(" defined at {:#x}", self.ops[k].insn.addr),
                        _ => String::new(),
                    }
                ))
            })?;
        }
        Ok(())
    }

    /// The element category of the array a value holds, when its type is known.
    pub fn elem_cat(&self, v: u32) -> Option<Mask> {
        let t = self.ref_type.get(&self.tvar[v as usize])?;
        Some(cat(t.strip_prefix(b"[")?)).filter(|&c| c != 0)
    }

    pub fn ref_desc(&self, v: u32) -> Option<&[u8]> {
        self.ref_type.get(&self.tvar[v as usize]).map(Vec::as_slice)
    }

    /// A reference type (descriptor) per type variable where one is known:
    /// parameters, allocations, casts, members, results, element reads.
    fn infer_ref_types(&mut self, live: &[usize]) {
        let mut types: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        let desc = |t: u32| self.dex.type_desc(t).ok().map(<[u8]>::to_vec);
        let tv = &self.tvar;
        for (k, &(_, m)) in self.params.iter().enumerate() {
            if m != L || (!self.is_static && k == 0) {
                continue;
            }
            let pi = k - usize::from(!self.is_static);
            if let Some(t) = self.proto.params.get(pi).and_then(|&t| desc(t)) {
                types.entry(tv[k]).or_insert(t);
            }
        }
        for _round in 0..4 {
            let mut changed = false;
            for &k in live {
                let o = &self.ops[k];
                let Some(d) = &o.def else { continue };
                if d.mask & L == 0 {
                    continue;
                }
                let x = &o.insn;
                let t: Option<Vec<u8>> = match (o.kind, x.op) {
                    (Kind::NewObject { ty }, _) => desc(ty),
                    (_, 0x1a | 0x1b) => Some(b"Ljava/lang/String;".to_vec()),
                    (_, 0x1c) => Some(b"Ljava/lang/Class;".to_vec()),
                    (_, 0x1f | 0x22 | 0x23 | 0x24 | 0x25) => desc(x.idx),
                    (_, 0x46) => types
                        .get(&tv[o.uses[0].val as usize])
                        .and_then(|t| t.strip_prefix(b"["))
                        .map(<[u8]>::to_vec),
                    (_, 0x52..=0x58 | 0x60..=0x66) => self.dex.field(x.idx).ok().and_then(|f| desc(f.ty)),
                    (_, 0x6e..=0x72 | 0x74..=0x78) => self
                        .dex
                        .method(x.idx)
                        .ok()
                        .and_then(|m| self.dex.proto(m.proto).ok())
                        .and_then(|p| desc(p.ret)),
                    _ => None,
                };
                if let Some(t) = t {
                    let slot = types.entry(tv[d.id as usize]).or_insert_with(|| {
                        changed = true;
                        t.clone()
                    });
                    if slot.first() != Some(&b'[') && t.first() == Some(&b'[') {
                        *slot = t;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        self.ref_type = types;
    }

    /// The category of the value a use reads.
    pub fn use_cat(&self, o: &Op, u: usize) -> Mask {
        match self.cat[o.uses[u].val as usize] {
            0 => self.inline_cat(o, u),
            c => c,
        }
    }

    /// The category an inlined constant takes at one use.
    pub fn inline_cat(&self, o: &Op, u: usize) -> Mask {
        let use_ = &o.uses[u];
        let mut m = use_.mask;
        let known = |v: u32| Some(self.cat[v as usize]).filter(|&c| c != 0);
        match o.rel {
            Rel::Same => {
                if let Some(c) = o.def.as_ref().and_then(|d| known(d.id)) {
                    m &= c;
                }
            }
            Rel::SameUses => {
                if let Some(c) = known(o.uses[1 - u].val) {
                    m &= c;
                }
            }
            Rel::ArrayPut if u == 2 => {
                if let Some(e) = self.elem_cat(o.uses[0].val) {
                    m &= e;
                }
            }
            _ => {}
        }
        if let Some(d) = self.ops.get(self.def_op[use_.val as usize]).and_then(|o| o.def.as_ref()) {
            m &= d.mask;
        }
        resolve(m, self.value_wide[use_.val as usize]).unwrap_or(I)
    }

    pub fn def_cat(&self, d: &Def) -> Mask {
        match self.cat[d.id as usize] {
            0 => resolve(d.mask, is_wide(d.mask)).unwrap_or(I),
            c => c,
        }
    }
}

/// The forward value pass: block entry states (a value per live register),
/// the phi created for each (block, register), the one predecessor that
/// supplied an entry value no phi joins yet, and the worklist in reverse
/// postorder.
struct Flow {
    entry: Vec<Option<Vec<u32>>>,
    phis: BTreeMap<(u32, u32), u32>,
    from: BTreeMap<(u32, u32), u32>,
    live: Vec<Vec<u64>>,
    rpo: Vec<u32>,
    work: BTreeSet<(u32, u32)>,
}

impl Flow {
    /// Merges `incoming` into block `s`'s entry, creating or joining the
    /// block's phi where a live register receives a second value.
    fn merge(&mut self, m: &mut Method, from: u32, s: u32, incoming: &[u32]) {
        let su = s as usize;
        let mut changed = self.entry[su].is_none();
        let target = self.entry[su].get_or_insert_with(|| vec![UNDEF; incoming.len()]);
        for (r, &inc) in incoming.iter().enumerate() {
            if inc == UNDEF || self.live[su][r / 64] >> (r % 64) & 1 == 0 {
                continue;
            }
            let k = m.def_op[inc as usize];
            if m.ops.get(k).is_some_and(|o| matches!(o.insn.op, 0x01..=0x09 | 0x12..=0x1c)) {
                m.edge_const.insert((s, r as u32, from), inc);
            } else {
                m.edge_const.remove(&(s, r as u32, from));
            }
            let cur = target[r];
            let key = (s, r as u32);
            if cur == UNDEF {
                target[r] = inc;
                self.from.insert(key, from);
                changed = true;
            } else if cur == inc {
                if self.from.get(&key).is_some_and(|&f| f != from) {
                    self.from.insert(key, u32::MAX);
                }
            } else if self.from.get(&key) == Some(&from) {
                target[r] = inc;
                changed = true;
            } else {
                self.from.remove(&key);
                let phi = *self.phis.entry((s, r as u32)).or_insert_with(|| {
                    m.phi_block.push(s);
                    m.phi_reg.push(r as u32);
                    m.new_value(m.value_wide[inc as usize], usize::MAX - 1)
                });
                m.union(phi, inc);
                if cur != phi {
                    m.union(phi, cur);
                    target[r] = phi;
                    changed = true;
                }
            }
        }
        if changed {
            self.work.insert((self.rpo[su], s));
        }
    }
}

/// One category from a mask: int over float over reference for narrow
/// values, long over double for wide ones.
pub fn resolve(m: Mask, wide: bool) -> Option<Mask> {
    let m = if wide { m & W } else { m & (N | L) };
    [I, J, F, D, L].into_iter().find(|&c| m & c != 0)
}
