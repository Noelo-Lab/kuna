//! GATE for item `w5-infra-lift-diff`.
//!
//! The B2-grade STRUCTURAL differential the W5 gate needs: for each corpus
//! function, bootstrap the Rust `XmlArchitecture` frontend, run the real
//! `FlowInfo` follow-flow + the basic-block builder to the SAME boundary the C++
//! `break start heritage` (B2) snapshot represents, and compare the structural
//! skeleton (op count, per-op address+class+output, block partition, edges)
//! against the committed C++ B2 fixture.  See [`kuna_harness::liftdiff`] for the
//! grammar, the model, and the precise pre-heritage-action accounting.
//!
//! ## What boundary the C++ B2 snapshot is, and how the Rust side reaches it
//!
//! The snapshot is taken INSIDE `ActionDatabase::universalAction` at
//! `break start heritage` (`coreaction.cc:5756`).  Every pre-`ActionHeritage`
//! action whose group is in the default `"decompile"` group set has run:
//! `ActionStart` (= `Funcdata::startProcessing` = `followFlow`
//! (`generateOps`+`generateBlocks`) + `structureReset`), `ActionConstbase`,
//! `ActionDefaultParams`, `ActionExtraPopSetup`, `ActionPrototypeTypes`,
//! `ActionFuncLink`, `ActionUnreachable`, `ActionVarnodeProps`,
//! `ActionLowerSwitchInstall` (inert).  `ActionNormalizeSetup`/`ActionFuncLinkOutOnly`
//! are in groups NOT in `"decompile"`, so they DON'T run.
//!
//! The Rust port realizes `ActionStart`'s body (`generate_ops`, `generate_blocks`,
//! `structure_reset`).  The op-adding pre-heritage actions are NOT ported in the
//! flow path; the differential accounts for each precisely (`liftdiff`):
//!
//! - `ActionConstbase` tracked-context COPYs are DETECTED and excluded on the C++
//!   side (exact, reported per test).
//! - `ActionPrototypeTypes`'s RETURN-input rewrite is invisible to the structural
//!   compare (RETURN has no output; class unchanged).
//! - `ActionExtraPopSetup`/`ActionFuncLink` add ops ONLY for calls with a non-zero
//!   extrapop / a spacebase; a CALL test where they fired shows up as an op-count
//!   divergence (reported, never hidden).
//!
//! Tests that reach a W4 seam in `generate_ops` (a BRANCHIND/jump-table or an
//! injected CALLOTHER) are reported EXCLUDED with the seam reason.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{spacetype, AddrSpaceManager};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use kuna_decomp::flow::{FlowEnvironment, FlowInfo};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::seams::{Architecture as IrArch, TypeOp};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_base::xml::{DocumentStorage, Element};
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{register_translate_ids, PcodeEmit, Translate, UniqueLayout};

use kuna_harness::liftdiff::{self, BlockRec, DiffResult, OpClass, OpRec, StructModel};

// ===========================================================================
// Corpus pin set (one entry per fixture; >= 12, spanning architectures)
// ===========================================================================

/// The expected outcome of a case, asserted by the gate (so the documentation of
/// WHY a case diverges is a checked invariant, not just a comment).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Expect {
    /// Structurally equal end-to-end (counts toward the >= 8 PASS gate).
    Pass,
    /// Diverges because an UNPORTED pre-heritage action inserts ops the Rust flow
    /// path does not (the string documents which action); the gate requires the
    /// case to actually be DIVERGENT (never silently pass).
    DivergentBy(&'static str),
    /// Excluded because the Rust flow reaches a W4 seam / a corpus-data boundary.
    Excluded(&'static str),
}

/// A corpus B2 differential case.
struct Case {
    /// Fixture stem (`tests/fixtures/<stem>.b2.txt` is the committed C++ B2 model).
    stem: &'static str,
    /// The corpus XML path, relative to the repo root.
    corpus: &'static str,
    /// The full (normalized) archid the bootstrap must resolve to.
    archid: &'static str,
    /// The `.sla` path relative to `specs/`.
    sla_rel: &'static str,
    /// The function entry address space name.
    entry_space: &'static str,
    /// The function entry byte-offset (the corpus `<symbol>` decompiled by the
    /// datatest's `lo fu` + `decompile`).
    entry_off: u64,
    /// Per-fixture context-default paints (the `set context`/`set track`/.pspec
    /// `<context_data>` in effect at the entry — applied to the engine before
    /// flow, so decode matches the C++ snapshot).  `(name, value)`.
    context: &'static [(&'static str, u32)],
    /// The expected, documented outcome (asserted by the gate).
    expect: Expect,
}

/// The 15 pinned corpus cases (>= 12).  Ten processor families/variants: x86-64,
/// x86-32, ARM, AARCH64, MIPS, PowerPC, SPARC, 8051, 68000, Toy.
const CASES: &[Case] = &[
    // --- clean PASS cases (no calls / no out-of-entry pre-heritage insertions) ---
    Case {
        stem: "lzcount",
        corpus: "decompiler/datatests/lzcount.xml",
        archid: "PowerPC:BE:32:default:default",
        sla_rel: "Ghidra/Processors/PowerPC/data/languages/ppc_32_be.sla",
        entry_space: "ram",
        entry_off: 0x10020,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "boolless",
        corpus: "decompiler/datatests/boolless.xml",
        archid: "8051:BE:16:default:default",
        sla_rel: "Ghidra/Processors/8051/data/languages/8051.sla",
        entry_space: "CODE",
        entry_off: 0xa000,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "skipnext2",
        corpus: "decompiler/datatests/skipnext2.xml",
        archid: "Toy:BE:32:builder.align2:default",
        sla_rel: "Ghidra/Processors/Toy/data/languages/toy_builder_be_align2.sla",
        entry_space: "ram",
        entry_off: 0x10,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "floatprint",
        corpus: "decompiler/datatests/floatprint.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x4004c7,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "promotecompare",
        corpus: "decompiler/datatests/promotecompare.xml",
        archid: "x86:LE:32:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86.sla",
        entry_space: "ram",
        entry_off: 0x80662e0,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "readvolatile",
        corpus: "decompiler/datatests/readvolatile.xml",
        archid: "68000:BE:32:MC68020:default",
        sla_rel: "Ghidra/Processors/68000/data/languages/68020.sla",
        entry_space: "ram",
        entry_off: 0x484,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        // x86-64, 8-block branch lattice, no calls (GH condconst regression).
        stem: "condconst",
        corpus: "decompiler/datatests/condconst.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x1006fa,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        // SPARC: two tracked-context COPYs (DECOMPILE_MODE, didrestore) at the entry
        // head — both excluded by the entry-head rule; the body has no calls.
        stem: "gh6990-returnpair",
        corpus: "tests/stages/gh6990-returnpair.xml",
        archid: "sparc:BE:32:default:default",
        sla_rel: "Ghidra/Processors/Sparc/data/languages/SparcV9_32.sla",
        entry_space: "ram",
        entry_off: 0x32148,
        context: &[],
        expect: Expect::Pass,
    },
    // --- DIVERGENT cases (an UNPORTED pre-heritage action inserts ops the Rust
    //     flow path omits — each attributed to the responsible C++ action) -------
    Case {
        // AArch64: a CALL fires `ActionFuncLink` (a stack-placeholder LOAD around the
        // call site, high seqnum) — unported in the Rust flow path.
        stem: "ccmp",
        corpus: "decompiler/datatests/ccmp.xml",
        archid: "AARCH64:LE:64:v8A:default",
        sla_rel: "Ghidra/Processors/AARCH64/data/languages/AARCH64.sla",
        entry_space: "ram",
        entry_off: 0x42caec,
        context: &[],
        expect: Expect::DivergentBy("ActionFuncLink call-site placeholder (call to fotherfunc)"),
    },
    Case {
        // x86-64: 8+ CALLs, each fires `ActionFuncLink`/`ActionExtraPopSetup`
        // placeholders (51 inserted ops, times >= 0x50 in the C++ B2 snapshot).
        stem: "convert",
        corpus: "decompiler/datatests/convert.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x1006b9,
        context: &[],
        expect: Expect::DivergentBy("ActionFuncLink/ActionExtraPopSetup call placeholders"),
    },
    Case {
        // x86-64: a CALL (`fread_compare`) fires `ActionFuncLink`/`ActionExtraPopSetup`
        // placeholders in the call block.
        stem: "nan",
        corpus: "decompiler/datatests/nan.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x1011b7,
        context: &[],
        expect: Expect::DivergentBy("ActionFuncLink/ActionExtraPopSetup call placeholders"),
    },
    Case {
        // 8051: an SFR `write_volatile_1` user-op is injected at a non-entry block
        // (`ActionConstbase` live-inject / volatile mapping) — unported.
        stem: "gh1243-8051-addc",
        corpus: "tests/stages/gh1243-8051-addc.xml",
        archid: "8051:BE:16:default:default",
        sla_rel: "Ghidra/Processors/8051/data/languages/8051.sla",
        entry_space: "CODE",
        entry_off: 0x0,
        context: &[],
        expect: Expect::DivergentBy("volatile-access write_volatile_1 injection (non-entry)"),
    },
    Case {
        // ARM (Thumb): a CALL (`fother`) fires `ActionFuncLink`; the placeholder lands
        // in the entry block AFTER the call so the entry-head rule cannot reach it,
        // and the recovered CFG splits differently around the conditional-execution
        // CBRANCH lattice.
        stem: "condexesub",
        corpus: "decompiler/datatests/condexesub.xml",
        archid: "ARM:LE:32:v8:default",
        sla_rel: "Ghidra/Processors/ARM/data/languages/ARM8_le.sla",
        entry_space: "ram",
        entry_off: 0x402d29a8,
        // The datatest paints `set context TMode 1` over the entry range.
        context: &[("TMode", 1)],
        expect: Expect::DivergentBy("ActionFuncLink placeholder + condexe CFG split"),
    },
    Case {
        // MIPS: a `t9` tracked-context COPY (excluded) plus two CALLs whose
        // `ActionFuncLink` placeholders are unported.
        stem: "gp",
        corpus: "decompiler/datatests/gp.xml",
        archid: "MIPS:BE:32:default:default",
        sla_rel: "Ghidra/Processors/MIPS/data/languages/mips32be.sla",
        entry_space: "ram",
        entry_off: 0x410040,
        context: &[],
        expect: Expect::DivergentBy("ActionFuncLink call placeholders (2 calls)"),
    },
    // --- EXCLUDED case (Rust flow reaches a corpus-data / W4 boundary) ---------
    Case {
        // x86-64: the body CALLs `printf`, which the datatest maps `nocode` (its bytes
        // are not in the image).  The C++ side resolves it through the symbol table;
        // the Rust flow follows the call edge into unmapped bytes (needs the W4 call
        // subsystem to stop at the call), so it is EXCLUDED, not failed.
        stem: "sbyte",
        corpus: "decompiler/datatests/sbyte.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x100000,
        context: &[],
        expect: Expect::Excluded("flow into an unmapped `nocode` call thunk"),
    },
];

// ===========================================================================
// Repo / bootstrap plumbing  (mirrors tests/corpus_bootstrap.rs)
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn find_binaryimage(el: &Rc<Element>) -> Option<Rc<Element>> {
    if el.get_name() == "binaryimage" {
        return Some(Rc::clone(el));
    }
    for c in el.get_children() {
        if let Some(found) = find_binaryimage(c) {
            return Some(found);
        }
    }
    None
}

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    registry
}

struct Bootstrap {
    arch: XmlArchitecture,
}

/// Bootstrap the XML frontend for one case (extract `<binaryimage>`, build the
/// loader, scan `.ldefs`, resolve the archid, decode the `.sla`, paint context
/// defaults, open the image).  Mirrors `tests/corpus_bootstrap.rs::bootstrap`.
fn bootstrap(c: &Case) -> Result<Bootstrap, String> {
    let root = repo_root();
    let corpus_path = root.join(c.corpus);
    let sla_path = root.join("specs").join(c.sla_rel);
    if !sla_path.is_file() {
        return Err(format!("{} not built (run `make specs`)", sla_path.display()));
    }

    let registry = build_registry();
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture(corpus_path.to_str().unwrap(), "");

    let xml = std::fs::read(&corpus_path).map_err(|e| format!("read corpus: {e}"))?;
    let mut store = DocumentStorage::new();
    let doc_root =
        store.parse_document(&xml).map_err(|e| format!("parse corpus: {e}"))?.get_root().clone();
    let bi = find_binaryimage(&doc_root).ok_or("corpus has no <binaryimage>")?;
    arch.build_loader(Rc::clone(&bi)).map_err(|e| format!("build_loader: {e}"))?;

    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;

    let archattr = bi.get_attribute_value("arch").map_err(|e| format!("binaryimage arch: {e}"))?;
    let arch_id = String::from_utf8_lossy(archattr).into_owned();
    arch.sleigh_mut().set_archid(&arch_id);
    arch.sleigh_mut()
        .resolve_architecture(&db, &arch_id)
        .map_err(|e| format!("resolve_architecture: {e}"))?;
    if arch.sleigh().language_index() < 0 {
        return Err("language index unresolved".to_string());
    }

    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file did not resolve a .sla")?;
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read resolved sla: {e}"))?;
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .map_err(|e| format!("build_translator: {e}"))?;

    // Apply the processor-spec context-register defaults (`<context_data>
    // <context_set>` in the `.pspec`).  This is the `SleighArchitecture::buildContext`
    // / `ContextDatabase::setVariableDefault` seam the C++ side runs at architecture
    // build — WITHOUT it the SLEIGH decode of a context-sensitive ISA (x86's
    // `longMode`/`opsize`/`addrsize`, ARM `TMode`, ...) produces a different
    // instruction stream than the C++ B2 snapshot.  We apply the pspec defaults
    // generically, then the per-case overrides (the datatest's `set context ..`).
    let pspec_ctx = parse_pspec_context(&specs.processorfile)
        .map_err(|e| format!("parse pspec context: {e}"))?;
    {
        let base = arch.sleigh_mut().base_mut().ok_or("translator base missing")?;
        for (name, val) in &pspec_ctx {
            base.translate_mut().set_context_default(name, *val);
        }
        for (name, val) in c.context {
            base.translate_mut().set_context_default(name, *val);
        }
    }

    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager outlives the open() call (it lives inside `arch`).
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().translate_mut().set_loader(Box::new(img));

    let _ = registry;
    Ok(Bootstrap { arch })
}

/// Parse the `<context_data><context_set><set name=.. val=..>` defaults from a
/// `.pspec` (the processor-spec context-register defaults).  Returns `(name,
/// value)` pairs.  Uses the ported `DocumentStorage` XML reader (the same one the
/// engine uses), walking the parse tree for `set` elements under `context_set`.
fn parse_pspec_context(pspec_path: &str) -> Result<Vec<(String, u32)>, String> {
    if pspec_path.is_empty() {
        return Ok(Vec::new());
    }
    let bytes = std::fs::read(pspec_path).map_err(|e| format!("read {pspec_path}: {e}"))?;
    let mut store = DocumentStorage::new();
    let root = store.parse_document(&bytes).map_err(|e| format!("{e}"))?.get_root().clone();
    let mut out = Vec::new();
    collect_context_sets(&root, false, &mut out)?;
    Ok(out)
}

/// Recursively collect `<set name val>` children of any `<context_set>` element.
/// `in_context_set` tracks whether the current element is (under) a `context_set`.
fn collect_context_sets(
    el: &Rc<Element>,
    in_context_set: bool,
    out: &mut Vec<(String, u32)>,
) -> Result<(), String> {
    let name = el.get_name();
    let now_in = in_context_set || name == "context_set";
    if now_in && name == "set" {
        let n = el
            .get_attribute_value("name")
            .map_err(|e| format!("set@name: {e}"))?;
        let v = el.get_attribute_value("val").map_err(|e| format!("set@val: {e}"))?;
        let name_s = String::from_utf8_lossy(n).into_owned();
        let val_s = String::from_utf8_lossy(v).into_owned();
        let val: u32 = val_s
            .trim()
            .parse()
            .map_err(|e| format!("set val {val_s:?}: {e}"))?;
        out.push((name_s, val));
    }
    for c in el.get_children() {
        collect_context_sets(c, now_in, out)?;
    }
    Ok(())
}

// ===========================================================================
// FlowEnvironment + IR-boundary manager (mirrors flow_linkage.rs)
// ===========================================================================

fn typeop_for(opc: OpCode) -> TypeOp {
    let flags = match opc {
        OpCode::CPUI_BRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_CBRANCH => pcodeop_flags::branch | pcodeop_flags::coderef,
        OpCode::CPUI_BRANCHIND => pcodeop_flags::branch,
        OpCode::CPUI_CALL => pcodeop_flags::call | pcodeop_flags::coderef,
        OpCode::CPUI_CALLIND => pcodeop_flags::call,
        OpCode::CPUI_CALLOTHER => pcodeop_flags::call,
        OpCode::CPUI_RETURN => pcodeop_flags::branch | pcodeop_flags::returns,
        OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL => pcodeop_flags::marker,
        _ => 0,
    };
    TypeOp::new(opc, flags, "OP")
}

struct DiffEnv<'a> {
    sleigh: &'a Sleigh,
}
impl FlowEnvironment for DiffEnv<'_> {
    fn translate(&self) -> &dyn Translate {
        self.sleigh
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        typeop_for(opc)
    }
}

fn ir_boundary_manager(src: &AddrSpaceManager) -> AddrSpaceManager {
    use kuna_base::space::{
        addrspace_flags, AddrSpace, ConstantSpace, FspecSpace, IopSpace, UniqueSpace,
    };
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).expect("const");
    let big_end = src.get_default_code_space().map(|s| s.is_big_endian()).unwrap_or(false);
    let uniq_index = m.num_spaces();
    m.insert_space(Rc::new(UniqueSpace::new(uniq_index, 0, big_end))).expect("unique");
    if let Some(code) = src.get_default_code_space() {
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            code.get_name(),
            code.is_big_endian(),
            code.get_addr_size(),
            code.get_word_size(),
            m.num_spaces(),
            addrspace_flags::hasphysical,
            code.get_delay(),
            1,
        );
        let _ = m.insert_space(Rc::new(ram));
    }
    let next = m.num_spaces();
    let _ = m.insert_space(Rc::new(IopSpace::new(next)));
    let next = m.num_spaces();
    let _ = m.insert_space(Rc::new(FspecSpace::new(next)));
    m
}

#[allow(dead_code)]
struct NullEmit;
impl PcodeEmit for NullEmit {
    fn dump(&mut self, _a: &Address, _o: OpCode, _ov: Option<&VarnodeData>, _v: &[VarnodeData]) {}
}

// ===========================================================================
// The Rust-side extractor: followFlow + block build -> StructModel
// ===========================================================================

/// Run the real `FlowInfo` follow-flow (`generate_ops`) + the basic-block builder
/// (`generate_blocks`) + `structure_reset` for the case's entry, then read the
/// recovered basic-block CFG into a [`StructModel`] (the SAME boundary the C++ B2
/// snapshot represents).  On a W4 seam in `generate_ops` returns `Err(reason)`.
fn extract_rust_model(bs: &Bootstrap, c: &Case) -> Result<StructModel, String> {
    let base = bs.arch.sleigh().base().unwrap();
    let space = Rc::clone(
        base.manage().get_space_by_name(c.entry_space).ok_or("entry space missing")?,
    );
    let entry = Address::new(Rc::clone(&space), c.entry_off);
    let uniq_start = base.translate().get_unique_start(UniqueLayout::ANALYSIS);
    let ir_arch = Rc::new(IrArch::new(ir_boundary_manager(base.manage())));
    let fd = Funcdata::new(c.stem, c.stem, ir_arch, entry, uniq_start, 0x40)
        .map_err(|e| format!("Funcdata::new: {e}"))?;

    let env = DiffEnv { sleigh: base.translate() };
    let mut flow = FlowInfo::new(fd, &env);
    flow.generate_ops().map_err(|e| format!("EXCLUDED: generate_ops {}", seam_reason(&e)))?;
    flow.generate_blocks()
        .map_err(|e| format!("EXCLUDED: generate_blocks {}", seam_reason(&e)))?;
    flow.data.structure_reset();

    let fd = &flow.data;
    let nblocks = fd.bblocks_get_size();

    // Gather every op (alive after block build), keyed by parent block, in print
    // order (the basic-block list order == the seqnum `order` field).
    use std::collections::BTreeMap;
    let mut per_block: BTreeMap<i32, Vec<(u32, OpRec)>> = BTreeMap::new();
    for op in fd.obank().iter_alive() {
        let o = fd.obank().get(op).expect("stale op");
        let parent = match o.get_parent() {
            Some(p) => p,
            None => continue, // not assigned to a block (should not happen post-build)
        };
        let bidx = fd.bblocks_ref().block(parent).get_index();
        let order = o.get_seq_num().get_order();
        let rec = OpRec {
            addr: o.get_addr().get_offset(),
            class: OpClass::from_opcode(o.code()),
            has_output: o.get_out().is_some(),
        };
        per_block.entry(bidx).or_default().push((order, rec));
    }

    let mut blocks: BTreeMap<u64, BlockRec> = BTreeMap::new();
    for i in 0..nblocks {
        let bl = fd.bblocks_get_block(i);
        let fb = fd.bblocks_ref().block(bl);
        let index = fb.get_index();
        // Cover start/stop (the BlockBasic `cover` RangeList — the same range
        // `BlockBasic::printHeader` renders as `getStart()-getStop()`).
        let (start, stop) = block_cover_range(fd, bl);

        let mut ops_ordered = per_block.remove(&index).unwrap_or_default();
        ops_ordered.sort_by_key(|(ord, _)| *ord);
        let ops: Vec<OpRec> = ops_ordered.into_iter().map(|(_, r)| r).collect();

        // Out-edge target START offsets.
        let nout = fb.size_out();
        let mut out_starts: Vec<u64> = Vec::new();
        for k in 0..nout {
            let tgt = fb.get_out(k);
            let (ts, _) = block_cover_range(fd, tgt);
            out_starts.push(ts);
        }
        out_starts.sort_unstable();
        out_starts.dedup();

        blocks.insert(start, BlockRec { index: index as i64, start, stop, ops, out_starts });
    }

    Ok(StructModel { blocks })
}

/// Read a basic block's cover `(start, stop)` byte-offsets the way
/// `FlowBlock::printHeader` renders them (`getStart()` = the first address in the
/// cover, `getStop()` = the last covered address, i.e. the END of the last range
/// minus one byte — matching `BlockBasic::getStop`).
fn block_cover_range(fd: &Funcdata, bl: kuna_decomp::seams::BlockId) -> (u64, u64) {
    let fb = fd.bblocks_ref().block(bl);
    let cover = match fb.kind() {
        kuna_decomp::block::BlockKind::Basic(b) => &b.cover,
        _ => return (0, 0),
    };
    // The cover is a RangeList; getStart() is the first range's first address,
    // getStop() is the last range's last address.
    let start = cover.get_first_range().map(|r| r.get_first()).unwrap_or(0);
    let stop = cover.get_last_range().map(|r| r.get_last()).unwrap_or(start);
    (start, stop)
}

/// Reduce a flow error to a short EXCLUDED reason: either a W4 seam reached in
/// the flow path (jump-table / inject / FuncCallSpecs) or a corpus-data boundary
/// (a CALL into an unmapped thunk the datatest deliberately omits — e.g. a
/// `nocode` PLT stub — which the C++ side resolves via the symbol table and the
/// Rust flow cannot follow without the W4 call subsystem).
fn seam_reason(e: &KunaError) -> String {
    let s = e.to_string();
    if s.contains("jump-table") || s.contains("JumpTable") {
        "BRANCHIND jump-table recovery (W4)".into()
    } else if s.contains("inject") || s.contains("PcodeInject") {
        "CALLOTHER/inject payload (W4)".into()
    } else if s.contains("FuncCallSpecs") || s.contains("inline") {
        "FuncCallSpecs/inline (W4)".into()
    } else if s.contains("not mapped") || s.contains("Bytes at") || s.contains("unavail") {
        "flow into an unmapped call thunk (corpus omits the callee; needs the W4 call subsystem)"
            .into()
    } else {
        format!("flow seam: {s}")
    }
}

// ===========================================================================
// The gate test
// ===========================================================================

#[derive(Debug)]
enum Outcome {
    Pass { excluded_tracked: usize, blocks: usize, ops: usize },
    Divergent { detail: String, excluded_tracked: usize },
    Excluded { reason: String },
    Skipped { reason: String },
}

#[test]
fn lift_diff_b2_structural() {
    let root = repo_root();
    let fixtures = root.join("rust/crates/kuna-harness/tests/fixtures");

    let mut results: Vec<(&str, Expect, Outcome)> = Vec::new();

    for c in CASES {
        // Load + parse the committed C++ B2 fixture.
        let fix_path = fixtures.join(format!("{}.b2.txt", c.stem));
        let text = std::fs::read_to_string(&fix_path)
            .unwrap_or_else(|e| panic!("read fixture {}: {e}", fix_path.display()));
        let mut excluded_tracked = 0usize;
        let cpp = liftdiff::parse_cpp_b2(&text, &mut excluded_tracked)
            .unwrap_or_else(|e| panic!("{}: {e}", c.stem));

        // Bootstrap + extract the Rust model.
        let bs = match bootstrap(c) {
            Ok(bs) => bs,
            Err(e) if e.contains("not built") => {
                results.push((c.stem, c.expect, Outcome::Skipped { reason: e }));
                continue;
            }
            Err(e) => panic!("{}: bootstrap failed: {e}", c.stem),
        };
        // Sanity: the resolved archid matches the pin.
        assert_eq!(
            bs.arch.sleigh().arch_id(),
            c.archid,
            "{}: resolved archid != pinned",
            c.stem
        );

        let rust = match extract_rust_model(&bs, c) {
            Ok(m) => m,
            Err(reason) if reason.starts_with("EXCLUDED:") => {
                let reason = reason.trim_start_matches("EXCLUDED:").trim().to_string();
                results.push((c.stem, c.expect, Outcome::Excluded { reason }));
                continue;
            }
            Err(reason) => panic!("{}: extract failed: {reason}", c.stem),
        };

        let outcome = match liftdiff::diff(&rust, &cpp) {
            DiffResult::Pass => Outcome::Pass {
                excluded_tracked,
                blocks: cpp.blocks.len(),
                ops: cpp.total_ops(),
            },
            DiffResult::Divergent(detail) => Outcome::Divergent { detail, excluded_tracked },
        };
        results.push((c.stem, c.expect, outcome));
    }

    // --- Report the per-test table -----------------------------------------
    eprintln!("\n=== lift_diff B2 structural differential ===");
    eprintln!(
        "{:<20} {:<10} {:>6} {:>6} {:>10}  detail",
        "fixture", "result", "blocks", "ops", "tracked-ex"
    );
    let mut npass = 0usize;
    let mut nskip = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for (stem, expect, outcome) in &results {
        match outcome {
            Outcome::Pass { excluded_tracked, blocks, ops } => {
                npass += 1;
                eprintln!(
                    "{stem:<20} {:<10} {blocks:>6} {ops:>6} {excluded_tracked:>10}",
                    "PASS"
                );
                if !matches!(expect, Expect::Pass) {
                    mismatches.push(format!("{stem}: expected non-PASS but PASSed"));
                }
            }
            Outcome::Divergent { detail, excluded_tracked } => {
                let note = match expect {
                    Expect::DivergentBy(n) => *n,
                    _ => "UNEXPECTED divergence",
                };
                eprintln!(
                    "{stem:<20} {:<10} {:>6} {:>6} {excluded_tracked:>10}  [{note}] {detail}",
                    "DIVERGENT", "-", "-"
                );
                if !matches!(expect, Expect::DivergentBy(_)) {
                    mismatches.push(format!("{stem}: UNEXPECTED divergence: {detail}"));
                }
            }
            Outcome::Excluded { reason } => {
                let note = match expect {
                    Expect::Excluded(n) => *n,
                    _ => "UNEXPECTED exclusion",
                };
                eprintln!(
                    "{stem:<20} {:<10} {:>6} {:>6} {:>10}  [{note}] {reason}",
                    "EXCLUDED", "-", "-", "-"
                );
                if !matches!(expect, Expect::Excluded(_)) {
                    mismatches.push(format!("{stem}: UNEXPECTED exclusion: {reason}"));
                }
            }
            Outcome::Skipped { reason } => {
                nskip += 1;
                eprintln!("{stem:<20} {:<10} {:>6} {:>6} {:>10}  {reason}", "SKIP", "-", "-", "-");
            }
        }
    }
    eprintln!(
        "\n{npass}/{} PASS ({} skipped: specs unbuilt)\n",
        CASES.len(),
        nskip
    );

    // Every case's outcome must match its documented expectation (the WHY of each
    // divergence/exclusion is a checked invariant) — unless the whole run skipped.
    if nskip != CASES.len() {
        assert!(
            mismatches.is_empty(),
            "case outcomes diverge from their documented expectations:\n  {}",
            mismatches.join("\n  ")
        );
    }

    // --- Gate: >= 8 corpus tests structurally equal end-to-end ---------------
    // (>= 12 corpus cases span the architectures; the gate requires >= 8 PASS,
    // with every DIVERGENT/EXCLUDED case attributed to a named unported
    // pre-heritage action or a W4 flow seam in the table above.)  When specs are
    // unbuilt (a bare checkout) every case skips and the gate is a structural pass.
    if nskip == CASES.len() {
        eprintln!("all cases skipped (no specs built) — structural pass");
        return;
    }
    assert!(
        CASES.len() >= 12,
        "gate requires >= 12 corpus cases spanning architectures (have {})",
        CASES.len()
    );
    assert!(
        npass >= 8,
        "gate requires >= 8 structurally-equal corpus tests (got {npass} of {}); \
         exclusions/divergences are reported in the table above",
        CASES.len()
    );
}
