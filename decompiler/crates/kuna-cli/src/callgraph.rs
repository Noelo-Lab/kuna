//! Call-graph queries over the shared reference index and canonical inventory.

mod plan;
pub(crate) use plan::callee_first_plan;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use kuna_analysis::listing::xrefs::{SwitchTable, Xref, XrefIndex, XrefKind};
use kuna_analysis::loader::macho_fat::SlicePref;
use kuna_console::engine::ConsoleProgram;
use object::{Object, ObjectSection};

use crate::image::image_bytes;

/// The shared reference index, seeded with the canonical function inventory.
pub(crate) struct CallGraph {
    index: XrefIndex,
    /// Canonical `(entry, extent)` pairs, ascending by entry.
    entries: Vec<(u64, u64)>,
}

impl CallGraph {
    pub(crate) fn build(
        prog: &ConsoleProgram,
        binary: &str,
        pref: SlicePref,
    ) -> Result<CallGraph, String> {
        CallGraph::load(prog, binary, pref, false)
    }

    /// [`Self::build`]; `measured` also keeps what [`crate::limits`] measures
    /// each function's own body on (`xrefs::build_measured`).
    pub(crate) fn load(
        prog: &ConsoleProgram,
        binary: &str,
        pref: SlicePref,
        measured: bool,
    ) -> Result<CallGraph, String> {
        let bytes = image_bytes(binary, pref)?;
        let file = kuna_analysis::loadimage_object::parse_object(&bytes)
            .map_err(|e| format!("could not parse {binary}: {e}"))?;
        Ok(CallGraph::walk(prog, &file, measured))
    }

    /// [`Self::build`] off an already-parsed image, for a caller that holds one.
    pub(crate) fn build_from(prog: &ConsoleProgram, file: &object::File) -> CallGraph {
        CallGraph::walk(prog, file, false)
    }

    fn walk(prog: &ConsoleProgram, file: &object::File, measured: bool) -> CallGraph {
        let mut entries: Vec<(u64, u64)> = prog
            .function_entries_canonical()
            .iter()
            .map(|e| (e.addr.get_offset(), e.size))
            .collect();
        entries.sort_unstable();
        entries.dedup_by_key(|(addr, _)| *addr);
        let seeds: Vec<u64> = entries.iter().map(|(addr, _)| *addr).collect();
        let walk = if measured {
            kuna_analysis::listing::xrefs::build_measured
        } else {
            kuna_analysis::listing::xrefs::build
        };
        let index = walk(file, prog.arch(), prog.arch().translate(), &seeds);
        // PE names both the IAT slot and its veneer in the canonical inventory;
        // ELF traditionally names only the PLT veneer. Keep the graph vocabulary
        // symmetric by admitting every decoded forwarding slot as a zero-extent
        // node. decompile-graph materializes the missing rows from the same
        // relation; inventory-only surfaces still enumerate their established
        // loader records.
        for (_, slot) in index.forwarding_veneers() {
            if !entries.iter().any(|(entry, _)| *entry == slot) {
                entries.push((slot, 0));
            }
        }
        entries.sort_unstable();
        CallGraph { index, entries }
    }

    /// (kuna `callbacktype`) The entries of the functions that CALL or tail-jump
    /// to `entry`, itself included, or `None` when one such instruction sits in
    /// no function.
    pub(crate) fn direct_callers(&self, entry: u64) -> Option<Vec<u64>> {
        let mut out = BTreeSet::new();
        for r in self.index.refs_to(entry) {
            if matches!(r.kind, XrefKind::Call | XrefKind::Jump) {
                out.insert(self.owner_of(r.from)?);
            }
        }
        Some(out.into_iter().collect())
    }

    /// (kuna `callbacktype`) Does anything call or tail-jump to `entry` --
    /// another function, the function itself, or code no function owns?
    pub(crate) fn called_directly(&self, entry: u64) -> bool {
        self.index
            .refs_to(entry)
            .iter()
            .any(|r| matches!(r.kind, XrefKind::Call | XrefKind::Jump))
    }

    /// (kuna `callbacktype`) The instructions that materialize the address of
    /// `entry` without calling it -- the `lea`/immediate a callback argument is
    /// built by, and anything else that takes the same address -- each with the
    /// function it sits in.
    pub(crate) fn address_taken_refs(&self, entry: u64) -> Vec<(u64, Option<u64>)> {
        self.index
            .refs_to(entry)
            .iter()
            .filter(|r| matches!(r.kind, XrefKind::Data | XrefKind::Read | XrefKind::Write))
            .map(|r| (r.from, self.owner_of(r.from)))
            .collect()
    }

    /// (kuna `calleevote`) Every direct call and tail jump to the function
    /// entered at `entry` from another function, by instruction address, or
    /// `None` when its callers cannot all be listed: it is in `open`
    /// ([`open_function_entries`]), its address is taken in code, or a
    /// reference comes from no known function.
    pub(crate) fn direct_call_sites(&self, entry: u64, open: &BTreeSet<u64>) -> Option<Vec<u64>> {
        if open.contains(&entry) {
            return None;
        }
        let mut out = Vec::new();
        for r in self.index.refs_to(entry) {
            match r.kind {
                XrefKind::Call | XrefKind::Jump => {
                    if self.owner_of(r.from)? != entry {
                        out.push(r.from);
                    }
                }
                XrefKind::Data | XrefKind::Read | XrefKind::Write => return None,
            }
        }
        Some(out)
    }

    /// Every inventory entry reachable from `spec` through call, tail-jump, and
    /// address-taken-function-pointer edges, `spec`'s own function included.
    ///
    /// Function pointers count: a callback registered with `CreateThread` or an
    /// atexit handler is code the named function reaches, and dropping it would
    /// under-report exactly the indirection an obfuscated crackme leans on. A
    /// materialized address that does NOT land on a known function entry is not
    /// an edge (that is a string or a global, not a callee).
    pub(crate) fn reachable_from(
        &self,
        prog: &ConsoleProgram,
        spec: &str,
    ) -> Result<BTreeSet<u64>, String> {
        let start = resolve_function_spec(prog, spec)?;
        let seed = self.node_at(start).ok_or_else(|| {
            format!("--reachable-from {spec:?}: 0x{start:x} is in no discovered function")
        })?;

        let mut reached: BTreeSet<u64> = BTreeSet::new();
        let mut queue: VecDeque<u64> = VecDeque::new();
        reached.insert(seed);
        queue.push_back(seed);
        while let Some(node) = queue.pop_front() {
            for r in self.index.refs_from_function(node) {
                if let Some(callee) = self.callee_of(r) {
                    if reached.insert(callee) {
                        queue.push_back(callee);
                    }
                }
            }
        }

        // The walk's function set and kuna's inventory are two views of the same
        // program and need not agree entry-for-entry, so fold every reached
        // address onto the inventory record that contains it before answering.
        let mut owners: BTreeSet<u64> = BTreeSet::new();
        for vma in reached {
            owners.insert(vma);
            if let Some(owner) = self.owner_of(vma) {
                owners.insert(owner);
            }
        }
        Ok(owners)
    }

    /// Callees in reference order, including address-taken functions and folding
    /// interior targets onto inventory owners. Duplicate edges retain the first
    /// position and strongest kind: call, jump, then data. Intra-function jumps
    /// and targets outside the inventory are excluded.
    pub(crate) fn callees_of(&self, entry: u64) -> Vec<(u64, XrefKind)> {
        let mut out: Vec<(u64, XrefKind)> = Vec::new();
        let mut seen: BTreeMap<u64, usize> = BTreeMap::new();
        let mut refs: Vec<&Xref> = self.index.refs_from_function(entry);
        refs.sort_by_key(|r| (r.from, r.to));
        for r in refs {
            let Some(callee) = self.callee_of(r).and_then(|c| self.owner_of(c)) else {
                continue;
            };
            // A jump back onto the caller is a loop edge inside one body reached
            // through a second entry symbol, not a call.
            if callee == entry && r.kind == XrefKind::Jump {
                continue;
            }
            match seen.get(&callee) {
                Some(&at) if edge_rank(r.kind) < edge_rank(out[at].1) => out[at].1 = r.kind,
                Some(_) => {}
                None => {
                    seen.insert(callee, out.len());
                    out.push((callee, r.kind));
                }
            }
        }
        out
    }

    /// The call-graph node a reference edge lands on, or `None` when the edge is
    /// not a call-graph edge at all.
    ///
    /// A flow edge falls back to the inventory ([`Self::owner_of`]) when the
    /// walk has no node for the target. The walk only ever calls code a
    /// function, and an import is reached through its **IAT/GOT slot**, which
    /// lives in data: `CALL qword ptr [__imp_HeapAlloc]` lands on an address the
    /// walk correctly refuses to decode or attribute instructions to. PE names
    /// that slot in the inventory; the graph's forwarding-node completion above
    /// supplies the corresponding ELF GOT node. Without those fallbacks the
    /// import call is silently not an edge.
    fn callee_of(&self, r: &Xref) -> Option<u64> {
        match r.kind {
            XrefKind::Call | XrefKind::Jump => self.node_at(r.to).or_else(|| self.owner_of(r.to)),
            XrefKind::Data => self.index.is_function_entry(r.to).then_some(r.to),
            XrefKind::Read | XrefKind::Write => None,
        }
    }

    /// The walk's function entry for `vma`: the entry itself, else the function
    /// containing it (a branch into the middle of a body is still that body).
    fn node_at(&self, vma: u64) -> Option<u64> {
        if self.index.is_function_entry(vma) {
            return Some(vma);
        }
        self.index.function_containing(vma)
    }

    /// The inventory entry owning `vma`, preferring reachable instruction bodies.
    ///
    /// Project a known body owner onto the inventory; otherwise use the address's
    /// bounded inventory extent, including undecoded import slots. Addresses in
    /// gaps between inventory extents remain unowned.
    pub(crate) fn owner_of(&self, vma: u64) -> Option<u64> {
        let vma = self.index.reachable_function_containing(vma).unwrap_or(vma);
        let at = self.entries.partition_point(|(addr, _)| *addr <= vma);
        let (addr, size) = *self.entries.get(at.checked_sub(1)?)?;
        (vma == addr || vma - addr < size).then_some(addr)
    }

    /// Is `vma` an inventory entry?
    pub(crate) fn is_entry(&self, vma: u64) -> bool {
        self.entries
            .binary_search_by_key(&vma, |(addr, _)| *addr)
            .is_ok()
    }

    /// Every jump table the walk read.
    pub(crate) fn switch_tables(&self) -> &[SwitchTable] {
        self.index.switch_tables()
    }

    /// Each of `entries`' own instruction count, stopping at `cap`; zero unless
    /// the graph was loaded `measured` ([`Self::load`]).
    pub(crate) fn instruction_counts(&self, entries: &[u64], cap: usize) -> Vec<usize> {
        self.index.function_instruction_counts(entries, cap)
    }

    /// Does the function entered at `entry` make a computed call?
    ///
    /// A caller reading [`Self::callees_of`] needs this to tell "no callees"
    /// from "the callee is computed at run time and has no static target".
    pub(crate) fn has_indirect_calls(&self, entry: u64) -> bool {
        self.index.has_indirect_calls(entry)
    }

    /// The fixed pointer slot a forwarding veneer at `entry` jumps through
    /// (`jmp [slot]`), or `None` when `entry` is not one.
    pub(crate) fn veneer_slot(&self, entry: u64) -> Option<u64> {
        self.index.veneer_slot(entry)
    }

    /// Every decoded forwarding veneer and slot. Some formats (PE) already
    /// inventory both; ELF generally inventories only the veneer, so graph
    /// exporters use this relation to materialize the missing slot row.
    pub(crate) fn forwarding_veneers(&self) -> Vec<(u64, u64)> {
        self.index.forwarding_veneers()
    }

    /// (kuna `calleevote`, `callbacktype`) The function entries whose address the
    /// image puts somewhere nobody can list ([`open_function_entries`]). An image
    /// that cannot be read again leaves every function open.
    pub(crate) fn open_entries(&self, binary: &str, pref: SlicePref) -> BTreeSet<u64> {
        let seeds: Vec<u64> = self.entries.iter().map(|(entry, _)| *entry).collect();
        image_bytes(binary, pref)
            .ok()
            .and_then(|bytes| {
                let file = kuna_analysis::loadimage_object::parse_object(&bytes).ok()?;
                Some(open_function_entries(&file, &bytes, &seeds))
            })
            .unwrap_or_else(|| seeds.iter().copied().collect())
    }

    /// Does anything CALL `vma`?  Data and branch references do not count: the
    /// question a triaging agent asks is which functions no call site reaches.
    pub(crate) fn has_caller(&self, vma: u64) -> bool {
        self.index
            .refs_to(vma)
            .iter()
            .any(|r| r.kind == XrefKind::Call)
    }
}

/// (kuna `calleevote`) The function entries among `entries` whose callers the
/// image cannot list by its instructions alone.
///
/// The reference walk reads one instruction at a time, so it sees a function's
/// address taken in code only where one instruction carries all of it. That
/// holds on x86-64 (a RIP-relative `lea`, an immediate); elsewhere an address
/// is commonly built from two instructions (AArch64 `adrp`+`add`, MIPS
/// `lui`+`addiu`, ARM `movw`+`movt`, a PIC base plus an offset on i386), so on
/// every other architecture no function has every caller known. Neither does
/// one of a relocatable object, whose sections the loader lays out itself (a
/// stored address is a relocation against a zero word), nor of an image whose
/// sections hold none of its entries (no section headers).
///
/// Otherwise an entry is open when its address is stored in the image: a
/// pointer-width word of a section the image loads (`SHF_ALLOC` on ELF, at
/// any address, code included), a Mach-O chained-fixup rebase target, a
/// dynamic relocation's target, an exported symbol, or the entry point. A data
/// word that only happens to equal an entry leaves that function open, which
/// states nothing about it.
fn open_function_entries(file: &object::File, bytes: &[u8], entries: &[u64]) -> BTreeSet<u64> {
    use object::{ObjectSymbol, ObjectSymbolTable};
    let code_at = |e: u64| {
        file.sections().any(|s| {
            s.kind() == object::SectionKind::Text
                && s.address() <= e
                && e < s.address().saturating_add(s.size())
        })
    };
    if file.architecture() != object::Architecture::X86_64
        || file.kind() == object::ObjectKind::Relocatable
        || !entries.iter().any(|&e| code_at(e))
    {
        return entries.iter().copied().collect();
    }
    let mut scan = StoredScan::new(entries);
    let width: u64 = if file.is_64() { 8 } else { 4 };
    let little = file.is_little_endian();
    for section in file.sections() {
        if !image_loads(&section) {
            continue;
        }
        if let Ok(data) = section.data() {
            scan.words(section.address(), data, width, little);
        }
    }
    for v in kuna_analysis::loader::format::macho::resolve_chained_fixups(file, bytes).targets() {
        scan.note(v);
    }
    if let Some(relocs) = file.dynamic_relocations() {
        for (_, r) in relocs {
            let base = match r.target() {
                object::RelocationTarget::Symbol(i) => file
                    .dynamic_symbol_table()
                    .and_then(|t| t.symbol_by_index(i).ok())
                    .map(|s| s.address()),
                object::RelocationTarget::Absolute => Some(0),
                _ => None,
            };
            if let Some(b) = base {
                scan.note(b.wrapping_add(r.addend() as u64));
            }
        }
    }
    for sym in file.dynamic_symbols() {
        if sym.is_definition() {
            scan.note(sym.address());
        }
    }
    for export in file.exports().unwrap_or_default() {
        scan.note(export.address());
    }
    scan.note(file.entry());
    scan.out
}

/// (kuna `calleevote`) Does the image put this section's bytes in memory? On
/// ELF that is the `SHF_ALLOC` flag, whatever the address (firmware loads code
/// at 0); other formats map every section they carry bytes for.
fn image_loads(section: &object::Section) -> bool {
    if section.kind() == object::SectionKind::UninitializedData {
        return false;
    }
    match section.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & u64::from(object::elf::SHF_ALLOC) != 0,
        _ => !matches!(
            section.kind(),
            object::SectionKind::Debug | object::SectionKind::DebugString
        ),
    }
}

/// (kuna `calleevote`) The function entries found stored so far.
struct StoredScan {
    #[expect(clippy::disallowed_types, reason = "Membership only; the separate BTreeSet orders the results.")]
    wanted: std::collections::HashSet<u64>,
    out: BTreeSet<u64>,
}

impl StoredScan {
    fn new(entries: &[u64]) -> StoredScan {
        StoredScan {
            wanted: entries.iter().copied().collect(),
            out: BTreeSet::new(),
        }
    }

    /// A stored value.
    fn note(&mut self, v: u64) {
        if self.wanted.contains(&v) {
            self.out.insert(v);
        }
    }

    /// Every `width`-byte word of `data`, loaded at `addr`, that sits on a
    /// `width`-aligned address.
    fn words(&mut self, addr: u64, data: &[u8], width: u64, little: bool) {
        let skip = ((width - addr % width) % width) as usize;
        let Some(data) = data.get(skip..) else { return };
        for w in data.chunks_exact(width as usize) {
            let v = match (width, little) {
                (8, true) => u64::from_le_bytes(w.try_into().unwrap_or_default()),
                (8, false) => u64::from_be_bytes(w.try_into().unwrap_or_default()),
                (_, true) => u32::from_le_bytes(w.try_into().unwrap_or_default()) as u64,
                (_, false) => u32::from_be_bytes(w.try_into().unwrap_or_default()) as u64,
            };
            self.note(v);
        }
    }
}

/// How strong a claim one reference kind makes about a call-graph edge, lowest
/// first: a call, then a tail jump, then a materialized address.
fn edge_rank(kind: XrefKind) -> u8 {
    match kind {
        XrefKind::Call => 0,
        XrefKind::Jump => 1,
        _ => 2,
    }
}

/// Resolve a `<name|0xaddr>` operand against the loaded program.
///
/// A name is looked up FIRST and a bare-hex reading is only the fallback, so a
/// function genuinely called `abc` is not silently read as `0xabc` — the same
/// order `kuna xrefs` resolves its `--to`/`--from` operand in. An address is
/// resolved THROUGH the inventory when it names a known entry, which is what
/// folds the ARM Thumb mode bit out of an odd `--reachable-from 0x3dd`.
fn resolve_function_spec(prog: &ConsoleProgram, spec: &str) -> Result<u64, String> {
    let spec = spec.trim();
    let through_inventory = |addr: u64| {
        prog.find_entry_at(addr)
            .map_or(addr, |e| e.addr.get_offset())
    };
    if let Some(body) = spec.strip_prefix("0x").or_else(|| spec.strip_prefix("0X")) {
        return u64::from_str_radix(body, 16)
            .map(through_inventory)
            .map_err(|_| format!("invalid address {spec:?}"));
    }
    if let Some(entry) = prog.find_entry_by_name(spec) {
        return Ok(entry.addr.get_offset());
    }
    if let Some(addr) = prog.lookup_symbol(spec) {
        return Ok(through_inventory(addr.get_offset()));
    }
    u64::from_str_radix(spec, 16)
        .map(through_inventory)
        .map_err(|_| format!("no function named {spec:?} (and it is not an address)"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn established_entries_preserve_distant_callers_in_graph_queries() {
        use kuna_console::engine::{bootstrap_from_object_with_isa, ArmIsa};
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("split-body.elf");
        let specs = root.join("../../../specs");
        for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
            assert!(std::process::Command::new("python3")
                .arg(root.join("../kuna-analysis/tests/fixtures/arm_xref_bodies.py"))
                .arg(&path)
                .arg(endian)
                .status()
                .unwrap()
                .success());
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [
                ("listing", "on"),
                ("funcstart_patterns", "on"),
                ("aif", "off"),
            ] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            for measured in [false, true] {
                let mut graph = CallGraph::walk(&prog, &file, measured);
                assert!(graph.is_entry(0x1600));
                assert_eq!(graph.direct_callers(0x1100), Some(vec![0x1500]));
                assert_eq!(graph.owner_of(0x1754), Some(0x1500));
                assert_eq!(graph.owner_of(0x1604), Some(0x1600));
                assert_eq!(graph.callees_of(0x1500), vec![(0x1100, XrefKind::Call)]);
                assert!(graph.callees_of(0x1600).is_empty());
                assert!(graph.has_indirect_calls(0x1500));
                assert!(!graph.has_indirect_calls(0x1600));
                assert_eq!(
                    graph.direct_call_sites(0x1100, &BTreeSet::new()),
                    Some(vec![0x1750])
                );
                assert_eq!(
                    graph.reachable_from(&prog, "0x1500").unwrap(),
                    BTreeSet::from([0x1100, 0x1500])
                );
                graph.entries.push((0x1900, 0));
                assert_eq!(graph.owner_of(0x1900), Some(0x1900));
                assert_eq!(graph.owner_of(0x1901), None);
                assert_eq!(graph.owner_of(0x18fc), None);
            }
        }
    }

    #[test]
    fn recovered_frames_preserve_distant_callers_in_graph_queries() {
        use kuna_console::engine::{bootstrap_from_object_with_isa, ArmIsa};
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("split-body.elf");
        let specs = root.join("../../../specs");
        for (endian, target) in [("little", "ARM:LE:32:v8"), ("big", "ARM:BE:32:v8")] {
            assert!(std::process::Command::new("python3")
                .arg(root.join("../kuna-analysis/tests/fixtures/arm_xref_roots.py"))
                .arg(&path)
                .args(["arm", endian, "splitbody"])
                .status()
                .unwrap()
                .success());
            let mut prog = bootstrap_from_object_with_isa(
                path.to_str().unwrap(),
                target,
                &[specs.to_str().unwrap().into()],
                Some(ArmIsa::Arm),
            )
            .unwrap();
            for (name, value) in [
                ("listing", "on"),
                ("funcstart_patterns", "on"),
                ("aif", "off"),
            ] {
                prog.arch_mut().set_kuna_option(name, value).unwrap();
            }
            prog.commit_pending_analysis().unwrap();
            let bytes = std::fs::read(&path).unwrap();
            let file = object::File::parse(&*bytes).unwrap();
            for measured in [false, true] {
                let mut graph = CallGraph::walk(&prog, &file, measured);
                assert!(graph.is_entry(0x1600));
                assert_eq!(graph.direct_callers(0x1100), Some(vec![0x1500]));
                assert_eq!(graph.owner_of(0x1754), Some(0x1500));
                assert_eq!(graph.owner_of(0x1604), Some(0x1600));
                assert_eq!(graph.callees_of(0x1500), vec![(0x1100, XrefKind::Call)]);
                assert!(graph.callees_of(0x1600).is_empty());
                assert!(graph.has_indirect_calls(0x1500));
                assert!(!graph.has_indirect_calls(0x1600));
                assert_eq!(
                    graph.direct_call_sites(0x1100, &BTreeSet::new()),
                    Some(vec![0x1750])
                );
                assert_eq!(
                    graph.reachable_from(&prog, "0x1500").unwrap(),
                    BTreeSet::from([0x1100, 0x1500])
                );
                graph.entries.push((0x1900, 0));
                assert_eq!(graph.owner_of(0x1900), Some(0x1900));
                assert_eq!(graph.owner_of(0x1901), None);
                assert_eq!(graph.owner_of(0x18fc), None);
            }
        }
    }

    fn fixture(name: &str) -> Vec<u8> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-analysis/tests/fixtures")
            .join(name);
        std::fs::read(path).expect("the fixture is checked in")
    }

    /// Only aligned words are read, and a word one past an entry is not that
    /// entry.
    #[test]
    fn only_an_aligned_word_equal_to_an_entry_stores_it() {
        let mut scan = StoredScan::new(&[0x68f0, 0x7000]);
        let mut data = vec![0u8; 4];
        data.extend_from_slice(&0x68f1u64.to_le_bytes());
        data.extend_from_slice(&0x7000u64.to_le_bytes());
        scan.words(0x3ffc, &data, 8, true);
        assert_eq!(scan.out.into_iter().collect::<Vec<_>>(), vec![0x7000]);
    }

    /// A relocatable object's table words are zero until a relocation fills
    /// them, so no function in it has every caller known.
    #[test]
    fn every_function_of_a_relocatable_object_is_open() {
        let bytes = fixture("calleevote_ops_x86_64.o");
        let file = kuna_analysis::loadimage_object::parse_object(&*bytes).expect("an ELF object");
        let entries = [0x400000, 0x400010, 0x400020, 0x400040, 0x400060];
        assert_eq!(
            open_function_entries(&file, &bytes, &entries).len(),
            entries.len()
        );
    }

    /// MIPS builds a function's address from `lui`+`addiu`, which the
    /// one-instruction reference walk does not see, so off x86-64 no function
    /// has every caller known, even one whose address no data word holds.
    #[test]
    fn every_function_off_x86_64_is_open() {
        let bytes = fixture("calleevote_callback_mipsel");
        let file = kuna_analysis::loadimage_object::parse_object(&*bytes).expect("an ELF image");
        let entries = [0x400110, 0x40011c, 0x400144, 0x400168, 0x40018c];
        assert_eq!(
            open_function_entries(&file, &bytes, &entries).len(),
            entries.len()
        );
    }

    /// A const table in a `.text` loaded at address 0: the section is read
    /// because the image loads it, so both functions the table names are
    /// stored, and the ones it does not name are not.
    #[test]
    fn a_table_in_code_loaded_at_zero_stores_its_entries() {
        let bytes = fixture("calleevote_zero_x86_64");
        let file = kuna_analysis::loadimage_object::parse_object(&*bytes).expect("an ELF image");
        let entries = [0x10, 0x20, 0x30, 0x60, 0x80];
        let open = open_function_entries(&file, &bytes, &entries);
        assert_eq!(open.into_iter().collect::<Vec<_>>(), vec![0x10, 0x20]);
    }
}
