//! Port of `decompiler/cpp/codedata.{cc,hh}` — the `codedata` console module:
//! `CodeDataAnalysis` (a disassembly-target / control-flow model built directly
//! off the SLEIGH `Translate` and the `LoadImage` section map) plus its console
//! commands (`codedata init/target/run`, `codedata dump hits/crossrefs/starts/
//! unlinked/targethits`).
//!
//! # What is ported
//!
//! - [`CodeUnit`] (with its flag constants in [`code_unit_flags`]),
//!   [`DisassemblyResult`], [`DisassemblyEngine`] (the [`PcodeEmit`] that scans
//!   one decoded instruction for its fall-through / jump / call / target-hit
//!   shape), [`TargetHit`], [`TargetFeature`], [`AddrLink`], and
//!   [`CodeDataAnalysis`] (the `IfaceData` holding the whole model: code-unit
//!   map, the two cross-reference maps, the taint work-list, the unlinked /
//!   target-hit lists, and the model-hit range cover).  Every analysis method
//!   (`runModel` and all its helpers) is a verbatim transcription, BTree-keyed
//!   (ADR 0002).
//!
//! - The eight console-command actions ([`IfcCodeDataInit`],
//!   [`IfcCodeDataTarget`], [`IfcCodeDataRun`], and the five `dump` commands)
//!   plus [`register_commands`], the verbatim port of
//!   `IfaceCodeDataCapability::registerCommands`.
//!
//! # Cross-module boundaries
//!
//! The C++ `CodeDataAnalysis` caches `Architecture *glb` and reaches the
//! disassembler (`glb->translate`) and the load image (`glb->loader`) through
//! it.  In the Rust port the analysis object cannot own/borrow the
//! `Architecture` (it lives in the `"decompile"` module's `IfaceData`, owned
//! elsewhere in the datamap), so the engine state that depends on the
//! `Translate`/`LoadImage` is **threaded in per call** rather than cached:
//!
//! - [`DisassemblyEngine::disassemble`] takes the `&dyn Translate`.
//! - the `CodeDataAnalysis` model methods that decode
//!   ([`disassemble_block`](CodeDataAnalysis::disassemble_block),
//!   [`run_model`](CodeDataAnalysis::run_model), ...) take the `&dyn Translate`
//!   and (for `run_model`) the `&mut dyn LoadImage` plus the
//!   `&AddrSpaceManager` (for `getDefaultCodeSpace`).
//!
//! This is a structural deviation; the *computation* is byte-faithful.
//!
//! The console commands fetch the `Architecture` the same way C++
//! `IfaceCodeDataCommand::setData` does — `status.get_data("decompile")` — and
//! the `"decompile"` module's `IfaceData` (ported in `w9-con-ifacedecomp`)
//! surfaces it.  Until that lands, the command bodies that need the
//! architecture report the same execution error the C++ would hit dereferencing
//! a null `conf`; the analysis itself is fully testable against a synthetic
//! [`Translate`]/[`LoadImage`] directly (see this module's tests).
//!
//! # Documented losses
//!
//! - `IfcCodeDataTarget` in C++ reads the BFD import table
//!   (`LoadImageBfd::getImportTable`).  `loadimage_bfd` is GPL-3 (the file's own
//!   header note) and is **not** part of the kuna Rust port (no `ImportRecord`
//!   / `LoadImageBfd` exists).  The command is ported with the same dispatch and
//!   parse/error surface; the import-table lookup itself is a documented loss —
//!   it surfaces "Unable to find reference to call <name>" for any name, exactly
//!   the C++ not-found branch, since the (absent) import table is empty.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;
use std::rc::Rc;

use kuna_base::address::{Address, Range, RangeList};
use kuna_base::error::KunaError;
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageSection};
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::interface::{
    CommandStream, IfaceCommandAction, IfaceData, IfaceError, IfaceResult, IfaceStatus,
};

// ---------------------------------------------------------------------------
// CodeUnit (codedata.hh:34-50)
// ---------------------------------------------------------------------------

/// C++ `CodeUnit`'s anonymous flag enum (`codedata.hh:36-47`).
pub mod code_unit_flags {
    use kuna_base::types::uint4;

    /// Instruction falls through to the next unit
    pub const FALLTHRU: uint4 = 1;
    /// Instruction is a (direct) jump
    pub const JUMP: uint4 = 2;
    /// Instruction is a call
    pub const CALL: uint4 = 4;
    /// Unit is not (valid) code
    pub const NOTCODE: uint4 = 8;
    /// Another unit falls through into this one
    pub const HIT_BY_FALLTHRU: uint4 = 16;
    /// A jump targets this unit
    pub const HIT_BY_JUMP: uint4 = 32;
    /// A call targets this unit
    pub const HIT_BY_CALL: uint4 = 64;
    /// Likely an errant (mid-instruction) start
    pub const ERRANTSTART: uint4 = 128;
    /// Unit references a known target
    pub const TARGETHIT: uint4 = 256;
    /// Unit references a target via an indirect jump (a thunk)
    pub const THUNKHIT: uint4 = 512;
}

/// C++ `CodeUnit` (`codedata.hh:34`): one disassembled instruction's flags and
/// byte length.
#[derive(Debug, Clone, Copy, Default)]
pub struct CodeUnit {
    /// C++ `flags`: an OR of [`code_unit_flags`].
    pub flags: uint4,
    /// C++ `size`: the byte length of the instruction.
    pub size: int4,
}

// ---------------------------------------------------------------------------
// DisassemblyResult (codedata.hh:52-58)
// ---------------------------------------------------------------------------

/// C++ `DisassemblyResult` (`codedata.hh:52`): the outcome of decoding one
/// instruction.
#[derive(Debug, Clone)]
pub struct DisassemblyResult {
    /// C++ `success`: did the decode succeed?
    pub success: bool,
    /// C++ `length`: the instruction byte length.
    pub length: int4,
    /// C++ `flags`: the [`code_unit_flags`] inferred for the instruction.
    pub flags: uint4,
    /// C++ `jumpaddress`: a direct jump's destination (only meaningful when
    /// `JUMP` is set).
    pub jumpaddress: Address,
    /// C++ `targethit`: the target offset hit (only meaningful when `TARGETHIT`
    /// is set).
    pub targethit: uintb,
}

impl Default for DisassemblyResult {
    fn default() -> Self {
        DisassemblyResult {
            success: false,
            length: 0,
            flags: 0,
            jumpaddress: Address::new_invalid(),
            targethit: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// DisassemblyEngine (codedata.hh:61-74, codedata.cc:44-130)
// ---------------------------------------------------------------------------

/// C++ `DisassemblyEngine` (`codedata.hh:61`): a [`PcodeEmit`] that, decoding a
/// single instruction, records the destination of any branch/call and whether
/// the instruction references one of the tracked target offsets.
///
/// The C++ caches the `const Translate *trans` set by `init`; here the
/// `Translate` is passed to [`disassemble`](DisassemblyEngine::disassemble) (see
/// the module's cross-module boundary note), so the engine holds only the
/// per-decode scratch state and the tracked target-offset set.
#[derive(Debug)]
pub struct DisassemblyEngine {
    /// C++ `jumpaddr`: destinations gathered during a decode.
    jumpaddr: Vec<Address>,
    /// C++ `targetoffsets`: offsets of the tracked target thunks.
    targetoffsets: BTreeSet<uintb>,
    /// C++ `lastop`: the last p-code op seen.
    lastop: OpCode,
    /// C++ `hascall`: did the instruction contain a call?
    hascall: bool,
    /// C++ `hitsaddress`: did the instruction reference a tracked target?
    hitsaddress: bool,
    /// C++ `targethit`: the offset referenced when `hitsaddress`.
    targethit: uintb,
}

impl Default for DisassemblyEngine {
    fn default() -> Self {
        DisassemblyEngine {
            jumpaddr: Vec::new(),
            targetoffsets: BTreeSet::new(),
            lastop: OpCode::CPUI_COPY,
            hascall: false,
            hitsaddress: false,
            targethit: 0,
        }
    }
}

impl DisassemblyEngine {
    /// C++ `DisassemblyEngine::init` (`codedata.cc:44`).
    ///
    /// The C++ also stores `trans = t`; in the port the `Translate` is supplied
    /// per `disassemble` call (cross-module boundary), so this only clears the
    /// scratch and tracked-target state.
    pub fn init(&mut self) {
        self.jumpaddr.clear();
        self.targetoffsets.clear();
    }

    /// C++ `DisassemblyEngine::addTarget` (`codedata.hh:73`): track an address
    /// offset to detect references to.
    pub fn add_target(&mut self, addr: &Address) {
        self.targetoffsets.insert(addr.get_offset());
    }

    /// C++ `DisassemblyEngine::disassemble` (`codedata.cc:83`): decode one
    /// instruction at `addr` and fill `res` with its inferred flags.
    pub fn disassemble(
        &mut self,
        trans: &dyn Translate,
        addr: &Address,
        res: &mut DisassemblyResult,
    ) {
        self.jumpaddr.clear();
        self.lastop = OpCode::CPUI_COPY;
        self.hascall = false;
        self.hitsaddress = false;
        res.flags = 0;
        match trans.one_instruction(self, addr) {
            Ok(length) => {
                res.length = length;
            }
            Err(KunaError::BadData { .. }) => {
                res.success = false;
                return;
            }
            Err(KunaError::DataUnavail { .. }) => {
                res.success = false;
                return;
            }
            Err(KunaError::Unimpl { instruction_length, .. }) => {
                res.length = instruction_length;
            }
            // C++ only catches those three; any other throw escapes the try.
            // No other variant arises from oneInstruction in practice; surface
            // it as a failed decode rather than panicking the console.
            Err(_) => {
                res.success = false;
                return;
            }
        }
        res.success = true;
        if self.hascall {
            res.flags |= code_unit_flags::CALL;
        }
        if self.hitsaddress {
            res.flags |= code_unit_flags::TARGETHIT;
            res.targethit = self.targethit;
        }
        let lastaddr = addr + res.length as i64;
        match self.lastop {
            OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND => {
                if self.hitsaddress {
                    res.flags |= code_unit_flags::THUNKHIT; // Hits target via indirect jump
                }
            }
            OpCode::CPUI_RETURN => {}
            _ => {
                res.flags |= code_unit_flags::FALLTHRU;
            }
        }
        for i in 0..self.jumpaddr.len() {
            if self.jumpaddr[i] == lastaddr {
                res.flags |= code_unit_flags::FALLTHRU;
            } else if self.jumpaddr[i] != *addr {
                res.flags |= code_unit_flags::JUMP;
                res.jumpaddress = self.jumpaddr[i].clone();
            }
        }
    }

    /// C++ `jumpaddr.push_back(Address(vars[0].space,vars[0].offset))`.
    fn push_jump(&mut self, v: &VarnodeData) {
        let addr = match &v.space {
            // C++ builds `Address(vars[0].space, vars[0].offset)` directly from
            // the VarnodeData space pointer.
            Some(spc) => Address::new(Rc::clone(spc), v.offset),
            None => Address::new_invalid(),
        };
        self.jumpaddr.push(addr);
    }
}

impl PcodeEmit for DisassemblyEngine {
    /// C++ `DisassemblyEngine::dump` (`codedata.cc:52`).
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        _outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.lastop = opc;
        match opc {
            OpCode::CPUI_CALL => {
                self.hascall = true;
                // fallthru
                self.push_jump(&vars[0]);
            }
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                self.push_jump(&vars[0]);
            }
            OpCode::CPUI_COPY | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND => {
                if self.targetoffsets.contains(&vars[0].offset) {
                    self.hitsaddress = true;
                    self.targethit = vars[0].offset;
                }
            }
            OpCode::CPUI_LOAD => {
                if self.targetoffsets.contains(&vars[1].offset) {
                    self.hitsaddress = true;
                    self.targethit = vars[1].offset;
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// TargetHit / TargetFeature / AddrLink (codedata.hh:76-101)
// ---------------------------------------------------------------------------

/// C++ `TargetHit` (`codedata.hh:76`): a single reference to a tracked target.
#[derive(Debug, Clone)]
pub struct TargetHit {
    /// C++ `funcstart`: start of the function making the target call.
    pub funcstart: Address,
    /// C++ `codeaddr`: address of the instruction referencing the target.
    pub codeaddr: Address,
    /// C++ `thunkaddr`: the target itself.
    pub thunkaddr: Address,
    /// C++ `mask`: the feature mask associated with the target.
    pub mask: uint4,
}

impl TargetHit {
    /// C++ `TargetHit(func,code,thunk,m)` (`codedata.hh:82`).
    pub fn new(func: Address, code: Address, thunk: Address, m: uint4) -> Self {
        TargetHit { funcstart: func, codeaddr: code, thunkaddr: thunk, mask: m }
    }
}

/// C++ `TargetFeature` (`codedata.hh:87`): a tracked target's name and feature
/// mask.
#[derive(Debug, Clone, Default)]
pub struct TargetFeature {
    /// C++ `name`: the target function's name.
    pub name: String,
    /// C++ `featuremask`: id of this target for ORing into a mask.
    pub featuremask: uint4,
}

/// C++ `AddrLink` (`codedata.hh:92`): an ordered pair of addresses used as a
/// cross-reference key.
#[derive(Debug, Clone)]
pub struct AddrLink {
    /// C++ `a`.
    pub a: Address,
    /// C++ `b`.
    pub b: Address,
}

impl AddrLink {
    /// C++ `AddrLink(Address i)` (`codedata.hh:95`): `b` is invalid.
    pub fn single(i: Address) -> Self {
        AddrLink { a: i, b: Address::new_invalid() }
    }

    /// C++ `AddrLink(Address i,Address j)` (`codedata.hh:96`).
    pub fn pair(i: Address, j: Address) -> Self {
        AddrLink { a: i, b: j }
    }
}

impl PartialEq for AddrLink {
    fn eq(&self, op2: &AddrLink) -> bool {
        self.a == op2.a && self.b == op2.b
    }
}
impl Eq for AddrLink {}

impl Ord for AddrLink {
    /// C++ `AddrLink::operator<` (`codedata.hh:97`): order on `a`, then `b`.
    fn cmp(&self, op2: &AddrLink) -> std::cmp::Ordering {
        if self.a != op2.a {
            return self.a.cmp(&op2.a);
        }
        self.b.cmp(&op2.b)
    }
}

impl PartialOrd for AddrLink {
    fn partial_cmp(&self, op2: &AddrLink) -> Option<std::cmp::Ordering> {
        Some(self.cmp(op2))
    }
}

// ---------------------------------------------------------------------------
// CodeDataAnalysis (codedata.hh:103-146, codedata.cc:132-716)
// ---------------------------------------------------------------------------

/// C++ `CodeDataAnalysis` (`codedata.hh:103`): the whole disassembly-target /
/// control-flow model built directly off the disassembler.
///
/// This is the `codedata` module's `IfaceData`.
#[derive(Debug, Default)]
pub struct CodeDataAnalysis {
    /// C++ `alignment`: instruction alignment.
    pub alignment: int4,
    /// C++ `disengine`.
    pub disengine: DisassemblyEngine,
    /// C++ `modelhits`.
    pub modelhits: RangeList,
    /// C++ `codeunit`: address -> code unit.
    pub codeunit: BTreeMap<Address, CodeUnit>,
    /// C++ `fromto_crossref`.
    pub fromto_crossref: BTreeMap<AddrLink, uint4>,
    /// C++ `tofrom_crossref`.
    pub tofrom_crossref: BTreeMap<AddrLink, uint4>,
    /// C++ `taintlist`: work-list of code-unit keys to taint.
    ///
    /// The C++ stores `map<...>::iterator`s; the port stores the `Address` keys
    /// (the map key is the stable identity an iterator denotes), which is what
    /// every dereference reads back.
    pub taintlist: Vec<Address>,
    /// C++ `unlinkedstarts`.
    pub unlinkedstarts: Vec<Address>,
    /// C++ `targethits`.
    pub targethits: Vec<TargetHit>,
    /// C++ `targets`: thunk address -> feature.
    pub targets: BTreeMap<Address, TargetFeature>,
}

impl IfaceData for CodeDataAnalysis {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl CodeDataAnalysis {
    /// C++ `CodeDataAnalysis::init` (`codedata.cc:132`).
    ///
    /// The C++ takes `Architecture *g` and caches it as `glb`, reading
    /// `glb->translate` for the alignment and the engine's `Translate`.  Here
    /// the `Translate` is supplied so the cached-pointer state collapses to the
    /// alignment.
    pub fn init(&mut self, trans: &dyn Translate) {
        self.disengine.init();
        self.alignment = trans.get_alignment();
        self.modelhits.clear();
        self.codeunit.clear();
        self.fromto_crossref.clear();
        self.tofrom_crossref.clear();
        self.taintlist.clear();
        self.unlinkedstarts.clear();
        self.targethits.clear();
        self.targets.clear();
    }

    /// C++ `CodeDataAnalysis::pushTaintAddress` (`codedata.cc:148`).
    pub fn push_taint_address(&mut self, addr: &Address) {
        // The last entry with key <= addr.
        let prev = self.codeunit.range(..=addr).next_back().map(|(k, v)| (k.clone(), *v));
        let (key, cu) = match prev {
            Some(x) => x,
            None => return, // upper_bound==begin (no element <= addr)
        };
        if key.get_offset().wrapping_add(cu.size as u64).wrapping_sub(1) < addr.get_offset() {
            return;
        }
        if (cu.flags & code_unit_flags::NOTCODE) != 0 {
            return; // Already visited
        }
        self.taintlist.push(key);
    }

    /// C++ `CodeDataAnalysis::processTaint` (`codedata.cc:162`).
    pub fn process_taint(&mut self) {
        let key = self.taintlist.pop().expect("processTaint on empty taintlist");
        let cu_size = {
            let cu = self.codeunit.get_mut(&key).expect("taint key absent");
            cu.flags |= code_unit_flags::NOTCODE;
            cu.size
        };
        let startaddr = key.clone();
        let endaddr = &startaddr + cu_size as i64;
        if let Some((prevkey, cu2)) = self.codeunit.range(..&startaddr).next_back() {
            let cu2 = *cu2;
            // ((cu2.flags & (fallthru&notcode))==fallthru): note (fallthru &
            // notcode) == 0, so the mask is 0 and the test is (0 == fallthru),
            // i.e. always false.  Transcribed verbatim (C++ bug-for-bug); the
            // dead branch is upstream's, not ours, so the always-false mask is
            // intentional.
            #[allow(clippy::bad_bit_mask)]
            if (cu2.flags & (code_unit_flags::FALLTHRU & code_unit_flags::NOTCODE))
                == code_unit_flags::FALLTHRU
            {
                // The C++ adds the *tainted* unit's size here (cu, not cu2) —
                // transcribed verbatim.
                let addr2 = prevkey + cu_size as i64;
                if addr2 == startaddr {
                    let pk = prevkey.clone();
                    self.taintlist.push(pk);
                }
            }
        }
        // Erase all cross-references coming out of this block.
        self.erase_fromto_range(&startaddr, &endaddr);
        // tofrom: push each .b as a taint, then erase.
        let to_remove: Vec<(AddrLink, Address)> = self
            .tofrom_crossref
            .range(AddrLink::single(startaddr.clone())..AddrLink::single(endaddr.clone()))
            .map(|(k, _)| (k.clone(), k.b.clone()))
            .collect();
        for (k, b) in to_remove {
            self.push_taint_address(&b);
            self.tofrom_crossref.remove(&k);
        }
    }

    /// Helper: `fromto_crossref.erase(lower_bound(start),lower_bound(end))`.
    fn erase_fromto_range(&mut self, startaddr: &Address, endaddr: &Address) {
        let keys: Vec<AddrLink> = self
            .fromto_crossref
            .range(AddrLink::single(startaddr.clone())..AddrLink::single(endaddr.clone()))
            .map(|(k, _)| k.clone())
            .collect();
        for k in keys {
            self.fromto_crossref.remove(&k);
        }
    }

    /// C++ `CodeDataAnalysis::commitCodeVec` (`codedata.cc:196`).
    //
    // mutable_key_type: AddrLink's Address keys hold interior-mutable AddrSpace
    // Cells (flags/shortcut) that do not participate in the ported Ord (it reads
    // only the immutable space index/offset), so the key order cannot change
    // while a key is in the map.  (Same rationale as translate.rs
    // get_all_registers.)
    #[allow(clippy::mutable_key_type)]
    pub fn commit_code_vec(
        &mut self,
        addr: &Address,
        codevec: &[CodeUnit],
        fromto_vec: &BTreeMap<AddrLink, uint4>,
    ) -> Address {
        let mut curaddr = addr.clone();
        for cu in codevec.iter() {
            self.codeunit.insert(curaddr.clone(), *cu);
            curaddr = &curaddr + cu.size as i64;
        }
        for (fromto, &val) in fromto_vec.iter() {
            self.fromto_crossref.insert(fromto.clone(), val);
            let tofrom = AddrLink::pair(fromto.b.clone(), fromto.a.clone());
            self.tofrom_crossref.insert(tofrom, val);
        }
        curaddr
    }

    /// C++ `CodeDataAnalysis::clearHitBy` (`codedata.cc:214`).
    pub fn clear_hit_by(&mut self) {
        for cu in self.codeunit.values_mut() {
            cu.flags &= !(code_unit_flags::HIT_BY_FALLTHRU
                | code_unit_flags::HIT_BY_JUMP
                | code_unit_flags::HIT_BY_CALL);
        }
    }

    /// C++ `CodeDataAnalysis::clearCrossRefs` (`codedata.cc:225`).
    pub fn clear_cross_refs(&mut self, addr: &Address, endaddr: &Address) {
        let keys: Vec<AddrLink> = self
            .fromto_crossref
            .range(AddrLink::single(addr.clone())..AddrLink::single(endaddr.clone()))
            .map(|(k, _)| k.clone())
            .collect();
        for fromto in &keys {
            let tf = AddrLink::pair(fromto.b.clone(), fromto.a.clone());
            self.tofrom_crossref.remove(&tf);
        }
        for k in keys {
            self.fromto_crossref.remove(&k);
        }
    }

    /// C++ `CodeDataAnalysis::addTarget` (`codedata.cc:241`).
    pub fn add_target(&mut self, nm: &str, addr: &Address, mask: uint4) {
        let targfeat = self.targets.entry(addr.clone()).or_default();
        targfeat.name = nm.to_string();
        targfeat.featuremask = mask;
        self.disengine.add_target(addr); // Tell the disassembler to search for address
    }

    /// C++ `CodeDataAnalysis::clearCodeUnits` (`codedata.cc:250`).
    pub fn clear_code_units(&mut self, addr: &Address, endaddr: &Address) {
        let keys: Vec<Address> =
            self.codeunit.range(addr.clone()..endaddr.clone()).map(|(k, _)| k.clone()).collect();
        for k in keys {
            self.codeunit.remove(&k);
        }
        self.clear_cross_refs(addr, endaddr);
    }

    /// C++ `CodeDataAnalysis::disassembleBlock` (`codedata.cc:261`).
    // mutable_key_type: see commit_code_vec — the AddrLink keys' interior
    // mutability does not affect their Ord.
    #[allow(clippy::mutable_key_type)]
    pub fn disassemble_block(
        &mut self,
        trans: &dyn Translate,
        addr: &Address,
        endaddr: &Address,
    ) -> Address {
        let mut disresult = DisassemblyResult::default();
        let mut codevec: Vec<CodeUnit> = Vec::new();
        let mut fromto_vec: BTreeMap<AddrLink, uint4> = BTreeMap::new();
        let mut flowin = false;
        let mut hardend = false;

        let mut curaddr = addr.clone();
        // Track the lower_bound(addr) key as the C++ iterator.
        let mut iter_key: Option<Address> =
            self.codeunit.range(addr.clone()..).next().map(|(k, _)| k.clone());
        let mut lastaddr: Address;
        if let Some(ref k) = iter_key {
            lastaddr = k.clone();
            if *endaddr < lastaddr {
                lastaddr = endaddr.clone();
                hardend = true;
            }
        } else {
            lastaddr = endaddr.clone();
            hardend = true;
        }
        loop {
            self.disengine.disassemble(trans, &curaddr, &mut disresult);
            codevec.push(CodeUnit::default());
            if !disresult.success {
                let back = codevec.last_mut().unwrap();
                back.flags = code_unit_flags::NOTCODE;
                back.size = 1;
                curaddr = &curaddr + 1;
                break;
            }
            if (disresult.flags & code_unit_flags::JUMP) != 0 {
                fromto_vec.insert(
                    AddrLink::pair(curaddr.clone(), disresult.jumpaddress.clone()),
                    disresult.flags,
                );
            }
            {
                let back = codevec.last_mut().unwrap();
                back.flags = disresult.flags;
                back.size = disresult.length;
            }
            curaddr = &curaddr + disresult.length as i64;
            while lastaddr < curaddr {
                // (*iter).second flags / size — iter_key denotes the unit.
                let iter_flags =
                    iter_key.as_ref().and_then(|k| self.codeunit.get(k)).map(|cu| cu.flags);
                let iter_size =
                    iter_key.as_ref().and_then(|k| self.codeunit.get(k)).map(|cu| cu.size);
                if (!hardend)
                    && iter_flags.map(|f| (f & code_unit_flags::NOTCODE) != 0).unwrap_or(false)
                {
                    if iter_size == Some(1) {
                        // delete the bad disassembly (it looks unaligned).
                        let badkey = iter_key.clone().unwrap();
                        // ++iter before erase (advance to the next key)
                        iter_key = self
                            .codeunit
                            .range((Bound::Excluded(badkey.clone()), Bound::Unbounded))
                            .next()
                            .map(|(k, _)| k.clone());
                        self.codeunit.remove(&badkey);
                        if let Some(ref k) = iter_key {
                            lastaddr = k.clone();
                            if *endaddr < lastaddr {
                                lastaddr = endaddr.clone();
                                hardend = true;
                            }
                        } else {
                            lastaddr = endaddr.clone();
                            hardend = true;
                        }
                    } else {
                        disresult.success = false;
                        flowin = true;
                        break;
                    }
                } else {
                    disresult.success = false;
                    break;
                }
            }
            if !disresult.success {
                break;
            }
            if curaddr == lastaddr {
                let iter_flags =
                    iter_key.as_ref().and_then(|k| self.codeunit.get(k)).map(|cu| cu.flags);
                if iter_flags.map(|f| (f & code_unit_flags::NOTCODE) != 0).unwrap_or(false) {
                    flowin = true;
                    break;
                }
            }
            if ((disresult.flags & code_unit_flags::FALLTHRU) == 0) || (curaddr == lastaddr) {
                // found the end of a block
                return self.commit_code_vec(addr, &codevec, &fromto_vec);
            }
        }
        // If we reach here, we have bad disassembly of some sort.
        if hardend && (lastaddr < curaddr) {
            curaddr = lastaddr.clone();
        }
        let mut wholesize = curaddr.get_offset().wrapping_sub(addr.get_offset()) as int4;
        if (!flowin) && (wholesize < 10) {
            wholesize = 1;
        }
        let cu = self.codeunit.entry(addr.clone()).or_default();
        cu.flags = code_unit_flags::NOTCODE;
        cu.size = wholesize;
        addr + cu.size as i64
    }

    /// C++ `CodeDataAnalysis::disassembleRange` (`codedata.cc:355`).
    pub fn disassemble_range(&mut self, trans: &dyn Translate, range: &Range) {
        let mut addr = range.get_first_addr();
        let lastaddr = range.get_last_addr();
        while addr <= lastaddr {
            addr = self.disassemble_block(trans, &addr, &lastaddr);
        }
    }

    /// C++ `CodeDataAnalysis::disassembleRangeList` (`codedata.cc:365`).
    pub fn disassemble_range_list(&mut self, trans: &dyn Translate, rangelist: &RangeList) {
        // The C++ iterates the live RangeList; disassemble_range only reads it,
        // so a snapshot of the ranges is equivalent.
        let ranges: Vec<Range> = rangelist.iter().cloned().collect();
        for range in &ranges {
            self.disassemble_range(trans, range);
        }
    }

    /// C++ `CodeDataAnalysis::findNotCodeUnits` (`codedata.cc:378`).
    pub fn find_not_code_units(&mut self) {
        // Build the initial work list with known "notcode".
        let initial: Vec<Address> = self
            .codeunit
            .iter()
            .filter(|(_, cu)| (cu.flags & code_unit_flags::NOTCODE) != 0)
            .map(|(k, _)| k.clone())
            .collect();
        for k in initial {
            self.taintlist.push(k);
        }
        while !self.taintlist.is_empty() {
            self.process_taint();
        }
    }

    /// C++ `CodeDataAnalysis::markFallthruHits` (`codedata.cc:395`).
    pub fn mark_fallthru_hits(&mut self) {
        // An invalid address.
        let mut fallthruaddr = Address::new_invalid();
        let keys: Vec<Address> = self.codeunit.keys().cloned().collect();
        for key in keys {
            let cu = *self.codeunit.get(&key).unwrap();
            if (cu.flags & code_unit_flags::NOTCODE) != 0 {
                continue;
            }
            if fallthruaddr == key {
                self.codeunit.get_mut(&key).unwrap().flags |= code_unit_flags::HIT_BY_FALLTHRU;
            }
            if (cu.flags & code_unit_flags::FALLTHRU) != 0 {
                fallthruaddr = &key + cu.size as i64;
            }
        }
    }

    /// C++ `CodeDataAnalysis::markCrossHits` (`codedata.cc:412`).
    pub fn mark_cross_hits(&mut self) {
        let entries: Vec<(Address, uint4)> =
            self.tofrom_crossref.iter().map(|(k, &v)| (k.a.clone(), v)).collect();
        for (akey, fromflags) in entries {
            if let Some(to) = self.codeunit.get_mut(&akey) {
                if (fromflags & code_unit_flags::CALL) != 0 {
                    to.flags |= code_unit_flags::HIT_BY_CALL;
                } else if (fromflags & code_unit_flags::JUMP) != 0 {
                    to.flags |= code_unit_flags::HIT_BY_JUMP;
                }
            }
        }
    }

    /// C++ `CodeDataAnalysis::addTargetHit` (`codedata.cc:430`).
    ///
    /// `code_space` is `glb->translate->getDefaultCodeSpace()` (threaded in).
    pub fn add_target_hit(
        &mut self,
        code_space: &Rc<AddrSpace>,
        codeaddr: &Address,
        targethit: uintb,
    ) -> IfaceResult<()> {
        let funcstart = self.find_function_start(codeaddr);
        let thunkaddr = Address::new(Rc::clone(code_space), targethit);
        let mask = match self.targets.get(&thunkaddr) {
            Some(t) => t.featuremask,
            None => return Err(IfaceError::execution("Found thunk without a feature mask")),
        };
        self.targethits.push(TargetHit::new(funcstart, codeaddr.clone(), thunkaddr, mask));
        Ok(())
    }

    /// C++ `CodeDataAnalysis::resolveThunkHit` (`codedata.cc:445`).
    pub fn resolve_thunk_hit(
        &mut self,
        code_space: &Rc<AddrSpace>,
        codeaddr: &Address,
        targethit: uintb,
    ) -> IfaceResult<()> {
        let endaddr = codeaddr + 1;
        let calls: Vec<Address> = self
            .tofrom_crossref
            .range(AddrLink::single(codeaddr.clone())..AddrLink::single(endaddr))
            .filter(|(_, &flags)| (flags & code_unit_flags::CALL) != 0)
            .map(|(k, _)| k.b.clone())
            .collect();
        for b in calls {
            self.add_target_hit(code_space, &b, targethit)?;
        }
        Ok(())
    }

    /// C++ `CodeDataAnalysis::findUnlinked` (`codedata.cc:462`).
    pub fn find_unlinked(
        &mut self,
        trans: &dyn Translate,
        code_space: &Rc<AddrSpace>,
    ) -> IfaceResult<()> {
        let keys: Vec<Address> = self.codeunit.keys().cloned().collect();
        for key in keys {
            let cu = *self.codeunit.get(&key).unwrap();
            if (cu.flags
                & (code_unit_flags::HIT_BY_FALLTHRU
                    | code_unit_flags::HIT_BY_JUMP
                    | code_unit_flags::HIT_BY_CALL
                    | code_unit_flags::NOTCODE
                    | code_unit_flags::ERRANTSTART))
                == 0
            {
                self.unlinkedstarts.push(key.clone());
            }
            if (cu.flags & (code_unit_flags::TARGETHIT | code_unit_flags::NOTCODE))
                == code_unit_flags::TARGETHIT
            {
                let codeaddr = key.clone();
                let mut res = DisassemblyResult::default();
                self.disengine.disassemble(trans, &codeaddr, &mut res);
                if (cu.flags & code_unit_flags::THUNKHIT) != 0 {
                    self.resolve_thunk_hit(code_space, &codeaddr, res.targethit)?;
                } else {
                    self.add_target_hit(code_space, &codeaddr, res.targethit)?;
                }
            }
        }
        Ok(())
    }

    /// C++ `CodeDataAnalysis::checkErrantStart` (`codedata.cc:485`).
    ///
    /// `start` is the code-unit key the C++ iterator points at; the walk steps
    /// back through the map.
    pub fn check_errant_start(&mut self, start: &Address) -> bool {
        let mut count = 0;
        let mut key = start.clone();
        while count < 1000 {
            let flags = self.codeunit.get(&key).unwrap().flags;
            if (flags & (code_unit_flags::HIT_BY_JUMP | code_unit_flags::HIT_BY_CALL)) != 0 {
                return false; // Something else jumped in
            }
            if (flags & code_unit_flags::HIT_BY_FALLTHRU) == 0 {
                self.codeunit.get_mut(&key).unwrap().flags |= code_unit_flags::ERRANTSTART;
                return true;
            }
            match self.codeunit.range(..key.clone()).next_back().map(|(k, _)| k.clone()) {
                Some(prev) => key = prev,
                None => return false,
            }
            count += 1;
        }
        false
    }

    /// C++ `CodeDataAnalysis::repairJump` (`codedata.cc:505`).
    // mutable_key_type: see commit_code_vec — the AddrLink keys' interior
    // mutability does not affect their Ord.
    #[allow(clippy::mutable_key_type)]
    pub fn repair_jump(&mut self, trans: &dyn Translate, addr: &Address, max: int4) -> bool {
        let mut disresult = DisassemblyResult::default();
        let mut codevec: Vec<CodeUnit> = Vec::new();
        let mut fromto_vec: BTreeMap<AddrLink, uint4> = BTreeMap::new();
        let mut curaddr = addr.clone();
        let mut count: int4 = 0;

        let mut iter_key: Option<Address> =
            self.codeunit.range(addr.clone()..).next().map(|(k, _)| k.clone());
        if iter_key.is_none() {
            return false;
        }
        loop {
            count += 1;
            if count >= max {
                return false;
            }
            loop {
                match &iter_key {
                    Some(k) if *k < curaddr => {
                        iter_key = self
                            .codeunit
                            .range((Bound::Excluded(k.clone()), Bound::Unbounded))
                            .next()
                            .map(|(kk, _)| kk.clone());
                        if iter_key.is_none() {
                            return false;
                        }
                    }
                    Some(_) => break,
                    None => return false,
                }
            }
            if curaddr == *iter_key.as_ref().unwrap() {
                break; // Back on cut
            }
            self.disengine.disassemble(trans, &curaddr, &mut disresult);
            if !disresult.success {
                return false;
            }
            codevec.push(CodeUnit::default());
            if (disresult.flags & code_unit_flags::JUMP) != 0 {
                fromto_vec.insert(
                    AddrLink::pair(curaddr.clone(), disresult.jumpaddress.clone()),
                    disresult.flags,
                );
            }
            let back = codevec.last_mut().unwrap();
            back.flags = disresult.flags;
            back.size = disresult.length;
            curaddr = &curaddr + disresult.length as i64;
        }
        self.clear_code_units(addr, &curaddr);
        self.commit_code_vec(addr, &codevec, &fromto_vec);
        true
    }

    /// C++ `CodeDataAnalysis::findOffCut` (`codedata.cc:542`).
    pub fn find_off_cut(&mut self, trans: &dyn Translate) {
        // The C++ walks the live tofrom_crossref map; only the repairJump branch
        // mutates it, and it re-seeks via upper_bound after.
        let mut iter_key: Option<AddrLink> = self.tofrom_crossref.keys().next().cloned();
        while let Some(curlink) = iter_key.clone() {
            let addr = curlink.a.clone(); // Destination of a jump
            let citer = self.codeunit.range(addr.clone()..).next().map(|(k, _)| k.clone());
            if let Some(ckey) = &citer {
                if *ckey == addr {
                    // Not off cut.
                    let cu_flags = self.codeunit.get(ckey).unwrap().flags;
                    if (cu_flags
                        & (code_unit_flags::HIT_BY_FALLTHRU | code_unit_flags::HIT_BY_CALL))
                        == (code_unit_flags::HIT_BY_FALLTHRU | code_unit_flags::HIT_BY_CALL)
                    {
                        // Somebody falls through into the call.
                        if let Some(prev) =
                            self.codeunit.range(..ckey.clone()).next_back().map(|(k, _)| k.clone())
                        {
                            self.check_errant_start(&prev);
                        }
                        // (--begin() is UB in C++; unreachable here — the call
                        // unit was hit by fallthru, so a predecessor exists.)
                    }
                    iter_key = self.next_tofrom(&curlink);
                    continue;
                }
            }
            let prevkey = self.codeunit.range(..addr.clone()).next_back().map(|(k, _)| k.clone());
            let prevkey = match prevkey {
                Some(k) => k, // last <= addr
                None => {
                    iter_key = self.next_tofrom(&curlink);
                    continue;
                }
            };
            if prevkey == addr {
                iter_key = self.next_tofrom(&curlink);
                continue; // on cut
            }
            let prev_size = self.codeunit.get(&prevkey).unwrap().size;
            let endaddr = &prevkey + prev_size as i64;
            if endaddr <= addr {
                iter_key = self.next_tofrom(&curlink);
                continue;
            }
            if !self.check_errant_start(&prevkey) {
                iter_key = self.next_tofrom(&curlink);
                continue;
            }
            let addrlink = curlink.clone();
            self.repair_jump(trans, &addr, 10); // This may delete tofrom_crossref nodes
            iter_key = self
                .tofrom_crossref
                .range((Bound::Excluded(addrlink), Bound::Unbounded))
                .next()
                .map(|(k, _)| k.clone());
        }
    }

    /// Helper: `++iter` over `tofrom_crossref` from a key.
    fn next_tofrom(&self, key: &AddrLink) -> Option<AddrLink> {
        self.tofrom_crossref
            .range((Bound::Excluded(key.clone()), Bound::Unbounded))
            .next()
            .map(|(k, _)| k.clone())
    }

    /// C++ `CodeDataAnalysis::findFunctionStart` (`codedata.cc:589`).
    pub fn find_function_start(&self, addr: &Address) -> Address {
        let mut range = self.tofrom_crossref.range(..AddrLink::single(addr.clone()));
        while let Some((k, &flags)) = range.next_back() {
            if (flags & code_unit_flags::CALL) != 0 {
                return k.a.clone();
            }
        }
        Address::new_invalid() // Return invalid address
    }

    /// C++ `CodeDataAnalysis::dumpModelHits` (`codedata.cc:603`).
    pub fn dump_model_hits(&self, s: &mut String) {
        let mut iter = self.modelhits.iter().peekable();
        while let Some(range) = iter.next() {
            let off = range.get_first();
            s.push_str(&format!("0x{off:x} "));
            let endoff = range.get_last();
            s.push_str(&format!("0x{endoff:x}"));
            if let Some(nextrange) = iter.peek() {
                let noff = nextrange.get_first();
                let diff = noff.wrapping_sub(endoff) as int4;
                s.push_str(&format!(" {diff}"));
            }
            s.push('\n');
        }
    }

    /// C++ `CodeDataAnalysis::dumpCrossRefs` (`codedata.cc:623`).
    pub fn dump_cross_refs(&self, s: &mut String) {
        for (addrlink, &flags) in self.fromto_crossref.iter() {
            s.push_str(&format!(
                "0x{:x} -> 0x{:x}",
                addrlink.a.get_offset(),
                addrlink.b.get_offset()
            ));
            if (flags & code_unit_flags::CALL) != 0 {
                s.push_str(" call");
            }
            s.push('\n');
        }
    }

    /// C++ `CodeDataAnalysis::dumpFunctionStarts` (`codedata.cc:639`).
    pub fn dump_function_starts(&self, s: &mut String) {
        for (addrlink, &flags) in self.tofrom_crossref.iter() {
            if (flags & code_unit_flags::CALL) != 0 {
                s.push_str(&format!("0x{:x}\n", addrlink.a.get_offset()));
            }
        }
    }

    /// C++ `CodeDataAnalysis::dumpUnlinked` (`codedata.cc:653`).
    pub fn dump_unlinked(&self, s: &mut String) {
        for addr in self.unlinkedstarts.iter() {
            s.push_str(&format!("0x{:x}\n", addr.get_offset()));
        }
    }

    /// C++ `CodeDataAnalysis::dumpTargetHits` (`codedata.cc:663`).
    pub fn dump_target_hits(&self, s: &mut String) {
        for hit in self.targethits.iter() {
            let funcaddr = &hit.funcstart;
            let addr = &hit.codeaddr;
            let nm = &self.targets.get(&hit.thunkaddr).expect("target hit thunk absent").name;
            if !funcaddr.is_invalid() {
                s.push_str(&format!("{:x} ", funcaddr.get_offset()));
            } else {
                s.push_str("nostart ");
            }
            s.push_str(&format!("{:x} {nm}\n", addr.get_offset()));
        }
    }

    /// C++ `CodeDataAnalysis::runModel` (`codedata.cc:680`).
    ///
    /// `loader` is `glb->loader`; `manage` is the architecture's
    /// `AddrSpaceManager` (for `getDefaultCodeSpace`); `trans` is
    /// `glb->translate`.
    pub fn run_model(
        &mut self,
        trans: &dyn Translate,
        loader: &mut dyn LoadImage,
        manage: &AddrSpaceManager,
    ) -> IfaceResult<()> {
        let mut secinfo = LoadImageSection::default();
        let mut moresections;
        loader.open_section_info();
        let mut lastaddr = Address::new_invalid();
        loop {
            moresections = loader.get_next_section(&mut secinfo);
            let endaddr = &secinfo.address + secinfo.size as i64;
            if secinfo.size == 0 {
                if !moresections {
                    break;
                }
                continue;
            }
            // Two arms assigning the same value, transcribed verbatim from the
            // C++ (`if (lastaddr.isInvalid()) lastaddr=endaddr; else if
            // (lastaddr < endaddr) lastaddr=endaddr;`).
            #[allow(clippy::if_same_then_else)]
            if lastaddr.is_invalid() {
                lastaddr = endaddr.clone();
            } else if lastaddr < endaddr {
                lastaddr = endaddr.clone();
            }
            if (secinfo.flags & (section_flags::UNALLOC | section_flags::NOLOAD)) == 0 {
                let spc = Rc::clone(
                    secinfo.address.get_space().expect("section address in a null space"),
                );
                self.modelhits.insert_range(spc, secinfo.address.get_offset(), endaddr.get_offset());
            }
            if !moresections {
                break;
            }
        }
        loader.close_section_info();
        {
            let cu = self.codeunit.entry(lastaddr).or_default();
            cu.size = 100;
            cu.flags = code_unit_flags::NOTCODE;
        }
        // disassembleRangeList(modelhits) — take/restore to satisfy the borrow
        // checker; the list is not mutated during the walk.
        let modelhits = std::mem::take(&mut self.modelhits);
        self.disassemble_range_list(trans, &modelhits);
        self.modelhits = modelhits;
        self.find_not_code_units();
        self.mark_fallthru_hits();
        self.mark_cross_hits();
        self.find_off_cut(trans);
        self.clear_hit_by();
        self.mark_fallthru_hits();
        self.mark_cross_hits();
        let code_space =
            Rc::clone(manage.get_default_code_space().expect("runModel: no default code space"));
        self.find_unlinked(trans, &code_space)?;
        // C++ `list<TargetHit>::sort` orders by operator< (funcstart) and is
        // stable; sort_by with the stable sort matches.
        self.targethits.sort_by(|a, b| a.funcstart.cmp(&b.funcstart));
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Console commands (codedata.hh:148-197, codedata.cc:718-790)
// ---------------------------------------------------------------------------

/// The shared module name for every `codedata` command (C++
/// `IfaceCodeDataCommand::getModule` returns "codedata").
const CODEDATA_MODULE: &str = "codedata";

/// Fetch the `codedata` module's [`CodeDataAnalysis`], or an execution error
/// (the C++ `codedata` back-pointer is always set by `setData`).
fn codedata_mut(status: &mut IfaceStatus) -> IfaceResult<&mut CodeDataAnalysis> {
    status
        .get_data_mut(CODEDATA_MODULE)
        .and_then(|d| d.as_any_mut().downcast_mut::<CodeDataAnalysis>())
        .ok_or_else(|| IfaceError::execution("No codedata module"))
}

/// The `dcp->conf` architecture access (`status.get_data("decompile")` in C++
/// `setData`) is the `"decompile"` module's `IfaceData` (`w9-con-ifacedecomp`),
/// not a concrete type in this crate.  Until it lands, the commands that need it
/// (`init`, `run`) report the execution error the C++ would hit dereferencing a
/// null `conf` — the analysis is otherwise driven directly through
/// [`CodeDataAnalysis`] (see this module's tests).
fn no_architecture(what: &str) -> IfaceError {
    IfaceError::execution(format!(
        "No architecture loaded (codedata {what} needs the 'decompile' module data)"
    ))
}

/// C++ `IfcCodeDataInit` (`codedata.cc:726`): `codedata->init(dcp->conf)`.
pub struct IfcCodeDataInit;

impl IfaceCommandAction for IfcCodeDataInit {
    fn execute(&self, _status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        // C++ caches dcp->conf in glb; init reads the translate alignment.
        Err(no_architecture("init"))
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        // C++ IfaceCodeDataCommand::createData() returns new CodeDataAnalysis().
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataTarget` (`codedata.cc:732`).
///
/// The BFD import-table lookup (`LoadImageBfd::getImportTable`) is a documented
/// loss (GPL-3, unported); the (absent) table is empty, so any name takes the
/// C++ not-found branch.
pub struct IfcCodeDataTarget;

impl IfaceCommandAction for IfcCodeDataTarget {
    fn execute(&self, status: &mut IfaceStatus, s: &mut CommandStream) -> IfaceResult<()> {
        s.skip_ws();
        if s.eof() {
            return Err(IfaceError::parse("Missing system call name"));
        }
        let token = s.read_token();
        // C++: LoadImageBfd::getImportTable(irec); search for funcname==token.
        // No BFD loader in the Rust port -> the import table is empty -> the
        // loop falls through to the not-found branch for every token.
        status.file_out(&format!("Unable to find reference to call {token}\n"));
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataRun` (`codedata.cc:756`): `codedata->runModel()`.
pub struct IfcCodeDataRun;

impl IfaceCommandAction for IfcCodeDataRun {
    fn execute(&self, _status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        Err(no_architecture("run"))
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataDumpModelHits` (`codedata.cc:762`).
pub struct IfcCodeDataDumpModelHits;

impl IfaceCommandAction for IfcCodeDataDumpModelHits {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut out = String::new();
        codedata_mut(status)?.dump_model_hits(&mut out);
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataDumpCrossRefs` (`codedata.cc:768`).
pub struct IfcCodeDataDumpCrossRefs;

impl IfaceCommandAction for IfcCodeDataDumpCrossRefs {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut out = String::new();
        codedata_mut(status)?.dump_cross_refs(&mut out);
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataDumpStarts` (`codedata.cc:774`).
pub struct IfcCodeDataDumpStarts;

impl IfaceCommandAction for IfcCodeDataDumpStarts {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut out = String::new();
        codedata_mut(status)?.dump_function_starts(&mut out);
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataDumpUnlinked` (`codedata.cc:780`).
pub struct IfcCodeDataDumpUnlinked;

impl IfaceCommandAction for IfcCodeDataDumpUnlinked {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut out = String::new();
        codedata_mut(status)?.dump_unlinked(&mut out);
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

/// C++ `IfcCodeDataDumpTargetHits` (`codedata.cc:786`).
pub struct IfcCodeDataDumpTargetHits;

impl IfaceCommandAction for IfcCodeDataDumpTargetHits {
    fn execute(&self, status: &mut IfaceStatus, _s: &mut CommandStream) -> IfaceResult<()> {
        let mut out = String::new();
        codedata_mut(status)?.dump_target_hits(&mut out);
        status.file_out(&out);
        Ok(())
    }
    fn module(&self) -> String {
        CODEDATA_MODULE.to_string()
    }
    fn create_data(&self) -> Option<Box<dyn IfaceData>> {
        Some(Box::new(CodeDataAnalysis::default()))
    }
}

// ---------------------------------------------------------------------------
// Capability registration (codedata.cc:31-42)
// ---------------------------------------------------------------------------

/// C++ `IfaceCodeDataCapability::registerCommands` (`codedata.cc:31`): register
/// every `codedata` command on the console.
///
/// In C++ the capability is a self-registering singleton
/// (`IfaceCodeDataCapability::ifaceCodeDataCapability`) whose
/// `registerCommands` is invoked by `IfaceCapability::registerAllCommands`; the
/// Rust port exposes the registration as a free function the console driver
/// calls, matching how the other module capabilities are wired.
pub fn register_commands(status: &mut IfaceStatus) {
    status.register_com(Box::new(IfcCodeDataInit), &["codedata", "init"]);
    status.register_com(Box::new(IfcCodeDataTarget), &["codedata", "target"]);
    status.register_com(Box::new(IfcCodeDataRun), &["codedata", "run"]);
    status.register_com(Box::new(IfcCodeDataDumpModelHits), &["codedata", "dump", "hits"]);
    status.register_com(Box::new(IfcCodeDataDumpCrossRefs), &["codedata", "dump", "crossrefs"]);
    status.register_com(Box::new(IfcCodeDataDumpStarts), &["codedata", "dump", "starts"]);
    status.register_com(Box::new(IfcCodeDataDumpUnlinked), &["codedata", "dump", "unlinked"]);
    status.register_com(Box::new(IfcCodeDataDumpTargetHits), &["codedata", "dump", "targethits"]);
}

// ---------------------------------------------------------------------------
// Tests — codedata analysis on a synthetic image.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use kuna_base::error::KunaResult;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, JoinSpace,
        OtherSpace, UniqueSpace, VarnodeStorage,
    };
    use kuna_base::xml::DocumentStorage;
    use kuna_sleigh::loadimage::LoadImage;
    use kuna_sleigh::translate::{AssemblyEmit, TranslateBase};

    /// One synthetic instruction: an opcode, an optional jump/target offset (the
    /// first input varnode's offset), and a byte length.
    #[derive(Clone)]
    struct Insn {
        opc: OpCode,
        operand: Option<u64>,
        len: i32,
    }

    /// A synthetic disassembler: each `ram` offset maps to one [`Insn`]; an
    /// unmapped offset decodes as `BadData` (a failed decode).
    struct SynthTranslate {
        base: TranslateBase,
        ram: Rc<AddrSpace>,
        prog: BTreeMap<u64, Insn>,
    }

    impl Translate for SynthTranslate {
        fn translate_base(&self) -> &TranslateBase {
            &self.base
        }
        fn translate_base_mut(&mut self) -> &mut TranslateBase {
            &mut self.base
        }
        fn initialize(&mut self, _store: &mut DocumentStorage) -> KunaResult<()> {
            Ok(())
        }
        #[allow(clippy::mutable_key_type)] // register map keys' Ord is index/offset/size
        fn get_all_registers(&self, _reglist: &mut BTreeMap<VarnodeData, String>) {}
        fn get_user_op_names(&self, _res: &mut Vec<String>) {}
        fn instruction_length(&self, baseaddr: &Address) -> KunaResult<i32> {
            match self.prog.get(&baseaddr.get_offset()) {
                Some(i) => Ok(i.len),
                None => Err(KunaError::bad_data("no instruction")),
            }
        }
        fn one_instruction(
            &self,
            emit: &mut dyn PcodeEmit,
            baseaddr: &Address,
        ) -> KunaResult<i32> {
            let insn = match self.prog.get(&baseaddr.get_offset()) {
                Some(i) => i.clone(),
                None => return Err(KunaError::bad_data("no instruction")),
            };
            let mk = |off: u64| VarnodeData {
                space: Some(Rc::clone(&self.ram)),
                offset: off,
                size: 4,
            };
            match insn.opc {
                OpCode::CPUI_LOAD => {
                    // The engine inspects vars[1] for LOAD; emit a leading dummy
                    // vars[0] as real SLEIGH does (the space-id constant).
                    let v0 = mk(0);
                    let v1 = mk(insn.operand.unwrap_or(0));
                    emit.dump(baseaddr, insn.opc, None, &[v0, v1]);
                }
                OpCode::CPUI_RETURN => {
                    emit.dump(baseaddr, insn.opc, None, &[mk(0)]);
                }
                _ => {
                    let v0 = mk(insn.operand.unwrap_or(0));
                    emit.dump(baseaddr, insn.opc, None, &[v0]);
                }
            }
            Ok(insn.len)
        }
        fn print_assembly(
            &self,
            _emit: &mut dyn AssemblyEmit,
            _baseaddr: &Address,
        ) -> KunaResult<i32> {
            Ok(4)
        }
    }

    impl kuna_base::space::RegisterLookup for SynthTranslate {
        fn get_register(&self, _nm: &str) -> KunaResult<VarnodeStorage> {
            Err(KunaError::sleigh("no registers"))
        }
        fn get_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
            String::new()
        }
        fn get_exact_register_name(&self, _b: &Rc<AddrSpace>, _o: u64, _s: i32) -> String {
            String::new()
        }
    }

    /// One section the [`SynthLoadImage`] reports.
    #[derive(Clone)]
    struct Section {
        address: Address,
        size: u64,
        flags: u32,
    }

    /// A synthetic load image yielding a fixed list of sections.
    struct SynthLoadImage {
        sections: Vec<Section>,
        cursor: RefCell<usize>,
    }

    impl LoadImage for SynthLoadImage {
        fn get_file_name(&self) -> &str {
            "synth"
        }
        fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
            Ok(())
        }
        fn get_arch_type(&self) -> Vec<u8> {
            b"synth".to_vec()
        }
        fn adjust_vma(&mut self, _adjust: i64) {}
        fn open_section_info(&self) {
            *self.cursor.borrow_mut() = 0;
        }
        fn close_section_info(&self) {}
        fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
            let mut c = self.cursor.borrow_mut();
            let sec = &self.sections[*c];
            record.address = sec.address.clone();
            record.size = sec.size;
            record.flags = sec.flags;
            *c += 1;
            // C++ getNextSection returns true while more records remain after
            // filling this one.
            *c < self.sections.len()
        }
    }

    fn manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(OtherSpace::new(1))).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(2, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            3,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m.insert_space(Rc::new(JoinSpace::new(4, false))).unwrap();
        m.set_default_code_space(3).unwrap();
        m
    }

    fn synth(prog: BTreeMap<u64, Insn>, ram: Rc<AddrSpace>) -> SynthTranslate {
        SynthTranslate { base: TranslateBase::new(), ram, prog }
    }

    fn fallthru(operand: Option<u64>) -> Insn {
        // COPY -> lastop COPY -> fallthru flag.
        Insn { opc: OpCode::CPUI_COPY, operand, len: 4 }
    }

    #[test]
    fn addrlink_orders_on_a_then_b() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let a0 = Address::new(Rc::clone(&ram), 0x10);
        let a1 = Address::new(Rc::clone(&ram), 0x20);
        // single(a) has b = invalid, which sorts before any concrete b.
        assert!(AddrLink::single(a0.clone()) < AddrLink::pair(a0.clone(), a1.clone()));
        assert!(AddrLink::pair(a0.clone(), a1.clone()) < AddrLink::single(a1.clone()));
        assert_eq!(AddrLink::pair(a0.clone(), a1.clone()), AddrLink::pair(a0, a1));
    }

    #[test]
    fn disassembly_engine_flags_fallthru_jump_call_return() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut prog: BTreeMap<u64, Insn> = BTreeMap::new();
        prog.insert(0x100, fallthru(None));
        prog.insert(0x104, Insn { opc: OpCode::CPUI_BRANCH, operand: Some(0x200), len: 4 });
        prog.insert(0x108, Insn { opc: OpCode::CPUI_CALL, operand: Some(0x300), len: 4 });
        prog.insert(0x10c, Insn { opc: OpCode::CPUI_RETURN, operand: None, len: 4 });
        let t = synth(prog, Rc::clone(&ram));

        let mut eng = DisassemblyEngine::default();
        eng.init();
        let mut res = DisassemblyResult::default();

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x100), &mut res);
        assert!(res.success);
        assert_eq!(res.flags & code_unit_flags::FALLTHRU, code_unit_flags::FALLTHRU);

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x104), &mut res);
        assert_eq!(res.flags & code_unit_flags::JUMP, code_unit_flags::JUMP);
        assert_eq!(res.flags & code_unit_flags::FALLTHRU, 0);
        assert_eq!(res.jumpaddress.get_offset(), 0x200);

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x108), &mut res);
        assert_eq!(res.flags & code_unit_flags::CALL, code_unit_flags::CALL);
        assert_eq!(res.flags & code_unit_flags::FALLTHRU, code_unit_flags::FALLTHRU);

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x10c), &mut res);
        assert_eq!(res.flags & code_unit_flags::FALLTHRU, 0);
        assert_eq!(res.flags & code_unit_flags::JUMP, 0);

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x500), &mut res);
        assert!(!res.success);
    }

    #[test]
    fn disassembly_engine_target_hit_and_thunk() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut prog: BTreeMap<u64, Insn> = BTreeMap::new();
        prog.insert(0x10, Insn { opc: OpCode::CPUI_COPY, operand: Some(0x900), len: 4 });
        prog.insert(0x20, Insn { opc: OpCode::CPUI_BRANCHIND, operand: Some(0x900), len: 4 });
        let t = synth(prog, Rc::clone(&ram));

        let mut eng = DisassemblyEngine::default();
        eng.init();
        eng.add_target(&Address::new(Rc::clone(&ram), 0x900));
        let mut res = DisassemblyResult::default();

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x10), &mut res);
        assert_eq!(res.flags & code_unit_flags::TARGETHIT, code_unit_flags::TARGETHIT);
        assert_eq!(res.flags & code_unit_flags::THUNKHIT, 0);
        assert_eq!(res.targethit, 0x900);

        eng.disassemble(&t, &Address::new(Rc::clone(&ram), 0x20), &mut res);
        assert_eq!(res.flags & code_unit_flags::TARGETHIT, code_unit_flags::TARGETHIT);
        assert_eq!(res.flags & code_unit_flags::THUNKHIT, code_unit_flags::THUNKHIT);
    }

    /// A straight-line block of fall-through units ending in returns, with a
    /// direct call, run end-to-end through `run_model`.
    #[test]
    fn run_model_straightline_with_call() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        // Program in [0x1000, 0x1018):
        //   0x1000 COPY    (fallthru)
        //   0x1004 CALL 0x1010 (call + fallthru + jump)
        //   0x1008 COPY    (fallthru)
        //   0x100c RETURN
        //   0x1010 COPY    (fallthru)   -- the call target
        //   0x1014 RETURN
        let mut prog: BTreeMap<u64, Insn> = BTreeMap::new();
        prog.insert(0x1000, fallthru(None));
        prog.insert(0x1004, Insn { opc: OpCode::CPUI_CALL, operand: Some(0x1010), len: 4 });
        prog.insert(0x1008, fallthru(None));
        prog.insert(0x100c, Insn { opc: OpCode::CPUI_RETURN, operand: None, len: 4 });
        prog.insert(0x1010, fallthru(None));
        prog.insert(0x1014, Insn { opc: OpCode::CPUI_RETURN, operand: None, len: 4 });
        let t = synth(prog, Rc::clone(&ram));

        let mut loader = SynthLoadImage {
            sections: vec![Section {
                // The section is larger than the code (which ends at 0x1018) so
                // the section-end "notcode" boundary marker C++ places at the
                // max section end sits well past the real code; the final
                // RETURN-terminated block does not flow into it.
                address: Address::new(Rc::clone(&ram), 0x1000),
                size: 0x40,
                flags: section_flags::CODE,
            }],
            cursor: RefCell::new(0),
        };

        let mut cd = CodeDataAnalysis::default();
        cd.init(&t);
        cd.run_model(&t, &mut loader, &m).unwrap();

        // The CALL at 0x1004 produced a fromto crossref to 0x1010 with CALL.
        let call_link = AddrLink::pair(
            Address::new(Rc::clone(&ram), 0x1004),
            Address::new(Rc::clone(&ram), 0x1010),
        );
        assert_eq!(
            cd.fromto_crossref.get(&call_link).copied().unwrap_or(0) & code_unit_flags::CALL,
            code_unit_flags::CALL
        );

        // 0x1010 is hit by the call.
        let target = Address::new(Rc::clone(&ram), 0x1010);
        assert_ne!(cd.codeunit.get(&target).unwrap().flags & code_unit_flags::HIT_BY_CALL, 0);

        // dumpFunctionStarts walks `tofrom_crossref` and prints `.a`, which for
        // a call cross-reference is the call *destination* (the function start),
        // i.e. 0x1010 — not the call site.  (C++ codedata.cc:639.)
        let mut starts = String::new();
        cd.dump_function_starts(&mut starts);
        assert_eq!(starts, "0x1010\n");

        // dumpCrossRefs walks `fromto_crossref` and prints `a -> b`, the call
        // site to its destination.
        let mut crossrefs = String::new();
        cd.dump_cross_refs(&mut crossrefs);
        assert_eq!(crossrefs, "0x1004 -> 0x1010 call\n");

        // The model-hit range cover spans the section: first=0x1000,
        // last=0x1040 (insertRange(0x1000, address+size=0x1040)).
        let mut hits = String::new();
        cd.dump_model_hits(&mut hits);
        assert_eq!(hits, "0x1000 0x1040\n");
    }

    #[test]
    fn run_model_target_hits_sorted_and_dumped() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        // The function at 0x2000 references the tracked target 0xBEEF; a caller
        // at 0x2020 calls it, so `findFunctionStart(0x2004)` resolves the hit's
        // function to the call *destination* 0x2000 (codedata.cc:589).
        //   0x2000 COPY          (function body, fallthru)
        //   0x2004 COPY refs 0xBEEF (target hit inside the function)
        //   0x2008 RETURN
        //   0x2020 CALL 0x2000   (the caller)
        //   0x2024 RETURN
        let mut prog: BTreeMap<u64, Insn> = BTreeMap::new();
        prog.insert(0x2000, Insn { opc: OpCode::CPUI_COPY, operand: None, len: 4 });
        prog.insert(0x2004, Insn { opc: OpCode::CPUI_COPY, operand: Some(0xBEEF), len: 4 });
        prog.insert(0x2008, Insn { opc: OpCode::CPUI_RETURN, operand: None, len: 4 });
        prog.insert(0x2020, Insn { opc: OpCode::CPUI_CALL, operand: Some(0x2000), len: 4 });
        prog.insert(0x2024, Insn { opc: OpCode::CPUI_RETURN, operand: None, len: 4 });
        let t = synth(prog, Rc::clone(&ram));

        let mut loader = SynthLoadImage {
            sections: vec![Section {
                // Larger than the code (ends at 0x2010) so the boundary marker
                // sits past it (see run_model_straightline_with_call).
                address: Address::new(Rc::clone(&ram), 0x2000),
                size: 0x40,
                flags: section_flags::CODE,
            }],
            cursor: RefCell::new(0),
        };

        let mut cd = CodeDataAnalysis::default();
        cd.init(&t);
        // Track 0xBEEF as a named target before running the model.
        cd.add_target("sys_target", &Address::new(Rc::clone(&ram), 0xBEEF), 1);
        cd.run_model(&t, &mut loader, &m).unwrap();

        // One target hit, attributed to the function at 0x2000.
        assert_eq!(cd.targethits.len(), 1);
        assert_eq!(cd.targethits[0].funcstart.get_offset(), 0x2000);
        assert_eq!(cd.targethits[0].codeaddr.get_offset(), 0x2004);
        assert_eq!(cd.targethits[0].thunkaddr.get_offset(), 0xBEEF);
        assert_eq!(cd.targethits[0].mask, 1);

        let mut out = String::new();
        cd.dump_target_hits(&mut out);
        assert_eq!(out, "2000 2004 sys_target\n");
    }

    #[test]
    fn init_resets_all_state() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let t = synth(BTreeMap::new(), Rc::clone(&ram));

        let mut cd = CodeDataAnalysis::default();
        cd.codeunit.insert(Address::new(Rc::clone(&ram), 0x10), CodeUnit::default());
        cd.unlinkedstarts.push(Address::new(Rc::clone(&ram), 0x10));
        cd.add_target("x", &Address::new(Rc::clone(&ram), 0x40), 1);
        assert!(!cd.codeunit.is_empty());

        cd.init(&t);
        assert!(cd.codeunit.is_empty());
        assert!(cd.unlinkedstarts.is_empty());
        assert!(cd.targets.is_empty());
        assert!(cd.fromto_crossref.is_empty());
        assert_eq!(cd.alignment, t.get_alignment());
    }

    #[test]
    fn target_command_reports_unfound_without_bfd() {
        let mut status = make_status();
        register_commands(&mut status);
        let mut s = CommandStream::new("printf");
        IfcCodeDataTarget.execute(&mut status, &mut s).unwrap();
        // The message goes to fileoptr (here folded into optr).
        assert_eq!(status.optr, "Unable to find reference to call printf\n");
    }

    #[test]
    fn target_command_missing_name_is_parse_error() {
        let mut status = make_status();
        let mut s = CommandStream::new("   ");
        let e = IfcCodeDataTarget.execute(&mut status, &mut s).unwrap_err();
        assert!(e.is_parse());
        assert_eq!(e.explain, "Missing system call name");
    }

    #[test]
    fn registration_creates_codedata_module_data() {
        let mut status = make_status();
        register_commands(&mut status);
        let cd = status
            .get_data_mut(CODEDATA_MODULE)
            .and_then(|d| d.as_any_mut().downcast_mut::<CodeDataAnalysis>())
            .expect("codedata module data present after registration");
        assert!(cd.codeunit.is_empty());
        // dump crossrefs on the (empty) module data emits nothing.
        let mut s = CommandStream::new("");
        IfcCodeDataDumpCrossRefs.execute(&mut status, &mut s).unwrap();
        assert_eq!(status.optr, "");
    }

    #[test]
    fn init_and_run_without_decompile_data_error() {
        let mut status = make_status();
        register_commands(&mut status);
        let mut s = CommandStream::new("");
        let e = IfcCodeDataInit.execute(&mut status, &mut s).unwrap_err();
        assert!(e.is_execution());
        let mut s2 = CommandStream::new("");
        let e2 = IfcCodeDataRun.execute(&mut status, &mut s2).unwrap_err();
        assert!(e2.is_execution());
    }

    // -- w9-con-codedata verifier adversarial tests ------------------------
    //
    // These target the hunt-list spots flagged most fragile for this item:
    //   * the erase-while-iterating taint loop over `tofrom_crossref`
    //     (`process_taint`) and the `pushTaintAddress` `key+size-1 < addr`
    //     boundary;
    //   * `findFunctionStart`'s `lower_bound` + reverse `--iter` walk;
    //   * `dumpModelHits`' multi-line sequential grammar and the
    //     `(int4)(off-endoff)` signed-narrowing cast;
    //   * `findUnlinked`'s flag-mask classification.

    /// `find_function_start` (codedata.cc:589) `lower_bound(AddrLink(addr))` then
    /// reverse `--iter`: walks every key strictly < `single(addr)` backwards,
    /// returning the `.a` of the nearest CALL crossref before `addr`, else an
    /// invalid address.  Boundary: a CALL whose key offset equals `addr`
    /// (`AddrLink(addr,...)` is NOT < `single(addr)` because b sorts after the
    /// invalid b of `single`), and the no-call-before case.
    #[test]
    fn verify_w9_find_function_start_lower_bound_reverse_walk() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut cd = CodeDataAnalysis::default();
        let a = |o: u64| Address::new(Rc::clone(&ram), o);

        // tofrom keys: a==0x10 (CALL), a==0x40 (JUMP, not call), a==0x80 (CALL).
        cd.tofrom_crossref.insert(AddrLink::pair(a(0x10), a(0x500)), code_unit_flags::CALL);
        cd.tofrom_crossref.insert(AddrLink::pair(a(0x40), a(0x600)), code_unit_flags::JUMP);
        cd.tofrom_crossref.insert(AddrLink::pair(a(0x80), a(0x700)), code_unit_flags::CALL);

        // For addr=0x90: lower_bound(single(0x90)) is end()/past 0x80's entry,
        // reverse walk hits 0x80's CALL first -> 0x80.
        assert_eq!(cd.find_function_start(&a(0x90)).get_offset(), 0x80);
        // For addr=0x50: nearest call strictly before is 0x10 (0x40 is JUMP).
        assert_eq!(cd.find_function_start(&a(0x50)).get_offset(), 0x10);
        // For addr=0x80 (== a CALL key): the real key pair(0x80,0x700) is >=
        // single(0x80) (b=0x700 sorts after single's invalid b), so lower_bound
        // lands ON the 0x80 entry and the reverse `--iter` walk skips it.  The
        // call AT 0x80 is therefore NOT seen for addr==0x80; the next call before
        // is 0x10.  This pins the lower_bound/`--iter` boundary exclusion (C++
        // codedata.cc:594-599), matching the Rust `range(..single(addr))`.
        assert_eq!(cd.find_function_start(&a(0x80)).get_offset(), 0x10);
        // For addr=0x05: nothing strictly before -> invalid (C++ Address()).
        assert!(cd.find_function_start(&a(0x05)).is_invalid());
    }

    /// `process_taint` (codedata.cc:162) propagates the `notcode` taint along
    /// `tofrom_crossref` edges out of the tainted block, erasing each edge as it
    /// goes (the erase-while-iterating loop, ported as collect-then-remove).
    /// Drive `find_not_code_units` from a seeded `notcode` unit and confirm the
    /// taint spreads to the cross-referenced successor and that the crossref
    /// maps are drained.  Also exercises `pushTaintAddress`' `key+size-1 < addr`
    /// containment check (the successor key must contain the edge's `.b`).
    #[test]
    fn verify_w9_process_taint_spreads_along_tofrom_and_erases() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut cd = CodeDataAnalysis::default();
        let a = |o: u64| Address::new(Rc::clone(&ram), o);

        // Two code units: 0x100 (seeded notcode) and 0x200 (good code, size 4).
        cd.codeunit
            .insert(a(0x100), CodeUnit { flags: code_unit_flags::NOTCODE, size: 4 });
        cd.codeunit.insert(a(0x200), CodeUnit { flags: code_unit_flags::FALLTHRU, size: 4 });

        // A from-to edge 0x100 -> 0x200 (and its tofrom mirror), so taint at the
        // 0x100 block flows to 0x200 via the tofrom_crossref edge keyed [start..end)
        // of the 0x100 block. The tofrom key's `.a` is the destination 0x100's
        // range; we key it inside [0x100,0x104).
        cd.fromto_crossref.insert(AddrLink::pair(a(0x100), a(0x200)), code_unit_flags::JUMP);
        // tofrom mirror: key.a in [start,end) of the tainted block, key.b is the
        // successor whose enclosing unit (0x200) gets tainted.
        cd.tofrom_crossref.insert(AddrLink::pair(a(0x100), a(0x200)), code_unit_flags::JUMP);

        cd.find_not_code_units();

        // 0x100 stays notcode; 0x200's enclosing unit picked up the taint.
        assert_ne!(cd.codeunit[&a(0x100)].flags & code_unit_flags::NOTCODE, 0);
        assert_ne!(cd.codeunit[&a(0x200)].flags & code_unit_flags::NOTCODE, 0);
        // Both crossref maps drained for the tainted block's edges.
        assert!(cd.fromto_crossref.is_empty());
        assert!(cd.tofrom_crossref.is_empty());
        // Taint work-list fully consumed.
        assert!(cd.taintlist.is_empty());
    }

    /// `push_taint_address` (codedata.cc:148) containment boundary: a `.b` that
    /// falls *past* the last code unit's extent (`key+size-1 < addr`) must NOT be
    /// tainted, while a `.b` exactly at the last byte must.  Drives the taint
    /// directly to isolate the off-by-one.
    #[test]
    fn verify_w9_push_taint_address_containment_boundary() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let a = |o: u64| Address::new(Rc::clone(&ram), o);

        // Unit at 0x100 spanning [0x100, 0x104) (size 4): last byte is 0x103.
        let mut cd = CodeDataAnalysis::default();
        cd.codeunit
            .insert(a(0x100), CodeUnit { flags: code_unit_flags::FALLTHRU, size: 4 });

        // addr == 0x103 (last byte): key(0x100)+4-1 = 0x103, NOT < 0x103 -> taint.
        cd.push_taint_address(&a(0x103));
        assert_eq!(cd.taintlist, vec![a(0x100)]);
        cd.taintlist.clear();

        // addr == 0x104 (one past): 0x103 < 0x104 -> no taint.
        cd.push_taint_address(&a(0x104));
        assert!(cd.taintlist.is_empty());

        // A notcode unit is never re-tainted regardless of containment.
        cd.codeunit.get_mut(&a(0x100)).unwrap().flags |= code_unit_flags::NOTCODE;
        cd.push_taint_address(&a(0x100));
        assert!(cd.taintlist.is_empty());
    }

    /// `dump_model_hits` (codedata.cc:603) multi-line sequential grammar and the
    /// `(int4)(off-endoff)` cast.  Three disjoint ranges in one space produce two
    /// inter-range gap numbers and a final lone range with no trailing number.
    /// The cast is exercised by a gap whose magnitude is well within i32; the
    /// point is the exact "0x<first> 0x<last> <dec-gap>\n" grammar that any
    /// downstream parser greps.
    #[test]
    fn verify_w9_dump_model_hits_multiline_gap_grammar() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut cd = CodeDataAnalysis::default();

        // insertRange(space, first, last): last is stored verbatim (one-past, as
        // runModel passes address+size). Three disjoint ranges.
        cd.modelhits.insert_range(Rc::clone(&ram), 0x1000, 0x1010);
        cd.modelhits.insert_range(Rc::clone(&ram), 0x2000, 0x2010);
        cd.modelhits.insert_range(Rc::clone(&ram), 0x3000, 0x3008);

        let mut out = String::new();
        cd.dump_model_hits(&mut out);
        // Range 1: "0x1000 0x1010", gap to next.first 0x2000 - last 0x1010 = 0xFF0 = 4080.
        // Range 2: "0x2000 0x2010", gap 0x3000 - 0x2010 = 0xFF0 = 4080.
        // Range 3: "0x3000 0x3008" (no trailing gap).
        assert_eq!(out, "0x1000 0x1010 4080\n0x2000 0x2010 4080\n0x3000 0x3008\n");

        // Single range: no trailing number at all.
        let mut cd2 = CodeDataAnalysis::default();
        cd2.modelhits.insert_range(Rc::clone(&ram), 0x40, 0x80);
        let mut out2 = String::new();
        cd2.dump_model_hits(&mut out2);
        assert_eq!(out2, "0x40 0x80\n");

        // Empty: no output.
        let cd3 = CodeDataAnalysis::default();
        let mut out3 = String::new();
        cd3.dump_model_hits(&mut out3);
        assert_eq!(out3, "");
    }

    /// `find_unlinked` (codedata.cc:462) classification: a code unit with NONE of
    /// {hit_by_fallthru,hit_by_jump,hit_by_call,notcode,errantstart} is an
    /// unlinked start; one with any of those is not.  Exercises the exact flag
    /// mask (a `notcode` unit is excluded even though it has no hit_by_* bits).
    #[test]
    fn verify_w9_find_unlinked_flag_mask_classification() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let t = synth(BTreeMap::new(), Rc::clone(&ram));
        let code_space = Rc::clone(m.get_default_code_space().unwrap());
        let a = |o: u64| Address::new(Rc::clone(&ram), o);

        let mut cd = CodeDataAnalysis::default();
        // 0x10: clean -> unlinked.
        cd.codeunit.insert(a(0x10), CodeUnit { flags: 0, size: 4 });
        // 0x20: hit_by_fallthru -> not unlinked.
        cd.codeunit
            .insert(a(0x20), CodeUnit { flags: code_unit_flags::HIT_BY_FALLTHRU, size: 4 });
        // 0x30: notcode (no hit_by_*) -> still NOT unlinked (mask includes notcode).
        cd.codeunit
            .insert(a(0x30), CodeUnit { flags: code_unit_flags::NOTCODE, size: 4 });
        // 0x40: errantstart -> not unlinked.
        cd.codeunit
            .insert(a(0x40), CodeUnit { flags: code_unit_flags::ERRANTSTART, size: 4 });

        cd.find_unlinked(&t, &code_space).unwrap();

        assert_eq!(cd.unlinkedstarts, vec![a(0x10)]);
        // dumpUnlinked prints each as "0x<off>\n" in codeunit (BTree) order.
        let mut out = String::new();
        cd.dump_unlinked(&mut out);
        assert_eq!(out, "0x10\n");
    }

    /// Build a minimal `IfaceStatus` for the command tests (a detached source).
    fn make_status() -> IfaceStatus {
        struct NoSource;
        impl crate::interface::LineSource for NoSource {
            fn read_line(&mut self, _s: &mut IfaceStatus) -> String {
                String::new()
            }
            fn push_input(&mut self, _lines: Vec<String>) {}
            fn pop_input(&mut self) {}
            fn current_stream_finished(&self) -> bool {
                true
            }
        }
        IfaceStatus::with_default_history("> ", Box::new(NoSource))
    }
}
