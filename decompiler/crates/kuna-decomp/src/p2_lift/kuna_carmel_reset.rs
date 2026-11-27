//! Prove which older issue slots can overlap a Carmel D2 reset.

use std::collections::BTreeMap;

use kuna_base::{address::Address, error::{KunaError, KunaResult}};

use crate::{context::OpId, flow::VisitedMap, funcdata::Funcdata};

const OVERLAP: usize = 4;

fn add_edge<'a>(incoming: &mut BTreeMap<u64, Option<&'a Address>>, to: &Address,
                from: Option<&'a Address>) {
    if from == Some(to) {
        return;
    }
    incoming.entry(to.get_offset()).and_modify(|prior| {
        if *prior != from {
            *prior = None;
        }
    }).or_insert(from);
}

/// Visit logical issue edges, ignoring the internal branches of one p-code expansion.
fn issue_predecessors(fd: &Funcdata) -> BTreeMap<u64, Option<&Address>> {
    let address = |id| fd.obank().get(id).unwrap().get_addr();
    let mut incoming = BTreeMap::new();
    let graph = fd.bblocks_ref();
    for index in 0..fd.bblocks_get_size() {
        let block = fd.bblocks_get_block(index);
        let mut prior = None;
        let mut op = fd.bb_op_head(block);
        if let Some(head) = op {
            let edges = graph.block(block);
            for edge in 0..edges.size_in() {
                let from = fd.bb_op_tail(edges.get_in(edge)).map(address);
                add_edge(&mut incoming, address(head), from);
            }
        }
        while let Some(id) = op {
            let addr = address(id);
            if let Some(from) = prior {
                add_edge(&mut incoming, addr, Some(from));
            }
            prior = Some(addr);
            op = fd.bb_op_next(id);
        }
    }
    incoming
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
                            end: impl Fn(u64) -> Option<OpId>) -> KunaResult<()> {
    let incoming = issue_predecessors(fd);
    let entry = fd.get_address();
    let mut reorder = None;
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
                Some(Some(prior)) if visits.get(*prior).is_some_and(|visit|
                    *prior + i64::from(visit.size) == *current) => {
                    if idle(prior.get_offset()) {
                        current = prior;
                        continue;
                    }
                    if age == 0 && *prior == entry && prior.get_offset() == 0 && !incoming.contains_key(&0) {
                        // RA address zero survives; the register-only EX has no RA effects to preserve.
                        if let (Some(ex), Some(end)) = (cold_ex, end(address.get_offset())) {
                            reorder = Some((id, end, ex));
                            proved = true;
                        }
                    }
                    break;
                }
                None if current == entry => {
                    // Only CODE zero is the physical empty-pipeline entry.
                    proved = entry.get_offset() == 0;
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
    Ok(())
}
