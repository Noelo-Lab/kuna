//! (kuna `floatglobals`) Which globals the program only ever moves through
//! floating-point registers: the whole-program scan behind
//! `kuna_decomp::kuna_floatglobals`.
//!
//! Every function of the inventory (and every function it calls) is walked
//! from its entry, switch tables included, and its registers are followed as
//! constants through the function's control flow, the way the decompiler's own
//! constant propagation follows them: a value is known at an instruction only
//! when every path into it agrees.  That is what makes an access the
//! decompiler resolves to a global's address one this scan sees too, however
//! the address is formed -- a RIP-relative operand, a literal-pool word
//! (`ldr r3, =gd; vstr d0, [r3]`), `adrp`/`add`, `movw`/`movt`, `auipc`, a
//! GOT slot, a tracked register the loader seeds at the entry (`t9` on MIPS).
//!
//! Each load and store whose address is known is filed against that address
//! with its width and its class: a float access when the value goes to or
//! comes from a floating-point register (`movsd`, `vldr`, `ldr d0`, `flw`) or
//! a float operation (`addsd [g]`, `cvtss2sd`, `fld`); an integer access for
//! everything else, an immediate stored there included (`movl $0, g`).  Stack
//! slots are followed like registers, so an address spilled and reloaded is
//! still known; an address stored where the walk has lost the stack pointer
//! may come back unseen, so every global within [`ESCAPE_REACH`] bytes of it
//! is left undecided.
//!
//! A global in writable data is a float of width `w` when every access to it
//! is a float access of exactly `w` bytes (4 or 8), no access of another width
//! overlaps it, and its address never escapes.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::kuna_floatglobals::{FloatGlobals, FloatScan};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::loadimage::section_flags;
use kuna_sleigh::translate::Translate;

use super::classify::classify;
use super::kuna_poolref::PoolImage;
use super::kuna_switchtable;
use super::model::RawOp;
use super::xrefs::{decode, in_range, range_union, FullCapture, FullOp};

/// How far past an address the program stores to memory a load or store
/// through a copy of it may reach: the globals in that window stay undecided.
const ESCAPE_REACH: u64 = 256;
/// The most instructions one function's walk decodes.
const MAX_FUNCTION_INSNS: usize = 50_000;
/// The most register values one program point carries.
const MAX_TRACKED: usize = 64;
/// The widest access that may overlap a global from below.
const MAX_ACCESS: u64 = 64;

/// The globals of the loaded program that it only moves through float
/// registers.  Empty for a processor family whose global addressing the scan
/// does not model (i386 and SPARC position-independent code reach globals
/// through a register a thunk call sets).
pub fn scan(arch: &Architecture, input: &FloatScan) -> FloatGlobals {
    let family = arch.archid.split(':').collect::<Vec<_>>();
    let wide = family.get(2).copied();
    let supported = match family.first().copied() {
        Some("x86") => wide == Some("64"),
        Some("ARM" | "AARCH64" | "RISCV" | "MIPS" | "PowerPC") => true,
        _ => false,
    };
    if !supported {
        return FloatGlobals::new();
    }
    let manage = arch.manage();
    let (Some(code), Some(data)) = (manage.get_default_code_space().cloned(), manage.get_default_data_space().cloned())
    else {
        return FloatGlobals::new();
    };
    let translate: &dyn Translate = arch.translate();
    let floats = FloatRegisters::new(translate, family[0]);
    if floats.ranges.is_empty() {
        return FloatGlobals::new();
    }
    let image = Image::new(arch, &data, &input.sections, translate.is_big_endian());
    if image.exec.is_empty() {
        return FloatGlobals::new();
    }
    let ptr = code.get_addr_size();
    let pool = PoolImage::from_ranges(
        image.sections.iter().filter(|s| s.flags & (section_flags::READONLY | section_flags::CODE) != 0)
            .filter_map(|s| s.bytes.as_deref().map(|b| (s.lo, s.lo + b.len() as u64, b))).collect(),
        ptr,
        !translate.is_big_endian(),
    );
    let clobbered = clobbered_registers(arch);
    let sp = manage
        .get_stack_space()
        .and_then(|s| s.get_spacebase(0).ok())
        .map(|b| (b.offset, b.size));
    let extrapop = arch
        .default_fp()
        .map(|m| m.get_extra_pop())
        .filter(|&p| p != kuna_decomp::fspec::EXTRAPOP_UNKNOWN)
        .map(i64::from);
    let ctx = Ctx {
        arch,
        translate,
        code: &code,
        data: &data,
        image: &image,
        pool: pool.as_ref(),
        floats: &floats,
        ptr,
        clobbered,
        sp,
        extrapop,
        thumb_bit: family[0] == "ARM",
        max_entries: match arch.max_jumptable_size {
            0 => kuna_switchtable::DEFAULT_MAX_ENTRIES,
            n => n as usize,
        },
    };

    let entries: BTreeSet<u64> = input.seeds.iter().copied().filter(|&e| in_range(&image.exec, e)).collect();
    let mut queue: VecDeque<u64> = entries.iter().copied().collect();
    let mut walked: HashSet<u64> = HashSet::new();
    let mut cap = FullCapture::default();
    let mut evidence = Evidence::default();
    while let Some(entry) = queue.pop_front() {
        if !walked.insert(entry) {
            continue;
        }
        let mut body = Body::new(entry);
        body.extend(&ctx, &entries, &mut cap, vec![entry]);
        let mut at = body.solve(&ctx);
        for _ in 0..4 {
            let found = body.reopen(&ctx, &at);
            if found.is_empty() {
                break;
            }
            body.extend(&ctx, &entries, &mut cap, found);
            at = body.solve(&ctx);
        }
        body.file(&ctx, &at, &mut evidence);
        for &callee in &body.callees {
            if !walked.contains(&callee) && in_range(&image.exec, callee) {
                queue.push_back(callee);
            }
        }
    }
    evidence.decide(&image)
}

/// The register-space ranges that hold floating-point values, by the names the
/// processor's own SLEIGH specification gives them.
struct FloatRegisters {
    space: Option<Rc<AddrSpace>>,
    ranges: Vec<(u64, u64)>,
}

impl FloatRegisters {
    fn new(translate: &dyn Translate, family: &str) -> FloatRegisters {
        let numbered = |name: &str, prefixes: &[&str]| {
            prefixes.iter().any(|p| {
                name.strip_prefix(p).is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()))
            })
        };
        let is_float = |name: &str| match family {
            "x86" => {
                ["XMM", "YMM", "ZMM"].iter().any(|p| {
                    name.strip_prefix(p).is_some_and(|rest| rest.bytes().next().is_some_and(|b| b.is_ascii_digit()))
                }) || numbered(name, &["ST"])
            }
            "ARM" => numbered(name, &["s", "d", "q"]),
            "AARCH64" => numbered(name, &["b", "h", "s", "d", "q", "v", "z"]),
            "RISCV" => numbered(name, &["f", "ft", "fs", "fa"]),
            "MIPS" => numbered(name, &["f"]),
            "PowerPC" => numbered(name, &["f", "fp", "vs", "vr"]),
            _ => false,
        };
        let mut all = BTreeMap::new();
        translate.get_all_registers(&mut all);
        let mut space = None;
        let mut ranges = Vec::new();
        for (vn, name) in &all {
            if !is_float(name) {
                continue;
            }
            if space.is_none() {
                space = vn.space.clone();
            }
            if vn.space.as_ref().zip(space.as_ref()).is_some_and(|(a, b)| Rc::ptr_eq(a, b)) {
                ranges.push((vn.offset, vn.offset + vn.size as u64));
            }
        }
        FloatRegisters { space, ranges: range_union(ranges) }
    }

    fn contains(&self, vn: &VarnodeData) -> bool {
        let same = vn.space.as_ref().zip(self.space.as_ref()).is_some_and(|(a, b)| Rc::ptr_eq(a, b));
        same && {
            let n = self.ranges.partition_point(|&(lo, _)| lo <= vn.offset);
            n > 0 && vn.offset + vn.size as u64 <= self.ranges[n - 1].1
        }
    }
}

/// The registers a call may change: every storage location the default
/// convention passes a parameter or returns a value in.
fn clobbered_registers(arch: &Architecture) -> Vec<(u64, u64)> {
    let Some(model) = arch.default_fp() else { return Vec::new() };
    let mut out = Vec::new();
    for list in [model.input_opt(), model.output_list()].into_iter().flatten() {
        for e in list.get_entry() {
            if e.get_space().get_type() == spacetype::IPTR_PROCESSOR && e.get_size() > 0 {
                out.push((e.get_base(), e.get_base() + e.get_size() as u64));
            }
        }
    }
    range_union(out)
}

struct Section {
    lo: u64,
    hi: u64,
    flags: u32,
    bytes: Option<Vec<u8>>,
}

/// The loaded image as the scan reads it: which addresses are code, data and
/// writable data, and the bytes behind them.
struct Image {
    sections: Vec<Section>,
    exec: Vec<(u64, u64)>,
    big_endian: bool,
}

impl Image {
    fn new(arch: &Architecture, data: &Rc<AddrSpace>, rows: &[(u64, u64, u32)], big_endian: bool) -> Image {
        let loader = arch.translate().loader_rc();
        let mut sections = Vec::new();
        for &(lo, size, flags) in rows {
            if flags & section_flags::UNALLOC != 0 || size == 0 {
                continue;
            }
            let bytes = if flags & section_flags::NOLOAD != 0 || size > (1 << 28) {
                None
            } else {
                let mut buf = vec![0u8; size as usize];
                let read = loader.try_borrow_mut().ok().map(|mut l| l.load_fill(&mut buf, &Address::new(Rc::clone(data), lo)));
                matches!(read, Some(Ok(()))).then_some(buf)
            };
            sections.push(Section { lo, hi: lo.saturating_add(size), flags, bytes });
        }
        sections.sort_by_key(|s| s.lo);
        let exec = range_union(
            sections.iter().filter(|s| s.flags & section_flags::CODE != 0).map(|s| (s.lo, s.hi)).collect(),
        );
        Image { sections, exec, big_endian }
    }

    fn section(&self, at: u64) -> Option<&Section> {
        let n = self.sections.partition_point(|s| s.lo <= at);
        self.sections[..n].iter().rev().find(|s| at < s.hi)
    }

    /// Is `at` an address in data the program maps (not code)?
    fn is_data(&self, at: u64) -> bool {
        self.section(at).is_some_and(|s| s.flags & section_flags::CODE == 0)
    }

    fn is_writable_data(&self, at: u64) -> bool {
        self.section(at).is_some_and(|s| s.flags & (section_flags::CODE | section_flags::READONLY) == 0)
    }

    /// The constant a `size`-byte read of `at` yields: anything read-only, or
    /// a pointer-sized word of writable data that holds a data address.
    fn fold(&self, at: u64, size: u32, ptr: u32) -> Option<u64> {
        let s = self.section(at)?;
        let bytes = s.bytes.as_deref()?;
        if size == 0 || size > 8 {
            return None;
        }
        let off = (at - s.lo) as usize;
        let raw = bytes.get(off..off + size as usize)?;
        let mut v = 0u64;
        for i in 0..size as usize {
            let b = if self.big_endian { raw[i] } else { raw[size as usize - 1 - i] };
            v = (v << 8) | u64::from(b);
        }
        let constant = s.flags & (section_flags::READONLY | section_flags::CODE) != 0;
        (constant || (size == ptr && self.is_data(v))).then_some(v)
    }
}

struct Ctx<'a> {
    arch: &'a Architecture,
    translate: &'a dyn Translate,
    code: &'a Rc<AddrSpace>,
    data: &'a Rc<AddrSpace>,
    image: &'a Image,
    pool: Option<&'a PoolImage<'a>>,
    floats: &'a FloatRegisters,
    ptr: u32,
    clobbered: Vec<(u64, u64)>,
    sp: Option<(u64, u32)>,
    extrapop: Option<i64>,
    thumb_bit: bool,
    max_entries: usize,
}

struct Insn {
    vma: u64,
    len: u32,
    ops: Vec<FullOp>,
    is_call: bool,
    targets: Vec<u64>,
    succ: Vec<usize>,
}

/// One function's instructions, from its entry to every return, with their
/// successors inside the function.
struct Body {
    entry: u64,
    insns: Vec<Insn>,
    index: HashMap<u64, usize>,
    decoded: HashSet<u64>,
    /// Computed jumps whose table is not yet read, by instruction index.
    open: Vec<usize>,
    callees: Vec<u64>,
}

impl Body {
    fn new(entry: u64) -> Body {
        Body { entry, insns: Vec::new(), index: HashMap::new(), decoded: HashSet::new(), open: Vec::new(), callees: Vec::new() }
    }

    /// Decode everything reachable from `start` that is not decoded yet.
    fn extend(&mut self, ctx: &Ctx<'_>, entries: &BTreeSet<u64>, cap: &mut FullCapture, start: Vec<u64>) {
        let mut pending = VecDeque::from(start);
        while let Some(vma) = pending.pop_front() {
            if self.index.contains_key(&vma) || self.insns.len() >= MAX_FUNCTION_INSNS {
                continue;
            }
            if (vma != self.entry && entries.contains(&vma)) || !in_range(&ctx.image.exec, vma) {
                continue;
            }
            let Some(len) = decode(ctx.translate, vma, ctx.code, cap) else { continue };
            let raw: Vec<RawOp> =
                cap.ops().iter().map(|op| RawOp { opcode: op.opcode, in0: op.ins.first().cloned() }).collect();
            let c = classify(&raw, vma, len);
            let mut next = Vec::new();
            let mut halts = false;
            if c.flow.is_call {
                for &t in &c.flows {
                    self.callees.push(t);
                    halts |= ctx.arch.symboltab.function_is_no_return_across_scopes(&Address::new(Rc::clone(ctx.code), t));
                }
            } else {
                next.extend(c.flows.iter().copied());
            }
            if let Some(f) = c.fall_through.filter(|_| !halts) {
                next.push(f);
            }
            let dispatch = c.flows.is_empty() && c.flow.is_computed && c.flow.is_jump && !c.flow.is_call;
            if dispatch {
                let mut cases = ctx
                    .pool
                    .map(|p| switch_targets(ctx, p, &self.decoded, vma, cap.ops(), ctx.max_entries))
                    .unwrap_or_default();
                if cases.is_empty() {
                    let ops = cap.ops().to_vec();
                    cases = emulated_switch(ctx, vma, &ops, self, None);
                }
                if ctx.thumb_bit {
                    for case in &mut cases {
                        *case &= !1;
                    }
                }
                if cases.is_empty() {
                    self.open.push(self.insns.len());
                }
                next.extend(cases);
            }
            self.decoded.insert(vma);
            self.index.insert(vma, self.insns.len());
            self.insns.push(Insn { vma, len, ops: cap.ops().to_vec(), is_call: c.flow.is_call, targets: next.clone(), succ: Vec::new() });
            pending.extend(next);
        }
        for i in 0..self.insns.len() {
            let mut succ: Vec<usize> = self.insns[i].targets.iter().filter_map(|t| self.index.get(t).copied()).collect();
            succ.sort_unstable();
            succ.dedup();
            self.insns[i].succ = succ;
        }
    }

    /// Follow the function's registers as constants to a fixed point: the state
    /// each instruction is reached in.
    fn solve(&self, ctx: &Ctx<'_>) -> Vec<Option<State>> {
        let mut at: Vec<Option<State>> = vec![None; self.insns.len()];
        if self.insns.is_empty() {
            return at;
        }
        at[0] = Some(entry_state(ctx, self.entry));
        let mut queued = vec![false; self.insns.len()];
        let mut work = VecDeque::from([0usize]);
        queued[0] = true;
        while let Some(i) = work.pop_front() {
            queued[i] = false;
            let Some(state) = at[i].clone() else { continue };
            let out = step(ctx, &self.insns[i], state, None);
            for &j in &self.insns[i].succ {
                let changed = match &mut at[j] {
                    slot @ None => {
                        *slot = Some(out.clone());
                        true
                    }
                    Some(s) => s.meet(&out),
                };
                if changed && !queued[j] {
                    queued[j] = true;
                    work.push_back(j);
                }
            }
        }
        at
    }

    /// Read the tables of the computed jumps `extend` could not, with what the
    /// function's constants say each one is reached with: a table base set up
    /// before a loop that dispatches on every iteration.  The case bodies found.
    fn reopen(&mut self, ctx: &Ctx<'_>, at: &[Option<State>]) -> Vec<u64> {
        let mut found = Vec::new();
        let open = std::mem::take(&mut self.open);
        for i in open {
            let cases = match &at[i] {
                Some(_) => emulated_switch(ctx, self.insns[i].vma, &self.insns[i].ops, self, Some(at)),
                None => Vec::new(),
            };
            if cases.is_empty() {
                self.open.push(i);
                continue;
            }
            self.insns[i].targets.extend(cases.iter().copied());
            found.extend(cases);
        }
        found
    }

    /// File every access and escape against the state each instruction is
    /// reached in.
    fn file(&self, ctx: &Ctx<'_>, at: &[Option<State>], evidence: &mut Evidence) {
        for (i, insn) in self.insns.iter().enumerate() {
            if let Some(state) = at[i].clone() {
                step(ctx, insn, state, Some(evidence));
            }
        }
    }
}

/// The case bodies of the computed jump at `vma`, read out of its table.
fn switch_targets(
    ctx: &Ctx<'_>,
    pool: &PoolImage<'_>,
    decoded: &HashSet<u64>,
    vma: u64,
    ops: &[FullOp],
    max_entries: usize,
) -> Vec<u64> {
    for op in ops {
        for vn in &op.ins {
            let constant = vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT);
            if constant && ctx.image.section(vn.offset).is_some() {
                let cases = kuna_switchtable::targets(vn.offset, vma, pool, &ctx.image.exec, None, max_entries);
                if !cases.is_empty() {
                    return cases;
                }
            }
        }
    }
    let Some(branch) = ops.iter().find(|o| o.opcode == OpCode::CPUI_BRANCHIND).and_then(|o| o.ins.first()) else {
        return Vec::new();
    };
    kuna_switchtable::register_targets(
        ctx.translate,
        ctx.code,
        Some(ctx.data),
        decoded,
        vma,
        branch,
        pool,
        &ctx.image.exec,
        max_entries,
    )
    .0
}

/// The case bodies of a computed jump through a table: `tbb [pc, r3]`,
/// `adr r2, table; ldr pc, [r2, r3, lsl #2]`, `movslq (%rbp,%rax,4),%rax; add
/// %rbp,%rax; jmp *%rax`.  The range check just ahead of the jump names the
/// index and bounds it; the instructions after the check are evaluated, with
/// what is known where they start, at every index the check admits.  `at` is
/// the function's solved constants, when there are any yet: a base set up
/// before a loop that dispatches on every iteration is known only there.
fn emulated_switch(ctx: &Ctx<'_>, vma: u64, ops: &[FullOp], body: &Body, at: Option<&[Option<State>]>) -> Vec<u64> {
    let mut lead: Vec<&Insn> = Vec::new();
    let mut cur = vma;
    for _ in 0..6 {
        let Some(prev) = (1..=16u64)
            .filter_map(|d| cur.checked_sub(d))
            .find_map(|a| body.index.get(&a).map(|&i| &body.insns[i]).filter(|p| p.vma + u64::from(p.len) == cur))
        else {
            break;
        };
        lead.push(prev);
        cur = prev.vma;
    }
    let Some((k, reg, limit)) = range_check(ctx, &lead).or_else(|| index_mask(ctx, &lead)) else {
        return Vec::new();
    };
    let Some(count) = usize::try_from(limit).ok().and_then(|l| l.checked_add(1)).filter(|&c| c <= ctx.max_entries) else {
        return Vec::new();
    };
    let after: Vec<&Insn> = lead[..k].iter().rev().copied().collect();
    let first = after.first().map_or(vma, |i| i.vma);
    let known = match at.and_then(|at| body.index.get(&first).and_then(|&i| at[i].as_ref())) {
        Some(state) => state.clone(),
        None => lead[k..].iter().rev().fold(State::default(), |state, prev| step(ctx, prev, state, None)),
    };
    let width = after
        .iter()
        .flat_map(|i| i.ops.iter())
        .chain(ops.iter())
        .flat_map(|op| op.ins.iter())
        .filter(|vn| vn.offset == reg && vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR && !Rc::ptr_eq(s, ctx.data)))
        .map(|vn| vn.size)
        .max();
    let Some(width) = width else { return Vec::new() };
    let Some(&section) = ctx.image.exec.iter().find(|&&(lo, hi)| vma >= lo && vma < hi) else { return Vec::new() };
    let mut out = Vec::new();
    for i in 0..count as u64 {
        let mut state = known.clone();
        state.set_reg(reg, width, Some(Val::Const(i)));
        for insn in &after {
            state = step(ctx, insn, state, None);
        }
        let Some(target) = jump_target(ctx, ops, state) else { return Vec::new() };
        let target = if ctx.thumb_bit { target & !1 } else { target };
        if !in_range(&[section], target) {
            return Vec::new();
        }
        out.push(target);
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The range check ahead of a dispatch: the comparison of a register with a
/// constant that feeds the nearest conditional branch in `lead` (nearest
/// first), as `(the branch's position in lead, register offset, bound)`.
fn range_check(ctx: &Ctx<'_>, lead: &[&Insn]) -> Option<(usize, u64, u64)> {
    let k = lead.iter().position(|i| i.ops.iter().any(|o| o.opcode == OpCode::CPUI_CBRANCH))?;
    for prev in lead[k..].iter().take(3) {
        for op in &prev.ops {
            if !matches!(
                op.opcode,
                OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL
            ) {
                continue;
            }
            let (Some(a), Some(b)) = (op.ins.first(), op.ins.get(1)) else { continue };
            let (reg, limit) = if is_constant(b) && !is_constant(a) {
                (a, b.offset)
            } else if is_constant(a) && !is_constant(b) {
                (b, a.offset)
            } else {
                continue;
            };
            let register =
                reg.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR && !Rc::ptr_eq(s, ctx.data));
            if register && limit > 0 {
                return Some((k, reg.offset, limit));
            }
        }
    }
    None
}

/// A dispatch index bounded by a mask instead of a check (`switch (k & 7)`):
/// the nearest instruction in `lead` that ands a register with `2^n - 1`, as
/// `(its position in lead, the register it writes, the mask)`.
fn index_mask(ctx: &Ctx<'_>, lead: &[&Insn]) -> Option<(usize, u64, u64)> {
    for (k, insn) in lead.iter().enumerate() {
        for (j, op) in insn.ops.iter().enumerate() {
            if op.opcode != OpCode::CPUI_INT_AND {
                continue;
            }
            let Some(mask) = op.ins.iter().find(|i| is_constant(i)).map(|c| c.offset) else { continue };
            if mask == 0 || mask.checked_add(1).is_none_or(|m| !m.is_power_of_two() || m as usize > ctx.max_entries) {
                continue;
            }
            let mut value = op.out.clone();
            for later in &insn.ops[j + 1..] {
                let Some(v) = value.as_ref() else { break };
                if matches!(later.opcode, OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT) && later.ins.first() == Some(v) {
                    value = later.out.clone();
                }
            }
            let register = value.as_ref().filter(|v| {
                v.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR && !Rc::ptr_eq(s, ctx.data))
            });
            if let Some(v) = register {
                return Some((k, v.offset, mask));
            }
        }
    }
    None
}

/// Where the computed jump among `ops` goes from `state`, when that is known.
fn jump_target(ctx: &Ctx<'_>, ops: &[FullOp], state: State) -> Option<u64> {
    let mut temps: Vec<Temp> = Vec::new();
    let mut state = state;
    for op in ops {
        let value = |vn: &VarnodeData, state: &State, temps: &[Temp]| value(ctx, vn, state, temps);
        if op.opcode == OpCode::CPUI_BRANCHIND {
            return match op.ins.first().and_then(|t| value(t, &state, &temps)) {
                Some(Val::Const(t)) => Some(t),
                _ => None,
            };
        }
        let result = match op.opcode {
            OpCode::CPUI_LOAD => {
                let size = op.out.as_ref().map_or(0, |o| o.size);
                match op.ins.get(1).and_then(|p| value(p, &state, &temps)) {
                    Some(Val::Const(a)) => ctx.image.fold(a, size, ctx.ptr).map(Val::Const),
                    _ => None,
                }
            }
            OpCode::CPUI_STORE => None,
            _ => {
                let ins: Vec<Option<Val>> = op.ins.iter().map(|vn| value(vn, &state, &temps)).collect();
                evaluate(op, &ins)
            }
        };
        let Some(out) = &op.out else { continue };
        let Some(space) = out.space.as_ref() else { continue };
        if space.get_type() == spacetype::IPTR_INTERNAL {
            temps.retain(|t| !(t.0 == out.offset && t.1 == out.size));
            temps.push((out.offset, out.size, result));
        } else if space.get_type() == spacetype::IPTR_PROCESSOR && !Rc::ptr_eq(space, ctx.code) {
            state.set_reg(out.offset, out.size, result);
        }
    }
    None
}

/// The registers a function's entry is known to hold: the stack pointer, and
/// the tracked values the loader and the processor specification seed there.
fn entry_state(ctx: &Ctx<'_>, entry: u64) -> State {
    let at = Address::new(Rc::clone(ctx.code), entry);
    let mut state = State::default();
    if let Some((off, size)) = ctx.sp {
        state.set_reg(off, size, Some(Val::Stack(0)));
    }
    let mut seed = |loc: &VarnodeData, val: u64| {
        if loc.space.as_ref().is_some_and(|s| !Rc::ptr_eq(s, ctx.data) && s.get_type() == spacetype::IPTR_PROCESSOR) {
            state.set_reg(loc.offset, loc.size, Some(Val::Const(val)));
        }
    };
    let tracked = ctx.arch.with_context_db_mut(|db| db.get_tracked_set(&at).clone());
    for t in &tracked {
        seed(&t.loc, t.val);
    }
    if let Some(extra) = ctx.arch.loader_entry_tracks.get(&at) {
        for t in extra {
            seed(&t.loc, t.val);
        }
    }
    state
}

/// A value the scan knows: a constant, or the entry stack pointer plus an
/// offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Val {
    Const(u64),
    Stack(i64),
}

/// What is known at one program point: register values as `(offset, size,
/// value)` in the register space, and stack slots as `(offset from the entry
/// stack pointer, size, value)`, each sorted by offset.
#[derive(Clone, Default, PartialEq)]
struct State {
    regs: Vec<(u64, u32, Val)>,
    slots: Vec<(i64, u32, Val)>,
}

impl State {
    fn get_reg(&self, off: u64, size: u32, big_endian: bool) -> Option<Val> {
        self.regs.iter().find_map(|&(o, s, v)| {
            if o == off && s == size {
                return Some(v);
            }
            let Val::Const(c) = v else { return None };
            if off < o || off + u64::from(size) > o + u64::from(s) {
                return None;
            }
            let shift = if big_endian { o + u64::from(s) - (off + u64::from(size)) } else { off - o };
            Some(Val::Const(mask(c.checked_shr((shift * 8) as u32).unwrap_or(0), size)))
        })
    }

    /// Set the register at `off`, answering the constant the state could not
    /// keep: past [`MAX_TRACKED`] values, or wider than eight bytes.
    fn set_reg(&mut self, off: u64, size: u32, val: Option<Val>) -> Option<u64> {
        let end = off + u64::from(size);
        self.regs.retain(|&(o, s, _)| o + u64::from(s) <= off || end <= o);
        let v = match val? {
            Val::Const(c) => Val::Const(mask(c, size)),
            stack => stack,
        };
        if self.regs.len() >= MAX_TRACKED || size > 8 {
            return if let Val::Const(c) = v { Some(c) } else { None };
        }
        let at = self.regs.partition_point(|&(o, _, _)| o < off);
        self.regs.insert(at, (off, size, v));
        None
    }

    fn get_slot(&self, off: i64, size: u32, big_endian: bool) -> Option<Val> {
        self.slots.iter().find_map(|&(o, s, v)| {
            if o == off && s == size {
                return Some(v);
            }
            let Val::Const(c) = v else { return None };
            if off < o || off + i64::from(size) > o + i64::from(s) {
                return None;
            }
            let shift = if big_endian { (o + i64::from(s) - (off + i64::from(size))) as u64 } else { (off - o) as u64 };
            Some(Val::Const(mask(c.checked_shr((shift * 8) as u32).unwrap_or(0), size)))
        })
    }

    /// Set the stack slot at `off`, answering the constant the state could not
    /// keep, as [`State::set_reg`] does.
    fn set_slot(&mut self, off: i64, size: u32, val: Option<Val>) -> Option<u64> {
        let end = off + i64::from(size);
        self.slots.retain(|&(o, s, _)| o + i64::from(s) <= off || end <= o);
        let v = val?;
        if self.slots.len() >= MAX_TRACKED || size > 8 {
            return if let Val::Const(c) = v { Some(c) } else { None };
        }
        let at = self.slots.partition_point(|&(o, _, _)| o < off);
        self.slots.insert(at, (off, size, v));
        None
    }

    fn kill(&mut self, ranges: &[(u64, u64)]) {
        self.regs.retain(|&(o, s, _)| !ranges.iter().any(|&(lo, hi)| o < hi && lo < o + u64::from(s)));
    }

    /// Keep only what `other` also knows; `true` when anything was dropped.
    fn meet(&mut self, other: &State) -> bool {
        let before = (self.regs.len(), self.slots.len());
        self.regs.retain(|e| other.regs.contains(e));
        self.slots.retain(|e| other.slots.contains(e));
        (self.regs.len(), self.slots.len()) != before
    }
}

fn mask(v: u64, size: u32) -> u64 {
    if size >= 8 { v } else { v & ((1u64 << (size * 8)) - 1) }
}

/// Every access and escape the scan has filed.
#[derive(Default)]
struct Evidence {
    /// Accesses by address: `(width, float)`.
    accesses: BTreeMap<u64, Vec<(u8, bool)>>,
    escapes: BTreeSet<u64>,
}

impl Evidence {
    fn access(&mut self, at: u64, size: u32, float: bool) {
        let width = size.min(255) as u8;
        let list = self.accesses.entry(at).or_default();
        if !list.contains(&(width, float)) {
            list.push((width, float));
        }
    }

    fn decide(&self, image: &Image) -> FloatGlobals {
        let mut out = FloatGlobals::new();
        for (&at, list) in &self.accesses {
            let width = list[0].0;
            if !matches!(width, 4 | 8) || !list.iter().all(|&(w, float)| float && w == width) || !image.is_writable_data(at) {
                continue;
            }
            let end = at + u64::from(width);
            let overlapped = self
                .accesses
                .range(at.saturating_sub(MAX_ACCESS)..end)
                .any(|(&b, l)| b != at && l.iter().any(|&(w, _)| b + u64::from(w) > at));
            let escaped = self.escapes.range(at.saturating_sub(ESCAPE_REACH - 1)..end).next().is_some();
            if !overlapped && !escaped {
                out.insert(at, width);
            }
        }
        out
    }
}

/// One instruction's effect on `state`, filing its accesses and escapes into
/// `evidence` when there is one.
fn step(ctx: &Ctx<'_>, insn: &Insn, mut state: State, mut evidence: Option<&mut Evidence>) -> State {
    let big = ctx.image.big_endian;
    let mut temps: Vec<Temp> = Vec::new();
    let is_data_space = |vn: &VarnodeData| vn.space.as_ref().is_some_and(|s| Rc::ptr_eq(s, ctx.data));
    let mut conditional = false;
    for (k, op) in insn.ops.iter().enumerate() {
        if op.opcode == OpCode::CPUI_CBRANCH && op.ins.first().is_some_and(is_constant) {
            conditional = true;
        }
        let value = |vn: &VarnodeData, state: &State, temps: &[Temp]| value(ctx, vn, state, temps);
        let result = match op.opcode {
            OpCode::CPUI_LOAD => {
                let size = op.out.as_ref().map_or(0, |o| o.size);
                match op.ins.get(1).and_then(|p| value(p, &state, &temps)) {
                    Some(Val::Const(a)) => {
                        if let Some(ev) = evidence.as_deref_mut().filter(|_| ctx.image.is_data(a)) {
                            ev.access(a, size, read_is_float(ctx, &insn.ops, k));
                        }
                        ctx.image.fold(a, size, ctx.ptr).map(Val::Const)
                    }
                    Some(Val::Stack(o)) => state.get_slot(o, size, big),
                    None => None,
                }
            }
            OpCode::CPUI_STORE => {
                if let Some(v) = op.ins.get(2) {
                    let stored = value(v, &state, &temps);
                    match op.ins.get(1).and_then(|p| value(p, &state, &temps)) {
                        Some(Val::Const(a)) => {
                            if let Some(ev) = evidence.as_deref_mut().filter(|_| ctx.image.is_data(a)) {
                                ev.access(a, v.size, from_float(ctx, &insn.ops, k, v, 0));
                            }
                        }
                        Some(Val::Stack(o)) => {
                            let kept = stored.filter(|&x| !conditional || state.get_slot(o, v.size, big) == Some(x));
                            if let Some(x) = state.set_slot(o, v.size, kept).filter(|&x| ctx.image.is_data(x)) {
                                if let Some(ev) = evidence.as_deref_mut() {
                                    ev.escapes.insert(x);
                                }
                            }
                        }
                        None => {
                            let lost = ctx.sp.is_some_and(|(off, size)| {
                                !matches!(state.get_reg(off, size, big), Some(Val::Stack(_)))
                            });
                            if let (Some(ev), Some(Val::Const(x))) = (evidence.as_deref_mut(), stored) {
                                if lost && v.size == ctx.ptr && ctx.image.is_data(x) {
                                    ev.escapes.insert(x);
                                }
                            }
                        }
                    }
                }
                None
            }
            _ => {
                if let Some(ev) = evidence.as_deref_mut() {
                    for vn in op.ins.iter().filter(|vn| is_data_space(vn) && ctx.image.is_data(vn.offset)) {
                        ev.access(vn.offset, vn.size, read_is_float(ctx, &insn.ops, k));
                    }
                }
                let ins: Vec<Option<Val>> = op.ins.iter().map(|vn| value(vn, &state, &temps)).collect();
                evaluate(op, &ins)
            }
        };
        let Some(out) = &op.out else { continue };
        let Some(space) = out.space.as_ref() else { continue };
        if is_data_space(out) {
            if let Some(ev) = evidence.as_deref_mut().filter(|_| ctx.image.is_data(out.offset)) {
                ev.access(out.offset, out.size, made_float(ctx, &insn.ops, k));
            }
        } else if space.get_type() == spacetype::IPTR_INTERNAL {
            temps.retain(|t| !(t.0 == out.offset && t.1 == out.size));
            temps.push((out.offset, out.size, result));
        } else if space.get_type() == spacetype::IPTR_PROCESSOR && !Rc::ptr_eq(space, ctx.code) {
            let kept = result.filter(|&x| !conditional || state.get_reg(out.offset, out.size, big) == Some(x));
            if let Some(x) = state.set_reg(out.offset, out.size, kept).filter(|&x| ctx.image.is_data(x)) {
                if let Some(ev) = evidence.as_deref_mut() {
                    ev.escapes.insert(x);
                }
            }
        }
    }
    if insn.is_call {
        state.kill(&ctx.clobbered);
        if let (Some((off, size)), Some(pop)) = (ctx.sp, ctx.extrapop) {
            if let Some(Val::Stack(o)) = state.get_reg(off, size, big) {
                state.set_reg(off, size, Some(Val::Stack(o + pop)));
            }
        }
    }
    state
}

/// A temporary one instruction's p-code defines: offset, size and value.
type Temp = (u64, u32, Option<Val>);

/// What `vn` holds, read with `state` and the instruction's temporaries.
fn value(ctx: &Ctx<'_>, vn: &VarnodeData, state: &State, temps: &[Temp]) -> Option<Val> {
    let space = vn.space.as_ref()?;
    match space.get_type() {
        spacetype::IPTR_CONSTANT => Some(Val::Const(mask(vn.offset, vn.size))),
        spacetype::IPTR_INTERNAL => temps.iter().rev().find(|t| t.0 == vn.offset && t.1 == vn.size).and_then(|t| t.2),
        spacetype::IPTR_PROCESSOR if Rc::ptr_eq(space, ctx.data) || Rc::ptr_eq(space, ctx.code) => {
            ctx.image.fold(vn.offset, vn.size, ctx.ptr).map(Val::Const)
        }
        spacetype::IPTR_PROCESSOR => state.get_reg(vn.offset, vn.size, ctx.image.big_endian),
        _ => None,
    }
}

/// The value `op` computes from known inputs: the integer operations constant
/// propagation folds, and an offset added to or taken from a stack address.
fn evaluate(op: &FullOp, ins: &[Option<Val>]) -> Option<Val> {
    let out = op.out.as_ref()?.size;
    if out > 8 {
        return None;
    }
    let signed = |v: u64, size: u32| -> i64 {
        if size == 0 || size >= 8 {
            v as i64
        } else {
            let sh = 64 - size * 8;
            ((v << sh) as i64) >> sh
        }
    };
    let first = ins.first().copied().flatten();
    let second = ins.get(1).copied().flatten();
    let stack = match (op.opcode, first, second) {
        (OpCode::CPUI_COPY, Some(Val::Stack(o)), _) => Some(o),
        (OpCode::CPUI_INT_ADD, Some(Val::Stack(o)), Some(Val::Const(c)))
        | (OpCode::CPUI_INT_ADD, Some(Val::Const(c)), Some(Val::Stack(o))) => Some(o.wrapping_add(signed(c, out))),
        (OpCode::CPUI_INT_SUB, Some(Val::Stack(o)), Some(Val::Const(c))) => Some(o.wrapping_sub(signed(c, out))),
        _ => None,
    };
    if let Some(o) = stack {
        return Some(Val::Stack(o));
    }
    let constant = |v: Option<Val>| match v {
        Some(Val::Const(c)) => Some(c),
        _ => None,
    };
    let a = constant(first);
    let b = constant(second);
    let in0 = op.ins.first().map_or(0, |v| v.size);
    let v = match op.opcode {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => a?,
        OpCode::CPUI_INT_SEXT => signed(a?, in0) as u64,
        OpCode::CPUI_INT_ADD => a?.wrapping_add(b?),
        OpCode::CPUI_INT_SUB => a?.wrapping_sub(b?),
        OpCode::CPUI_INT_MULT => a?.wrapping_mul(b?),
        OpCode::CPUI_INT_AND => a? & b?,
        OpCode::CPUI_INT_OR => a? | b?,
        OpCode::CPUI_INT_XOR => a? ^ b?,
        OpCode::CPUI_INT_LEFT => a?.checked_shl(u32::try_from(b?).ok()?).unwrap_or(0),
        OpCode::CPUI_INT_RIGHT => a?.checked_shr(u32::try_from(b?).ok()?).unwrap_or(0),
        OpCode::CPUI_INT_SRIGHT => {
            let n = u32::try_from(b?).ok()?.min(63);
            (signed(a?, in0) >> n) as u64
        }
        OpCode::CPUI_INT_NEGATE => !a?,
        OpCode::CPUI_INT_2COMP => a?.wrapping_neg(),
        OpCode::CPUI_SUBPIECE => a?.checked_shr(u32::try_from(b?).ok()?.checked_mul(8)?).unwrap_or(0),
        OpCode::CPUI_PIECE => {
            let low = op.ins.get(1).map_or(0, |v| v.size);
            a?.checked_shl(low * 8).unwrap_or(0) | b?
        }
        _ => return None,
    };
    Some(Val::Const(mask(v, out)))
}

/// Does the operation `code` read its inputs as floats?
fn reads_float(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
            | OpCode::CPUI_FLOAT_TRUNC
    )
}

/// Does the operation `code` produce a float?
fn makes_float(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_INT2FLOAT
            | OpCode::CPUI_FLOAT_FLOAT2FLOAT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
    )
}

/// Does the operation `code` only move or place bits, so the class of what it
/// writes is the class of what it reads?
fn moves_bits(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_AND
    )
}

/// Is the value op `k` reads out of memory (a LOAD's result, or a data-space
/// input of any other op) a float: read by a float operation, or moved whole
/// into a float register?
fn read_is_float(ctx: &Ctx<'_>, ops: &[FullOp], k: usize) -> bool {
    let op = &ops[k];
    if op.opcode == OpCode::CPUI_LOAD {
        return op.out.as_ref().is_some_and(|o| goes_float(ctx, ops, k + 1, o, 0));
    }
    if reads_float(op.opcode) {
        return true;
    }
    moves_bits(op.opcode) && op.out.as_ref().is_some_and(|o| goes_float(ctx, ops, k + 1, o, 0))
}

/// Does `v`, written just before op `from`, reach only float registers and
/// float operations?
fn goes_float(ctx: &Ctx<'_>, ops: &[FullOp], from: usize, v: &VarnodeData, depth: u32) -> bool {
    let Some(space) = v.space.as_ref() else { return false };
    if space.get_type() == spacetype::IPTR_PROCESSOR {
        return ctx.floats.contains(v);
    }
    if space.get_type() != spacetype::IPTR_INTERNAL || depth > 6 {
        return false;
    }
    let mut used = false;
    for (j, op) in ops.iter().enumerate().skip(from) {
        if op.ins.iter().any(|i| i == v) {
            used = true;
            let ok = reads_float(op.opcode)
                || (moves_bits(op.opcode) && op.out.as_ref().is_some_and(|o| goes_float(ctx, ops, j + 1, o, depth + 1)));
            if !ok {
                return false;
            }
        }
        if op.out.as_ref() == Some(v) {
            break;
        }
    }
    used
}

/// Is the value op `k` writes straight to memory a float?
fn made_float(ctx: &Ctx<'_>, ops: &[FullOp], k: usize) -> bool {
    let op = &ops[k];
    makes_float(op.opcode)
        || (moves_bits(op.opcode)
            && op.ins.iter().filter(|i| !is_constant(i)).all(|i| from_float(ctx, ops, k, i, 0))
            && op.ins.iter().any(|i| !is_constant(i)))
}

/// Does `v`, read by op `before`, come from a float register or a float
/// operation?
fn from_float(ctx: &Ctx<'_>, ops: &[FullOp], before: usize, v: &VarnodeData, depth: u32) -> bool {
    let Some(space) = v.space.as_ref() else { return false };
    if space.get_type() == spacetype::IPTR_PROCESSOR {
        return ctx.floats.contains(v);
    }
    if space.get_type() != spacetype::IPTR_INTERNAL || depth > 6 {
        return false;
    }
    ops[..before].iter().rposition(|o| o.out.as_ref() == Some(v)).is_some_and(|j| {
        let op = &ops[j];
        makes_float(op.opcode)
            || (moves_bits(op.opcode)
                && op.ins.iter().any(|i| !is_constant(i))
                && op.ins.iter().filter(|i| !is_constant(i)).all(|i| from_float(ctx, ops, j, i, depth + 1)))
    })
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
}
