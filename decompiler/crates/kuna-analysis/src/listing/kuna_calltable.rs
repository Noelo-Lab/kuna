//! (kuna `calltable`) Tables of code addresses the Listing reaches through a
//! computed jump or call.
//!
//! A table a computed transfer indexes is either a switch or a table of
//! functions, and [`is_call_table`] votes: an indexed CALL, or a jump whose
//! distinct targets are 40% or more other functions' entries, dispatches to
//! functions (a call table); any other jump table is a switch. A jump through
//! a pointer table with no range check is a call table only when every target
//! is already another function's entry, and is otherwise left alone: an
//! unchecked switch or a computed goto reads the same way. A table with no
//! range check ends before the first target that is already code inside a
//! function ([`call_table_run`]), which is where the read ran into the next
//! table.
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
use super::kuna_switchtable::{preserved_registers, register_targets, Reach, TableRead, DEFAULT_MAX_ENTRIES};
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
/// `unchecked` says no range check bounded the read; `foreign` says a target
/// is the entry of a function other than the jump's own.
pub(super) fn is_call_table(cases: &[u64], unchecked: bool, foreign: impl Fn(u64) -> bool) -> bool {
    let distinct: BTreeSet<u64> = cases.iter().copied().collect();
    if distinct.is_empty() {
        return false;
    }
    let foreign = distinct.iter().filter(|&&t| foreign(t)).count();
    if unchecked {
        foreign == distinct.len()
    } else {
        foreign * 5 >= distinct.len() * 2
    }
}

/// The entries of a call table with no range check: the leading run of
/// `cases` before the first target `inside` says is already code inside a
/// function, or nothing when fewer than two remain. An ARM word with the low
/// bit set is a Thumb pointer and is not taken.
pub(super) fn call_table_run(mut cases: Vec<u64>, arm: bool, inside: impl Fn(u64) -> bool) -> Vec<u64> {
    if let Some(at) = cases.iter().position(|&t| inside(t)) {
        cases.truncate(at);
    }
    cases.retain(|&t| !(arm && t & 1 == 1));
    if cases.len() < 2 {
        cases.clear();
    }
    cases
}

/// Read every computed jump and call's table in the Listing.
pub fn scan(file: &object::File, listing: &Listing, arch: &Architecture, translate: &dyn Translate) -> CallTables {
    let mut out = CallTables::default();
    let Some(pool) = PoolImage::new(file) else { return out };
    let Some(code_space) = arch.manage().get_default_code_space().map(Rc::clone) else { return out };
    let data_space = arch.manage().get_default_data_space().map(Rc::clone);
    let decoded: HashSet<u64> = listing.instructions().map(|(&a, _)| a).collect();
    let preserved = preserved_registers(arch);
    let named_set = std::cell::OnceCell::new();
    let named = |at: u64| named_set.get_or_init(|| pool.code_named(file)).contains(&at);
    let exec = listing.exec_ranges().to_vec();
    let entries: BTreeSet<u64> = listing.functions().map(|(&e, _)| e).collect();
    let tables = super::kuna_switchextent::FunctionTables::new(&pool);
    let next_table = |from: u64, table: u64| {
        let lo = entries.range(..=from).next_back().copied()?;
        let hi = entries.range(from + 1..).next().copied();
        tables.next_after(translate, &code_space, data_space.as_ref(), &exec, lo, hi, table)
    };
    let reach = Reach {
        max_entries: match arch.max_jumptable_size {
            0 => DEFAULT_MAX_ENTRIES,
            n => n as usize,
        },
        preserved: &preserved,
        settled: true,
        named: Some(&named),
        next_table: arch.switch_extent.then_some(&next_table),
        entered: None,
    };
    let arm = object::Object::architecture(file) == object::Architecture::Arm;
    let mut calls: BTreeSet<u64> = BTreeSet::new();
    let mut cases: BTreeSet<u64> = BTreeSet::new();
    // An indexed call can only add an entry where a read-only word points at
    // code the walk did not decode; without one the calls are not read at all.
    let undecoded = |word: u64| {
        exec.iter().any(|&(lo, hi)| word >= lo && word < hi) && listing.instruction_at(word).is_none()
    };
    let read_calls = pool.words().any(undecoded);
    let mut cap = FullCapture::default();
    let mut reads = Vec::new();
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
        let read = register_targets(translate, &code_space, data_space.as_ref(), &|a| decoded.contains(&a), vma, target, &pool, &exec, &reach);
        let owner = listing.function_containing(vma).map(|f| f.entry);
        reads.push((insn.flow.is_call, owner, read));
    }
    let unchecked = |call: bool, read: &TableRead| read.unbounded || (call && read.stated.is_none());
    let known = &entries;
    let foreign = |owner: Option<u64>| move |t: u64| known.contains(&t) && Some(t) != owner;
    for (call, owner, read) in &reads {
        if !call && !unchecked(*call, read) && !is_call_table(&read.cases, false, foreign(*owner)) {
            cases.extend(read.cases.iter().copied());
        }
    }
    for (call, owner, read) in reads {
        if unchecked(call, &read) {
            let inside = |t: u64| cases.contains(&t) || (decoded.contains(&t) && !entries.contains(&t));
            let run = call_table_run(read.cases, arm, inside);
            if call || is_call_table(&run, true, foreign(owner)) {
                calls.extend(run);
            }
        } else if call || is_call_table(&read.cases, false, foreign(owner)) {
            calls.extend(read.cases);
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
