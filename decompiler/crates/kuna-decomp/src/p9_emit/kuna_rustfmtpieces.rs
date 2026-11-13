//! Verified pre-template Rust print descriptors with default primitive formatting.

use std::rc::Rc;

use super::kuna_rustfmtargs::{self, constant_word, stack_offset, Plan};
use super::kuna_rustfmtflags::Trait;
use crate::architecture::Architecture;
use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub(crate) fn recover(fd: &Funcdata, arch: &Architecture, id: OpId) -> Option<Plan> {
    if !arch.rust_fmt.recovers()
        || !arch.rustc_pieces_layout
        || arch.source_lang() != crate::kuna_sourcelang::SourceLang::Rust
    {
        return None;
    }
    let call = fd.obank().get(id)?;
    if call.code() != OpCode::CPUI_CALL || call.num_input() != 2 || call.get_out().is_some() {
        return None;
    }
    let stderr = super::kuna_rustfmtliteral::print_sink(fd, id)?;
    let base = stack_offset(fd, call.get_in(1)?)?;
    if base % 8 != 0 {
        return None;
    }
    let cells = kuna_rustfmtargs::stack_cells(fd, id, base, 5)?;
    let table = constant_word(fd, cells[0])?;
    let pieces = usize::try_from(constant_word(fd, cells[1])?).ok()?;
    let count = usize::try_from(constant_word(fd, cells[3])?).ok()?;
    if count > 16
        || !(count..=count + 1).contains(&pieces)
        || constant_word(fd, cells[4])? != 0
        || table == 0
        || table % 8 != 0
    {
        return None;
    }
    let (arguments, formatters) = if count == 0 {
        let pointer = constant_word(fd, cells[2])?;
        if pointer == 0 || pointer % 8 != 0 {
            return None;
        }
        (Vec::new(), Vec::new())
    } else {
        let array = stack_offset(fd, cells[2])?;
        let result = kuna_rustfmtargs::primitive_arguments(fd, arch, id, array, count)?;
        if result
            .1
            .iter()
            .any(|formatter| formatter.kind != Trait::Display)
        {
            return None;
        }
        result
    };
    let descriptors = read_constant(fd, arch, table, pieces * 16)?;
    let mut literals = Vec::with_capacity(pieces);
    let mut total = 0usize;
    for descriptor in descriptors.chunks_exact(16) {
        let pointer = u64::from_le_bytes(descriptor[..8].try_into().ok()?);
        let length = usize::try_from(u64::from_le_bytes(descriptor[8..].try_into().ok()?)).ok()?;
        total = total.checked_add(length)?;
        if pointer == 0 || total > 4096 {
            return None;
        }
        literals.push(String::from_utf8(read_constant(fd, arch, pointer, length)?).ok()?);
    }
    kuna_rustfmtargs::render_pieces(literals, arguments, formatters, stderr)
}

fn read_constant(
    fd: &Funcdata,
    arch: &Architecture,
    pointer: u64,
    length: usize,
) -> Option<Vec<u8>> {
    let space = fd.get_arch().manage().get_default_data_space()?;
    if space.get_addr_size() != 8 || space.is_big_endian() || length > 4096 {
        return None;
    }
    let end = pointer.checked_add(length as u64)?;
    for offset in pointer..end {
        let address = Address::new(Rc::clone(space), offset);
        let flags = fd.get_arch().query_global_range_properties(&address);
        let relro = arch
            .rust_fmt_pieces_relro
            .is_some_and(|(lo, hi)| lo <= offset && offset < hi);
        if flags & varnode_flags::volatil != 0 || (flags & varnode_flags::readonly == 0 && !relro) {
            return None;
        }
    }
    let mut bytes = vec![0; length];
    if length != 0 {
        fd.get_arch()
            .loader_fill(&mut bytes, &Address::new(Rc::clone(space), pointer))
            .ok()?;
    }
    Some(bytes)
}
