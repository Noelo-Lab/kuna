//! Preserve the byte result of formatting into a verified String writer.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::printlanguage::{OpToken, TokenType};
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriterMethod {
    DropString,
    WriteStr,
    WriteChar,
    WriteFmt,
}

pub fn writer_symbols(symbols: &[(u64, Option<String>)]) -> BTreeMap<u64, WriterMethod> {
    fn classify(name: Option<&str>) -> Option<WriterMethod> {
        Some(match name? {
            "core::ptr::drop_in_place<alloc::string::String>" => WriterMethod::DropString,
            "<alloc::string::String as core::fmt::Write>::write_str" => WriterMethod::WriteStr,
            "<alloc::string::String as core::fmt::Write>::write_char" => WriterMethod::WriteChar,
            "core::fmt::Write::write_fmt" => WriterMethod::WriteFmt,
            _ => return None,
        })
    }
    let mut entries: BTreeMap<_, _> = symbols
        .iter()
        .filter_map(|(address, name)| {
            classify(name.as_deref()).map(|method| (*address, Some(method)))
        })
        .collect();
    for (address, name) in symbols {
        if let Some(method) = entries.get_mut(address) {
            if *method != classify(name.as_deref()) {
                *method = None;
            }
        }
    }
    entries
        .into_iter()
        .filter_map(|(address, method)| method.map(|m| (address, m)))
        .collect()
}

pub(crate) struct Plan {
    pub writer: VarnodeId,
    pub pointer_bridge: bool,
    pub result_type: Option<&'static str>,
    pub format: super::kuna_rustfmtargs::Plan,
}

pub(crate) static WITH_WRITE: OpToken = OpToken {
    print1: "({ use std::fmt::Write as _; ",
    print2: " })",
    stage: 2,
    precedence: 66,
    associative: false,
    token_type: TokenType::Postsurround,
    spacing: 0,
    bump: 0,
    negate: None,
    paren_before_angle: false,
    left_to_right_only: false,
};

pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, id: OpId) -> Option<Plan> {
    if !arch.rust_fmt.writers()
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || !matches!(arch.rustc_stable_version, Some((1, 93, 0 | 1)))
        || arch.rust_string_writer_methods.is_empty()
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 5 {
        return None;
    }
    let output = fd.vbank().get(call.get_out()?)?;
    if output.get_size() != 1 {
        return None;
    }
    let result_type = match output.get_type_def_facing().get_metatype() {
        type_metatype::TYPE_BOOL => None,
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => Some("u8"),
        type_metatype::TYPE_INT => Some("i8"),
        _ => return None,
    };
    let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
    match spec.get_name() {
        "core::fmt::write" => (),
        "write" => {
            let (leaf, offset, _, scope) = fd
                .get_arch()
                .global_name_scope()?
                .name_for_varnode_scoped(spec.get_entry_address(), 1, call.get_addr())?;
            if offset != 0 || leaf != "write" || scope != ["fmt", "core"] {
                return None;
            }
        }
        _ => return None,
    }
    let writer = call.get_in(1)?;
    let value = fd.vbank().get(writer)?;
    if value.get_size() != 8 || !super::kuna_rustfmtresult::entry_value(fd, writer) {
        return None;
    }
    let pointer_bridge = match value.get_type_read_facing(id).get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => true,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    let table = super::kuna_rustfmtargs::constant_word(fd, call.get_in(2)?)?;
    if table == 0 || table % 8 != 0 {
        return None;
    }
    let bytes = vtable_bytes(fd, arch, table)?;
    let words: Vec<_> = bytes
        .chunks_exact(8)
        .map(|word| u64::from_le_bytes(word.try_into().unwrap()))
        .collect();
    if words[1] != 24 || words[2] != 8 {
        return None;
    }
    for (index, method) in [
        (0, WriterMethod::DropString),
        (3, WriterMethod::WriteStr),
        (4, WriterMethod::WriteChar),
        (5, WriterMethod::WriteFmt),
    ] {
        if arch.rust_string_writer_methods.get(&words[index]) != Some(&method) {
            return None;
        }
    }
    let format = super::kuna_rustfmtliteral::literal_bytes(fd, call.get_in(3)?, call.get_in(4)?)
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .map(|text| {
            let (name, literal) = super::kuna_rustfmtliteral::render(&text, false);
            super::kuna_rustfmtargs::Plan {
                name: if name == "println!" {
                    "writeln!"
                } else {
                    "write!"
                },
                literal,
                arguments: Vec::new(),
            }
        });
    let format = format.or_else(|| {
        super::kuna_rustfmtargs::recover_inputs_at(
            fd,
            arch,
            id,
            3,
            4,
            super::kuna_rustfmtargs::MacroKind::Write,
        )
    })?;
    Some(Plan {
        writer,
        pointer_bridge,
        result_type,
        format,
    })
}

fn vtable_bytes(fd: &Funcdata, arch: &Architecture, pointer: u64) -> Option<Vec<u8>> {
    let space = fd.get_arch().manage().get_default_data_space()?;
    if space.get_addr_size() != 8 || space.is_big_endian() {
        return None;
    }
    let end = pointer.checked_add(48)?;
    for offset in pointer..end {
        let address = Address::new(Rc::clone(space), offset);
        let flags = fd.get_arch().query_global_range_properties(&address);
        let relro = arch
            .rust_fmt_write_relro
            .is_some_and(|(lo, hi)| lo <= offset && offset < hi);
        if flags & varnode_flags::volatil != 0 || (flags & varnode_flags::readonly == 0 && !relro) {
            return None;
        }
    }
    let mut bytes = vec![0; 48];
    fd.get_arch()
        .loader_fill(&mut bytes, &Address::new(Rc::clone(space), pointer))
        .ok()?;
    Some(bytes)
}
