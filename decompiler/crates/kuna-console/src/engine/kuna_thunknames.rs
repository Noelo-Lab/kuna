//! (kuna `thunknames`) Name a placeholder jump thunk after the function it jumps to.
//!
//! A function whose first instruction is a direct unconditional branch to another
//! function's entry, and does nothing else, is a jump thunk: its whole body is
//! that branch. A delay slot belongs to its branch, so a MIPS `j f; addiu
//! a0,a0,16` (a `this` adjustor) is not one. A linker emits
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
//! free `_<n>` suffix, in thunk address order within a round. Once every
//! prototype stream has run, [`inherit_prototypes`] gives each renamed thunk
//! the prototype, prototype model and Swift `self` classification of its
//! chain's target. A thunk with a name of its own keeps its own prototype
//! recovery: its source signature need not be the target's.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::database::SymbolId;

use super::*;

const MAX_CHAIN_HOPS: usize = 4;

/// Every `(thunk, target)` among `entries`: an entry whose first instruction is a
/// direct unconditional branch to another address and does nothing else. A
/// delay slot is part of its branch, so MIPS `j f; addiu a0,a0,16` (a `this`
/// adjustor) is not a thunk.
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
            if insn.ops.len() != 1 {
                return None;
            }
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
) -> KunaResult<BTreeSet<u64>> {
    let mut renamed = BTreeSet::new();
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
            renamed.insert(addr.get_offset());
            prog.register_symbol(&name, addr);
        }
    }
    Ok(renamed)
}

/// Every `renamed` thunk of `jumps` takes what its chain's target carries for a
/// call to it: the parked prototype and the bound prototype model (each only where the
/// thunk has none of its own), and the target's Swift `self` classification. A
/// call through the thunk is then typed as a call to the target.
pub(super) fn inherit_prototypes(
    prog: &mut ConsoleProgram,
    code_space: &Rc<AddrSpace>,
    jumps: &BTreeMap<u64, u64>,
    renamed: &BTreeSet<u64>,
) {
    if renamed.is_empty() {
        return;
    }
    let space = code_space.get_index();
    let mut models = (*prog.arch().loader_proto_models).clone();
    let mut abi = prog.arch().loader_swift_abi.as_deref().map(|abi| kuna_decomp::kuna_swiftcall::SwiftAbi {
        self_reg: abi.self_reg.clone(),
        error_reg: abi.error_reg.clone(),
        call: Rc::clone(&abi.call),
        throws: Rc::clone(&abi.throws),
        maybe_self: abi.maybe_self.clone(),
        no_self: abi.no_self.clone(),
    });
    for &thunk in renamed {
        let Some(target) = jumps.get(&thunk).and_then(|&first| chain_end(jumps, thunk, first)) else {
            continue;
        };
        if let Some(model) = models.get(&(space, target)).cloned() {
            models.entry((space, thunk)).or_insert(model);
        }
        if let Some(abi) = abi.as_mut() {
            if abi.maybe_self.contains(&(space, target)) {
                abi.maybe_self.insert((space, thunk));
            }
            if abi.no_self.contains(&(space, target)) {
                abi.no_self.insert((space, thunk));
            }
        }
        let symboltab = &prog.arch().symboltab;
        let Some((sid, _)) = symboltab.find_function_across_scopes(&Address::new(Rc::clone(code_space), thunk))
        else {
            continue;
        };
        if symboltab.function_proto_pieces_of(sid).is_some() {
            continue;
        }
        let Some(mut pieces) = symboltab
            .find_function_across_scopes(&Address::new(Rc::clone(code_space), target))
            .and_then(|(target_sid, _)| symboltab.function_proto_pieces_of(target_sid))
            .cloned()
        else {
            continue;
        };
        pieces.name = symboltab.symbol(sid).get_name().to_string();
        prog.arch_mut().symboltab.set_function_proto_pieces(sid, pieces);
    }
    prog.arch_mut().loader_proto_models = Rc::new(models);
    if let Some(abi) = abi {
        prog.arch_mut().loader_swift_abi = Some(Rc::new(abi));
    }
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
