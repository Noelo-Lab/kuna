//! The embedder hook registry: a rule added to a default pool and a callback
//! at a phase boundary both reach the printed C, and an empty registry leaves
//! the tree and the output unchanged.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::AddrSpaceManager;
use kuna_base::xml::{DocumentStorage, Element};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

use kuna_decomp::action::{ActionGroupList, Rule, RuleState};
use kuna_decomp::architecture::Architecture;
use kuna_decomp::context::{OpId, VarnodeId};
use kuna_decomp::decompile_drive::{decompile_func, print_c};
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_hooks::HookRegistry;
use kuna_decomp::kuna_phases::KunaPhase;
use kuna_decomp::universalaction::{universal_sched, universal_sched_hooked, ActionListFilter};
use kuna_decomp::varnode::varnode_flags;
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase};
use kuna_decomp::xml_arch::{XmlArchitecture, XmlArchitectureCapability};

/// `(x | y) + (x & y)  =>  x + y`, a fold the default pools do not have.
struct RuleOrAndSum;

impl RuleOrAndSum {
    fn operands(data: &Funcdata, vn: VarnodeId, code: OpCode) -> Option<(VarnodeId, VarnodeId)> {
        let op = data.obank().get(data.vbank().get(vn)?.get_def()?)?;
        (op.code() == code).then_some((op.get_in(0)?, op.get_in(1)?))
    }
}

impl Rule for RuleOrAndSum {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ADD]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        grouplist.contains("analysis").then(|| Box::new(RuleOrAndSum) as Box<dyn Rule>)
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> i32 {
        let Some(add) = data.obank().get(op) else { return 0 };
        let (Some(lhs), Some(rhs)) = (add.get_in(0), add.get_in(1)) else { return 0 };
        let pair = |first, second| {
            let or = Self::operands(data, first, OpCode::CPUI_INT_OR)?;
            let and = Self::operands(data, second, OpCode::CPUI_INT_AND)?;
            (or == and || or == (and.1, and.0)).then_some(or)
        };
        let Some((x, y)) = pair(lhs, rhs).or_else(|| pair(rhs, lhs)) else { return 0 };
        data.op_set_input(op, x, 0).expect("set input 0");
        data.op_set_input(op, y, 1).expect("set input 1");
        1
    }
}

struct NullLoad;

impl LoadImage for NullLoad {
    fn get_file_name(&self) -> &str {
        "null"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("null"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn find_binaryimage(el: &Rc<Element>) -> Option<Rc<Element>> {
    if el.get_name() == "binaryimage" {
        return Some(Rc::clone(el));
    }
    el.get_children().iter().find_map(find_binaryimage)
}

/// Load `tests/fixtures/hook_registry.xml` the way an embedder loads an XML
/// image: language, translator, cspec and pspec, then the opened image.
fn bootstrap() -> XmlArchitecture {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let xml = std::fs::read(manifest.join("tests/fixtures/hook_registry.xml")).expect("read fixture");
    let mut store = DocumentStorage::new();
    let image = find_binaryimage(store.parse_document(&xml).expect("parse fixture").get_root()).expect("<binaryimage>");
    let arch_id = String::from_utf8_lossy(image.get_attribute_value("arch").expect("arch attribute")).into_owned();

    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    register_option_elements(&mut registry);

    let mut arch = XmlArchitectureCapability::new().build_architecture("hook_registry", "");
    arch.build_loader(image).expect("build_loader");
    let mut db = LanguageDatabase::new();
    db.scan_for_sleigh_directories(manifest.join("../../../specs").to_str().expect("utf-8 path"));
    db.get_descriptions(&registry).expect("ldefs");
    arch.sleigh_mut().set_archid(&arch_id);
    arch.sleigh_mut().resolve_architecture(&db, &arch_id).expect("resolve_architecture");

    let specs = arch.sleigh().build_spec_file(&db).expect("build_spec_file");
    let sla = db.read_spec(&specs.slafile.expect("a .sla for x86-64")).expect("read .sla");
    arch.sleigh_mut().build_translator(Box::new(NullLoad), &sla).expect("build_translator");
    let base = arch.sleigh_mut().base_mut().expect("Architecture base");
    base.set_cspec_xml(db.read_spec(&specs.compilerfile).expect("read .cspec").into_owned());
    base.set_pspec_xml(db.read_spec(&specs.processorfile).expect("read .pspec").into_owned());
    base.translate_mut().as_sleigh_mut().expect("Sleigh engine").install_register_lookup().expect("register lookup");
    base.init_post_engine().expect("init_post_engine");

    let manager: *const AddrSpaceManager = arch.sleigh().base().expect("Architecture base").manage();
    // SAFETY: the manager lives inside `arch` and outlives the call, and no
    // `&mut` use of `arch` overlaps the borrow (as in decompile_e2e.rs).
    arch.open_image(unsafe { &*manager }, &registry).expect("open_image");
    let img = arch.take_loader().expect("the opened image");
    arch.sleigh_mut().base_mut().expect("Architecture base").set_loader(Box::new(img));
    arch
}

fn fixture() -> (XmlArchitecture, Address) {
    let mut xarch = bootstrap();
    let base = xarch.sleigh_mut().base_mut().expect("Architecture base");
    let space = Rc::clone(base.manage().get_space_by_name("ram").expect("ram space"));
    (xarch, Address::new(space, 0x100000))
}

fn decompile(arch: &mut Architecture, entry: &Address) -> String {
    let fd = decompile_func(arch, "mba_sum", entry.clone(), 0).expect("decompile mba_sum");
    print_c(arch, &fd)
}

fn decompile_filter() -> ActionListFilter {
    ActionListFilter::from_names([
        "base", "analysis", "subvar", "stackvars", "typerecovery", "merge", "blockrecovery", "protorecovery",
    ])
}

/// Rename the function's one stack local and lock the name.
fn rename_stack_local(data: &mut Funcdata, name: &str) {
    let scope = data.get_scope_local_mut().expect("local scope");
    let locals = scope.mapped_symbol_specs();
    let (old, ..) = locals.first().expect("a mapped stack local");
    let sym = *scope.query_by_name(old).first().expect("the local's symbol");
    scope.rename_symbol(sym, name).expect("rename");
    scope.set_attribute(sym, varnode_flags::namelock);
}

#[test]
fn added_rule_and_phase_callback_reach_the_printed_c() {
    let (mut xarch, entry) = fixture();
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");

    let plain = decompile(arch, &entry);
    assert!(plain.contains("|") && plain.contains("&"), "the fold is not a default:\n{plain}");

    let order = Rc::new(RefCell::new(Vec::new()));
    arch.add_rule("analysis", RuleState::new("analysis", 0, "orandsum"), Box::new(RuleOrAndSum))
        .expect("add_rule");
    for phase in [KunaPhase::P8, KunaPhase::P2, KunaPhase::P6] {
        let order = Rc::clone(&order);
        arch.on_phase(phase, move |_| order.borrow_mut().push(phase)).expect("on_phase");
    }
    arch.on_phase(KunaPhase::P5, |data| rename_stack_local(data, "seed")).expect("on_phase P5");

    let hooked = decompile(arch, &entry);
    assert!(!hooked.contains("|") && !hooked.contains("& "), "rule did not fold:\n{hooked}");
    assert!(hooked.contains(" + "), "the sum is gone:\n{hooked}");
    assert!(hooked.contains("&seed") && hooked.contains("^ seed"), "local not renamed:\n{hooked}");
    assert_eq!(*order.borrow(), [KunaPhase::P2, KunaPhase::P6, KunaPhase::P8]);
}

#[test]
fn registration_errors_name_the_missing_anchor() {
    let mut hooks = HookRegistry::default();
    let err = hooks
        .add_rule("nosuchgroup", RuleState::new("analysis", 0, "orandsum"), Box::new(RuleOrAndSum))
        .unwrap_err();
    assert!(format!("{err}").contains("nosuchgroup"), "{err}");
    let err = hooks
        .add_rule("analysis", RuleState::new("cleanup", 0, "orandsum"), Box::new(RuleOrAndSum))
        .unwrap_err();
    assert!(format!("{err}").contains("does not clone"), "{err}");
    for phase in [KunaPhase::Infra, KunaPhase::P0, KunaPhase::P1, KunaPhase::P9] {
        assert!(hooks.on_phase(phase, Box::new(|_| ())).is_err(), "{phase:?}");
    }
    assert!(hooks.is_empty());
}

#[test]
fn hooks_appear_in_the_listing_at_their_schedule_points() {
    let mut hooks = HookRegistry::default();
    hooks
        .add_rule("typerecovery", RuleState::new("typerecovery", 0, "orandsum_a"), Box::new(RuleOrAndSum))
        .unwrap_err();
    hooks.add_rule("subvar", RuleState::new("analysis", 0, "orandsum_a"), Box::new(RuleOrAndSum)).unwrap();
    hooks.add_rule("subvar", RuleState::new("analysis", 0, "orandsum_b"), Box::new(RuleOrAndSum)).unwrap();
    hooks.add_rule("stackvars", RuleState::new("analysis", 0, "orandsum_c"), Box::new(RuleOrAndSum)).unwrap();
    for phase in [KunaPhase::P2, KunaPhase::P3, KunaPhase::P4, KunaPhase::P5, KunaPhase::P6, KunaPhase::P7, KunaPhase::P8] {
        hooks.on_phase(phase, Box::new(|_| ())).unwrap();
    }
    let dump = universal_sched_hooked(None, None, vec![], false, &hooks).list_action_dump(&decompile_filter());
    let names: Vec<&str> = dump.lines().filter_map(|l| l.split_whitespace().last()).collect();
    let after = |name: &str| names[names.iter().position(|n| *n == name).unwrap_or_else(|| panic!("{name}\n{dump}")) + 1];
    assert_eq!(after("subvar_sext"), "orandsum_a");
    assert_eq!(after("orandsum_a"), "orandsum_b");
    assert_eq!(after("callpush"), "orandsum_c");
    assert_eq!(after("start"), "phasehook_p2");
    assert_eq!(after("activereturn"), "phasehook_p3");
    assert_eq!(after("phasehook_p3"), "phasehook_p4");
    assert_eq!(after("phasehook_p4"), "phasehook_p5");
    assert_eq!(after("copymarker"), "phasehook_p6");
    assert_eq!(after("phasehook_p7"), "prefercomplement");
    assert_eq!(after("phasehook_p8"), "prototypewarnings");
}

#[test]
fn empty_registry_changes_neither_tree_nor_output() {
    let filter = decompile_filter();
    assert_eq!(
        universal_sched_hooked(None, None, vec![], false, &HookRegistry::default()).list_action_dump(&filter),
        universal_sched(None, None, vec![], false).list_action_dump(&filter),
    );

    let (mut xarch, entry) = fixture();
    let arch = xarch.sleigh_mut().base_mut().expect("Architecture base");
    assert!(arch.hooks().is_empty());
    let first = decompile(arch, &entry);
    let second = decompile(arch, &entry);
    assert_eq!(first, second);
    assert!(first.contains("|") && first.contains("&"), "{first}");
}
