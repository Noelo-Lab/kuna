//! Keep overwritten argument components live in an unambiguous ABI register partition.

use crate::dtype::type_class;
use crate::fspec::{FuncProto, ParamEntry};
use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpaceManager};
use kuna_base::types::int4;
use kuna_num::pcoderaw::VarnodeData;

pub(super) fn input_partition(
    proto: &FuncProto,
    manager: &AddrSpaceManager,
    addr: &Address,
    size: int4,
) -> Option<Vec<VarnodeData>> {
    let space = addr.get_space()?;
    let register = manager.get_space_by_name("register")?;
    if space.get_type() != spacetype::IPTR_PROCESSOR
        || space.get_index() != register.get_index()
        || size <= 0
        || size > 64
    {
        return None;
    }
    let start = addr.get_offset();
    let end = start.checked_add(size as u64)?;
    let mut entries: [Option<&ParamEntry>; 8] = [None; 8];
    let mut count = 0;
    for entry in proto.model().input().get_entry() {
        if let Some(join) = entry.get_join_record() {
            for i in 0..join.num_pieces() {
                let piece = join.get_piece(i);
                if piece
                    .space
                    .as_ref()
                    .is_some_and(|s| s.get_index() == space.get_index())
                    && piece.offset < end
                    && start < piece.offset.checked_add(piece.size as u64)?
                {
                    return None;
                }
            }
        }
        if entry.get_space().get_index() != space.get_index() {
            continue;
        }
        let low = entry.get_base();
        let width = entry.get_size();
        if width <= 0 {
            return None;
        }
        let high = low.checked_add(width as u64)?;
        if low >= end || high <= start {
            continue;
        }
        if low < start
            || high > end
            || entry.get_type() != type_class::TYPECLASS_GENERAL
            || !entry.is_exclusion()
            || count == entries.len()
        {
            return None;
        }
        entries[count] = Some(entry);
        count += 1;
    }
    if count < 2 {
        return None;
    }
    entries[..count].sort_unstable_by_key(|e| e.unwrap().get_base());
    let mut cursor = start;
    for entry in entries[..count].iter().flatten() {
        if entry.get_base() != cursor {
            return None;
        }
        cursor += entry.get_size() as u64;
    }
    if cursor != end {
        return None;
    }
    Some(
        entries[..count]
            .iter()
            .flatten()
            .map(|entry| VarnodeData {
                space: Some(space.clone()),
                offset: entry.get_base(),
                size: entry.get_size() as u32,
            })
            .collect(),
    )
}

/// Whether `op` truncates `vn`, a PIECE output that an `onlyOpUse` walk
/// reached through exactly one input, to bytes outside that input.
pub(crate) fn subpiece_drops_reached_piece(
    fd: &crate::funcdata::Funcdata,
    op: crate::context::OpId,
    vn: crate::context::VarnodeId,
) -> bool {
    if !fd.get_arch().multi_reg_args {
        return false;
    }
    let Some(sub) = fd.obank().get(op) else { return false };
    let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    let Some(piece) = fd.obank().get(def) else { return false };
    if piece.code() != kuna_num::opcodes::OpCode::CPUI_PIECE {
        return false;
    }
    let (Some(high), Some(low)) = (piece.get_in(0), piece.get_in(1)) else { return false };
    let (Some(high), Some(low)) = (fd.vbank().get(high), fd.vbank().get(low)) else { return false };
    let Some(offset) = sub.get_in(1).and_then(|c| fd.vbank().get(c)).map(|c| c.get_offset()) else {
        return false;
    };
    let Some(size) = sub.get_out().and_then(|o| fd.vbank().get(o)).map(|o| o.get_size()) else {
        return false;
    };
    let low_end = low.get_size() as u64;
    let end = offset.saturating_add(size as u64);
    match (high.is_mark(), low.is_mark()) {
        (true, false) => end <= low_end,
        (false, true) => offset >= low_end,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{type_class, type_metatype, Datatype};
    use crate::fspec::{ParamEntry, ProtoModel};
    use kuna_base::space::{AddrSpace, AddrSpaceManager, ConstantSpace};
    use std::rc::Rc;

    fn prototype(
        big: bool,
        offsets: &[u64],
        width: int4,
    ) -> (FuncProto, AddrSpaceManager, Rc<AddrSpace>) {
        let ranges: Vec<_> = offsets.iter().map(|&off| (off, width)).collect();
        prototype_ranges(big, "register", &ranges)
    }

    pub(super) fn prototype_ranges(
        big: bool,
        name: &str,
        ranges: &[(u64, int4)],
    ) -> (FuncProto, AddrSpaceManager, Rc<AddrSpace>) {
        prototype_ranges_class(big, name, ranges, type_class::TYPECLASS_GENERAL)
    }

    fn prototype_ranges_class(
        big: bool,
        name: &str,
        ranges: &[(u64, int4)],
        class: type_class,
    ) -> (FuncProto, AddrSpaceManager, Rc<AddrSpace>) {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "register",
                big,
                8,
                1,
                1,
                0,
                0,
                0,
            )))
            .unwrap();
        let space = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            big,
            8,
            1,
            2,
            0,
            0,
            0,
        ));
        let space = if name == "register" {
            manager.get_space_by_name("register").unwrap().clone()
        } else {
            manager.insert_space(space.clone()).unwrap();
            space
        };
        let mut model = ProtoModel::new(&manager);
        model.build_param_list("standard").unwrap();
        for (i, &(offset, width)) in ranges.iter().enumerate() {
            let input = model.input_mut();
            let entry = ParamEntry::seed(
                i as int4,
                class,
                space.clone(),
                offset,
                width,
                1,
                0,
                0,
                true,
                false,
                input.get_entry(),
                &manager,
            )
            .unwrap();
            input.push_entry(entry);
        }
        model.input_mut().finish_decode();
        let mut proto = FuncProto::new();
        proto.set_internal(
            Rc::new(model),
            Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)),
        );
        (proto, manager, space)
    }

    #[test]
    fn complete_two_and_four_slot_partitions_in_both_byte_orders() {
        for big in [false, true] {
            for width in [4, 8] {
                for count in [2, 4] {
                    let offsets: Vec<_> = (0..count).map(|i| 0x20 + (i * width) as u64).collect();
                    let (proto, manager, space) = prototype(big, &offsets, width);
                    let pieces = input_partition(
                        &proto,
                        &manager,
                        &Address::new(space, 0x20),
                        count * width,
                    )
                    .unwrap();
                    assert_eq!(pieces.iter().map(|p| p.offset).collect::<Vec<_>>(), offsets);
                    assert!(pieces.iter().all(|p| p.size == width as u32));
                }
            }
        }
    }

    #[test]
    fn one_entry_padding_gaps_and_partial_entries_are_not_complete_partitions() {
        for big in [false, true] {
            let (proto, manager, space) = prototype(big, &[0x20], 8);
            assert!(
                input_partition(&proto, &manager, &Address::new(space.clone(), 0x20), 16).is_none()
            );
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 8).is_none());
            let (proto, manager, space) = prototype(big, &[0x20, 0x28], 4);
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 12).is_none());
            let (proto, manager, space) = prototype(big, &[0x20, 0x24], 4);
            for (start, size) in [
                (0x1c, 12),
                (0x22, 6),
                (0x20, 6),
                (0x20, 10),
                (0x20, 0),
                (0x20, 128),
            ] {
                assert!(input_partition(
                    &proto,
                    &manager,
                    &Address::new(space.clone(), start),
                    size
                )
                .is_none());
            }
        }
    }

    #[test]
    fn floating_register_aliases_are_not_integer_argument_partitions() {
        for big in [false, true] {
            let (proto, manager, space) = prototype_ranges_class(
                big,
                "register",
                &[(0x20, 4), (0x24, 4)],
                type_class::TYPECLASS_FLOAT,
            );
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 8).is_none());
        }
    }

    #[test]
    fn mixed_size_unique_entries_are_sorted_by_address() {
        let (proto, manager, space) = prototype_ranges(false, "register", &[(0x24, 8), (0x20, 4)]);
        let pieces = input_partition(&proto, &manager, &Address::new(space, 0x20), 12).unwrap();
        assert_eq!(
            pieces
                .iter()
                .map(|p| (p.offset, p.size))
                .collect::<Vec<_>>(),
            vec![(0x20, 4), (0x24, 8)]
        );
    }

    #[test]
    fn overlapping_aliases_data_spaces_and_too_many_slots_decline() {
        for big in [false, true] {
            let (proto, manager, space) =
                prototype_ranges(big, "register", &[(0x20, 4), (0x24, 4), (0x20, 8)]);
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 8).is_none());
            let (proto, manager, space) = prototype_ranges(big, "data", &[(0x20, 4), (0x24, 4)]);
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 8).is_none());
            let ranges: Vec<_> = (0..9).map(|i| (0x20 + i, 1)).collect();
            let (proto, manager, space) = prototype_ranges(big, "register", &ranges);
            assert!(input_partition(&proto, &manager, &Address::new(space, 0x20), 9).is_none());
        }
    }
}
