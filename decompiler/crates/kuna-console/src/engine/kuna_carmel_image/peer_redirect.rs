//! Admit a compact dual redirect only when its speculative reads and delay
//! peers stay within the witnessed, side-effect-bounded geometry.

use kuna_base::error::KunaResult;

use super::{Fetch, FETCH, SLOT};

pub(super) const CONTEXT: &str = "carmel_peer_redirect_safe";
const NOP: u32 = 0x2bec00;
const SEQUENTIAL: u32 = 0x800000;

pub(super) struct Redirect {
    deltas: [u64; 2],
}

impl Redirect {
    pub(super) fn parse(fetch: &Fetch) -> Option<Self> {
        if fetch[0] != 0x28 || fetch[3] != 0x2b {
            return None;
        }
        let (left, right) = slots(fetch);
        if left & 0xff8000 != 0x280000 || right & 0xffe001 != 0x2b0000
            || (left >> 9) & 0x3f > 1 || left & 3 > 2
        {
            return None;
        }
        let a = (left >> 2) & 0x7f;
        let b = (right >> 3) & 0x3ff;
        // Both signed displacements must be forward, even, and past the
        // maximum three-fetch peer extent. Coincident targets are unproved.
        if !(8..64).contains(&a) || !(8..512).contains(&b) || (a | b) & 1 != 0 || a == b {
            return None;
        }
        Some(Self { deltas: [SLOT * u64::from(a), SLOT * u64::from(b)] })
    }

    pub(super) fn admissible(
        self,
        start: u64,
        mut fetch_at: impl FnMut(u64) -> KunaResult<Option<Fetch>>,
    ) -> KunaResult<bool> {
        let mut written = 0;
        for i in 1..=3 {
            let Some(fetch) = fetch_at(start + FETCH * i)? else { return Ok(false); };
            let Some(effects) = bounded_effects(&fetch, i == 1) else { return Ok(false); };
            if written & effects != 0 {
                return Ok(false);
            }
            written |= effects;
        }
        for delta in self.deltas {
            let Some(fetch) = fetch_at(start + delta)? else { return Ok(false); };
            if bounded_effects(&fetch, false).is_none() {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn slots(fetch: &Fetch) -> (u32, u32) {
    let &[a, b, c, d, e, f] = fetch;
    (u32::from_be_bytes([0, a, b, c]), u32::from_be_bytes([0, d, e, f]))
}

fn writes(word: u32) -> Option<u8> {
    match word {
        NOP => Some(0),
        0x21ac20 => Some(1 << 0),
        0x21ac62 => Some(1 << 1),
        0x21aca4 => Some(1 << 2),
        0x21ace6 => Some(1 << 3),
        0x21ad28 => Some(1 << 4),
        _ => None,
    }
}

fn bounded_effects(fetch: &Fetch, allow_sequential: bool) -> Option<u8> {
    let (left, right) = slots(fetch);
    if allow_sequential && left & SEQUENTIAL != 0 {
        let lhs = writes(left & !SEQUENTIAL)?;
        let rhs = writes(right)?;
        (lhs & rhs == 0).then_some(lhs | rhs)
    } else if right == NOP {
        writes(left)
    } else {
        None
    }
}
