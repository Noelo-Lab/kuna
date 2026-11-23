//! Spell the all-zero return of a declared two-pointer carrier.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_langgo::GO_SPELLER;
use crate::kuna_langtypes::TypeSpeller;
use crate::printc::{op_key, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};

use super::kuna_gopairreturns::is_zero_word_carrier;

fn pointer_pair(ty: &Datatype) -> bool {
    ty.get_metatype() == type_metatype::TYPE_STRUCT
        && !ty.get_name().is_empty()
        && ty.get_size() == 16
        && ty.get_alignment() == 8
        && !ty.is_incomplete()
        && !ty.has_bitfields()
        && ty.num_depend() == 2
        && (0..2).all(|index| {
            ty.get_field(index).is_some_and(|field| {
                field.offset == index * 8
                    && field.field_type.get_size() == 8
                    && field.field_type.get_metatype() == type_metatype::TYPE_PTR
                    && field.field_type.get_ptr_to().is_some_and(|pointee| {
                        pointee.get_metatype() == type_metatype::TYPE_VOID
                    })
            })
        })
}

impl PrintC {
    pub(crate) fn push_go_zero_record_return(
        &mut self,
        fd: &Funcdata,
        value: VarnodeId,
        op: OpId,
    ) -> bool {
        let proto = fd.get_func_proto();
        let Some(target) = proto.get_output_type() else {
            return false;
        };
        if !proto.is_output_locked() || !pointer_pair(target) || !is_zero_word_carrier(fd, value) {
            return false;
        }
        let spelling = GO_SPELLER.declarator(&self.rt_ctx, target).0;
        self.push_atom(&Atom::with_op(
            format!("{spelling}{{}}"),
            TagType::TypeToken,
            SyntaxHighlight::type_color,
            op_key(op),
        ));
        true
    }
}
