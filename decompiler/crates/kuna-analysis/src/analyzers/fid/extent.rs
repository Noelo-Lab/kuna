//! The function-extent generator — the code units the FID hasher fingerprints.
//!
//! Ghidra hashes `func.getBody()`: the address set the FunctionManager recorded as
//! the function's body, iterated as instructions in ascending address order
//! (`FunctionBodyFunctionExtentGenerator`). kuna's program model has function
//! **entries**, not bodies, so this module defines the extent explicitly and both
//! sides of a match use it — the builder that fingerprints a library function and
//! the pass that fingerprints the same function in a stripped image. A single
//! instruction of disagreement makes the hashes differ and the lookup miss
//! silently, so the shared definition is the whole point of this module.
//!
//! Walk direct branches, fall-through and complete jump tables from the entry.
//! This keeps reachable code after an earlier return without absorbing padding
//! or interleaved code. The next function entry bounds the walk; an unresolved
//! computed jump makes the whole fingerprint unavailable.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/hash/FunctionBodyFunctionExtentGenerator.java`
//! - `.../hash/FunctionExtentGenerator.java` — the strategy interface.

use std::cell::OnceCell;
use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::listing::BoundedSwitches;

/// One decoded instruction in a function's extent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtentInsn {
    pub addr: u64,
    pub len: u32,
    pub is_call: bool,
    /// The instruction is a return / unconditional branch out (the builder sets
    /// [`FLAG_HAS_TERMINATOR`](crate::fid::db::FLAG_HAS_TERMINATOR) from it).
    pub is_terminal: bool,
}

const MAX_EXTENT_WORK: u32 = 1 << 20;

/// True when `target` cannot belong to the body entered at `entry`: it lies at or
/// past the end bound (another function's entry), or before `entry` itself. Such a
/// branch is a tail call, not intra-function flow.
fn is_other_entry(entry: u64, target: u64, end: Option<u64>) -> bool {
    target < entry || end.is_some_and(|n| target >= n)
}

/// Return the complete, address-ordered instruction body, or a reason it cannot
/// be fingerprinted. Callers can reuse the lazily loaded switch metadata.
pub(crate) fn decode_extent(
    arch: &Architecture,
    translate: &dyn Translate,
    entry: u64,
    end: Option<u64>,
    mapped_range: Option<(u64, u64)>,
    code_space: &Rc<AddrSpace>,
    switches: &OnceCell<Option<BoundedSwitches>>,
) -> Result<Vec<ExtentInsn>, &'static str> {
    let outside_image = |addr| mapped_range.is_some_and(|(lo, hi)| addr < lo || addr >= hi);
    let mut extent = Vec::new();
    let mut seen = HashSet::new();
    let mut pending = vec![entry];
    let mut dispatches = Vec::new();
    let mut branch_entries = Vec::new();
    let mut proven_chains = Vec::new();
    let mut deferred = Vec::new();
    let mut work = MAX_EXTENT_WORK;
    loop {
        while let Some(vma) = pending.pop() {
            // The raw loader pads a partial read at EOF with zeroes. Do not
            // accept those bytes as part of an instruction or invent a body
            // boundary when a live path would run out of the mapped image.
            if outside_image(vma) {
                return Err("extent_outside_image");
            }
            if is_other_entry(entry, vma, end) || !seen.insert(vma) {
                continue;
            }
            work = work.checked_sub(1).ok_or("extent_limit")?;
            let dec = decode_one(translate, vma, code_space, false, false)
                .map_err(|_| "extent_decode_failed")?;
            let len = dec.len;
            let insn_end = vma.checked_add(u64::from(len)).ok_or("extent_decode_failed")?;
            if mapped_range.is_some_and(|(_, hi)| insn_end > hi) {
                return Err("extent_outside_image");
            }
            if len == 0 || end.is_some_and(|n| insn_end > n) {
                return Err("extent_decode_failed");
            }
            let cls = classify(&dec.ops, vma, len);
            extent.push(ExtentInsn {
                addr: vma,
                len,
                is_call: cls.flow.is_call,
                is_terminal: cls.flow.is_terminal,
            });
            if !cls.flow.is_terminal {
                pending.extend(cls.fall_through);
            }
            branch_entries.extend(cls.flows.iter().copied());
            if !cls.flow.is_call {
                // A direct branch to another function is a tail transfer. For
                // computed switches below, unknown ownership is not enough to
                // omit a case and still claim a complete fingerprint.
                pending.extend(cls.flows);
            }
            if dec.ops.iter().any(|op| op.opcode == OpCode::CPUI_BRANCHIND) {
                dispatches.push(vma);
            }
        }
        if dispatches.is_empty() {
            break;
        }

        // The solver slices through already-decoded predecessors. Finish all
        // direct paths first, and retry a pending dispatch if another switch
        // opens up more of its predecessors.
        let tables = switches.get_or_init(|| BoundedSwitches::from_loader(arch, code_space))
            .as_ref().ok_or("unresolved_computed_jump")?;
        for dispatch in dispatches.drain(..) {
            work = work.checked_sub(1).ok_or("extent_limit")?;
            if let Some((start, targets)) = tables.prove_targets(arch, translate, code_space, &seen, dispatch) {
                if targets.iter().copied().any(outside_image) {
                    return Err("extent_outside_image");
                }
                if targets.iter().any(|&target| is_other_entry(entry, target, end)) {
                    return Err("unresolved_computed_jump");
                }
                proven_chains.push((start, dispatch));
                branch_entries.extend(targets.iter().copied());
                for target in targets.into_iter().filter(|target| !seen.contains(target)) {
                    if pending.len() >= work as usize {
                        return Err("extent_limit");
                    }
                    pending.push(target);
                }
            } else {
                deferred.push(dispatch);
            }
        }
        if pending.is_empty() && !deferred.is_empty() {
            return Err("unresolved_computed_jump");
        }
        std::mem::swap(&mut dispatches, &mut deferred);
    }
    // The compare and dispatch must form one single-entry chain. Case bodies
    // from a switch discovered later can introduce new incoming edges, so
    // validate only after every reachable direct and computed edge is known.
    if !proven_chains.is_empty() {
        branch_entries.push(entry);
        branch_entries.sort_unstable();
        if proven_chains.into_iter().any(|(start, dispatch)| {
            let after_start = branch_entries.partition_point(|&target| target <= start);
            branch_entries.get(after_start).is_some_and(|&target| target <= dispatch)
        }) {
            return Err("unresolved_computed_jump");
        }
    }
    extent.sort_unstable_by_key(|insn| insn.addr);
    if extent.windows(2).any(|pair| pair[0].addr + u64::from(pair[0].len) > pair[1].addr) {
        return Err("extent_decode_failed");
    }
    Ok(extent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extent_insn_carries_the_facts_the_hasher_and_flags_need() {
        let insn = ExtentInsn { addr: 0x1000, len: 5, is_call: true, is_terminal: false };
        assert_eq!(insn.addr, 0x1000);
        assert_eq!(insn.len, 5);
        assert!(insn.is_call);
        assert!(!insn.is_terminal);
    }
}
