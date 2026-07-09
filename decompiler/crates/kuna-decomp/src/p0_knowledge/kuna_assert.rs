//! Port of `decompiler/cpp/kuna_assert.{cc,hh}` — the typed assertion API
//! (`docs/history/stage-model.md` section 12).
//!
//! `kassert` is a thin facade over the existing battle-tested stores (Override,
//! Symbol-DB locks, FuncProto locks, the option database): it validates a
//! `<stage> <substage>` pair against the stage registry ([`kuna_phases`]) and
//! dispatches to the store for that sub-stage. What it ADDS is the model —
//! every applied assertion is recorded with its stage, sub-stage, strength, and
//! the **computed minimal rewind scope** (`docs/history/stage-model.md` section 12), which
//! `kassert list` REPORTS (Ghidra-actual re-run remains whole-function).
//!
//! ## What this module owns vs. STUB(W5)
//!
//! The C++ `IfcKunaAssert::execute` is a console command bound to
//! `IfaceDecompData` (the `dcp->conf`/`dcp->fd`, the `Override`/`FuncProto`/
//! symbol-scope stores, and the option database). Those stores and the console
//! `IfaceDecompCommand` machinery are a W5+ surface. This module ports the
//! console-independent core that the command is built on:
//!
//!   - [`KunaAssertion`]: the recorded-assertion struct (the session log entry);
//!   - [`AssertLog`]: the session log (the C++ static `assertLog` vector) with
//!     its `kassert list` rendering;
//!   - [`validate_assertion`]: the stage/sub-stage validation + strength
//!     resolution + the **minimal-rewind computation** the command performs
//!     before it dispatches (the half that is independent of the live stores);
//!   - [`Dispatch`]: the sub-stage -> store routing *decision* (which store a
//!     sub-stage routes to, and whether it is latent/unroutable), transcribed
//!     from the C++ `if/else` chain — the actual store mutation is the W5 stub.
//!
//! STUB(W5): the live dispatch (`Override::insertForceGoto`, `FuncProto`
//! locks, `OptionDatabase::set`, symbol retype/rename) and the console parsing
//! of `<args>` land with the console wave on top of [`Dispatch`].

use crate::kuna_phases::{lookup_subphase, KunaPhase, KunaStrength};
use kuna_base::error::{KunaError, KunaResult};

/// (kuna) One recorded typed assertion (session log entry) — C++ `KunaAssertion`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KunaAssertion {
    /// Function it applies to, or "(global)".
    pub func_name: String,
    /// Stage asserted.
    pub phase: KunaPhase,
    /// Sub-stage name (validated against the catalog).
    pub subphase: String,
    /// The assertion arguments as given.
    pub args: String,
    /// Requested strength (hard/hint).
    pub strength: KunaStrength,
    /// Strength of the underlying mechanism actually used.
    pub applied: KunaStrength,
    /// Computed minimal rewind stage (REPORTED, not enacted).
    pub rewind: KunaPhase,
}

/// Which existing store a sub-stage's assertion routes to — the *decision*
/// half of the C++ `IfcKunaAssert::execute` dispatch chain.
///
/// The C++ command performs the matching store mutation inline; the Rust port
/// separates the routing decision (here, console-independent) from the store
/// mutation (W5). [`Dispatch::for_substage`] reproduces the C++ `if/else` order
/// exactly, so an unknown/unroutable sub-stage yields the same outcome
/// ([`Dispatch::Latent`] vs [`Dispatch::Unroutable`]) the C++ `else` branch
/// produces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dispatch {
    /// -> `Override::insertForceGoto` (S7 edge-virtualization).
    ForceGoto,
    /// -> `Override::insertDeadcodeDelay` (S3 dead-definition-gate).
    DeadcodeDelay,
    /// -> `Override::insertMultistageJump` (S2 switch-model).
    MultistageJump,
    /// -> `Override::insertFlowOverride` (S2 flow-classification).
    FlowOverride,
    /// -> `FuncProto` input/output locks (S4 prototype-source).
    ProtoLock,
    /// -> retype + typelock (S5 type-propagation).
    Retype,
    /// -> `Symbol::setIsolated` (S6 merge-aggressiveness).
    Isolate,
    /// -> rename + namelock (S9 naming-policy).
    Rename,
    /// -> `option <name> ...` (the option database). Carries the option name.
    Option(&'static str),
    /// The sub-stage is LATENT (no override surface exists yet).
    Latent,
    /// The sub-stage is known but not routable through `kassert` (use its
    /// native surface, reported in the sub-stage's `exposure`).
    Unroutable,
}

impl Dispatch {
    /// Route a sub-stage name to its store — verbatim transcription of the C++
    /// `IfcKunaAssert::execute` `if/else` dispatch chain (`kuna_assert.cc:99`),
    /// in the same order. The trailing `else` distinguishes LATENT sub-stages
    /// (`sub->latent`) from merely-unroutable ones, exactly as the C++ does.
    pub fn for_substage(sname: &str, latent: bool) -> Dispatch {
        match sname {
            "edge-virtualization" => Dispatch::ForceGoto,
            "dead-definition-gate" => Dispatch::DeadcodeDelay,
            "switch-model" => Dispatch::MultistageJump,
            "flow-classification" => Dispatch::FlowOverride,
            "prototype-source" => Dispatch::ProtoLock,
            "type-propagation" => Dispatch::Retype,
            "merge-aggressiveness" => Dispatch::Isolate,
            "naming-policy" => Dispatch::Rename,
            "comparison-canonicalization" => Dispatch::Option("compareform"),
            "pointer-notation" => Dispatch::Option("arraynotation"),
            "simplification-quiescence" => Dispatch::Option("togglerule"),
            "readability-rewrites" => Dispatch::Option("analyzeforloops"),
            "extrapop" => Dispatch::Option("extrapop"),
            "pipeline-variant" => Dispatch::Option("setaction"),
            // C++ else branch: LATENT sub-stages report a distinct message from
            // the otherwise-not-routable ones.
            _ => {
                if latent {
                    Dispatch::Latent
                } else {
                    Dispatch::Unroutable
                }
            }
        }
    }
}

/// The result of validating a `kassert <phase> <subphase> [hard|hint]` request,
/// before any store mutation — the console-independent half of
/// `IfcKunaAssert::execute`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedAssertion {
    /// The validated stage.
    pub phase: KunaPhase,
    /// The validated sub-stage name.
    pub subphase: String,
    /// The requested strength (the catalog default, or the trailing hard|hint
    /// override).
    pub requested: KunaStrength,
    /// The strength of the underlying mechanism actually used (the sub-stage's
    /// catalog strength — Ghidra stores are uniformly HARD mechanisms).
    pub applied: KunaStrength,
    /// The computed minimal rewind stage (REPORTED; section 12).
    pub rewind: KunaPhase,
    /// Which store the sub-stage routes to.
    pub dispatch: Dispatch,
}

/// Validate a `kassert` request and compute its strength + minimal rewind —
/// the console-independent core of `IfcKunaAssert::execute`
/// (`kuna_assert.cc:51`), up to (but not including) the live store mutation.
///
/// `stagecode` is the `<phase>` token ("P0".."P9", legacy "S1".."S9" accepted); `subname` is the
/// `<substage>` token; `strength_override` is the optional trailing `hard|hint`
/// (the C++ pops it off the token list). Errors mirror the C++ `IfaceParseError`
/// throws (as [`KunaError::Parse`]), in the same precedence order:
///
///   1. bad stage code,
///   2. missing sub-stage name,
///   3. unknown sub-stage,
///   4. sub-stage belongs to a different stage.
///
/// The **minimal-rewind computation** is the sub-stage's catalog `rewind`
/// field (`docs/history/stage-model.md` section 12: format->P9, force-goto->P7, typelock->P5,
/// proto->P4, jumptable/flow/context->P2, deadcode-delay->P3), looked up here.
pub fn validate_assertion(
    stagecode: &str,
    subname: &str,
    strength_override: Option<KunaStrength>,
) -> KunaResult<ValidatedAssertion> {
    let phase = KunaPhase::from_code(stagecode).ok_or_else(|| KunaError::Parse {
        explain: format!("Bad phase code (expecting P0..P9): {stagecode}"),
    })?;
    if subname.is_empty() {
        return Err(KunaError::Parse {
            explain: "Missing sub-phase name (see `phase list`)".to_string(),
        });
    }
    let sub = lookup_subphase(subname).ok_or_else(|| KunaError::Parse {
        explain: format!("Unknown sub-phase: {subname} (see `phase list`)"),
    })?;
    if sub.phase != phase {
        return Err(KunaError::Parse {
            explain: format!(
                "Sub-phase {subname} belongs to phase {}, not {stagecode}",
                sub.phase.code()
            ),
        });
    }

    let requested = strength_override.unwrap_or(sub.strength);

    Ok(ValidatedAssertion {
        phase,
        subphase: subname.to_string(),
        requested,
        applied: sub.strength,
        rewind: sub.rewind,
        dispatch: Dispatch::for_substage(subname, sub.latent),
    })
}

/// The session log of applied assertions — the C++ static `assertLog` vector,
/// reified so it can be owned by the console state (console-lifetime; survives
/// function clears like the Override store does).
#[derive(Clone, Debug, Default)]
pub struct AssertLog {
    entries: Vec<KunaAssertion>,
}

impl AssertLog {
    /// A new, empty log.
    pub fn new() -> AssertLog {
        AssertLog::default()
    }

    /// Record an applied assertion (C++ `assertLog.push_back(rec)`).
    pub fn push(&mut self, rec: KunaAssertion) {
        self.entries.push(rec);
    }

    /// The recorded assertions, in insertion order.
    pub fn entries(&self) -> &[KunaAssertion] {
        &self.entries
    }

    /// true if no assertions are recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Render `kassert list` — byte-for-byte port of
    /// `IfcKunaAssert::listAssertions` (`kuna_assert.cc:249`). Returns the exact
    /// text the C++ writes to the bulk stream (one trailing newline per line).
    pub fn render_list(&self) -> String {
        let mut os = String::new();
        if self.entries.is_empty() {
            os.push_str("No kuna assertions recorded this session\n");
            return os;
        }
        os.push_str(
            "Recorded assertions (rewind scope is REPORTED per stage-model.md s12; \
             Ghidra-actual re-run remains whole-function):\n",
        );
        for (i, rec) in self.entries.iter().enumerate() {
            os.push_str("  ");
            os.push_str(&i.to_string());
            os.push_str(": [");
            os.push_str(&rec.func_name);
            os.push_str("] ");
            os.push_str(rec.phase.code());
            os.push('/');
            os.push_str(&rec.subphase);
            if !rec.args.is_empty() {
                os.push(' ');
                os.push_str(&rec.args);
            }
            os.push_str("  strength=");
            os.push_str(match rec.strength {
                KunaStrength::Hard => "HARD",
                KunaStrength::Hint => "HINT",
                KunaStrength::None => "-",
            });
            if rec.strength == KunaStrength::Hint && rec.applied == KunaStrength::Hard {
                os.push_str(" (applied via HARD mechanism)");
            }
            os.push_str("  rewind->");
            os.push_str(rec.rewind.code());
            os.push('\n');
        }
        os
    }
}

#[cfg(test)]
mod tests;
