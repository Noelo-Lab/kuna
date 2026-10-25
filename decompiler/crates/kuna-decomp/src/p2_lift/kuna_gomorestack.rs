//! **gomorestack** — fold the Go stack-growth prologue
//! (`docs/multilang.md` W2/W5, the Go pack's entry-guard idiom).
//!
//! ```text
//!   func:  cmp  0x10(%r14),%rsp     ; SP vs g.stackguard0
//!          jbe  grow                ;
//!   body:  ...                      ;      -->  body: ...
//!          ret                      ;            ret
//!   grow:  call runtime.morestack_noctxt
//!          jmp  func                ; re-run the entry on the grown stack
//! ```
//!
//! Every non-leaf function the Go compiler emits opens with this guard: if the
//! goroutine's stack cannot hold the frame, call into the runtime to grow it,
//! then **loop back to the function entry** and run the prologue again. The
//! check is pure runtime machinery — the function's own semantics are the
//! fall-through path — but to a decompiler the back-edge is poison, not just
//! noise: the entry becomes a loop head, so every live argument register
//! reaches a join of its incoming value with its after-growth value, and
//! parameter recovery sees phi-fed arguments on **every** non-leaf Go
//! function. Folding the guard restores straight-line entry flow.
//!
//! # Matching, and why a name alone is not enough
//!
//! The shape is [`crate::idiom::match_entry_guard_loop`] — the tight
//! guard/grow two-block cycle sitting on the inserted entry block — with the
//! cold-arm callee required to be a `runtime.morestack*` routine
//! ([`is_morestack_name`]: `runtime.morestack`, `runtime.morestack_noctxt`,
//! their `.abi0` spellings). Those names exist only in the Go runtime, and the
//! name is only accepted *in the shape*, so a C function calling something it
//! named `runtime.morestack` in straight-line code is left alone. The names
//! come from the pclntab (stripped) or the symbol table (unstripped), so a
//! stripped Go binary folds too.
//!
//! # The edit, and the window
//!
//! One `remove_branch` on the guard's cold edge plus the unreachable-block
//! sweep — the `securitycheck`/`stackguard` primitives — performed **pre-SSA**
//! (`get_heritage_pass() == 0`, the `cleanupcode`/`loweredswitch` window), so
//! the entry phi never forms at all and the guard's compare/load die to the
//! ordinary dead-code fixpoint.
//!
//! # The option
//!
//! `option gomorestack off|auto|always`, default **auto** — act only when the
//! image's source language is Go ([`crate::context::ArchContext::source_lang`],
//! the `sourcelang` fact), which makes the default a structural no-op on every
//! non-Go binary; `always` drops the language test (for corpora with no
//! detection signal, such as a stage-test bytechunk), `off` keeps the guard
//! for auditing. Destructive by declared intent: it deletes a real (runtime)
//! call and a real branch.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::idiom::match_entry_guard_loop;

/// The three-valued `gomorestack` gate, stored on the `Architecture` as a `u8`
/// (the `rustabi` pattern).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GoMorestackMode {
    /// Keep the stack-growth guard.
    Off,
    /// Fold it when the image's source language is Go (the shipped default).
    #[default]
    Auto,
    /// Fold it whatever the detected language (no-signal corpora).
    Always,
}

impl GoMorestackMode {
    /// The catalog `current` token.
    pub fn as_str(self) -> &'static str {
        match self {
            GoMorestackMode::Off => "off",
            GoMorestackMode::Auto => "auto",
            GoMorestackMode::Always => "always",
        }
    }

    /// Stored form for the `Architecture` field.
    pub fn as_u8(self) -> u8 {
        match self {
            GoMorestackMode::Off => 0,
            GoMorestackMode::Auto => 1,
            GoMorestackMode::Always => 2,
        }
    }

    /// Inverse of [`GoMorestackMode::as_u8`] (unknown values read as `Off`).
    pub fn from_u8(v: u8) -> GoMorestackMode {
        match v {
            1 => GoMorestackMode::Auto,
            2 => GoMorestackMode::Always,
            _ => GoMorestackMode::Off,
        }
    }
}

/// Parse an `option gomorestack` value; the confirmation string follows the
/// option-arm convention.
pub fn parse_gomorestack_mode(value: &str) -> Result<(GoMorestackMode, String), String> {
    let mode = match value.trim().to_ascii_lowercase().as_str() {
        "off" | "0" | "false" => GoMorestackMode::Off,
        "auto" | "" => GoMorestackMode::Auto,
        "always" | "on" | "1" | "true" => GoMorestackMode::Always,
        other => {
            return Err(format!(
                "unknown gomorestack value {other:?} (expected off, auto or always)"
            ))
        }
    };
    Ok((mode, format!("Go stack-growth prologue folding set to {}", mode.as_str())))
}

/// Is `name` a Go runtime stack-growth entry point?
///
/// `runtime.morestack` plus its `_noctxt`/`c` and `.abi0` spellings — all
/// names only the Go runtime package can define. Matched by prefix so a new
/// runtime spelling folds too; the *shape* requirement is what keeps a
/// coincidental name in ordinary flow from cutting anything.
pub fn is_morestack_name(name: &str) -> bool {
    name.strip_prefix("runtime.morestack")
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('_') || rest.starts_with('.') || rest == "c")
}

/// Is the fold live for this function?
pub fn live(data: &Funcdata) -> bool {
    match GoMorestackMode::from_u8(data.get_arch().go_morestack) {
        GoMorestackMode::Off => false,
        GoMorestackMode::Auto => data.get_arch().source_lang.is_go(),
        GoMorestackMode::Always => true,
    }
}

/// (kuna) `ActionGoMorestack` — fold the Go stack-growth entry guard
/// (option `gomorestack`, default auto).
pub struct ActionGoMorestack {
    base: ActionBase,
}

impl ActionGoMorestack {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionGoMorestack { base: ActionBase::new(0, "gomorestack", g) })
    }
}

impl Action for ActionGoMorestack {
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
        Some(Box::new(ActionGoMorestack { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !live(data) {
            return 0;
        }
        // The pre-SSA window: cut the edge before heritage so the entry phi
        // never forms (a function carries one entry guard; the loop covers a
        // wrapper that inlined another and keeps each edit's match fresh).
        if data.get_heritage_pass() != 0 || data.num_calls() == 0 {
            return 0;
        }
        let mut changed = 0;
        while let Some(m) = match_entry_guard_loop(data, &is_morestack_name) {
            data.remove_branch(m.guard_block, m.guard_edge)
                .expect("ActionGoMorestack: removeBranch");
            data.remove_unreachable_blocks(false, true)
                .expect("ActionGoMorestack: removeUnreachableBlocks");
            self.base.count += 1;
            changed = 1;
        }
        changed
    }
}

#[cfg(test)]
#[path = "kuna_gomorestack/tests.rs"]
mod tests;
