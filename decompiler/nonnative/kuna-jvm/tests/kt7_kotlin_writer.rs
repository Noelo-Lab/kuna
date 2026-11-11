//! `KotlinWriter` against Vineflower 1.12.0's own Kotlin plugin (`tests/data/kt7`,
//! `gen.sh`), on what needs no other plugin body: `toValidKotlinIdentifier`, the
//! modifier table, `appendJvmAnnotations`, `processParameterAnnotations`,
//! `isNullable`, `searchForMethod`, `hideConstructor`, `appendTypeParameters`,
//! `appendAnnotations` where every declaration annotation is filtered out,
//! `writeField` (Java-shaped fields) and `writeMethod` on methods whose wrapper
//! carries a decompile error (header, `writeMethodBody`, `dumpError`), in compat mode
//! (kuna fix `kt_failed_method_hidden` prints a method Vineflower hides).

use std::path::PathBuf;

use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::code_constants::{
    ACC_ABSTRACT, ACC_ANNOTATION, ACC_ENUM, ACC_FINAL, ACC_INTERFACE, ACC_NATIVE, ACC_PRIVATE,
    ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED, ACC_TRANSIENT,
    ACC_VOLATILE,
};
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::import_collector::ImportCollector;
use kuna_jvm::interpreter_util::InterpreterUtil;
use kuna_jvm::kotlin_writer::KotlinWriter;
use kuna_jvm::kuna_fixes::{self, KunaFixes};
use kuna_jvm::method_descriptor::MethodDescriptor;
use kuna_jvm::method_wrapper::MethodWrapper;
use kuna_jvm::rt::{throw, try_catch, JavaStr};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_member::StructMember;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::type_annotation::TypeAnnotation;
use kuna_jvm::var_processor::VarProcessor;

use crate::common::{self, ContextGuard};

const IDENTIFIERS: [&str; 138] = [
    "as",
    "break",
    "class",
    "continue",
    "do",
    "else",
    "false",
    "for",
    "fun",
    "if",
    "in",
    "interface",
    "is",
    "null",
    "object",
    "package",
    "return",
    "super",
    "this",
    "throw",
    "true",
    "try",
    "typealias",
    "typeof",
    "val",
    "var",
    "when",
    "while",
    "by",
    "catch",
    "constructor",
    "delegate",
    "dynamic",
    "field",
    "file",
    "finally",
    "get",
    "import",
    "init",
    "param",
    "property",
    "receiver",
    "set",
    "setparam",
    "value",
    "where",
    "abstract",
    "actual",
    "annotation",
    "companion",
    "const",
    "crossinline",
    "data",
    "enum",
    "expect",
    "external",
    "final",
    "infix",
    "inline",
    "inner",
    "internal",
    "lateinit",
    "noinline",
    "open",
    "operator",
    "out",
    "override",
    "private",
    "protected",
    "public",
    "reified",
    "sealed",
    "suspend",
    "tailrec",
    "vararg",
    "it",
    "int",
    "static",
    "new",
    "void",
    "boolean",
    "default",
    "case",
    "switch",
    "goto",
    "abc",
    "Abc",
    "_",
    "__",
    "_x",
    "x1",
    "1abc",
    "",
    " ",
    "a b",
    "a-b",
    "a.b",
    "a/b",
    "<init>",
    "<clinit>",
    "$",
    "$x",
    "x$",
    "a$b",
    "$this$foo",
    "`",
    "a`b",
    "`a`",
    "``",
    "a`$",
    "$`",
    "1`",
    "\u{fc}n\u{ef}",
    "caf\u{e9}",
    "\u{3c0}",
    "\u{df}",
    "\u{65e5}\u{672c}\u{8a9e}",
    "a\u{0}b",
    "\u{0}",
    "\u{1f600}x",
    "x\u{1f600}",
    "\u{200b}x",
    "x\u{200b}",
    "\u{660}x",
    "x\u{660}",
    "a\tb",
    "a\nb",
    "a\u{2028}b",
    "\u{aa}",
    "x\u{b7}",
    "\u{2b0}x",
    "Ab$Companion",
    "access$getX$p",
    "get-impl",
    "component1",
    "x;y",
    "a:b",
    "A[]",
];

const CLASSES: [&str; 11] = [
    "k7/Fix",
    "k7/Fix$Inner",
    "k7/Iface",
    "k7/Base",
    "k7/Annos",
    "k7/One",
    "k7/PkgOne",
    "k7/En",
    "k7/Sfp",
    "k7/Gen",
    "k7/Rec",
];

const FLAG_SETS: [i32; 14] = [
    0,
    ACC_PUBLIC,
    ACC_PRIVATE,
    ACC_PROTECTED,
    ACC_PUBLIC | ACC_STATIC | ACC_FINAL,
    ACC_ABSTRACT,
    ACC_PUBLIC | ACC_ABSTRACT,
    ACC_NATIVE | ACC_PRIVATE,
    ACC_STATIC | ACC_STRICT,
    ACC_SYNCHRONIZED | ACC_PROTECTED,
    ACC_TRANSIENT | ACC_VOLATILE | ACC_PUBLIC,
    ACC_ENUM | ACC_INTERFACE | ACC_ANNOTATION,
    0xffff,
    -1,
];

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt7")
}

/// The oracle's escaping: per UTF-16 unit, `\\`, `\t`, `\n`, other non-printable-ASCII as `\uXXXX`.
fn esc(s: Option<&str>) -> String {
    let Some(s) = s else {
        return "\\N".to_string();
    };
    let mut out = String::new();
    for u in s.utf16() {
        match u {
            0x5C => out.push_str("\\\\"),
            0x09 => out.push_str("\\t"),
            0x0A => out.push_str("\\n"),
            u if !(0x20..=0x7e).contains(&u) => out.push_str(&format!("\\u{u:04x}")),
            u => out.push(u as u8 as char),
        }
    }
    out
}

fn text(buffer: &TextBuffer) -> String {
    esc(Some(&buffer.convert_to_string_and_allow_data_discard()))
}

fn golden(kind: &str) -> Vec<String> {
    std::fs::read_to_string(data_dir().join("golden.txt"))
        .unwrap()
        .lines()
        .filter(|l| l.starts_with(&format!("{kind}\t")))
        .map(str::to_string)
        .collect()
}

fn assert_lines(out: &[String], kind: &str) {
    let expected = golden(kind);
    for (got, want) in out.iter().zip(&expected) {
        assert_eq!(got, want);
    }
    assert_eq!(out.len(), expected.len(), "{kind} line count");
}

/// The oracle's context: no JDK, failed methods dump their bytecode only.
fn context() -> ContextGuard {
    let guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
        (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1"),
    ]);
    guard
        .struct_context()
        .add_space_file(&data_dir().join("classes"), true);
    guard
}

/// The class as a root node whose methods all failed to decompile.
fn node(name: &str) -> ClassNode {
    let cl = DecompilerContext::get_struct_context()
        .get_class(name)
        .unwrap_or_else(|| panic!("fixture class {name}"));
    let node = ClassNode::new(ClassNodeType::Root, cl);
    node.with_mut(|d| {
        d.simple_name = Some(name[name.rfind('/').unwrap() + 1..].to_string());
        d.access = cl.get_access_flags();
    });
    let wrapper = ClassWrapper::new(node);
    node.with_mut(|d| d.wrapper = Some(wrapper));

    let boom = try_catch(|| throw("RuntimeException", "boom")).unwrap_err();
    for mt in cl.get_methods().to_vec() {
        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
        let mw = MethodWrapper::new(
            None,
            VarProcessor::new(mt, &md),
            mt,
            cl,
            CounterContainer::new(),
        );
        mw.with_mut(|d| d.decompile_error = Some(boom.clone()));
        wrapper.get_methods().borrow_mut().add_with_key(
            mw,
            InterpreterUtil::make_unique_key(&mt.get_name(), &mt.get_descriptor()),
        );
    }
    node
}

fn enter(node: ClassNode) -> StructClass {
    DecompilerContext::set_current_class_node(Some(node));
    DecompilerContext::start_class(ImportCollector::new(node));
    node.with(|d| d.class_struct)
}

/// The class, its fields and its methods, labelled like the oracle.
fn members(cl: StructClass) -> Vec<(String, StructMember)> {
    let mut list = vec![("class".to_string(), cl.0)];
    for fd in cl.get_fields().to_vec() {
        list.push((format!("{}:{}", fd.get_name(), fd.get_descriptor()), fd.0));
    }
    for mt in cl.get_methods().to_vec() {
        list.push((method_label(mt), mt.0));
    }
    list
}

fn method_label(mt: StructMethod) -> String {
    format!("{}{}", mt.get_name(), mt.get_descriptor())
}

#[test]
fn valid_kotlin_identifiers_match_vineflower() {
    kuna_fixes::activate(KunaFixes::COMPAT);
    let mut out: Vec<String> = IDENTIFIERS
        .iter()
        .map(|id| {
            format!(
                "ident\t{}\t{}",
                esc(Some(id)),
                esc(Some(&KotlinWriter::to_valid_kotlin_identifier(id)))
            )
        })
        .collect();
    let none: Option<&str> = None;
    out.push(format!(
        "ident\t\\N\t{}",
        esc(none
            .map(KotlinWriter::to_valid_kotlin_identifier)
            .as_deref())
    ));
    assert_lines(&out, "ident");
}

#[test]
fn modifiers_match_vineflower() {
    let _guard = common::install_default_context();
    let expected = golden("mods");
    assert_eq!(expected.len(), FLAG_SETS.len() * 5 * 2);
    for want in &expected {
        let fields: Vec<&str> = want.split('\t').collect();
        let flags: i32 = fields[1].parse().unwrap();
        let allowed: i32 = fields[2].parse().unwrap();
        let is_interface: bool = fields[3].parse().unwrap();
        let excluded: i32 = fields[4].parse().unwrap();
        assert!(FLAG_SETS.contains(&flags));
        let mut buffer = TextBuffer::new();
        KotlinWriter::append_modifiers(&mut buffer, flags, allowed, is_interface, excluded);
        assert_eq!(
            format!(
                "mods\t{flags}\t{allowed}\t{is_interface}\t{excluded}\t{}",
                text(&buffer)
            ),
            *want
        );
    }
}

#[test]
fn member_annotations_and_lookups_match_vineflower() {
    let _guard = context();
    kuna_fixes::activate(KunaFixes::COMPAT);
    let targets = [
        TypeAnnotation::METHOD_RETURN_TYPE,
        TypeAnnotation::FIELD,
        TypeAnnotation::CLASS_TYPE_PARAMETER,
        TypeAnnotation::METHOD_PARAMETER,
    ];
    let (mut jvm, mut nullable, mut search, mut param) = (vec![], vec![], vec![], vec![]);
    for name in CLASSES {
        let cl = enter(node(name));
        for (label, mb) in members(cl) {
            for target in targets {
                for combo in 0..4 {
                    let mut buffer = TextBuffer::new();
                    KotlinWriter::append_jvm_annotations(
                        &mut buffer,
                        1,
                        mb,
                        (combo & 1) != 0,
                        (combo & 2) != 0,
                        cl.get_pool(),
                        target,
                    );
                    jvm.push(format!(
                        "jvm\t{name}\t{label}\t{target}\t{combo}\t{}",
                        text(&buffer)
                    ));
                }
            }
        }
        for mt in cl.get_methods().to_vec() {
            let label = method_label(mt);
            nullable.push(format!(
                "nullable\t{name}\t{label}\t{}",
                KotlinWriter::is_nullable(mt.0)
            ));
            let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());
            search.push(format!(
                "search\t{name}\t{label}\t{}\t{}",
                KotlinWriter::search_for_method(Some(cl), &mt.get_name(), &md, false),
                KotlinWriter::search_for_method(Some(cl), &mt.get_name(), &md, true)
            ));
            for p in 0..=md.params.len() as i32 {
                let mut buffer = TextBuffer::new();
                let is_nullable = KotlinWriter::process_parameter_annotations(&mut buffer, mt, p);
                param.push(format!(
                    "param\t{name}\t{label}\t{p}\t{is_nullable}\t{}",
                    text(&buffer)
                ));
            }
        }
    }
    assert_lines(&jvm, "jvm");
    assert_lines(&nullable, "nullable");
    assert_lines(&search, "search");
    assert_lines(&param, "param");
}

#[test]
fn filtered_and_type_annotations_match_vineflower() {
    let _guard = context();
    let cl = enter(node("k7/Annos"));
    let mut out = Vec::new();
    for (label, mb) in members(cl) {
        let target = if label == "class" {
            -1
        } else if label.contains(':') {
            TypeAnnotation::FIELD
        } else {
            TypeAnnotation::METHOD_RETURN_TYPE
        };
        for indent in [-1, 0, 2] {
            let mut buffer = TextBuffer::new();
            KotlinWriter::append_annotations(&mut buffer, indent, mb, target);
            out.push(format!("annos\t{label}\t{indent}\t{}", text(&buffer)));
        }
    }
    assert_lines(&out, "annos");
}

#[test]
fn default_constructor_hiding_matches_vineflower() {
    let _guard = context();
    let mut out = Vec::new();
    for name in ["k7/One", "k7/PkgOne", "k7/Fix", "k7/En"] {
        let node = node(name);
        enter(node);
        for hdc in ["1", "0"] {
            DecompilerContext::set_property(IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR, hdc);
            for combo in 0..8 {
                for flags in [
                    0,
                    ACC_PUBLIC,
                    ACC_PRIVATE,
                    ACC_PROTECTED,
                    ACC_PUBLIC | ACC_STATIC,
                ] {
                    let init = (combo & 1) == 0;
                    let thr = (combo & 2) != 0;
                    let params = if (combo & 4) != 0 { 1 } else { 0 };
                    out.push(format!(
                        "hide\t{name}\t{hdc}\t{init}\t{thr}\t{params}\t{flags}\t{}",
                        KotlinWriter::hide_constructor(node, init, thr, params, flags)
                    ));
                }
            }
        }
        DecompilerContext::set_property(IFernflowerPreferences::HIDE_DEFAULT_CONSTRUCTOR, "1");
    }
    assert_lines(&out, "hide");
}

#[test]
fn type_parameters_match_vineflower() {
    let _guard = context();
    let cl = enter(node("k7/Gen"));
    let mut out = Vec::new();
    let sig = cl.get_signature().unwrap();
    let (fparameters, fbounds) = (sig.fparameters().clone(), sig.fbounds().clone());
    let mut buffer = TextBuffer::new();
    KotlinWriter::append_type_parameters(&mut buffer, &fparameters, &fbounds);
    out.push(format!("typeparams\tclass\t{}", text(&buffer)));
    for mt in cl.get_methods().to_vec() {
        let Some(msig) = mt.get_signature() else {
            continue;
        };
        if msig.type_parameters().is_empty() {
            continue;
        }
        let (parameters, bounds) = (
            msig.type_parameters().clone(),
            msig.type_parameter_bounds().clone(),
        );
        let mut buffer = TextBuffer::new();
        KotlinWriter::append_type_parameters(&mut buffer, &parameters, &bounds);
        out.push(format!(
            "typeparams\t{}\t{}",
            method_label(mt),
            text(&buffer)
        ));
    }
    assert_lines(&out, "typeparams");
}

#[test]
fn java_shaped_fields_match_vineflower() {
    let _guard = context();
    kuna_fixes::activate(KunaFixes::COMPAT);
    let writer = KotlinWriter::new();
    let mut out = Vec::new();
    for name in [
        "k7/Fix",
        "k7/Fix$Inner",
        "k7/Iface",
        "k7/En",
        "k7/Annos",
        "k7/Rec",
    ] {
        let node = node(name);
        let cl = enter(node);
        for fd in cl.get_fields().to_vec() {
            let mut buffer = TextBuffer::new();
            writer.write_field(node.get_wrapper().unwrap(), cl, fd, &mut buffer, 1);
            out.push(format!(
                "field\t{name}\t{}:{}\t{}",
                fd.get_name(),
                fd.get_descriptor(),
                text(&buffer)
            ));
        }
    }
    assert_lines(&out, "field");
}

#[test]
fn failed_methods_match_vineflower() {
    let _guard = context();
    kuna_fixes::activate(KunaFixes::COMPAT);
    let writer = KotlinWriter::new();
    let mut out = Vec::new();
    for name in CLASSES {
        let node = node(name);
        let cl = enter(node);
        let methods = cl.get_methods().to_vec();
        for (i, &mt) in methods.iter().enumerate() {
            let writable =
                mt.get_descriptor() == "()V" || (name == "k7/En" && mt.get_name() == "<init>");
            if !writable {
                continue;
            }
            let mut buffer = TextBuffer::new();
            let shown = writer.write_method(node, mt, i as i32, &mut buffer, 1);
            out.push(format!(
                "method\t{name}\t{}\t{shown}\t{}",
                method_label(mt),
                text(&buffer)
            ));
            assert_eq!(DecompilerContext::get_current_method_wrapper(), None);
        }
    }
    assert_lines(&out, "method");
}

#[test]
fn exception_dumps_precede_the_bytecode_listing() {
    let _guard = common::install_context_with(&[
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0"),
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "1"),
        (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1"),
    ]);
    _guard
        .struct_context()
        .add_space_file(&data_dir().join("classes"), true);
    kuna_fixes::activate(KunaFixes::COMPAT);
    let node = node("k7/One");
    let cl = enter(node);
    let index = cl
        .get_methods()
        .to_vec()
        .iter()
        .position(|mt| mt.get_name() == "<init>")
        .unwrap();
    let mut buffer = TextBuffer::new();
    let mt = cl.get_methods().get(index);
    let shown = KotlinWriter::new().write_method(node, mt, index as i32, &mut buffer, 0);
    assert!(
        !shown,
        "a method that failed to decompile is reported as hidden"
    );
    let text = buffer.convert_to_string_and_allow_data_discard();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "open fun One() {");
    assert_eq!(lines[1], "   // $VF: Couldn't be decompiled");
    assert!(lines[2].starts_with("   // Please report this to the Vineflower issue tracker"));
    assert_eq!(lines[3], "   // RuntimeException: boom");
    assert!(lines[4].starts_with("   //   at "), "{text}");
    assert_eq!(lines[5], "   //");
    assert_eq!(lines[6], "   // Bytecode:");
    assert_eq!(lines[7], "   // 0: aload 0");
    assert_eq!(*lines.last().unwrap(), "}");
}

struct KotlinOnly;

impl kuna_jvm::plugin_source::PluginSource for KotlinOnly {
    fn find_plugins(&self) -> Vec<std::rc::Rc<dyn kuna_jvm::plugin::Plugin>> {
        vec![std::rc::Rc::new(
            kuna_jvm::kotlin_plugin::KotlinPlugin::new(),
        )]
    }
}

/// Vendored Kotlin corpus cases through the whole pipeline with only the Kotlin plugin
/// injected, the way the plugin's own test fixture loads it: the same text as the
/// bundled plugins (the corpus runner) give.
#[test]
fn kotlin_only_plugin_source_matches_the_fixture() {
    use std::path::Path;
    use std::rc::Rc;

    use kuna_jvm::fernflower::Fernflower;
    use kuna_jvm::i_fernflower_logger::CollectingLogger;
    use kuna_jvm::kuna_corpus::{
        engine_options, evaluate, load_cases, reset_engine_thread, with_engine_stack, CaseStatus,
        EngineRun,
    };

    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let cases: Vec<_> = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .filter(|c| {
            [
                "kotlin-entire-classpath/pkg/TestAnyType",
                "kotlin-entire-classpath/pkg/TestDataClass",
                "kotlin-multifile-facade/pkg/TestMultifile",
            ]
            .contains(&c.id.as_str())
        })
        .collect();
    assert_eq!(cases.len(), 3);
    with_engine_stack(move || {
        for mut case in cases {
            reset_engine_thread();
            let logger = Rc::new(CollectingLogger::new());
            case.options
                .push(("jvm_source_locals".to_string(), "0".to_string()));
            let mut fernflower = Fernflower::new_with_plugin_sources(
                &engine_options(&case),
                logger,
                vec![Rc::new(KotlinOnly)],
            );
            for class in &case.class_files {
                fernflower.add_source_file(class);
            }
            let mut units = Vec::new();
            fernflower.decompile_context(&mut |unit| units.push(unit));
            fernflower.clear_context();
            reset_engine_thread();
            let run = EngineRun {
                units,
                ..EngineRun::default()
            };
            let result = evaluate(&case, run);
            assert_eq!(
                result.status,
                CaseStatus::Match,
                "{}: {:?}",
                case.id,
                result.first_diff
            );
        }
    });
}
