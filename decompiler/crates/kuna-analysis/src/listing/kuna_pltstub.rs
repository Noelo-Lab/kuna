//! (kuna) `pltstub` — a PLT stub that builds its slot's address in a register
//! references the slot, not the partial sums on the way to it.
//!
//! GNU ld's A32 PLT entry adds the PC and two (three with `--long-plt`) rotated
//! immediates into `ip`, then jumps through the slot that sum names:
//!
//! ```text
//! 0x10310  add ip,pc,#0,12      ; ip = 0x10318          (adr r12,0x10318)
//! 0x10314  add ip,ip,#18,20     ; ip += 0x12000
//! 0x10318  ldr pc,[ip,#0xcf4]!  ; jump through the slot 0x2300c
//! ```
//!
//! Each step lifts to a constant the generic operand rule files when it lands in
//! a mapped section: the PC value (`0x10318`) and, once the GOT is far enough
//! from the stub, the immediate itself (`0x12000`). Neither is a reference; the
//! one address the stub uses is the slot.
//!
//! # The rule
//!
//! A chain is a straight run that starts with an instruction setting one
//! register to a constant, adds constants to that register in one or more
//! further instructions, and ends in an instruction that loads through the
//! register plus a constant and jumps to what it loaded (`BRANCHIND`). Its
//! instructions file no constant [`XrefKind::Data`] row; the last one files the
//! slot as a [`XrefKind::Read`]. The values are only `COPY`/`INT_ADD`/`INT_SUB`
//! of constants, so no other instruction shape matches, and a run that leaves
//! the shape at any step keeps the rows the generic rule gives it.
//!
//! Chains run only inside the image's linker stub sections (`.plt`, `.iplt` for
//! the IFUNC stubs of a static image, and the other names of
//! [`STUB_SECTIONS`]), where the linker puts its stubs; an image without section headers follows none, since
//! nothing says where its stubs are. Code elsewhere built the same way (a
//! hand-written `adr; add; ldr pc` veneer) keeps its rows.
//!
//! The chain is followed in the walk's own order: each instruction is the one
//! the walk decoded, in the mode it chose, and a chain waits for the walk to
//! decode the instruction at its fall-through. The constant rows of a chain
//! still open are held back, and filed as they were when it breaks or when the
//! body drains with it open.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use object::{Object, ObjectSection};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use super::xrefs::{in_range, FullOp, XrefKind};
use crate::analyzers::entry::kuna_fdeinterior::STUB_SECTIONS;

/// The most constant adds a chain may take before its load: one in the
/// standard entry, two with `--long-plt`.
const MAX_ADDS: usize = 2;

/// A constant row held back: `(from, to, instruction text)`.
pub(super) type Held = (u64, u64, String);

/// A chain the walk is following.
struct Chain {
    /// The register the chain builds the address in, and its value.
    reg: VarnodeData,
    value: u64,
    /// The constant adds taken so far.
    adds: usize,
    held: Vec<Held>,
}

/// The chains one body's walk has open, keyed by the VMA of the instruction
/// each expects next.
pub(super) struct PltStubs<'a> {
    data_space: Option<&'a Rc<AddrSpace>>,
    /// The mapped ranges a slot must lie in.
    mapped: &'a [(u64, u64)],
    /// The stub section ranges, sorted: the only place a chain runs.
    plt: Vec<(u64, u64)>,
    open: HashMap<u64, Chain>,
}

impl<'a> PltStubs<'a> {
    pub(super) fn new(data_space: Option<&'a Rc<AddrSpace>>, mapped: &'a [(u64, u64)], mut plt: Vec<(u64, u64)>) -> Self {
        plt.sort_unstable();
        PltStubs { data_space, mapped, plt, open: HashMap::new() }
    }

    /// The matcher for `file`, or `None` when it has no stub section: without
    /// section headers the stubs' extent is unknown, so no chain is followed
    /// and every row stays as the constant scan files it.
    pub(super) fn for_file(
        file: &object::File,
        data_space: Option<&'a Rc<AddrSpace>>,
        mapped: &'a [(u64, u64)],
    ) -> Option<Self> {
        let plt: Vec<(u64, u64)> = file
            .sections()
            .filter(|s| s.name().is_ok_and(|name| STUB_SECTIONS.contains(&name)) && s.size() > 0)
            .map(|s| (s.address(), s.address() + s.size()))
            .collect();
        (!plt.is_empty()).then(|| Self::new(data_space, mapped, plt))
    }

    /// Take the instruction at `vma` (`len` bytes, lifted to `ops`, rendered
    /// `text`) as the walk decodes it: continue the chain that expects it, or
    /// start one. A chain's constant rows leave `drefs` while it is open, and
    /// the load that ends it gets its slot as a `Read`. Returns the rows of
    /// the chains this instruction broke, to file as they were.
    pub(super) fn step(
        &mut self,
        vma: u64,
        len: u32,
        ops: &[FullOp],
        drefs: &mut Vec<(u64, XrefKind)>,
        text: &str,
    ) -> Vec<Held> {
        if !in_range(&self.plt, vma) {
            return self.open.remove(&vma).map(|chain| chain.held).unwrap_or_default();
        }
        let next = vma.wrapping_add(u64::from(len));
        let mut released = Vec::new();
        if let Some(mut chain) = self.open.remove(&vma) {
            let mut values = Values::holding(self.data_space, &chain);
            if chain.adds > 0 {
                if let Some(slot) = values.load_jump(ops).filter(|&slot| in_range(self.mapped, slot)) {
                    drefs.retain(|&(_, kind)| kind != XrefKind::Data);
                    if !drefs.contains(&(slot, XrefKind::Read)) {
                        drefs.push((slot, XrefKind::Read));
                    }
                    return released;
                }
            }
            if chain.adds < MAX_ADDS {
                if let Some(value) = values.add_to(ops, &chain.reg) {
                    chain.value = value;
                    chain.adds += 1;
                    hold(&mut chain.held, vma, drefs, text);
                    released.extend(self.open.insert(next, chain).into_iter().flat_map(|c| c.held));
                    return released;
                }
            }
            released = chain.held;
        }
        if let Some((reg, value)) = Values::new(self.data_space).single_write(ops) {
            let mut chain = Chain { reg, value, adds: 0, held: Vec::new() };
            hold(&mut chain.held, vma, drefs, text);
            released.extend(self.open.insert(next, chain).into_iter().flat_map(|c| c.held));
        }
        released
    }

    /// The rows of every chain still open when the body drains, to file as
    /// they were.
    pub(super) fn drain(&mut self) -> Vec<Held> {
        self.open.drain().flat_map(|(_, chain)| chain.held).collect()
    }
}

/// Move the constant rows of the instruction at `vma` out of `drefs`.
fn hold(held: &mut Vec<Held>, vma: u64, drefs: &mut Vec<(u64, XrefKind)>, text: &str) {
    drefs.retain(|&(to, kind)| {
        if kind == XrefKind::Data {
            held.push((vma, to, text.to_string()));
        }
        kind != XrefKind::Data
    });
}

/// Constant values of registers and temporaries in one instruction.
struct Values<'a> {
    data_space: Option<&'a Rc<AddrSpace>>,
    known: HashMap<(usize, u64, u32), u64>,
}

impl<'a> Values<'a> {
    fn new(data_space: Option<&'a Rc<AddrSpace>>) -> Self {
        Values { data_space, known: HashMap::new() }
    }

    /// The values at the start of an instruction a chain reaches: only its
    /// register's.
    fn holding(data_space: Option<&'a Rc<AddrSpace>>, chain: &Chain) -> Self {
        let mut values = Self::new(data_space);
        if let Some(key) = Self::key(&chain.reg) {
            values.known.insert(key, chain.value);
        }
        values
    }

    fn key(vn: &VarnodeData) -> Option<(usize, u64, u32)> {
        let space = vn.space.as_ref()?;
        Some((Rc::as_ptr(space) as usize, vn.offset, vn.size))
    }

    fn is_register(&self, vn: &VarnodeData) -> bool {
        let Some(space) = &vn.space else { return false };
        space.get_type() == spacetype::IPTR_PROCESSOR
            && !self.data_space.is_some_and(|data| Rc::ptr_eq(space, data))
    }

    fn value(&self, vn: &VarnodeData) -> Option<u64> {
        let space = vn.space.as_ref()?;
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Some(vn.offset);
        }
        self.known.get(&Self::key(vn)?).copied()
    }

    /// Evaluate one arithmetic op, recording its output.
    fn eval(&mut self, op: &FullOp) -> Option<()> {
        let out = op.out.as_ref()?;
        let mask = if out.size >= 8 { u64::MAX } else { (1u64 << (out.size * 8)) - 1 };
        let value = match (op.opcode, op.ins.as_slice()) {
            (OpCode::CPUI_COPY, [a]) => self.value(a)?,
            (OpCode::CPUI_INT_ADD, [a, b]) => self.value(a)?.wrapping_add(self.value(b)?),
            (OpCode::CPUI_INT_SUB, [a, b]) => self.value(a)?.wrapping_sub(self.value(b)?),
            _ => return None,
        };
        let key = Self::key(out)?;
        self.known.insert(key, value & mask);
        Some(())
    }

    /// The one register an instruction of only constant arithmetic writes, and
    /// its value, or `None` for any other instruction.
    fn single_write(&mut self, ops: &[FullOp]) -> Option<(VarnodeData, u64)> {
        let mut written = None;
        for op in ops {
            self.eval(op)?;
            let out = op.out.as_ref()?;
            if self.is_register(out) {
                if written.as_ref().is_some_and(|w: &VarnodeData| w != out) {
                    return None;
                }
                written = Some(out.clone());
            }
        }
        let reg = written?;
        let value = self.value(&reg)?;
        Some((reg, value))
    }

    /// Evaluate an instruction that neither touches memory nor branches and
    /// adds a constant to `reg`, returning the sum. A value derived from `reg`
    /// is `reg` itself, a `COPY` of a derived value, or an `INT_ADD`/`INT_SUB`
    /// of exactly one derived value and one constant (the derived value first
    /// for `INT_SUB`); a constant is a value the instruction computes from
    /// constants alone. Every write of `reg` must be derived, and the last one
    /// must have taken at least one constant step, so `mov ip,#K`,
    /// `mov ip,ip` and `add ip,ip,ip` are no add. `add ip,ip,#N` also computes
    /// flag temporaries, which are not followed.
    fn add_to(&mut self, ops: &[FullOp], reg: &VarnodeData) -> Option<u64> {
        let reg_key = Self::key(reg)?;
        let mut derived: HashMap<(usize, u64, u32), bool> = HashMap::from([(reg_key, false)]);
        let mut wrote = false;
        for op in ops {
            if is_memory_or_flow(op.opcode) {
                return None;
            }
            let step = match (op.opcode, op.ins.as_slice()) {
                (OpCode::CPUI_COPY, [a]) => Self::derived(&derived, a),
                (OpCode::CPUI_INT_ADD, [a, b]) => match (Self::derived(&derived, a), Self::derived(&derived, b)) {
                    (Some(_), Some(_)) | (None, None) => None,
                    (Some(_), None) => self.constant(&derived, b).then_some(true),
                    (None, Some(_)) => self.constant(&derived, a).then_some(true),
                },
                (OpCode::CPUI_INT_SUB, [a, b]) => {
                    Self::derived(&derived, a).filter(|_| self.constant(&derived, b)).map(|_| true)
                }
                _ => None,
            };
            let evaluated = self.eval(op).is_some();
            let Some(out) = op.out.as_ref().and_then(Self::key) else { continue };
            match step {
                Some(added) => derived.insert(out, added),
                None => derived.remove(&out),
            };
            if out == reg_key {
                if !evaluated || step.is_none() {
                    return None;
                }
                wrote = step == Some(true);
            }
        }
        if wrote { self.value(reg) } else { None }
    }

    /// Whether `vn` is derived from the chain register, and if so whether a
    /// constant step was taken on the way.
    fn derived(derived: &HashMap<(usize, u64, u32), bool>, vn: &VarnodeData) -> Option<bool> {
        Self::key(vn).and_then(|key| derived.get(&key).copied())
    }

    /// Whether `vn` is a constant: a constant-space value, or a value this
    /// instruction computed from constants alone.
    fn constant(&self, derived: &HashMap<(usize, u64, u32), bool>, vn: &VarnodeData) -> bool {
        Self::derived(derived, vn).is_none() && self.value(vn).is_some()
    }

    /// The slot whose word an instruction loads from a known address and then
    /// jumps to (`BRANCHIND`), or `None`. A value computed from a loaded word
    /// (`ldr pc` clears the Thumb bit first) stands for that word's slot, so
    /// `ldm ip,{r0,pc}` names `pc`'s word, not `r0`'s. Every op's output loses
    /// the value it held before, so a reloaded register names no address.
    fn load_jump(&mut self, ops: &[FullOp]) -> Option<u64> {
        if ops.last()?.opcode != OpCode::CPUI_BRANCHIND {
            return None;
        }
        let mut loaded: HashMap<(usize, u64, u32), u64> = HashMap::new();
        for op in ops {
            let from = |vn: &VarnodeData| Self::key(vn).and_then(|key| loaded.get(&key).copied());
            let slot = match op.opcode {
                OpCode::CPUI_BRANCHIND => return from(op.ins.first()?),
                OpCode::CPUI_LOAD => op.ins.get(1).and_then(|at| self.value(at)),
                _ => op.ins.iter().find_map(from),
            };
            let Some(out) = op.out.as_ref().and_then(Self::key) else { continue };
            match slot {
                Some(slot) => {
                    loaded.insert(out, slot);
                    self.known.remove(&out);
                }
                None => {
                    loaded.remove(&out);
                    if self.eval(op).is_none() {
                        self.known.remove(&out);
                    }
                }
            }
        }
        None
    }
}

fn is_memory_or_flow(opcode: OpCode) -> bool {
    matches!(
        opcode,
        OpCode::CPUI_LOAD
            | OpCode::CPUI_STORE
            | OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Spaces {
        konst: Rc<AddrSpace>,
        ram: Rc<AddrSpace>,
        reg: Rc<AddrSpace>,
        uniq: Rc<AddrSpace>,
    }

    fn spaces() -> Spaces {
        Spaces {
            konst: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
            ram: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            reg: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            uniq: Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
        }
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    const MAPPED: &[(u64, u64)] = &[(0x1000, 0x2000), (0x100000, 0x104000)];
    /// The `.plt` the stubs of these tests lie in.
    const PLT: &[(u64, u64)] = &[(0x1000, 0x3000)];
    const IP: u64 = 0x50;
    const PC: u64 = 0x5c;

    /// `add ip,pc,#0` as the ARM SLEIGH lifts it: a copy of `pc + 8`.
    fn adr(s: &Spaces, value: u64) -> Vec<FullOp> {
        vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, IP, 4)), vec![vn(&s.konst, value, 4)])]
    }

    /// `add ip,ip,#imm`, with one of the flag ops it also computes.
    fn add(s: &Spaces, imm: u64) -> Vec<FullOp> {
        vec![
            op(OpCode::CPUI_INT_CARRY, Some(vn(&s.reg, 0x66, 1)), vec![vn(&s.reg, IP, 4), vn(&s.konst, imm, 4)]),
            op(OpCode::CPUI_INT_ADD, Some(vn(&s.reg, IP, 4)), vec![vn(&s.reg, IP, 4), vn(&s.konst, imm, 4)]),
        ]
    }

    /// `ldr pc,[ip,#imm]!`: the load, the Thumb bit cleared, the jump.
    fn ldr_pc(s: &Spaces, imm: u64) -> Vec<FullOp> {
        let loaded = vn(&s.uniq, 0xa7300, 4);
        vec![
            op(OpCode::CPUI_INT_ADD, Some(vn(&s.reg, IP, 4)), vec![vn(&s.reg, IP, 4), vn(&s.konst, imm, 4)]),
            op(OpCode::CPUI_LOAD, Some(loaded.clone()), vec![vn(&s.konst, 3, 8), vn(&s.reg, IP, 4)]),
            op(OpCode::CPUI_INT_AND, Some(vn(&s.reg, PC, 4)), vec![loaded, vn(&s.konst, 0xffff_fffe, 4)]),
            op(OpCode::CPUI_BRANCHIND, None, vec![vn(&s.reg, PC, 4)]),
        ]
    }

    /// Drive one instruction, returning its rows and the rows released.
    fn step(stubs: &mut PltStubs, vma: u64, ops: &[FullOp], drefs: &[(u64, XrefKind)]) -> (Vec<(u64, XrefKind)>, Vec<Held>) {
        let mut drefs = drefs.to_vec();
        let released = stubs.step(vma, 4, ops, &mut drefs, "text");
        (drefs, released)
    }

    /// A stub whose first partial sum falls in an unmapped gap (so it files no
    /// row of its own) is still recognised: the mapped immediate the next
    /// instruction adds files nothing, and the load files the slot.
    #[test]
    fn a_chain_starts_whatever_its_first_value_maps_to() {
        let s = spaces();
        assert!(!in_range(MAPPED, 0x3008));
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        assert_eq!(step(&mut stubs, 0x1000, &adr(&s, 0x3008), &[]), (vec![], vec![]));
        assert_eq!(step(&mut stubs, 0x1004, &add(&s, 0x100000), &[(0x100000, XrefKind::Data)]), (vec![], vec![]));
        assert_eq!(step(&mut stubs, 0x1008, &ldr_pc(&s, 0x10), &[]), (vec![(0x103018, XrefKind::Read)], vec![]));
        assert!(stubs.drain().is_empty());
    }

    /// A run that leaves the shape files the rows it held, as they were.
    #[test]
    fn a_broken_chain_files_its_rows() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        assert_eq!(step(&mut stubs, 0x1000, &adr(&s, 0x2000), &[(0x2000, XrefKind::Data)]), (vec![], vec![]));
        let store = vec![op(OpCode::CPUI_STORE, None, vec![vn(&s.konst, 3, 8), vn(&s.reg, IP, 4), vn(&s.reg, 0, 4)])];
        let (rows, released) = step(&mut stubs, 0x1004, &store, &[]);
        assert_eq!((rows, released), (vec![], vec![(0x1000, 0x2000, "text".to_string())]));
        step(&mut stubs, 0x2000, &adr(&s, 0x3000), &[(0x3000, XrefKind::Data)]);
        assert_eq!(stubs.drain(), vec![(0x2000, 0x3000, "text".to_string())]);
    }

    /// `mov ip,#K` overwrites the register instead of adding to it: the chain
    /// breaks there, the `adr`'s row is filed as it was, and the load after
    /// the `mov` files no slot.
    #[test]
    fn an_overwrite_is_not_an_add() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        step(&mut stubs, 0x1000, &adr(&s, 0x1800), &[(0x1800, XrefKind::Data)]);
        let (_, released) = step(&mut stubs, 0x1004, &adr(&s, 0x101000), &[(0x101000, XrefKind::Data)]);
        assert_eq!(released, vec![(0x1000, 0x1800, "text".to_string())]);
        let (rows, released) = step(&mut stubs, 0x1008, &ldr_pc(&s, 0xc), &[]);
        assert_eq!(rows, vec![]);
        assert_eq!(released, vec![(0x1004, 0x101000, "text".to_string())]);
    }

    /// `add ip,ip,ip` doubles the register and `mov ip,ip` copies it: neither
    /// adds a constant, so the chain breaks there, the `adr`'s row is filed as
    /// it was, and the load after files no slot.
    #[test]
    fn a_step_must_add_one_constant() {
        let s = spaces();
        let double = vec![op(OpCode::CPUI_INT_ADD, Some(vn(&s.reg, IP, 4)), vec![vn(&s.reg, IP, 4), vn(&s.reg, IP, 4)])];
        let copy = vec![op(OpCode::CPUI_COPY, Some(vn(&s.reg, IP, 4)), vec![vn(&s.reg, IP, 4)])];
        // Each load's immediate puts the slot the old rule took at 0x100000.
        for (step_ops, imm) in [(double, 0xfd000), (copy, 0xfe800)] {
            let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
            step(&mut stubs, 0x1000, &adr(&s, 0x1800), &[(0x1800, XrefKind::Data)]);
            let (rows, released) = step(&mut stubs, 0x1004, &step_ops, &[]);
            assert_eq!((rows, released), (vec![], vec![(0x1000, 0x1800, "text".to_string())]));
            assert_eq!(step(&mut stubs, 0x1008, &ldr_pc(&s, imm), &[]), (vec![], vec![]));
        }
    }

    /// A constant the instruction computes into a temporary first is still a
    /// constant: `add ip,ip,t` with `t = #K` is a step.
    #[test]
    fn a_constant_computed_in_the_instruction_is_a_step() {
        let s = spaces();
        let t = vn(&s.uniq, 0x200, 4);
        let add_t = vec![
            op(OpCode::CPUI_COPY, Some(t.clone()), vec![vn(&s.konst, 0x100000, 4)]),
            op(OpCode::CPUI_INT_ADD, Some(vn(&s.reg, IP, 4)), vec![t, vn(&s.reg, IP, 4)]),
        ];
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        step(&mut stubs, 0x1000, &adr(&s, 0x1008), &[(0x1008, XrefKind::Data)]);
        assert_eq!(step(&mut stubs, 0x1004, &add_t, &[(0x100000, XrefKind::Data)]), (vec![], vec![]));
        assert_eq!(step(&mut stubs, 0x1008, &ldr_pc(&s, 0x10), &[]), (vec![(0x101018, XrefKind::Read)], vec![]));
    }

    /// `ldm ip,{r0,pc}` loads two words and jumps to the second: the slot is
    /// `pc`'s word, not the first word loaded.
    #[test]
    fn the_slot_is_the_word_the_jump_takes() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        let r0 = vn(&s.reg, 0x20, 4);
        let at = vn(&s.uniq, 0x100, 4);
        let ldm = vec![
            op(OpCode::CPUI_LOAD, Some(r0), vec![vn(&s.konst, 3, 8), vn(&s.reg, IP, 4)]),
            op(OpCode::CPUI_INT_ADD, Some(at.clone()), vec![vn(&s.reg, IP, 4), vn(&s.konst, 4, 4)]),
            op(OpCode::CPUI_LOAD, Some(vn(&s.reg, PC, 4)), vec![vn(&s.konst, 3, 8), at]),
            op(OpCode::CPUI_BRANCHIND, None, vec![vn(&s.reg, PC, 4)]),
        ];
        step(&mut stubs, 0x1000, &adr(&s, 0x1008), &[(0x1008, XrefKind::Data)]);
        step(&mut stubs, 0x1004, &add(&s, 0x100000), &[(0x100000, XrefKind::Data)]);
        assert_eq!(step(&mut stubs, 0x1008, &ldm, &[]), (vec![(0x10100c, XrefKind::Read)], vec![]));
    }

    /// A load-and-jump that reloads the chain register and then jumps through
    /// the word the reload names (`ldr ip,[ip]; ldr pc,[ip]`) reads an address
    /// the chain does not know: it files no slot and keeps the held rows.
    #[test]
    fn a_reloaded_register_names_no_slot() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        let reload = vec![
            op(OpCode::CPUI_LOAD, Some(vn(&s.reg, IP, 4)), vec![vn(&s.konst, 3, 8), vn(&s.reg, IP, 4)]),
            op(OpCode::CPUI_LOAD, Some(vn(&s.reg, PC, 4)), vec![vn(&s.konst, 3, 8), vn(&s.reg, IP, 4)]),
            op(OpCode::CPUI_BRANCHIND, None, vec![vn(&s.reg, PC, 4)]),
        ];
        step(&mut stubs, 0x1000, &adr(&s, 0x1008), &[(0x1008, XrefKind::Data)]);
        step(&mut stubs, 0x1004, &add(&s, 0x100000), &[(0x100000, XrefKind::Data)]);
        let (rows, released) = step(&mut stubs, 0x1008, &reload, &[]);
        assert_eq!(rows, vec![]);
        assert_eq!(released, vec![(0x1000, 0x1008, "text".to_string()), (0x1004, 0x100000, "text".to_string())]);
    }

    /// A veneer outside the `.plt` built like a stub (`adr; add; ldr pc`) is
    /// not one: each instruction keeps its rows and the load files no slot.
    #[test]
    fn a_veneer_outside_the_plt_keeps_its_rows() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        let adr_rows = [(0x1800, XrefKind::Data)];
        assert_eq!(step(&mut stubs, 0x4000, &adr(&s, 0x1800), &adr_rows), (adr_rows.to_vec(), vec![]));
        let add_rows = [(0x100000, XrefKind::Data)];
        assert_eq!(step(&mut stubs, 0x4004, &add(&s, 0x100000), &add_rows), (add_rows.to_vec(), vec![]));
        assert_eq!(step(&mut stubs, 0x4008, &ldr_pc(&s, 0x10), &[]), (vec![], vec![]));
        assert!(stubs.drain().is_empty());
    }

    /// A chain whose slot is in no mapped section is not a stub: it files no
    /// `Read` and keeps the rows the generic rule gave it.
    #[test]
    fn an_unmapped_slot_is_no_stub() {
        let s = spaces();
        let mut stubs = PltStubs::new(Some(&s.ram), MAPPED, PLT.to_vec());
        step(&mut stubs, 0x1000, &adr(&s, 0x1008), &[(0x1008, XrefKind::Data)]);
        step(&mut stubs, 0x1004, &add(&s, 0x100000), &[(0x100000, XrefKind::Data)]);
        let (rows, released) = step(&mut stubs, 0x1008, &ldr_pc(&s, 0x4000), &[]);
        assert_eq!(rows, vec![]);
        assert_eq!(released, vec![(0x1000, 0x1008, "text".to_string()), (0x1004, 0x100000, "text".to_string())]);
    }
}
