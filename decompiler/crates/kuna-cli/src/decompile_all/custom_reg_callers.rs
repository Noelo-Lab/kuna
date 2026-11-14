//! (kuna `calleereginputs custom`) The whole-image caller side of the custom
//! register-argument rule, and the decision that joins it with the callee side.
//!
//! A callee-saved register (EBX, ESI, EDI) is a custom argument of a direct call
//! only when BOTH halves hold: the callee's own body genuinely reads the
//! register's entry value (the post-heritage entry reads, filed per function in
//! the first callee-first pass as [`Architecture::kuna_custom_reg_reads`]), AND
//! every direct call site in the whole image sets the register in the call's own
//! basic block before the call (so the write dominates the call and a value
//! merely left in the register by an earlier call does not count).
//!
//! Before the pass, while the call graph is alive, [`analyze`] lists each
//! eligible callee's direct call sites and the functions they sit in. A callee
//! whose complete caller set the graph cannot list is not eligible: its address
//! is taken in code or stored as a pointer word in the image, it is exported or
//! the entry point, a call to it comes from no known function, a branch reaches
//! it, or it is an interior chunk inside another function's extent. After the
//! pass [`decide`] takes only the callees that read a custom register, drops a
//! stack-moving CRT helper (`_chkstk`/`__SEH_prolog`) and a callee that also
//! reads ECX or EDX, and decodes just those callees' callers to see whether
//! every site sets every register the callee reads. [`Decision::install`] primes
//! the per-callee models the engine applies
//! ([`Architecture::kuna_custom_reg_callers`]).

use std::collections::{BTreeMap, BTreeSet};

use kuna_analysis::loader::macho_fat::SlicePref;
use kuna_base::address::Address;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::kuna_calleereginputs::CUSTOM;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;
use object::{Object, ObjectSection};

use crate::callgraph::CallGraph;
use crate::image::image_bytes;

/// Instructions decoded per caller body before the scan gives up on it.
const BUDGET: usize = 1 << 16;

/// Each eligible callee's direct call sites, taken off the call graph before the
/// first pass.
pub(crate) struct CustomRegPlan {
    /// Per eligible callee entry: `(call instruction, owning function entry)`.
    sites: BTreeMap<u64, Vec<(u64, u64)>>,
    /// Every inventory function as `(entry, extent)`, ascending by entry.
    entries: Vec<(u64, u64)>,
}

impl CustomRegPlan {
    pub(crate) fn empty() -> Self {
        CustomRegPlan { sites: BTreeMap::new(), entries: Vec::new() }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sites.is_empty()
    }
}

/// The decided callees with their input-register keys, and the functions that
/// call each.
pub(crate) struct Decision {
    keys: BTreeMap<u64, u16>,
    callers: BTreeMap<u64, BTreeSet<u64>>,
}

impl Decision {
    pub(crate) fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Each decided callee with the functions whose output the decision changes:
    /// itself and every function that calls it.
    pub(crate) fn groups(&self) -> impl Iterator<Item = (u64, BTreeSet<u64>)> + '_ {
        self.keys.keys().map(|&callee| {
            let mut members = self.callers.get(&callee).cloned().unwrap_or_default();
            members.insert(callee);
            (callee, members)
        })
    }

    /// Build the synthesized model of each key the `kept` callees take and prime
    /// both maps on the architecture; a callee left out states nothing, on either
    /// side.
    pub(crate) fn install(&self, prog: &mut ConsoleProgram, kept: &BTreeSet<u64>) {
        use kuna_decomp::kuna_calleereginputs::key_registers;
        let mut models: BTreeMap<u16, std::rc::Rc<kuna_decomp::fspec::ProtoModel>> = BTreeMap::new();
        let mut map: BTreeMap<u64, u16> = BTreeMap::new();
        for (&callee, &key) in self.keys.iter().filter(|(c, _)| kept.contains(c)) {
            if let std::collections::btree_map::Entry::Vacant(slot) = models.entry(key) {
                let (regs, thisptr) = key_registers(key);
                let Some(model) = prog.arch().kuna_build_reginput_model(&format!("__kuna_reg_{key:x}"), &regs, thisptr)
                else {
                    continue;
                };
                slot.insert(std::rc::Rc::new(model));
            }
            map.insert(callee, key);
        }
        prog.arch_mut().kuna_custom_reg_callers = std::rc::Rc::new(map.into_iter().collect());
        prog.arch_mut().kuna_custom_reg_models = std::rc::Rc::new(models.into_iter().collect());
    }
}

#[derive(Default)]
struct Collect(Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>);

impl PcodeEmit for Collect {
    fn dump(&mut self, _at: &Address, opcode: OpCode, out: Option<&VarnodeData>, inputs: &[VarnodeData]) {
        self.0.push((opcode, out.cloned(), inputs.to_vec()));
    }
}

/// The custom registers as `(register-space offset, size, mask-bit)`.
fn custom_regs(prog: &ConsoleProgram) -> Vec<(u64, u32, u8)> {
    let t = prog.arch().translate();
    CUSTOM
        .iter()
        .filter_map(|&(name, bit)| t.get_register_data(name).ok().map(|v| (v.offset, v.size, bit)))
        .collect()
}

/// List every eligible callee's direct call sites off the call graph. Decodes
/// nothing; empty unless the `custom` extension is on for a 32-bit Windows image.
pub(crate) fn analyze(prog: &ConsoleProgram, graph: &CallGraph, binary: &str, pref: SlicePref) -> CustomRegPlan {
    if !kuna_decomp::kuna_calleereginputs::custom_applies(prog.arch()) {
        return CustomRegPlan::empty();
    }
    let entries: Vec<(u64, u64)> = graph.entry_extents().to_vec();
    let entry_set: BTreeSet<u64> = entries.iter().map(|(e, _)| *e).collect();
    let taken = address_taken(binary, pref, &entry_set);
    let mut reach = 0u64;
    let mut sites = BTreeMap::new();
    for &(entry, extent) in &entries {
        let interior = entry < reach;
        reach = reach.max(entry.saturating_add(extent));
        if interior || taken.contains(&entry) {
            continue;
        }
        match graph.reg_call_sites(entry) {
            Some(s) if !s.is_empty() => {
                sites.insert(entry, s);
            }
            _ => {}
        }
    }
    CustomRegPlan { sites, entries }
}

/// Join the callee half the first pass recorded with the caller half, decoding
/// only the callers of callees that read a custom register.
pub(crate) fn decide(prog: &ConsoleProgram, plan: &CustomRegPlan) -> Decision {
    use kuna_decomp::kuna_calleereginputs::{ecx_edx_key, KEY_CUSTOM_SHIFT, READS_ECX_EDX};
    let mut decision = Decision { keys: BTreeMap::new(), callers: BTreeMap::new() };
    let (Some(code), Some(space)) = (
        prog.arch().manage().get_default_code_space().cloned(),
        prog.arch().manage().get_space_by_name("register").cloned(),
    ) else {
        return decision;
    };
    let regs = custom_regs(prog);
    let mut reads: Vec<(u64, u8)> = prog
        .arch()
        .kuna_custom_reg_reads
        .iter()
        .filter(|(e, m)| *m & READS_ECX_EDX == 0 && plan.sites.contains_key(e))
        .map(|(&e, &m)| (e, m))
        .collect();
    reads.sort_unstable();
    let mut scanned: BTreeMap<u64, BTreeMap<u64, u8>> = BTreeMap::new();
    for (entry, callee_mask) in reads {
        let sites = &plan.sites[&entry];
        let printed = prog.arch().kuna_custom_reg_calls.get(&entry);
        if printed.is_some_and(|calls| {
            calls.iter().any(|&(caller, site)| caller != entry && !sites.contains(&(site, caller)))
        }) {
            continue;
        }
        let addr = Address::new(code.clone(), entry);
        if kuna_decomp::kuna_calleepurge::moves_stack(prog.arch(), &addr) || ecx_edx_key(prog.arch(), &addr) != 0 {
            continue;
        }
        let mut callers = BTreeSet::new();
        let all_set = sites.iter().all(|&(site, owner)| {
            callers.insert(owner);
            let masks = scanned.entry(owner).or_insert_with(|| {
                let extent = plan.entries.binary_search_by_key(&owner, |(e, _)| *e).map_or(0, |i| plan.entries[i].1);
                scan_caller(prog, &code, space.get_index(), &regs, owner, extent)
            });
            masks.get(&site).is_some_and(|m| m & callee_mask == callee_mask)
        });
        if all_set {
            decision.keys.insert(entry, (callee_mask as u16) << KEY_CUSTOM_SHIFT);
            decision.callers.insert(entry, callers);
        }
    }
    decision
}

/// The callee entries whose address is stored as a 4-byte pointer word in the
/// image (any word of a data section, an aligned word of a code section),
/// exported, or the image entry point: callees reachable other than by the
/// direct calls the graph lists, so their convention is fixed.
fn address_taken(binary: &str, pref: SlicePref, entries: &BTreeSet<u64>) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    let Ok(bytes) = image_bytes(binary, pref) else { return out };
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(&bytes) else { return out };
    let (lo, hi) = match (entries.iter().min(), entries.iter().max()) {
        (Some(&lo), Some(&hi)) => (lo, hi),
        _ => return out,
    };
    for section in file.sections() {
        let step = match section.kind() {
            object::SectionKind::UninitializedData => continue,
            object::SectionKind::Text => 4,
            _ => 1,
        };
        let Ok(data) = section.data() else { continue };
        for w in data.windows(4).step_by(step) {
            let v = u32::from_le_bytes([w[0], w[1], w[2], w[3]]) as u64;
            if (lo..=hi).contains(&v) && entries.contains(&v) {
                out.insert(v);
            }
        }
    }
    for export in file.exports().unwrap_or_default() {
        out.insert(export.address());
    }
    out.insert(file.entry());
    out
}

/// Decode one caller body by recursive descent from its entry, inside its
/// extent, and return for each direct call the custom registers written earlier
/// in the call's own basic block: a block starts at the entry, at a branch
/// target, after a branch or call, and wherever the decoded instructions are not
/// contiguous.
fn scan_caller(
    prog: &ConsoleProgram,
    code: &std::rc::Rc<kuna_base::space::AddrSpace>,
    reg_idx: i32,
    regs: &[(u64, u32, u8)],
    entry: u64,
    extent: u64,
) -> BTreeMap<u64, u8> {
    struct Insn {
        len: u64,
        writes: u8,
        call: bool,
    }
    let translate = prog.arch().translate();
    let end = entry.wrapping_add(extent);
    let in_body = |t: u64| t >= entry && t < end;
    let mut insns: BTreeMap<u64, Insn> = BTreeMap::new();
    let mut leaders: BTreeSet<u64> = BTreeSet::from([entry]);
    let mut work = vec![entry];
    let mut collect = Collect::default();
    'paths: while let Some(mut at) = work.pop() {
        while in_body(at) && !insns.contains_key(&at) {
            if insns.len() >= BUDGET {
                break 'paths;
            }
            collect.0.clear();
            let Ok(len) = translate.one_instruction(&mut collect, &Address::new(code.clone(), at)) else { break };
            let len = len.max(1) as u64;
            let mut insn = Insn { len, writes: 0, call: false };
            let mut ends = false;
            let mut falls = true;
            for (opc, outv, ins) in &collect.0 {
                if let Some(o) = outv.as_ref().filter(|o| o.space.as_ref().is_some_and(|s| s.get_index() == reg_idx)) {
                    for &(off, size, bit) in regs {
                        if o.offset.max(off) < (o.offset + o.size as u64).min(off + size as u64) {
                            insn.writes |= bit;
                        }
                    }
                }
                let target = ins
                    .first()
                    .filter(|d| d.space.as_ref().map(|s| s.get_index()) == Some(code.get_index()))
                    .map(|d| d.offset);
                match opc {
                    OpCode::CPUI_CALL => {
                        ends = true;
                        insn.call |= target.is_some();
                    }
                    OpCode::CPUI_CALLIND => ends = true,
                    OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => {
                        ends = true;
                        falls = false;
                    }
                    OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH if target.is_some() => {
                        ends = true;
                        falls &= *opc == OpCode::CPUI_CBRANCH;
                        let t = target.unwrap_or_default();
                        if in_body(t) {
                            leaders.insert(t);
                            work.push(t);
                        }
                    }
                    _ => {}
                }
            }
            insns.insert(at, insn);
            let next = at.wrapping_add(len);
            if !falls {
                break;
            }
            if ends {
                leaders.insert(next);
            }
            at = next;
        }
    }
    let mut out = BTreeMap::new();
    let mut written = 0u8;
    let mut expected = None;
    for (&at, insn) in &insns {
        if leaders.contains(&at) || expected != Some(at) {
            written = 0;
        }
        if insn.call {
            out.insert(at, written);
        }
        written |= insn.writes;
        expected = Some(at.wrapping_add(insn.len));
    }
    out
}
