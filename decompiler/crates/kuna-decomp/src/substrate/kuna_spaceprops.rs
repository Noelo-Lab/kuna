//! Per address space answers to the two range lookups the global scope makes
//! for every new Varnode: the boolean-property map and the owned ranges of a
//! [`GlobalQuery`](crate::context::GlobalQuery).
//!
//! The property map is a partition keyed by address, ordered by space index
//! first, so a space holding no split point of its own reads one value at
//! every address: that of the last split point below the space. A space
//! holding no owned range is owned at no address. Registers, temporaries and
//! the stack are such spaces in every binary, and their Varnodes are most of
//! a function's; they are answered from a table built the first time each
//! space is asked about, without the two tree searches.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::partmap::PartMap;
use kuna_base::types::{int4, uint4};

#[derive(Debug, Clone, Copy)]
struct Space {
    /// The property value of every address, when the space has no split point.
    flags: Option<uint4>,
    /// Does any owned range lie in the space?
    owns: bool,
}

/// The table, indexed by address-space index. It describes the maps it was
/// built from, so whoever changes them must [`clear`](SpaceProps::clear) it.
#[derive(Debug, Clone, Default)]
pub(crate) struct SpaceProps(RefCell<Vec<Option<Space>>>);

impl SpaceProps {
    fn space(&self, addr: &Address, flagbase: &PartMap<Address, uint4>, owned: &RangeList) -> Option<Space> {
        let spc = addr.get_space()?;
        let index = usize::try_from(spc.get_index()).ok()?;
        let mut table = self.0.borrow_mut();
        if table.len() <= index {
            table.resize(index + 1, None);
        }
        if let Some(space) = table[index] {
            return Some(space);
        }
        let in_space = |a: &Address| a.get_space().is_some_and(|s| s.get_index() == spc.get_index());
        let flags = (!flagbase.iter().any(|(k, _)| in_space(k)))
            .then(|| *flagbase.get_value(&Address::new(Rc::clone(spc), 0)));
        let owns = owned.iter().any(|r| r.get_space().get_index() == spc.get_index());
        let space = Space { flags, owns };
        table[index] = Some(space);
        Some(space)
    }

    /// `flagbase.get_value(addr)`.
    pub(crate) fn property(&self, addr: &Address, flagbase: &PartMap<Address, uint4>, owned: &RangeList) -> uint4 {
        match self.space(addr, flagbase, owned).and_then(|s| s.flags) {
            Some(flags) => flags,
            None => *flagbase.get_value(addr),
        }
    }

    /// `owned.in_range(addr, size)`.
    pub(crate) fn in_owned(&self, addr: &Address, size: int4, flagbase: &PartMap<Address, uint4>, owned: &RangeList) -> bool {
        if self.space(addr, flagbase, owned).is_some_and(|s| !s.owns) {
            return false;
        }
        owned.in_range(addr, size)
    }

    /// Forget every space, for maps that are about to change.
    pub(crate) fn clear(&mut self) {
        self.0.get_mut().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::address::mach_extreme;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

    fn space(name: &str, index: int4) -> AddrSpace {
        AddrSpace::new(spacetype::IPTR_PROCESSOR, name, false, 4, 1, index, addrspace_flags::hasphysical, 1, 1)
    }

    /// Random property maps and owned ranges over five spaces, split points at
    /// space starts, ends and the sentinels included, answer every address of
    /// every space as the two maps themselves do.
    #[test]
    fn space_answers_agree_with_the_maps() {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        for (i, name) in ["ram", "register", "stack", "other"].iter().enumerate() {
            m.insert_space(Rc::new(space(name, 2 + i as int4))).unwrap();
        }
        let spaces: Vec<Rc<AddrSpace>> = (1..6).map(|i| Rc::clone(m.get_space(i).unwrap())).collect();
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let (mut held, mut looked) = (0, 0);
        for _ in 0..400 {
            let mut flagbase: PartMap<Address, uint4> = PartMap::new((next() % 4) as uint4);
            let mut owned = RangeList::default();
            for _ in 0..next() % 6 {
                let spc = &spaces[(next() % 5) as usize];
                let at = match next() % 4 {
                    0 => Address::new(Rc::clone(spc), 0),
                    1 => Address::new(Rc::clone(spc), 0xffff_ffff),
                    2 if next() % 3 == 0 => Address::new_extreme(mach_extreme::m_maximal),
                    _ => Address::new(Rc::clone(spc), next() % 0x40),
                };
                *flagbase.split(&at) = (next() % 16) as uint4;
            }
            for _ in 0..next() % 4 {
                let spc = &spaces[(next() % 5) as usize];
                let first = next() % 0x40;
                owned.insert_range(Rc::clone(spc), first, first + next() % 0x10);
            }
            let props = SpaceProps::default();
            for _ in 0..60 {
                let spc = &spaces[(next() % 5) as usize];
                let addr = Address::new(Rc::clone(spc), if next() % 8 == 0 { 0xffff_fff0 } else { next() % 0x50 });
                let size = (next() % 8 + 1) as int4;
                if props.space(&addr, &flagbase, &owned).is_some_and(|s| s.flags.is_some() && !s.owns) {
                    held += 1;
                } else {
                    looked += 1;
                }
                assert_eq!(props.property(&addr, &flagbase, &owned), *flagbase.get_value(&addr));
                assert_eq!(props.in_owned(&addr, size, &flagbase, &owned), owned.in_range(&addr, size));
            }
            let invalid = Address::new_invalid();
            assert_eq!(props.in_owned(&invalid, 4, &flagbase, &owned), owned.in_range(&invalid, 4));
        }
        assert!(held > 1_000 && looked > 1_000, "held {held} looked {looked}");
    }
}
