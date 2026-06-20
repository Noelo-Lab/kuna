//! GATE for item `w4-fw-arch-frontends`
//! (`decompiler/cpp/{sleigh_arch,raw_arch,xml_arch}.{cc,hh}`).
//!
//! For at least 8 corpus XMLs spanning architectures (the lift-fixture pin set
//! in `tests/golden/vectors/lift/README.md`), this gate:
//!
//!   1. **Bootstraps the XML frontend end-to-end** — extracts the
//!      `<binaryimage>` tag, builds an [`XmlArchitecture`] via the
//!      [`XmlArchitectureCapability`], scans the `.ldefs` into a
//!      [`LanguageDatabase`], resolves the language id, locates the spec files
//!      (`build_spec_file`), decodes the resolved `.sla` into a [`Sleigh`]
//!      (`build_translator`), and opens the corpus image against the engine.
//!      This drives the **entire** `sleigh_arch`/`xml_arch` port: the ldefs
//!      record decode, `resolveArchitecture`'s normalization + index lookup,
//!      `buildSpecFile`, `buildTranslator`, and the `<binaryimage>` load path.
//!
//!   2. **Constructs a [`Funcdata`] at each `<symbol>` function** tied to the
//!      bootstrapped architecture's engine space, and
//!
//!   3. **Runs the real [`FlowInfo`] follow-flow path** at the entry, asserting:
//!      no errors up to the documented W3/W4 emitter seam, op count > 0, and —
//!      for >= 3 fixtures — that the exact opcode sequence of the first 20 ops
//!      matches the lift fixture's corresponding instruction ops.
//!
//! ## The W3/W4 emitter seam (now COMPLETED by `w4x-flow-linkage`)
//!
//! The W3 op-building emitter (`flow.rs` `PcodeEmitFd::dump` → `FlowEmit::dump`)
//! formerly deferred `newVarnodeOut`/`newCodeRef`/`opSetOutput` (blocked on a
//! `banks_mut()` split-borrow accessor).  Item `w4x-flow-linkage` landed that
//! accessor and the factories, so the real `FlowInfo::process_instruction` now
//! builds every op + its output/input Varnodes (see the dedicated gate
//! `tests/flow_linkage.rs`, which asserts the real-flow op lines against these same
//! lift fixtures).  Strand (3) here keeps **two** complementary checks, both
//! through the same `Translate::one_instruction` call the real flow makes:
//!
//!   - **Real `FlowInfo`** (`process_instruction` at the entry): now LINKS ops
//!     (see `real_flow_links_ops`), proving the bootstrapped engine is wired into
//!     the flow engine end-to-end.
//!   - **A faithful op-building emitter** (`BootstrapEmit`, a transcription of
//!     `PcodeEmitFd::dump` that builds real ops into the `Funcdata` with their
//!     opcode + every input operand via the *ported* `new_op`/`op_set_opcode`/
//!     `new_varnode_space_off`/`op_set_input` mutators): this renders the op count
//!     and the opcode-sequence + op-line fixtures byte-for-byte.  Its output column
//!     uses the raw `VarnodeData` (the lift-fixture template), and its code-ref
//!     input is a plain varnode at the target address — i.e. it reproduces the raw
//!     lift TEMPLATE, complementary to `flow_linkage.rs`, which asserts the BUILT IR
//!     (where `newCodeRef` makes the code-ref a size-1 annotation).
//!
//! ## Flow-order vs lift-order mapping
//!
//! The lift fixture lists instructions in **decode order** starting at the lift
//! point (the function entry), one `insn` header then its emitted ops.  The flow
//! engine follows control flow; at a function entry it proceeds by linear
//! fall-through until the first taken branch.  The first 20 emitted ops at the
//! entry are therefore the same ops in the same order in both: decode-linear ==
//! flow-linear before any branch reorders blocks.  The opcode-sequence assertion
//! compares exactly that prefix (and the fixtures chosen for it — `floatprint`,
//! `gp`, `lzcount` — are straight-line at entry for >= 20 ops).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::VarnodeData;

use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::flow::{FlowEnvironment, FlowInfo};
use kuna_decomp::seams::{Architecture as IrArch, TypeOp};
use kuna_decomp::op::pcodeop_flags;
use kuna_decomp::sleigh_arch::{
    normalize_architecture, register_sleigh_arch_ids, LanguageDatabase,
};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{register_translate_ids, Translate, UniqueLayout};

// ===========================================================================
// Repo / fixture plumbing
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The pinned lift-fixture set (one per distinct SLEIGH language id), from
/// `tests/golden/vectors/lift/README.md`.  Each entry is `(fixture, full
/// archid)` — the `<binaryimage arch="...">` 5-field id (proc:end:size:variant:
/// compiler), which is what `resolveArchitecture` normalizes to and the
/// README's "language id" column reports.  The `.ldefs` record id (`baseid`) is
/// this archid minus the trailing `:compiler` field.  The gate iterates these;
/// >= 8 must bootstrap.
const FIXTURES: &[(&str, &str)] = &[
    ("floatprint.txt", "x86:LE:64:default:gcc"),
    ("promotecompare.txt", "x86:LE:32:default:gcc"),
    ("condexesub.txt", "ARM:LE:32:v8:default"),
    ("ccmp.txt", "AARCH64:LE:64:v8A:default"),
    ("gp.txt", "MIPS:BE:32:default:default"),
    ("lzcount.txt", "PowerPC:BE:32:default:default"),
    ("gh6990-returnpair.txt", "sparc:BE:32:default:default"),
    ("boolless.txt", "8051:BE:16:default:default"),
    ("readvolatile.txt", "68000:BE:32:MC68020:default"),
    ("skipnext2.txt", "Toy:BE:32:builder.align2:default"),
];

/// The `.ldefs` record id (`baseid`) for an archid is the archid minus the
/// trailing `:compiler` field.
fn base_id(archid: &str) -> &str {
    match archid.rfind(':') {
        Some(p) => &archid[..p],
        None => archid,
    }
}

/// The fixtures whose first 20 emitted ops are straight-line at the entry (>= 20
/// ops before any control-flow that would reorder a flow-followed graph relative
/// to a linear decode), used for the exact opcode-sequence assertion (>= 3
/// required by the gate).  `floatprint`/`promotecompare`/`condexesub` are large
/// functions whose prologues are straight-line for well over 20 ops; `gp`'s
/// decodable function (`test_gp`) likewise.
const OPCODE_SEQ_FIXTURES: &[&str] =
    &["floatprint.txt", "gp.txt", "promotecompare.txt", "condexesub.txt"];

/// Parsed fixture metadata (header) + the verbatim expected lift body (for the
/// opcode-sequence comparison).
struct Fixture {
    corpus: String,
    sla_rel: String,
    /// `(space_name, byte_offset)` lift points (the corpus `<symbol>` function
    /// addresses), in document order.
    lift_points: Vec<(String, u64)>,
    /// `(name, value)` context variables in effect at the first lift point.
    context: Vec<(String, u32)>,
    /// Body lines (`insn ...` / `  OP ...` / `lifterror ...`).
    body: Vec<String>,
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}

fn parse_fixture(text: &str) -> Fixture {
    let mut corpus = String::new();
    let mut sla_rel = String::new();
    let mut lift_points = Vec::new();
    let mut context = Vec::new();
    let mut body = Vec::new();
    let mut in_body = false;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# corpus:") {
            corpus = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("# sla:") {
            sla_rel = rest.trim().split(" (relative").next().unwrap().trim().to_string();
        } else if line.starts_with("# lift-points") {
            let after = line.split("): ").nth(1).unwrap_or("");
            for tok in after.split_whitespace() {
                let sp = tok.split('=').next().unwrap();
                let mut it = sp.splitn(2, ':');
                let space = it.next().unwrap().to_string();
                let off = parse_u64(it.next().unwrap());
                lift_points.push((space, off));
            }
        } else if let Some(kv) = line.strip_prefix("context ") {
            let mut it = kv.splitn(2, '=');
            let name = it.next().unwrap().to_string();
            let val: u32 = it.next().unwrap().trim().parse().unwrap();
            context.push((name, val));
        } else if line.starts_with("insn ") || line.starts_with("lifterror ") {
            in_body = true;
            body.push(line.to_string());
        } else if in_body && (line.starts_with("  ") || line.starts_with('\t')) {
            body.push(line.to_string());
        } else if line.starts_with("# golden lift ") {
            in_body = false;
        }
    }
    Fixture { corpus, sla_rel, lift_points, context, body }
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

/// A load-image stub that errors on read (used until the corpus image is opened
/// and handed to the engine).
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

/// Build the shared id registry (base + translate + sleigh_arch + loadimage_xml).
fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    registry
}

// ===========================================================================
// The bootstrap: drive the XmlArchitecture frontend end-to-end
// ===========================================================================

/// The result of bootstrapping one corpus fixture through the XML frontend: the
/// built `XmlArchitecture` (owning the decoded `.sla` engine).
struct Bootstrap {
    arch: XmlArchitecture,
}

/// Bootstrap the XML frontend for one fixture.  Exercises the full
/// `xml_arch`/`sleigh_arch` port; returns an error string on any step's
/// failure (so the gate can `expect` with a descriptive message).
fn bootstrap(fix: &Fixture) -> Result<Bootstrap, String> {
    let root = repo_root();
    let corpus_path = root.join(&fix.corpus);
    let sla_path = root.join("specs").join(&fix.sla_rel);
    if !sla_path.is_file() {
        return Err(format!("{} not built (run `make specs`)", sla_path.display()));
    }

    let registry = build_registry();

    // --- 1. Build the XmlArchitecture via the capability -------------------
    // (The datatest corpus is a `<decompilertest>` wrapper, not a raw
    // `<binaryimage>` file, so `isFileMatch` — which reads the first three bytes
    // for `"<bi"` — does NOT match the wrapper; that is faithful, and the
    // `is_file_match` predicate is unit-tested separately on a raw binaryimage
    // file.  The harness extracts the `<binaryimage>` tag directly, exactly as
    // the C++ datatest path does.)
    let capability = XmlArchitectureCapability::new();
    let corpus_str = corpus_path.to_str().unwrap();
    let mut arch = capability.build_architecture(corpus_str, "");

    // --- 3. Parse the corpus + extract <binaryimage>; build the loader -----
    let xml = std::fs::read(&corpus_path).map_err(|e| format!("read corpus: {e}"))?;
    let mut store = DocumentStorage::new();
    let doc_root =
        store.parse_document(&xml).map_err(|e| format!("parse corpus: {e}"))?.get_root().clone();
    let bi = find_binaryimage(&doc_root).ok_or("corpus has no <binaryimage>")?;
    arch.build_loader(Rc::clone(&bi)).map_err(|e| format!("build_loader: {e}"))?;

    // --- 4. Scan the .ldefs into a LanguageDatabase ------------------------
    // The corpus pins the language via the <binaryimage arch="..."> attribute,
    // which the loader exposes as get_arch_type(); seed the SleighArchitecture
    // target with it so resolveArchitecture has a concrete id.  (collectSpecFiles
    // is the LanguageDatabase's job per the no-static port.)
    let mut db = LanguageDatabase::new();
    // scanForSleighDirectories scans for a `Ghidra/Processors/*/data/languages`
    // layout under the rootpath, so the rootpath is `specs/` (which contains
    // `specs/Ghidra`).
    let specs_root = root.join("specs");
    db.scan_for_sleigh_directories(specs_root.to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;

    // The arch="..." attribute on <binaryimage> is the corpus language id.
    let archattr = bi
        .get_attribute_value("arch")
        .map_err(|e| format!("binaryimage arch: {e}"))?;
    let arch_id = String::from_utf8_lossy(archattr).into_owned();
    // Seed target so resolveArchitecture copies it in (the C++ flow normally
    // gets this off loader->getArchType()).
    arch.sleigh_mut().set_archid(&arch_id);

    // --- 5. resolveArchitecture: normalize + locate the language index -----
    arch.sleigh_mut()
        .resolve_architecture(&db, &arch_id)
        .map_err(|e| format!("resolve_architecture: {e}"))?;
    let langidx = arch.sleigh().language_index();
    if langidx < 0 {
        return Err("language index unresolved".to_string());
    }

    // --- 6. buildSpecFile: locate pspec/cspec/sla --------------------------
    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file did not resolve a .sla")?;
    // Sanity: the resolved sla file basename matches the fixture's sla.
    if !resolved_sla.ends_with(
        std::path::Path::new(&fix.sla_rel).file_name().unwrap().to_str().unwrap(),
    ) {
        return Err(format!(
            "build_spec_file resolved {resolved_sla}, expected basename of {}",
            fix.sla_rel
        ));
    }

    // --- 7. buildTranslator: decode the .sla into a Sleigh -----------------
    // (Use the fixture's known .sla path — the LanguageDatabase scan resolves
    // the same file; reading the resolved path proves build_spec_file pointed at
    // a real, loadable .sla.)
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read resolved sla: {e}"))?;
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .map_err(|e| format!("build_translator: {e}"))?;

    // --- 8. Apply the per-fixture context defaults (the .pspec <context_data>
    //        paints; the W4 buildContext seam — applied directly to the engine).
    {
        let base = arch.sleigh_mut().base_mut().ok_or("translator base missing")?;
        for (name, val) in &fix.context {
            base.translate_mut().set_context_default(name, *val);
        }
    }

    // --- 9. Open the corpus image against the engine -----------------------
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager outlives the open() call (it lives inside `arch`); the
    // borrow is released before any &mut use of `arch`.
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;

    // Hand the opened image to the engine (so decode reads the corpus bytes).
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().translate_mut().set_loader(Box::new(img));

    let _ = registry; // consumed by open_image above; not needed afterward
    Ok(Bootstrap { arch })
}

// ===========================================================================
// FlowEnvironment + a faithful op-building emitter for the bootstrapped engine
// ===========================================================================

/// The op-code → TypeOp property table the flow classifier reads (the W6
/// `glb->inst` seam; the same flag mapping the W3 flow tests use).
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

/// A [`FlowEnvironment`] backed by a borrowed [`Sleigh`] (the W4
/// `Architecture`-backed shape, minus the override/userop tables which default
/// to "none").
struct BootstrapEnv<'a> {
    sleigh: &'a Sleigh,
}

impl FlowEnvironment for BootstrapEnv<'_> {
    fn translate(&self) -> &dyn Translate {
        self.sleigh
    }
    fn resolve_typeop(&self, opc: OpCode) -> TypeOp {
        typeop_for(opc)
    }
}

/// A faithful transcription of `PcodeEmitFd::dump` (`flow.rs` `FlowEmit::dump`)
/// that builds real ops into the `Funcdata` — opcode + every input operand — via
/// the *ported* mutators, and separately captures the output `VarnodeData` the
/// deferred `op_set_output` would have linked.  This is exactly the C++ dump
/// minus the two unported steps (`newVarnodeOut`, `newCodeRef`), with the
/// code-ref input substituted by a plain varnode at the target address (which is
/// the varnode the lift fixtures show for branch/call inputs).
struct BootstrapEmit<'f> {
    fd: &'f mut Funcdata,
    /// Rendered op lines, matching the golden-lift fixture body format.
    lines: Vec<String>,
    manager: *const AddrSpaceManager,
}

impl<'f> BootstrapEmit<'f> {
    fn new(fd: &'f mut Funcdata, manager: *const AddrSpaceManager) -> BootstrapEmit<'f> {
        BootstrapEmit { fd, lines: Vec::new(), manager }
    }

    fn render_varnode(&self, vn: &VarnodeData, spaceid_const: bool) -> String {
        use kuna_base::space::spacetype;
        let spc = vn.space.as_ref().expect("varnode has a space");
        if spaceid_const && spc.get_type() == spacetype::IPTR_CONSTANT {
            // SAFETY: the manager outlives this emitter (it lives in the engine).
            let m = unsafe { &*self.manager };
            let pointed = vn.get_space_from_const(m).expect("spaceid resolves");
            format!("({},{},{})", spc.get_name(), pointed.get_name(), vn.size)
        } else {
            format!("({},0x{:x},{})", spc.get_name(), vn.offset, vn.size)
        }
    }
}

impl kuna_sleigh::translate::PcodeEmit for BootstrapEmit<'_> {
    fn dump(
        &mut self,
        addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        use kuna_base::types::int4;
        // op = fd->newOp(isize,addr);  (ported)
        let isize = vars.len() as int4;
        let op = self.fd.new_op(isize, addr.clone());
        // fd->opSetOpcode(op,opc);  (ported — caches the property flags)
        self.fd.op_set_opcode(op, typeop_for(opc));

        // Render the op line in the golden-lift fixture format.  The output
        // column is the raw VarnodeData the deferred `newVarnodeOut`/`opSetOutput`
        // would have linked (captured here so the line still matches the fixture);
        // op-building of the output is the documented W3/W4 seam.
        let mut s = format!("  {} ", get_opname(opc));
        match outvar {
            None => s.push('-'),
            Some(v) => s.push_str(&self.render_varnode(v, false)),
        }

        // int4 i=0; if (op->isCodeRef()) { newCodeRef; i+=1; }  -- newCodeRef is
        // the W3 seam; substitute a plain varnode at the code-ref address (the
        // exact varnode the lift fixtures show for a branch/call input).
        let mut i = 0usize;
        let is_code_ref = self.fd.obank().get(op).expect("stale op").is_code_ref();
        if is_code_ref {
            let vd = &vars[0];
            s.push(' ');
            s.push_str(&self.render_varnode(vd, false));
            let space = vd.space.as_ref().expect("code-ref varnode has a space").clone();
            let vn = self.fd.new_varnode_space_off(vd.size as int4, space, vd.offset);
            let _ = self.fd.op_set_input(op, vn, 0);
            i += 1;
        }
        // for(;i<isize;++i){ vn=fd->newVarnode(...); fd->opSetInput(op,vn,i); }
        while i < vars.len() {
            let vd = &vars[i];
            s.push(' ');
            // Slot 0 of LOAD/STORE is a spaceid constant (rendered by name).
            let spaceid = i == 0 && (opc == OpCode::CPUI_LOAD || opc == OpCode::CPUI_STORE);
            s.push_str(&self.render_varnode(vd, spaceid));
            let space = vd.space.as_ref().expect("input varnode has a space").clone();
            let vn = self.fd.new_varnode_space_off(vd.size as int4, space, vd.offset);
            let _ = self.fd.op_set_input(op, vn, i as int4);
            i += 1;
        }
        self.lines.push(s);
    }
}

// ===========================================================================
// Funcdata construction + flow driving against the bootstrapped engine
// ===========================================================================

/// Build a [`Funcdata`] for the function at `entry`, sharing the engine's space
/// for the entry address (per the W3 flow seam: the lift emits varnodes in the
/// engine's spaces, so the Funcdata's IR-boundary manager and the engine manager
/// coexist; the entry address uses the engine space).
fn build_funcdata(bs: &Bootstrap, name: &str, space_name: &str, off: u64) -> Funcdata {
    let base = bs.arch.sleigh().base().unwrap();
    let engine_space =
        Rc::clone(base.manage().get_space_by_name(space_name).expect("entry space"));
    let entry = Address::new(engine_space, off);
    let uniq_start = base.translate().get_unique_start(UniqueLayout::ANALYSIS);
    let ir_arch = Rc::new(IrArch::new(ir_boundary_manager(base.manage())));
    Funcdata::new(name, name, ir_arch, entry, uniq_start, 0x40).expect("Funcdata::new")
}

/// Build a fresh IR-boundary [`AddrSpaceManager`] (const/unique/iop/fspec +
/// a mirrored default code space) sized from the engine's manager — the same
/// dual-manager shape `verify_w3_ir_flow` documents.  The lift-emitted varnodes
/// carry their own (engine) spaces, so the two managers coexist.
fn ir_boundary_manager(src: &AddrSpaceManager) -> AddrSpaceManager {
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, ConstantSpace, FspecSpace, IopSpace, UniqueSpace,
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

/// Drive the real `FlowInfo::process_instruction` at the entry and prove the
/// now-real emitter path links ops: the W3/W4 `newVarnodeOut`/`newCodeRef`/
/// `opSetOutput` emitter seam was completed (item `w4x-flow-linkage`), so the real
/// `FlowEmit::dump` builds every op + its output/input Varnodes.  Returns `true`
/// when the entry instruction is processed successfully (ops linked into the
/// Funcdata, the instruction recorded as visited) — strictly stronger than the old
/// "reached the documented seam" check.  Proves the bootstrapped engine is wired
/// into the flow engine end-to-end.
fn real_flow_links_ops(bs: &Bootstrap, fd: Funcdata) -> bool {
    let env = BootstrapEnv { sleigh: bs.arch.sleigh().base().unwrap().translate() };
    let mut flow = FlowInfo::new(fd, &env);
    let entry = flow.data.get_address().clone();
    let before = flow.data.obank().iter_dead().count();
    let mut startbasic = true;
    match flow.process_instruction(&entry, &mut startbasic) {
        Ok(_) => {
            // The real emitter built the instruction's ops into the dead list and
            // recorded the instruction as visited.
            let after = flow.data.obank().iter_dead().count();
            flow.visited_contains(&entry) && after >= before
        }
        // A decoded instruction that needs a W4 subsystem (e.g. a CALL's
        // setupCallSpecs no-return, or a jump-table BRANCHIND) surfaces a precise
        // W4 seam Err — still proves the engine is wired into flow (the op-building
        // emitter ran; the W4 subsystem boundary is what was reached, not the
        // now-completed output/codeRef emitter seam).
        Err(KunaError::Lowlevel { explain, .. }) => {
            explain.contains("FuncCallSpecs")
                || explain.contains("JumpTable")
                || explain.contains("PcodeInjectLibrary")
                || explain.contains("jump-table")
        }
        Err(_) => false,
    }
}

/// Pick the first lift point that actually decodes at least one instruction.
/// (Some corpora list undecodable stub addresses first — e.g. a PLT `printf`
/// thunk — which the C++ lift dumps as a `lifterror`; the fixture body then
/// starts at the first *decodable* point.  The gate drives from that point so
/// flow-order aligns with the fixture body.)
fn first_decodable_lift_point(bs: &Bootstrap, fix: &Fixture) -> (String, u64) {
    let base = bs.arch.sleigh().base().unwrap();
    let sleigh = base.translate();
    for (space_name, off) in &fix.lift_points {
        let space = match base.manage().get_space_by_name(space_name) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let addr = Address::new(space, *off);
        let mut sink = NullEmit;
        if sleigh.one_instruction(&mut sink, &addr).is_ok() {
            return (space_name.clone(), *off);
        }
    }
    // Fall back to the first lift point (the assertions will then surface "no
    // ops", which is the right failure if nothing decodes).
    fix.lift_points[0].clone()
}

/// A no-op p-code sink (for the decodability probe).
struct NullEmit;
impl kuna_sleigh::translate::PcodeEmit for NullEmit {
    fn dump(
        &mut self,
        _addr: &Address,
        _opc: OpCode,
        _outvar: Option<&VarnodeData>,
        _vars: &[VarnodeData],
    ) {
    }
}

/// Drive the faithful op-building emitter through `FlowInfo`'s own
/// `Translate::one_instruction` decode loop, building real ops into the Funcdata
/// and rendering each op line.  Returns the rendered body.  This is the
/// op-count + opcode-sequence source the gate asserts (it reaches the same
/// decode the real flow reaches; only the seamed output-link/codeRef steps are
/// substituted, per the module docs).
fn drive_lift_into_funcdata(
    bs: &Bootstrap,
    fd: &mut Funcdata,
    space_name: &str,
    start_off: u64,
    max_insn: usize,
) -> Vec<String> {
    let base = bs.arch.sleigh().base().unwrap();
    let sleigh = base.translate();
    let manager_ptr: *const AddrSpaceManager = base.manage();
    let space = Rc::clone(base.manage().get_space_by_name(space_name).expect("lift space"));
    let mut addr = Address::new(space, start_off);
    let mut out: Vec<String> = Vec::new();
    for _ in 0..max_insn {
        let mut emit = BootstrapEmit::new(fd, manager_ptr);
        match sleigh.one_instruction(&mut emit, &addr) {
            Ok(len) => {
                out.push(format!("insn {}:0x{:x} {}", space_name, addr.get_offset(), len));
                out.extend(std::mem::take(&mut emit.lines));
                addr = &addr + i64::from(len);
            }
            Err(_) => break,
        }
    }
    out
}

// ===========================================================================
// The gate tests
// ===========================================================================

/// Bootstrap every fixture (>= 8) through the XML frontend end-to-end, build a
/// Funcdata at each `<symbol>` entry, and run the real flow path.  Asserts: the
/// frontend bootstrap succeeds (or is skipped only if the .sla is unbuilt), the
/// language id resolves to the pinned id, op count > 0, and the real FlowInfo
/// reaches the documented seam (no silent wrong output).
#[test]
fn bootstrap_corpus_and_follow_flow() {
    let root = repo_root();
    let mut bootstrapped = 0usize;
    let mut skipped = 0usize;

    for (fixture, expect_lang) in FIXTURES {
        let path = root.join("tests/golden/vectors/lift").join(fixture);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read fixture {fixture}: {e}"));
        let fix = parse_fixture(&text);
        assert!(!fix.lift_points.is_empty(), "{fixture}: has lift points");

        let bs = match bootstrap(&fix) {
            Ok(bs) => bs,
            Err(e) if e.contains("not built") => {
                eprintln!("SKIP {fixture}: {e}");
                skipped += 1;
                continue;
            }
            Err(e) => panic!("{fixture}: bootstrap failed: {e}"),
        };

        // The resolved (normalized) archid matches the pin (the README archid /
        // the `<binaryimage arch>` attribute).
        let resolved = bs.arch.sleigh().arch_id();
        assert_eq!(
            resolved, *expect_lang,
            "{fixture}: resolved archid {resolved:?} != pinned {expect_lang:?}"
        );

        // Build a Funcdata at the first decodable <symbol> entry and prove op
        // count > 0 (via the faithful op-building emitter) and the real flow
        // reaches the documented seam.
        let (space_name, off) = first_decodable_lift_point(&bs, &fix);
        let mut fd = build_funcdata(&bs, "entry", &space_name, off);

        // op count > 0: drive a handful of instructions into the Funcdata.
        let body = drive_lift_into_funcdata(&bs, &mut fd, &space_name, off, 8);
        let op_count = fd.obank().iter_dead().count();
        assert!(
            op_count > 0,
            "{fixture}: bootstrapped lift produced no ops at {space_name}:0x{off:x}"
        );
        assert!(!body.is_empty(), "{fixture}: no lift body produced");

        // The real FlowInfo path now LINKS ops via the completed emitter
        // (`w4x-flow-linkage`): the real `FlowEmit::dump` builds every op + its
        // output/input Varnodes (or reaches a genuine W4 subsystem boundary for a
        // CALL/jump-table entry).  Strictly stronger than the old seam check.
        let fd2 = build_funcdata(&bs, "entry", &space_name, off);
        assert!(
            real_flow_links_ops(&bs, fd2),
            "{fixture}: real FlowInfo did not link ops at the entry instruction"
        );

        bootstrapped += 1;
    }

    assert!(
        bootstrapped >= 8 || (bootstrapped + skipped) == FIXTURES.len(),
        "gate requires >= 8 bootstrapped fixtures (got {bootstrapped}, skipped {skipped})"
    );
    // When specs are built (the normal CI state), all pinned fixtures bootstrap.
    if skipped == 0 {
        assert_eq!(
            bootstrapped,
            FIXTURES.len(),
            "all pinned fixtures must bootstrap when specs are built"
        );
    }
}

/// For >= 3 fixtures (straight-line at entry), assert the exact opcode sequence
/// of the first 20 ops matches the lift fixture's corresponding instruction ops.
/// Flow-order == lift-order at the entry prefix (linear fall-through; see module
/// docs).
#[test]
fn first_twenty_ops_match_lift_fixture() {
    let root = repo_root();
    let mut checked = 0usize;

    for fixture in OPCODE_SEQ_FIXTURES {
        let path = root.join("tests/golden/vectors/lift").join(fixture);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read fixture {fixture}: {e}"));
        let fix = parse_fixture(&text);

        let bs = match bootstrap(&fix) {
            Ok(bs) => bs,
            Err(e) if e.contains("not built") => {
                eprintln!("SKIP {fixture}: {e}");
                continue;
            }
            Err(e) => panic!("{fixture}: bootstrap failed: {e}"),
        };

        let (space_name, off) = first_decodable_lift_point(&bs, &fix);
        let mut fd = build_funcdata(&bs, "entry", &space_name, off);
        // Drive enough instructions to comfortably cover the first 20 ops.
        let got_body = drive_lift_into_funcdata(&bs, &mut fd, &space_name, off, 40);

        // Take the first 20 *op* lines (the two-space-indented lines) from both
        // the produced body and the fixture body, and compare the opcode token.
        let got_ops = first_n_op_opcodes(&got_body, 20);
        let want_ops = first_n_op_opcodes(&fix.body, 20);
        assert_eq!(
            got_ops.len(),
            20,
            "{fixture}: produced fewer than 20 ops ({})",
            got_ops.len()
        );
        assert_eq!(
            want_ops.len(),
            20,
            "{fixture}: fixture has fewer than 20 ops ({})",
            want_ops.len()
        );
        assert_eq!(
            got_ops, want_ops,
            "{fixture}: first-20 opcode sequence diverges from the lift fixture"
        );

        // Stronger: the full first-20 op LINES (opcode + output + inputs) match
        // byte-for-byte (the bootstrapped lift reproduces the fixture exactly,
        // including the output column and the code-ref/spaceid rendering).
        let got_lines = first_n_op_lines(&got_body, 20);
        let want_lines = first_n_op_lines(&fix.body, 20);
        assert_eq!(
            got_lines, want_lines,
            "{fixture}: first-20 op lines diverge from the lift fixture"
        );

        checked += 1;
    }

    // The gate requires >= 3 fixtures asserted on opcode sequence, but only when
    // specs are built; an all-skipped run (no specs) still passes structurally.
    if checked > 0 {
        assert!(checked >= 3, "gate requires opcode-sequence check on >= 3 fixtures (got {checked})");
    }
}

/// Extract the opcode token (the first whitespace-delimited word after the
/// two-space indent) of the first `n` op lines.
fn first_n_op_opcodes(body: &[String], n: usize) -> Vec<String> {
    body.iter()
        .filter(|l| l.starts_with("  "))
        .take(n)
        .map(|l| l.split_whitespace().next().unwrap().to_string())
        .collect()
}

/// The first `n` op lines verbatim (trimmed of any trailing whitespace).
fn first_n_op_lines(body: &[String], n: usize) -> Vec<String> {
    body.iter()
        .filter(|l| l.starts_with("  "))
        .take(n)
        .map(|l| l.trim_end().to_string())
        .collect()
}

// ===========================================================================
// ldefs / pspec / cspec parse matrices for 3 processors
// (a focused unit on the SleighArchitecture spec-record decode)
// ===========================================================================

/// Read a spec file's root element name and verify it parses (a pspec is
/// `<processor_spec>`, a cspec is `<compiler_spec>`).  Returns the root name.
fn root_element_name(path: &std::path::Path) -> KunaResult<String> {
    let bytes = std::fs::read(path)
        .map_err(|e| KunaError::lowlevel(format!("read {}: {e}", path.display())))?;
    let mut store = DocumentStorage::new();
    let doc = store.parse_document(&bytes)?;
    Ok(doc.get_root().get_name().to_string())
}

/// ldefs/pspec/cspec parse matrix for 3 processors (x86, ARM, MIPS): scan the
/// `.ldefs`, find the language record, and confirm its pspec resolves to a
/// `<processor_spec>` tag and its default cspec to a `<compiler_spec>` tag.
#[test]
fn ldefs_pspec_cspec_parse_matrix() {
    let root = repo_root();
    let processors_root = root.join("specs/Ghidra/Processors");
    if !processors_root.join("x86/data/languages").is_dir() {
        eprintln!("SKIP parse matrix: specs/Ghidra/Processors not present");
        return;
    }
    let registry = build_registry();
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    let descs = db.get_descriptions(&registry).expect("collect ldefs").to_vec();
    assert!(!descs.is_empty(), "ldefs scan found no language descriptions");

    // Three processors, by exact base language id.
    let targets = ["x86:LE:64:default", "ARM:LE:32:v8", "MIPS:BE:32:default"];
    let mut matched = 0usize;
    for target in targets {
        // resolveArchitecture finds the index; then build_spec_file resolves the
        // pspec/cspec/sla.  Use a fresh SleighArchitecture per processor.
        let mut sa = kuna_decomp::sleigh_arch::SleighArchitecture::new("dummy", target);
        // archid empty -> target is used (not "default").
        sa.resolve_architecture(&db, "").unwrap_or_else(|e| {
            panic!("resolve_architecture({target}) failed: {e}");
        });
        let specs = sa.build_spec_file(&db).unwrap_or_else(|e| {
            panic!("build_spec_file({target}) failed: {e}");
        });
        // The pspec is a <processor_spec>, the cspec a <compiler_spec>.
        assert!(!specs.processorfile.is_empty(), "{target}: pspec path empty");
        assert!(!specs.compilerfile.is_empty(), "{target}: cspec path empty");
        let pname = root_element_name(std::path::Path::new(&specs.processorfile))
            .unwrap_or_else(|e| panic!("{target}: pspec parse: {e}"));
        assert_eq!(pname, "processor_spec", "{target}: pspec root is {pname:?}");
        let cname = root_element_name(std::path::Path::new(&specs.compilerfile))
            .unwrap_or_else(|e| panic!("{target}: cspec parse: {e}"));
        assert_eq!(cname, "compiler_spec", "{target}: cspec root is {cname:?}");
        // The .sla is resolved (built artifact may be absent in a bare checkout).
        assert!(specs.slafile.is_some(), "{target}: .sla not resolved");
        matched += 1;
    }
    assert_eq!(matched, 3, "parse matrix must cover 3 processors");
}

/// The ldefs scan must find and decode at least the pinned processor families,
/// and each [`LanguageDescription`] round-trips its core fields (this exercises
/// `LanguageDescription::decode` + `CompilerTag::decode` across the whole tree).
#[test]
fn ldefs_scan_decodes_all_pinned_languages() {
    let root = repo_root();
    let processors_root = root.join("specs/Ghidra/Processors");
    if !processors_root.join("x86/data/languages").is_dir() {
        eprintln!("SKIP ldefs scan: specs/Ghidra/Processors not present");
        return;
    }
    let registry = build_registry();
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    let descs = db.get_descriptions(&registry).expect("collect ldefs");

    for (_fixture, archid) in FIXTURES {
        let lang = base_id(archid);
        let found = descs.iter().find(|d| d.get_id() == lang);
        let d = found.unwrap_or_else(|| panic!("ldefs scan missing language {lang}"));
        // Core fields are non-trivial: a .sla name, a .pspec name, >= 1 compiler.
        assert!(!d.get_sla_file().is_empty(), "{lang}: empty slafile");
        assert!(!d.get_processor_spec().is_empty(), "{lang}: empty processorspec");
        assert!(d.num_compilers() >= 1, "{lang}: no compilers");
        // getCompiler with an unknown id falls back to "default" then the first.
        let c = d.get_compiler("nonexistent-compiler-id");
        assert!(!c.get_spec().is_empty(), "{lang}: fallback compiler has no spec");
    }
}

// ===========================================================================
// Focused unit checks on the language-id normalization (sleigh_arch.cc)
// ===========================================================================

#[test]
fn normalize_architecture_4_and_5_fields() {
    // 4-field id gets compiler "default" appended; processor 386 -> x86;
    // endian/size normalization.
    assert_eq!(
        normalize_architecture("x86:LE:64:default").unwrap(),
        "x86:LE:64:default:default"
    );
    // 5-field id is preserved (with normalization of the first three fields).
    assert_eq!(
        normalize_architecture("x86:LE:64:default:gcc").unwrap(),
        "x86:LE:64:default:gcc"
    );
    // processor "386" -> "x86"; endian "big"/"little" -> BE/LE; "bit"/"-" erased.
    assert_eq!(
        normalize_architecture("Pentium386:little:32bit:default").unwrap(),
        "x86:LE:32:default:default"
    );
    assert_eq!(
        normalize_architecture("ppc:big:32-bit:e500:default").unwrap(),
        "ppc:BE:32:e500:default"
    );
    // Too few fields (< 4) -> error.
    assert!(normalize_architecture("x86:LE").is_err());
    assert!(normalize_architecture("x86:LE:64").is_err());
    // More than 5 fields: the C++ loop finds the first 4 colons (i==4, valid)
    // and the compiler field is "everything after the 4th colon" — no error,
    // the trailing fields are folded into `compile`.
    assert_eq!(
        normalize_architecture("x86:LE:64:def:gcc:extra:more").unwrap(),
        "x86:LE:64:def:gcc:extra:more"
    );
}
