//! Prove which older issue slots can overlap a Carmel D2 reset.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::{address::Address, error::{KunaError, KunaResult}};

use crate::{context::OpId, flow::VisitedMap, funcdata::Funcdata};

const OVERLAP: usize = 4;

type IssueEdges<'a> = BTreeMap<u64, Option<&'a Address>>;

fn add_edge<'a>(edges: &mut IssueEdges<'a>, from: &Address, to: Option<&'a Address>) {
    if to == Some(from) {
        return;
    }
    edges.entry(from.get_offset()).and_modify(|other| {
        if *other != to {
            *other = None;
        }
    }).or_insert(to);
}

/// Visit logical issue edges, ignoring the internal branches of one p-code expansion.
fn issue_edges(fd: &Funcdata) -> (IssueEdges<'_>, IssueEdges<'_>) {
    let address = |id| fd.obank().get(id).unwrap().get_addr();
    let mut incoming = BTreeMap::new();
    let mut outgoing = BTreeMap::new();
    let mut link = |from, to| {
        add_edge(&mut incoming, to, from);
        if let Some(from) = from {
            add_edge(&mut outgoing, from, Some(to));
        }
    };
    let graph = fd.bblocks_ref();
    for index in 0..fd.bblocks_get_size() {
        let block = fd.bblocks_get_block(index);
        let mut prior = None;
        let mut op = fd.bb_op_head(block);
        if let Some(head) = op {
            let edges = graph.block(block);
            for edge in 0..edges.size_in() {
                let from = fd.bb_op_tail(edges.get_in(edge)).map(address);
                link(from, address(head));
            }
        }
        while let Some(id) = op {
            let addr = address(id);
            if let Some(from) = prior {
                link(Some(from), addr);
            }
            prior = Some(addr);
            op = fd.bb_op_next(id);
        }
    }
    (incoming, outgoing)
}

/// Move only the straight-line control effects, not the physical issue marker.
fn d2_before_ex(fd: &mut Funcdata, d2: OpId, end: OpId, ex: OpId) -> KunaResult<()> {
    if fd.obank().get(d2).unwrap().get_parent() != fd.obank().get(end).unwrap().get_parent() {
        return Err(KunaError::lowlevel("Nonlinear Carmel RESET control effects"));
    }
    let mut op = fd.bb_op_next(d2);
    while let Some(id) = op {
        if id == end {
            return Ok(());
        }
        op = fd.bb_op_next(id);
        fd.op_uninsert(id);
        fd.op_insert_before(id, ex);
    }
    Err(KunaError::lowlevel("Unterminated Carmel RESET control effects"))
}

pub(super) fn lower_control(fd: &mut Funcdata, visits: &VisitedMap, resets: &[OpId],
                            idle: impl Fn(u64) -> bool, cold_ex: Option<OpId>,
                            end: impl Fn(u64) -> Option<OpId>,
                            operand_fetch: impl Fn(u64) -> Option<OpId>) -> KunaResult<BTreeSet<OpId>> {
    let (incoming, outgoing) = issue_edges(fd);
    let entry = fd.get_address();
    let mut reorder = None;
    let mut canceled = BTreeSet::new();
    for &id in resets {
        let address = fd.obank().get(id).unwrap().get_addr();
        let mut current = address;
        let mut proved = false;
        for age in 0..=OVERLAP {
            if age == OVERLAP {
                proved = true;
                break;
            }
            match incoming.get(&current.get_offset()) {
                Some(Some(prior)) if outgoing.get(&prior.get_offset()) == Some(&Some(current))
                    && visits.get(*prior).is_some_and(|visit|
                        *prior + i64::from(visit.size) == *current) => {
                    if idle(prior.get_offset()) {
                        current = prior;
                        continue;
                    }
                    if age == 0 && prior.get_offset() == 0 {
                        // RA address zero survives; the register-only EX has no RA effects to preserve.
                        if *prior == entry && !incoming.contains_key(&0) {
                            if let (Some(ex), Some(end)) = (cold_ex, end(address.get_offset())) {
                                reorder = Some((id, end, ex));
                                proved = true;
                            }
                        }
                    } else if age < 2 {
                        if let Some(fetch) = operand_fetch(prior.get_offset()) {
                            canceled.insert(fetch);
                            current = prior;
                            continue;
                        }
                    }
                    break;
                }
                None if current == entry => {
                    // At nonzero entry, D2 needs occupied RA/OF history to avoid the startup alias.
                    proved = entry.get_offset() == 0 || age >= 2;
                    break;
                }
                _ => break,
            }
        }
        if !proved {
            return Err(KunaError::lowlevel(format!("Unproved Carmel RESET retirement window at CODE {:#x}",
                                                  address.get_offset() / 3)));
        }
    }
    if let Some((d2, end, ex)) = reorder {
        d2_before_ex(fd, d2, end, ex)?;
    }
    for &id in resets {
        fd.op_destroy(id);
    }
    Ok(canceled)
}
