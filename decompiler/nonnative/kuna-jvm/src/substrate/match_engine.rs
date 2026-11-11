//! Port of `org/jetbrains/java/decompiler/struct/match/MatchEngine.java`.

use std::cell::RefCell;

use crate::exit_exprent::Type as ExitType;
use crate::exprent::ExprentType;
use crate::function_exprent::FunctionType;
use crate::i_matchable::{IMatchable, MatchProperties};
use crate::if_statement::IfStatement;
use crate::match_node::{MatchNode, MatchValue, RuleValue};
use crate::rt::JHashMap;
use crate::statement::StatementType;
use crate::var_type::{java_split, JStr, VarType};

fn stat_properties(key: &str) -> Option<MatchProperties> {
    Some(match key {
        "type" => MatchProperties::StatementType,
        "ret" => MatchProperties::StatementRet,
        "position" => MatchProperties::StatementPosition,
        "statsize" => MatchProperties::StatementStatsize,
        "exprsize" => MatchProperties::StatementExprsize,
        "iftype" => MatchProperties::StatementIftype,
        _ => return None,
    })
}

fn expr_properties(key: &str) -> Option<MatchProperties> {
    Some(match key {
        "type" => MatchProperties::ExprentType,
        "ret" => MatchProperties::ExprentRet,
        "position" => MatchProperties::ExprentPosition,
        "functype" => MatchProperties::ExprentFunctype,
        "exittype" => MatchProperties::ExprentExittype,
        "consttype" => MatchProperties::ExprentConsttype,
        "constvalue" => MatchProperties::ExprentConstvalue,
        "invclass" => MatchProperties::ExprentInvocationClass,
        "signature" => MatchProperties::ExprentInvocationSignature,
        "parameter" => MatchProperties::ExprentParameter,
        "index" => MatchProperties::ExprentVarIndex,
        "name" => MatchProperties::ExprentName,
        _ => return None,
    })
}

fn stat_type(key: &str) -> Option<StatementType> {
    Some(match key {
        "if" => StatementType::If,
        "do" => StatementType::Do,
        "switch" => StatementType::Switch,
        "trycatch" => StatementType::TryCatch,
        "basicblock" => StatementType::BasicBlock,
        "sequence" => StatementType::Sequence,
        _ => return None,
    })
}

fn expr_type(key: &str) -> Option<ExprentType> {
    Some(match key {
        "annotation" => ExprentType::Annotation,
        "array" => ExprentType::Array,
        "assert" => ExprentType::Assert,
        "assignment" => ExprentType::Assignment,
        "constant" => ExprentType::Const,
        "exit" => ExprentType::Exit,
        "field" => ExprentType::Field,
        "function" => ExprentType::Function,
        "if" => ExprentType::If,
        "invocation" => ExprentType::Invocation,
        "monitor" => ExprentType::Monitor,
        "new" => ExprentType::New,
        "switch" => ExprentType::Switch,
        "switchhead" => ExprentType::SwitchHead,
        "var" => ExprentType::Var,
        "yield" => ExprentType::Yield,
        _ => return None,
    })
}

fn expr_func_type(key: &str) -> Option<FunctionType> {
    Some(match key {
        "eq" => FunctionType::Eq,
        "neq" => FunctionType::Ne,
        "ternary" => FunctionType::Ternary,
        _ => return None,
    })
}

fn expr_exit_type(key: &str) -> Option<ExitType> {
    Some(match key {
        "return" => ExitType::Return,
        "throw" => ExitType::Throw,
        _ => return None,
    })
}

fn stat_if_type(key: &str) -> Option<i32> {
    Some(match key {
        "if" => IfStatement::IFTYPE_IF,
        "ifelse" => IfStatement::IFTYPE_IFELSE,
        _ => return None,
    })
}

fn expr_const_type(key: &str) -> Option<VarType> {
    Some(match key {
        "null" => VarType::VARTYPE_NULL(),
        "string" => VarType::VARTYPE_STRING(),
        _ => return None,
    })
}

/// `Integer.parseInt`.
#[track_caller]
fn parse_int(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(v) => v,
        Err(_) => crate::jthrow!(NumberFormatException; "For input string: \"{}\"", s),
    }
}

pub struct MatchEngine {
    root_node: MatchNode,
    variables: RefCell<JHashMap<String, Option<MatchValue>>>,
}

impl MatchEngine {
    /// Java: `MatchEngine(String...)`
    pub fn new(description: &[&str]) -> MatchEngine {
        MatchEngine::new_description(&description.join("\n"))
    }

    /// Java: `MatchEngine(String)`
    pub fn new_description(description: &str) -> MatchEngine {
        let lines = java_split(description, |c| c == '\n', false);

        let mut depth: i32 = 0;
        let mut stack: Vec<MatchNode> = Vec::new();

        for line in lines {
            let mut properties: Vec<&str> = java_split(
                line,
                |c| matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r'),
                true,
            );
            if properties[0].is_empty() {
                properties.remove(0);
            }

            let node_type = if properties[0] == "statement" {
                MatchNode::MATCHNODE_STATEMENT
            } else {
                MatchNode::MATCHNODE_EXPRENT
            };

            let match_node = MatchNode::new(node_type);
            for prop in properties.iter().skip(1) {
                let values = java_split(prop, |c| c == ':', false);

                let property = if node_type == MatchNode::MATCHNODE_STATEMENT {
                    stat_properties(values[0])
                } else {
                    expr_properties(values[0])
                };
                let Some(property) = property else {
                    crate::jthrow!("Unknown matching property");
                };

                let mut parameter = 0;
                let mut str_value = values[1];
                if values.len() == 3 {
                    parameter = parse_int(values[1]);
                    str_value = values[2];
                }

                let value: Option<MatchValue> = match property {
                    MatchProperties::StatementType => {
                        stat_type(str_value).map(MatchValue::StatementType)
                    }
                    MatchProperties::StatementStatsize | MatchProperties::StatementExprsize => {
                        Some(MatchValue::Integer(parse_int(str_value)))
                    }
                    MatchProperties::StatementPosition
                    | MatchProperties::ExprentPosition
                    | MatchProperties::ExprentInvocationClass
                    | MatchProperties::ExprentInvocationSignature
                    | MatchProperties::ExprentParameter
                    | MatchProperties::ExprentVarIndex
                    | MatchProperties::ExprentName
                    | MatchProperties::ExprentConstvalue
                    | MatchProperties::StatementRet
                    | MatchProperties::ExprentRet => {
                        Some(MatchValue::String(str_value.to_string()))
                    }
                    MatchProperties::StatementIftype => {
                        stat_if_type(str_value).map(MatchValue::Integer)
                    }
                    MatchProperties::ExprentFunctype => {
                        expr_func_type(str_value).map(MatchValue::FunctionType)
                    }
                    MatchProperties::ExprentExittype => {
                        expr_exit_type(str_value).map(MatchValue::ExitType)
                    }
                    MatchProperties::ExprentConsttype => {
                        expr_const_type(str_value).map(MatchValue::VarType)
                    }
                    MatchProperties::ExprentType => {
                        expr_type(str_value).map(MatchValue::ExprentType)
                    }
                };

                match_node.add_rule(property, RuleValue::new(parameter, value));
            }

            if stack.is_empty() {
                stack.push(match_node);
            } else {
                let new_depth = JStr::new(line).last_index_of_from(b' ' as u16, depth) + 1;
                for _ in new_depth..=depth {
                    if stack.pop().is_none() {
                        crate::jthrow!(NoSuchElementException; "");
                    }
                }

                match stack.last() {
                    Some(top) => top.add_child(match_node.clone()),
                    None => crate::jthrow!(NoSuchElementException; ""),
                }
                stack.push(match_node);

                depth = new_depth;
            }
        }

        MatchEngine {
            root_node: stack
                .first()
                .cloned()
                .unwrap_or_else(|| crate::jthrow!(NoSuchElementException; "")),
            variables: RefCell::new(JHashMap::new()),
        }
    }

    pub fn get_root_node(&self) -> MatchNode {
        self.root_node.clone()
    }

    /// Java: `match(IMatchable)`
    pub fn match_(&self, object: &dyn IMatchable) -> bool {
        self.variables.borrow_mut().clear();
        self.match_node(&self.root_node, object)
    }

    /// Java: `match(MatchNode, IMatchable)`
    fn match_node(&self, match_node: &MatchNode, object: &dyn IMatchable) -> bool {
        if !object.match_(match_node, self) {
            return false;
        }

        let mut expr_index = 0;
        let mut stat_index = 0;
        for child_node in match_node.get_children() {
            let is_statement = child_node.get_type() == MatchNode::MATCHNODE_STATEMENT;

            let child_object = object.find_object(
                &child_node,
                if is_statement { stat_index } else { expr_index },
            );
            match child_object {
                Some(child) if self.match_node(&child_node, child.as_ref()) => {}
                _ => return false,
            }

            if is_statement {
                stat_index += 1;
            } else {
                expr_index += 1;
            }
        }

        true
    }

    pub fn check_and_set_variable_value(&self, name: &str, value: Option<MatchValue>) -> bool {
        let old_value = self
            .variables
            .borrow()
            .get(&name.to_string())
            .cloned()
            .flatten();
        match old_value {
            Some(old) => value.is_some_and(|v| old.equals(&v)),
            None => {
                self.variables.borrow_mut().put(name.to_string(), value);
                true
            }
        }
    }

    pub fn get_variable_value(&self, name: &str) -> Option<MatchValue> {
        self.variables
            .borrow()
            .get(&name.to_string())
            .cloned()
            .flatten()
    }
}
