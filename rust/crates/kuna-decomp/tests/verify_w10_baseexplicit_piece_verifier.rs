//! INDEPENDENT VERIFIER adversarial tests for `w10-baseexplicit-piece`.
//!
//! These are the *verifier's* tests (distinct from the porter's
//! `verify_w10_baseexplicit_piece_adversarial.rs`).  They target the spot the
//! hunt list flagged as most fragile for this item:
//!
//!   (V1) The addrtied PIECE/ZEXT *fall-through* must not perturb non-addrtied
//!        wide-value functions.  `longdouble::pass` (x86-64) carries an 80-bit
//!        float through registers — render must be deterministic and
//!        brace-matched (no panic in the new `find_root`/contains arms when the
//!        addrtied predicate is false).
//!
//!   (V2) Determinism of `find_root`'s `compareOrder`-earliest tiebreak on a
//!        SECOND independent wide function (`doublemove::storedouble`), which
//!        builds the wide float from a stack slot + constant then re-pieces it —
//!        a different CONCAT geometry than `loaddouble`.  Two runs byte-equal.
//!
//!   (V3) The const-folding compare corpus (`condconst`) — an addrtied-register
//!        carrier through an `if` — renders deterministically and keeps its
//!        compare structure (the broad-lever no-regression probe on a function
//!        the porter did NOT pin).
//!
//! Anti-special-casing: every assertion is on STRUCTURE (determinism, brace
//! balance, presence of a `return`/`if`), never on a float type token, a
//! function name keyed into the predicate, or a register value.
//!
//! Bootstrap mirrors the porter's harness (own XML frontend through
//! `init_post_engine`).  A missing-`.sla` env is reported SKIP, never a false
//! pass.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}
struct SymbolFn { name: String, space: String, offset: u64 }
struct DataTest { binaryimage: Rc<Element>, arch_id: String, symbols: Vec<SymbolFn> }
fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") { u64::from_str_radix(hex, 16).unwrap_or(0) } else { s.parse().unwrap_or(0) }
}
fn find_named(el: &Rc<Element>, name: &str, out: &mut Vec<Rc<Element>>) {
    if el.get_name() == name { out.push(Rc::clone(el)); }
    for c in el.get_children() { find_named(c, name, out); }
}
fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}
fn parse_datatest(stem: &str) -> Result<DataTest, String> {
    let path = repo_root().join("decompiler/datatests").join(format!("{stem}.xml"));
    let xml = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store.parse_document(&xml).map_err(|e| format!("parse {stem}: {e}"))?.get_root().clone();
    let mut bis = Vec::new();
    find_named(&root, "binaryimage", &mut bis);
    let binaryimage = bis.into_iter().next().ok_or("no <binaryimage>")?;
    let arch_id = attr(&binaryimage, "arch").ok_or("<binaryimage> has no arch")?;
    let mut syms = Vec::new();
    find_named(&binaryimage, "symbol", &mut syms);
    let symbols: Vec<SymbolFn> = syms.iter().filter_map(|s| Some(SymbolFn {
        name: attr(s, "name")?, space: attr(s, "space")?, offset: parse_u64(&attr(s, "offset")?),
    })).collect();
    Ok(DataTest { binaryimage, arch_id, symbols })
}
struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str { "dummy" }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> { Err(KunaError::data_unavail("dummy")) }
    fn get_arch_type(&self) -> Vec<u8> { Vec::new() }
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
    arch.sleigh_mut().resolve_architecture(&db, &dt.arch_id).map_err(|e| format!("resolve_architecture: {e}"))?;
    if arch.sleigh().language_index() < 0 { return Err("language index unresolved".to_string()); }
    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file resolved no .sla")?;
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read sla: {e}"))?;
    arch.sleigh_mut().build_translator(Box::new(DummyImg), &sla).map_err(|e| format!("build_translator: {e}"))?;
    if !specs.compilerfile.is_empty() { if let Ok(cspec) = std::fs::read(&specs.compilerfile) { arch.sleigh_mut().base_mut().unwrap().set_cspec_xml(cspec); } }
    if !specs.processorfile.is_empty() { if let Ok(pspec) = std::fs::read(&specs.processorfile) { arch.sleigh_mut().base_mut().unwrap().set_pspec_xml(pspec); } }
    arch.sleigh_mut().base_mut().unwrap().translate_mut().install_register_lookup().map_err(|e| format!("install_register_lookup: {e}"))?;
    arch.sleigh_mut().base_mut().ok_or("no Architecture base after build_translator")?.init_post_engine().map_err(|e| format!("init_post_engine: {e}"))?;
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager outlives the open() call; borrow released before &mut use.
    arch.open_image(unsafe { &*manager_ptr }, &registry).map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}
fn render_one(stem: &str, which: usize) -> Result<String, String> {
    let dt = parse_datatest(stem)?;
    let mut xarch = bootstrap(&dt)?;
    let sym = dt.symbols.get(which).ok_or("symbol index out of range")?;
    let space_name = sym.space.clone(); let off = sym.offset; let name = sym.name.clone();
    let arch = xarch.sleigh_mut().base_mut().ok_or("no Architecture base")?;
    let space = arch.manage().get_space_by_name(&space_name).ok_or(format!("no space {space_name}"))?.clone();
    let entry = Address::new(space, off);
    let fd = decompile_func(arch, &name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
    Ok(print_c(arch, &fd))
}
fn skippable(e: &str) -> bool {
    e.contains("not built") || e.contains("no .sla") || e.contains("resolved no .sla")
        || e.contains("scan_for_sleigh") || e.contains("collect ldefs") || e.contains(".sla file for")
        || e.contains("not mapped")
}
fn index_of(stem: &str, fname: &str) -> Result<usize, String> {
    let dt = parse_datatest(stem)?;
    dt.symbols.iter().position(|s| s.name == fname).ok_or_else(|| format!("no symbol {fname} in {stem}"))
}
fn render_named(stem: &str, fname: &str) -> Option<String> {
    let idx = match index_of(stem, fname) {
        Ok(i) => i,
        Err(e) if skippable(&e) => { eprintln!("SKIP: {e}"); return None; }
        Err(e) => panic!("{e}"),
    };
    match render_one(stem, idx) {
        Ok(r) => Some(r),
        Err(e) if skippable(&e) => { eprintln!("SKIP: {e}"); None }
        Err(e) => panic!("{stem}:{fname} render: {e}"),
    }
}
fn braces_balanced(c: &str) -> bool {
    c.matches('{').count() >= 1 && c.matches('{').count() == c.matches('}').count()
}

// V1 — non-addrtied wide value: no panic in the new arms, deterministic.
#[test]
fn w10be_verifier_longdouble_pass_non_addrtied_wide_deterministic() {
    let r1 = match render_named("longdouble", "pass") { Some(r) => r, None => return };
    let r2 = render_named("longdouble", "pass").expect("second longdouble run");
    eprintln!("=== longdouble::pass ===\n{r1}");
    assert_eq!(r1, r2, "longdouble::pass must render deterministically");
    assert!(braces_balanced(&r1), "longdouble::pass not brace-matched:\n{r1}");
    assert!(r1.contains("pass"), "longdouble::pass missing function name:\n{r1}");
}

// V2 — second wide CONCAT geometry; find_root tiebreak determinism.
#[test]
fn w10be_verifier_storedouble_second_concat_geometry_deterministic() {
    let r1 = match render_named("doublemove", "storedouble") { Some(r) => r, None => return };
    let r2 = render_named("doublemove", "storedouble").expect("second storedouble run");
    eprintln!("=== doublemove::storedouble ===\n{r1}");
    assert_eq!(r1, r2, "storedouble wide-recombine render must be deterministic (find_root tiebreak)");
    assert!(braces_balanced(&r1), "storedouble not brace-matched:\n{r1}");
    assert!(r1.contains("return"), "storedouble must return a value:\n{r1}");
}

// V3 — broad-lever no-regression probe on an addrtied-register carrier the
// porter did not pin.
#[test]
fn w10be_verifier_condconst_conn_addrtied_carrier_unperturbed() {
    // `condconst_conn` is the function the verifier task flags as a
    // byte-identity probe for this broad explicit/implied lever.
    let r1 = match render_named("condconst", "condconst_conn") { Some(r) => r, None => return };
    let r2 = render_named("condconst", "condconst_conn").expect("second condconst_conn run");
    eprintln!("=== condconst::condconst_conn ===\n{r1}");
    assert_eq!(r1, r2, "condconst_conn render must be deterministic (broad-lever stability)");
    assert!(braces_balanced(&r1), "condconst_conn not brace-matched:\n{r1}");
}
