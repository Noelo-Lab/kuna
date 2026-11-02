//! Preserve the receiver ABI and owned error payload of verified Stdout formatting.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::printlanguage::{Atom, OpToken, SyntaxHighlight, TagType, TokenType};
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiverAbi {
    ReferenceCell,
    PromotedInner,
}

#[derive(Clone, Debug)]
pub struct ImageSpan {
    pub address: u64,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct WriterProfile {
    pub receiver: ReceiverAbi,
    pub protected: Vec<ImageSpan>,
    pub relro: Option<(u64, u64)>,
}

#[derive(Clone, Copy)]
pub(crate) struct Writer {
    pub value: VarnodeId,
    pub pointer_bridge: bool,
    pub receiver: ReceiverAbi,
}

pub(crate) struct Plan {
    pub writer: Writer,
    pub signed_result: bool,
    pub format: super::kuna_rustfmtargs::Plan,
}

pub(crate) static WITH_UNSIGNED_RESULT: OpToken = OpToken {
    print1: "({ use std::io::Write as _; match ",
    print2: " { std::result::Result::Ok(()) => 0u64, std::result::Result::Err(_kuna_io_error) => std::mem::transmute::<std::io::Error, *mut ()>(_kuna_io_error) as usize as u64 } })",
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

pub(crate) static WITH_SIGNED_RESULT: OpToken = OpToken {
    print1: "({ use std::io::Write as _; match ",
    print2: " { std::result::Result::Ok(()) => 0i64, std::result::Result::Err(_kuna_io_error) => std::mem::transmute::<std::io::Error, *mut ()>(_kuna_io_error) as usize as i64 } })",
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
    if !arch.rust_fmt_io
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
        || arch.rustc_stable_version != Some((1, 93, 1))
        || arch.rust_io_writer_profiles.is_empty()
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 4 {
        return None;
    }
    let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
    let profile = arch
        .rust_io_writer_profiles
        .get(&spec.get_entry_address().get_offset())?;
    let output = fd.vbank().get(call.get_out()?)?;
    if output.get_size() != 8 {
        return None;
    }
    let signed_result = match output.get_type_def_facing().get_metatype() {
        type_metatype::TYPE_INT => true,
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    let writer = call.get_in(1)?;
    let value = fd.vbank().get(writer)?;
    if value.get_size() != 8 || super::kuna_rustfmtargs::constant_word(fd, writer) == Some(0) {
        return None;
    }
    let pointer_bridge = match value.get_type_read_facing(id).get_metatype() {
        type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL => true,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    let literal_bytes = if arch.rust_fmt_literal {
        super::kuna_rustfmtliteral::literal_bytes(fd, call.get_in(2)?, call.get_in(3)?)
    } else {
        None
    };
    if literal_bytes.as_ref().is_some_and(Vec::is_empty) {
        return None;
    }
    let literal = literal_bytes
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
    let format = literal.or_else(|| {
        super::kuna_rustfmtargs::recover_inputs_at(
            fd,
            arch,
            id,
            2,
            3,
            super::kuna_rustfmtargs::MacroKind::Write,
        )
    })?;
    if !profile_unchanged(fd, profile) {
        return None;
    }
    Some(Plan {
        writer: Writer {
            value: writer,
            pointer_bridge,
            receiver: profile.receiver,
        },
        signed_result,
        format,
    })
}

fn profile_unchanged(fd: &Funcdata, profile: &WriterProfile) -> bool {
    let Some(space) = fd.get_arch().manage().get_default_data_space() else {
        return false;
    };
    if space.get_addr_size() != 8 || space.is_big_endian() || profile.protected.is_empty() {
        return false;
    }
    for span in &profile.protected {
        let Some(end) = span.address.checked_add(span.bytes.len() as u64) else {
            return false;
        };
        if span.address == 0 || span.bytes.is_empty() || span.bytes.len() > 4096 {
            return false;
        }
        for offset in span.address..end {
            let address = Address::new(Rc::clone(space), offset);
            let flags = fd.get_arch().query_global_range_properties(&address);
            let relro = profile
                .relro
                .is_some_and(|(lo, hi)| lo <= offset && offset < hi);
            if flags & varnode_flags::volatil != 0
                || (flags & varnode_flags::readonly == 0 && !relro)
            {
                return false;
            }
        }
        let mut bytes = vec![0; span.bytes.len()];
        if fd
            .get_arch()
            .loader_fill(&mut bytes, &Address::new(Rc::clone(space), span.address))
            .is_err()
            || bytes != span.bytes
        {
            return false;
        }
    }
    true
}

pub(crate) fn push_writer(
    printer: &mut crate::printc::PrintC,
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
    writer: Writer,
) {
    use crate::printc::{op_key, tokens};
    let key = Some(op_key(op));
    printer.push_op(&super::kuna_langrust::rust_tokens::ADDRESSOF_MUT, key);
    match writer.receiver {
        ReceiverAbi::ReferenceCell => printer.push_op(&tokens::DEREFERENCE, key),
        ReceiverAbi::PromotedInner => {
            printer.push_op(&tokens::FUNCTION_CALL, key);
            printer.push_atom(&Atom::syntax(
                "std::mem::transmute::<*mut (), std::io::Stdout>",
                TagType::Syntax,
                SyntaxHighlight::funcname_color,
            ));
        }
    }
    printer.push_op(printer.lang().tok_typecast, key);
    if writer.pointer_bridge {
        printer.push_op(printer.lang().tok_typecast, key);
    }
    printer.push_vn_ir(fd, arch, writer.value, op);
    if writer.pointer_bridge {
        printer.push_atom(&Atom::syntax(
            "*mut _",
            TagType::TypeToken,
            SyntaxHighlight::type_color,
        ));
    }
    printer.push_atom(&Atom::syntax(
        match writer.receiver {
            ReceiverAbi::ReferenceCell => "*mut &std::io::Stdout",
            ReceiverAbi::PromotedInner => "*mut ()",
        },
        TagType::TypeToken,
        SyntaxHighlight::type_color,
    ));
}
