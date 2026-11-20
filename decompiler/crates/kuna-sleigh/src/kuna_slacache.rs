//! Decoded `.sla` reuse within a thread, for a process that turns it on.
//!
//! Decoding a `.sla` is most of what building a translator costs, and a
//! process that builds many architectures (the datatest harness, a verifier
//! worker) decodes the same few files over and over. Once [`enable`] has run,
//! the first time a thread sees a `.sla` it decodes as before and remembers
//! only a hash of the bytes; the second time it decodes a template it keeps,
//! and every translator from then on is a [`SleighBase::clone_decoded`] of
//! that template. Entries are keyed by the full `.sla` contents, so a changed
//! file is a new entry and never a stale one. A template costs about as much
//! memory as the translator it copies, so a thread keeps at most a few.

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use kuna_base::error::KunaResult;

use crate::sleighbase::SleighBase;

const CAPACITY: usize = 8;

static ENABLED: AtomicBool = AtomicBool::new(false);

/// Keep decoded `.sla` templates on every thread of this process from now on.
pub fn enable() {
    ENABLED.store(true, Ordering::Relaxed);
}

struct Entry {
    sla: Box<[u8]>,
    core: Rc<SleighBase>,
}

#[derive(Default)]
struct SlaCache {
    seen: Vec<(u64, usize)>,
    entries: Vec<Entry>,
}

thread_local! {
    static CACHE: RefCell<SlaCache> = RefCell::default();
}

/// The template decoded from `sla` when reuse is on and this thread has met
/// the same bytes before, decoding it with `decode` on the second meeting;
/// otherwise `None`.
pub(crate) fn template(
    sla: &[u8],
    decode: impl FnOnce() -> KunaResult<SleighBase>,
) -> KunaResult<Option<Rc<SleighBase>>> {
    if !ENABLED.load(Ordering::Relaxed) {
        return Ok(None);
    }
    let key = {
        let mut hasher = DefaultHasher::new();
        sla.hash(&mut hasher);
        (hasher.finish(), sla.len())
    };
    let first = CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(pos) = cache.entries.iter().position(|e| *e.sla == *sla) {
            let entry = cache.entries.remove(pos);
            let core = Rc::clone(&entry.core);
            cache.entries.push(entry);
            return Err(core);
        }
        if cache.seen.contains(&key) {
            return Ok(false);
        }
        cache.seen.push(key);
        Ok(true)
    });
    match first {
        Err(core) => Ok(Some(core)),
        Ok(true) => Ok(None),
        Ok(false) => {
            let core = Rc::new(decode()?);
            CACHE.with(|cache| {
                let mut cache = cache.borrow_mut();
                if cache.entries.len() >= CAPACITY {
                    cache.entries.remove(0);
                }
                cache.entries.push(Entry {
                    sla: sla.into(),
                    core: Rc::clone(&core),
                });
            });
            Ok(Some(core))
        }
    }
}
