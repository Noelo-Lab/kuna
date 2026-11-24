//! Qualify one conditional-return schedule without assuming a dynamic stack link.

use kuna_base::error::KunaResult;

use super::{FETCH, Fetch};

pub(super) const CONTEXT: &str = "carmel_conditional_return_safe";
const COMPARE: Fetch = [0x20, 0x9a, 0x02, 0x2b, 0xec, 0];
const INCREMENT: Fetch = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];

pub(super) struct ReturnDelay(u64);

impl ReturnDelay {
    pub(super) fn parse(fetch: &Fetch) -> Option<Self> {
        match *fetch {
            [0x2b, 0xe0, d, 0x2b, 0xec, 0] if d & 0xf8 == 0x70 => Some(Self(u64::from((d & 7).min(6)))),
            _ => None,
        }
    }

    pub(super) fn admissible(
        self,
        start: u64,
        mut fetch_at: impl FnMut(u64) -> KunaResult<Option<Fetch>>,
    ) -> KunaResult<bool> {
        let Some(previous) = start.checked_sub(FETCH) else { return Ok(false); };
        // A layout witness, not a condition value: an alternate CFG entry
        // may bypass the compare. The root still samples incoming EST.
        if fetch_at(previous)? != Some(COMPARE) {
            return Ok(false);
        }
        for i in 1..=self.0 {
            if fetch_at(start + FETCH * i)? != Some(INCREMENT) {
                return Ok(false);
            }
        }
        Ok(fetch_at(start + FETCH * (self.0 + 1))?.is_some_and(|f| (f[0] | f[3]) & 0x80 == 0))
    }
}
