//! Preserve case-label signedness while recovering a lowered switch.
//!
//! A lowered cascade's required range comparisons carry the controlling C
//! interpretation: `INT_SLESS`/`INT_SLESSEQUAL` are signed and
//! `INT_LESS`/`INT_LESSEQUAL` are unsigned. `INT_ZEXT` and `INT_SEXT` on the
//! canonical selector path independently prove unsigned and signed widening.
//! Equality-case bit patterns do not vote, but their widening paths must agree.
//! Recovery therefore accepts one consistent interpretation and declines mixed,
//! conflicting, or missing evidence. The option-off path bypasses these checks
//! and retains the historical case-bit behavior exactly.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::options::on_or_off;

/// Marshaling element `<loweredswitchlabels>` (kuna 4000+ range).
pub const ELEM_LOWEREDSWITCHLABELS: ElementId = ElementId::new("loweredswitchlabels", 4093);

/// Signedness proved by a lowered-switch range comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoweredSwitchSignedness {
    Signed,
    Unsigned,
}

impl LoweredSwitchSignedness {
    pub fn from_range_opcode(opcode: OpCode) -> Option<Self> {
        match opcode {
            OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => Some(Self::Signed),
            OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => Some(Self::Unsigned),
            _ => None,
        }
    }

    pub fn signed_labels(self) -> bool {
        self == Self::Signed
    }
}

/// Reconcile one range-comparison fact with the cascade's controlling fact.
pub fn reconcile_range_signedness(
    current: Option<LoweredSwitchSignedness>,
    next: Option<LoweredSwitchSignedness>,
) -> Option<LoweredSwitchSignedness> {
    let next = next?;
    match current {
        None => Some(next),
        Some(cur) if cur == next => Some(cur),
        Some(_) => None,
    }
}

/// The historical case-bit guess retained only for explicit option ablation.
pub fn legacy_signed_labels(var_size: int4, case_vals: &[uintb]) -> bool {
    let signbit = if var_size >= 8 {
        1u64 << 63
    } else {
        1u64 << (var_size * 8 - 1)
    };
    case_vals.iter().any(|&v| (v & signbit) != 0)
}

/// Toggle comparison-derived signedness for recovered lowered-switch labels.
pub struct OptionLowerSwitchLabels;

impl OptionLowerSwitchLabels {
    pub const NAME: &'static str = "loweredswitchlabels";

    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((
            val,
            format!("Lowered-switch comparison-derived label signedness turned {prop}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_opcodes_carry_signedness() {
        assert_eq!(
            LoweredSwitchSignedness::from_range_opcode(OpCode::CPUI_INT_SLESS),
            Some(LoweredSwitchSignedness::Signed)
        );
        assert_eq!(
            LoweredSwitchSignedness::from_range_opcode(OpCode::CPUI_INT_LESSEQUAL),
            Some(LoweredSwitchSignedness::Unsigned)
        );
        assert_eq!(
            LoweredSwitchSignedness::from_range_opcode(OpCode::CPUI_INT_EQUAL),
            None
        );
    }

    #[test]
    fn mixed_or_ambiguous_evidence_is_rejected() {
        let signed = reconcile_range_signedness(None, Some(LoweredSwitchSignedness::Signed));
        assert_eq!(signed, Some(LoweredSwitchSignedness::Signed));
        assert_eq!(
            reconcile_range_signedness(signed, Some(LoweredSwitchSignedness::Unsigned)),
            None
        );
        assert_eq!(reconcile_range_signedness(signed, None), None);
    }

    #[test]
    fn option_parses_on_and_off() {
        assert_eq!(OptionLowerSwitchLabels::NAME, "loweredswitchlabels");
        assert!(OptionLowerSwitchLabels.apply("on").unwrap().0);
        assert!(!OptionLowerSwitchLabels.apply("off").unwrap().0);
        assert!(OptionLowerSwitchLabels.apply("maybe").is_err());
    }
}
