//! Port of `decompiler/cpp/kuna_memsetsequence.{cc,hh}` — constant-fill
//! (memset/bzero) recovery sub-stage (kuna GH-9230/1537, S5 constsequence).
//!
//! This extends the S5 "constsequence" group ([`constseq`](crate::constseq)):
//! the existing string recovery (`RuleStringCopy`/`RuleStringStore`) only fires
//! for runs that spell a printable STRING, so an unrolled `memset`/`bzero`
//! (clang's inlined `movaps xmm0,N(rsp)` run, or an all-zero `bzero`) renders as
//! dozens of individual `buf[0xNN] = '\0';` element stores instead of one
//! `builtin_memset` (GH-9230).  Gated default-on by the arch flag
//! `memset_recover` (option `memsetrecover on|off`, shipped default `on`);
//! inert when off, byte-identical to upstream.
//!
//! [`MemsetSequence`] derives from `StringSequence` to reuse its
//! pointer-construction (`constructTypedPointer`) and COPY teardown
//! (`removeCopyOps`).  In the Rust port — where there is no inheritance and the
//! `StringSequence` driver is itself a deferred constseq seam (see below) — it
//! **composes** the ported [`ArraySequence`] base
//! ([`constseq`](crate::constseq) *Cross-pack visibility* note: the base fields
//! and methods are `pub(crate)` precisely so this module can build on them).
//!
//! # What this port covers
//!
//! The reusable, output-determining **single-value-fill detection** is fully
//! ported:
//!
//! * [`MemsetSequence::form_fill_run`] — sort the collected constant COPYs by
//!   offset, verify they tile a contiguous byte region (no gap/overlap) holding
//!   ONE fill byte, require a RUN (at least 2 COPYs) and a minimum 16-byte
//!   footprint, then truncate `move_ops` to the contiguous run and record
//!   `fill_value`/`fill_count`.  This is the routine that distinguishes a memset
//!   from a string (and from a lone NUL terminator), so it is the core that
//!   decides whether GH-9230 fires.
//!
//! # The live recovery path (now ported, GH-9230/1537)
//!
//! The *build*/*transform*/*teardown* halves the C++ `MemsetSequence : public
//! StringSequence` inherits are now ported on the shared [`StringSequence`]
//! machinery — the same machinery `RuleStringCopy` drives — exposed as
//! `pub(crate)` entry points in `constseq.rs`:
//!
//! * [`StringSequence::build_for_fill`](crate::constseq) — the containing-array
//!   type-walk + the constant COPY collection (`collect_fill_run`, recording each
//!   COPY's byte stride in the `WriteNode` slot).
//! * [`detect_fill_run`] — the single-value-fill detection (`formFillRun`),
//!   shared with [`MemsetSequence::form_fill_run`] (the detection model the unit
//!   tests drive via `from_collected`).
//! * `StringSequence::build_memset` — the `builtin_memset(dest,value,count)`
//!   CALLOTHER (`registerBuiltin(BUILTIN_MEMSET)` is wired at arch init) using the
//!   inherited `constructTypedPointer`; `StringSequence::transform_memset` then
//!   reuses `removeCopyOps` to tear down the COPY run.
//! * [`RuleMemsetCopy::apply_op`] — the live driver (the `getTypeDefFacing`/
//!   `isCharPrint`/`isOpaqueString`/`isAddrTied` guards + `Scope::queryContainer`,
//!   then `build_for_fill` → `detect_fill_run` → `transform_memset`), mirroring
//!   `RuleStringCopy::apply_op`.
//!
//! The remaining stub is [`MemsetSequence::collect_fill_run`] (the
//! direct-construction `collectFillRun`); the live path collects via
//! `StringSequence::build_for_fill` instead, so it is unused.
//!
//! ## Gate wiring — STUB(W4)
//!
//! As in the sibling kuna rules, the C++
//! `if (!data.getArch()->memset_recover) return 0;` gate is resolved at
//! construction (the seam `Funcdata::glb` does carry `memset_recover`).  W8
//! threads the live `Architecture::memset_recover`; [`specs`] uses the shipped
//! default (`on` => `memset_recover = true`, kuna DIV-2 default-on, GH-9230/1537).

// The collection/build/transform/driver halves are seam-blocked stubs until the
// W4 symbol table + W6 type factory land (see module docs); their helper inputs
// read as unused in the interim.  This mirrors the `#![allow(dead_code)]` the
// merged `constseq` base carries for the same reason.
#![allow(dead_code)]

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::constseq::{ArraySequence, StringSequence, WriteNode};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::context::OpId;

/// (kuna GH-9230) A constant-fill run of COPY ops collapsed into builtin_memset
/// (C++ `class MemsetSequence : public StringSequence`).
///
/// Composes the ported [`ArraySequence`] base (no inheritance in Rust; the base
/// fields/methods are `pub(crate)` for exactly this reuse — see module docs).
pub struct MemsetSequence {
    /// The shared array-sequence machinery (`char_type`, `move_ops`,
    /// `num_elements`, `byte_array`) — the C++ `StringSequence`/`ArraySequence`
    /// base.
    base: ArraySequence,
    /// The single repeated fill byte (C++ `uint1 fillValue`).
    fill_value: u8,
    /// Number of contiguous bytes holding `fill_value` (C++ `int4 fillCount`).
    fill_count: int4,
}

impl MemsetSequence {
    /// Minimum contiguous fill footprint in bytes (one SIMD store) before a run
    /// is accepted as a memset (C++ `formFillRun`'s `if (totalBytes < 16)`).
    const MINIMUM_FILL_BYTES: int4 = 16;

    /// Construct over the element data-type, mirroring the C++ ctor which clears
    /// any base state, runs `collectFillRun`, and (if anything was collected)
    /// `formFillRun` (C++ `MemsetSequence::MemsetSequence`).
    ///
    /// STUB: `collect_fill_run` is seam-blocked (see module docs), so in the
    /// current port the live driver supplies the collected `move_ops` directly
    /// via [`MemsetSequence::from_collected`] and this constructor leaves the run
    /// empty.  The C++ control flow is preserved for restoration.
    pub fn new(char_type: Rc<Datatype>, data: &Funcdata) -> MemsetSequence {
        let mut seq = MemsetSequence {
            base: ArraySequence::new(char_type),
            fill_value: 0,
            fill_count: 0,
        };
        seq.base.num_elements = 0; // Clear any state the base ctor may have left
        seq.base.move_ops.clear();
        seq.collect_fill_run(data);
        if seq.base.move_ops.is_empty() {
            return seq;
        }
        seq.form_fill_run(data);
        seq
    }

    /// Build a sequence directly from an already-collected COPY run (the
    /// `move_ops`), then run [`form_fill_run`](MemsetSequence::form_fill_run).
    ///
    /// This is the seam bridge for the not-yet-ported [`collect_fill_run`]
    /// (`collectFillRun` reaches the W4 loc-set/symbol surfaces — see module
    /// docs).  Each `WriteNode`'s `slot` field holds the COPY byte-size and its
    /// `op` references the COPY (`getIn(0)` is the constant fill), exactly as the
    /// C++ `collectFillRun` populates `moveOps`.
    ///
    /// `pub(crate)`: [`WriteNode`] is the constseq base's `pub(crate)` type, so
    /// this seam bridge is in-crate (the live driver and tests build it).
    pub(crate) fn from_collected(
        char_type: Rc<Datatype>,
        move_ops: Vec<WriteNode>,
        data: &Funcdata,
    ) -> MemsetSequence {
        let mut seq = MemsetSequence {
            base: ArraySequence::new(char_type),
            fill_value: 0,
            fill_count: 0,
        };
        seq.base.move_ops = move_ops;
        if !seq.base.move_ops.is_empty() {
            seq.form_fill_run(data);
        }
        seq
    }

    /// Return `true` if a fill run was found (C++ `MemsetSequence::isValidFill`:
    /// `return fillCount != 0;`).
    pub fn is_valid_fill(&self) -> bool {
        self.fill_count != 0
    }

    /// The detected single fill byte (valid only when [`is_valid_fill`] is true).
    pub fn fill_value(&self) -> u8 {
        self.fill_value
    }
    /// The detected contiguous fill length in bytes.
    pub fn fill_count(&self) -> int4 {
        self.fill_count
    }
    /// The collected/truncated COPY run (read access for the driver/tests).
    /// `pub(crate)`: returns the constseq base's `pub(crate)` [`WriteNode`].
    pub(crate) fn move_ops(&self) -> &[WriteNode] {
        &self.base.move_ops
    }

    /// Collect the contiguous constant COPY run (C++ `MemsetSequence::collectFillRun`).
    ///
    /// STUB(W4/W6): walks the Symbol's type to the containing array
    /// (`entry->getSymbol()->getType()`, `getSubType`/`resolveTruncation`), then
    /// iterates `data.beginLoc(startAddr)..endLoc(endAddr)` — the address-only
    /// location-set overload — gathering same-block constant uniform-fill COPYs.
    /// The symbol table (W4) and the address-only loc-set overload (W4 loc-set
    /// surface) are not yet ported, so this is a documented stub; the live driver
    /// feeds the collected run through [`from_collected`].  Recorded as a loss.
    fn collect_fill_run(&mut self, _data: &Funcdata) {
        // STUB(W4/W6): entry/Symbol::getType + getSubType/resolveTruncation walk,
        // data.beginLoc(addr)/endLoc(addr) address-only loc-set overload, per-op
        // block membership, uniform-byte constant check.  Leaves move_ops empty.
    }

    /// Detect a single-value constant fill across the collected ops
    /// (C++ `MemsetSequence::formFillRun`) — transcribed.
    ///
    /// Sort the collected COPYs by offset, verify they tile a contiguous byte
    /// region (no gaps, no overlap) starting at the lowest offset and all hold
    /// the same low fill byte; require a RUN (at least 2 COPYs, so a lone store
    /// such as a string's NUL terminator is never claimed) and a minimum 16-byte
    /// footprint.  On success `move_ops` is truncated to the contiguous run and
    /// `fill_value`/`fill_count`/`num_elements` are recorded.
    fn form_fill_run(&mut self, data: &Funcdata) {
        if let Some((fb, total_bytes)) = detect_fill_run(data, &mut self.base.move_ops) {
            self.fill_value = fb;
            self.fill_count = total_bytes;
            self.base.num_elements = total_bytes;
        }
    }

    // The live `build_memset`/`transform`/`removeCopyOps` (C++
    // `MemsetSequence::buildMemset`/`transform`) are now ported on the shared
    // `StringSequence` machinery (`build_for_fill` -> `detect_fill_run` ->
    // `transform_memset`), driven by `RuleMemsetCopy::apply_op` below.  This
    // `MemsetSequence` struct remains the detection model (`form_fill_run`) the
    // unit tests drive directly via `from_collected`.
}

/// (kuna GH-9230) Recognize a constant-fill run of COPY ops as builtin_memset
/// (C++ `RuleMemsetCopy`).
///
/// Mirrors `RuleStringCopy` but, when `option memsetrecover on`, routes a run of
/// COPYs writing the SAME constant byte into a char array to builtin_memset.
pub struct RuleMemsetCopy {
    /// Resolved `glb->memset_recover` gate (STUB(W4); see module docs).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleMemsetCopy {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleMemsetCopy {
        RuleMemsetCopy { enabled, group: String::from("analysis") }
    }
    /// Construct with an explicit group (C++ `RuleMemsetCopy(const string &g)`).
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleMemsetCopy {
        RuleMemsetCopy { enabled, group: group.into() }
    }
}

impl Rule for RuleMemsetCopy {
    /// C++ `RuleMemsetCopy::getOpList`: `oplist.push_back(CPUI_COPY);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleMemsetCopy { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleMemsetCopy::applyOp` (`kuna_memsetsequence.cc:188`) — gate +
    /// structure ported; the seam-blocked body declines.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !self.enabled && !data.get_arch().memset_recover {
            return 0;
        }
        let in0 = match data.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return 0,
        };
        if !data.vbank().get(in0).map(|v| v.is_constant()).unwrap_or(false) {
            return 0;
        }
        let outvn = match data.obank().get(op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return 0,
        };
        let ct = Rc::clone(
            data.vbank().get(outvn).expect("RuleMemsetCopy: stale outvn").get_type_def_facing(),
        );
        if !ct.is_char_print() {
            return 0;
        }
        if ct.is_opaque_string() {
            return 0;
        }
        if !data.vbank().get(outvn).unwrap().is_addr_tied() {
            return 0;
        }
        // Local stack scope first, then the frozen global-scope snapshot
        // (mirrors `RuleStringCopy::apply_op`).
        let out_addr = data.vbank().get(outvn).unwrap().get_addr().clone();
        let out_size = data.vbank().get(outvn).unwrap().get_size();
        let op_addr = data.obank().get(op).unwrap().get_addr().clone();
        let entry = data
            .get_scope_local()
            .and_then(|lm| lm.query_container(&out_addr, out_size, &op_addr))
            .or_else(|| {
                data.get_arch().query_container_global(&out_addr, out_size, &op_addr).and_then(|g| {
                    let sym_type = g.symbol_type.clone()?;
                    Some(crate::varmap::StringContainerEntry {
                        first: g.entry_addr.get_offset(),
                        size: sym_type.get_size(),
                        addr: g.entry_addr.clone(),
                        sym_type,
                    })
                })
            });
        let entry = match entry {
            Some(e) => e,
            None => return 0,
        };
        // The array type-walk + COPY collection (reusing the StringSequence
        // machinery), then the single-value-fill detection.
        let mut sequence = match StringSequence::build_for_fill(data, ct, entry, op, out_addr) {
            Some(s) => s,
            None => return 0,
        };
        let (fill_value, fill_count) = match detect_fill_run(data, sequence.fill_move_ops_mut()) {
            Some(fv) => fv,
            None => return 0,
        };
        if !sequence.transform_memset(data, fill_value, fill_count) {
            return 0;
        }
        1
    }
}

/// (kuna GH-9230) How constant-fill recovery is toggled (the two values of
/// `option memsetrecover`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemsetRecoverForm {
    /// `on`: collapse constant-fill runs into builtin_memset
    /// (`glb->memset_recover = true`).
    On,
    /// `off`: leave the individual element stores (`glb->memset_recover = false`,
    /// byte-identical upstream).
    Off,
}

impl MemsetRecoverForm {
    /// The resolved `glb->memset_recover` flag for this form.
    pub fn memset_recover(self) -> bool {
        matches!(self, MemsetRecoverForm::On)
    }
}

/// Parse the `option memsetrecover on|off` argument and produce the resolved
/// form plus the confirmation message (C++ `OptionMemsetRecover::apply`).
///
///
/// `onOrOff` is the shared parser ([`on_or_off`](crate::options::on_or_off)): an
/// empty string or `"on"` is `true`, `"off"` is `false`, anything else is a parse
/// error.  The caller writes [`MemsetRecoverForm::memset_recover`] into
/// [`Architecture::memset_recover`].
pub fn parse_memset_recover_form(p1: &str) -> KunaResult<(MemsetRecoverForm, String)> {
    let val = crate::options::on_or_off(p1).map_err(|_: KunaError| {
        KunaError::parse("Must specify toggle value, on/off")
    })?;
    // The memset_recover flag is left to the caller (Architecture::memset_recover).
    let form = if val { MemsetRecoverForm::On } else { MemsetRecoverForm::Off };
    let prop = if val { "on" } else { "off" };
    Ok((form, format!("Constant-fill (memset) recovery turned {prop}")))
}

/// Per-file registration rows in C++ definition order (one rule).
///
/// Shipped default: `option memsetrecover on` (`memset_recover = true`; kuna
/// DIV-2 default-on, GH-9230/1537).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        ctor: || Box::new(RuleMemsetCopy::with_group(true, "analysis")),
    }]
}

// --- Local IR read helpers (see kuna_addcarrychain for the rationale) --------

/// Read the constant offset of a COPY op's input 0 (C++
/// `moveOps[i].op->getIn(0)->getOffset()`).
fn copy_const_offset(data: &Funcdata, op: OpId) -> uintb {
    let in0 = data
        .obank()
        .get(op)
        .and_then(|o| o.get_in(0))
        .expect("memset: COPY missing input 0");
    data.vbank().get(in0).expect("memset: stale COPY input").get_offset()
}

/// The single-value-fill detection (C++ `MemsetSequence::formFillRun`), factored
/// out of [`MemsetSequence::form_fill_run`] so the live driver
/// ([`RuleMemsetCopy::apply_op`]) can run it over the COPY run gathered into a
/// [`StringSequence`] (via `build_for_fill`/`collect_fill_run`).
///
/// Sort the collected COPYs by offset, verify they tile a contiguous byte region
/// (no gap/overlap) all holding the SAME low fill byte, require a RUN (at least 2
/// COPYs — a lone store like a string NUL is never claimed) and a minimum 16-byte
/// footprint.  On success `move_ops` is truncated to the contiguous run and
/// `(fill_byte, total_bytes)` is returned.  Each `WriteNode.slot` carries the
/// COPY's byte stride.
pub(crate) fn detect_fill_run(data: &Funcdata, move_ops: &mut Vec<WriteNode>) -> Option<(u8, int4)> {
    if move_ops.is_empty() {
        return None;
    }
    // sort by offset.
    move_ops.sort_by(|a, b| a.offset.cmp(&b.offset));
    let fb: u8 = (copy_const_offset(data, move_ops[0].op) & 0xff) as u8;
    let run_start: u64 = move_ops[0].offset;
    let mut expect: u64 = run_start;
    let mut last_idx: i64 = -1;
    for i in 0..move_ops.len() {
        // Gap or overlap
        if move_ops[i].offset != expect {
            break;
        }
        // Different fill byte
        if (copy_const_offset(data, move_ops[i].op) & 0xff) as u8 != fb {
            break;
        }
        // slot holds the COPY byte-size
        expect = move_ops[i].offset + move_ops[i].slot as u64;
        last_idx = i as i64;
    }
    // A memset is a RUN: require at least 2 COPYs
    if last_idx < 1 {
        return None;
    }
    let total_bytes: int4 = (expect - run_start) as int4;
    // ...and a minimum fill footprint
    if total_bytes < MemsetSequence::MINIMUM_FILL_BYTES {
        return None;
    }
    // Drop ops past the run.
    let keep = (last_idx + 1) as usize;
    if keep != move_ops.len() {
        move_ops.truncate(keep);
    }
    Some((fb, total_bytes))
}

#[cfg(test)]
#[path = "kuna_memsetsequence/tests.rs"]
mod tests;
