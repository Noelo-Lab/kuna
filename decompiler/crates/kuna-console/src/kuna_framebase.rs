//! Where a function's frame pointer and stack pointer sit once its prologue has
//! run, as offsets from the stack pointer at entry (the origin of the
//! decompiler's stack space, so of every `// stack - 0x58` comment).
//!
//! The prologue is read off the raw p-code of the entry's straight-line run,
//! tracking which registers hold `SP@entry + k` and which hold constants. The run
//! ends, before the instruction that does it, at any control flow, at a
//! stack-pointer change that releases stack (an epilogue) and at one that stores
//! anything but a register's entry value (an argument push). The stack pointer
//! is the value the frame setup left it at.
//!
//! The frame pointer is the target's conventional one (`RBP`, `x29`, `r11`/`r7`,
//! `s8`, `s0`, `r31`, `A6`) when the run sets it up in that target's frame
//! shape: pointing at the frame record that holds its own saved value (x86,
//! AArch64, ARM, 68000), a copy of the stack pointer (MIPS, PowerPC, GCC's Thumb
//! frames), or the stack pointer at entry (RISC-V). Optimized code that keeps a
//! pointer to a local in the same register (`LEA RBP,[RSP + 0x30]`) has none.

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

/// The frame pointer, when the function sets one up, and the stack pointer
/// after the frame setup, when the prologue leaves it at a known offset.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrameBases {
    pub frame_pointer: Option<FrameRegister>,
    pub stack_pointer: Option<FrameRegister>,
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

/// The frame pointer and stack pointer of the function at `entry` (an engine
/// byte address), read off its prologue.
pub fn frame_bases(prog: &ConsoleProgram, entry: u64) -> FrameBases {
    let arch = prog.arch();
    let Some(stack) = arch.manage().get_stack_space() else { return FrameBases::default() };
    let Ok(sp) = stack.get_spacebase(0) else { return FrameBases::default() };
    let sp = VarnodeData { space: sp.space, offset: sp.offset, size: sp.size };
    let Some(sp_key) = key(&sp) else { return FrameBases::default() };
    let Some(code_space) = arch.manage().get_default_code_space().cloned() else {
        return FrameBases::default();
    };
    let grows_down = stack.stack_grows_negative();
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

    let mut run = Run { known: vec![(sp_key, Val::Stack(0))], ..Run::default() };
    let mut sp_frame = Some(0i64);
    let mut copied_sp = vec![false; candidates.len()];
    let mut at = entry;
    for _ in 0..MAX_PROLOGUE {
        let mut emit = OpsEmit::default();
        let addr = Address::new(Rc::clone(&code_space), at);
        let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            translate.one_instruction(&mut emit, &addr)
        }));
        let Ok(Ok(len)) = decoded else { break };
        if len <= 0 || emit.ops.iter().any(|(opc, _, _)| is_flow(*opc)) {
            break;
        }
        let (next, entry_stores) = run.step(&emit.ops);
        let (before, after) = (run.stack(&sp), next.stack(&sp));
        if after != before {
            match (before, after) {
                (Some(b), Some(a)) if (a < b) == grows_down && entry_stores => sp_frame = Some(a),
                (_, None) => {
                    sp_frame = None;
                    break;
                }
                _ => break,
            }
        }
        for (i, c) in candidates.iter().enumerate() {
            let Some(k) = key(c) else { continue };
            if next.written.contains(&k) && (!run.written.contains(&k) || next.val(c) != run.val(c)) {
                let value = next.stack(c);
                copied_sp[i] = value.is_some() && (value == before || value == after);
            }
        }
        run = next;
        at = at.wrapping_add(len as u64);
    }

    let word = sp.size as i64;
    let set_up = |i: usize, c: &VarnodeData, fp: i64| {
        let saved = key(c).and_then(|k| run.saved.iter().find(|(r, _)| *r == k).map(|(_, at)| *at));
        let record = saved.is_some_and(|s| fp == s || fp == s + word);
        match shape {
            Shape::Record => record,
            Shape::RecordOrSpCopy => record || copied_sp[i],
            Shape::SpCopy => copied_sp[i],
            Shape::EntrySp => fp == 0,
        }
    };
    let name = |v: &VarnodeData| match v.space.as_ref() {
        Some(space) => translate.get_register_name(space, v.offset, v.size as i32).to_ascii_lowercase(),
        None => String::new(),
    };
    FrameBases {
        frame_pointer: candidates.iter().enumerate().find_map(|(i, c)| {
            let fp = run.stack(c)?;
            set_up(i, c, fp).then(|| FrameRegister { register: name(c), offset: fp })
        }),
        stack_pointer: sp_frame.map(|offset| FrameRegister { register: name(&sp), offset }),
    }
}
