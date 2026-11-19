//! (kuna `calltable`) Tables of code addresses the Listing reaches through a
//! computed jump or call.
//!
//! A table a computed transfer indexes is either a switch or a table of
//! functions, and [`is_call_table`] votes: an indexed CALL, or a jump whose
//! distinct targets are 40% or more other functions' entries, dispatches to
//! functions (a call table); any other jump table is a switch. A jump through
//! a pointer table with no range check is a call table on x86 only when every
//! target sits on a 16-byte function boundary (a switch whose range check the
//! slice did not reach has case labels in the middle of its function), and is
//! otherwise left alone.
//!
//! [`scan`] reads the tables with [`super::kuna_switchtable::register_targets`]
//! over the decoded Listing and returns both halves: the call-table targets as
//! new entries, and the discovered (not symbol) entries that sit on a case of a
//! switch and that no call reaches, which are not functions.

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use super::kuna_poolref::PoolImage;
use super::kuna_switchtable::{preserved_registers, register_targets, Reach, DEFAULT_MAX_ENTRIES};
use super::xrefs::{decode, FullCapture};
use super::{Listing, RefKind};

/// What the scan found.
#[derive(Debug, Default)]
pub struct CallTables {
    /// Targets of call tables: new function entries.
    pub entries: Vec<u64>,
    /// Discovered entries that are switch cases no call reaches.
    pub case_entries: Vec<u64>,
}

/// Whether the table a computed jump read (`cases`) dispatches to functions.
/// `unbounded` says no range check bounded the read; `foreign` says a target
/// is the entry of a function other than the jump's own.
pub(super) fn is_call_table(cases: &[u64], unbounded: bool, x86: bool, foreign: impl Fn(u64) -> bool) -> bool {
    let distinct: BTreeSet<u64> = cases.iter().copied().collect();
    if distinct.is_empty() {
        return false;
    }
    let aligned = x86 && distinct.len() >= 2 && distinct.iter().all(|t| t % 16 == 0);
    let foreign = distinct.iter().filter(|&&t| foreign(t)).count();
    (unbounded && aligned) || foreign * 5 >= distinct.len() * 2
}

/// Read every computed jump and call's table in the Listing.
pub fn scan(file: &object::File, listing: &Listing, arch: &Architecture, translate: &dyn Translate) -> CallTables {
    let mut out = CallTables::default();
    let Some(pool) = PoolImage::new(file) else { return out };
    let Some(code_space) = arch.manage().get_default_code_space().map(Rc::clone) else { return out };
    let data_space = arch.manage().get_default_data_space().map(Rc::clone);
    let decoded: HashSet<u64> = listing.instructions().map(|(&a, _)| a).collect();
    let preserved = preserved_registers(arch);
    let reach = Reach {
        max_entries: match arch.max_jumptable_size {
            0 => DEFAULT_MAX_ENTRIES,
            n => n as usize,
        },
        preserved: &preserved,
        settled: true,
    };
    let exec = listing.exec_ranges().to_vec();
    let x86 = arch.archid.starts_with("x86");
    let entries: BTreeSet<u64> = listing.functions().map(|(&e, _)| e).collect();
    let mut calls: BTreeSet<u64> = BTreeSet::new();
    let mut cases: BTreeSet<u64> = BTreeSet::new();
    // An indexed call can only add an entry where a read-only word points at
    // code the walk did not decode; without one the calls are not read at all.
    let undecoded = |word: u64| {
        exec.iter().any(|&(lo, hi)| word >= lo && word < hi) && listing.instruction_at(word).is_none()
    };
    let read_calls = pool.words().any(undecoded);
    let mut cap = FullCapture::default();
    for (&vma, insn) in listing.instructions() {
        if !insn.flow.is_computed || !insn.flows.is_empty() || (insn.flow.is_call && !read_calls) {
            continue;
        }
        if decode(translate, vma, &code_space, &mut cap).is_none() {
            continue;
        }
        let Some(target) = cap
            .ops()
            .iter()
            .find(|o| matches!(o.opcode, OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND))
            .and_then(|o| o.ins.first())
        else {
            continue;
        };
        let read = register_targets(translate, &code_space, data_space.as_ref(), &decoded, vma, target, &pool, &exec, &reach);
        let owner = listing.function_containing(vma).map(|f| f.entry);
        let foreign = |t: u64| entries.contains(&t) && Some(t) != owner;
        if insn.flow.is_call || is_call_table(&read.cases, read.unbounded, x86, foreign) {
            calls.extend(read.cases);
        } else if !read.unbounded {
            cases.extend(read.cases);
        }
    }
    out.entries = calls.iter().copied().filter(|t| !entries.contains(t)).collect();
    out.case_entries = listing
        .functions()
        .filter(|(e, f)| !f.from_symbol && cases.contains(e) && !calls.contains(e))
        .filter(|(&e, _)| !listing.refs_to(e).iter().any(|r| r.kind == RefKind::Call))
        .map(|(&e, _)| e)
        .collect();
    out
}
