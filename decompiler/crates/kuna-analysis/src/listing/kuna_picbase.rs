//! (kuna) `picbase` — resolve an i386 PIC base register to its constant, so a
//! GOT-relative address formation becomes a cross-reference to the thing it
//! forms.
//!
//! # The gap
//!
//! [`super::xrefs`] reads a reference out of one instruction's p-code: a
//! decode-time constant that lands in a mapped section. That is the whole
//! answer on x86-64, where `lea rax,[rip+0x36a]` lifts to `RAX = COPY 0x13c9:8`.
//! It is *no* answer in 32-bit position-independent code, where the address of
//! a string is not in the instruction at all:
//!
//! ```text
//!     call  0x80489f4          ; push the address of the next instruction
//! 0x80489f4:
//!     pop   ebx                ; ebx = 0x80489f4
//!     add   ebx,0x184c         ; ebx = 0x804a240 = _GLOBAL_OFFSET_TABLE_
//!     ...
//!     lea   eax,[ebx-0x1119]   ; eax = 0x8049127 = "Password: "
//! ```
//!
//! The literal `0x8049127` occurs nowhere in the image, so no scan over
//! constants — however wide — can find it. The reference exists only as the sum
//! of a register the program computed at run time and a displacement, and every
//! string, global and function pointer such a binary touches is reached that
//! way. The observed consequence is total: on the crackme this was filed
//! against, **all 23** strings reported `xrefs_count: 0`.
//!
//! # What is recovered, and on whose authority
//!
//! The `call`-to-its-own-fall-through / `pop` idiom exists for exactly one
//! reason — to materialize the program counter — and the i386 System V ABI
//! reserves the register it feeds (`%ebx`) as the module's GOT pointer, callee-
//! saved, for the whole module. So there are two separate claims here, and they
//! are licensed differently:
//!
//!  * **Function-local.** A function that runs the idiom itself computes the
//!    value; nothing is assumed. This needs no ABI argument at all.
//!  * **Module-wide.** A function that never establishes the base still *uses*
//!    it, because its caller set it up and the ABI says it survives. Claiming
//!    that needs evidence that the register really is the module's GOT pointer,
//!    so the recovered value is cross-checked against the image's own
//!    `_GLOBAL_OFFSET_TABLE_` ([`got_base`]) and every idiom in the program must
//!    agree on one register and one value. Absent the match, nothing is claimed.
//!
//! The module-wide half is not a nicety: kuna's function inventory splits the
//! filing crackme's one prompt routine into four entries at its `int3` traps,
//! and the `lea` that forms `"Password: "` lands in a *different* entry from the
//! idiom that set `ebx` up.
//!
//! # The rule that keeps ownership honest
//!
//! Attributing a string to a function that merely sits near it is worse than
//! reporting nothing, and no gate in this repo would see it. So the base is
//! offered to a function only when its own body cannot have changed it
//! ([`scope`]):
//!
//!  * the base register is **never written** in the body → the value is the one
//!    the ABI says it inherited, live throughout;
//!  * it is written, but only by the idiom that establishes it, and the
//!    establishment is provable → live after that point;
//!  * anything else — a scratch `mov ebx,...`, a `pop ebx` restoring a saved
//!    value, an unprovable establishment → **nothing is claimed for that
//!    function**. A function that uses `ebx` for its own purposes contributes no
//!    references rather than wrong ones.
//!
//! # The evaluator
//!
//! Recovering the idiom means reading a value through the stack (`call` pushes,
//! `pop` reads back), so this module carries a deliberately tiny abstract
//! machine over the same whole p-code [`super::xrefs`] already keeps: a value is
//! a constant or a stack offset, memory is modelled only at stack offsets, and
//! every op that is not plain integer arithmetic makes its output unknown. It is
//! the smallest thing that can follow `push`/`pop`, and it is *not* a dataflow
//! analysis: the harvest ([`refs_through_base`]) runs on one instruction at a
//! time with the base seeded and nothing else assumed, so no state ever crosses
//! a control-flow edge and there is no path sensitivity to get wrong.
//!
//! A constant is tainted with `pc` when it equals its own instruction's
//! fall-through — the return address the `call` pushed — and only a `pc`-tainted
//! value may establish a base. A plain `mov eax,0x804a240` therefore cannot.
//!
//! Default-**on** (`--option picbase off` restores the pre-fix answer, where a
//! 32-bit PIC binary's strings and globals are referenced by nothing).

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};
use object::read::{Object, ObjectSection};

use super::xrefs::{in_range, FullOp, XrefKind};

/// How far into a function the base-establishing idiom is looked for. GCC emits
/// it in the prologue, after at most a frame setup and the callee-saved pushes;
/// the filing crackme's is the 8th instruction. Bounded because this runs once
/// per function entry before the main walk.
const MAX_PROLOGUE_INSNS: usize = 24;

/// How many instructions an out-of-line PC thunk may take. `__x86.get_pc_thunk.bx`
/// is exactly two (`mov (%esp),%ebx; ret`); more slack would start admitting
/// ordinary functions that happen to read their own return address.
const MAX_THUNK_INSNS: usize = 2;

/// The return address an out-of-line thunk is probed with. Its identity does not
/// matter — the probe asks which register comes to hold *this* value — but it
/// must not look like anything the thunk itself could spell as a literal.
const PROBE_RET: u64 = 0xb0c0_d0e0;

/// `ScalarOperandAnalyzer.checkOperands`' floor, shared with [`super::xrefs`]: a
/// smaller value "could be a number, even if it is in the address space".
const MIN_ADDRESS_VALUE: u64 = 4096;

/// The module's PIC base: a register and the constant it holds.
#[derive(Debug, Clone)]
pub(super) struct PicBase {
    /// The base register, as the p-code varnode that names it.
    pub(super) reg: VarnodeData,
    /// Its module-wide value — the image's `_GLOBAL_OFFSET_TABLE_`.
    pub(super) value: u64,
}

/// Where in one function's body the base may be believed.
#[derive(Debug, Clone, Copy)]
pub(super) enum Scope {
    /// The register is never written here, so the inherited value is live
    /// throughout.
    Whole,
    /// The body establishes the base itself: live strictly after `from`, and up
    /// to the first write that is not part of the establishment (`until`, which
    /// is normally the epilogue's `mov ebx,[ebp-0x4]` restore).
    Between { from: u64, until: u64 },
}

impl Scope {
    /// May `vma` be evaluated with the base seeded?
    pub(super) fn admits(self, vma: u64) -> bool {
        match self {
            Scope::Whole => true,
            Scope::Between { from, until } => vma > from && vma < until,
        }
    }
}

/// The fixed facts the evaluator needs from the program.
pub(super) struct Ctx {
    /// The stack-pointer register, seeded symbolically so `push`/`pop` resolve.
    sp: VarnodeData,
    /// Manager index of the default data space, so a `LOAD`/`STORE` through
    /// some other space is not mistaken for a memory reference.
    data_index: Option<i32>,
}

impl Ctx {
    /// `None` when the compiler spec declares no stack pointer: without one the
    /// `call`/`pop` idiom cannot be followed, so the whole feature declines.
    pub(super) fn new(
        arch: &kuna_decomp::architecture::Architecture,
        data_space: Option<&Rc<AddrSpace>>,
    ) -> Option<Ctx> {
        let stack = arch.manage().get_stack_space()?;
        let sp = stack.get_spacebase(0).ok()?;
        Some(Ctx {
            sp: VarnodeData { space: sp.space, offset: sp.offset, size: sp.size },
            data_index: data_space.map(|d| d.get_index()),
        })
    }

    /// Is `vn` a register — the space the stack pointer lives in? `register` and
    /// `ram` are both processor spaces, so the type alone cannot tell them apart.
    fn is_register(&self, vn: &VarnodeData) -> bool {
        matches!((&vn.space, &self.sp.space), (Some(a), Some(b)) if Rc::ptr_eq(a, b))
    }
}

/// The address of the image's `_GLOBAL_OFFSET_TABLE_`.
///
/// GNU ld defines the symbol at the start of `.got.plt` when the image has a
/// PLT and at `.got` otherwise; the symbol itself is local and gone from a
/// stripped binary, so the section address is what is actually readable. `None`
/// leaves the module-wide claim unlicensed — a function that establishes its own
/// base is unaffected.
fn got_base(file: &object::File) -> Option<u64> {
    for name in [".got.plt", ".got"] {
        if let Some(sec) = file.section_by_name(name) {
            if sec.address() != 0 {
                return Some(sec.address());
            }
        }
    }
    None
}

// --- the abstract machine ----------------------------------------------------

/// One abstract value. Deliberately two-point: a constant (with the "this came
/// from a pushed return address" taint) or an offset from the stack pointer at
/// the point the machine was seeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Val {
    Const { v: u64, pc: bool },
    Stack(i64),
}

/// A varnode's identity: space index, offset, size.
type Key = (i32, u64, u32);

fn key_of(vn: &VarnodeData) -> Option<Key> {
    vn.space.as_ref().map(|s| (s.get_index(), vn.offset, vn.size))
}

/// The byte mask of an `n`-byte value.
fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 {
        u64::MAX
    } else {
        (1u64 << (8 * size)) - 1
    }
}

/// The live-varnode map. A `HashMap` here made every written op pay a
/// bucket-wide `retain`, which over a whole program was this pass's dominant
/// cost; the map never holds more than one instruction's worth of varnodes, so a
/// linear vector is both smaller and faster. Insertion order is not observed —
/// the one iterating reader ([`holder_of`]) takes a minimum.
#[derive(Default)]
struct KeyMap(Vec<(Key, Val)>);

impl KeyMap {
    fn insert(&mut self, k: Key, v: Val) {
        match self.0.iter_mut().find(|(known, _)| *known == k) {
            Some(slot) => slot.1 = v,
            None => self.0.push((k, v)),
        }
    }

    fn get(&self, k: &Key) -> Option<&Val> {
        self.0.iter().find(|(known, _)| known == k).map(|(_, v)| v)
    }

    fn retain(&mut self, mut keep: impl FnMut(&Key) -> bool) {
        self.0.retain(|(k, _)| keep(k));
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn iter(&self) -> impl Iterator<Item = (&Key, &Val)> {
        self.0.iter().map(|(k, v)| (k, v))
    }
}

#[derive(Default)]
struct Machine {
    vals: KeyMap,
    /// Constants stored at offsets from the seeded stack pointer.
    stack: HashMap<i64, (u64, bool)>,
}

impl Machine {
    fn seed(&mut self, vn: &VarnodeData, val: Val) {
        if let Some(k) = key_of(vn) {
            self.vals.insert(k, val);
        }
    }

    /// Drop every value whose bytes overlap `vn`. Keying on the exact triple
    /// would let a write to `bl` leave a stale `ebx` behind, which is the one
    /// way this machine could hand out a value the program does not hold.
    fn invalidate(&mut self, vn: &VarnodeData) {
        let Some((idx, off, size)) = key_of(vn) else { return };
        let (lo, hi) = (off, off.saturating_add(u64::from(size)));
        self.vals.retain(|&(i, o, s)| {
            i != idx || o.saturating_add(u64::from(s)) <= lo || o >= hi
        });
    }

    fn get(&self, vn: &VarnodeData) -> Option<Val> {
        let space = vn.space.as_ref()?;
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Some(Val::Const { v: vn.offset & mask(vn.size), pc: false });
        }
        self.vals.get(&key_of(vn)?).copied()
    }

    /// The constant `vn` holds, if it holds one.
    fn konst(&self, vn: &VarnodeData) -> Option<u64> {
        match self.get(vn)? {
            Val::Const { v, .. } => Some(v),
            Val::Stack(_) => None,
        }
    }

    fn write(&mut self, out: &VarnodeData, val: Option<Val>) {
        self.invalidate(out);
        if let (Some(k), Some(v)) = (key_of(out), val) {
            let v = match v {
                Val::Const { v, pc } => Val::Const { v: v & mask(out.size), pc },
                other => other,
            };
            self.vals.insert(k, v);
        }
    }

    /// Interpret one op. `fall_through` is the instruction's own successor: a
    /// constant equal to it is the return address a `call` just pushed, and only
    /// such a value may go on to establish a base.
    fn apply(&mut self, op: &FullOp, ctx: &Ctx, fall_through: u64) {
        let arg = |m: &Machine, i: usize| -> Option<Val> {
            let vn = op.ins.get(i)?;
            let v = m.get(vn)?;
            Some(match v {
                Val::Const { v, pc } => Val::Const { v, pc: pc || v == fall_through },
                other => other,
            })
        };

        // A call clobbers the caller-saved registers and may rewrite the frame
        // below the stack pointer; nothing survives it that this machine models.
        if matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            // The stack pointer survives: the callee's own `ret` puts it back,
            // and dropping it would strand every push and pop after the call.
            let sp = self.get(&ctx.sp);
            self.vals.clear();
            self.stack.clear();
            if let Some(v) = sp {
                self.seed(&ctx.sp, v);
            }
            if let Some(out) = &op.out {
                self.invalidate(out);
            }
            return;
        }

        if op.opcode == OpCode::CPUI_STORE {
            let addr = op.ins.get(1).and_then(|vn| self.get(vn));
            let value = arg(self, 2);
            if let Some(Val::Stack(d)) = addr {
                match value {
                    Some(Val::Const { v, pc }) => {
                        self.stack.insert(d, (v, pc));
                    }
                    _ => {
                        self.stack.remove(&d);
                    }
                }
            } else if addr.is_none() {
                // An unknown store address could alias any slot.
                self.stack.clear();
            }
            return;
        }

        let Some(out) = &op.out else { return };

        if op.opcode == OpCode::CPUI_LOAD {
            let from_default = op
                .ins
                .first()
                .zip(ctx.data_index)
                .is_some_and(|(vn, idx)| vn.offset == u64::try_from(idx).unwrap_or(u64::MAX));
            let loaded = match op.ins.get(1).and_then(|vn| self.get(vn)) {
                Some(Val::Stack(d)) if from_default => {
                    self.stack.get(&d).map(|&(v, pc)| Val::Const { v, pc })
                }
                _ => None,
            };
            self.write(out, loaded);
            return;
        }

        let a = arg(self, 0);
        let b = arg(self, 1);
        let folded = match op.opcode {
            OpCode::CPUI_COPY
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_CAST => a,
            OpCode::CPUI_INT_ADD => match (a, b) {
                (Some(Val::Const { v: x, pc: p }), Some(Val::Const { v: y, pc: q })) => {
                    Some(Val::Const { v: x.wrapping_add(y), pc: p || q })
                }
                (Some(Val::Stack(d)), Some(Val::Const { v, .. }))
                | (Some(Val::Const { v, .. }), Some(Val::Stack(d))) => {
                    Some(Val::Stack(d.wrapping_add(sign_extend(v, out.size))))
                }
                _ => None,
            },
            OpCode::CPUI_INT_SUB => match (a, b) {
                (Some(Val::Const { v: x, pc: p }), Some(Val::Const { v: y, pc: q })) => {
                    Some(Val::Const { v: x.wrapping_sub(y), pc: p || q })
                }
                (Some(Val::Stack(d)), Some(Val::Const { v, .. })) => {
                    Some(Val::Stack(d.wrapping_sub(sign_extend(v, out.size))))
                }
                _ => None,
            },
            OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_SUBPIECE => match (a, b) {
                (Some(Val::Const { v: x, pc: p }), Some(Val::Const { v: y, pc: q })) => {
                    let v = match op.opcode {
                        OpCode::CPUI_INT_MULT => x.wrapping_mul(y),
                        OpCode::CPUI_INT_AND => x & y,
                        OpCode::CPUI_INT_OR => x | y,
                        OpCode::CPUI_INT_XOR => x ^ y,
                        OpCode::CPUI_INT_LEFT => x.checked_shl(y as u32).unwrap_or(0),
                        OpCode::CPUI_INT_RIGHT => x.checked_shr(y as u32).unwrap_or(0),
                        _ => x.checked_shr((y as u32).saturating_mul(8)).unwrap_or(0),
                    };
                    Some(Val::Const { v, pc: p || q })
                }
                _ => None,
            },
            _ => None,
        };
        self.write(out, folded);
    }
}

/// Read an `n`-byte two's-complement value as a signed offset.
fn sign_extend(v: u64, size: u32) -> i64 {
    if size == 0 || size >= 8 {
        return v as i64;
    }
    let bits = 8 * size;
    let sign = 1u64 << (bits - 1);
    let v = v & mask(size);
    if v & sign != 0 {
        (v | !mask(size)) as i64
    } else {
        v as i64
    }
}

// --- decoding ----------------------------------------------------------------

/// One instruction's byte length and whole p-code, kept only long enough to
/// interpret it. The xref walk's own [`super::xrefs`] capture is richer (it also
/// renders the assembly); this one exists for the bounded probes, which run
/// before the walk and would pay for a render nothing reads.
struct Insn {
    len: u32,
    ops: Vec<FullOp>,
}

#[derive(Default)]
struct Capture {
    ops: Vec<FullOp>,
}

impl PcodeEmit for Capture {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push(FullOp { opcode: opc, out: outvar.cloned(), ins: vars.to_vec() });
    }
}

fn decode(translate: &dyn Translate, vma: u64, code_space: &Rc<AddrSpace>) -> Option<Insn> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = Capture::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        translate.one_instruction(&mut cap, &addr)
    }));
    match decoded {
        Ok(Ok(len)) if len > 0 => Some(Insn { len: len as u32, ops: cap.ops }),
        _ => None,
    }
}

// --- detection ---------------------------------------------------------------

/// The register an out-of-line PC thunk (`__x86.get_pc_thunk.bx`) delivers its
/// caller's return address in, or `None` when `entry` is not one.
///
/// Probed exactly as [`super::xrefs`] probes a forwarding veneer: at most
/// [`MAX_THUNK_INSNS`] instructions, decoded on demand, with the return address
/// the caller pushed already on the stack.
fn pc_thunk_reg(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    ctx: &Ctx,
    entry: u64,
) -> Option<VarnodeData> {
    let mut m = Machine::default();
    m.seed(&ctx.sp, Val::Stack(0));
    m.stack.insert(0, (PROBE_RET, true));
    let mut vma = entry;
    for _ in 0..MAX_THUNK_INSNS {
        let insn = decode(translate, vma, code_space)?;
        let fall = vma.wrapping_add(u64::from(insn.len));
        // Read the answer BEFORE the `ret` is interpreted: a return pops the very
        // value the thunk was probed with into the program counter, which would
        // otherwise look like the register the thunk delivers in.
        if insn.ops.iter().any(|o| o.opcode == OpCode::CPUI_RETURN) {
            return holder_of(&m, ctx, PROBE_RET);
        }
        for op in &insn.ops {
            m.apply(op, ctx, fall);
        }
        vma = fall;
    }
    None
}

/// The register currently holding `want` as a `pc`-tainted constant.
///
/// Restricted to the register space: the same value is routinely live in a
/// temporary at the same moment (SLEIGH lifts `pop ebx` to a `LOAD` into a
/// unique followed by a copy), and a temporary is not something a later
/// instruction can read.
fn holder_of(m: &Machine, ctx: &Ctx, want: u64) -> Option<VarnodeData> {
    let space = ctx.sp.space.as_ref()?;
    let reg_index = space.get_index();
    let mut best: Option<Key> = None;
    for (&k, v) in m.vals.iter() {
        if k.0 != reg_index {
            continue;
        }
        if let Val::Const { v: got, pc: true } = v {
            if *got == want && (best.is_none() || Some(k) < best) {
                best = Some(k);
            }
        }
    }
    let (_, offset, size) = best?;
    Some(VarnodeData { space: Some(Rc::clone(space)), offset, size })
}

/// Run a linear forward interpretation and report the first address after which
/// some register holds `want` as a `pc`-tainted constant.
///
/// Linear is exactly right for what it is used for — the base-establishing idiom
/// sits in a straight-line prologue — and the walk stops at the first control
/// flow that leaves it, with the sole exception of the idiom's own `call` to its
/// fall-through, which SLEIGH lifts to a branch to the next instruction.
fn establish(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    ctx: &Ctx,
    thunks: &mut HashMap<u64, Option<VarnodeData>>,
    start: u64,
    limit: usize,
    want: u64,
    stop_at: &dyn Fn(u64) -> bool,
) -> Option<(u64, VarnodeData)> {
    let mut m = Machine::default();
    m.seed(&ctx.sp, Val::Stack(0));
    let mut vma = start;
    for _ in 0..limit {
        if stop_at(vma) {
            return None;
        }
        let insn = decode(translate, vma, code_space)?;
        let fall = vma.wrapping_add(u64::from(insn.len));

        // An out-of-line thunk delivers the return address in a register the
        // callee names; the call's own p-code cannot say which one.
        let mut delivered = None;
        for op in &insn.ops {
            if op.opcode == OpCode::CPUI_CALL {
                if let Some(t) = op.ins.first().map(|vn| vn.offset) {
                    let reg = thunks
                        .entry(t)
                        .or_insert_with(|| pc_thunk_reg(translate, code_space, ctx, t))
                        .clone();
                    delivered = reg;
                }
            }
        }
        for op in &insn.ops {
            m.apply(op, ctx, fall);
        }
        if let Some(reg) = &delivered {
            m.write(reg, Some(Val::Const { v: fall, pc: true }));
        }
        if let Some(reg) = holder_of(&m, ctx, want) {
            return Some((vma, reg));
        }

        // Stop at anything that does not simply fall through. The idiom's own
        // `call <next>` lifts to a branch whose target IS the fall-through, so
        // it survives this test; a real branch, a return or a halt does not.
        let leaves = insn.ops.iter().any(|op| match op.opcode {
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                op.ins.first().is_none_or(|vn| vn.offset != fall)
            }
            OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => true,
            _ => false,
        });
        if leaves {
            return None;
        }
        vma = fall;
    }
    None
}

/// The module's PIC base, or `None` when the program has none this can prove.
///
/// Every seed's prologue is probed; the answer stands only if each idiom that
/// fires agrees on one register and that register's value is the image's own
/// `_GLOBAL_OFFSET_TABLE_`. Disagreement is treated as "this is not the ABI's
/// GOT register" and claims nothing.
pub(super) fn detect(
    file: &object::File,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    ctx: &Ctx,
    seeds: &std::collections::BTreeSet<u64>,
) -> Option<PicBase> {
    let got = got_base(file)?;
    let mut thunks: HashMap<u64, Option<VarnodeData>> = HashMap::new();
    let mut found: Option<VarnodeData> = None;
    let never = |_: u64| false;
    for &seed in seeds {
        let Some((_, reg)) =
            establish(translate, code_space, ctx, &mut thunks, seed, MAX_PROLOGUE_INSNS, got, &never)
        else {
            continue;
        };
        match &found {
            None => found = Some(reg),
            Some(prev) if *prev == reg => {}
            // Two idioms naming different registers: the module has no single
            // GOT register, so no function may inherit one.
            Some(_) => return None,
        }
    }
    found.map(|reg| PicBase { reg, value: got })
}

// --- per-function admission --------------------------------------------------

/// Where in `body` the base may be believed, or `None` to claim nothing.
///
/// `body` is every instruction the walk decoded for one function, in address
/// order. Three answers, and the third is the point of the whole rule:
///
///  * no write of the base register at all → the value is what the ABI says the
///    function inherited, live throughout;
///  * the body establishes the base itself → live from the establishment up to
///    the next write of the register, which in GCC output is the epilogue's
///    `mov ebx,[ebp-0x4]` restore;
///  * no provable establishment, but the register IS written → nothing is
///    claimed. A body that uses the register for its own purposes must
///    contribute no references rather than wrong ones.
///
/// Both live windows are read in address order rather than over the CFG. That is
/// the approximation every disassembler makes about a prologue-established base:
/// a body whose base is set up in its prologue has no path into its middle that
/// skips it, and a body that clobbers the register does not then branch backwards
/// into code that uses it as a base.
pub(super) fn scope(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    ctx: &Ctx,
    thunks: &mut HashMap<u64, Option<VarnodeData>>,
    base: &PicBase,
    body: &[BaseCandidate],
) -> Option<Scope> {
    if key_of(&base.reg).is_none() {
        return None;
    }
    if !body.iter().any(|c| c.writes_base) {
        return Some(Scope::Whole);
    }

    let entry = body.first()?.vma;
    let last = body.last()?.vma;
    let outside = |vma: u64| vma > last;
    let (from, reg) = establish(
        translate,
        code_space,
        ctx,
        thunks,
        entry,
        MAX_PROLOGUE_INSNS,
        base.value,
        &outside,
    )?;
    if reg != base.reg {
        return None;
    }
    let until = body
        .iter()
        .find(|c| c.vma > from && c.writes_base)
        .map_or(u64::MAX, |c| c.vma);
    Some(Scope::Between { from, until })
}

/// One walked instruction's contribution to the deferred base-relative pass.
///
/// The pass used to buffer every instruction's whole p-code and re-derive both
/// halves afterwards, which meant cloning every emitted op of every function on
/// any image with a PIC base. Both halves are pure functions of the ops, so they
/// are computed once, in the walk, and only their answers are carried.
pub(super) struct BaseCandidate {
    pub(super) vma: u64,
    /// Does this instruction write the base register (the live window's cut)?
    pub(super) writes_base: bool,
    /// The references it forms through the base — filed only if the scope
    /// admits it. Empty for the overwhelming majority of instructions.
    pub(super) refs: Vec<(u64, XrefKind)>,
}

/// Does `ops` write the base register (the `scope` live-window cut)?
pub(super) fn writes_base(ops: &[FullOp], base: &PicBase) -> bool {
    let Some((idx, lo, size)) = key_of(&base.reg) else { return false };
    let hi = lo.saturating_add(u64::from(size));
    ops.iter().any(|op| {
        op.out.as_ref().and_then(key_of).is_some_and(|(i, o, s)| {
            i == idx && o < hi && o.saturating_add(u64::from(s)) > lo
        })
    })
}

// --- the harvest -------------------------------------------------------------

/// The references one instruction forms through the base register.
///
/// Stateless: the machine is seeded with the base and the stack pointer and
/// nothing else, so a value is reported only when this single instruction
/// computes it. Three shapes are read, and only three — the address a `LOAD`
/// reads, the address a `STORE` writes, and a constant that lands in a
/// *register*, which is the address-taken case (`lea eax,[ebx-0x1119]`).
/// Intermediate temporaries are deliberately not reported: in an indexed access
/// like `mov al,[ebx+edx+0x87]` the array base is computed into one, and filing
/// it would claim a reference to an address the instruction never forms.
pub(super) fn refs_through_base(
    ops: &[FullOp],
    base: &PicBase,
    ctx: &Ctx,
    mapped: &[(u64, u64)],
    fall_through: u64,
) -> Vec<(u64, XrefKind)> {
    let mut m = Machine::default();
    m.seed(&base.reg, Val::Const { v: base.value, pc: true });
    m.seed(&ctx.sp, Val::Stack(0));

    let mut out = Vec::new();
    let admit = |v: u64, kind: XrefKind, out: &mut Vec<(u64, XrefKind)>| {
        if v >= MIN_ADDRESS_VALUE && in_range(mapped, v) {
            out.push((v, kind));
        }
    };
    for op in ops {
        match op.opcode {
            OpCode::CPUI_LOAD => {
                if let Some(a) = op.ins.get(1).and_then(|vn| m.konst(vn)) {
                    admit(a, XrefKind::Read, &mut out);
                }
            }
            OpCode::CPUI_STORE => {
                if let Some(a) = op.ins.get(1).and_then(|vn| m.konst(vn)) {
                    admit(a, XrefKind::Write, &mut out);
                }
            }
            _ => {}
        }
        m.apply(op, ctx, fall_through);
        if let Some(o) = &op.out {
            if ctx.is_register(o) {
                if let Some(v) = m.konst(o) {
                    // A call materializes its own return address in a register on
                    // several architectures; that is the instruction after this
                    // one, not a reference.
                    if v != fall_through {
                        admit(v, XrefKind::Data, &mut out);
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::AddrSpace;

    /// A throwaway `(constant, register, unique)` space triple, in the idiom
    /// [`super::xrefs`]' own tests use. The unique space matters: an address a
    /// real instruction computes lands in a temporary first, and only what
    /// reaches a *register* is address-taken.
    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>, Rc<AddrSpace>) {
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL)),
        )
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    fn ctx(reg: &Rc<AddrSpace>) -> Ctx {
        Ctx { sp: vn(reg, 0x10, 4), data_index: Some(2) }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    const MAPPED: [(u64, u64); 2] = [(0x8048000, 0x8049200), (0x804a000, 0x804a400)];

    #[test]
    fn a_base_relative_lea_is_a_data_reference() {
        let (k, r, _u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        // EAX = EBX + (-0x1119)
        let ops = [op(
            OpCode::CPUI_INT_ADD,
            Some(vn(&r, 0, 4)),
            vec![vn(&r, 0xc, 4), vn(&k, 0xffffeee7, 4)],
        )];
        let got = refs_through_base(&ops, &base, &ctx(&r), &MAPPED, 0);
        assert_eq!(got, vec![(0x8049127, XrefKind::Data)]);
    }

    #[test]
    fn a_base_relative_load_is_a_read_and_a_store_a_write() {
        let (k, r, u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        let tmp = vn(&u, 0x80, 4);
        let addr = vec![
            op(OpCode::CPUI_INT_ADD, Some(tmp.clone()), vec![vn(&r, 0xc, 4), vn(&k, 0xa8, 4)]),
            op(OpCode::CPUI_LOAD, Some(vn(&r, 0, 4)), vec![vn(&k, 2, 4), tmp.clone()]),
        ];
        assert_eq!(
            refs_through_base(&addr, &base, &ctx(&r), &MAPPED, 0),
            vec![(0x804a2e8, XrefKind::Read)]
        );
        let store = vec![
            op(OpCode::CPUI_INT_ADD, Some(tmp.clone()), vec![vn(&r, 0xc, 4), vn(&k, 0xa8, 4)]),
            op(OpCode::CPUI_STORE, None, vec![vn(&k, 2, 4), tmp, vn(&k, 0xa, 1)]),
        ];
        assert_eq!(
            refs_through_base(&store, &base, &ctx(&r), &MAPPED, 0),
            vec![(0x804a2e8, XrefKind::Write)]
        );
    }

    #[test]
    fn an_indexed_access_reports_nothing_rather_than_its_array_base() {
        let (k, r, u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        let tmp = vn(&u, 0x80, 4);
        // $tmp = EBX + 0x87; $tmp = $tmp + EDX (unknown); AL = *$tmp
        let ops = vec![
            op(OpCode::CPUI_INT_ADD, Some(tmp.clone()), vec![vn(&r, 0xc, 4), vn(&k, 0x87, 4)]),
            op(OpCode::CPUI_INT_ADD, Some(tmp.clone()), vec![tmp.clone(), vn(&r, 0x8, 4)]),
            op(OpCode::CPUI_LOAD, Some(vn(&r, 0, 1)), vec![vn(&k, 2, 4), tmp]),
        ];
        // The array base is computed into a temporary, which is not something a
        // later instruction can read, so nothing at all is claimed: the address
        // this instruction actually forms is unknown.
        assert!(refs_through_base(&ops, &base, &ctx(&r), &MAPPED, 0).is_empty());
    }

    #[test]
    fn a_partial_write_to_the_base_invalidates_the_whole_register() {
        let (k, r, _u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        // BL = 0; EAX = EBX + (-0x1119)  -- EBX is no longer known.
        let ops = vec![
            op(OpCode::CPUI_COPY, Some(vn(&r, 0xc, 1)), vec![vn(&k, 0, 1)]),
            op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&r, 0, 4)),
                vec![vn(&r, 0xc, 4), vn(&k, 0xffffeee7, 4)],
            ),
        ];
        assert!(refs_through_base(&ops, &base, &ctx(&r), &MAPPED, 0).is_empty());
    }

    #[test]
    fn an_unmapped_or_small_result_is_not_a_reference() {
        let (k, r, _u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        // EAX = EBX - 0x804a000 -- lands at 0x240, below the address floor.
        let ops = [op(
            OpCode::CPUI_INT_SUB,
            Some(vn(&r, 0, 4)),
            vec![vn(&r, 0xc, 4), vn(&k, 0x804a000, 4)],
        )];
        assert!(refs_through_base(&ops, &base, &ctx(&r), &MAPPED, 0).is_empty());
    }

    #[test]
    fn a_calls_own_return_address_is_not_a_reference() {
        let (k, r, _u) = spaces();
        let base = PicBase { reg: vn(&r, 0xc, 4), value: 0x804a240 };
        let ops = [op(OpCode::CPUI_COPY, Some(vn(&r, 0, 4)), vec![vn(&k, 0x8048ad9, 4)])];
        assert!(refs_through_base(&ops, &base, &ctx(&r), &MAPPED, 0x8048ad9).is_empty());
    }

    #[test]
    fn the_stack_carries_a_pushed_return_address_back_into_a_register() {
        let (k, r, u) = spaces();
        let c = ctx(&r);
        let mut m = Machine::default();
        m.seed(&c.sp, Val::Stack(0));
        let tmp = vn(&u, 0x80, 4);
        // push 0x80489f4:  $tmp = 0x80489f4; ESP = ESP - 4; *ESP = $tmp
        for o in [
            op(OpCode::CPUI_COPY, Some(tmp.clone()), vec![vn(&k, 0x80489f4, 4)]),
            op(OpCode::CPUI_INT_SUB, Some(c.sp.clone()), vec![c.sp.clone(), vn(&k, 4, 4)]),
            op(OpCode::CPUI_STORE, None, vec![vn(&k, 2, 4), c.sp.clone(), tmp.clone()]),
        ] {
            m.apply(&o, &c, 0x80489f4);
        }
        // pop ebx: $tmp = *ESP; ESP = ESP + 4; EBX = $tmp
        for o in [
            op(OpCode::CPUI_LOAD, Some(tmp.clone()), vec![vn(&k, 2, 4), c.sp.clone()]),
            op(OpCode::CPUI_INT_ADD, Some(c.sp.clone()), vec![c.sp.clone(), vn(&k, 4, 4)]),
            op(OpCode::CPUI_COPY, Some(vn(&r, 0xc, 4)), vec![tmp.clone()]),
        ] {
            m.apply(&o, &c, 0x80489f5);
        }
        // add ebx,0x184c
        m.apply(
            &op(
                OpCode::CPUI_INT_ADD,
                Some(vn(&r, 0xc, 4)),
                vec![vn(&r, 0xc, 4), vn(&k, 0x184c, 4)],
            ),
            &c,
            0x80489fb,
        );
        assert_eq!(holder_of(&m, &c, 0x804a240).map(|v| v.offset), Some(0xc));
    }

    #[test]
    fn a_plain_constant_move_cannot_establish_a_base() {
        let (k, r, _u) = spaces();
        let c = ctx(&r);
        let mut m = Machine::default();
        m.apply(
            &op(OpCode::CPUI_COPY, Some(vn(&r, 0xc, 4)), vec![vn(&k, 0x804a240, 4)]),
            &c,
            0x8048000,
        );
        assert_eq!(m.konst(&vn(&r, 0xc, 4)), Some(0x804a240));
        assert!(holder_of(&m, &c, 0x804a240).is_none(), "untainted constant established a base");
    }

    #[test]
    fn sign_extension_reads_a_negative_displacement() {
        assert_eq!(sign_extend(0xffffeee7, 4), -0x1119);
        assert_eq!(sign_extend(0x87, 4), 0x87);
        assert_eq!(sign_extend(0xfc, 1), -4);
    }
}
