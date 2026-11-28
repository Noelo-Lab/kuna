//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IfNode.java`.
//!
//! A plain owned tree: nodes are built once by [`IfNode::build`], never shared between
//! parents and never mutated afterwards, so children are boxed rather than arena handles.

use std::fmt;

use crate::if_statement::IfStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::validation_helper::ValidationHelper;

pub struct IfNode {
    pub value: Statement,
    pub inner_node: Option<Box<IfNode>>,
    pub inner_type: Option<EdgeType>,
    pub successor_node: Option<Box<IfNode>>,
    pub successor_type: Option<EdgeType>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum EdgeType {
    Direct,
    Indirect,
    Else,
}

impl EdgeType {
    pub fn name(self) -> &'static str {
        match self {
            EdgeType::Direct => "DIRECT",
            EdgeType::Indirect => "INDIRECT",
            EdgeType::Else => "ELSE",
        }
    }
}

impl crate::rt::JavaHash for EdgeType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl crate::rt::JavaEq for EdgeType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl fmt::Display for EdgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl IfNode {
    fn new(value: Statement) -> IfNode {
        ValidationHelper::not_null(Some(&value));
        IfNode {
            value,
            inner_node: None,
            inner_type: None,
            successor_node: None,
            successor_type: None,
        }
    }

    fn set_inner(&mut self, if_node: IfNode, indirect: EdgeType) {
        ValidationHelper::not_null(Some(&if_node));
        ValidationHelper::not_null(Some(&indirect));
        self.inner_node = Some(Box::new(if_node));
        self.inner_type = Some(indirect);
    }

    fn set_successor(&mut self, if_node: IfNode, indirect: EdgeType) {
        ValidationHelper::not_null(Some(&if_node));
        ValidationHelper::not_null(Some(&indirect));
        self.successor_node = Some(Box::new(if_node));
        self.successor_type = Some(indirect);
    }

    pub fn build(stat: IfStatement, stsingle: bool) -> IfNode {
        let mut res = IfNode::new(stat.0);

        match stat.get_ifstat() {
            None => res.set_inner(
                IfNode::new(stat.get_if_edge().unwrap().get_destination()),
                EdgeType::Indirect,
            ),
            Some(ifstat) => res.set_inner(IfNode::build_sub_if_node(ifstat), EdgeType::Direct),
        }

        if stat.d(|d| d.iftype) == IfStatement::IFTYPE_IFELSE {
            res.set_successor(
                IfNode::build_sub_if_node(stat.get_elsestat().unwrap()),
                EdgeType::Else,
            );
        } else {
            let edge = stat.get_first_successor();
            if stsingle || edge.get_type() != StatEdge::TYPE_REGULAR {
                res.set_successor(IfNode::new(edge.get_destination()), EdgeType::Indirect);
            } else {
                res.set_successor(
                    IfNode::build_sub_if_node(edge.get_destination()),
                    EdgeType::Direct,
                );
            }
        }

        res
    }

    fn build_sub_if_node(statement: Statement) -> IfNode {
        let mut ifnode = IfNode::new(statement);

        if let Some(if_statement) = statement.try_as_if() {
            if if_statement.d(|d| d.iftype) == IfStatement::IFTYPE_IF {
                let inner_type = if if_statement.get_ifstat().is_none() {
                    EdgeType::Indirect
                } else {
                    EdgeType::Direct
                };
                ifnode.set_inner(
                    IfNode::new(if_statement.get_if_edge().unwrap().get_destination()),
                    inner_type,
                );
            }
        }

        if statement.has_any_successor() {
            ifnode.set_successor(
                IfNode::new(statement.get_first_successor().get_destination()),
                EdgeType::Indirect,
            );
        }

        ifnode
    }
}

impl fmt::Display for IfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.with(|d| d.id))?;
        if let Some(inner) = &self.inner_node {
            write!(f, ": inner({}) {{{}}}", fmt_edge(self.inner_type), inner)?;
        }
        if let Some(successor) = &self.successor_node {
            if self.inner_node.is_none() {
                f.write_str(": ")?;
            }
            write!(
                f,
                " successor({}) {{{}}}",
                fmt_edge(self.successor_type),
                successor
            )?;
        }
        Ok(())
    }
}

fn fmt_edge(edge: Option<EdgeType>) -> &'static str {
    edge.map_or("null", EdgeType::name)
}
