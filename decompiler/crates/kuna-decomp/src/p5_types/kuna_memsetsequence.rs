//! Recover constant-fill COPY runs as `builtin_memset`.
//!
//! [`RuleMemsetCopy`] collects array writes through [`StringSequence`], tests
//! the shared fill predicate, then reuses its pointer construction and teardown.
//! `memsetrecover` controls the live rule. [`MemsetSequence`] is retained as a
//! detection model; its legacy direct constructor does not collect IR writes.

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
    /// This legacy constructor leaves an empty, invalid run. Live recovery
    /// collects writes through `StringSequence::build_for_fill` instead.
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

    /// Collect the contiguous constant COPY run (C++ `MemsetSequence::collectFillRun`).
    ///
    /// The legacy model does not scan IR; the live rule uses StringSequence.
    fn collect_fill_run(&mut self, _data: &Funcdata) {}

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

}

/// (kuna GH-9230) Recognize a constant-fill run of COPY ops as builtin_memset
/// (C++ `RuleMemsetCopy`).
///
/// Mirrors `RuleStringCopy` but, when `option memsetrecover on`, routes a run of
/// COPYs writing the SAME constant byte into a char array to builtin_memset.
pub struct RuleMemsetCopy {
    /// Gate supplied by rule registration.
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

    /// Recover a validated constant-fill run, subject to the resolved gate.
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
