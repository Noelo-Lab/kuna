//! Compact string facts materialized only when a function reads them.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;

use crate::architecture::Architecture;
use crate::funcdata::Funcdata;
use crate::varnode::varnode_flags;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    pub addr: u64,
    pub len: u32,
    pub char_size: u8,
    pub priority: u8,
}

pub fn materialize(arch: &mut Architecture, fd: &Funcdata) -> KunaResult<usize> {
    if arch.lazy_string_facts.is_empty() {
        return Ok(0);
    }
    let mut constants: Vec<u64> = fd
        .vbank()
        .iter_loc()
        .filter_map(|id| fd.vbank().get(id))
        .filter(|vn| vn.is_constant())
        .map(|vn| vn.get_offset())
        .collect();
    constants.sort_unstable();
    constants.dedup();
    let facts = Rc::clone(&arch.lazy_string_facts);
    let mut selected = Vec::new();
    for value in constants {
        let above = facts.partition_point(|fact| fact.addr <= value);
        for fact in facts[..above].iter().rev().take(4) {
            if value >= fact.addr && value - fact.addr < u64::from(fact.len) {
                selected.push(*fact);
            }
        }
    }
    selected.sort_by_key(|fact| (fact.addr, fact.priority));
    selected.dedup();
    let Some(space) = fd.get_address().get_space().cloned() else {
        return Ok(0);
    };
    let num_spaces = arch.manage().num_spaces();
    let mut added = 0;
    for fact in selected {
        let addr = Address::new(Rc::clone(&space), fact.addr);
        let occupied = match arch.symboltab.get_global_scope() {
            Some(global) => {
                arch.symboltab.find_function(global, &addr).is_some()
                    || arch.is_lazy_entry(fact.addr)
                    || arch
                        .symboltab
                        .find_container(global, &addr, 1, &Address::new_invalid())
                        .is_some()
            }
            None => false,
        };
        if occupied {
            continue;
        }
        let width = int4::from(fact.char_size);
        let ch = if fact.char_size == 1 {
            arch.types()
                .get_type_char(arch.types().get_size_of_char())?
        } else {
            match arch.types().get_type_char(width) {
                Ok(ch) => ch,
                Err(_) => continue,
            }
        };
        let arr = arch
            .types()
            .get_type_array((fact.len / u32::from(fact.char_size)) as i32, ch)?;
        let name = format!("s_{:x}", fact.addr);
        let (scope, base) = arch
            .symboltab
            .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
        let (sid, _) =
            arch.symboltab
                .add_symbol_mapped(scope, &base, arr, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, varnode_flags::typelock);
        added += 1;
    }
    Ok(added)
}
