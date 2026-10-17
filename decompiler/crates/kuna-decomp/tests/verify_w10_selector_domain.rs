//! End-to-end regression for a bounded byte map feeding a jump table.

use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::varnode::varnode_flags;
use kuna_decomp::xml_arch::XmlArchitectureCapability;

use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
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

fn registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    register_option_elements(&mut registry);
    registry
}

fn find_element(el: &Rc<Element>, name: &str) -> Option<Rc<Element>> {
    if el.get_name() == name {
        return Some(Rc::clone(el));
    }
    el.get_children()
        .iter()
        .find_map(|child| find_element(child, name))
}

fn bootstrap(
    bytes: &[u8],
    readonly: bool,
) -> (
    kuna_decomp::xml_arch::XmlArchitecture,
    Rc<Element>,
    IdRegistry,
) {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut hex, "{byte:02x}").unwrap();
    }
    let readonly = if readonly { "true" } else { "false" };
    let xml = format!(
        "<decompilertest><binaryimage arch=\"x86:LE:64:default:gcc\">\
         <bytechunk space=\"ram\" offset=\"0x401000\" readonly=\"{readonly}\">{hex}</bytechunk>\
         <symbol space=\"ram\" offset=\"0x401000\" name=\"dispatch\"/>\
         </binaryimage></decompilertest>"
    );
    let mut storage = DocumentStorage::new();
    let root = storage
        .parse_document(xml.as_bytes())
        .unwrap()
        .get_root()
        .clone();
    let binaryimage = find_element(&root, "binaryimage").unwrap();
    let registry = registry();

    let mut arch = XmlArchitectureCapability::new().build_architecture("selector-domain", "");
    arch.build_loader(Rc::clone(&binaryimage)).unwrap();
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(repo_root().join("specs").to_str().unwrap());
    db.get_descriptions(&registry).unwrap();
    arch.sleigh_mut().set_archid("x86:LE:64:default:gcc");
    arch.sleigh_mut()
        .resolve_architecture(&db, "x86:LE:64:default:gcc")
        .unwrap();
    assert!(
        arch.sleigh().language_index() >= 0,
        "x86-64 language was not resolved"
    );
    let specs = arch.sleigh().build_spec_file(&db).unwrap();
    let sla = std::fs::read(specs.slafile.unwrap()).unwrap();
    arch.sleigh_mut()
        .build_translator(Box::new(DummyImg), &sla)
        .unwrap();
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
        .as_sleigh_mut()
        .expect("standalone Sleigh engine")
        .install_register_lookup()
        .unwrap();
    arch.sleigh_mut()
        .base_mut()
        .unwrap()
        .init_post_engine()
        .unwrap();

    let manager = arch.sleigh().base().unwrap().manage() as *const AddrSpaceManager;
    // SAFETY: the manager is owned by `arch` and `open_image` does not retain the borrow.
    arch.open_image(unsafe { &*manager }, &registry).unwrap();
    let image = arch.take_loader().unwrap();
    arch.sleigh_mut()
        .base_mut()
        .unwrap()
        .set_loader(Box::new(image));
    (arch, binaryimage, registry)
}

fn decompile_dispatch(bytes: &[u8], readonly: bool) -> String {
    let (mut xarch, _image, _registry) = bootstrap(bytes, readonly);
    let arch = xarch.sleigh_mut().base_mut().unwrap();
    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
    if readonly {
        // `XmlArchitecture::open_image` loads bytes but does not run the normal
        // post-spec readonly-range painting into the global symbol table. Mirror
        // that loader fact so this regression exercises the immutable-map proof.
        let start = Address::new(Rc::clone(&ram), 0x401000);
        let end = Address::new(Rc::clone(&ram), 0x401000 + bytes.len() as u64);
        arch.symboltab
            .set_property_range(varnode_flags::readonly, &start, &end);
    }
    let fd = decompile_func(arch, "dispatch", Address::new(ram, 0x401000), 0).unwrap();
    print_c(arch, &fd)
}

#[test]
fn bounded_byte_map_recovers_only_its_reachable_selector_values() {
    let code = decompile_dispatch(include_bytes!("fixtures/selector_domain_text.bin"), true);

    assert_eq!(
        code.matches("case ").count(),
        3,
        "unexpected switch labels:\n{code}"
    );
    for target in ["0x00400000", "0x00400001", "0x00400002"] {
        assert!(
            !code.contains(&format!("flows to r{target}")),
            "invented target {target}:\n{code}"
        );
    }
}

#[test]
fn writable_byte_map_keeps_the_conservative_selector_domain() {
    let code = decompile_dispatch(include_bytes!("fixtures/selector_domain_text.bin"), false);

    assert_ne!(
        code.matches("case ").count(),
        3,
        "writable map was narrowed from its image snapshot:\n{code}"
    );
}

#[test]
fn complete_byte_selector_still_recovers_all_256_destinations() {
    let code = decompile_dispatch(
        include_bytes!("fixtures/selector_domain_full_text.bin"),
        true,
    );

    assert_eq!(
        code.matches("case ").count(),
        256,
        "full byte selector lost a target:\n{code}"
    );
}
