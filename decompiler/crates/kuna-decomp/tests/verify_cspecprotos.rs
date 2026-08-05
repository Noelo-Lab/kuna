//! GATE for `cspecprotos`: the compiler spec's NAMED prototype models are
//! registered, so the model registry can hold more than one calling convention.
//!
//! Before this, `Architecture::build_default_proto` read only
//! `<default_proto><prototype>`, so `protoModels` held exactly one entry and
//! `getModel("__thiscall")` — the lookup behind `option defaultprototype` and
//! behind every `hasThisPointer` consumer — could never resolve.
//!
//! These assertions run against the *real* vendored cspecs (x86win, x86-64-win,
//! x86gcc, x86-64-gcc), decoded through the same `init_post_engine` path the CLI
//! uses.  They are the sharp form of the end-to-end witness in
//! `tests/stages/ghdec-cspecprotos.xml`: a registry query, not a rendering.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

use kuna_base::address::Address;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
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

/// Bootstrap an architecture for `arch_id` over the vendored specs, running the
/// cspec decode (`init_post_engine` -> `build_default_proto`) but not opening an
/// image: the prototype-model registry is fully populated at that point.
fn bootstrap(arch_id: &str) -> Result<XmlArchitecture, String> {
    let root = repo_root();
    let registry = build_registry();
    let mut store = DocumentStorage::new();
    let img_xml = format!(
        r#"<binaryimage arch="{arch_id}"><bytechunk space="ram" offset="0x1000">c3</bytechunk></binaryimage>"#
    );
    let binaryimage: Rc<Element> = store
        .parse_document(img_xml.as_bytes())
        .map_err(|e| format!("parse image: {e}"))?
        .get_root()
        .clone();

    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("cspecprotos", "");
    arch.build_loader(binaryimage).map_err(|e| format!("build_loader: {e}"))?;
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(root.join("specs").to_str().unwrap());
    db.get_descriptions(&registry).map_err(|e| format!("collect ldefs: {e}"))?;
    arch.sleigh_mut().set_archid(arch_id);
    arch.sleigh_mut()
        .resolve_architecture(&db, arch_id)
        .map_err(|e| format!("resolve_architecture: {e}"))?;
    let specs = arch.sleigh().build_spec_file(&db).map_err(|e| format!("build_spec_file: {e}"))?;
    let resolved_sla = specs.slafile.ok_or("build_spec_file resolved no .sla")?;
    let sla = std::fs::read(&resolved_sla).map_err(|e| format!("read sla: {e}"))?;
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .map_err(|e| format!("build_translator: {e}"))?;
    let cspec = std::fs::read(&specs.compilerfile)
        .map_err(|e| format!("read cspec {}: {e}", specs.compilerfile))?;
    arch.sleigh_mut().base_mut().ok_or("no Architecture base")?.set_cspec_xml(cspec);
    arch.sleigh_mut()
        .base_mut()
        .ok_or("no Architecture base")?
        .init_post_engine()
        .map_err(|e| format!("init_post_engine: {e}"))?;
    Ok(arch)
}

fn arch_of(x: &XmlArchitecture) -> &Architecture {
    x.sleigh().base().expect("Architecture base")
}

/// The x86 Windows spec declares four models plus a merged resolver; every one
/// of them must resolve by name, and `__thiscall` must carry `hasThisPointer`.
#[test]
fn x86win_registers_every_named_model() {
    let x = bootstrap("x86:LE:32:default:windows").expect("bootstrap x86win");
    let arch = arch_of(&x);
    // The whole point: the registry used to hold exactly one model.
    assert!(
        arch.num_proto_models() > 1,
        "registry holds {} model(s): {:?}",
        arch.num_proto_models(),
        arch.proto_model_names().collect::<Vec<_>>()
    );
    assert_eq!(arch.default_fp().unwrap().get_name(), "__stdcall");
    for nm in ["__stdcall", "__cdecl", "__fastcall", "__thiscall"] {
        assert!(arch.has_model(nm), "x86win cspec model {nm} is not registered");
    }
    let thiscall = arch.get_model("__thiscall").unwrap();
    assert!(thiscall.has_this_pointer(), "__thiscall must set hasThisPointer");
    assert!(!arch.get_model("__stdcall").unwrap().has_this_pointer());
    // The merged <resolveprototype> folds its three constituents in.
    let merged = arch
        .get_model("__fastcall/__thiscall/__stdcall")
        .expect("x86win <resolveprototype> is not registered");
    assert!(merged.is_merged());
    assert_eq!(merged.num_models(), 3);
}

/// `__thiscall`'s first `<pentry>` is ECX, which is what makes the model worth
/// registering: the default `__stdcall` list has no register entry at all.
#[test]
fn x86win_thiscall_passes_the_first_parameter_in_ecx() {
    let x = bootstrap("x86:LE:32:default:windows").expect("bootstrap x86win");
    let arch = arch_of(&x);
    let ecx = arch.translate().get_register_varnode(b"ECX").expect("ECX register");
    let thiscall = arch.get_model("__thiscall").unwrap();
    let addr = Address::new(Rc::clone(ecx.space.as_ref().unwrap()), ecx.offset);
    assert!(
        thiscall.possible_input_param(&addr, ecx.size as i32),
        "__thiscall must accept ECX as an input parameter"
    );
    let stdcall = arch.get_model("__stdcall").unwrap();
    assert!(
        !stdcall.possible_input_param(&addr, ecx.size as i32),
        "__stdcall must not accept ECX as an input parameter"
    );
}

/// x86-64 Windows declares `__thiscall` plus two `<modelalias>` copies of the
/// default; the aliases are named copies, not the parent object.
#[test]
fn x86_64_win_registers_thiscall_and_the_model_aliases() {
    let x = bootstrap("x86:LE:64:default:windows").expect("bootstrap x86-64-win");
    let arch = arch_of(&x);
    assert_eq!(arch.default_fp().unwrap().get_name(), "__fastcall");
    for nm in ["__fastcall", "__thiscall", "__cdecl", "__stdcall"] {
        assert!(arch.has_model(nm), "x86-64-win cspec model {nm} is not registered");
    }
    assert!(arch.get_model("__thiscall").unwrap().has_this_pointer());
    // <modelalias name="__cdecl" parent="__fastcall"/>: a compatible named copy.
    let alias = arch.get_model("__cdecl").unwrap();
    let parent = arch.get_model("__fastcall").unwrap();
    assert_eq!(alias.get_name(), "__cdecl");
    assert!(alias.is_compatible(parent), "a <modelalias> must stay compatible with its parent");
}

/// The ELF specs register their own named models, and the 32-bit gcc spec has a
/// `__thiscall` too — so this is not a PE-only capability.
#[test]
fn elf_specs_register_their_named_models() {
    let x = bootstrap("x86:LE:32:default:gcc").expect("bootstrap x86gcc");
    let arch = arch_of(&x);
    assert_eq!(arch.default_fp().unwrap().get_name(), "__cdecl");
    for nm in ["__cdecl", "__thiscall", "__regparm1", "__regparm2", "__regparm3", "syscall"] {
        assert!(arch.has_model(nm), "x86gcc cspec model {nm} is not registered");
    }
    assert!(arch.get_model("__thiscall").unwrap().has_this_pointer());
    assert!(arch.get_model("__cdecl/__regparm").unwrap().is_merged());

    let x = bootstrap("x86:LE:64:default:gcc").expect("bootstrap x86-64-gcc");
    let arch = arch_of(&x);
    assert_eq!(arch.default_fp().unwrap().get_name(), "__stdcall");
    for nm in ["__stdcall", "MSABI", "syscall", "processEntry"] {
        assert!(arch.has_model(nm), "x86-64-gcc cspec model {nm} is not registered");
    }
}

/// C++ `parseCompilerConfig` guarantees a `__thiscall` model on every language,
/// cloning it off the default when the spec declares none (architecture.cc:1342).
#[test]
fn every_language_has_a_thiscall_model() {
    // x86-64-gcc and AARCH64 gcc declare no <prototype name="__thiscall">.
    for id in ["x86:LE:64:default:gcc", "AARCH64:LE:64:v8A", "ARM:LE:32:v7"] {
        let x = bootstrap(id).unwrap_or_else(|e| panic!("bootstrap {id}: {e}"));
        let arch = arch_of(&x);
        let m = arch
            .get_model("__thiscall")
            .unwrap_or_else(|| panic!("{id} has no __thiscall model"));
        assert!(m.has_this_pointer(), "{id}: the cloned __thiscall must set hasThisPointer");
        // Cloned off the default, so it is a compatible named copy of it.
        assert!(m.is_compatible(arch.default_fp().unwrap()));
    }
}

/// Registration must not disturb the default model: it is still the object
/// `defaultfp` points at, with the same storage the `<default_proto>` decode
/// produced before named models were read.
#[test]
fn the_default_model_is_untouched_by_the_named_models() {
    let x = bootstrap("x86:LE:32:default:windows").expect("bootstrap x86win");
    let arch = arch_of(&x);
    let defaultfp = arch.default_fp().unwrap();
    assert!(Rc::ptr_eq(defaultfp, arch.get_model("__stdcall").unwrap()));
    // The x86win <default_proto> is `extrapop="unknown"` with an ESP/EBP/...
    // <unaffected> block; those are what proto recovery reads.
    assert_eq!(defaultfp.get_extra_pop(), kuna_decomp::fspec::EXTRAPOP_UNKNOWN);
    assert!(!defaultfp.effect_list().is_empty());
    assert!(!defaultfp.has_this_pointer());
}
