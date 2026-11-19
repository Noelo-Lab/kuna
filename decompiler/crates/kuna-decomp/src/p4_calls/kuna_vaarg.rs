//! (kuna `vaarg`) The x86-64 SysV `va_arg` expansion prints as `va_arg(ap, T)`.
//!
//! `va_arg(ap, T)` compiles to a test of the offset into the register save
//! area (gcc `ix86_gimplify_va_arg`, clang `X86_64ABIInfo::EmitVAArg`): below
//! `48 - 8n` (`176 - 16` for a double) the register arm `R` reads
//! `reg_save_area + offset` and steps the offset, otherwise the memory arm `M`
//! reads `overflow_arg_area` and steps that pointer. kuna printed both arms.
//!
//! The va_list state takes one of three forms, and [`ActionVaArg`] collects
//! each into *webs* of values: the offset web (`gp_offset` or `fp_offset`)
//! and the overflow web.
//! - Through a `va_list` pointer `ap` (a `va_list` parameter, or one whose
//!   address escapes): each site loads `ap->gp_offset` and the arms store the
//!   steps back; a web is that site's loads and steps.
//! - Register-cached (clang loops): one load of `ap->gp_offset` before the
//!   loop, the offset carried in a register and stored back after each step.
//! - SSA (a `va_list` local that heritage promoted, gcc and clang at every
//!   level): the offset web starts at the `va_start` constant (`8` per named
//!   integer argument, `48 + 16` per named double) and the overflow web at the
//!   caller's stack arguments (`SP + 8`); no memory `va_list` is left.
//!
//! A web member may only feed other members, the site tests, the read
//! addresses and (pointer forms) the stores back to its `ap` field; anything
//! else and its sites stay as they are. A read is a LOAD through
//! `reg_save_area + offset`, through an overflow member, or through the join
//! of the two.
//!
//! The arms need not meet at a join. gcc duplicates the code after the read
//! into each arm (a loop's latch, or the whole rest of the loop once the
//! offset passes the limit). With every read taken as an abstract `va_arg`,
//! the action checks that the path from `M` behaves as the path from `R`:
//! it walks the two in step, matching their loads, stores, calls, reads and
//! returns, their branch conditions (up to `x < c` / `x >= c` and `==` / `!=`
//! swaps) and the values they hand to the blocks where they meet. A site test
//! met on the way counts as already folded, going to its register arm; that is
//! how gcc's overflow loop, which never tests the offset again, matches the
//! register loop through the test.
//!
//! The webs that share a site, a read or a join fold together, or not at all,
//! since an SSA offset or a register-cached one is only right while every
//! read that steps it is still there: every read becomes the CALLOTHER
//! `va_arg(ap, class)`, every site's edge to `M` goes and with it the blocks
//! only `M` reached, and the stores back to `ap` go. The SSA form has no `ap`
//! to pass: the call's `ap` input is the constant 0, printed `ap`, and the
//! action adds the `va_start(ap, last)` the promoted prologue lost (`last` as
//! in `vastart`). The class input is 1 for the SSE half. It prints
//! `va_arg(ap,T)`, `T` the type of its output, or `double` for an SSE read of
//! an integer or untyped output, `int`/`long` by size for an untyped one, and
//! `int` for a read narrower than `int`.

use std::collections::{HashMap, HashSet};

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_strlenscan::{eval, taken};

/// `(base, offset)` of a pointer expression `base + offset`.
fn field(data: &Funcdata, mut v: VarnodeId) -> Option<(VarnodeId, u64)> {
    let mut off = 0u64;
    for _ in 0..4 {
        let Some(d) = data.vbank().get(v)?.get_def().and_then(|d| data.obank().get(d)) else { return Some((v, off)) };
        let k = |i: int4| d.get_in(i).and_then(|x| data.vbank().get(x)).filter(|x| x.is_constant()).map(|x| x.get_offset());
        match d.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => v = d.get_in(0)?,
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB if k(1).is_some() => {
                off = off.wrapping_add(k(1)?);
                v = d.get_in(0)?;
            }
            OpCode::CPUI_PTRADD if k(1).is_some() && k(2).is_some() => {
                off = off.wrapping_add(k(1)?.wrapping_mul(k(2)?));
                v = d.get_in(0)?;
            }
            _ => return Some((v, off)),
        }
    }
    Some((v, off))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    Gp,
    Fp,
}

/// What a value is to the va_list state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Va {
    /// A member of offset web `w`.
    Off(usize),
    /// A member of overflow web `w`.
    Ovf(usize),
    /// `ap->reg_save_area`, loaded.
    Save,
    /// `reg_save_area + offset`, the offset of web `w`.
    Addr(usize),
    /// The join of a register address of offset web `w` and overflow web `o`.
    Merged(usize, usize),
}

#[derive(Default)]
struct Web {
    class: Option<Class>,
    /// The `va_list` pointer of a pointer-form web.
    ap: Option<VarnodeId>,
    /// The `va_start` constant of an SSA offset web.
    init: Option<u64>,
    bad: bool,
}

struct Site {
    c: BlockId,
    r: BlockId,
    m: BlockId,
    web: usize,
}

/// A read: the LOAD, its offset web (register or merged address) and its
/// overflow web (overflow or merged address).
#[derive(Clone, Copy)]
struct Read {
    off: Option<usize>,
    ovf: Option<usize>,
}

#[derive(Default)]
struct Cls {
    va: HashMap<VarnodeId, Va>,
    webs: Vec<Web>,
    sites: Vec<Site>,
    /// Site tests (the compare and any BOOL_NEGATEs) and their CBRANCHes.
    guards: HashSet<OpId>,
    /// Stores of a web member back to its `ap` field, by web.
    stores: HashMap<OpId, usize>,
    reads: HashMap<OpId, Read>,
    /// The LOADs of each `(base, offset)` field, by output.
    fields: HashMap<(VarnodeId, u64), Vec<VarnodeId>>,
}

fn def_op(data: &Funcdata, v: VarnodeId) -> Option<(OpId, &crate::op::PcodeOp)> {
    let d = data.vbank().get(v)?.get_def()?;
    Some((d, data.obank().get(d)?))
}

fn constant(data: &Funcdata, v: VarnodeId) -> Option<u64> {
    data.vbank().get(v).filter(|n| n.is_constant()).map(|n| n.get_offset())
}

fn size(data: &Funcdata, v: VarnodeId) -> int4 {
    data.vbank().get(v).map_or(0, |n| n.get_size())
}

fn is_sp(data: &Funcdata, v: VarnodeId) -> bool {
    data.vbank().get(v).is_some_and(|n| n.is_spacebase() && n.is_input())
}

/// The stack offset of `SP + c`, through COPYs and CASTs.
fn sp_offset(data: &Funcdata, mut v: VarnodeId) -> Option<i64> {
    for _ in 0..4 {
        let (_, o) = def_op(data, v)?;
        match o.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => v = o.get_in(0)?,
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB if is_sp(data, o.get_in(0)?) => return Some(constant(data, o.get_in(1)?)? as i64),
            _ => return None,
        }
    }
    None
}

/// The site test of CBRANCH block `c`: the tested value, the test ops, the
/// class, and the register and memory arms.
fn guard(data: &Funcdata, c: BlockId) -> Option<(VarnodeId, Vec<OpId>, Class, BlockId, BlockId)> {
    let (t, f) = taken(data, c)?;
    let branch = data.bb_op_tail(c)?;
    let cond = data.obank().get(branch)?.get_in(1)?;
    let mut ops = vec![branch];
    let mut v = cond;
    let cmp = loop {
        let (d, o) = def_op(data, v)?;
        if data.descend_snapshot(v).len() != 1 {
            return None;
        }
        ops.push(d);
        match o.code() {
            OpCode::CPUI_BOOL_NEGATE => v = o.get_in(0)?,
            OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => break o,
            _ => return None,
        }
        if ops.len() > 4 {
            return None;
        }
    };
    let (a, b) = (cmp.get_in(0)?, cmp.get_in(1)?);
    let x = match (constant(data, a), constant(data, b)) {
        (Some(_), None) => b,
        (None, Some(_)) => a,
        _ => return None,
    };
    if !matches!(size(data, x), 4 | 8) {
        return None;
    }
    let goes = |val: u64| eval(data, cond, &[(x, val)], &[c], 0).map(|r| if r != 0 { t } else { f });
    let arms = |lo: u64, limit: u64, step: u64, most: u64| -> Option<(BlockId, BlockId)> {
        let (r, m) = (goes(lo)?, goes(limit)?);
        (r != m && (1..=most).any(|n| goes(limit - step * n) == Some(r) && goes(limit - step * n + step) == Some(m))).then_some((r, m))
    };
    let (class, (r, m)) = match arms(0, 48, 8, 2) {
        Some(rm) => (Class::Gp, rm),
        None => (Class::Fp, arms(48, 176, 16, 1)?),
    };
    if r == c || m == c || data.bblocks_ref().block(r).size_in() != 1 {
        return None;
    }
    Some((x, ops, class, r, m))
}

impl Cls {
    fn new_web(&mut self, class: Option<Class>) -> usize {
        self.webs.push(Web { class, ..Web::default() });
        self.webs.len() - 1
    }

    /// Collect the offset web through `x`.
    fn offset_web(&mut self, data: &Funcdata, x: VarnodeId, class: Class) -> Option<usize> {
        if let Some(Va::Off(w)) = self.va.get(&x) {
            return (self.webs[*w].class == Some(class)).then_some(*w);
        }
        let mut members: HashSet<VarnodeId> = HashSet::new();
        let mut work = vec![x];
        let mut addrs: Vec<(OpId, VarnodeId)> = Vec::new();
        let mut stores = Vec::new();
        let mut init = None;
        let mut ap = None;
        let mut compares = Vec::new();
        let field_off = if class == Class::Gp { 0 } else { 4 };
        while let Some(v) = work.pop() {
            if !members.insert(v) || members.len() > 64 {
                if members.len() > 64 {
                    return None;
                }
                continue;
            }
            match def_op(data, v) {
                None => return None,
                Some((_, o)) => match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => {
                        let n = if matches!(o.code(), OpCode::CPUI_MULTIEQUAL) { o.num_input() } else { 1 };
                        for i in 0..n {
                            let input = o.get_in(i)?;
                            match constant(data, input) {
                                Some(k) => {
                                    let ok = match class {
                                        Class::Gp => k % 8 == 0 && k <= 48,
                                        Class::Fp => (48..=176).contains(&k) && (k - 48) % 16 == 0,
                                    };
                                    if !ok || init.is_some_and(|i| i != k) {
                                        return None;
                                    }
                                    init = Some(k);
                                }
                                None => work.push(input),
                            }
                        }
                    }
                    OpCode::CPUI_INT_ADD if constant(data, o.get_in(1)?).is_some() => work.push(o.get_in(0)?),
                    OpCode::CPUI_LOAD => {
                        let (base, off) = field(data, o.get_in(1)?)?;
                        if off != field_off || size(data, v) != 4 || ap.is_some_and(|a| a != base) {
                            return None;
                        }
                        ap = Some(base);
                    }
                    _ => return None,
                },
            }
            for u in data.descend_snapshot(v) {
                let o = data.obank().get(u)?;
                let out = o.get_out();
                match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_MULTIEQUAL => work.push(out?),
                    OpCode::CPUI_INDIRECT if o.get_in(0) == Some(v) => work.push(out?),
                    OpCode::CPUI_INT_ADD => {
                        let other = if o.get_in(0) == Some(v) { o.get_in(1)? } else { o.get_in(0)? };
                        if constant(data, other).is_some() {
                            work.push(out?);
                        } else {
                            addrs.push((u, other));
                        }
                    }
                    OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => compares.push(u),
                    OpCode::CPUI_STORE if o.get_in(2) == Some(v) => stores.push(u),
                    _ => return None,
                }
            }
        }
        if init.is_some() == ap.is_some() {
            return None;
        }
        if compares.iter().any(|c| !self.guards.contains(c)) {
            return None;
        }
        for &s in &stores {
            let p = data.obank().get(s)?.get_in(1)?;
            if ap.is_none() || field(data, p) != Some((ap?, field_off)) {
                return None;
            }
        }
        let w = self.new_web(Some(class));
        self.webs[w].init = init;
        self.webs[w].ap = ap;
        for &m in &members {
            self.va.insert(m, Va::Off(w));
        }
        for s in stores {
            self.stores.insert(s, w);
        }
        for (add, save) in addrs {
            if !self.save_area(data, save, ap) || data.obank().get(add).and_then(|o| o.get_out()).is_none_or(|o| size(data, o) != 8) {
                self.webs[w].bad = true;
                continue;
            }
            let out = data.obank().get(add)?.get_out()?;
            self.address(data, out, w);
        }
        Some(w)
    }

    /// Is `v` the register save area: `SP + c` for an SSA web, the loaded
    /// `ap->reg_save_area` for a pointer-form one?
    fn save_area(&mut self, data: &Funcdata, v: VarnodeId, ap: Option<VarnodeId>) -> bool {
        match ap {
            None => sp_offset(data, v).is_some_and(|c| c < 0),
            Some(ap) => {
                let ok = def_op(data, v).is_some_and(|(_, o)| o.code() == OpCode::CPUI_LOAD && o.get_in(1).and_then(|p| field(data, p)) == Some((ap, 16)) && size(data, v) == 8);
                if ok {
                    self.va.insert(v, Va::Save);
                }
                ok
            }
        }
    }

    /// Record register address `v` of offset web `w` and what reads it.
    fn address(&mut self, data: &Funcdata, v: VarnodeId, w: usize) {
        let mut work = vec![v];
        while let Some(v) = work.pop() {
            if self.va.insert(v, Va::Addr(w)).is_some() {
                continue;
            }
            for u in data.descend_snapshot(v) {
                let Some(o) = data.obank().get(u) else { continue };
                match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST => work.extend(o.get_out()),
                    OpCode::CPUI_LOAD if o.get_in(1) == Some(v) => {
                        self.reads.insert(u, Read { off: Some(w), ovf: None });
                    }
                    OpCode::CPUI_MULTIEQUAL => {}
                    _ => self.webs[w].bad = true,
                }
            }
        }
    }

    /// Collect the overflow web through `x`, rooted at `SP + 8k` (or `SP`
    /// itself, a phi input) or at the loaded `ap->overflow_arg_area`.
    fn overflow_web(&mut self, data: &Funcdata, x: VarnodeId) -> Option<usize> {
        if let Some(Va::Ovf(w)) = self.va.get(&x) {
            return Some(*w);
        }
        if self.va.contains_key(&x) {
            return None;
        }
        let mut members: HashSet<VarnodeId> = HashSet::new();
        let mut work = vec![x];
        let mut reads = Vec::new();
        let mut stores = Vec::new();
        let mut ap = None;
        let mut rooted = false;
        while let Some(v) = work.pop() {
            if !members.insert(v) {
                continue;
            }
            if members.len() > 64 || self.va.contains_key(&v) {
                return None;
            }
            let (_, o) = def_op(data, v)?;
            match o.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => work.push(o.get_in(0)?),
                OpCode::CPUI_MULTIEQUAL => {
                    for i in 0..o.num_input() {
                        let input = o.get_in(i)?;
                        if is_sp(data, input) {
                            rooted = true;
                        } else {
                            work.push(input);
                        }
                    }
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB if is_sp(data, o.get_in(0)?) => {
                    let k = constant(data, o.get_in(1)?)?;
                    if k < 8 || k % 8 != 0 || k > 0x1000 {
                        return None;
                    }
                    rooted = true;
                }
                OpCode::CPUI_INT_ADD if constant(data, o.get_in(1)?).is_some_and(|k| k % 8 == 0) => work.push(o.get_in(0)?),
                OpCode::CPUI_PTRADD if constant(data, o.get_in(1)?).is_some() && constant(data, o.get_in(2)?).is_some() => work.push(o.get_in(0)?),
                OpCode::CPUI_LOAD => {
                    let (base, off) = field(data, o.get_in(1)?)?;
                    if off != 8 || size(data, v) != 8 || ap.is_some_and(|a| a != base) {
                        return None;
                    }
                    ap = Some(base);
                    rooted = true;
                }
                _ => return None,
            }
            for u in data.descend_snapshot(v) {
                let o = data.obank().get(u)?;
                match o.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST => work.push(o.get_out()?),
                    OpCode::CPUI_INDIRECT if o.get_in(0) == Some(v) => work.push(o.get_out()?),
                    OpCode::CPUI_INT_ADD if o.get_in(0) == Some(v) && constant(data, o.get_in(1)?).is_some_and(|k| k % 8 == 0) => work.push(o.get_out()?),
                    OpCode::CPUI_PTRADD if o.get_in(0) == Some(v) && constant(data, o.get_in(1)?).is_some() && constant(data, o.get_in(2)?).is_some() => work.push(o.get_out()?),
                    OpCode::CPUI_MULTIEQUAL => {
                        let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
                        if !ins.iter().any(|&i| matches!(self.va.get(&i), Some(Va::Addr(_)))) {
                            work.push(o.get_out()?);
                        }
                    }
                    OpCode::CPUI_LOAD if o.get_in(1) == Some(v) => reads.push(u),
                    OpCode::CPUI_STORE if o.get_in(2) == Some(v) => stores.push(u),
                    _ => return None,
                }
            }
        }
        if !rooted {
            return None;
        }
        for &s in &stores {
            let p = data.obank().get(s)?.get_in(1)?;
            if ap.is_none() || field(data, p) != Some((ap?, 8)) {
                return None;
            }
        }
        let w = self.new_web(None);
        self.webs[w].ap = ap;
        for &m in &members {
            self.va.insert(m, Va::Ovf(w));
        }
        for s in stores {
            self.stores.insert(s, w);
        }
        for r in reads {
            self.reads.insert(r, Read { off: None, ovf: Some(w) });
        }
        Some(w)
    }

    /// The joins of register and overflow addresses, and their reads.
    fn joins(&mut self, data: &Funcdata) {
        let addrs: Vec<VarnodeId> = self.va.iter().filter(|(_, k)| matches!(k, Va::Addr(_))).map(|(v, _)| *v).collect();
        let mut work: Vec<OpId> = addrs.iter().flat_map(|&a| data.descend_snapshot(a)).filter(|&u| data.obank().get(u).is_some_and(|o| o.code() == OpCode::CPUI_MULTIEQUAL)).collect();
        let mut seen = HashSet::new();
        while let Some(phi) = work.pop() {
            if !seen.insert(phi) || seen.len() > 128 {
                continue;
            }
            let Some(o) = data.obank().get(phi) else { continue };
            let Some(out) = o.get_out() else { continue };
            let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
            let mut off = None;
            let mut ovf = None;
            let mut ok = true;
            for &i in &ins {
                let kind = match self.va.get(&i).copied() {
                    Some(k) => Some(k),
                    None => self.overflow_web(data, i).map(Va::Ovf),
                };
                match kind {
                    Some(Va::Addr(w)) => ok &= off.replace(w).is_none_or(|x| x == w),
                    Some(Va::Ovf(w)) => ok &= ovf.replace(w).is_none_or(|x| x == w),
                    Some(Va::Merged(w, v)) => ok &= off.replace(w).is_none_or(|x| x == w) && ovf.replace(v).is_none_or(|x| x == v),
                    _ => {
                        // A join input still to be classified waits for its own phi.
                        ok = false;
                    }
                }
            }
            let (Some(w), Some(v)) = (off, ovf) else {
                if let Some(w) = off {
                    self.webs[w].bad = true;
                }
                continue;
            };
            if !ok {
                self.webs[w].bad = true;
                self.webs[v].bad = true;
                continue;
            }
            if self.va.insert(out, Va::Merged(w, v)).is_some() {
                continue;
            }
            for u in data.descend_snapshot(out) {
                let Some(uo) = data.obank().get(u) else { continue };
                match uo.code() {
                    OpCode::CPUI_LOAD if uo.get_in(1) == Some(out) => {
                        self.reads.insert(u, Read { off: Some(w), ovf: Some(v) });
                    }
                    OpCode::CPUI_MULTIEQUAL => work.push(u),
                    _ => {
                        self.webs[w].bad = true;
                        self.webs[v].bad = true;
                    }
                }
            }
        }
    }

    fn classify(data: &Funcdata) -> Cls {
        let mut cls = Cls::default();
        let blocks: Vec<BlockId> = (0..data.bblocks_get_size()).map(|i| data.bblocks_get_block(i)).collect();
        let mut tests = Vec::new();
        for &c in &blocks {
            if let Some(g) = guard(data, c) {
                cls.guards.extend(g.1.iter().copied());
                tests.push((c, g));
            }
        }
        if tests.is_empty() {
            return cls;
        }
        for &b in &blocks {
            for op in data.bb_ops(b) {
                let Some(o) = data.obank().get(op) else { continue };
                if o.code() == OpCode::CPUI_LOAD {
                    if let (Some(p), Some(out)) = (o.get_in(1), o.get_out()) {
                        if let Some(f) = field(data, p) {
                            cls.fields.entry(f).or_default().push(out);
                        }
                    }
                }
            }
        }
        for (c, (x, _, class, r, m)) in tests {
            if let Some(web) = cls.offset_web(data, x, class) {
                cls.sites.push(Site { c, r, m, web });
            }
        }
        // A pointer-form site may load its offset again for the address.
        let aps: HashSet<VarnodeId> = cls.webs.iter().filter_map(|w| w.ap).collect();
        let reloads: Vec<(VarnodeId, Class)> = cls
            .fields
            .iter()
            .filter(|((base, off), _)| aps.contains(base) && matches!(off, 0 | 4))
            .flat_map(|((_, off), vs)| vs.iter().map(move |&v| (v, if *off == 0 { Class::Gp } else { Class::Fp })))
            .collect();
        for (v, class) in reloads {
            if !cls.va.contains_key(&v) {
                cls.offset_web(data, v, class);
            }
        }
        for i in 0..cls.sites.len() {
            for op in data.bb_ops(cls.sites[i].m) {
                let Some(o) = data.obank().get(op) else { continue };
                if o.code() == OpCode::CPUI_LOAD && !cls.reads.contains_key(&op) {
                    if let Some(p) = o.get_in(1) {
                        cls.overflow_web(data, p);
                    }
                }
            }
        }
        cls.joins(data);
        cls
    }

    fn is_va_op(&self, data: &Funcdata, op: OpId) -> bool {
        self.guards.contains(&op)
            || self.stores.contains_key(&op)
            || data.obank().get(op).and_then(|o| o.get_out()).is_some_and(|v| self.va.contains_key(&v))
    }

    fn read_webs(&self, op: OpId) -> impl Iterator<Item = usize> + '_ {
        self.reads.get(&op).into_iter().flat_map(|r| r.off.into_iter().chain(r.ovf))
    }
}

fn is_effect(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN | OpCode::CPUI_NEW | OpCode::CPUI_BRANCHIND
    )
}

fn commutes(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
    )
}

/// A branch condition up to negation: `x >= t`, `x == c`, `a == b`, or a value.
enum Pred {
    AtLeast(bool, VarnodeId, u64),
    EqConst(VarnodeId, u64),
    Eq(VarnodeId, VarnodeId),
    Value(VarnodeId),
}

/// Whether the walk from `M` behaves as the walk from `R` (see the module docs).
struct Walk<'a> {
    data: &'a Funcdata,
    cls: &'a Cls,
    /// Folded site tests, to their register arms.
    fold: &'a HashMap<BlockId, BlockId>,
    /// Webs whose reads are abstract.
    live: &'a HashSet<usize>,
    sigma: HashMap<VarnodeId, VarnodeId>,
    ops: HashMap<OpId, OpId>,
    m_vis: HashSet<BlockId>,
    r_vis: HashSet<BlockId>,
    /// Blocks defining a value both sides read as is.
    shared: HashSet<BlockId>,
    done: HashSet<(BlockId, BlockId)>,
    phi_targets: HashSet<VarnodeId>,
    m_reads: Vec<OpId>,
    r_reads: Vec<OpId>,
}

#[derive(Clone, Copy)]
struct Pos {
    b: BlockId,
    from: BlockId,
}

impl<'a> Walk<'a> {
    fn op(&self, op: OpId) -> Option<&'a crate::op::PcodeOp> {
        self.data.obank().get(op)
    }

    fn block_of(&self, v: VarnodeId) -> Option<BlockId> {
        def_op(self.data, v).and_then(|(_, o)| o.get_parent())
    }

    fn is_read(&self, op: OpId) -> bool {
        self.cls.reads.contains_key(&op) && self.cls.read_webs(op).all(|w| self.live.contains(&w))
    }

    fn eq(&mut self, a: VarnodeId, b: VarnodeId, depth: u32) -> bool {
        if a == b {
            if let Some(blk) = self.block_of(a) {
                self.shared.insert(blk);
            }
            return true;
        }
        if let Some(&x) = self.sigma.get(&a) {
            return x == b;
        }
        let data = self.data;
        if let (Some(x), Some(y)) = (constant(data, a), constant(data, b)) {
            return x == y && size(data, a) == size(data, b);
        }
        if depth > 24 || self.cls.va.contains_key(&a) || self.cls.va.contains_key(&b) {
            return false;
        }
        let (Some((da, oa)), Some((db, ob))) = (def_op(data, a), def_op(data, b)) else { return false };
        let (Some(ba), Some(bb)) = (oa.get_parent(), ob.get_parent()) else { return false };
        if !self.m_vis.contains(&ba) || !self.r_vis.contains(&bb) {
            return false;
        }
        if oa.code() != ob.code() || is_effect(oa.code()) || oa.code() == OpCode::CPUI_MULTIEQUAL || oa.num_input() != ob.num_input() || size(data, a) != size(data, b) {
            return false;
        }
        if oa.code() == OpCode::CPUI_INDIRECT {
            let iop = |o: &crate::op::PcodeOp| o.get_in(1).and_then(|v| data.vbank().get(v)).map(|v| {
                use slotmap::KeyData;
                OpId::from(KeyData::from_ffi(v.get_offset()))
            });
            let (ia, ib) = (iop(oa), iop(ob));
            let paired = match (ia, ib) {
                (Some(x), Some(y)) => self.ops.get(&x) == Some(&y) || (self.cls.is_va_op(data, x) && self.cls.is_va_op(data, y)),
                _ => false,
            };
            let ok = paired && self.eq(oa.get_in(0).unwrap_or(a), ob.get_in(0).unwrap_or(b), depth + 1);
            if ok {
                self.sigma.insert(a, b);
            }
            return ok;
        }
        let n = oa.num_input();
        let ins = |o: &crate::op::PcodeOp| -> Vec<VarnodeId> { (0..n).filter_map(|i| o.get_in(i)).collect() };
        let (ia, ib) = (ins(oa), ins(ob));
        let _ = (da, db);
        let straight = ia.iter().zip(&ib).all(|(&x, &y)| self.eq(x, y, depth + 1));
        let ok = straight || (n == 2 && commutes(oa.code()) && self.eq(ia[0], ib[1], depth + 1) && self.eq(ia[1], ib[0], depth + 1));
        if ok {
            self.sigma.insert(a, b);
        }
        ok
    }

    /// Is `x`, handed into the m-side block, the value `t` takes once the
    /// r-side enters `r`: `t` with each phi of `r.b` read on the edge from
    /// `r.from`, and each op `t` computes in the r-side segment `seg` matched
    /// by the same op on the m-side.
    fn eq_entry(&mut self, x: VarnodeId, t: VarnodeId, r: Pos, seg: &[BlockId], depth: u32) -> bool {
        let data = self.data;
        let Some((_, ot)) = def_op(data, t) else { return self.eq(x, t, depth) };
        let Some(bt) = ot.get_parent() else { return false };
        if ot.code() == OpCode::CPUI_MULTIEQUAL && bt == r.b {
            let i = data.bblocks_ref().block(r.b).get_in_index(r.from);
            return ot.get_in(i).is_some_and(|y| self.eq(x, y, depth));
        }
        if !seg.contains(&bt) || depth > 24 {
            return self.eq(x, t, depth);
        }
        if constant(data, x).is_some() || self.cls.va.contains_key(&x) || self.cls.va.contains_key(&t) {
            return false;
        }
        let Some((_, ox)) = def_op(data, x) else { return false };
        if ox.code() != ot.code() || is_effect(ot.code()) || ot.code() == OpCode::CPUI_MULTIEQUAL || ox.num_input() != ot.num_input() || size(data, x) != size(data, t) {
            return false;
        }
        if !ox.get_parent().is_some_and(|b| self.m_vis.contains(&b)) {
            return false;
        }
        let n = ot.num_input();
        let ins = |o: &crate::op::PcodeOp| -> Vec<VarnodeId> { (0..n).filter_map(|i| o.get_in(i)).collect() };
        let (ix, it) = (ins(ox), ins(ot));
        ix.iter().zip(&it).all(|(&a, &b)| match constant(data, b) {
            Some(k) => constant(data, a) == Some(k),
            None => self.eq_entry(a, b, r, seg, depth + 1),
        })
    }

    /// The blocks run from `start`: single-successor chains into
    /// single-predecessor blocks, and a folded test into its register arm.
    fn segment(&self, start: BlockId) -> Vec<BlockId> {
        let g = self.data.bblocks_ref();
        let mut seg = vec![start];
        while seg.len() < 16 {
            let last = *seg.last().unwrap_or(&start);
            let next = match self.fold.get(&last) {
                Some(&r) => r,
                None => {
                    let blk = g.block(last);
                    if blk.size_out() != 1 {
                        break;
                    }
                    let s = blk.get_out(0);
                    if g.block(s).size_in() != 1 {
                        break;
                    }
                    s
                }
            };
            if seg.contains(&next) {
                break;
            }
            seg.push(next);
        }
        seg
    }

    fn effects(&self, seg: &[BlockId]) -> Option<Vec<OpId>> {
        let mut out = Vec::new();
        for &b in seg {
            for op in self.data.bb_ops(b) {
                let o = self.op(op)?;
                if o.code() == OpCode::CPUI_MULTIEQUAL || self.cls.is_va_op(self.data, op) {
                    continue;
                }
                if o.code() == OpCode::CPUI_BRANCHIND {
                    return None;
                }
                if is_effect(o.code()) {
                    out.push(op);
                }
            }
        }
        Some(out)
    }

    fn phis(&self, b: BlockId) -> Vec<OpId> {
        self.data
            .bb_ops(b)
            .into_iter()
            .filter(|&op| self.op(op).is_some_and(|o| o.code() == OpCode::CPUI_MULTIEQUAL && o.get_out().is_some_and(|v| !self.cls.va.contains_key(&v))))
            .collect()
    }

    fn phi_in(&self, phi: OpId, b: BlockId, from: BlockId) -> Option<VarnodeId> {
        let i = self.data.bblocks_ref().block(b).get_in_index(from);
        self.op(phi)?.get_in(i)
    }

    fn pred(&self, cond: VarnodeId) -> (Pred, bool) {
        let data = self.data;
        let mut v = cond;
        let mut neg = false;
        for _ in 0..4 {
            let Some((_, o)) = def_op(data, v) else { break };
            let k = |i: int4| o.get_in(i).and_then(|x| constant(data, x));
            let (a, b) = (o.get_in(0), o.get_in(1));
            let sz = a.map_or(8, |a| size(data, a)).clamp(1, 8);
            let mask = if sz >= 8 { u64::MAX } else { (1u64 << (8 * sz)) - 1 };
            let atleast = |signed: bool, x: Option<VarnodeId>, t: u64, n: bool| x.map(|x| (Pred::AtLeast(signed, x, t & mask), n));
            let got = match o.code() {
                OpCode::CPUI_BOOL_NEGATE => {
                    neg = !neg;
                    v = o.get_in(0).unwrap_or(v);
                    continue;
                }
                code @ (OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESSEQUAL) => {
                    let signed = matches!(code, OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL);
                    let strict = matches!(code, OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_SLESS);
                    match (k(0), k(1)) {
                        (None, Some(c)) => atleast(signed, a, if strict { c } else { c.wrapping_add(1) }, true),
                        (Some(c), None) => atleast(signed, b, if strict { c.wrapping_add(1) } else { c }, false),
                        _ => None,
                    }
                }
                code @ (OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL) => {
                    let n = code == OpCode::CPUI_INT_NOTEQUAL;
                    match (k(0), k(1), a, b) {
                        (None, Some(c), Some(x), _) | (Some(c), None, _, Some(x)) => Some((Pred::EqConst(x, c), n)),
                        (None, None, Some(x), Some(y)) => Some((Pred::Eq(x, y), n)),
                        _ => None,
                    }
                }
                _ => None,
            };
            if let Some((p, n)) = got {
                return (p, n != neg);
            }
            break;
        }
        (Pred::Value(v), neg)
    }

    fn same_pred(&mut self, a: &Pred, b: &Pred) -> bool {
        match (a, b) {
            (Pred::AtLeast(s, x, t), Pred::AtLeast(s2, y, t2)) => s == s2 && t == t2 && self.eq(*x, *y, 0),
            (Pred::EqConst(x, c), Pred::EqConst(y, c2)) => c == c2 && self.eq(*x, *y, 0),
            (Pred::Eq(x, x2), Pred::Eq(y, y2)) => (self.eq(*x, *y, 0) && self.eq(*x2, *y2, 0)) || (self.eq(*x, *y2, 0) && self.eq(*x2, *y, 0)),
            (Pred::Value(x), Pred::Value(y)) => self.eq(*x, *y, 0),
            _ => false,
        }
    }

    /// Step past a segment that does nothing but hand on to one successor.
    fn advance(&self, p: Pos, vis: &mut HashSet<BlockId>) -> Pos {
        let mut p = p;
        for _ in 0..8 {
            if !self.phis(p.b).is_empty() {
                break;
            }
            let seg = self.segment(p.b);
            let Some(&last) = seg.last() else { break };
            let blk = self.data.bblocks_ref().block(last);
            if blk.size_out() != 1 || !self.effects(&seg).is_some_and(|e| e.is_empty()) {
                break;
            }
            vis.extend(seg.iter().copied());
            p = Pos { b: blk.get_out(0), from: last };
        }
        p
    }

    fn prove(&mut self, site: &Site) -> bool {
        let mut work = vec![(Pos { b: site.m, from: site.c }, Pos { b: site.r, from: site.c }, true)];
        let mut steps = 0;
        while let Some((m, r, first)) = work.pop() {
            steps += 1;
            if steps > 48 {
                return false;
            }
            let (m, r) = if m.b == r.b {
                (m, r)
            } else {
                let mut mv = std::mem::take(&mut self.m_vis);
                let m = self.advance(m, &mut mv);
                self.m_vis = mv;
                let mut rv = std::mem::take(&mut self.r_vis);
                let r = self.advance(r, &mut rv);
                self.r_vis = rv;
                (m, r)
            };
            if m.b == r.b {
                for phi in self.phis(m.b) {
                    let (Some(x), Some(y)) = (self.phi_in(phi, m.b, m.from), self.phi_in(phi, r.b, r.from)) else { return false };
                    if !self.eq(x, y, 0) {
                        return false;
                    }
                }
                continue;
            }
            let r_phis = self.phis(r.b);
            let revisit = !self.done.insert((m.b, r.b));
            self.m_vis.insert(m.b);
            self.r_vis.insert(r.b);
            for phi in self.phis(m.b) {
                let (Some(out), Some(x)) = (self.op(phi).and_then(|o| o.get_out()), self.phi_in(phi, m.b, m.from)) else { return false };
                if let Some(&target) = self.sigma.get(&out) {
                    let seg = self.segment(r.b);
                    if !self.eq_entry(x, target, r, &seg, 0) {
                        return false;
                    }
                    continue;
                }
                let mut target = None;
                for &p in &r_phis {
                    let (Some(pout), Some(y)) = (self.op(p).and_then(|o| o.get_out()), self.phi_in(p, r.b, r.from)) else { continue };
                    if self.phi_targets.contains(&pout) {
                        continue;
                    }
                    if self.eq(x, y, 0) {
                        target = Some(pout);
                        break;
                    }
                }
                let target = match target {
                    Some(t) => {
                        self.phi_targets.insert(t);
                        t
                    }
                    None if first => x,
                    None => match self.sigma.get(&x) {
                        Some(&t) => t,
                        None => {
                            if let Some(b) = self.block_of(x) {
                                self.shared.insert(b);
                            }
                            x
                        }
                    },
                };
                self.sigma.insert(out, target);
            }
            if revisit {
                continue;
            }
            let (sm, sr) = (self.segment(m.b), self.segment(r.b));
            self.m_vis.extend(sm.iter().copied());
            self.r_vis.extend(sr.iter().copied());
            let (Some(em), Some(er)) = (self.effects(&sm), self.effects(&sr)) else { return false };
            if em.len() != er.len() {
                return false;
            }
            for (&a, &b) in em.iter().zip(&er) {
                if !self.same_effect(a, b) {
                    return false;
                }
            }
            let (Some(&lm), Some(&lr)) = (sm.last(), sr.last()) else { return false };
            let g = self.data.bblocks_ref();
            let (bm, br) = (g.block(lm), g.block(lr));
            match (bm.size_out(), br.size_out()) {
                (0, 0) => {}
                (1, 1) => work.push((Pos { b: bm.get_out(0), from: lm }, Pos { b: br.get_out(0), from: lr }, false)),
                (2, 2) => {
                    let (Some((tm, fm)), Some((tr, fr))) = (taken(self.data, lm), taken(self.data, lr)) else { return false };
                    let cond = |b: BlockId| self.data.bb_op_tail(b).and_then(|t| self.op(t)).and_then(|o| o.get_in(1));
                    let (Some(cm), Some(cr)) = (cond(lm), cond(lr)) else { return false };
                    let ((pm, nm), (pr, nr)) = (self.pred(cm), self.pred(cr));
                    if !self.same_pred(&pm, &pr) {
                        return false;
                    }
                    let (tr, fr) = if nm == nr { (tr, fr) } else { (fr, tr) };
                    work.push((Pos { b: tm, from: lm }, Pos { b: tr, from: lr }, false));
                    work.push((Pos { b: fm, from: lm }, Pos { b: fr, from: lr }, false));
                }
                _ => return false,
            }
        }
        self.shared.iter().all(|b| !self.m_vis.contains(b) && !self.r_vis.contains(b))
    }

    fn same_effect(&mut self, a: OpId, b: OpId) -> bool {
        let data = self.data;
        let (Some(oa), Some(ob)) = (self.op(a), self.op(b)) else { return false };
        let (ra, rb) = (self.is_read(a), self.is_read(b));
        if ra || rb {
            if !(ra && rb) {
                return false;
            }
            let class = |op: OpId| self.cls.reads.get(&op).and_then(|r| r.off).and_then(|w| self.cls.webs[w].class);
            let (ca, cb) = (class(a), class(b));
            let (Some(xa), Some(xb)) = (oa.get_out(), ob.get_out()) else { return false };
            if size(data, xa) != size(data, xb) || (ca.is_some() && cb.is_some() && ca != cb) {
                return false;
            }
            self.m_reads.push(a);
            self.r_reads.push(b);
            self.ops.insert(a, b);
            self.sigma.insert(xa, xb);
            return true;
        }
        if oa.code() != ob.code() || oa.num_input() != ob.num_input() || oa.get_out().is_some() != ob.get_out().is_some() {
            return false;
        }
        for i in 0..oa.num_input() {
            let (Some(x), Some(y)) = (oa.get_in(i), ob.get_in(i)) else { return false };
            let plain = |v: VarnodeId| data.vbank().get(v).is_some_and(|n| !n.is_constant() && n.get_def().is_none() && !n.is_input());
            let ok = if i == 0 && matches!(oa.code(), OpCode::CPUI_CALL) || plain(x) && plain(y) {
                data.vbank().get(x).map(|n| (n.get_addr().clone(), n.get_size())) == data.vbank().get(y).map(|n| (n.get_addr().clone(), n.get_size()))
            } else {
                self.eq(x, y, 0)
            };
            if !ok {
                return false;
            }
        }
        if let (Some(x), Some(y)) = (oa.get_out(), ob.get_out()) {
            if size(data, x) != size(data, y) {
                return false;
            }
            self.sigma.insert(x, y);
        }
        self.ops.insert(a, b);
        true
    }
}

/// The blocks reachable from the entry once every folded site's edge to its
/// memory arm is gone.
fn reachable(data: &Funcdata, cut: &HashSet<(BlockId, BlockId)>) -> HashSet<BlockId> {
    let g = data.bblocks_ref();
    let mut seen = HashSet::new();
    if data.bblocks_get_size() == 0 {
        return seen;
    }
    let mut work = vec![data.bblocks_get_block(0)];
    while let Some(b) = work.pop() {
        if !seen.insert(b) {
            continue;
        }
        let blk = g.block(b);
        for i in 0..blk.size_out() {
            let s = blk.get_out(i);
            if !cut.contains(&(b, s)) {
                work.push(s);
            }
        }
    }
    seen
}

struct Plan {
    sites: Vec<(BlockId, BlockId)>,
    reads: Vec<(OpId, Option<VarnodeId>, bool)>,
    stores: Vec<OpId>,
    /// `(gp, fp)` named-argument counts of an SSA va_list.
    ssa: Option<(Option<u64>, Option<u64>)>,
}

fn find(data: &Funcdata, cls: &Cls) -> Option<Plan> {
    let n = cls.webs.len();
    let mut parent: Vec<usize> = (0..n).collect();
    fn root(p: &mut [usize], x: usize) -> usize {
        let mut x = x;
        while p[x] != x {
            p[x] = p[p[x]];
            x = p[x];
        }
        x
    }
    let union = |p: &mut Vec<usize>, a: usize, b: usize| {
        let (ra, rb) = (root(p, a), root(p, b));
        p[ra] = rb;
    };
    for r in cls.reads.values() {
        if let (Some(a), Some(b)) = (r.off, r.ovf) {
            union(&mut parent, a, b);
        }
    }
    let mut live: HashSet<usize> = (0..n).filter(|&w| !cls.webs[w].bad).collect();
    for _ in 0..4 {
        let fold: HashMap<BlockId, BlockId> = cls.sites.iter().filter(|s| live.contains(&s.web)).map(|s| (s.c, s.r)).collect();
        let mut p = parent.clone();
        let mut failed: HashSet<usize> = HashSet::new();
        let mut m_reads = Vec::new();
        for s in cls.sites.iter().filter(|s| live.contains(&s.web)) {
            let mut walk = Walk {
                data,
                cls,
                fold: &fold,
                live: &live,
                sigma: HashMap::new(),
                ops: HashMap::new(),
                m_vis: HashSet::new(),
                r_vis: HashSet::new(),
                shared: HashSet::new(),
                done: HashSet::new(),
                phi_targets: HashSet::new(),
                m_reads: Vec::new(),
                r_reads: Vec::new(),
            };
            if !walk.prove(s) {
                failed.insert(s.web);
                continue;
            }
            for &op in walk.m_reads.iter().chain(&walk.r_reads) {
                for w in cls.read_webs(op) {
                    union(&mut p, s.web, w);
                }
            }
            m_reads.extend(walk.m_reads);
        }
        let cut: HashSet<(BlockId, BlockId)> = fold.iter().filter_map(|(&c, _)| cls.sites.iter().find(|s| s.c == c).map(|s| (c, s.m))).collect();
        let reach = reachable(data, &cut);
        for (&op, r) in &cls.reads {
            let at = data.obank().get(op).and_then(|o| o.get_parent());
            if at.is_some_and(|b| reach.contains(&b)) && r.off.is_none() {
                failed.extend(cls.read_webs(op));
            }
        }
        for &op in &m_reads {
            if data.obank().get(op).and_then(|o| o.get_parent()).is_some_and(|b| reach.contains(&b)) {
                failed.extend(cls.read_webs(op));
            }
        }
        // Every web of a group with a failed web fails.
        let bad_roots: HashSet<usize> = (0..n).filter(|&w| failed.contains(&w) || !live.contains(&w)).map(|w| root(&mut p, w)).collect();
        let next: HashSet<usize> = live.iter().copied().filter(|&w| !bad_roots.contains(&root(&mut p, w))).collect();
        if next == live {
            if fold.is_empty() {
                return None;
            }
            let mut plan = Plan { sites: cut.into_iter().collect(), reads: Vec::new(), stores: Vec::new(), ssa: None };
            plan.sites.sort();
            let mut reads: Vec<(&OpId, &Read)> = cls.reads.iter().collect();
            reads.sort_by_key(|(op, _)| **op);
            for (&op, r) in reads {
                let at = data.obank().get(op).and_then(|o| o.get_parent());
                if !at.is_some_and(|b| reach.contains(&b)) || !cls.read_webs(op).all(|w| live.contains(&w)) {
                    continue;
                }
                let Some(w) = r.off else { continue };
                let web = &cls.webs[w];
                plan.reads.push((op, web.ap, web.class == Some(Class::Fp)));
                if web.ap.is_none() {
                    let (gp, fp) = plan.ssa.unwrap_or((None, None));
                    plan.ssa = Some(match web.class {
                        Some(Class::Fp) => (gp, web.init.or(fp)),
                        _ => (web.init.or(gp), fp),
                    });
                }
            }
            let mut stores: Vec<OpId> = cls.stores.iter().filter(|(_, w)| live.contains(w)).map(|(op, _)| *op).collect();
            stores.sort();
            plan.stores = stores;
            return (!plan.reads.is_empty()).then_some(plan);
        }
        live = next;
        if live.is_empty() {
            return None;
        }
    }
    None
}

/// Destroy `store` and the INDIRECTs it left.
fn destroy_store(data: &mut Funcdata, store: OpId) -> kuna_base::error::KunaResult<()> {
    let Some(b) = data.obank().get(store).and_then(|o| o.get_parent()) else { return Ok(()) };
    for op in data.bb_ops(b) {
        let made = data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_INDIRECT).and_then(|o| o.get_in(1)).and_then(|iop| data.vbank().get(iop)).is_some_and(|v| {
            use slotmap::KeyData;
            OpId::from(KeyData::from_ffi(v.get_offset())) == store
        });
        if made {
            if let Some((out, input)) = data.obank().get(op).and_then(|o| Some((o.get_out()?, o.get_in(0)?))) {
                data.total_replace(out, input)?;
            }
            data.op_destroy(op);
        }
    }
    data.op_destroy(store);
    Ok(())
}

/// The named integer arguments when no read says: the leading argument
/// registers the function reads other than to spill them to the stack.
fn named_registers(data: &Funcdata) -> u64 {
    use kuna_base::address::Address;
    let Some(lookup) = data.get_arch().manage().register_lookup() else { return 0 };
    let mut n = 0;
    for name in ["RDI", "RSI", "RDX", "RCX", "R8", "R9"] {
        let Some(st) = lookup.probe_register(name) else { break };
        let Some(space) = st.space.clone() else { break };
        let addr = Address::new(space, st.offset);
        let Some(v) = [8, 4].into_iter().find_map(|size| data.vbank().find_input(size, &addr)) else { break };
        let spill = |op: &OpId| {
            data.obank().get(*op).is_some_and(|o| {
                o.code() == OpCode::CPUI_COPY && o.get_out().and_then(|w| data.vbank().get(w)).is_some_and(|w| w.get_addr().get_space().is_some_and(|s| Some(s.get_index()) == data.get_arch().manage().get_stack_space().map(|t| t.get_index())))
            })
        };
        if data.descend_snapshot(v).iter().all(spill) {
            break;
        }
        n += 1;
    }
    n
}

fn has_va_start(data: &Funcdata) -> bool {
    data.obank().iter_alive().any(|op| {
        data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_CALLOTHER && o.get_in(0).and_then(|v| constant(data, v)) == Some(crate::userop::BUILTIN_VA_START as u64))
    })
}

fn rewrite(data: &mut Funcdata, plan: &Plan) -> kuna_base::error::KunaResult<()> {
    use kuna_base::error::KunaError;
    let id = crate::userop::BUILTIN_VAARG as uintb;
    for &(op, ap, sse) in &plan.reads {
        let ap = match ap {
            Some(ap) => ap,
            None => data.new_constant(8, 0),
        };
        let idv = data.new_constant(4, id);
        let class = data.new_constant(4, sse as uintb);
        data.op_set_all_input(op, &[idv, ap, class])?;
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_CALLOTHER));
    }
    for &s in &plan.stores {
        if data.obank().get(s).is_some_and(|o| o.get_parent().is_some()) {
            destroy_store(data, s)?;
        }
    }
    for &(c, m) in &plan.sites {
        let i = data.bblocks_ref().block(c).get_out_index(m);
        if i < 0 {
            return Err(KunaError::lowlevel("vaarg: stale arm"));
        }
        data.remove_branch(c, i)?;
    }
    data.remove_unreachable_blocks(false, true)?;
    if let Some((gp, fp)) = plan.ssa {
        if !has_va_start(data) && data.bblocks_get_size() > 0 {
            let entry = data.bblocks_get_block(0);
            let at = data.bb_op_head(entry).and_then(|o| data.obank().get(o)).map(|o| o.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("vaarg: empty entry"))?;
            let fp = fp.unwrap_or(48);
            let gp = gp.unwrap_or_else(|| 8 * named_registers(data));
            let last = crate::kuna_vastart::last_named(data, (gp / 8 + (fp - 48) / 16) as int4, (gp / 8) as int4);
            let call = data.new_op(if last.is_some() { 3 } else { 2 }, at);
            data.op_set_opcode_code(call, OpCode::CPUI_CALLOTHER);
            let idv = data.new_constant(4, crate::userop::BUILTIN_VA_START as uintb);
            data.op_set_input(call, idv, 0)?;
            let ap = data.new_constant(8, 0);
            data.op_set_input(call, ap, 1)?;
            if let Some(last) = last {
                data.op_set_input(call, last, 2)?;
            }
            data.op_insert_begin(call, entry);
        }
        if !data.get_func_proto().is_input_locked() {
            data.get_func_proto_mut().set_dotdotdot(true);
        }
    }
    Ok(())
}


/// Fold the `va_arg` expansion into one call (see the module docs).
pub struct ActionVaArg {
    base: ActionBase,
}

impl ActionVaArg {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionVaArg { base: ActionBase::new(0, "vaarg", g) })
    }
}

impl Action for ActionVaArg {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionVaArg { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().va_arg {
            return 0;
        }
        let cls = Cls::classify(data);
        if cls.sites.is_empty() {
            return 0;
        }
        let Some(plan) = find(data, &cls) else { return 0 };
        if rewrite(data, &plan).is_err() {
            return 0;
        }
        self.base.count += 1;
        1
    }
}
