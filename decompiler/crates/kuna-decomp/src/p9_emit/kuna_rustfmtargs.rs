//! Primitive Display arguments in the verified Rust 1.93 template layout.

use std::collections::BTreeMap;
use std::rc::Rc;

use super::kuna_rustfmtflags::{Formatter, Options, Trait};
use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

const MAX_BYTES: usize = 4096;
const MAX_ARGS: usize = 16;

pub struct Argument {
    pub pointer: VarnodeId,
    pub primitive: &'static str,
}

pub struct Plan {
    pub name: &'static str,
    pub literal: String,
    pub arguments: Vec<Argument>,
}

/// Full demangled identities, before display-name normalization. Conflicting
/// aliases invalidate an address even when one alias names a supported trait.
pub fn formatter_symbols(
    symbols: impl IntoIterator<Item = (u64, Option<String>)>,
) -> BTreeMap<u64, &'static str> {
    super::kuna_rustfmtflags::formatter_symbols(symbols)
        .into_iter()
        .filter_map(|(address, formatter)| {
            (formatter.kind == Trait::Display).then_some((address, formatter.primitive))
        })
        .collect()
}

pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, id: OpId) -> Option<Plan> {
    if !arch.rust_fmt_args
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || !matches!(arch.rustc_stable_version, Some((1, 93, 0 | 1)))
        || (arch.rust_display_formatters.is_empty()
            && (!arch.rust_fmt_flags || arch.rust_integer_formatters.is_empty()))
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 3 || call.get_out().is_some() {
        return None;
    }
    let stderr = super::kuna_rustfmtliteral::print_sink(fd, id)?;
    let space = fd.get_arch().manage().get_default_data_space()?;
    if space.get_addr_size() != 8 || space.is_big_endian() {
        return None;
    }
    let template = constant_word(fd, call.get_in(1)?)?;
    if template == 0 {
        return None;
    }
    let array = stack_offset(fd, call.get_in(2)?)?;
    if array % 8 != 0 {
        return None;
    }
    let mut position = 0usize;
    let decoded = decode_template(arch.rust_fmt_flags, |size| {
        let end = position.checked_add(size)?;
        if end > MAX_BYTES {
            return None;
        }
        let start = template.checked_add(position as u64)?;
        template.checked_add(end as u64)?;
        for offset in 0..size {
            let address = Address::new(Rc::clone(space), start + offset as u64);
            let flags = fd.get_arch().query_global_range_properties(&address);
            if flags & varnode_flags::readonly == 0 || flags & varnode_flags::volatil != 0 {
                return None;
            }
        }
        let mut bytes = vec![0; size];
        fd.get_arch()
            .loader_fill(&mut bytes, &Address::new(Rc::clone(space), start))
            .ok()?;
        position = end;
        Some(bytes)
    })?;
    let cells = argument_cells(fd, id, array, decoded.count)?;
    let mut arguments = Vec::with_capacity(decoded.count);
    let mut formatters = Vec::with_capacity(decoded.count);
    for index in 0..decoded.count {
        let pointer = cells[index * 2];
        let address = constant_word(fd, cells[index * 2 + 1])?;
        let formatter = if arch.rust_fmt_flags {
            *arch.rust_integer_formatters.get(&address)?
        } else {
            Formatter {
                primitive: *arch.rust_display_formatters.get(&address)?,
                kind: Trait::Display,
            }
        };
        let primitive = formatter.primitive;
        let offset = stack_offset(fd, pointer)?;
        let size = match primitive {
            "u8" | "i8" => 1,
            "u16" | "i16" => 2,
            "u32" | "i32" => 4,
            "u64" | "i64" => 8,
            _ => return None,
        };
        if offset % size != 0 {
            return None;
        }
        arguments.push(Argument { pointer, primitive });
        formatters.push(formatter);
    }
    let (name, literal) = decoded.render(stderr, &formatters)?;
    Some(Plan {
        name,
        literal,
        arguments,
    })
}

fn argument_cells(fd: &Funcdata, call: OpId, base: i64, count: usize) -> Option<Vec<VarnodeId>> {
    let stack = fd.get_arch().manage().get_stack_space()?;
    let mut cells = vec![None; count * 2];
    let end = base.checked_add((count * 16) as i64)?;
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id)?;
        if op.code() != OpCode::CPUI_INDIRECT || op.is_indirect_creation() {
            continue;
        }
        let Some(effect) = op.get_in(1).and_then(|v| fd.vbank().get(v)) else {
            continue;
        };
        if OpId::from(slotmap::KeyData::from_ffi(effect.get_offset())) != call {
            continue;
        }
        let output = fd.vbank().get(op.get_out()?)?;
        if output.get_space().get_index() != stack.get_index() {
            continue;
        }
        let start = output.get_offset() as i64;
        let stop = start.checked_add(output.get_size() as i64)?;
        if start >= end || stop <= base {
            continue;
        }
        if start < base || stop > end || output.get_size() != 8 || (start - base) % 8 != 0 {
            return None;
        }
        let cell = &mut cells[((start - base) / 8) as usize];
        if cell.replace(op.get_in(0)?).is_some() {
            return None;
        }
    }
    cells.into_iter().collect()
}

fn constant_word(fd: &Funcdata, mut id: VarnodeId) -> Option<u64> {
    for _ in 0..8 {
        let value = fd.vbank().get(id)?;
        if value.get_size() != 8 {
            return None;
        }
        if value.is_constant() {
            return Some(value.get_offset());
        }
        let op = fd.obank().get(value.get_def()?)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => id = op.get_in(0)?,
            OpCode::CPUI_PTRSUB => {
                let base = fd.vbank().get(op.get_in(0)?)?;
                if !base.is_constant() || base.get_offset() != 0 {
                    return None;
                }
                id = op.get_in(1)?;
            }
            _ => return None,
        }
    }
    None
}

fn stack_offset(fd: &Funcdata, mut id: VarnodeId) -> Option<i64> {
    let base = fd
        .get_arch()
        .manage()
        .get_stack_space()?
        .get_spacebase(0)
        .ok()?;
    let mut offset = 0i64;
    for _ in 0..8 {
        let value = fd.vbank().get(id)?;
        if value.get_size() != 8 || offset.unsigned_abs() > 1024 * 1024 {
            return None;
        }
        let Some(def) = value.get_def() else {
            return (value.is_input()
                && value.get_space().get_index() == base.space.as_ref()?.get_index()
                && value.get_offset() == base.offset
                && value.get_size() as u32 == base.size)
                .then_some(offset);
        };
        let op = fd.obank().get(def)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => id = op.get_in(0)?,
            OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                let literal = fd.vbank().get(op.get_in(1)?)?;
                if !literal.is_constant() || literal.get_size() != 8 {
                    return None;
                }
                let delta = literal.get_offset() as i64;
                offset = if op.code() == OpCode::CPUI_INT_SUB {
                    offset.checked_sub(delta)?
                } else {
                    offset.checked_add(delta)?
                };
                id = op.get_in(0)?;
            }
            _ => return None,
        }
    }
    None
}

#[derive(Debug, PartialEq)]
enum Piece {
    Literal(String),
    Argument(usize, Options),
}

#[derive(Debug, PartialEq)]
struct Template {
    pieces: Vec<Piece>,
    count: usize,
    indexed: bool,
}

fn decode_template(
    flags: bool,
    mut take: impl FnMut(usize) -> Option<Vec<u8>>,
) -> Option<Template> {
    let mut pieces = Vec::new();
    let mut next = 0usize;
    let mut count = 0;
    let mut used = 0u32;
    let mut indexed = false;
    for _ in 0..128 {
        let head = *take(1)?.first()?;
        match head {
            0 => {
                if count == 0 || used != (1 << count) - 1 {
                    return None;
                }
                return Some(Template {
                    pieces,
                    count,
                    indexed,
                });
            }
            1..=0x80 => {
                let len = if head == 0x80 {
                    u16::from_le_bytes(take(2)?.try_into().ok()?) as usize
                } else {
                    head as usize
                };
                if len == 0 || len > MAX_BYTES {
                    return None;
                }
                pieces.push(Piece::Literal(String::from_utf8(take(len)?).ok()?));
            }
            0xc0..=0xff => {
                let options = if flags {
                    Options::decode(head, &mut take)?
                } else if matches!(head, 0xc0 | 0xc8) {
                    Options::default()
                } else {
                    return None;
                };
                if head & 8 != 0 {
                    next = u16::from_le_bytes(take(2)?.try_into().ok()?) as usize;
                    indexed = true;
                }
                if next >= MAX_ARGS {
                    return None;
                }
                pieces.push(Piece::Argument(next, options));
                used |= 1 << next;
                next += 1;
                count = count.max(next);
            }
            _ => return None,
        }
    }
    None
}

impl Template {
    fn render(mut self, stderr: bool, formatters: &[Formatter]) -> Option<(&'static str, String)> {
        let newline = match self.pieces.last_mut() {
            Some(Piece::Literal(text)) if text.ends_with('\n') => {
                text.pop();
                true
            }
            _ => false,
        };
        let name = match (stderr, newline) {
            (false, false) => "print!",
            (false, true) => "println!",
            (true, false) => "eprint!",
            (true, true) => "eprintln!",
        };
        let mut literal = String::from("\"");
        for piece in self.pieces {
            match piece {
                Piece::Literal(text) => {
                    for ch in text.chars() {
                        match ch {
                            '{' => literal.push_str("{{"),
                            '}' => literal.push_str("}}"),
                            ch => literal.extend(ch.escape_debug()),
                        }
                    }
                }
                Piece::Argument(index, options) => {
                    literal.push('{');
                    if self.indexed {
                        literal.push_str(&index.to_string());
                    }
                    let spec = options.render(formatters.get(index)?.kind)?;
                    if !spec.is_empty() {
                        literal.push(':');
                        literal.push_str(&spec);
                    }
                    literal.push('}');
                }
            }
        }
        literal.push('"');
        Some((name, literal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(mut bytes: &[u8]) -> Option<Template> {
        decode_template(false, |n| {
            let value = bytes.get(..n)?.to_vec();
            bytes = &bytes[n..];
            Some(value)
        })
    }

    #[test]
    fn templates_preserve_literals_and_argument_indices() {
        assert_eq!(
            decode(b"\x08value = \xc0\x01\n\0")
                .unwrap()
                .render(
                    false,
                    &[Formatter {
                        primitive: "u64",
                        kind: Trait::Display
                    }; 16]
                )
                .unwrap(),
            ("println!", "\"value = {}\"".into())
        );
        assert_eq!(
            decode(b"\xc0\x01 \xc0\x01 \xc8\0\0\x01\n\0")
                .unwrap()
                .render(
                    false,
                    &[Formatter {
                        primitive: "u64",
                        kind: Trait::Display
                    }; 16]
                )
                .unwrap(),
            ("println!", "\"{0} {1} {0}\"".into())
        );
        assert_eq!(
            decode(b"\x03{\"\\\xc0\x03}\0\n\0")
                .unwrap()
                .render(
                    true,
                    &[Formatter {
                        primitive: "u64",
                        kind: Trait::Display
                    }; 16]
                )
                .unwrap(),
            ("eprintln!", "\"{{\\\"\\\\{}}}\\0\"".into())
        );
        assert_eq!(
            decode(b"\x80\x02\0ok\xc0\0")
                .unwrap()
                .render(
                    false,
                    &[Formatter {
                        primitive: "u64",
                        kind: Trait::Display
                    }; 16]
                )
                .unwrap(),
            ("print!", "\"ok{}\"".into())
        );
    }

    #[test]
    fn unsupported_or_malformed_templates_are_refused() {
        for bytes in [
            &b""[..],
            b"\0",
            b"\x01x\0",
            b"\xc1\x20\0\0\x60\0",
            b"\x81\0",
            b"\xc0",
            b"\x80\x01",
            b"\x01\xff\xc0\0",
            b"\xc8\x10\0\0",
            b"\xc8\x01\0\0",
            b"\x80\0\0\xc0\0",
        ] {
            assert!(decode(bytes).is_none(), "{bytes:?}");
        }
        assert!(decode(&[0xc0; 129]).is_none());
    }

    #[test]
    fn only_unambiguous_full_display_identities_qualify() {
        let display = |ty| format!("core::fmt::num::imp::<impl core::fmt::Display for {ty}>::fmt");
        assert_eq!(
            formatter_symbols([(0, Some(display("i64")))]).get(&0),
            Some(&"i64")
        );
        for name in [
            "fmt",
            "core::fmt::num::imp::u64::fmt",
            "core::fmt::num::imp::<impl core::fmt::LowerExp for u64>::fmt",
            "core::fmt::num::<impl core::fmt::Debug for u64>::fmt",
            "<T as core::fmt::Display>::fmt",
        ] {
            assert!(formatter_symbols([(0, Some(name.into()))]).is_empty());
        }
        let found = formatter_symbols([
            (1, Some(display("u64"))),
            (1, Some(display("usize"))),
            (2, Some(display("u64"))),
            (2, None),
            (2, Some(display("u64"))),
            (3, Some(display("u64"))),
            (3, Some(display("i64"))),
        ]);
        assert_eq!(found, BTreeMap::from([(1, "u64")]));
    }
}
