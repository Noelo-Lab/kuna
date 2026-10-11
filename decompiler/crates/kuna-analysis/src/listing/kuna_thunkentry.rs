//! (kuna) `thunkentry` — the target of a jump thunk is a function entry.
//!
//! [`super::walk`] makes a function only at a CALL target; every other flow
//! target is a same-function successor. A function whose whole body is one
//! direct `jmp` is a thunk, and the routine it jumps to was never a function of
//! its own: its body was attributed to the thunk. MSVC `/INCREMENTAL` routes
//! every call and the image entry through a table of `jmp rel32` thunks, so on
//! such an image the walk finds the thunks and none of the bodies.
//!
//! This reads the completed walk and promotes a jump target to a function entry
//! when all of these hold:
//!
//! 1. the jump is the first instruction of a function entry (a walk seed, a CALL
//!    target, or a target this pass already promoted), it is a direct
//!    unconditional branch, and the target is a decoded instruction that is not
//!    the thunk itself;
//! 2. no decoded instruction falls through into the target and no conditional
//!    branch targets it, so it is not the middle of a straight-line run or a
//!    loop head;
//! 3. the instruction right after the thunk's `jmp` is not ordinary code: it is
//!    undecoded, a function entry, another direct jump (the next thunk of a
//!    table), or the target itself. A function that opens with a jump over its
//!    own loop body to the loop condition fails this test, because the walk
//!    decoded the body as a branch target of the condition.
//!
//! Promoting a target cannot change which instructions the walk decodes: the
//! target is a branch successor of the thunk, so it is decoded either way. The
//! result is therefore a pure addition to the walk's function set, computed after
//! every rebuild. Rules 1 and 3 read the set as it grows, so acceptance runs to its
//! least fixpoint over a worklist, which no visiting order changes.
//!
//! A thunk may jump to another thunk (an MSVC CRT stub reaches its body through
//! the incremental-link table). The chain is followed at most [`MAX_CHAIN_HOPS`]
//! jumps from a walk entry, so an obfuscator's chain of tens of thousands of
//! jumps adds a few entries, not one per jump.
//!
//! x86 only (I386 and x86-64); every other architecture is a no-op.

use std::collections::{BTreeMap, BTreeSet};

use super::{FlowKind, Insn, Listing};

/// How many thunk-to-thunk jumps are followed from a walk entry.
const MAX_CHAIN_HOPS: usize = 4;

/// The thunk-target function entries of `listing`, address-sorted.
///
/// Every returned address is a decoded instruction start that is not already a
/// function of `listing`.
pub fn thunk_entries(file: &object::File, listing: &Listing) -> Vec<u64> {
    use object::read::Object;
    if !matches!(
        file.architecture(),
        object::Architecture::I386 | object::Architecture::X86_64
    ) {
        return Vec::new();
    }
    promote(listing)
}

/// The least fixpoint of the three rules over `listing`'s function set.
fn promote(listing: &Listing) -> Vec<u64> {
    let known: BTreeSet<u64> = listing.functions().map(|(&a, _)| a).collect();
    let jumps = thunk_jumps(listing, &known);
    let targets: BTreeSet<u64> = jumps
        .values()
        .copied()
        .filter(|t| !known.contains(t))
        .collect();
    if targets.is_empty() {
        return Vec::new();
    }
    let impure = impure_targets(listing, &targets);
    let mut accepted: BTreeSet<u64> = BTreeSet::new();
    let mut waiting: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    let mut work: Vec<u64> = jumps
        .keys()
        .copied()
        .filter(|t| known.contains(t))
        .collect();
    while let Some(thunk) = work.pop() {
        let target = jumps[&thunk];
        if !targets.contains(&target) || accepted.contains(&target) || impure.contains(&target) {
            continue;
        }
        let is_entry = |vma: u64| known.contains(&vma) || accepted.contains(&vma);
        match code_after(listing, thunk, target, &is_entry) {
            Some(next) => waiting.entry(next).or_default().push(thunk),
            None => {
                accepted.insert(target);
                if jumps.contains_key(&target) {
                    work.push(target);
                }
                work.extend(waiting.remove(&target).unwrap_or_default());
            }
        }
    }
    accepted.into_iter().collect()
}

/// The direct unconditional-branch target of the instruction at `vma`, when that
/// instruction is one and the target is another decoded instruction.
fn jump_target(listing: &Listing, vma: u64) -> Option<u64> {
    let insn = listing.instruction_at(vma)?;
    if insn.flow.kind != FlowKind::UnconditionalBranch || insn.flow.is_call || insn.flows.len() != 1
    {
        return None;
    }
    let target = insn.flows[0];
    (target != vma && listing.is_instruction_start(target)).then_some(target)
}

/// Every potential thunk keyed to its jump target: the known entries that open
/// with a direct jump, and the targets of those jumps that do too, up to
/// [`MAX_CHAIN_HOPS`] jumps from a known entry.
fn thunk_jumps(listing: &Listing, known: &BTreeSet<u64>) -> BTreeMap<u64, u64> {
    let mut jumps: BTreeMap<u64, u64> = BTreeMap::new();
    let mut frontier: Vec<u64> = known.iter().copied().collect();
    for _ in 0..MAX_CHAIN_HOPS {
        let mut next = Vec::new();
        for thunk in frontier {
            if jumps.contains_key(&thunk) {
                continue;
            }
            let Some(target) = jump_target(listing, thunk) else {
                continue;
            };
            jumps.insert(thunk, target);
            if !known.contains(&target) {
                next.push(target);
            }
        }
        frontier = next;
    }
    jumps
}

/// The candidate targets some decoded instruction falls through into or
/// conditionally branches to.
fn impure_targets(listing: &Listing, targets: &BTreeSet<u64>) -> BTreeSet<u64> {
    let mut impure = BTreeSet::new();
    for (_, insn) in listing.instructions() {
        if let Some(fall) = insn.fall_through {
            if targets.contains(&fall) {
                impure.insert(fall);
            }
        }
        if insn.flow.kind == FlowKind::ConditionalBranch {
            impure.extend(insn.flows.iter().copied().filter(|t| targets.contains(t)));
        }
    }
    impure
}

/// Rule 3: the address right after the thunk's jump when it holds ordinary
/// code, so the target may be the rest of the thunk's own body; `None` when it
/// does not.
fn code_after(
    listing: &Listing,
    thunk: u64,
    target: u64,
    is_entry: &dyn Fn(u64) -> bool,
) -> Option<u64> {
    let next = after(listing.instruction_at(thunk)?);
    let clear = next == target
        || !listing.is_instruction_start(next)
        || is_entry(next)
        || jump_target(listing, next).is_some();
    (!clear).then_some(next)
}

fn after(insn: &Insn) -> u64 {
    insn.addr.wrapping_add(u64::from(insn.len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::{DiscoveredFunction, FlowType};

    fn insn(addr: u64, len: u32, kind: FlowKind, flows: &[u64]) -> Insn {
        let falls = matches!(
            kind,
            FlowKind::Fallthrough | FlowKind::ConditionalBranch | FlowKind::Call
        );
        Insn {
            addr,
            len,
            fall_through: falls.then_some(addr + u64::from(len)),
            flow: FlowType {
                kind,
                is_call: kind == FlowKind::Call,
                is_jump: matches!(
                    kind,
                    FlowKind::UnconditionalBranch | FlowKind::ConditionalBranch
                ),
                is_terminal: kind == FlowKind::Return,
                is_conditional: kind == FlowKind::ConditionalBranch,
                has_fallthrough: falls,
                ..Default::default()
            },
            flows: flows.to_vec(),
            mnemonic: String::new(),
            operands: String::new(),
            pcode: None,
        }
    }

    fn jmp(addr: u64, to: u64) -> Insn {
        insn(addr, 5, FlowKind::UnconditionalBranch, &[to])
    }

    fn op(addr: u64, len: u32) -> Insn {
        insn(addr, len, FlowKind::Fallthrough, &[])
    }

    fn ret(addr: u64) -> Insn {
        insn(addr, 1, FlowKind::Return, &[])
    }

    fn listing(insns: Vec<Insn>, entries: &[u64]) -> Listing {
        let funcs = entries
            .iter()
            .map(|&entry| {
                let f = DiscoveredFunction {
                    entry,
                    name: None,
                    from_symbol: false,
                    has_no_return: false,
                    call_fixup: None,
                };
                (entry, f)
            })
            .collect();
        Listing::from_partition(
            insns.into_iter().map(|i| (i.addr, i)).collect(),
            funcs,
            vec![(0, u64::MAX)],
        )
    }

    fn accepted(l: &Listing) -> Vec<u64> {
        promote(l)
    }

    /// The issue's table: three adjacent thunks, then three bodies.
    #[test]
    fn a_thunk_table_promotes_every_target() {
        let l = listing(
            vec![
                jmp(0x1000, 0x1040),
                jmp(0x1005, 0x1010),
                jmp(0x100a, 0x1030),
                op(0x1010, 3),
                ret(0x1013),
                op(0x1030, 4),
                ret(0x1034),
                op(0x1040, 3),
                ret(0x1043),
            ],
            &[0x1000, 0x1005, 0x100a],
        );
        assert_eq!(accepted(&l), vec![0x1010, 0x1030, 0x1040]);
    }

    /// `f: jmp cond; body: ...; cond: ...; jne body` is one function.
    #[test]
    fn a_jump_to_a_rotated_loop_condition_is_not_a_thunk() {
        let fallen_into = listing(
            vec![
                jmp(0x1000, 0x1008),
                op(0x1005, 3),
                op(0x1008, 2),
                insn(0x100a, 2, FlowKind::ConditionalBranch, &[0x1005]),
                ret(0x100c),
            ],
            &[0x1000],
        );
        assert!(accepted(&fallen_into).is_empty());

        let jumped_to = listing(
            vec![
                jmp(0x1000, 0x1010),
                op(0x1005, 3),
                jmp(0x1008, 0x1010),
                op(0x1010, 2),
                insn(0x1012, 2, FlowKind::ConditionalBranch, &[0x1005]),
                ret(0x1014),
            ],
            &[0x1000],
        );
        assert!(
            accepted(&jumped_to).is_empty(),
            "the body after the jump is reached from the target"
        );
    }

    #[test]
    fn a_jump_into_a_straight_line_run_is_not_a_thunk() {
        let l = listing(
            vec![
                jmp(0x1000, 0x1013),
                op(0x1010, 3),
                op(0x1013, 2),
                ret(0x1015),
            ],
            &[0x1000, 0x1010],
        );
        assert!(accepted(&l).is_empty());
    }

    /// A thunk to a thunk promotes both, and only from a thunk that is an entry.
    #[test]
    fn chained_thunks_promote_in_order() {
        let l = listing(
            vec![
                jmp(0x1000, 0x1020),
                jmp(0x1020, 0x1040),
                op(0x1040, 2),
                ret(0x1042),
            ],
            &[0x1000],
        );
        assert_eq!(accepted(&l), vec![0x1020, 0x1040]);

        let orphan = listing(
            vec![
                op(0x1000, 2),
                ret(0x1002),
                jmp(0x1010, 0x1040),
                op(0x1040, 2),
                ret(0x1042),
            ],
            &[0x1000],
        );
        assert!(
            accepted(&orphan).is_empty(),
            "a jump that is not a function entry is not a thunk"
        );
    }

    /// A chain of jumps is followed [`MAX_CHAIN_HOPS`] jumps deep and no further.
    #[test]
    fn a_long_jump_chain_is_cut_at_the_hop_bound() {
        let links: Vec<Insn> = (0..10u64)
            .map(|i| jmp(0x1000 + 0x10 * i, 0x1010 + 0x10 * i))
            .collect();
        let l = listing(
            links
                .into_iter()
                .chain([op(0x10a0, 2), ret(0x10a2)])
                .collect(),
            &[0x1000],
        );
        assert_eq!(accepted(&l), vec![0x1010, 0x1020, 0x1030, 0x1040]);
    }

    /// The thunk at 0x1020 is followed by the body the thunk at 0x1040 jumps to:
    /// that body is code, so the neighbour test passes only once it is promoted.
    #[test]
    fn the_neighbour_test_reads_the_growing_entry_set() {
        let l = listing(
            vec![
                op(0x1000, 2),
                ret(0x1002),
                jmp(0x1020, 0x1000),
                op(0x1025, 2),
                ret(0x1027),
                jmp(0x1040, 0x1025),
            ],
            &[0x1020, 0x1040],
        );
        assert_eq!(accepted(&l), vec![0x1000, 0x1025]);
    }
}
