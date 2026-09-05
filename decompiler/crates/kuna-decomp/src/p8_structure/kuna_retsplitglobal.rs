//! (kuna) `retsplitglobal` — a shared epilogue that WRITES GLOBALS is not a bare
//! epilogue, so it must not be cloned into every predecessor.
//!
//! # The symptom
//!
//! A macOS crackme's initializer `sub_100003790` ends in one block that fills
//! four message buffers, seventy-two byte stores long, and returns. Three paths
//! reach it: the `n < 0` guard, the vectorised loop's exact-fit exit, and the
//! scalar tail. kuna emits that block **three times** — 216 statements, 46% of
//! the whole function's text:
//!
//! ```text
//! if ((int)dat_100008044 < 0) {
//!   dat_1000080c8 = dat_100008077;      // 72 stores
//!   ...
//!   return;
//! }
//! ...
//! if (v3 == v1) {
//!   dat_1000080c8 = dat_100008077;      // the same 72 stores
//!   ...
//!   return;
//! }
//! label_1000038c0:
//!   do { ... } while (v1 != v3);
//!   dat_1000080c8 = dat_100008077;      // and again
//! ```
//!
//! # Why it is cloned
//!
//! `ActionReturnSplit::isSplittable` (`blockaction.cc:2242`) decides whether a
//! multi-predecessor RETURN block is a *bare epilogue* — cheap enough that giving
//! every predecessor its own private copy is free. It walks the block and admits
//! only `MULTIEQUAL`, `COPY` and `RETURN`, rejecting anything else, and for the
//! `COPY`/`RETURN` cases it checks that no INPUT is a free Varnode.
//!
//! It never looks at the COPY's OUTPUT. Upstream that is harmless, because the
//! epilogue COPYs it was written for move registers, which die at the return. A
//! `COPY` into a *persistent* Varnode is a different animal: it is a store to a
//! global that the program can observe, and it is exactly what the C prints as
//! `dat_<addr> = ...`. Seventy-two of them are neither bare nor cheap, and both
//! splitters — upstream `ActionReturnSplit` and kuna's `ActionReturnDup`
//! (`returndup`) — share this one predicate, which is why turning `returndup` off
//! does not stop the cloning: the goto-driven upstream action picks it up
//! instead.
//!
//! Nothing here is incorrect. `node_split` gives each clone its own in-edge, so
//! every copy runs on exactly one path and the program's behaviour is preserved.
//! It is a readability defect, and a large one: an agent reading the initializer
//! has to notice that three 72-line blocks are the same block before it can read
//! any of them.
//!
//! # The rule
//!
//! With the option ON the candidate block's observable global stores are
//! COUNTED, and the split is declined only when there are more than
//! [`MAX_CLONED_GLOBAL_STORES`] of them. A bound rather than a veto is the whole
//! design, and it is the house style of this phase: `crossjumprevert` and
//! `dedupitetail` cap a duplicated leaf run at 8 printed ops and `taildup` at
//! 12. A two-store epilogue is cheap to clone and reads better cloned — each
//! path gets its own private exit and the merged form would need a phi per
//! global, sometimes printing a global's own value back into it on a path where
//! the binary stores nothing. Seventy-two stores across three predecessors is
//! 216 statements. The clause is deliberately narrow:
//!
//! * Only `COPY` is counted. A `MULTIEQUAL` writing a persistent Varnode is a
//!   phi, not an observable write — `node_split` rewires phis rather than
//!   duplicating an effect — and `RETURN` writes nothing.
//! * `Heritage::guardReturns`' synthetic `glob = glob` self-copies do not count.
//!   Those are liveness markers, one per persistent global, present before every
//!   RETURN in every function that has a global; counting them would price a
//!   bare epilogue by how many globals the whole PROGRAM has. They are excluded
//!   by the `return_copy` flag and, independently, by input and output naming the
//!   same storage.
//! * Persistence, not the address space, is the test: a global the program can
//!   observe after the function returns. A unique/register temporary, which is
//!   what upstream's epilogue COPYs move, is never persistent, so a register-only
//!   epilogue splits exactly as before however long it is.
//! * Declining is one-directional. It can only ever make a split NOT happen, so
//!   it can neither create a duplication nor change what the surviving merged
//!   block does.
//!
//! Gated by the arch flag `ret_split_global` (option `retsplitglobal on|off`,
//! shipped default `on`); with the option off the predicate is upstream's and
//! the output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::p0_knowledge::options::on_or_off;

/// Marshalling element id for the option (kuna 4000+ range).
pub const ELEM_RETSPLITGLOBAL: ElementId = ElementId::new("retsplitglobal", 4143);

/// The `retsplitglobal on|off` option (`Architecture::ret_split_global`).
pub struct OptionRetSplitGlobal;

impl OptionRetSplitGlobal {
    /// The option name.
    pub const NAME: &'static str = "retsplitglobal";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::ret_split_global`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Global-writing shared-epilogue split refusal turned {prop}")))
    }
}

/// How many observable global stores a shared epilogue may carry and still be
/// cloned into every predecessor.
///
/// The bound, not a veto, is the point. Cloning a handful of stores is what
/// upstream's split is for and it reads well — each path gets its own private
/// epilogue and no phi is needed. Cloning seventy-two of them into three
/// predecessors is 216 statements. Every sibling duplication pass in this phase
/// is bounded the same way and in the same range (`crossjumprevert` and
/// `dedupitetail` cap a duplicated leaf run at 8 printed ops, `taildup` at 12),
/// so an epilogue that stores to more than eight distinct globals is where a
/// "bare epilogue" stops being bare.
pub const MAX_CLONED_GLOBAL_STORES: usize = 8;

/// Is this op an OBSERVABLE store to a global — one of the writes the block
/// would have to clone?
///
/// True for a `COPY` whose output is persistent that is not one of
/// `Heritage::guardReturns`' synthetic self-copies.
///
/// The self-copy exclusion is the whole reason this is a function and not an
/// inline `is_persist()` test. Before every RETURN, `Heritage::guardReturns`
/// inserts a synthetic `glob = glob` COPY for each persistent global so the
/// global's final value stays live past the end of the function. Those are
/// liveness markers, not writes — they store a location's own value back into
/// it, `Merge::markInternalCopies` hides them from the printer, and EVERY
/// function with a global has one per global, so counting them would price a
/// bare epilogue by how many globals the PROGRAM has. They are recognised two
/// ways, either sufficient: the `return_copy` flag `guardReturns` sets, and the
/// fact that input and output name the same storage.
pub fn is_global_store(is_copy: bool, out_is_persist: bool, is_return_copy: bool, input_is_same_storage: bool) -> bool {
    is_copy && out_is_persist && !is_return_copy && !input_is_same_storage
}

/// Does a block carrying `stores` observable global stores stay splittable?
///
/// `gate` is the resolved `Architecture::ret_split_global`; with it off the
/// answer is always true and the upstream predicate is unchanged.
pub fn split_is_declined(gate: bool, stores: usize) -> bool {
    gate && stores > MAX_CLONED_GLOBAL_STORES
}

#[cfg(test)]
#[path = "kuna_retsplitglobal/tests.rs"]
mod tests;
