//! (kuna) The cross-reference query behind `kuna xrefs` — "what references this?"
//! and "what does this reference?", answered over the decoded instruction stream.
//!
//! This is a **read-only query**, not an [`AnalysisPass`](crate::pass::AnalysisPass):
//! it produces no facts, commits nothing, and is never wired into
//! `commit_pending_analysis`, so no existing invocation can change output because
//! of it. It runs on demand, after the caller has already bootstrapped and
//! committed a program, and re-walks the same bytes the Listing tier walks.
//!
//! # Why not [`super::Listing`]
//!
//! The keystone Listing already models Call/Code edges, but its xref model is
//! deliberately control-flow-only ([`RefKind::Data`](super::model::RefKind::Data)
//! and friends are documented as reserved and never populated), it files
//! fall-through as an edge, and it drops the p-code after classification
//! ([`Insn::pcode`](super::model::Insn::pcode) is lazily `None`). An RE agent
//! asking "who touches this string / global / function pointer" needs exactly the
//! part the Listing does not keep. So this module runs the same recursive descent
//! and keeps the *whole* p-code op — every input varnode, not just `in0` — long
//! enough to read the data references out of it.
//!
//! # Where the data references come from
//!
//! SLEIGH resolves a PC-relative operand at decode time, so an instruction's
//! absolute target is already in the p-code it emits — in one of two shapes, and
//! both have to be read or half the references vanish:
//!
//!  * **A varnode in the default data space.** A memory operand whose address is
//!    a decode-time constant is exported as a direct `ram` varnode, not a `LOAD`
//!    — `MOV EAX,[RIP+0x2c3a]` lifts to `EAX = COPY (ram,0x4014,4)`. A `ram`
//!    *input* is a [`XrefKind::Read`] of that address, a `ram` *output* a
//!    [`XrefKind::Write`].
//!  * **A constant-space input varnode.** The value form: `LEA RDI,[RIP+0x36a]`
//!    lifts to `RDI = COPY 0x13c9:8`, and a `LOAD`/`STORE` through a
//!    computed-then-folded address carries the pointer as a constant. Scanning
//!    those constants is the faithful projection of Ghidra's per-operand `Scalar`
//!    walk — the same projection [`crate::operand_refs`] makes — so it reuses that
//!    pass's upstream `ScalarOperandAnalyzer.checkOperands` value filter
//!    (`>= 4096`, no byte masks), and a bare integer that happens to look like an
//!    address is not reported. A materialized address that is not dereferenced is
//!    [`XrefKind::Data`]: the address-taken case — a function pointer, a string
//!    pointer, a global's address.
//!
//! # One import, two addresses
//!
//! An imported function is reached through an indirection, and both ends of that
//! indirection carry the import's name. A PE has the **IAT slot** the loader
//! fills in and a MinGW **`FF 25` veneer** (`jmp qword ptr [slot]`) that a direct
//! `call` can target; `pe_iat` registers the import name on both, so
//! `kuna functions --filter VirtualProtect` answers with two entries. An ELF PLT
//! is the same shape with the GOT slot playing the IAT's role.
//!
//! Which of the two a given call site references is a compiler decision the agent
//! asking "who calls VirtualProtect?" has no reason to care about, and answering
//! per-address makes the tool lie by omission in both directions: a program that
//! calls only through the slot reports the veneer as referenced by nothing, and a
//! program that calls only the veneer reports the slot as referenced by nothing.
//! So [`XrefIndex::refs_to_unified`] answers over the whole **alias class** —
//! the veneer and the slot it jumps through, joined by the decoded forwarding
//! edge itself ([`veneer_at`]), never by a shared name. The forwarding jump is
//! excluded from the answer: it is the other half of the callable, not a caller
//! of it, which is what makes the two addresses answer identically.

use std::collections::{BTreeMap, BTreeSet, VecDeque, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};
use object::read::{Object, ObjectSection, ObjectSegment};
use object::SectionKind;

use super::classify::classify;
use super::context::ContextPainter;
use super::kuna_picbase::{self, Ctx as PicCtx, PicBase};
use super::kuna_picpool::PicPool;
use super::kuna_poolref::PoolImage;
use super::kuna_switchtable;
use super::model::{FlowKind, RawOp};

/// ELF section-header flag `SHF_ALLOC` (the section occupies memory at runtime).
const SHF_ALLOC: u64 = 0x2;

/// `ScalarOperandAnalyzer.checkOperands`: a value below this "could be a number,
/// even if it is in the address space". Shared floor with [`crate::operand_refs`].
const MIN_ADDRESS_VALUE: u64 = 4096;

/// `ScalarOperandAnalyzer.checkOperands`: byte-mask values that are never
/// addresses however well they land. Ported alongside [`crate::operand_refs`].
const MASK_VALUES: [u64; 10] = [
    0xffff, 0xff00, 0xffffff, 0xff0000, 0xff00ff, 0xffffffff, 0xffffff00, 0xffff0000, 0xff000000,
    0xff,
];

/// The kind of a cross-reference edge, in the vocabulary the DecLib CLI's
/// `xref_to` / `xref_from` rows carry (`kind` is the field an agent filters on).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum XrefKind {
    /// A direct CALL to the target (a call site).
    Call,
    /// A direct branch to the target (a tail call, a PLT thunk, a loop edge).
    Jump,
    /// The target's address is materialized as a value (address-taken).
    Data,
    /// The target is loaded from.
    Read,
    /// The target is stored to.
    Write,
}

impl XrefKind {
    /// The wire name (`kind` in the JSON surface).
    pub fn as_str(self) -> &'static str {
        match self {
            XrefKind::Call => "call",
            XrefKind::Jump => "jump",
            XrefKind::Data => "data",
            XrefKind::Read => "read",
            XrefKind::Write => "write",
        }
    }
}

/// One `from -> to` reference edge, carrying the rendered source instruction so a
/// caller never has to re-disassemble to explain the row.
#[derive(Debug, Clone)]
pub struct Xref {
    /// VMA of the referencing instruction.
    pub from: u64,
    /// VMA of the referenced location.
    pub to: u64,
    /// What kind of reference this is.
    pub kind: XrefKind,
    /// The referencing instruction's disassembly (`"CALL 0x00001030"`), empty if
    /// the assembly emit produced nothing.
    pub instruction: String,
}

/// Every reference edge the walk found, indexed both ways.
pub struct XrefIndex {
    /// Incoming edges, keyed by target VMA; sorted by source then kind.
    by_target: BTreeMap<u64, Vec<Xref>>,
    /// Outgoing edges, keyed by the referencing instruction's VMA.
    by_source: BTreeMap<u64, Vec<Xref>>,
    /// Outgoing edges, keyed by the entry of the function the source lies in;
    /// sorted by target then source.
    by_source_function: BTreeMap<u64, Vec<Xref>>,
    /// Every instruction VMA the walk decoded (membership only).
    decoded: HashSet<u64>,
    /// Every function entry the walk seeded or discovered, in address order.
    funcs: BTreeSet<u64>,
    /// Function entries whose decoded body contains a `CALLIND`
    /// ([`XrefIndex::has_indirect_calls`]).
    indirect_callers: BTreeSet<u64>,
    /// Forwarding veneers, keyed by function entry ([`veneer_at`]).
    veneers: BTreeMap<u64, Veneer>,
    /// The reverse of [`XrefIndex::veneers`]: a slot mapped to every veneer that
    /// forwards through it (normally one, but a program may emit several).
    veneers_of_slot: BTreeMap<u64, Vec<u64>>,
    /// How many distinct instructions the walk decoded (a coverage signal for a
    /// caller that wants to say "nothing decoded" rather than "no references").
    insns: usize,
}

impl XrefIndex {
    /// Everything that references `vma` — call sites, branches, and data
    /// references — sorted by source VMA.
    pub fn refs_to(&self, vma: u64) -> &[Xref] {
        self.by_target.get(&vma).map_or(&[], Vec::as_slice)
    }

    /// Everything that references the *callable* `vma` names, rather than the
    /// literal address: [`refs_to`](Self::refs_to) taken over `vma`'s whole
    /// [`alias_class`](Self::alias_class), with the forwarding jumps that join
    /// the class to itself removed.
    ///
    /// This is the answer to "who calls VirtualProtect?" on an import that a
    /// program reaches through a veneer, a slot, or both — see the module
    /// header. Off an alias class it is exactly `refs_to`.
    pub fn refs_to_unified(&self, vma: u64) -> Vec<&Xref> {
        let class = self.alias_class(vma);
        if class.len() == 1 {
            return self.refs_to(vma).iter().collect();
        }
        // A veneer's own `jmp [slot]` is not a reference TO the import, it IS the
        // import's other half; counting it would make the two addresses answer
        // differently for no reason a caller can see. The exclusion is the
        // veneer's exact instruction range and nothing wider: ordered containment
        // would swallow whatever code happens to follow the veneer in memory
        // before the next known entry, which is a real caller.
        let bodies: Vec<(u64, u64)> = class
            .iter()
            .filter_map(|m| self.veneers.get(m).map(|v| (*m, v.end)))
            .collect();
        let mut rows: Vec<&Xref> = Vec::new();
        for &member in &class {
            for r in self.refs_to(member) {
                if bodies.iter().any(|&(lo, hi)| r.from >= lo && r.from < hi) {
                    continue;
                }
                rows.push(r);
            }
        }
        rows.sort_by(|a, b| {
            a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)).then_with(|| a.kind.cmp(&b.kind))
        });
        rows
    }

    /// Every address that names the same callable as `vma`, `vma` included: a
    /// forwarding veneer and the pointer slot it jumps through are one import
    /// under two addresses.
    ///
    /// The class is the connected component of the forwarding relation, so two
    /// veneers through one slot are in it together. It is derived from decoded
    /// `jmp [slot]` instructions only — never from two symbols sharing a name,
    /// which would fold genuinely distinct functions together.
    pub fn alias_class(&self, vma: u64) -> BTreeSet<u64> {
        let mut class = BTreeSet::from([vma]);
        let mut queue = vec![vma];
        while let Some(at) = queue.pop() {
            let slot = self.veneers.get(&at).map(|v| v.slot).into_iter();
            let veneers = self.veneers_of_slot.get(&at).into_iter().flatten().copied();
            for next in slot.chain(veneers) {
                if class.insert(next) {
                    queue.push(next);
                }
            }
        }
        class
    }

    /// The fixed pointer slot the forwarding veneer at `entry` jumps through, or
    /// `None` when `entry` is not a veneer.
    pub fn veneer_slot(&self, entry: u64) -> Option<u64> {
        self.veneers.get(&entry).map(|v| v.slot)
    }

    /// Everything the single instruction at `vma` references.
    pub fn refs_from_instruction(&self, vma: u64) -> &[Xref] {
        self.by_source.get(&vma).map_or(&[], Vec::as_slice)
    }

    /// Everything the function entered at `entry` references: its callees, the
    /// functions it tail-jumps to, and the data it touches.
    ///
    /// Intra-function branches are dropped — a loop edge inside the body is
    /// control flow, not a cross-reference, and listing it would bury the
    /// callees an agent asked for. They remain visible from the other direction
    /// (`refs_to` on the branch target returns them).
    pub fn refs_from_function(&self, entry: u64) -> Vec<&Xref> {
        self.by_source_function
            .get(&entry)
            .map_or(Vec::new(), |refs| {
                refs.iter()
                    .filter(|r| {
                        r.kind != XrefKind::Jump || self.function_containing(r.to) != Some(entry)
                    })
                    .collect()
            })
    }

    /// The function containing `vma`: the greatest known entry `<= vma`, the
    /// ordered containment Ghidra's `FunctionManager` answers with.
    ///
    /// `None` unless the walk actually decoded `vma`, so a data address never
    /// gets attributed to whichever function happens to precede it in memory.
    pub fn function_containing(&self, vma: u64) -> Option<u64> {
        if !self.decoded.contains(&vma) {
            return None;
        }
        self.funcs.range(..=vma).next_back().copied()
    }

    /// Did the walk treat `vma` as a function entry (seeded or CALL-discovered)?
    pub fn is_function_entry(&self, vma: u64) -> bool {
        self.funcs.contains(&vma)
    }

    /// How many distinct instructions the walk decoded.
    pub fn instruction_count(&self) -> usize {
        self.insns
    }

    /// Does the walk of the function entered at `entry` decode a **computed
    /// call** — a `CALLIND`, whose destination is a value rather than an
    /// address?
    ///
    /// Such a call files no Call edge (there is no static target to file one
    /// against), so a caller reading [`Self::refs_from_function`] cannot
    /// otherwise tell "this function calls nothing" from "this function's
    /// callee is decided at run time".
    ///
    /// `CALLIND` only. An indirect *branch* is not one: a jump table and a
    /// forwarding veneer's `jmp [slot]` both lift to `BRANCHIND`, and the
    /// veneer's target is recoverable anyway ([`Self::veneer_slot`]). The call
    /// site is attributed by the same ordered containment
    /// [`Self::refs_from_function`] buckets that instruction's references by, so
    /// the two can never name different functions.
    pub fn has_indirect_calls(&self, entry: u64) -> bool {
        self.indirect_callers.contains(&entry)
    }
}

/// One emitted p-code op, whole: the opcode, the output varnode, and every
/// input. [`super::decode::decode_one`] keeps only `in0` (all the flow
/// classifier needs); the parts it drops are what the data-reference scan is
/// made of — the output says a memory location was written, the later inputs
/// carry the addresses.
#[derive(Clone)]
pub(super) struct FullOp {
    pub(super) opcode: OpCode,
    pub(super) out: Option<VarnodeData>,
    pub(super) ins: Vec<VarnodeData>,
}

/// A capturing [`PcodeEmit`] that keeps every emitted op whole.
///
/// One capture is reused for the whole walk: [`FullCapture::begin`] rewinds the
/// cursor instead of dropping the ops, so each slot's input vector is refilled in
/// place. Allocating per op cost one heap allocation for every p-code op in the
/// program (1.44 M on a 466 KB obfuscated i386 image).
#[derive(Default)]
struct FullCapture {
    ops: Vec<FullOp>,
    /// How many of `ops` the current instruction has filled.
    filled: usize,
}

impl FullCapture {
    /// Start capturing a new instruction over the retained storage.
    fn begin(&mut self) {
        self.filled = 0;
    }

    /// The ops the current instruction emitted.
    fn ops(&self) -> &[FullOp] {
        &self.ops[..self.filled]
    }
}

impl PcodeEmit for FullCapture {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        if self.filled == self.ops.len() {
            self.ops.push(FullOp { opcode: opc, out: outvar.cloned(), ins: vars.to_vec() });
        } else {
            let slot = &mut self.ops[self.filled];
            slot.opcode = opc;
            slot.out = outvar.cloned();
            slot.ins.clear();
            slot.ins.extend_from_slice(vars);
        }
        self.filled += 1;
    }
}

/// A capturing [`AssemblyEmit`] for the one instruction being decoded.
#[derive(Default)]
struct AsmCapture {
    text: String,
}

impl AssemblyEmit for AsmCapture {
    fn dump(&mut self, _addr: &Address, mnem: &str, body: &str) {
        self.text.clear();
        self.text.push_str(mnem);
        if !body.is_empty() {
            self.text.push(' ');
            self.text.push_str(body);
        }
    }
}

/// The seed set a STANDALONE reference walk needs: the caller's committed
/// function inventory, plus the `<patternpairs>` prologue starts on the
/// architectures the drivers route through the Listing tier (DIV-20/DIV-68).
///
/// `kuna xrefs` does its own recursive descent ([`build`] follows every direct
/// CALL out of its seeds), so the only thing the analysis-tier Listing walk
/// contributed to a reference query was a richer seed set — and building that
/// Listing means decoding the whole program a second time. Handing the same
/// prologue starts straight to the reference walk keeps the seeds and drops the
/// duplicate decode. x86-64 is untouched (its entry oracles already carry the
/// prologue scan, and the drivers inject nothing there), so the seed set on that
/// architecture is exactly the caller's inventory.
pub fn discovery_seeds(file: &object::File, entries: &[u64], patterns: bool) -> Vec<u64> {
    let mut seeds: Vec<u64> = entries.to_vec();
    if patterns && file.architecture() != object::Architecture::X86_64 {
        let execs = crate::entry::executable_sections(file);
        seeds.extend(
            crate::entry::full_pattern_starts(file)
                .into_iter()
                .filter(|&vma| crate::entry::in_executable_section(&execs, vma)),
        );
    }
    seeds.sort_unstable();
    seeds.dedup();
    seeds
}

/// Walk every function reachable from `seeds` and index every reference edge.
///
/// `file` supplies the section partition (which VMAs are code, which are mapped
/// data); `arch` + `translate` are the live engine the caller already
/// bootstrapped, so the decode reads through the loader that is already attached
/// and honours the same decode-mode context (ARM Thumb / MIPS16) the Listing
/// walk paints. `seeds` is the caller's function inventory — the walk explores
/// the call graph out of it, so a callee the inventory missed is still covered.
///
/// Never fails and never panics past a bad decode: an undecodable address just
/// ends that path, exactly as [`super::walk`] does.
pub fn build(
    file: &object::File,
    arch: &Architecture,
    translate: &dyn Translate,
    seeds: &[u64],
) -> XrefIndex {
    build_with_focus(file, arch, translate, seeds, &[])
}

/// [`build`], plus the addresses the CALLER named.
///
/// A recursive descent answers for the code it can reach, and the one address a
/// reference query is certainly interested in is the one it was asked about — an
/// entry reached only through a function-pointer table has no direct CALL edge
/// pointing at it, so no seed set built from prologues and symbols reaches it and
/// `--from <that entry>` answers zero references about a function that plainly
/// has some. Each `focus` address is walked as a function of its own, but only
/// AFTER the seeded walk has drained: anything the natural descent claims is
/// already in `decoded` and skipped, so a focus address can only ADD coverage,
/// never re-attribute an instruction some other entry already owns. An address
/// that does not decode is dropped rather than recorded as a function.
pub fn build_with_focus(
    file: &object::File,
    arch: &Architecture,
    translate: &dyn Translate,
    seeds: &[u64],
    focus: &[u64],
) -> XrefIndex {
    let Some(code_space) = arch.manage().get_default_code_space().map(Rc::clone) else {
        return empty();
    };

    let exec: Vec<(u64, u64)> = {
        let mut r: Vec<(u64, u64)> = crate::entry::executable_sections(file)
            .into_iter()
            .map(|(lo, hi, _data)| (lo, hi))
            .collect();
        r.sort_unstable();
        r
    };
    // A relocatable object is laid out synthetically by the loader, so the raw
    // `object` view's section addresses are pre-link and describe a different
    // address space than the seeds live in (`reloc_object::is_synthetically_laid_out`).
    // Detect it structurally instead of by format: if not one seed lands in an
    // executable section, the partition is not the runtime one — decline to gate
    // on it (the decode's own "no bytes here" error bounds the walk) and decline
    // to classify data references against it (they would all be wrong).
    let sections_are_runtime = !exec.is_empty() && seeds.iter().any(|&s| in_range(&exec, s));
    let mapped = if sections_are_runtime { mapped_ranges(file) } else { Vec::new() };

    // Paint the decode-mode context (ARM `TMode` / MIPS `ISA_MODE`) before the
    // first decode, exactly as the Listing walk does — without it a Thumb
    // function reads as A32 garbage and its references are fiction. Empty (and
    // free) on x86-64 and every language with no decode-mode context.
    let painter = ContextPainter::new(file);
    if !painter.is_empty() {
        painter.paint_all(arch, &code_space);
    }

    let mut seed_set: BTreeSet<u64> = seeds.iter().copied().collect();
    if sections_are_runtime {
        seed_set.retain(|&s| in_range(&exec, s));
    }

    // The space a direct memory operand lives in (`ram` on every vendored
    // processor). `None` when the program has no data space: no varnode can
    // match it, so the direct-access projection simply contributes nothing.
    let data_space = arch.manage().get_default_data_space().cloned();

    // (kuna) `picbase`: the module's PIC base register, when the program has one
    // this can prove. Detected before the walk because the base a function
    // *inherits* is established in a different function's prologue -- see
    // `kuna_picbase`. `None` (every non-PIC target, every image with no GOT)
    // leaves the walk below byte-identical to the pre-feature one.
    let picbase: Option<(PicCtx, PicBase)> = if arch.analysis_picbase && !mapped.is_empty() {
        PicCtx::new(arch, data_space.as_ref()).and_then(|ctx| {
            kuna_picbase::detect(file, translate, &code_space, &ctx, &seed_set).map(|b| (ctx, b))
        })
    } else {
        None
    };
    let mut pc_thunks = std::collections::HashMap::new();

    // (kuna) `poolref`: the read-only half of the image, so a read of a literal
    // pool word can be followed to what it points at. `None` (an image with no
    // read-only mapped section, and every relocatable object, whose sections are
    // not the runtime ones) leaves every reference below exactly as it was.
    let pool = if mapped.is_empty() { None } else { PoolImage::new(file) };

    let mut st = State {
        by_target: BTreeMap::new(),
        by_source: BTreeMap::new(),
        decoded: HashSet::new(),
        funcs: seed_set.clone(),
        indirect_call_sites: BTreeSet::new(),
    };

    // Reused across every decode in the walk (see [`FullCapture`]).
    let mut cap = FullCapture::default();
    let mut raw: Vec<RawOp> = Vec::new();

    // (kuna, `aif`) The instruction partition the gap-walk consumes, recorded
    // only when it will run. A `push` per decode is the whole cost of keeping
    // AIF reachable without a second decode of the program.
    let gapwalk = arch.analysis_aif;
    let mut gapwalk_done = false;
    let mut partition: Vec<(u64, u32)> = Vec::new();

    let mut func_queue: VecDeque<u64> = seed_set.iter().copied().collect();
    let mut walked: HashSet<u64> = HashSet::new();

    // The caller-named addresses the seeded walk did not already cover, tried one
    // at a time once the queue drains (see [`build_with_focus`]).
    let mut pending_focus: Vec<u64> =
        focus.iter().copied().filter(|f| !seed_set.contains(f)).collect();
    pending_focus.sort_unstable();
    pending_focus.dedup();
    pending_focus.reverse();
    let mut focused: Vec<u64> = Vec::new();

    loop {
        // The seeded walk drains first; only then is the next caller-named
        // address it never reached taken up as a function of its own.
        let entry = match func_queue.pop_front() {
            Some(entry) => entry,
            None => match pending_focus.pop() {
                Some(f) => {
                    if st.decoded.contains(&f) || (sections_are_runtime && !in_range(&exec, f)) {
                        continue;
                    }
                    st.funcs.insert(f);
                    focused.push(f);
                    f
                }
                // (kuna, `aif`) Nothing reachable is left: run the speculative
                // gap-walk over the partition THIS walk left behind, and take up
                // whatever it finds as more entries to walk. See `gap_entries`.
                None if gapwalk && !gapwalk_done => {
                    gapwalk_done = true;
                    let found = gap_entries(
                        arch,
                        translate,
                        &code_space,
                        &partition,
                        &st.funcs,
                        &exec,
                    );
                    pending_focus.extend(found.into_iter().rev());
                    continue;
                }
                None => break,
            },
        };
        if !walked.insert(entry) {
            continue;
        }
        let mut insn_queue: VecDeque<u64> = VecDeque::from([entry]);
        // Only collected when a base exists: the admission rule needs the whole
        // body before any of it can be attributed (`kuna_picbase::scope`), and
        // buffering it costs nothing on the overwhelmingly common `None` path.
        let mut body: Vec<kuna_picbase::BaseCandidate> = Vec::new();
        // (kuna) `picpool`: the pool words this body is carrying towards the
        // `add` that turns each into an address. Per function, because the values
        // it tracks are live only inside one straight-line run.
        let mut picpool = PicPool::default();
        while let Some(vma) = insn_queue.pop_front() {
            if st.decoded.contains(&vma) {
                continue; // already decoded (the VisitStat dedup)
            }
            // Never walk out of this function into another *known* entry: the
            // instructions past that boundary belong to that function's own walk,
            // and mis-attributing them would put a callee's call sites under this
            // caller's name.
            if vma != entry && seed_set.contains(&vma) {
                continue;
            }
            if sections_are_runtime && !in_range(&exec, vma) {
                continue; // out of bounds (the `flow.rs` gate)
            }
            let Some(len) = decode(translate, vma, &code_space, &mut cap) else {
                continue; // undecodable (or zero-length): stop this path
            };
            st.decoded.insert(vma);
            if gapwalk {
                partition.push((vma, len));
            }

            raw.clear();
            raw.extend(
                cap.ops()
                    .iter()
                    .map(|op| RawOp { opcode: op.opcode, in0: op.ins.first().cloned() }),
            );
            let c = classify(&raw, vma, len);
            let drefs = if mapped.is_empty() {
                Vec::new()
            } else {
                let fall_through = vma.wrapping_add(len as u64);
                data_refs(cap.ops(), data_space.as_ref(), &mapped, fall_through, pool.as_ref())
            };
            // (kuna) `picpool`: the address a PC-relative literal-pool pair forms,
            // which is in neither of its two instructions on its own. See
            // [`super::kuna_picpool`].
            let picrefs = match pool.as_ref() {
                Some(p) => picpool.step(
                    vma,
                    len,
                    cap.ops(),
                    p,
                    &mapped,
                    data_space.as_ref(),
                ),
                None => Vec::new(),
            };
            // Every row this instruction produces carries the same render, and an
            // instruction that produces none needs no render at all.
            let text = if c.flows.is_empty() && drefs.is_empty() && picrefs.is_empty() {
                String::new()
            } else {
                assembly(translate, vma, &code_space)
            };

            if cap.ops().iter().any(|op| op.opcode == OpCode::CPUI_CALLIND) {
                st.indirect_call_sites.insert(vma);
            }

            for &target in &c.flows {
                let kind = if c.flow.is_call { XrefKind::Call } else { XrefKind::Jump };
                st.file(vma, target, kind, &text);
                if c.flow.is_call {
                    st.funcs.insert(target);
                    func_queue.push_back(target);
                } else {
                    insn_queue.push_back(target);
                }
            }
            if let Some(fall) = c.fall_through {
                // Fall-through is not a reference; it is only a walk successor.
                insn_queue.push_back(fall);
            }

            // (kuna) `switchtable`: a computed jump has no static successor, so
            // the descent stops dead at every switch dispatch and the case
            // bodies -- and every reference they form -- are invisible. Read the
            // table the jump indexes and take its entries up as successors of
            // THIS function: a case body is the dispatcher's own code. See
            // [`super::kuna_switchtable`].
            if c.flows.is_empty() && c.flow.is_computed && c.flow.is_jump && !c.flow.is_call {
                if let Some(p) = pool.as_ref() {
                    for &(base, kind) in &drefs {
                        if kind != XrefKind::Data {
                            continue;
                        }
                        for target in kuna_switchtable::targets(base, vma, p, &exec) {
                            st.file(vma, target, XrefKind::Jump, &text);
                            insn_queue.push_back(target);
                        }
                    }
                }
            }

            for &(to, kind) in &drefs {
                st.file(vma, to, kind, &text);
            }
            for &to in &picrefs {
                st.file(vma, to, XrefKind::Data, &text);
            }
            // Both halves the deferred base-relative pass needs are pure
            // functions of this instruction's ops, so they are computed here
            // rather than by buffering (and cloning) the whole p-code.
            if let Some((ctx, base)) = &picbase {
                let fall_through = vma.wrapping_add(u64::from(len));
                body.push(kuna_picbase::BaseCandidate {
                    vma,
                    writes_base: kuna_picbase::writes_base(cap.ops(), base),
                    refs: kuna_picbase::refs_through_base(
                        cap.ops(),
                        base,
                        ctx,
                        &mapped,
                        fall_through,
                    ),
                });
            }
        }

        // (kuna) `picbase`: the references this body forms THROUGH the base
        // register, filed only where the body cannot have changed it.
        if let Some((ctx, base)) = &picbase {
            body.sort_by_key(|c| c.vma);
            if let Some(scope) =
                kuna_picbase::scope(translate, &code_space, ctx, &mut pc_thunks, base, &body)
            {
                for cand in &body {
                    if cand.refs.is_empty() || !scope.admits(cand.vma) {
                        continue;
                    }
                    // Rendered here, not in the walk: an admitted instruction
                    // that forms a reference is rare, and rendering every
                    // buffered instruction up front was a second full SLEIGH
                    // parse of the whole program on any image with a PIC base
                    // (an i386 `__x86.get_pc_thunk` binary: 154,608 renders to
                    // file 151 references).
                    let text = assembly(translate, cand.vma, &code_space);
                    for &(to, kind) in &cand.refs {
                        st.file(cand.vma, to, kind, &text);
                    }
                }
            }
            let _ = ctx;
        }
    }
    // A focus address that did not decode is not a function: recording it as one
    // would answer `sub_<addr>` for a byte in the middle of a string.
    for f in focused {
        if !st.decoded.contains(&f) {
            st.funcs.remove(&f);
        }
    }

    // The forwarding relation, over the entries the walk actually decoded. It
    // re-decodes at most `MAX_VENEER_INSNS` instructions per entry (a veneer is
    // one or two), which is a rounding error beside the walk itself, and keeps
    // the detection readable instead of threading a per-entry prefix through the
    // BFS above.
    let mut veneers: BTreeMap<u64, Veneer> = BTreeMap::new();
    if !mapped.is_empty() {
        for &entry in &st.funcs {
            if !st.decoded.contains(&entry) {
                continue;
            }
            if let Some(v) =
                veneer_at(translate, &code_space, entry, data_space.as_ref(), &mapped)
            {
                veneers.insert(entry, v);
            }
        }
    }

    st.finish(veneers)
}

/// Ghidra's `MINIMUM_FUNCTION_COUNT`, mirrored here so the partition is not even
/// assembled for a program the gap-walk would decline to fingerprint.
const AIF_MIN_FUNCTIONS: usize = 20;

/// The functions the speculative gap-walk (`aif`) finds in what THIS walk left
/// undecoded.
///
/// A function reached only through a function-pointer table has no direct CALL
/// edge, so a recursive descent structurally cannot reach it and every reference
/// it makes is missing from the answer — on a stripped i386 PE, 61 of the 174
/// callers of one function. That recall is what the analysis-tier Listing was
/// buying a reference query, and it is the only thing it was buying one: the
/// Listing's own walk duplicates this one over the same bytes. Assembling the
/// partition from the decode already done keeps the recall and drops the
/// duplicate decode.
///
/// The gap-walk fingerprints each candidate against the prologues of the already
/// -discovered functions, so the two leading instructions of each are rendered
/// here — `2 * functions` renders, against the whole program's worth the Listing
/// path rendered.
fn gap_entries(
    arch: &Architecture,
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    partition: &[(u64, u32)],
    funcs: &BTreeSet<u64>,
    exec: &[(u64, u64)],
) -> Vec<u64> {
    if funcs.len() < AIF_MIN_FUNCTIONS || partition.is_empty() {
        return Vec::new();
    }
    let mut insns: BTreeMap<u64, super::Insn> = BTreeMap::new();
    for &(addr, len) in partition {
        insns.insert(
            addr,
            super::Insn {
                addr,
                len,
                fall_through: None,
                flow: super::FlowType::default(),
                flows: Vec::new(),
                mnemonic: String::new(),
                operands: String::new(),
                pcode: None,
            },
        );
    }
    // The fingerprint reads the first two instructions of every discovered
    // function; nothing else in the gap-walk reads a mnemonic.
    for &entry in funcs {
        let mut vma = entry;
        for _ in 0..2 {
            let Some(insn) = insns.get(&vma) else { break };
            let next = vma.wrapping_add(u64::from(insn.len));
            let text = assembly(translate, vma, code_space);
            if let Some(slot) = insns.get_mut(&vma) {
                slot.mnemonic =
                    text.split_whitespace().next().unwrap_or_default().to_string();
            }
            vma = next;
        }
    }
    let listing = super::Listing::from_partition(
        insns,
        funcs
            .iter()
            .map(|&entry| {
                (
                    entry,
                    super::DiscoveredFunction {
                        entry,
                        name: None,
                        from_symbol: false,
                        has_no_return: false,
                        call_fixup: None,
                    },
                )
            })
            .collect(),
        exec.to_vec(),
    );
    crate::aif::run_aif(
        &listing,
        translate,
        Rc::clone(code_space),
        listing.exec_ranges(),
        arch.analysis_aifstrict,
        arch.analysis_aifcorroborate,
    )
}

/// A forwarding veneer: the fixed pointer slot it jumps through, and the VMA one
/// past its own last instruction. The extent is what lets the unified answer
/// exclude the veneer's own forwarding jump without excluding the unrelated code
/// that happens to sit after it in memory.
#[derive(Debug, Clone, Copy)]
struct Veneer {
    slot: u64,
    end: u64,
}

/// How many instructions a forwarding veneer may take to reach its indirect
/// jump. One covers the MinGW `FF 25` import thunk and the legacy ELF `.plt`
/// entry, which lead with the jump; two covers a CET `.plt.sec` entry
/// (`endbr64; jmp *GOT(%rip)`) and a PLT0 resolver stub. Deliberately no more
/// than that: measured over every veneer in the fixture corpus, nothing needs a
/// third instruction, and each one of slack widens the relation from "this
/// function IS the jump" to "this function ends in one", which would fold a
/// tail-calling wrapper into the callable it forwards to.
const MAX_VENEER_INSNS: usize = 2;

/// The forwarding veneer entered at `entry`, or `None` when `entry` is not one.
///
/// A veneer is a function whose control leaves through a single indirect jump to
/// whatever a **decode-time constant** address holds: `jmp qword ptr
/// [__imp_VirtualProtect]` in a PE, `jmp *malloc@GOT(%rip)` in an ELF PLT. The
/// constant-address requirement is what separates a veneer from a jump table —
/// `jmp [rax*8 + table]` computes its address and lifts to a `LOAD` through a
/// temporary, never to a `BRANCHIND` on a `ram` varnode — and it is why the
/// relation can be read straight out of the p-code with no format knowledge.
///
/// The scan follows fall-through from `entry` and refuses at the first static
/// branch, call or return, so only a straight run into the indirect jump counts.
fn veneer_at(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    entry: u64,
    data_space: Option<&Rc<AddrSpace>>,
    mapped: &[(u64, u64)],
) -> Option<Veneer> {
    let mut vma = entry;
    let mut cap = FullCapture::default();
    for _ in 0..MAX_VENEER_INSNS {
        let len = decode(translate, vma, code_space, &mut cap)?;
        let raw: Vec<RawOp> = cap
            .ops()
            .iter()
            .map(|op| RawOp { opcode: op.opcode, in0: op.ins.first().cloned() })
            .collect();
        let c = classify(&raw, vma, len);
        if !c.flows.is_empty() || c.flow.is_call || c.flow.kind == FlowKind::Return {
            return None;
        }
        if let Some(op) = cap.ops().iter().find(|o| o.opcode == OpCode::CPUI_BRANCHIND) {
            let vn = op.ins.first()?;
            let in_data = matches!((&vn.space, data_space), (Some(s), Some(d)) if Rc::ptr_eq(s, d));
            let end = vma.wrapping_add(u64::from(len));
            return (in_data && in_range(mapped, vn.offset))
                .then_some(Veneer { slot: vn.offset, end });
        }
        vma = c.fall_through?;
    }
    None
}

/// The accumulating state of [`build`].
struct State {
    by_target: BTreeMap<u64, Vec<Xref>>,
    by_source: BTreeMap<u64, Vec<Xref>>,
    /// Membership only (the `VisitStat` dedup), so it is hashed rather than
    /// ordered: it is probed once per successor edge over the whole program.
    decoded: HashSet<u64>,
    funcs: BTreeSet<u64>,
    /// VMAs of the decoded `CALLIND` instructions; folded onto their containing
    /// function in [`State::finish`].
    indirect_call_sites: BTreeSet<u64>,
}

impl State {
    fn file(&mut self, from: u64, to: u64, kind: XrefKind, instruction: &str) {
        let r = Xref { from, to, kind, instruction: instruction.to_string() };
        self.by_target.entry(to).or_default().push(r.clone());
        self.by_source.entry(from).or_default().push(r);
    }

    /// Close the index: the per-function bucket is grouped by ordered
    /// containment (not by which entry's descent happened to reach the
    /// instruction first), so a row's `from_function` and the `--from` bucket it
    /// lands in can never disagree. The computed-call set is folded the same
    /// way, for the same reason.
    fn finish(mut self, veneers: BTreeMap<u64, Veneer>) -> XrefIndex {
        let mut by_source_function: BTreeMap<u64, Vec<Xref>> = BTreeMap::new();
        for (&from, refs) in &self.by_source {
            let Some(&entry) = self.funcs.range(..=from).next_back() else {
                continue;
            };
            by_source_function.entry(entry).or_default().extend(refs.iter().cloned());
        }
        let indirect_callers: BTreeSet<u64> = self
            .indirect_call_sites
            .iter()
            .filter_map(|from| self.funcs.range(..=*from).next_back().copied())
            .collect();
        for refs in self.by_target.values_mut() {
            sort_dedup(refs, /* by_source = */ true);
        }
        for refs in self.by_source.values_mut() {
            sort_dedup(refs, /* by_source = */ false);
        }
        for refs in by_source_function.values_mut() {
            sort_dedup(refs, /* by_source = */ false);
        }
        let insns = self.decoded.len();
        let mut veneers_of_slot: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
        for (&entry, v) in &veneers {
            veneers_of_slot.entry(v.slot).or_default().push(entry);
        }
        XrefIndex {
            by_target: self.by_target,
            by_source: self.by_source,
            by_source_function,
            decoded: self.decoded,
            funcs: self.funcs,
            indirect_callers,
            veneers,
            veneers_of_slot,
            insns,
        }
    }
}

/// Lock one bucket's read ordering and collapse duplicates on `(from, to, kind)`,
/// so a target reached twice from one site contributes exactly one row (the same
/// contract [`super::Listing`]'s `finalize_refs` holds).
fn sort_dedup(refs: &mut Vec<Xref>, by_source: bool) {
    refs.sort_by(|a, b| {
        let (pa, sa) = if by_source { (a.from, a.to) } else { (a.to, a.from) };
        let (pb, sb) = if by_source { (b.from, b.to) } else { (b.to, b.from) };
        pa.cmp(&pb).then_with(|| sa.cmp(&sb)).then_with(|| a.kind.cmp(&b.kind))
    });
    refs.dedup_by(|a, b| a.from == b.from && a.to == b.to && a.kind == b.kind);
}

fn empty() -> XrefIndex {
    XrefIndex {
        by_target: BTreeMap::new(),
        by_source: BTreeMap::new(),
        by_source_function: BTreeMap::new(),
        decoded: HashSet::new(),
        funcs: BTreeSet::new(),
        indirect_callers: BTreeSet::new(),
        veneers: BTreeMap::new(),
        veneers_of_slot: BTreeMap::new(),
        insns: 0,
    }
}

/// Decode the instruction at `vma` into `cap`, keeping every input varnode, and
/// return its byte length.
///
/// A translator panic on exotic bytes is contained to `None` — a query surface
/// must never take the process down over one bad address.
fn decode(
    translate: &dyn Translate,
    vma: u64,
    code_space: &Rc<AddrSpace>,
    cap: &mut FullCapture,
) -> Option<u32> {
    let addr = Address::new(Rc::clone(code_space), vma);
    cap.begin();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        translate.one_instruction(cap, &addr)
    }));
    match decoded {
        Ok(Ok(len)) if len > 0 => Some(len as u32),
        _ => None,
    }
}

/// Render the instruction at `vma`, best-effort (empty when the render errs).
///
/// This is a SECOND full SLEIGH parse of the address — `print_assembly` shares no
/// resolved state with `one_instruction` — so the walk pays it only where a row
/// will actually carry the text. On a 466 KB obfuscated i386 image (154,638
/// instructions) rendering every decode cost 0.22 s of a 1.3 s walk, and the
/// large majority of instructions file no reference at all.
fn assembly(translate: &dyn Translate, vma: u64, code_space: &Rc<AddrSpace>) -> String {

    let addr = Address::new(Rc::clone(code_space), vma);
    let mut asm = AsmCapture::default();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = translate.print_assembly(&mut asm, &addr);
    }));
    asm.text
}

/// The data references one instruction's p-code carries.
///
/// Two projections, per the module header: a varnode in the default data space
/// (`data_space`) is a direct memory access — an input reads it, an output
/// writes it — and a constant-space input that survives the
/// `ScalarOperandAnalyzer` value filter is a materialized address.
///
/// Some operand positions are skipped because their constant is not an address
/// at all: a `LOAD`/`STORE` `in0` is the address-space id, a `CALLOTHER` `in0`
/// the userop index, a *direct* flow op's `in0` the branch target (already filed
/// as a Call/Jump edge).
///
/// An **indirect** flow op is deliberately not in that set. `BRANCHIND`/`CALLIND`
/// `in0` is the varnode the destination is read *out of*, not a static target,
/// and [`classify`] files no edge for it — so skipping it loses the reference
/// outright. It is a `ram` varnode exactly in the shape this query exists to
/// answer: SLEIGH lifts `JMP qword ptr [__imp_VirtualProtect]` (a PE import
/// veneer) and `jmp *malloc@GOT(%rip)` (an ELF PLT entry) to `goto [rm64]`,
/// i.e. one `BRANCHIND` whose `in0` is the import slot, and dropping it left
/// every import veneer in the program referencing nothing at all.
///
/// `fall_through` (`vma + len`) is skipped as a value for the same reason: a
/// call materializes its own return address, and every architecture spells that
/// as this instruction's fall-through — x86 stores the constant to the stack,
/// ARM copies it into `LR`, MIPS into `$ra`. Reported, it would put a phantom
/// data reference on the instruction after every single call site.
fn data_refs(
    ops: &[FullOp],
    data_space: Option<&Rc<AddrSpace>>,
    mapped: &[(u64, u64)],
    fall_through: u64,
    pool: Option<&PoolImage>,
) -> Vec<(u64, XrefKind)> {
    let mut out = Vec::new();
    // (kuna) `poolref`: the second edge a literal-pool load forms, from this same
    // instruction to the address the pool word holds. See [`super::kuna_poolref`].
    // `width` is how many bytes the instruction READ, which a `LOAD`'s address
    // varnode does not carry -- it is pointer-sized whatever the access is, so
    // `ldrh r0,[0x1003c]` would read as a pointer-sized load of the pool word.
    let follow = |out: &mut Vec<(u64, XrefKind)>, at: u64, width: Option<u32>| {
        if let Some(target) = width.and_then(|w| pool.and_then(|p| p.follow(at, w, mapped))) {
            out.push((target, XrefKind::Data));
        }
    };
    let read_width = |op: &FullOp, slot: usize, vn: &VarnodeData| match op.opcode {
        OpCode::CPUI_LOAD if slot == 1 => op.out.as_ref().map(|o| o.size),
        OpCode::CPUI_STORE => None,
        _ => Some(vn.size),
    };
    // Space identity is pointer identity throughout the engine (`VarnodeData`'s
    // own `PartialEq` compares spaces that way), so match on the `Rc`, never on
    // the space's name or index.
    let in_data_space = |vn: &VarnodeData| {
        matches!((&vn.space, data_space), (Some(s), Some(d)) if Rc::ptr_eq(s, d))
            && in_range(mapped, vn.offset)
    };
    for op in ops {
        if let Some(vn) = &op.out {
            if in_data_space(vn) {
                out.push((vn.offset, XrefKind::Write));
            }
        }
        for (i, vn) in op.ins.iter().enumerate() {
            let is_target_slot = i == 0
                && matches!(
                    op.opcode,
                    OpCode::CPUI_LOAD
                        | OpCode::CPUI_STORE
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_BRANCH
                        | OpCode::CPUI_CBRANCH
                        | OpCode::CPUI_CALL
                );
            if is_target_slot {
                continue;
            }
            if in_data_space(vn) {
                out.push((vn.offset, XrefKind::Read));
                follow(&mut out, vn.offset, read_width(op, i, vn));
                continue;
            }
            let Some(space) = &vn.space else { continue };
            if space.get_type() != spacetype::IPTR_CONSTANT {
                continue;
            }
            let value = vn.offset;
            if !looks_like_address(value) || !in_range(mapped, value) {
                continue;
            }
            let kind = match op.opcode {
                OpCode::CPUI_LOAD if i == 1 => XrefKind::Read,
                OpCode::CPUI_STORE if i == 1 => XrefKind::Write,
                _ => XrefKind::Data,
            };
            if kind == XrefKind::Data && value == fall_through {
                continue; // this call's own return address
            }
            out.push((value, kind));
            if kind == XrefKind::Read {
                follow(&mut out, value, read_width(op, i, vn));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// `ScalarOperandAnalyzer.checkOperands`' value filter.
pub(super) fn looks_like_address(value: u64) -> bool {
    value >= MIN_ADDRESS_VALUE && !MASK_VALUES.contains(&value)
}

/// The `[lo, hi)` ranges an address must land in to be a data reference: every
/// section the image maps at runtime, code included (an immediate that
/// materializes a function entry is the address-taken case, and is exactly what
/// makes an indirect-call target findable).
///
/// (kuna) An image with NO section table at all presents no sections, so this
/// would answer "nothing is mapped" and every data reference in it would be
/// discarded while its control flow survived — a sectionless PIE's strings come
/// back owner-less and `xrefs --to` a string reports zero. The program header is
/// the other, independent description of the same image and *is* the runtime
/// layout there, so fall back to it, exactly as [`crate::entry::executable_sections`]
/// does. Only the no-section-table arm takes the fallback: an image that has
/// sections which are not the runtime layout (a relocatable object) is declined
/// before this is ever called.
fn mapped_ranges(file: &object::File) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    if file.sections().next().is_none() {
        // `object`'s ELF segment iterator already yields only `PT_LOAD`, which
        // is by definition mapped; a format with no segment view yields none and
        // leaves the answer empty, as before.
        for seg in file.segments() {
            let (lo, size) = (seg.address(), seg.size());
            if size == 0 {
                continue;
            }
            out.push((lo, lo.saturating_add(size)));
        }
        out.sort_unstable();
        return out;
    }
    for sec in file.sections() {
        let (lo, size) = (sec.address(), sec.size());
        if lo == 0 || size == 0 {
            continue;
        }
        let allocated = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
            _ => matches!(
                sec.kind(),
                SectionKind::Text
                    | SectionKind::Data
                    | SectionKind::ReadOnlyData
                    | SectionKind::ReadOnlyDataWithRel
                    | SectionKind::ReadOnlyString
                    | SectionKind::UninitializedData
                    | SectionKind::Common
            ),
        };
        if allocated {
            out.push((lo, lo.saturating_add(size)));
        }
    }
    out.sort_unstable();
    out
}

/// Does `vma` land in any `[lo, hi)` of a sorted, possibly overlapping range list?
pub(super) fn in_range(ranges: &[(u64, u64)], vma: u64) -> bool {
    ranges.iter().any(|&(lo, hi)| vma >= lo && vma < hi)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::space::{spacetype, AddrSpace};
    use kuna_num::opcodes::OpCode;
    use kuna_num::pcoderaw::VarnodeData;

    use super::*;

    /// A throwaway `(ram, constant)` space pair. `ram` stands in for the default
    /// data space; its index is what [`data_refs`] is told to match.
    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>) {
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
        )
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size: 8 }
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> FullOp {
        FullOp { opcode, out, ins }
    }

    const MAPPED: [(u64, u64); 2] = [(0x1000, 0x2000), (0x4000, 0x4100)];

    /// A read-only literal pool at 0x1000: a mapped address, a number, a mapped
    /// address. See [`super::super::kuna_poolref`].
    const POOL: [u8; 12] = [
        0x00, 0x40, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x00, 0x10, 0x40, 0x00, 0x00,
    ];

    fn pool_image() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x1000, 0x100c, &POOL[..])], 4, true).unwrap()
    }

    /// Run [`data_refs`] with `ram` as the default data space and a fall-through
    /// no op under test materializes.
    fn refs(ram: &Rc<AddrSpace>, ops: &[FullOp]) -> Vec<(u64, XrefKind)> {
        data_refs(ops, Some(ram), &MAPPED, 0, None)
    }

    /// The shape a constant-address memory operand actually lifts to: a direct
    /// data-space varnode. An input reads it, an output writes it.
    #[test]
    fn a_direct_data_space_varnode_is_a_read_or_a_write() {
        let (ram, _cst) = spaces();
        let load = op(OpCode::CPUI_COPY, None, vec![vn(&ram, 0x4014)]);
        assert_eq!(refs(&ram, &[load]), vec![(0x4014, XrefKind::Read)]);
        let store = op(OpCode::CPUI_COPY, Some(vn(&ram, 0x4010)), vec![]);
        assert_eq!(refs(&ram, &[store]), vec![(0x4010, XrefKind::Write)]);
    }

    /// A branch/call target is also a data-space varnode, and it is already
    /// filed as control flow — it must never come back as a read.
    #[test]
    fn a_flow_ops_target_is_not_a_data_reference() {
        let (ram, cst) = spaces();
        for opcode in [OpCode::CPUI_CALL, OpCode::CPUI_BRANCH, OpCode::CPUI_CBRANCH] {
            assert!(
                refs(&ram, &[op(opcode, None, vec![vn(&ram, 0x1030)])]).is_empty(),
                "{opcode:?} target leaked as a data reference"
            );
            assert!(
                refs(&ram, &[op(opcode, None, vec![vn(&cst, 0x1030)])]).is_empty(),
                "{opcode:?} constant target leaked as a data reference"
            );
        }
    }

    /// An *indirect* flow op's `in0` is the opposite case: it is the varnode the
    /// destination is read out of, not a static target, and no Call/Jump edge is
    /// filed for it. This is the whole import-veneer shape — `JMP qword ptr
    /// [__imp_X]` is one `BRANCHIND` on the slot — so skipping it loses the only
    /// reference the instruction makes.
    #[test]
    fn an_indirect_flow_ops_operand_is_the_slot_it_reads() {
        let (ram, _cst) = spaces();
        for opcode in [OpCode::CPUI_BRANCHIND, OpCode::CPUI_CALLIND] {
            assert_eq!(
                refs(&ram, &[op(opcode, None, vec![vn(&ram, 0x1030)])]),
                vec![(0x1030, XrefKind::Read)],
                "{opcode:?} lost the slot it jumps through"
            );
        }
        // A register destination is neither a slot nor an address.
        let regs = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL));
        let ops = [op(OpCode::CPUI_BRANCHIND, None, vec![vn(&regs, 0x10)])];
        assert!(refs(&ram, &ops).is_empty());
    }

    /// A `LOAD` through a constant pointer is a read of that address; the
    /// space-id `in0` (a huge constant) must never be mistaken for one.
    #[test]
    fn load_through_a_constant_pointer_is_a_read() {
        let (ram, cst) = spaces();
        let ops = [op(OpCode::CPUI_LOAD, None, vec![vn(&cst, 0x1b), vn(&cst, 0x4010)])];
        assert_eq!(refs(&ram, &ops), vec![(0x4010, XrefKind::Read)]);
    }

    /// A `STORE` through a constant pointer is a write of the pointer, and the
    /// stored value is judged on its own merits.
    #[test]
    fn store_reports_the_pointer_it_writes_through() {
        let (ram, cst) = spaces();
        let ops = [op(
            OpCode::CPUI_STORE,
            None,
            vec![vn(&cst, 0x1b), vn(&cst, 0x4010), vn(&cst, 0x1900)],
        )];
        assert_eq!(
            refs(&ram, &ops),
            vec![(0x1900, XrefKind::Data), (0x4010, XrefKind::Write)]
        );
    }

    /// Every architecture materializes a call's return address as this
    /// instruction's fall-through — x86 stores it, ARM copies it to `LR`. It is
    /// never a reference, so it must not survive either spelling.
    #[test]
    fn a_calls_own_return_address_is_never_a_data_reference() {
        let (ram, cst) = spaces();
        let arm = [op(OpCode::CPUI_COPY, None, vec![vn(&cst, 0x1104)])];
        let x86 = [op(
            OpCode::CPUI_STORE,
            None,
            vec![vn(&cst, 0x1b), vn(&ram, 0x4010), vn(&cst, 0x1104)],
        )];
        assert!(data_refs(&arm, Some(&ram), &MAPPED, 0x1104, None).is_empty());
        assert_eq!(
            data_refs(&x86, Some(&ram), &MAPPED, 0x1104, None),
            vec![(0x4010, XrefKind::Read)]
        );
    }

    /// (kuna) `poolref`: a pointer-sized read of a read-only word files the
    /// address that word holds, from the same instruction. The ARM shape — the
    /// `LOAD`'s address is a direct `ram` varnode and its OUTPUT carries the
    /// access width.
    #[test]
    fn a_literal_pool_load_also_references_what_the_pool_word_holds() {
        let (ram, cst) = spaces();
        let pool = pool_image();
        let mut out = vn(&ram, 0x4020);
        out.size = 4;
        let load = op(
            OpCode::CPUI_LOAD,
            Some(out),
            vec![vn(&cst, 0x1b), vn(&ram, 0x1000)],
        );
        assert_eq!(
            data_refs(&[load], Some(&ram), &MAPPED, 0, Some(&pool)),
            vec![(0x1000, XrefKind::Read), (0x4000, XrefKind::Data), (0x4020, XrefKind::Write)]
        );
    }

    /// `ldrh r0,[pool]` reads a number out of the pool, not a pointer — and the
    /// `LOAD`'s address varnode is pointer-sized either way, so the width has to
    /// come from the output.
    #[test]
    fn a_narrow_load_of_a_pool_word_files_only_the_read() {
        let (ram, cst) = spaces();
        let pool = pool_image();
        let mut out = vn(&ram, 0x4020);
        out.size = 2;
        let load = op(
            OpCode::CPUI_LOAD,
            Some(out),
            vec![vn(&cst, 0x1b), vn(&ram, 0x1000)],
        );
        assert_eq!(
            data_refs(&[load], Some(&ram), &MAPPED, 0, Some(&pool)),
            vec![(0x1000, XrefKind::Read), (0x4020, XrefKind::Write)]
        );
    }

    /// The x86 shape: the operand is a direct `ram` varnode input of a `COPY`,
    /// which carries its own access width.
    #[test]
    fn a_direct_ram_read_of_a_pool_word_follows_it_too() {
        let (ram, _cst) = spaces();
        let pool = pool_image();
        let mut src = vn(&ram, 0x1008);
        src.size = 4;
        let copy = op(OpCode::CPUI_COPY, None, vec![src]);
        assert_eq!(
            data_refs(&[copy], Some(&ram), &MAPPED, 0, Some(&pool)),
            vec![(0x1008, XrefKind::Read), (0x4010, XrefKind::Data)]
        );
    }

    /// Without the read-only image nothing is followed — the pre-feature answer.
    #[test]
    fn no_pool_image_leaves_every_reference_as_it_was() {
        let (ram, _cst) = spaces();
        let mut src = vn(&ram, 0x1008);
        src.size = 4;
        let copy = op(OpCode::CPUI_COPY, None, vec![src]);
        assert_eq!(refs(&ram, &[copy]), vec![(0x1008, XrefKind::Read)]);
    }

    /// The `LEA` shape: a materialized address is address-taken data, even when
    /// it points into code — that is what makes an indirect-call target findable.
    #[test]
    fn a_materialized_code_address_is_a_data_reference() {
        let (ram, cst) = spaces();
        let ops = [op(OpCode::CPUI_COPY, None, vec![vn(&cst, 0x13c9)])];
        assert_eq!(refs(&ram, &ops), vec![(0x13c9, XrefKind::Data)]);
    }

    /// The upstream `checkOperands` filter: small integers and byte masks are
    /// numbers, not addresses, however well they land in a mapped range — and an
    /// unmapped value is rejected even though it clears the filter.
    #[test]
    fn small_integers_byte_masks_and_unmapped_values_are_not_addresses() {
        let (ram, cst) = spaces();
        for value in [0x8, 0xff, 0xffff, 0xfff, 0x9000] {
            let ops = [op(OpCode::CPUI_COPY, None, vec![vn(&cst, value)])];
            assert!(refs(&ram, &ops).is_empty(), "{value:#x} accepted as an address");
        }
    }

    /// A register/temp input lives in neither space and is never an address.
    #[test]
    fn varnodes_outside_the_data_and_constant_spaces_are_ignored() {
        let (ram, _cst) = spaces();
        let regs = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL));
        let ops = [op(OpCode::CPUI_INT_ADD, Some(vn(&regs, 0x10)), vec![vn(&regs, 0x1040)])];
        assert!(refs(&ram, &ops).is_empty());
    }

    /// The PE/ELF import shape, hand-built: a veneer at `0x1030` jumping through
    /// the slot at `0x4008`, one call site on the veneer and one slot read
    /// somewhere else. Both addresses must answer with both references, and
    /// neither may report the veneer's own forwarding jump as a caller.
    fn import_index() -> XrefIndex {
        let mk = |from, to, kind| Xref { from, to, kind, instruction: String::new() };
        let edges = [
            mk(0x1102, 0x1030, XrefKind::Call), // a direct call to the veneer
            mk(0x1030, 0x4008, XrefKind::Read), // the veneer's own jmp [slot]
            mk(0x1200, 0x4008, XrefKind::Read), // a call straight through the slot
        ];
        let mut st = State {
            by_target: BTreeMap::new(),
            by_source: BTreeMap::new(),
            decoded: HashSet::from([0x1030, 0x1102, 0x1200]),
            funcs: BTreeSet::from([0x1000, 0x1030, 0x1180]),
            indirect_call_sites: BTreeSet::new(),
        };
        for e in edges {
            st.file(e.from, e.to, e.kind, "");
        }
        // The veneer is the single 6-byte `jmp [0x4008]` at 0x1030.
        st.finish(BTreeMap::from([(0x1030, Veneer { slot: 0x4008, end: 0x1036 })]))
    }

    /// The alias class is the veneer plus its slot, and it is symmetric: asking
    /// either address is asking the same question.
    #[test]
    fn a_veneer_and_its_slot_are_one_alias_class() {
        let index = import_index();
        let class = BTreeSet::from([0x1030, 0x4008]);
        assert_eq!(index.alias_class(0x1030), class);
        assert_eq!(index.alias_class(0x4008), class);
        assert_eq!(index.veneer_slot(0x1030), Some(0x4008));
        // Everything that is not an import is a class of one.
        assert_eq!(index.alias_class(0x1000), BTreeSet::from([0x1000]));
        assert!(index.veneer_slot(0x1000).is_none());
    }

    /// Both members answer with both real references, and the forwarding jump —
    /// the edge that defines the class — is never one of them.
    #[test]
    fn both_ends_of_an_import_answer_with_every_real_reference() {
        let index = import_index();
        for at in [0x1030u64, 0x4008] {
            let rows = index.refs_to_unified(at);
            let got: Vec<(u64, u64)> = rows.iter().map(|r| (r.from, r.to)).collect();
            assert_eq!(
                got,
                vec![(0x1102, 0x1030), (0x1200, 0x4008)],
                "asking 0x{at:x} gave the wrong references"
            );
        }
        // The per-address buckets are untouched: `refs_to` still answers for the
        // literal address, which is what `strings` and the call graph read.
        assert_eq!(index.refs_to(0x1030).len(), 1);
        assert_eq!(index.refs_to(0x4008).len(), 2);
    }

    /// A computed call belongs to the function that CONTAINS it, which is the
    /// rule `refs_from_function` buckets by. The walk can reach an instruction
    /// while descending from a different entry — a fall-through into a body some
    /// later CALL names as its own function — and attributing the call site to
    /// that descent would make the two answers name different functions.
    #[test]
    fn a_computed_call_is_attributed_to_the_function_containing_it() {
        let mut st = State {
            by_target: BTreeMap::new(),
            by_source: BTreeMap::new(),
            decoded: HashSet::from([0x1188]),
            funcs: BTreeSet::from([0x1000, 0x1030, 0x1180]),
            indirect_call_sites: BTreeSet::from([0x1188]),
        };
        st.file(0x1188, 0x4008, XrefKind::Read, "");
        let index = st.finish(BTreeMap::new());
        assert!(index.has_indirect_calls(0x1180), "the call site lost its own function");
        assert!(!index.has_indirect_calls(0x1030), "attributed to the preceding entry");
        // And it lands in the same bucket the instruction's references do.
        assert_eq!(index.refs_from_function(0x1180).len(), 1);
    }

    /// Off an alias class, unifying is exactly `refs_to`.
    #[test]
    fn a_plain_target_is_unaffected_by_unification() {
        let index = import_index();
        assert_eq!(index.refs_to_unified(0x1102).len(), 0);
        let direct: Vec<_> = index.refs_to(0x1030).iter().collect();
        let mut plain = import_index();
        plain.veneers.clear();
        plain.veneers_of_slot.clear();
        assert_eq!(plain.refs_to_unified(0x1030).len(), direct.len());
    }

    /// A section-less image is classified against its program header. With no
    /// section table [`mapped_ranges`] would otherwise answer "nothing is
    /// mapped", and every data operand in the image — the `LEA RDI,[0x2000]`
    /// this fixture is built around — would be discarded while its control flow
    /// survived.
    #[test]
    fn mapped_ranges_falls_back_to_load_segments_without_a_section_table() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sectionless_x86_64");
        let data = std::fs::read(path).unwrap();
        let file = object::File::parse(&*data).unwrap();
        assert!(file.sections().next().is_none(), "fixture grew a section table");
        let mapped = mapped_ranges(&file);
        assert_eq!(mapped, vec![(0x1000, 0x100d), (0x2000, 0x200e)]);
        assert!(in_range(&mapped, 0x2000), "the string the LEA forms reads as unmapped");
    }

    /// The fallback is the no-section-table arm only: an image that has sections
    /// is still classified against them, so the segments' coarser spans (a
    /// `PT_LOAD` also covers inter-section padding and the ELF header) never
    /// widen the oracle on an ordinary linked image.
    #[test]
    fn mapped_ranges_answers_from_the_sections_when_there_are_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let data = std::fs::read(path).unwrap();
        let file = object::File::parse(&*data).unwrap();
        let mapped = mapped_ranges(&file);
        assert!(!mapped.is_empty());
        let sections: Vec<(u64, u64)> =
            file.sections().map(|s| (s.address(), s.address() + s.size())).collect();
        for r in &mapped {
            assert!(sections.contains(r), "{r:x?} is not any section's span");
        }
        assert!(!in_range(&mapped, 0), "the ELF header read as mapped data");
    }

    /// `sort_dedup` collapses the `(from, to, kind)` triple: one row per site.
    #[test]
    fn duplicate_edges_from_one_site_collapse() {
        let mk = |from, to, kind| Xref { from, to, kind, instruction: String::new() };
        let mut rows = vec![
            mk(0x1102, 0x1030, XrefKind::Call),
            mk(0x1102, 0x1030, XrefKind::Call),
            mk(0x1010, 0x1030, XrefKind::Jump),
        ];
        sort_dedup(&mut rows, true);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].from, 0x1010);
        assert_eq!(rows[1].from, 0x1102);
    }
}
