//! The features of a function-tail site, the saved-weights score over them, and
//! the training dump (`KUNA_TAILFORM_DUMP=<dir>`, one JSON line per decided site).
//!
//! Every feature is read off the structured tree and is the same in either form
//! of the site: the conditional part `S` (the guarded tail or the wrapped body,
//! without the returns the rewrite adds, drops or elides), the condition in the
//! polarity that enters `S`, and the statements before the site.

use std::io::Write;
use std::sync::OnceLock;

use kuna_num::opcodes::OpCode;

use super::{bare_return, cond_of, count_lines, plain_if, RetKey, TailShape};
use crate::block::{BlockKind, BlockType};
use crate::context::{BlockId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p8_structure::kuna_tailform_weights as weights;

/// What the conditional part ends with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastKind {
    Call,
    Assign,
    Ret,
    If,
    Loop,
    Switch,
    Other,
}

/// The condition, read in the polarity that enters the conditional part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondKind {
    /// `x`, `x != 0`
    Truthy,
    /// `!x`, `x == 0`
    Eqz,
    Eq,
    Ne,
    Rel,
    And,
    Or,
    Bit,
    NotBit,
    Call,
    NotCall,
    Other,
    NotOther,
}

/// The value the site returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KKind {
    Void,
    Const,
    Var,
}

/// The features of one site (see the module doc).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TailFeatures {
    pub void: bool,
    /// The binary laid the site out wrapped.
    pub wrapped: bool,
    pub lines: u32,
    /// Top-level statements of `S`.
    pub top: u32,
    /// Statements of `S` at every depth.
    pub stmts: u32,
    pub calls: u32,
    pub assigns: u32,
    pub rets: u32,
    pub ifs: u32,
    pub loops: u32,
    pub switches: u32,
    pub gotos: u32,
    pub last: LastKind,
    /// `S` ends in a call whose block returns (a tail call).
    pub tail_call: bool,
    /// `S` holds every call of the function, and at least one.
    pub only_calls: bool,
    pub cond: CondKind,
    /// Operations in the condition expression.
    pub cond_ops: u32,
    /// The condition reads a call's result.
    pub cond_call: bool,
    /// The condition tests a parameter against zero.
    pub null_param: bool,
    pub k: KKind,
    /// Top-level statements before the site.
    pub p_top: u32,
    pub p_stmts: u32,
    pub p_rets: u32,
    pub p_loops: u32,
    /// Guard clauses (`if (c) return K;`) before the site.
    pub p_guards: u32,
    /// The function looks unoptimized (`kuna_unoptimized`).
    pub unopt: bool,
}

impl LastKind {
    pub fn as_str(self) -> &'static str {
        match self {
            LastKind::Call => "call",
            LastKind::Assign => "assign",
            LastKind::Ret => "ret",
            LastKind::If => "if",
            LastKind::Loop => "loop",
            LastKind::Switch => "switch",
            LastKind::Other => "other",
        }
    }
}

impl CondKind {
    pub fn as_str(self) -> &'static str {
        match self {
            CondKind::Truthy => "truthy",
            CondKind::Eqz => "eqz",
            CondKind::Eq => "eq",
            CondKind::Ne => "ne",
            CondKind::Rel => "rel",
            CondKind::And => "and",
            CondKind::Or => "or",
            CondKind::Bit => "bit",
            CondKind::NotBit => "not_bit",
            CondKind::Call => "call",
            CondKind::NotCall => "not_call",
            CondKind::Other => "other",
            CondKind::NotOther => "not_other",
        }
    }

    fn negate(self) -> CondKind {
        match self {
            CondKind::Truthy => CondKind::Eqz,
            CondKind::Eqz => CondKind::Truthy,
            CondKind::Eq => CondKind::Ne,
            CondKind::Ne => CondKind::Eq,
            CondKind::Rel => CondKind::Rel,
            CondKind::And => CondKind::Or,
            CondKind::Or => CondKind::And,
            CondKind::Bit => CondKind::NotBit,
            CondKind::NotBit => CondKind::Bit,
            CondKind::Call => CondKind::NotCall,
            CondKind::NotCall => CondKind::Call,
            CondKind::Other => CondKind::NotOther,
            CondKind::NotOther => CondKind::Other,
        }
    }
}

impl KKind {
    pub fn as_str(self) -> &'static str {
        match self {
            KKind::Void => "void",
            KKind::Const => "const",
            KKind::Var => "var",
        }
    }
}

/// The numeric features by name, in dump order.
pub const NUMERIC: &[&str] = &[
    "void", "wrapped", "lines", "top", "stmts", "calls", "assigns", "rets", "ifs", "loops", "switches", "gotos",
    "tail_call", "only_calls", "cond_ops", "cond_call", "null_param", "p_top", "p_stmts", "p_rets", "p_loops",
    "p_guards", "unopt", "unopt_wrapped",
];

impl TailFeatures {
    fn raw(&self, name: &str) -> Option<f64> {
        let b = |x: bool| f64::from(u8::from(x));
        Some(match name {
            "void" => b(self.void),
            "wrapped" => b(self.wrapped),
            "lines" => self.lines.into(),
            "top" => self.top.into(),
            "stmts" => self.stmts.into(),
            "calls" => self.calls.into(),
            "assigns" => self.assigns.into(),
            "rets" => self.rets.into(),
            "ifs" => self.ifs.into(),
            "loops" => self.loops.into(),
            "switches" => self.switches.into(),
            "gotos" => self.gotos.into(),
            "tail_call" => b(self.tail_call),
            "only_calls" => b(self.only_calls),
            "cond_ops" => self.cond_ops.into(),
            "cond_call" => b(self.cond_call),
            "null_param" => b(self.null_param),
            "p_top" => self.p_top.into(),
            "p_stmts" => self.p_stmts.into(),
            "p_rets" => self.p_rets.into(),
            "p_loops" => self.p_loops.into(),
            "p_guards" => self.p_guards.into(),
            "unopt" => b(self.unopt),
            "unopt_wrapped" => b(self.unopt && self.wrapped),
            _ => return None,
        })
    }

    /// The model input named `name`: `sign(x)*ln(1+|x|)` of a numeric feature,
    /// or 1/0 for a one-hot `kind=value` name.
    pub fn value(&self, name: &str) -> f64 {
        if let Some((kind, val)) = name.split_once('=') {
            let got = match kind {
                "last" => self.last.as_str(),
                "cond" => self.cond.as_str(),
                "k" => self.k.as_str(),
                _ => return 0.0,
            };
            return f64::from(u8::from(got == val));
        }
        self.raw(name).map_or(0.0, |x| x.signum() * x.abs().ln_1p())
    }

    fn json(&self) -> String {
        let mut out = String::from("{");
        for name in NUMERIC {
            out.push_str(&format!("\"{name}\": {}, ", self.raw(name).unwrap_or(0.0)));
        }
        out.push_str(&format!(
            "\"last\": \"{}\", \"cond\": \"{}\", \"k\": \"{}\"}}",
            self.last.as_str(),
            self.cond.as_str(),
            self.k.as_str()
        ));
        out
    }
}

/// The saved model's score for `f`: positive means wrap.
pub fn model_score(f: &TailFeatures) -> f64 {
    weights::NAMES.iter().zip(weights::WEIGHTS.iter()).fold(weights::BIAS, |acc, (n, w)| acc + w * f.value(n))
}

#[derive(Default, Clone, Copy)]
struct Tally {
    ops: u32,
    calls: u32,
    assigns: u32,
    rets: u32,
    ifs: u32,
    loops: u32,
    switches: u32,
    gotos: u32,
}

impl Tally {
    fn stmts(&self) -> u32 {
        self.ops + self.ifs + self.loops + self.switches + self.gotos
    }
}

fn tally(data: &Funcdata, b: BlockId, conds: &[(BlockId, BlockId)], t: &mut Tally) {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    match blk.get_type() {
        BlockType::Copy | BlockType::Basic => {
            let Some(bb) = blk.get_copy() else { return };
            let skip = super::skipped_return(data, b);
            let only = super::only_return(data, b);
            let mut cur = data.bb_op_head(bb);
            while let Some(o) = cur {
                cur = data.bb_op_next(o);
                let Some(op) = data.obank().get(o) else { continue };
                if only.is_some_and(|r| r != o) || Some(o) == skip {
                    continue;
                }
                if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                    t.calls += 1;
                }
                if !super::op_prints(data, o) {
                    continue;
                }
                t.ops += 1;
                if op.code() == OpCode::CPUI_RETURN {
                    t.rets += 1;
                } else if op.code() == OpCode::CPUI_STORE || op.get_out().is_some() {
                    t.assigns += 1;
                }
            }
        }
        BlockType::If => {
            t.ifs += 1;
            if blk.get_if_goto_target().is_some() {
                t.gotos += 1;
            }
            tally(data, cond_of(data, b, conds), conds, t);
            for i in 1..blk.get_size() {
                tally(data, blk.get_block(i), conds, t);
            }
        }
        ty => {
            match ty {
                BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => t.loops += 1,
                BlockType::Switch => t.switches += 1,
                BlockType::Goto => t.gotos += 1,
                BlockType::MultiGoto => {
                    if let BlockKind::MultiGoto { gotoedges, .. } = blk.kind() {
                        t.gotos += gotoedges.len() as u32;
                    }
                }
                _ => {}
            }
            for &c in blk.get_list() {
                tally(data, c, conds, t);
            }
        }
    }
}

/// The statements `b` prints at its own level: a leaf's statements, an `if`'s
/// condition statements and the `if` itself, or one construct.
fn top_of(data: &Funcdata, b: BlockId, conds: &[(BlockId, BlockId)]) -> u32 {
    let g = data.sblocks_ref();
    match g.block(b).get_type() {
        BlockType::Copy | BlockType::Basic => count_lines(data, b, conds) as u32,
        BlockType::If => count_lines(data, cond_of(data, b, conds), conds) as u32 + 1,
        BlockType::Ls | BlockType::Graph => g.block(b).get_list().iter().map(|&c| top_of(data, c, conds)).sum(),
        _ => 1,
    }
}

/// The last statement of `b`, through lists, not counting a final return the
/// rewrite hides (`skip_ret`).
fn last_kind(data: &Funcdata, b: BlockId, skip_ret: bool) -> (LastKind, Option<BlockId>) {
    let g = data.sblocks_ref();
    let blk = g.block(b);
    match blk.get_type() {
        BlockType::Ls | BlockType::Graph => {
            blk.get_list().last().map_or((LastKind::Other, None), |&l| last_kind(data, l, skip_ret))
        }
        BlockType::If => (LastKind::If, None),
        BlockType::WhileDo | BlockType::DoWhile | BlockType::InfLoop => (LastKind::Loop, None),
        BlockType::Switch => (LastKind::Switch, None),
        BlockType::Copy | BlockType::Basic => {
            let Some(bb) = blk.get_copy() else { return (LastKind::Other, None) };
            let skip = super::skipped_return(data, b);
            let mut kind = LastKind::Other;
            let mut cur = data.bb_op_head(bb);
            while let Some(o) = cur {
                cur = data.bb_op_next(o);
                let Some(op) = data.obank().get(o) else { continue };
                if Some(o) == skip || !super::op_prints(data, o) || (skip_ret && op.code() == OpCode::CPUI_RETURN) {
                    continue;
                }
                kind = match op.code() {
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => LastKind::Call,
                    OpCode::CPUI_RETURN => LastKind::Ret,
                    c if c == OpCode::CPUI_STORE || op.get_out().is_some() => LastKind::Assign,
                    _ => LastKind::Other,
                };
            }
            (kind, Some(bb))
        }
        _ => (LastKind::Other, None),
    }
}

/// The calls of the whole function.
fn function_calls(data: &Funcdata) -> u32 {
    let g = data.sblocks_ref();
    let Some(root) = g.root else { return 0 };
    let mut t = Tally::default();
    tally(data, root, &[], &mut t);
    t.calls
}

/// The kind, operation count, call-read and zero-tested-parameter facts of the
/// condition `c`, read in the polarity that enters the part (`enter_neg`: the
/// part runs when the printed condition is false).
fn cond_facts(data: &Funcdata, c: BlockId, enter_neg: bool) -> (CondKind, u32, bool, bool) {
    let g = data.sblocks_ref();
    let blk = g.block(c);
    match blk.get_type() {
        BlockType::Condition => {
            let (_, o0, c0, _) = cond_facts(data, blk.get_block(0), false);
            let (_, o1, c1, _) = cond_facts(data, blk.get_block(1), false);
            let and = blk.get_condition_opcode() == Some(OpCode::CPUI_BOOL_AND);
            let kind = if and != enter_neg { CondKind::And } else { CondKind::Or };
            (kind, o0 + o1 + 1, c0 || c1, false)
        }
        BlockType::Ls | BlockType::Graph => {
            blk.get_list().last().map_or((CondKind::Other, 0, false, false), |&l| cond_facts(data, l, enter_neg))
        }
        BlockType::Copy => {
            let Some(bb) = blk.get_copy() else { return (CondKind::Other, 0, false, false) };
            let Some(op) = data.bb_op_tail(bb).and_then(|t| data.obank().get(t)) else {
                return (CondKind::Other, 0, false, false);
            };
            if op.code() != OpCode::CPUI_CBRANCH {
                return (CondKind::Other, 0, false, false);
            }
            let Some(vn) = op.get_in(1) else { return (CondKind::Other, 0, false, false) };
            let neg = op.is_boolean_flip() != enter_neg;
            let mut ops = 0;
            let mut call = false;
            expr_scan(data, vn, &mut ops, &mut call, 0);
            let (kind, null_param) = cond_kind(data, vn, neg, 0);
            (kind, ops, call, null_param)
        }
        _ => (CondKind::Other, 0, false, false),
    }
}

fn expr_scan(data: &Funcdata, vn: VarnodeId, ops: &mut u32, call: &mut bool, depth: u32) {
    if depth > 16 || *ops > 64 {
        return;
    }
    let Some(v) = data.vbank().get(vn) else { return };
    let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else { return };
    if matches!(def.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        *call = true;
        return;
    }
    if !v.is_implied() {
        return;
    }
    *ops += 1;
    for i in 0..def.num_input() {
        if let Some(input) = def.get_in(i) {
            expr_scan(data, input, ops, call, depth + 1);
        }
    }
}

fn is_zero(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
    vn.and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_constant() && v.get_offset() == 0)
}

fn is_param(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
    vn.and_then(|v| data.vbank().get(v)).is_some_and(|v| v.is_input())
}

fn cond_kind(data: &Funcdata, vn: VarnodeId, neg: bool, depth: u32) -> (CondKind, bool) {
    let flip = |k: CondKind| if neg { k.negate() } else { k };
    let Some(v) = data.vbank().get(vn) else { return (flip(CondKind::Other), false) };
    if v.is_input() {
        return (flip(CondKind::Truthy), true);
    }
    let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else {
        return (flip(CondKind::Truthy), false);
    };
    match def.code() {
        OpCode::CPUI_BOOL_NEGATE if depth < 8 => match def.get_in(0) {
            Some(x) => cond_kind(data, x, !neg, depth + 1),
            None => (flip(CondKind::Other), false),
        },
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let eq = def.code() == OpCode::CPUI_INT_EQUAL;
            let (a, b) = (def.get_in(0), def.get_in(1));
            if is_zero(data, a) || is_zero(data, b) {
                let other = if is_zero(data, b) { a } else { b };
                (flip(if eq { CondKind::Eqz } else { CondKind::Truthy }), is_param(data, other))
            } else {
                (flip(if eq { CondKind::Eq } else { CondKind::Ne }), false)
            }
        }
        OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_FLOAT_LESS
        | OpCode::CPUI_FLOAT_LESSEQUAL => (CondKind::Rel, false),
        OpCode::CPUI_BOOL_AND => (flip(CondKind::And), false),
        OpCode::CPUI_BOOL_OR => (flip(CondKind::Or), false),
        OpCode::CPUI_INT_AND => (flip(CondKind::Bit), false),
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => (flip(CondKind::Call), false),
        _ => (flip(CondKind::Other), false),
    }
}

/// The site inputs [`compute`] reads.
pub(super) struct SiteParts<'a> {
    pub list: &'a [BlockId],
    /// The site's `if` in `list`.
    pub at: usize,
    /// The conditional part.
    pub part: &'a [BlockId],
    pub shape: TailShape,
    pub void: bool,
    pub key: Option<RetKey>,
    pub lines: usize,
    /// Returns in the part that do not print after the rewrite.
    pub hidden_rets: u32,
    pub conds: &'a [(BlockId, BlockId)],
    pub unopt: bool,
}

pub(super) fn compute(data: &Funcdata, s: &SiteParts) -> TailFeatures {
    let ifb = s.list[s.at];
    let mut t = Tally::default();
    for &p in s.part {
        tally(data, p, s.conds, &mut t);
    }
    t.rets = t.rets.saturating_sub(s.hidden_rets);
    t.ops = t.ops.saturating_sub(s.hidden_rets);
    let top: u32 = s.part.iter().map(|&p| top_of(data, p, s.conds)).sum::<u32>().saturating_sub(s.hidden_rets);
    let (last, last_bb) = s.part.last().map_or((LastKind::Other, None), |&l| last_kind(data, l, s.hidden_rets > 0));
    let last_tail = last_bb.and_then(|bb| data.bb_op_tail(bb)).and_then(|o| data.obank().get(o));
    let tail_call = last == LastKind::Call && last_tail.is_some_and(|o| o.code() == OpCode::CPUI_RETURN);
    let only_calls = t.calls > 0 && function_calls(data) == t.calls;
    let enter_neg = s.shape == TailShape::Guard;
    let (cond, cond_ops, cond_call, null_param) = cond_facts(data, cond_of(data, ifb, s.conds), enter_neg);
    let mut pre = Tally::default();
    let mut p_top = 0;
    let mut p_guards = 0;
    for &b in &s.list[..s.at] {
        tally(data, b, s.conds, &mut pre);
        p_top += top_of(data, b, s.conds);
        if plain_if(data, b) && bare_return(data, data.sblocks_ref().block(b).get_block(1)).is_some() {
            p_guards += 1;
        }
    }
    let k = match s.key {
        Some(RetKey::Void) | None => KKind::Void,
        Some(RetKey::Const(..)) => KKind::Const,
        Some(RetKey::Vn(_)) => KKind::Var,
    };
    TailFeatures {
        void: s.void,
        wrapped: s.shape == TailShape::Wrapped,
        lines: s.lines as u32,
        top,
        stmts: t.stmts(),
        calls: t.calls,
        assigns: t.assigns,
        rets: t.rets,
        ifs: t.ifs,
        loops: t.loops,
        switches: t.switches,
        gotos: t.gotos,
        last,
        tail_call,
        only_calls,
        cond,
        cond_ops,
        cond_call,
        null_param,
        k,
        p_top,
        p_stmts: pre.stmts(),
        p_rets: pre.rets,
        p_loops: pre.loops,
        p_guards,
        unopt: s.unopt,
    }
}

/// The training dump's directory, read once.
pub(super) fn dump_dir() -> Option<&'static std::path::Path> {
    static DIR: OnceLock<Option<std::path::PathBuf>> = OnceLock::new();
    DIR.get_or_init(|| std::env::var_os("KUNA_TAILFORM_DUMP").map(std::path::PathBuf::from)).as_deref()
}

/// Append one site to `<dir>/<pid>.jsonl`.
pub(super) fn dump(
    data: &Funcdata,
    dir: &std::path::Path,
    idx: u32,
    site_addr: u64,
    policy: &super::TailFormPolicy,
    decision: super::TailDecision,
    f: &TailFeatures,
) {
    let file = data
        .get_arch()
        .loader
        .as_ref()
        .and_then(|l| l.try_borrow().ok().map(|l| l.get_file_name().to_string()))
        .unwrap_or_default();
    let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"");
    let line = format!(
        "{{\"file\": \"{}\", \"addr\": {}, \"fn\": \"{}\", \"idx\": {idx}, \"site\": {site_addr}, \"rule\": \"{:?}\", \"decision\": \"{:?}\", \"feat\": {}}}\n",
        esc(&file),
        data.get_address().get_offset(),
        esc(data.get_name()),
        policy.rule,
        decision,
        f.json()
    );
    let path = dir.join(format!("{}.jsonl", std::process::id()));
    if let Ok(mut out) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = out.write_all(line.as_bytes());
    }
}

/// The address that names a site: its condition's branch.
pub(super) fn site_addr(data: &Funcdata, ifb: BlockId, conds: &[(BlockId, BlockId)]) -> u64 {
    let g = data.sblocks_ref();
    let mut c = cond_of(data, ifb, conds);
    loop {
        let blk = g.block(c);
        match blk.get_type() {
            BlockType::Copy => {
                return blk
                    .get_copy()
                    .and_then(|bb| data.bb_op_tail(bb))
                    .and_then(|o| data.obank().get(o))
                    .map_or(0, |o| o.get_addr().get_offset());
            }
            BlockType::Condition | BlockType::Ls | BlockType::Graph if blk.get_size() > 0 => {
                c = if blk.get_type() == BlockType::Condition { blk.get_block(0) } else { blk.get_block(blk.get_size() - 1) };
            }
            _ => return 0,
        }
    }
}
