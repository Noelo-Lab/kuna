//! (kuna) Itanium constructor and destructor recovery from vptr stores — the
//! refinement of `itaniumrtti` that names `C::C` and `C::~C`.
//!
//! A constructor or destructor of a polymorphic class `C` is the one kind of
//! function that writes `C`'s vptr — the address of the first slot of `C`'s
//! primary sub-vtable — into the object it was called on. Materializing the
//! constant is not evidence: at `-O2` a factory that inlines `new C(...)`, or a
//! caller that builds a `C` on its stack, does that too. So a candidate is lifted
//! to p-code and walked along its fall-through path from the entry with a small
//! symbolic state — the first parameter is `this`, the stack pointer is the frame
//! base, spills to the frame are followed, and a call keeps only what the calling
//! convention leaves untouched — and only a STORE of a vptr constant through
//! `this + off` counts. A factory stores through `operator new`'s result and a
//! stack builder through its own frame; neither is `this`. The fall-through path
//! is a real execution path, so a store seen on it is one the function performs.
//!
//! # Which name
//!
//! * A function in `C`'s own primary sub-vtable that stores `C`'s vptr is one of
//!   its virtual destructors, `C::~C` — the name a symbol table gives both the
//!   complete-object (`D1`) and the deleting (`D0`) one — and it replaces
//!   its slot-index name. The ABI puts `D0` in the slot after `D1`, but which of the
//!   two stores the vptr depends on the optimizer (at `-O2` the deleting one may
//!   inline the other and be the only one that stores), so the next slot is named
//!   too only when it calls the one found.
//! * A function in no vtable that stores `C`'s vptr is a constructor `C::C` —
//!   but only once `C`, or a base of `C`, is known to have a virtual destructor.
//!   Without one, the same store is what a non-virtual destructor does, and
//!   nothing in the store tells the two apart, so the function is left alone.
//! * A function storing vptrs of classes that are not one inheritance chain, or a
//!   vtable slot of another class that stores `C`'s vptr (an inlined `clone`), is
//!   left alone.
//!
//! Where several classes' primary vptrs are stored at offset 0 — an inlined base
//! constructor ahead of the derived one — the most-derived class owns the
//! function. Both halves run where the Listing is built ([`ItaniumCtorsPass`]);
//! without it nothing is named.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::fspec::{effect_type, PrototypePieces, EXTRAPOP_UNKNOWN};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::listing::Listing;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, SymFact, SymKind};

/// How far the walk follows one candidate from its entry. A vptr store sits in
/// the first few dozen instructions of a constructor even at `-O0`.
const MAX_INSNS: usize = 256;

/// A destructor stores its vptr before anything else it does.
const MAX_DTOR_INSNS: usize = 48;

/// The constructor/destructor half of `itaniumrtti`. The vtable graph is
/// recovered at load, before the Listing exists and before the image is attached
/// to the decoder; the walk needs the decoder and the constructors' candidates
/// need the Listing, so both are named here. It answers to the same option.
/// A destructor's slot already carries its slot-index name by then; the commit
/// replaces a slot-index name with a real one.
#[derive(Default)]
pub struct ItaniumCtorsPass;

impl AnalysisPass for ItaniumCtorsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "itaniumrtti"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let Some(listing) = ctx.listing else {
            return out;
        };
        let classes = super::kuna_itaniumrtti::recovered_classes(ctx);
        if let Some(walker) = Walker::new(ctx) {
            let (mut symbols, has_virtual_dtor) = destructor_slots(&walker, &classes);
            symbols.extend(constructors(&walker, listing, &classes, &has_virtual_dtor));
            out.symbols = symbols;
        }
        out
    }
}

/// One polymorphic class, as the vtable recovery left it.
pub struct ClassVtables {
    /// The qualified class name the slot symbols use (`shapes::Circle`).
    pub name: String,
    /// Indices (into the same slice) of every transitive base class.
    pub ancestors: HashSet<usize>,
    /// The primary sub-vtable's slot targets, `None` for an imported slot.
    pub primary_slots: Vec<Option<u64>>,
    /// Every vptr value of this class: the primary first, then each secondary.
    pub vptrs: Vec<u64>,
}

/// Every `(D1 or D0 entry, class)` pair, as facts, plus the classes found to have
/// a virtual destructor.
fn destructor_slots(walker: &Walker, classes: &[ClassVtables]) -> (Vec<SymFact>, HashSet<usize>) {
    let owner_of = owners(classes);
    let mut walked: HashMap<u64, (Option<usize>, Vec<u64>)> = HashMap::new();
    let mut walk = |f: u64| -> (Option<usize>, Vec<u64>) {
        walked
            .entry(f)
            .or_insert_with(|| {
                let w = walker.walk(f, &owner_of, MAX_DTOR_INSNS);
                (owning_class(&w.stores, &owner_of, classes), w.calls)
            })
            .clone()
    };
    let mut out: Vec<SymFact> = Vec::new();
    let mut has_virtual_dtor: HashSet<usize> = HashSet::new();
    for (c, cls) in classes.iter().enumerate() {
        for (i, slot) in cls.primary_slots.iter().enumerate() {
            let Some(f) = *slot else { continue };
            if walk(f).0 != Some(c) {
                continue;
            }
            has_virtual_dtor.insert(c);
            let name = dtor_name(&cls.name);
            out.push(SymFact {
                addr: f,
                name: name.clone(),
                kind: SymKind::Function,
            });
            // The deleting destructor follows the complete one, and at `-O0` it
            // calls it rather than storing the vptr itself. Which of the two `f`
            // is cannot be read off the store (at `-O2` only the deleting one may
            // store), so the neighbour is named only when it calls `f`.
            if let Some(Some(next)) = cls.primary_slots.get(i + 1) {
                if *next != f && walk(*next).1.contains(&f) {
                    out.push(SymFact {
                        addr: *next,
                        name,
                        kind: SymKind::Function,
                    });
                }
            }
        }
    }
    dedup(&mut out);
    (out, has_virtual_dtor)
}

/// The constructor names of `classes`: functions in no vtable whose `this` stores
/// make them a constructor of a class with a virtual destructor.
fn constructors(
    walker: &Walker,
    listing: &Listing,
    classes: &[ClassVtables],
    has_virtual_dtor: &HashSet<usize>,
) -> Vec<SymFact> {
    let owner_of = owners(classes);
    let in_a_vtable: HashSet<u64> = classes
        .iter()
        .flat_map(|c| c.primary_slots.iter().flatten().copied())
        .collect();
    let mut candidates = candidate_functions(listing, owner_of.keys().copied());
    candidates.sort_unstable();
    candidates.dedup();

    let mut out: Vec<SymFact> = Vec::new();
    for entry in candidates {
        if in_a_vtable.contains(&entry) {
            continue;
        }
        let stores = walker.walk(entry, &owner_of, MAX_INSNS).stores;
        let Some(c) = owning_class(&stores, &owner_of, classes) else {
            continue;
        };
        let virtual_dtor = has_virtual_dtor.contains(&c)
            || classes[c]
                .ancestors
                .iter()
                .any(|a| has_virtual_dtor.contains(a));
        if virtual_dtor {
            out.push(SymFact {
                addr: entry,
                name: ctor_name(&classes[c].name),
                kind: SymKind::Function,
            });
        }
    }
    dedup(&mut out);
    out
}

fn owners(classes: &[ClassVtables]) -> HashMap<u64, (usize, bool)> {
    let mut m = HashMap::new();
    for (c, cls) in classes.iter().enumerate() {
        for (i, &v) in cls.vptrs.iter().enumerate() {
            m.insert(v, (c, i == 0));
        }
    }
    m
}

fn dedup(out: &mut Vec<SymFact>) {
    out.sort_by(|a, b| (a.addr, &a.name).cmp(&(b.addr, &b.name)));
    out.dedup_by(|a, b| a.addr == b.addr && a.name == b.name);
}

/// The functions that may materialize a vptr: those with an instruction whose
/// decoded operand names a vptr value or its 4 KiB page (the first half of an
/// `adrp`/`add` pair). The Listing keeps control-flow edges only, and its
/// operand text is the cheap superset; the p-code walk decides.
fn candidate_functions(listing: &Listing, vptrs: impl Iterator<Item = u64>) -> Vec<u64> {
    if !listing.has_assembly() {
        return Vec::new();
    }
    let mut wanted: HashSet<u64> = HashSet::new();
    for v in vptrs {
        wanted.insert(v);
        wanted.insert(v & !0xfff);
    }
    let mut out: Vec<u64> = Vec::new();
    for (&addr, insn) in listing.instructions() {
        if hex_literals(&insn.operands).any(|x| wanted.contains(&x)) {
            if let Some(f) = listing.function_containing(addr) {
                out.push(earliest_entry(listing, f.entry));
            }
        }
    }
    out
}

/// The entry a function really starts at when the Listing holds two for it: a
/// prologue pattern can file a start just past an entry the code falls through
/// from (`endbr64` before `push rbp`), and `function_containing` answers the
/// nearer one.
fn earliest_entry(listing: &Listing, entry: u64) -> u64 {
    let mut best = entry;
    let mut at = entry;
    for _ in 0..4 {
        let Some(prev) = listing.instruction_before(at) else {
            break;
        };
        if prev.fall_through != Some(at) || prev.flow.is_jump || prev.flow.is_call {
            break;
        }
        at = prev.addr;
        if listing.function_at(at).is_some() {
            best = at;
        }
    }
    best
}

/// Every `0x…` literal in a disassembly operand string.
fn hex_literals(text: &str) -> impl Iterator<Item = u64> + '_ {
    text.match_indices("0x").filter_map(move |(i, _)| {
        let digits = &text[i + 2..];
        let end = digits
            .find(|c: char| !c.is_ascii_hexdigit())
            .unwrap_or(digits.len());
        u64::from_str_radix(&digits[..end], 16).ok()
    })
}

/// The class a function's `this` stores make it a constructor or destructor of:
/// the most-derived class whose primary vptr is stored at offset 0, provided every
/// stored vptr belongs to it or to one of its bases.
fn owning_class(
    stores: &[(u64, i64)],
    owner_of: &HashMap<u64, (usize, bool)>,
    classes: &[ClassVtables],
) -> Option<usize> {
    let stored: HashSet<usize> = stores
        .iter()
        .filter_map(|(v, _)| owner_of.get(v))
        .map(|o| o.0)
        .collect();
    let primaries: HashSet<usize> = stores
        .iter()
        .filter(|(_, off)| *off == 0)
        .filter_map(|(v, _)| owner_of.get(v).filter(|o| o.1).map(|o| o.0))
        .collect();
    let mut owners = primaries.iter().copied().filter(|&c| {
        stored
            .iter()
            .all(|&o| o == c || classes[c].ancestors.contains(&o))
    });
    let owner = owners.next()?;
    owners.next().is_none().then_some(owner)
}

fn leaf(class: &str) -> &str {
    class.rsplit("::").next().unwrap_or(class)
}

fn ctor_name(class: &str) -> String {
    format!("{class}::{}", leaf(class))
}

fn dtor_name(class: &str) -> String {
    format!("{class}::~{}", leaf(class))
}

/// A fold key: storage space index, offset and width.
type Key = (usize, u64, u32);

/// One lifted instruction: its p-code and its length.
type Ops = Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>;

/// What one walk saw: every `(vptr, offset)` stored through `this`, and every
/// direct call target.
#[derive(Default)]
struct Walk {
    stores: Vec<(u64, i64)>,
    calls: Vec<u64>,
}

/// What the walk knows a register or frame slot holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Val {
    Const(u64),
    /// `this` plus a byte offset.
    This(i64),
    /// The stack pointer at entry plus a byte offset.
    Sp(i64),
}

#[derive(Default)]
struct State {
    regs: HashMap<Key, Val>,
    frame: HashMap<(i64, u32), Val>,
}

/// The per-binary facts every walk shares: where `this` and the stack pointer
/// live, what a call leaves intact, and how much a call pops.
struct Walker<'a> {
    ctx: &'a AnalysisCtx<'a>,
    sp: Key,
    this_reg: Option<Key>,
    /// `this` passed in memory: its frame offset from the entry stack pointer.
    this_slot: Option<(i64, u32)>,
    extrapop: i64,
}

impl<'a> Walker<'a> {
    fn new(ctx: &'a AnalysisCtx<'a>) -> Option<Self> {
        let stack = ctx.arch.manage().get_stack_space()?;
        let base = stack.get_spacebase(0).ok()?;
        let sp = (
            base.space.as_ref()?.get_index() as usize,
            base.offset,
            base.size,
        );
        let model = ctx.arch.default_fp()?;
        let types = ctx.arch.types();
        let (_, word_size) = ctx.arch.data_org();
        let void = types.get_type_void().ok()?;
        let ptr = types
            .get_type_pointer(types.get_size_of_pointer(), void, word_size)
            .ok()?;
        let pieces = PrototypePieces {
            name: String::new(),
            outtype: Some(types.get_type_void().ok()?),
            intypes: vec![ptr],
            innames: vec!["this".to_string()],
            first_var_arg_slot: -1,
            output_storage: None,
            input_storage: Vec::new(),
            kuna_library_names: None,
        };
        let mut assigned = Vec::new();
        model
            .assign_parameter_storage(&pieces, &mut assigned, false, types, ctx.arch.manage())
            .ok()?;
        let this = assigned.get(1)?;
        let space = this.addr.get_space()?;
        let size = this.type_.as_ref()?.get_size() as u32;
        let (this_reg, this_slot) = if space.get_index() == stack.get_index() {
            let bits = size.saturating_mul(8).min(64);
            (
                None,
                Some((sign_extend(this.addr.get_offset(), bits), size)),
            )
        } else {
            (
                Some((space.get_index() as usize, this.addr.get_offset(), size)),
                None,
            )
        };
        let extrapop = model.get_extra_pop();
        let extrapop = if extrapop == EXTRAPOP_UNKNOWN {
            0
        } else {
            i64::from(extrapop)
        };
        Some(Walker {
            ctx,
            sp,
            this_reg,
            this_slot,
            extrapop,
        })
    }

    /// Walk the function at `entry` along its fall-through path for at most
    /// `limit` instructions.
    fn walk(&self, entry: u64, owner_of: &HashMap<u64, (usize, bool)>, limit: usize) -> Walk {
        let mut st = State::default();
        st.regs.insert(self.sp, Val::Sp(0));
        if let Some(k) = self.this_reg {
            st.regs.insert(k, Val::This(0));
        }
        if let Some(slot) = self.this_slot {
            st.frame.insert(slot, Val::This(0));
        }
        let mut walk = Walk::default();
        let mut at = entry;
        for _ in 0..limit {
            let Some((ops, len)) = self.lift(at) else {
                break;
            };
            let mut conditional = false;
            let mut leaves = false;
            for (opc, out, ins) in &ops {
                match opc {
                    OpCode::CPUI_CBRANCH => conditional = true,
                    OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => {
                        leaves = !conditional;
                        conditional = true;
                    }
                    OpCode::CPUI_CALL => {
                        if let Some(target) = ins.first().filter(|t| !is_constant(t)) {
                            walk.calls.push(target.offset);
                        }
                        self.after_call(&mut st);
                    }
                    OpCode::CPUI_CALLIND => self.after_call(&mut st),
                    OpCode::CPUI_STORE => self.store(&mut st, ins, owner_of, &mut walk.stores),
                    _ => self.step(&mut st, *opc, out.as_ref(), ins, conditional),
                }
            }
            if leaves || len == 0 {
                break;
            }
            at += len;
        }
        walk
    }

    fn lift(&self, vma: u64) -> Option<(Ops, u64)> {
        let space = Rc::clone(self.ctx.arch.manage().get_default_code_space()?);
        let mut l = Lifted { ops: Vec::new() };
        let len = self
            .ctx
            .arch
            .translate()
            .one_instruction(&mut l, &Address::new(space, vma))
            .ok()?;
        Some((l.ops, u64::try_from(len).ok()?))
    }

    /// A call keeps what the calling convention says it does not touch and pops
    /// what the convention says it pops.
    fn after_call(&self, st: &mut State) {
        let ctx = self.ctx;
        let Some(model) = ctx.arch.default_fp() else {
            st.regs.clear();
            return;
        };
        let sp = st.regs.get(&self.sp).copied();
        st.regs.retain(|&(space, off, size), _| {
            let Some(spc) = ctx.arch.manage().get_space(space as i32) else {
                return false;
            };
            let addr = Address::new(Rc::clone(spc), off);
            model.has_effect(&addr, size as i32) == effect_type::UNAFFECTED
        });
        if let Some(Val::Sp(o)) = sp {
            st.regs.insert(self.sp, Val::Sp(o + self.extrapop));
        }
    }

    fn store(
        &self,
        st: &mut State,
        ins: &[VarnodeData],
        owner_of: &HashMap<u64, (usize, bool)>,
        found: &mut Vec<(u64, i64)>,
    ) {
        let (Some(addr), Some(value)) = (ins.get(1), ins.get(2)) else {
            return;
        };
        let stored = self.value(st, value);
        match self.value(st, addr) {
            Some(Val::Sp(o)) => {
                let size = value.size;
                st.frame
                    .retain(|&(fo, fs), _| fo + i64::from(fs) <= o || o + i64::from(size) <= fo);
                if let Some(v) = stored {
                    st.frame.insert((o, size), v);
                }
            }
            Some(Val::This(o)) => {
                if let Some(Val::Const(v)) = stored {
                    if owner_of.contains_key(&v) {
                        found.push((v, o));
                    }
                }
            }
            _ => {}
        }
    }

    fn step(
        &self,
        st: &mut State,
        opc: OpCode,
        out: Option<&VarnodeData>,
        ins: &[VarnodeData],
        conditional: bool,
    ) {
        let value = match opc {
            OpCode::CPUI_COPY => ins.first().and_then(|v| self.value(st, v)),
            OpCode::CPUI_INT_ZEXT => match ins.first().and_then(|v| self.value(st, v)) {
                Some(Val::Const(c)) => Some(Val::Const(c)),
                _ => None,
            },
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => self.add(st, ins, false),
            OpCode::CPUI_INT_SUB => self.add(st, ins, true),
            OpCode::CPUI_LOAD => match ins.get(1).and_then(|a| self.value(st, a)) {
                Some(Val::Sp(o)) => st.frame.get(&(o, out.map_or(0, |o| o.size))).copied(),
                _ => None,
            },
            _ => None,
        };
        if opc == OpCode::CPUI_CALLOTHER {
            st.regs.retain(|k, _| *k == self.sp);
        }
        let Some(out) = out else { return };
        let Some(k) = key_of(out) else { return };
        kill_overlapping(&mut st.regs, &k);
        if let Some(v) = value.filter(|_| !conditional) {
            st.regs.insert(k, v);
        }
    }

    fn add(&self, st: &State, ins: &[VarnodeData], sub: bool) -> Option<Val> {
        let a = self.value(st, ins.first()?)?;
        let b = self.value(st, ins.get(1)?)?;
        let bits = ins[0].size.saturating_mul(8).min(64);
        let delta = |c: u64| {
            let d = sign_extend(c, bits);
            if sub {
                d.wrapping_neg()
            } else {
                d
            }
        };
        match (a, b) {
            (Val::Const(x), Val::Const(y)) => {
                let r = if sub {
                    x.wrapping_sub(y)
                } else {
                    x.wrapping_add(y)
                };
                Some(Val::Const(truncate(r, bits)))
            }
            (Val::This(o), Val::Const(c)) => Some(Val::This(o.wrapping_add(delta(c)))),
            (Val::Sp(o), Val::Const(c)) => Some(Val::Sp(o.wrapping_add(delta(c)))),
            (Val::Const(c), Val::This(o)) if !sub => Some(Val::This(o.wrapping_add(delta(c)))),
            (Val::Const(c), Val::Sp(o)) if !sub => Some(Val::Sp(o.wrapping_add(delta(c)))),
            _ => None,
        }
    }

    fn value(&self, st: &State, vn: &VarnodeData) -> Option<Val> {
        if is_constant(vn) {
            return Some(Val::Const(vn.offset));
        }
        st.regs.get(&key_of(vn)?).copied()
    }
}

struct Lifted {
    ops: Ops,
}

impl PcodeEmit for Lifted {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

fn is_constant(vn: &VarnodeData) -> bool {
    vn.space
        .as_ref()
        .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
}

fn key_of(vn: &VarnodeData) -> Option<Key> {
    let space = vn.space.as_ref()?;
    Some((space.get_index() as usize, vn.offset, vn.size))
}

fn kill_overlapping(regs: &mut HashMap<Key, Val>, k: &Key) {
    let (space, off, size) = *k;
    let hi = off.wrapping_add(u64::from(size));
    regs.retain(|(s, o, sz), _| *s != space || o.wrapping_add(u64::from(*sz)) <= off || *o >= hi);
}

fn truncate(v: u64, bits: u32) -> u64 {
    if bits >= 64 {
        v
    } else {
        v & ((1u64 << bits) - 1)
    }
}

fn sign_extend(v: u64, bits: u32) -> i64 {
    if bits == 0 || bits >= 64 {
        return v as i64;
    }
    let shift = 64 - bits;
    ((v << shift) as i64) >> shift
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(name: &str, ancestors: &[usize], vptrs: &[u64]) -> ClassVtables {
        ClassVtables {
            name: name.to_string(),
            ancestors: ancestors.iter().copied().collect(),
            primary_slots: Vec::new(),
            vptrs: vptrs.to_vec(),
        }
    }

    #[test]
    fn the_most_derived_primary_store_owns_the_function() {
        let classes = [
            class("Shape", &[], &[0x100]),
            class("Circle", &[0], &[0x200]),
        ];
        let o = owners(&classes);
        assert_eq!(
            owning_class(&[(0x100, 0), (0x200, 0)], &o, &classes),
            Some(1)
        );
        assert_eq!(owning_class(&[(0x100, 0)], &o, &classes), Some(0));
    }

    #[test]
    fn unrelated_classes_or_a_secondary_alone_own_nothing() {
        let classes = [class("A", &[], &[0x100]), class("B", &[], &[0x200, 0x210])];
        let o = owners(&classes);
        assert_eq!(owning_class(&[(0x100, 0), (0x200, 0)], &o, &classes), None);
        assert_eq!(owning_class(&[(0x210, 16)], &o, &classes), None);
        assert_eq!(
            owning_class(&[(0x200, 0), (0x210, 16)], &o, &classes),
            Some(1)
        );
        assert_eq!(owning_class(&[(0x200, 8)], &o, &classes), None);
    }

    #[test]
    fn names_follow_the_symbol_table_spelling() {
        assert_eq!(ctor_name("shapes::Circle"), "shapes::Circle::Circle");
        assert_eq!(dtor_name("shapes::Circle"), "shapes::Circle::~Circle");
        assert_eq!(dtor_name("Vec_int"), "Vec_int::~Vec_int");
    }

    #[test]
    fn operand_text_yields_its_hex_literals() {
        assert_eq!(
            hex_literals("RDX,[0x3d00]").collect::<Vec<_>>(),
            vec![0x3d00]
        );
        assert_eq!(hex_literals("x0, 0x3000").collect::<Vec<_>>(), vec![0x3000]);
        assert_eq!(
            hex_literals("RAX,qword ptr [RBP + -0x8]").collect::<Vec<_>>(),
            vec![8]
        );
        assert_eq!(hex_literals("EAX,EDX").count(), 0);
    }

    #[test]
    fn sign_extension_reads_a_frame_offset() {
        assert_eq!(sign_extend(0xffff_fff8, 32), -8);
        assert_eq!(sign_extend(0xffff_ffff_ffff_fff0, 64), -16);
        assert_eq!(sign_extend(8, 64), 8);
    }
}
