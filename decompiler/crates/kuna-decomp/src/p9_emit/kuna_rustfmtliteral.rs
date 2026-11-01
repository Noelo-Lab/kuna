//! Literal-only Rust 1.93 fmt::Arguments at known standard output sinks.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub(crate) fn recover(
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
) -> Option<(&'static str, String)> {
    if !arch.rust_fmt_literal
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || !matches!(arch.rustc_stable_version, Some((1, 93, 0 | 1)))
    {
        return None;
    }
    let call = fd.obank().get(op)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 3 || call.get_out().is_some() {
        return None;
    }
    let name = fd.get_call_specs(fd.get_call_specs_index(op)?).get_name();
    let stderr = match name {
        "std::io::stdio::_print" => false,
        "std::io::stdio::_eprint" => true,
        _ => return None,
    };
    let ptr = constant_word(fd, call.get_in(1)?)?;
    let tag = constant_word(fd, call.get_in(2)?)?;
    let len = tag >> 1;
    if ptr == 0 || tag & 1 == 0 || len > 4096 || ptr.checked_add(len).is_none() {
        return None;
    }
    let space = fd.get_arch().manage().get_default_data_space()?;
    if space.get_addr_size() != 8 || space.is_big_endian() {
        return None;
    }
    for offset in 0..len {
        let addr = Address::new(Rc::clone(space), ptr + offset);
        let flags = fd.get_arch().query_global_range_properties(&addr);
        if flags & varnode_flags::readonly == 0 || flags & varnode_flags::volatil != 0 {
            return None;
        }
    }
    let mut bytes = vec![0; len as usize];
    if len != 0 {
        fd.get_arch()
            .loader_fill(&mut bytes, &Address::new(Rc::clone(space), ptr))
            .ok()?;
    }
    let text = std::str::from_utf8(&bytes).ok()?;
    Some(render(text, stderr))
}

fn constant_word(fd: &Funcdata, mut id: VarnodeId) -> Option<u64> {
    for _ in 0..4 {
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

fn render(text: &str, stderr: bool) -> (&'static str, String) {
    let (text, newline) = text.strip_suffix('\n').map_or((text, false), |s| (s, true));
    let name = match (stderr, newline) {
        (false, false) => "print!",
        (false, true) => "println!",
        (true, false) => "eprint!",
        (true, true) => "eprintln!",
    };
    let mut literal = String::from("\"");
    for ch in text.chars() {
        match ch {
            '{' => literal.push_str("{{"),
            '}' => literal.push_str("}}"),
            ch => literal.extend(ch.escape_debug()),
        }
    }
    literal.push('"');
    (name, literal)
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn literals_preserve_bytes_and_escape_format_braces() {
        assert_eq!(
            render("{a}\"\\\0\n\n", false),
            ("println!", "\"{{a}}\\\"\\\\\\0\\n\"".into())
        );
        assert_eq!(
            render("hello\r\n", true),
            ("eprintln!", "\"hello\\r\"".into())
        );
        assert_eq!(render("", false), ("print!", "\"\"".into()));
        assert_eq!(render("é🙂", true), ("eprint!", "\"é🙂\"".into()));
    }
}
