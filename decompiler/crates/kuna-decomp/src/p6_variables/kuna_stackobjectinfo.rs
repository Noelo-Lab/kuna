//! Portable logical stack-object selectors and native use sites for front ends.

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackObjectUseInfo {
    pub address: u64,
    pub slot: i32,
    pub type_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackObjectInfo {
    pub id: String,
    pub name: String,
    pub stack_offset: i64,
    pub size: i64,
    pub defined: bool,
    pub uses: Vec<StackObjectUseInfo>,
}

pub fn extract(arch: &Architecture, fd: &Funcdata) -> Vec<StackObjectInfo> {
    crate::kuna_stackobjects::objects(fd)
        .iter()
        .map(|object| {
            let asserted = crate::kuna_stackobjectasserts::specs(fd)
                .iter()
                .find(|assertion| assertion.selector == object.name);
            StackObjectInfo {
                id: object.name.clone(),
                name: asserted.map_or_else(|| object.name.clone(), |a| a.name.clone()),
                stack_offset: crate::decompile_drive::signed_space_offset(
                    object
                        .address
                        .get_space()
                        .expect("stack object has storage"),
                    object.address.get_offset(),
                ),
                size: object.size as i64,
                defined: object.is_defined(),
                uses: object
                    .uses
                    .iter()
                    .map(|use_| StackObjectUseInfo {
                        address: use_.point.address,
                        slot: use_.slot,
                        type_name: crate::kuna_bytehonest::exported_type_name(arch, &use_.datatype),
                    })
                    .collect(),
            }
        })
        .collect()
}
