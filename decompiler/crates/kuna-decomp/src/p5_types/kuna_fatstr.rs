//! (kuna `fatstr`) Length-carried string literals — the `(ptr, len)` constant
//! pair a fat-pointer language materializes (`docs/multilang.md` W3/W4).
//!
//! ```text
//!   takes_str((char *)&DAT_00102000, 15)  -->  takes_str("hello multilang", 15)
//! ```
//!
//! Rust (`&str`, `&[u8]`) and Go (`string`) carry a string's length beside its
//! pointer and never NUL-terminate the bytes, so the classic literal path —
//! which reads the image until a terminator — refuses them, and every string
//! argument renders as a raw rodata address beside a magic length. The length
//! is RIGHT THERE in the adjacent argument; this rule reads exactly that many
//! bytes and routes them through the existing bounded literal mechanism
//! ([`Funcdata::get_internal_string`] → the `BUILTIN_STRINGDATA` user-op,
//! `constseq`'s), which needs no terminator.
//!
//! # Matching: adjacency, read-only bytes, and text evidence
//!
//! A CALL argument pair `(ptr, len)` is rewritten only when every one of
//! these holds:
//!
//! * both are p-code CONSTANTS, adjacent in the call's inputs — the shape a
//!   fat pointer's two words take after constant propagation, whichever two
//!   registers or stack slots carried them;
//! * `2 <= len <= 4096` — one byte is no evidence, and a page of "text" is a
//!   buffer;
//! * the pointed-at `len` bytes are **read-only** in the image
//!   (`query_global_properties`, the same test `msvcstrappend` applies to its
//!   literal), so the rendered literal is a fact of the binary;
//! * the bytes decode as UTF-8 **text**: no control characters beyond
//!   `\n`/`\t`/`\r`, at least one alphanumeric — or the pointer's facing type
//!   carries a text [`LangFacet`](crate::kuna_langfacet::LangFacet) (the W3
//!   side table), in which case valid UTF-8 alone suffices, the facet being a
//!   type provider's statement rather than a guess.
//!
//! The rewrite replaces the pointer argument with the `BUILTIN_STRINGDATA`
//! value (typed `char *`); the length argument stays, because the call really
//! does pass it.
//!
//! # The option
//!
//! `option fatstr off|auto|always`, default **auto** — act only when the
//! image's source language is Rust or Go (the `sourcelang` fact), a
//! structural no-op elsewhere; `always` drops the language test (no-signal
//! corpora such as a stage-test bytechunk); `off` keeps the raw pointer.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::context::OpId;
use crate::dtype::TypeFactory;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

/// The three-valued `fatstr` gate, stored on the `Architecture` as a `u8`
/// (the `rustabi` pattern).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FatStrMode {
    /// Keep the raw pointer/length pair.
    Off,
    /// Rewrite when the image's source language is Rust or Go (the shipped
    /// default).
    #[default]
    Auto,
    /// Rewrite whatever the detected language (no-signal corpora).
    Always,
}

impl FatStrMode {
    /// The catalog `current` token.
    pub fn as_str(self) -> &'static str {
        match self {
            FatStrMode::Off => "off",
            FatStrMode::Auto => "auto",
            FatStrMode::Always => "always",
        }
    }

    /// Stored form for the `Architecture` field.
    pub fn as_u8(self) -> u8 {
        match self {
            FatStrMode::Off => 0,
            FatStrMode::Auto => 1,
            FatStrMode::Always => 2,
        }
    }

    /// Inverse of [`FatStrMode::as_u8`] (unknown values read as `Off`).
    pub fn from_u8(v: u8) -> FatStrMode {
        match v {
            1 => FatStrMode::Auto,
            2 => FatStrMode::Always,
            _ => FatStrMode::Off,
        }
    }
}

/// Parse an `option fatstr` value.
pub fn parse_fatstr_mode(value: &str) -> Result<(FatStrMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => FatStrMode::Off,
        "auto" | "" => FatStrMode::Auto,
        "always" | "on" | "1" | "true" => FatStrMode::Always,
        other => {
            return Err(format!("unknown fatstr value {other:?} (expected off, auto or always)"))
        }
    };
    Ok((mode, format!("Length-carried string literals set to {}", mode.as_str())))
}

/// Is the rewrite live for this function?
pub fn live(data: &Funcdata) -> bool {
    match FatStrMode::from_u8(data.get_arch().fat_str) {
        FatStrMode::Off => false,
        FatStrMode::Auto => {
            let lang = data.get_arch().source_lang;
            lang.is_rust() || lang.is_go()
        }
        FatStrMode::Always => true,
    }
}

/// The length bounds a pair must satisfy: one byte is no evidence, a page of
/// "text" is a buffer.
pub const MIN_LEN: u64 = 2;
/// See [`MIN_LEN`].
pub const MAX_LEN: u64 = 4096;

/// Is `bytes` string-literal text?  UTF-8, no control characters beyond
/// `\n`/`\t`/`\r`, and at least one alphanumeric so pad bytes never read as a
/// string.
pub fn bytes_are_text(bytes: &[u8]) -> bool {
    let Ok(s) = core::str::from_utf8(bytes) else { return false };
    let mut has_alnum = false;
    for c in s.chars() {
        if c.is_control() && !matches!(c, '\n' | '\t' | '\r') {
            return false;
        }
        has_alnum |= c.is_alphanumeric();
    }
    has_alnum
}

/// (kuna) `RuleFatStr` — rewrite a read-only `(ptr, len)` constant argument
/// pair into a bounded string literal (option `fatstr`, default auto).
pub struct RuleFatStr {
    /// Unit-test OR-override of the mode gate (the scheduled instance passes
    /// `false` and reads the live mode off the ArchContext).
    enabled: bool,
    group: String,
}

impl RuleFatStr {
    /// `enabled` forces the rule on regardless of the architecture gate (unit
    /// tests); the scheduled instance passes `false`.
    pub fn with_group(enabled: bool, g: impl Into<String>) -> RuleFatStr {
        RuleFatStr { enabled, group: g.into() }
    }
}

impl Rule for RuleFatStr {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CALL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleFatStr { enabled: self.enabled, group: self.group.clone() }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !live(data) {
            return 0;
        }
        let Some(o) = data.obank().get(op) else { return 0 };
        let nin = o.num_input();
        if nin < 3 {
            return 0;
        }
        let call_addr = o.get_addr().clone();
        // The default data space: where a constant that is really an address
        // points.
        let Some(spc) = data.get_arch().manage().get_default_data_space().cloned() else {
            return 0;
        };
        let ptr_size = spc.get_addr_size() as int4;
        for i in 1..nin - 1 {
            let Some((ptr_val, len_val)) = constant_pair(data, op, i, ptr_size) else { continue };
            if !(MIN_LEN..=MAX_LEN).contains(&len_val) {
                continue;
            }
            let addr = Address::new(Rc::clone(&spc), ptr_val);
            // The literal must be a fact of the image: read-only, and
            // readable.
            if data.get_arch().query_global_properties(&addr, len_val as int4, &call_addr)
                & varnode_flags::readonly
                == 0
            {
                continue;
            }
            let mut buf = vec![0u8; len_val as usize];
            if data.get_arch().loader_fill(&mut buf, &addr).is_err() {
                continue;
            }
            // Text evidence, or a type provider's facet statement.
            if !bytes_are_text(&buf) && !pointer_has_text_facet(data, op, i) {
                continue;
            }
            let Some(tf) = data.get_arch().types() else { return 0 };
            let Ok(ch) = tf.get_type_char(1) else { return 0 };
            let Ok(char_ptr) = tf.get_type_pointer(ptr_size, ch, spc.get_word_size()) else {
                return 0;
            };
            let Some(res) = data.get_internal_string(&buf, len_val as int4, char_ptr, op) else {
                continue;
            };
            data.op_set_input(op, res, i).expect("RuleFatStr: setInput");
            return 1;
        }
        0
    }
}

/// The adjacent-constant pair at input slots `(i, i+1)`: the pointer-sized
/// constant's value and the length constant's value, or `None`.
fn constant_pair(data: &Funcdata, op: OpId, i: int4, ptr_size: int4) -> Option<(u64, u64)> {
    let o = data.obank().get(op)?;
    let ptr_vn = data.vbank().get(o.get_in(i)?)?;
    if !ptr_vn.is_constant() || ptr_vn.get_size() != ptr_size {
        return None;
    }
    let len_vn = data.vbank().get(o.get_in(i + 1)?)?;
    if !len_vn.is_constant() {
        return None;
    }
    Some((ptr_vn.get_offset(), len_vn.get_offset()))
}

/// Does the pointer argument's facing type carry a text facet (W3)?
fn pointer_has_text_facet(data: &Funcdata, op: OpId, i: int4) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    let Some(vn) = o.get_in(i).and_then(|v| data.vbank().get(v)) else { return false };
    let dt = vn.get_type_read_facing(op);
    let name = dt.get_name();
    if name.is_empty() {
        return false;
    }
    let Some(tf) = data.get_arch().types() else { return false };
    tf.kuna_lang_facet(name).is_some_and(|f| f.is_text())
}

/// The per-root rule registrations (the `rodatastringcopy` pattern).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        ctor: || Box::new(RuleFatStr::with_group(true, "analysis")),
    }]
}

#[cfg(test)]
#[path = "kuna_fatstr/tests.rs"]
mod tests;
