//! Prove which older issue slots can overlap a Carmel D2 reset.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::{address::Address, error::{KunaError, KunaResult}};

use crate::{context::OpId, flow::VisitedMap, funcdata::Funcdata};

#[derive(Clone, Copy)]
enum Stage {
    Ra,
    Of,
    Ex,
    Wb,
}

#[derive(Default)]
pub(super) struct IssueEffects {
    pub idle: bool,
    pub register_ex: Option<OpId>,
    pub operand_fetch: Option<OpId>,
    pub staged_data: bool,
    pub reset_ex_end: Option<OpId>,
}

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
                            effects: &BTreeMap<u64, IssueEffects>) -> KunaResult<BTreeSet<OpId>> {
    let (incoming, outgoing) = issue_edges(fd);
    let entry = fd.get_address();
    let mut reorder = Vec::new();
    let mut canceled = BTreeSet::new();
    for &id in resets {
        let address = fd.obank().get(id).unwrap().get_addr();
        let mut current = address;
        let Some(end) = effects.get(&address.get_offset()).and_then(|issue| issue.reset_ex_end) else {
            return Err(KunaError::lowlevel("Missing Carmel RESET execution controls"));
        };
        let mut stages = [Stage::Ra, Stage::Of, Stage::Ex, Stage::Wb].into_iter();
        let proved = loop {
            let Some(stage) = stages.next() else { break true };
            let prior = match incoming.get(&current.get_offset()) {
                Some(Some(prior)) if outgoing.get(&prior.get_offset()) == Some(&Some(current))
                    && visits.get(*prior).is_some_and(|visit|
                        *prior + i64::from(visit.size) == *current) => *prior,
                None if current == entry => {
                    // A nonzero entry needs both younger stages filled to avoid its startup alias.
                    break entry.get_offset() == 0 || matches!(stage, Stage::Ex | Stage::Wb);
                }
                _ => break false,
            };
            let Some(issue) = effects.get(&prior.get_offset()) else { break false };
            if !issue.idle {
                match stage {
                    Stage::Ra if prior.get_offset() == 0 => {
                        // Physical RA address zero survives, but only the covered entry EX can move.
                        if prior == entry && !incoming.contains_key(&0) {
                            if let Some(ex) = issue.register_ex {
                                reorder.push((id, end, ex));
                                break true;
                            }
                        }
                        break false;
                    }
                    Stage::Ra | Stage::Of => {
                        let Some(fetch) = issue.operand_fetch else { break false };
                        canceled.insert(fetch);
                    }
                    Stage::Ex => {
                        let Some(ex) = issue.register_ex else { break false };
                        reorder.push((id, end, ex));
                    }
                    Stage::Wb if issue.staged_data => {}
                    Stage::Wb => break false,
                }
            }
            current = prior;
        };
        if !proved {
            return Err(KunaError::lowlevel(format!("Unproved Carmel RESET retirement window at CODE {:#x}",
                                                  address.get_offset() / 3)));
        }
    }
    for (d2, end, ex) in reorder {
        d2_before_ex(fd, d2, end, ex)?;
    }
    for &id in resets {
        fd.op_destroy(id);
    }
    Ok(canceled)
}
