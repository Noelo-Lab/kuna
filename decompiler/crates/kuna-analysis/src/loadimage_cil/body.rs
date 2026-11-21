//! CIL method bodies (II.25.4): the header, the instruction stream, and the
//! evaluation-stack state before each instruction.
//!
//! CIL names no operand types: `add` adds whatever the stack holds. The lift
//! needs them, so [`type_body`] walks the body's control flow carrying the
//! stack as a list of [`Kind`]s, as the verifier does (III.1.8), and records
//! for each reached instruction the depth and the operand class the decoder
//! is told through the `cilctx` decode context. A body it cannot follow is
//! refused whole.

use std::collections::BTreeMap;

/// What one evaluation-stack slot holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `int32`, and every narrower integer.
    I4,
    /// `int64`, `native int`, object references and pointers.
    I8,
    R4,
    R8,
}

/// How a typed location is read onto the stack: its [`Kind`] there and the
/// load class the decoder widens it by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub kind: Kind,
    pub class: u32,
}

/// Load classes: the `k` field of the decode context for a load or a store.
pub const LOAD_S4: u32 = 0;
pub const LOAD_8: u32 = 1;
pub const LOAD_U1: u32 = 2;
pub const LOAD_S1: u32 = 3;
pub const LOAD_U2: u32 = 4;
pub const LOAD_S2: u32 = 5;
pub const LOAD_F4: u32 = 6;
const CLASS_VOID: u32 = 15;

impl Slot {
    pub const REF: Slot = Slot { kind: Kind::I8, class: LOAD_8 };

    pub fn log2_size(self) -> u32 {
        match self.class {
            LOAD_U1 | LOAD_S1 => 0,
            LOAD_U2 | LOAD_S2 => 1,
            LOAD_8 => 3,
            _ => 2,
        }
    }
}

/// The evaluation stack may hold this many slots, and a call may pass this
/// many arguments: the registers the language defines for each.
pub const MAX_STACK: usize = 64;
pub const MAX_ARGS: usize = 16;

/// A method body's header.
#[derive(Debug)]
pub struct BodyHeader {
    /// Bytes before the first instruction.
    pub header_size: u32,
    pub code_size: u32,
    /// The `StandAloneSig` token of the locals, zero for none.
    pub locals_token: u32,
    /// The body has exception-handling clauses.
    pub has_clauses: bool,
}

impl BodyHeader {
    /// Read the tiny or fat header at the start of `b`.
    pub fn parse(b: &[u8]) -> Option<BodyHeader> {
        let first = *b.first()?;
        match first & 3 {
            2 => Some(BodyHeader {
                header_size: 1,
                code_size: u32::from(first >> 2),
                locals_token: 0,
                has_clauses: false,
            }),
            3 => {
                let flags = u16::from_le_bytes(b.get(0..2)?.try_into().ok()?);
                let header_size = u32::from(flags >> 12) * 4;
                if header_size < 12 {
                    return None;
                }
                Some(BodyHeader {
                    header_size,
                    code_size: u32::from_le_bytes(b.get(4..8)?.try_into().ok()?),
                    locals_token: u32::from_le_bytes(b.get(8..12)?.try_into().ok()?),
                    has_clauses: flags & 8 != 0,
                })
            }
            _ => None,
        }
    }
}

/// The target of a `call`, `callvirt`, `newobj` or `ldftn`.
pub struct Callee {
    pub addr: u32,
    /// Arguments passed, the instance included.
    pub args: usize,
    pub ret: Option<Slot>,
}

/// What the loader knows about the tokens a body names.
pub trait Resolve {
    fn callee(&mut self, token: u32) -> Option<Callee>;
    /// The address and type of a static field.
    fn static_field(&mut self, token: u32) -> Option<(u32, Slot)>;
    /// A type token as an array element or a boxed value.
    fn element(&mut self, token: u32) -> Option<Slot>;
    /// The offset and type of an instance field.
    fn instance_field(&mut self, token: u32) -> Option<(u32, Slot)>;
    /// The address of a `#US` string.
    fn string(&mut self, token: u32) -> Option<u32>;
    /// The value `ldtoken` pushes.
    fn handle(&mut self, token: u32) -> u32;
}

/// One reached instruction.
#[derive(Debug, PartialEq, Eq)]
pub struct Insn {
    pub offset: u32,
    /// Bytes the decoder consumes: a `switch` stops before its table.
    pub len: u32,
    /// The `cilctx` value.
    pub context: u32,
    /// A 32-bit operand to rewrite, as `(offset in the body, value)`.
    pub patch: Option<(u32, u32)>,
}

/// Pack the decode context: `ok`, the depth before the instruction, the
/// operand class and the argument count.
fn context(depth: usize, class: u32, args: usize) -> u32 {
    1 << 14 | (depth as u32) << 26 | class << 22 | (args as u32) << 15
}

fn operand_len(code: &[u8], at: usize) -> Option<usize> {
    Some(match *code.get(at)? {
        0x0e..=0x13 | 0x1f | 0x2b..=0x37 | 0xde => 1,
        0x20 | 0x22 | 0x27..=0x29 | 0x38..=0x45 | 0x6f..=0x75 | 0x79 | 0x7b..=0x81 => 4,
        0x8c | 0x8d | 0x8f | 0xa3..=0xa5 | 0xc2 | 0xc6 | 0xd0 | 0xdd => 4,
        0x21 | 0x23 => 8,
        0xfe => match *code.get(at + 1)? {
            0x09..=0x0e => 3,
            0x12 | 0x19 => 2,
            0x06 | 0x07 | 0x15 | 0x16 | 0x1c => 5,
            0x00..=0x05 | 0x0f | 0x11 | 0x13 | 0x14 | 0x17 | 0x18 | 0x1a | 0x1d | 0x1e => 1,
            _ => return None,
        },
        0x24 | 0x77 | 0x78 | 0xa6..=0xb2 | 0xbb..=0xc1 | 0xc4 | 0xc5 | 0xc7..=0xcf | 0xe1..=0xfd | 0xff => {
            return None
        }
        _ => 0,
    })
}

fn numeric(a: Kind, b: Kind) -> Option<(u32, Kind)> {
    Some(match (a, b) {
        (Kind::I4, Kind::I4) => (0, Kind::I4),
        (Kind::I4 | Kind::I8, Kind::I4 | Kind::I8) => (1, Kind::I8),
        (Kind::R4, Kind::R4) => (4, Kind::R4),
        (Kind::R8, Kind::R8) => (5, Kind::R8),
        _ => return None,
    })
}

fn integer(a: Kind, b: Kind) -> Option<(u32, Kind)> {
    numeric(a, b).filter(|&(class, _)| class < 2)
}

fn unary(kind: Kind) -> u32 {
    match kind {
        Kind::I4 => 0,
        Kind::I8 => 1,
        Kind::R4 => 4,
        Kind::R8 => 5,
    }
}

fn stores(value: Kind, into: Kind) -> bool {
    value == into || (value == Kind::I4 && into == Kind::I8)
}

struct Walk<'a> {
    code: &'a [u8],
    stack: Vec<Kind>,
}

impl Walk<'_> {
    fn pop(&mut self) -> Option<Kind> {
        self.stack.pop()
    }

    fn pop_as(&mut self, into: Kind) -> Option<()> {
        stores(self.pop()?, into).then_some(())
    }

    fn pop_int(&mut self) -> Option<Kind> {
        self.pop().filter(|k| matches!(k, Kind::I4 | Kind::I8))
    }

    fn push(&mut self, kind: Kind) -> Option<()> {
        (self.stack.len() < MAX_STACK).then(|| self.stack.push(kind))
    }

    fn u8(&self, at: usize) -> Option<u32> {
        self.code.get(at).map(|&b| u32::from(b))
    }

    fn u16(&self, at: usize) -> Option<u32> {
        Some(u32::from(u16::from_le_bytes(self.code.get(at..at + 2)?.try_into().ok()?)))
    }

    fn u32(&self, at: usize) -> Option<u32> {
        Some(u32::from_le_bytes(self.code.get(at..at + 4)?.try_into().ok()?))
    }

    fn target(&self, next: usize, delta: i64) -> Option<u32> {
        let to = i64::try_from(next).ok()?.checked_add(delta)?;
        (0..self.code.len() as i64).contains(&to).then_some(to as u32)
    }
}

/// The frame of the method a body belongs to.
pub struct Frame<'a> {
    pub args: &'a [Slot],
    /// A local the lift cannot hold is `None`; a body that touches it is refused.
    pub locals: &'a [Option<Slot>],
    pub ret: Option<Slot>,
}

/// Type every instruction of `code` reachable from its first; `None` when the
/// body uses something the lift does not cover or is not verifiable.
pub fn type_body(code: &[u8], frame: &Frame, resolve: &mut dyn Resolve) -> Option<Vec<Insn>> {
    if frame.args.len() > MAX_ARGS || code.len() > u32::MAX as usize / 2 {
        return None;
    }
    let mut seen: BTreeMap<u32, Vec<Kind>> = BTreeMap::new();
    let mut out: BTreeMap<u32, Insn> = BTreeMap::new();
    let mut work = vec![(0u32, Vec::new())];
    while let Some((offset, stack)) = work.pop() {
        if let Some(prior) = seen.get(&offset) {
            if *prior != stack {
                return None;
            }
            continue;
        }
        seen.insert(offset, stack.clone());
        let mut walk = Walk { code, stack };
        let (insn, successors) = step(&mut walk, offset as usize, frame, resolve)?;
        out.insert(offset, insn);
        for to in successors {
            work.push((to, walk.stack.clone()));
        }
    }
    Some(out.into_values().collect())
}

fn step(
    w: &mut Walk,
    at: usize,
    frame: &Frame,
    resolve: &mut dyn Resolve,
) -> Option<(Insn, Vec<u32>)> {
    let op = *w.code.get(at)?;
    let operand = at + 1;
    let mut len = 1 + operand_len(w.code, at)?;
    let next = at.checked_add(len).filter(|&n| n <= w.code.len())?;
    let depth = w.stack.len();
    let (mut class, mut args, mut patch) = (0u32, 0usize, None);
    let mut successors = vec![next as u32];
    let arg = |n: u32| frame.args.get(n as usize).copied();
    let local = |n: u32| frame.locals.get(n as usize).copied().flatten();
    let short = |op: u8| (u32::from(op) + 2) & 3;
    match op {
        0x00 | 0x01 => {}
        0x02..=0x05 | 0x0e => {
            let slot = arg(if op == 0x0e { w.u8(operand)? } else { short(op) })?;
            class = slot.class;
            w.push(slot.kind)?;
        }
        0x10 => {
            let slot = arg(w.u8(operand)?)?;
            class = slot.class;
            w.pop_as(slot.kind)?;
        }
        0x06..=0x09 | 0x11 => {
            let slot = local(if op == 0x11 { w.u8(operand)? } else { short(op) })?;
            class = slot.class;
            w.push(slot.kind)?;
        }
        0x0a..=0x0d | 0x13 => {
            let slot = local(if op == 0x13 { w.u8(operand)? } else { short(op) })?;
            class = slot.class;
            w.pop_as(slot.kind)?;
        }
        0x12 => {
            local(w.u8(operand)?)?;
            w.push(Kind::I8)?;
        }
        0x14 | 0x21 => w.push(Kind::I8)?,
        0x15..=0x20 => w.push(Kind::I4)?,
        0x22 => w.push(Kind::R4)?,
        0x23 => w.push(Kind::R8)?,
        0x25 => {
            let top = *w.stack.last()?;
            w.push(top)?;
        }
        0x26 => {
            w.pop()?;
        }
        0x28 | 0x6f | 0x73 => {
            let callee = resolve.callee(w.u32(operand)?)?;
            args = if op == 0x73 { callee.args.checked_sub(1)? } else { callee.args };
            if callee.args > MAX_ARGS || args > depth {
                return None;
            }
            w.stack.truncate(depth - args);
            match (op, callee.ret) {
                (0x73, _) => w.push(Kind::I8)?,
                (_, Some(ret)) => {
                    class = ret.class;
                    w.push(ret.kind)?;
                }
                _ => class = CLASS_VOID,
            }
            patch = Some((operand as u32, callee.addr));
        }
        0x2a => {
            match frame.ret {
                Some(ret) => w.pop_as(ret.kind)?,
                None => class = CLASS_VOID,
            }
            if !w.stack.is_empty() {
                return None;
            }
            successors.clear();
        }
        0x2b | 0x38 => {
            let delta = if op == 0x2b { i64::from(w.u8(operand)? as u8 as i8) } else { i64::from(w.u32(operand)? as i32) };
            successors = vec![w.target(next, delta)?];
        }
        0x2c | 0x2d | 0x39 | 0x3a => {
            class = match w.pop()? {
                Kind::I4 => 0,
                Kind::I8 => 1,
                _ => return None,
            };
            let delta = if op < 0x38 { i64::from(w.u8(operand)? as u8 as i8) } else { i64::from(w.u32(operand)? as i32) };
            successors.push(w.target(next, delta)?);
        }
        0x2e..=0x37 | 0x3b..=0x44 => {
            let (b, a) = (w.pop()?, w.pop()?);
            class = numeric(a, b)?.0;
            let delta = if op < 0x38 { i64::from(w.u8(operand)? as u8 as i8) } else { i64::from(w.u32(operand)? as i32) };
            successors.push(w.target(next, delta)?);
        }
        0x45 => {
            (w.pop()? == Kind::I4).then_some(())?;
            let count = w.u32(operand)? as usize;
            let table = operand + 4;
            let end = table.checked_add(count.checked_mul(4)?).filter(|&e| e <= w.code.len())?;
            successors = vec![end as u32];
            for i in 0..count {
                successors.push(w.target(end, i64::from(w.u32(table + 4 * i)? as i32))?);
            }
            len = 5;
        }
        0x46..=0x50 => {
            w.pop_as(Kind::I8)?;
            w.push(match op {
                0x46..=0x4b => Kind::I4,
                0x4e => Kind::R4,
                0x4f => Kind::R8,
                _ => Kind::I8,
            })?;
        }
        0x51..=0x57 | 0xdf => {
            w.pop_as(match op {
                0x52..=0x54 => Kind::I4,
                0x56 => Kind::R4,
                0x57 => Kind::R8,
                _ => Kind::I8,
            })?;
            w.pop_as(Kind::I8)?;
        }
        0x58..=0x5b | 0x5d => {
            let (b, a) = (w.pop()?, w.pop()?);
            let (c, kind) = numeric(a, b)?;
            class = c;
            w.push(kind)?;
        }
        0x5c | 0x5e..=0x61 => {
            let (b, a) = (w.pop()?, w.pop()?);
            let (c, kind) = integer(a, b)?;
            class = c;
            w.push(kind)?;
        }
        0x62..=0x64 => {
            w.pop_int()?;
            let value = w.pop_int()?;
            class = unary(value);
            w.push(value)?;
        }
        0x65 | 0x66 => {
            let top = *w.stack.last()?;
            if op == 0x66 && !matches!(top, Kind::I4 | Kind::I8) {
                return None;
            }
            class = unary(top);
        }
        0x67..=0x6e | 0x76 | 0xd1..=0xd3 | 0xe0 => {
            let from = w.pop()?;
            if op == 0x76 && !matches!(from, Kind::I4 | Kind::I8) {
                return None;
            }
            class = unary(from);
            w.push(match op {
                0x6a | 0x6e | 0xd3 | 0xe0 => Kind::I8,
                0x6b => Kind::R4,
                0x6c | 0x76 => Kind::R8,
                _ => Kind::I4,
            })?;
        }
        0x72 => {
            patch = Some((operand as u32, resolve.string(w.u32(operand)?)?));
            w.push(Kind::I8)?;
        }
        0x74 | 0x75 | 0x8e => (*w.stack.last()? == Kind::I8).then_some(())?,
        0x7a => {
            w.pop_as(Kind::I8)?;
            successors.clear();
        }
        0x7b => {
            let (offset, slot) = resolve.instance_field(w.u32(operand)?)?;
            class = slot.class;
            patch = Some((operand as u32, offset));
            w.pop_as(Kind::I8)?;
            w.push(slot.kind)?;
        }
        0x7c => {
            let (offset, _) = resolve.instance_field(w.u32(operand)?)?;
            patch = Some((operand as u32, offset));
            (*w.stack.last()? == Kind::I8).then_some(())?;
        }
        0x7d => {
            let (offset, slot) = resolve.instance_field(w.u32(operand)?)?;
            class = slot.class;
            patch = Some((operand as u32, offset));
            w.pop_as(slot.kind)?;
            w.pop_as(Kind::I8)?;
        }
        0x7e => {
            let (addr, slot) = resolve.static_field(w.u32(operand)?)?;
            class = slot.class;
            patch = Some((operand as u32, addr));
            w.push(slot.kind)?;
        }
        0x7f => {
            let (addr, _) = resolve.static_field(w.u32(operand)?)?;
            patch = Some((operand as u32, addr));
            w.push(Kind::I8)?;
        }
        0x80 => {
            let (addr, slot) = resolve.static_field(w.u32(operand)?)?;
            class = slot.class;
            patch = Some((operand as u32, addr));
            w.pop_as(slot.kind)?;
        }
        0x8c => {
            let slot = resolve.element(w.u32(operand)?)?;
            w.pop_as(slot.kind)?;
            w.push(Kind::I8)?;
        }
        0xa5 => {
            let slot = resolve.element(w.u32(operand)?)?;
            (slot.class <= LOAD_8).then_some(())?;
            class = slot.class;
            w.pop_as(Kind::I8)?;
            w.push(slot.kind)?;
        }
        0x8d => {
            class = resolve.element(w.u32(operand)?)?.log2_size();
            w.pop_int()?;
            w.push(Kind::I8)?;
        }
        0x8f => {
            class = resolve.element(w.u32(operand)?)?.log2_size();
            w.pop_int()?;
            w.pop_as(Kind::I8)?;
            w.push(Kind::I8)?;
        }
        0x90..=0x9a | 0xa3 => {
            let kind = match op {
                0xa3 => {
                    let slot = resolve.element(w.u32(operand)?)?;
                    class = slot.class;
                    slot.kind
                }
                0x90..=0x95 => Kind::I4,
                0x98 => Kind::R4,
                0x99 => Kind::R8,
                _ => Kind::I8,
            };
            w.pop_int()?;
            w.pop_as(Kind::I8)?;
            w.push(kind)?;
        }
        0x9b..=0xa2 | 0xa4 => {
            let kind = match op {
                0xa4 => {
                    let slot = resolve.element(w.u32(operand)?)?;
                    class = slot.class;
                    slot.kind
                }
                0x9c..=0x9e => Kind::I4,
                0xa0 => Kind::R4,
                0xa1 => Kind::R8,
                _ => Kind::I8,
            };
            w.pop_as(kind)?;
            w.pop_int()?;
            w.pop_as(Kind::I8)?;
        }
        0xc3 => matches!(*w.stack.last()?, Kind::R4 | Kind::R8).then_some(())?,
        0xd0 => {
            patch = Some((operand as u32, resolve.handle(w.u32(operand)?)));
            w.push(Kind::I8)?;
        }
        0xfe => {
            let operand = operand + 1;
            match *w.code.get(at + 1)? {
                0x01..=0x05 => {
                    let (b, a) = (w.pop()?, w.pop()?);
                    class = numeric(a, b)?.0;
                    w.push(Kind::I4)?;
                }
                0x06 => {
                    patch = Some((operand as u32, resolve.callee(w.u32(operand)?)?.addr));
                    w.push(Kind::I8)?;
                }
                0x09 => {
                    let slot = arg(w.u16(operand)?)?;
                    class = slot.class;
                    w.push(slot.kind)?;
                }
                0x0b => {
                    let slot = arg(w.u16(operand)?)?;
                    class = slot.class;
                    w.pop_as(slot.kind)?;
                }
                0x0c => {
                    let slot = local(w.u16(operand)?)?;
                    class = slot.class;
                    w.push(slot.kind)?;
                }
                0x0d => {
                    local(w.u16(operand)?)?;
                    w.push(Kind::I8)?;
                }
                0x0e => {
                    let slot = local(w.u16(operand)?)?;
                    class = slot.class;
                    w.pop_as(slot.kind)?;
                }
                0x12 | 0x13 | 0x14 | 0x16 | 0x1e => {}
                _ => return None,
            }
        }
        _ => return None,
    }
    let insn = Insn { offset: at as u32, len: len as u32, context: context(depth, class, args), patch };
    Some((insn, successors))
}
