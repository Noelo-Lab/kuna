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

pub type Probe = dyn Fn(&[u64], bool) -> Vec<Fact>;

/// Facts sorted by address, with the running maximum end so a containment query
/// visits every fact holding a value, however many shorter facts start between.
#[derive(Debug, Default)]
pub struct Facts {
    facts: Vec<Fact>,
    reach: Vec<u64>,
}

impl Facts {
    pub fn new(mut facts: Vec<Fact>) -> Self {
        facts.sort_by_key(|fact| (fact.addr, fact.priority));
        facts.dedup();
        let mut end = 0;
        let reach = facts
            .iter()
            .map(|fact| {
                end = end.max(fact.addr.saturating_add(u64::from(fact.len)));
                end
            })
            .collect();
        Self { facts, reach }
    }

    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }

    pub fn starts_at(&self, vma: u64) -> bool {
        self.facts.binary_search_by_key(&vma, |fact| fact.addr).is_ok()
    }

    fn containing(&self, value: u64) -> impl Iterator<Item = &Fact> {
        let above = self.facts.partition_point(|fact| fact.addr <= value);
        (0..above)
            .rev()
            .take_while(move |&i| self.reach[i] > value)
            .map(move |i| &self.facts[i])
            .filter(move |fact| value - fact.addr < u64::from(fact.len))
    }
}

pub fn materialize(arch: &mut Architecture, fd: &Funcdata) -> KunaResult<usize> {
    if arch.lazy_string_facts.is_empty() && arch.lazy_string_probe.is_none() {
        return Ok(0);
    }
    let code = fd.get_address().get_space().map(|space| space.get_index());
    let mut values = Vec::new();
    for vn in fd.vbank().iter_loc().filter_map(|id| fd.vbank().get(id)) {
        if vn.is_constant() {
            values.push(vn.get_offset());
        } else if code.is_some() && vn.get_addr().get_space().map(|space| space.get_index()) == code {
            let last = u64::try_from(vn.get_size().max(1) - 1).unwrap_or(0);
            values.push(vn.get_offset());
            values.push(vn.get_offset().saturating_add(last));
        }
    }
    values.sort_unstable();
    values.dedup();
    let facts = Rc::clone(&arch.lazy_string_facts);
    let mut selected = Vec::new();
    for &value in &values {
        selected.extend(facts.containing(value).copied());
    }
    if let Some(probe) = &arch.lazy_string_probe {
        selected.extend(probe(&values, arch.analysis_widestrings));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(addr: u64, len: u32, priority: u8) -> Fact {
        Fact { addr, len, char_size: 1, priority }
    }

    #[test]
    fn containment_reaches_a_long_fact_past_shorter_nested_ones() {
        let blob = fact(0x1000, 0x400, 3);
        let mut all = vec![blob];
        all.extend((0..8).map(|i| fact(0x1010 + i * 0x20, 8, 0)));
        let facts = Facts::new(all);
        let held: Vec<_> = facts.containing(0x1300).copied().collect();
        assert_eq!(held, vec![blob]);
        let held: Vec<_> = facts.containing(0x1012).copied().collect();
        assert_eq!(held, vec![fact(0x1010, 8, 0), blob]);
        assert_eq!(facts.containing(0x1400).count(), 0);
        assert!(facts.starts_at(0x1030) && !facts.starts_at(0x1031));
    }
}
