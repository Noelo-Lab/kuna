//! (kuna `loweredswitch`, unoptimized functions) Read how each test of a
//! compare chain was compiled, before simplification erases it.

use std::collections::BTreeMap;
use std::io::Write;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Longest chain of ops followed back from a branch condition.
const SLICE_DEPTH: u32 = 16;

/// What a compared value is, at the first heritage pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Loaded in the test's own block, by the load at this address, from this
    /// location.
    Own(u64, String),
    /// Carried in a register from an op in another block (the load or the
    /// computation at this address).
    Carried(u64, String),
    /// A function input.
    Input(u64),
    /// No single compared value.
    Other,
}

/// One conditional branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Test {
    /// `Some(true)` for an equality test that jumps on a match, `Some(false)`
    /// for one that falls through on a match, `None` for any other test.
    pub match_taken: Option<bool>,
    /// The compared constant of an equality test.
    pub value: Option<u64>,
    pub source: Source,
}

#[derive(Debug, Clone, Default)]
pub struct Record {
    pub tests: BTreeMap<u64, Test>,
}

struct Walk<'a> {
    fd: &'a Funcdata,
    block: BlockId,
    leaves: Vec<VarnodeId>,
}

impl Walk<'_> {
    fn go(&mut self, vn: VarnodeId, depth: u32) {
        let Some(v) = self.fd.vbank().get(vn) else { return };
        if v.is_constant() || self.leaves.len() > 4 {
            return;
        }
        let def = v.get_def().and_then(|d| self.fd.obank().get(d));
        let Some(op) = def else {
            self.leaves.push(vn);
            return;
        };
        if depth == 0 || op.get_parent() != Some(self.block) || op.code() == OpCode::CPUI_LOAD {
            if !self.leaves.contains(&vn) {
                self.leaves.push(vn);
            }
            return;
        }
        if matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            if !self.leaves.contains(&vn) {
                self.leaves.push(vn);
            }
            return;
        }
        for i in 0..op.num_input() {
            if let Some(x) = op.get_in(i) {
                self.go(x, depth - 1);
            }
        }
    }
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = fd.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

fn def_op(fd: &Funcdata, vn: VarnodeId) -> Option<&crate::op::PcodeOp> {
    fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d))
}

/// The value an equality test compares and its constant: `x == c`, or the
/// flag form `x - c == 0`.
fn equality(fd: &Funcdata, cond: VarnodeId) -> Option<(bool, u64)> {
    let mut cond = cond;
    let mut negated = false;
    for _ in 0..4 {
        let op = def_op(fd, cond)?;
        match op.code() {
            OpCode::CPUI_BOOL_NEGATE => {
                negated = !negated;
                cond = op.get_in(0)?;
            }
            OpCode::CPUI_COPY => cond = op.get_in(0)?,
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                let eq = (op.code() == OpCode::CPUI_INT_EQUAL) != negated;
                let (a, b) = (op.get_in(0)?, op.get_in(1)?);
                let (x, c) = match (constant(fd, a), constant(fd, b)) {
                    (None, Some(c)) => (a, c),
                    (Some(c), None) => (b, c),
                    _ => return None,
                };
                if c == 0 {
                    if let Some(sub) = def_op(fd, x) {
                        if sub.code() == OpCode::CPUI_INT_SUB {
                            if let Some(k) = sub.get_in(1).and_then(|k| constant(fd, k)) {
                                return Some((eq, k));
                            }
                        }
                    }
                }
                return Some((eq, c));
            }
            _ => return None,
        }
    }
    None
}

fn is_move(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT)
}

fn key(fd: &Funcdata, frame: &mut crate::p3_dataflow::kuna_unoptimized::Frame, vn: VarnodeId, depth: u32) -> String {
    if depth == 0 {
        return "?".into();
    }
    if let Some(c) = constant(fd, vn) {
        return format!("G{c:x}");
    }
    if let Some((off, _)) = frame.offset(vn) {
        return format!("F{off}");
    }
    let Some(op) = def_op(fd, vn) else {
        return fd.vbank().get(vn).map_or("?".into(), |v| format!("R{:x}", v.get_offset()));
    };
    match op.code() {
        OpCode::CPUI_LOAD => format!("L({})", op.get_in(1).map_or("?".into(), |a| key(fd, frame, a, depth - 1))),
        c if is_move(c) => op.get_in(0).map_or("?".into(), |a| key(fd, frame, a, depth - 1)),
        OpCode::CPUI_INT_ADD => {
            let a = op.get_in(0).map_or("?".into(), |a| key(fd, frame, a, depth - 1));
            let b = op.get_in(1).map_or("?".into(), |b| key(fd, frame, b, depth - 1));
            format!("{a}+{b}")
        }
        c => format!("{:?}", c),
    }
}

fn source(fd: &Funcdata, frame: &mut crate::p3_dataflow::kuna_unoptimized::Frame, cbranch: OpId, block: BlockId) -> Source {
    let Some(cond) = fd.obank().get(cbranch).and_then(|o| o.get_in(1)) else { return Source::Other };
    let mut w = Walk { fd, block, leaves: Vec::new() };
    w.go(cond, SLICE_DEPTH);
    let leaves: Vec<VarnodeId> = w.leaves;
    if leaves.len() != 1 {
        return Source::Other;
    }
    let mut vn = leaves[0];
    for _ in 0..8 {
        let Some(v) = fd.vbank().get(vn) else { return Source::Other };
        if v.is_input() {
            return Source::Input(v.get_offset());
        }
        let Some(op) = def_op(fd, vn) else { return Source::Other };
        let at = op.get_addr().get_offset();
        if op.code() == OpCode::CPUI_LOAD {
            let k = op.get_in(1).map_or("?".into(), |a| key(fd, frame, a, 6));
            return if op.get_parent() == Some(block) { Source::Own(at, k) } else { Source::Carried(at, k) };
        }
        if is_move(op.code()) {
            if let Some(x) = op.get_in(0) {
                vn = x;
                continue;
            }
        }
        return Source::Carried(at, format!("{:?}", op.code()));
    }
    Source::Other
}

/// Read every conditional branch of an unoptimized function; once, at the
/// first heritage pass.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 {
        return;
    }
    let dump = std::env::var("KUNA_O0SW_DUMP").ok();
    if dump.is_none() {
        return;
    }
    let Some(mut frame) = crate::p3_dataflow::kuna_unoptimized::Frame::for_function(fd) else { return };
    let mut rec = Record::default();
    let n = fd.bblocks_get_size();
    let mut lines: Vec<String> = Vec::new();
    for i in 0..n {
        let b = fd.bblocks_get_block(i);
        let Some(tail) = fd.bb_op_tail(b) else { continue };
        let Some(op) = fd.obank().get(tail) else { continue };
        if op.code() != OpCode::CPUI_CBRANCH {
            continue;
        }
        let at = op.get_addr().get_offset();
        let dest = op.get_in(0).and_then(|d| fd.vbank().get(d)).map_or(0, |d| d.get_offset());
        let eq = op.get_in(1).and_then(|c| equality(fd, c));
        let src = source(fd, &mut frame, tail, b);
        let blk = fd.bblocks_ref().block(b);
        let fall = if blk.size_out() == 2 {
            fd.bb_op_head(blk.get_out(0)).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset())
        } else {
            0
        };
        let start = fd.bb_op_head(b).and_then(|o| fd.obank().get(o)).map_or(0, |o| o.get_addr().get_offset());
        let (k, v) = match src {
            Source::Own(a, ref k) => ("own", format!("{a:x}:{k}")),
            Source::Carried(a, ref k) => ("car", format!("{a:x}:{k}")),
            Source::Input(r) => ("inp", format!("{r:x}")),
            Source::Other => ("oth", String::new()),
        };
        lines.push(format!(
            "[\"{at:x}\",\"{start:x}\",{},{},\"{k}\",\"{v}\",\"{dest:x}\",\"{fall:x}\"]",
            eq.map_or("null".into(), |(t, _)| if t { "1".to_string() } else { "0".to_string() }),
            eq.map_or("null".into(), |(_, c)| format!("{c}")),
        ));
        rec.tests.insert(at, Test { match_taken: eq.map(|(t, _)| t), value: eq.map(|(_, c)| c), source: src });
    }
    if let Some(path) = dump {
        let unopt = crate::p3_dataflow::kuna_unoptimized::is_unoptimized(fd);
        let line = format!(
            "{{\"fn\":\"{:x}\",\"unopt\":{},\"tests\":[{}]}}\n",
            fd.get_address().get_offset(),
            unopt,
            lines.join(",")
        );
        let file = format!("{}/{}.jsonl", path, std::process::id());
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(file) {
            let _ = f.write_all(line.as_bytes());
        }
    }
    let _ = rec;
    let _: int4 = 0;
}
