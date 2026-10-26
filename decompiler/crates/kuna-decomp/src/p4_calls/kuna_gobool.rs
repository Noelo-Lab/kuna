//! **gobool** — narrow a Go boolean return's stale-high concatenation
//! (`docs/multilang.md` W4, the Go pack's return-ABI reading).
//!
//! ```text
//!   v2 = CONCAT71((undefined7)(v2 >> 8), a0[2] == '_');   -->  return a0[2] == '_';
//!   return v2 & 0xffffffff;
//! ```
//!
//! Go's ABI materializes a `bool` result as 0/1 in the LOW BYTE of the return
//! register and leaves the upper bytes unspecified: the compiler emits a bare
//! `SETcc AL` with no zeroing `XOR`, so the lifted dataflow is a
//! `PIECE(stale7, bool1)` feeding the `RETURN` — printed as the `CONCAT71`
//! artifact above. `SubvariableFlow::try_return_pull` rightly refuses to
//! narrow it (nothing proves the high bytes zero), and rustc never triggers
//! this because it zeroes first. The option asserts the ABI fact instead: a
//! RETURN whose value is a 7+1 concatenation whose low byte is a boolean
//! definition returns THAT BOOLEAN, so the value is rewritten to
//! `ZEXT(bool)` and the stale concatenation dies to ordinary dead code.
//!
//! The high bytes' content is deliberately overwritten (zeroed) — the
//! declared, documented intent of the option, mirroring `rustabi`'s reading
//! of the ScalarPair ABI: at `auto` (the default) the rule acts only when the
//! image's source language is Go, where those bytes are unspecified by the
//! ABI; `always` drops the language test (no-signal corpora, the stage test);
//! `off` keeps the faithful concatenation.

use kuna_num::opcodes::OpCode;

use kuna_base::types::int4;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::action::{ActionGroupList, Rule};

/// The three-valued `gobool` gate (the `gomorestack` pattern).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GoBoolMode {
    /// Keep the faithful concatenation.
    Off,
    /// Narrow when the image's source language is Go (the shipped default).
    #[default]
    Auto,
    /// Narrow whatever the detected language.
    Always,
}

impl GoBoolMode {
    /// The catalog `current` token.
    pub fn as_str(self) -> &'static str {
        match self {
            GoBoolMode::Off => "off",
            GoBoolMode::Auto => "auto",
            GoBoolMode::Always => "always",
        }
    }

    /// Stored form for the `Architecture` field.
    pub fn as_u8(self) -> u8 {
        match self {
            GoBoolMode::Off => 0,
            GoBoolMode::Auto => 1,
            GoBoolMode::Always => 2,
        }
    }

    /// Inverse of [`GoBoolMode::as_u8`] (unknown values read as `Off`).
    pub fn from_u8(v: u8) -> GoBoolMode {
        match v {
            1 => GoBoolMode::Auto,
            2 => GoBoolMode::Always,
            _ => GoBoolMode::Off,
        }
    }
}

/// Parse an `option gobool` value.
pub fn parse_gobool_mode(value: &str) -> Result<(GoBoolMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => GoBoolMode::Off,
        "auto" | "" => GoBoolMode::Auto,
        "always" | "on" | "1" | "true" => GoBoolMode::Always,
        other => {
            return Err(format!("unknown gobool value {other:?} (expected off, auto or always)"))
        }
    };
    Ok((mode, format!("Go boolean-return narrowing set to {}", mode.as_str())))
}

/// Is `code` an opcode whose output is a genuine boolean (0/1) byte?
fn produces_boolean(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN
    )
}

/// (kuna) `RuleGoBoolReturn` — rewrite `RETURN(PIECE(stale7, bool1))` into
/// `RETURN(ZEXT(bool1))` (option `gobool`, default auto).
pub struct RuleGoBoolReturn {
    /// Unit-test OR-override of the mode gate (the scheduled instance
    /// registers `false` and reads the live mode off the ArchContext).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleGoBoolReturn {
    /// Construct in group `g`; `enabled` forces the rule on (unit tests).
    pub fn with_group(enabled: bool, g: impl Into<String>) -> RuleGoBoolReturn {
        RuleGoBoolReturn { enabled, group: g.into() }
    }
}

/// Is the narrowing live for this function?
pub fn live(data: &Funcdata) -> bool {
    match GoBoolMode::from_u8(data.get_arch().go_bool) {
        GoBoolMode::Off => false,
        GoBoolMode::Auto => data.get_arch().source_lang.is_go(),
        GoBoolMode::Always => true,
    }
}

impl Rule for RuleGoBoolReturn {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_RETURN]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleGoBoolReturn { enabled: self.enabled, group: self.group.clone() }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !live(data) {
            return 0;
        }
        // RETURN carries the value in slot 1 (slot 0 is the return address).
        let Some(o) = data.obank().get(op) else { return 0 };
        if o.num_input() < 2 {
            return 0;
        }
        let Some(ret_vn) = o.get_in(1) else { return 0 };
        let Some(rv) = data.vbank().get(ret_vn) else { return 0 };
        if rv.get_size() != 8 || !rv.is_written() {
            return 0;
        }
        let Some(piece) = rv.get_def() else { return 0 };
        let Some(p) = data.obank().get(piece) else { return 0 };
        if p.code() != OpCode::CPUI_PIECE {
            return 0;
        }
        // PIECE(most-significant, least-significant): a 7-byte stale high half
        // over a 1-byte boolean low half.
        let (Some(hi), Some(lo)) = (p.get_in(0), p.get_in(1)) else { return 0 };
        if data.vbank().get(hi).map(|v| v.get_size()) != Some(7)
            || data.vbank().get(lo).map(|v| v.get_size()) != Some(1)
        {
            return 0;
        }
        let Some(lv) = data.vbank().get(lo) else { return 0 };
        if !lv.is_written() {
            return 0;
        }
        let Some(ld) = lv.get_def() else { return 0 };
        let Some(ldo) = data.obank().get(ld) else { return 0 };
        if !produces_boolean(ldo.code()) {
            return 0;
        }
        // The rewrite: RETURN reads ZEXT(bool); the PIECE loses its reader and
        // dies to ordinary dead code, taking the stale high bytes with it.
        let at = data.obank().get(op).expect("return op").get_addr().clone();
        let zext = data.new_op(1, at);
        data.op_set_opcode_code(zext, OpCode::CPUI_INT_ZEXT);
        if data.op_set_input(zext, lo, 0).is_err() {
            return 0;
        }
        let Ok(out) = data.new_unique_out(8, zext) else { return 0 };
        data.op_insert_before(zext, op);
        if data.op_set_input(op, out, 1).is_err() {
            return 0;
        }
        1
    }
}

#[cfg(test)]
#[path = "kuna_gobool/tests.rs"]
mod tests;
