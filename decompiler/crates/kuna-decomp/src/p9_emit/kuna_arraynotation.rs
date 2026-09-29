//! Array-notation policy for standalone pointer arithmetic.
//!
//! `arraynotation on` renders a standalone `PTRADD` as `&base[index]`;
//! `off` retains upstream `base + index`. Kuna enables it by default.
//! This module parses the option; the dispatcher applies its result through
//! [`crate::printc::PrintCOptions::set_array_notation`].

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::options::on_or_off;

/// Marshaling element `<arraynotation>` (kuna, C++
/// `ELEM_ARRAYNOTATION = ElementId("arraynotation",4001)`; kuna 4000+ range).
pub const ELEM_ARRAYNOTATION: ElementId = ElementId::new("arraynotation", 4001);

/// (kuna) Toggle array-notation rendering of standalone pointer arithmetic:
/// `arraynotation on|off` (C++ `OptionArrayNotation`, kuna_arraynotation.cc:15-25).
///
/// "off" keeps upstream behavior (`base + index`); "on" renders a standalone
/// `PTRADD` as `&base[index]` ([`crate::printc::PrintC`]'s `opPtradd`).  Note the
/// kuna Rust port ships this **default-on** (DIV-2; `PrintCOptions::new` sets
/// `array_notation = true`).
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionArrayNotation;

impl OptionArrayNotation {
    /// The option name (C++ `name = "arraynotation"`).
    pub const NAME: &'static str = "arraynotation";

    /// Validate the toggle and return its value and confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Array notation for pointer arithmetic turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_arraynotation/tests.rs"]
mod tests;
