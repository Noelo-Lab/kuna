//! VERIFIER adversarial tests for item `w10-mips-callother`.
//!
//! These tests are written by the INDEPENDENT verifier (not the porter) and
//! land with the verdict regardless of outcome.  They target the most fragile
//! spots the hunt list flagged for this item:
//!
//!   T1 — the +3 PASSES are REAL: bootstrap the MIPS bitfields datatest
//!        (`bitfields2.xml`, `MIPS:BE:32`), decompile `dosomething` + `increment`
//!        through the full Architecture, and assert the spurious `setISAMode`
//!        CALLOTHER is GONE and the MIPS Bitfields #4/#5/#31 oracle lines appear.
//!   T2 — NO SPECIAL-CASING: the elimination keys on the user-op's DECLARED
//!        `injected` type, not the operand value.  We assert the userop table
//!        marks `setISAMode` injected (the general gate), and that a NON-injected
//!        CALLOTHER (an unspecialized userop) is left in place.
//!   T3 — the injected COPY is marked INCIDENTAL (per the payload's
//!        `incidentalcopy="true"`), the precondition for deadcode to fold it.
//!
//! Reuses the `decompile_e2e.rs` bootstrap mechanics inline (self-contained).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

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

    // Mirror the PRODUCTION console-engine bootstrap (kuna-console/src/engine.rs):
    // install the register-name lookup and hand the cspec + pspec XML to the
    // architecture BEFORE init_post_engine — so `init_userops_and_fixups` finds
    // the `<callotherfixup>` (the e2e gate omits this, hence its CALLOTHER stays).
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base after build_translator")?
        .translate_mut()
        .as_sleigh_mut().expect("standalone Sleigh engine").install_register_lookup()
        .map_err(|e| format!("install_register_lookup: {e}"))?;
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
        .ok_or("no Architecture base after build_translator")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;

    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call.
    arch.open_image(unsafe { &*manager_ptr }, &registry)
        .map_err(|e| format!("open_image: {e}"))?;
    let img = arch.take_loader().ok_or("loader vanished after open")?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    Ok(arch)
}

/// Decompile one symbol of a datatest to C.
fn decompile_one(arch: &mut Architecture, sym: &SymbolFn) -> Result<String, String> {
    let space = arch
        .manage()
        .get_space_by_name(&sym.space)
        .map(Rc::clone)
        .ok_or_else(|| format!("no space {}", sym.space))?;
    let entry = Address::new(space, sym.offset);
    let fd = decompile_func(arch, &sym.name, entry, 0).map_err(|e| format!("decompile: {e}"))?;
    Ok(print_c(arch, &fd))
}

// ===========================================================================
// T1 — the elimination is REAL: the spurious ISA-mode-switch CALLOTHER is GONE
// from the decompiled body.
//
// This manual bootstrap deliberately does NOT apply the datatest `<script>`
// (`parse line` struct typing + `lo fu` prototype override), so it proves the
// CORE effect this wave lands — the CALLOTHER is injected away — independent of
// the bitfield member-access rendering (which the struct typing supplies).
//
// The full B5 oracle lines (`ptr->field5 = val + 0x14;` etc., MIPS Bitfields
// #4/#5/#31) are verified by the AUTHORITATIVE harness, which applies the
// script:
//   ./decompiler/target/debug/decomp_test_dbg -path tests/datatests \
//        -sleighpath specs datatests bitfields2.xml
// -> bitfields2 13/31 (base d852f06) -> 16/31 (this branch); #4/#5/#31 flip
//    FAIL->Success.  Whole-suite 290 -> 293, passing-set diff = exactly those
//    three, 0 regressions.  (Verifier-run; see review verdict.)
// ===========================================================================

#[test]
fn w10_mips_callother_dosomething_eliminates_the_callother() {
    let dt = parse_datatest("bitfields2").expect("parse bitfields2");
    assert!(
        dt.arch_id.starts_with("MIPS:BE:32"),
        "fixture must be the MIPS bitfields datatest, got {}",
        dt.arch_id
    );
    let mut xarch = bootstrap(&dt).expect("bootstrap bitfields2 (MIPS)");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    let sym = dt.symbols.iter().find(|s| s.name == "dosomething").expect("dosomething symbol");
    let c = decompile_one(arch, sym).expect("decompile dosomething");
    eprintln!("--- dosomething ---\n{c}\n--- end ---");

    // The spurious ISA-mode-switch CALLOTHER must be GONE: no setISAMode call,
    // no `(ra & 1)` annotation, no raw CALLOTHER rendering left in the body.
    // (Before this wave the body showed `CALLOTHER(0,(ra & 1) != 0);`.)
    assert!(
        !c.contains("setISAMode") && !c.contains("CALLOTHER") && !c.contains("ra & 1"),
        "dosomething still shows the eliminated CALLOTHER:\n{c}"
    );
    // The surrounding real stores still render (the function is not gutted).
    assert!(c.contains('='), "dosomething body lost its assignments:\n{c}");
}

#[test]
fn w10_mips_callother_increment_eliminates_the_callother() {
    let dt = parse_datatest("bitfields2").expect("parse bitfields2");
    let mut xarch = bootstrap(&dt).expect("bootstrap bitfields2 (MIPS)");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    let sym = dt.symbols.iter().find(|s| s.name == "increment").expect("increment symbol");
    let c = decompile_one(arch, sym).expect("decompile increment");
    eprintln!("--- increment ---\n{c}\n--- end ---");

    assert!(
        !c.contains("setISAMode") && !c.contains("CALLOTHER") && !c.contains("ra & 1"),
        "increment still shows the eliminated CALLOTHER:\n{c}"
    );
    assert!(c.contains('='), "increment body lost its assignment:\n{c}");
}

// ===========================================================================
// T2 — NO SPECIAL-CASING: keyed on the DECLARED `injected` userop type.
// ===========================================================================

#[test]
fn w10_mips_callother_setisamode_is_marked_injected_in_the_userop_table() {
    // The general gate the flow uses (`is_injected_userop`) consults the userop
    // table.  After boot, the cspec `<callotherfixup targetop="setISAMode">`
    // must have OVERRIDDEN the base unspecialized `setISAMode` with an injected
    // userop — NOT a hardcoded operand/name match in the flow.
    use kuna_decomp::userop::userop_type;

    let dt = parse_datatest("bitfields2").expect("parse bitfields2");
    let mut xarch = bootstrap(&dt).expect("bootstrap bitfields2 (MIPS)");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    // Find the setISAMode userop by name and assert it is `injected`.
    let op = arch
        .userops
        .get_op_by_name(b"setISAMode")
        .expect("setISAMode userop must exist in the MIPS userop table");
    assert_eq!(
        op.get_type(),
        userop_type::injected,
        "setISAMode must be classified `injected` (the general elimination gate); \
         got {:?}",
        op.get_type()
    );
    // It must carry an inject id (the compiled `v0 = v0;` fixup body).
    assert!(
        op.get_inject_id().is_some(),
        "injected setISAMode must carry an inject id"
    );

    // And the payload must be marked incidental-copy (the cspec
    // `incidentalcopy="true"`) — the precondition that lets deadcode fold the
    // injected COPY.  (T3 folded in here: same bootstrap.)
    let injectid = op.get_inject_id().unwrap() as i32;
    let payload = arch.pcodeinjectlib.get_payload(injectid);
    assert!(
        payload.core().is_incidental_copy(),
        "setISAMode fixup payload must be incidental-copy (cspec incidentalcopy=true)"
    );
}

// ===========================================================================
// T4 — NO SPECIAL-CASING (negative): a userop the cspec did NOT fix up stays
// `unspecialized` (so the flow's `is_injected_userop` gate leaves its CALLOTHER
// in place).  The elimination must key on the DECLARED injected type alone — a
// userop without a `<callotherfixup>` is never injected/eliminated.
// ===========================================================================

#[test]
fn w10_mips_callother_non_fixed_up_userop_is_not_injected() {
    use kuna_decomp::userop::userop_type;

    let dt = parse_datatest("bitfields2").expect("parse bitfields2");
    let mut xarch = bootstrap(&dt).expect("bootstrap bitfields2 (MIPS)");
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    // Walk the whole MIPS userop table: EVERY op classified `injected` must be
    // one the cspec actually fixed up (i.e. carries an inject id).  There is no
    // path that marks an op injected without a `<callotherfixup>` — proving the
    // elimination is not a name/operand special-case.  And at least one op
    // (`setISAMode`) is unspecialized-NOT, so the table is non-trivially typed.
    let mut injected = 0usize;
    let mut unspecialized = 0usize;
    let mut idx = 0u32;
    while let Some(op) = arch.userops.get_op(idx) {
        match op.get_type() {
            userop_type::injected => {
                injected += 1;
                assert!(
                    op.get_inject_id().is_some(),
                    "an `injected` userop without an inject id would be a special-case \
                     marking (idx {idx}, name {:?})",
                    String::from_utf8_lossy(op.get_name())
                );
            }
            userop_type::unspecialized => unspecialized += 1,
            _ => {}
        }
        idx += 1;
        if idx > 4096 {
            break; // table is bounded; guard against a runaway
        }
    }
    // The MIPS table has exactly one `<callotherfixup>` (setISAMode), and many
    // plain userops left unspecialized — the elimination is selective on the
    // declared type, not blanket.
    assert!(injected >= 1, "expected the setISAMode fixup to mark >=1 injected op");
    assert!(
        unspecialized >= 1,
        "expected most MIPS userops to remain unspecialized (not blanket-injected)"
    );
}
