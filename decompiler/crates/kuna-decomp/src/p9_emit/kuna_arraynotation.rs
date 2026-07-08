//! Port of `decompiler/cpp/kuna_arraynotation.{cc,hh}` — the pointer-notation
//! (array-indexing) presentation sub-stage.
//!
//! This is a kuna-owned policy file (NOT upstream Ghidra).  It exposes the second
//! GH-558 decision: how a standalone `PTRADD` is rendered.  Upstream
//! `PrintC::opPtradd` uses array notation only inside a load/store context and
//! falls back to `base + index` otherwise — a latent S9 "literal/format policy"
//! decision.  kuna parameterizes it: `option arraynotation on` renders
//! `&base[index]` instead.
//!
//! The toggle itself ([`crate::printc::PrintCOptions::array_notation`], default-on
//! per the DIV-2 / GH-558 reset) and the `opPtradd` consumer live in
//! [`crate::printc`].  This module owns only the option struct that flips it
//! (C++ `OptionArrayNotation`).
//!
//! # Apply seam
//!
//! C++ `OptionArrayNotation::apply` does `dynamic_cast<PrintC *>(glb->print)`,
//! erroring `"Can only set array notation for C language"` when the active
//! printer is not C, then calls `lng->setArrayNotation(val)`.  Following the
//! kuna-option idiom (`OptionLowerSwitch::apply`), this `apply` parses + validates
//! the `on`/`off` value and returns the resolved flag plus the confirmation
//! message; the caller performs the `PrintC` downcast (and the not-C error) and
//! calls [`crate::printc::PrintCOptions::set_array_notation`].

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

    /// C++ `OptionArrayNotation::apply`:
    ///
    /// ```text
    ///   bool val = onOrOff(p1);
    ///   PrintC *lng = dynamic_cast<PrintC *>(glb->print);
    ///   if (lng == 0) return "Can only set array notation for C language";
    ///   lng->setArrayNotation(val);
    ///   string prop = val ? "on" : "off";
    ///   return "Array notation for pointer arithmetic turned "+prop;
    /// ```
    ///
    /// Returns the resolved flag plus the confirmation message.  The caller does
    /// the `dynamic_cast<PrintC *>` (and the `"Can only set array notation for C
    /// language"` error if the active printer is not C) and calls
    /// `lng->setArrayNotation(val)` (STUB: W4/W9 option dispatch).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        // bool val = onOrOff(p1);
        let val = on_or_off(p1)?;
        // lng->setArrayNotation(val);  -- left to the caller (STUB(W4/W9)).
        // string prop = val ? "on" : "off";
        let prop = if val { "on" } else { "off" };
        // return "Array notation for pointer arithmetic turned "+prop;
        Ok((val, format!("Array notation for pointer arithmetic turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_arraynotation/tests.rs"]
mod tests;
