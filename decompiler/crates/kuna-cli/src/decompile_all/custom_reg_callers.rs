//! (kuna `calleereginputs custom`) The whole-image caller side of the custom
//! register-argument rule, and the decision that joins it with the callee side.
//!
//! A callee-saved register (EBX, ESI, EDI) is a custom argument of a direct call
//! only when BOTH halves hold: the callee's own body genuinely reads the
//! register's entry value (the post-heritage read `passthrough` records and
//! `protoorder` files per callee as [`RecoveredTypes::reads`]), AND every direct
//! call site in the whole image sets the register on the path to the call.
//!
//! The caller half is computed once, before any function is decompiled
//! ([`analyze`]), over the call graph. The callee half is read after the first
//! callee-first pass ([`decide`]), when every callee's recorded reads are known,
//! and the two are intersected into the per-callee model the engine applies
//! ([`Architecture::kuna_custom_reg_callers`]); the functions whose output the
//! decision changes — the callees and their callers — are decompiled again.
//!
//! "Sets the register on the path to the call" is read conservatively: the
//! register must be written in the call's own basic block before the call (so
//! the write dominates the call and a value merely left in the register by an
//! earlier call does not count). A callee whose complete caller set the call
//! graph cannot list (its address is taken in code or stored as a pointer in
//! data, it is exported, or a call comes from no known function), an import, an
//! interior code chunk inside another function's extent, or a stack-moving CRT
//! helper (`_chkstk`/`__SEH_prolog`/…) states nothing. A register one call site
//! fails to set is disproved for every site.

use std::collections::{BTreeSet, HashMap, HashSet};

use kuna_analysis::loader::macho_fat::SlicePref;
use kuna_base::address::Address;
use kuna_console::engine::ConsoleProgram;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;
use object::{Object, ObjectSection};

use crate::callgraph::CallGraph;
use crate::image::image_bytes;

/// Instructions decoded per caller body before the scan gives up on it.
const BUDGET: usize = 1 << 16;

/// The callee-saved registers the rule recovers, `(name, mask-bit)`; the bit
/// matches [`kuna_decomp::kuna_calleereginputs::CUSTOM`].
const CUSTOM: [(&str, u8); 3] = [("EBX", 1), ("ESI", 2), ("EDI", 4)];

/// The caller-side verdict and call-graph edges the decision needs after the
/// first pass.
pub(crate) struct CustomRegPlan {
    /// Per eligible callee entry: the callee-saved registers every direct call
    /// site sets on the path to the call.
    caller_side: HashMap<u64, u8>,
    /// Per eligible callee entry: the entries of the functions that call it.
    callers_of: HashMap<u64, BTreeSet<u64>>,
}

impl CustomRegPlan {
    pub(crate) fn empty() -> Self {
        CustomRegPlan { caller_side: HashMap::new(), callers_of: HashMap::new() }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.caller_side.is_empty()
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

/// The caller-side analysis: which callee-saved registers every direct call site
/// of each eligible callee sets. Decodes every function body once, before the
/// first decompile. Empty unless the `custom` extension is on for a 32-bit
/// Windows image.
pub(crate) fn analyze(
    prog: &ConsoleProgram,
    graph: &CallGraph,
    binary: &str,
    pref: SlicePref,
) -> CustomRegPlan {
    let empty = CustomRegPlan { caller_side: HashMap::new(), callers_of: HashMap::new() };
    if !kuna_decomp::kuna_calleereginputs::custom_applies(prog.arch()) {
        return empty;
    }
    let Some(space) = prog.arch().manage().get_space_by_name("register").cloned() else { return empty };
    let regs = custom_regs(prog);
    if regs.is_empty() {
        return empty;
    }
    let reg_idx = space.get_index();
    let Some(code) = prog.arch().manage().get_default_code_space().cloned() else { return empty };

    let entries: Vec<(u64, u64)> = graph.entry_extents().to_vec();
    let entry_set: HashSet<u64> = entries.iter().map(|(e, _)| *e).collect();
    let taken = address_taken(binary, pref, &entry_set);

    // The custom registers set in each direct call's own basic block before the
    // call, and the caller the call sits in, by call-instruction address.
    let mut site_set: HashMap<u64, (u8, u64)> = HashMap::new();
    for &(entry, extent) in &entries {
        if extent == 0 {
            continue;
        }
        scan_caller(prog, &code, reg_idx, &regs, entry, extent, &mut site_set);
    }

    let mut caller_side: HashMap<u64, u8> = HashMap::new();
    let mut callers_of: HashMap<u64, BTreeSet<u64>> = HashMap::new();
    for &(entry, _) in &entries {
        if taken.contains(&entry) || excluded(prog, &entries, entry) {
            continue;
        }
        let Some(sites) = graph.reg_call_sites(entry) else { continue };
        if sites.is_empty() {
            continue;
        }
        let mut mask = 0xffu8;
        let mut callers = BTreeSet::new();
        let mut ok = true;
        for s in &sites {
            match site_set.get(s) {
                Some(&(m, caller)) => {
                    mask &= m;
                    callers.insert(caller);
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && mask != 0 {
            caller_side.insert(entry, mask);
            callers_of.insert(entry, callers);
        }
    }
    CustomRegPlan { caller_side, callers_of }
}

/// Join the caller side with the callee side (the reads `protoorder` recorded in
/// the first pass), build the synthesized models, prime them on the
/// architecture, and return the entries whose output changed — the decided
/// callees and all their callers — to decompile again.
pub(crate) fn decide(prog: &mut ConsoleProgram, plan: &CustomRegPlan) -> BTreeSet<u64> {
    use kuna_decomp::kuna_calleereginputs::{ecx_edx_key, key_registers, KEY_CUSTOM_SHIFT};
    let Some(code) = prog.arch().manage().get_default_code_space().cloned() else {
        return BTreeSet::new();
    };

    let mut map: HashMap<u64, u16> = HashMap::new();
    let mut affected: BTreeSet<u64> = BTreeSet::new();
    let mut keys: Vec<&u64> = plan.caller_side.keys().collect();
    keys.sort_unstable();
    for &entry in keys {
        let caller_mask = plan.caller_side[&entry];
        let addr = Address::new(code.clone(), entry);
        // The callee half: the callee-saved registers the body read on entry,
        // from the reads `protoorder` recorded in the first pass.
        // The callee half: the callee-saved registers the body read on entry,
        // recorded in the first pass from the post-heritage entry reads.
        let callee_mask = prog.arch().kuna_custom_reg_reads.get(&entry).copied().unwrap_or(0);
        let decided = caller_mask & callee_mask;
        // Decide a callee only when EVERY callee-saved register it reads is set
        // by every call site: a partial model (one register of two) leaves the
        // other a phantom read and misaligns the recovery, which can drop a real
        // stack argument. All-or-nothing keeps the change sound.
        if decided == 0 || decided != callee_mask {
            continue;
        }
        // A callee that also reads ECX/EDX is left to the normal
        // `__thiscall`/`__fastcall` resolution (#1575): forcing a synthesized
        // model over its own flexible evaluation mis-handles a /GS or fastcall
        // frame and can drop a stack argument from one of its calls. Only a
        // callee whose register inputs are purely the callee-saved customs plus
        // the stack takes a custom model.
        if ecx_edx_key(prog.arch(), &addr) != 0 {
            continue;
        }
        let key = (decided as u16) << KEY_CUSTOM_SHIFT;
        map.insert(entry, key);
        affected.insert(entry);
        if let Some(callers) = plan.callers_of.get(&entry) {
            affected.extend(callers.iter().copied());
        }
    }
    if map.is_empty() {
        return BTreeSet::new();
    }
    let mut models: HashMap<u16, std::rc::Rc<kuna_decomp::fspec::ProtoModel>> = HashMap::new();
    for &key in map.values() {
        if models.contains_key(&key) {
            continue;
        }
        let (regs, thisptr) = key_registers(key);
        let name = format!("__kuna_reg_{key:x}");
        if let Some(model) = prog.arch().kuna_build_reginput_model(&name, &regs, thisptr) {
            models.insert(key, std::rc::Rc::new(model));
        }
    }
    map.retain(|_, key| models.contains_key(key));
    affected.retain(|e| map.contains_key(e) || plan.callers_of.values().any(|c| c.contains(e)));
    prog.arch_mut().kuna_custom_reg_callers = std::rc::Rc::new(map);
    prog.arch_mut().kuna_custom_reg_models = std::rc::Rc::new(models);
    affected
}

/// The callee entries whose address is stored as a 4-byte pointer word in a
/// loadable data section, exported, or the image entry point — callees reachable
/// other than by the direct calls the graph lists, so their convention is fixed.
fn address_taken(binary: &str, pref: SlicePref, entries: &HashSet<u64>) -> HashSet<u64> {
    let mut out = HashSet::new();
    let Ok(bytes) = image_bytes(binary, pref) else { return out };
    let Ok(file) = kuna_analysis::loadimage_object::parse_object(&bytes) else { return out };
    for section in file.sections() {
        if section.kind() == object::SectionKind::Text
            || section.kind() == object::SectionKind::UninitializedData
        {
            continue;
        }
        let Ok(data) = section.data() else { continue };
        for w in data.windows(4) {
            let v = u32::from_le_bytes([w[0], w[1], w[2], w[3]]) as u64;
            if entries.contains(&v) {
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

/// Is `entry` ineligible for a custom register convention regardless of the
/// evidence: an interior chunk inside another function's extent, or a
/// stack-moving CRT helper (`_chkstk`/SEH)? (Imports are already answered `None`
/// by [`kuna_decomp::kuna_calleereginputs::genuine_custom_reads`] and never
/// recorded; address-taken and open callees are handled by the caller.)
fn excluded(prog: &ConsoleProgram, entries: &[(u64, u64)], entry: u64) -> bool {
    let addr = Address::new(
        prog.arch().manage().get_default_code_space().expect("code space").clone(),
        entry,
    );
    if kuna_decomp::kuna_calleepurge::moves_stack(prog.arch(), &addr) {
        return true;
    }
    let at = entries.partition_point(|(a, _)| *a <= entry);
    if let Some(i) = at.checked_sub(1) {
        let (a, size) = entries[i];
        if a != entry && entry.wrapping_sub(a) < size {
            return true;
        }
    }
    false
}

/// Scan one caller body, recording for each direct call the custom registers
/// written earlier in the call's own basic block, and the caller it sits in.
fn scan_caller(
    prog: &ConsoleProgram,
    code: &std::rc::Rc<kuna_base::space::AddrSpace>,
    reg_idx: i32,
    regs: &[(u64, u32, u8)],
    entry: u64,
    extent: u64,
    out: &mut HashMap<u64, (u8, u64)>,
) {
    let translate = prog.arch().translate();
    let end = entry.wrapping_add(extent);
    struct Insn {
        at: u64,
        writes: u8,
        ends_block: bool,
    }
    let mut insns: Vec<Insn> = Vec::new();
    let mut targets: HashSet<u64> = HashSet::new();
    let mut calls: HashSet<u64> = HashSet::new();
    let mut at = entry;
    let mut steps = 0usize;
    while at < end {
        steps += 1;
        if steps > BUDGET {
            break;
        }
        let a = Address::new(code.clone(), at);
        let mut collect = Collect::default();
        let Ok(len) = translate.one_instruction(&mut collect, &a) else { break };
        let len = len.max(1) as u64;
        let mut writes = 0u8;
        let mut ends_block = false;
        for (opc, outv, ins) in &collect.0 {
            if let Some(o) = outv {
                if o.space.as_ref().is_some_and(|s| s.get_index() == reg_idx) {
                    for &(off, size, bit) in regs {
                        let lo = o.offset.max(off);
                        let hi = (o.offset + o.size as u64).min(off + size as u64);
                        if lo < hi {
                            writes |= bit;
                        }
                    }
                }
            }
            match opc {
                OpCode::CPUI_CALL => {
                    ends_block = true;
                    if let Some(d) = ins.first() {
                        if d.space.as_ref().map(|s| s.get_index()) == Some(code.get_index()) {
                            calls.insert(at);
                        }
                    }
                }
                OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => {
                    ends_block = true;
                }
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    ends_block = true;
                    if let Some(d) = ins.first() {
                        if d.space.as_ref().map(|s| s.get_index()) == Some(code.get_index())
                            && d.offset >= entry
                            && d.offset < end
                        {
                            targets.insert(d.offset);
                        }
                    }
                }
                _ => {}
            }
        }
        insns.push(Insn { at, writes, ends_block });
        at = at.wrapping_add(len);
    }
    if calls.is_empty() {
        return;
    }
    let mut written = 0u8;
    let mut block_boundary = true;
    for ins in &insns {
        if block_boundary || targets.contains(&ins.at) {
            written = 0;
        }
        if calls.contains(&ins.at) {
            let prev = out.get(&ins.at).map(|(m, _)| *m).unwrap_or(0xff);
            out.insert(ins.at, (prev & written, entry));
        }
        written |= ins.writes;
        block_boundary = ins.ends_block;
    }
}
