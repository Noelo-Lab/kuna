//! A stack object spanning entry SP=0 has two physical ranges in one symbol.

use kuna_base::address::Address;
use kuna_base::types::int4;
use std::rc::Rc;

pub(crate) fn pieces(addr: &Address, size: int4) -> Option<[(Address, int4, int4); 2]> {
    let space = addr.get_space()?;
    if !space.is_formal_stack_space() || space.get_word_size() != 1 || size <= 0 {
        return None;
    }
    let start = addr.get_offset();
    if start <= space.get_highest() / 2 || (addr + i64::from(size - 1)).get_offset() >= start {
        return None;
    }
    let first = space.get_highest() - start + 1;
    if first >= size as u64 || size as u64 - first > space.get_highest() {
        return None;
    }
    Some([
        (addr.clone(), 0, first as int4),
        (
            Address::new(Rc::clone(space), 0),
            first as int4,
            size - first as int4,
        ),
    ])
}

pub(crate) fn is_wrapped_join(join: &kuna_base::space::JoinRecord) -> bool {
    if join.num_pieces() != 2 {
        return false;
    }
    let big = join
        .get_piece(0)
        .space
        .as_ref()
        .is_some_and(|space| space.is_big_endian());
    let first = join.get_piece(if big { 0 } else { 1 });
    let second = join.get_piece(if big { 1 } else { 0 });
    pieces(&first.get_addr(), join.get_unified().size as int4).is_some_and(|pieces| {
        pieces[0].2 == first.size as int4
            && pieces[1].0 == second.get_addr()
            && pieces[1].2 == second.size as int4
    })
}

pub(crate) fn is_wrapped_address(addr: &Address) -> bool {
    addr.is_join()
        && addr
            .get_space()
            .and_then(|space| space.find_join(addr.get_offset()).ok())
            .is_some_and(|join| is_wrapped_join(&join))
}

pub(crate) fn is_wrapped_in_space(addr: &Address, index: usize) -> bool {
    is_wrapped_address(addr)
        && addr
            .get_space()
            .and_then(|space| space.find_join(addr.get_offset()).ok())
            .is_some_and(|join| {
                join.get_piece(0)
                    .space
                    .as_ref()
                    .is_some_and(|space| space.get_index() as usize == index)
            })
}

pub(crate) fn whole_address(
    db: &crate::database::Database,
    scope: crate::database::ScopeId,
    piece: &crate::database::SymbolEntry,
) -> Option<Address> {
    db.symbol(piece.symbol).mapentry.iter().find_map(|&entry| {
        let main = db.entry(scope, entry);
        let addr = main.get_addr();
        if main.is_piece() || !is_wrapped_address(addr) {
            return None;
        }
        let join = addr.get_space()?.find_join(addr.get_offset()).ok()?;
        let mut position = 0;
        let equivalent = join
            .get_equivalent_address(addr.get_offset() + piece.get_offset() as u64, &mut position);
        (equivalent == *piece.get_addr()
            && join.get_piece(position).size as int4 == piece.get_size())
        .then(|| addr.clone())
    })
}

/// Ghidra's awkward-JOIN fallback shares unique:0x20000000 for equal-size joins.
pub(crate) fn wire_storage(fd: &mut crate::funcdata::Funcdata) {
    let specs = fd
        .get_scope_local()
        .map(|scope| scope.database().wrapped_stack_wire_specs(scope.scope_id()))
        .unwrap_or_default();
    for (id, size) in specs {
        reserve_wire_storage(fd, id, size);
    }
}

pub(crate) fn reserve_wire_storage(fd: &mut crate::funcdata::Funcdata, id: u64, size: int4) {
    if fd
        .kuna_wrapped_wire_storage
        .get(&id)
        .is_some_and(|(_, reserved)| *reserved == size)
    {
        return;
    }
    let mut addr = fd.vbank_mut().reserve_unique(size);
    if addr.get_offset() == 0x20000000 {
        addr = fd.vbank_mut().reserve_unique(size);
    }
    fd.kuna_wrapped_wire_storage.insert(id, (addr, size));
}
