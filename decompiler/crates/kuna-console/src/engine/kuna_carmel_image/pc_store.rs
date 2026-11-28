//! Resolve the compact PC store's physical fetch position before lifting.

use kuna_base::error::KunaResult;

use super::{FETCH, Fetch, Issue, SLOT};

pub(super) const CONTEXT: &str = "carmel_pc_store_delta";
const PC_STORE: u32 = 0x31e520;
const NOP: u32 = 0x2bec00;
const SEQUENTIAL: u32 = 0x800000;

pub(super) struct PcStore {
    branch_words: u32,
}

impl PcStore {
    pub(super) fn parse(issue: Issue, fetch: &Fetch) -> Option<(u64, Self)> {
        if fetch[0] & 0x7f != 0x31 && fetch[3] != 0x31 { return None; }
        let (left, right) = slots(fetch);
        match issue {
            Issue::SeqLeft if left & !SEQUENTIAL == PC_STORE => Some((0, Self { branch_words: 0 })),
            Issue::SeqLeft if left == (NOP | SEQUENTIAL) && right == PC_STORE => {
                Some((SLOT, Self { branch_words: 0 }))
            }
            Issue::Parallel if (left, right) == (NOP, PC_STORE) || (left, right) == (PC_STORE, NOP) => {
                Some((0, Self { branch_words: 0 }))
            }
            Issue::Parallel if left & 0xffe007 == 0x2b0000 && right == PC_STORE => {
                let branch_words = (left >> 3) & 0x3ff;
                (branch_words >= 8 && branch_words < 512 && branch_words % 2 == 0)
                    .then_some((0, Self { branch_words }))
            }
            _ => None,
        }
    }

    pub(super) fn plan(
        self, start: u64, issue_size: impl Fn(u64) -> Option<u64>,
        mut fetch_at: impl FnMut(u64) -> KunaResult<Option<Fetch>>,
    ) -> KunaResult<Option<u32>> {
        let mut at = start;
        let mut checked: Option<(u64, Fetch)> = None;
        for _ in 0..3 {
            let Some(size) = issue_size(at) else { return Ok(None); };
            at += size;
            let fetch = at / FETCH * FETCH;
            if checked.as_ref().is_none_or(|(address, _)| *address != fetch) {
                let Some(raw) = fetch_at(fetch)? else { return Ok(None); };
                checked = Some((fetch, raw));
            }
            if !harmless(&checked.as_ref().unwrap().1, at != fetch) { return Ok(None); }
        }
        if self.branch_words != 0 {
            let target = start + u64::from(self.branch_words) * SLOT;
            if target <= at || !fetch_at(target)?.is_some_and(|raw| harmless(&raw, false)) {
                return Ok(None);
            }
        }
        Ok(Some(((at - start) / SLOT) as u32))
    }
}

fn slots(fetch: &Fetch) -> (u32, u32) {
    let &[a, b, c, d, e, f] = fetch;
    (u32::from_be_bytes([0, a, b, c]), u32::from_be_bytes([0, d, e, f]))
}

fn harmless(fetch: &Fetch, right_slot: bool) -> bool {
    let (left, right) = slots(fetch);
    let register_only = |word| matches!(word, NOP | 0x21ac20 | 0x21ac62 | 0x21aca4 | 0x21ace6 | 0x21ad28 | 0x21ad6a);
    if left & SEQUENTIAL != 0 {
        register_only(if right_slot { right } else { left & !SEQUENTIAL })
    } else {
        (left == NOP && register_only(right)) || (right == NOP && register_only(left))
    }
}
