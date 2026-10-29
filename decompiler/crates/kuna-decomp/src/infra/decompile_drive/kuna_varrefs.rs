//! Per-render storage queries share one varnode-bank traversal.

use super::{parameter_storage, select_storage_varrefs, signed_space_offset, VarInfo};
use crate::context::HighVariableId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StorageKey {
    Parameter((u8, i32, u64), Option<usize>, i64),
    Stack(i64, i64),
}

impl StorageKey {
    fn parameter(address: &Address, size: i64) -> Self {
        Self::Parameter(
            address.sort_key(),
            address.get_space().map(|space| Rc::as_ptr(space) as usize),
            size,
        )
    }

    fn variable(fd: &Funcdata, variable: &VarInfo) -> Option<Self> {
        if variable.is_param {
            let (address, size) = parameter_storage(fd, variable.arg_index?)?;
            Some(Self::parameter(&address, size))
        } else {
            fd.get_scope_local()?;
            Some(Self::Stack(variable.stack_offset?, variable.size))
        }
    }
}

type StorageMatch = (u64, Option<HighVariableId>, bool);

pub(super) struct StorageIndex {
    matches: BTreeMap<StorageKey, Vec<StorageMatch>>,
}

impl StorageIndex {
    pub(super) fn new(fd: &Funcdata, variables: &[VarInfo]) -> Self {
        let mut matches: BTreeMap<_, Vec<StorageMatch>> = variables.iter()
            .filter_map(|variable| StorageKey::variable(fd, variable))
            .map(|key| (key, Vec::new()))
            .collect();
        if matches.is_empty() {
            return Self { matches };
        }
        let has_parameters = matches.keys().any(|key| matches!(key, StorageKey::Parameter(..)));
        let stack_space = fd.get_scope_local().map(|scope| scope.get_space_id());
        for id in fd.vbank().iter_loc() {
            let Some(varnode) = fd.vbank().get(id) else { continue };
            let entry = (varnode.get_create_index() as u64, varnode.get_high(), varnode.is_input());
            if has_parameters {
                let key = StorageKey::parameter(varnode.get_addr(), varnode.get_size() as i64);
                if let Some(bucket) = matches.get_mut(&key) {
                    bucket.push(entry);
                }
            }
            if let Some(space) = stack_space {
                if varnode.get_addr().get_space().is_some_and(|s| s.get_index() == space.get_index()) {
                    let key = StorageKey::Stack(
                        signed_space_offset(space, varnode.get_offset()), varnode.get_size() as i64,
                    );
                    if let Some(bucket) = matches.get_mut(&key) {
                        bucket.push((entry.0, entry.1, false));
                    }
                }
            }
        }
        Self { matches }
    }

    pub(super) fn varrefs(&self, fd: &Funcdata, variable: &VarInfo) -> BTreeSet<u64> {
        let Some(key) = StorageKey::variable(fd, variable) else { return BTreeSet::new() };
        let Some(bucket) = self.matches.get(&key) else { return BTreeSet::new() };
        let inputs_only = variable.is_param && bucket.iter().any(|entry| entry.2);
        let matches: Vec<_> = bucket.iter().filter(|entry| !inputs_only || entry.2)
            .map(|entry| (entry.0, entry.1)).collect();
        select_storage_varrefs(fd, &matches, &variable.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use crate::dtype::{type_metatype, Datatype};
    use crate::fspec::ParameterPieces;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, SpacebaseSpace, UniqueSpace};

    fn fixture() -> Funcdata {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 2, addrspace_flags::hasphysical, 1, 1));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager.insert_space(Rc::new(SpacebaseSpace::new("stack", 3, 4, &ram, 1, true, false))).unwrap();
        Funcdata::new("f", "f", Rc::new(ArchContext::new(manager)), Address::new(ram, 0x1000), 0x1000_0000, 0x40).unwrap()
    }

    fn variable(offset: i64, size: i64) -> VarInfo {
        VarInfo { name: "local".into(), type_name: "int".into(), stack_offset: Some(offset), size, is_param: false, arg_index: None, line_numbers: vec![], addresses: vec![], split_piece: false }
    }

    #[test]
    fn stack_matches_normalize_offsets_and_keep_exact_widths() {
        let mut fd = fixture();
        let space = Rc::clone(fd.get_scope_local().unwrap().get_space_id());
        for offset in [0xffff_fffc, 0xffff_ffff_ffff_fffc, 0, 4] {
            for size in [1, 4, 8] {
                fd.new_varnode(size, &Address::new(Rc::clone(&space), offset), None);
            }
        }
        let vars = vec![variable(-4, 4), variable(0, 8), variable(4, 1), variable(8, 4), variable(-4, i64::MAX)];
        let index = StorageIndex::new(&fd, &vars);
        for var in &vars {
            assert_eq!(index.varrefs(&fd, var), super::super::variable_storage_varrefs(&fd, var));
        }
        assert_eq!(index.varrefs(&fd, &vars[0]).len(), 2);
        assert!(index.varrefs(&fd, &vars[3]).is_empty());
        assert!(index.varrefs(&fd, &vars[4]).is_empty());
    }

    #[test]
    fn parameter_inputs_take_precedence_over_same_storage_temporaries() {
        let mut fd = fixture();
        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let address = Address::new(ram, 0x20);
        fd.get_func_proto_mut().attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        fd.get_func_proto_mut().set_param(0, "param", &ParameterPieces { addr: address.clone(), type_: Some(Rc::new(Datatype::new(4, type_metatype::TYPE_INT))), flags: 0 });
        let input = fd.new_varnode(4, &address, None);
        let input = fd.set_input_varnode(input).unwrap();
        fd.new_varnode(4, &address, None);
        fd.new_varnode(8, &address, None);
        let mut var = variable(0, 4);
        var.is_param = true;
        var.arg_index = Some(0);
        let index = StorageIndex::new(&fd, &[var.clone()]);
        let expected = BTreeSet::from([fd.vbank().get(input).unwrap().get_create_index() as u64]);
        assert_eq!(index.varrefs(&fd, &var), expected);
        assert_eq!(index.varrefs(&fd, &var), super::super::variable_storage_varrefs(&fd, &var));
    }

    #[test]
    fn reused_storage_still_requires_a_unique_named_high() {
        let mut fd = fixture();
        fd.set_high_level();
        let space = Rc::clone(fd.get_scope_local().unwrap().get_space_id());
        let address = Address::new(space, 0xffff_fffc);
        let a = fd.new_varnode(4, &address, None);
        let b = fd.new_varnode(4, &address, None);
        let high = fd.vbank().get(a).unwrap().get_high().unwrap();
        fd.high_bank_mut().get_mut(high).unwrap().set_kuna_name("selected");
        let mut selected = variable(-4, 4);
        selected.name = "selected".into();
        let ambiguous = variable(-4, 4);
        let index = StorageIndex::new(&fd, &[selected.clone(), ambiguous.clone()]);
        assert_eq!(index.varrefs(&fd, &selected), BTreeSet::from([fd.vbank().get(a).unwrap().get_create_index() as u64]));
        assert!(index.varrefs(&fd, &ambiguous).is_empty());
        assert_ne!(fd.vbank().get(a).unwrap().get_high(), fd.vbank().get(b).unwrap().get_high());
        for var in [selected, ambiguous] {
            assert_eq!(index.varrefs(&fd, &var), super::super::variable_storage_varrefs(&fd, &var));
        }
    }

    #[test]
    fn parameter_keys_distinguish_spaces_with_equal_indices() {
        let space = || Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 4, 1, 2, addrspace_flags::hasphysical, 1, 1));
        let a = Address::new(space(), 0x20);
        let b = Address::new(space(), 0x20);
        assert!(StorageKey::parameter(&a, 4) != StorageKey::parameter(&b, 4));
        assert!(StorageKey::parameter(&a, 4) == StorageKey::parameter(&a.clone(), 4));
    }
}
