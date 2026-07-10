//! GATE for `rport/w10-deadcode-simplify`: ActionDeadCode + the simplification
//! rule bodies reduce the heritaged IR toward the C++ B4 oracle.
//!
//! After the LOSS-132 keystone (single AddrSpaceManager), heritage reaches the
//! real lifted varnodes and produces oracle-exact SSA (`tests/heritage_b3.rs`).
//! This gate measures the NEXT layer: drive `boolless` through the *full*
//! universalAction pipeline and compare the post-pipeline op graph against the
//! C++ B4 snapshot (`tests/golden/snapshots/cpp/boolless/000-B4.txt`).
//!
//! The C++ B4 has ~7 alive ops; the raw lifted IR (B2) has ~21.  Before this
//! wave the pipeline no-op'd most pass bodies and ~23 ops reached the printer.
//! This gate asserts ActionDeadCode + the simplify rules demonstrably reduce
//! the op count toward the oracle (it need not fully reach 7 if merge / type /
//! structuring are the remaining blockers — those are reported, not closed).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::{Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{AddrSpaceManager, RegisterLookup};
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::decompile_drive::decompile_func;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_num::opcodes::OpCode;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

// ===========================================================================
// Repo plumbing (mirrors decompile_e2e.rs / heritage_b3.rs).
// ===========================================================================

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

struct SymbolFn {
    name: String,
    space: String,
    offset: u64,
}

struct DataTest {
    binaryimage: Rc<Element>,
    arch_id: String,
    symbols: Vec<SymbolFn>,
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}

fn find_named(el: &Rc<Element>, name: &str, out: &mut Vec<Rc<Element>>) {
    if el.get_name() == name {
        out.push(Rc::clone(el));
    }
    for c in el.get_children() {
        find_named(c, name, out);
    }
}

fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

fn parse_datatest(stem: &str) -> Result<DataTest, String> {
    let path = repo_root().join("tests/datatests").join(format!("{stem}.xml"));
    let xml = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store
        .parse_document(&xml)
        .map_err(|e| format!("parse {stem}: {e}"))?
        .get_root()
        .clone();
    let mut bis = Vec::new();
    find_named(&root, "binaryimage", &mut bis);
    let binaryimage = bis.into_iter().next().ok_or("no <binaryimage>")?;
    let arch_id = attr(&binaryimage, "arch").ok_or("<binaryimage> has no arch")?;
    let mut syms = Vec::new();
    find_named(&binaryimage, "symbol", &mut syms);
    let symbols: Vec<SymbolFn> = syms
        .iter()
        .filter_map(|s| {
            Some(SymbolFn {
                name: attr(s, "name")?,
                space: attr(s, "space")?,
                offset: parse_u64(&attr(s, "offset")?),
            })
        })
        .collect();
    Ok(DataTest { binaryimage, arch_id, symbols })
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

fn bootstrap(dt: &DataTest) -> Result<XmlArchitecture, String> {
    let root = repo_root();
    let registry = build_registry();
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("datatest", "");
    arch.build_loader(Rc::clone(&dt.binaryimage)).map_err(|e| format!("build_loader: {e}"))?;
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;
    arch.sleigh_mut().set_archid(&dt.arch_id);
    arch.sleigh_mut()
        .resolve_architecture(&db, &dt.arch_id)
        .map_err(|e| format!("resolve_architecture: {e}"))?;
    if arch.sleigh().language_index() < 0 {
        return Err("language index unresolved".to_string());
    }
    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file resolved no .sla")?;
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read sla: {e}"))?;
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .map_err(|e| format!("build_translator: {e}"))?;
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager outlives the open() call; borrow released before &mut use.
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

// ===========================================================================
// Op-listing render (a coarse, deterministic "print raw"-ish op line).
// ===========================================================================

fn render_vn(arch: &Architecture, fd: &Funcdata, vn: kuna_decomp::context::VarnodeId) -> String {
    let v = match fd.vbank().get(vn) {
        Some(v) => v,
        None => return "<stale>".to_string(),
    };
    let loc = v.get_addr().clone();
    let size = v.get_size();
    let trans = arch.translate();
    let spc = match loc.get_space() {
        Some(s) => s,
        None => return "<nospace>".to_string(),
    };
    let regname = trans.get_register_name(spc, loc.get_offset(), size);
    let mut tok = if !regname.is_empty() {
        regname
    } else {
        let mut s = String::new();
        s.push(spc.get_shortcut());
        let _ = loc.print_raw(&mut s);
        s
    };
    let expect = arch.manage().get_default_size();
    if expect != size {
        tok.push(':');
        tok.push_str(&size.to_string());
    }
    tok
}

/// Render one alive op as `<seq>: <out> = <OPCODE>(<ins>)` (a coarse line, just
/// enough to read the reduction and diff op identity against the B4 oracle).
fn render_op(arch: &Architecture, fd: &Funcdata, op: kuna_decomp::context::OpId) -> String {
    let o = fd.obank().get(op).expect("render_op: stale op");
    let sq: &SeqNum = o.get_seq_num();
    let mut s = String::new();
    let _ = sq.get_addr().print_raw(&mut s);
    s.push(':');
    s.push_str(&format!("{:x}", sq.get_time()));
    s.push_str(":\t");
    if let Some(out) = o.get_out() {
        s.push_str(&render_vn(arch, fd, out));
        s.push_str(" = ");
    }
    s.push_str(&format!("{:?}", o.code()));
    s.push('(');
    let n = o.num_input();
    for i in 0..n {
        if i != 0 {
            s.push(',');
        }
        if let Some(inv) = o.get_in(i) {
            s.push_str(&render_vn(arch, fd, inv));
        }
    }
    s.push(')');
    s
}

/// Drive boolless through the full pipeline; return (arch, fd).
fn run_full(stem: &str, which: usize) -> Result<(XmlArchitecture, Funcdata), String> {
    let dt = parse_datatest(stem)?;
    let mut xarch = bootstrap(&dt)?;
    let sym = dt.symbols.get(which).ok_or("symbol index out of range")?;
    let space_name = sym.space.clone();
    let off = sym.offset;
    let name = sym.name.clone();
    let arch = xarch.sleigh_mut().base_mut().ok_or("no Architecture base")?;
    let space = arch
        .manage()
        .get_space_by_name(&space_name)
        .ok_or(format!("no space {space_name}"))?
        .clone();
    let entry = Address::new(space, off);
    let fd = decompile_func(arch, &name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
    Ok((xarch, fd))
}

/// The number of alive op-lines (basic-block op lines) in the C++ B4 snapshot.
fn cpp_b4_op_count(b4: &str) -> usize {
    b4.lines()
        .filter(|l| {
            // A B4 raw op line starts with the seqnum address "0x..:..:".
            l.starts_with("0x") && l.contains(":\t")
        })
        .count()
}

fn b4_snapshot(stem: &str) -> Option<String> {
    let p = repo_root().join("tests/golden/snapshots/cpp").join(stem).join("000-B4.txt");
    std::fs::read_to_string(p).ok()
}

#[test]
fn boolless_b4_deadcode_reduces_toward_oracle() {
    let (xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP boolless: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh().base().unwrap();

    let mut alive: Vec<kuna_decomp::context::OpId> = fd.obank().iter_alive().collect();
    // Stable order: by block index then seqnum time.
    alive.sort_by_key(|&op| {
        let o = fd.obank().get(op).unwrap();
        let blk = o.get_parent().map(|b| fd.bblocks_ref().block(b).get_index()).unwrap_or(-1);
        (blk, o.get_seq_num().get_time())
    });

    let n_alive = alive.len();
    eprintln!("=== boolless B4 (post-pipeline) ===");
    eprintln!("alive ops: {n_alive}");
    for &op in &alive {
        eprintln!("  {}", render_op(arch, &fd, op));
    }

    // Per-opcode tally (the reduction signature).
    use std::collections::BTreeMap;
    let mut by_code: BTreeMap<String, usize> = BTreeMap::new();
    for &op in &alive {
        *by_code.entry(format!("{:?}", fd.obank().get(op).unwrap().code())).or_default() += 1;
    }
    eprintln!("by opcode: {by_code:?}");

    // The C++ B4 snapshot is gitignored (produced by `make` + `kuna.goldens`),
    // so the multiset-overlap *report* runs only when it is present; the
    // reduction *assertions* below are independent of it (they are about the
    // Rust op count) and always bite.
    if let Some(b4) = b4_snapshot("boolless") {
        let cpp_ops = cpp_b4_op_count(&b4);
        let cpp_codes = cpp_b4_opcode_multiset(&b4);
        eprintln!("C++ B4 alive op-lines: {cpp_ops}");
        eprintln!("C++ B4 opcode multiset: {cpp_codes:?}");
        let matched = opcode_multiset_overlap(&by_code, &cpp_codes);
        eprintln!("Rust/B4 opcode-multiset overlap: {matched} / {cpp_ops}");
        eprintln!("--- C++ B4 ---\n{}\n--- end B4 ---", b4);
    } else {
        eprintln!("(no C++ B4 snapshot to compare against; reduction asserts still bite)");
    }

    // The teeth (snapshot-independent): ActionDeadCode + the simplify rules must
    // reduce the raw IR (21 lifted ops, ~25 reaching the printer pre-wave with
    // the rule decompositions) **toward** the C++ oracle (7).  The remaining gap
    // is the proto-recovery (return value) + comparison/branch normalization +
    // block-structuring layers (merge/types/structuring — reported, not closed
    // here).  Require a strict, substantial reduction.
    assert!(
        n_alive < 21,
        "ActionDeadCode + simplify must reduce below the 21 raw-lifted ops; got {n_alive}"
    );
    // Regression guard: the wave demonstrably converged to <= 12 ops (the
    // measured post-wave count is 9).  If a future change regresses the
    // reduction this trips.
    assert!(
        n_alive <= 12,
        "the deadcode+collapse convergence regressed (expected <= 12 ops, got {n_alive})"
    );
    // The constant-fold (RuleCollapseConstants) eliminated every INT_MULT — the
    // `1*0xff` (and siblings) collapsed to constants, then deadcode swept the
    // now-dead products.  No INT_MULT survives in B4.
    assert!(
        !by_code.contains_key("CPUI_INT_MULT"),
        "RuleCollapseConstants must have folded the constant INT_MULT products away"
    );

    // The graph is still well-formed: a RETURN survives.
    let n_return = alive
        .iter()
        .filter(|&&op| fd.obank().get(op).unwrap().code() == OpCode::CPUI_RETURN)
        .count();
    assert!(n_return >= 1, "the RETURN op must survive dead-code elimination");
}

/// Per-opcode multiset of the C++ B4 raw op-lines (the opcode keyword of each
/// `0x..:..:` basic-block op line, mapped to its Rust `CPUI_*` name).
fn cpp_b4_opcode_multiset(b4: &str) -> std::collections::BTreeMap<String, usize> {
    use std::collections::BTreeMap;
    let mut out: BTreeMap<String, usize> = BTreeMap::new();
    for l in b4.lines() {
        if !(l.starts_with("0x") && l.contains(":\t")) {
            continue;
        }
        let body = l.split(":\t").nth(1).unwrap_or("");
        // Recognize the printer's symbolic forms (typeop.cc printRaw):
        let code = if body.contains(" <= ") {
            "CPUI_INT_LESSEQUAL"
        } else if body.contains(" < ") {
            "CPUI_INT_LESS"
        } else if body.contains(" ? ") {
            "CPUI_MULTIEQUAL"
        } else if body.starts_with("goto") || body.contains(" if (") {
            "CPUI_CBRANCH"
        } else if body.starts_with("return") {
            "CPUI_RETURN"
        } else if body.contains(" = ") && !body.contains('(') {
            // `out = in` plain assignment -> COPY (the B4 ACC=#1 / i0x52=i0x52).
            "CPUI_COPY"
        } else {
            "OTHER"
        };
        *out.entry(code.to_string()).or_default() += 1;
    }
    out
}

/// Number of op-classes the Rust IR shares with the C++ B4 (multiset overlap,
/// counting min multiplicity per class).
fn opcode_multiset_overlap(
    rust: &std::collections::BTreeMap<String, usize>,
    cpp: &std::collections::BTreeMap<String, usize>,
) -> usize {
    cpp.iter().map(|(k, &cv)| rust.get(k).copied().unwrap_or(0).min(cv)).sum()
}
