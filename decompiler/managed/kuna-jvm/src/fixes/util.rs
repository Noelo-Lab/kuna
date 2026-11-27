//! Helpers several fix families share.

use crate::classes_processor::ClassNode;
use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::statement::Statement;

/// Java's binary numeric promotion of two written numeric types.
pub fn promoted(a: CodeType, b: CodeType) -> CodeType {
    for wide in [CodeType::Double, CodeType::Float, CodeType::Long] {
        if a == wide || b == wide {
            return wide;
        }
    }
    CodeType::Int
}

/// Every exprent of `stat`'s tree, each after the exprents it holds; a statement's
/// children come before its head exprents, and variable definitions are left out.
pub fn all_exprents(stat: Statement) -> Vec<Exprent> {
    fn collect(stat: Statement, out: &mut Vec<Exprent>) {
        let top: Vec<Exprent> = match stat.get_exprents() {
            Some(exprents) => exprents.iter().collect(),
            None => {
                for st in stat.get_stats().borrow().iter() {
                    collect(*st, out);
                }
                stat.get_stat_exprents()
            }
        };
        for e in top {
            out.extend(e.get_all_exprents_rec(true));
            out.push(e);
        }
    }
    let mut out = Vec::new();
    collect(stat, &mut out);
    out
}

/// The top-level class `node` is nested in (itself when it is one).
pub fn top_level(node: ClassNode) -> ClassNode {
    let mut node = node;
    while let Some(parent) = node.with(|d| d.parent) {
        node = parent;
    }
    node
}
