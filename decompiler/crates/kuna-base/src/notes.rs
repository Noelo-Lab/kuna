//! Say a loader note once per process, however often the image is loaded.
//!
//! (kuna) The loader's tolerance notes — a section table it recovered from, a
//! data directory it clamped, an ET_REL layout it reports — are facts about the
//! *image*, not about a load. A surface that loads the same image twice in one
//! process (`kuna disassemble` answers a caller-bounded window without the
//! discovery walk and reloads with it when only the walk can answer) would
//! otherwise announce each of them twice, which reads as two different findings.
//!
//! Keyed by the image the note is about, so two images whose notes happen to
//! read alike each get theirs.

use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};

fn said() -> &'static Mutex<BTreeSet<String>> {
    static SAID: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();
    SAID.get_or_init(|| Mutex::new(BTreeSet::new()))
}

/// Print `message` on stderr the first time this process says it about `about`.
pub fn say_once(about: &str, message: &str) {
    let key = format!("{about}\u{1}{message}");
    let fresh = match said().lock() {
        Ok(mut set) => set.insert(key),
        // A poisoned set is not a reason to swallow a diagnostic.
        Err(_) => true,
    };
    if fresh {
        eprintln!("{message}");
    }
}
