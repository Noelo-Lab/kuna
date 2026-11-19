//! (kuna) WebAssembly function bodies lowered from stack form to register form.
//!
//! A wasm body is validated code for a stack machine, so the height of the
//! operand stack at every instruction is a static fact. The lowering walks one
//! body with the validation algorithm's control and type stacks and writes one
//! fixed 16-byte record per operation that still does something once the
//! stack is gone:
//!
//! ```text
//! u16 opcode   the wasm opcode (0xfcNN for the 0xfc prefix), or 0xffNN for a
//!              record the lowering adds
//! u8  type     0 i32, 1 i64, 2 f32, 3 f64, where the opcode alone is untyped
//! u8  zero
//! u32 height   operand-stack height before the operation (a slot index for
//!              the added records)
//! u32 lo, hi   immediates; a branch or call target is an absolute address
//! ```
//!
//! `Wasm.slaspec` gives each record its p-code, reading operand `height - 1`
//! and so on as register slots. A body opens with a `param` record for each
//! parameter it reads and a `zero` record for each other local it reads. `block`, `loop`, `end`, `nop`, `drop` and the
//! reinterpret casts leave no record; `if` is a `br_unless`; a branch that
//! carries values to a different height is preceded by `move` records; a call
//! is `arg` records, the call, and `result` records; `br_table` is a bounds
//! test and an indirect jump through a table of addresses stored after it.
//! Code that validation proves unreachable is dropped.
//!
//! A body that uses an operation outside the supported set, or that does not
//! validate, becomes a single `unsupported` record and a reason.

use std::collections::BTreeMap;

use crate::loader::format::wasm::{FuncType, GlobalType, Module, Reader, ValType, WasmResult};

pub const RECORD_LEN: u32 = 16;

const OP_BR: u16 = 0x0c;
const OP_BR_IF: u16 = 0x0d;
const OP_RETURN: u16 = 0x0f;
const OP_CALL_INDIRECT: u16 = 0x11;
const OP_PARAM: u16 = 0xff00;
const OP_ZERO: u16 = 0xff01;
const OP_ARG: u16 = 0xff02;
const OP_RESULT: u16 = 0xff03;
const OP_RETVAL: u16 = 0xff04;
const OP_MOVE: u16 = 0xff05;
const OP_BR_UNLESS: u16 = 0xff06;
const OP_BR_TABLE_DEFAULT: u16 = 0xff07;
const OP_BR_TABLE: u16 = 0xff08;
const OP_UNSUPPORTED: u16 = 0xff09;

/// How many locals, operand-stack slots, arguments and results `Wasm.slaspec`
/// names a register for. A body that needs more of one kind is refused.
const MAX_LOCALS: usize = 8192;
const MAX_SLOTS: usize = 1024;
const MAX_ARGS: usize = 1024;
const MAX_RESULTS: usize = 1;
/// A body may lower to this many records per byte of its code and no more.
/// Compiled code stays under one; the bound keeps a hostile body, whose
/// branches each carry a tall stack, from lowering to gigabytes.
const RECORDS_PER_BYTE: usize = 4;

/// The type field of a `global.get` or `global.set` of the stack pointer.
const TYPE_STACK_POINTER: u8 = 4;

/// What a record's `lo` word becomes once the image is laid out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fix {
    None,
    /// A label of the same function.
    Label(u32),
    /// The entry of a function, by function index.
    Function(u32),
    /// The base address of a table, by table index.
    Table(u32),
}

#[derive(Clone, Debug)]
enum Item {
    Record { op: u16, ty: u8, height: u32, lo: u32, hi: u32, fix: Fix },
    /// A `br_table` jump table: one label per entry.
    Table(Vec<u32>),
}

impl Item {
    fn len(&self) -> u32 {
        match self {
            Item::Record { .. } => RECORD_LEN,
            // cast: a table has at most as many entries as its body has bytes
            Item::Table(labels) => (labels.len() as u32 * 4).next_multiple_of(RECORD_LEN),
        }
    }
}

/// One function body in register form, addresses still symbolic.
#[derive(Clone, Debug, Default)]
pub struct Lowered {
    items: Vec<Item>,
    /// Item index each label is bound to.
    labels: Vec<u32>,
    /// Why the body was not lowered, when it was not. Such a body may hold
    /// any instruction, a table write included.
    pub refused: Option<String>,
}

impl Lowered {
    /// Bytes the encoded body takes.
    pub fn len(&self) -> u32 {
        self.items.iter().map(Item::len).sum()
    }

    /// Append the encoded body, placed at address `base`, to `out`.
    pub fn encode(&self, base: u32, resolve: impl Fn(Fix) -> u32, out: &mut Vec<u8>) {
        let mut offsets = Vec::with_capacity(self.items.len() + 1);
        let mut offset = 0u32;
        for item in &self.items {
            offsets.push(offset);
            offset += item.len();
        }
        offsets.push(offset);
        // cast: u32 to usize widens
        let label = |id: u32| base.wrapping_add(offsets[self.labels[id as usize] as usize]);
        for item in &self.items {
            match item {
                Item::Record { op, ty, height, lo, hi, fix } => {
                    let lo = match *fix {
                        Fix::None => *lo,
                        Fix::Label(id) => label(id),
                        other => resolve(other),
                    };
                    out.extend_from_slice(&(u32::from(*op) | u32::from(*ty) << 16).to_le_bytes());
                    out.extend_from_slice(&height.to_le_bytes());
                    out.extend_from_slice(&lo.to_le_bytes());
                    out.extend_from_slice(&hi.to_le_bytes());
                }
                Item::Table(labels) => {
                    let start = out.len();
                    for &id in labels {
                        out.extend_from_slice(&label(id).to_le_bytes());
                    }
                    // cast: u32 to usize widens
                    out.resize(start + item.len() as usize, 0);
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameKind {
    Function,
    Block,
    Loop,
    If,
    Else,
}

struct Frame {
    kind: FrameKind,
    /// Operand-stack height under the frame's parameters.
    height: usize,
    params: Vec<ValType>,
    results: Vec<ValType>,
    /// Where a branch to the frame lands: a loop's start, any other's end.
    label: u32,
    /// An `if`'s false arm, until `else` or `end` binds it.
    else_label: Option<u32>,
    unreachable: bool,
}

enum Refusal {
    Malformed(String),
    Unsupported(String),
}

type Step<T> = Result<T, Refusal>;

fn malformed<T>(what: &str) -> Step<T> {
    Err(Refusal::Malformed(what.to_string()))
}

fn unsupported<T>(what: impl Into<String>) -> Step<T> {
    Err(Refusal::Unsupported(what.into()))
}

fn read<T>(value: WasmResult<T>) -> Step<T> {
    value.map_err(Refusal::Malformed)
}

fn type_code(ty: ValType) -> Step<u8> {
    match ty {
        ValType::I32 => Ok(0),
        ValType::I64 => Ok(1),
        ValType::F32 => Ok(2),
        ValType::F64 => Ok(3),
        ValType::V128 => unsupported("a v128 value"),
        ValType::Ref => unsupported("a reference value"),
    }
}

fn check_arity(ty: &FuncType) -> Step<()> {
    if ty.params.len() > MAX_ARGS {
        return unsupported(format!("a function type with more than {MAX_ARGS} parameters"));
    }
    if ty.results.len() > MAX_RESULTS {
        return unsupported("a function type with more than one result");
    }
    Ok(())
}

/// Operand and result types of a numeric instruction, by lowered opcode.
fn numeric_signature(op: u16) -> Option<(&'static [ValType], ValType)> {
    use ValType::{F32, F64, I32, I64};
    Some(match op {
        0x45 | 0x67..=0x69 | 0xc0 | 0xc1 => (&[I32], I32),
        0x46..=0x4f | 0x6a..=0x78 => (&[I32, I32], I32),
        0x50 | 0xa7 => (&[I64], I32),
        0x51..=0x5a => (&[I64, I64], I32),
        0x5b..=0x60 => (&[F32, F32], I32),
        0x61..=0x66 => (&[F64, F64], I32),
        0x79..=0x7b | 0xc2..=0xc4 => (&[I64], I64),
        0x7c..=0x8a => (&[I64, I64], I64),
        0x8b..=0x91 => (&[F32], F32),
        0x92..=0x98 => (&[F32, F32], F32),
        0x99..=0x9f => (&[F64], F64),
        0xa0..=0xa6 => (&[F64, F64], F64),
        0xa8 | 0xa9 | 0xbc | 0xfc00 | 0xfc01 => (&[F32], I32),
        0xaa | 0xab | 0xfc02 | 0xfc03 => (&[F64], I32),
        0xac | 0xad => (&[I32], I64),
        0xae | 0xaf | 0xfc04 | 0xfc05 => (&[F32], I64),
        0xb0 | 0xb1 | 0xbd | 0xfc06 | 0xfc07 => (&[F64], I64),
        0xb2 | 0xb3 | 0xbe => (&[I32], F32),
        0xb4 | 0xb5 => (&[I64], F32),
        0xb6 => (&[F64], F32),
        0xb7 | 0xb8 => (&[I32], F64),
        0xb9 | 0xba | 0xbf => (&[I64], F64),
        0xbb => (&[F32], F64),
        _ => return None,
    })
}

/// The value type a load pushes or a store pops.
fn memory_value(op: u16) -> ValType {
    match op {
        0x28 | 0x2c..=0x2f | 0x36 | 0x3a | 0x3b => ValType::I32,
        0x2a | 0x38 => ValType::F32,
        0x2b | 0x39 => ValType::F64,
        _ => ValType::I64,
    }
}

/// What every body of one module is lowered against.
pub struct Lowering<'a> {
    module: &'a Module,
    bytes: &'a [u8],
    globals: Vec<GlobalType>,
    stack_pointer: Option<u32>,
}

struct Lowerer<'a> {
    module: &'a Module,
    reader: Reader<'a>,
    globals: &'a [GlobalType],
    stack_pointer: Option<u32>,
    locals: Vec<ValType>,
    /// How many of `locals` are parameters.
    params: usize,
    /// Which locals a `local.get` reads.
    read: Vec<bool>,
    /// The most records the body may lower to.
    budget: usize,
    stack: Vec<ValType>,
    frames: Vec<Frame>,
    /// Blocks opened inside unreachable code, which are skipped whole.
    dead_depth: u32,
    out: Lowered,
}

impl Lowerer<'_> {
    fn height(&self) -> u32 {
        // cast: the stack is no taller than the body is long
        self.stack.len() as u32
    }

    fn record(&mut self, op: u16, ty: u8, height: u32, lo: u32, hi: u32, fix: Fix) {
        self.out.items.push(Item::Record { op, ty, height, lo, hi, fix });
    }

    /// Refuse the body when `more` further records would pass its budget.
    fn spend(&self, more: usize) -> Step<()> {
        if self.out.items.len().saturating_add(more) > self.budget {
            return unsupported(format!("branches that lower to more than {RECORDS_PER_BYTE} records per byte"));
        }
        Ok(())
    }

    fn new_label(&mut self) -> u32 {
        // cast: one label per control instruction at most
        let id = self.out.labels.len() as u32;
        self.out.labels.push(u32::MAX);
        id
    }

    fn bind(&mut self, label: u32) {
        // cast: u32 to usize widens; items are bounded by the body length
        self.out.labels[label as usize] = self.out.items.len() as u32;
    }

    fn frame(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("the function frame outlives its body")
    }

    fn push(&mut self, ty: ValType) {
        self.stack.push(ty);
    }

    fn pop(&mut self, want: ValType) -> Step<()> {
        let floor = self.frames.last().map_or(0, |frame| frame.height);
        if self.stack.len() <= floor {
            return malformed("operand stack underflow");
        }
        match self.stack.pop() {
            Some(ty) if ty == want => Ok(()),
            _ => malformed("operand type mismatch"),
        }
    }

    fn pop_any(&mut self) -> Step<ValType> {
        let floor = self.frames.last().map_or(0, |frame| frame.height);
        if self.stack.len() <= floor {
            return malformed("operand stack underflow");
        }
        Ok(self.stack.pop().expect("checked against the frame floor"))
    }

    fn set_unreachable(&mut self) {
        let height = self.frame().height;
        self.stack.truncate(height);
        self.frame().unreachable = true;
    }

    fn block_type(&mut self) -> Step<FuncType> {
        let code = read(self.reader.s33())?;
        if code >= 0 {
            // cast: a non-negative s33 fits u32
            return match self.module.func_type(code as u32) {
                Some(ty) => Ok(ty.clone()),
                None => malformed("block type index out of range"),
            };
        }
        // cast: the single-byte encodings are the low 7 bits of the s33
        let results = match (code & 0x7f) as u8 {
            0x40 => Vec::new(),
            0x7f => vec![ValType::I32],
            0x7e => vec![ValType::I64],
            0x7d => vec![ValType::F32],
            0x7c => vec![ValType::F64],
            0x7b => vec![ValType::V128],
            _ => vec![ValType::Ref],
        };
        Ok(FuncType { params: Vec::new(), results })
    }

    fn open(&mut self, kind: FrameKind, ty: FuncType) -> Step<()> {
        for &param in ty.params.iter().rev() {
            self.pop(param)?;
        }
        let height = self.stack.len();
        self.stack.extend_from_slice(&ty.params);
        let label = self.new_label();
        if kind == FrameKind::Loop {
            self.bind(label);
        }
        let else_label = (kind == FrameKind::If).then(|| self.new_label());
        self.frames.push(Frame {
            kind,
            height,
            params: ty.params,
            results: ty.results,
            label,
            else_label,
            unreachable: false,
        });
        Ok(())
    }

    /// Whether the operand stack above `frame` holds exactly its results.
    fn holds_results(&self, frame: &Frame) -> bool {
        self.stack.get(frame.height..).is_some_and(|top| top == frame.results)
    }

    /// `else`: close the true arm and open the false one at the same height.
    fn else_arm(&mut self) -> Step<()> {
        let frame = self.frames.last().expect("the function frame outlives its body");
        if frame.kind != FrameKind::If {
            return malformed("else without if");
        }
        let live = !frame.unreachable;
        if live && !self.holds_results(frame) {
            return malformed("a block ends without its results");
        }
        let (end, height, params) = (frame.label, frame.height, frame.params.clone());
        if live {
            self.record(OP_BR, 0, self.height(), 0, 0, Fix::Label(end));
        }
        let frame = self.frame();
        frame.kind = FrameKind::Else;
        frame.unreachable = false;
        if let Some(label) = frame.else_label.take() {
            self.bind(label);
        }
        self.stack.truncate(height);
        self.stack.extend_from_slice(&params);
        Ok(())
    }

    /// `end`: bind the frame's labels and leave its results on the stack.
    /// Returns `true` when it closed the function frame.
    fn end(&mut self) -> Step<bool> {
        let frame = self.frames.pop().expect("the function frame outlives its body");
        if !frame.unreachable && !self.holds_results(&frame) {
            return malformed("a block ends without its results");
        }
        if let Some(label) = frame.else_label {
            self.bind(label);
        }
        if frame.kind != FrameKind::Loop {
            self.bind(frame.label);
        }
        self.stack.truncate(frame.height);
        self.stack.extend_from_slice(&frame.results);
        if frame.kind != FrameKind::Function {
            return Ok(false);
        }
        for (slot, &ty) in frame.results.iter().enumerate() {
            // cast: a result index is below the type's result count
            self.record(OP_RETVAL, type_code(ty)?, slot as u32, 0, slot as u32, Fix::None);
        }
        self.record(OP_RETURN, 0, 0, 0, 0, Fix::None);
        Ok(true)
    }

    /// The label a branch to `depth` lands on and the moves that put the
    /// values it carries where that label expects them.
    fn branch(&mut self, depth: u32) -> Step<(u32, Vec<(u32, u32, u8)>)> {
        // cast: u32 to usize widens
        let Some(index) = self.frames.len().checked_sub(1 + depth as usize) else {
            return malformed("branch depth out of range");
        };
        let frame = &self.frames[index];
        let carried = if frame.kind == FrameKind::Loop { &frame.params } else { &frame.results };
        let Some(source) = self.stack.len().checked_sub(carried.len()) else {
            return malformed("operand stack underflow");
        };
        if self.stack[source..] != carried[..] {
            return malformed("branch operand type mismatch");
        }
        let mut moves = Vec::new();
        for (slot, &ty) in carried.iter().enumerate() {
            let code = type_code(ty)?;
            if source != frame.height {
                // cast: slots are bounded by the body length
                moves.push(((source + slot) as u32, (frame.height + slot) as u32, code));
            }
        }
        Ok((frame.label, moves))
    }

    fn moves(&mut self, moves: &[(u32, u32, u8)]) {
        for &(from, to, ty) in moves {
            self.record(OP_MOVE, ty, from, to, 0, Fix::None);
        }
    }

    fn br(&mut self, depth: u32) -> Step<()> {
        let (label, moves) = self.branch(depth)?;
        self.moves(&moves);
        self.record(OP_BR, 0, self.height(), 0, 0, Fix::Label(label));
        self.set_unreachable();
        Ok(())
    }

    fn br_if(&mut self, depth: u32) -> Step<()> {
        let height = self.height();
        self.pop(ValType::I32)?;
        let (label, moves) = self.branch(depth)?;
        if moves.is_empty() {
            self.record(OP_BR_IF, 0, height, 0, 0, Fix::Label(label));
            return Ok(());
        }
        let skip = self.new_label();
        self.record(OP_BR_UNLESS, 0, height, 0, 0, Fix::Label(skip));
        self.moves(&moves);
        self.record(OP_BR, 0, height, 0, 0, Fix::Label(label));
        self.bind(skip);
        Ok(())
    }

    fn br_table(&mut self) -> Step<()> {
        let count = read(self.reader.count(1))?;
        let mut depths = Vec::with_capacity(count + 1);
        for _ in 0..=count {
            depths.push(read(self.reader.u32())?);
        }
        let height = self.height();
        self.pop(ValType::I32)?;
        // Per distinct depth: the landing label, and the moves a trampoline makes.
        let mut landings = BTreeMap::new();
        let mut carried = 0;
        for &depth in &depths {
            if landings.contains_key(&depth) {
                continue;
            }
            let (label, moves) = self.branch(depth)?;
            carried += moves.len();
            self.spend(carried)?;
            let landing = if moves.is_empty() { label } else { self.new_label() };
            landings.insert(depth, (landing, label, moves));
        }
        let default = landings[&depths[count]].0;
        if count == 0 {
            self.record(OP_BR, 0, height, 0, 0, Fix::Label(default));
        } else {
            // cast: `count` came from a u32
            self.record(OP_BR_TABLE_DEFAULT, 0, height, 0, count as u32, Fix::Label(default));
            let table = self.new_label();
            self.record(OP_BR_TABLE, 0, height, 0, 0, Fix::Label(table));
            self.bind(table);
            self.out.items.push(Item::Table(depths[..count].iter().map(|depth| landings[depth].0).collect()));
        }
        for (landing, label, moves) in landings.into_values() {
            if landing != label {
                self.bind(landing);
                self.moves(&moves);
                self.record(OP_BR, 0, height, 0, 0, Fix::Label(label));
            }
        }
        self.set_unreachable();
        Ok(())
    }

    fn call(&mut self, op: u16, ty: &FuncType, fix: Fix) -> Step<()> {
        check_arity(ty)?;
        let height = self.height();
        if op == OP_CALL_INDIRECT {
            self.pop(ValType::I32)?;
        }
        for &param in ty.params.iter().rev() {
            self.pop(param)?;
        }
        let base = self.height();
        for (index, &param) in ty.params.iter().enumerate() {
            // cast: an argument index is below the type's parameter count
            let index = index as u32;
            self.record(OP_ARG, type_code(param)?, base + index, 0, index, Fix::None);
        }
        self.record(op, 0, height, 0, 0, fix);
        for (index, &result) in ty.results.iter().enumerate() {
            // cast: a result index is below the type's result count
            let index = index as u32;
            self.record(OP_RESULT, type_code(result)?, base + index, 0, index, Fix::None);
            self.push(result);
        }
        Ok(())
    }

    fn callee(&mut self, indirect: bool) -> Step<(FuncType, Fix)> {
        let index = read(self.reader.u32())?;
        if !indirect {
            let ty = self.module.function_type_index(index).and_then(|ty| self.module.func_type(ty));
            return match ty {
                Some(ty) => Ok((ty.clone(), Fix::Function(index))),
                None => malformed("call to an unknown function"),
            };
        }
        let table = read(self.reader.u32())?;
        match self.module.func_type(index) {
            Some(ty) => Ok((ty.clone(), Fix::Table(table))),
            None => malformed("call_indirect with an unknown type"),
        }
    }

    fn memory_offset(&mut self) -> Step<u32> {
        let align = read(self.reader.u32())?;
        if align & 0x40 != 0 && read(self.reader.u32())? != 0 {
            return unsupported("a memory other than memory 0");
        }
        match u32::try_from(read(self.reader.u64())?) {
            Ok(offset) => Ok(offset),
            Err(_) => unsupported("a 64-bit memory offset"),
        }
    }

    /// Skip one instruction of unreachable code, tracking only its nesting.
    fn skip(&mut self, op: u8) -> Step<bool> {
        match op {
            0x02..=0x04 => {
                self.block_type()?;
                self.dead_depth += 1;
            }
            0x05 if self.dead_depth == 0 => self.else_arm()?,
            0x0b if self.dead_depth == 0 => return self.end(),
            0x0b => self.dead_depth -= 1,
            0x00 | 0x01 | 0x05 | 0x0f | 0x1a | 0x1b | 0x45..=0xc4 => {}
            0x0c | 0x0d | 0x10 | 0x12 | 0x20..=0x24 => drop(read(self.reader.u32())?),
            0x0e => {
                for _ in 0..=read(self.reader.count(1))? {
                    read(self.reader.u32())?;
                }
            }
            0x11 | 0x13 => {
                read(self.reader.u32())?;
                read(self.reader.u32())?;
            }
            0x1c => {
                for _ in 0..read(self.reader.count(1))? {
                    read(self.reader.byte())?;
                }
            }
            0x28..=0x3e => drop(self.memory_offset()?),
            0x3f | 0x40 => drop(read(self.reader.byte())?),
            0x41 => drop(read(self.reader.s32())?),
            0x42 => drop(read(self.reader.s64())?),
            0x43 => drop(read(self.reader.fixed_u32())?),
            0x44 => drop(read(self.reader.fixed_u64())?),
            0xfc => match read(self.reader.u32())? {
                0..=7 => {}
                10 => drop(read(self.reader.take(2))?),
                11 => drop(read(self.reader.take(1))?),
                sub => return unsupported(format!("instruction 0xfc {sub}")),
            },
            _ => return unsupported(format!("instruction 0x{op:02x}")),
        }
        Ok(false)
    }

    /// Lower one instruction. Returns `true` at the function's final `end`.
    fn step(&mut self) -> Step<bool> {
        let op = read(self.reader.byte())?;
        if self.frame().unreachable {
            return self.skip(op);
        }
        let height = self.height();
        let wide = u16::from(op);
        match op {
            0x00 => {
                self.record(wide, 0, height, 0, 0, Fix::None);
                self.set_unreachable();
            }
            0x01 => {}
            0x02 | 0x03 => {
                let ty = self.block_type()?;
                self.open(if op == 0x02 { FrameKind::Block } else { FrameKind::Loop }, ty)?;
            }
            0x04 => {
                let ty = self.block_type()?;
                self.pop(ValType::I32)?;
                self.open(FrameKind::If, ty)?;
                let label = self.frame().else_label.expect("an if frame opens with a false arm");
                self.record(OP_BR_UNLESS, 0, height, 0, 0, Fix::Label(label));
            }
            0x05 => self.else_arm()?,
            0x0b => return self.end(),
            0x0c => {
                let depth = read(self.reader.u32())?;
                self.br(depth)?;
            }
            0x0d => {
                let depth = read(self.reader.u32())?;
                self.br_if(depth)?;
            }
            0x0e => self.br_table()?,
            0x0f => {
                // cast: frames nest no deeper than the body is long
                self.br(self.frames.len() as u32 - 1)?;
            }
            0x10 | 0x11 => {
                let (ty, fix) = self.callee(op == 0x11)?;
                self.call(wide, &ty, fix)?;
            }
            0x12 | 0x13 => {
                let (ty, fix) = self.callee(op == 0x13)?;
                self.call(wide - 2, &ty, fix)?;
                // cast: frames nest no deeper than the body is long
                self.br(self.frames.len() as u32 - 1)?;
            }
            0x1a => drop(self.pop_any()?),
            0x1b | 0x1c => {
                if op == 0x1c {
                    for _ in 0..read(self.reader.count(1))? {
                        read(self.reader.byte())?;
                    }
                }
                self.pop(ValType::I32)?;
                let ty = self.pop_any()?;
                self.pop(ty)?;
                self.push(ty);
                self.record(0x1b, type_code(ty)?, height, 0, 0, Fix::None);
            }
            0x20..=0x24 => self.variable(op, height)?,
            0x28..=0x40 => self.memory(op, height)?,
            0x41..=0x44 => self.constant(op, height)?,
            0x45..=0xc4 => self.numeric(wide, height)?,
            0xfc => self.prefixed(height)?,
            _ => return unsupported(format!("instruction 0x{op:02x}")),
        }
        Ok(false)
    }

    /// `local.get`, `local.set`, `local.tee`, `global.get` and `global.set`.
    fn variable(&mut self, op: u8, height: u32) -> Step<()> {
        let index = read(self.reader.u32())?;
        if op >= 0x23 {
            // cast: u32 to usize widens
            let Some(&global) = self.globals.get(index as usize) else {
                return malformed("global index out of range");
            };
            if op == 0x23 {
                self.push(global.ty);
            } else {
                self.pop(global.ty)?;
            }
            let ty =
                if self.stack_pointer == Some(index) { TYPE_STACK_POINTER } else { type_code(global.ty)? };
            self.record(u16::from(op), ty, height, index, 0, Fix::None);
            return Ok(());
        }
        // cast: u32 to usize widens
        let Some(&ty) = self.locals.get(index as usize) else {
            return malformed("local index out of range");
        };
        match op {
            0x20 => {
                // cast: u32 to usize widens
                self.read[index as usize] = true;
                self.push(ty);
            }
            0x21 => self.pop(ty)?,
            _ => {
                self.pop(ty)?;
                self.push(ty);
            }
        }
        self.record(u16::from(op), type_code(ty)?, height, index, 0, Fix::None);
        Ok(())
    }

    /// The loads and stores, `memory.size` and `memory.grow`.
    fn memory(&mut self, op: u8, height: u32) -> Step<()> {
        let wide = u16::from(op);
        let lo = match op {
            0x28..=0x35 => {
                let offset = self.memory_offset()?;
                self.pop(ValType::I32)?;
                self.push(memory_value(wide));
                offset
            }
            0x36..=0x3e => {
                let offset = self.memory_offset()?;
                self.pop(memory_value(wide))?;
                self.pop(ValType::I32)?;
                offset
            }
            _ => {
                if read(self.reader.byte())? != 0 {
                    return unsupported("a memory other than memory 0");
                }
                if op == 0x40 {
                    self.pop(ValType::I32)?;
                }
                self.push(ValType::I32);
                0
            }
        };
        self.record(wide, 0, height, lo, 0, Fix::None);
        Ok(())
    }

    /// The four `const` instructions; the record holds the bit pattern.
    fn constant(&mut self, op: u8, height: u32) -> Step<()> {
        let (ty, bits) = match op {
            // cast: the constants' bit patterns
            0x41 => (ValType::I32, u64::from(read(self.reader.s32())? as u32)),
            0x42 => (ValType::I64, read(self.reader.s64())? as u64),
            0x43 => (ValType::F32, u64::from(read(self.reader.fixed_u32())?)),
            _ => (ValType::F64, read(self.reader.fixed_u64())?),
        };
        self.push(ty);
        // cast: the low and high words
        self.record(u16::from(op), 0, height, bits as u32, (bits >> 32) as u32, Fix::None);
        Ok(())
    }

    fn numeric(&mut self, op: u16, height: u32) -> Step<()> {
        let Some((operands, result)) = numeric_signature(op) else {
            return unsupported(format!("instruction 0x{op:02x}"));
        };
        for &operand in operands.iter().rev() {
            self.pop(operand)?;
        }
        self.push(result);
        // A reinterpret cast keeps the slot's bits and width: nothing to run.
        if !matches!(op, 0xbc..=0xbf) {
            self.record(op, 0, height, 0, 0, Fix::None);
        }
        Ok(())
    }

    fn prefixed(&mut self, height: u32) -> Step<()> {
        let sub = read(self.reader.u32())?;
        match sub {
            // cast: `sub` is below 8
            0..=7 => self.numeric(0xfc00 | sub as u16, height),
            10 | 11 => {
                read(self.reader.take(if sub == 10 { 2 } else { 1 }))?;
                for _ in 0..3 {
                    self.pop(ValType::I32)?;
                }
                // cast: `sub` is 10 or 11
                self.record(0xfc00 | sub as u16, 0, height, 0, 0, Fix::None);
                Ok(())
            }
            _ => unsupported(format!("instruction 0xfc {sub}")),
        }
    }

    fn body(&mut self) -> Step<()> {
        while !self.step()? {
            if self.stack.len() > MAX_SLOTS {
                return unsupported(format!("an operand stack deeper than {MAX_SLOTS}"));
            }
            self.spend(0)?;
        }
        if !self.reader.is_empty() {
            return malformed("code after the function's end");
        }
        Ok(())
    }
}

impl<'a> Lowering<'a> {
    /// Prepare to lower the bodies of `module`, read from `bytes`.
    pub fn new(module: &'a Module, bytes: &'a [u8]) -> Lowering<'a> {
        Lowering { module, bytes, globals: module.global_types(), stack_pointer: module.stack_pointer() }
    }

    /// Lower the body of defined function `index`.
    pub fn lower(&self, index: usize) -> Lowered {
        let module = self.module;
        let body = &module.bodies[index];
        let mut lowerer = Lowerer {
            module,
            reader: Reader::new(self.bytes, body.code.clone()),
            globals: &self.globals,
            stack_pointer: self.stack_pointer,
            locals: Vec::new(),
            params: 0,
            read: Vec::new(),
            budget: body.code.len().saturating_mul(RECORDS_PER_BYTE) + 64,
            stack: Vec::new(),
            frames: Vec::new(),
            dead_depth: 0,
            out: Lowered::default(),
        };
        let lowered = match module.functions.get(index).and_then(|&ty| module.func_type(ty)) {
            Some(signature) => lowerer
                .entry(signature, &body.locals)
                .and_then(|()| lowerer.body())
                .and_then(|()| lowerer.prologue()),
            None => unsupported("a type that is not a plain function type"),
        };
        let mut out = lowerer.out;
        let reason = match lowered {
            Ok(()) => return out,
            Err(Refusal::Malformed(reason)) => format!("malformed body: {reason}"),
            Err(Refusal::Unsupported(what)) => format!("uses {what}"),
        };
        out.labels.clear();
        out.items = vec![Item::Record { op: OP_UNSUPPORTED, ty: 0, height: 0, lo: 0, hi: 0, fix: Fix::None }];
        out.refused = Some(reason);
        out
    }
}

impl Lowerer<'_> {
    /// Declare the parameters and locals and open the function frame.
    fn entry(&mut self, signature: &FuncType, locals: &[(u32, ValType)]) -> Step<()> {
        check_arity(signature)?;
        self.locals.extend_from_slice(&signature.params);
        self.params = signature.params.len();
        for &(count, ty) in locals {
            // cast: u32 to usize widens
            if self.locals.len().saturating_add(count as usize) > MAX_LOCALS {
                return unsupported(format!("more than {MAX_LOCALS} locals"));
            }
            // cast: u32 to usize widens
            self.locals.extend(std::iter::repeat_n(ty, count as usize));
        }
        self.read = vec![false; self.locals.len()];
        let label = self.new_label();
        self.frames.push(Frame {
            kind: FrameKind::Function,
            height: 0,
            params: Vec::new(),
            results: signature.results.clone(),
            label,
            else_label: None,
            unreachable: false,
        });
        Ok(())
    }

    /// Put the records that run before the body in front of it: each
    /// parameter the body reads is copied from its argument register to its
    /// local, and every other local it reads is zeroed. A local that is only
    /// written needs neither.
    fn prologue(&mut self) -> Step<()> {
        let mut records = Vec::new();
        for (index, &ty) in self.locals.iter().enumerate().filter(|&(index, _)| self.read[index]) {
            // cast: a local index is below MAX_LOCALS
            let index = index as u32;
            let (op, hi) = if (index as usize) < self.params { (OP_PARAM, index) } else { (OP_ZERO, 0) };
            records.push(Item::Record { op, ty: type_code(ty)?, height: 0, lo: index, hi, fix: Fix::None });
        }
        // cast: bounded by MAX_LOCALS
        let shift = records.len() as u32;
        self.out.labels.iter_mut().for_each(|label| *label += shift);
        self.out.items.splice(0..0, records);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::format::wasm::synthetic::{block, i32_const, indexed, ModuleBuilder, F64, I32, I64};

    const BASE: u32 = 0x8000_0000;
    const VOID: u8 = 0x40;

    /// Lower function `index` of the built module and decode its records as
    /// `(opcode, type, height, lo, hi)`, calls and tables resolved to a marker.
    fn records(builder: &ModuleBuilder, index: usize) -> (Lowered, Vec<(u16, u8, u32, u32, u32)>) {
        let bytes = builder.build();
        let module = Module::parse(&bytes).unwrap();
        let lowered = Lowering::new(&module, &bytes).lower(index);
        let mut code = Vec::new();
        lowered.encode(
            BASE,
            |fix| match fix {
                Fix::Function(index) => 0xf000_0000 + index,
                Fix::Table(index) => 0xe000_0000 + index,
                Fix::None | Fix::Label(_) => 0,
            },
            &mut code,
        );
        assert_eq!(code.len(), lowered.len() as usize);
        let word = |at: usize| u32::from_le_bytes(code[at..at + 4].try_into().unwrap());
        let records = (0..code.len())
            .step_by(16)
            .map(|at| ((word(at) & 0xffff) as u16, (word(at) >> 16) as u8, word(at + 4), word(at + 8), word(at + 12)))
            .collect();
        (lowered, records)
    }

    #[test]
    fn straight_line_code_reads_its_operands_by_stack_height() {
        let mut builder = ModuleBuilder::default();
        let code = [indexed(0x20, 0), indexed(0x20, 1), vec![0x6c], indexed(0x20, 2), vec![0x6a]].concat();
        builder.function("mul_add", &[I32, I32, I32], &[I32], &[], &code);
        let (lowered, records) = records(&builder, 0);
        assert_eq!(lowered.refused, None);
        assert_eq!(
            records,
            vec![
                (OP_PARAM, 0, 0, 0, 0),
                (OP_PARAM, 0, 0, 1, 1),
                (OP_PARAM, 0, 0, 2, 2),
                (0x20, 0, 0, 0, 0),
                (0x20, 0, 1, 1, 0),
                (0x6c, 0, 2, 0, 0),
                (0x20, 0, 1, 2, 0),
                (0x6a, 0, 2, 0, 0),
                (OP_RETVAL, 0, 0, 0, 0),
                (OP_RETURN, 0, 0, 0, 0),
            ]
        );
    }

    #[test]
    fn only_the_locals_a_body_reads_are_copied_in_or_zeroed() {
        let mut builder = ModuleBuilder::default();
        let code = [indexed(0x20, 3), vec![0x1a], indexed(0x20, 0), indexed(0x21, 1), i32_const(1), indexed(0x21, 2)].concat();
        builder.function("f", &[I64, I64], &[], &[(1, I32), (1, F64)], &code);
        let (_, records) = records(&builder, 0);
        assert_eq!(records[..2], [(OP_PARAM, 1, 0, 0, 0), (OP_ZERO, 3, 0, 3, 0)]);
        assert_eq!(records.len(), 8);
    }

    #[test]
    fn a_branch_table_that_would_lower_to_a_flood_of_moves_is_refused() {
        // Thirty nested blocks of fifty results, each opened one slot higher,
        // and a br_table from the innermost naming every one of them.
        let wide: Vec<u8> = vec![I32; 50];
        let mut code = Vec::new();
        for _ in 0..30 {
            code.extend([i32_const(0), vec![0x02, 0]].concat());
        }
        (0..50).for_each(|_| code.extend(i32_const(1)));
        code.extend(i32_const(0));
        code.extend([vec![0x0e, 30], (0..30).collect::<Vec<u8>>(), vec![0]].concat());
        (0..30).for_each(|_| code.extend([0x0b, 0x00]));
        let mut builder = ModuleBuilder::default();
        builder.function("wide", &[], &wide, &[], &[]).function("f", &[], &[], &[], &code);
        let (lowered, _) = records(&builder, 1);
        assert_eq!(lowered.refused.as_deref(), Some("uses branches that lower to more than 4 records per byte"));
    }

    #[test]
    fn a_branch_moves_its_value_down_to_the_height_its_target_expects() {
        // i32.const 100; block (result i32) i32.const 1; i32.const 2; local.get 0; br_if 0; i32.add end; i32.add
        let inner = [i32_const(1), i32_const(2), indexed(0x20, 0), indexed(0x0d, 0), vec![0x6a]].concat();
        let code = [i32_const(100), block(0x02, I32, &inner), vec![0x6a]].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &code);
        let (_, records) = records(&builder, 0);
        let skip = BASE + 16 * 8;
        let end = BASE + 16 * 9;
        assert_eq!(
            records[4..],
            [
                (0x20, 0, 3, 0, 0),
                (OP_BR_UNLESS, 0, 4, skip, 0),
                (OP_MOVE, 0, 2, 1, 0),
                (OP_BR, 0, 4, end, 0),
                (0x6a, 0, 3, 0, 0),
                (0x6a, 0, 2, 0, 0),
                (OP_RETVAL, 0, 0, 0, 0),
                (OP_RETURN, 0, 0, 0, 0),
            ]
        );
    }

    #[test]
    fn an_if_branches_past_its_true_arm_and_the_true_arm_past_the_false_one() {
        let then = [i32_const(2), vec![0x05], i32_const(3)].concat();
        let code = [indexed(0x20, 0), block(0x04, I32, &then)].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &code);
        let (_, records) = records(&builder, 0);
        assert_eq!(
            records[2..6],
            [
                (OP_BR_UNLESS, 0, 1, BASE + 16 * 5, 0),
                (0x41, 0, 0, 2, 0),
                (OP_BR, 0, 1, BASE + 16 * 6, 0),
                (0x41, 0, 0, 3, 0),
            ]
        );
    }

    #[test]
    fn br_table_is_a_bounds_test_and_a_jump_through_a_table_of_addresses() {
        // block block local.get 0; br_table 0 1 0, default 1; end; i32.const 5; return; end; i32.const 6
        let table = [vec![0x0e, 3, 0, 1, 0, 1]].concat();
        let inner = block(0x02, VOID, &[indexed(0x20, 0), table].concat());
        let code = [block(0x02, VOID, &[inner, i32_const(5), vec![0x0f]].concat()), i32_const(6)].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &code);
        let bytes = builder.build();
        let module = Module::parse(&bytes).unwrap();
        let lowered = Lowering::new(&module, &bytes).lower(0);
        let mut image = Vec::new();
        lowered.encode(BASE, |_| 0, &mut image);
        let word = |at: usize| u32::from_le_bytes(image[at..at + 4].try_into().unwrap());
        // param, local.get, default test, br_table, [3 entries + pad], const 5, br, const 6, retval, return
        let (inner_end, outer_end) = (BASE + 16 * 5, BASE + 16 * 7);
        assert_eq!((word(32) & 0xffff, word(40), word(44)), (u32::from(OP_BR_TABLE_DEFAULT), outer_end, 3));
        assert_eq!((word(48) & 0xffff, word(56)), (u32::from(OP_BR_TABLE), BASE + 64));
        assert_eq!([word(64), word(68), word(72), word(76)], [inner_end, outer_end, inner_end, 0]);
        assert_eq!(word(80) & 0xffff, 0x41);
    }

    #[test]
    fn a_call_copies_its_arguments_out_and_its_result_back() {
        let code = [i32_const(9), indexed(0x20, 0), indexed(0x20, 1), indexed(0x10, 0), vec![0x1a]].concat();
        let mut builder = ModuleBuilder::default();
        builder.import("ext", &[I64, I32], &[I64]).function("f", &[I64, I32], &[I32], &[], &code);
        let (lowered, records) = records(&builder, 0);
        assert_eq!(lowered.refused, None);
        assert_eq!(
            records[5..9],
            [(OP_ARG, 1, 1, 0, 0), (OP_ARG, 0, 2, 0, 1), (0x10, 0, 3, 0xf000_0000, 0), (OP_RESULT, 1, 1, 0, 0)]
        );
    }

    #[test]
    fn an_indirect_call_reads_its_table_and_a_tail_call_returns() {
        let code = [indexed(0x20, 0), indexed(0x20, 0), vec![0x13, 0, 0]].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &code).table(&[0], false);
        let (lowered, records) = records(&builder, 0);
        assert_eq!(lowered.refused, None);
        assert_eq!(
            records[3..],
            [
                (OP_ARG, 0, 0, 0, 0),
                (OP_CALL_INDIRECT, 0, 2, 0xe000_0000, 0),
                (OP_RESULT, 0, 0, 0, 0),
                (OP_BR, 0, 1, BASE + 16 * 7, 0),
                (OP_RETVAL, 0, 0, 0, 0),
                (OP_RETURN, 0, 0, 0, 0),
            ]
        );
    }

    #[test]
    fn unreachable_code_is_dropped_and_its_blocks_skipped_whole() {
        // br 0 (to the function); then dead: i32.const 1; block i32.const 2; end; drop; then the end
        let dead = [i32_const(1), block(0x02, I32, &i32_const(2)), vec![0x1a, 0x1a]].concat();
        let code = [i32_const(4), indexed(0x0c, 0), dead].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[], &[I32], &[], &code);
        let (lowered, records) = records(&builder, 0);
        assert_eq!(lowered.refused, None);
        assert_eq!(
            records,
            vec![(0x41, 0, 0, 4, 0), (OP_BR, 0, 1, BASE + 32, 0), (OP_RETVAL, 0, 0, 0, 0), (OP_RETURN, 0, 0, 0, 0)]
        );
    }

    #[test]
    fn the_stack_pointer_global_is_marked() {
        let code = [indexed(0x23, 0), indexed(0x24, 0), indexed(0x23, 1), vec![0x1a]].concat();
        let mut builder = ModuleBuilder::default();
        builder
            .function("f", &[], &[], &[], &code)
            .global("__stack_pointer", I32, true, 0x1_0000)
            .global("other", I32, true, 0);
        let (_, records) = records(&builder, 0);
        assert_eq!(
            records[..3],
            [(0x23, TYPE_STACK_POINTER, 0, 0, 0), (0x24, TYPE_STACK_POINTER, 1, 0, 0), (0x23, 0, 0, 1, 0)]
        );
    }

    #[test]
    fn a_body_outside_the_supported_set_is_one_unsupported_record_and_a_reason() {
        let cases: [(&[u8], &[u8], &[u8], &str); 6] = [
            (&[], &[], &[0xfd, 0x0c], "uses instruction 0xfd"),
            (&[], &[I32, I32], &[0x41, 1, 0x41, 2], "uses a function type with more than one result"),
            (&[], &[I32], &[], "malformed body: a block ends without its results"),
            (&[], &[], &[0x6a], "malformed body: operand stack underflow"),
            (&[I32], &[], &[0x20, 0, 0x50, 0x1a], "malformed body: operand type mismatch"),
            (&[], &[], &[0x0c, 4], "malformed body: branch depth out of range"),
        ];
        for (params, results, code, reason) in cases {
            let mut builder = ModuleBuilder::default();
            builder.function("f", params, results, &[], code);
            let (lowered, records) = records(&builder, 0);
            assert_eq!(lowered.refused.as_deref(), Some(reason));
            assert_eq!(records, vec![(OP_UNSUPPORTED, 0, 0, 0, 0)]);
        }
    }

    #[test]
    fn a_body_cut_short_anywhere_is_refused_without_a_panic() {
        let then = [i32_const(2), vec![0x05], i32_const(3)].concat();
        let code = [indexed(0x20, 0), block(0x04, I32, &then), vec![0x0e, 1, 0, 0]].concat();
        let mut builder = ModuleBuilder::default();
        builder.function("f", &[I32], &[I32], &[], &code);
        let bytes = builder.build();
        let mut module = Module::parse(&bytes).unwrap();
        let full = module.bodies[0].code.clone();
        for end in full.start..full.end {
            module.bodies[0].code = full.start..end;
            assert!(Lowering::new(&module, &bytes).lower(0).refused.is_some(), "cut at {end}");
        }
    }
}
