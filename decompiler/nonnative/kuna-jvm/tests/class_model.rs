//! The class model, pass hooks and collectors of slice K3.
//!
//! Only `dummy_class`, `dummy_method` and `dummy_var_proc` build objects owned
//! by other slices.

use std::cell::Cell;
use std::rc::Rc;

use kuna_jvm::bytecode_mapping_tracer::BytecodeMappingTracer;
use kuna_jvm::bytecode_source_mapper::BytecodeSourceMapper;
use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::class_writer::ClassWriter;
use kuna_jvm::classes_processor::{
    string_compare_to, ClassNode, ClassNodeData, ClassNodeType, LambdaInformation,
};
use kuna_jvm::decompile_record::DecompileRecord;
use kuna_jvm::import_collector::{ImportCollector, ImportCollectorData};
use kuna_jvm::java_pass_location::JavaPassLocation;
use kuna_jvm::java_pass_registrar::JavaPassRegistrar;
use kuna_jvm::looping_pass_builder::LoopingPassBuilder;
use kuna_jvm::main_pass_builder::MainPassBuilder;
use kuna_jvm::named_pass::NamedPass;
use kuna_jvm::pass::{Pass, NO_OP};
use kuna_jvm::pass_context::PassContext;
use kuna_jvm::plugin_context::{Plugin, PluginContext};
use kuna_jvm::rt::{reset_class_scope, try_catch, BitSet, JHashMap, JHashSet, JList, Shared};
use kuna_jvm::struct_class::{StructClass, StructClassData};
use kuna_jvm::struct_method::{StructMethod, StructMethodData};
use kuna_jvm::var_processor::{VarProcessor, VarProcessorData};
use kuna_jvm::wrapped_pass::WrappedPass;

fn dummy_class(name: &str) -> StructClass {
    StructClass::alloc(StructClassData {
        qualified_name: name.to_string(),
        super_class: None,
    })
}

fn dummy_method() -> StructMethod {
    StructMethod::alloc(StructMethodData {
        name: "run".to_string(),
    })
}

fn dummy_var_proc() -> VarProcessor {
    VarProcessor::alloc(VarProcessorData {})
}

fn node(name: &str, type_: ClassNodeType) -> ClassNode {
    ClassNode::alloc(ClassNodeData {
        type_,
        access: 0,
        simple_name: Some(name.rsplit('/').next().unwrap().to_string()),
        class_struct: dummy_class(name),
        wrapper: None,
        enclosing_method: None,
        super_invocation: None,
        map_fields_to_vars: JHashMap::new(),
        anonymous_class_type: None,
        nested: JList::new(),
        enclosing_classes: Shared::new(JHashSet::new()),
        parent: None,
        lambda_information: None,
    })
}

fn qualified_names<'a>(nodes: impl IntoIterator<Item = &'a ClassNode>) -> Vec<String> {
    nodes
        .into_iter()
        .map(|n| {
            n.with(|d| d.class_struct)
                .with(|d| d.qualified_name.clone())
        })
        .collect()
}

fn pass_context(rec: DecompileRecord) -> PassContext {
    PassContext::new(
        None,
        None,
        dummy_method(),
        dummy_class("a/A"),
        dummy_var_proc(),
        rec,
    )
}

#[test]
fn decompile_record_prefixes_loop_iterations() {
    let rec = DecompileRecord::new(dummy_method());
    rec.add("Initial", None);
    rec.increment_main_loop();
    rec.add("Start", None);
    rec.increment_merge_loop();
    rec.increment_merge_loop();
    rec.add("MergeLoopStart", None);
    rec.reset_merge_loop();
    rec.add("MergeLoopEnd", None);
    rec.reset_main_loop();
    rec.add_name("raw");
    assert_eq!(
        rec.get_names(),
        vec![
            "Initial",
            "Loop_1_Start",
            "Loop_1_Merge_2_MergeLoopStart",
            "Loop_1_MergeLoopEnd",
            "raw"
        ]
    );
}

#[test]
fn class_nodes_sort_and_collect_nested_in_java_order() {
    let root = node("p/Outer", ClassNodeType::Root);
    let b = node("p/Outer$B", ClassNodeType::Member);
    let a = node("p/Outer$A", ClassNodeType::Member);
    let a1 = node("p/Outer$A$1", ClassNodeType::Anonymous);
    let nested = root.with(|d| d.nested.clone());
    nested.add(b);
    nested.add(a);
    a.with(|d| d.nested.clone()).add(a1);
    let mut sorted = nested.to_vec();
    sorted.sort_by(|x, y| x.compare_to(*y));
    nested.set_all(sorted);
    assert_eq!(
        qualified_names(&nested.to_vec()),
        vec!["p/Outer$A", "p/Outer$B"]
    );
    assert_eq!(
        qualified_names(root.get_all_nested().iter()),
        vec!["p/Outer$A", "p/Outer$B", "p/Outer$A$1"]
    );
    assert_eq!(root.get_class_node("p/Outer$B"), Some(b));
    assert_eq!(root.get_class_node("p/Outer$A$1"), None);
    assert_eq!(root.to_string(), "ROOT class p/Outer");
    root.with_mut(|d| d.access = 0x0011);
    assert!(root.has_modifier(0x0001) && root.has_modifier(0x0011) && !root.has_modifier(0x0008));
}

#[test]
fn string_order_is_utf16_order() {
    assert!(string_compare_to("a/B", "a/b").is_lt());
    assert!(string_compare_to("a/b", "a/b$1").is_lt());
    // U+FF21 sorts before U+1F600 by code point but after its surrogate 0xD83D in UTF-16.
    assert!(string_compare_to("\u{FF21}", "\u{1F600}").is_gt());
}

#[test]
fn wrapper_lookup_skips_lambdas_and_drops_finished_classes() {
    let root = node("p/Outer", ClassNodeType::Root);
    let lambda = node("p/Outer##Lambda_0_1", ClassNodeType::Lambda);
    lambda.with_mut(|d| {
        d.parent = Some(root);
        d.lambda_information = Some(LambdaInformation::default());
    });
    assert_eq!(lambda.get_wrapper(), None);
    let wrapper = ClassWrapper::new(root);
    root.with_mut(|d| d.wrapper = Some(wrapper));
    assert_eq!(lambda.get_wrapper(), Some(wrapper));
    assert_eq!(wrapper.get_class_struct(), root.with(|d| d.class_struct));
    wrapper
        .get_hidden_members()
        .borrow_mut()
        .add("f I".to_string());
    assert!(root
        .get_wrapper()
        .unwrap()
        .get_hidden_members()
        .borrow()
        .contains(&"f I".to_string()));
    reset_class_scope();
    assert!(root.is_live());
    assert_eq!(root.get_wrapper(), None);
}

#[test]
fn modifiers_print_in_declaration_order() {
    assert_eq!(
        ClassWriter::get_modifiers(0x0001 | 0x0008 | 0x0010),
        "public static final"
    );
    assert_eq!(
        ClassWriter::get_modifiers(0x0400 | 0x0002 | 0x0800),
        "private abstract strictfp"
    );
    assert_eq!(ClassWriter::get_modifiers(0x1000), "");
}

fn collector(
    simple_names: &[(&str, &str)],
    not_imported: &[&str],
    package: &str,
) -> ImportCollector {
    let mut map_simple_names = JHashMap::new();
    for (name, pkg) in simple_names {
        map_simple_names.put(name.to_string(), pkg.to_string());
    }
    let mut set_not_imported_names = JHashSet::new();
    for name in not_imported {
        set_not_imported_names.add(name.to_string());
    }
    ImportCollector::alloc(ImportCollectorData {
        map_simple_names,
        set_not_imported_names,
        set_field_names: JHashSet::new(),
        map_inner_class_names: JHashMap::new(),
        current_package_slash: format!("{}/", package.replace('.', "/")),
        current_package_point: package.to_string(),
        write_locked: false,
    })
}

#[test]
fn imports_are_filtered_and_sorted_by_package_then_name() {
    let imp = collector(
        &[
            ("List", "java.util"),
            ("String", "java.lang"),
            ("Map", "java.util"),
            ("Local", "com.example"),
            ("Hidden", "org.other"),
            ("Default", ""),
            ("Atomic", "java.util.concurrent.atomic"),
            ("Zed", "a.b"),
        ],
        &["Hidden"],
        "com.example",
    );
    assert_eq!(
        imp.pack_imports(),
        vec![
            "a.b.Zed",
            "java.util.List",
            "java.util.Map",
            "java.util.concurrent.atomic.Atomic"
        ]
    );
    assert!(imp.keep_import(("List", "java.util")));
    assert!(!imp.keep_import(("Object", "java.lang")));
    let copy = ImportCollector::new_copy(imp);
    assert_eq!(copy.pack_imports(), imp.pack_imports());
    assert_eq!(
        copy.with(|d| d.current_package_point.clone()),
        "com.example"
    );
}

#[test]
fn import_lock_restores_the_previous_state() {
    let imp = collector(&[], &[], "");
    {
        let _outer = imp.lock();
        assert!(imp.with(|d| d.write_locked));
        imp.lock().close();
        assert!(imp.with(|d| d.write_locked));
    }
    assert!(!imp.with(|d| d.write_locked));
    let caught = try_catch(|| {
        let _lock = imp.lock();
        panic!("unwinds through the lock");
    });
    assert!(caught.is_err());
    assert!(!imp.with(|d| d.write_locked));
}

#[test]
fn mapping_tracer_keeps_the_first_line_per_offset() {
    let tracer = BytecodeMappingTracer::new_line(10);
    tracer.add_mapping(4);
    tracer.increment_current_source_line();
    tracer.add_mapping(4);
    tracer.add_mapping(7);
    let mut offsets = BitSet::new();
    offsets.set(2);
    offsets.set(9);
    tracer.increment_current_source_line_by(3);
    tracer.add_mapping_bitset(Some(&offsets));
    tracer.add_mapping_bitset(None);
    assert_eq!(tracer.get_current_source_line(), 14);
    assert_eq!(
        tracer.get_mapping().borrow().entry_vec(),
        vec![(2, 14), (4, 10), (7, 11), (9, 14)]
    );

    let other = BytecodeMappingTracer::new();
    other.add_mapping(4);
    other.add_mapping(5);
    tracer.add_tracer(Some(other));
    tracer.add_tracer(None);
    assert_eq!(tracer.get_mapping().borrow().get_cloned(&4), Some(10));
    assert_eq!(tracer.get_mapping().borrow().get_cloned(&5), Some(0));
    assert!(tracer.get_original_lines_mapping().is_empty());

    let dummy = BytecodeMappingTracer::DUMMY();
    assert_eq!(BytecodeMappingTracer::DUMMY(), dummy);
    reset_class_scope();
    assert!(BytecodeMappingTracer::DUMMY().is_live());
}

#[test]
fn source_mapper_collects_tracers_per_class_and_method() {
    let mapper = BytecodeSourceMapper::new();
    let tracer = BytecodeMappingTracer::new_line(3);
    tracer.add_mapping(0);
    tracer.add_mapping(8);
    mapper.add_tracer("a/A", "run ()V", tracer);
    mapper.add_mapping("a/A", "run ()V", 8, 99);
    mapper.add_mapping("a/A", "<init> ()V", 1, 1);
    mapper.add_mapping("a/B", "x ()V", 2, 5);
    let (classes, methods) = mapper.with(|d| {
        (
            d.mapping.key_vec(),
            d.mapping.get(&"a/A".to_string()).unwrap().key_vec(),
        )
    });
    assert_eq!(classes, vec!["a/A", "a/B"]);
    assert_eq!(methods, vec!["run ()V", "<init> ()V"]);
    let run_line = mapper.with(|d| {
        d.mapping
            .get(&"a/A".to_string())
            .unwrap()
            .get(&"run ()V".to_string())
            .unwrap()
            .get_cloned(&8)
    });
    assert_eq!(run_line, Some(3));

    mapper.add_total_offset(5);
    mapper.add_total_offset(2);
    mapper.with_mut(|d| {
        d.lines_mapping.put(12, 3);
        d.lines_mapping.put(4, 1);
        d.unmapped_lines.insert(4);
        d.unmapped_lines.insert(30);
    });
    assert_eq!(mapper.get_original_lines_mapping(), vec![4, 9, 12, 11]);
    assert_eq!(
        mapper.with(|d| d.unmapped_lines.iter().copied().collect::<Vec<_>>()),
        vec![30]
    );
}

struct CountingPlugin {
    id: &'static str,
    runs: Rc<Cell<i32>>,
}

impl Plugin for CountingPlugin {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn description(&self) -> String {
        format!("counting plugin {}", self.id)
    }

    fn register_java_passes(&self, registrar: &mut JavaPassRegistrar) {
        let runs = self.runs.clone();
        let changing: Rc<dyn Pass> = Rc::new(move |_: &mut PassContext| {
            runs.set(runs.get() + 1);
            true
        });
        registrar.register(
            JavaPassLocation::MainLoop,
            NamedPass::of("First", changing.clone()),
        );
        registrar.register(
            JavaPassLocation::MainLoop,
            NamedPass::of("Second", changing.clone()),
        );
        registrar.register(JavaPassLocation::AtEnd, NamedPass::of("Quiet", NO_OP()));
        registrar.register(JavaPassLocation::AtEnd, NamedPass::of("End", changing));
    }
}

#[test]
fn plugin_passes_run_per_location() {
    let runs = Rc::new(Cell::new(0));
    let plugins = PluginContext::new();
    plugins.register_plugin(Rc::new(CountingPlugin {
        id: "counting",
        runs: runs.clone(),
    }));
    assert_eq!(plugins.find_plugins(), 0);
    plugins.initialize();
    plugins.initialize();

    let rec = DecompileRecord::new(dummy_method());
    let mut ctx = pass_context(rec);
    assert!(plugins.run_passes(JavaPassLocation::MainLoop, &mut ctx));
    assert_eq!(runs.get(), 1);
    assert!(!plugins.run_passes(JavaPassLocation::AtEnd, &mut ctx));
    assert_eq!(runs.get(), 2);
    assert!(!plugins.run_passes(JavaPassLocation::BeforeMain, &mut ctx));
    assert_eq!(rec.get_names(), vec!["First", "End"]);
    assert!(plugins.get_language_spec(dummy_class("a/A")).is_none());
    assert!(plugins.get_variable_renamer().is_none());
    assert_eq!(plugins.get_plugins().len(), 1);
}

#[test]
fn duplicate_plugin_ids_are_rejected() {
    let plugins = PluginContext::new();
    for _ in 0..2 {
        plugins.register_plugin(Rc::new(CountingPlugin {
            id: "same",
            runs: Rc::new(Cell::new(0)),
        }));
    }
    let caught = try_catch(|| plugins.initialize()).unwrap_err();
    assert!(caught.is("IllegalStateException"), "{}", caught.message);
    assert!(
        caught.message.contains("with id same"),
        "{}",
        caught.message
    );
}

#[test]
fn pass_builders_compose_like_vineflower() {
    let rec = DecompileRecord::new(dummy_method());
    let mut ctx = pass_context(rec);

    let budget = Rc::new(Cell::new(2));
    let looped = Rc::new(Cell::new(0));
    let fell = Rc::new(Cell::new(0));
    let (b, l, f) = (budget.clone(), looped.clone(), fell.clone());
    let looping = LoopingPassBuilder::new("Loop")
        .add_looping_pass(
            "Shrink",
            Rc::new(move |_: &mut PassContext| {
                l.set(l.get() + 1);
                if b.get() > 0 {
                    b.set(b.get() - 1);
                    true
                } else {
                    false
                }
            }),
        )
        .add_fallthrough_pass(
            "After",
            Rc::new(move |_: &mut PassContext| {
                f.set(f.get() + 1);
                true
            }),
        )
        .build();
    assert!(looping.run(&mut ctx));
    assert_eq!((looped.get(), fell.get()), (3, 1));
    assert_eq!(rec.get_names(), vec!["Shrink", "Shrink", "After"]);

    let wrapped_runs = Rc::new(Cell::new(0));
    let w = wrapped_runs.clone();
    let main = MainPassBuilder::new()
        .add_pass("Nothing", NO_OP())
        .add_pass_named(NamedPass::new(
            "Wrapped",
            WrappedPass::of(move |_| w.set(w.get() + 1)),
        ))
        .build();
    assert!(main.run(&mut ctx));
    assert_eq!(wrapped_runs.get(), 1);
    let quiet = MainPassBuilder::new().add_pass("Nothing", NO_OP()).build();
    assert!(!quiet.run(&mut ctx));
    assert_eq!(rec.get_names().last().map(String::as_str), Some("Wrapped"));
}
