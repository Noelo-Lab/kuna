//! Reads of incoming stack argument bytes before the callee overwrites them.

use std::collections::BTreeMap;

use crate::calls::Site;
use crate::state::Env;

#[derive(Clone, Copy, Debug)]
enum Access {
    Pending,
    Written,
    Read(u8),
}

#[derive(Clone, Debug)]
pub(crate) struct StackInputs {
    base: u64,
    bytes: BTreeMap<u64, Access>,
}

impl StackInputs {
    pub(crate) fn new(env: &Env, site: &Site, base: u64) -> Option<Self> {
        if env.ret_slot != 4 || site.resolved || !env.in_stack(base, 4) {
            return None;
        }
        let mut bytes = BTreeMap::new();
        for st in (0..site.input_storage.len()).filter_map(|i| site.stack_input(i)) {
            let size = u64::from(st.size);
            let addr = base + st.off;
            let last = addr + size - 1;
            if !env.in_stack(addr, size) || last != env.ram.wrap_offset(last) {
                continue;
            }
            for off in st.off..st.off + size {
                if bytes.insert(off, Access::Pending).is_some() {
                    return None;
                }
            }
        }
        (!bytes.is_empty()).then_some(Self { base, bytes })
    }

    pub(crate) fn read(&mut self, addr: u64, value: u8) {
        if let Some(access @ Access::Pending) = self.bytes.get_mut(&addr.wrapping_sub(self.base)) {
            *access = Access::Read(value);
        }
    }

    pub(crate) fn write(&mut self, addr: u64) {
        if let Some(access @ Access::Pending) = self.bytes.get_mut(&addr.wrapping_sub(self.base)) {
            *access = Access::Written;
        }
    }

    pub(crate) fn contains(&self, off: u64, size: u32) -> bool {
        (off..off + u64::from(size)).all(|off| self.bytes.contains_key(&off))
    }

    pub(crate) fn bytes(&self) -> impl Iterator<Item = (u64, Option<u8>)> + '_ {
        self.bytes.iter().map(|(&off, access)| {
            (off, if let Access::Read(value) = access { Some(*value) } else { None })
        })
    }
}
