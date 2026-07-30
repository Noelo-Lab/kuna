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

use kuna_sleigh::loadimage::{LoadImage, LoadImageFunc, LoadImageSection};
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_sleigh::loadimage_xml::LoadImageXml;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

/// One function symbol discovered in the `<binaryimage>` (name → entry address).
#[derive(Debug, Clone)]
struct ProgramSymbol {
    name: String,
    addr: Address,
}

/// (kuna, issue #197) One **canonical** function entry — the unit of a
/// whole-binary run, produced by [`ConsoleProgram::function_entries_canonical`].
///
/// Exactly one of these exists per function entry address.  `name` is the most
/// informative symbol the entry carries ([`entry_name_rank`]); `aliases` holds
/// every other name for it — a generic `sub_<addr>` placeholder, a debug-info
/// name alongside the linker one, a decorated/undecorated pair — in the same
/// preference order, so nothing the raw symbol stream knew is lost and a
/// name-keyed lookup still resolves.
#[derive(Debug, Clone)]
pub struct FunctionEntry {
    /// The most informative name at this entry (never a placeholder when the
    /// entry also carries a real symbol).
    pub name: String,
    /// The entry address, Thumb-bit normalized on an ARM-family spec.
    pub addr: Address,
    /// Every other name this same entry carries, in the same preference order.
    pub aliases: Vec<String>,
}

/// (kuna) A one-shot [`AssemblyEmit`](kuna_sleigh::translate::AssemblyEmit)
/// sink: `Translate::print_assembly` dumps exactly one instruction, whose
/// mnemonic/body strings are captured here (the single-instruction form of the
/// `IfcPrintdisasm` listing emitter in `ifacedecomp.rs`).
#[derive(Default)]
struct OneShotAssemblyEmit {
    mnem: String,
    body: String,
}

impl kuna_sleigh::translate::AssemblyEmit for OneShotAssemblyEmit {
    fn dump(&mut self, _addr: &Address, mnem: &str, body: &str) {
        self.mnem = mnem.to_string();
        self.body = body.to_string();
    }
}

/// (kuna) A one-shot [`PcodeEmit`](kuna_sleigh::translate::PcodeEmit) sink:
/// `Translate::one_instruction` dumps exactly one instruction's p-code, each
/// op captured here as opcode + first input (the single-instruction pcode
/// analogue of [`OneShotAssemblyEmit`], mirroring `kuna_analysis`'s
/// `listing/decode.rs::OpCapture` — `in0` is all
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
    /// funcsym stream `read_loader_symbols` already installs, so it carries no
    /// `--option` gate — kuna has never made "name what the symbol table names"
    /// optional. It is installed at the analysis commit rather than eagerly at
    /// bootstrap purely for *precedence*: a DWARF-described global and a detected
    /// string literal both claim their address first, and this arm only fills the
    /// addresses neither covered (which is where the imported libc objects —
    /// `optind`, `stdin`, `stdout`, `optarg` — live). Empty on the XML datatest
    /// path and for a relocatable object.
    loader_data_objects: Vec<(u64, String, u64)>,
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
        self.symbols.iter().find(|s| s.name == name).map(|s| s.addr.clone())
    }

    /// (kuna) Iterate every function symbol kuna knows for the loaded program as
    /// `(name, entry Address)` — the enumeration a whole-binary driver
    /// (`kuna decompile-all`) loops over.
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
        let mut groups: BTreeMap<u64, (Address, Vec<String>)> = BTreeMap::new();
        for s in &self.symbols {
            // A sentinel (spaceless) address cannot be normalized or rebuilt;
            // such a record is not a real entry, so skip it rather than guess.
            let Some(space) = s.addr.get_space() else { continue };
            let vma = normalize(s.addr.get_offset());
            let entry = groups
                .entry(vma)
                .or_insert_with(|| (Address::new(Rc::clone(space), vma), Vec::new()));
            if !entry.1.iter().any(|n| n == &s.name) {
                entry.1.push(s.name.clone());
            }
        }

        groups
            .into_iter()
            .map(|(_, (addr, mut names))| {
                // Most informative first — see `entry_name_rank`.
                names.sort_by(|a, b| {
                    entry_name_rank(a).cmp(&entry_name_rank(b)).then_with(|| a.cmp(b))
                });
                let name = names.remove(0);
                FunctionEntry { name, addr, aliases: names }
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
    /// for.  Shared by both front-ends so the two `resolve_targets` copies cannot
    /// drift apart on it.
    pub fn find_entry_by_name(&self, want: &str) -> Option<FunctionEntry> {
        self.function_entries_canonical()
            .into_iter()
            .find(|e| e.name == want || e.aliases.iter().any(|a| a == want))
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

    /// (kuna) Disassemble the single machine instruction at code-space VMA
    /// `vma`, returning `(length, mnemonic, body)` — the one-instruction form
    /// of the `disassemble` console command (`IfcPrintdisasm`), for a caller
    /// that walks a range itself (advance by `length` per step).
    ///
    /// Builds the `Address` in the engine's default code space (the
    /// `function_named_at` idiom) and drives `Translate::print_assembly`
    /// through a one-shot [`AssemblyEmit`](kuna_sleigh::translate::AssemblyEmit)
    /// sink. An undecodable/unmapped address surfaces as the translator's
    /// `Err` (C++ `BadDataError`/`DataUnavailError`).
    pub fn disassemble_at(&self, vma: u64) -> KunaResult<(int4, String, String)> {
        let code_space = Rc::clone(
            self.arch()
                .manage()
                .get_default_code_space()
                .ok_or_else(|| KunaError::lowlevel("no default code space"))?,
        );
        let addr = Address::new(code_space, vma);
        let mut emit = OneShotAssemblyEmit::default();
        let length = self.arch().translate().print_assembly(&mut emit, &addr)?;
        Ok((length, emit.mnem, emit.body))
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

    /// (kuna) Read `size` raw image bytes at code-space VMA `vma` through the
    /// engine's load image (`LoadImage::load`, the `loader_rc()` handle) —
    /// `None` when the range is not backed by the image (e.g. `.bss`, an
    /// unmapped address, or a `size` beyond the C++ `int4` read contract).
    pub fn read_bytes(&self, vma: u64, size: usize) -> Option<Vec<u8>> {
        if size > i32::MAX as usize {
            return None; // LoadImage::load reads at most int4 bytes (C++ contract).
        }
        let code_space = Rc::clone(self.arch().manage().get_default_code_space()?);
        let addr = Address::new(code_space, vma);
        let loader_rc = self.arch().translate().loader_rc();
        // `load` is `&mut self` (it seeks/caches); the RefCell covers it.
        let bytes = loader_rc.borrow_mut().load(size as i32, &addr);
        bytes.ok()
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
    /// namespaced virtual-method function (e.g. `Box::vftable_0` in scope `Box`).
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
        self.symbols.retain(|s| s.name != name);
        self.symbols.push(ProgramSymbol { name: name.to_string(), addr });
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
    /// Listing is built, and the Listing-consumer passes run; their (per-pass
    /// gated) facts merge into the same `merged` output committed below. Default
    /// (listing off) ⇒ this whole block is skipped ⇒ byte-identical to today.
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
        // Real-ELF path only (the XML path stashes no image). The stash is consumed
        // once and shared by both deferred passes.
        let want_listing = self.arch().analysis_listing;
        let want_operand_refs = self.arch().analysis_operand_refs;
        if (want_listing || want_operand_refs) && self.analysis_image.is_some() {
            if let Some((path, bytes)) = self.analysis_image.take() {
                // A throwaway loadimage just to satisfy the pass contracts (their
                // `image` arg is unused — the decode reads through `translate`); a
                // parse failure makes the deferred step a graceful no-op.
                if let Ok(image) = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(
                    &path, &bytes,
                ) {
                    // Deferred Listing build + consumer run, gated on the listing
                    // flag. The call-fixup seed list is the names the load-time pass
                    // flagged resolved to addresses via the (already installed)
                    // symbol table.
                    if want_listing {
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
        commit_analysis_output(self, &code_space, merged)
    }
}

/// (kuna) Read a `kuna_analysis` pass's per-run enable flag off the
/// [`Architecture`] by the pass's `AnalysisPass::id`. An unknown id defaults to
/// enabled (a new pass with no registered gate still runs — fail-open, additive).
fn analysis_pass_enabled(arch: &Architecture, pass_id: &str) -> bool {
    match pass_id {
        "noreturn_known" => arch.analysis_noreturn_known,
        "libproto" => arch.analysis_libproto,
        "strings" => arch.analysis_strings,
        "entry_disc" => arch.analysis_entry_disc,
        // (kuna) `.eh_frame` LSDA landing-pad discovery (GccExceptionAnalyzer) — a
        // standalone stashed pass whose facts (the exception landing pads) are
        // computed at LOAD but COMMITTED only when this gate is on. Default-off
        // (output-changing: adds entries), so a default run never commits them and
        // the discovery set is byte-identical to FDE-pcBegin-only.
        "eh_frame_full" => arch.analysis_eh_frame_full,
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
        // Explicit (NOT the fail-open `_ => true` default): the source-line pass is
        // default-OFF (it changes the output), so it must be registered here to be
        // gated by the `analysis_dwarf_lines` flag rather than running by default.
        "dwarf_lines" => arch.analysis_dwarf_lines,
        "callfixup" => arch.analysis_callfixup,
        "addrtable" => arch.analysis_addrtable,
        "operand_refs" => arch.analysis_operand_refs,
        "listing" => arch.analysis_listing,
        "noreturn_disc" => arch.analysis_noreturn_disc,
        "noreturn_propagate" => arch.analysis_noreturn_propagate,
        "fid" => arch.analysis_fid,
        // (kuna) MSVC RTTI / vftable recovery — a standalone load-time pass whose
        // class-name + RTTI_* labels are computed at LOAD but COMMITTED only when
        // this gate is on. Default-OFF (output-changing: adds named data symbols),
        // PE-only (the pass + its passes_for registration both self-gate on PE), so
        // a default run / every non-PE binary commits nothing here.
        "rtti" => arch.analysis_rtti,
        "aif" => arch.analysis_aif,
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
fn is_structural_entry_name(name: &str) -> bool {
    name == "_DT_INIT"
        || name == "_DT_FINI"
        || name
            .strip_prefix("_INIT_")
            .or_else(|| name.strip_prefix("_FINI_"))
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
///    run to run rather than dependent on symbol-stream order.
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
        description,
        pending_analysis: Vec::new(),
        analysis_code_space: None,
        dwarf_locals: Vec::new(),
        analysis_image: None,
        loader_data_objects: Vec::new(),
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

    // readLoaderSymbols (the ELF FUNC symbols) BEFORE handing the loader off.
    let symbols = read_loader_symbols_generic(&loader);
    // The data half of the same symbol tables (`STT_OBJECT`), read here for the
    // same reason — the loader is about to be moved into the engine. Installed at
    // the analysis commit, after DWARF and the detected strings have claimed their
    // addresses. See `ConsoleProgram::loader_data_objects`.
    let loader_data_objects = loader.data_symbols();

    // Hand the loader to the engine (the C++ `loader` back-pointer the decode
    // reads on load_fill).
    sleigh.base_mut().unwrap().set_loader(Box::new(loader));

    let description = sleigh.base().unwrap().get_description().to_string();

    let mut prog = ConsoleProgram {
        arch: sleigh,
        registry,
        symbols,
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
            .get_base(d.size.max(1) as int4, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)?;
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

    // 2. Discovered entry points (stripped targets): name + add_function +
    //    register_symbol (the `map function` recipe).
    //
    //    Naming: an entry that carries a Ghidra-faithful name in the `entry_names`
    //    overlay (the dynamic INIT/FINI array elements — `_INIT_<i>` / `_FINI_<i>`,
    //    and the single `_DT_INIT`/`_DT_FINI`, per `ElfProgramBuilder`) is named
    //    with it; every other discovered VMA falls back to the generic
    //    `name_function` (`sub_<addr>`), exactly as before. The idempotent
    //    `find_function` no-op below still lets a real `.symtab`/`.dynsym` name win
    //    on a non-stripped binary — only genuinely new starts take the overlay name.
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
            let (scope, base) = arch
                .symboltab
                .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
            if arch.symboltab.find_function(scope, &addr).is_none() {
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
    for fact in &out.strings {
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
        let char_size = prog.arch().types().get_size_of_char();
        let ch = prog.arch().types().get_type_char(char_size)?;
        let arr = prog.arch().types().get_type_array(fact.len as i32, ch)?;

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
    let loader_data_objects = std::mem::take(&mut prog.loader_data_objects);
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
        let ct = prog
            .arch()
            .types()
            .get_base((*size).max(1) as int4, kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN)?;
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
        out.push(ProgramSymbol { name, addr: record.address });
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
