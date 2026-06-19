//! WS4c -- the `ConsistencyChecker` (port of `slgh_compile.cc:215-1776`).
//!
//! The three passes the post-parse `process()` runs over every constructor's
//! p-code template trees:
//!
//! 1. **`test_size_restrictions`** -- a post-order walk of the subtables
//!    deriving export sizes and enforcing per-opcode size rules; this also
//!    converts unnecessary `INT_ZEXT`/`INT_SEXT`/`SUBPIECE` into `COPY`
//!    (`deal_with_unnecessary_ext`/`trunc`), which *mutates* the templates.
//! 2. **`test_truncations`** -- resolves `v_offset_plus` truncated varnode
//!    offsets now that all sizes are known (`adjust_truncation`).
//! 3. **`optimize_all`** -- the limited COPY-propagation: for each temporary
//!    read-once/written-once through a `COPY`, remove the `COPY` and rewire
//!    (`apply_optimization`), then the dead-temp / read-before-write checks.
//!
//! Because these passes MODIFY the `ConstructTpl`s (which by `process()` time
//! live in the `SleighBase` template arena, referenced from each constructor by
//! handle), the checker is implemented as inherent methods on
//! [`crate::slgh_compile::SleighCompile`] so it has direct access to the base /
//! symbol table / template arena (the C++ class holds a `compiler` back-pointer
//! for exactly this).
//!
//! Faithful to the C++: the per-opcode `size_restriction` switch, the
//! `UniqueState`/`OptimizeRecord` machinery (`getDefinitions` interval
//! splitting, `findValidRule` interference checks), and the post-order subtable
//! traversal are transcribed 1:1.

#![allow(clippy::needless_range_loop)]

use std::collections::BTreeMap;

use kuna_num::opcodes::OpCode;
use kuna_sleigh::semantics::{
    ConstTpl, ConstType, VField, VarnodeTpl, BUILD, CROSSBUILD, DELAY_SLOT, LABELBUILD, MACROBUILD,
};
use kuna_sleigh::slghsymbol::{ConstructTplHandle, ConstructorRef, SymbolKind, SymbolType};

use crate::slgh_compile::{SleighCompile, SymbolId};

/// C++ `SleighBase::MAX_UNIQUE_SIZE` (sleighbase.cc:20).
const MAX_UNIQUE_SIZE: u64 = 256;

/// C++ `ConsistencyChecker::OptimizeRecord` (slgh_compile.hh): read/write usage
/// of a temporary register within a constructor.
#[derive(Clone, Debug)]
struct OptimizeRecord {
    offset: u64,
    size: i32,
    writeop: i32,
    readop: i32,
    inslot: i32,
    writecount: i32,
    readcount: i32,
    writesection: i32,
    readsection: i32,
    opttype: i32,
}

impl OptimizeRecord {
    fn new(offset: u64, size: i32) -> OptimizeRecord {
        OptimizeRecord {
            offset,
            size,
            writeop: -1,
            readop: -1,
            inslot: -1,
            writecount: 0,
            readcount: 0,
            writesection: -2,
            readsection: -2,
            opttype: -1,
        }
    }

    /// C++ `OptimizeRecord(vector<OptimizeRecord*> &records)`: merge overlapping.
    fn coalesce(records: &[OptimizeRecord]) -> OptimizeRecord {
        let mut min_off: Option<u64> = None;
        let mut max_off: Option<u64> = None;
        for r in records {
            if min_off.map(|m| r.offset < m).unwrap_or(true) {
                min_off = Some(r.offset);
            }
            let end = r.offset + r.size as u64;
            if max_off.map(|m| end > m).unwrap_or(true) {
                max_off = Some(end);
            }
        }
        let offset = min_off.unwrap_or(0);
        let mut res = OptimizeRecord::new(offset, (max_off.unwrap_or(0) - offset) as i32);
        for r in records {
            res.update_combine(r);
        }
        res
    }

    fn update_read(&mut self, i: i32, inslot: i32, sec_num: i32) {
        self.readop = i;
        self.readcount += 1;
        self.inslot = inslot;
        self.readsection = sec_num;
    }
    fn update_write(&mut self, i: i32, sec_num: i32) {
        self.writeop = i;
        self.writecount += 1;
        self.writesection = sec_num;
    }
    fn update_export(&mut self) {
        self.writeop = 0;
        self.readop = 0;
        self.writecount = 2;
        self.readcount = 2;
        self.readsection = -2;
        self.writesection = -2;
    }
    fn update_combine(&mut self, that: &OptimizeRecord) {
        if that.writecount != 0 {
            self.writeop = that.writeop;
            self.writesection = that.writesection;
        }
        if that.readcount != 0 {
            self.readop = that.readop;
            self.inslot = that.inslot;
            self.readsection = that.readsection;
        }
        self.writecount += that.writecount;
        self.readcount += that.readcount;
    }
}

/// C++ `ConsistencyChecker::UniqueState`: a map from unique-space offset to
/// `OptimizeRecord`, kept disjoint by `set`.
#[derive(Default)]
struct UniqueState {
    recs: BTreeMap<u64, OptimizeRecord>,
}

impl UniqueState {
    fn clear(&mut self) {
        self.recs.clear();
    }

    fn end_of(rec: &OptimizeRecord) -> u64 {
        rec.offset + rec.size as u64
    }

    /// C++ `lesserIter(offset)`: the last record starting strictly before
    /// `offset` (the predecessor of `lower_bound(offset)`), or `None`.
    fn lesser_key(&self, offset: u64) -> Option<u64> {
        self.recs.range(..offset).next_back().map(|(k, _)| *k)
    }

    /// C++ `set(OptimizeRecord &rec)`: coalesce overlaps and replace.
    fn set(&mut self, rec: OptimizeRecord) {
        let defs = self.get_definition_keys(rec.offset, rec.size);
        let mut records: Vec<OptimizeRecord> =
            defs.iter().map(|k| self.recs[k].clone()).collect();
        records.push(rec);
        let coalesced = OptimizeRecord::coalesce(&records);
        // erase [coalesced.offset, coalesced.offset+coalesced.size)
        let lo = coalesced.offset;
        let hi = coalesced.offset + coalesced.size as u64;
        let to_remove: Vec<u64> = self.recs.range(lo..hi).map(|(k, _)| *k).collect();
        for k in to_remove {
            self.recs.remove(&k);
        }
        self.recs.insert(coalesced.offset, coalesced);
    }

    /// C++ `getDefinitions`: collect all records overlapping `[offset,
    /// offset+size)`, splitting/inserting gap records as the C++ does, and
    /// return the **keys** of the resulting overlapping records (so callers can
    /// then mutate through `self.recs`).
    fn get_definition_keys(&mut self, offset: u64, mut size: i32) -> Vec<u64> {
        if size == 0 {
            size = 1;
        }
        let mut result: Vec<u64> = Vec::new();
        let mut cursor = offset;
        if let Some(lk) = self.lesser_key(offset) {
            if Self::end_of(&self.recs[&lk]) > offset {
                cursor = Self::end_of(&self.recs[&lk]);
                result.push(lk);
            }
        }
        let end = offset + size as u64;
        // Walk lower_bound(offset).. while key < end.
        let keys: Vec<u64> = self.recs.range(offset..end).map(|(k, _)| *k).collect();
        for k in keys {
            if k > cursor {
                // Insert a gap record [cursor, k).
                let gap = OptimizeRecord::new(cursor, (k - cursor) as i32);
                self.recs.insert(cursor, gap);
                result.push(cursor);
            }
            result.push(k);
            cursor = Self::end_of(&self.recs[&k]);
        }
        if end > cursor {
            let gap = OptimizeRecord::new(cursor, (end - cursor) as i32);
            self.recs.insert(cursor, gap);
            result.push(cursor);
        }
        result
    }

    fn keys_in_order(&self) -> Vec<u64> {
        self.recs.keys().copied().collect()
    }
}

impl SleighCompile {
    /// C++ `SleighCompile::checkConsistency()` (slgh_compile.cc:2148): run the
    /// three ConsistencyChecker passes, bumping the driver error count on a
    /// fatal pass.
    pub(crate) fn check_consistency_real(&mut self) {
        let mut cc = CcState::default();
        self.set_post_order(&mut cc);
        if !self.test_size_restrictions(&mut cc) {
            self.bump_error();
            return;
        }
        if !self.test_truncations(&cc) {
            self.bump_error();
            return;
        }
        cc.unnecessarypcode = self.cc_take_unnecessary();
        if !self.warnunnecessarypcode() && cc.unnecessarypcode > 0 {
            self.report_warning_plain(&format!(
                "{} unnecessary extensions/truncations were converted to copies",
                cc.unnecessarypcode
            ));
            self.report_warning_plain("Use -u switch to list each individually");
        }
        self.optimize_all(&mut cc);
        if cc.readnowrite > 0 {
            self.bump_error();
            return;
        }
        if !self.warndeadtemps() && cc.writenoread > 0 {
            self.report_warning_plain(&format!(
                "{} operations wrote to temporaries that were not read",
                cc.writenoread
            ));
            self.report_warning_plain("Use -t switch to list each individually");
        }
        self.test_large_temporary(&cc);
    }

    // --- post-order subtable traversal (setPostOrder) ---

    fn set_post_order(&self, cc: &mut CcState) {
        let root = match self.root_id() {
            Some(r) => r,
            None => return,
        };
        cc.postorder.clear();
        cc.sizemap.clear();

        let mut path: Vec<SymbolId> = Vec::new();
        let mut state: Vec<i32> = Vec::new();
        let mut ctstate: Vec<i32> = Vec::new();

        cc.sizemap.insert(root, -1);
        path.push(root);
        state.push(0);
        ctstate.push(0);

        while let Some(&cur) = path.last() {
            let ctind = *state.last().unwrap();
            let numconst = self.subtable_num_constructors(cur);
            if ctind >= numconst {
                path.pop();
                state.pop();
                ctstate.pop();
                cc.postorder.push(cur);
            } else {
                let oper = *ctstate.last().unwrap();
                let numoper = self.constructor_num_operands(cur, ctind as u32);
                if oper >= numoper {
                    *state.last_mut().unwrap() = ctind + 1;
                    *ctstate.last_mut().unwrap() = 0;
                } else {
                    *ctstate.last_mut().unwrap() = oper + 1;
                    if let Some(subsym) =
                        self.operand_defining_subtable(cur, ctind as u32, oper as u32)
                    {
                        if !cc.sizemap.contains_key(&subsym) {
                            cc.sizemap.insert(subsym, -1);
                            path.push(subsym);
                            state.push(0);
                            ctstate.push(0);
                        }
                    }
                }
            }
        }
    }

    // --- pass 1: size restrictions (testSizeRestrictions) ---

    fn test_size_restrictions(&mut self, cc: &mut CcState) -> bool {
        let mut testresult = true;
        for i in 0..cc.postorder.len() {
            let sym = cc.postorder[i];
            if !self.check_subtable(sym, cc) {
                testresult = false;
            }
        }
        testresult
    }

    fn check_subtable(&mut self, sym: SymbolId, cc: &mut CcState) -> bool {
        let mut tablesize: i32 = -1;
        let numconstruct = self.subtable_num_constructors(sym);
        let mut testresult = true;
        let mut seenemptyexport = false;
        let mut seennonemptyexport = false;

        for i in 0..numconstruct {
            let ctidx = i as u32;
            let main_handle = self.constructor_templ_handle(sym, ctidx);
            if !self.check_constructor_section(sym, ctidx, main_handle, cc) {
                testresult = false;
            }
            let numsection = self.constructor_num_sections(sym, ctidx);
            for j in 0..numsection {
                let nh = self.constructor_named_templ_handle(sym, ctidx, j);
                if !self.check_constructor_section(sym, ctidx, nh, cc) {
                    testresult = false;
                }
            }

            if main_handle.is_none() {
                continue; // Unimplemented
            }
            let exportsize = self.constructor_export_size(main_handle, sym, ctidx, cc);
            match exportsize {
                Some(exsize) => {
                    if seenemptyexport && !seennonemptyexport {
                        let line = self.constructor_lineno(sym, ctidx);
                        self.cc_report_error_ct(sym, ctidx, &format!(
                            "Table '{}' exports inconsistently; Constructor starting at line {} is first inconsistency",
                            self.symbol_name_str(sym), line));
                        testresult = false;
                    }
                    seennonemptyexport = true;
                    if tablesize == -1 {
                        tablesize = exsize;
                    }
                    if exsize != tablesize {
                        let line = self.constructor_lineno(sym, ctidx);
                        self.cc_report_error_ct(sym, ctidx, &format!(
                            "Table '{}' has inconsistent export size; Constructor starting at line {} is first conflict",
                            self.symbol_name_str(sym), line));
                        testresult = false;
                    }
                }
                None => {
                    if seennonemptyexport && !seenemptyexport {
                        let line = self.constructor_lineno(sym, ctidx);
                        self.cc_report_error_ct(sym, ctidx, &format!(
                            "Table '{}' exports inconsistently; Constructor starting at line {} is first inconsistency",
                            self.symbol_name_str(sym), line));
                        testresult = false;
                    }
                    seenemptyexport = true;
                }
            }
        }
        if seennonemptyexport {
            if tablesize == 0 {
                self.report_warning_plain(&format!(
                    "Table '{}' exports size 0",
                    self.symbol_name_str(sym)
                ));
            }
            cc.sizemap.insert(sym, tablesize);
        } else {
            cc.sizemap.insert(sym, -1);
        }
        testresult
    }

    /// Recover the export size of a constructor's main section (or `None` if no
    /// export).  Mirrors the inline in `checkSubtable`.
    fn constructor_export_size(
        &mut self,
        handle: Option<ConstructTplHandle>,
        sym: SymbolId,
        ctidx: u32,
        cc: &CcState,
    ) -> Option<i32> {
        let h = handle?;
        let szconst = self
            .base
            .templates()
            .get(h)
            .and_then(|t| t.get_result())
            .map(|r| r.get_size().clone())?;
        match self.recover_size(&szconst, sym, ctidx, cc) {
            Ok(sz) => Some(sz),
            Err(_) => Some(-1),
        }
    }

    fn check_constructor_section(
        &mut self,
        sym: SymbolId,
        ctidx: u32,
        handle: Option<ConstructTplHandle>,
        cc: &CcState,
    ) -> bool {
        let h = match handle {
            Some(h) => h,
            None => return true,
        };
        let numops = self.base.templates().get(h).map(|t| t.get_opvec().len()).unwrap_or(0);
        let mut testresult = true;
        for i in 0..numops {
            if !self.size_restriction(h, i, sym, ctidx, cc) {
                testresult = false;
            }
            if !self.check_op_misuse(h, i, sym, ctidx) {
                testresult = false;
            }
        }
        testresult
    }

    /// C++ `recoverSize(const ConstTpl &sizeconst,Constructor *ct)`.
    fn recover_size(
        &self,
        sizeconst: &ConstTpl,
        sym: SymbolId,
        ctidx: u32,
        cc: &CcState,
    ) -> Result<i32, ()> {
        match sizeconst.get_type() {
            ConstType::Real => Ok(sizeconst.get_real() as i32),
            ConstType::Handle => {
                let handindex = sizeconst.get_handle_index();
                let opid = self.constructor_operand_id(sym, ctidx, handindex)?;
                let mut size = self
                    .base
                    .symtab()
                    .find_symbol_by_id(opid)
                    .and_then(|s| s.get_size(self.base.symtab()).ok())
                    .unwrap_or(0);
                if size == -1 {
                    // The operand's defining symbol must be a subtable.
                    let subid = self
                        .operand_defining_subtable_by_op(opid)
                        .ok_or(())?;
                    size = *cc.sizemap.get(&subid).ok_or(())?;
                }
                Ok(size)
            }
            _ => Err(()),
        }
    }

    /// C++ `sizeRestriction(OpTpl *op,Constructor *ct)`: per-opcode size rules.
    /// Mutates the template (unnecessary ext/trunc -> COPY).
    fn size_restriction(
        &mut self,
        h: ConstructTplHandle,
        opidx: usize,
        sym: SymbolId,
        ctidx: u32,
        cc: &CcState,
    ) -> bool {
        let opcode = self.template_op_opcode(h, opidx);
        // `recover` helper closure analog: read a varnode size from the op.
        macro_rules! rsize {
            ($slot:expr) => {{
                let szc = self.template_vn_size(h, opidx, $slot);
                match self.recover_size(&szc, sym, ctidx, cc) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            }};
        }
        use OpCode::*;
        match opcode {
            CPUI_COPY | CPUI_INT_2COMP | CPUI_INT_NEGATE | CPUI_FLOAT_NEG | CPUI_FLOAT_ABS
            | CPUI_FLOAT_SQRT | CPUI_FLOAT_CEIL | CPUI_FLOAT_FLOOR | CPUI_FLOAT_ROUND => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                if vnout == vn0 {
                    return true;
                }
                if vnout == 0 || vn0 == 0 {
                    return true;
                }
                self.op_err(h, opidx, sym, ctidx, -1, 0, "Input and output sizes must match")
            }
            CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_XOR | CPUI_INT_AND | CPUI_INT_OR
            | CPUI_INT_MULT | CPUI_INT_DIV | CPUI_INT_SDIV | CPUI_INT_REM | CPUI_INT_SREM
            | CPUI_FLOAT_ADD | CPUI_FLOAT_DIV | CPUI_FLOAT_MULT | CPUI_FLOAT_SUB => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                let vn1 = match rsize!(1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 1, 1, "Using subtable with exports in expression"),
                };
                if vnout != 0 && vn0 != 0 && vnout != vn0 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 0, "The output and all input sizes must match");
                }
                if vnout != 0 && vn1 != 0 && vnout != vn1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 1, "The output and all input sizes must match");
                }
                if vn0 != 0 && vn1 != 0 && vn0 != vn1 {
                    return self.op_err(h, opidx, sym, ctidx, 0, 1, "The output and all input sizes must match");
                }
                true
            }
            CPUI_FLOAT_NAN => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                if vnout != 1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, -1, "Output must be a boolean (size 1)");
                }
                true
            }
            CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL
            | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_CARRY | CPUI_INT_SCARRY
            | CPUI_INT_SBORROW | CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_LESS
            | CPUI_FLOAT_LESSEQUAL => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                if vnout != 1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, -1, "Output must be a boolean (size 1)");
                }
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                let vn1 = match rsize!(1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 1, 1, "Using subtable with exports in expression"),
                };
                if vn0 == 0 || vn1 == 0 {
                    return true;
                }
                if vn0 != vn1 {
                    return self.op_err(h, opidx, sym, ctidx, 0, 1, "Inputs must be the same size");
                }
                true
            }
            CPUI_BOOL_XOR | CPUI_BOOL_AND | CPUI_BOOL_OR => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                if vnout != 1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, -1, "Output must be a boolean (size 1)");
                }
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                if vn0 != 1 {
                    return self.op_err(h, opidx, sym, ctidx, 0, 0, "Input must be a boolean (size 1)");
                }
                true
            }
            CPUI_BOOL_NEGATE => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                if vnout != 1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, -1, "Output must be a boolean (size 1)");
                }
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                if vn0 != 1 {
                    return self.op_err(h, opidx, sym, ctidx, 0, 0, "Input must be a boolean (size 1)");
                }
                true
            }
            CPUI_INT_LEFT | CPUI_INT_RIGHT | CPUI_INT_SRIGHT => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                if vnout == 0 || vn0 == 0 {
                    return true;
                }
                if vnout != vn0 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 0, "Output and first input must be the same size");
                }
                true
            }
            CPUI_INT_ZEXT | CPUI_INT_SEXT => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                if vnout == 0 || vn0 == 0 {
                    return true;
                }
                if vnout == vn0 {
                    self.deal_with_unnecessary_ext(h, opidx, sym, ctidx);
                    return true;
                } else if vnout < vn0 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 0, "Output size must be strictly bigger than input size");
                }
                true
            }
            CPUI_CBRANCH => {
                let vn1 = match rsize!(1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 1, 1, "Using subtable with exports in expression"),
                };
                if vn1 != 1 {
                    return self.op_err(h, opidx, sym, ctidx, 1, 1, "Input must be a boolean (size 1)");
                }
                true
            }
            CPUI_LOAD | CPUI_STORE => {
                let off0 = self.template_vn_offset(h, opidx, 0);
                if off0.get_type() != ConstType::Spaceid {
                    return true;
                }
                let spc = off0.get_space();
                let vn1 = match rsize!(1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 1, 1, "Using subtable with exports in expression"),
                };
                if vn1 != 0 && vn1 != spc.get_addr_size() as i32 {
                    return self.op_err(h, opidx, sym, ctidx, 1, 1, "Pointer size must match size of space");
                }
                true
            }
            CPUI_SUBPIECE => {
                let vnout = match rsize!(-1) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, -1, -1, "Using subtable with exports in expression"),
                };
                let vn0 = match rsize!(0) {
                    Some(v) => v,
                    None => return self.op_err(h, opidx, sym, ctidx, 0, 0, "Using subtable with exports in expression"),
                };
                let vn1 = self.template_vn_offset(h, opidx, 1).get_real() as i32;
                if vnout == 0 || vn0 == 0 {
                    return true;
                }
                if vnout == vn0 && vn1 == 0 {
                    self.deal_with_unnecessary_trunc(h, opidx, sym, ctidx);
                    return true;
                } else if vnout >= vn0 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 0, "Output must be strictly smaller than input");
                }
                if vnout > vn0 - vn1 {
                    return self.op_err(h, opidx, sym, ctidx, -1, 0, "Too much truncation");
                }
                true
            }
            _ => true,
        }
    }

    /// C++ `checkOpMisuse`: unsigned-less-than-zero warning.
    fn check_op_misuse(&mut self, h: ConstructTplHandle, opidx: usize, sym: SymbolId, ctidx: u32) -> bool {
        if self.template_op_opcode(h, opidx) == OpCode::CPUI_INT_LESS {
            let is_const = self
                .base
                .templates()
                .get(h)
                .map(|t| {
                    let op = &t.get_opvec()[opidx];
                    let vn = op.get_in(1);
                    vn.get_space().is_const_space() && vn.get_offset().is_zero()
                })
                .unwrap_or(false);
            if is_const {
                self.cc_report_warning_ct(sym, ctidx, "Unsigned comparison with zero is always false");
            }
        }
        true
    }

    fn deal_with_unnecessary_ext(&mut self, h: ConstructTplHandle, opidx: usize, sym: SymbolId, ctidx: u32) {
        if self.warnunnecessarypcode() {
            let nm = self.op_name(h, opidx);
            self.cc_report_warning_ct(sym, ctidx, &format!("Unnecessary {nm}"));
        }
        if let Some(t) = self.base.template_mut(h) {
            t.get_opvec_mut()[opidx].set_opcode(OpCode::CPUI_COPY);
        }
        self.cc_bump_unnecessary();
    }

    fn deal_with_unnecessary_trunc(&mut self, h: ConstructTplHandle, opidx: usize, sym: SymbolId, ctidx: u32) {
        if self.warnunnecessarypcode() {
            let nm = self.op_name(h, opidx);
            self.cc_report_warning_ct(sym, ctidx, &format!("Unnecessary {nm}"));
        }
        if let Some(t) = self.base.template_mut(h) {
            let op = &mut t.get_opvec_mut()[opidx];
            op.set_opcode(OpCode::CPUI_COPY);
            op.remove_input(1);
        }
        self.cc_bump_unnecessary();
    }

    // --- pass 2: truncations (testTruncations) ---

    fn test_truncations(&mut self, cc: &CcState) -> bool {
        let isbig = self.base.is_big_endian();
        let mut testresult = true;
        for i in 0..cc.postorder.len() {
            let sym = cc.postorder[i];
            let numconstruct = self.subtable_num_constructors(sym);
            for j in 0..numconstruct {
                let ctidx = j as u32;
                let numsections = self.constructor_num_sections(sym, ctidx);
                for k in -1..numsections {
                    let h = if k < 0 {
                        self.constructor_templ_handle(sym, ctidx)
                    } else {
                        self.constructor_named_templ_handle(sym, ctidx, k)
                    };
                    if let Some(h) = h {
                        if !self.check_section_truncations(sym, ctidx, h, isbig, cc) {
                            testresult = false;
                        }
                    }
                }
            }
        }
        testresult
    }

    fn check_section_truncations(
        &mut self,
        sym: SymbolId,
        ctidx: u32,
        h: ConstructTplHandle,
        isbig: bool,
        cc: &CcState,
    ) -> bool {
        let numops = self.base.templates().get(h).map(|t| t.get_opvec().len()).unwrap_or(0);
        let mut testresult = true;
        for i in 0..numops {
            // output
            if self.template_has_out(h, i)
                && !self.check_varnode_truncation(sym, ctidx, h, i, -1, isbig, cc)
            {
                testresult = false;
            }
            let ninput = self.template_op_num_input(h, i);
            for j in 0..ninput {
                if !self.check_varnode_truncation(sym, ctidx, h, i, j, isbig, cc) {
                    testresult = false;
                }
            }
        }
        testresult
    }

    fn check_varnode_truncation(
        &mut self,
        sym: SymbolId,
        ctidx: u32,
        h: ConstructTplHandle,
        opidx: usize,
        slot: i32,
        isbig: bool,
        cc: &CcState,
    ) -> bool {
        let off = self.template_vn_offset_for(h, opidx, slot);
        if off.get_type() != ConstType::Handle {
            return true;
        }
        if off.get_select() != VField::VOffsetPlus {
            return true;
        }
        let sztype = self.template_vn_size_for(h, opidx, slot).get_type();
        if sztype != ConstType::Real && sztype != ConstType::Handle {
            return self.op_err(h, opidx, sym, ctidx, slot, slot, "Bad truncation expression");
        }
        let sz = match self.recover_size(&off, sym, ctidx, cc) {
            Ok(v) => v,
            Err(_) => return self.op_err(h, opidx, sym, ctidx, slot, slot, "Could not recover size"),
        };
        if sz <= 0 {
            return self.op_err(h, opidx, sym, ctidx, slot, slot, "Could not recover size");
        }
        let res = if let Some(t) = self.base.template_mut(h) {
            let op = &mut t.get_opvec_mut()[opidx];
            let vn: &mut VarnodeTpl = if slot < 0 {
                op.get_out_mut().expect("out present")
            } else {
                op.get_in_mut(slot)
            };
            vn.adjust_truncation(sz, isbig)
        } else {
            true
        };
        if !res {
            return self.op_err(h, opidx, sym, ctidx, slot, slot, "Truncation operator out of bounds");
        }
        true
    }

    // --- pass 3: optimization (optimizeAll) ---

    fn optimize_all(&mut self, cc: &mut CcState) {
        for i in 0..cc.postorder.len() {
            let sym = cc.postorder[i];
            let numconstruct = self.subtable_num_constructors(sym);
            for j in 0..numconstruct {
                self.optimize(sym, j as u32, cc);
            }
        }
    }

    fn optimize(&mut self, sym: SymbolId, ctidx: u32, cc: &mut CcState) {
        let numsections = self.constructor_num_sections(sym, ctidx);
        let mut state = UniqueState::default();
        loop {
            state.clear();
            for i in -1..numsections {
                self.optimize_gather1(sym, ctidx, &mut state, i);
                self.optimize_gather2(sym, ctidx, &mut state, i);
            }
            match self.find_valid_rule(sym, ctidx, &mut state) {
                Some(rec) => self.apply_optimization(sym, ctidx, &rec),
                None => break,
            }
        }
        self.check_unused_temps(sym, ctidx, &state, cc);
    }

    fn optimize_gather1(&self, sym: SymbolId, ctidx: u32, state: &mut UniqueState, secnum: i32) {
        let h = self.section_handle(sym, ctidx, secnum);
        let h = match h {
            Some(h) => h,
            None => return,
        };
        let numops = self.base.templates().get(h).map(|t| t.get_opvec().len()).unwrap_or(0);
        for i in 0..numops {
            let ninput = self.template_op_num_input(h, i);
            for j in 0..ninput {
                let vn = self.template_vn_clone(h, i, j);
                examine_vn(state, &vn, i as u32, j, secnum);
            }
            if self.template_has_out(h, i) {
                let vn = self.template_vn_clone(h, i, -1);
                examine_vn(state, &vn, i as u32, -1, secnum);
            }
        }
    }

    fn optimize_gather2(&self, sym: SymbolId, ctidx: u32, state: &mut UniqueState, secnum: i32) {
        let h = match self.section_handle(sym, ctidx, secnum) {
            Some(h) => h,
            None => return,
        };
        let result = match self.base.templates().get(h).and_then(|t| t.get_result()) {
            Some(r) => r,
            None => return,
        };
        // Two near-identical clauses in C++ (ptrspace and space).
        let ptr_is_unique = result.get_ptr_space().is_unique_space();
        let space_is_unique = result.get_space().is_unique_space();
        let ptroff_real = result.get_ptr_offset().get_type() == ConstType::Real;
        let ptrspace_real = result.get_ptr_space().get_type() == ConstType::Real;
        let offset = result.get_ptr_offset().get_real();
        let size = result.get_ptr_size().get_real() as i32;
        if ptr_is_unique && ptroff_real {
            for k in state.get_definition_keys(offset, size) {
                state.recs.get_mut(&k).unwrap().update_export();
            }
        }
        if space_is_unique && ptrspace_real && ptroff_real {
            for k in state.get_definition_keys(offset, size) {
                state.recs.get_mut(&k).unwrap().update_export();
            }
        }
    }

    fn find_valid_rule(&self, sym: SymbolId, ctidx: u32, state: &mut UniqueState) -> Option<OptimizeRecord> {
        for key in state.keys_in_order() {
            let currec = state.recs[&key].clone();
            if currec.writecount == 1 && currec.readcount == 1 && currec.readsection == currec.writesection {
                let h = self.section_handle(sym, ctidx, currec.readsection)?;
                if currec.writeop >= currec.readop {
                    // C++ throws SleighError; we treat as no rule (the caller's
                    // size pass already errored on genuinely malformed p-code).
                    continue;
                }
                let writevn = self.template_vn_clone(h, currec.writeop as usize, -1);
                let readvn = self.template_vn_clone(h, currec.readop as usize, currec.inslot);
                if writevn != readvn {
                    continue;
                }
                let readop_code = self.template_op_opcode(h, currec.readop as usize);
                let writeop_code = self.template_op_opcode(h, currec.writeop as usize);
                if readop_code == OpCode::CPUI_COPY {
                    let mut rec = currec.clone();
                    rec.opttype = 0;
                    let vn = self.template_vn_clone(h, currec.readop as usize, -1);
                    let mut save = true;
                    for i in (currec.writeop + 1)..currec.readop {
                        if self.read_write_interference(h, &vn, i as usize, true) {
                            save = false;
                            break;
                        }
                    }
                    if save {
                        return Some(rec);
                    }
                }
                if writeop_code == OpCode::CPUI_COPY {
                    let mut rec = currec.clone();
                    rec.opttype = 1;
                    let vn = self.template_vn_clone(h, currec.writeop as usize, 0);
                    let mut save = true;
                    for i in (currec.writeop + 1)..currec.readop {
                        if self.read_write_interference(h, &vn, i as usize, false) {
                            save = false;
                            break;
                        }
                    }
                    if save {
                        return Some(rec);
                    }
                }
            }
        }
        None
    }

    fn apply_optimization(&mut self, sym: SymbolId, ctidx: u32, rec: &OptimizeRecord) {
        let h = match self.section_handle(sym, ctidx, rec.readsection) {
            Some(h) => h,
            None => return,
        };
        if rec.opttype == 0 {
            // Read op is COPY: COPY's output becomes the write op's output.
            let vnout = self
                .base
                .templates()
                .get(h)
                .map(|t| t.get_opvec()[rec.readop as usize].get_out().expect("out").clone())
                .unwrap();
            if let Some(t) = self.base.template_mut(h) {
                t.set_output(vnout, rec.writeop);
                t.delete_ops(&[rec.readop]);
            }
        } else if rec.opttype == 1 {
            // Write op is COPY: COPY's input becomes the read op's input.
            let vnin = self
                .base
                .templates()
                .get(h)
                .map(|t| t.get_opvec()[rec.writeop as usize].get_in(0).clone())
                .unwrap();
            if let Some(t) = self.base.template_mut(h) {
                t.set_input(vnin, rec.readop, rec.inslot);
                t.delete_ops(&[rec.writeop]);
            }
        }
    }

    fn check_unused_temps(&mut self, sym: SymbolId, ctidx: u32, state: &UniqueState, cc: &mut CcState) {
        for key in state.keys_in_order() {
            let currec = &state.recs[&key];
            if currec.readcount == 0 {
                if self.warndeadtemps() {
                    self.cc_report_warning_ct(sym, ctidx, "Temporary is written but not read");
                }
                cc.writenoread += 1;
            } else if currec.writecount == 0 {
                self.cc_report_error_ct(sym, ctidx, "Temporary is read but not written");
                cc.readnowrite += 1;
            }
        }
    }

    fn read_write_interference(&self, h: ConstructTplHandle, vn: &VarnodeTpl, opidx: usize, checkread: bool) -> bool {
        let opcode = self.template_op_opcode(h, opidx);
        if matches!(
            opcode,
            BUILD
                | CROSSBUILD
                | DELAY_SLOT
                | MACROBUILD
                | OpCode::CPUI_LOAD
                | OpCode::CPUI_STORE
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
        ) || opcode == LABELBUILD
        {
            return true;
        }
        if checkread {
            let ninput = self.template_op_num_input(h, opidx);
            for i in 0..ninput {
                let other = self.template_vn_clone(h, opidx, i);
                if possible_intersection(vn, &other) {
                    return true;
                }
            }
        }
        if self.template_has_out(h, opidx) {
            let other = self.template_vn_clone(h, opidx, -1);
            if possible_intersection(vn, &other) {
                return true;
            }
        }
        false
    }

    // --- testLargeTemporary ---

    fn test_large_temporary(&mut self, cc: &CcState) {
        for i in 0..cc.postorder.len() {
            let sym = cc.postorder[i];
            let numconstruct = self.subtable_num_constructors(sym);
            for j in 0..numconstruct {
                let ctidx = j as u32;
                let numsections = self.constructor_num_sections(sym, ctidx);
                for k in -1..numsections {
                    let h = if k < 0 {
                        self.constructor_templ_handle(sym, ctidx)
                    } else {
                        self.constructor_named_templ_handle(sym, ctidx, k)
                    };
                    if let Some(h) = h {
                        self.check_large_temporaries(sym, ctidx, h);
                    }
                }
            }
        }
    }

    fn check_large_temporaries(&mut self, sym: SymbolId, ctidx: u32, h: ConstructTplHandle) {
        let numops = self.base.templates().get(h).map(|t| t.get_opvec().len()).unwrap_or(0);
        for i in 0..numops {
            if self.has_large_temporary(h, i) {
                self.cc_report_error_ct(
                    sym,
                    ctidx,
                    &format!(
                        "Constructor uses temporary varnode larger than {MAX_UNIQUE_SIZE} bytes."
                    ),
                );
                return;
            }
        }
    }

    fn has_large_temporary(&self, h: ConstructTplHandle, opidx: usize) -> bool {
        let t = match self.base.templates().get(h) {
            Some(t) => t,
            None => return false,
        };
        let op = &t.get_opvec()[opidx];
        if let Some(out) = op.get_out() {
            if is_temp_too_big(out) {
                return true;
            }
        }
        for i in 0..op.num_input() {
            if is_temp_too_big(op.get_in(i)) {
                return true;
            }
        }
        false
    }
}

/// C++ `examineVn`: accumulate read/write info if `vn` is a temporary.
fn examine_vn(state: &mut UniqueState, vn: &VarnodeTpl, i: u32, inslot: i32, secnum: i32) {
    if !vn.get_space().is_unique_space() {
        return;
    }
    if vn.get_offset().get_type() != ConstType::Real {
        return;
    }
    let offset = vn.get_offset().get_real();
    let size = vn.get_size().get_real() as i32;
    if inslot >= 0 {
        for k in state.get_definition_keys(offset, size) {
            state.recs.get_mut(&k).unwrap().update_read(i as i32, inslot, secnum);
        }
    } else {
        let mut rec = OptimizeRecord::new(offset, size);
        rec.update_write(i as i32, secnum);
        state.set(rec);
    }
}

/// C++ `possibleIntersection`: conservative storage-overlap test.
fn possible_intersection(vn1: &VarnodeTpl, vn2: &VarnodeTpl) -> bool {
    if vn1.get_space().is_const_space() {
        return false;
    }
    if vn2.get_space().is_const_space() {
        return false;
    }
    let u1 = vn1.get_space().is_unique_space();
    let u2 = vn2.get_space().is_unique_space();
    if u1 != u2 {
        return false;
    }
    if vn1.get_space().get_type() != ConstType::Spaceid {
        return true;
    }
    if vn2.get_space().get_type() != ConstType::Spaceid {
        return true;
    }
    let spc1 = vn1.get_space().get_space();
    let spc2 = vn2.get_space().get_space();
    if spc1.get_index() != spc2.get_index() {
        return false;
    }
    if vn2.get_offset().get_type() != ConstType::Real {
        return true;
    }
    if vn2.get_size().get_type() != ConstType::Real {
        return true;
    }
    if vn1.get_offset().get_type() != ConstType::Real {
        return true;
    }
    if vn1.get_size().get_type() != ConstType::Real {
        return true;
    }
    let offset = vn1.get_offset().get_real();
    let size = vn1.get_size().get_real();
    let off = vn2.get_offset().get_real();
    if off + vn2.get_size().get_real() - 1 < offset {
        return false;
    }
    if off > offset + size - 1 {
        return false;
    }
    true
}

fn is_temp_too_big(vn: &VarnodeTpl) -> bool {
    vn.get_space().is_unique_space() && vn.get_size().get_real() > MAX_UNIQUE_SIZE
}

/// Scratch state for the ConsistencyChecker (the per-run fields the C++ class
/// holds: `postorder`, `sizemap`, the four counters).
#[derive(Default)]
pub(crate) struct CcState {
    postorder: Vec<SymbolId>,
    sizemap: BTreeMap<SymbolId, i32>,
    unnecessarypcode: i32,
    readnowrite: i32,
    writenoread: i32,
}

// ---------------------------------------------------------------------------
// Driver accessors used by the checker (kept here so consistency.rs is the only
// owner of the checker logic; they reach into SleighCompile's base/symtab).
// ---------------------------------------------------------------------------

impl SleighCompile {
    fn root_id(&self) -> Option<SymbolId> {
        self.base.get_root()
    }
    fn warnunnecessarypcode(&self) -> bool {
        self.cc_warn_unnecessary()
    }
    fn warndeadtemps(&self) -> bool {
        self.cc_warn_deadtemps()
    }
    fn subtable_num_constructors(&self, sym: SymbolId) -> i32 {
        self.base
            .symtab()
            .find_symbol_by_id(sym)
            .and_then(|s| s.as_subtable())
            .map(|st| st.get_num_constructors())
            .unwrap_or(0)
    }
    fn constructor_num_operands(&self, sym: SymbolId, ctidx: u32) -> i32 {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .map(|ct| ct.get_num_operands())
            .unwrap_or(0)
    }
    fn constructor_num_sections(&self, sym: SymbolId, ctidx: u32) -> i32 {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .map(|ct| ct.get_num_sections())
            .unwrap_or(0)
    }
    fn constructor_lineno(&self, sym: SymbolId, ctidx: u32) -> i32 {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .map(|ct| ct.get_lineno())
            .unwrap_or(0)
    }
    fn constructor_templ_handle(&self, sym: SymbolId, ctidx: u32) -> Option<ConstructTplHandle> {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .ok()
            .and_then(|ct| ct.get_templ())
    }
    fn constructor_named_templ_handle(&self, sym: SymbolId, ctidx: u32, sec: i32) -> Option<ConstructTplHandle> {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .ok()
            .and_then(|ct| ct.get_named_templ(sec))
    }
    fn section_handle(&self, sym: SymbolId, ctidx: u32, secnum: i32) -> Option<ConstructTplHandle> {
        if secnum < 0 {
            self.constructor_templ_handle(sym, ctidx)
        } else {
            self.constructor_named_templ_handle(sym, ctidx, secnum)
        }
    }
    fn constructor_operand_id(&self, sym: SymbolId, ctidx: u32, hand: i32) -> Result<SymbolId, ()> {
        self.base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .ok()
            .and_then(|ct| ct.get_operand(hand).ok())
            .ok_or(())
    }
    /// The subtable that defines a constructor's `oper`-th operand, or `None`.
    fn operand_defining_subtable(&self, sym: SymbolId, ctidx: u32, oper: u32) -> Option<SymbolId> {
        let opid = self
            .base
            .symtab()
            .get_constructor(ConstructorRef { table_id: sym, ct_id: ctidx })
            .ok()
            .and_then(|ct| ct.get_operand(oper as i32).ok())?;
        self.operand_defining_subtable_by_op(opid)
    }
    fn operand_defining_subtable_by_op(&self, opid: SymbolId) -> Option<SymbolId> {
        let defid = self
            .base
            .symtab()
            .find_symbol_by_id(opid)
            .and_then(|s| match s.kind() {
                SymbolKind::Operand(op) => op.get_defining_symbol(),
                _ => None,
            })?;
        let is_sub = self
            .base
            .symtab()
            .find_symbol_by_id(defid)
            .map(|s| s.get_type() == SymbolType::Subtable)
            .unwrap_or(false);
        if is_sub {
            Some(defid)
        } else {
            None
        }
    }
    fn symbol_name_str(&self, sym: SymbolId) -> String {
        String::from_utf8_lossy(
            self.base
                .symtab()
                .find_symbol_by_id(sym)
                .map(|s| s.get_name())
                .unwrap_or(b""),
        )
        .into_owned()
    }

    // --- template element reads ---

    fn template_op_opcode(&self, h: ConstructTplHandle, opidx: usize) -> OpCode {
        self.base.templates().get(h).map(|t| t.get_opvec()[opidx].get_opcode()).unwrap()
    }
    fn template_op_num_input(&self, h: ConstructTplHandle, opidx: usize) -> i32 {
        self.base.templates().get(h).map(|t| t.get_opvec()[opidx].num_input()).unwrap_or(0)
    }
    fn template_has_out(&self, h: ConstructTplHandle, opidx: usize) -> bool {
        self.base.templates().get(h).map(|t| t.get_opvec()[opidx].get_out().is_some()).unwrap_or(false)
    }
    fn template_vn_clone(&self, h: ConstructTplHandle, opidx: usize, slot: i32) -> VarnodeTpl {
        let t = self.base.templates().get(h).unwrap();
        let op = &t.get_opvec()[opidx];
        if slot < 0 {
            op.get_out().expect("out present").clone()
        } else {
            op.get_in(slot).clone()
        }
    }
    fn template_vn_size(&self, h: ConstructTplHandle, opidx: usize, slot: i32) -> ConstTpl {
        self.template_vn_size_for(h, opidx, slot)
    }
    fn template_vn_size_for(&self, h: ConstructTplHandle, opidx: usize, slot: i32) -> ConstTpl {
        let t = self.base.templates().get(h).unwrap();
        let op = &t.get_opvec()[opidx];
        if slot < 0 {
            op.get_out().expect("out present").get_size().clone()
        } else {
            op.get_in(slot).get_size().clone()
        }
    }
    fn template_vn_offset(&self, h: ConstructTplHandle, opidx: usize, slot: i32) -> ConstTpl {
        self.template_vn_offset_for(h, opidx, slot)
    }
    fn template_vn_offset_for(&self, h: ConstructTplHandle, opidx: usize, slot: i32) -> ConstTpl {
        let t = self.base.templates().get(h).unwrap();
        let op = &t.get_opvec()[opidx];
        if slot < 0 {
            op.get_out().expect("out present").get_offset().clone()
        } else {
            op.get_in(slot).get_offset().clone()
        }
    }
    fn op_name(&self, h: ConstructTplHandle, opidx: usize) -> String {
        op_name_string(self.template_op_opcode(h, opidx))
    }

    // --- error/warning reporters keyed by constructor ---

    fn op_err(
        &mut self,
        h: ConstructTplHandle,
        opidx: usize,
        sym: SymbolId,
        ctidx: u32,
        err1: i32,
        err2: i32,
        msg: &str,
    ) -> bool {
        // C++ printOpError: build the table/operand-aware error.
        let table_name = self.symbol_name_str(sym);
        let op1 = self.operand_name_for_slot(h, opidx, sym, ctidx, err1);
        let op2 = if err2 != err1 {
            self.operand_name_for_slot(h, opidx, sym, ctidx, err2)
        } else {
            None
        };
        let problem = match (&op1, &op2) {
            (Some(a), Some(b)) => format!("  Problem with operands '{a}' and '{b}'"),
            (Some(a), None) => format!("  Problem with operand 1 '{a}'"),
            (None, Some(b)) => format!("  Problem with operand 2 '{b}'"),
            (None, None) => "  Problem".to_string(),
        };
        let opname = self.op_name(h, opidx);
        let full = format!(
            "Size restriction error in table '{table_name}'\n{problem} in {opname} operator\n  {msg}"
        );
        self.cc_report_error_ct(sym, ctidx, &full);
        false
    }

    /// C++ `getOperandSymbol(slot,op,ct)`: if the slot varnode's *size* is a
    /// handle, the operand at that handle index.
    fn operand_name_for_slot(
        &self,
        h: ConstructTplHandle,
        opidx: usize,
        sym: SymbolId,
        ctidx: u32,
        slot: i32,
    ) -> Option<String> {
        let szc = self.template_vn_size_for(h, opidx, slot);
        if szc.get_type() != ConstType::Handle {
            return None;
        }
        let handindex = szc.get_handle_index();
        let opid = self.constructor_operand_id(sym, ctidx, handindex).ok()?;
        Some(self.symbol_name_str(opid))
    }
}

/// C++ `printOpName`: the operator's syntax name for messages.  Only the names
/// that appear in `sizeRestriction`/`checkOpMisuse` paths need be exact.
fn op_name_string(opc: OpCode) -> String {
    use OpCode::*;
    let s = match opc {
        CPUI_COPY => "Copy(=)",
        CPUI_INT_ADD => "Add(+)",
        CPUI_INT_SUB => "Subtract(-)",
        CPUI_INT_MULT => "Multiply(*)",
        CPUI_INT_DIV => "Divide(/)",
        CPUI_INT_SDIV => "Signed Divide(s/)",
        CPUI_INT_REM => "Remainder(%)",
        CPUI_INT_SREM => "Signed Remainder(s%)",
        CPUI_INT_2COMP => "Twos Complement(-)",
        CPUI_INT_NEGATE => "Negate(~)",
        CPUI_INT_XOR => "Xor(^)",
        CPUI_INT_AND => "And(&)",
        CPUI_INT_OR => "Or(|)",
        CPUI_INT_LEFT => "Left Shift(<<)",
        CPUI_INT_RIGHT => "Right Shift(>>)",
        CPUI_INT_SRIGHT => "Signed Right Shift(s>>)",
        CPUI_INT_EQUAL => "Equal(==)",
        CPUI_INT_NOTEQUAL => "Not Equal(!=)",
        CPUI_INT_LESS => "Less(<)",
        CPUI_INT_LESSEQUAL => "Less Equal(<=)",
        CPUI_INT_SLESS => "Signed Less(s<)",
        CPUI_INT_SLESSEQUAL => "Signed Less Equal(s<=)",
        CPUI_INT_ZEXT => "Zero Extension(zext)",
        CPUI_INT_SEXT => "Signed Extension(sext)",
        CPUI_INT_CARRY => "Carry(carry)",
        CPUI_INT_SCARRY => "Signed Carry(scarry)",
        CPUI_INT_SBORROW => "Signed Borrow(sborrow)",
        CPUI_BOOL_XOR => "Boolean Xor(^^)",
        CPUI_BOOL_AND => "Boolean And(&&)",
        CPUI_BOOL_OR => "Boolean Or(||)",
        CPUI_BOOL_NEGATE => "Boolean Negate(!)",
        CPUI_FLOAT_EQUAL => "Float Equal(f==)",
        CPUI_FLOAT_NOTEQUAL => "Float Not Equal(f!=)",
        CPUI_FLOAT_LESS => "Float Less(f<)",
        CPUI_FLOAT_LESSEQUAL => "Float Less Equal(f<=)",
        CPUI_FLOAT_ADD => "Float Add(f+)",
        CPUI_FLOAT_SUB => "Float Subtract(f-)",
        CPUI_FLOAT_MULT => "Float Multiply(f*)",
        CPUI_FLOAT_DIV => "Float Divide(f/)",
        CPUI_FLOAT_NEG => "Float Negate(f-)",
        CPUI_FLOAT_ABS => "Absolute Value(abs)",
        CPUI_FLOAT_SQRT => "Square Root(sqrt)",
        CPUI_FLOAT_CEIL => "Ceiling(ceil)",
        CPUI_FLOAT_FLOOR => "Floor(floor)",
        CPUI_FLOAT_ROUND => "Round(round)",
        CPUI_FLOAT_NAN => "Not a Number(nan)",
        CPUI_SUBPIECE => "Truncation(:)",
        CPUI_LOAD => "Load(*)",
        CPUI_STORE => "Store(*)",
        CPUI_CBRANCH => "Conditional Branch(if)",
        _ => "",
    };
    s.to_string()
}
