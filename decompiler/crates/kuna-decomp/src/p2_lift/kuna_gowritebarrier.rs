//! **gowritebarrier** — fold the Go GC write-barrier diamond
//! (`docs/multilang.md` W2/W5, the second entry-guard idiom).
//!
//! ```text
//!   cmp  $0, runtime.writeBarrier     ;                 -->  *p = v;
//!   jz   store                        ;                      head = n;
//!   call runtime.gcWriteBarrier4      ; record old/new
//!   mov  [r11], ...                   ; in the GC buffer
//!   store: mov [p], v ...             ; the real stores
//! ```
//!
//! Every pointer store the garbage collector must see compiles to a flag-test
//! diamond: when `runtime.writeBarrier` is set, a `runtime.gcWriteBarrier<N>`
//! thunk hands back a buffer (R11 on amd64) and the cold arm records the old
//! and new pointer values in it; both paths then perform the program's own
//! stores at the join. The cold arm is GC bookkeeping with no bearing on what
//! the function computes — and it is poison for the output on top of noise:
//! the thunk's buffer register is not a modeled return, so the recording
//! stores read an undefined local, and on measured functions the value split
//! even corrupts the *real* store's address expression. Folding the diamond
//! leaves exactly the program's stores.
//!
//! # Matching, and the same name-in-shape rule as `gomorestack`
//!
//! The shape is [`crate::idiom::match_guarded_cold_call`] — a one-block cold
//! arm holding a single call, whose guard rejoins the hot path in one hop —
//! with the callee held to [`is_write_barrier_name`]: `runtime.gcWriteBarrier`
//! and its numbered `1`–`8` variants, plus the bare assembly thunk
//! `gcWriteBarrier` (its pclntab name carries no package path). Those names
//! exist only in the Go runtime, and they are only consulted *inside* the
//! shape, so an ordinary call in straight-line flow is never touched. The
//! `runtime.wbBufFlush*` family is deliberately not matched: user code never
//! calls it inline — it is the thunks' own slow path.
//!
//! # The edit and the window
//!
//! One `remove_branch` on the guard's cold edge plus the unreachable sweep,
//! **pre-SSA** (the `cleanupcode`/`gomorestack` window, and this action is
//! registered directly after `gomorestack`): no INDIRECT effects to unpick,
//! no MULTIEQUAL to patch, and the loads that existed only to feed the GC
//! buffer lose their readers and die to the ordinary dead-code fixpoint. The
//! real stores live at the join, which both arms reached — nothing the
//! program performs is deleted.
//!
//! # The option
//!
//! `option gowritebarrier off|auto|always`, default **auto** — act only when
//! the image's source language is Go (the `sourcelang` fact), a structural
//! no-op everywhere else; `always` drops the language test (no-signal
//! corpora); `off` keeps the barrier for auditing GC behavior. Destructive by
//! declared intent: it deletes a real (runtime-bookkeeping) call and branch.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::idiom::match_guarded_cold_call;

/// The three-valued `gowritebarrier` gate, stored on the `Architecture` as a
/// `u8` (the `gomorestack`/`rustabi` pattern).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GoWriteBarrierMode {
    /// Keep the write-barrier diamond.
    Off,
    /// Fold it when the image's source language is Go (the shipped default).
    #[default]
    Auto,
    /// Fold it whatever the detected language (no-signal corpora).
    Always,
}

impl GoWriteBarrierMode {
    /// The catalog `current` token.
    pub fn as_str(self) -> &'static str {
        match self {
            GoWriteBarrierMode::Off => "off",
            GoWriteBarrierMode::Auto => "auto",
            GoWriteBarrierMode::Always => "always",
        }
    }

    /// Stored form for the `Architecture` field.
    pub fn as_u8(self) -> u8 {
        match self {
            GoWriteBarrierMode::Off => 0,
            GoWriteBarrierMode::Auto => 1,
            GoWriteBarrierMode::Always => 2,
        }
    }

    /// Inverse of [`GoWriteBarrierMode::as_u8`] (unknown values read as `Off`).
    pub fn from_u8(v: u8) -> GoWriteBarrierMode {
        match v {
            1 => GoWriteBarrierMode::Auto,
            2 => GoWriteBarrierMode::Always,
            _ => GoWriteBarrierMode::Off,
        }
    }
}

/// Parse an `option gowritebarrier` value.
pub fn parse_gowritebarrier_mode(value: &str) -> Result<(GoWriteBarrierMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => GoWriteBarrierMode::Off,
        "auto" | "" => GoWriteBarrierMode::Auto,
        "always" | "on" | "1" | "true" => GoWriteBarrierMode::Always,
        other => {
            return Err(format!(
                "unknown gowritebarrier value {other:?} (expected off, auto or always)"
            ))
        }
    };
    Ok((mode, format!("Go write-barrier folding set to {}", mode.as_str())))
}

/// Is `name` a Go GC write-barrier thunk?
///
/// `runtime.gcWriteBarrier` plus its numbered and `.abi0` spellings, and the
/// bare `gcWriteBarrier` assembly symbol (the generic thunk's pclntab name has
/// no package path). The *shape* requirement is what keeps a coincidental
/// name in ordinary flow from cutting anything.
pub fn is_write_barrier_name(name: &str) -> bool {
    let rest = match name.strip_prefix("runtime.gcWriteBarrier") {
        Some(rest) => rest,
        None => match name.strip_prefix("gcWriteBarrier") {
            Some(rest) => rest,
            None => return false,
        },
    };
    rest.is_empty()
        || rest.starts_with('.')
        || (rest.len() == 1 && rest.bytes().next().is_some_and(|b| b.is_ascii_digit()))
        || (rest.len() >= 2
            && rest.bytes().next().is_some_and(|b| b.is_ascii_digit())
            && rest[1..].starts_with('.'))
}

/// Is the fold live for this function?
pub fn live(data: &Funcdata) -> bool {
    match GoWriteBarrierMode::from_u8(data.get_arch().go_write_barrier) {
        GoWriteBarrierMode::Off => false,
        GoWriteBarrierMode::Auto => data.get_arch().source_lang.is_go(),
        GoWriteBarrierMode::Always => true,
    }
}

/// (kuna) `ActionGoWriteBarrier` — fold the Go GC write-barrier diamond
/// (option `gowritebarrier`, default auto).
pub struct ActionGoWriteBarrier {
    base: ActionBase,
}

impl ActionGoWriteBarrier {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionGoWriteBarrier { base: ActionBase::new(0, "gowritebarrier", g) })
    }
}

impl Action for ActionGoWriteBarrier {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionGoWriteBarrier { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !live(data) {
            return 0;
        }
        // The pre-SSA window; a function carries one diamond per guarded
        // pointer-store group, so loop and re-match after every edit.
        if data.get_heritage_pass() != 0 || data.num_calls() == 0 {
            return 0;
        }
        let mut changed = 0;
        while let Some(m) = match_guarded_cold_call(data, &is_write_barrier_name) {
            data.remove_branch(m.guard_block, m.guard_edge)
                .expect("ActionGoWriteBarrier: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionGoWriteBarrier: removeUnreachableBlocks");
            self.base.count += 1;
            changed = 1;
        }
        changed
    }
}

#[cfg(test)]
#[path = "kuna_gowritebarrier/tests.rs"]
mod tests;
