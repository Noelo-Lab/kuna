//! (kuna `armshiftguard`) When a register-controlled ARM LSL prints its range
//! guard.
//!
//! ARM yields zero for a register LSL count of 32 through 255 and C leaves a
//! shift by the operand width or more undefined, so
//! [`crate::p9_emit::kuna_armregistershift`] prints `n < 32u ? x << n : 0`.
//! The guard is dead when the count's non-zero mask already keeps it below
//! the width (a `& 0x1f` mask, a narrow field, a remainder); this module owns
//! that proof and the per-run choice of when to print the guard at all.
//! [`crate::p9_emit::kuna_armshiftwide`] finds the counts of a lowered
//! double-word shift, the ones `wide` still guards.

use crate::architecture::Architecture;
use crate::context::VarnodeId;
use crate::funcdata::Funcdata;

/// (kuna `armshiftguard`) Which register-controlled ARM LSLs print the guard.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ShiftGuardMode {
    /// Every repeatable register LSL is guarded.
    Always,
    /// Only a count not proven below the operand width is guarded.
    Needed,
    /// Only an unproven count of a lowered double-word shift is guarded; any
    /// other count prints the plain C shift.
    #[default]
    Wide,
    /// No guard: the plain C shift, undefined where the count reaches the width.
    Never,
}

impl ShiftGuardMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ShiftGuardMode::Always => "always",
            ShiftGuardMode::Needed => "needed",
            ShiftGuardMode::Wide => "wide",
            ShiftGuardMode::Never => "never",
        }
    }
}

/// The `armshiftguard` option parser.
pub struct OptionArmShiftGuard;

impl OptionArmShiftGuard {
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(ShiftGuardMode, String)> {
        let mode = match p1 {
            "always" => ShiftGuardMode::Always,
            "needed" => ShiftGuardMode::Needed,
            "wide" | "" => ShiftGuardMode::Wide,
            "never" => ShiftGuardMode::Never,
            other => {
                return Err(kuna_base::error::KunaError::parse(format!(
                    "Unknown armshiftguard value: {other} (expected always|needed|wide|never)"
                )))
            }
        };
        Ok((mode, format!("ARM register-shift guard set to {}", mode.as_str())))
    }
}

/// Whether `count` is proven below `width`, a power of two: every bit its
/// value can set lies under the width.
pub(crate) fn count_below_width(fd: &Funcdata, count: VarnodeId, width: u64) -> bool {
    crate::kuna_boolbyte::value_mask(fd, count) < width
}

/// Whether a 32-bit register LSL by `count` prints the range guard.
pub(crate) fn guard_needed(fd: &Funcdata, arch: &Architecture, count: VarnodeId) -> bool {
    match arch.arm_shift_guard {
        ShiftGuardMode::Always => true,
        ShiftGuardMode::Needed => !count_below_width(fd, count, 32),
        ShiftGuardMode::Wide => {
            !count_below_width(fd, count, 32)
                && crate::p9_emit::kuna_armshiftwide::double_word_count(fd, count)
        }
        ShiftGuardMode::Never => false,
    }
}
