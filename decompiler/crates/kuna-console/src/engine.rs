//! Engine bootstrap glue for the console front-end (item `w9x-bins-runner`).
//!
//! The C++ `IfcLoadFile` (`consolemain.cc:46`) hands a file path to
//! `ArchitectureCapability::findCapability`, builds the leaf `Architecture`
//! (`XmlArchitecture`), and runs `Architecture::init` — the whole
//! `restoreFromSpec`/`buildTranslator`/`buildTypegrp`/`buildAction` chain.  In
//! the kuna Rust port that chain is assembled by the XML frontend
//! ([`XmlArchitecture`]) through `build_translator`, then
//! [`Architecture::init_post_engine`] (the merged `w9x-arch-engine-glue` item),
//! which the integration-test `decompile_e2e.rs` `bootstrap()` proved end to
//! end.  This module lifts that bootstrap into a reusable shape so the two
//! console paths — the interactive `decomp_dbg` `load file` command and the
//! datatest harness `buildProgram` — drive the **same** real engine assembly.
//!
//! ## What `load file` accepts
//!
//! The C++ console drives a real binary through BFD; the kuna Rust engine's only
//! load-image backend is the XML `<binaryimage>` format the datatests use (the
//! BFD `RawBinaryArchitecture`/`LoadImageBfd` backends are their own port item).
//! So the Rust `load file <path>` accepts a `<binaryimage>` (or
//! `<decompilertest>`-wrapping) XML file — exactly the corpus image format the
//! Python tools (`kuna/decompile.py`) and the datatests feed, which is what the
//! `KUNA_ENGINE=rust` path is wired to drive.
//!
//! ## Symbol resolution (the `readLoaderSymbols`/`queryFunction` hook)
//!
//! The W4 symbol-table population from the loader (`Architecture::readLoaderSymbols`
//! → `Scope::addFunction`) and `Scope::queryFunction` are later port items.  The
//! `<binaryimage>` itself carries `<symbol>` records (name + address), and the
//! opened [`LoadImageXml`] exposes them via `open_symbols`/`get_next_symbol`.
//! This module reads those records once (at `load file`) into a name→address
//! table on the [`ConsoleProgram`], so `load function <name>` resolves a
//! function entry the faithful way (the binaryimage's own symbol records, which
//! is precisely what `readLoaderSymbols` reads).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase, SleighArchitecture};
use kuna_decomp::xml_arch::XmlArchitectureCapability;

use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageFunc, LoadImageSection};
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_sleigh::loadimage_xml::LoadImageXml;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

use crate::entry_selector::ObjectSectionLocation;
pub use crate::entry_selector::{
    EntryLookupError, EntryProvenance, EntrySelector, FunctionEntry, ObjectLocation,
};

/// One function symbol discovered in the `<binaryimage>` (name → entry address).
#[derive(Debug, Clone)]
struct ProgramSymbol {
    name: String,
    addr: Address,
    object_location: Option<ObjectLocation>,
    binding: Option<String>,
    provenance: EntryProvenance,
}

/// (kuna) One emitted p-code op, whole: the opcode, the output varnode and
/// every input. [`OneShotPcodeEmit`] keeps only `in0`; the parts it drops are
/// what a data-reference scan is made of.
struct WholeOp {
    opcode: OpCode,
    out: Option<VarnodeData>,
    ins: Vec<VarnodeData>,
}

/// (kuna) Every FIXED address a RANGE of instructions names, in the two flavours
/// a listing needs to tell code from data — plus the decode buffer the walk
/// filling it reuses.
///
/// One value is filled by many [`ConsoleProgram::add_fixed_refs_at`] calls, and
/// the op capture is rewound rather than dropped between them: allocating per op
/// costs one heap allocation for every p-code op walked, which on a whole-image
/// listing is millions of them.
#[derive(Default)]
pub struct FixedRefs {
    /// `(address, width)` for each read of an address an instruction spelled
    /// out. The width is what was READ, not the size of the address varnode.
    pub reads: Vec<(u64, u32)>,
    /// Every address a `BRANCH`/`CBRANCH`/`CALL` named outright.
    pub flow_targets: Vec<u64>,
    /// The current instruction's ops, over storage retained across instructions.
    ops: Vec<WholeOp>,
    /// How many of `ops` the current instruction has filled.
    filled: usize,
}

impl kuna_sleigh::translate::PcodeEmit for FixedRefs {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        if self.filled == self.ops.len() {
            self.ops.push(WholeOp { opcode: opc, out: outvar.cloned(), ins: vars.to_vec() });
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

impl FixedRefs {
    /// Project the ops the last decode emitted onto the two reference lists.
    ///
    /// A read is harvested in the two shapes SLEIGH spells one in: a `LOAD`
    /// whose address input is a constant (`ldr r3,[0x8458]`) and a direct memory
    /// varnode in the default data space (`mov eax,[0x404018]`). The width is
    /// the width of the ACCESS — a `LOAD`'s address varnode is pointer-sized
    /// whatever the access is, so `ldrh r0,[0x1003c]` must not read as a
    /// pointer-sized one. The `in0` slot of `LOAD`/`STORE`/`CALLOTHER` and of
    /// every flow op is skipped: it carries a space id or a destination, not an
    /// address that is read.
    ///
    /// `fall_through` (`vma + len`) is not recorded as a flow target, for the
    /// same reason [`super::super`]'s reference walk declines it as a value:
    /// every conditionally-executed ARM instruction lowers to a `CBRANCH` over
    /// its own body, so its successor is named by a flow op whatever it is.
    /// Counted, that would mark the word after every predicated instruction as a
    /// branch label — and a literal pool is a run of them.
    fn harvest(&mut self, data_space: Option<&Rc<AddrSpace>>, fall_through: u64) {
        let Self { reads, flow_targets, ops, filled } = self;
        let is_constant = |vn: &VarnodeData| {
            vn.space
                .as_ref()
                .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT)
        };
        // Space identity is pointer identity throughout the engine, so match on
        // the `Rc`, never on the space's name or index.
        let in_data_space = |vn: &VarnodeData| {
            matches!((&vn.space, data_space), (Some(s), Some(d)) if Rc::ptr_eq(s, d))
        };
        for op in &ops[..*filled] {
            let flow = matches!(
                op.opcode,
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL
            );
            if flow {
                if let Some(vn) = op.ins.first() {
                    if !is_constant(vn) && vn.offset != fall_through {
                        flow_targets.push(vn.offset);
                    }
                }
            }
            for (i, vn) in op.ins.iter().enumerate() {
                let target_slot = i == 0
                    && (flow
                        || matches!(
                            op.opcode,
                            OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_CALLOTHER
                        ));
                if target_slot {
                    continue;
                }
                let names_an_address = in_data_space(vn)
                    || (i == 1 && op.opcode == OpCode::CPUI_LOAD && is_constant(vn));
                if !names_an_address {
                    continue;
                }
                // The width of the ACCESS, which for a `LOAD` is its output and
                // NOT its address varnode -- SLEIGH gives `ldrh r0,[0x1003c]` a
                // pointer-sized `ram` address and a 2-byte output, and reading
                // the address would call a halfword read a word.
                let width = match op.opcode {
                    OpCode::CPUI_LOAD if i == 1 => op.out.as_ref().map(|o| o.size),
                    OpCode::CPUI_STORE => None,
                    _ => Some(vn.size),
                };
                if let Some(width) = width {
                    reads.push((vn.offset, width));
                }
            }
        }
    }
}

/// (kuna) A one-shot [`PcodeEmit`](kuna_sleigh::translate::PcodeEmit) sink:
/// `Translate::one_instruction` dumps exactly one instruction's p-code, each
/// op captured here as opcode + first input (the single-instruction pcode
/// analogue of [`ConsoleProgram::disassemble_at_into`], mirroring
/// `kuna_analysis`'s `listing/decode.rs::OpCapture` — `in0` is all
/// [`ConsoleProgram::lone_jump_target`]'s shape tests need).
#[derive(Default)]
struct OneShotPcodeEmit {
    ops: Vec<(OpCode, Option<VarnodeData>)>,
}

impl kuna_sleigh::translate::PcodeEmit for OneShotPcodeEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        opc: OpCode,
        _outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops.push((opc, vars.first().cloned()));
    }
}

/// The console's loaded program: the engine assembly (C++ `dcp->conf`, an
/// `XmlArchitecture : Architecture`) plus the console-owned marshaling registry
/// and option database the `option` command needs.
///
/// In C++ `dcp->conf` IS the leaf `Architecture` (the `XmlArchitecture`
/// subobject); the Rust leaf [`XmlArchitecture`] owns the `Architecture` base,
/// reachable via [`Self::arch_mut`].  The `IdRegistry`/`OptionDatabase` are the
/// process globals the C++ `ElementId::find` + `dcp->conf->options->set` read; we
/// keep them on the program so the console can resolve an option name to its
/// element id and dispatch it against the real architecture.
pub struct ConsoleProgram {
    /// The engine assembly (owns the `Architecture` god object).  Both the XML
    /// `<binaryimage>` frontend and the real-ELF frontend slice their leaf
    /// architecture back to this `SleighArchitecture` once the loader is opened
    /// and handed to the engine, so the console program is loader-agnostic.
    arch: SleighArchitecture,
    /// The marshaling id registry (C++ `ElementId` global table) for option-name
    /// resolution.
    registry: IdRegistry,
    /// The binaryimage's function symbols (name → entry address), read once at
    /// load (the `readLoaderSymbols` hook).
    symbols: Vec<ProgramSymbol>,
    /// Original-to-synthetic section map for relocatable objects.
    object_sections: Vec<ObjectSectionLocation>,
    /// A human-readable description of the loaded program (C++
    /// `conf->getDescription()`).
    description: String,
    /// (kuna) Per-pass analysis facts stashed at load (real-ELF path only),
    /// keyed by `AnalysisPass::id`, awaiting the gated commit at `read symbols`.
    ///
    /// The commit is **deferred** out of `bootstrap_from_object` so it runs AFTER the
    /// per-pass `--option <id> on|off` flags are applied (the CLI emits the
    /// `option` lines before `read symbols`). `IfcReadSymbols` consults each pass's
    /// enable flag on the `Architecture` and commits only the enabled passes'
    /// facts (see [`commit_analysis_passes`]). Empty on the XML datatest path (no
    /// analysis tier runs), so the gated commit is a faithful no-op there. The
    /// stash is drained on commit so a second `read symbols` does not re-commit.
    pending_analysis: Vec<(&'static str, kuna_analysis::pass::AnalysisOutput)>,
    /// (kuna) The engine default code space captured at load, used to build the
    /// `Address`es when the stashed analysis facts are committed at `read symbols`.
    analysis_code_space: Option<Rc<AddrSpace>>,
    /// (kuna) DWARF stack-LOCAL recommendations parked at the analysis commit
    /// (`commit_analysis_output`, DWARF subtask 3), keyed by owning-function entry
    /// VMA. Each is `(name, type, stack_offset)`; the entry's `Vec` is the function's
    /// locals. `IfcDecompile` looks the function up by its entry address and appends
    /// these (built into stack-space `map addr` symbol specs by [`Self::dwarf_locals_for`])
    /// to the `mapped_symbols` it threads through `decompile_func_full_with_override_dyn`,
    /// so the rebuilt `Funcdata`'s `ScopeLocal` is re-seeded with each as a
    /// typelock|namelock stack symbol — the same path the console `map addr` directive
    /// uses. Real-ELF DWARF path only (empty on the XML datatest path).
    dwarf_locals: Vec<(u64, String, Rc<kuna_decomp::dtype::Datatype>, i64)>,
    /// (kuna) The image bytes + path stashed at load for the **deferred Listing
    /// build** (the Listing/xref PR6 build-timing fix). The Listing is gated on
    /// `--option listing on`, a flag the live CLI sets AFTER `load file` (before
    /// `read symbols`), so it cannot be built at load (the flag is still default-off
    /// there). Instead `commit_pending_analysis` (reached at `read symbols`, after
    /// the flag is applied) re-parses these bytes, builds the Listing, and runs the
    /// Listing-consumer passes (e.g. discovered-no-return). `None` on the XML
    /// datatest path (no Listing tier), so the gated build is a structural no-op
    /// there. Empty/`None` when the listing flag is off ⇒ zero cost.
    analysis_image: Option<(String, Vec<u8>)>,
    /// (kuna) The loader's defined `STT_OBJECT` data symbols as `(addr, name,
    /// size)`, read from `.symtab`/`.dynsym` at load
    /// ([`ObjectLoadImage::data_symbols`]) and installed as named globals by
    /// [`commit_analysis_output`].
    ///
    /// This is **loader markup**, not an analysis pass: it is the data twin of the
    /// funcsym stream `read_loader_symbols` already installs. Per the standing
    /// options contract it is gated by `--option datasyms on|off` (default ON,
    /// DIV-76), consulted at the commit via `Architecture::analysis_datasyms` —
    /// the stream is collected at `load file` but committed at `read symbols`,
    /// after the option lines are applied, so both CLI paths honor the flag with
    /// no env bridge. It is installed at the analysis commit rather than eagerly
    /// at bootstrap purely for *precedence*: a DWARF-described global and a
    /// detected string literal both claim their address first, and this arm only
    /// fills the addresses neither covered (which is where the imported libc
    /// objects — `optind`, `stdin`, `stdout`, `optarg` — live). Empty on the XML
    /// datatest path and for a relocatable object.
    loader_data_objects: Vec<(u64, String, u64)>,
    /// (kuna) **Caller-declared function extents**, entry VMA → byte size — what
    /// the console `function bounds <start> <end>` and the CLI
    /// `kuna --define-function <start>-<end>` assert, and the only place kuna
    /// carries "function F spans [start,end)" at all. (`map function` still
    /// ignores its size argument; only these two surfaces write here.)
    ///
    /// Consulted by every later load of that entry (`load function`, `load addr`,
    /// the whole-binary loop) so the declaration outlives the one command that
    /// made it, and by [`crate::funcextent`] so the inventory reports what the
    /// caller asserted rather than the address-contiguous clip it guesses.
    /// Non-zero sizes only: a bare declaration with no extent leaves the entry
    /// unbounded, which is the engine-wide `UNBOUNDED_SIZE` default.
    declared_extents: BTreeMap<u64, int4>,
    /// (kuna `--assert`) The caller-supplied assertions this program was loaded
    /// with, in the order they were given -- the one override plane an agent
    /// states facts through (`crate::assertions`).  Empty for every invocation
    /// that passed none, which is what keeps the plane free.
    assertions: Vec<crate::assertions::Directive>,
    /// One slot per [`Self::assertions`] entry: what became of that directive.
    /// `None` until something claims it, so a directive no surface reached is
    /// still distinguishable from one that was applied.
    assertion_outcomes: Vec<Option<crate::assertions::Outcome>>,
    /// Prototypes parked by an `assert prototype <func> <decl>` directive, keyed
    /// by function name -- the in-process twin of the console's
    /// `IfaceDecompData::pending_prototypes` (`parse line extern`).  Consulted by
    /// the decompile loop, because the drive rebuilds the `Funcdata` and the
    /// symbol-table prototype link does not survive that rebuild.
    pending_prototypes: BTreeMap<String, kuna_decomp::fspec::PrototypePieces>,
}

impl ConsoleProgram {
    /// Borrow the `Architecture` god object (C++ `dcp->conf`, viewed as the base).
    pub fn arch(&self) -> &Architecture {
        self.arch
            .base()
            .expect("ConsoleProgram: Architecture base present after bootstrap")
    }

    /// Mutably borrow the `Architecture` god object.
    pub fn arch_mut(&mut self) -> &mut Architecture {
        self.arch
            .base_mut()
            .expect("ConsoleProgram: Architecture base present after bootstrap")
    }

    /// The marshaling id registry (for `ElementId::find`).
    pub fn registry(&self) -> &IdRegistry {
        &self.registry
    }

    /// The number of function symbols read from the binaryimage (what the
    /// `readLoaderSymbols` hook yields).
    pub fn num_symbols(&self) -> usize {
        self.symbols.len()
    }

    /// C++ `conf->getDescription()` — the load-success description line.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Resolve a function entry address by symbol name (the `queryFunction`
    /// hook): scan the binaryimage symbols read at load.  `None` if no symbol of
    /// that name exists.
    pub fn lookup_symbol(&self, name: &str) -> Option<Address> {
        // (kuna `symbolnamebound`) `self.symbols` carries the bounded spelling,
        // so bound the query too: `load function <name>` must accept both the
        // binary's ORIGINAL name and the one the listing renders. Idempotent, and
        // a no-op for every real name.
        let name = &*kuna_decomp::kuna_symbolnamebound::bound_scope_path(name, "::");
        self.symbols.iter().find(|s| s.name == name).map(|s| s.addr.clone())
    }

    /// (kuna) Iterate every function symbol kuna knows for the loaded program as
    /// `(name, entry Address)` — the full callable-symbol inventory.
    ///
    /// After [`Self::commit_pending_analysis`] (`read symbols`) this covers both
    /// the loader's function symbols (`.symtab`/`.dynsym`/PLT stubs, read at load
    /// into `self.symbols`) and the analysis-tier discovered/renamed functions
    /// (committed via [`Self::register_symbol`]).  The addresses are engine
    /// code-space VMAs, which equal the ELF file virtual addresses for
    /// `ET_EXEC`/`ET_DYN`.  CRT/thunk/PLT filtering is intentionally left to the
    /// caller (the decbench backend's `should_skip_function`), matching the
    /// raw-backend convention — the engine reports every function it found.
    pub fn function_entries(&self) -> impl Iterator<Item = (&str, &Address)> {
        self.symbols.iter().map(|s| (s.name.as_str(), &s.addr))
    }

    /// (kuna, issue #197) The **canonical** whole-binary function enumeration:
    /// exactly one [`FunctionEntry`] per function ENTRY, address-ordered, with
    /// every other name that entry carries kept as an alias.
    ///
    /// [`Self::function_entries`] is the raw symbol stream, in which one function
    /// can appear several times — [`Self::register_symbol`] retains by NAME, so
    /// two names at one address are two records.  Three producers feed that:
    ///
    /// 1. **A generic alias at the same address.** An analysis pass re-registers a
    ///    discovered entry under the engine placeholder `sub_<addr>` even though
    ///    the loader already named it, so `compute` and `sub_100b8` both sit at
    ///    `0x100b8`.  Likewise every FID/PDB/demangle *rename* leaves the old
    ///    placeholder behind in the symbol stream.
    /// 2. **The ARM Thumb `entry|1` twin.** An ARM/Thumb function's ELF symbol
    ///    stores the mode bit IN `st_value` (this repo's `arm_thumb_linked_le32`
    ///    fixture has `compute` at `st_value = 0x100b9`), so an unmasked consumer
    ///    reports a second entry one byte above the real one.  That twin is not
    ///    merely redundant: `0x100b9` is not an instruction boundary, so it
    ///    decompiles to a bogus empty `void sub_100b9(void)`.
    /// 3. **A second, genuinely different name for one function.** A debug-info
    ///    pass (DWARF/PDB/pclntab/objc) emits a name the linker symbol table does
    ///    not have, and it is registered beside the loader's: `macho_dwarf.o`
    ///    carries `_l0`+`first_byte` at `0x0` and `_main`+`main` at `0x40`, and a
    ///    decorated/undecorated PE pair takes the same path.  (The loader's own
    ///    funcsym stream cannot do this — it is address-deduped as it is read.)
    ///
    /// Producers 1 and 3 collapse by address; producer 2 needs the same Thumb-bit
    /// normalization [`crate::project::build_asm`] already applies to its labels
    /// (`vma & !1` on an ARM-family spec), which is also what the loader does when
    /// it turns an odd `st_value` into an even entry address.
    ///
    /// The retained `name` is the most informative one ([`entry_name_rank`]);
    /// `aliases` keeps the rest so a name-keyed lookup
    /// ([`Self::find_entry_by_name`], behind `--functions <alias>`) still
    /// resolves.  Nothing is lost — one function is simply reported, and
    /// decompiled, once.
    pub fn function_entries_canonical(&self) -> Vec<FunctionEntry> {
        let normalize = |vma: u64| self.thumb_normalized(vma);

        // Group every name by normalized entry offset, keeping one Address per
        // group (rebuilt at the normalized offset, so an ARM twin reports the
        // real, even entry).
        let mut groups: BTreeMap<u64, (Address, Vec<&ProgramSymbol>)> = BTreeMap::new();
        for s in &self.symbols {
            // A sentinel (spaceless) address cannot be normalized or rebuilt;
            // such a record is not a real entry, so skip it rather than guess.
            let Some(space) = s.addr.get_space() else { continue };
            let vma = normalize(s.addr.get_offset());
            let entry = groups
                .entry(vma)
                .or_insert_with(|| (Address::new(Rc::clone(space), vma), Vec::new()));
            if !entry.1.iter().any(|record| record.name == s.name) {
                entry.1.push(s);
            }
        }

        let mut entries: Vec<FunctionEntry> = groups
            .into_iter()
            .map(|(_, (addr, mut records))| {
                // Most informative first — see `entry_name_rank`.
                records.sort_by(|a, b| {
                    entry_name_rank(&a.name)
                        .cmp(&entry_name_rank(&b.name))
                        .then_with(|| a.name.cmp(&b.name))
                });
                let canonical = records.remove(0);
                let object_record = std::iter::once(canonical)
                    .chain(records.iter().copied())
                    .find(|record| record.object_location.is_some());
                let provenance = object_record
                    .map(|_| EntryProvenance::DefinedObject)
                    .unwrap_or_else(|| {
                        if canonical.provenance == EntryProvenance::UndefinedExternal
                            || records.iter().any(|record| {
                                record.provenance == EntryProvenance::UndefinedExternal
                            })
                        {
                            EntryProvenance::UndefinedExternal
                        } else {
                            EntryProvenance::Mapped
                        }
                    });
                FunctionEntry {
                    name: canonical.name.clone(),
                    addr,
                    aliases: records.iter().map(|record| record.name.clone()).collect(),
                    object_location: object_record
                        .and_then(|record| record.object_location.clone()),
                    binding: object_record
                        .and_then(|record| record.binding.clone())
                        .or_else(|| canonical.binding.clone()),
                    provenance,
                    size: 0,
                }
            })
            .collect();
        // (kuna, `functions-json-size`) Measure each entry's extent in one pass.
        // The `BTreeMap` above already put the list in ascending address order,
        // which is what the clip needs; the section table is the loader's, so
        // this adds no decode to the cheap inventory call.
        crate::funcextent::assign_extents(
            &mut entries,
            &crate::funcextent::spans(&self.sections(), &self.segments()),
        );
        // A caller-declared extent is an assertion, not a guess: it outranks the
        // clip everywhere the clip is reported (`declared_extents`).
        if self.has_declared_extents() {
            for entry in &mut entries {
                let declared = self.declared_extent(entry.addr.get_offset());
                if declared > 0 {
                    entry.size = declared as u64;
                }
            }
        }
        entries
    }

    /// (kuna, `functions-json-size`) The byte extent of an arbitrary address,
    /// for the single-target paths that synthesize a [`FunctionEntry`] the
    /// enumeration does not know (`--addr` on an undiscovered function).
    ///
    /// Same clip, same upper-bound meaning as the bulk pass — see
    /// [`crate::funcextent`].
    pub fn function_extent_at(&self, vma: u64) -> u64 {
        let entry = self.thumb_normalized(vma);
        let declared = self.declared_extent(entry);
        if declared > 0 {
            return declared as u64;
        }
        let entries: Vec<u64> = self
            .function_entries_canonical()
            .iter()
            .map(|e| e.addr.get_offset())
            .collect();
        crate::funcextent::extent_at(
            entry,
            &entries,
            &crate::funcextent::spans(&self.sections(), &self.segments()),
        )
    }

    /// The canonical entries eligible for automatic whole-binary decompilation.
    ///
    /// Import pointer slots remain in the canonical inventory for call naming
    /// and explicit address selection, but data addresses are not function bodies.
    /// Loaders without section metadata retain the complete inventory.
    pub fn function_entries_executable(&self) -> Vec<FunctionEntry> {
        let sections = self.sections();
        if sections.is_empty() {
            return self.function_entries_canonical();
        }

        self.function_entries_canonical()
            .into_iter()
            .filter(|entry| {
                let vma = entry.addr.get_offset();
                sections.iter().any(|&(start, size, flags)| {
                    flags & section_flags::CODE != 0 && vma >= start && vma - start < size
                })
            })
            .collect()
    }

    /// (kuna, issue #197) Resolve a canonical entry by ANY of its names — the
    /// reported `name` or any of its `aliases`.
    ///
    /// The name-keyed lookup behind `kuna decompile-all --functions <name>` (and
    /// the wasm `decompile <name>` command).  Collapsing the enumeration must not
    /// make a name that used to select a function stop working, so the filter
    /// searches the alias set too — this is what preserves the decbench
    /// name-narrowing the old `(name, offset)` dedup was keeping duplicate records
    /// for.
    ///
    /// `None` also when the name identifies SEVERAL entries: a caller that can
    /// only answer yes-or-no must not silently pick one of them. A caller that
    /// can report the ambiguity asks [`Self::resolve_entry`] instead, which
    /// names every candidate.
    pub fn find_entry_by_name(&self, want: &str) -> Option<FunctionEntry> {
        // (kuna `symbolnamebound`) The enumeration reports the bounded spelling,
        // so bound the query too -- a caller holding the binary's ORIGINAL name
        // must still resolve. Idempotent, so the bounded spelling resolves as
        // well, and a no-op for every real name.
        let want = &*kuna_decomp::kuna_symbolnamebound::bound_scope_path(want, "::");
        let mut matches = self
            .function_entries_canonical()
            .into_iter()
            .filter(|e| e.name == want || e.aliases.iter().any(|a| a == want));
        let entry = matches.next()?;
        matches.next().is_none().then_some(entry)
    }

    /// (kuna, issue #197) Resolve the canonical entry AT `vma`, tolerating an
    /// ARM/Thumb `entry|1` address.
    ///
    /// Backs `kuna decompile-all --addr 0xVMA` (and the wasm `decompile 0xADDR`
    /// command).  An ARM caller legitimately holds odd addresses — an ELF symbol
    /// value, a DWARF entry PC, or a benchmark case address all carry the Thumb
    /// mode bit — and decompiling literally there lands mid-instruction and
    /// yields an empty body.  Folding the bit reaches the real function.
    ///
    /// Deliberately conservative: it only ever returns an entry the enumeration
    /// already knows, so an address that is not a discovered function start is
    /// still `None` and the caller keeps decompiling exactly where it was asked.
    pub fn find_entry_at(&self, vma: u64) -> Option<FunctionEntry> {
        let want = self.thumb_normalized(vma);
        self.function_entries_canonical()
            .into_iter()
            .find(|e| e.addr.get_offset() == want)
    }

    /// Resolve a selector without guessing between object-file coordinates.
    ///
    /// Numeric resolution checks the mapped synthetic address space first. On
    /// a relocatable object only, an otherwise-unmapped value may then match a
    /// defined function's raw section offset; that compatibility form succeeds
    /// only when the match is unique.
    pub fn resolve_entry(
        &self,
        selector: &EntrySelector,
    ) -> Result<FunctionEntry, EntryLookupError> {
        match selector {
            EntrySelector::Name(want) => {
                // (kuna `symbolnamebound`) The enumeration reports the bounded
                // spelling, so bound the query too -- a caller holding the
                // binary's ORIGINAL name must still resolve. Idempotent, so the
                // bounded spelling resolves as well.
                let want = &*kuna_decomp::kuna_symbolnamebound::bound_scope_path(want, "::");
                let candidates: Vec<FunctionEntry> = self
                    .function_entries_canonical()
                    .into_iter()
                    .filter(|entry| {
                        entry.name == *want || entry.aliases.iter().any(|alias| alias == want)
                    })
                    .collect();
                self.one_candidate(selector, candidates)
            }
            EntrySelector::SectionOffset { section, offset } => {
                let sections: Vec<&ObjectSectionLocation> = self
                    .object_sections
                    .iter()
                    .filter(|candidate| candidate.name == *section && *offset < candidate.size)
                    .collect();
                self.resolve_sections(selector, sections, *offset)
            }
            EntrySelector::SectionIndexOffset {
                section_index,
                offset,
            } => {
                let sections: Vec<&ObjectSectionLocation> = self
                    .object_sections
                    .iter()
                    .filter(|candidate| {
                        candidate.index == *section_index && *offset < candidate.size
                    })
                    .collect();
                self.resolve_sections(selector, sections, *offset)
            }
            EntrySelector::Numeric(vma) => {
                let vma = self.thumb_normalized(*vma);
                if self.vma_is_mapped(vma) {
                    return Ok(self.entry_at_or_named(vma));
                }

                let raw_matches: Vec<FunctionEntry> = self
                    .function_entries_canonical()
                    .into_iter()
                    .filter(|entry| {
                        entry.provenance == EntryProvenance::DefinedObject
                            && entry
                                .object_location
                                .as_ref()
                                .is_some_and(|location| location.offset == vma)
                    })
                    .collect();
                if !raw_matches.is_empty() {
                    return self.one_candidate(selector, raw_matches);
                }

                // A genuine undefined symbol is intentionally unmapped but may
                // still be selected explicitly for its external declaration.
                if let Some(entry) = self.find_entry_at(vma) {
                    if entry.provenance == EntryProvenance::UndefinedExternal {
                        return Ok(entry);
                    }
                }
                Err(EntryLookupError::Unmapped {
                    selector: selector.display(),
                    relocatable: !self.object_sections.is_empty(),
                })
            }
        }
    }

    /// Resolve an already-parsed machine address without discarding its address
    /// space. Numeric front-end selectors intentionally use the default code
    /// space and retain relocatable raw-offset compatibility; console address
    /// grammar can instead name processor and overlay spaces explicitly.
    pub fn resolve_address(&self, address: &Address) -> Result<FunctionEntry, EntryLookupError> {
        let mut selector = String::new();
        address
            .print_raw(&mut selector)
            .map_err(|_| EntryLookupError::NotFound {
                selector: format!("0x{:x}", address.get_offset()),
            })?;
        let known = self.entry_at_exact_address(address);
        if self.entry_bytes_mapped(address) {
            return Ok(known.unwrap_or_else(|| {
                let name = self
                    .arch()
                    .symboltab
                    .function_display_name_across_scopes(address)
                    .unwrap_or_else(|| self.arch().name_function(address));
                FunctionEntry {
                    name,
                    addr: address.clone(),
                    aliases: Vec::new(),
                    object_location: None,
                    provenance: EntryProvenance::Mapped,
                    binding: None,
                    size: self.function_extent_at(address.get_offset()),
                }
            }));
        }
        if let Some(entry) = known {
            if entry.provenance == EntryProvenance::UndefinedExternal {
                return Ok(entry);
            }
        }
        Err(EntryLookupError::Unmapped {
            selector,
            relocatable: !self.object_sections.is_empty(),
        })
    }

    fn entry_at_exact_address(&self, address: &Address) -> Option<FunctionEntry> {
        let mut records: Vec<&ProgramSymbol> = self
            .symbols
            .iter()
            .filter(|record| record.addr == *address)
            .collect();
        if records.is_empty() {
            return None;
        }
        records.sort_by(|a, b| {
            entry_name_rank(&a.name)
                .cmp(&entry_name_rank(&b.name))
                .then_with(|| a.name.cmp(&b.name))
        });
        let canonical = records.remove(0);
        let object_record = std::iter::once(canonical)
            .chain(records.iter().copied())
            .find(|record| record.object_location.is_some());
        let provenance = object_record
            .map(|_| EntryProvenance::DefinedObject)
            .unwrap_or_else(|| {
                if canonical.provenance == EntryProvenance::UndefinedExternal
                    || records.iter().any(|record| {
                        record.provenance == EntryProvenance::UndefinedExternal
                    })
                {
                    EntryProvenance::UndefinedExternal
                } else {
                    EntryProvenance::Mapped
                }
            });
        Some(FunctionEntry {
            name: canonical.name.clone(),
            addr: address.clone(),
            aliases: records.iter().map(|record| record.name.clone()).collect(),
            object_location: object_record.and_then(|record| record.object_location.clone()),
            binding: object_record
                .and_then(|record| record.binding.clone())
                .or_else(|| canonical.binding.clone()),
            provenance,
            size: self.function_extent_at(address.get_offset()),
        })
    }

    fn resolve_sections(
        &self,
        selector: &EntrySelector,
        sections: Vec<&ObjectSectionLocation>,
        offset: u64,
    ) -> Result<FunctionEntry, EntryLookupError> {
        if sections.is_empty() {
            return Err(EntryLookupError::NotFound {
                selector: selector.display(),
            });
        }
        let candidates = sections
            .into_iter()
            .map(|section| self.entry_at_or_named(section.vma.wrapping_add(offset)))
            .collect();
        self.one_candidate(selector, candidates)
    }

    fn one_candidate(
        &self,
        selector: &EntrySelector,
        mut candidates: Vec<FunctionEntry>,
    ) -> Result<FunctionEntry, EntryLookupError> {
        candidates.sort_by_key(|entry| entry.addr.get_offset());
        candidates.dedup_by_key(|entry| entry.addr.get_offset());
        match candidates.len() {
            0 => Err(EntryLookupError::NotFound {
                selector: selector.display(),
            }),
            1 => Ok(candidates.remove(0)),
            _ => Err(EntryLookupError::Ambiguous {
                selector: selector.display(),
                candidates,
            }),
        }
    }

    fn entry_at_or_named(&self, vma: u64) -> FunctionEntry {
        if let Some(entry) = self.find_entry_at(vma) {
            return entry;
        }
        let code_space = self
            .arch()
            .manage()
            .get_default_code_space()
            .expect("default code space after bootstrap");
        let addr = Address::new(Rc::clone(code_space), vma);
        let object_location = self.object_location_at(vma);
        let name = self
            .function_named_at(vma)
            .unwrap_or_else(|| self.arch().name_function(&addr));
        let size = self.function_extent_at(vma);
        FunctionEntry {
            name,
            addr,
            aliases: Vec::new(),
            provenance: if object_location.is_some() {
                EntryProvenance::DefinedObject
            } else {
                EntryProvenance::Mapped
            },
            object_location,
            binding: None,
            size,
        }
    }

    fn object_location_at(&self, vma: u64) -> Option<ObjectLocation> {
        self.object_sections.iter().find_map(|section| {
            (vma >= section.vma && vma - section.vma < section.size).then(|| ObjectLocation {
                section_index: section.index,
                section: section.name.clone(),
                offset: vma - section.vma,
            })
        })
    }

    fn vma_is_mapped(&self, vma: u64) -> bool {
        let sections = self.sections();
        if sections.is_empty() {
            return self.vma_bytes_mapped(vma);
        }
        sections
            .iter()
            .any(|&(start, size, _)| vma >= start && vma - start < size)
    }

    /// (kuna, issue #197) Fold the ARM/Thumb mode bit out of `vma`.
    ///
    /// On an ARM-family spec a Thumb function's recorded address carries the mode
    /// bit in bit 0 while the instructions live at the even VMA — the same test
    /// and mask [`crate::project::build_asm`] applies to its labels, and the
    /// console-tier counterpart of the analysis tier's `thumb_masked`.  A no-op on
    /// every other architecture, where an odd code address is genuine.
    fn thumb_normalized(&self, vma: u64) -> u64 {
        if self.description().starts_with("ARM") {
            vma & !1
        } else {
            vma
        }
    }

    /// (kuna) Every section of bytes the load image maps, as `(vma, size, flags)`
    /// triples — `flags` bits per [`kuna_sleigh::loadimage::section_flags`]
    /// (CODE/DATA/READONLY/UNALLOC/NOLOAD).
    ///
    /// Consumes the LoadImage section-iteration API
    /// (`openSectionInfo`/`getNextSection`/`closeSectionInfo`) in one shot, the
    /// same loop shape as `CodeDataAnalysis::runModel` (`codedata.rs`), reached
    /// through the engine's shared loader handle (`translate().loader_rc()`) —
    /// the loader the bootstrap handed to the engine via `set_loader`.
    /// Zero-size records are skipped (the `runModel` convention). Empty when the
    /// loader publishes no section info (e.g. the XML `<binaryimage>` corpus
    /// loader, which keeps the trait's default no-op iteration).
    pub fn sections(&self) -> Vec<(u64, u64, u32)> {
        let loader_rc = self.arch().translate().loader_rc();
        // The iteration methods are `&self` (C++ `const` with a `mutable`
        // cursor; interior mutability here), so a shared borrow suffices.
        let loader = loader_rc.borrow();
        let mut out = Vec::new();
        let mut secinfo = LoadImageSection::default();
        loader.open_section_info();
        loop {
            // getNextSection fills `secinfo` and returns whether ANOTHER record
            // follows — so the record is consumed BEFORE the loop-exit check
            // (the runModel shape). An empty section list leaves the default
            // record (size 0), which the skip below drops.
            let moresections = loader.get_next_section(&mut secinfo);
            if secinfo.size != 0 {
                out.push((secinfo.address.get_offset(), secinfo.size, secinfo.flags));
            }
            if !moresections {
                break;
            }
        }
        loader.close_section_info();
        out
    }

    /// (kuna, `zero-function-sizes-make`) The loadable segments as
    /// `(vma, size, flags)` triples — the coarser mapping unit under
    /// [`Self::sections`], with the same [`section_flags`] vocabulary (a
    /// segment's CODE bit is its execute permission).
    ///
    /// The answer for an image whose section table is absent or unusable, where
    /// `sections()` is empty and every section-keyed question loses its
    /// container. Empty for a loader that does not model segments (the XML
    /// `<binaryimage>` corpus loader, a relocatable object) — the trait method
    /// defaults to it.
    pub fn segments(&self) -> Vec<(u64, u64, u32)> {
        self.arch().translate().loader_rc().borrow().get_segments()
    }

    /// (kuna) Whether the load image can actually read bytes at `addr` — i.e.
    /// whether an entry there could have a body at all.
    ///
    /// A function entry does not imply mapped bytes. A relocatable object's
    /// **undefined externals** (`puts`, `CellClass::Cell_Coord`) are bound to
    /// synthetic addresses in an extern area above the laid-out sections
    /// ([`kuna_analysis::loader::reloc_object`]) so that a call to one renders by
    /// name; nothing is mapped there, and nothing ever will be, because the
    /// definition lives in a different translation unit. The same is true of a
    /// PE import pointer slot. Asking the lifter for such a function's body can
    /// only produce "Unable to load N bytes", so the callers that would have
    /// asked check here first and report the entry for what it is.
    ///
    /// Probes with a one-byte `load_fill` — the exact question, rather than a
    /// section-flag approximation of it, so an address that *is* mapped but sits
    /// outside any CODE section (packed code in `.data`, a hand-picked `--addr`)
    /// still decompiles as it does today.
    pub fn entry_bytes_mapped(&self, addr: &Address) -> bool {
        let loader_rc = self.arch().translate().loader_rc();
        let mut loader = loader_rc.borrow_mut();
        let mut probe = [0u8; 1];
        loader.load_fill(&mut probe, addr).is_ok()
    }

    /// [`Self::entry_bytes_mapped`] for a caller holding a bare code-space VMA.
    /// `false` when the program has no code space to resolve the VMA against.
    pub fn vma_bytes_mapped(&self, vma: u64) -> bool {
        let Some(space) = self.arch().manage().get_default_code_space() else {
            return false;
        };
        self.entry_bytes_mapped(&Address::new(Rc::clone(space), vma))
    }

    /// (kuna) Disassemble the single machine instruction at code-space VMA
    /// `vma`, returning `(length, mnemonic, body)` — the one-instruction form
    /// of the `disassemble` console command (`IfcPrintdisasm`), for a caller
    /// that walks a range itself (advance by `length` per step).
    ///
    /// This allocating convenience API is retained for callers decoding an
    /// occasional instruction. Whole-image walks should use
    /// [`Self::disassemble_at_into`] so the mnemonic and body allocations are
    /// retained between instructions.
    pub fn disassemble_at(&self, vma: u64) -> KunaResult<(int4, String, String)> {
        let mut mnem = String::new();
        let mut body = String::new();
        let length = self.disassemble_at_into(vma, &mut mnem, &mut body)?;
        Ok((length, mnem, body))
    }

    /// (kuna) Disassemble one instruction into caller-owned mnemonic and body
    /// buffers, retaining their allocations across calls.
    ///
    /// Both buffers are cleared before every decode, including failed decodes.
    /// An undecodable or unmapped address surfaces as the translator's `Err`
    /// (C++ `BadDataError`/`DataUnavailError`).
    pub fn disassemble_at_into(
        &self,
        vma: u64,
        mnem: &mut String,
        body: &mut String,
    ) -> KunaResult<int4> {
        mnem.clear();
        body.clear();
        let code_space = Rc::clone(
            self.arch()
                .manage()
                .get_default_code_space()
                .ok_or_else(|| KunaError::lowlevel("no default code space"))?,
        );
        let addr = Address::new(code_space, vma);
        self.arch().translate().print_assembly_into(&addr, mnem, body)
    }

    /// (kuna) The `kuna_wasm` per-function `kind` classification probe: lift
    /// the single instruction at code-space VMA `vma` to p-code (a one-shot
    /// [`PcodeEmit`](kuna_sleigh::translate::PcodeEmit) sink against the
    /// translator — the pcode analogue of [`Self::disassemble_at`]'s
    /// `AssemblyEmit` dance) and test it for the two **lone-jump** shapes a
    /// thunk/PLT-stub entry decompiles from:
    ///
    /// * `Some(Some(target))` — the p-code ends in an unconditional `BRANCH`
    ///   to a non-constant-space address (a real code-space target, not a
    ///   p-code-relative one — `flow.rs`'s constant-space-`in0` rule) and
    ///   contains no other flow op (`CALL`/`CBRANCH`/`RETURN`/...): a direct
    ///   lone jump; `target` is the destination offset.
    /// * `Some(None)` — the p-code ends in `BRANCHIND` with no other flow op
    ///   (address-computation ops like `LOAD`/`INT_ADD`/`COPY` before it are
    ///   fine): an indirect lone jump (the `jmp [GOT]` PLT shape).
    /// * `None` — anything else, including a decode error or an unmapped
    ///   address (conservative: the probe never panics).
    pub fn lone_jump_target(&self, vma: u64) -> Option<Option<u64>> {
        let code_space = Rc::clone(self.arch().manage().get_default_code_space()?);
        let addr = Address::new(code_space, vma);
        let mut emit = OneShotPcodeEmit::default();
        // Advisory probe: contain a decode `Err` AND any translator panic on
        // exotic bytes to `None` (the classification falls back to "func").
        let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.arch().translate().one_instruction(&mut emit, &addr)
        }));
        match decoded {
            Ok(Ok(_len)) => {}
            _ => return None,
        }
        let (last_opc, last_in0) = emit.ops.last()?;
        // Exactly ONE flow op, and it is the last op — anything richer (a
        // conditional, a call, a fall-through mid-branch) is not a lone jump.
        let flow_ops = emit
            .ops
            .iter()
            .filter(|(opc, _)| {
                matches!(
                    opc,
                    OpCode::CPUI_BRANCH
                        | OpCode::CPUI_CBRANCH
                        | OpCode::CPUI_BRANCHIND
                        | OpCode::CPUI_CALL
                        | OpCode::CPUI_CALLIND
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_RETURN
                )
            })
            .count();
        if flow_ops != 1 {
            return None;
        }
        match last_opc {
            OpCode::CPUI_BRANCH => {
                let in0 = last_in0.as_ref()?;
                let space = in0.space.as_ref()?;
                if space.get_type() == kuna_base::space::spacetype::IPTR_CONSTANT {
                    None // p-code-relative branch, not a code-space target
                } else {
                    Some(Some(in0.offset))
                }
            }
            OpCode::CPUI_BRANCHIND => Some(None),
            _ => None,
        }
    }

    /// (kuna) Add the fixed addresses the instruction at `vma` names — the
    /// constant locations it reads and the constant addresses it branches or
    /// calls to — to `into`.
    ///
    /// The projection a listing needs to tell a literal-pool word from an
    /// instruction. A pool word is proved by the code around it (some
    /// instruction spells its address out and reads it) and disproved the same
    /// way (a branch names it as a label), so a caller walking a range
    /// accumulates both facts into one [`FixedRefs`] as it goes. What is
    /// harvested from the p-code is [`FixedRefs::harvest`].
    ///
    /// Adds nothing (never an error, never a panic) for an address that does not
    /// decode or a program with no code space — this is advisory evidence, and a
    /// caller that gets none simply learns nothing.
    pub fn add_fixed_refs_at(&self, vma: u64, into: &mut FixedRefs) {
        let Some(code_space) = self.arch().manage().get_default_code_space().cloned() else {
            return;
        };
        let addr = Address::new(code_space, vma);
        into.filled = 0;
        // Advisory probe: contain a decode `Err` AND any translator panic on
        // exotic bytes to "no evidence", exactly as `lone_jump_target` does.
        let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.arch().translate().one_instruction(into, &addr)
        }));
        let Ok(Ok(len)) = decoded else {
            return;
        };
        let data_space = self.arch().manage().get_default_data_space().cloned();
        into.harvest(data_space.as_ref(), vma.wrapping_add(len as u64));
    }

    /// This allocating convenience API is retained for occasional reads.
    /// Whole-image walks should use [`Self::read_bytes_into`] so the byte-buffer
    /// allocation is retained between reads.
    pub fn read_bytes(&self, vma: u64, size: usize) -> Option<Vec<u8>> {
        let mut bytes = Vec::new();
        self.read_bytes_into(vma, size, &mut bytes).then_some(bytes)
    }

    /// (kuna) Read `size` raw image bytes at code-space VMA `vma` into a
    /// caller-owned buffer, retaining its allocation across calls.
    ///
    /// Returns `false` and leaves `bytes` empty when the range is not backed by
    /// the image (e.g. `.bss`, an unmapped address, or a `size` beyond the C++
    /// `int4` read contract).
    pub fn read_bytes_into(&self, vma: u64, size: usize, bytes: &mut Vec<u8>) -> bool {
        bytes.clear();
        if size > i32::MAX as usize {
            return false;
        }
        let Some(code_space) = self.arch().manage().get_default_code_space().cloned() else {
            return false;
        };
        let addr = Address::new(code_space, vma);
        bytes.resize(size, 0);
        let loader_rc = self.arch().translate().loader_rc();
        let loaded = loader_rc.borrow_mut().load_fill(bytes, &addr).is_ok();
        if !loaded {
            bytes.clear();
        }
        loaded
    }

    /// (kuna) Every **named global data symbol** mapped into the engine's
    /// default data space, as `(name, vma, type_size)` — the label set a
    /// whole-binary exporter cross-references against the `dat_<addr>` tokens
    /// the C printer generates for unnamed data.
    ///
    /// Enumerates the global scope (`Database::get_global_scope`) via
    /// `scope_space_symbol_specs` over the default data space (`ram` on every
    /// vendored processor; Harvard-style splits follow the data space).
    /// FunctionSymbols live in the SAME per-space rangemap (`add_function` maps
    /// them at their entry address) and the specs tuple's `uint4` is the
    /// varnode `flags` word — which does NOT distinguish them (functions carry
    /// `namelock|typelock`, but so can data) — so functions are excluded by
    /// their datatype instead: a FunctionSymbol's type is exactly the
    /// TYPE_CODE base, and every `metatype == TYPE_CODE` spec is dropped.
    /// This also drops the deliberately code-typed untyped Data placeholders
    /// some gated analysis passes install (size-1, no real extent — useless as
    /// data labels); typed data (DWARF `undefined<N>` globals, `char[N]`
    /// string symbols) is kept. `type_size` is the mapped datatype's byte size.
    /// Sorted by VMA.
    pub fn global_data_symbols(&self) -> Vec<(String, u64, i64)> {
        use kuna_decomp::dtype::type_metatype;
        let arch = self.arch();
        let Some(scope) = arch.symboltab.get_global_scope() else {
            return Vec::new();
        };
        let Some(data_space) = arch.manage().get_default_data_space() else {
            return Vec::new();
        };
        let space_index = data_space.get_index() as usize;
        let mut out: Vec<(String, u64, i64)> = arch
            .symboltab
            .scope_space_symbol_specs(scope, space_index)
            .into_iter()
            .filter(|(_, ct, _, _)| ct.get_metatype() != type_metatype::TYPE_CODE)
            .map(|(name, ct, addr, _)| (name, addr.get_offset(), i64::from(ct.get_size())))
            .collect();
        out.sort_by(|a, b| (a.1, &a.0).cmp(&(b.1, &b.0)));
        out
    }

    /// (kuna) Is a (possibly `::`-scoped) symbol of full name `full_name` present in
    /// the engine symbol table? Resolves the namespace path (`Box::vftable` → scope
    /// `Box`, basename `vftable`) read-only, then queries that scope by basename.
    ///
    /// Unlike [`Self::lookup_symbol`] (which only sees the console `register_symbol`
    /// name→addr map, populated by the Function-symbol commit arm), this reaches the
    /// `Database` directly, so it sees the **Data** symbols the analysis commit
    /// installs via `add_symbol_mapped` (the RTTI `<Class>::vftable` /
    /// `<Class>::RTTI_*` labels the `rtti` pass emits). The verification hook for
    /// the gated MSVC-RTTI recovery e2e.
    pub fn has_symbol_named(&self, full_name: &str) -> bool {
        let db = &self.arch().symboltab;
        let (scope, base) = db.resolve_scope_from_symbol_name(full_name, "::", None);
        match scope {
            Some(s) => !db.query_by_name(s, &base).is_empty(),
            None => false,
        }
    }

    /// (kuna) The basename of the FunctionSymbol installed at code-space VMA `vma`, or
    /// `None` if no function is mapped there. Resolves across scopes
    /// (`find_function_across_scopes`, the no-return/FID arm's resolver), so it sees a
    /// namespaced virtual-method function (e.g. `Box::vfunc_0` in scope `Box`).
    ///
    /// The verification hook for the MSVC-RTTI **vftable** e2e (R3): a vtable slot's
    /// target VA should now resolve to a named virtual method — the virtual dispatch
    /// `(**(code **)*p)()` points at this function instead of a bare `DAT_*`.
    pub fn function_named_at(&self, vma: u64) -> Option<String> {
        let code_space = Rc::clone(self.arch().manage().get_default_code_space()?);
        let addr = Address::new(code_space, vma);
        let db = &self.arch().symboltab;
        let (sid, _) = db.find_function_across_scopes(&addr)?;
        Some(db.symbol(sid).get_name().to_string())
    }

    /// Read the binaryimage's loader symbols into the symbol table as
    /// FunctionSymbols (C++ `Architecture::readLoaderSymbols`, `architecture.cc:347`,
    /// called by `testfunction.cc:160` / `consolemain.cc:104` after load).
    ///
    /// Each loader symbol (name → entry address, already in `self.symbols`) becomes
    /// a `FunctionSymbol` in its (namespace-resolved) scope, so a CALL to that
    /// entry address resolves to the callee's name at flow-analysis time
    /// (`FlowInfo::queryCall`).  Idempotent: a symbol whose function is already in
    /// the table is skipped (the C++ `addFunction` no-ops on an existing match via
    /// `queryFunction`).
    pub fn read_loader_symbols(&mut self) -> KunaResult<()> {
        let type_code = self.arch().types().get_type_code()?;
        let min_size = self.arch().min_funcsymbol_size;
        let num_spaces = self.arch().manage().num_spaces();
        // Clone the (name, addr) pairs so the borrow of `self.arch_mut()` below
        // does not overlap `self.symbols`.
        let records: Vec<(String, Address)> =
            self.symbols.iter().map(|s| (s.name.clone(), s.addr.clone())).collect();
        let arch = self.arch_mut();
        for (name, addr) in records {
            let (scope, basename) = arch
                .symboltab
                .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
            // C++ queryFunction: skip if a function already maps this address.
            if arch.symboltab.find_function(scope, &addr).is_some() {
                continue;
            }
            arch.symboltab.add_function(scope, &addr, &basename, min_size, type_code.clone())?;
        }
        Ok(())
    }

    /// Register a console-created function symbol (the `map function` hook): make
    /// `name`->`addr` resolvable by `load function <name>`.  C++ `Scope::addFunction`
    /// installs the symbol in the symbol table; the kuna console additionally needs
    /// the name->address entry so the (binaryimage-symbol-backed) `load function`
    /// path can find a function the user mapped by hand.  Replaces any prior entry
    /// of the same name.
    pub fn register_symbol(&mut self, name: &str, addr: Address) {
        // (kuna `symbolnamebound`) Same bound as the loader stream above, so an
        // analysis-discovered or hand-mapped name agrees with the scope path the
        // symbol table nests it under.
        let name = &*kuna_decomp::kuna_symbolnamebound::bound_scope_path(name, "::");
        self.symbols.retain(|s| s.name != name);
        let object_location = self.object_location_at(addr.get_offset());
        self.symbols.push(ProgramSymbol {
            name: name.to_string(),
            addr,
            provenance: if object_location.is_some() {
                EntryProvenance::DefinedObject
            } else {
                EntryProvenance::Mapped
            },
            object_location,
            binding: None,
        });
    }

    /// Record a caller-declared byte extent for the function entered at `vma`
    /// (`declared_extents`); `size <= 0` clears it back to unbounded.
    pub fn declare_extent(&mut self, vma: u64, size: int4) {
        if size > 0 {
            self.declared_extents.insert(vma, size);
        } else {
            self.declared_extents.remove(&vma);
        }
    }

    /// The caller-declared byte extent of the function entered at `vma`, or
    /// [`UNBOUNDED_SIZE`] when none was declared — the value every load of that
    /// entry passes as the analysis-size bound.
    pub fn declared_extent(&self, vma: u64) -> int4 {
        self.declared_extents.get(&vma).copied().unwrap_or(UNBOUNDED_SIZE)
    }

    /// Is any function extent declared for this program?  Cheap enough to gate a
    /// per-entry lookup in the inventory bulk pass.
    pub fn has_declared_extents(&self) -> bool {
        !self.declared_extents.is_empty()
    }

    // --- the `--assert` override plane (see `crate::assertions`) -------------

    /// Install the caller's assertions.  One outcome slot is reserved per
    /// directive so the report keeps the caller's own order however the
    /// directives are later dispatched.
    pub fn set_assertions(&mut self, directives: Vec<crate::assertions::Directive>) {
        self.assertion_outcomes = vec![None; directives.len()];
        self.assertions = directives;
    }

    /// The installed assertions, in the order they were given.
    pub fn assertions(&self) -> &[crate::assertions::Directive] {
        &self.assertions
    }

    /// Record what became of the `i`-th directive.
    pub fn set_assertion_outcome(&mut self, i: usize, outcome: crate::assertions::Outcome) {
        if let Some(slot) = self.assertion_outcomes.get_mut(i) {
            *slot = Some(outcome);
        }
    }

    /// The per-directive report, one row per installed assertion.
    pub fn assertion_outcomes(&self) -> Vec<crate::assertions::Outcome> {
        self.assertions
            .iter()
            .zip(self.assertion_outcomes.iter())
            .map(|(directive, outcome)| {
                outcome.clone().unwrap_or_else(|| crate::assertions::unclaimed(directive))
            })
            .collect()
    }

    /// Park a prototype for `func` (an `assert prototype` directive).
    pub fn set_pending_prototype(
        &mut self,
        func: &str,
        pieces: kuna_decomp::fspec::PrototypePieces,
    ) {
        self.pending_prototypes.insert(func.to_string(), pieces);
    }

    /// The prototype parked for `func`, if any.
    pub fn pending_prototype(&self, func: &str) -> Option<&kuna_decomp::fspec::PrototypePieces> {
        self.pending_prototypes.get(func)
    }

    /// Declare a function at `addr` — the in-process twin of the console
    /// `map function <addr> [name]` command (`IfcMapfunction`), plus the extent
    /// the console form cannot express on its own.
    ///
    /// Installs the `FunctionSymbol` (so a CALL here resolves to `name`),
    /// registers the name→address entry (so `load function <name>` and the
    /// whole-binary enumeration both see it), and records `size` as the entry's
    /// declared extent (`0` = unbounded).
    ///
    /// An address that already carries a function symbol is RENAMED rather than
    /// given a second one, but only when the caller named it: the symbol table
    /// keys a function by address, so adding a second symbol there would leave
    /// two names competing for one entry.  With no explicit name, an already-named
    /// address keeps its name and only the extent is recorded.
    pub fn declare_function(
        &mut self,
        addr: Address,
        name: Option<&str>,
        size: int4,
    ) -> KunaResult<String> {
        let explicit = name.map(str::to_string).filter(|n| !n.is_empty());
        let name = match &explicit {
            Some(n) => n.clone(),
            None => self.arch().name_function(&addr),
        };
        let type_code = self.arch().types().get_type_code()?;
        let min_size = self.arch().min_funcsymbol_size;
        let num_spaces = self.arch().manage().num_spaces() as int4;
        let arch = self.arch_mut();
        let (scope, basename) =
            arch.symboltab.find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
        let name = match arch.symboltab.find_function_across_scopes(&addr) {
            Some((sym, _)) => match &explicit {
                Some(_) => {
                    arch.symboltab.rename_symbol(sym, &basename)?;
                    name
                }
                // Nothing asked for: keep the name the image or the analysis gave
                // this entry rather than overwriting it with `sub_<addr>`.
                None => arch.symboltab.symbol(sym).get_display_name().to_string(),
            },
            None => {
                arch.symboltab.add_function(scope, &addr, &basename, min_size, type_code)?;
                name
            }
        };
        let vma = addr.get_offset();
        self.register_symbol(&name, addr);
        self.declare_extent(vma, size);
        Ok(name)
    }

    /// (kuna) Build the `map addr`-shaped stack-symbol specs for the DWARF stack
    /// LOCALS parked on the function whose entry VMA is `func_addr` (DWARF subtask 3).
    ///
    /// Returns `(name, type, stack_Address, flags)` tuples in the exact shape
    /// `Funcdata::seed_mapped_symbols` consumes — `IfcDecompile` appends them to the
    /// `mapped_symbols` it threads into the decompile drive, so each parked DWARF
    /// local is re-seeded into the rebuilt `Funcdata`'s `ScopeLocal` as a
    /// `typelock|namelock` stack symbol (the typelock keeps the DWARF type through
    /// propagation; the namelock keeps the DWARF name). The stack `Address` is built
    /// here against the live stack space (`getStackSpace`), wrapping the signed
    /// `stack_offset` to the space's unsigned offset (the same convention as the
    /// console `map addr s<off>` directive — e.g. copytrim.xml's
    /// `map addr s0xffffffffffffffe4`). Empty when the function has no parked locals
    /// or the architecture has no stack space.
    pub fn dwarf_locals_for(
        &self,
        func_addr: u64,
    ) -> Vec<(String, Rc<kuna_decomp::dtype::Datatype>, Address, kuna_base::types::uint4)> {
        use kuna_decomp::varnode::varnode_flags;
        let Some(stack) = self.arch().manage().get_stack_space() else {
            return Vec::new();
        };
        let flags = varnode_flags::typelock | varnode_flags::namelock;
        self.dwarf_locals
            .iter()
            .filter(|(addr, _, _, _)| *addr == func_addr)
            .map(|(_, name, ty, off)| {
                // Wrap the signed stack offset to the stack space's unsigned address
                // (negative locals live at the high end of the unsigned range, the
                // `map addr s0xffff...` convention).
                let waddr = stack.wrap_offset(*off as u64);
                (
                    name.clone(),
                    Rc::clone(ty),
                    Address::new(Rc::clone(stack), waddr),
                    flags,
                )
            })
            .collect()
    }

    /// (kuna) Commit the stashed per-pass analysis facts, gated by the per-pass
    /// `--option <id> on|off` enable flags — the deferred half of the analysis
    /// boundary (conflict #4). Called from `IfcReadSymbols` (`read symbols`), which
    /// runs AFTER the CLI's `option` lines, so a disabled pass's facts are
    /// dropped here rather than committed.
    ///
    /// Drains the stash (so a second `read symbols` does not re-commit), merges
    /// only the **enabled** passes' outputs in pass order, and commits the merged
    /// [`AnalysisOutput`] via [`commit_analysis_output`]. A no-op when nothing is
    /// stashed (the XML datatest path — parity is structurally untouched).
    ///
    /// (kuna, PR6) This is also the **deferred Listing build** point: the Listing
    /// is gated on `--option listing on`, a flag the CLI sets after `load file`,
    /// so the Listing — and any pass that reads it (e.g. discovered-no-return) —
    /// is built/run HERE (when the flag is finally in effect), not at load. When
    /// `arch.analysis_listing` is on, the stashed image bytes are re-parsed, the
    /// Listing is built, and the enabled Listing-consumer passes run; their
    /// defensively re-gated facts merge into the same `merged` output committed
    /// below. With Listing off, this whole block is skipped.
    pub fn commit_pending_analysis(&mut self) -> KunaResult<()> {
        if self.pending_analysis.is_empty() && self.loader_data_objects.is_empty() {
            // Drop the deferred-Listing stash too: nothing to commit against, and
            // a session with no analysis tier (XML path) must not build a Listing.
            self.analysis_image = None;
            return Ok(());
        }
        let pending = std::mem::take(&mut self.pending_analysis);
        let Some(code_space) = self.analysis_code_space.take() else {
            // No code space captured (should not happen on the ELF path); nothing
            // to commit against.
            self.analysis_image = None;
            return Ok(());
        };
        // The addresses the load-time Known passes flagged no-return / call-fixup'd
        // (read off the pending split before it is consumed) — seed metadata the
        // deferred Listing build hands to the discovered-no-return consumer so it
        // skips already-modeled callees and seeds the fixpoint's terminal set.
        let noreturn_seed_addrs: Vec<u64> = pending
            .iter()
            .flat_map(|(_, out)| out.noreturn.iter().map(|f| f.addr))
            .collect();
        // Filter by the per-pass enable flags (default-on, set by the user's
        // `--option <id> on|off`), then merge the survivors in pass order.
        let mut merged = kuna_analysis::pass::AnalysisOutput::default();
        for (id, out) in pending {
            if analysis_pass_enabled(self.arch(), id) {
                merged.merge(out);
            }
        }
        // (kuna, PR6 + operand_refs) Deferred decode-driven passes, run at the
        // commit point because they decode through the engine `Translate` whose
        // program loadimage is only attached AFTER the load-time pass list
        // (`set_loader` in `bootstrap_from_object`). Each is gated on its own
        // `--option` flag (now in effect). Default-off (both) ⇒ skipped ⇒ zero cost.
        // Real-object path only (the XML path stashes no image). The stash is consumed
        // once and shared by both deferred passes.
        let want_listing = self.arch().analysis_listing;
        let want_fast_funcdisc = self.arch().analysis_fast_funcdisc;
        let want_operand_refs = self.arch().analysis_operand_refs;
        if (want_listing || want_fast_funcdisc || want_operand_refs)
            && self.analysis_image.is_some()
        {
            if let Some((path, bytes)) = self.analysis_image.take() {
                // A throwaway loadimage just to satisfy the pass contracts (their
                // `image` arg is unused — the decode reads through `translate`); a
                // parse failure makes the deferred step a graceful no-op.
                if let Ok(image) =
                    kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes_silent(
                        &path, &bytes,
                    )
                {
                    // Deferred Listing build + consumer/fast-inventory run, gated
                    // on the matching option. The call-fixup seed list is the names
                    // the load-time pass flagged resolved to addresses via the
                    // (already installed) symbol table.
                    if want_listing || want_fast_funcdisc {
                        let arch = self.arch();
                        // The call-fixup seed list is empty: a fixup'd callee is also
                        // skipped via the consumer's no-return-disc `function_at(..)`
                        // checks, and there is no fixup-address index here. The
                        // no-return seeds (above) are the load-bearing skip set.
                        let consumer_out = kuna_analysis::passes::run_listing_consumers(
                            &bytes,
                            &image,
                            arch,
                            arch.translate(),
                            &noreturn_seed_addrs,
                            &[],
                        );
                        for (id, out) in consumer_out {
                            if analysis_pass_enabled(self.arch(), id) {
                                merged.merge(out);
                            }
                        }
                    }
                    // Deferred scalar/operand reference-markup pass, gated on the
                    // operand_refs flag (the kuna analog of Ghidra's
                    // ScalarOperandAnalyzer). Independent of the Listing tier — it
                    // does its own linear decode (the Listing never populates the
                    // data references it needs). Its facts go through the existing
                    // string/readonly commit arms.
                    if want_operand_refs {
                        let out = kuna_analysis::passes::run_operand_refs(
                            &bytes,
                            &image,
                            self.arch(),
                        );
                        merged.merge(out);
                    }
                }
            }
        } else {
            // Both deferred passes off: drop the stash (no deferred build).
            self.analysis_image = None;
        }
        // (kuna, `fdeinterior`) Reject every discovered entry that falls strictly
        // inside a single-function `.eh_frame` FDE body. Applied HERE, on the fully
        // merged set, so it covers the deferred Listing consumers (`aif`'s gap
        // starts) as well as the load-time oracles (`eh_frame_full`'s landing pads,
        // the prologue patterns). `fde_bodies` is empty unless the `fdeinterior`
        // gate let its pass through above, so `off` is a byte-identical no-op.
        let fde_bodies = std::mem::take(&mut merged.fde_bodies);
        kuna_analysis::entry::kuna_fdeinterior::suppress_interior_entries(
            &mut merged.entries,
            &fde_bodies,
        );
        commit_analysis_output(self, &code_space, merged)
    }
}

/// (kuna) Read a `kuna_analysis` pass's per-run enable flag off the
/// [`Architecture`] by the pass's `AnalysisPass::id`. An unknown id defaults to
/// enabled (a new pass with no registered gate still runs — fail-open, additive).
fn analysis_pass_enabled(arch: &Architecture, pass_id: &str) -> bool {
    match pass_id {
        "noreturn_known" => arch.analysis_noreturn_known,
        // (kuna) PE import-call binding — the IAT-slot `externref` paint + the
        // Win32 no-return API names, both computed at LOAD and COMMITTED only when
        // this gate is on. Off ⇒ a PE renders exactly as before.
        "peimportcall" => arch.analysis_peimportcall,
        "libproto" => arch.analysis_libproto,
        // (kuna) The measured libc signature extension — the ~200 prototypes the
        // 27-entry base table does not carry, seeded onto IMPORTED names only. The
        // facts are computed at LOAD and COMMITTED only when this gate is on, so
        // `off` renders exactly what the base table alone renders.
        "libcsigs" => arch.analysis_libcsigs,
        "strings" => arch.analysis_strings,
        "entry_disc" => arch.analysis_entry_disc,
        // (kuna) `.eh_frame` LSDA landing-pad discovery (GccExceptionAnalyzer) — a
        // standalone stashed pass whose facts (the exception landing pads) are
        // computed at LOAD but COMMITTED only when this gate is on. Default-off
        // (output-changing: adds entries), so a default run never commits them and
        // the discovery set is byte-identical to FDE-pcBegin-only.
        "eh_frame_full" => arch.analysis_eh_frame_full,
        // (kuna) PE CRT entry-function prototype recovery — a standalone stashed pass
        // whose one prototype is computed at LOAD but COMMITTED only when this gate
        // is on. Default-ON; off renders the `void(void)` form exactly.
        "entrymainproto" => arch.analysis_entrymainproto,
        // (kuna) Mach-O `LC_MAIN` entry naming + prototype — a standalone stashed
        // pass whose one name and one prototype are computed at LOAD but COMMITTED
        // only when this gate is on. Default-ON; off renders the `sub_<addr>` /
        // `void(void)` form exactly.
        "machomain" => arch.analysis_machomain,
        // (kuna) `.eh_frame` FDE-interior entry suppression — the pass reports the
        // single-function FDE bodies and the commit rejects any discovered entry
        // strictly inside one. Default-ON; with the gate off the fact stream is
        // dropped here and the discovery set is exactly what it was before.
        "fdeinterior" => arch.analysis_fdeinterior,
        // (kuna) The widened Cortex-M vector-table oracle — a standalone stashed
        // pass whose handler seeds + Thumb region paint are computed at LOAD but
        // COMMITTED only when this gate is on. Default-off (output-changing: adds
        // entries), so a default run never commits them and the discovery set is
        // byte-identical to the shipped (e_entry-matching) signature.
        "cortexmvectors" => arch.analysis_cortexmvectors,
        "ptrentry" => arch.analysis_ptrentry,
        // (kuna) ARM literal-pool inference — the additive pool-end entries are
        // emitted under this id so the commit gate mirrors the pre-invocation check
        // in `run_listing_consumers`. The subtractive half rides the `aif` stream it
        // filters, which the `aif` arm above already gates.
        "poolentry" => arch.analysis_poolentry,
        // (kuna) The full byte-pattern function-start pass — default-OFF
        // (output-changing). The `_ => true` fail-open below would otherwise run it
        // by default, so this explicit arm reading the (default-false)
        // `analysis_funcstart_patterns` flag is load-bearing for the default-off
        // contract.
        "funcstart_patterns" => arch.analysis_funcstart_patterns,
        // (kuna) The recursive-descent discovery commit — promotes the Listing walk's
        // followed CALL targets to functions. Coupled to the same flag as the prologue
        // seeds that feed it (default-OFF; DIV-20 turns it on for non-x86-64 on the
        // decompile-all surface). x86-64 keeps it off ⇒ byte-identical there.
        "funcdisc_recursive" => arch.analysis_funcstart_patterns,
        "arm_markers" => arch.analysis_arm_markers,
        "mips_gp" => arch.analysis_mips_gp,
        "mips_isa" => arch.analysis_mips_isa,
        "dwarf" => arch.analysis_dwarf,
        // (kuna `cppsig`) The whole-pass gate; WHICH certainty tier survives is a
        // second decision, made on the mode in `commit_analysis_output`.
        "cppsig" => arch.analysis_cppsig.enabled(),
        // Explicit (NOT the fail-open `_ => true` default): the source-line pass is
        // default-OFF (it changes the output), so it must be registered here to be
        // gated by the `analysis_dwarf_lines` flag rather than running by default.
        "dwarf_lines" => arch.analysis_dwarf_lines,
        "callfixup" => arch.analysis_callfixup,
        "addrtable" => arch.analysis_addrtable,
        "operand_refs" => arch.analysis_operand_refs,
        "listing" => arch.analysis_listing,
        "fast_funcdisc" => arch.analysis_fast_funcdisc,
        "noreturn_disc" => arch.analysis_noreturn_disc,
        // (kuna, GH-312) The positive-evidence-only tally has no fact stream of its
        // own — it shapes which callees `noreturn_disc` concludes, which the arm
        // above already gates. Registered here so the fail-open `_ => true` never
        // silently re-enables a pass id that does not exist.
        "noreturn_discstrict" => arch.analysis_noreturn_discstrict,
        "noreturn_propagate" => arch.analysis_noreturn_propagate,
        "fid" => arch.analysis_fid,
        // (kuna) MSVC RTTI / vftable recovery — a standalone load-time pass whose
        // class-name + RTTI_* labels are computed at LOAD but COMMITTED only when
        // this gate is on. Default-OFF (output-changing: adds named data symbols),
        // PE-only (the pass + its passes_for registration both self-gate on PE), so
        // a default run / every non-PE binary commits nothing here.
        "rtti" => arch.analysis_rtti,
        // (kuna, NOVEL) Itanium (GCC/Clang) RTTI + vtable recovery — the class
        // names, typeinfo/vtable labels and per-slot virtual-method names are
        // computed at LOAD but COMMITTED only when this gate is on. Default-OFF
        // (output-changing: adds named data + function symbols), ELF-only (the pass
        // + its passes_for registration both self-gate on ELF), so a default run /
        // every non-ELF binary commits nothing here.
        "itaniumrtti" => arch.analysis_itaniumrtti,
        "aif" => arch.analysis_aif,
        // (kuna, GH-299) The AIF gap-cursor aligned slide has no fact stream of its
        // own — it shapes the `aif` accept list inside `run_aif`, which the `aif` arm
        // above already gates. Registered here so the fail-open `_ => true` never
        // silently re-enables a pass id that does not exist.
        "aifstrict" => arch.analysis_aifstrict,
        // (kuna, GH-313) The AIF accept corroboration test — like `aifstrict` it has
        // no fact stream of its own, shaping the `aif` accept list inside `run_aif`.
        // Registered here so the fail-open `_ => true` never silently re-enables a
        // pass id that does not exist.
        "aifcorroborate" => arch.analysis_aifcorroborate,
        // (kuna) Tail-call function-entry recovery — default-OFF (discovers more
        // functions, so it changes emitted C by construction). Listing consumer;
        // the live gate is the pre-invocation check in `run_listing_consumers`.
        "tailcallentry" => arch.analysis_tailcallentry,
        "gopclntab" => arch.analysis_gopclntab,
        // (kuna) Mach-O Objective-C metadata recovery — default-OFF (output-changing:
        // renames IMP functions + adds class/selector symbols). Registered here so it
        // is gated by `analysis_objc` rather than running by the fail-open default.
        "objc" => arch.analysis_objc,
        // (kuna) PE PDB metadata recovery — default-OFF (output-changing: renames
        // stripped functions to their real PDB names). PE-only + externally gated on
        // a fingerprint-matching `.pdb` (the `kuna_pdb_path` env var). Registered
        // here so it is gated by `analysis_pdb` rather than running by the fail-open
        // default.
        "pdb" => arch.analysis_pdb,
        _ => true,
    }
}

/// The FID label gate: is `name` an engine-generated placeholder a FID match may
/// overwrite (never a real `.symtab`/DWARF/imported name)?
///
/// kuna names an un-symboled function `sub_<addr>` (`kuna_function_name`, the
/// default angr-style) or `func_<addr>` (`Architecture::name_function`, the upstream
/// style); a Ghidra-imported binary uses `FUN_<addr>`. Those are the only names FID
/// renames. The kuna analog of Ghidra's `!alwaysApplyFidLabels &&
/// hasUserOrImportedSymbols` gate — on a stripped binary every function is a
/// placeholder ⇒ FID fires; a function with a real name is left alone.
fn is_generic_placeholder_name(name: &str) -> bool {
    name.starts_with("sub_")
        || name.starts_with("func_")
        || name.starts_with("FUN_")
        || name.starts_with("LAB_")
}

/// (kuna, issue #197) Is `name` a synthesized STRUCTURAL name rather than one the
/// binary actually carries?
///
/// `_INIT_<i>` / `_FINI_<i>` / `_DT_INIT` / `_DT_FINI` are minted by the entry
/// oracle for the ELF dynamic INIT/FINI tables (Ghidra `ElfProgramBuilder`'s
/// naming, threaded through `AnalysisOutput::entry_names`).  They say what the
/// table SLOT is, not what the function is called, so a real symbol at the same
/// address must outrank them — `fmt_arm`'s `0x4c0` reports
/// `__do_global_dtors_aux` with `_FINI_0` as an alias, not the reverse.
///
/// A recovered virtual method named by its VTABLE SLOT INDEX is the same kind of
/// name and is treated the same way ([`is_vtable_slot_name`]).
fn is_structural_entry_name(name: &str) -> bool {
    name == "_DT_INIT"
        || name == "_DT_FINI"
        || name
            .strip_prefix("_INIT_")
            .or_else(|| name.strip_prefix("_FINI_"))
            .is_some_and(|i| !i.is_empty() && i.bytes().all(|b| b.is_ascii_digit()))
        || is_vtable_slot_name(name)
}

/// (kuna, `pe-function-inventory-labels`) Is `name` a virtual method the RTTI passes
/// could only name by its vtable SLOT INDEX — `<Class>::vfunc_<i>` (MSVC `rtti`) or
/// `<Class>::vtable_<i>` (Itanium `itaniumrtti`)?
///
/// Neither metadata graph records per-method names, only the class name, so the slot
/// index is all those passes have. That makes the name structural in exactly the sense
/// `_FINI_<i>` is: it says which SLOT the function occupies, not what the function is
/// called. A real method name recovered at the same address must therefore outrank it —
/// on a Windows PE the `std::basic_streambuf` thunks carry both, and the length
/// tie-break alone decided it, so `showmanyc` reported as `std::basic_stringbuf::vfunc_5`
/// while its one-byte-shorter neighbour `uflow` kept its real name.
///
/// The unindexed `<Class>::vftable` / `<Class>::vtable` labels are DATA symbols and never
/// reach this ranking; the digit test excludes descriptive suffixes such as
/// `Widget::vtable_for_Drawable` as well.
fn is_vtable_slot_name(name: &str) -> bool {
    let Some((class, slot)) = name.rsplit_once("::") else {
        return false;
    };
    if class.is_empty() {
        return false;
    }
    slot.strip_prefix("vfunc_")
        .or_else(|| slot.strip_prefix("vtable_"))
        .is_some_and(|i| !i.is_empty() && i.bytes().all(|b| b.is_ascii_digit()))
}

/// (kuna, issue #197) How informative is `name`?  Smaller sorts first when
/// [`ConsoleProgram::function_entries_canonical`] picks which of an entry's names
/// to report; the rest become aliases.  Ordered by decreasing importance:
///
/// 1. **Not an engine placeholder.**  `sub_`/`func_`/`FUN_`/`LAB_`
///    ([`is_generic_placeholder_name`]) carry no information beyond the address,
///    which the record already holds, so they always lose.
/// 2. **Not a synthesized structural name** ([`is_structural_entry_name`]).
/// 3. **No leading underscore.**  Where one function carries both a linker symbol
///    and a debug-info symbol, the underscore-prefixed one is the mangled/ABI
///    spelling: `macho_dwarf.o` has `_l0`+`first_byte` at `0x0` and `_main`+`main`
///    at `0x40`, and the reported mingw-PE pair is `_pre_c_init`+`pre_c_init`.
///    The unprefixed name is the one a reader wants.
/// 4. **Shorter**, then **lexicographic** — so the choice is total and stable
///    run to run rather than dependent on symbol-stream order.  This is a tie-break
///    between names of equal standing ONLY; a difference in kind must be caught by a
///    tier above it, or a one-character length accident decides the report.
fn entry_name_rank(name: &str) -> (bool, bool, bool, usize) {
    (
        is_generic_placeholder_name(name),
        is_structural_entry_name(name),
        name.starts_with('_'),
        name.len(),
    )
}

/// Build the marshaling [`IdRegistry`] the console bootstrap needs (the same
/// per-module id registration the `decompile_e2e` gate's `build_registry` does:
/// translate + sleigh-arch + loadimage-xml + option element ids).
fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    register_option_elements(&mut registry);
    registry
}

/// Locate the SLEIGH specs root (C++ `SleighArchitecture::specpaths`).
///
/// The Python tools pass `-s <specs>` and set `SLEIGHHOME=<specs>`; the bin's
/// arg parser records the spec roots and hands them here.  The first existing
/// root wins (mirroring the C++ `scanForSleighDirectories` over the recorded
/// roots).
fn scan_language_database(spec_roots: &[String], registry: &IdRegistry) -> KunaResult<LanguageDatabase> {
    let mut db = LanguageDatabase::new();
    for root in spec_roots {
        db.scan_for_sleigh_directories(root);
    }
    db.get_descriptions(registry)?;
    Ok(db)
}

/// Find the `<binaryimage>` element inside a parsed document root (which may be a
/// bare `<binaryimage>` or a `<decompilertest>` wrapping one).
fn find_binaryimage(root: &Rc<Element>) -> Option<Rc<Element>> {
    if root.get_name() == "binaryimage" {
        return Some(Rc::clone(root));
    }
    for c in root.get_children() {
        if let Some(found) = find_binaryimage(c) {
            return Some(found);
        }
    }
    None
}

/// Read an attribute as a `String` (lossy ASCII), `None` if absent.
fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

/// Run the spec-file resolution, translator build, and `Architecture::init`
/// tail on an already-language-resolved [`SleighArchitecture`] — the chain both
/// the XML and ELF frontends share (C++ `buildSpecFile` → `buildTranslator` →
/// `buildTypegrp`/`buildCoreTypes`/`buildAction`/…).
///
/// The caller must have set `archid` + resolved the language index first; this
/// reads the `.sla`/`.cspec`/`.pspec`, builds the translator (with a [`NullLoad`]
/// placeholder image the caller replaces after open), installs the register
/// lookup, and runs `init_post_engine`.  It deliberately does **not** open or
/// attach the loader — that is the frontend's `postSpecFile` job (the XML path
/// opens the `<binaryimage>`; the ELF path attaches the default code space).
fn build_engine_and_init(sleigh: &mut SleighArchitecture, db: &LanguageDatabase) -> KunaResult<()> {
    // buildSpecFile -> the resolved .sla; buildTranslator (decode the .sla).
    let specs = sleigh.build_spec_file(db)?;
    let resolved_sla = specs
        .slafile
        .ok_or_else(|| KunaError::lowlevel("build_spec_file resolved no .sla"))?;
    let sla = std::fs::read(&resolved_sla)
        .map_err(|e| KunaError::lowlevel(format!("read sla {resolved_sla}: {e}")))?;

    // The loader is handed to the translator as a dummy first; the real opened
    // image replaces it after init (mirrors corpus_bootstrap.rs / the e2e gate).
    sleigh.build_translator(Box::new(NullLoad), &sla)?;

    // Apply the active language's `.ldefs` `<truncate_space>` records (C++
    // `Architecture::restoreFromSpec` -> `SleighArchitecture::modifySpaces`,
    // architecture.cc:631).  This shrinks an address space's addr size before the
    // type factory reads `getDefaultDataSpace()->getAddrSize()` for the default
    // pointer width — e.g. PowerPC:BE:32:e500 truncates `ram` to 4 so a `void *`
    // is a 32-bit pointer even though the GPRs (and the space) are modeled 64-bit.
    {
        let langindex = sleigh.language_index();
        let arch = sleigh
            .base_mut()
            .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?;
        db.modify_spaces(langindex, arch.manage())?;
    }

    // Hand the resolved compiler-spec (`.cspec`) XML to the architecture so
    // `build_default_proto` can decode the real `<default_proto>` input/output
    // parameter lists (the C++ `parseCompilerConfig` reads the cspec here).
    // A read failure is non-fatal: the architecture falls back to the name-only
    // default model (proto recovery simply won't fire).
    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            sleigh
                .base_mut()
                .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
                .set_cspec_xml(cspec);
        }
    }

    // Hand the resolved processor-spec (`.pspec`) XML to the architecture so
    // `parse_processor_config` (run inside `init_post_engine`) can apply the
    // `<context_data>` `<context_set>` paints (the C++ `parseProcessorConfig`
    // reads the pspec here).  This is what selects the SLEIGH disassembly mode:
    // without it x86-64 lifts as 16-bit real mode.  A read failure is non-fatal
    // (the engine keeps the zero-default context).
    if !specs.processorfile.is_empty() {
        if let Ok(pspec) = std::fs::read(&specs.processorfile) {
            sleigh
                .base_mut()
                .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
                .set_pspec_xml(pspec);
        }
    }

    // Install the register-name lookup on the engine's manager (the C++
    // `AddrSpace::trans` back-pointer) while the engine is still the sole owner
    // of the manager — before `init_post_engine`'s `parse_processor_config`
    // resolves the pspec `<tracked_set>` register names (e.g. `DF`).
    sleigh
        .base_mut()
        .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
        .translate_mut()
        .as_sleigh_mut()
        .expect("install_register_lookup: standalone Sleigh engine")
        .install_register_lookup()?;

    // The tail of Architecture::init (buildTypegrp/buildCoreTypes/buildAction/…).
    sleigh
        .base_mut()
        .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
        .init_post_engine()?;
    Ok(())
}

/// Bootstrap a [`ConsoleProgram`] from a `<binaryimage>` element + arch id,
/// against the SLEIGH specs at `spec_roots` (C++ `IfcLoadFile`'s
/// `buildArchitecture` + `conf->init(store)`).
///
/// This is the faithful console body of C++ `IfcLoadFile::execute` reduced to the
/// kuna XML engine backend: build the loader from the `<binaryimage>`, resolve
/// the language, build the spec file, build the translator, run
/// `init_post_engine` (the tail of `Architecture::init`), open the image, hand it
/// to the engine, then read the loader symbols.  Errors carry the C++ failure
/// message so the console's `Could not create architecture` path is faithful.
pub fn bootstrap_program(
    binaryimage: Rc<Element>,
    arch_id: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let registry = build_registry();

    // capa->buildArchitecture(filename,target,...)
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("loadfile", "");

    // XmlArchitecture::buildLoader (find the <binaryimage>, wrap in LoadImageXml).
    arch.build_loader(Rc::clone(&binaryimage))?;

    // collectSpecFiles + resolveArchitecture (language-id normalization/index).
    let db = scan_language_database(spec_roots, &registry)?;
    arch.sleigh_mut().set_archid(arch_id);
    arch.sleigh_mut().resolve_architecture(&db, arch_id)?;
    if arch.sleigh().language_index() < 0 {
        return Err(KunaError::lowlevel(format!(
            "No sleigh specification for architecture {arch_id}"
        )));
    }

    // buildSpecFile -> buildTranslator -> the Architecture::init tail (shared
    // by both the XML and ELF frontends).
    build_engine_and_init(arch.sleigh_mut(), &db)?;

    // postSpecFile: open the corpus image against the engine spaces.
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call; the
    // borrow is released before any &mut use of `arch` (same shape as the e2e
    // gate / corpus_bootstrap.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)?;

    // Read the loader symbols (the readLoaderSymbols hook) from the opened image
    // BEFORE handing it to the engine: the LoadImageXml exposes name+address.
    let symbols = read_loader_symbols(arch.loader());

    // C++ `Architecture::fillinReadOnlyFromLoader` (architecture.cc:1375), part of
    // the `Architecture::init` chain: query the load image for its read-only
    // address ranges and OR `Varnode::readonly` over them in the symbol table's
    // property map.  `setVarnodeProperties`/`queryProperties` then paints the
    // `readonly` flag on varnodes reading those ranges, which `ActionVarnodeProps`
    // folds into constants when `option readonly` is on (the float-cluster's
    // IEEE-754 literals live in read-only RAM).  Collected here, while the opened
    // `LoadImageXml` is still in hand, then applied to the symboltab below.
    let readonly_ranges: Vec<(kuna_base::address::Address, kuna_base::address::Address)> =
        if let Some(loader) = arch.loader() {
            use kuna_base::address::RangeList;
            use kuna_sleigh::loadimage::LoadImage;
            let manage_ro: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
            let mut rangelist = RangeList::new();
            loader.get_readonly(&mut rangelist);
            // SAFETY: same outlives-the-call shape as the open() borrow above; the
            // manager lives inside `arch` and is only read here.
            let manage_ref = unsafe { &*manage_ro };
            rangelist
                .iter()
                .map(|r| (r.get_first_addr(), r.get_last_addr_open(manage_ref)))
                .collect()
        } else {
            Vec::new()
        };

    // Hand the opened loader to the engine (the C++ `loader` back-pointer the
    // decode reads on load_fill).
    let img = arch
        .take_loader()
        .ok_or_else(|| KunaError::lowlevel("loader vanished after open"))?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    // Apply the collected read-only ranges to the symbol table's property map
    // (C++ `symboltab->setPropertyRange(Varnode::readonly, *iter)`).
    if let Some(base) = arch.sleigh_mut().base_mut() {
        for (first, last_open) in &readonly_ranges {
            base.symboltab
                .set_property_range(kuna_decomp::varnode::varnode_flags::readonly, first, last_open);
        }
    }

    let description = arch.sleigh().base().unwrap().get_description().to_string();

    // Slice the XML leaf back to its `SleighArchitecture` (the XML-specific
    // loader/adjustvma machinery is spent; the engine owns the opened image).
    // The XML path runs no analysis tier, so no facts are stashed and the gated
    // commit at `read symbols` is a no-op (parity is structurally untouched).
    let mut prog = ConsoleProgram {
        arch: arch.into_sleigh(),
        registry,
        symbols,
        object_sections: Vec::new(),
        description,
        pending_analysis: Vec::new(),
        analysis_code_space: None,
        dwarf_locals: Vec::new(),
        analysis_image: None,
        loader_data_objects: Vec::new(),
        declared_extents: BTreeMap::new(),
        assertions: Vec::new(),
        assertion_outcomes: Vec::new(),
        pending_prototypes: BTreeMap::new(),
    };
    // C++ `conf->readLoaderSymbols("::")` (testfunction.cc:160 / consolemain.cc:104):
    // install the binaryimage symbols as FunctionSymbols so a CALL to one resolves
    // to its callee name at flow-analysis time.
    prog.read_loader_symbols()?;
    Ok(prog)
}

/// Bootstrap a [`ConsoleProgram`] from a **real object-format** binary on disk
/// (the kuna analog of the C++ console's BFD path: `LoadImageBfd` +
/// `RawBinaryArchitecture`/the resolved arch).
///
/// Format-neutral by construction: it drives the `object`-crate
/// [`ObjectLoadImage`] (which funnels every format-specific decision through the
/// `ObjectFormat` boundary) in place of the XML loader, so it serves ELF, PE,
/// Mach-O and COFF uniformly. Open the object (parse
/// machine/segments/symbols), take the SLEIGH language id straight off the
/// loader's `getArchType()` (the `resolveArchitecture` loader branch — C++
/// `loader->getArchType()`), build the engine, attach the default code space to
/// the loader (the C++ `RawBinaryArchitecture::postSpecFile` /
/// `LoadImageBfd::attachToSpace` tail), read the function symbols, then hand the
/// loader to the engine.
///
/// `target` is an optional explicit language id (the `load file <target> <path>`
/// first token, C++ BFD target): when non-empty it overrides the object-derived
/// id (so an unmapped machine can still be driven), exactly as the C++
/// `getTarget()` path takes precedence over the loader's arch type.
pub fn bootstrap_from_object(
    path: &str,
    target: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let registry = build_registry();

    // Read the image bytes once: reused for the loader AND the analysis-pass
    // object::File view (the analyzers need a parsed object the loader drops).
    let bytes = std::fs::read(path)
        .map_err(|e| KunaError::lowlevel(format!("Unable to open image file: {path}: {e}")))?;
    // (PR-8) Mach-O fat / universal (`0xcafebabe`) peel: `object::File::parse`
    // is a thin, single-arch view and cannot parse a fat header, so a universal
    // binary is reduced to ONE arch slice's bytes here — the single, canonical
    // slice-selection point (design §3.4). Everything downstream (the loader, the
    // analysis passes, the deferred-Listing stash) then sees the SAME thin slice,
    // exactly as for a thin Mach-O. The slice preference is `--slice` (env
    // `KUNA_MACHO_SLICE`) or the `--target` stem, else x86-64 → arm64 → first.
    // Non-fat input is untouched (the peel returns the bytes verbatim).
    let bytes = select_macho_slice(bytes, target);
    // (kuna) ELF section-table tolerance: the second normalization at this same
    // canonical point. An ELF's section table is link-time metadata, but `object`
    // validates it eagerly, so one corrupt half-word rejected an image whose
    // program headers described every loadable byte -- and every kuna surface
    // exited 1 on a file `readelf -l` reads happily. Clearing the unusable table
    // here (and only here) means the loader AND the analysis passes below see the
    // same recovered view. A file with a usable section table is untouched.
    let (bytes, shdr_note) = kuna_analysis::loader::elf_shdr::tolerate_unusable_section_table(bytes);
    if let Some(note) = shdr_note {
        eprintln!("[kuna] {note}");
    }
    // (kuna) PE data-directory tolerance: the same normalization one format over.
    // A PE's `NumberOfRvaAndSizes` is declared separately from the room its own
    // optional header gives the directory array, and packers overwrite it; Windows
    // reads whatever fits, `object` insists on the declared count and rejects the
    // image before any code is mapped. Clamping here (and only here) keeps the
    // loader and the analysis passes on the same recovered view, and reads the
    // imports from the real table rather than fabricating one.
    let (bytes, datadir_note) =
        kuna_analysis::loader::pe_datadirs::tolerate_oversized_data_directories(bytes);
    if let Some(note) = datadir_note {
        eprintln!("[kuna] {note}");
    }
    // LoadImageBfd(filename) + open(): parse the ELF (machine, segments, symbols).
    let mut loader = ObjectLoadImage::from_bytes(path, &bytes)?;

    // resolveArchitecture: the arch id is the loader's getArchType() (the object
    // machine → SLEIGH language id), unless an explicit target overrides it.
    let arch_type = String::from_utf8_lossy(&loader.get_arch_type()).into_owned();
    let mut sleigh = SleighArchitecture::new(path, target);
    let db = scan_language_database(spec_roots, &registry)?;
    // SleighArchitecture::resolveArchitecture: if target is set it wins (archid
    // stays empty here so the base resolve uses target||arch_type).
    sleigh.resolve_architecture(&db, &arch_type)?;
    // (kuna §2.2) Compiler-model fallback: if the format's chosen id (e.g. a PE's
    // `...:windows`) is not vendored for this arch *and* no explicit --target was
    // given, retry with the per-arch default-model id (`...:gcc`/`...:default`)
    // before erroring — wrong calling-convention details beat no decompile.
    // ELF carries no fallback (its primary already uses the default model), so
    // the established path is unaffected.
    if sleigh.language_index() < 0 && target.is_empty() {
        if let Some(fb) = loader.fallback_arch_id() {
            let fb = String::from_utf8_lossy(fb).into_owned();
            let mut retry = SleighArchitecture::new(path, "");
            retry.resolve_architecture(&db, &fb)?;
            if retry.language_index() >= 0 {
                sleigh = retry;
            }
        }
    }
    if sleigh.language_index() < 0 {
        return Err(KunaError::lowlevel(format!(
            "No sleigh specification for architecture {arch_type}"
        )));
    }

    // buildSpecFile -> buildTranslator -> the Architecture::init tail (shared).
    build_engine_and_init(&mut sleigh, &db)?;

    // (kuna) MIPS import-name recovery (Increment 27): the o32 ABI calls libc
    // imports indirectly through a GOT slot (`lw $t9, off($gp); jalr $t9`), with no
    // `.plt` code section and no `R_MIPS_JUMP_SLOT` relocations.  `elf_plt` names
    // each import's `.MIPS.stubs` stub and marks the GOT external slots constant
    // (`ObjectLoadImage::const_ranges` → `get_readonly`); turning on
    // `readonlypropagate` for MIPS lets `ActionVarnodeProps::fillinReadOnly` fold
    // the GOT load to the stub address so the call resolves to the import name
    // (`puts`/`printf`) instead of `sub_<addr>`.  Scoped to MIPS so non-MIPS
    // ELF output is unchanged; `option readonly off` restores the raw GOT load.
    // The XML datatest path never reaches `bootstrap_from_object`, so the parity
    // oracles are structurally untouched.
    if arch_type.starts_with("MIPS:") {
        sleigh.base_mut().unwrap().readonlypropagate = true;
    }

    // postSpecFile: attach the engine's default code space to the loader so its
    // loadFill/getNextSymbol build Addresses in the right space (C++
    // `RawBinaryArchitecture::postSpecFile`'s `attachToSpace(getDefaultCodeSpace())`).
    let code_space = Rc::clone(
        sleigh
            .base()
            .unwrap()
            .manage()
            .get_default_code_space()
            .ok_or_else(|| KunaError::lowlevel("no default code space after init"))?,
    );
    loader.attach_to_space(Rc::clone(&code_space));

    // Architecture::fillinReadOnlyFromLoader on the ELF path (the analog of the
    // XML path's collect-before-handoff above): gather the loader's read-only
    // ranges while the image is still in hand. Load-bearing for string rendering
    // (the printer's push_ptr_char_constant_ir gates a string literal on
    // is_read_only at the constant's address).
    let readonly_ranges: Vec<(Address, Address)> = {
        use kuna_base::address::RangeList;
        use kuna_sleigh::loadimage::LoadImage;
        let manage_ptr: *const AddrSpaceManager = sleigh.base().unwrap().manage();
        let mut rangelist = RangeList::new();
        loader.get_readonly(&mut rangelist);
        // SAFETY: same outlives-the-call shape as the XML path's open() borrow;
        // the manager lives inside `sleigh` and is only read here.
        let manage_ref = unsafe { &*manage_ptr };
        rangelist
            .iter()
            .map(|r| (r.get_first_addr(), r.get_last_addr_open(manage_ref)))
            .collect()
    };

    // Run the program-prep analysis passes (the kuna analyzer tier) over the
    // parsed object, keeping each pass's facts keyed by id. Read-only; the facts
    // are STASHED here and committed (gated) at `read symbols` — NOT eagerly —
    // so the per-pass `--option <id> on|off` flags (emitted by the CLI before
    // `read symbols`) are in effect when the commit runs (the deferred fix for
    // conflict #4, analysis-port-log.md). Bound to the real-ELF path ONLY — the
    // XML <binaryimage> bootstrap never runs these, so the datatest parity oracle
    // is structurally untouched.
    let pending_analysis = kuna_analysis::passes::run_default_analyses_per_pass(
        &bytes,
        &loader,
        sleigh.base().unwrap(),
        // The Listing/xref tier's decoder: the engine's `Translate` (the same
        // `Sleigh` the decompile drives), coerced to `&dyn Translate`. The
        // `.sla` is loaded and the loadimage attached (above), so a flag-gated
        // `Listing::build` can decode through it. Default-off ⇒ unused.
        sleigh.base().unwrap().translate(),
    );

    // (kuna `dynrelocs`) The PT_GNU_RELRO-frozen dynamic-relocation slots, read
    // off the same loader while it is still in hand. `readonly_ranges` above
    // already covers them (the loader reports them through `get_readonly`), which
    // paints `Varnode::readonly`; this second list is what lets
    // `ActionVarnodeProps` FOLD those particular reads without the program-wide
    // `option readonly`. Empty for a non-ELF, an `ET_REL` object, or the gate off.
    let dynreloc_const: Vec<(u64, u64)> = loader.dynreloc_const_ranges().to_vec();

    // readLoaderSymbols (the ELF FUNC symbols) BEFORE handing the loader off.
    let mut symbols = read_loader_symbols_generic(&loader);
    let object_sections: Vec<ObjectSectionLocation> = loader
        .reloc_sections()
        .iter()
        .map(|section| ObjectSectionLocation {
            index: section.index,
            name: section.name.clone(),
            vma: section.vma,
            size: section.size,
        })
        .collect();
    for symbol in &mut symbols {
        if let Some(info) = loader
            .reloc_symbols()
            .iter()
            .find(|info| info.vma == symbol.addr.get_offset())
        {
            symbol.object_location = match (
                info.section_index,
                info.section_name.as_ref(),
                info.section_offset,
            ) {
                (Some(section_index), Some(section), Some(offset)) => Some(ObjectLocation {
                    section_index,
                    section: section.clone(),
                    offset,
                }),
                _ => None,
            };
            symbol.binding = Some(info.binding.clone());
            symbol.provenance = if info.undefined {
                EntryProvenance::UndefinedExternal
            } else {
                EntryProvenance::DefinedObject
            };
        }
    }
    // The data half of the same symbol tables (`STT_OBJECT`), read here for the
    // same reason — the loader is about to be moved into the engine. Installed at
    // the analysis commit, after DWARF and the detected strings have claimed their
    // addresses. See `ConsoleProgram::loader_data_objects`.
    let loader_data_objects = loader.data_symbols();

    // (kuna `rustabi`) Record the loader's source-language verdict on the
    // Architecture. Unlike the analyzer facts below this is not a pass output but
    // a one-bit property of the image, and it has to reach the ENGINE (the
    // per-function rules in `kuna_rustabi`) rather than the Listing, so it is
    // written straight onto the arch here at load, upstream of every `option`
    // command. The XML `<binaryimage>` bootstrap never reaches this line, which is
    // why `option rustabi auto` is inert on the datatest corpus by construction.
    let source_is_rust = kuna_analysis::sourcelang::detect_compiler_bytes(&bytes).is_rust();
    sleigh.base_mut().unwrap().source_is_rust = source_is_rust;

    // Hand the loader to the engine (the C++ `loader` back-pointer the decode
    // reads on load_fill).
    sleigh.base_mut().unwrap().set_loader(Box::new(loader));

    let description = sleigh.base().unwrap().get_description().to_string();

    let mut prog = ConsoleProgram {
        arch: sleigh,
        registry,
        symbols,
        object_sections,
        description,
        // Stash the per-pass analysis facts + the code space for the gated commit
        // at `read symbols` (IfcReadSymbols -> commit_analysis_passes).
        pending_analysis,
        analysis_code_space: Some(Rc::clone(&code_space)),
        dwarf_locals: Vec::new(),
        // Stash the image bytes + path for the DEFERRED Listing build (PR6
        // build-timing fix): the Listing is gated on `--option listing on`, set
        // by the CLI after `load file`, so it is built at the deferred commit
        // (`read symbols`) when the flag is known — not at load. `bytes` is moved
        // here (it is unused below this point).
        analysis_image: Some((path.to_string(), bytes)),
        loader_data_objects,
        declared_extents: BTreeMap::new(),
        assertions: Vec::new(),
        assertion_outcomes: Vec::new(),
        pending_prototypes: BTreeMap::new(),
    };
    // conf->readLoaderSymbols("::"): install the ELF symbols as FunctionSymbols.
    // The deferred analysis commit at `read symbols` REQUIRES this to have run
    // first (no-return/callfixup address+name resolution finds the funcsyms).
    prog.read_loader_symbols()?;

    // Apply the collected read-only ranges to the symbol table property map
    // (C++ symboltab->setPropertyRange(Varnode::readonly, *iter)), now that prog
    // owns the architecture. This is NOT a gated analysis pass (it is loader
    // markup), so it stays eager here, before any analysis commit.
    for (first, last_open) in &readonly_ranges {
        prog.arch_mut().symboltab.set_property_range(
            kuna_decomp::varnode::varnode_flags::readonly,
            first,
            last_open,
        );
    }

    // (kuna `dynrelocs`) ... and hand the foldable subset of them to the engine.
    // Sorted so `GlobalContainer::dynreloc_const_contains` can binary-search.
    {
        let mut ranges = dynreloc_const;
        ranges.sort_unstable();
        prog.arch_mut().dynreloc_const = std::rc::Rc::new(ranges);
    }

    // NB: the analysis-pass facts are committed later, gated, in
    // `commit_analysis_passes` (called from `IfcReadSymbols`), after the per-pass
    // `--option` flags are applied. See `ConsoleProgram::pending_analysis`.
    Ok(prog)
}

/// Commit a merged [`kuna_analysis::pass::AnalysisOutput`] into the engine's
/// symbol/type tables — the kuna-console side of the kuna-analysis pass boundary.
///
/// Each fact is **additive** (only adds knowledge) and **idempotent** against the
/// funcsym stream `read_loader_symbols` already committed (the `find_function`
/// overlap check no-ops a duplicate). Bound to the real-ELF path; the XML
/// `<binaryimage>` path never produces an `AnalysisOutput`, so this never runs
/// there. See `docs/missing-analyses.md` for the per-fact-kind API rationale.
fn commit_analysis_output(
    prog: &mut ConsoleProgram,
    code_space: &Rc<AddrSpace>,
    out: kuna_analysis::pass::AnalysisOutput,
) -> KunaResult<()> {
    use kuna_analysis::pass::SymKind;

    let num_spaces = prog.arch().manage().num_spaces();

    // 0. (kuna `cppproto`) The DWARF C++ arm's facts. The producing pass runs at
    //    `load file`, upstream of the `option` commands, so it stashes them apart
    //    and the live gate is HERE: on, they fold into the normal symbol/local
    //    streams below and their address-keyed prototypes are applied at step 5a;
    //    off, they are dropped and the DWARF recovery is the name-only walk,
    //    byte-identical to before this arm existed.
    let mut out = out;
    if prog.arch().analysis_cppproto {
        let cpp = std::mem::take(&mut out.cpp_dwarf);
        out.symbols.extend(cpp.symbols);
        out.locals.extend(cpp.locals);
        out.cpp_dwarf.prototypes = cpp.prototypes;
    } else {
        out.cpp_dwarf.prototypes.clear();
    }
    // 0a. (kuna `cppsig`) The demangled-signature arm's prototypes. Same deferred
    //     shape as `cppproto` above, but the gate is three-valued, so the mode
    //     selects WHICH certainty tiers survive: `proven` only the prototypes the
    //     mangling entails, `inferred` those plus the class-evidence inferences,
    //     `off` neither.
    let cppsig_mode = prog.arch().analysis_cppsig;
    let cppsig_protos = if cppsig_mode.enabled() {
        kuna_analysis::demangle::kuna_cppsig::select(
            std::mem::take(&mut out.cpp_sig),
            cppsig_mode.inferred(),
        )
    } else {
        Vec::new()
    };
    let out = out;

    // 1. Extra symbols a pass discovered. Function symbols install like the
    //    funcsym stream (idempotent); Data symbols (typed string/data objects)
    //    land via add_symbol_mapped. (No pass emits these yet in this increment;
    //    the commit path is wired so the string/entry passes plug in cleanly.)
    for s in &out.symbols {
        let addr = Address::new(Rc::clone(code_space), s.addr);
        match s.kind {
            SymKind::Function => {
                let type_code = prog.arch().types().get_type_code()?;
                let min_size = prog.arch().min_funcsymbol_size;
                {
                    let arch = prog.arch_mut();
                    let (scope, base) = arch
                        .symboltab
                        .find_create_scope_from_symbol_name(&s.name, "::", None, num_spaces)?;
                    if arch.symboltab.find_function(scope, &addr).is_none() {
                        arch.symboltab.add_function(scope, &addr, &base, min_size, type_code)?;
                    }
                }
                prog.register_symbol(&s.name, addr);
            }
            SymKind::Data => {
                let arch = prog.arch_mut();
                let (scope, base) = arch
                    .symboltab
                    .find_create_scope_from_symbol_name(&s.name, "::", None, num_spaces)?;
                // Untyped data object (a typed string symbol carries its char[N]
                // type through a dedicated fact added with the string pass).
                let ct = arch.types().get_type_code()?;
                let (sid, _) =
                    arch.symboltab.add_symbol_mapped(scope, &base, ct, &addr, &Address::new_invalid())?;
                arch.symboltab
                    .set_attribute(sid, kuna_decomp::varnode::varnode_flags::namelock);
            }
        }
    }

    // 1a. Named data globals with a known byte size (`out.data_objects`, the DWARF
    //     top-level variables). Mapped into the global scope with an
    //     `undefined<size>` type so a memory access of the object's storage
    //     (`mov [max_width], eax`) queries `queryContainer(addr, size)` and finds a
    //     covering SymbolEntry — the ActionNameVars global-scope query then binds the
    //     name, rendering `max_width` instead of `dat_<addr>`. A plain `SymFact{Data}`
    //     is installed with a size-1 code type, which only covers a 1-byte access, so
    //     multi-byte globals (`int`/`char*`) were left unnamed. Matches IDA Pro /
    //     Ghidra, which name data globals from the symbol table. `namelock` keeps the
    //     name; the `undefined<size>` type is NOT typelocked, so type propagation still
    //     infers the object's real type from its uses. A synthetic-`namelock` symbol
    //     is skipped where the address already carries a function or a covering data
    //     Symbol (a string `s_<addr>` or a hand-`map addr`ed global must not be
    //     shadowed) — the same guard the string-literal placement uses.
    //     The extent is clamped into the type factory's `int4` domain exactly as
    //     arm 4a does. This arm cannot actually overflow — `d.size` is produced
    //     from a `Datatype::get_size()`, itself an `int4`, and filtered to `>= 1`
    //     (`analyzers/dwarf/mod.rs`), so it already lives in `1..=int4::MAX` —
    //     but the two arms must not differ in shape, or the safe one reads as an
    //     endorsement of the unsafe one.
    for d in &out.data_objects {
        let addr = Address::new(Rc::clone(code_space), d.addr);
        let occupied = {
            let arch = prog.arch();
            match arch.symboltab.get_global_scope() {
                Some(global) => {
                    arch.symboltab.find_function(global, &addr).is_some()
                        || arch
                            .symboltab
                            .find_container(global, &addr, 1, &Address::new_invalid())
                            .is_some()
                }
                None => false,
            }
        };
        if occupied {
            continue;
        }
        let ct = prog
            .arch()
            .types()
            .get_base(
                d.size.clamp(1, int4::MAX as u32) as int4,
                kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN,
            )?;
        let arch = prog.arch_mut();
        let (scope, base) =
            arch.symboltab.find_create_scope_from_symbol_name(&d.name, "::", None, num_spaces)?;
        let (sid, _) =
            arch.symboltab.add_symbol_mapped(scope, &base, ct, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, kuna_decomp::varnode::varnode_flags::namelock);
    }

    // 1b. Extra read-only address ranges a pass discovered (`out.readonly`) — e.g.
    //     the MSVC RTTI vftable slot arrays (`rtti` R3). OR `Varnode::readonly`
    //     over each `[first, last_open)` range in the symbol-table property map, the
    //     same `symboltab->setPropertyRange(Varnode::readonly, *iter)` the loader's
    //     section-derived read-only ranges use (bootstrap_program). With `option
    //     readonly` on this lets a load of a vtable slot fold to its constant; the
    //     ranges are additive and only ever WIDEN the read-only set. PE `.rdata` is
    //     already read-only from its section flags, so this is belt-and-suspenders
    //     there; it is load-bearing for any pass that marks a range the loader did
    //     not. Empty on every default run (only the gated real-binary passes emit).
    for &(first, last_open) in &out.readonly {
        let begin = Address::new(Rc::clone(code_space), first);
        let end = Address::new(Rc::clone(code_space), last_open);
        prog.arch_mut().symboltab.set_property_range(
            kuna_decomp::varnode::varnode_flags::readonly,
            &begin,
            &end,
        );
    }

    // 1c. (kuna) External-reference address ranges (`out.externref`) — the PE Import
    //     Address Table slots the `peimportcall` pass reports. OR `Varnode::externref`
    //     over each `[first, last_open)` range in the same symbol-table property map,
    //     which `Scope::queryProperties` folds into every global Varnode covering the
    //     range. That one flag is what `ActionDeindirect`'s `queryExternalRefFunction`
    //     arm requires (`isPersist() && isExternalRef()`) before it will resolve a
    //     CALLIND through the slot to the import FunctionSymbol registered there — the
    //     kuna stand-in for Ghidra's `ExternRefSymbol` (`Scope::addExternalRef`), which
    //     the port never carried. Empty on every non-PE target and whenever the gate is
    //     off.
    for &(first, last_open) in &out.externref {
        let begin = Address::new(Rc::clone(code_space), first);
        let end = Address::new(Rc::clone(code_space), last_open);
        prog.arch_mut().symboltab.set_property_range(
            kuna_decomp::varnode::varnode_flags::externref,
            &begin,
            &end,
        );
    }

    // 2. Discovered entry points (stripped targets): name + add_function +
    //    register_symbol (the `map function` recipe).
    //
    //    Naming: an entry that carries a Ghidra-faithful name in the `entry_names`
    //    overlay (the dynamic INIT/FINI array elements — `_INIT_<i>` / `_FINI_<i>`,
    //    and the single `_DT_INIT`/`_DT_FINI`, per `ElfProgramBuilder`) is named
    //    with it; every other discovered VMA falls back to the generic
    //    `name_function` (`sub_<addr>`), exactly as before.
    //
    //    The idempotence probe resolves ACROSS SCOPES (C++ `Scope::queryFunction`
    //    spans the scope tree), which is what lets a real `.symtab`/`.dynsym` name
    //    win on a non-stripped binary. A scope-local probe only ever saw the GLOBAL
    //    scope (the synthetic `sub_<addr>`/`_INIT_<i>` names carry no `::`), so a
    //    demangled C++ callee living in a namespace scope — `Account::deposit` in
    //    scope `Account` — looked absent and a duplicate generic FunctionSymbol was
    //    installed in GLOBAL alongside it. `find_function_across_scopes` searches
    //    global first, so that duplicate then shadowed the real name at every call
    //    site and the printer rendered `sub_<addr>` (DIV-59).
    for &vma in &out.entries {
        let addr = Address::new(Rc::clone(code_space), vma);
        let name = out
            .entry_names
            .iter()
            .find(|(a, _)| *a == vma)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| prog.arch().name_function(&addr));
        let type_code = prog.arch().types().get_type_code()?;
        let min_size = prog.arch().min_funcsymbol_size;
        {
            let arch = prog.arch_mut();
            if arch.symboltab.find_function_across_scopes(&addr).is_none() {
                let (scope, base) = arch
                    .symboltab
                    .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
                arch.symboltab.add_function(scope, &addr, &base, min_size, type_code)?;
            }
        }
        prog.register_symbol(&name, addr);
    }

    // 3. No-return functions: resolve each matched function and set the no-return
    //    flag (the batch form of OptionNoReturn::apply). A fact with no matching
    //    function is the faithful no-op — Ghidra only iterates existing symbols.
    //
    //    Resolution is by **address** (find_function_across_scopes, the same
    //    cross-scope resolver the call resolver uses): the address is the stable
    //    key. The demangle pass renames the funcsym before it is installed, so a
    //    mangled C++ no-return symbol (`_ZSt9terminatev`) is installed as
    //    `std::terminate` in scope `std` — a name lookup of the raw mangled string
    //    would miss it, but the funcsym is still at the same address the no-return
    //    scan saw. The name path is the fallback for an import that exists only as
    //    a differently-addressed PLT stub (e.g. when the scan's `.dynsym` address
    //    is 0 / unmapped but the PLT-named FunctionSymbol resolves by name).
    //    NOTE: the public Database method, NOT the private
    //    Architecture::set_function_no_return.
    let mut nr = out.noreturn.clone();
    nr.sort_by(|a, b| (a.addr, &a.name).cmp(&(b.addr, &b.name)));
    nr.dedup();
    for fact in &nr {
        // Address path (preferred): resolves a demangled/namespaced funcsym.
        let by_addr = if fact.addr != 0 {
            let addr = Address::new(Rc::clone(code_space), fact.addr);
            prog.arch().symboltab.find_function_across_scopes(&addr).map(|(sid, _)| sid)
        } else {
            None
        };
        let sid = match by_addr {
            Some(sid) => Some(sid),
            // Name fallback: an import with no function installed at `fact.addr`
            // (e.g. an undefined-address `.dynsym` entry whose only installed
            // FunctionSymbol is the PLT stub, resolved by its installed name).
            None => prog.arch().query_global_function(&fact.name).ok(),
        };
        if let Some(sid) = sid {
            prog.arch_mut().symboltab.set_function_no_return(sid, true);
        }
    }

    // 3a'. (kuna, Ghidra-gap) Stash the `call error(nonzero,…)` prune list on the arch
    //      so `decompile-all` applies CALL_RETURN flow overrides per function (prunes the
    //      fall-through Ghidra also prunes). Empty unless the Listing + `noreturn_error`
    //      are on, so the datatest/console parity paths (no Listing) are unaffected.
    if !out.no_fallthru_calls.is_empty() {
        let mut sites = out.no_fallthru_calls.clone();
        sites.sort_unstable();
        sites.dedup();
        prog.arch_mut().error_noreturn_callsites = sites;
    }

    // 3b. FID re-identification (the kuna analog of Ghidra's FID identification
    //     analyzer): RENAME a function whose instruction-stream fingerprint matched
    //     a known-library record. Unlike the SymFact arm (step 1) — an idempotent
    //     *add* that no-ops on an already-installed function — FID overwrites the
    //     engine placeholder name of a function that DOES exist. Resolution is by
    //     ADDRESS (`find_function_across_scopes`, the no-return arm's resolver).
    //
    //     The LABEL GATE (`is_generic_placeholder_name`) is kuna's analog of
    //     Ghidra's `!alwaysApplyFidLabels && hasUserOrImportedSymbols` gate: FID
    //     only ever overwrites the engine's OWN `sub_<addr>`/`func_<addr>`/`FUN_*`
    //     placeholder, NEVER a real `.symtab`/DWARF/imported name. On a stripped
    //     binary every function is a placeholder ⇒ FID fires; on a named binary it
    //     defers (the real name wins). A fact with no function at `addr`, or whose
    //     function already carries a real name, is the faithful no-op.
    let mut fids = out.fid_names.clone();
    fids.sort_by(|a, b| (a.addr, &a.name).cmp(&(b.addr, &b.name)));
    fids.dedup();
    for m in &fids {
        let addr = Address::new(Rc::clone(code_space), m.addr);
        let sid = prog.arch().symboltab.find_function_across_scopes(&addr).map(|(sid, _)| sid);
        if let Some(sid) = sid {
            // The label gate: only rename an engine placeholder.
            let is_placeholder = {
                let cur = prog.arch().symboltab.symbol(sid).get_name();
                is_generic_placeholder_name(cur)
            };
            if is_placeholder {
                prog.arch_mut().symboltab.rename_symbol(sid, &m.name)?;
                // Make the new name resolvable by `load function <name>` (the same
                // name->addr binding the entry/symbol arms register).
                prog.register_symbol(&m.name, addr);
            }
        }
    }

    // 4. Detected string literals: place a typelocked `char[len]` data symbol at
    //    each (the kuna analog of `DataUtilities.createData` for an ASCII string),
    //    so the engine+printer render the literal instead of the bare constant.
    //    Ghidra's StringsAnalyzer marks the data and locks its type; the typelock
    //    is what keeps the array char[N] type through type propagation.
    //    (kuna `widestrings`) The 2-byte width rides the same arm: a UTF-16LE
    //    literal gets a `wchar2[N]` (element count `len / 2`) instead of a
    //    `char[N]`, which is what makes the printer emit the `L` prefix and read the
    //    bytes two at a time (`L"ntdll.dll"` rather than `"n"`).
    //
    //    The wide facts are committed FIRST, and the order is the fix rather than a
    //    detail. Against `StringLiteralPass`'s own 1-byte facts the order is
    //    immaterial — a wide unit demands a zero high byte, so five consecutive
    //    1-byte-charset bytes never occur inside a wide run and neither width can
    //    reach the other's address. It matters against `operand_refs`, whose facts
    //    arrive in this same stream and whose run test accepts a SINGLE visible
    //    character: at a wide literal it reads the first unit's low byte and the
    //    high-byte NUL behind it as a complete `char[2]`, which is exactly the
    //    `LoadLibraryW("n")` defect. Whichever fact is planted first wins the
    //    `occupied` guard below, so the width that read the whole literal has to go
    //    first. The stream is dropped entirely when the gate is off — `off` is
    //    byte-identical to the 1-byte markup alone.
    let wide = if prog.arch().analysis_widestrings { out.wide_strings.as_slice() } else { &[] };
    for (fact, char_size) in
        wide.iter().map(|f| (f, 2u32)).chain(out.strings.iter().map(|f| (f, 1u32)))
    {
        let addr = Address::new(Rc::clone(code_space), fact.addr);
        // Conservative guard: skip an address that already carries a symbol (an
        // existing data/function symbol must not be shadowed). Ghidra likewise
        // only lays string data where the listing is undefined.
        let occupied = {
            let arch = prog.arch();
            match arch.symboltab.get_global_scope() {
                Some(global) => {
                    arch.symboltab.find_function(global, &addr).is_some()
                        || arch
                            .symboltab
                            .find_container(global, &addr, 1, &Address::new_invalid())
                            .is_some()
                }
                None => false,
            }
        };
        if occupied {
            continue;
        }

        // char[len]: getTypeChar(getSizeOfChar()) -> getTypeArray(len, char). Both
        // are fallible TypeFactory queries (the type group must be built by now).
        // `fact.len` is the BYTE span including the terminator, so the array's
        // element count is `len / char_size` — identical for the 1-byte width.
        let ch = if char_size == 1 {
            prog.arch().types().get_type_char(prog.arch().types().get_size_of_char())?
        } else {
            // A language whose <coretypes> declares no 2-byte character type has no
            // `wchar2` to plant. Skip the wide fact rather than fail the whole
            // commit — the 1-byte arm keeps its original hard failure.
            match prog.arch().types().get_type_char(2) {
                Ok(ch) => ch,
                Err(_) => continue,
            }
        };
        let arr = prog
            .arch()
            .types()
            .get_type_array((fact.len / char_size) as i32, ch)?;

        // A cosmetic synthetic name `s_<addr>` (the symbol exists to carry the
        // char[N] type at the address; the printer renders the literal, not the
        // name). Placed in its namespace-resolved scope (global here).
        let name = format!("s_{:x}", fact.addr);
        let arch = prog.arch_mut();
        let (scope, base) =
            arch.symboltab.find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
        let (sid, _) =
            arch.symboltab.add_symbol_mapped(scope, &base, arr, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, kuna_decomp::varnode::varnode_flags::typelock);
    }

    // 4a. Loader data symbols: the defined `STT_OBJECT` entries of `.symtab` /
    //     `.dynsym` (`ConsoleProgram::loader_data_objects`). Installed exactly like
    //     the DWARF data globals of arm 1a — an `undefined<size>` global with
    //     `namelock` only, so the container query matches at the real access width
    //     and type propagation still infers the object's real type — but committed
    //     HERE, last, so the two richer sources win every address they claim: a
    //     DWARF-described global keeps its DWARF extent (arm 1a) and a detected
    //     string literal keeps its `char[N]` typelock (arm 4). What is left is the
    //     set neither reaches, and that is precisely the interesting one: a
    //     copy-relocated libc extern (`optind`, `stdin`, `stdout`, `optarg`) has a
    //     `.bss` address and a `.dynsym` entry but no `.debug_info` DIE, so before
    //     this arm it rendered `dat_20a098`. IDA Pro and Ghidra both name data
    //     objects from the symbol table independently of debug info.
    //     Gated by `--option datasyms on|off` (default ON, DIV-76): the stream is
    //     collected at `load file` but committed HERE at `read symbols`, after the
    //     option lines are applied, so the flag needs no env bridge on either CLI
    //     path. The stash is drained either way (a second `read symbols` must not
    //     re-commit); off simply drops it and the rendering is exactly pre-DIV-26.
    let mut loader_data_objects = std::mem::take(&mut prog.loader_data_objects);
    if !prog.arch().analysis_datasyms {
        loader_data_objects.clear();
    }
    for (sym_addr, name, size) in &loader_data_objects {
        let addr = Address::new(Rc::clone(code_space), *sym_addr);
        let occupied = {
            let arch = prog.arch();
            match arch.symboltab.get_global_scope() {
                Some(global) => {
                    arch.symboltab.find_function(global, &addr).is_some()
                        || arch
                            .symboltab
                            .find_container(global, &addr, 1, &Address::new_invalid())
                            .is_some()
                }
                None => false,
            }
        };
        if occupied {
            continue;
        }
        // `size` is a raw ELF/PE/Mach-O `st_size`: a `u64` read straight out of
        // attacker-controlled bytes that no header check validates. Clamp it into
        // the type factory's `int4` domain BEFORE the cast. Clamping after the cast
        // (`.max(1) as int4`) truncates first, so `0x1_0000_0000` became a size-0
        // type that `add_symbol_internal` rejects — aborting the WHOLE analysis
        // commit for the binary, not one symbol — and `0xffff_fff0` became a
        // NEGATIVE size that indexed the type factory's cache out of bounds and
        // took the process down. A declared extent is never trusted to be
        // representable. GH-339.
        let ct = prog
            .arch()
            .types()
            .get_base(
                (*size).clamp(1, int4::MAX as u64) as int4,
                kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN,
            )?;
        let arch = prog.arch_mut();
        let (scope, base) =
            arch.symboltab.find_create_scope_from_symbol_name(name, "::", None, num_spaces)?;
        let (sid, _) =
            arch.symboltab.add_symbol_mapped(scope, &base, ct, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, kuna_decomp::varnode::varnode_flags::namelock);
    }

    // 5. Library prototypes (the kuna analog of ApplyDataArchiveAnalyzer): park each
    //    on its named global callee. `ActionDefaultParams` later copies the callee's
    //    prototype into the caller's call spec, so the argument constants get typed
    //    (e.g. `puts(char*)` types `0x400915` as a char pointer, which — with the
    //    read-only markup applied above and the StringManager — renders the string
    //    literal `puts("Username: ")`). A name with no matching FunctionSymbol is a
    //    silent no-op.
    for pieces in out.prototypes {
        let name = pieces.name.clone();
        prog.arch_mut().set_function_prototype_pieces(&name, pieces);
    }

    // 5b. (kuna `cppsig`) The DEMANGLED prototypes, bound by entry address like
    //     the DWARF ones below. Applied BEFORE them on purpose: a mangled symbol
    //     carries a DECLARATION (which can disagree with the code a compiler
    //     actually emitted), DWARF carries ground truth, so wherever both reach a
    //     function the DWARF signature must be the one that survives. Empty when
    //     `--option cppsig off`.
    for (addr, pieces) in cppsig_protos {
        let a = Address::new(Rc::clone(code_space), addr);
        prog.arch_mut().set_function_prototype_pieces_at(&a, pieces);
    }

    // 5a. (kuna `cppproto`) The DWARF prototypes bound by ENTRY ADDRESS. Applied
    //     AFTER the by-name pass so the address-resolved signature wins wherever
    //     both reach the same function — address is the key the read side already
    //     uses, and the only one that survives C++ (a demangled template name is
    //     normalized to `maxof`; a qualified name lives in a nested scope the
    //     global by-name query never reaches). Empty when the gate is off.
    for (addr, pieces) in out.cpp_dwarf.prototypes {
        let a = Address::new(Rc::clone(code_space), addr);
        prog.arch_mut().set_function_prototype_pieces_at(&a, pieces);
    }

    // 6. Processor-context decode-mode paints (the kuna analog of ARM's
    //    `ARM_ElfExtension`/`ArmSymbolAnalyzer` `programContext.setValue(TMode,…)`).
    //    Paint each over the engine's ContextDatabase BEFORE any instruction is
    //    decoded (we are still inside bootstrap_from_object, before any `load
    //    function` decode — the timing the ARM Thumb mode requires). `end: None`
    //    is the single-address point set (paint-to-next-change-point, Ghidra's
    //    per-symbol `setValue(v,a,a,val)` shape); `Some(end)` paints the explicit
    //    `[addr, end)` range (the `$t`-run form).
    //
    //    CRITICAL gate-safety: `set_variable`/`set_variable_region` return Err when
    //    the named context variable is NOT registered by the active language (e.g.
    //    `TMode` on x86-64). That MUST be a SILENT no-op — otherwise every non-ARM
    //    ELF decompile would regress. The producing pass already gates on the
    //    object being ARM (so on a non-ARM binary `out.context_paints` is empty),
    //    and this swallow is the belt-and-suspenders second guard.
    for paint in &out.context_paints {
        let begin = Address::new(Rc::clone(code_space), paint.addr);
        // Drop the Result: an unregistered context variable (non-ARM language) is
        // a faithful no-op, mirroring ArmSymbolAnalyzer.canAnalyze == false.
        let _ = prog.arch().with_context_db_mut(|db| match paint.end {
            Some(end) => {
                let endad = Address::new(Rc::clone(code_space), end);
                db.set_variable_region(paint.var.as_bytes(), &begin, &endad, paint.value)
            }
            None => db.set_variable(paint.var.as_bytes(), &begin, paint.value),
        });
    }

    // 7. Tracked register VALUES (the kuna analog of MipsAddressAnalyzer's
    //    per-function register-value seeding / the console `set track <reg> <val>
    //    <start> <end>`). For each fact, resolve the register varnode and seed the
    //    constant over `[func_addr, func_addr+1)` via `create_set` — the exact
    //    `IfcSettrackedrange` recipe (ifacedecomp.rs IfcSettrackedrange). The
    //    per-function `build_arch_handle` then snapshots the track base into the
    //    per-function ArchContext (`tracked_sets = clone_trackbase()`), and `ActionConstbase` (S3)
    //    emits `COPY #val -> reg` at the entry block, which constant propagation
    //    consumes (so a MIPS PIC `$gp`-relative load resolves). Committed here, at
    //    `read symbols`, BEFORE any `load function` decode — the correct timing.
    //
    //    CRITICAL gate-safety (same shape as the context paints above):
    //    `get_register_varnode` returns Err when the named register is NOT defined
    //    by the active language (e.g. `t9` on x86-64). That MUST be a SILENT no-op —
    //    otherwise a non-MIPS decompile would regress. The producing pass already
    //    gates on the object being MIPS (so on a non-MIPS binary `out.tracked_regs`
    //    is empty), and this swallow is the belt-and-suspenders second guard.
    for fact in &out.tracked_regs {
        let Ok(loc) = prog.arch().get_register_varnode(fact.reg.as_bytes()) else {
            // Register not defined by this language (non-MIPS): faithful no-op.
            continue;
        };
        let begin = Address::new(Rc::clone(code_space), fact.func_addr);
        // `[func_addr, func_addr+1)` — the per-function point range Ghidra uses
        // (setRegisterValue(funcAddr, funcAddr, …)); +1 makes the [addr1, addr2)
        // create_set range non-empty (addr2 > addr1, as IfcSettrackedrange requires).
        let end = Address::new(Rc::clone(code_space), fact.func_addr.wrapping_add(1));
        let val = fact.value;
        prog.arch().with_context_db_mut(|db| {
            // C++ createSet(addr1,addr2); track = def (copy default as base); push —
            // the exact IfcSettrackedrange body for a ranged `set track`.
            let def = db.get_tracked_default().clone();
            let track = db.create_set(&begin, &end);
            *track = def;
            track.push(kuna_sleigh::globalcontext::TrackedContext { loc, val });
        });
    }

    // 8. Call-fixups (the kuna analog of CallFixupAnalyzer's install loop): for
    //    each function the pass matched to a cspec call-fixup `<target>`, tag it
    //    with that fixup's inject id so the engine replaces the CALL with the fixup
    //    body (e.g. the `-pg` `mcount`/`__fentry__` profiling call dissolves). This
    //    is precisely the `IfcFixupApply` body (resolve fixup name → inject id;
    //    resolve function name → sid; set_function_inject_id), driven by the
    //    analyzer instead of by hand — and guarded by Ghidra's `getCallFixup()==null`
    //    check (`CallFixupAnalyzer.java:89`): only set when no fixup is already
    //    parked, so a hand-applied `fixup apply` is never clobbered. A name with no
    //    registered fixup or no matching FunctionSymbol is a silent no-op.
    if !out.call_fixups.is_empty() {
        // Rebuild the same target→fixup map the pass used (the fact carries only
        // the function name; the fixup is re-derived from the live pcodeinjectlib).
        let map = kuna_analysis::callfixup::target_fixup_map(prog.arch());
        for fact in &out.call_fixups {
            let Some(fixup_name) =
                kuna_analysis::callfixup::call_fixup_name_for_function(&fact.func_name, &map)
            else {
                continue;
            };
            // fixup name → inject id (CALLFIXUP_TYPE); -1 if unknown (no-op).
            let injectid = prog
                .arch()
                .pcodeinjectlib
                .base
                .get_payload_id(kuna_decomp::pcodeinject::CALLFIXUP_TYPE, fixup_name.as_bytes());
            if injectid < 0 {
                continue;
            }
            // function name → FunctionSymbol id; skip if it no longer resolves.
            let Ok(sid) = prog.arch().query_global_function(&fact.func_name) else {
                continue;
            };
            // getCallFixup()==null guard: only auto-apply when no fixup is set.
            if prog.arch().symboltab.function_inject_id_for_symbol(sid) >= 0 {
                continue;
            }
            prog.arch_mut().symboltab.set_function_inject_id(sid, injectid);
        }
    }

    // 8. DWARF stack LOCALS (the kuna analog of `DWARFFunctionImporter`'s
    //    `commitLocal` loop — DWARF subtask 3): park each named, typed
    //    `DW_OP_fbreg` local on its owning function (by entry VMA). Unlike the
    //    other arms, a local is NOT installed into a persistent symbol table here:
    //    a function's `ScopeLocal` is built fresh per-decompile (it is owned by the
    //    transient `Funcdata`, not the global symboltab). So these are stashed and
    //    re-seeded into the rebuilt `Funcdata`'s stack scope at decompile time by
    //    `IfcDecompile` (via `dwarf_locals_for` -> the `map addr`/`seed_mapped_symbols`
    //    path) — exactly how the console carries a hand-typed `map addr` stack
    //    symbol across the IR rebuild. `stack_offset` is already in stack-space
    //    coordinates (`call_frame_cfa + fbreg`); the Address is built lazily at
    //    lookup so it binds the live stack space.
    for fact in out.locals {
        prog.dwarf_locals
            .push((fact.func_addr, fact.name, fact.type_, fact.stack_offset));
    }

    // 9. DWARF SOURCE-LINE comments (the kuna analog of Ghidra's
    //    `DWARFLineInfoCommentScript`, `.debug_line` → instruction comments).
    //    Each `.debug_line` row's `file:line` is installed into the architecture's
    //    `commentdb` as a `Comment::user2` (the instruction-comment type the C
    //    printer emits as a `/* … */` line at the op's address). The printer reads
    //    `arch.commentdb` at `print C` time and `CommentSorter` places each comment
    //    in the basic block holding its instruction. `func_addr`/`addr` build their
    //    Address in the code space; a duplicate (same fad,ad,text) is dropped by
    //    `add_comment_no_duplicate` (the script also de-dups via `appendComment`).
    //    Produced only by the `dwarf_lines` pass (default-off); empty otherwise, so
    //    the default output is byte-identical to before this arm.
    for fact in out.comments {
        let fad = Address::new(Rc::clone(code_space), fact.func_addr);
        let ad = Address::new(Rc::clone(code_space), fact.addr);
        prog.arch_mut().commentdb.add_comment_no_duplicate(
            kuna_decomp::comment::comment_type::USER2,
            &fad,
            &ad,
            &fact.text,
        );
    }

    Ok(())
}

/// Bootstrap from a parsed XML document root (a `<binaryimage>` or a
/// `<decompilertest>` wrapping one), reading the arch id off the
/// `<binaryimage>` element.  Used by `load file` (path → parse → bootstrap).
pub fn bootstrap_from_root(root: &Rc<Element>, spec_roots: &[String]) -> KunaResult<ConsoleProgram> {
    let binaryimage = find_binaryimage(root)
        .ok_or_else(|| KunaError::lowlevel("Could not find binaryimage tag"))?;
    let arch_id = attr(&binaryimage, "arch")
        .ok_or_else(|| KunaError::lowlevel("<binaryimage> has no arch attribute"))?;
    bootstrap_program(binaryimage, &arch_id, spec_roots)
}

/// The ELF magic (`\x7fELF`), used to route `load file` to the real-binary path.
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
// Mach-O magics (design §1.4). On-disk byte orders of `MH_MAGIC*`/`FAT_MAGIC`.
const MACHO_LE64: [u8; 4] = [0xcf, 0xfa, 0xed, 0xfe]; // 0xfeedfacf, little-endian
const MACHO_LE32: [u8; 4] = [0xce, 0xfa, 0xed, 0xfe]; // 0xfeedface, little-endian
const MACHO_BE64: [u8; 4] = [0xfe, 0xed, 0xfa, 0xcf]; // 0xfeedfacf, big-endian
const MACHO_BE32: [u8; 4] = [0xfe, 0xed, 0xfa, 0xce]; // 0xfeedface, big-endian
const MACHO_FAT: [u8; 4] = [0xca, 0xfe, 0xba, 0xbe]; // FAT_MAGIC (big-endian on disk)

/// The COFF `IMAGE_FILE_MACHINE_*` values that begin a bare COFF object — the
/// leading little-endian `u16` of a relocatable `.obj`/`.o` (no `MZ`/`PE` header).
/// Limited to the machines kuna ships a `.sla` for (mirrors the design's
/// "COFF machine-type prefix" set); an unknown machine simply isn't claimed as a
/// COFF object and falls through to the XML branch (or `object`'s own reject).
const COFF_MACHINES: &[u16] = &[
    0x014c, // IMAGE_FILE_MACHINE_I386
    0x8664, // IMAGE_FILE_MACHINE_AMD64
    0x01c0, // IMAGE_FILE_MACHINE_ARM
    0x01c4, // IMAGE_FILE_MACHINE_ARMNT (Thumb-2)
    0xaa64, // IMAGE_FILE_MACHINE_ARM64
];

/// Does `bytes` look like an object-format binary the [`ObjectLoadImage`] loader
/// can drive (design §1.4)? This admits **all** the supported object formats
/// unconditionally — ELF, Mach-O (`0xfeedfac*` / fat `0xcafebabe`), PE (`MZ` DOS
/// stub — the typed parser validates the PE header downstream), and a bare COFF
/// object (a leading `IMAGE_FILE_MACHINE_*` `u16`). Anything else routes to the
/// XML branch. (Multi-format support was promoted from the former
/// `--experimental-formats` flag to the default in increment 46; the XML/datatest
/// corpus never carries a PE/Mach-O/COFF magic, so its dispatch is unchanged.)
fn is_object_binary(bytes: &[u8]) -> bool {
    if bytes.len() < 4 {
        return false;
    }
    let m: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    if m == ELF_MAGIC {
        return true; // ELF — always admitted (the established path).
    }
    // Mach-O thin (any of the four byte orders) or fat/universal.
    if matches!(m, MACHO_LE64 | MACHO_LE32 | MACHO_BE64 | MACHO_BE32 | MACHO_FAT) {
        return true;
    }
    // PE: the `MZ` DOS stub. `object`'s typed PE parser validates the `PE\0\0`
    // header at the e_lfanew offset; a bare `MZ` that isn't a real PE will be
    // rejected there with a clean error (not silently mis-loaded).
    if &bytes[..2] == b"MZ" {
        return true;
    }
    // Bare COFF object: a leading little-endian `IMAGE_FILE_MACHINE_*` machine
    // type (no DOS stub). Restricted to known machines so a coincidental 2-byte
    // prefix on XML/other input doesn't get mis-claimed.
    let machine = u16::from_le_bytes([bytes[0], bytes[1]]);
    COFF_MACHINES.contains(&machine)
}

/// The environment-variable name carrying a Mach-O fat-slice override
/// (`--slice <arch>`). Read live (per `load file`) so a test can set it
/// in-process; empty/unset selects the deterministic default slice.
const MACHO_SLICE_ENV: &str = "KUNA_MACHO_SLICE";

/// Reduce a Mach-O fat / universal binary to one arch slice's bytes — the single,
/// canonical slice-selection point at dispatch (design §3.4 / §8 PR-8). For a
/// thin (non-fat) input the `bytes` are returned **verbatim** (an exact, zero-copy
/// move), so the ELF / thin-Mach-O / PE / COFF paths are byte-identical.
///
/// The slice preference is, in order: an explicit `--slice <arch>` (the
/// [`MACHO_SLICE_ENV`] env var the CLI exports), then the `--target` token's
/// leading arch stem (so the existing language-override flag also steers the
/// slice), else the deterministic default (x86-64 → arm64 → first arch present).
/// A fat header that cannot be peeled (unparsable, or no usable slice) is left
/// untouched, so the downstream `object::File::parse` produces the existing
/// "Unsupported file format" error rather than this silently mis-loading.
fn select_macho_slice(bytes: Vec<u8>, target: &str) -> Vec<u8> {
    use kuna_analysis::loader::macho_fat::{is_fat, select_fat_slice, SlicePref};
    if !is_fat(&bytes) {
        return bytes; // thin / ELF / PE / COFF — verbatim, no copy of a slice.
    }
    // `--slice` (env) wins; else fall back to the `--target` arch stem.
    let slice_token = std::env::var(MACHO_SLICE_ENV).unwrap_or_default();
    let pref = if !slice_token.trim().is_empty() {
        SlicePref::parse(&slice_token)
    } else if !target.trim().is_empty() {
        SlicePref::parse(target)
    } else {
        SlicePref::default()
    };
    match select_fat_slice(&bytes, pref) {
        Some(slice) => slice.to_vec(),
        None => bytes, // unpeelable fat: leave it for object::File::parse to reject.
    }
}

/// Bootstrap from a file path (the `decomp_dbg` `load file [<target>] <path>`
/// body).  Detects the format by its leading bytes: an object-format magic
/// ([`is_object_binary`]) routes to the real-binary [`ObjectLoadImage`] path;
/// anything else is parsed as the XML `<binaryimage>`/`<decompilertest>` corpus
/// format.
///
/// This mirrors the C++ `ArchitectureCapability::findCapability` dispatch: the
/// `xml` capability's `isFileMatch` claims a `<bi…` document, otherwise the BFD
/// path handles the real binary.  `target` is the optional `load file` target
/// token (the C++ BFD target / an explicit SLEIGH language id); it is honored on
/// the object path and ignored on the XML path (the XML carries its own `arch`).
///
/// [`is_object_binary`] admits ELF, PE, Mach-O and COFF; the XML / datatest
/// corpus never carries an object-format magic, so its dispatch routes to the XML
/// branch exactly as before.
pub fn bootstrap_from_file(
    path: &str,
    target: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let bytes = std::fs::read(path)
        .map_err(|e| KunaError::lowlevel(format!("Unable to recognize imagefile {path}: {e}")))?;
    if is_object_binary(&bytes) {
        // Real object-format binary (ELF / PE / Mach-O / COFF): drive the
        // object-crate loader.
        return bootstrap_from_object(path, target, spec_roots);
    }
    let mut store = DocumentStorage::new();
    let root = store.parse_document(&bytes)?.get_root().clone();
    bootstrap_from_root(&root, spec_roots)
}

/// Iterate the opened [`LoadImageXml`]'s symbol records (name → address) into a
/// [`ProgramSymbol`] list (the `readLoaderSymbols` hook).
fn read_loader_symbols(loader: Option<&LoadImageXml>) -> Vec<ProgramSymbol> {
    match loader {
        Some(l) => read_loader_symbols_generic(l),
        None => Vec::new(),
    }
}

/// `readLoaderSymbols` over any opened [`LoadImage`] (the ELF path reuses this
/// against the [`ObjectLoadImage`]; the symbol must already be attached to a
/// space so `getNextSymbol` can build the `Address`).
fn read_loader_symbols_generic(loader: &dyn LoadImage) -> Vec<ProgramSymbol> {
    let mut out = Vec::new();
    loader.open_symbols();
    loop {
        let mut record = LoadImageFunc::default();
        if !loader.get_next_symbol(&mut record) {
            break;
        }
        let name = String::from_utf8_lossy(&record.name).into_owned();
        // (kuna `symbolnamebound`) Bound the loader's own name list to the same
        // scope path the symbol table will nest it under, so ONE spelling
        // reaches every surface -- `functions`, the `// Function:` header, the
        // emitted declaration and the call site. A no-op (borrowed, unallocated)
        // for every real name.
        let name =
            kuna_decomp::kuna_symbolnamebound::bound_scope_path(&name, "::").into_owned();
        out.push(ProgramSymbol {
            name,
            addr: record.address,
            object_location: None,
            binding: None,
            provenance: EntryProvenance::Mapped,
        });
    }
    out
}

/// A load image that errors on every read — handed to `build_translator` as the
/// placeholder before the real opened image replaces it (the C++ `DummyImg` /
/// e2e-gate shape).
struct NullLoad;

impl LoadImage for NullLoad {
    fn get_file_name(&self) -> &str {
        "null"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("null load image"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// Default the analysis-size bound (`load function`/`load addr` size 0 = the
/// function's natural extent), mirroring the C++ `IfcFuncload` / `IfcAddrrangeLoad`
/// unbounded follow.
pub const UNBOUNDED_SIZE: int4 = 0;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::space::{spacetype, AddrSpace};
    use kuna_num::opcodes::OpCode;
    use kuna_num::pcoderaw::VarnodeData;

    use super::{entry_name_rank, is_vtable_slot_name, FixedRefs, WholeOp};

    /// A throwaway `(ram, constant)` space pair; `ram` stands in for the default
    /// data space, exactly as `listing::xrefs`'s own tests build it.
    fn spaces() -> (Rc<AddrSpace>, Rc<AddrSpace>) {
        (
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)),
            Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_CONSTANT)),
        )
    }

    fn vn(space: &Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(Rc::clone(space)), offset, size }
    }

    /// Harvest `ops` as one instruction of length 4 at `vma`.
    fn harvest(ram: &Rc<AddrSpace>, vma: u64, ops: Vec<WholeOp>) -> FixedRefs {
        let mut refs = FixedRefs { ops, filled: 0, ..FixedRefs::default() };
        refs.filled = refs.ops.len();
        refs.harvest(Some(ram), vma + 4);
        refs
    }

    fn op(opcode: OpCode, out: Option<VarnodeData>, ins: Vec<VarnodeData>) -> WholeOp {
        WholeOp { opcode, out, ins }
    }

    /// The literal-pool read, in the two shapes SLEIGH spells one in: a `LOAD`
    /// off a constant address, and a direct memory varnode.
    #[test]
    fn a_fixed_address_read_is_reported_with_the_width_of_the_access() {
        let (ram, cst) = spaces();
        let load = op(
            OpCode::CPUI_LOAD,
            Some(vn(&cst, 0x100, 4)),
            vec![vn(&cst, 0x1b, 8), vn(&cst, 0x8458, 4)],
        );
        assert_eq!(harvest(&ram, 0x8440, vec![load]).reads, vec![(0x8458, 4)]);

        let direct = op(OpCode::CPUI_COPY, None, vec![vn(&ram, 0x404018, 4)]);
        assert_eq!(harvest(&ram, 0x1000, vec![direct]).reads, vec![(0x404018, 4)]);

        // A `LOAD`'s address varnode is pointer-sized whatever the access is, so
        // the width has to come from the output: `ldrh r0,[0x1003c]` lifts to a
        // 4-byte `ram` address and a 2-byte load, and calling that a word would
        // fold four bytes for a halfword read.
        let narrow = op(
            OpCode::CPUI_LOAD,
            Some(vn(&cst, 0x100, 2)),
            vec![vn(&cst, 0x1b, 8), vn(&ram, 0x1003c, 4)],
        );
        assert_eq!(harvest(&ram, 0x10034, vec![narrow]).reads, vec![(0x1003c, 2)]);
    }

    /// The `in0` of a `LOAD`/`STORE`/`CALLOTHER` is a space id, and of a flow op
    /// a destination; neither is an address that was read.
    #[test]
    fn a_target_slot_is_never_a_read() {
        let (ram, cst) = spaces();
        let store = op(
            OpCode::CPUI_STORE,
            None,
            vec![vn(&cst, 0x1b, 8), vn(&ram, 0x404018, 4), vn(&cst, 7, 4)],
        );
        assert!(harvest(&ram, 0x1000, vec![store]).reads.is_empty());
        let call = op(OpCode::CPUI_CALL, None, vec![vn(&ram, 0x2000, 4)]);
        let got = harvest(&ram, 0x1000, vec![call]);
        assert!(got.reads.is_empty());
        assert_eq!(got.flow_targets, vec![0x2000]);
    }

    /// Every conditionally-executed ARM instruction lowers to a `CBRANCH` over
    /// its own body, so its successor is named by a flow op whatever it is.
    /// Counted, that marks the word after each predicated instruction as a
    /// branch label — and a literal pool is a run of them, so `andeq` at
    /// 0x1000c would veto the pool word at 0x10010.
    #[test]
    fn an_instructions_own_fall_through_is_not_a_branch_target() {
        let (ram, _cst) = spaces();
        let skip = op(OpCode::CPUI_CBRANCH, None, vec![vn(&ram, 0x10010, 4)]);
        assert!(harvest(&ram, 0x1000c, vec![skip]).flow_targets.is_empty());
        // A real branch to anywhere else is kept.
        let real = op(OpCode::CPUI_CBRANCH, None, vec![vn(&ram, 0x83d0, 4)]);
        assert_eq!(harvest(&ram, 0x1000c, vec![real]).flow_targets, vec![0x83d0]);
    }


    /// Which of an entry's names `function_entries_canonical` reports.
    fn wins<'a>(a: &'a str, b: &'a str) -> &'a str {
        if entry_name_rank(a).cmp(&entry_name_rank(b)).then_with(|| a.cmp(b)).is_le() {
            a
        } else {
            b
        }
    }

    /// (kuna, `pe-function-inventory-labels`) A vtable-slot-index name is structural:
    /// a real method name at the same address outranks it however long that name is.
    /// Before, only the length tie-break separated them, so which name a PE inventory
    /// reported turned on a one-character accident — `std::basic_streambuf::showmanyc`
    /// lost to `std::basic_stringbuf::vfunc_5` while the shorter `uflow` next door kept
    /// its own name.
    #[test]
    fn a_real_method_name_outranks_its_vtable_slot_index() {
        for (real, slot) in [
            ("std::basic_streambuf::showmanyc", "std::basic_stringbuf::vfunc_5"),
            ("std::basic_streambuf::uflow", "std::basic_stringbuf::vfunc_7"),
            ("Shape::perimeter", "Circle::vtable_2"),
            // The real name being much the longer of the two is the whole point.
            ("Widget::an_extremely_long_method_name", "Widget::vfunc_0"),
        ] {
            assert_eq!(wins(real, slot), real, "{real} must outrank {slot}");
        }
    }

    /// The slot name still beats an engine placeholder — it is structural, not useless.
    #[test]
    fn a_vtable_slot_index_still_outranks_a_placeholder() {
        for placeholder in ["sub_140002c2c", "FUN_140002c2c", "func_140002c2c"] {
            assert_eq!(wins("std::bad_alloc::vfunc_0", placeholder), "std::bad_alloc::vfunc_0");
        }
    }

    /// Only `<Class>::v{func,table}_<digits>` is a slot name. The unindexed labels are
    /// DATA symbols, and a descriptive suffix names a base subobject, not a slot.
    #[test]
    fn only_an_indexed_slot_name_is_structural() {
        assert!(is_vtable_slot_name("Box::vfunc_0"));
        assert!(is_vtable_slot_name("std::basic_stringbuf::vtable_11"));
        for not_a_slot in [
            "Box::vftable",
            "Box::vtable",
            "Widget::vtable_for_Drawable",
            "Box::vfunc_",
            "Box::vfunc_1a",
            "vfunc_0",
            "::vfunc_0",
            "vtable_3",
        ] {
            assert!(!is_vtable_slot_name(not_a_slot), "{not_a_slot} is not a slot name");
        }
    }
}
