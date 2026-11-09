//! `ClassWriter` against Vineflower 1.12.0 (`tests/data/b18_writer`, `gen.sh`):
//! `toValidJavaIdentifier`, and `writeMethod` on methods whose wrapper carries a
//! decompile error, which writes the method header and `dumpError`'s bytecode
//! listing (`collectBytecode`, constants, bootstrap calls, exception table).

use std::path::PathBuf;

use kuna_jvm::class_wrapper::ClassWrapper;
use kuna_jvm::class_writer::ClassWriter;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::interpreter_util::InterpreterUtil;
use kuna_jvm::method_descriptor::MethodDescriptor;
use kuna_jvm::method_wrapper::MethodWrapper;
use kuna_jvm::rt::{throw, try_catch, JavaStr};
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::var_processor::VarProcessor;

use crate::common::{self, ContextGuard};

const IDENTIFIERS: [&str; 17] = [
    "abc",
    "1abc",
    "a-b",
    "\u{fc}n\u{ef}",
    "a\u{0}b",
    "$x",
    "_",
    "",
    "a b",
    "\u{1f600}x",
    "x\u{1f600}",
    "\u{200b}x",
    "x\u{200b}",
    "<init>",
    "a.b/c",
    "\u{660}x",
    "x\u{660}",
];

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/b18_writer")
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

/// The fixture class as a root node whose methods all failed to decompile.
fn failed_class(options: &[(&str, &str)]) -> (ContextGuard, ClassNode, Vec<StructMethod>) {
    let mut all = vec![(IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, "0")];
    all.extend_from_slice(options);
    let guard = common::install_context_with(&all);
    let sc = guard.struct_context();
    sc.add_space_file(&data_dir().join("classes"), true);

    let cl = sc.get_class("w/Dump").expect("fixture class w/Dump");
    let node = ClassNode::new(ClassNodeType::Root, cl);
    node.with_mut(|d| {
        d.simple_name = Some("Dump".to_string());
        d.access = cl.get_access_flags();
    });
    let wrapper = ClassWrapper::new(node);
    node.with_mut(|d| d.wrapper = Some(wrapper));
    DecompilerContext::set_current_class_node(Some(node));

    let boom = try_catch(|| throw("RuntimeException", "boom")).unwrap_err();
    let methods = cl.get_methods().to_vec();
    for &mt in &methods {
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
    (guard, node, methods)
}

fn golden(kind: &str) -> Vec<String> {
    std::fs::read_to_string(data_dir().join("golden.txt"))
        .unwrap()
        .lines()
        .filter(|l| l.starts_with(&format!("{kind}\t")))
        .map(str::to_string)
        .collect()
}

#[test]
fn valid_java_identifiers_match_vineflower() {
    let mut out: Vec<String> = IDENTIFIERS
        .iter()
        .map(|id| {
            format!(
                "ident\t{}\t{}",
                esc(Some(id)),
                esc(ClassWriter::to_valid_java_identifier(Some(id)).as_deref())
            )
        })
        .collect();
    out.push(format!(
        "ident\t\\N\t{}",
        esc(ClassWriter::to_valid_java_identifier(None).as_deref())
    ));
    assert_eq!(out, golden("ident"));
}

#[test]
fn failed_methods_dump_their_bytecode_like_vineflower() {
    let (_guard, node, methods) = failed_class(&[
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
        (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1"),
    ]);
    let writer = ClassWriter::new();
    let mut out = Vec::new();
    for (i, &mt) in methods.iter().enumerate() {
        let key = format!("{}{}", mt.get_name(), mt.get_descriptor());
        if key != "<clinit>()V" && key != "<init>()V" {
            continue;
        }
        let mut buffer = TextBuffer::new();
        let shown = writer.write_method(node, mt, i as i32, &mut buffer, 1);
        out.push(format!(
            "method\t{key}\t{shown}\t{}",
            esc(Some(&buffer.convert_to_string_and_allow_data_discard()))
        ));
    }
    let expected = golden("method");
    assert_eq!(out.len(), expected.len());
    for (got, want) in out.iter().zip(&expected) {
        assert_eq!(got, want);
    }
    assert_eq!(DecompilerContext::get_current_method_wrapper(), None);
    assert_eq!(DecompilerContext::get_current_class_node(), Some(node));
}

#[test]
#[ignore = "needs ExprProcessor.getTypeName (another stage-2 slice)"]
fn typed_method_headers_match_vineflower() {
    let (_guard, node, methods) = failed_class(&[
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "0"),
        (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "0"),
    ]);
    let writer = ClassWriter::new();
    let mut out = Vec::new();
    for use_names in ["1", "0"] {
        DecompilerContext::set_property(IFernflowerPreferences::USE_METHOD_PARAMETERS, use_names);
        for (i, &mt) in methods.iter().enumerate() {
            let name = mt.get_name();
            if name.starts_with('<') || name.starts_with("lambda$") {
                continue;
            }
            let mut buffer = TextBuffer::new();
            let shown = writer.write_method(node, mt, i as i32, &mut buffer, 1);
            out.push(format!(
                "typed\t{use_names}\t{name}{}\t{shown}\t{}",
                mt.get_descriptor(),
                esc(Some(&buffer.convert_to_string_and_allow_data_discard()))
            ));
        }
    }
    assert_eq!(out, golden("typed"));
}

#[test]
fn exception_and_bytecode_dumps_are_separated_by_an_empty_comment() {
    let (_guard, node, methods) = failed_class(&[
        (IFernflowerPreferences::DUMP_EXCEPTION_ON_ERROR, "1"),
        (IFernflowerPreferences::DUMP_BYTECODE_ON_ERROR, "1"),
    ]);
    let index = methods
        .iter()
        .position(|mt| mt.get_name() == "<init>")
        .unwrap();
    let mut buffer = TextBuffer::new();
    assert!(ClassWriter::new().write_method(node, methods[index], index as i32, &mut buffer, 0));
    let text = buffer.convert_to_string_and_allow_data_discard();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "public Dump() {");
    assert_eq!(lines[1], "   // $VF: Couldn't be decompiled");
    assert!(lines[2].starts_with("   // Please report this to the Vineflower issue tracker"));
    assert_eq!(lines[3], "   // RuntimeException: boom");
    assert!(lines[4].starts_with("   //   at "), "{text}");
    assert_eq!(lines[5], "   //");
    assert_eq!(lines[6], "   // Bytecode:");
    assert_eq!(lines[7], "   // 0: aload 0");
    assert_eq!(*lines.last().unwrap(), "}");
}
