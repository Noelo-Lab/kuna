//! A single nominal string formal and its two verified AMD64 Go ABI inputs.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::dtype::type_metatype;
use crate::fspec::FuncProto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoStringInput {
    pub entry: Address,
    pub name: String,
    pub first_slot: int4,
    pub storage: [Address; 2],
}

impl GoStringInput {
    /// Require the effective locked prototype to retain both original inputs.
    pub fn matches(&self, proto: &FuncProto) -> bool {
        if !proto.has_store()
            || !proto.has_model()
            || proto.get_model_name() != "abi-internal"
            || !proto.is_input_locked()
            || proto.is_dotdotdot()
        {
            return false;
        }
        let Some(second_slot) = self.first_slot.checked_add(1) else {
            return false;
        };
        let (Some(data), Some(len)) = (
            proto.get_param(self.first_slot),
            proto.get_param(second_slot),
        ) else {
            return false;
        };
        let exact = |slot: &dyn crate::fspec::ProtoParameter, word: usize| {
            slot.is_type_locked()
                && slot.get_size() == 8
                && slot.get_address() == self.storage[word]
        };
        exact(data, 0)
            && exact(len, 1)
            && data.get_type().is_some_and(|ty| {
                ty.get_metatype() == type_metatype::TYPE_PTR
                    && ty.get_ptr_to().is_some_and(|byte| {
                        byte.get_size() == 1 && byte.get_metatype() == type_metatype::TYPE_UINT
                    })
            })
            && len
                .get_type()
                .is_some_and(|ty| ty.get_metatype() == type_metatype::TYPE_INT)
    }
}
