//! (kuna) The source map of a rendered function: every non-blank token of the
//! C text with its line, UTF-16 column, kind, colour and what it stands for
//! (the instruction behind it, the callee a call names, the reported variable).
//!
//! The tokens come from the markup pass ([`crate::printc::PrintC::doc_function_tokens`]),
//! which receives the same emit sequence as the plain render and replays the
//! plain emitter's column model beside the packed encoding. [`resolve`] places
//! them in the trimmed `code` and binds their references; [`verify`] proves the
//! placement against the text, so a caller ships tokens only when they rebuild
//! the code exactly.

use std::collections::BTreeMap;

use kuna_num::opcodes::OpCode;

use crate::decompile_drive::{CodeProvenance, VarInfo};
use crate::funcdata::Funcdata;
use crate::prettyprint::{EmitToken, SyntaxHighlight, TokenKind};

/// One token of the `code` a surface reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeToken {
    /// 1-based line in the trimmed `code`.
    pub line: usize,
    /// UTF-16 column of the first code unit.
    pub col: usize,
    /// Length in UTF-16 code units.
    pub len: usize,
    pub kind: TokenKind,
    pub color: SyntaxHighlight,
    pub text: String,
    /// Byte address of the instruction the token stands for (a comment's or
    /// label's own address).
    pub address: Option<u64>,
    /// Entry address of the function a call's name names.
    pub callee: Option<u64>,
    /// Index into the reported variables.
    pub var: Option<usize>,
    /// `local`, `param`, `return` or `function` inside a declaration.
    pub decl: Option<&'static str>,
    pub type_text: Option<String>,
}

/// One `v = K; checker(); return v;` rewrite the presentation pass made, in
/// the emitter's line numbering: the assignment line is blanked and the return
/// line becomes `return K;` at the assignment's indent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieRewrite {
    pub assignment_line: usize,
    pub return_line: usize,
    pub indent: usize,
    pub literal: char,
}

/// Apply the text rewrites to the tokens of the same render.
pub fn apply_cookie_rewrites(tokens: &mut Vec<EmitToken>, rewrites: &[CookieRewrite]) {
    for rw in rewrites {
        let literal = rw.literal.to_string();
        let value = tokens
            .iter()
            .find(|t| t.line == rw.assignment_line && t.text == literal)
            .cloned();
        let keyword = tokens.iter().find(|t| t.line == rw.return_line && t.text == "return").cloned();
        let semi = tokens.iter().rev().find(|t| t.line == rw.return_line && t.text == ";").cloned();
        tokens.retain(|t| t.line != rw.assignment_line && t.line != rw.return_line);
        let (Some(mut keyword), Some(mut semi)) = (keyword, semi) else { continue };
        let mut value = value.unwrap_or_else(|| EmitToken {
            text: literal.clone(),
            kind: TokenKind::Value,
            color: SyntaxHighlight::ConstColor,
            opref: None,
            varref: None,
            at: None,
            type_name: None,
            ..keyword.clone()
        });
        keyword.col = rw.indent;
        value.line = rw.return_line;
        value.col = rw.indent + 7;
        semi.col = rw.indent + 8;
        let at = tokens.iter().position(|t| t.line > rw.return_line).unwrap_or(tokens.len());
        tokens.splice(at..at, [keyword, value, semi]);
    }
}

/// Place `raw` in `untrimmed.trim_matches('\n')` and bind each token's
/// references against `fd`: `opref` to its instruction, a call's name to its
/// callee, a variable to its row in `variables` (by the varrefs its evidence
/// came from, else by a unique name).
pub fn resolve(
    fd: &Funcdata,
    provenance: &CodeProvenance,
    raw: &[EmitToken],
    untrimmed: &str,
    variables: &[VarInfo],
) -> Vec<CodeToken> {
    let mut op_addresses: BTreeMap<u64, u64> = BTreeMap::new();
    for (_, op_id) in fd.obank().iter_all() {
        let Some(op) = fd.obank().get(op_id) else { continue };
        if op.get_addr().get_space().is_some() {
            op_addresses.insert(op.get_time() as u64, op.get_addr().get_offset());
        }
    }
    let mut callees: BTreeMap<u64, u64> = BTreeMap::new();
    for i in 0..fd.num_calls() {
        let spec = fd.get_call_specs(i);
        let Some(op) = fd.obank().get(spec.get_op()) else { continue };
        if op.code() != OpCode::CPUI_CALL {
            continue;
        }
        let entry = spec.get_entry_address();
        if entry.get_space().is_some() {
            callees.insert(op.get_time() as u64, entry.get_offset());
        }
    }
    let mut by_ref: BTreeMap<u64, usize> = BTreeMap::new();
    for (i, v) in variables.iter().enumerate() {
        for r in provenance.variable_varrefs(fd, v) {
            by_ref.entry(r).or_insert(i);
        }
    }
    let unique_name = |text: &str| {
        let mut hits = variables.iter().enumerate().filter(|(_, v)| v.name == text);
        match (hits.next(), hits.next()) {
            (Some((i, _)), None) => Some(i),
            _ => None,
        }
    };
    let skip = leading_breaks(untrimmed);
    raw.iter()
        .filter_map(|t| {
            let line = code_line(t.line, skip)?;
            let var = match t.kind {
                TokenKind::Variable => t
                    .varref
                    .and_then(|r| by_ref.get(&r).copied())
                    .filter(|&i| variables[i].name == t.text)
                    .or_else(|| {
                        matches!(t.color, SyntaxHighlight::VarColor | SyntaxHighlight::ParamColor)
                            .then(|| unique_name(&t.text))
                            .flatten()
                    }),
                _ => None,
            };
            let address = match t.kind {
                TokenKind::Comment | TokenKind::Label => t.at,
                _ => t.opref.and_then(|o| op_addresses.get(&o).copied()),
            };
            let callee = match t.kind {
                TokenKind::FuncName => t.opref.and_then(|o| callees.get(&o).copied()),
                _ => None,
            };
            let decl = if t.in_return_type {
                Some("return")
            } else if t.in_proto {
                match t.kind {
                    TokenKind::FuncName => Some("function"),
                    TokenKind::Variable | TokenKind::Type => Some("param"),
                    _ => None,
                }
            } else if t.in_var_decl && matches!(t.kind, TokenKind::Variable | TokenKind::Type) {
                Some("local")
            } else {
                None
            };
            let type_text = t
                .type_name
                .clone()
                .or_else(|| (t.kind == TokenKind::Type).then(|| t.text.clone()));
            Some(CodeToken {
                line,
                col: t.col,
                len: t.text.encode_utf16().count(),
                kind: t.kind,
                color: t.color,
                text: t.text.clone(),
                address,
                callee,
                var,
                decl,
                type_text,
            })
        })
        .collect()
}

/// How many line breaks `trim_matches('\n')` removes from the front of a render.
fn leading_breaks(untrimmed: &str) -> usize {
    untrimmed.len() - untrimmed.trim_start_matches('\n').len()
}

/// The 1-based `code` line of emitter line `line` after `skip` leading breaks
/// were trimmed, or `None` for a line the trim removed.
fn code_line(line: usize, skip: usize) -> Option<usize> {
    (line + 1).checked_sub(skip).filter(|&l| l >= 1)
}

/// Check that `tokens` rebuild `code`: in order, inside their line, each slice
/// equal to its text, and every code unit between them a space.
pub fn verify(tokens: &[CodeToken], code: &str) -> Result<(), String> {
    let lines: Vec<Vec<u16>> = code.split('\n').map(|l| l.encode_utf16().collect()).collect();
    let mut cursor: Vec<usize> = vec![0; lines.len()];
    let mut last = (0usize, 0usize);
    for t in tokens {
        if t.line == 0 || t.line > lines.len() {
            return Err(format!("token {:?} on line {} outside 1..={}", t.text, t.line, lines.len()));
        }
        if (t.line, t.col) < last {
            return Err(format!("token {:?} at {}:{} is out of order", t.text, t.line, t.col));
        }
        let units: Vec<u16> = t.text.encode_utf16().collect();
        if units.is_empty() || units.len() != t.len {
            return Err(format!("token {:?} at {}:{} has length {}", t.text, t.line, t.col, t.len));
        }
        let line = &lines[t.line - 1];
        let start = cursor[t.line - 1];
        if t.col < start {
            return Err(format!("token {:?} at {}:{} overlaps the previous one", t.text, t.line, t.col));
        }
        let end = t.col + t.len;
        if end > line.len() || line[t.col..end] != units[..] {
            let found = String::from_utf16_lossy(&line[t.col.min(line.len())..end.min(line.len())]);
            return Err(format!("token {:?} at {}:{} reads {found:?} in the code", t.text, t.line, t.col));
        }
        if let Some(gap) = line[start..t.col].iter().position(|&u| u != u16::from(b' ')) {
            return Err(format!("line {} col {} is not covered by a token", t.line, start + gap));
        }
        cursor[t.line - 1] = end;
        last = (t.line, end);
    }
    for (i, line) in lines.iter().enumerate() {
        if let Some(gap) = line[cursor[i]..].iter().position(|&u| u != u16::from(b' ')) {
            return Err(format!("line {} col {} is not covered by a token", i + 1, cursor[i] + gap));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
