//! Decode-context resolution for the recursive-descent walker (design §4.2).
//!
//! Some ISAs steer instruction decode with a SLEIGH *processor-context* variable:
//! ARM `TMode` (0 = A32, 1 = Thumb) and MIPS `ISA_MODE` (0 = MIPS32, 1 = the
//! alternate MIPS16e/microMIPS ISA). That context **must be painted into the
//! engine's `ContextDatabase` before the bytes at an address are decoded**, or the
//! same byte stream misdecodes — a Thumb function read as A32, a MIPS16 function
//! read as MIPS32: garbage.
//!
//! The decompiler's normal pipeline gets this from the analysis-tier commit boundary
//! (`kuna-console::engine::commit_analysis_output` step 6 paints
//! [`crate::pass::ContextPaint`] facts over the engine's `ContextDatabase` before
//! `load function`). The Listing's recursive-descent walker decodes **outside**
//! that path — it drives [`crate::listing::decode::decode_one`] directly — so it
//! must perform the same paint itself, with the same facts, before it walks.
//!
//! # How the per-address decode mode is obtained
//!
//! [`ContextPainter`] does **not** re-derive the decode mode: it *calls into the
//! existing marker logic* — [`crate::loader::arm_markers::scan_arm_markers`]
//! (ARM `$t`/`$a` mapping symbols + STT_FUNC LSB → `TMode`) and
//! [`crate::loader::mips_markers::scan_mips_isa_markers`] (STT_FUNC LSB /
//! `STO_MIPS_MIPS16` `st_other` → `ISA_MODE`) — and reuses their
//! [`crate::pass::ContextPaint`] facts verbatim. The same computation the
//! decompiler's analysis tier trusts is the one the walker paints.
//!
//! # Mechanism (mirrors the commit boundary exactly)
//!
//! [`ContextPainter::paint_all`] applies every collected paint to the engine's
//! `ContextDatabase` via [`Architecture::with_context_db_mut`] →
//! `db.set_variable(var, addr, value)` — the *same* call
//! `commit_analysis_output` makes. `set_variable` sets the value from `addr` up to
//! the next change point, so painting only the markers (where the mode *changes*)
//! correctly fills every region in between. The walk then decodes against an
//! already-painted context DB, so each [`decode_one`](crate::listing::decode::decode_one)
//! reads the right mode.
//! An explicit file input ARM/Thumb selection suppresses metadata TMode paints;
//! the bootstrap has already painted that selection over the code sections.
//!
//! # Gate safety (x86-64 and every non-ARM/MIPS language is untouched)
//!
//! The marker scans gate on the object architecture (ARM-only / MIPS-only), so on
//! x86-64 (and anything else with no decode-mode context) [`ContextPainter::new`]
//! collects **zero** paints and [`paint_all`] is a no-op — x86-64 decode is
//! byte-identical to no painter at all. Belt-and-suspenders: `set_variable`
//! returns `Err` for a context variable the active language does not register
//! (e.g. `TMode` on a MIPS object), and that error is swallowed — the exact
//! `ContextDatabase`-not-registered no-op the commit boundary relies on.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;

use crate::pass::ContextPaint;
use crate::loader::arm_markers::scan_arm_markers;
use crate::loader::mips_markers::scan_mips_isa_markers;

/// Resolves and applies the per-address decode-mode context (ARM `TMode`, MIPS
/// `ISA_MODE`) the recursive-descent walker needs before it decodes (design §4.2).
///
/// Built once per [`Listing::build`](crate::listing::Listing::build) from the
/// object file by **reusing the marker logic** (not re-deriving it); holds the
/// collected [`ContextPaint`] facts so [`paint_all`](ContextPainter::paint_all)
/// can apply them to the engine's `ContextDatabase` before the walk.
pub(super) struct ContextPainter {
    /// The decode-mode paints to apply, collected from the marker scans. Empty on
    /// any language with no decode-mode context (e.g. x86-64) ⇒ a no-op painter.
    paints: Vec<ContextPaint>,
}

impl ContextPainter {
    /// Collect the decode-mode paints for `file` by calling into the existing ARM
    /// and MIPS marker logic (design §4.2). The marker scans self-gate on the
    /// object architecture, so for x86-64 (or any language without a decode-mode
    /// context variable) this yields an empty, no-op painter.
    pub(super) fn new(file: &object::File, arch: &Architecture) -> Self {
        let mut paints = Vec::new();
        // ARM `$t`/`$a` mapping symbols + STT_FUNC-LSB → `TMode` (Thumb). The scan
        // is ARM-gated; on a non-ARM object it returns an empty output.
        paints.extend(scan_arm_markers(file).context_paints);
        // ARM Cortex-M: a stripped bare-metal firmware image carries none of the
        // `$t`/FUNC-LSB markers `scan_arm_markers` reads. When a hardware vector
        // table is detected, the whole image is Thumb-only, so region-paint
        // `TMode=1` across every executable section — the same facts the analysis
        // commit path (`EntryDiscoveryPass`) paints, mirrored here for the walk (a
        // Thumb `BL` does not `globalset` the callee mode, so the region paint is
        // what lets `main` and the rest of the call tree decode as Thumb). Empty on
        // any non-Cortex-M ARM object and every non-ARM arch.
        paints.extend(crate::analyzers::entry::cortexm_thumb_paints(file, false));
        if arch.input_arm_isa_override {
            paints.retain(|paint| paint.var != "TMode");
        }
        // MIPS STT_FUNC-LSB / `STO_MIPS_MIPS16` `st_other` → `ISA_MODE` (MIPS16e /
        // microMIPS). The scan is MIPS-gated; empty on a non-MIPS object.
        paints.extend(scan_mips_isa_markers(file).context_paints);
        ContextPainter { paints }
    }

    /// `true` iff there is nothing to paint (x86-64 / any language with no
    /// decode-mode context) — the walk needs no context step at all.
    pub(super) fn is_empty(&self) -> bool {
        self.paints.is_empty()
    }

    /// Paint every collected decode-mode context value into the engine's
    /// `ContextDatabase`, BEFORE the walk decodes anything (the timing ARM Thumb /
    /// MIPS16 mode requires). Mirrors `commit_analysis_output` step 6: a `None`
    /// end is the point set (`set_variable`, paint-to-next-change-point); a `Some`
    /// end paints the explicit `[addr, end)` region (`set_variable_region`).
    ///
    /// Gate-safe: an unregistered context variable (a faithful no-op for a
    /// language that does not define it) is swallowed via the dropped `Result`.
    pub(super) fn paint_all(&self, arch: &Architecture, code_space: &Rc<AddrSpace>) {
        for paint in &self.paints {
            let begin = Address::new(Rc::clone(code_space), paint.addr);
            // Drop the Result: an unregistered context variable is a faithful
            // no-op (the commit boundary relies on the same swallow).
            let _ = arch.with_context_db_mut(|db| match paint.end {
                Some(end) => {
                    let endad = Address::new(Rc::clone(code_space), end);
                    db.set_variable_region(paint.var.as_bytes(), &begin, &endad, paint.value)
                }
                None => db.set_variable(paint.var.as_bytes(), &begin, paint.value),
            });
        }
    }
}
