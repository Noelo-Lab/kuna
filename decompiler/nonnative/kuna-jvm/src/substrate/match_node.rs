//! Port of `org/jetbrains/java/decompiler/struct/match/MatchNode.java`.
//!
//! Match nodes belong to (static) `MatchEngine`s that outlive every arena, so
//! they are `Rc`-shared rather than arena objects.

use std::cell::RefCell;
use std::rc::Rc;

use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::i_matchable::MatchProperties;
use crate::primitive_constant::ConstValue;
use crate::rt::{JHashMap, JavaEq, JavaStr};
use crate::statement::{Statement, StatementType};
use crate::var_type::VarType;

/// A Java `Object` held by a match rule or bound to a match variable.
#[derive(Clone)]
pub enum MatchValue {
    String(String),
    Integer(i32),
    StatementType(StatementType),
    ExprentType(ExprentType),
    FunctionType(FunctionType),
    ExitType(ExitType),
    VarType(VarType),
    Statement(Statement),
    Exprent(Exprent),
    Constant(ConstValue),
}

/// `TryCatch` -> `TRY_CATCH`: the Java `name()` of a variant spelled as the port spells it.
fn java_enum_name(variant: &str) -> String {
    let mut out = String::new();
    for (i, c) in variant.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_uppercase());
    }
    out
}

impl MatchValue {
    /// `Object.equals`.
    pub fn equals(&self, other: &MatchValue) -> bool {
        match (self, other) {
            (MatchValue::String(a), MatchValue::String(b)) => a == b,
            (MatchValue::Integer(a), MatchValue::Integer(b)) => a == b,
            (MatchValue::StatementType(a), MatchValue::StatementType(b)) => a == b,
            (MatchValue::ExprentType(a), MatchValue::ExprentType(b)) => a == b,
            (MatchValue::FunctionType(a), MatchValue::FunctionType(b)) => a == b,
            (MatchValue::ExitType(a), MatchValue::ExitType(b)) => a == b,
            (MatchValue::VarType(a), MatchValue::VarType(b)) => a.equals(b),
            (MatchValue::Statement(a), MatchValue::Statement(b)) => a == b,
            (MatchValue::Exprent(a), MatchValue::Exprent(b)) => a.java_equals(b),
            (MatchValue::Constant(a), MatchValue::Constant(b)) => a.equals(b),
            _ => false,
        }
    }

    /// `Object.toString` (statements and expressions print their debug form).
    pub fn to_java_string(&self) -> String {
        match self {
            MatchValue::String(s) => s.clone(),
            MatchValue::Integer(v) => v.to_string(),
            MatchValue::StatementType(t) => java_enum_name(&format!("{:?}", t)),
            MatchValue::ExprentType(t) => java_enum_name(&format!("{:?}", t)),
            MatchValue::FunctionType(t) => java_enum_name(&format!("{:?}", t)),
            MatchValue::ExitType(t) => java_enum_name(&format!("{:?}", t)),
            MatchValue::VarType(t) => t.to_string(),
            MatchValue::Statement(s) => format!("{:?}", s),
            MatchValue::Exprent(e) => format!("{:?}", e),
            MatchValue::Constant(c) => c.to_java_string(),
        }
    }

    /// `(String) value`.
    #[track_caller]
    pub fn as_string(&self) -> String {
        match self {
            MatchValue::String(s) => s.clone(),
            _ => crate::rt::throw("ClassCastException", "match value is not a String"),
        }
    }

    /// `(Integer) value`.
    #[track_caller]
    pub fn as_integer(&self) -> i32 {
        match self {
            MatchValue::Integer(v) => *v,
            _ => crate::rt::throw("ClassCastException", "match value is not an Integer"),
        }
    }
}

impl std::fmt::Debug for MatchValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_java_string())
    }
}

#[derive(Clone, Debug)]
pub struct RuleValue {
    pub parameter: i32,
    pub value: Option<MatchValue>,
}

impl RuleValue {
    pub fn new(parameter: i32, value: Option<MatchValue>) -> RuleValue {
        RuleValue { parameter, value }
    }

    #[track_caller]
    fn value_text(&self) -> String {
        match &self.value {
            Some(v) => v.to_java_string(),
            None => crate::rt::throw("NullPointerException", "rule value is null"),
        }
    }

    pub fn is_variable(&self) -> bool {
        let str_value = self.value_text();
        str_value.jchar_at(0) == b'$' as u16
            && str_value.jchar_at(str_value.jlen() - 1) == b'$' as u16
    }
}

impl std::fmt::Display for RuleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value_text())
    }
}

#[derive(Clone)]
pub struct MatchNode(Rc<MatchNodeData>);

struct MatchNodeData {
    type_: i32,
    rules: RefCell<JHashMap<MatchProperties, Vec<RuleValue>>>,
    children: RefCell<Vec<MatchNode>>,
}

impl MatchNode {
    pub const MATCHNODE_STATEMENT: i32 = 0;
    pub const MATCHNODE_EXPRENT: i32 = 1;

    pub fn new(type_: i32) -> MatchNode {
        MatchNode(Rc::new(MatchNodeData {
            type_,
            rules: RefCell::new(JHashMap::new()),
            children: RefCell::new(Vec::new()),
        }))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &MatchNode) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn add_child(&self, child: MatchNode) {
        self.0.children.borrow_mut().push(child);
    }

    pub fn add_rule(&self, property: MatchProperties, value: RuleValue) {
        self.0
            .rules
            .borrow_mut()
            .compute_if_absent(property, Vec::new)
            .push(value);
    }

    pub fn get_type(&self) -> i32 {
        self.0.type_
    }

    pub fn get_children(&self) -> Vec<MatchNode> {
        self.0.children.borrow().clone()
    }

    pub fn iterate_rules(
        &self,
        mut consumer: impl FnMut(MatchProperties, &RuleValue) -> bool,
    ) -> bool {
        let entries = self.0.rules.borrow().entry_vec();
        for (key, rules) in &entries {
            for rule in rules {
                if !consumer(*key, rule) {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_raw_rule(&self, property: MatchProperties) -> Option<RuleValue> {
        self.0
            .rules
            .borrow()
            .get(&property)
            .map(|list| list[0].clone())
    }

    pub fn get_rule_value(&self, property: MatchProperties) -> Option<MatchValue> {
        self.0
            .rules
            .borrow()
            .get(&property)
            .and_then(|rule| rule[0].value.clone())
    }
}

impl std::fmt::Debug for MatchNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatchNode")
            .field("type", &self.0.type_)
            .field("rules", &*self.0.rules.borrow())
            .field("children", &*self.0.children.borrow())
            .finish()
    }
}
