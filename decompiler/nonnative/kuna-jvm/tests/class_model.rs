//! The class model, pass hooks and collectors of slice K3.
//!
//! Classes come from `tests/data/class_model` (own `classes/`, library `lib/`);
//! `golden.txt` is Vineflower 1.12.0's view of the same fixture (`gen.sh`).
//! `loadClasses` is not ported yet, so [`load_nodes`] builds the node tree it
//! produces and the golden `nodes` section checks it.

use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

use kuna_jvm::bytecode_mapping_tracer::BytecodeMappingTracer;
use kuna_jvm::bytecode_source_mapper::BytecodeSourceMapper;
use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::class_writer::ClassWriter;
use kuna_jvm::classes_processor::{string_compare_to, ClassNode, ClassNodeType};
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompile_record::DecompileRecord;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::import_collector::{ImportCollector, ImportCollectorData};
use kuna_jvm::interpreter_util::InterpreterUtil;
use kuna_jvm::java_pass_location::JavaPassLocation;
use kuna_jvm::java_pass_registrar::JavaPassRegistrar;
use kuna_jvm::looping_pass_builder::LoopingPassBuilder;
use kuna_jvm::main_pass_builder::MainPassBuilder;
use kuna_jvm::method_descriptor::MethodDescriptor;
use kuna_jvm::method_wrapper::MethodWrapper;
use kuna_jvm::named_pass::NamedPass;
use kuna_jvm::pass::{Pass, NO_OP};
use kuna_jvm::pass_context::PassContext;
use kuna_jvm::plugin_context::{Plugin, PluginContext};
use kuna_jvm::rt::{reset_class_scope, throw, try_catch, BitSet, JHashMap, JHashSet};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_context::StructContext;
use kuna_jvm::struct_general_attribute::StructGeneralAttribute;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type::VarType;
use kuna_jvm::wrapped_pass::WrappedPass;

use crate::common::{self, ContextGuard};

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/class_model")
}

/// The default context, minus exception dumps, over the fixture classes.
fn fixture() -> (ContextGuard, StructContext) {
    let guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
    ]);
    let sc = guard.struct_context();
    sc.add_space_file(&fixture_dir().join("classes"), true);
    sc.add_space_file(&fixture_dir().join("lib"), false);
    (guard, sc)
}

fn class(sc: StructContext, name: &str) -> StructClass {
    sc.get_class(name)
        .unwrap_or_else(|| panic!("fixture class {name}"))
}

fn run_method(sc: StructContext) -> StructMethod {
    class(sc, "p/Outer").get_method("run", "()V").unwrap()
}

fn pass_context(sc: StructContext, rec: DecompileRecord) -> PassContext {
    let mt = run_method(sc);
    let md = MethodDescriptor::parse_descriptor_struct(mt, None);
    PassContext::new(
        None,
        None,
        mt,
        class(sc, "p/Outer"),
        VarProcessor::new(mt, &md),
        rec,
    )
}

/// `ClassesProcessor.loadClasses` over the fixture, as Vineflower builds it.
fn load_nodes(guard: &ContextGuard, sc: StructContext) -> ClassNode {
    let roots = guard.class_processor().get_map_root_classes();
    let mut nodes = Vec::new();
    for name in [
        "p/Outer",
        "p/Outer$1Local",
        "p/Outer$A",
        "p/Outer$A$1",
        "p/Outer$B",
    ] {
        let cl = class(sc, name);
        let node = ClassNode::new(ClassNodeType::Root, cl);
        node.with_mut(|d| d.access = cl.get_access_flags());
        roots.borrow_mut().put(name.to_string(), node);
        nodes.push(node);
    }
    let [outer, local, a, a1, b] = nodes[..] else {
        unreachable!()
    };

    let nest = |parent: ClassNode,
                child: ClassNode,
                type_: ClassNodeType,
                simple_name: Option<&str>,
                access: i32,
                enclosing: &[&str]| {
        child.with_mut(|d| {
            d.simple_name = simple_name.map(str::to_string);
            d.type_ = type_;
            if type_ == ClassNodeType::Anonymous {
                d.enclosing_method = None;
            }
            d.access = access;
            d.parent = Some(parent);
        });
        if type_ == ClassNodeType::Anonymous {
            let cl = child.with(|d| d.class_struct);
            let type_ = VarType::new_signature_cl(&cl.get_interface(0), true);
            child.with_mut(|d| d.anonymous_class_type = Some(type_));
        }
        parent.with(|d| d.nested.clone()).add(child);
        let set = child.with(|d| d.enclosing_classes.clone());
        for name in enclosing {
            set.borrow_mut().add(name.to_string());
        }
    };
    nest(
        outer,
        local,
        ClassNodeType::Local,
        Some("Local"),
        0,
        &["p/Outer"],
    );
    nest(
        outer,
        a,
        ClassNodeType::Member,
        Some("A"),
        0x0001,
        &["p/Outer"],
    );
    nest(
        outer,
        b,
        ClassNodeType::Member,
        Some("B"),
        0x0009,
        &["p/Outer"],
    );
    nest(
        a,
        a1,
        ClassNodeType::Anonymous,
        None,
        0,
        &["p/Outer", "p/Outer$A"],
    );
    for node in [outer, a] {
        let nested = node.with(|d| d.nested.clone());
        let mut sorted = nested.to_vec();
        sorted.sort_by(|x, y| x.compare_to(*y));
        nested.set_all(sorted);
    }
    outer
}

fn esc(s: Option<&str>) -> String {
    match s {
        None => "\\N".to_string(),
        Some(s) => s
            .replace('\\', "\\\\")
            .replace('\t', "\\t")
            .replace('\n', "\\n"),
    }
}

fn qualified_name(node: ClassNode) -> String {
    node.with(|d| d.class_struct).qualified_name()
}

fn node_line(n: ClassNode) -> String {
    let (type_, simple_name, access, parent, nested, enclosing_method, enclosing, anonymous) = n
        .with(|d| {
            (
                d.type_,
                d.simple_name.clone(),
                d.access,
                d.parent,
                d.nested.to_vec(),
                d.enclosing_method.clone(),
                d.enclosing_classes.borrow().to_vec(),
                d.anonymous_class_type.clone(),
            )
        });
    let mut enclosing = enclosing;
    enclosing.sort();
    [
        qualified_name(n),
        type_.name().to_string(),
        esc(simple_name.as_deref()),
        access.to_string(),
        parent.map_or("\\N".to_string(), qualified_name),
        nested
            .into_iter()
            .map(qualified_name)
            .collect::<Vec<_>>()
            .join(","),
        esc(enclosing_method.as_deref()),
        enclosing.join(","),
        anonymous.map_or("\\N".to_string(), |t| t.value_str().to_string()),
        n.to_string(),
    ]
    .join("\t")
}

fn lambda_line(n: ClassNode) -> String {
    let (type_, simple_name, l, anonymous) = n.with(|d| {
        (
            d.type_,
            d.simple_name.clone(),
            d.lambda_information.clone().unwrap(),
            d.anonymous_class_type.clone().unwrap(),
        )
    });
    [
        type_.name().to_string(),
        esc(simple_name.as_deref()),
        l.method_name,
        l.method_descriptor,
        l.content_class_name,
        l.content_method_name,
        l.content_method_descriptor,
        l.content_method_invocation_type.to_string(),
        l.content_method_key,
        l.is_method_reference.to_string(),
        l.is_content_method_static.to_string(),
        anonymous.value_str().to_string(),
        n.to_string(),
    ]
    .join("\t")
}

fn text(buffer: &TextBuffer) -> String {
    esc(Some(&buffer.convert_to_string_and_allow_data_discard()))
}

#[test]
fn class_model_matches_vineflower() {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let mut out: Vec<String> = Vec::new();

    out.push("== nodes".to_string());
    let roots = guard.class_processor().get_map_root_classes();
    let mut names = roots.borrow().key_vec();
    names.sort_by(|a, b| string_compare_to(a, b));
    for name in &names {
        let node = roots.borrow().get_cloned(name).unwrap();
        out.push(node_line(node));
    }
    let all: Vec<String> = outer
        .get_all_nested()
        .to_vec()
        .into_iter()
        .map(qualified_name)
        .collect();
    out.push(format!("all\t{}", all.join(",")));

    out.push("== lambdas".to_string());
    let cl = class(sc, "p/Outer");
    for (content_class, content_name, content_desc, kind, lambda_class, name, desc) in [
        (
            "p/Outer",
            "lambda$new$0",
            "()V",
            6,
            "java/lang/Runnable",
            "run",
            "()V",
        ),
        (
            "java/lang/Object",
            "toString",
            "()Ljava/lang/String;",
            5,
            "java/util/function/Supplier",
            "get",
            "()Ljava/lang/Object;",
        ),
        (
            "p/Outer",
            "run",
            "()V",
            5,
            "java/lang/Runnable",
            "run",
            "()V",
        ),
    ] {
        let node = ClassNode::new_lambda(
            content_class,
            content_name,
            content_desc,
            kind,
            lambda_class,
            name,
            desc,
            cl,
        );
        out.push(lambda_line(node));
    }

    out.push("== short names".to_string());
    let imports = ImportCollector::new(outer);
    DecompilerContext::set_current_class_node(Some(outer));
    for q in [
        "java.util.List",
        "java.awt.List",
        "java.lang.String",
        "p.Outer",
        "p.Outer$A",
        "p.Outer$A$1",
        "p.Outer$1Local",
        "q.Outer",
        "x.y.A",
        "p.Outer.B",
        "q.Other$A",
        "java.util.concurrent.Future",
        "a.b.Zed",
        "Default",
        "java.util.List",
        "x.field",
    ] {
        out.push(format!(
            "short\t{q}\t{}",
            esc(imports.get_short_name(q).as_deref())
        ));
    }
    out.push(format!(
        "short\tjava.util.Map\tnot-imported\t{}",
        esc(imports
            .get_short_name_imported("java.util.Map", false)
            .as_deref())
    ));
    {
        let _lock = imports.lock();
        out.push(format!(
            "short\tjava.util.Set\tlocked\t{}",
            esc(imports.get_short_name("java.util.Set").as_deref())
        ));
    }
    out.push(format!(
        "short\tjava.awt.Set\t{}",
        esc(imports.get_short_name("java.awt.Set").as_deref())
    ));
    for q in ["x.field", "java.util.List", "p.Outer$A$1"] {
        out.push(format!(
            "context\t{q}\t{}",
            esc(Some(&imports.get_short_name_in_class_context(q)))
        ));
    }
    let mut buffer = TextBuffer::new();
    imports.write_imports(&mut buffer, true);
    out.push(format!("imports\t{}", text(&buffer)));
    let mut header = TextBuffer::new();
    ClassWriter::new().write_class_header(cl, &mut header, imports);
    out.push(format!("header\t{}", text(&header)));
    let b = roots.borrow().get_cloned(&"p/Outer$B".to_string()).unwrap();
    let empty = ImportCollector::new(b);
    let mut header = TextBuffer::new();
    ClassWriter::new().write_class_header(b.with(|d| d.class_struct), &mut header, empty);
    out.push(format!("header-empty\t{}", text(&header)));

    out.push("== mapper".to_string());
    let run = run_method(sc);
    let tracer = BytecodeMappingTracer::new_line(3);
    tracer.set_line_number_table(
        run.get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE),
    );
    tracer.add_mapping(0);
    tracer.increment_current_source_line();
    tracer.add_mapping(8);
    let mapper = BytecodeSourceMapper::new();
    let mut none = TextBuffer::new();
    mapper.dump_mapping(&mut none, false);
    out.push(format!("empty\t{}", text(&none)));
    mapper.add_tracer("p/Outer", "run ()V", tracer);
    mapper.add_mapping("p/Outer", "<init> ()V", 4, 7);
    mapper.add_mapping("p/Outer", "run ()V", 8, 99);
    mapper.add_mapping("q/Other", "x ()V", 255, 5);
    mapper.add_total_offset(2);
    for hex in [false, true] {
        let mut dump = TextBuffer::new();
        mapper.dump_mapping(&mut dump, hex);
        out.push(format!("dump\t{hex}\t{}", text(&dump)));
    }
    let lines: Vec<String> = mapper
        .get_original_lines_mapping()
        .iter()
        .map(i32::to_string)
        .collect();
    out.push(format!("lines\t{}", lines.join(",")));
    let mut after = TextBuffer::new();
    mapper.dump_mapping(&mut after, false);
    out.push(format!("dump\tafter\t{}", text(&after)));

    out.push("== writer".to_string());
    let caught = try_catch(|| {
        throw("RuntimeException", "boom");
    })
    .unwrap_err();
    let mut exception = TextBuffer::new();
    ClassWriter::write_exception(&mut exception, &caught);
    out.push(format!("exception\t{}", text(&exception)));
    out.push(format!(
        "error-comment\t{}",
        esc(Some(&ClassWriter::get_error_comment().join("\n")))
    ));

    let golden = std::fs::read_to_string(fixture_dir().join("golden.txt")).unwrap();
    let expected: Vec<&str> = golden.lines().collect();
    for (i, (want, got)) in expected.iter().zip(&out).enumerate() {
        assert_eq!(want, got, "golden line {}", i + 1);
    }
    assert_eq!(expected.len(), out.len());
}

#[test]
fn class_nodes_answer_like_vineflower() {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let a = outer.get_class_node("p/Outer$A").unwrap();
    assert_eq!(qualified_name(a), "p/Outer$A");
    assert_eq!(outer.get_class_node("p/Outer$A$1"), None);
    assert!(a.compare_to(outer).is_gt());
    assert!(outer.has_modifier(0x0001) && outer.has_modifier(0x0021));
    assert!(!outer.has_modifier(0x0008));
    let local = outer.get_class_node("p/Outer$1Local").unwrap();
    let enclosing = class(sc, "p/Outer$1Local");
    assert_eq!(
        ClassNode::new(ClassNodeType::Local, enclosing).with(|d| d.enclosing_method.clone()),
        Some("run ()V".to_string())
    );
    assert_eq!(local.with(|d| d.type_), ClassNodeType::Local);

    let cp = guard.class_processor();
    assert!(cp.is_whitelisted("q/Other"));
    cp.add_whitelist("p/");
    assert!(cp.is_whitelisted("p/Outer"));
    assert!(!cp.is_whitelisted("q/Other"));
}

#[test]
fn string_order_is_utf16_order() {
    assert!(string_compare_to("a/B", "a/b").is_lt());
    assert!(string_compare_to("a/b", "a/b$1").is_lt());
    // U+FF21 sorts before U+1F600 by code point but after its surrogate 0xD83D in UTF-16.
    assert!(string_compare_to("\u{FF21}", "\u{1F600}").is_gt());
}

#[test]
fn wrappers_live_for_the_run_and_lambdas_use_the_parent() {
    let (guard, sc) = fixture();
    let outer = load_nodes(&guard, sc);
    let cl = class(sc, "p/Outer");
    let lambda = ClassNode::new_lambda(
        "p/Outer",
        "lambda$new$0",
        "()V",
        6,
        "java/lang/Runnable",
        "run",
        "()V",
        cl,
    );
    lambda.with_mut(|d| d.parent = Some(outer));
    assert_eq!(lambda.get_wrapper(), None);
    let wrapper = ClassWrapper::new(outer);
    outer.with_mut(|d| d.wrapper = Some(wrapper));
    assert_eq!(lambda.get_wrapper(), Some(wrapper));
    assert_eq!(wrapper.get_class_struct(), cl);
    assert_eq!(wrapper.to_string(), "p/Outer");
    wrapper
        .get_hidden_members()
        .borrow_mut()
        .add("field I".to_string());
    assert!(outer
        .get_wrapper()
        .unwrap()
        .get_hidden_members()
        .borrow()
        .contains(&"field I".to_string()));

    let mt = run_method(sc);
    let md = MethodDescriptor::parse_descriptor_struct(mt, None);
    let mw = MethodWrapper::new(
        None,
        VarProcessor::new(mt, &md),
        mt,
        cl,
        CounterContainer::new(),
    );
    mw.add_comment("first");
    mw.add_comment("second");
    mw.add_comment("first");
    assert_eq!(
        mw.with(|d| d.comment_lines.as_ref().unwrap().to_vec()),
        vec!["first", "second"]
    );
    assert_eq!(mw.to_string(), "{p/Outer: run}");
    assert!(mw.desc().params.is_empty());
    assert!(mw.get_or_build_graph().is_none());
    wrapper
        .get_methods()
        .borrow_mut()
        .add_with_key(mw, InterpreterUtil::make_unique_key("run", "()V"));
    assert_eq!(wrapper.get_method_wrapper("run", "()V"), Some(mw));
    assert_eq!(wrapper.get_method_wrapper("run", "(I)V"), None);
    assert_eq!(wrapper.get_method_wrapper_index(0), mw);

    reset_class_scope();
    assert!(outer.is_live());
    assert_eq!(lambda.get_wrapper(), None);
}

#[test]
fn decompile_record_prefixes_loop_iterations() {
    let (_guard, sc) = fixture();
    let rec = DecompileRecord::new(run_method(sc));
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

#[test]
fn removed_imports_write_nothing() {
    let (_guard, sc) = fixture();
    DecompilerContext::set_property(IFernflowerPreferences::REMOVE_IMPORTS, "1");
    let imports = ImportCollector::new(ClassNode::new(ClassNodeType::Root, class(sc, "p/Outer")));
    assert_eq!(
        imports.get_short_name("java.util.List").as_deref(),
        Some("List")
    );
    let mut buffer = TextBuffer::new();
    imports.write_imports(&mut buffer, true);
    assert_eq!(buffer.convert_to_string_and_allow_data_discard(), "");
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
    let (_guard, sc) = fixture();
    let runs = Rc::new(Cell::new(0));
    let plugins = PluginContext::new();
    plugins.register_plugin(Rc::new(CountingPlugin {
        id: "counting",
        runs: runs.clone(),
    }));
    assert_eq!(plugins.find_plugins(), 0);
    plugins.initialize();
    plugins.initialize();

    let rec = DecompileRecord::new(run_method(sc));
    let mut ctx = pass_context(sc, rec);
    assert!(plugins.run_passes(JavaPassLocation::MainLoop, &mut ctx));
    assert_eq!(runs.get(), 1);
    assert!(!plugins.run_passes(JavaPassLocation::AtEnd, &mut ctx));
    assert_eq!(runs.get(), 2);
    assert!(!plugins.run_passes(JavaPassLocation::BeforeMain, &mut ctx));
    assert_eq!(rec.get_names(), vec!["First", "End"]);
    assert!(plugins.get_language_spec(class(sc, "p/Outer")).is_none());
    assert!(plugins.get_variable_renamer().is_none());
    assert_eq!(plugins.get_plugins().len(), 1);
    assert_eq!(
        PluginContext::get_current_context(),
        sc.get_plugin_context()
    );
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
    let (_guard, sc) = fixture();
    let rec = DecompileRecord::new(run_method(sc));
    let mut ctx = pass_context(sc, rec);
    assert_eq!(ctx.get_method_descriptor().params.len(), 0);

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
