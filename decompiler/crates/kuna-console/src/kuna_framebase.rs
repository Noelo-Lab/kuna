//! Where a function's frame pointer and stack pointer sit once its prologue has
//! run, as offsets from the stack pointer at entry (the origin of the
//! decompiler's stack space, so of every `// stack - 0x58` comment).
//!
//! The prologue is read off the raw p-code from the entry, tracking which
//! registers hold `SP@entry + k` and which hold constants. Until the stack
//! pointer first moves, the run follows a jump and both ways of a conditional
//! one into code it has not passed, and the way on past any other conditional
//! exit (a shrink-wrapped prologue sets the frame up after an early-exit test or
//! a conditional return); it ends, before the instruction that does it, at any
//! other control flow, a stack-pointer change that releases stack (an
//! epilogue) and one that stores anything but a register's entry value (an
//! argument push), and at any later move once the locals are reserved (`SUB
//! RSP,N`) unless a stack probe came between (a call site's `SUB ESP,0xc`). A
//! call to a get-PC thunk is stepped over. The stack pointer
//! is the deepest value the frame setup leaves it at on any path, and unknown
//! when a path realigns it (`AND RSP,-16`) or moves it in a loop (a stack probe).
//!
//! The frame pointer is the target's conventional one (`RBP`, `x29`, `r11`/`r7`,
//! `s8`, `s0`, `r31`, `A6`) when the run sets it up in that target's frame
//! shape: pointing at the frame record that holds its own saved value (x86,
//! AArch64, ARM, 68000), a copy of the stack pointer (MIPS, PowerPC, GCC's Thumb
//! frames), or the stack pointer at entry (RISC-V). Optimized code that keeps a
//! pointer to a local in the same register (`LEA RBP,[RSP + 0x30]`) has none.
//!
//! The base the stack is read through is the frame pointer on x86 whenever there
//! is one. Elsewhere it is the register more of the function's instructions
//! read: clang's ARM and AArch64 frames keep a frame pointer but reach most
//! locals through `sp`.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::engine::ConsoleProgram;

/// How many instructions the prologue run may span.
const MAX_PROLOGUE: usize = 64;

/// A register and its value as an offset from the stack pointer at entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameRegister {
    pub register: String,
    pub offset: i64,
}

/// Which of the two registers the function's code addresses its stack through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    FramePointer,
    StackPointer,
}

impl Base {
    pub fn as_str(self) -> &'static str {
        match self {
            Base::FramePointer => "frame_pointer",
            Base::StackPointer => "stack_pointer",
        }
    }
}

/// The frame pointer, when the function sets one up, the stack pointer after
/// the frame setup, when the prologue leaves it at a known offset, and which of
/// them the function addresses its stack through.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrameBases {
    pub frame_pointer: Option<FrameRegister>,
    pub stack_pointer: Option<FrameRegister>,
    pub base: Option<Base>,
}

type Key = (i32, u64, u32);
type Op = (OpCode, Option<VarnodeData>, Vec<VarnodeData>);

#[derive(Default)]
struct OpsEmit {
    ops: Vec<Op>,
}

impl kuna_sleigh::translate::PcodeEmit for OpsEmit {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        self.ops.push((opc, outvar.cloned(), vars.to_vec()));
    }
}

fn key(v: &VarnodeData) -> Option<Key> {
    Some((v.space.as_ref()?.get_index(), v.offset, v.size))
}

fn sext(value: i64, size: u32) -> i64 {
    kuna_base::address::sign_extend(value, (size.clamp(1, 8) * 8) as i32 - 1)
}

fn constant(v: &VarnodeData) -> Option<i64> {
    let space = v.space.as_ref()?;
    (space.get_type() == spacetype::IPTR_CONSTANT).then(|| sext(v.offset as i64, v.size))
}

fn overlaps(a: Key, b: Key) -> bool {
    a.0 == b.0 && a.1 < b.1.saturating_add(b.2 as u64) && b.1 < a.1.saturating_add(a.2 as u64)
}

fn is_flow(opc: OpCode) -> bool {
    matches!(
        opc,
        OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_RETURN
    )
}

/// How a target's prologue sets its frame pointer up.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// At the saved frame pointer, or the word above it.
    Record,
    /// `Record`, or a copy of the stack pointer.
    RecordOrSpCopy,
    /// A copy of the stack pointer.
    SpCopy,
    /// The stack pointer at entry.
    EntrySp,
}

/// The conventional frame pointers of a processor, most likely first, and the
/// shape its prologues give them.
fn frame_pointer_names(processor: &str, bits: u32) -> (&'static [&'static str], Shape) {
    match (processor, bits) {
        ("x86", 64) => (&["RBP"], Shape::Record),
        ("x86", 16) => (&["BP"], Shape::Record),
        ("x86", _) => (&["EBP"], Shape::Record),
        ("AARCH64", _) => (&["x29"], Shape::Record),
        ("ARM", _) => (&["r11", "r7"], Shape::RecordOrSpCopy),
        ("68000", _) => (&["A6"], Shape::Record),
        ("MIPS", _) => (&["s8"], Shape::SpCopy),
        ("PowerPC", _) => (&["r31"], Shape::SpCopy),
        ("RISCV", _) => (&["s0"], Shape::EntrySp),
        _ => (&[], Shape::Record),
    }
}

/// A tracked value: `SP@entry + k`, or a constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Val {
    Stack(i64),
    Const(i64),
}

/// The run's state: the values it tracks, the registers written so far, the
/// temporaries holding some register's entry value, and where each register's
/// entry value was first stored.
#[derive(Default, Clone)]
struct Run {
    known: Vec<(Key, Val)>,
    written: BTreeSet<Key>,
    entry_copies: Vec<(Key, Key)>,
    saved: Vec<(Key, i64)>,
}

impl Run {
    fn val(&self, v: &VarnodeData) -> Option<Val> {
        if let Some(c) = constant(v) {
            return Some(Val::Const(c));
        }
        let k = key(v)?;
        self.known.iter().find(|(kk, _)| *kk == k).map(|(_, val)| *val)
    }

    fn stack(&self, v: &VarnodeData) -> Option<i64> {
        match self.val(v)? {
            Val::Stack(off) => Some(off),
            Val::Const(_) => None,
        }
    }

    /// The register whose entry value `v` holds.
    fn entry_of(&self, v: &VarnodeData) -> Option<Key> {
        let k = key(v)?;
        match v.space.as_ref()?.get_type() {
            spacetype::IPTR_PROCESSOR => (!self.written.iter().any(|w| overlaps(*w, k))).then_some(k),
            spacetype::IPTR_INTERNAL => self.entry_copies.iter().find(|(t, _)| *t == k).map(|(_, r)| *r),
            _ => None,
        }
    }

    fn result(&self, opc: OpCode, out: &VarnodeData, ins: &[VarnodeData]) -> Option<Val> {
        use Val::*;
        let a = self.val(ins.first()?)?;
        let b = ins.get(1).and_then(|b| self.val(b));
        let same = ins.len() == 2 && key(&ins[0]) == key(&ins[1]);
        let fit = |c: i64| Const(sext(c, out.size));
        match (opc, a, b) {
            (OpCode::CPUI_COPY, v, _) => Some(v),
            (OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT, Const(c), _) => Some(fit(c)),
            (OpCode::CPUI_INT_ADD, Stack(x), Some(Const(c))) | (OpCode::CPUI_INT_ADD, Const(c), Some(Stack(x))) => {
                Some(Stack(x.wrapping_add(c)))
            }
            (OpCode::CPUI_INT_ADD, Const(x), Some(Const(c))) => Some(fit(x.wrapping_add(c))),
            (OpCode::CPUI_INT_SUB, Stack(x), Some(Const(c))) => Some(Stack(x.wrapping_sub(c))),
            (OpCode::CPUI_INT_SUB, Const(x), Some(Const(c))) => Some(fit(x.wrapping_sub(c))),
            (OpCode::CPUI_INT_MULT, Const(x), Some(Const(c))) => Some(fit(x.wrapping_mul(c))),
            (OpCode::CPUI_INT_LEFT, Const(x), Some(Const(c))) if (0..64).contains(&c) => Some(fit(x << c)),
            (OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR, v, Some(Const(0))) => Some(v),
            (OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND, v, _) if same => Some(v),
            _ => None,
        }
    }

    fn apply(&mut self, (opc, out, ins): &Op) {
        if *opc == OpCode::CPUI_STORE {
            let at = ins.get(1).and_then(|a| self.stack(a));
            let reg = ins.get(2).and_then(|v| self.entry_of(v));
            if let (Some(at), Some(reg)) = (at, reg) {
                if !self.saved.iter().any(|(r, _)| *r == reg) {
                    self.saved.push((reg, at));
                }
            }
            return;
        }
        let Some(out) = out else { return };
        let Some(k) = key(out) else { return };
        let value = if *opc == OpCode::CPUI_CALLOTHER { None } else { self.result(*opc, out, ins) };
        let origin = if *opc == OpCode::CPUI_COPY { ins.first().and_then(|v| self.entry_of(v)) } else { None };
        self.known.retain(|(kk, _)| !overlaps(*kk, k));
        self.entry_copies.retain(|(t, _)| !overlaps(*t, k));
        match out.space.as_ref().map(|s| s.get_type()) {
            Some(spacetype::IPTR_PROCESSOR) => {
                self.written.insert(k);
            }
            Some(spacetype::IPTR_INTERNAL) => {
                if let Some(reg) = origin {
                    self.entry_copies.push((k, reg));
                }
            }
            _ => {}
        }
        if let Some(v) = value {
            self.known.push((k, v));
        }
    }

    /// Run `ops` from this state, and say whether each STORE among them stored
    /// some register's entry value.
    fn step(&self, ops: &[Op]) -> (Run, bool) {
        let mut next = self.clone();
        let mut entry_stores = true;
        for op in ops {
            if op.0 == OpCode::CPUI_STORE {
                entry_stores &= op.2.get(2).and_then(|v| next.entry_of(v)).is_some();
            }
            next.apply(op);
        }
        (next, entry_stores)
    }
}

/// Where a branch goes in the code space, when it names an address there.
fn code_target(ins: &[VarnodeData]) -> Option<u64> {
    let v = ins.first()?;
    (v.space.as_ref()?.get_type() != spacetype::IPTR_CONSTANT).then_some(v.offset)
}

/// Whether an op of the instruction at `at` passes control elsewhere: a branch
/// within its own p-code, to itself or to the next instruction (`REP`,
/// `CMOVcc`) does not.
fn leaves((opc, _, ins): &Op, at: u64, fall: u64) -> bool {
    is_flow(*opc)
        && !(matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
            && code_target(ins).is_none_or(|t| t == at || t == fall))
}

/// One path of the prologue run: its state, where it is, the stack pointer the
/// frame setup has left on it (`None` once it is unknown), whether each frame
/// pointer candidate was last written as a copy of the stack pointer, the stack
/// pointer at each address it has passed, whether it has moved it yet, and
/// whether it has reserved its locals with no stack probe since.
#[derive(Clone)]
struct Path {
    run: Run,
    at: u64,
    sp_frame: Option<i64>,
    copied_sp: Vec<bool>,
    passed: Vec<(u64, Option<i64>)>,
    moved: bool,
    reserved: bool,
}

/// The frame pointer and stack pointer of the function at `entry` (an engine
/// byte address), read off its prologue; `body` is the function's instruction
/// addresses, the only places a branch in the run is followed to.
pub fn frame_bases(prog: &ConsoleProgram, entry: u64, body: &[u64]) -> FrameBases {
    let arch = prog.arch();
    let Some(stack) = arch.manage().get_stack_space() else { return FrameBases::default() };
    let Ok(sp) = stack.get_spacebase(0) else { return FrameBases::default() };
    let sp = VarnodeData { space: sp.space, offset: sp.offset, size: sp.size };
    let Some(sp_key) = key(&sp) else { return FrameBases::default() };
    let Some(code_space) = arch.manage().get_default_code_space().cloned() else {
        return FrameBases::default();
    };
    let grows_down = stack.stack_grows_negative();
    let deeper = |a: i64, b: i64| if grows_down { a < b } else { a > b };
    let translate = arch.translate();
    let mut fields = arch.archid.split(':');
    let processor = fields.next().unwrap_or("");
    let bits = fields.nth(1).and_then(|b| b.parse::<u32>().ok()).unwrap_or(0);
    let (names, shape) = frame_pointer_names(processor, bits);
    let candidates: Vec<VarnodeData> = names
        .iter()
        .filter_map(|name| translate.probe_register(name))
        .map(|r| VarnodeData { space: r.space, offset: r.offset, size: r.size })
        .filter(|r| key(r).is_some_and(|k| k != sp_key))
        .collect();
    let in_body: BTreeSet<u64> = body.iter().copied().collect();
    let decode = |at: u64| {
        let mut emit = OpsEmit::default();
        let addr = Address::new(Rc::clone(&code_space), at);
        let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            translate.one_instruction(&mut emit, &addr)
        }));
        match decoded {
            Ok(Ok(len)) if len > 0 => Some((emit.ops, len as u64)),
            _ => None,
        }
    };

    let word = sp.size as i64;
    // A get-PC thunk (`CALL __x86.get_pc_thunk.bx`) reads its own return address
    // off the stack into a register and returns: the call moves nothing, and the
    // registers the thunk writes come back unknown.
    let thunk = |run: &Run, target: u64| -> Option<Vec<Op>> {
        let (first, len) = decode(target)?;
        let (second, _) = decode(target.wrapping_add(len))?;
        if first.iter().any(|op| is_flow(op.0)) || !second.iter().any(|op| op.0 == OpCode::CPUI_RETURN) {
            return None;
        }
        let top = run.stack(&sp)?.wrapping_sub(word);
        let mut inner = run.clone();
        inner.known.retain(|(k, _)| *k != sp_key);
        inner.known.push((sp_key, Val::Stack(top)));
        let mut reads_return = false;
        for op in &first {
            reads_return |= op.0 == OpCode::CPUI_LOAD && op.2.get(1).and_then(|a| inner.stack(a)) == Some(top);
            inner.apply(op);
        }
        let clobbered = first.iter().filter_map(|op| op.1.clone()).filter(|o| {
            o.space.as_ref().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR) && key(o) != Some(sp_key)
        });
        reads_return.then(|| clobbered.map(|o| (OpCode::CPUI_CALLOTHER, Some(o), Vec::new())).collect())
    };

    let start = Path {
        run: Run { known: vec![(sp_key, Val::Stack(0))], ..Run::default() },
        at: entry,
        sp_frame: Some(0),
        copied_sp: vec![false; candidates.len()],
        passed: Vec::new(),
        moved: false,
        reserved: false,
    };
    let mut work = vec![start];
    let mut ended: Vec<Path> = Vec::new();
    let mut forks = 0;
    while let Some(mut path) = work.pop() {
        for _ in 0..MAX_PROLOGUE {
            let Some((ops, len)) = decode(path.at) else { break };
            let before = path.run.stack(&sp);
            let fall = path.at.wrapping_add(len);
            let flow = ops.iter().position(|op| leaves(op, path.at, fall));
            let skip = flow.and_then(|i| {
                ops[..i].iter().position(|(opc, _, ins)| *opc == OpCode::CPUI_CBRANCH && code_target(ins) == Some(fall))
            });
            let mut fork_to = None;
            let through_thunk = flow.filter(|_| skip.is_none()).and_then(|i| match &ops[i] {
                (OpCode::CPUI_CALL, _, ins) => code_target(ins).and_then(|t| thunk(&path.run, t)),
                _ => None,
            });
            let next_at = match flow.map(|i| &ops[i]) {
                None => fall,
                Some(_) if through_thunk.is_some() => fall,
                Some((opc, _, ins)) if matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH) => {
                    let target = code_target(ins).unwrap_or(u64::MAX);
                    let conditional = *opc == OpCode::CPUI_CBRANCH || skip.is_some();
                    let looped = path.passed.iter().find(|(a, _)| *a == target).map(|(_, s)| *s);
                    if target <= path.at && looped.is_some_and(|s| s != before) {
                        path.sp_frame = None;
                        break;
                    }
                    let unseen = looped.is_none() && in_body.contains(&target);
                    if path.moved || (!conditional && !unseen) {
                        break;
                    }
                    if !conditional {
                        target
                    } else {
                        fork_to = unseen.then_some(target);
                        fall
                    }
                }
                Some(_) if skip.is_some() && !path.moved => fall,
                Some(_) => break,
            };
            if let Some(target) = fork_to.filter(|_| forks < 8) {
                forks += 1;
                let mut fork = path.clone();
                fork.at = target;
                fork.passed.push((path.at, before));
                let taken: Vec<Op> = ops.iter().filter(|op| !is_flow(op.0)).cloned().collect();
                fork.run = fork.run.step(&taken).0;
                work.push(fork);
            }
            let applied = if next_at == fall { skip.unwrap_or(ops.len()) } else { ops.len() };
            let ops: Vec<Op> = match through_thunk {
                Some(clobbers) => clobbers,
                None => ops.into_iter().take(applied).filter(|op| !is_flow(op.0)).collect(),
            };
            let (next, entry_stores) = path.run.step(&ops);
            let after = next.stack(&sp);
            let stores: Vec<Option<i64>> =
                ops.iter().filter(|op| op.0 == OpCode::CPUI_STORE).map(|op| op.2.get(1).and_then(|a| next.stack(a))).collect();
            if after == before && before.is_some() && stores.contains(&before) {
                path.reserved = false;
            }
            if after != before {
                path.moved = true;
                match (before, after) {
                    (Some(b), Some(a)) if deeper(a, b) && entry_stores && !path.reserved => {
                        path.sp_frame = path.sp_frame.map(|_| a);
                        path.reserved = stores.is_empty();
                    }
                    (_, None) => {
                        path.sp_frame = None;
                        break;
                    }
                    _ => break,
                }
            }
            for (i, c) in candidates.iter().enumerate() {
                let Some(k) = key(c) else { continue };
                if next.written.contains(&k) && (!path.run.written.contains(&k) || next.val(c) != path.run.val(c)) {
                    let value = next.stack(c);
                    path.copied_sp[i] = value.is_some() && (value == before || value == after);
                }
            }
            path.passed.push((path.at, before));
            path.run = next;
            path.at = next_at;
        }
        ended.push(path);
    }

    let frame_pointer = |path: &Path| {
        candidates.iter().enumerate().find_map(|(i, c)| {
            let fp = path.run.stack(c)?;
            let saved = key(c).and_then(|k| path.run.saved.iter().find(|(r, _)| *r == k).map(|(_, at)| *at));
            let record = saved.is_some_and(|s| fp == s || fp == s + word);
            let set_up = match shape {
                Shape::Record => record,
                Shape::RecordOrSpCopy => record || path.copied_sp[i],
                Shape::SpCopy => path.copied_sp[i],
                Shape::EntrySp => fp == 0,
            };
            set_up.then_some((c, fp))
        })
    };
    let unknown = ended.iter().any(|p| p.sp_frame.is_none());
    let deepest = ended
        .iter()
        .filter(|p| p.sp_frame.is_some())
        .reduce(|a, b| match (a.sp_frame, b.sp_frame) {
            (Some(x), Some(y)) if deeper(y, x) || (y == x && frame_pointer(a).is_none()) => b,
            _ => a,
        });
    let chosen = deepest.filter(|_| !unknown).or_else(|| ended.iter().find(|p| frame_pointer(p).is_some()));
    let name = |v: &VarnodeData| match v.space.as_ref() {
        Some(space) => translate.get_register_name(space, v.offset, v.size as i32).to_ascii_lowercase(),
        None => String::new(),
    };
    let fp = chosen.and_then(frame_pointer);
    let stack_pointer = if unknown { None } else { deepest.and_then(|p| p.sp_frame) };
    let base = match (&fp, stack_pointer) {
        (Some((c, _)), Some(_)) if processor != "x86" => {
            let (fp_uses, sp_uses) = uses(body, &decode, c, &sp);
            Some(if fp_uses >= sp_uses { Base::FramePointer } else { Base::StackPointer })
        }
        (Some(_), _) => Some(Base::FramePointer),
        (None, Some(_)) => Some(Base::StackPointer),
        (None, None) => None,
    };
    FrameBases {
        frame_pointer: fp.map(|(c, offset)| FrameRegister { register: name(c), offset }),
        stack_pointer: stack_pointer.map(|offset| FrameRegister { register: name(&sp), offset }),
        base,
    }
}

/// How many of `body`'s instructions read the frame pointer and how many the
/// stack pointer, leaving out control flow and whatever writes either one (the
/// prologue, the epilogue, pushes and pops).
fn uses(body: &[u64], decode: &dyn Fn(u64) -> Option<(Vec<Op>, u64)>, fp: &VarnodeData, sp: &VarnodeData) -> (usize, usize) {
    let (Some(fk), Some(sk)) = (key(fp), key(sp)) else { return (0, 0) };
    let (mut fp_uses, mut sp_uses) = (0, 0);
    for &at in body {
        let Some((ops, len)) = decode(at) else { continue };
        let touches = |k: Key, op: &Op| op.1.as_ref().and_then(key).is_some_and(|o| overlaps(o, k));
        if ops.iter().any(|op| leaves(op, at, at.wrapping_add(len)) || touches(fk, op) || touches(sk, op)) {
            continue;
        }
        let reads = |k: Key| ops.iter().any(|op| op.2.iter().filter_map(key).any(|i| overlaps(i, k)));
        fp_uses += reads(fk) as usize;
        sp_uses += reads(sk) as usize;
    }
    (fp_uses, sp_uses)
}
