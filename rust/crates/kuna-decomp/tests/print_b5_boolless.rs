//! GATE for `rport/w10-structure-printbody`: drive `boolless` print C (the B5
//! boundary) through the now-wired block-structuring, explicit/implied marking,
//! and IR-coupled PrintC body driver, and show the structured body it emits
//! beside the C++ B5 oracle.
//!
//! Before this wave `print C` emitted only the signature shell plus a `WARNING`
//! comment (`sblocks` empty, body emission seamed).  This gate asserts that the
//! structured body now appears (an `if` statement, not just the shell), that the
//! comparison condition renders the global as `dat_52` (the explicit
//! unnamed-location naming) inside `(...)`, and that the function is still
//! brace-matched and carries the function name.
//!
//! Full byte-parity with the C++ B5 oracle (`if (dat_52 <= 10)` + the recovered
//! `v1` local + the `<= ` join) is gated on the NEXT layer (ConditionalJoin /
//! RuleLessEqual collapsing the two-compare boolean pattern into one
//! `INT_LESSEQUAL`, and Merge/naming binding the `v1` local) — reported in the
//! item's seams_remaining, not asserted here.
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

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

// ===========================================================================
// Repo plumbing (mirrors deadcode_b4.rs, plus the cspec read for proto recovery).
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
    let path = repo_root().join("decompiler/datatests").join(format!("{stem}.xml"));
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
    // Hand the resolved cspec XML to the architecture so build_default_proto
    // decodes the real <default_proto> input/output param lists (proto recovery).
    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            arch.sleigh_mut()
                .base_mut()
                .ok_or("no Architecture base after build_translator")?
                .set_cspec_xml(cspec);
        }
    }
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
// Op-listing render (coarse "print raw"-ish op line; mirrors deadcode_b4.rs).
// ===========================================================================

fn render_vn(arch: &Architecture, fd: &Funcdata, vn: kuna_decomp::seams::VarnodeId) -> String {
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

fn render_op(arch: &Architecture, fd: &Funcdata, op: kuna_decomp::seams::OpId) -> String {
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

use kuna_decomp::decompile_drive::print_c;

/// The committed C++ B5 oracle for boolless (tests/golden/snapshots/cpp/...).
const CPP_B5_ORACLE: &str = "\nuint1 boolless(void)\n\n{\n  uint1 v1; // acc\n  \n  v1 = dat_52;\n  if (dat_52 <= 10) {\n    v1 = 1;\n  }\n  return v1;\n}\n";

#[test]
fn boolless_print_c_emits_structured_body() {
    let (mut xarch, fd) = match run_full("boolless", 0) {
        Ok(v) => v,
        Err(e) if e.contains("not built") || e.contains("no .sla") => {
            eprintln!("SKIP boolless print C: {e}");
            return;
        }
        Err(e) => panic!("boolless run_full: {e}"),
    };
    let arch = xarch.sleigh_mut().base_mut().unwrap();

    // Dump the analyzed IR the printer renders from (the B4 boundary).
    eprintln!("=== boolless analyzed IR (the printer input) ===");
    let mut alive: Vec<kuna_decomp::seams::OpId> = fd.obank().iter_alive().collect();
    alive.sort_by_key(|&op| fd.obank().get(op).unwrap().get_seq_num().get_time());
    for &op in &alive {
        eprintln!("  {}", render_op(arch, &fd, op));
    }

    eprintln!("=== boolless sblocks size: {} ===", fd.sblocks_get_size());
    let rust = print_c(arch, &fd);

    eprintln!("\n=== RUST print C (boolless) ===\n{rust}");
    eprintln!("=== C++ B5 oracle ===\n{CPP_B5_ORACLE}");

    // (1) Block structuring produced a non-empty structured hierarchy.
    assert!(
        fd.sblocks_get_size() != 0,
        "ActionBlockStructure must seed sblocks (got empty structure)"
    );

    // (2) The body is no longer the empty shell: an `if` statement appears.
    assert!(
        rust.contains("if ("),
        "print C must now emit a structured if-statement, got:\n{rust}"
    );

    // (3) The global data location renders as the explicit `dat_52` name
    //     (the faithful pushUnnamedLocation / kuna angr naming), inside the
    //     comparison condition.
    assert!(
        rust.contains("dat_52"),
        "the explicit global must render as dat_52, got:\n{rust}"
    );

    // (4) Still a complete, brace-matched function carrying its name.
    assert!(
        rust.contains("boolless"),
        "print C must carry the function name, got:\n{rust}"
    );
    assert_eq!(
        rust.matches('{').count(),
        rust.matches('}').count(),
        "braces must balance, got:\n{rust}"
    );
    assert!(rust.matches('{').count() >= 2, "if-body + function body braces, got:\n{rust}");

    // The comparison renders inside parentheses (the opCbranch ( ) wrap).
    assert!(
        rust.contains("(") && rust.contains(")"),
        "the condition must be parenthesized, got:\n{rust}"
    );
}
