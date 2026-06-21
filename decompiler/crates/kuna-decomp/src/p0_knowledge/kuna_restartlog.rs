//! Port of `decompiler/cpp/kuna_restartlog.{cc,hh}` — restart observability
//! (kuna roadmap item 5).
//!
//! Mechanism (c) of the stage model — global restart with persisted hints — is
//! the single most diagnostic signal in the engine and is otherwise invisible:
//! a function silently decompiles twice and only the persisted Override hints
//! betray why. This component records WHY at each trigger site (dead-definition
//! gate bump, multistage jump-table, late prototype, suppressed bump, lowered
//! switch). Observability only: recording has no effect on decompilation.
//!
//! ## Ownership model vs. the C++ global table
//!
//! The C++ keeps a single file-static `std::map<KunaFuncKey, ...>` keyed by
//! `(const Architecture*, entry space index, entry offset)`, so events survive
//! `Funcdata::clear()`/`clearAnalysis()` (the side table is not part of the
//! Funcdata) and the arch pointer separates functions across loaded programs.
//! A global mutable static keyed by a raw pointer is neither idiomatic nor
//! sound in Rust. The faithful equivalent is a [`RestartLog`] **owned by the
//! `Architecture`** (one log per loaded program — the arch-pointer dimension of
//! the C++ key becomes the owner): it lives outside the `Funcdata`, so it
//! survives function clears exactly as the C++ table does, and the entry key
//! reduces to `(space_index, offset)` ([`KunaFuncKey`]). The per-function event
//! cap, the global (per-log) sequence counter, and the dump format are
//! transcribed verbatim.
//!
//! SEAM(W5): the trigger sites (`Heritage::bumpDeadcodeDelay`,
//! `JumpTable::recoverLabels`, `FuncCallSpecs::deindirect`/`forceSet`) gain
//! their `record(...)` calls when those passes port; this module owns the log
//! and its API, not the call sites.

use crate::funcdata::Funcdata;
use std::collections::BTreeMap;

/// (kuna) Why a function restart (or suppressed restart) was triggered —
/// C++ `KunaRestartReason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KunaRestartReason {
    /// Dead-definition gate bumped (`Override::insertDeadcodeDelay`) — restart.
    /// C++ `krestart_deadcode_bump` (0).
    DeadcodeBump,
    /// Gate bump wanted but a delay was already installed — no restart.
    /// C++ `krestart_deadcode_suppressed` (1).
    DeadcodeSuppressed,
    /// Jump-table size mismatch -> multistage hint — restart.
    /// C++ `krestart_multistage_jump` (2).
    MultistageJump,
    /// Prototype discovered too late at an indirect call — restart.
    /// C++ `krestart_proto_deindirect` (3).
    ProtoDeindirect,
    /// Restrictive prototype forced too late at a call site — restart.
    /// C++ `krestart_proto_forced` (4).
    ProtoForced,
    /// (kuna) Lowered comparison-cascade switch detected late -> restart.
    /// C++ `krestart_lowered_switch` (5).
    LoweredSwitch,
}

impl KunaRestartReason {
    /// Human-readable reason name — byte-for-byte port of
    /// `kunaRestartReasonName` (`kuna_restartlog.cc:46`).
    pub fn name(self) -> &'static str {
        match self {
            KunaRestartReason::DeadcodeBump => {
                "deadcode-delay bump (S3 dead-definition gate, edge 3)"
            }
            KunaRestartReason::DeadcodeSuppressed => {
                "deadcode-delay bump SUPPRESSED (delay already installed)"
            }
            KunaRestartReason::MultistageJump => {
                "multistage jump-table hint (S2 switch model, edge 2)"
            }
            KunaRestartReason::ProtoDeindirect => {
                "prototype discovered late at indirect call (S4, edge 5)"
            }
            KunaRestartReason::ProtoForced => "prototype forced late at call site (S4, edge 5)",
            KunaRestartReason::LoweredSwitch => {
                "lowered comparison-cascade switch (S2 switch model, kuna angr-port)"
            }
        }
    }
}

/// One recorded trigger event — C++ `KunaRestartEvent`.
#[derive(Clone, Debug, PartialEq, Eq)]
struct KunaRestartEvent {
    /// Global (per-log) sequence number ordering events across functions.
    seq: i32,
    /// Why.
    reason: KunaRestartReason,
    /// Site description.
    detail: String,
}

/// Side-table key identifying a function within one `Architecture` — C++
/// `KunaFuncKey` minus the (now owner-implied) arch pointer.
///
/// `Ord` is the transcribed C++ `operator<`: space index then offset (the C++
/// also compares the arch pointer first, which is constant within one owning
/// log, so it drops out).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KunaFuncKey {
    /// Entry address space index (C++ `spaceindex`; -1 for a null space).
    pub space_index: i32,
    /// Entry offset (C++ `offset`).
    pub offset: u64,
}

/// Ring cap per function — C++ `MAX_EVENTS_PER_FUNC`.
const MAX_EVENTS_PER_FUNC: usize = 32;

/// (kuna) The restart-trigger side table for one `Architecture` — the owned
/// equivalent of the C++ file-static `restartTable` + `restartSeq`.
#[derive(Clone, Debug, Default)]
pub struct RestartLog {
    table: BTreeMap<KunaFuncKey, Vec<KunaRestartEvent>>,
    seq: i32,
}

impl RestartLog {
    /// A new, empty restart log.
    pub fn new() -> RestartLog {
        RestartLog::default()
    }

    /// The side-table key for `fd` — C++ `keyForFunc` (`kuna_restartlog.cc:35`).
    ///
    /// `space_index` is the entry address's space index, or -1 if the entry has
    /// no space (the C++ `(entry.getSpace() != 0) ? ->getIndex() : -1`).
    fn key_for_func(fd: &Funcdata) -> KunaFuncKey {
        let entry = fd.get_address();
        let space_index = match entry.get_space() {
            Some(s) => s.get_index(),
            None => -1,
        };
        KunaFuncKey {
            space_index,
            offset: entry.get_offset(),
        }
    }

    /// Record a restart trigger against `fd`'s entry — byte-for-byte port of
    /// `kunaRecordRestart(Funcdata, reason, detail)` (`kuna_restartlog.cc:66`).
    ///
    /// Caps at [`MAX_EVENTS_PER_FUNC`] per function (the earliest events are the
    /// diagnostic ones, so a full bucket is left untouched).
    pub fn record(&mut self, fd: &Funcdata, reason: KunaRestartReason, detail: &str) {
        let key = Self::key_for_func(fd);
        let events = self.table.entry(key).or_default();
        // C++: if ((int4)events.size() >= MAX_EVENTS_PER_FUNC) return;
        if events.len() >= MAX_EVENTS_PER_FUNC {
            return;
        }
        let seq = self.seq;
        self.seq += 1; // C++: ev.seq = restartSeq++;
        events.push(KunaRestartEvent {
            seq,
            reason,
            detail: detail.to_string(),
        });
    }

    /// Record a restart trigger, formatting the given site address as the
    /// detail — port of the `Address`-overload `kunaRecordRestart`
    /// (`kuna_restartlog.cc:79`), which does `site.printRaw(oss)`.
    pub fn record_at(&mut self, fd: &Funcdata, reason: KunaRestartReason, site: &kuna_base::address::Address) {
        let mut detail = String::new();
        // C++: site.printRaw(oss); kunaRecordRestart(fd,reason,oss.str());
        // printRaw cannot fail for a real address; an invalid space prints
        // "invalid_addr". Fall back to empty on the (unreachable) error path.
        let _ = site.print_raw(&mut detail);
        self.record(fd, reason, &detail);
    }

    /// Dump the recorded restart events for `fd` — byte-for-byte port of
    /// `kunaDumpRestarts` (`kuna_restartlog.cc:87`). Returns the exact text the
    /// C++ writes (one trailing newline per line).
    pub fn render(&self, fd: &Funcdata) -> String {
        let mut os = String::new();
        let key = Self::key_for_func(fd);
        match self.table.get(&key) {
            None => {
                // C++: "No restart events recorded for " << name << endl
                os.push_str("No restart events recorded for ");
                os.push_str(fd.get_name());
                os.push('\n');
            }
            Some(events) if events.is_empty() => {
                os.push_str("No restart events recorded for ");
                os.push_str(fd.get_name());
                os.push('\n');
            }
            Some(events) => {
                // C++: "Restart events for " << name << " (mechanism c; ...):" << endl
                os.push_str("Restart events for ");
                os.push_str(fd.get_name());
                os.push_str(" (mechanism c; hints persist in the Override store):\n");
                for (i, ev) in events.iter().enumerate() {
                    // C++: "  " << dec << i << ": " << reasonName
                    os.push_str("  ");
                    os.push_str(&i.to_string());
                    os.push_str(": ");
                    os.push_str(ev.reason.name());
                    // C++: if (!detail.empty()) "  [" << detail << "]"
                    if !ev.detail.is_empty() {
                        os.push_str("  [");
                        os.push_str(&ev.detail);
                        os.push(']');
                    }
                    os.push('\n');
                }
            }
        }
        os
    }

    /// true if no events are recorded for `fd`.
    pub fn is_empty_for(&self, fd: &Funcdata) -> bool {
        let key = Self::key_for_func(fd);
        self.table.get(&key).map(|v| v.is_empty()).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests;
