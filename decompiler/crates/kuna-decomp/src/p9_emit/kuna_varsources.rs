//! Machine storage for declared variables, including homes removed by simplification.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::HighVariableId;
use crate::funcdata::Funcdata;

#[derive(Debug, Clone)]
pub struct StorageSource {
    pub address: Address,
    pub size: int4,
    pub write_address: Option<Address>,
}

pub type StorageComment = (String, Rc<AddrSpace>, u64);

fn remove_join_pieces(homes: &mut BTreeMap<String, (Rc<AddrSpace>, u64)>) {
    let pieces: Vec<_> = homes
        .keys()
        .filter(|home| home.contains(':'))
        .flat_map(|home| home.split(':').map(str::to_string))
        .collect();
    for piece in pieces {
        homes.remove(&piece);
    }
}

pub fn combine_comments(
    comments: impl IntoIterator<Item = StorageComment>,
) -> Option<StorageComment> {
    let mut homes = BTreeMap::new();
    let mut fallback = None;
    for (text, space, offset) in comments {
        if text == "tmp" {
            fallback.get_or_insert((text, space, offset));
            continue;
        }
        for home in text.split(" | ") {
            homes
                .entry(home.to_string())
                .or_insert((space.clone(), offset));
        }
    }
    remove_join_pieces(&mut homes);
    if homes.is_empty() {
        return fallback;
    }
    let text = homes.keys().cloned().collect::<Vec<_>>().join(" | ");
    let (_, (space, offset)) = homes.into_iter().next()?;
    Some((text, space, offset))
}

/// Keep declaration collapse independent of diagnostic source enrichment.
pub fn representative_comment(
    fd: &Funcdata,
    arch: &Architecture,
    high: HighVariableId,
    varnode: &crate::varnode::Varnode,
) -> Option<StorageComment> {
    let address = varnode.get_addr();
    let space = address.get_space()?;
    let register =
        arch.translate()
            .get_register_name(space, address.get_offset(), varnode.get_size());
    if !register.is_empty() {
        return Some((
            register.to_ascii_lowercase(),
            space.clone(),
            address.get_offset(),
        ));
    }
    let stack = fd.get_arch().manage().get_stack_space()?;
    if stack.get_index() != space.get_index() {
        return None;
    }
    let offset = fd.high_bank().get(high)?.kuna_symbol_offset().max(0);
    let base = address - offset as i64;
    storage_comment(fd, arch, &base, varnode.get_size())
        .map(|(text, space, _)| (text, space, address.get_offset()))
}

pub fn storage_comment(
    fd: &Funcdata,
    arch: &Architecture,
    address: &Address,
    size: int4,
) -> Option<StorageComment> {
    let space = address.get_space()?;
    let offset = address.get_offset();
    let register = arch.translate().get_register_name(space, offset, size);
    if !register.is_empty() {
        return Some((register.to_ascii_lowercase(), space.clone(), offset));
    }
    if fd
        .get_arch()
        .manage()
        .get_stack_space()
        .is_some_and(|stack| stack.get_index() == space.get_index())
    {
        let signed =
            kuna_base::address::sign_extend(offset as i64, space.get_addr_size() as int4 * 8 - 1);
        let text = if signed < 0 {
            format!("stack - 0x{:x}", signed.unsigned_abs())
        } else {
            format!("stack + 0x{:x}", signed as u64)
        };
        return Some((text, space.clone(), offset));
    }
    if space.get_type() == spacetype::IPTR_JOIN {
        let record = arch.manage().find_join(offset).ok()?;
        let pieces: Option<Vec<_>> = (0..record.num_pieces())
            .map(|i| {
                let piece = record.get_piece(i);
                storage_comment(fd, arch, &piece.get_addr(), piece.size as int4)
                    .map(|(text, _, _)| text)
            })
            .collect();
        return pieces
            .filter(|pieces| !pieces.is_empty())
            .map(|pieces| (pieces.join(":"), space.clone(), offset));
    }
    if space.get_type() == spacetype::IPTR_PROCESSOR {
        return Some((
            format!("{} 0x{offset:x}", space.get_name()),
            space.clone(),
            offset,
        ));
    }
    None
}

pub fn high_comment(
    fd: &Funcdata,
    arch: &Architecture,
    high: HighVariableId,
) -> Option<StorageComment> {
    if !arch.name_style_angr {
        return None;
    }
    let high = fd.high_bank().get(high)?;
    let mut homes = BTreeMap::new();
    let mut fallback = None;
    if let Some((scope, symbol)) = fd.get_scope_local().zip(high.kuna_ref_symbol()) {
        for (address, size) in scope.symbol_storage(symbol) {
            if let Some((text, space, offset)) = storage_comment(fd, arch, &address, size) {
                homes.entry(text).or_insert((space, offset));
            }
        }
    }
    let mut pending: Vec<_> = (0..high.num_instances())
        .map(|i| high.get_instance(i))
        .collect();
    let mut visited = std::collections::HashSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        let Some(varnode) = fd.vbank().get(id) else {
            continue;
        };
        fallback.get_or_insert_with(|| (varnode.get_space().clone(), varnode.get_offset()));
        let mut address = varnode.get_addr().clone();
        let symbol_offset = varnode
            .get_high()
            .and_then(|id| fd.high_bank().get(id))
            .map(|h| h.kuna_symbol_offset())
            .unwrap_or(-1);
        if varnode.is_addr_tied()
            && symbol_offset > 0
            && fd
                .get_arch()
                .manage()
                .get_stack_space()
                .is_some_and(|stack| stack.get_index() == varnode.get_space().get_index())
        {
            address = &address - symbol_offset as i64;
        }
        if !fd.kuna_is_call_transport(id) {
            if let Some((text, space, offset)) =
                storage_comment(fd, arch, &address, varnode.get_size())
            {
                homes.entry(text).or_insert((space, offset));
            }
        }
        for source in fd.kuna_storage_sources(id) {
            if let Some((text, space, offset)) =
                storage_comment(fd, arch, &source.address, source.size)
            {
                homes.entry(text).or_insert((space, offset));
            }
        }
        if let Some(op) = varnode.get_def().and_then(|op| fd.obank().get(op)) {
            if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL) {
                for i in 0..op.num_input() {
                    if let Some(input) = op.get_in(i).filter(|&input| {
                        fd.vbank()
                            .get(input)
                            .is_some_and(|v| !v.is_constant() && v.get_size() == varnode.get_size())
                    }) {
                        pending.push(input);
                    }
                }
            }
        }
    }
    remove_join_pieces(&mut homes);
    if homes.is_empty() {
        let (space, offset) = fallback?;
        return Some(("tmp".to_string(), space, offset));
    }
    let text = homes.keys().cloned().collect::<Vec<_>>().join(" | ");
    let (_, (space, offset)) = homes.into_iter().next()?;
    Some((text, space, offset))
}
