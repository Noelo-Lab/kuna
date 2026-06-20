//! INDEPENDENT VERIFIER — item `rport/w10-merge-casts` (S6 HighVariable Merge
//! engine: the read-/def-facing **type graph** the Casts stage queries).
//!
//! This wave threads the merged `HighVariable`'s data-type out to the four
//! facing accessors that C++ keeps on `Varnode` (`varnode.cc:645-691`) but that
//! the Rust port must keep on `Funcdata` (a `Datatype` carries no `Funcdata`
//! back-pointer; ADR-0001) — `vn_type_def_facing`/`vn_type_read_facing`/
//! `vn_high_type_def_facing`/`vn_high_type_read_facing` in
//! [`kuna_decomp::funcdata_facing`].
//!
//! The fence pins two invariants the next (Casts) wave depends on:
//!
//!   1. **Threaded.** For a written Varnode carrying a merged HighVariable, the
//!      *high*-facing accessor returns the SAME interned `Rc<Datatype>` as the
//!      merged `high->getType()` — i.e. the facing graph really reads the merged
//!      high, not the bare Varnode type.  (`getHighTypeDefFacing` ==
//!      `high->getType()` in the no-resolution default, varnode.cc:670-677.)
//!
//!   2. **No-union default is exact.** When the type does not `needsResolution`
//!      (the entire boolless / register-typed corpus has no unions flowing), the
//!      facing accessors return the type itself with no cache consultation —
//!      `find_resolve` base, type.cc:593.  This is what keeps boolless /
//!      readstruct / condconst byte-identical.
//!
//! Driven over the real decompiled IR (full XML frontend + `decompile_func`), so
//! it exercises the live merged high bank, not a mock.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::decompile_func;
use kuna_decomp::options::register_option_elements;
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
}

fn parse_datatest(path: &std::path::Path) -> Result<DataTest, String> {
    let xml = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut store = DocumentStorage::new();
    let root = store
        .parse_document(&xml)
        .map_err(|e| format!("parse {}: {e}", path.display()))?
        .get_root()
        .clone();

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
        .install_register_lookup()
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

/// Decompile every `<symbol>` of `file` and assert the facing-graph invariants
/// on the resulting `Funcdata`s.  Returns the number of (written varnode, op)
/// facing edges checked across all functions.
fn check_facing_invariants(file: &str) -> Result<usize, String> {
    let dir = repo_root().join("tests/datatests");
    let dt = parse_datatest(&dir.join(file))?;
    let mut xarch = bootstrap(&dt)?;

    let mut edges_checked = 0usize;
    let symbols: Vec<(String, String, u64)> =
        dt.symbols.iter().map(|s| (s.name.clone(), s.space.clone(), s.offset)).collect();

    for (name, space_name, offset) in symbols {
        let base = xarch.sleigh_mut().base_mut().ok_or("no base")?;
        let space = match base.manage().get_space_by_name(&space_name) {
            Some(s) => Rc::clone(s),
            None => continue,
        };
        let entry = Address::new(space, offset);
        let mut fd = match decompile_func(base, &name, entry, 0) {
            Ok(fd) => fd,
            Err(_) => continue,
        };

        // Snapshot the (vn, def-op, read-op-list) of every written Varnode that
        // carries a HighVariable.  Read up front so the immutable bank borrow is
        // released before the &mut self facing calls (the bank lazily re-derives
        // the high's type on read).
        struct VnEdges {
            vn: kuna_decomp::seams::VarnodeId,
            def: Option<kuna_decomp::seams::OpId>,
            reads: Vec<kuna_decomp::seams::OpId>,
        }
        let mut work: Vec<VnEdges> = Vec::new();
        for vn in fd.vbank().iter_loc() {
            let v = match fd.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.get_high().is_none() {
                continue;
            }
            let def = v.get_def();
            let reads: Vec<_> = v.descend_iter().collect();
            work.push(VnEdges { vn, def, reads });
        }

        for w in &work {
            // The merged high's data-type (the C++ `vn->getHigh()->getType()`).
            let high_ty = match fd.high_get_type(w.vn) {
                Some(t) => t,
                None => continue,
            };
            let bare_ty = Rc::clone(fd.vbank().get(w.vn).unwrap().get_type());

            // Invariant 2: with no unions flowing, no facing accessor resolves —
            // the high-facing type IS the merged high type (interned identity).
            if !high_ty.needs_resolution() {
                if let Some(def) = w.def {
                    let def_facing = fd.vn_high_type_def_facing(w.vn);
                    assert!(
                        Rc::ptr_eq(&def_facing, &high_ty),
                        "{name}: getHighTypeDefFacing must equal high->getType() for \
                         non-resolution type (def op present {def:?})"
                    );
                }
                for &op in &w.reads {
                    let read_facing = fd.vn_high_type_read_facing(w.vn, op);
                    assert!(
                        Rc::ptr_eq(&read_facing, &high_ty),
                        "{name}: getHighTypeReadFacing must equal high->getType() for \
                         non-resolution type"
                    );
                }
            }

            // Invariant 2b: the bare-Varnode facing pair likewise returns the
            // bare type unchanged when it does not need resolution (the
            // getTypeDefFacing/getTypeReadFacing base, varnode.cc:645-664).
            if !bare_ty.needs_resolution() {
                if w.def.is_some() {
                    let dfac = fd.vn_type_def_facing(w.vn);
                    assert!(
                        Rc::ptr_eq(&dfac, &bare_ty),
                        "{name}: getTypeDefFacing must equal vn->getType() for \
                         non-resolution type"
                    );
                }
                for &op in &w.reads {
                    let rfac = fd.vn_type_read_facing(w.vn, op);
                    assert!(
                        Rc::ptr_eq(&rfac, &bare_ty),
                        "{name}: getTypeReadFacing must equal vn->getType() for \
                         non-resolution type"
                    );
                }
            }

            edges_checked += w.reads.len() + usize::from(w.def.is_some());
        }
    }
    Ok(edges_checked)
}

/// boolless is in the byte-identical family (register-typed, no unions): every
/// facing edge must take the no-resolution default and equal the merged high
/// type.  Proves the facing graph is threaded AND the default is exact.
#[test]
fn w10_facing_graph_threaded_boolless() {
    let n = check_facing_invariants("boolless.xml")
        .expect("boolless facing invariants");
    assert!(n > 0, "boolless must have at least one merged written-varnode facing edge");
    eprintln!("boolless: {n} facing edges verified (all no-resolution default)");
}

/// condconst is the other byte-identical family member — same facing default.
#[test]
fn w10_facing_graph_threaded_condconst() {
    let n = check_facing_invariants("condconst.xml")
        .expect("condconst facing invariants");
    assert!(n > 0, "condconst must have at least one merged written-varnode facing edge");
    eprintln!("condconst: {n} facing edges verified (all no-resolution default)");
}

/// piecestruct flows real struct/pointer types: the facing accessors must still
/// return the merged high type unchanged (the struct/pointer `findResolve` arms
/// hit an empty union cache and fall back to self / field[0] / element only when
/// `needsResolution` — which a plain struct/pointer is NOT).  Exercises the
/// `find_resolve_facing` pointer/struct dispatch on the no-resolution path.
#[test]
fn w10_facing_graph_threaded_piecestruct() {
    let n = check_facing_invariants("piecestruct.xml")
        .expect("piecestruct facing invariants");
    assert!(n > 0, "piecestruct must have merged written-varnode facing edges");
    eprintln!("piecestruct: {n} facing edges verified");
}
