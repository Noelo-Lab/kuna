// Included by verify_w10_divmod.rs as `mod corpus`.  Real-parity divopt render:
// independently bootstraps the XML frontend (same machinery as
// verify_w10_struct_corpus.rs — test infra, not the SUT) and asserts the EXACT
// oracle div/mod forms appear, proving the new PASS is genuine parity, not a
// coincidental `/ 81` substring.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::options::{register_option_elements, OptionDatabase};
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

fn find_named(el: &Rc<Element>, name: &str, out: &mut Vec<Rc<Element>>) {
    if el.get_name() == name {
        out.push(Rc::clone(el));
    }
    for c in el.get_children() {
        find_named(c, name, out);
    }
}

fn parse_u64(s: &str) -> u64 {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
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
    options: Vec<(String, String, String, String)>,
}

fn xml_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn parse_option_com(text: &str) -> Option<(String, String, String, String)> {
    let mut it = text.split_whitespace();
    if it.next()? != "option" {
        return None;
    }
    let name = it.next()?.to_string();
    let p1 = it.next().unwrap_or("").to_string();
    let p2 = it.next().unwrap_or("").to_string();
    let p3 = it.next().unwrap_or("").to_string();
    Some((name, p1, p2, p3))
}

fn parse_datatest(path: &std::path::Path) -> Result<DataTest, String> {
    let xml = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root =
        store.parse_document(&xml).map_err(|e| format!("parse {}: {e}", path.display()))?.get_root().clone();

    let mut bins = Vec::new();
    find_named(&root, "binaryimage", &mut bins);
    let binaryimage = bins.first().ok_or("no <binaryimage>")?.clone();

    let mut langs = Vec::new();
    find_named(&root, "language", &mut langs);
    let arch_id = langs
        .first()
        .and_then(|l| attr(l, "id"))
        .or_else(|| attr(&binaryimage, "arch"))
        .ok_or("no language/arch id")?;

    let mut syms = Vec::new();
    find_named(&root, "symbol", &mut syms);
    let symbols = syms
        .iter()
        .filter_map(|s| {
            Some(SymbolFn {
                name: attr(s, "name")?,
                space: attr(s, "space").unwrap_or_else(|| "ram".to_string()),
                offset: parse_u64(&attr(s, "offset")?),
            })
        })
        .collect();

    let mut coms = Vec::new();
    find_named(&root, "com", &mut coms);
    let options = coms
        .iter()
        .filter_map(|c| parse_option_com(&xml_unescape(&String::from_utf8_lossy(c.get_content()))))
        .collect();

    Ok(DataTest { binaryimage, arch_id, symbols, options })
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
    register_option_elements(&mut registry);
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

    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            arch.sleigh_mut().base_mut().unwrap().set_cspec_xml(cspec);
        }
    }
    if !specs.processorfile.is_empty() {
        if let Ok(pspec) = std::fs::read(&specs.processorfile) {
            arch.sleigh_mut().base_mut().unwrap().set_pspec_xml(pspec);
        }
    }

    arch.sleigh_mut()
        .base_mut()
        .unwrap()
        .translate_mut()
        .as_sleigh_mut().expect("standalone Sleigh engine").install_register_lookup()
        .map_err(|e| format!("install_register_lookup: {e}"))?;

    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;

    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    arch.open_image(unsafe { &*manager_ptr }, &registry).map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

fn render_corpus(dt: &DataTest) -> Result<String, String> {
    let registry = build_registry();
    let mut xarch = bootstrap(dt)?;

    {
        let options = OptionDatabase::new();
        if let Some(base) = xarch.sleigh_mut().base_mut() {
            for (name, p1, p2, p3) in &dt.options {
                let id = registry.find_element(name, 0);
                if id != 0 {
                    let _ = options.set(base, id, p1, p2, p3);
                }
            }
        }
    }

    let mut out = String::new();
    for sym in &dt.symbols {
        let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
        let space = match base.manage().get_space_by_name(&sym.space) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let entry = Address::new(space, sym.offset);
        if let Ok(fd) = decompile_func(base, &sym.name, entry, 0) {
            out.push_str(&print_c(base, &fd));
            out.push('\n');
        }
    }
    Ok(out)
}

/// REAL parity: the divopt render must reproduce the oracle div/mod STATEMENT
/// SHAPE — full `lhs = lhs / N;` / `lhs = lhs % N;` assignments whose pointer
/// LHS recurs on the RHS — not a coincidental `/ 81` substring.  The pointer
/// parameter renders here as `a0` (the corpus's `parse line ... void
/// divoptu(uint8 *divu)` naming command is NOT replayed by this minimal
/// harness, so the oracle's `divu`/`divi`/`modu` names are not bound — a
/// pre-existing symbol-naming gap, orthogonal to the divmod reconstruction).
/// We therefore pin the full-statement reconstruction using the rendered LHS.
#[test]
fn w10_divmod_divopt_renders_exact_oracle_forms() {
    let path = repo_root().join("tests/datatests/divopt.xml");
    let dt = parse_datatest(&path).expect("parse divopt.xml");
    let rendered = render_corpus(&dt).expect("divopt must decompile");

    // Full-statement oracle SHAPE (divopt.xml stringmatch content with the
    // harness's `a0` param name): each is a complete assignment whose LHS recurs
    // on the RHS — exactly the C++ oracle's `<lhs> = <lhs> / N;` form.  A
    // coincidental `/ 81` substring (e.g. inside a surviving `* magic >> shift`)
    // could not produce the full recurring-LHS statement.
    let expected = [
        "*a0 = *a0 / 81;",       // Unsigned Division #1 (oracle: *divu = *divu / 81;)
        "a0[1] = a0[1] / 89;",   // Unsigned Division #2
        "a0[2] = a0[2] / 91;",   // Unsigned Division #3
        "a0[16] = a0[16] / 125;",// Unsigned Division #17 (the LAST divisor)
        "*a0 = *a0 % 81;",       // Unsigned Modulo #1
        "a0[16] = a0[16] % 125;",// Unsigned Modulo #17
    ];
    let mut missing = Vec::new();
    for form in expected {
        if !rendered.contains(form) {
            missing.push(form);
        }
    }
    assert!(
        missing.is_empty(),
        "divopt must render the full oracle div/mod statement shape (real \
         parity, recurring-LHS assignment, not a `/ 81` substring); missing: \
         {missing:?}\n--- rendered ---\n{rendered}"
    );

    // The raw reciprocal magic for /81 must be CONSUMED by the reconstruction
    // (its survival would mean RuleDivOpt declined on that divisor).
    assert!(
        !rendered.contains("0x948b0fcd6e9e0653"),
        "reciprocal magic must be reconstructed away, not left raw:\n{rendered}"
    );
}
