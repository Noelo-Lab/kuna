//! Nominal Go DWARF string input layout on AMD64.

use super::Facts;
use std::collections::BTreeSet;

impl Facts {
    pub(super) fn string_input_fields(&self, offset: usize) -> Option<[usize; 2]> {
        let die = self.string_layout_type(offset, gimli::DW_TAG_structure_type, 24, 16, 8)?;
        if die.type_ref.is_some() || die.children.len() != 2 {
            return None;
        }

        let mut fields = [0; 2];
        for (index, child) in die.children.iter().enumerate() {
            let member = self.dies.get(child)?;
            if self.extras.get(child)?.invalid
                || member.tag != gimli::DW_TAG_member
                || member.name != ["str", "len"][index]
                || member.data_member_location != Some(index as i64 * 8)
                || member.member_location_unknown
                || member.declaration
                || member.origin_ref.is_some()
                || member.bit_size.is_some()
                || member.bit_offset.is_some()
                || member.data_bit_offset.is_some()
                || member.byte_size.is_some_and(|size| size != 8)
                || member.alignment.is_some_and(|alignment| alignment != 8)
            {
                return None;
            }
            fields[index] = member.type_ref?;
        }

        let pointer = self.string_layout_type(fields[0], gimli::DW_TAG_pointer_type, 22, 8, 8)?;
        let data = self.string_layout_type(pointer.type_ref?, gimli::DW_TAG_base_type, 8, 1, 1)?;
        let len = self.string_layout_type(fields[1], gimli::DW_TAG_base_type, 2, 8, 8)?;
        (data.encoding == Some(gimli::DW_ATE_unsigned)
            && len.encoding == Some(gimli::DW_ATE_signed)
            && data.type_ref.is_none()
            && len.type_ref.is_none())
        .then_some(fields)
    }

    fn string_layout_type(
        &self,
        mut offset: usize,
        tag: gimli::DwTag,
        kind: u64,
        size: u64,
        alignment: u64,
    ) -> Option<&super::DieSnap> {
        let mut visited = BTreeSet::new();
        while visited.len() < 64 && visited.insert(offset) {
            let die = self.dies.get(&offset)?;
            let extra = self.extras.get(&offset)?;
            if extra.invalid
                || extra.kind.is_some_and(|actual| actual != kind)
                || die.byte_size.is_some_and(|actual| actual != size)
                || die.alignment.is_some_and(|actual| actual != alignment)
                || die.declaration
                || die.origin_ref.is_some()
                || die.bit_size.is_some()
                || die.bit_offset.is_some()
                || die.data_bit_offset.is_some()
            {
                return None;
            }
            if die.tag == gimli::DW_TAG_typedef {
                offset = die.type_ref?;
            } else {
                let exact_size = die.byte_size == Some(size)
                    || (tag == gimli::DW_TAG_pointer_type && die.byte_size.is_none());
                return (die.tag == tag && extra.kind == Some(kind) && exact_size).then_some(die);
            }
        }
        None
    }
}
