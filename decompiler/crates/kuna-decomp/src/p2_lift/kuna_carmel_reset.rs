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

pub(super) fn prove_quiescent(fd: &Funcdata, visits: &VisitedMap, reset: &[OpId],
                             idle: impl Fn(u64) -> bool) -> KunaResult<()> {
    let incoming = issue_predecessors(fd);
    let entry = fd.get_address();
    for &id in reset {
        let address = fd.obank().get(id).unwrap().get_addr();
        let mut current = address;
        let mut proved = false;
        for age in 0..=OVERLAP {
            if age == OVERLAP {
                proved = true;
                break;
            }
            match incoming.get(&current.get_offset()) {
                Some(Some(prior)) if visits.get(*prior).is_some_and(|visit| {
                    *prior + i64::from(visit.size) == *current && idle(prior.get_offset())
                }) => current = prior,
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
    Ok(())
}
