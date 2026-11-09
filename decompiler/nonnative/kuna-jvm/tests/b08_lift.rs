//! `ExprProcessor.processBlock` against Vineflower 1.12.0: every basic block of
//! the raw control flow graph of each fixture method is lifted on an empty
//! stack and the expressions it builds (ids, types, offsets, structure) are
//! compared with `data/b08/lift_golden.txt` (`data/b08/GenB08Oracle.java`).
//! A method whose lift reaches a body other slices have not ported yet
//! (`UnportedError`) is counted as blocked, not compared.

use std::collections::BTreeMap;

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::classes_processor::{ClassNode, ClassNodeType};
use kuna_jvm::const_exprent::ConstValue;
use kuna_jvm::control_flow_graph::ControlFlowGraph;
use kuna_jvm::counter_container::CounterContainer;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::expr_processor::ExprProcessor;
use kuna_jvm::exprent::{Exprent, ExprentType};
use kuna_jvm::import_collector::ImportCollector;
use kuna_jvm::primitive_exprs_list::PrimitiveExprsList;
use kuna_jvm::rt::{try_catch, BitSet};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::var_processor::VarProcessor;
use kuna_jvm::var_type::VarType;

use crate::common;

fn t(v: Option<VarType>) -> String {
    v.map_or_else(|| "null".to_string(), |v| v.to_string())
}

fn bits(b: &BitSet) -> String {
    let parts: Vec<String> = b.iter().map(|i| i.to_string()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn e(x: Option<Exprent>) -> String {
    let Some(x) = x else {
        return "null".to_string();
    };
    let mut s = format!(
        "{}#{}:{}",
        x.get_type().name(),
        x.get_id(),
        x.get_expr_type()
    );
    if let Some(b) = x.get_bytecode() {
        s.push_str(&bits(&b));
    }
    s.push('[');
    match x.get_type() {
        ExprentType::Var => {
            let v = x.as_var();
            s.push_str(&v.get_index().to_string());
            if v.is_stack() {
                s.push_str(" stack");
            }
            if let Some(lvt) = v.get_lvt() {
                s.push_str(&format!(" lvt={}", lvt.name));
            }
            if v.is_catch_temp_var() {
                s.push_str(" catch");
            }
        }
        ExprentType::Const => {
            let c = x.as_const();
            s.push_str(&match c.get_value() {
                None => "null".to_string(),
                Some(val) => {
                    let class = match val {
                        ConstValue::Int(_) => "Integer",
                        ConstValue::Long(_) => "Long",
                        ConstValue::Float(_) => "Float",
                        ConstValue::Double(_) => "Double",
                        ConstValue::String(_) => "String",
                    };
                    format!("{class}={}", val.java_to_string())
                }
            });
            if c.is_bool_permitted() {
                s.push_str(" bool");
            }
        }
        ExprentType::Assignment => {
            let a = x.as_assignment();
            s.push_str(&format!(
                "{} = {}",
                e(Some(a.get_left())),
                e(Some(a.get_right()))
            ));
        }
        ExprentType::Function => {
            let f = x.as_function();
            s.push_str(f.get_func_type().name());
            for op in f.get_lst_operands().to_vec() {
                s.push(' ');
                s.push_str(&e(Some(op)));
            }
        }
        ExprentType::Array => {
            let a = x.as_array();
            s.push_str(&format!(
                "{} {}",
                e(Some(a.get_array())),
                e(Some(a.get_index()))
            ));
        }
        ExprentType::Field => {
            let f = x.as_field();
            s.push_str(&format!(
                "{}.{} {}",
                f.get_classname(),
                f.get_name(),
                f.get_descriptor().descriptor_string
            ));
            if f.is_static() {
                s.push_str(" static");
            } else {
                s.push(' ');
                s.push_str(&e(f.get_instance()));
            }
        }
        ExprentType::If => s.push_str(&e(Some(x.as_if().get_condition()))),
        ExprentType::Exit => {
            let ex = x.as_exit();
            s.push_str(&format!(
                "{} {} {}",
                ex.get_exit_type().name(),
                e(ex.get_value()),
                t(ex.get_ret_type())
            ));
        }
        ExprentType::Monitor => {
            let m = x.as_monitor();
            s.push_str(&format!(
                "{} {}",
                m.get_mon_type().name(),
                e(Some(m.get_value()))
            ));
            if m.is_removable() {
                s.push_str(" remove");
            }
        }
        ExprentType::New => {
            let n = x.as_new();
            s.push_str(&n.get_new_type().to_string());
            for d in n.get_lst_dims().to_vec() {
                s.push(' ');
                s.push_str(&e(Some(d)));
            }
        }
        ExprentType::SwitchHead => s.push_str(&e(Some(x.as_switch_head().get_value()))),
        ExprentType::Invocation => {
            let i = x.as_invocation();
            s.push_str(&format!(
                "{}.{}{}",
                i.get_classname(),
                i.get_name(),
                i.get_string_descriptor()
            ));
            if i.is_static() {
                s.push_str(" static");
            } else {
                s.push(' ');
                s.push_str(&e(i.get_instance()));
            }
            for p in i.get_lst_parameters().to_vec() {
                s.push(' ');
                s.push_str(&e(Some(p)));
            }
        }
        _ => {}
    }
    s.push(']');
    s
}

/// The dump of one method, or `None` when the lift reached an `unported!` body.
fn lift_method(cl: StructClass, mt: StructMethod) -> Option<String> {
    let mut out = String::new();
    mt.expand_data(cl);
    let md = mt.method_descriptor();
    let var_proc = VarProcessor::new(mt, &md);
    DecompilerContext::start_method(var_proc);
    let graph = ControlFlowGraph::new(mt.get_instruction_sequence().expect("expanded method"));
    let proc = ExprProcessor::new(&md, var_proc);
    let blocks = graph.get_blocks().borrow().to_vec();
    for block in blocks {
        let stat = BasicBlockStatement::new(block);
        let mut data = PrimitiveExprsList::new();
        out.push_str(&format!("B {} stat {}\n", block.get_id(), stat.id()));
        if let Err(caught) = try_catch(|| proc.process_block(stat, &mut data, cl)) {
            if caught.is("UnportedError") {
                mt.release_resources();
                return None;
            }
            out.push_str("  EXC\n");
        }
        for x in data.get_lst_exprents().to_vec() {
            out.push_str(&format!("  {}\n", e(Some(x))));
        }
        for x in data.get_stack().to_vec() {
            out.push_str(&format!("  S {}\n", e(Some(x))));
        }
    }
    let counters = DecompilerContext::get_counter_container();
    out.push_str(&format!(
        "C expr={} stat={}\n",
        counters.get_counter(CounterContainer::EXPRESSION_COUNTER),
        counters.get_counter(CounterContainer::STATEMENT_COUNTER)
    ));
    mt.release_resources();
    Some(out)
}

fn golden_sections() -> BTreeMap<String, String> {
    let mut sections = BTreeMap::new();
    let mut key = String::new();
    for line in include_str!("data/b08/lift_golden.txt").lines() {
        if let Some(name) = line.strip_prefix("M ") {
            key = name.to_string();
            sections.insert(key.clone(), String::new());
        } else {
            let body = sections.get_mut(&key).expect("method header");
            body.push_str(line);
            body.push('\n');
        }
    }
    sections
}

#[test]
fn process_block_matches_vineflower() {
    let _guard = common::install_default_context();
    let golden = golden_sections();
    let dir = format!("{}/tests/data/b08/classes", env!("CARGO_MANIFEST_DIR"));

    let mut checked = 0;
    let mut blocked = Vec::new();
    let mut failures = Vec::new();
    let mut seen = 0;
    for class in ["B08Lift", "B08Stack"] {
        let bytes = std::fs::read(format!("{dir}/{class}.class")).expect("fixture class");
        let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
        DecompilerContext::set_current_class(Some(cl));
        let node = ClassNode::new(ClassNodeType::Root, cl);
        DecompilerContext::set_current_class_node(Some(node));
        DecompilerContext::start_class(ImportCollector::new(node));
        let mts: Vec<StructMethod> = cl.get_methods().iter().copied().collect();
        for mt in mts {
            if !mt.contains_code() {
                continue;
            }
            seen += 1;
            let name = format!(
                "{}.{}{}",
                cl.qualified_name(),
                mt.get_name(),
                mt.get_descriptor()
            );
            let want = golden
                .get(&name)
                .unwrap_or_else(|| panic!("no golden for {name}"));
            match lift_method(cl, mt) {
                None => blocked.push(name),
                Some(got) => {
                    checked += 1;
                    if &got != want {
                        let diff = got
                            .lines()
                            .zip(want.lines())
                            .find(|(g, w)| g != w)
                            .map(|(g, w)| format!("   rust: {g}\n   java: {w}"))
                            .unwrap_or_else(|| "   (length differs)".to_string());
                        failures.push(format!("{name}\n{diff}"));
                    }
                }
            }
        }
        DecompilerContext::set_current_class_node(None);
        DecompilerContext::set_current_class(None);
    }

    eprintln!(
        "processBlock: {seen} methods, {checked} compared, {} blocked on unported bodies: {blocked:?}",
        blocked.len()
    );
    assert_eq!(seen, golden.len());
    assert!(
        failures.is_empty(),
        "{} mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(checked >= 25, "only {checked} methods compared");
}
