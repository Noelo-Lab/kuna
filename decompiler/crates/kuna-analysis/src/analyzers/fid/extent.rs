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
//! The definition is a **linear decode** from the entry to its end bound: every
//! instruction in the function's address range, including blocks after a `ret`.
//! Two other readings were measured wrong on real binaries:
//!
//! - the address clip `[entry, next_entry)` absorbed alignment padding and
//!   interleaved code — a 347-code-unit extent for a 29-code-unit function, so
//!   every lookup missed;
//! - a control-flow walk stopping at the first `ret` hashed 9 code units of a
//!   function whose body is 16, so a database built from an object stopped
//!   matching its own linked program.
//!
//! # Origin (upstream Ghidra)
//!
//! - `Ghidra/Features/FunctionID/src/main/java/ghidra/feature/fid/hash/FunctionBodyFunctionExtentGenerator.java`
//! - `.../hash/FunctionExtentGenerator.java` — the strategy interface.

use std::rc::Rc;

use kuna_sleigh::translate::Translate;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;

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

/// The extent definition **both** sides of a FID match use: the builder hashing a
/// library function from its symbol and the lookup hashing the same function in a
/// stripped image.
///
/// It is a **linear decode** from `entry` to `end` (exclusive), which is what
/// Ghidra's `FunctionBodyFunctionExtentGenerator` hashes: every instruction in the
/// function's address range, including the blocks after a `ret` that a
/// flow-following walk would skip. Two earlier attempts were each wrong in a way
/// the tests caught:
///
/// - the address clip `[entry, next_entry)` absorbed alignment padding, giving a
///   347-code-unit extent for a 29-code-unit function, so every lookup missed;
/// - a control-flow walk that stopped at the first `ret` hashed 9 code units of a
///   function whose body is 16, so a database built from the object stopped
///   matching its own linked program.
///
/// Decoding stops at an undecodable byte, at `end`, or when an instruction would
/// cross `end`.
/// True when `target` cannot belong to the body entered at `entry`: it lies at or
/// past the end bound (another function's entry), or before `entry` itself. Such a
/// branch is a tail call, not intra-function flow.
fn is_other_entry(entry: u64, target: u64, end: Option<u64>) -> bool {
    target < entry || end.is_some_and(|n| target >= n)
}

pub fn decode_extent(
    translate: &dyn Translate,
    entry: u64,
    end: Option<u64>,
    code_space: &Rc<kuna_base::space::AddrSpace>,
) -> Vec<ExtentInsn> {
    // Flow-reachable decode, which is what `func.getBody()` is: start at the
    // entry, follow each instruction's fall-through and its in-function branch
    // targets, and stop at a terminal, an undecodable byte, or `end`. Reaching a
    // block after a `ret` (a second exit selected by an earlier conditional
    // jump) keeps it in the body; unreachable padding after the last terminal
    // does not.
    let mut seen: std::collections::BTreeMap<u64, ExtentInsn> = std::collections::BTreeMap::new();
    let mut pending: Vec<u64> = vec![entry];
    let mut visited: std::collections::BTreeSet<u64> = std::collections::BTreeSet::new();
    let mut guard = 1u32 << 20;
    while let Some(vma) = pending.pop() {
        if !visited.insert(vma) {
            continue;
        }
        if end.is_some_and(|n| vma >= n) {
            continue;
        }
        if guard == 0 {
            break;
        }
        guard -= 1;
        let Ok(dec) = decode_one(translate, vma, code_space, false, false) else {
            continue; // an undecodable byte contributes nothing
        };
        let len = dec.len.max(1); // never advance by 0
        if end.is_some_and(|n| vma.checked_add(u64::from(len)).is_none_or(|e| e > n)) {
            continue;
        }
        let cls = classify(&dec.ops, vma, dec.len);
        seen.insert(vma, ExtentInsn {
            addr: vma,
            len,
            is_call: cls.flow.is_call,
            is_terminal: cls.flow.is_terminal,
        });
        if !cls.flow.is_terminal {
            if let Some(fall) = cls.fall_through {
                pending.push(fall);
            }
        }
        // Follow an in-function branch target. An unconditional `JMP` is NOT
        // treated as external: a forward `JMP` to a shared epilogue or a jump into
        // a loop header is ordinary intra-function flow, and skipping it truncated
        // real bodies. A tail call is instead recognised by its DESTINATION — the
        // target is another entry, which is exactly what Ghidra records as that
        // other function's body — using the same `entries` set the end bound comes
        // from.
        if !cls.flow.is_call {
            for target in &cls.flows {
                if !is_other_entry(entry, *target, end) {
                    pending.push(*target);
                }
            }
        }
    }
    seen.into_values().collect()
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
