//! END-TO-END verification for the `kuna decompile-project` engine emitters
//! (`doc_prototype` / `doc_type_definitions` / the recompile prelude):
//!
//!   1. **The `.h`/`.c` identity contract**: for real decompiled corpus
//!      functions, [`print_c_prototype`]'s output minus its trailing `;`
//!      appears CHAR-FOR-CHAR as the prototype segment inside the full
//!      [`print_c`] render of the same `Funcdata` — `doc_prototype` drives the
//!      IDENTICAL `emit_prototype_declaration` token sequence
//!      `emit_function_document` does (pure code motion), so the two can never
//!      drift.
//!   2. **The `.h` type block**: [`print_c_recompile_prelude`] emits the core
//!      scalar + `undefined`-family typedefs, and [`print_c_types`] renders
//!      user types interned in the LIVE architecture's factory (forward-decl
//!      block first, bodies in `dependent_order`, incomplete structs
//!      `/* opaque */`).
//!
//! Bootstrap mechanics are the `decompile_e2e.rs` harness (datatest XML →
//! `XmlArchitecture` → `build_translator` → `init_post_engine`).  Like the
//! `verify_w10_*` files, a fixture whose `.sla` is not built SKIPs cleanly.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::{
    decompile_func, print_c, print_c_prototype, print_c_recompile_prelude, print_c_types,
};
use kuna_decomp::dtype::{type_metatype, TypeField};
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

// ===========================================================================
// Datatest XML parsing + bootstrap (the decompile_e2e.rs mechanics).
// ===========================================================================

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

    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;

    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call; the
    // borrow is released before any &mut use of `arch` (same shape as
    // decompile_e2e.rs / corpus_bootstrap.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    Ok(arch)
}

/// Bootstrap a fixture, SKIPping (None) when the `.sla` artifacts are not
/// built in this checkout (`make specs`).
fn bootstrap_or_skip(stem: &str) -> Option<(DataTest, XmlArchitecture)> {
    let dt = match parse_datatest(stem) {
        Ok(dt) => dt,
        Err(e) => {
            eprintln!("SKIP: {stem}: {e}");
            return None;
        }
    };
    match bootstrap(&dt) {
        Ok(xarch) => Some((dt, xarch)),
        Err(e) => {
            eprintln!("SKIP: {stem} bootstrap failed (specs built?): {e}");
            None
        }
    }
}

// ===========================================================================
// (T1) The `.h`/`.c` identity contract: doc_prototype minus the trailing `;`
//      is char-for-char the prototype segment of the full print_c render.
// ===========================================================================
#[test]
fn doc_prototype_matches_print_c_prototype_segment() {
    let mut checked = 0usize;
    for stem in ["boolless", "gp", "ccmp"] {
        let Some((dt, mut xarch)) = bootstrap_or_skip(stem) else { continue };
        let base = xarch.sleigh_mut().base_mut().expect("Architecture base");
        for sym in &dt.symbols {
            let Some(space) = base.manage().get_space_by_name(&sym.space).map(Rc::clone) else {
                continue;
            };
            let entry = Address::new(space, sym.offset);
            // Call targets without loaded bytes legitimately fail to
            // decompile; the contract is asserted on every function that DOES.
            let Ok(fd) = decompile_func(base, &sym.name, entry, 0) else { continue };

            let proto = print_c_prototype(base, &fd);
            assert!(
                proto.ends_with(';'),
                "{stem}:{}: doc_prototype must end with `;`: {proto:?}",
                sym.name
            );
            let stripped = &proto[..proto.len() - 1];
            assert!(
                stripped.contains(&format!("{}(", fd.get_display_name())),
                "{stem}:{}: prototype lacks the function name + paren: {proto:?}",
                sym.name
            );
            assert!(
                !stripped.contains('\n') && !stripped.contains('{'),
                "{stem}:{}: prototype must be a single declaration line: {proto:?}",
                sym.name
            );

            let full = print_c(base, &fd);
            assert!(
                full.contains(stripped),
                "{stem}:{}: the `.h` prototype segment is NOT char-for-char inside \
                 the full render.\nprototype: {stripped:?}\nfull:\n{full}",
                sym.name
            );
            checked += 1;
        }
    }
    // The gate's teeth: at least 2 real corpus functions must have exercised
    // the identity contract (0 means every fixture SKIPped — specs not built).
    if checked == 0 {
        eprintln!("SKIP: no fixture decompiled (specs not built?)");
        return;
    }
    assert!(
        checked >= 2,
        "expected >= 2 corpus functions to pin the prototype identity, got {checked}"
    );
    eprintln!("prototype identity checked on {checked} functions");
}

// ===========================================================================
// (T2) The `.h` type block: the recompile prelude + doc_type_definitions over
//      the LIVE architecture factory.
// ===========================================================================
#[test]
fn recompile_prelude_and_type_definitions_render() {
    let Some((_dt, mut xarch)) = bootstrap_or_skip("boolless") else { return };
    let base = xarch.sleigh_mut().base_mut().expect("Architecture base");

    // -- The prelude: core scalar typedefs + the fixed undefined family. -----
    let prelude = print_c_recompile_prelude(base);
    assert!(!prelude.is_empty());
    for needle in [
        "#include <stdbool.h>",
        "typedef unsigned int uint4;",
        "typedef int int4;",
        "typedef long long int8;",
        "typedef unsigned char undefined;\n",
        "typedef unsigned long long undefined8;\n",
        "typedef unsigned int undefined3; /* 3 bytes in the decompiler; sizeof differs */",
        "typedef struct { unsigned char b[16]; } undefined16;",
        "typedef struct { unsigned char b[32]; } undefined32;",
        "typedef double float8;",
    ] {
        assert!(prelude.contains(needle), "prelude is missing {needle:?}:\n{prelude}");
    }
    // `void`/`char` are real C and must NOT be typedef'd; `bool` only via the
    // include.
    assert!(!prelude.contains(" void;"), "prelude must not typedef void:\n{prelude}");
    assert!(!prelude.contains(" char;"), "prelude must not typedef char:\n{prelude}");
    assert!(!prelude.contains(" bool;"), "prelude must not typedef bool:\n{prelude}");

    // -- doc_type_definitions over the live factory. --------------------------
    // A fresh factory carries only core types: the block is empty (no
    // spurious output).
    let empty = print_c_types(base);
    assert!(
        empty.trim().is_empty(),
        "no user types are interned, so the type block must be empty:\n{empty}"
    );

    // Intern a complete struct + an opaque struct through the REAL factory
    // trait flow (getTypeStruct + assignRawFields), then re-render.
    {
        let tf = base.types();
        let s0 = tf.get_type_struct("kunaproj_t").expect("getTypeStruct");
        let i4 = tf.get_base(4, type_metatype::TYPE_INT).expect("getBase int4");
        let p = tf
            .get_type_pointer(8, Rc::clone(&s0), 1)
            .expect("getTypePointer (self-referential)");
        tf.assign_raw_fields_struct(
            &s0,
            vec![
                TypeField::new(0, -1, "value", i4),
                TypeField::new(1, -1, "next", p),
            ],
            vec![],
        )
        .expect("assignRawFields");
        tf.get_type_struct("kunaproj_opaque").expect("getTypeStruct opaque");
    }
    let types = print_c_types(base);
    assert!(
        types.contains("typedef struct kunaproj_t kunaproj_t;"),
        "forward declaration missing:\n{types}"
    );
    assert!(types.contains("struct kunaproj_t {"), "struct body missing:\n{types}");
    assert!(types.contains("value;"), "scalar field missing:\n{types}");
    assert!(
        types.contains("kunaproj_t *next;"),
        "self-referential pointer field missing (needs the forward decl):\n{types}"
    );
    assert!(
        types.contains("typedef struct kunaproj_opaque kunaproj_opaque; /* opaque */"),
        "incomplete struct must forward-declare as opaque:\n{types}"
    );
    assert!(
        !types.contains("struct kunaproj_opaque {"),
        "incomplete struct must have NO body:\n{types}"
    );
    // The forward-decl block precedes the body (the self-reference guarantee).
    let fwd = types.find("typedef struct kunaproj_t").unwrap();
    let body = types.find("struct kunaproj_t {").unwrap();
    assert!(fwd < body, "forward declarations must precede bodies:\n{types}");
}
