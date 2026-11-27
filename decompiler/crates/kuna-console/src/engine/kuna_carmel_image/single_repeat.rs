//! The direct DATA-count REP resumes after three issues, but its end is a fetch address.

use super::{FETCH, Fetch};

pub(super) const DIRECT: &str = "carmel_direct_repeat";

pub(super) struct DirectRepeat(u8);

impl DirectRepeat {
    pub(super) fn parse(fetch: &Fetch) -> Option<Self> {
        (fetch[0] == 0xb8 && fetch[3] == 0xcf && fetch[4] & 0xc7 == 0 && fetch[5] == 0)
            .then(|| Self(fetch[4] >> 3))
    }

    /// Zero is reserved for an unresolved layout. Value one has no backedge;
    /// larger values are one plus the number of replayed issues.
    pub(super) fn plan(&self, start: u64, issue_size: impl Fn(u64) -> Option<u64>) -> Option<u32> {
        let mut first = start + FETCH;
        for _ in 0..3 {
            first += issue_size(first)?;
        }
        let end = start + (u64::from(self.0) + 1) * FETCH;
        if self.0 < 2 || end < first {
            issue_size(first)?;
            return Some(1);
        }
        let mut pc = first;
        let mut body = 1;
        while pc <= end {
            pc += issue_size(pc)?;
            body += 1;
        }
        Some(body)
    }
}
