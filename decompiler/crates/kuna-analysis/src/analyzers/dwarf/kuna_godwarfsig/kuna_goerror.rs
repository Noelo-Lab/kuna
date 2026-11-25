//! Bounded Go int/error DWARF proof and its separate integer/table/data storage.

use super::*;

impl Facts {
    pub(super) fn int_error_results(&self, first: usize, error: usize) -> bool {
        self.dies.get(&error).is_some_and(|die| die.tag == gimli::DW_TAG_typedef && die.name == "error")
            && self.go_int(first).is_some() && self.error_words(error).is_some()
    }

    fn checked_type(&self, offset: usize, size: u64) -> Option<(&DieSnap, &Extra)> {
        let die = self.dies.get(&offset)?;
        let extra = self.extras.get(&offset)?;
        (!extra.invalid && !die.declaration && die.origin_ref.is_none()
            && die.byte_size.is_none_or(|actual| actual == size)
            && die.alignment.is_none_or(|alignment| alignment == 8)
            && die.bit_size.is_none() && die.bit_offset.is_none()
            && die.data_bit_offset.is_none()).then_some((die, extra))
    }

    fn go_int(&self, mut offset: usize) -> Option<()> {
        let mut visited = BTreeSet::new();
        for _ in 0..16 {
            if !visited.insert(offset) { return None; }
            let (die, extra) = self.checked_type(offset, 8)?;
            if die.name != "int" || !die.children.is_empty() { return None; }
            match die.tag {
                gimli::DW_TAG_typedef if matches!(extra.kind, None | Some(2)) => offset = die.type_ref?,
                gimli::DW_TAG_base_type => return (extra.kind == Some(2)
                    && die.byte_size == Some(8) && die.encoding == Some(gimli::DW_ATE_signed)
                    && die.type_ref.is_none()).then_some(()),
                _ => return None,
            }
        }
        None
    }

    fn error_words(&self, mut offset: usize) -> Option<()> {
        let mut visited = BTreeSet::new();
        let (mut error_kind, mut storage_alias) = (false, false);
        for _ in 0..16 {
            if !visited.insert(offset) { return None; }
            let (die, extra) = self.checked_type(offset, 16)?;
            if die.tag != gimli::DW_TAG_typedef {
                if !error_kind || die.tag != gimli::DW_TAG_structure_type
                    || die.name != "runtime.iface" || extra.kind != Some(25)
                    || die.byte_size != Some(16) || die.type_ref.is_some()
                    || die.children.len() != 2
                {
                    return None;
                }
                let mut members = die.children.iter();
                self.error_table(self.interface_member(*members.next()?, "tab", 0)?)?;
                self.error_data(self.interface_member(*members.next()?, "data", 8)?)?;
                return Some(());
            }
            if !die.children.is_empty() { return None; }
            match die.name.as_str() {
                "error" if !storage_alias && matches!(extra.kind, None | Some(20)) => {
                    error_kind |= extra.kind == Some(20);
                }
                "runtime.iface" if error_kind && matches!(extra.kind, None | Some(25)) => storage_alias = true,
                _ => return None,
            }
            offset = die.type_ref?;
        }
        None
    }

    fn interface_member(&self, offset: usize, name: &str, displacement: i64) -> Option<usize> {
        let (die, extra) = self.checked_type(offset, 8)?;
        (die.tag == gimli::DW_TAG_member && die.name == name && extra.kind.is_none()
            && die.data_member_location == Some(displacement) && !die.member_location_unknown
            && die.children.is_empty()).then_some(die.type_ref?)
    }

    fn error_table(&self, offset: usize) -> Option<()> {
        let (pointer, extra) = self.checked_type(offset, 8)?;
        if pointer.tag != gimli::DW_TAG_pointer_type || extra.kind != Some(22)
            || !pointer.children.is_empty()
        {
            return None;
        }
        let target_name = match pointer.name.as_str() {
            "*runtime.itab" => "runtime.itab",
            "*internal/abi.ITab" => "internal/abi.ITab",
            _ => return None,
        };
        let mut offset = pointer.type_ref?;
        let mut visited = BTreeSet::new();
        for _ in 0..16 {
            if !visited.insert(offset) { return None; }
            let (die, extra) = self.checked_type(offset, 32)?;
            if die.name != target_name { return None; }
            match die.tag {
                gimli::DW_TAG_typedef if die.children.is_empty() && matches!(extra.kind, None | Some(25)) => {
                    offset = die.type_ref?;
                }
                gimli::DW_TAG_structure_type => return (extra.kind == Some(25)
                    && die.byte_size == Some(32) && die.type_ref.is_none()).then_some(()),
                _ => return None,
            }
        }
        None
    }

    fn error_data(&self, offset: usize) -> Option<()> {
        let (die, extra) = self.checked_type(offset, 8)?;
        (die.tag == gimli::DW_TAG_pointer_type && die.name == "unsafe.Pointer"
            && matches!(extra.kind, None | Some(26)) && die.type_ref.is_none()
            && die.children.is_empty()).then_some(())
    }
}

pub(super) fn fields(integer: &Rc<Datatype>, pointer: &Rc<Datatype>) -> Vec<TypeField> {
    vec![
        TypeField::new(0, 0, "result0", Rc::clone(integer)),
        TypeField::new(8, 8, "error_tab", Rc::clone(pointer)),
        TypeField::new(16, 16, "error_data", Rc::clone(pointer)),
    ]
}

pub(super) fn storage_matches(arch: &Architecture, slot: &ParameterPieces) -> bool {
    if slot.flags != 0 || slot.type_.as_ref().is_none_or(|ty| LangFacet::GoIntErrorResults.go_int_error_fields(ty).is_none())
        || !slot.addr.get_space().is_some_and(|space| space.get_type() == kuna_base::space::spacetype::IPTR_JOIN)
    {
        return false;
    }
    let Ok(join) = arch.manage().find_join(slot.addr.get_offset()) else { return false };
    if join.num_pieces() != 3 || join.get_unified().size != 24 { return false; }
    ["RCX", "RBX", "RAX"].iter().enumerate().all(|(index, name)| {
        let Ok(expected) = arch.get_register_varnode(name.as_bytes()) else { return false };
        let piece = join.get_piece(index as i32);
        piece.size == 8 && expected.size == 8 && piece.offset == expected.offset
            && piece.space.as_ref().zip(expected.space.as_ref())
                .is_some_and(|(space, expected)| space.get_index() == expected.get_index())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(table: &str) -> Facts {
        let mut facts = Facts::default();
        for (offset, tag, name, size, kind, target) in [
            (1, gimli::DW_TAG_base_type, "int", Some(8), Some(2), None),
            (2, gimli::DW_TAG_typedef, "error", None, None, Some(3)),
            (3, gimli::DW_TAG_typedef, "error", None, Some(20), Some(4)),
            (4, gimli::DW_TAG_typedef, "runtime.iface", None, None, Some(5)),
            (5, gimli::DW_TAG_structure_type, "runtime.iface", Some(16), Some(25), None),
            (6, gimli::DW_TAG_member, "tab", None, None, Some(8)),
            (7, gimli::DW_TAG_member, "data", None, None, Some(11)),
            (8, gimli::DW_TAG_pointer_type, "", None, Some(22), Some(9)),
            (9, gimli::DW_TAG_typedef, table, None, None, Some(10)),
            (10, gimli::DW_TAG_structure_type, table, Some(32), Some(25), None),
            (11, gimli::DW_TAG_pointer_type, "unsafe.Pointer", None, None, None),
        ] {
            let mut die = DieSnap::new(tag, 1);
            die.name = name.into();
            die.byte_size = size;
            die.type_ref = target;
            facts.dies.insert(offset, die);
            facts.extras.insert(offset, Extra { kind, ..Extra::default() });
        }
        facts.dies.get_mut(&1).unwrap().encoding = Some(gimli::DW_ATE_signed);
        facts.dies.get_mut(&5).unwrap().children = vec![6, 7];
        facts.dies.get_mut(&6).unwrap().data_member_location = Some(0);
        facts.dies.get_mut(&7).unwrap().data_member_location = Some(8);
        facts.dies.get_mut(&8).unwrap().name = format!("*{table}");
        facts
    }

    #[test]
    fn observed_int_error_graphs_keep_table_and_data_separate() {
        for table in ["runtime.itab", "internal/abi.ITab"] {
            let facts = graph(table);
            assert!(facts.int_error_results(1, 2));
            assert!(!facts.int_error_results(1, 5), "storage alone does not establish error");
            assert!(facts.scalar(2, &mut BTreeSet::new(), &mut 256).is_none(), "error is not a scalar or an accepted input");
        }
    }

    #[test]
    fn first_result_requires_actual_go_int() {
        for change in 0..5 {
            let mut facts = graph("runtime.itab");
            match change {
                0 => facts.extras.get_mut(&1).unwrap().kind = Some(6),
                1 => facts.dies.get_mut(&1).unwrap().name = "int64".into(),
                2 => facts.dies.get_mut(&1).unwrap().byte_size = Some(4),
                3 => facts.dies.get_mut(&1).unwrap().encoding = Some(gimli::DW_ATE_unsigned),
                _ => facts.extras.get_mut(&1).unwrap().invalid = true,
            }
            assert!(!facts.int_error_results(1, 2), "mutation {change}");
        }
    }

    #[test]
    fn nominal_interface_and_runtime_layout_are_both_required() {
        for change in 0..9 {
            let mut facts = graph("runtime.itab");
            match change {
                0 => facts.extras.get_mut(&3).unwrap().kind = None,
                1 => facts.extras.get_mut(&3).unwrap().kind = Some(25),
                2 => facts.dies.get_mut(&2).unwrap().name = "main.Other".into(),
                3 => facts.dies.get_mut(&5).unwrap().name = "runtime.eface".into(),
                4 => facts.extras.get_mut(&5).unwrap().kind = Some(20),
                5 => facts.dies.get_mut(&5).unwrap().byte_size = Some(24),
                6 => facts.dies.get_mut(&4).unwrap().tag = gimli::DW_TAG_volatile_type,
                7 => facts.dies.get_mut(&5).unwrap().declaration = true,
                _ => facts.dies.get_mut(&3).unwrap().type_ref = Some(3),
            }
            assert!(!facts.int_error_results(1, 2), "mutation {change}");
        }
    }

    #[test]
    fn interface_members_refuse_changed_or_incomplete_storage() {
        for change in 0..9 {
            let mut facts = graph("runtime.itab");
            match change {
                0 => facts.dies.get_mut(&6).unwrap().name = "_type".into(),
                1 => facts.dies.get_mut(&7).unwrap().data_member_location = Some(0),
                2 => facts.dies.get_mut(&6).unwrap().bit_size = Some(64),
                3 => facts.dies.get_mut(&7).unwrap().member_location_unknown = true,
                4 => facts.dies.get_mut(&7).unwrap().type_ref = None,
                5 => facts.dies.get_mut(&5).unwrap().children.reverse(),
                6 => facts.dies.get_mut(&5).unwrap().children.push(7),
                7 => facts.dies.get_mut(&7).unwrap().alignment = Some(4),
                _ => facts.extras.get_mut(&6).unwrap().invalid = true,
            }
            assert!(!facts.int_error_results(1, 2), "mutation {change}");
        }
    }

    #[test]
    fn interface_pointers_require_the_observed_table_and_opaque_data() {
        for change in 0..10 {
            let mut facts = graph("internal/abi.ITab");
            match change {
                0 => facts.dies.get_mut(&8).unwrap().name = "*other.Table".into(),
                1 => facts.dies.get_mut(&8).unwrap().byte_size = Some(4),
                2 => facts.extras.get_mut(&8).unwrap().kind = None,
                3 => facts.dies.get_mut(&9).unwrap().type_ref = Some(9),
                4 => facts.dies.get_mut(&10).unwrap().byte_size = Some(24),
                5 => facts.extras.get_mut(&10).unwrap().kind = Some(20),
                6 => facts.dies.get_mut(&11).unwrap().type_ref = Some(1),
                7 => facts.dies.get_mut(&11).unwrap().name = "*void".into(),
                8 => facts.dies.get_mut(&11).unwrap().byte_size = Some(16),
                _ => facts.extras.get_mut(&11).unwrap().kind = Some(22),
            }
            assert!(!facts.int_error_results(1, 2), "mutation {change}");
        }
    }
}
