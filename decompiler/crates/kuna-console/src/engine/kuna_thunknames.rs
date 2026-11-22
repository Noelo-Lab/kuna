//! (kuna `thunknames`) Name a placeholder jump thunk after the function it jumps to.
//!
//! A function whose first instruction is a direct unconditional branch to another
//! function's entry is a jump thunk: its whole body is that branch. A linker emits
//! them as branch islands and identical-code-folding forwarders, a compiler as
//! outlined tail calls; on arm64 each is one `b <target>`. Left `sub_<addr>`, every
//! call through one prints an anonymous name although its target is known.
//!
//! [`jump_thunks`] decodes the first instruction of every discovered entry and keeps
//! each direct unconditional branch. After every other name source has named the
//! targets, [`commit`] renames each thunk still called by a placeholder to
//! `j_<target>` when its target carries a name that is not a placeholder itself. A
//! thunk to a thunk is named in the next round, after its target (`j_j_<name>`),
//! for at most [`MAX_CHAIN_HOPS`] rounds. A name already in use takes the first
//! free `_<n>` suffix, in thunk address order within a round.

use std::collections::BTreeMap;

use kuna_decomp::database::SymbolId;

use super::*;

const MAX_CHAIN_HOPS: usize = 4;

/// Every `(thunk, target)` among `entries`: an entry whose first instruction is a
/// direct unconditional branch to another address.
pub(super) fn jump_thunks(
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    entries: &[u64],
) -> BTreeMap<u64, u64> {
    entries
        .iter()
        .filter_map(|&vma| {
            let insn =
                kuna_analysis::listing::decode::decode_one(arch.translate(), vma, code_space, false, false)
                    .ok()?;
            let flow = kuna_analysis::listing::classify::classify(&insn.ops, vma, insn.len);
            let target = match flow.flows.as_slice() {
                [target] if *target != vma => *target,
                _ => return None,
            };
            (flow.flow.kind == kuna_analysis::listing::model::FlowKind::UnconditionalBranch
                && !flow.flow.is_call)
                .then_some((vma, target))
        })
        .collect()
}

/// The thunks of `jumps` whose chain ends at one of `targets`.
pub(super) fn thunks_to(
    jumps: &BTreeMap<u64, u64>,
    targets: &std::collections::HashSet<u64>,
) -> Vec<u64> {
    jumps
        .iter()
        .filter(|&(&thunk, &first)| chain_end(jumps, thunk, first).is_some_and(|end| targets.contains(&end)))
        .map(|(&thunk, _)| thunk)
        .collect()
}

pub(super) fn commit(
    prog: &mut ConsoleProgram,
    code_space: &Rc<AddrSpace>,
    jumps: &BTreeMap<u64, u64>,
) -> KunaResult<()> {
    for _ in 0..MAX_CHAIN_HOPS {
        let mut names: Vec<(SymbolId, Address, String)> = Vec::new();
        for (&thunk, &target) in jumps {
            let addr = Address::new(Rc::clone(code_space), thunk);
            let Some((sid, _)) = prog.arch().symboltab.find_function_across_scopes(&addr) else {
                continue;
            };
            if !is_generic_placeholder_name(prog.arch().symboltab.symbol(sid).get_name()) {
                continue;
            }
            let target_addr = Address::new(Rc::clone(code_space), target);
            let Some(target_name) = prog.arch().symboltab.function_display_name_across_scopes(&target_addr)
            else {
                continue;
            };
            if !is_generic_placeholder_name(&target_name) {
                names.push((sid, addr, format!("j_{target_name}")));
            }
        }
        if names.is_empty() {
            break;
        }
        for (sid, addr, base) in names {
            let name = (0..)
                .map(|n| if n == 0 { base.clone() } else { format!("{base}_{n}") })
                .find(|name| !name_taken(prog, name))
                .expect("an unbounded suffix range has a free name");
            prog.arch_mut().symboltab.rename_symbol(sid, &name)?;
            prog.register_symbol(&name, addr);
        }
    }
    Ok(())
}

/// The last address of the jump chain from `thunk`, or `None` when the chain
/// comes back to `thunk`.
fn chain_end(jumps: &BTreeMap<u64, u64>, thunk: u64, first: u64) -> Option<u64> {
    let mut target = first;
    for _ in 1..MAX_CHAIN_HOPS {
        match jumps.get(&target) {
            Some(&next) if next != thunk => target = next,
            Some(_) => return None,
            None => break,
        }
    }
    (target != thunk).then_some(target)
}

fn name_taken(prog: &ConsoleProgram, name: &str) -> bool {
    prog.lookup_symbol(name).is_some()
        || prog.lookup_data_alias(name).is_some()
        || prog.arch().symboltab.get_global_scope().is_some_and(|global| {
            !prog.arch().symboltab.find_by_name(global, name).is_empty()
        })
}
