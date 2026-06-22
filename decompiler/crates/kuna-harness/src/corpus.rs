//! The B2 STRUCTURAL-differential corpus driver (shared by the
//! `tests/lift_diff.rs` integration test and the `kuna-lift-diff` self-test bin).
//!
//! This module holds the pinned corpus case table, the `XmlArchitecture`
//! bootstrap (extract `<binaryimage>`, scan `.ldefs`, resolve the archid, decode
//! the `.sla`, paint context defaults, open the image), the Rust-side extractor
//! (`FlowInfo` follow-flow + the basic-block builder + `structure_reset` → a
//! [`StructModel`](crate::liftdiff::StructModel)), and the per-case differential.
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
//!   side at the ENTRY block head (index 0), exact and reported per test.
//! - `ActionPrototypeTypes`'s RETURN-input rewrite is invisible to the structural
//!   compare (RETURN has no output; class unchanged).
//! - `ActionExtraPopSetup`/`ActionFuncLink` add ops ONLY for calls with a non-zero
//!   extrapop / a spacebase; a CALL test where they fired shows up as an op-count /
//!   partition divergence (reported, never hidden).
//!
//! Tests that reach a W4 seam in `generate_ops` (a BRANCHIND/jump-table or an
//! injected CALLOTHER, or a CALL into corpus-omitted bytes) are reported EXCLUDED
//! with the seam reason.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{spacetype, AddrSpaceManager};
use kuna_base::xml::{DocumentStorage, Element};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use kuna_decomp::flow::{FlowEnvironment, FlowInfo};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::seams::{Architecture as IrArch, TypeOp};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{register_translate_ids, PcodeEmit, Translate, UniqueLayout};

use crate::liftdiff::{self, BlockRec, DiffResult, OpClass, OpRec, StructModel};

// ===========================================================================
// Corpus pin set (one entry per fixture; >= 12, spanning architectures)
// ===========================================================================

/// The expected outcome of a case, asserted by the gate (so the documentation of
/// WHY a case diverges is a checked invariant, not just a comment).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Expect {
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
pub struct Case {
    /// Fixture stem (`tests/fixtures/<stem>.b2.txt` is the committed C++ B2 model).
    pub stem: &'static str,
    /// The corpus XML path, relative to the repo root.
    pub corpus: &'static str,
    /// The full (normalized) archid the bootstrap must resolve to.
    pub archid: &'static str,
    /// The `.sla` path relative to `specs/`.
    pub sla_rel: &'static str,
    /// The function entry address space name.
    pub entry_space: &'static str,
    /// The function entry byte-offset (the corpus `<symbol>` decompiled by the
    /// datatest's `lo fu` + `decompile`).
    pub entry_off: u64,
    /// Per-fixture context-default paints (the `set context`/`set track`/.pspec
    /// `<context_data>` in effect at the entry — applied to the engine before
    /// flow, so decode matches the C++ snapshot).  `(name, value)`.
    pub context: &'static [(&'static str, u32)],
    /// The expected, documented outcome (asserted by the gate).
    pub expect: Expect,
}

/// The 15 pinned corpus cases (>= 12).  Ten processor families/variants: x86-64,
/// x86-32, ARM, AARCH64, MIPS, PowerPC, SPARC, 8051, 68000, Toy.
pub const CASES: &[Case] = &[
    // --- clean PASS cases (no calls / no out-of-entry pre-heritage insertions) ---
    Case {
        stem: "lzcount",
        corpus: "tests/datatests/lzcount.xml",
        archid: "PowerPC:BE:32:default:default",
        sla_rel: "Ghidra/Processors/PowerPC/data/languages/ppc_32_be.sla",
        entry_space: "ram",
        entry_off: 0x10020,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "boolless",
        corpus: "tests/datatests/boolless.xml",
        archid: "8051:BE:16:default:default",
        sla_rel: "Ghidra/Processors/8051/data/languages/8051.sla",
        entry_space: "CODE",
        entry_off: 0xa000,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "skipnext2",
        corpus: "tests/datatests/skipnext2.xml",
        archid: "Toy:BE:32:builder.align2:default",
        sla_rel: "Ghidra/Processors/Toy/data/languages/toy_builder_be_align2.sla",
        entry_space: "ram",
        entry_off: 0x10,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "floatprint",
        corpus: "tests/datatests/floatprint.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x4004c7,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "promotecompare",
        corpus: "tests/datatests/promotecompare.xml",
        archid: "x86:LE:32:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86.sla",
        entry_space: "ram",
        entry_off: 0x80662e0,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        stem: "readvolatile",
        corpus: "tests/datatests/readvolatile.xml",
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
        corpus: "tests/datatests/condconst.xml",
        archid: "x86:LE:64:default:gcc",
        sla_rel: "Ghidra/Processors/x86/data/languages/x86-64.sla",
        entry_space: "ram",
        entry_off: 0x1006fa,
        context: &[],
        expect: Expect::Pass,
    },
    Case {
        // SPARC: a FIVE-block CFG.  The register-window `save`/`restore` injects
        // split the single entry/return instructions into a sub-CFG whose blocks
        // share cover-start addresses (blocks 0,1,2 all cover 0x32148; blocks 3,4
        // cover 0x32150).  The index-keyed structural model (liftdiff F1 fix)
        // represents and compares all five; two `ActionConstbase` tracked-context
        // COPYs (DECOMPILE_MODE, didrestore) at the entry head (index 0) are
        // excluded.  The Rust flow reproduces the full sub-CFG => a genuine,
        // validated 5-block / 209-op PASS (NOT a degenerate projection).
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
        corpus: "tests/datatests/ccmp.xml",
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
        corpus: "tests/datatests/convert.xml",
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
        corpus: "tests/datatests/nan.xml",
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
        // CBRANCH lattice (the C++ side has 9 blocks, several sharing cover-starts).
        stem: "condexesub",
        corpus: "tests/datatests/condexesub.xml",
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
        corpus: "tests/datatests/gp.xml",
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
        corpus: "tests/datatests/sbyte.xml",
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

/// The repo root (the worktree containing `specs/`, `decompiler/`, `tests/`).
pub fn repo_root() -> PathBuf {
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

/// A bootstrapped XML frontend for one case.
pub struct Bootstrap {
    pub arch: XmlArchitecture,
}

/// Bootstrap the XML frontend for one case (extract `<binaryimage>`, build the
/// loader, scan `.ldefs`, resolve the archid, decode the `.sla`, paint context
/// defaults, open the image).  Mirrors `tests/corpus_bootstrap.rs::bootstrap`.
///
/// Returns `Err(msg)` where `msg` starts with `"not built"` if the `.sla` is
/// missing (a bare checkout without `make specs`).
pub fn bootstrap(c: &Case) -> Result<Bootstrap, String> {
    let root = repo_root();
    let corpus_path = root.join(c.corpus);
    let sla_path = root.join("specs").join(c.sla_rel);
    if !sla_path.is_file() {
        return Err(format!("not built: {} (run `make specs`)", sla_path.display()));
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

    // Apply the active language's `.ldefs` `<truncate_space>` records (C++
    // `Architecture::restoreFromSpec` -> `SleighArchitecture::modifySpaces`,
    // architecture.cc:631) before the type factory reads the default data
    // space's addr size for the default pointer width.  Mirrors the console
    // `build_engine_and_init` wiring — e.g. PowerPC:BE:32:e500 truncates `ram`
    // to 4 so a `void *` is a 32-bit pointer despite the 64-bit GPR model.
    {
        let langindex = arch.sleigh().language_index();
        let base = arch.sleigh_mut().base_mut().ok_or("translator base missing")?;
        db.modify_spaces(langindex, base.manage())
            .map_err(|e| format!("modify_spaces: {e}"))?;
    }

    // Apply the processor-spec context-register defaults (`<context_data>
    // <context_set>` in the `.pspec`), then the per-case overrides (the datatest's
    // `set context ..`).  Without these a context-sensitive ISA decode (x86
    // longMode/opsize/addrsize, ARM TMode, ...) diverges from the C++ snapshot.
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
fn collect_context_sets(
    el: &Rc<Element>,
    in_context_set: bool,
    out: &mut Vec<(String, u32)>,
) -> Result<(), String> {
    let name = el.get_name();
    let now_in = in_context_set || name == "context_set";
    if now_in && name == "set" {
        let n = el.get_attribute_value("name").map_err(|e| format!("set@name: {e}"))?;
        let v = el.get_attribute_value("val").map_err(|e| format!("set@val: {e}"))?;
        let name_s = String::from_utf8_lossy(n).into_owned();
        let val_s = String::from_utf8_lossy(v).into_owned();
        let val: u32 = val_s.trim().parse().map_err(|e| format!("set val {val_s:?}: {e}"))?;
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
/// snapshot represents).  On a W4 seam in `generate_ops` returns `Err("EXCLUDED:
/// ..")`.
pub fn extract_rust_model(bs: &Bootstrap, c: &Case) -> Result<StructModel, String> {
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

    // Gather every op (alive after block build), keyed by parent block index, in
    // print order (the basic-block list order == the seqnum `order` field).
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

    // Build the ordered block list in INDEX order (== the C++ print/list order at
    // B2).  `bblocks_get_block(i)` walks the list in list order, so the resulting
    // Vec is `blocks[i].index == i`; blocks are keyed by POSITION/index, never by
    // cover-start (several blocks can share a cover-start address — see liftdiff
    // F1).  Edges are keyed by the target block's index.
    let mut blocks: Vec<BlockRec> = Vec::with_capacity(nblocks as usize);
    for i in 0..nblocks {
        let bl = fd.bblocks_get_block(i);
        let fb = fd.bblocks_ref().block(bl);
        let index = fb.get_index();
        let (start, stop) = block_cover_range(fd, bl);

        let mut ops_ordered = per_block.remove(&index).unwrap_or_default();
        // C++ `BlockBasic::printRaw` walks the `op` list in list order; the Rust op
        // list order is reflected by the seqnum `order` field, so sorting by `order`
        // reproduces the C++ print order.
        ops_ordered.sort_by_key(|(ord, _)| *ord);
        let ops: Vec<OpRec> = ops_ordered.into_iter().map(|(_, r)| r).collect();

        let nout = fb.size_out();
        let mut out_indices: Vec<i64> = Vec::new();
        for k in 0..nout {
            let tgt = fb.get_out(k);
            let tidx = fd.bblocks_ref().block(tgt).get_index();
            out_indices.push(tidx as i64);
        }
        out_indices.sort_unstable();
        out_indices.dedup();

        blocks.push(BlockRec { index: index as i64, start, stop, ops, out_indices });
    }

    Ok(StructModel { blocks })
}

/// Read a basic block's cover `(start, stop)` byte-offsets the way
/// `FlowBlock::printHeader` renders them.
fn block_cover_range(fd: &Funcdata, bl: kuna_decomp::seams::BlockId) -> (u64, u64) {
    let fb = fd.bblocks_ref().block(bl);
    let cover = match fb.kind() {
        kuna_decomp::block::BlockKind::Basic(b) => &b.cover,
        _ => return (0, 0),
    };
    let start = cover.get_first_range().map(|r| r.get_first()).unwrap_or(0);
    let stop = cover.get_last_range().map(|r| r.get_last()).unwrap_or(start);
    (start, stop)
}

/// Reduce a flow error to a short EXCLUDED reason.
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
// The per-case differential + the self-test driver
// ===========================================================================

/// The outcome of running one corpus case.
#[derive(Debug)]
pub enum Outcome {
    Pass { excluded_tracked: usize, blocks: usize, ops: usize },
    Divergent { detail: String, excluded_tracked: usize },
    Excluded { reason: String },
    Skipped { reason: String },
}

/// Run one corpus case end-to-end: parse the committed C++ B2 fixture, bootstrap +
/// extract the Rust model, and diff.  `Err` is only for a genuine harness failure
/// (a bad fixture / a bootstrap that is not a spec-missing skip), which the caller
/// should surface loudly.
pub fn run_case(c: &Case) -> Result<Outcome, String> {
    let root = repo_root();
    let fixtures = root.join("decompiler/crates/kuna-harness/tests/fixtures");
    let fix_path = fixtures.join(format!("{}.b2.txt", c.stem));
    let text = std::fs::read_to_string(&fix_path)
        .map_err(|e| format!("read fixture {}: {e}", fix_path.display()))?;
    let mut excluded_tracked = 0usize;
    let cpp = liftdiff::parse_cpp_b2(&text, &mut excluded_tracked)
        .map_err(|e| format!("{}: {e}", c.stem))?;

    let bs = match bootstrap(c) {
        Ok(bs) => bs,
        Err(e) if e.starts_with("not built") => {
            return Ok(Outcome::Skipped { reason: e });
        }
        Err(e) => return Err(format!("{}: bootstrap failed: {e}", c.stem)),
    };
    if bs.arch.sleigh().arch_id() != c.archid {
        return Err(format!(
            "{}: resolved archid {:?} != pinned {:?}",
            c.stem,
            bs.arch.sleigh().arch_id(),
            c.archid
        ));
    }

    let rust = match extract_rust_model(&bs, c) {
        Ok(m) => m,
        Err(reason) if reason.starts_with("EXCLUDED:") => {
            let reason = reason.trim_start_matches("EXCLUDED:").trim().to_string();
            return Ok(Outcome::Excluded { reason });
        }
        Err(reason) => return Err(format!("{}: extract failed: {reason}", c.stem)),
    };

    Ok(match liftdiff::diff(&rust, &cpp) {
        DiffResult::Pass => {
            Outcome::Pass { excluded_tracked, blocks: cpp.blocks.len(), ops: cpp.total_ops() }
        }
        DiffResult::Divergent(detail) => Outcome::Divergent { detail, excluded_tracked },
    })
}

/// The aggregate result of the self-test (used by both the integration test and
/// the `kuna-lift-diff` bin).
pub struct SelfTestReport {
    pub rows: Vec<(&'static str, Expect, Outcome)>,
    pub npass: usize,
    pub nskip: usize,
    /// Cases whose observed outcome contradicts the documented `Expect` (these
    /// must be empty for the gate to pass).
    pub mismatches: Vec<String>,
}

impl SelfTestReport {
    /// All cases skipped (a bare checkout with no specs built) — the gate is a
    /// structural pass in that case.
    pub fn all_skipped(&self) -> bool {
        self.nskip == self.rows.len()
    }

    /// The gate predicate: every documented expectation held AND >= 8 of the >= 12
    /// corpus tests are structurally equal end-to-end (or everything was skipped).
    pub fn passed(&self) -> bool {
        if self.all_skipped() {
            return true;
        }
        self.mismatches.is_empty() && CASES.len() >= 12 && self.npass >= 8
    }

    /// Render the per-case table to a string (the report both the test and the bin
    /// print).
    pub fn render(&self) -> String {
        use std::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "=== lift_diff B2 structural differential ===");
        let _ = writeln!(
            s,
            "{:<20} {:<10} {:>6} {:>6} {:>10}  detail",
            "fixture", "result", "blocks", "ops", "tracked-ex"
        );
        for (stem, expect, outcome) in &self.rows {
            match outcome {
                Outcome::Pass { excluded_tracked, blocks, ops } => {
                    let _ = writeln!(
                        s,
                        "{stem:<20} {:<10} {blocks:>6} {ops:>6} {excluded_tracked:>10}",
                        "PASS"
                    );
                }
                Outcome::Divergent { detail, excluded_tracked } => {
                    let note = match expect {
                        Expect::DivergentBy(n) => *n,
                        _ => "UNEXPECTED divergence",
                    };
                    let _ = writeln!(
                        s,
                        "{stem:<20} {:<10} {:>6} {:>6} {excluded_tracked:>10}  [{note}] {detail}",
                        "DIVERGENT", "-", "-"
                    );
                }
                Outcome::Excluded { reason } => {
                    let note = match expect {
                        Expect::Excluded(n) => *n,
                        _ => "UNEXPECTED exclusion",
                    };
                    let _ = writeln!(
                        s,
                        "{stem:<20} {:<10} {:>6} {:>6} {:>10}  [{note}] {reason}",
                        "EXCLUDED", "-", "-", "-"
                    );
                }
                Outcome::Skipped { reason } => {
                    let _ =
                        writeln!(s, "{stem:<20} {:<10} {:>6} {:>6} {:>10}  {reason}", "SKIP", "-", "-", "-");
                }
            }
        }
        let _ = writeln!(
            s,
            "\n{}/{} PASS ({} skipped: specs unbuilt)",
            self.npass,
            CASES.len(),
            self.nskip
        );
        if !self.mismatches.is_empty() {
            let _ = writeln!(s, "\nEXPECTATION MISMATCHES:");
            for m in &self.mismatches {
                let _ = writeln!(s, "  {m}");
            }
        }
        s
    }
}

/// Run the full corpus differential (every [`CASES`] entry) and aggregate the
/// result.  `Err` only on a harness failure (bad fixture / bootstrap error that is
/// not a spec-missing skip).
pub fn run_self_test() -> Result<SelfTestReport, String> {
    let mut rows: Vec<(&'static str, Expect, Outcome)> = Vec::new();
    let mut npass = 0usize;
    let mut nskip = 0usize;
    let mut mismatches: Vec<String> = Vec::new();

    for c in CASES {
        let outcome = run_case(c)?;
        match &outcome {
            Outcome::Pass { .. } => {
                npass += 1;
                if !matches!(c.expect, Expect::Pass) {
                    mismatches.push(format!("{}: expected non-PASS but PASSed", c.stem));
                }
            }
            Outcome::Divergent { detail, .. } => {
                if !matches!(c.expect, Expect::DivergentBy(_)) {
                    mismatches.push(format!("{}: UNEXPECTED divergence: {detail}", c.stem));
                }
            }
            Outcome::Excluded { reason } => {
                if !matches!(c.expect, Expect::Excluded(_)) {
                    mismatches.push(format!("{}: UNEXPECTED exclusion: {reason}", c.stem));
                }
            }
            Outcome::Skipped { .. } => {
                nskip += 1;
            }
        }
        rows.push((c.stem, c.expect, outcome));
    }

    Ok(SelfTestReport { rows, npass, nskip, mismatches })
}
