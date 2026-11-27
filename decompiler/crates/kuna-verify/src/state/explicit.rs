use std::collections::BTreeMap;

use crate::value::Halt;

pub(crate) struct Region {
    pub address: u64,
    pub size: u64,
    pub bytes: Vec<u8>,
    pub permissions: &'static str,
    pub observe: bool,
}

impl Region {
    pub fn end(&self) -> u64 {
        self.address + self.size
    }

    pub fn initial_byte(&self, at: u64) -> u8 {
        self.bytes
            .get((at - self.address) as usize)
            .copied()
            .unwrap_or(0)
    }
}

#[derive(Default)]
pub(crate) struct ExplicitState {
    pub regions: Vec<Region>,
    pub registers: BTreeMap<u64, u8>,
    pub external_entries: BTreeMap<u64, u8>,
}

impl ExplicitState {
    fn region(&self, at: u64) -> Option<&Region> {
        let i = self
            .regions
            .partition_point(|r| r.address <= at)
            .checked_sub(1)?;
        self.regions.get(i).filter(|r| at < r.end())
    }

    pub fn byte(&self, at: u64) -> u8 {
        self.region(at).map_or(0, |r| r.initial_byte(at))
    }

    pub fn observable(&self, at: u64) -> bool {
        self.region(at).is_some_and(|r| r.observe)
    }

    pub fn check(&self, at: u64, size: u64, permission: u8) -> Result<(), Halt> {
        let end = at
            .checked_add(size)
            .ok_or_else(|| Halt::Unsupported("explicit memory access wraps".into()))?;
        let mut cursor = at;
        while cursor < end {
            let region = self
                .region(cursor)
                .filter(|r| r.permissions.as_bytes().contains(&permission));
            let Some(region) = region else {
                return Err(Halt::Trap(format!(
                    "explicit memory access ({}) at {cursor:#x}",
                    char::from(permission)
                )));
            };
            cursor = region.end();
        }
        Ok(())
    }
}
