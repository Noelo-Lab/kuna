//! kuna fix `kt_file_static_initializer` (spec 10.16): what is left of a
//! file class's static initializer is written as the initializers of the
//! properties it sets.
//!
//! kotlinc compiles the initializers of a file's top-level properties, in
//! their declaration order, into the file class's `<clinit>`.
//! `InitializerProcessor` moves each one it can into its property; one built
//! over several statements (a `try`, a loop, a checked value) stays in
//! `<clinit>`, which Vineflower printed as `fun {`, no Kotlin, and its
//! property had no value. A file has no `init` block, so with the fix the
//! remainder is cut at each store into a property of the file: the code
//! before a store and the stored value become that property's initializer,
//! `val x: T = run { ..; value }` (the value alone when nothing precedes
//! it). The properties are written in field order, which is the order
//! `<clinit>` stores them in (checked on the bytecode), so every statement
//! runs in its place. A local that a later part uses again is declared anew
//! there, which needs it written before it is read; when that cannot be
//! shown, when code follows the last store, or when a property is set
//! inside a nested statement, the remainder is left as it was.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::assignment_exprent::AssignmentExprent;
use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_STATIC, OPC_PUTSTATIC};
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::Exprent;
use crate::interpreter_util::InterpreterUtil;
use crate::k_element::KElement;
use crate::k_expr_processor::KExprProcessor;
use crate::k_var_exprent::{DeclarationType, KVarExprent};
use crate::kt_attrs::KtAttrs;
use crate::p2_lift::kuna_fix_disjoint_generic_cast;
use crate::rt::{try_catch, JList};
use crate::statement::{Statement, StatementType};
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;

thread_local! {
    static INITIALIZERS: RefCell<Vec<(StructField, TextBuffer)>> = const { RefCell::new(Vec::new()) };
}

/// One piece of the remainder: an exprent of a top-level block or a whole statement.
#[derive(Clone, Copy)]
enum Item {
    /// An exprent and the block it is in.
    Expr(Exprent, Statement),
    Stat(Statement),
}

/// The code before a store into `field`, the store, its value and its block.
struct Part {
    items: Vec<Item>,
    field: StructField,
    store: AssignmentExprent,
    value: Exprent,
    block: Statement,
}

/// A local an earlier part also uses, declared anew: by its definition, or
/// by its first store, item `at` of the part, written as a definition.
struct Redeclared {
    def: Exprent,
    at: Option<usize>,
}

/// Writes the remainder of the static initializer of the file class of
/// `wrapper` as property initializers (taken by [`initializer`] while the
/// properties are written); whether it was, so `<clinit>` is not written.
pub fn prepare(wrapper: ClassWrapper, indent: i32) -> bool {
    INITIALIZERS.with(|i| i.borrow_mut().clear());
    let Some(mw) = wrapper.get_method_wrapper("<clinit>", "()V") else {
        return false;
    };
    let Some(root) = mw
        .with(|d| d.root)
        .filter(|_| mw.with(|d| d.decompile_error.is_none()))
    else {
        return false;
    };
    let cl = wrapper.get_class_struct();
    let Some(first) = root.0.get_first() else {
        return false;
    };
    let mut defs: Vec<Exprent> = root.0.get_var_definitions().to_vec();
    let mut top = Vec::new();
    if !flatten(first, &mut top, &mut defs) {
        return false;
    }
    let Some(parts) = split(cl, wrapper, &top) else {
        return false;
    };
    if !stored_in_field_order(cl, &parts) {
        return false;
    }
    let Some(redeclared) = shared_locals(&parts, &defs) else {
        return false;
    };
    let outer = DecompilerContext::get_current_method_wrapper();
    DecompilerContext::set_current_method_wrapper(Some(mw));
    let texts = try_catch(|| {
        parts
            .iter()
            .enumerate()
            .map(|(i, part)| (part.field, text(part, &redeclared[i], indent)))
            .collect::<Vec<_>>()
    });
    DecompilerContext::set_current_method_wrapper(outer);
    let Ok(texts) = texts else {
        return false;
    };
    INITIALIZERS.with(|i| *i.borrow_mut() = texts);
    true
}

/// The initializer [`prepare`] made for the property of `field`.
pub fn initializer(field: StructField) -> Option<TextBuffer> {
    INITIALIZERS.with(|i| {
        let mut list = i.borrow_mut();
        let pos = list.iter().position(|(f, _)| *f == field)?;
        Some(list.remove(pos).1)
    })
}

/// The top-level statements of the remainder: nested unlabeled sequences
/// flattened, their variable definitions collected into `defs`.
fn flatten(stat: Statement, out: &mut Vec<Statement>, defs: &mut Vec<Exprent>) -> bool {
    if stat.type_() == StatementType::Sequence {
        if stat.is_labeled() {
            return false;
        }
        defs.extend(stat.get_var_definitions().to_vec());
        for st in stat.get_stats().borrow().to_vec() {
            if !flatten(st, out, defs) {
                return false;
            }
        }
        return true;
    }
    if stat.type_() == StatementType::BasicBlock {
        if stat.is_labeled() || !stat.get_var_definitions().is_empty() {
            return false;
        }
        let jumps = stat.get_all_successor_edges().iter().any(|e| e.explicit());
        if jumps {
            return false;
        }
    }
    out.push(stat);
    true
}

/// The static field of `cl` that `e` stores into at its top, if any.
fn own_static_store(cl: StructClass, e: Exprent) -> Option<(StructField, AssignmentExprent)> {
    let assign = e
        .try_as_assignment()
        .filter(|a| a.get_cond_type().is_none())?;
    let field = assign.get_left().try_as_field()?;
    if !field.is_static() || field.get_classname() != cl.qualified_name() {
        return None;
    }
    let fd = cl.get_field(&field.get_name(), &field.get_descriptor().descriptor_string)?;
    Some((fd, assign))
}

/// Whether anything under `stat` stores into a static field of `cl`.
fn stores_own_field(cl: StructClass, stat: Statement) -> bool {
    let mut lists = stat.get_stat_exprents();
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    lists.into_iter().any(|e| {
        e.get_all_exprents_rec_self(true, true)
            .into_iter()
            .any(|x| {
                x.try_as_assignment()
                    .and_then(|a| a.get_left().try_as_field())
                    .is_some_and(|f| f.is_static() && f.get_classname() == cl.qualified_name())
            })
    }) || stat
        .get_stats()
        .borrow()
        .to_vec()
        .into_iter()
        .any(|st| stores_own_field(cl, st))
}

/// Cuts the remainder at the stores into the file's properties.
fn split(cl: StructClass, wrapper: ClassWrapper, top: &[Statement]) -> Option<Vec<Part>> {
    let mut parts = Vec::new();
    let mut items = Vec::new();
    for &st in top {
        if st.type_() == StatementType::BasicBlock {
            for e in st.get_exprents()?.to_vec() {
                match own_static_store(cl, e) {
                    Some((field, store)) => {
                        let value = store.get_right();
                        if !takes_initializer(cl, wrapper, field) {
                            return None;
                        }
                        let nested =
                            value
                                .get_all_exprents_rec_self(true, true)
                                .into_iter()
                                .any(|x| {
                                    x.try_as_assignment()
                                        .and_then(|a| a.get_left().try_as_field())
                                        .is_some_and(|f| {
                                            f.is_static()
                                                && f.get_classname() == cl.qualified_name()
                                        })
                                });
                        if nested {
                            return None;
                        }
                        parts.push(Part {
                            items: std::mem::take(&mut items),
                            field,
                            store,
                            value,
                            block: st,
                        });
                    }
                    None => {
                        if e.get_all_exprents_rec_self(true, true)
                            .into_iter()
                            .any(|x| {
                                x.try_as_assignment()
                                    .and_then(|a| a.get_left().try_as_field())
                                    .is_some_and(|f| {
                                        f.is_static() && f.get_classname() == cl.qualified_name()
                                    })
                            })
                        {
                            return None;
                        }
                        items.push(Item::Expr(e, st));
                    }
                }
            }
        } else {
            if stores_own_field(cl, st) {
                return None;
            }
            items.push(Item::Stat(st));
        }
    }
    let mut seen = BTreeSet::new();
    let unique = parts.iter().all(|p| seen.insert(p.field.get_name()));
    (items.is_empty() && !parts.is_empty() && unique).then_some(parts)
}

/// Whether `field` is a property of the file written without an initializer.
fn takes_initializer(cl: StructClass, wrapper: ClassWrapper, field: StructField) -> bool {
    let key = InterpreterUtil::make_unique_key(&field.get_name(), &field.get_descriptor());
    let is_property = matches!(
        KtAttrs::get(field, &KElement::KEY()),
        Some(KElement::Property(p)) if p.class_struct == cl
    );
    is_property
        && field.has_modifier(ACC_STATIC)
        && wrapper
            .get_static_field_initializers()
            .borrow()
            .get_with_key(&key)
            .is_none()
}

/// Whether `<clinit>` stores the fields of `cl` in field order, each once,
/// and the parts' fields come in that order.
fn stored_in_field_order(cl: StructClass, parts: &[Part]) -> bool {
    let fields = cl.get_fields();
    let index = |name: &str, desc: &str| {
        let key = InterpreterUtil::make_unique_key(name, desc);
        fields
            .contains_key(&key)
            .then(|| fields.get_index_by_key(&key))
    };
    let order: Vec<usize> = parts
        .iter()
        .filter_map(|p| index(&p.field.get_name(), &p.field.get_descriptor()))
        .collect();
    if order.len() != parts.len() || order.windows(2).any(|w| w[0] >= w[1]) {
        return false;
    }
    let Some(mt) = cl.get_method("<clinit>", "()V") else {
        return false;
    };
    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return false;
    }
    let mut stores: Vec<usize> = Vec::new();
    let mut ok = true;
    if let Some(seq) = mt.get_instruction_sequence() {
        for i in 0..seq.length() {
            let instr = seq.get_instr(i);
            if instr.opcode() != OPC_PUTSTATIC {
                continue;
            }
            let Some(link) = cl.get_pool().get_link_constant(instr.operand(0)) else {
                ok = false;
                break;
            };
            if link.classname.as_deref() != Some(cl.qualified_name().as_str()) {
                continue;
            }
            match index(&link.elementname, &link.descriptor) {
                Some(ix) => stores.push(ix),
                None => {
                    ok = false;
                    break;
                }
            }
        }
    } else {
        ok = false;
    }
    if !expanded {
        mt.release_resources();
    }
    ok && stores.windows(2).all(|w| w[0] < w[1])
}

/// The locals of each part, by variable index, with the first reference
/// being a store (`true`) or a read.
fn references(part: &Part) -> BTreeMap<i32, bool> {
    let mut res = BTreeMap::new();
    for item in &part.items {
        match *item {
            Item::Expr(e, _) => refs_exprent(e, &mut res),
            Item::Stat(st) => refs_stat(st, &mut res),
        }
    }
    refs_exprent(part.value, &mut res);
    res
}

fn refs_exprent(e: Exprent, res: &mut BTreeMap<i32, bool>) {
    if let Some(assign) = e.try_as_assignment() {
        refs_exprent(assign.get_right(), res);
        match assign.get_left().try_as_var() {
            Some(var) if assign.get_cond_type().is_none() => {
                res.entry(var.get_index()).or_insert(true);
            }
            _ => refs_exprent(assign.get_left(), res),
        }
        return;
    }
    if let Some(var) = e.try_as_var() {
        let defines = e
            .try_as_k_var()
            .is_some_and(|k| k.kd(|d| d.declaration_type) == Some(DeclarationType::Definition));
        if !defines {
            res.entry(var.get_index()).or_insert(false);
        }
        return;
    }
    for x in e.get_all_exprents() {
        refs_exprent(x, res);
    }
}

fn refs_stat(stat: Statement, res: &mut BTreeMap<i32, bool>) {
    for e in stat.get_var_definitions().to_vec() {
        refs_exprent(e, res);
    }
    let mut lists = stat.get_stat_exprents();
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    let heads_first = stat.type_() != StatementType::Do;
    if heads_first {
        for e in lists.iter().copied() {
            refs_exprent(e, res);
        }
    }
    for st in stat.get_stats().borrow().to_vec() {
        refs_stat(st, res);
    }
    if !heads_first {
        for e in lists {
            refs_exprent(e, res);
        }
    }
}

/// For each part, the definitions of the locals an earlier part also uses,
/// to be declared anew; `None` when a part reads such a local before it
/// stores it. The definitions in `defs` go to the part that first uses them.
fn shared_locals(parts: &[Part], defs: &[Exprent]) -> Option<Vec<Vec<Redeclared>>> {
    let refs: Vec<BTreeMap<i32, bool>> = parts.iter().map(references).collect();
    let mut definitions: BTreeMap<i32, Exprent> = BTreeMap::new();
    for part in parts {
        for item in &part.items {
            let exprents = match *item {
                Item::Expr(e, _) => e.get_all_exprents_rec_self(true, true),
                Item::Stat(st) => stat_exprents(st),
            };
            for e in exprents {
                if let Some(k) = e.try_as_k_var() {
                    if k.kd(|d| d.declaration_type) == Some(DeclarationType::Definition) {
                        definitions.entry(k.get_index()).or_insert(e);
                    }
                }
            }
        }
    }
    for d in defs {
        if let Some(var) = d.try_as_var() {
            definitions.entry(var.get_index()).or_insert(*d);
        }
    }
    let mut res: Vec<Vec<Redeclared>> = parts.iter().map(|_| Vec::new()).collect();
    let mut seen: BTreeSet<i32> = BTreeSet::new();
    let mut hoisted: BTreeSet<i32> = defs
        .iter()
        .filter_map(|d| d.try_as_var())
        .map(|v| v.get_index())
        .collect();
    for (i, r) in refs.iter().enumerate() {
        for (&var, &stored_first) in r {
            if seen.contains(&var) {
                if !stored_first {
                    return None;
                }
                res[i].push(Redeclared {
                    def: *definitions.get(&var)?,
                    at: first_store_item(&parts[i], var),
                });
            } else if hoisted.remove(&var) {
                res[i].push(Redeclared {
                    def: *definitions.get(&var)?,
                    at: None,
                });
            }
        }
        seen.extend(r.keys().copied());
    }
    Some(res)
}

/// Every exprent under `stat`, definitions included.
fn stat_exprents(stat: Statement) -> Vec<Exprent> {
    let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
    lists.extend(stat.get_stat_exprents());
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    let mut res: Vec<Exprent> = lists
        .into_iter()
        .flat_map(|e| e.get_all_exprents_rec_self(true, true))
        .collect();
    for st in stat.get_stats().borrow().to_vec() {
        res.extend(stat_exprents(st));
    }
    res
}

/// A fresh declaration (no value) of the local `def` defines.
fn declaration(def: Exprent, indent: i32) -> TextBuffer {
    let var = def.try_as_var().expect("a definition");
    let copy = KVarExprent::new(VarExprent::copy(var).as_var());
    copy.set_definition(true);
    copy.set_declaration_type(Some(DeclarationType::Definition));
    let mut buf = TextBuffer::new();
    buf.append_indent(indent)
        .append_buffer(&copy.to_java(indent))
        .append_line_separator();
    buf
}

/// The item of `part` that first refers to the local `var` when it is a
/// store into it, `var = e`, at the top of a block.
fn first_store_item(part: &Part, var: i32) -> Option<usize> {
    for (i, item) in part.items.iter().enumerate() {
        let mut refs = BTreeMap::new();
        match *item {
            Item::Expr(e, _) => refs_exprent(e, &mut refs),
            Item::Stat(st) => refs_stat(st, &mut refs),
        }
        if !refs.contains_key(&var) {
            continue;
        }
        let Item::Expr(e, _) = *item else {
            return None;
        };
        let assign = e
            .try_as_assignment()
            .filter(|a| a.get_cond_type().is_none())?;
        let mut right = BTreeMap::new();
        refs_exprent(assign.get_right(), &mut right);
        let stores = assign
            .get_left()
            .try_as_var()
            .is_some_and(|v| v.get_index() == var);
        return (stores && !right.contains_key(&var)).then_some(i);
    }
    None
}

/// `e`, a store into a local, written as the local's definition.
fn as_definition(e: Exprent) -> Exprent {
    let copy = e.copy();
    let left = copy.as_assignment().get_left();
    let var = left.as_var();
    var.set_definition(true);
    if let Some(k) = left.try_as_k_var() {
        k.set_declaration_type(Some(DeclarationType::Definition));
    }
    copy
}

/// The initializer of the property `part` sets, its statements apart as a
/// sequence writes them.
fn text(part: &Part, redeclared: &[Redeclared], indent: i32) -> TextBuffer {
    if part.items.is_empty() && redeclared.is_empty() {
        return value_text(part.store, indent);
    }
    let inner = indent + 1;
    let mut chunks: Vec<TextBuffer> = Vec::new();
    let mut head = TextBuffer::new();
    for r in redeclared.iter().filter(|r| r.at.is_none()) {
        head.append_buffer(&declaration(r.def, inner));
    }
    let mut pending: Vec<Exprent> = Vec::new();
    let mut pending_block: Option<Statement> = None;
    let flush = |pending: &mut Vec<Exprent>, chunks: &mut Vec<TextBuffer>| {
        if !pending.is_empty() {
            let list = JList::from(std::mem::take(pending));
            chunks.push(ExprProcessor::list_to_java(Some(&list), inner));
        }
    };
    for (i, item) in part.items.iter().enumerate() {
        match *item {
            Item::Expr(e, block) => {
                if pending_block != Some(block) {
                    flush(&mut pending, &mut chunks);
                    pending_block = Some(block);
                }
                let e = if redeclared.iter().any(|r| r.at == Some(i)) {
                    as_definition(e)
                } else {
                    e
                };
                pending.push(e);
            }
            Item::Stat(st) => {
                flush(&mut pending, &mut chunks);
                pending_block = None;
                chunks.push(KExprProcessor::jmp_wrapper(st, inner, false));
            }
        }
    }
    let same_block = pending_block == Some(part.block);
    flush(&mut pending, &mut chunks);
    let mut value = TextBuffer::new();
    value
        .append_indent(inner)
        .append_buffer(&value_text(part.store, inner))
        .append_line_separator();
    match chunks.last_mut() {
        Some(last) if same_block => {
            last.append_buffer(&value);
        }
        _ => chunks.push(value),
    }
    let mut buf = TextBuffer::new();
    buf.append("run {")
        .append_line_separator()
        .append_buffer(&head);
    let mut not_empty = head.length() > 0;
    for chunk in chunks {
        if chunk.contains_only_whitespaces() {
            continue;
        }
        if not_empty {
            buf.append_line_separator();
        }
        buf.append_buffer(&chunk);
        not_empty = true;
    }
    buf.append_indent(indent).append("}");
    buf
}

/// The value of `store` as its assignment writes it: cast to the field's type.
fn value_text(store: AssignmentExprent, indent: i32) -> TextBuffer {
    store.get_inferred_expr_type(None);
    let left_type = store.get_left().get_inferred_expr_type(None);
    if let Some(right_const) = store.get_right().try_as_const() {
        right_const.adjust_const_type(&left_type);
    }
    store.optimize_cast_for_assign();
    let right = store.get_right();
    let cast_name = {
        let _lock = DecompilerContext::get_import_collector().lock();
        ExprProcessor::get_cast_type_name(&left_type)
    };
    let mut buf = TextBuffer::new();
    if cast_name == ExprProcessor::UNREPRESENTABLE_TYPE_STRING {
        buf.append_buffer(&right.to_java(indent));
    } else {
        kuna_fix_disjoint_generic_cast::mark_declared_target();
        ExprProcessor::get_casted_exprent_full(
            right,
            &left_type,
            &mut buf,
            indent,
            NullCastType::DontCastAtAll,
            false,
            false,
            false,
        );
    }
    buf
}
