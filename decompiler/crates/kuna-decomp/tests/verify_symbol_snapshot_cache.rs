//! `Architecture::build_arch_handle` memoizes the two whole-`symboltab`
//! derivations it hands each `Funcdata` (the `GlobalQuery` and the
//! declared-callee prototype list) on `Database::kuna_generation()`.  A stale hit
//! there silently emits wrong C, so this verifies the three halves of the
//! contract:
//!
//!   * an unchanged symbol table serves the *same* snapshot (the cache is live),
//!   * a mutated symbol table serves a *different* one (invalidation works),
//!   * the cached snapshot equals the one the bypass path re-derives.
//!
//! Bootstrap mechanics are the datatest-XML ones the other `verify_*` integration
//! tests use inline (the gate's helpers are private to its own test binary).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
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
    kuna_decomp::options::register_option_elements(&mut registry);
    registry
}

fn bootstrap(stem: &str) -> Result<XmlArchitecture, String> {
    let root = repo_root();
    let path = root.join("tests/datatests").join(format!("{stem}.xml"));
    let xml = std::fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let doc_root =
        store.parse_document(&xml).map_err(|e| format!("parse {stem}: {e}"))?.get_root().clone();
    let mut bis = Vec::new();
    find_named(&doc_root, "binaryimage", &mut bis);
    let binaryimage = bis.into_iter().next().ok_or("no <binaryimage>")?;
    let arch_id = attr(&binaryimage, "arch").ok_or("<binaryimage> has no arch")?;

    let registry = build_registry();
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("datatest", "");
    arch.build_loader(Rc::clone(&binaryimage)).map_err(|e| format!("build_loader: {e}"))?;
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;
    arch.sleigh_mut().set_archid(&arch_id);
    arch.sleigh_mut()
        .resolve_architecture(&db, &arch_id)
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
    // SAFETY: same shape as the gate / corpus_bootstrap — manager outlives open().
    arch.open_image(unsafe { &*manager_ptr }, &registry).map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));
    Ok(arch)
}

/// Render a handle's two snapshots for content comparison.
fn snapshot_text(arch: &Architecture) -> (String, String) {
    let glb = arch.build_arch_handle();
    (
        format!("{:?}", glb.global_query.as_ref().expect("global query")),
        format!("{:?}", glb.callee_protos),
    )
}

/// Map one new global into `symboltab`, the cheapest real mutation.
fn map_a_global(arch: &mut Architecture, name: &str, offset: u64) {
    let ram = arch.manage().get_default_code_space().cloned().expect("code space");
    let ty = arch.types().get_base(4, type_metatype::TYPE_INT).expect("int4 base type");
    let g = arch.symboltab.get_global_scope().expect("global scope");
    let addr = Address::new(ram, offset);
    arch.symboltab
        .add_symbol_mapped(g, name, ty, &addr, &Address::new_invalid())
        .expect("map global");
}

#[test]
fn symbol_snapshot_cache_reuses_and_invalidation_rebuilds() {
    let mut xarch = match bootstrap("boolless") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("skip (bootstrap, likely no .sla): {e}");
            return;
        }
    };
    let arch = xarch.sleigh_mut().base_mut().expect("base");

    // 1. Unchanged symbol table -> the identical snapshot objects, not copies.
    let a = arch.build_arch_handle();
    let b = arch.build_arch_handle();
    assert!(
        Rc::ptr_eq(a.global_query.as_ref().unwrap(), b.global_query.as_ref().unwrap()),
        "the global-symbol snapshot was rebuilt for an unmutated symbol table"
    );
    assert!(
        Rc::ptr_eq(&a.callee_protos, &b.callee_protos),
        "the callee-proto snapshot was rebuilt for an unmutated symbol table"
    );
    let (gq_before, cp_before) = snapshot_text(arch);

    // 2. The cached snapshot is what the bypass path re-derives, byte for byte.
    arch.set_symbol_snapshot_cache(false);
    let c = arch.build_arch_handle();
    let d = arch.build_arch_handle();
    assert!(
        !Rc::ptr_eq(c.global_query.as_ref().unwrap(), d.global_query.as_ref().unwrap()),
        "the bypass did not bypass: snapshots were still shared"
    );
    let (gq_bypass, cp_bypass) = snapshot_text(arch);
    assert_eq!(gq_before, gq_bypass, "cached global query differs from the re-derived one");
    assert_eq!(cp_before, cp_bypass, "cached callee protos differ from the re-derived ones");

    // 3. A mutation between two `build_arch_handle` calls must move the snapshot.
    arch.set_symbol_snapshot_cache(true);
    let e = arch.build_arch_handle();
    map_a_global(arch, "kuna_snapshot_probe", 0xb000);
    let f = arch.build_arch_handle();
    assert!(
        !Rc::ptr_eq(e.global_query.as_ref().unwrap(), f.global_query.as_ref().unwrap()),
        "a mapped global was added and the cache still served the stale snapshot"
    );
    let (gq_after, _) = snapshot_text(arch);
    assert_ne!(gq_before, gq_after, "the new global never reached the global query");
    assert!(
        gq_after.contains("kuna_snapshot_probe"),
        "the rebuilt global query does not carry the symbol that invalidated it"
    );

    // And the post-mutation cached snapshot still matches a full re-derivation.
    arch.set_symbol_snapshot_cache(false);
    let (gq_after_bypass, _) = snapshot_text(arch);
    assert_eq!(gq_after, gq_after_bypass, "post-invalidation snapshot is not the re-derived one");
}
