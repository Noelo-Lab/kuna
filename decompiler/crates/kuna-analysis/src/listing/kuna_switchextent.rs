//! (kuna `switchextent`) Where a jump table with no range check ends: at the
//! next jump table of the function that dispatches through it.
//!
//! # The gap
//!
//! A compiler that knows its selector is in range checks nothing before the
//! dispatch: Swift's `switch` over a multi-payload enum indexes a halfword
//! offset table straight with the case `swift_getEnumCaseMultiPayload`
//! returned, and clang does the same for a C `switch` whose `default` is
//! `__builtin_unreachable()`. [`super::kuna_switchtable::guard_limit`] then
//! finds no bound, the walk stops at the `br`, and the case bodies become
//! gaps the aggressive instruction finder fills with functions of their own.
//!
//! # The rule
//!
//! The entries of a relative table are offsets, so they say nothing about
//! where the table ends; the table after it does. A function's jump tables
//! are emitted together, back to back in one read-only section, so a table
//! runs exactly to the lowest table of the same function above it. The
//! function's extent, from its entry to the next known entry, is decoded
//! linearly and folded with the [`super::kuna_switchtable`] lattice, and
//! every computed jump the fold proves reads a table names that table. A
//! function's last table has no such bound and is left as it was: an object
//! the function merely reads (a lookup array, a constant) may sit anywhere
//! in the section, so a bound taken from one could run the table on into
//! another function's, whose entries are plausible code addresses measured
//! from the wrong base. An extent that runs on into the next function only
//! adds that function's tables, which all lie beyond this function's.
//!
//! [`FunctionTables`] caches one scan per extent; the walk asks it once per
//! unchecked relative dispatch, which is rare outside Swift.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{fold, invalidate, key_of, value_of, Key, Val, MAX_TRACKED};
use super::xrefs::{decode, in_range, FullCapture};

/// How far past an entry the linear decode reaches when no known entry ends
/// the extent sooner: a Swift copy helper runs to 13 KB, nothing to a MiB.
const MAX_EXTENT: u64 = 1 << 20;

/// The jump tables the code of each decoded extent dispatches through, by
/// extent start.
pub(super) struct FunctionTables<'a> {
    pool: &'a PoolImage<'a>,
    tables: RefCell<HashMap<u64, Vec<u64>>>,
}

impl<'a> FunctionTables<'a> {
    pub(super) fn new(pool: &'a PoolImage<'a>) -> Self {
        FunctionTables { pool, tables: RefCell::new(HashMap::new()) }
    }

    /// The lowest jump table above `table`, in `table`'s read-only section,
    /// that the code in `[lo, hi)` dispatches through; `hi` of `None` runs to
    /// [`MAX_EXTENT`].
    pub(super) fn next_after(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        data: Option<&Rc<AddrSpace>>,
        exec: &[(u64, u64)],
        lo: u64,
        hi: Option<u64>,
        table: u64,
    ) -> Option<u64> {
        let (sec_lo, sec_hi) = self.pool.section_of(table)?;
        let mut tables = self.tables.borrow_mut();
        let found = tables.entry(lo).or_insert_with(|| {
            let hi = hi.unwrap_or(u64::MAX).min(lo.saturating_add(MAX_EXTENT));
            self.scan(translate, code_space, data, exec, lo, hi)
        });
        let at = found.partition_point(|&a| a <= table);
        found[at..].iter().copied().find(|&a| a >= sec_lo && a < sec_hi)
    }

    /// The table of every computed jump in a linear decode of `[lo, hi)` whose
    /// register the fold proves holds a table entry, sorted.
    fn scan(
        &self,
        translate: &dyn Translate,
        code_space: &Rc<AddrSpace>,
        data: Option<&Rc<AddrSpace>>,
        exec: &[(u64, u64)],
        lo: u64,
        hi: u64,
    ) -> Vec<u64> {
        let mut out = Vec::new();
        let mut cap = FullCapture::default();
        let mut vals: Vec<(Key, Val)> = Vec::new();
        let mut at = lo;
        while at < hi && in_range(exec, at) {
            let Some(len) = decode(translate, at, code_space, &mut cap) else { break };
            // What the instruction writes after a branch inside it joins two
            // values (`csel` picks one of its operands), so it is unknown.
            let mut joined = false;
            for op in cap.ops() {
                if op.opcode == OpCode::CPUI_BRANCHIND {
                    match op.ins.first().and_then(|vn| value_of(vn, &vals, data)) {
                        Some(Val::Relative { table, .. } | Val::Case { table, .. }) => out.push(table),
                        Some(Val::Entry { table, width, .. }) if width == self.pool.ptr_width() => out.push(table),
                        _ => {}
                    }
                }
                if matches!(op.opcode, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
                    vals.clear();
                }
                if matches!(op.opcode, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
                    && op.ins.first().and_then(|vn| vn.space.as_ref()).is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
                {
                    joined = true;
                }
                let folded = if joined { None } else { fold(op, &vals, data, self.pool) };
                let Some(key) = op.out.as_ref().and_then(|vn| key_of(vn, data)) else { continue };
                invalidate(&mut vals, Some(key));
                if let Some(val) = folded {
                    if vals.len() < MAX_TRACKED {
                        vals.push((key, val));
                    }
                }
            }
            at = at.wrapping_add(u64::from(len));
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}
