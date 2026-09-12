//! Executable-pointer callback roots passed on the x86 stack.
//!
//! An address materialized by reachable code is not by itself a function: it can
//! be an integer, a label, or data embedded in an executable section. This pass
//! admits only the stack-argument shape: a `PUSH <executable address>` reaches the
//! next call through one straight-line run. The target must also be undefined in
//! the current Listing and pass the bounded strict-subroutine validator before it
//! is re-seeded by the caller.

use super::Listing;

const MAX_ARGUMENT_INSNS: usize = 16;
/// Maximum unique `PUSH imm` targets retained by one Listing walk. This is
/// intentionally larger than the total root-commit budget: invalid low-address
/// candidates should not readily starve a later valid callback, while the side
/// model remains bounded to a few thousand `(target, source)` pairs.
pub(crate) const MAX_CALLBACK_EVIDENCE: usize = 4096;

fn reaches_next_call(listing: &Listing, source: u64) -> bool {
    let Some(source_insn) = listing.instruction_at(source) else {
        return false;
    };
    let Some(mut next) = source_insn.fall_through else {
        return false;
    };
    for _ in 0..MAX_ARGUMENT_INSNS {
        let Some(insn) = listing.instruction_at(next) else {
            return false;
        };
        if insn.flow.is_call {
            return true;
        }
        if insn.flow.is_jump || insn.flow.is_terminal {
            return false;
        }
        let Some(fall) = insn.fall_through else {
            return false;
        };
        if fall != insn.addr.wrapping_add(u64::from(insn.len)) {
            return false;
        }
        next = fall;
    }
    false
}

pub(crate) fn stack_callback_candidates(listing: &Listing) -> Vec<u64> {
    let mut candidates = Vec::new();
    for &(source, target) in listing.stack_callback_refs() {
        if !reaches_next_call(listing, source) {
            continue;
        }
        if listing.is_undefined(target) {
            candidates.push(target);
        }
    }
    candidates.sort_unstable();
    candidates.dedup();
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::listing::{DiscoveredFunction, FlowKind, FlowType, Insn};

    fn insn(addr: u64, len: u32, mnemonic: &str, call: bool) -> Insn {
        Insn {
            addr,
            len,
            fall_through: Some(addr + u64::from(len)),
            flow: FlowType {
                kind: if call {
                    FlowKind::ComputedCall
                } else {
                    FlowKind::Fallthrough
                },
                is_call: call,
                has_fallthrough: true,
                ..FlowType::default()
            },
            flows: Vec::new(),
            mnemonic: mnemonic.into(),
            operands: String::new(),
            pcode: None,
        }
    }

    fn listing(target: u64, target_insn: Option<Insn>) -> Listing {
        let mut insns = vec![
            insn(0x2000, 5, "PUSH", false),
            insn(0x2005, 6, "CALL", true),
        ];
        if let Some(target_insn) = target_insn {
            insns.push(target_insn);
        }
        Listing::from_model_for_test(
            insns,
            vec![DiscoveredFunction {
                entry: 0x2000,
                name: None,
                from_symbol: false,
                has_no_return: false,
                call_fixup: None,
            }],
            vec![(0x2000, target)],
            vec![(0x1000, 0x3000)],
        )
    }

    #[test]
    fn pushed_executable_pointer_reaching_a_call_is_a_candidate() {
        assert_eq!(
            stack_callback_candidates(&listing(0x1000, None)),
            vec![0x1000]
        );
    }

    #[test]
    fn instruction_interior_is_not_a_candidate() {
        let covered = insn(0x1000, 8, "MOV", false);
        assert!(stack_callback_candidates(&listing(0x1004, Some(covered))).is_empty());
    }

    #[test]
    fn pushed_pointer_without_a_following_call_is_not_a_candidate() {
        let mut model = listing(0x1000, None);
        model.insns.remove(&0x2005);
        assert!(stack_callback_candidates(&model).is_empty());
    }
}
