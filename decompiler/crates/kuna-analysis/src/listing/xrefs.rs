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

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{AssemblyEmit, PcodeEmit, Translate};
use object::read::{Object, ObjectSection};
use object::SectionKind;

use super::classify::classify;
use super::context::ContextPainter;
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
    /// Every instruction VMA the walk decoded.
    decoded: BTreeSet<u64>,
    /// Every function entry the walk seeded or discovered, in address order.
    funcs: BTreeSet<u64>,
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
}

/// One emitted p-code op, whole: the opcode, the output varnode, and every
/// input. [`super::decode::decode_one`] keeps only `in0` (all the flow
/// classifier needs); the parts it drops are what the data-reference scan is
/// made of — the output says a memory location was written, the later inputs
/// carry the addresses.
struct FullOp {
    opcode: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// A capturing [`PcodeEmit`] that keeps every emitted op whole.
#[derive(Default)]
struct FullCapture {
    ops: Vec<FullOp>,
}

impl PcodeEmit for FullCapture {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push(FullOp { opcode: opc, out: outvar.cloned(), ins: vars.to_vec() });
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

    let mut st = State {
        by_target: BTreeMap::new(),
        by_source: BTreeMap::new(),
        decoded: BTreeSet::new(),
        funcs: seed_set.clone(),
    };

    let mut func_queue: VecDeque<u64> = seed_set.iter().copied().collect();
    let mut walked: BTreeSet<u64> = BTreeSet::new();

    while let Some(entry) = func_queue.pop_front() {
        if !walked.insert(entry) {
            continue;
        }
        let mut insn_queue: VecDeque<u64> = VecDeque::from([entry]);
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
            let Some(decoded) = decode(translate, vma, &code_space) else {
                continue; // undecodable: stop this path
            };
            if decoded.len == 0 {
                continue; // a zero-length decode would not advance
            }
            st.decoded.insert(vma);

            let raw: Vec<RawOp> = decoded
                .ops
                .iter()
                .map(|op| RawOp { opcode: op.opcode, in0: op.ins.first().cloned() })
                .collect();
            let c = classify(&raw, vma, decoded.len);

            for &target in &c.flows {
                let kind = if c.flow.is_call { XrefKind::Call } else { XrefKind::Jump };
                st.file(vma, target, kind, &decoded.text);
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

            if !mapped.is_empty() {
                let fall_through = vma.wrapping_add(decoded.len as u64);
                for (to, kind) in
                    data_refs(&decoded.ops, data_space.as_ref(), &mapped, fall_through)
                {
                    st.file(vma, to, kind, &decoded.text);
                }
            }
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
    for _ in 0..MAX_VENEER_INSNS {
        let decoded = decode(translate, vma, code_space)?;
        if decoded.len == 0 {
            return None;
        }
        let raw: Vec<RawOp> = decoded
            .ops
            .iter()
            .map(|op| RawOp { opcode: op.opcode, in0: op.ins.first().cloned() })
            .collect();
        let c = classify(&raw, vma, decoded.len);
        if !c.flows.is_empty() || c.flow.is_call || c.flow.kind == FlowKind::Return {
            return None;
        }
        if let Some(op) = decoded.ops.iter().find(|o| o.opcode == OpCode::CPUI_BRANCHIND) {
            let vn = op.ins.first()?;
            let in_data = matches!((&vn.space, data_space), (Some(s), Some(d)) if Rc::ptr_eq(s, d));
            let end = vma.wrapping_add(u64::from(decoded.len));
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
    decoded: BTreeSet<u64>,
    funcs: BTreeSet<u64>,
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
    /// lands in can never disagree.
    fn finish(mut self, veneers: BTreeMap<u64, Veneer>) -> XrefIndex {
        let mut by_source_function: BTreeMap<u64, Vec<Xref>> = BTreeMap::new();
        for (&from, refs) in &self.by_source {
            let Some(&entry) = self.funcs.range(..=from).next_back() else {
                continue;
            };
            by_source_function.entry(entry).or_default().extend(refs.iter().cloned());
        }
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
        decoded: BTreeSet::new(),
        funcs: BTreeSet::new(),
        veneers: BTreeMap::new(),
        veneers_of_slot: BTreeMap::new(),
        insns: 0,
    }
}

/// One decoded instruction: its byte length, its full p-code, and its rendering.
struct Decoded {
    len: u32,
    ops: Vec<FullOp>,
    text: String,
}

/// Decode the instruction at `vma`, keeping every input varnode.
///
/// The assembly render is best-effort (the p-code is the load-bearing half), and
/// a translator panic on exotic bytes is contained to `None` — a query surface
/// must never take the process down over one bad address.
fn decode(translate: &dyn Translate, vma: u64, code_space: &Rc<AddrSpace>) -> Option<Decoded> {
    let addr = Address::new(Rc::clone(code_space), vma);
    let mut cap = FullCapture::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        translate.one_instruction(&mut cap, &addr)
    }));
    let len = match decoded {
        Ok(Ok(len)) if len > 0 => len as u32,
        _ => return None,
    };
    let mut asm = AsmCapture::default();
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = translate.print_assembly(&mut asm, &addr);
    }));
    Some(Decoded { len, ops: cap.ops, text: asm.text })
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
) -> Vec<(u64, XrefKind)> {
    let mut out = Vec::new();
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
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// `ScalarOperandAnalyzer.checkOperands`' value filter.
fn looks_like_address(value: u64) -> bool {
    value >= MIN_ADDRESS_VALUE && !MASK_VALUES.contains(&value)
}

/// The `[lo, hi)` ranges an address must land in to be a data reference: every
/// section the image maps at runtime, code included (an immediate that
/// materializes a function entry is the address-taken case, and is exactly what
/// makes an indirect-call target findable).
fn mapped_ranges(file: &object::File) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
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
fn in_range(ranges: &[(u64, u64)], vma: u64) -> bool {
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

    /// Run [`data_refs`] with `ram` as the default data space and a fall-through
    /// no op under test materializes.
    fn refs(ram: &Rc<AddrSpace>, ops: &[FullOp]) -> Vec<(u64, XrefKind)> {
        data_refs(ops, Some(ram), &MAPPED, 0)
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
        assert!(data_refs(&arm, Some(&ram), &MAPPED, 0x1104).is_empty());
        assert_eq!(
            data_refs(&x86, Some(&ram), &MAPPED, 0x1104),
            vec![(0x4010, XrefKind::Read)]
        );
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
            decoded: BTreeSet::from([0x1030, 0x1102, 0x1200]),
            funcs: BTreeSet::from([0x1000, 0x1030, 0x1180]),
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
