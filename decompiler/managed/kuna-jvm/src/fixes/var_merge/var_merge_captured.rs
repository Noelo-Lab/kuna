//! Fix `var_merge_captured`: a variable a lambda captures must be
//! effectively final (JLS 4.12.4), so merging it with another variable of
//! its slot can give it a second assignment javac rejects: two blocks that
//! each declare an `o` and pass `() -> o.hashCode()` were printed as one
//! `Object o = ..; .. o = ..;`. A variable that is a captured argument of a
//! lambda in the method is merged only when no assignment of one of the two
//! can be followed by an assignment of the other: they are then one
//! variable of the source, assigned on paths that exclude each other (`T
//! v; if (c) { v = a; use(v); return; } v = b; run(() -> v.m());`).

use crate::exprent::Exprent;
use crate::fixes::util::all_exprents;
use crate::statement::Statement;
use crate::var_version_pair::VarVersionPair;

/// Whether `from` or `to` is a captured argument of a lambda created in
/// `root`'s tree and an assignment of one can be followed by an assignment
/// of the other, or one is assigned outside a block's statements.
pub fn captured_apart(root: Statement, from: VarVersionPair, to: VarVersionPair) -> bool {
    let captured = all_exprents(root).iter().any(|e| {
        e.try_as_new().filter(|n| n.is_lambda() && !n.is_method_reference()).and_then(|n| n.get_constructor()).is_some_and(
            |constructor| {
                constructor.get_lst_parameters().iter().any(|p| {
                    p.try_as_var().map(VarVersionPair::from_var).is_some_and(|pair| pair.equals(&from) || pair.equals(&to))
                })
            },
        )
    });
    if !captured {
        return false;
    }
    let mut sites = Sites { from, to, from_blocks: Vec::new(), to_blocks: Vec::new(), outside: false };
    sites.collect(root);
    if sites.outside || sites.from_blocks.is_empty() || sites.to_blocks.is_empty() {
        return true;
    }
    sites.from_blocks.iter().any(|a| sites.to_blocks.iter().any(|b| a == b || follows(*a, *b) || follows(*b, *a)))
}

/// The basic blocks that assign two variables.
struct Sites {
    from: VarVersionPair,
    to: VarVersionPair,
    from_blocks: Vec<Statement>,
    to_blocks: Vec<Statement>,
    outside: bool,
}

impl Sites {
    fn collect(&mut self, stat: Statement) {
        match stat.get_exprents() {
            Some(exprents) => {
                for e in exprents.iter() {
                    let (from, to) = self.assigned(e);
                    if from && !self.from_blocks.contains(&stat) {
                        self.from_blocks.push(stat);
                    }
                    if to && !self.to_blocks.contains(&stat) {
                        self.to_blocks.push(stat);
                    }
                }
            }
            None => {
                for e in stat.get_stat_exprents() {
                    let (from, to) = self.assigned(e);
                    self.outside |= from || to;
                }
                for st in stat.get_stats().borrow().iter() {
                    self.collect(*st);
                }
            }
        }
    }

    fn assigned(&self, e: Exprent) -> (bool, bool) {
        let mut all = e.get_all_exprents_rec(true);
        all.push(e);
        let mut found = (false, false);
        for target in all.iter().filter_map(|x| x.try_as_assignment()).filter_map(|a| a.get_left().try_as_var()) {
            let pair = VarVersionPair::from_var(target);
            found.0 |= pair.equals(&self.from);
            found.1 |= pair.equals(&self.to);
        }
        found
    }
}

/// Whether control can pass from the block `a` to the block `b` by the
/// statements' successor edges, a statement entered at its first.
fn follows(a: Statement, b: Statement) -> bool {
    let mut seen: Vec<Statement> = Vec::new();
    let mut queue: Vec<Statement> = a.get_all_successor_edges().iter().map(|e| e.get_destination()).collect();
    while let Some(stat) = queue.pop() {
        if seen.contains(&stat) {
            continue;
        }
        if stat == b {
            return true;
        }
        seen.push(stat);
        queue.extend(stat.get_first());
        queue.extend(stat.get_all_successor_edges().iter().map(|e| e.get_destination()));
    }
    false
}
