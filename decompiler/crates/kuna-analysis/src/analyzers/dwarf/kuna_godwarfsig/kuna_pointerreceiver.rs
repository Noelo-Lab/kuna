//! Join exact linked method facts to the Go DWARF prototype proof.

use std::collections::BTreeSet;

use kuna_decomp::dtype::type_metatype;
use kuna_decomp::kuna_gopointermethod::Receiver;

use crate::gotypes::kuna_pointermethods::{LinkedMethod, LinkedWord};

use super::{DieSnap, Facts, Location};

impl Facts {
    pub(super) fn pointer_receiver(
        &self,
        sub: &DieSnap,
        parameters: &[(&DieSnap, Location, bool)],
        linked: &LinkedMethod,
    ) -> Option<Receiver> {
        let [(receiver, Location::Register(0), false), (argument, Location::Register(3), false), (result, result_storage, true)] =
            parameters
        else {
            return None;
        };
        let unit = self.dies.get(&sub.parent?)?;
        if !matches!(
            result_storage,
            Location::Unavailable | Location::Register(0)
        ) || sub.name != linked.symbol
            || unit.tag != gimli::DW_TAG_compile_unit
            || unit.name != linked.package
            || receiver.name.is_empty()
            || receiver.name == "_"
            || argument.name.is_empty()
            || receiver.name == argument.name
            || argument.name == "_"
        {
            return None;
        }
        let receiver_type = receiver.type_ref?;
        let pointer = self.dies.get(&receiver_type)?;
        if pointer.tag != gimli::DW_TAG_pointer_type
            || pointer.declaration
            || self.extras.get(&receiver_type)?.runtime_type != Some(linked.receiver)
            || pointer.name.strip_prefix('*') != Some(linked.record_name.as_str())
            || self.extras.get(&receiver_type)?.kind != Some(22)
        {
            return None;
        }
        let mut record_type = pointer.type_ref?;
        let alias = self.dies.get(&record_type)?;
        if alias.tag == gimli::DW_TAG_typedef {
            if alias.name != linked.record_name || alias.declaration {
                return None;
            }
            record_type = alias.type_ref?;
        }
        let record = self.dies.get(&record_type)?;
        if record.tag != gimli::DW_TAG_structure_type
            || record.declaration
            || record.name != linked.record_name
            || record.byte_size != Some(linked.record_size)
            || self.extras.get(&record_type)?.runtime_type != Some(linked.record)
            || self.extras.get(&record_type)?.kind != Some(25)
            || record.children.len() != linked.fields.len()
        {
            return None;
        }
        for (child, (name, offset, runtime_type)) in record.children.iter().zip(&linked.fields) {
            let field = self.dies.get(child)?;
            if field.tag != gimli::DW_TAG_member
                || &field.name != name
                || field
                    .data_member_location
                    .and_then(|at| u64::try_from(at).ok())
                    != Some(*offset)
            {
                return None;
            }
            self.pointer_word(field.type_ref?, runtime_type)?;
        }
        Some(Receiver {
            symbol: linked.symbol.clone(),
            package: linked.package.clone(),
            type_name: linked.record_name.clone(),
            method: linked.method.clone(),
            argument: self.pointer_word(argument.type_ref?, &linked.argument)?,
            result: self.pointer_word(result.type_ref?, &linked.result)?,
        })
    }

    fn pointer_word(&self, at: usize, expected: &LinkedWord) -> Option<type_metatype> {
        let die = self.dies.get(&at)?;
        let extra = self.extras.get(&at)?;
        if die.tag != gimli::DW_TAG_base_type
            || die.name != expected.name
            || extra.runtime_type != Some(expected.offset)
            || extra.kind != Some(expected.kind.into())
        {
            return None;
        }
        let word = self.scalar(at, &mut BTreeSet::new(), &mut 16)?;
        (word.size == 8
            && !word.float
            && matches!(
                word.meta,
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT
            ))
        .then_some(word.meta)
    }
}
