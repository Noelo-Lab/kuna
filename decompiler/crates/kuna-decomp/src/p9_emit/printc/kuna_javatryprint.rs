//! (kuna `javatry`) Printing a DEX method's `java_try` dispatches as Java
//! `try { } catch (T e) { }`.
//!
//! Flow enters each catch handler from a dispatch at the start of its first
//! protected range (`p2_lift/kuna_javatry.rs`), and structuring drops the
//! exceptional edges, so the dispatch structures as an `if` whose one arm is the
//! handler. The other side is the protected code: the `if`'s other arm, or the
//! statements that follow it in the enclosing list. The protected part is the
//! leading run of those statements that starts inside the handler's ranges; the
//! rest follows the `try`. Consecutive dispatches with the same ranges are the
//! catch clauses of one `try`.

use kuna_num::opcodes::OpCode;

use super::{modifiers, to_emit_brace, Architecture, Funcdata, OpId, PrintC};
use crate::block::BlockType;
use crate::context::BlockId;
use crate::kuna_javatry::{caught_handler, dispatch_handler, CatchTable};
use crate::prettyprint::{Emit, SyntaxHighlight};

/// Where a part of a `try` statement comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    /// One structured block.
    Block(BlockId),
    /// The enclosing list's statements from this index on.
    Siblings(usize),
    /// Nothing: a handler that is the code after the `try` (an empty catch).
    Empty,
    /// The unstructured jump of an `if (dispatch) goto` BlockIf: a handler that
    /// is code reached by other paths too (a loop's next iteration).
    Goto(BlockId),
}

/// A list element and, for a basic block that straddles the end of the
/// protected range, which of its statements print: those inside the range
/// (`Some(true)`) or past it (`Some(false)`).
type Piece = (BlockId, Option<bool>);

/// A `java_try` dispatch, read as a `try` statement.
#[derive(Debug)]
pub(super) struct TryShape {
    /// The block holding the first dispatch; its other statements print first.
    cond: BlockId,
    catches: Vec<(u64, Part)>,
    body: Part,
    /// Siblings consumed by the clauses (the body's out-of-range tail is not).
    consumed: usize,
}

fn front_addr(fd: &Funcdata, blk: BlockId) -> Option<u64> {
    let leaf = fd.sblocks_ref().get_front_leaf(blk)?;
    let bb = fd.sblocks_ref().block(leaf).get_copy()?;
    Some(crate::block::block_get_start(&fd.bblocks_ref().arena, bb).get_offset())
}

fn basic_of(fd: &Funcdata, blk: BlockId) -> Option<BlockId> {
    let sb = fd.sblocks_ref();
    match sb.block(blk).get_type() {
        BlockType::Copy => sb.block(blk).get_copy(),
        _ => None,
    }
}

/// The handler `blk`'s condition dispatches to, and the basic block that
/// dispatch enters.
fn if_dispatch(fd: &Funcdata, blk: BlockId) -> Option<(u64, BlockId)> {
    let sb = fd.sblocks_ref();
    if sb.block(blk).get_type() != BlockType::If {
        return None;
    }
    let cond = sb.block(blk).get_block(0);
    let leaf = sb.get_exit_leaf(cond)?;
    let bb = sb.block(leaf).get_copy()?;
    let tail = fd.bb_op_tail(bb)?;
    let handler = dispatch_handler(fd, tail)?;
    let block = fd.bblocks_ref().block(bb);
    if block.size_out() != 2 {
        return None;
    }
    let taken = if fd.obank().get(tail)?.is_fallthru_true() { 0 } else { 1 };
    Some((handler, block.get_out(taken)))
}

/// Does `blk` start at the basic block `target`?
fn starts_at(fd: &Funcdata, blk: BlockId, target: BlockId) -> bool {
    let sb = fd.sblocks_ref();
    sb.get_front_leaf(blk).and_then(|leaf| sb.block(leaf).get_copy()) == Some(target)
}

/// Does `blk` hold nothing but a dispatch (so a dispatch chained after another
/// prints no statements of its own)?
fn pure_dispatch(fd: &Funcdata, blk: BlockId) -> bool {
    let Some(bb) = basic_of(fd, blk) else { return false };
    let mut cur = fd.bb_op_head(bb);
    while let Some(op) = cur {
        cur = fd.bb_op_next(op);
        let Some(o) = fd.obank().get(op) else { continue };
        if o.code() == OpCode::CPUI_CBRANCH || o.not_printed() {
            continue;
        }
        let implied = o.get_out().and_then(|v| fd.vbank().get(v)).is_some_and(|v| v.is_implied());
        if o.code() == OpCode::CPUI_CALLOTHER && implied {
            continue;
        }
        return false;
    }
    true
}

/// Does the statement rooted at `op` execute an instruction inside `handler`'s
/// ranges? The statement holds `op` and the ops whose outputs it inlines.
pub(super) fn statement_in_range(fd: &Funcdata, table: &CatchTable, handler: u64, op: OpId) -> bool {
    let mut stack = vec![op];
    let mut seen = 0;
    while let Some(cur) = stack.pop() {
        seen += 1;
        let Some(o) = fd.obank().get(cur) else { continue };
        if table.covers(handler, o.get_addr().get_offset()) {
            return true;
        }
        if seen > 64 {
            break;
        }
        for i in 0..o.num_input() {
            let Some(v) = o.get_in(i).and_then(|v| fd.vbank().get(v)) else { continue };
            if v.is_implied() {
                if let Some(def) = v.get_def() {
                    stack.push(def);
                }
            }
        }
    }
    false
}

impl PrintC {
    /// The `try` statement the BlockIf `blk` opens, with the enclosing list's
    /// `siblings` after it.
    pub(super) fn java_try_shape(&self, fd: &Funcdata, blk: BlockId, siblings: &[BlockId]) -> Option<TryShape> {
        let table = self.java_catches.as_ref()?;
        let (first, _) = if_dispatch(fd, blk)?;
        let ranges = table.ranges(first);
        let sb = fd.sblocks_ref();
        let cond = sb.block(blk).get_block(0);
        let mut shape = TryShape { cond, catches: Vec::new(), body: Part::Siblings(0), consumed: 0 };
        let mut cur = blk;
        loop {
            let (handler, target) = if_dispatch(fd, cur)?;
            let size = sb.block(cur).get_size();
            if size == 1 && sb.block(cur).get_if_goto_target().is_some() {
                shape.catches.push((handler, Part::Goto(cur)));
                shape.body = Part::Siblings(shape.consumed);
                return Some(shape);
            }
            if size < 2 {
                return None;
            }
            let then = sb.block(cur).get_block(1);
            let is_handler = |b: BlockId| starts_at(fd, b, target);
            let (arm, body) = match size {
                3 => {
                    let other = sb.block(cur).get_block(2);
                    if is_handler(then) {
                        (Part::Block(then), Part::Block(other))
                    } else if is_handler(other) {
                        (Part::Block(other), Part::Block(then))
                    } else {
                        return None;
                    }
                }
                2 if is_handler(then) => (Part::Block(then), Part::Siblings(shape.consumed)),
                2 if siblings.get(shape.consumed).is_some_and(|&b| is_handler(b)) => {
                    (Part::Siblings(shape.consumed), Part::Block(then))
                }
                2 => (Part::Empty, Part::Block(then)),
                _ => return None,
            };
            shape.catches.push((handler, arm));
            shape.body = body;
            match arm {
                Part::Siblings(_) => {
                    shape.consumed = siblings.len();
                    return Some(shape);
                }
                Part::Empty | Part::Goto(_) => return Some(shape),
                Part::Block(_) => {}
            }
            let next = match body {
                Part::Empty | Part::Goto(_) => return Some(shape),
                Part::Block(b) => b,
                Part::Siblings(k) => match siblings.get(k) {
                    Some(&b) => b,
                    None => return Some(shape),
                },
            };
            let chained = if_dispatch(fd, next)
                .filter(|&(h, _)| table.ranges(h) == ranges)
                .is_some_and(|_| pure_dispatch(fd, sb.block(next).get_block(0)));
            if !chained {
                return Some(shape);
            }
            if let Part::Siblings(k) = body {
                shape.consumed = k + 1;
            }
            cur = next;
        }
    }

    /// Does `shape` print from the BlockIf alone, without the list around it?
    pub(super) fn java_try_standalone(shape: &TryShape) -> bool {
        matches!(shape.body, Part::Block(_)) && shape.catches.iter().all(|c| !matches!(c.1, Part::Siblings(_)))
    }

    /// The statements a part prints, as list elements (nested lists flattened).
    fn part_elements(&self, fd: &Funcdata, part: Part, siblings: &[BlockId]) -> Vec<BlockId> {
        let roots: Vec<BlockId> = match part {
            Part::Block(b) => vec![b],
            Part::Siblings(k) => siblings.get(k..).unwrap_or(&[]).to_vec(),
            Part::Empty | Part::Goto(_) => Vec::new(),
        };
        let mut out = Vec::new();
        let mut stack: Vec<BlockId> = roots.into_iter().rev().collect();
        while let Some(b) = stack.pop() {
            if fd.sblocks_ref().block(b).get_type() == BlockType::Ls {
                stack.extend(fd.sblocks_ref().block(b).get_list().iter().rev());
            } else {
                out.push(b);
            }
        }
        out
    }

    /// Can an instruction of `blk` throw (of a straddling block, only those past
    /// the protected range)?
    fn part_can_throw(&self, fd: &Funcdata, table: &CatchTable, (blk, filter): Piece, handler: u64) -> bool {
        let sb = fd.sblocks_ref();
        let mut stack = vec![blk];
        while let Some(b) = stack.pop() {
            if let Some(bb) = basic_of(fd, b) {
                let mut cur = fd.bb_op_head(bb);
                while let Some(op) = cur {
                    cur = fd.bb_op_next(op);
                    let Some(addr) = fd.obank().get(op).map(|o| o.get_addr().get_offset()) else { continue };
                    if filter.is_some_and(|inside| table.covers(handler, addr) != inside) {
                        continue;
                    }
                    if table.can_throw_at(addr) {
                        return true;
                    }
                }
            } else {
                stack.extend_from_slice(sb.block(b).get_list());
            }
        }
        false
    }

    /// Does the basic block `blk` hold statements past `handler`'s ranges?
    fn straddles(&self, fd: &Funcdata, table: &CatchTable, blk: BlockId, handler: u64) -> bool {
        let Some(bb) = basic_of(fd, blk) else { return false };
        let mut cur = fd.bb_op_head(bb);
        while let Some(op) = cur {
            cur = fd.bb_op_next(op);
            if !self.is_suppressed_statement(fd, op) && self.java_visible(fd, bb, op) && !statement_in_range(fd, table, handler, op) {
                return true;
            }
        }
        false
    }

    /// Does control leave `blk` only by `return` or `throw`?
    fn part_exits(&self, fd: &Funcdata, blk: BlockId) -> bool {
        let sb = fd.sblocks_ref();
        let Some(bb) = sb.get_exit_leaf(blk).and_then(|l| sb.block(l).get_copy()) else { return false };
        fd.bb_op_tail(bb).and_then(|op| fd.obank().get(op)).is_some_and(|o| o.code() == OpCode::CPUI_RETURN)
    }

    /// Print the `try` statement `shape`; returns how many of `siblings` it
    /// printed.
    pub(super) fn emit_java_try(&mut self, fd: &Funcdata, arch: &Architecture, shape: TryShape, siblings: &[BlockId]) -> usize {
        let Some(table) = self.java_catches.clone() else { return 0 };
        let first = shape.catches[0].0;
        // The protected statements and the rest, in units the caller can skip:
        // the body block, or each sibling.
        let units: Vec<Vec<BlockId>> = match shape.body {
            Part::Block(_) | Part::Empty | Part::Goto(_) => vec![self.part_elements(fd, shape.body, siblings)],
            Part::Siblings(k) => {
                (k..siblings.len()).map(|i| self.part_elements(fd, Part::Block(siblings[i]), siblings)).collect()
            }
        };
        let mut inside: Vec<Piece> = Vec::new();
        let mut after: Vec<Piece> = Vec::new();
        // The protected statements run to the last one inside the ranges; a
        // statement between two of them that cannot throw is inside too (D8
        // leaves such instructions out of the ranges).
        let flat: Vec<(usize, BlockId)> =
            units.iter().enumerate().flat_map(|(u, list)| list.iter().map(move |&b| (u, b))).collect();
        let mut split = 0;
        for (k, &(_, b)) in flat.iter().enumerate() {
            if front_addr(fd, b).is_some_and(|a| table.covers(first, a)) {
                split = k + 1;
            } else if self.part_can_throw(fd, &table, (b, None), first) {
                break;
            }
        }
        let used_units = flat.get(..split).and_then(|s| s.last()).map_or(0, |&(u, _)| u + 1);
        inside.extend(flat[..split].iter().map(|&(_, b)| (b, None)));
        after.extend(flat[split..].iter().filter(|&&(u, _)| u < used_units).map(|&(_, b)| (b, None)));
        if let Some(&(last, _)) = inside.last() {
            if self.straddles(fd, &table, last, first) {
                inside.last_mut().expect("last").1 = Some(true);
                after.insert(0, (last, Some(false)));
            }
        }
        let mut consumed = shape.consumed;
        let mut loose_from: Option<usize> = None;
        if let Part::Siblings(k) = shape.body {
            consumed = k + used_units;
        } else if !after.is_empty() {
            // The protected code's own tail: after the `try` when no clause can
            // fall into it, else inside when it cannot throw.
            let clauses_exit = shape.catches.iter().all(|&(_, arm)| match arm {
                Part::Block(b) => self.part_exits(fd, b),
                Part::Siblings(_) => siblings.last().is_some_and(|&b| self.part_exits(fd, b)),
                Part::Empty => false,
                Part::Goto(_) => true,
            });
            if !clauses_exit {
                if after.iter().any(|&p| self.part_can_throw(fd, &table, p, first)) {
                    loose_from = Some(inside.len());
                }
                if loose_from.is_none() {
                    if let Some(straddler) = inside.last_mut().filter(|p| p.1.is_some()) {
                        straddler.1 = None;
                        after.remove(0);
                    }
                }
                inside.append(&mut after);
            }
        }

        let mut registered_pending = false;
        let mut my_pending_gen = 0u64;
        if self.context.is_set(modifiers::PENDING_BRACE) {
            self.emit.set_pending_brace(to_emit_brace(self.options.brace_ifelse));
            my_pending_gen = self.emit.pending_reg_gen();
            registered_pending = true;
        }
        self.context.push_mod();
        self.context.unset_mod(modifiers::NO_BRANCH | modifiers::ONLY_BRANCH | modifiers::PENDING_BRACE);

        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_block(fd, arch, shape.cond);
        self.context.pop_mod();
        self.emit_comment_block_tree(fd, shape.cond);

        self.emit.tag_line();
        let my_pending_indent = if registered_pending { self.emit.pending_fired_indent(my_pending_gen) } else { -1 };
        self.emit.print("try", SyntaxHighlight::KeywordColor);
        self.emit_java_try_part(fd, arch, &inside, first, loose_from);
        for &(handler, arm) in &shape.catches {
            let name = self.java_caught_name(fd, handler, &self.part_elements(fd, arm, siblings));
            self.emit.tag_line();
            self.emit.print("catch", SyntaxHighlight::KeywordColor);
            self.emit.spaces(1, 0);
            self.emit.print("(", SyntaxHighlight::NoColor);
            self.emit.print(&table.type_text(handler), SyntaxHighlight::TypeColor);
            self.emit.spaces(1, 0);
            self.emit.print(&name, SyntaxHighlight::VarColor);
            self.emit.print(")", SyntaxHighlight::NoColor);
            self.java_caught_shown.insert(handler);
            if let Part::Goto(blk) = arm {
                self.emit_java_catch_goto(fd, blk);
                continue;
            }
            let clause: Vec<Piece> = self.part_elements(fd, arm, siblings).into_iter().map(|b| (b, None)).collect();
            self.emit_java_try_part(fd, arch, &clause, first, None);
        }
        self.context.pop_mod();
        if my_pending_indent >= 0 {
            self.emit.close_brace_indent(self.lang().kw_close_curly, my_pending_indent);
        }
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        self.emit_java_piece_run(fd, arch, &after, first);
        self.context.pop_mod();
        consumed
    }

    fn emit_java_try_piece(&mut self, fd: &Funcdata, arch: &Architecture, (blk, filter): Piece, handler: u64) {
        let id = self.emit.begin_block(0);
        self.java_push_filter(fd, blk, filter, handler);
        self.emit_block(fd, arch, blk);
        self.java_pop_filter(fd, blk, filter);
        self.emit.end_block(id);
    }

    /// Filters are keyed by the basic block a straddling `BlockCopy` mirrors,
    /// the block `emit_basic_block_ops` prints.
    fn java_push_filter(&mut self, fd: &Funcdata, blk: BlockId, filter: Option<bool>, handler: u64) {
        if let (Some(inside), Some(bb)) = (filter, basic_of(fd, blk)) {
            self.java_block_filters.entry(bb).or_default().push((handler, inside));
        }
    }

    fn java_pop_filter(&mut self, fd: &Funcdata, blk: BlockId, filter: Option<bool>) {
        if let (Some(_), Some(bb)) = (filter, basic_of(fd, blk)) {
            if let Some(list) = self.java_block_filters.get_mut(&bb) {
                list.pop();
            }
        }
    }

    /// Does `op` in the basic block `bb` pass the ranges a straddling block
    /// prints under?
    pub(super) fn java_visible(&self, fd: &Funcdata, bb: BlockId, op: OpId) -> bool {
        let (Some(list), Some(table)) = (self.java_block_filters.get(&bb), self.java_catches.as_ref()) else {
            return true;
        };
        list.iter().all(|&(handler, inside)| statement_in_range(fd, table, handler, op) == inside)
    }

    /// A braced run of statements.
    /// A braced run of statements. From `loose_from` on, the statements lie
    /// past the protected range but print inside it (an exception they raise
    /// would not reach the catch); a comment says so.
    fn emit_java_try_part(&mut self, fd: &Funcdata, arch: &Architecture, pieces: &[Piece], handler: u64, loose_from: Option<usize>) {
        self.context.push_mod();
        self.context.set_mod(modifiers::NO_BRANCH);
        let id = self.emit.open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_ifelse));
        match loose_from {
            Some(k) => {
                self.emit_java_piece_run(fd, arch, &pieces[..k], handler);
                self.emit.tag_line();
                self.emit.print("// outside the try range from here: an exception below skips the catch", SyntaxHighlight::CommentColor);
                self.emit_java_piece_run(fd, arch, &pieces[k..], handler);
            }
            None => self.emit_java_piece_run(fd, arch, pieces, handler),
        }
        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
        self.context.pop_mod();
    }

    /// Statements in order, a straddling block printing only its share.
    fn emit_java_piece_run(&mut self, fd: &Funcdata, arch: &Architecture, pieces: &[Piece], handler: u64) {
        for &(blk, filter) in pieces {
            self.java_push_filter(fd, blk, filter, handler);
        }
        let blocks: Vec<BlockId> = pieces.iter().map(|p| p.0).collect();
        self.emit_block_ls_java_try(fd, arch, &blocks);
        for &(blk, filter) in pieces.iter().rev() {
            self.java_pop_filter(fd, blk, filter);
        }
    }

    /// A catch clause that is one jump: `{ continue; }`, `{ goto L; }`.
    fn emit_java_catch_goto(&mut self, fd: &Funcdata, blk: BlockId) {
        let sb = fd.sblocks_ref();
        let Some(target) = sb.block(blk).get_if_goto_target() else { return };
        let gototype = sb.block(blk).get_if_goto_type();
        let cond = sb.block(blk).get_block(0);
        let id = self.emit.open_brace_indent(self.lang().kw_open_curly, to_emit_brace(self.options.brace_ifelse));
        self.emit.tag_line();
        self.emit_goto_statement(fd, cond, target, gototype);
        self.emit.close_brace_indent(self.lang().kw_close_curly, id);
    }

    /// The name of the variable a handler binds: the `java_caught` output in
    /// the clause's own code.
    fn java_caught_name(&self, fd: &Funcdata, handler: u64, clause: &[BlockId]) -> String {
        let sb = fd.sblocks_ref();
        let mut stack: Vec<BlockId> = clause.to_vec();
        while let Some(b) = stack.pop() {
            if let Some(bb) = basic_of(fd, b) {
                let mut cur = fd.bb_op_head(bb);
                while let Some(op) = cur {
                    cur = fd.bb_op_next(op);
                    if caught_handler(fd, op) != Some(handler) {
                        continue;
                    }
                    let name = fd
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_out())
                        .and_then(|v| fd.vbank().get(v)?.get_high())
                        .and_then(|h| self.emitted_high_name(fd, h));
                    if let Some(name) = name {
                        return name;
                    }
                }
            } else {
                stack.extend_from_slice(sb.block(b).get_list());
            }
        }
        "e".to_string()
    }
}

