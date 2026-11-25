//! Declared receiver provenance for a proven Go pointer method.

use crate::architecture::Architecture;
use crate::dtype::type_metatype;
use crate::fspec::{FuncProto, ProtoParameter};

#[derive(Clone, Debug)]
pub struct Receiver {
    pub symbol: String,
    pub package: String,
    pub type_name: String,
    pub method: String,
    pub argument: type_metatype,
    pub result: type_metatype,
}

impl Receiver {
    pub(crate) fn matches(&self, arch: &Architecture, proto: &FuncProto) -> bool {
        if !proto.has_store()
            || proto.get_model_name() != "abi-internal"
            || !proto.is_input_locked()
            || !proto.is_output_locked()
            || proto.is_dotdotdot()
            || proto.num_params() != 2
        {
            return false;
        }
        let (Some(receiver), Some(argument)) = (proto.get_param(0), proto.get_param(1)) else {
            return false;
        };
        receiver.get_type().is_some_and(|ty| {
            ty.get_ptr_to().is_some_and(|pointee| {
                pointee.get_metatype() == type_metatype::TYPE_STRUCT
                    && pointee.get_name() == self.type_name
            })
        }) && stored(arch, receiver, b"RAX", type_metatype::TYPE_PTR)
            && stored(arch, argument, b"RBX", self.argument)
            && stored(arch, proto.get_output(), b"RAX", self.result)
    }
}

fn stored(
    arch: &Architecture,
    param: &dyn ProtoParameter,
    register: &[u8],
    meta: type_metatype,
) -> bool {
    param.get_size() == 8
        && param.get_type().is_some_and(|ty| ty.get_metatype() == meta)
        && arch
            .get_register_varnode(register)
            .ok()
            .is_some_and(|reg| param.get_address() == reg.get_addr())
}
