//! The context-dependent text paths (TextBuffer tracers and token dumps, type
//! and member tokens, TextUtil identifiers/indent/qualified super, GenericType
//! cast names) against Vineflower 1.12.0 run in the same kind of context
//! (`data/GenTracersGolden.java`, `data/GenTextContextGolden.java`).
//!
//! A row whose Rust side reaches a body other slices have not ported yet
//! (`UnportedError`) is counted as blocked, not failed.

use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::bytecode_version::BytecodeVersion;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::generic_type::GenericType;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::import_collector::ImportCollector;
use kuna_jvm::rt::Shared;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::text_token_dump_visitor::TextTokenDumpVisitor;
use kuna_jvm::text_token_visitor::TextTokenVisitor;
use kuna_jvm::text_util::TextUtil;
use kuna_jvm::var_type::VarType;

use crate::common;
use crate::f2_types::{esc, run, unesc};

fn dump(buf: TextBuffer) -> String {
    let shared = Shared::new(buf);
    let target = shared.clone();
    let mut visitor = <dyn TextTokenVisitor>::create_visitor_with(Rc::new(
        move |next: Box<dyn TextTokenVisitor>| {
            Box::new(TextTokenDumpVisitor::new(next, target.clone())) as Box<dyn TextTokenVisitor>
        },
    ));
    TextBuffer::visit_tokens_shared(&shared, Some(visitor.as_mut()));
    let text = shared.borrow().convert_to_string_and_allow_data_discard();
    text
}

#[test]
fn tracers_and_token_dump_match_vineflower() {
    let _guard = common::install_context_with(&[
        (IFernflowerPreferences::NEW_LINE_SEPARATOR, "1"),
        (IFernflowerPreferences::INDENT_STRING, "   "),
        (IFernflowerPreferences::PREFERRED_LINE_LENGTH, "160"),
    ]);
    let mut cls = TextBuffer::new();
    for m in 0..3 {
        let mut code = TextBuffer::new();
        for i in 0..4 {
            code.append_indent(1);
            code.add_bytecode_mapping_offset(i * 3 + m);
            code.push_newline_group(1, 1);
            code.append(&format!("call{i}("));
            for k in 0..(2 + m * 6) {
                if k > 0 {
                    code.append(",");
                    code.append_possible_newline_alt(" ");
                } else {
                    code.append_possible_newline();
                }
                code.append(&format!("argumentNumber{k}"));
                code.add_bytecode_mapping_offset(100 + i * 10 + k);
            }
            code.append(");");
            code.pop_newline_group();
            code.append_line_separator();
            if i == 1 {
                code.add_start_bytecode_mapping_offset(77);
            }
        }
        cls.append_class("Foo", true, "pkg/Foo")
            .append(&format!(" m{m}() {{"))
            .append_line_separator();
        cls.append_buffer_mapped(&code, Some("pkg/Foo"), Some(&format!("m{m} ()V")));
        cls.append("}").append_line_separator();
    }
    cls.reformat();
    let mut out = String::new();
    for (k, t) in cls.get_tracers().iter() {
        let mapping: BTreeMap<i32, i32> = t
            .with(|d| d.mapping.clone())
            .borrow()
            .entry_vec()
            .into_iter()
            .collect();
        let body: Vec<String> = mapping.iter().map(|(o, l)| format!("{o}={l}")).collect();
        out.push_str(&format!(
            "{}|{}|{{{}}}\n",
            k.a,
            k.b.as_deref().unwrap_or("null"),
            body.join(", ")
        ));
    }
    out.push_str(&dump(cls));
    assert_eq!(out, include_str!("data/tracers_golden.txt"));
}

struct Rows {
    checked: usize,
    blocked: usize,
    failures: Vec<String>,
}

impl Rows {
    fn check(&mut self, what: String, java: &str, rust: String) {
        if rust.starts_with("EXC:UnportedError:") {
            self.blocked += 1;
            return;
        }
        self.checked += 1;
        if java != rust {
            self.failures
                .push(format!("{what}\n   java: {java}\n   rust: {rust}"));
        }
    }
}

fn fresh(node: ClassNode) {
    DecompilerContext::start_class(ImportCollector::new(node));
}

#[test]
fn context_text_paths_match_vineflower() {
    let _guard = common::install_default_context();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/f6/tree/pkg/Hello.class");
    let bytes = std::fs::read(path).expect("Hello.class");
    let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
    let node = ClassNode::new(ClassNodeType::Root, cl);
    DecompilerContext::set_current_class_node(Some(node));
    DecompilerContext::set_current_class(Some(cl));
    let main = cl
        .get_method("main", "([Ljava/lang/String;)V")
        .expect("main");
    let init = cl.get_method("<init>", "()V").expect("<init>");

    let mut rows = Rows {
        checked: 0,
        blocked: 0,
        failures: Vec::new(),
    };
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for line in include_str!("data/text_context_golden.txt").lines() {
        let l: Vec<&str> = line.split('\t').collect();
        *kinds.entry(l[0]).or_default() += 1;
        let arg = |i: usize| unesc(l[i]).expect("non-null argument");
        let what = line.chars().take(120).collect::<String>();
        match l[0] {
            "IS" => {
                DecompilerContext::set_property(IFernflowerPreferences::INDENT_STRING, arg(1));
                let n: i32 = l[2].parse().unwrap();
                rows.check(
                    what,
                    l[3],
                    run(|| esc(Some(&TextUtil::get_indent_string(n)))),
                );
                DecompilerContext::set_property(IFernflowerPreferences::INDENT_STRING, "   ");
            }
            "KW" => {
                let id = arg(1);
                let version = BytecodeVersion::new(l[2].parse().unwrap(), 0);
                let mt = if l[3] == "1" { main } else { init };
                rows.check(
                    what.clone(),
                    l[4],
                    run(|| TextUtil::is_keyword(&id, &version, mt).to_string()),
                );
                rows.check(
                    what,
                    l[5],
                    run(|| TextUtil::is_valid_identifier(&id, &version, mt).to_string()),
                );
            }
            "QS" => {
                fresh(node);
                let q = arg(1);
                rows.check(
                    what,
                    l[2],
                    run(|| {
                        let mut buf = TextBuffer::new();
                        TextUtil::write_qualified_super(&mut buf, &q);
                        esc(Some(&dump(buf)))
                    }),
                );
            }
            "CN" => {
                fresh(node);
                let t = GenericType::parse(&arg(1));
                rows.check(
                    what,
                    l[2],
                    run(|| esc(Some(&t.as_generic().get_cast_name()))),
                );
            }
            "TS" => {
                fresh(node);
                let t = GenericType::parse(&arg(1));
                rows.check(what, l[2], run(|| esc(Some(&t.to_string()))));
            }
            "CT" | "TN" => {
                fresh(node);
                let t: VarType = GenericType::parse(&arg(1));
                let name = arg(2);
                let cast = l[0] == "CT";
                rows.check(
                    what,
                    l[3],
                    run(|| {
                        let mut buf = TextBuffer::new();
                        if cast {
                            buf.append_cast_type_name_str(&name, &t);
                        } else {
                            buf.append_type_name_str(&name, &t);
                        }
                        esc(Some(&dump(buf)))
                    }),
                );
            }
            "CT0" | "TN0" | "TT" => {
                fresh(node);
                let t: VarType = GenericType::parse(&arg(1));
                let kind = l[0];
                rows.check(
                    what,
                    l[2],
                    run(|| {
                        let mut buf = TextBuffer::new();
                        match kind {
                            "CT0" => drop(buf.append_cast_type_name(&t)),
                            "TN0" => drop(buf.append_type_name(&t)),
                            _ => {
                                let name =
                                    kuna_jvm::expr_processor::ExprProcessor::get_type_name(&t);
                                buf.append("= ").append(&name).add_type_name_token(&t, 2);
                            }
                        }
                        esc(Some(&dump(buf)))
                    }),
                );
            }
            "TK" => {
                fresh(node);
                let (kind, value, decl) = (arg(1), arg(2), l[3] == "1");
                let (class_name, name, desc) = (arg(4), arg(5), arg(6));
                let (param, var) = (l[7] == "1", arg(9));
                let index: i32 = l[8].parse().unwrap_or(0);
                rows.check(
                    what,
                    l[10],
                    run(|| {
                        let mut buf = TextBuffer::new();
                        buf.append("int a;")
                            .append_line_separator()
                            .append_indent(1)
                            .append("x = ");
                        match kind.as_str() {
                            "F" => drop(buf.append_field(&value, decl, &class_name, &name, &desc)),
                            "M" => drop(buf.append_method(&value, decl, &class_name, &name, &desc)),
                            _ => drop(buf.append_variable(
                                &value,
                                decl,
                                param,
                                &class_name,
                                &name,
                                &desc,
                                index,
                                &var,
                            )),
                        }
                        buf.append(";").append_line_separator();
                        esc(Some(&dump(buf)))
                    }),
                );
            }
            other => panic!("unknown row kind {other}"),
        }
    }
    eprintln!(
        "context text paths: {} rows {:?}, {} values checked, {} blocked on unported bodies",
        kinds.values().sum::<usize>(),
        kinds,
        rows.checked,
        rows.blocked
    );
    assert!(
        rows.failures.is_empty(),
        "{} mismatches:\n{}",
        rows.failures.len(),
        rows.failures
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(rows.checked > 900);
}
