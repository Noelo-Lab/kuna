//! The kuna option `jvm_source_locals` (`kuna_source_locals`), its declaration
//! scopes: Vineflower declares a variable in the innermost statement that holds
//! all its uses (`VarDefinitionHelper.setVarDefinitions`), so a local the
//! source declared before a loop, an if, a `try` or a `synchronized` block and
//! used only inside it is declared inside, where javac would give it another
//! slot; and it splits a local into two variables where a value stored in it
//! does not depend on the one before. [`hoist`] moves such a declaration out
//! to where the class file's slot numbers prove the source had it, and merges
//! the split variables back.
//!
//! javac gives a local the next free slot where it is declared and frees a
//! block's slots when the block ends; two locals in scope at once therefore
//! have slots in declaration order, and a `catch` parameter takes the first
//! slot after the locals in scope at its `try`. Three proofs follow, read from
//! the LocalVariableTable:
//! - a variable declared in a block inside the scope of a variable `w`
//!   declared outside that block (a `for` initializer's too), whose slot is
//!   lower than `w`'s and one of whose table entries overlaps one of `w`'s
//!   (both in scope at once), was declared before `w`: its declaration moves
//!   before `w`'s, in `w`'s block;
//! - a slot below a `catch` parameter's that no variable in scope at the `try`
//!   holds belongs to a local in scope there; when the table names one local
//!   declared inside the `try` body at that slot inside the body, that local
//!   is the one, and its declaration moves before the `try`;
//! - a variable whose slot, name and table entry are those of a variable in
//!   scope (or a parameter) is that local: it is merged into it.
//!
//! A declaration with an initializer is split into `T v;` and `v = e;`; other
//! declarations of the same local (same slot and name) in the new scope are
//! merged into it. A merge is made only when no read of one variable can see
//! a write of the other. A variable the table does not name, one a lambda or a
//! local or anonymous class captures (it must stay effectively final) and a
//! `final` one stay.

use crate::classes_processor::ClassNodeType;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::Type as DoType;
use crate::exprent::Exprent;
use crate::code_constants::{ACC_STATIC, OPC_ASTORE};
use crate::root_statement::RootStatement;
use crate::p3_dataflow::kuna_source_increments;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::statement::{Statement, StatementType};
use crate::struct_local_variable_table_attribute::{
    LocalVariable, StructLocalVariableTableAttribute,
};
use crate::var_exprent::VarExprent;
use crate::type_family::TypeFamily;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;
use crate::p6_variables::kuna_fix_var_merge_liveness;

/// Where a declaration is printed: in an exprent list (a block's statements
/// or the definitions printed before a statement), or in a statement's
/// header (a `for` initializer, a for-each variable, a `catch` parameter, a
/// resource).
#[derive(Clone)]
enum Place {
    Listed(JList<Exprent>, Exprent),
    Header(Statement),
}

/// A declaration's new position: in `list`, before `before` (at its end when
/// `None`).
#[derive(Clone)]
struct Target {
    list: JList<Exprent>,
    before: Option<Exprent>,
}

#[derive(Clone)]
struct Decl {
    var: VarExprent,
    /// The definition (`v` or `v = e`) in its list, for a listed declaration.
    place: Place,
    bare: bool,
    /// The variable's original slot, when it is one of the method's locals.
    slot: Option<i32>,
    size: i32,
    /// The table's name for it, with the entries that name it.
    name: Option<String>,
    entries: Vec<LocalVariable>,
    block: usize,
    seq: f64,
    movable: bool,
    /// Whether another declaration of its local can be merged into it, or it
    /// into another (it is not captured, not `final`, not a for-each
    /// variable, `catch` parameter or resource).
    mergeable: bool,
    alive: bool,
    /// A `catch` parameter: its `try` statement and body block, the first
    /// offset of the body and of the handler.
    catch: Option<(Statement, usize, i32, i32)>,
    target: Option<Target>,
    /// The declaration printed at the target (`T v;`).
    bare_decl: Option<Exprent>,
    merged: Vec<usize>,
}

struct Scopes {
    varproc: VarProcessor,
    table: StructLocalVariableTableAttribute,
    first_local: i32,
    max_locals: i32,
    parents: Vec<Option<usize>>,
    decls: Vec<Decl>,
    occurrences: JHashMap<i32, Vec<VarExprent>>,
    /// The ids of the occurrences that an assignment stores.
    stores: JHashSet<i32>,
    captured: JHashSet<i32>,
    /// For a `try` statement: its enclosing block and the position before it.
    try_scopes: JHashMap<i32, (usize, f64)>,
    seq: f64,
    moves: Vec<usize>,
    /// Declarations merged into a variable in scope: the declaration and the
    /// variable's index and version.
    merges: Vec<(usize, VarVersionPair)>,
    this_slots: i32,
}

/// Hoists the declarations of `root` whose slots prove the source declared
/// them in an enclosing block (see the module comment), and returns the
/// variables the method's LocalVariableTable does not name; needs the table,
/// does nothing without one (`None`).
pub fn hoist(
    root: RootStatement,
    varproc: VarProcessor,
    first_local: i32,
    max_locals: i32,
) -> Option<JHashSet<i32>> {
    let table = varproc.get_method().get_local_variable_attr()?;
    let mut scopes = Scopes {
        varproc,
        table,
        first_local,
        max_locals,
        parents: vec![None],
        decls: Vec::new(),
        occurrences: JHashMap::new(),
        stores: JHashSet::new(),
        captured: JHashSet::new(),
        try_scopes: JHashMap::new(),
        seq: 0.0,
        moves: Vec::new(),
        merges: Vec::new(),
        this_slots: i32::from(!varproc.get_method().has_modifier(ACC_STATIC)),
    };
    scopes.collect(root.0);
    scopes.walk(root.0, 0);
    scopes.complete();
    for i in 0..scopes.decls.len() {
        if !scopes.decls[i].alive {
            continue;
        }
        if scopes.decls[i].catch.is_some() {
            scopes.fill_gaps(root, i);
        } else if !scopes.merge_split(root, i) {
            scopes.lower_slot(root, i);
        }
    }
    scopes.apply();
    let mut unnamed = JHashSet::new();
    for d in scopes.decls.iter().filter(|d| d.alive && d.name.is_none()) {
        unnamed.add(d.var.get_index());
    }
    Some(unnamed)
}

impl Scopes {
    fn collect(&mut self, stat: Statement) {
        let mut exprents = stat.get_var_definitions().to_vec();
        match stat.get_exprents() {
            Some(list) => exprents.extend(list.to_vec()),
            None => exprents.extend(stat.get_stat_exprents()),
        }
        for exprent in exprents {
            for e in exprent.get_all_exprents_rec(true).into_iter().chain([exprent]) {
                if let Some(var) = e.try_as_var() {
                    self.occurrences
                        .compute_if_absent(var.get_index(), Vec::new)
                        .push(var);
                } else if let Some(var) = e.try_as_assignment().and_then(|a| a.get_left().try_as_var()) {
                    self.stores.add(var.0.get_id());
                } else if let Some(new) = e.try_as_new() {
                    if captures(new) {
                        for inner in e.get_all_exprents_rec(true) {
                            if let Some(var) = inner.try_as_var() {
                                self.captured.add(var.get_index());
                            }
                        }
                    }
                }
            }
        }
        for child in stat.get_stats().borrow().to_vec() {
            self.collect(child);
        }
    }

    fn new_block(&mut self, parent: usize) -> usize {
        self.parents.push(Some(parent));
        self.parents.len() - 1
    }

    fn walk(&mut self, stat: Statement, block: usize) {
        self.listed(stat.get_var_definitions(), block);
        if let Some(exprents) = stat.get_exprents() {
            self.listed(exprents, block);
            return;
        }
        let stats = stat.get_stats().borrow().to_vec();
        if stat.type_() == StatementType::Sequence && !stat.is_labeled() {
            for child in stats {
                self.walk(child, block);
            }
            return;
        }
        let head = match stat.type_() {
            StatementType::Root
            | StatementType::If
            | StatementType::Switch
            | StatementType::Synchronized => stat.get_first(),
            _ => None,
        };
        if let Some(head) = head {
            self.walk(head, block);
        }
        match stat.type_() {
            StatementType::Sequence => {
                let inner = self.new_block(block);
                for child in stats {
                    self.walk(child, inner);
                }
            }
            StatementType::Do => {
                let header = self.new_block(block);
                let init = stat
                    .as_do()
                    .get_init_exprent()
                    .filter(|e| stat.as_do().get_looptype() == DoType::For && splittable(*e))
                    .and_then(|e| e.try_as_assignment())
                    .map(|a| a.get_left());
                for exprent in stat.get_stat_exprents() {
                    for e in exprent.get_all_exprents_rec(true).into_iter().chain([exprent]) {
                        if let Some(var) = e.try_as_var().filter(|v| v.is_definition()) {
                            let at = self.header(var, stat, header);
                            if init == Some(var.0) {
                                self.decls[at].bare = false;
                                self.decls[at].movable = true;
                                self.decls[at].mergeable = true;
                            }
                        }
                    }
                }
                for child in stats {
                    let inner = self.new_block(header);
                    self.walk(child, inner);
                }
            }
            StatementType::TryCatch => {
                let catch = stat.as_catch();
                self.try_scopes.put(stat.id(), (block, self.seq + 0.5));
                let header = self.new_block(block);
                for resource in catch.get_resources().iter() {
                    if let Some(var) = resource
                        .try_as_assignment()
                        .and_then(|a| a.get_left().try_as_var())
                        .filter(|v| v.is_definition())
                    {
                        self.header(var, stat, header);
                    }
                }
                let body = self.new_block(header);
                self.walk(stats[0], body);
                let vars = catch.get_vars().to_vec();
                let try_start = first_offset(stats[0]);
                for (i, &handler) in stats.iter().enumerate().skip(1) {
                    let inner = self.new_block(block);
                    if let Some(&var) = vars.get(i - 1) {
                        let at = self.header(var, stat, inner);
                        if let (Some(start), Some(handler_start)) = (try_start, handler_offset(handler)) {
                            self.decls[at].catch = Some((stat, body, start, handler_start));
                        }
                    }
                    self.walk(handler, inner);
                }
            }
            _ => {
                for child in stats.into_iter().filter(|c| Some(*c) != head) {
                    let inner = self.new_block(block);
                    self.walk(child, inner);
                }
            }
        }
    }

    fn listed(&mut self, list: JList<Exprent>, block: usize) {
        for exprent in list.to_vec() {
            let top = if let Some(var) = exprent.try_as_var() {
                Some((var, true))
            } else {
                exprent
                    .try_as_assignment()
                    .and_then(|a| a.get_left().try_as_var())
                    .map(|var| (var, false))
            };
            let Some((var, bare)) = top.filter(|(var, _)| var.is_definition()) else {
                continue;
            };
            let movable = !var.is_class_def() && (bare || splittable(exprent));
            let at = self.add(var, Place::Listed(list.clone(), exprent), bare, block, movable);
            self.decls[at].mergeable = !var.is_class_def();
        }
    }

    fn header(&mut self, var: VarExprent, stat: Statement, block: usize) -> usize {
        self.add(var, Place::Header(stat), true, block, false)
    }

    fn add(&mut self, var: VarExprent, place: Place, bare: bool, block: usize, movable: bool) -> usize {
        self.seq += 1.0;
        self.decls.push(Decl {
            var,
            place,
            bare,
            slot: None,
            size: var.get_var_type().stack_size.max(1),
            name: None,
            entries: Vec::new(),
            block,
            seq: self.seq,
            movable,
            mergeable: false,
            alive: true,
            catch: None,
            target: None,
            bare_decl: None,
            merged: Vec::new(),
        });
        self.decls.len() - 1
    }

    /// Fills in each declaration's slot, table name and entries.
    fn complete(&mut self) {
        for i in 0..self.decls.len() {
            let var = self.decls[i].var;
            if var.is_class_def() {
                continue;
            }
            let slot = match self.decls[i].catch {
                Some(_) => self.catch_slot(i),
                None => self.varproc.get_var_original_index(var.get_index()),
            };
            let Some(slot) = slot else {
                continue;
            };
            if !(self.this_slots..self.max_locals).contains(&slot) {
                continue;
            }
            self.decls[i].slot = Some(slot);
            let pair = VarVersionPair::from_var(var);
            if self.captured.contains(&var.get_index())
                || self.varproc.get_var_final(pair) == FinalType::ExplicitFinal
            {
                self.decls[i].movable = false;
                self.decls[i].mergeable = false;
            }
            let mut pcs = self.offsets(var.get_index());
            if let Some((_, _, _, handler_start)) = self.decls[i].catch {
                pcs.push((handler_start, true));
            }
            let slot_entries = self.table.matching_vars_index(slot);
            let inside: Vec<usize> = (0..slot_entries.len())
                .filter(|&k| {
                    let lv = &slot_entries[k];
                    pcs.iter().any(|&(pc, _)| lv.get_start() <= pc && pc < lv.get_end())
                })
                .collect();
            let after: Vec<usize> = (0..slot_entries.len())
                .filter(|&k| pcs.iter().any(|&(pc, store)| store && starts_after(&slot_entries[k], pc)))
                .collect();
            let named = if inside.is_empty() { &after } else { &inside };
            let Some(&first) = named.first() else {
                continue;
            };
            let name = slot_entries[first].get_name();
            if !name.contains('$') && named.iter().all(|&k| slot_entries[k].get_name() == name) {
                self.decls[i].entries = (0..slot_entries.len())
                    .filter(|k| {
                        slot_entries[*k].get_name() == name && (inside.contains(k) || after.contains(k))
                    })
                    .map(|k| slot_entries[k].clone())
                    .collect();
                self.decls[i].name = Some(name);
            }
        }
    }

    /// A `catch` parameter's slot: that of the table entry of its name that
    /// starts right after the handler's store.
    fn catch_slot(&self, i: usize) -> Option<i32> {
        let d = &self.decls[i];
        let (_, _, _, handler_start) = d.catch?;
        let name = d.var.get_lvt()?.get_name();
        self.table
            .get_variables()
            .into_iter()
            .find(|lv| {
                lv.get_name() == name
                    && lv.get_start() > handler_start
                    && lv.get_start() <= handler_start + 4
            })
            .map(|lv| lv.index)
    }

    /// The instruction offsets of the occurrences of variable `index`, each
    /// with whether it is a store.
    fn offsets(&self, index: i32) -> Vec<(i32, bool)> {
        let mut pcs = Vec::new();
        for var in self.occurrences.get(&index).into_iter().flatten() {
            let store = self.stores.contains(&var.0.get_id());
            if let Some(bytecode) = var.0.get_bytecode() {
                let mut pc = bytecode.next_set_bit(0);
                while pc >= 0 {
                    pcs.push((pc, store));
                    pc = bytecode.next_set_bit(pc as usize + 1);
                }
            }
        }
        pcs
    }

    fn encloses(&self, outer: usize, mut inner: usize) -> bool {
        loop {
            if inner == outer {
                return true;
            }
            match self.parents[inner] {
                Some(parent) => inner = parent,
                None => return false,
            }
        }
    }

    /// The live declarations in scope at a position `seq` of `block`, in
    /// printed order.
    fn in_scope(&self, block: usize, seq: f64) -> Vec<usize> {
        let mut found: Vec<usize> = (0..self.decls.len())
            .filter(|&j| {
                let d = &self.decls[j];
                d.alive && d.seq < seq && self.encloses(d.block, block)
            })
            .collect();
        found.sort_by(|&a, &b| self.decls[a].seq.total_cmp(&self.decls[b].seq));
        found
    }

    fn holds(&self, scope: &[usize], slot: i32) -> bool {
        scope.iter().any(|&j| {
            let d = &self.decls[j];
            d.slot.is_some_and(|s| s <= slot && slot < s + d.size)
        })
    }

    /// A declaration of a local whose slot a variable in scope (or a
    /// parameter) holds under the same name, with a table entry both share:
    /// one local of the source (so of one declared type), which Vineflower
    /// split in two where a value stored in it does not depend on the one
    /// before (`it = new Wrapper(it);`).
    /// The declaration is merged into that variable when no read of either
    /// can see a write of the other.
    fn merge_split(&mut self, root: RootStatement, i: usize) -> bool {
        let d = self.decls[i].clone();
        let (Some(slot), Some(name), true) = (d.slot, d.name.clone(), d.mergeable) else {
            return false;
        };
        let scope = self.in_scope(d.block, d.seq);
        let (target, entries, type_) = match scope
            .iter()
            .rev()
            .find(|&&j| self.decls[j].slot == Some(slot))
        {
            Some(&h) => {
                let held = &self.decls[h];
                if held.name.as_deref() != Some(name.as_str()) || !held.mergeable {
                    return false;
                }
                (VarVersionPair::from_var(held.var), held.entries.clone(), Some(held.var.get_var_type()))
            }
            None if slot < self.first_local => {
                let entries: Vec<LocalVariable> = self
                    .table
                    .matching_vars_index(slot)
                    .into_iter()
                    .filter(|lv| lv.get_start() == 0 && lv.get_name() == name)
                    .collect();
                if self.captured.contains(&slot) {
                    return false;
                }
                let pair = VarVersionPair::new(slot, 0);
                (pair, entries, self.varproc.get_var_type(pair))
            }
            None => return false,
        };
        let shared = d.entries.iter().any(|e| {
            entries
                .iter()
                .any(|t| t.get_start() == e.get_start() && t.get_end() == e.get_end())
        });
        let pair = VarVersionPair::from_var(d.var);
        let own = d.var.get_var_type();
        let primitive = |t: &VarType| t.type_family != TypeFamily::Object;
        if !shared
            || type_.is_none_or(|t| (primitive(&t) || primitive(&own)) && !t.equals(&own))
            || kuna_fix_var_merge_liveness::overwrites_live(root, pair, target)
            || kuna_fix_var_merge_liveness::overwrites_live(root, target, pair)
        {
            return false;
        }
        self.decls[i].alive = false;
        self.merges.push((i, target));
        true
    }

    /// The first rule: a declaration with a lower slot than a variable in
    /// scope declared outside its block, while both are in scope.
    fn lower_slot(&mut self, root: RootStatement, i: usize) -> bool {
        let d = self.decls[i].clone();
        let (Some(slot), true, true) = (d.slot, d.name.is_some(), d.movable) else {
            return false;
        };
        if slot < self.first_local {
            return false;
        }
        let scope = self.in_scope(d.block, d.seq);
        if self.holds(&scope, slot) {
            return false;
        }
        let Some(&w) = scope.iter().find(|&&j| self.decls[j].slot.is_some_and(|s| s > slot)) else {
            return false;
        };
        let anchor = &self.decls[w];
        if anchor.block == d.block || anchor.name.is_none() || !overlap(&d.entries, &anchor.entries) {
            return false;
        }
        let target = self.before(w);
        self.move_to(root, i, anchor.block, anchor.seq, target)
    }

    /// The second rule, for the `catch` parameter `i`: hoists out of its
    /// `try` body the locals that hold the slots below the parameter's.
    fn fill_gaps(&mut self, root: RootStatement, i: usize) -> bool {
        let e = self.decls[i].clone();
        let (Some(slot), Some((stat, body, try_start, handler_start))) = (e.slot, e.catch) else {
            return false;
        };
        if e.name.is_none() {
            return false;
        }
        let Some((block, seq)) = self.try_scopes.get_cloned(&stat.id()) else {
            return false;
        };
        let mut changed = false;
        for k in self.first_local..slot {
            let scope = self.in_scope(block, seq);
            if self.holds(&scope, k) {
                continue;
            }
            let candidates: Vec<usize> = (0..self.decls.len())
                .filter(|&j| {
                    let x = &self.decls[j];
                    x.alive
                        && x.target.is_none()
                        && x.slot == Some(k)
                        && x.name.is_some()
                        && self.encloses(body, x.block)
                        && x.entries.iter().any(|lv| {
                            lv.get_start().max(try_start) < lv.get_end().min(handler_start)
                        })
                })
                .collect();
            let Some(&x) = candidates.first() else {
                continue;
            };
            if candidates.iter().any(|&j| self.decls[j].name != self.decls[x].name)
                || !self.decls[x].movable
                || k + self.decls[x].size > slot
            {
                continue;
            }
            let (target, at) = match scope.iter().find(|&&j| self.decls[j].slot.is_some_and(|s| s > k)) {
                Some(&w) => (self.before(w), self.decls[w].seq),
                None => (
                    Target {
                        list: stat.get_var_definitions(),
                        before: None,
                    },
                    seq,
                ),
            };
            changed |= self.move_to(root, x, block, at, target);
        }
        changed
    }

    /// The position right before declaration `w`.
    fn before(&self, w: usize) -> Target {
        let anchor = &self.decls[w];
        if let (Some(target), Some(bare)) = (&anchor.target, anchor.bare_decl) {
            return Target {
                list: target.list.clone(),
                before: Some(bare),
            };
        }
        match &anchor.place {
            Place::Listed(list, exprent) => Target {
                list: list.clone(),
                before: Some(*exprent),
            },
            Place::Header(stat) => Target {
                list: stat.get_var_definitions(),
                before: None,
            },
        }
    }

    /// Moves declaration `i` to `target` in `block`, right before position
    /// `at`, with the other declarations of its local that the new scope
    /// holds; refused when one of them cannot be merged into it or another
    /// variable of its name is declared there.
    fn move_to(&mut self, root: RootStatement, i: usize, block: usize, at: f64, target: Target) -> bool {
        let d = self.decls[i].clone();
        let name = d.var.get_name();
        let mut partners = Vec::new();
        for j in 0..self.decls.len() {
            let x = &self.decls[j];
            if j == i || !x.alive || x.seq < at || !self.encloses(block, x.block) {
                continue;
            }
            if x.var.get_name() != name && x.name != d.name {
                continue;
            }
            if x.slot != d.slot
                || x.name != d.name
                || !x.mergeable
                || x.var.get_definition_type() != d.var.get_definition_type()
                || !x.var.get_var_type().equals(&d.var.get_var_type())
            {
                return false;
            }
            partners.push(j);
        }
        let pair = VarVersionPair::from_var(d.var);
        for &j in &partners {
            let other = VarVersionPair::from_var(self.decls[j].var);
            if kuna_fix_var_merge_liveness::overwrites_live(root, other, pair)
                || kuna_fix_var_merge_liveness::overwrites_live(root, pair, other)
            {
                return false;
            }
        }
        let below = self
            .decls
            .iter()
            .filter(|x| x.alive && x.seq < at)
            .map(|x| x.seq)
            .fold(f64::MIN, f64::max)
            .max(at - 1.0);
        let bare = if d.bare {
            d.var.0
        } else {
            let copy = d.var.copy();
            copy.as_var().set_definition(true);
            copy
        };
        let decl = &mut self.decls[i];
        decl.block = block;
        decl.seq = (below + at) / 2.0;
        decl.target = Some(target);
        decl.bare_decl = Some(bare);
        decl.merged = partners.clone();
        for j in partners {
            self.decls[j].alive = false;
        }
        self.moves.push(i);
        true
    }

    /// Makes the moves, in the order they were decided (a later one may be
    /// placed before an earlier one's declaration).
    fn apply(&self) {
        for &i in &self.moves {
            let d = &self.decls[i];
            let (Some(target), Some(bare)) = (&d.target, d.bare_decl) else {
                continue;
            };
            match &d.place {
                Place::Listed(list, exprent) if d.bare => remove(list, *exprent),
                _ => d.var.set_definition(false),
            }
            let at = target
                .before
                .and_then(|b| position(&target.list, b))
                .unwrap_or(target.list.len());
            target.list.add_at(at, bare);
            for &j in &d.merged {
                self.merge(j, VarVersionPair::from_var(d.var));
            }
        }
        for &(j, target) in &self.merges {
            self.merge(j, target);
        }
    }

    /// Merges declaration `j`'s variable into `target`: its occurrences become
    /// `target`'s and its definition an assignment.
    fn merge(&self, j: usize, target: VarVersionPair) {
        let x = &self.decls[j];
        let lvt = self
            .occurrences
            .get(&target.var)
            .into_iter()
            .flatten()
            .find_map(|v| v.get_lvt());
        for &var in self.occurrences.get(&x.var.get_index()).into_iter().flatten() {
            var.set_index(target.var);
            var.set_version(target.version);
            if lvt.is_some() {
                var.set_lvt(lvt.clone());
            }
        }
        match &x.place {
            Place::Listed(list, exprent) if x.bare => remove(list, *exprent),
            _ => x.var.set_definition(false),
        }
    }
}

/// Whether a `new` captures the variables it is given: a lambda (not a method
/// reference) or an anonymous or local class.
fn captures(new: crate::new_exprent::NewExprent) -> bool {
    if new.is_lambda() {
        return !new.is_method_reference();
    }
    if new.is_anonymous() {
        return true;
    }
    let name = new.get_new_type().value.clone();
    name.is_some_and(|name| {
        DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .get(&name.to_string())
            .is_some_and(|node| node.with(|d| d.type_) == ClassNodeType::Local)
    })
}

/// Whether `lv` starts right after an instruction at `pc` (a store).
fn starts_after(lv: &LocalVariable, pc: i32) -> bool {
    pc < lv.get_start() && lv.get_start() - pc <= 4
}

fn overlap(a: &[LocalVariable], b: &[LocalVariable]) -> bool {
    a.iter().any(|x| {
        b.iter()
            .any(|y| x.get_start() < y.get_end() && y.get_start() < x.get_end())
    })
}

/// The first instruction offset of the exprents of `stat`.
fn first_offset(stat: Statement) -> Option<i32> {
    let mut exprents = stat.get_var_definitions().to_vec();
    match stat.get_exprents() {
        Some(list) => exprents.extend(list.to_vec()),
        None => exprents.extend(stat.get_stat_exprents()),
    }
    let own = exprents
        .into_iter()
        .filter_map(kuna_source_increments::range)
        .map(|values| values.next_set_bit(0))
        .min();
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().filter_map(first_offset).chain(own).min()
}

/// The offset of a `catch` handler's first instruction, the store of its
/// exception.
fn handler_offset(handler: Statement) -> Option<i32> {
    let block = handler.get_basichead().get_block();
    (!block.get_seq().is_empty() && block.get_instruction(0).with(|d| d.opcode) == OPC_ASTORE)
        .then(|| block.get_old_offset(0))
        .filter(|&offset| offset > -1)
}

/// Whether the definition `exprent` (`T v = e;`) can be written `T v;` and
/// `v = e;`: its type has a name (it is not printed `var`).
fn splittable(exprent: Exprent) -> bool {
    exprent
        .try_as_assignment()
        .and_then(|a| a.get_left().try_as_var())
        .is_some_and(|v| !v.is_intersection_type() && v.get_definition_type() != "var")
}

fn position(list: &JList<Exprent>, exprent: Exprent) -> Option<usize> {
    list.to_vec().iter().position(|e| *e == exprent)
}

fn remove(list: &JList<Exprent>, exprent: Exprent) {
    if let Some(at) = position(list, exprent) {
        list.remove_at(at);
    }
}
