//! kuna fix `kt_nullable_local` (spec 10.16): a Kotlin local that the method
//! stores a nullable value into is declared with a nullable type.
//!
//! Vineflower writes a local's declared type from its JVM type, never with a
//! `?`, so a local that holds a nullable value (`var s: Segment = this.head`,
//! `head` a `Segment?`; `var10000 = null`) is declared non-null and kotlinc
//! rejects the store. A store's value may be null when it is the `null`
//! constant; a field, a call or a parameter whose Kotlin type is nullable
//! (the member's `@Nullable` annotation, else the owner's metadata); a
//! mapped `Map.get`/`put`/`remove`, which Kotlin types `V?`; or a local that
//! may hold null. A cast or an `if` expression passes its operands'
//! nullability on; `x!!`, `new` and the other constants are non-null. A store
//! that re-reads the value a null check of the enclosing `if` or `if`
//! expression just tested (`if (this.head != null) { val h = this.head`) is
//! non-null: the bytecode checked that value. A local any other store of
//! which may be null is declared `T?`; kotlinc smart-casts its uses after
//! the method's own null checks. Which value and which call are unchanged.
//! With fix `kt_raw_generic_type`, an element read through a local that fix
//! typed with a star-projected or nullable argument may be null too. With
//! fix `kt_suspend_call_continuation`, in the body of a suspend function a
//! suspend call's result is as nullable as its callee's metadata declares:
//! kotlinc annotates the `Object` the method returns (the value or the
//! suspension marker) `@Nullable`.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::Type as DoType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::if_statement::IfStatement;
use crate::k_element::KElement;
use crate::kotlin::kuna_fix_kt_suspend_call_continuation;
use crate::kt_flags::Flags;
use crate::k_function_exprent::KFunctionType;
use crate::k_property::PropertyMethod;
use crate::k_type::KType;
use crate::k_type_parameter::KTypeParameter;
use crate::k_var_exprent::DeclarationType;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::kuna_fixes::{self, KunaFix};
use crate::root_statement::RootStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::struct_class::StructClass;
use crate::struct_member::StructMember;
use crate::struct_method::StructMethod;
use crate::type_family::TypeFamily;

const NOT_NULL: &str = "org/jetbrains/annotations/NotNull";
const NULLABLE: &str = "org/jetbrains/annotations/Nullable";

/// JDK maps whose `get`/`put`/`remove` Kotlin types as the mapped
/// `MutableMap`'s, `V?`.
const JDK_MAPS: &[&str] = &[
    "java/util/Map",
    "java/util/AbstractMap",
    "java/util/HashMap",
    "java/util/LinkedHashMap",
    "java/util/TreeMap",
    "java/util/SortedMap",
    "java/util/NavigableMap",
    "java/util/Hashtable",
    "java/util/Properties",
    "java/util/WeakHashMap",
    "java/util/IdentityHashMap",
    "java/util/EnumMap",
    "java/util/concurrent/ConcurrentMap",
    "java/util/concurrent/ConcurrentHashMap",
    "java/util/concurrent/ConcurrentNavigableMap",
    "java/util/concurrent/ConcurrentSkipListMap",
];

const MAP_LOOKUPS: &[(&str, &str)] = &[
    ("get", "(Ljava/lang/Object;)Ljava/lang/Object;"),
    ("remove", "(Ljava/lang/Object;)Ljava/lang/Object;"),
    (
        "put",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
    ),
];

/// A store into a local: the value, and the values found non-null there.
struct Store {
    var: i32,
    value: Exprent,
    checked: Vec<Exprent>,
}

/// Marks the definitions under `root` of the locals that may hold null.
pub fn infer(root: RootStatement, method: StructMethod) {
    LOOKUPS.with(|l| l.borrow_mut().clear());
    let _suspend_body = kuna_fix_kt_suspend_call_continuation::enter_body(method);
    let params = parameter_nullability(method);
    let mut walk = Walk {
        params: &params,
        casts: Vec::new(),
        uses: Vec::new(),
        derefs: Vec::new(),
        checked: Vec::new(),
        kills: Vec::new(),
        stores: Vec::new(),
        definitions: BTreeMap::new(),
    };
    walk.stat(root.0);
    if walk.definitions.is_empty() && walk.casts.is_empty() {
        return;
    }
    let mut nullable: BTreeSet<i32> = BTreeSet::new();
    let mut pending: Vec<&Store> = Vec::new();
    for store in &walk.stores {
        if may_be_null(store.value, &store.checked, &nullable, &params) == Some(true) {
            nullable.insert(store.var);
        } else if reads_local(store.value) {
            pending.push(store);
        }
    }
    loop {
        let mut changed = false;
        for store in &pending {
            if !nullable.contains(&store.var)
                && may_be_null(store.value, &store.checked, &nullable, &params) == Some(true)
            {
                nullable.insert(store.var);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    if kuna_fixes::enabled(KunaFix::KtNullableLocal) {
        for (var, defs) in walk.definitions {
            if !nullable.contains(&var) {
                continue;
            }
            for def in defs {
                let k = def.as_k_var();
                if k.get_definition_var_type().type_family == TypeFamily::Object {
                    k.kd_mut(|d| d.nullable = true);
                }
            }
        }
    }
    if kuna_fixes::enabled(KunaFix::KtNullableCast) {
        for var in &walk.uses {
            if nullable.contains(&var.as_var().get_index()) {
                var.as_k_var().kd_mut(|d| d.nullable = true);
            }
        }
        for cast in walk.casts {
            if walk.derefs.contains(&cast.value) {
                continue;
            }
            let func = cast.value.as_k_function();
            let operand = func.get_lst_operands().get(0);
            let to = func.get_lst_operands().get(1).get_expr_type();
            if to.type_family == TypeFamily::Object
                && may_be_null(operand, &cast.checked, &nullable, &params) == Some(true)
            {
                func.kd_mut(|d| d.nullable_cast = true);
            }
        }
    }
}

/// The stores and definitions of a method body in execution order, with the
/// values (locals, fields) its null checks have found non-null on the way.
struct Walk<'a> {
    params: &'a BTreeMap<i32, bool>,
    casts: Vec<Store>,
    uses: Vec<Exprent>,
    derefs: Vec<Exprent>,
    checked: Vec<Exprent>,
    kills: Vec<Exprent>,
    stores: Vec<Store>,
    definitions: BTreeMap<i32, Vec<Exprent>>,
}

impl Walk<'_> {
    fn stat(&mut self, stat: Statement) {
        if stat.type_() == StatementType::Do {
            let mut stored = Vec::new();
            stores_in(stat, &mut stored);
            self.checked
                .retain(|c| !stored.iter().any(|l| same(*c, *l)));
        }
        let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
        lists.extend(stat.get_stat_exprents());
        if let Some(exprents) = stat.get_exprents() {
            lists.extend(exprents.to_vec());
        }
        for expr in lists {
            self.exprent(expr);
        }
        let guards = guards(stat);
        let sequence = stat.type_() == StatementType::Sequence;
        for child in stat.get_stats().borrow().to_vec() {
            let saved = self.checked.clone();
            let mark = self.kills.len();
            self.checked.extend(
                guards
                    .iter()
                    .filter(|(body, _)| *body == child)
                    .map(|(_, v)| *v),
            );
            self.stat(child);
            let kills = &self.kills[mark..];
            self.checked = saved
                .into_iter()
                .filter(|c| !kills.iter().any(|l| same(*c, *l)))
                .collect();
            if sequence {
                let after = self.checked_after(child);
                self.checked.extend(after);
            }
        }
    }

    fn exprent(&mut self, expr: Exprent) {
        self.visit(expr);
    }

    /// Visits `e` and what it contains in evaluation order: the second
    /// operand of `&&`/`||` and the branches of `?:` with what the first
    /// operand found non-null.
    fn visit(&mut self, e: Exprent) {
        if let Some(func) = e.try_as_function() {
            let ops = func.get_lst_operands();
            let guarded: Option<Vec<(Exprent, Vec<Exprent>)>> = match func.get_func_type() {
                FunctionType::BooleanAnd | FunctionType::BooleanOr if ops.len() == 2 => {
                    let and = func.get_func_type() == FunctionType::BooleanAnd;
                    Some(vec![(ops.get(1), non_null_when(ops.get(0), and))])
                }
                FunctionType::Ternary if ops.len() == 3 => Some(vec![
                    (ops.get(1), non_null_when(ops.get(0), true)),
                    (ops.get(2), non_null_when(ops.get(0), false)),
                ]),
                _ => None,
            };
            if let Some(guarded) = guarded {
                self.visit(ops.get(0));
                for (operand, checks) in guarded {
                    let saved = self.checked.clone();
                    let mark = self.kills.len();
                    self.checked.extend(checks);
                    self.visit(operand);
                    let kills = &self.kills[mark..];
                    self.checked = saved
                        .into_iter()
                        .filter(|c| !kills.iter().any(|l| same(*c, *l)))
                        .collect();
                }
                return;
            }
        }
        if let Some(receiver) = dereferenced(e) {
            self.derefs.push(receiver);
        }
        for child in e.get_all_exprents() {
            self.visit(child);
        }
        self.record(e);
    }

    /// Records `e` itself: a cast, a definition, or a store.
    fn record(&mut self, e: Exprent) {
        if let Some(cast) = e.try_as_k_function() {
            if cast.get_func_type() == FunctionType::Cast && cast.does_cast() {
                self.casts.push(Store {
                    var: -1,
                    value: e,
                    checked: self.checked.clone(),
                });
            }
            return;
        }
        if let Some(k) = e.try_as_k_var() {
            if k.kd(|d| d.declaration_type) == Some(DeclarationType::Definition) {
                self.definitions.entry(k.get_index()).or_default().push(e);
            } else if !self.checked.iter().any(|c| same(*c, e)) {
                self.uses.push(e);
            }
            return;
        }
        let Some(assign) = e.try_as_assignment() else {
            return;
        };
        let left = assign.get_left();
        if assign.get_cond_type().is_none() {
            if let Some(var) = left.try_as_var() {
                self.stores.push(Store {
                    var: var.get_index(),
                    value: assign.get_right(),
                    checked: self.checked.clone(),
                });
            }
        }
        self.kills.push(left);
        self.checked.retain(|c| !same(*c, left));
    }

    /// What `stat`, a statement of a sequence, leaves non-null for the
    /// statements after it: `v` of `if (v == null || ..) { return }` (or of
    /// `if (v != null && ..) {..} else { return }`), and `v` of
    /// `if (v == null) { ..; v = e }`, `e` non-null.
    fn checked_after(&self, stat: Statement) -> Vec<Exprent> {
        let Some(ifstat) = stat.try_as_if() else {
            return Vec::new();
        };
        let cond = ifstat.get_headexprent().get_condition();
        let mut res = Vec::new();
        let then_jumps = match ifstat.get_ifstat() {
            Some(body) => jumps_away(body),
            None => ifstat.get_if_edge().is_some_and(|e| {
                e.explicit()
                    && (e.get_type() == StatEdge::TYPE_BREAK
                        || e.get_type() == StatEdge::TYPE_CONTINUE)
            }),
        };
        if then_jumps {
            res.extend(non_null_when(cond, false));
        } else if ifstat.iftype() == IfStatement::IFTYPE_IF {
            if let (Some(body), Some(v)) = (
                ifstat.get_ifstat(),
                non_null_operand(cond, FunctionType::Eq),
            ) {
                if v.is_var() && self.ends_storing_non_null(body, v) {
                    res.push(v);
                }
            }
        }
        if ifstat.iftype() == IfStatement::IFTYPE_IFELSE
            && ifstat.get_elsestat().is_some_and(jumps_away)
        {
            res.extend(non_null_when(cond, true));
        }
        res
    }

    /// Whether the last store into the local `v` in the last block of `body`
    /// stores a value that cannot be null.
    fn ends_storing_non_null(&self, body: Statement, v: Exprent) -> bool {
        let last = last_statement(body);
        let Some(exprents) = last.get_exprents() else {
            return false;
        };
        let store = exprents.to_vec().into_iter().rev().find_map(|e| {
            e.try_as_assignment()
                .filter(|a| a.get_cond_type().is_none() && same(a.get_left(), v))
        });
        store.is_some_and(|a| {
            may_be_null(a.get_right(), &[], &BTreeSet::new(), self.params) == Some(false)
        })
    }
}

/// What `e` dereferences at once: the receiver of a call or a field read, the
/// array of an element read or of `.size`. A cast there stays `as T`: on
/// `null` it throws where the dereference would.
fn dereferenced(e: Exprent) -> Option<Exprent> {
    match e.get_type() {
        ExprentType::Invocation => e.as_invocation().get_instance(),
        ExprentType::Field => e.as_field().get_instance(),
        ExprentType::Array => Some(e.as_array().get_array()),
        ExprentType::Function => {
            let func = e.as_function();
            (func.get_func_type() == FunctionType::ArrayLength)
                .then(|| func.get_lst_operands().get(0))
        }
        _ => None,
    }
}

/// The left sides of the assignments under `stat`.
fn stores_in(stat: Statement, out: &mut Vec<Exprent>) {
    let mut lists: Vec<Exprent> = stat.get_stat_exprents();
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    for expr in lists {
        for e in expr.get_all_exprents_rec_self(true, true) {
            if let Some(assign) = e.try_as_assignment() {
                out.push(assign.get_left());
            }
        }
    }
    for child in stat.get_stats().borrow().to_vec() {
        stores_in(child, out);
    }
}

/// Whether `a` and `b` are the same local or the same field.
fn same(a: Exprent, b: Exprent) -> bool {
    match (a.try_as_var(), b.try_as_var()) {
        (Some(x), Some(y)) => x.get_index() == y.get_index(),
        (None, None) => a.get_type() == ExprentType::Field && a.equals(b),
        _ => false,
    }
}

/// The children of `stat` its condition makes run with a value non-null:
/// the `then` of `if (v != null && ..)`, the `else` of `if (v == null || ..)`,
/// the body of `while (v != null && ..)`.
fn guards(stat: Statement) -> Vec<(Statement, Exprent)> {
    let mut res = Vec::new();
    if let Some(ifstat) = stat.try_as_if() {
        let cond = ifstat.get_headexprent().get_condition();
        if let Some(body) = ifstat.get_ifstat() {
            res.extend(non_null_when(cond, true).into_iter().map(|v| (body, v)));
        }
        if let Some(body) = ifstat.get_elsestat() {
            res.extend(non_null_when(cond, false).into_iter().map(|v| (body, v)));
        }
    } else if let Some(dostat) = stat.try_as_do() {
        if matches!(dostat.get_looptype(), DoType::While | DoType::For) {
            if let Some(cond) = dostat.get_condition_exprent() {
                for body in stat.get_stats().borrow().to_vec() {
                    res.extend(non_null_when(cond, true).into_iter().map(|v| (body, v)));
                }
            }
        }
    }
    res
}

/// The statement `stat` ends with: the last of a sequence, recursively.
fn last_statement(stat: Statement) -> Statement {
    let mut last = stat;
    while last.type_() == StatementType::Sequence {
        let Some(l) = last.get_stats().borrow().to_vec().last().copied() else {
            break;
        };
        last = l;
    }
    last
}

/// Whether `stat` never completes normally: it ends in a `return`, `throw`,
/// `break` or `continue`.
fn jumps_away(stat: Statement) -> bool {
    let last = last_statement(stat);
    if last.type_() != StatementType::BasicBlock {
        return false;
    }
    if let Some(exprents) = last.get_exprents() {
        if !exprents.is_empty() && exprents.get(exprents.len() - 1).is_exit() {
            return true;
        }
    }
    last.get_all_successor_edges().iter().any(|e| {
        (e.get_type() == StatEdge::TYPE_BREAK || e.get_type() == StatEdge::TYPE_CONTINUE)
            && e.explicit()
    })
}

/// The values `cond` tests non-null when it is `holds`: `v` of `v != null`
/// (and of the operands of `&&`) when it holds, of `v == null` (and of the
/// operands of `||`) when it does not.
fn non_null_when(cond: Exprent, holds: bool) -> Vec<Exprent> {
    let Some(func) = cond.try_as_function() else {
        return Vec::new();
    };
    let ops = func.get_lst_operands();
    match func.get_func_type() {
        FunctionType::BooleanAnd if holds => {
            let mut res = non_null_when(ops.get(0), true);
            res.extend(non_null_when(ops.get(1), true));
            res
        }
        FunctionType::BooleanOr if !holds => {
            let mut res = non_null_when(ops.get(0), false);
            res.extend(non_null_when(ops.get(1), false));
            res
        }
        FunctionType::BoolNot if ops.len() == 1 => non_null_when(ops.get(0), !holds),
        FunctionType::Instanceof if holds => vec![ops.get(0)],
        FunctionType::Ne if holds => non_null_operand(cond, FunctionType::Ne)
            .into_iter()
            .collect(),
        FunctionType::Eq if !holds => non_null_operand(cond, FunctionType::Eq)
            .into_iter()
            .collect(),
        _ => Vec::new(),
    }
}

/// `v` of `v != null` (`ne`) or `v == null`, `v` not the constant; `x` for
/// a cast `x as T`, which is null exactly when `x` is.
fn non_null_operand(cond: Exprent, ne: FunctionType) -> Option<Exprent> {
    let func = cond.try_as_function().filter(|f| f.get_func_type() == ne)?;
    let ops = func.get_lst_operands();
    if ops.len() != 2 {
        return None;
    }
    let is_null = |e: Exprent| e.try_as_const().is_some_and(|c| c.is_null());
    let mut value = match (is_null(ops.get(0)), is_null(ops.get(1))) {
        (false, true) => ops.get(0),
        (true, false) => ops.get(1),
        _ => return None,
    };
    while let Some(cast) = value
        .try_as_function()
        .filter(|f| f.get_func_type() == FunctionType::Cast)
    {
        value = cast.get_lst_operands().get(0);
    }
    Some(value)
}

/// Whether `value` may be null: `Some(true)` it may, `Some(false)` it cannot,
/// `None` unknown (a Java platform type, an array element).
fn may_be_null(
    value: Exprent,
    checked: &[Exprent],
    nullable: &BTreeSet<i32>,
    params: &BTreeMap<i32, bool>,
) -> Option<bool> {
    if checked.iter().any(|c| same(*c, value)) {
        return Some(false);
    }
    if kuna_fixes::enabled(KunaFix::KtRawGenericType)
        && crate::kotlin::kuna_fix_kt_raw_generic_type::reads_nullable_element(value)
    {
        return Some(true);
    }
    match value.get_type() {
        ExprentType::Const => Some(value.as_const().is_null()),
        ExprentType::New => Some(false),
        ExprentType::Var => {
            let index = value.as_var().get_index();
            if nullable.contains(&index) {
                Some(true)
            } else {
                params.get(&index).copied()
            }
        }
        ExprentType::Field => {
            let field = value.as_field();
            let (owner, name, desc) = (
                field.get_classname(),
                field.get_name(),
                field.get_descriptor().descriptor_string.clone(),
            );
            looked_up(true, &owner, &name, &desc, || {
                field_nullability(&owner, &name, &desc)
            })
        }
        ExprentType::Invocation => {
            let inv = value.as_invocation();
            if inv.get_name() == "<init>" {
                return None;
            }
            let (owner, name, desc) = (
                inv.get_classname(),
                inv.get_name(),
                inv.get_string_descriptor(),
            );
            looked_up(false, &owner, &name, &desc, || {
                method_nullability(&owner, &name, &desc)
            })
        }
        ExprentType::Assignment => {
            may_be_null(value.as_assignment().get_right(), checked, nullable, params)
        }
        ExprentType::Function => {
            if let Some(k) = value.try_as_k_function() {
                match k.kd(|d| d.k_type) {
                    KFunctionType::NotNull => return Some(false),
                    KFunctionType::IfNull => {
                        let ops = k.get_lst_operands();
                        return may_be_null(ops.get(1), checked, nullable, params);
                    }
                    _ => {}
                }
            }
            let func = value.as_function();
            let ops = func.get_lst_operands();
            match func.get_func_type() {
                FunctionType::Cast => may_be_null(ops.get(0), checked, nullable, params),
                FunctionType::Ternary => {
                    let cond = ops.get(0);
                    let mut then_checked = checked.to_vec();
                    then_checked.extend(non_null_when(cond, true));
                    let mut else_checked = checked.to_vec();
                    else_checked.extend(non_null_when(cond, false));
                    let a = may_be_null(ops.get(1), &then_checked, nullable, params);
                    let b = may_be_null(ops.get(2), &else_checked, nullable, params);
                    match (a, b) {
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        (Some(false), Some(false)) => Some(false),
                        _ => None,
                    }
                }
                FunctionType::Other => None,
                _ => Some(false),
            }
        }
        _ => None,
    }
}

/// Whether `value` may be null only through a local it reads (what
/// [`may_be_null`] answers from the locals found nullable so far).
fn reads_local(value: Exprent) -> bool {
    match value.get_type() {
        ExprentType::Var => true,
        ExprentType::Assignment => reads_local(value.as_assignment().get_right()),
        ExprentType::Function => {
            let ops = value.as_function().get_lst_operands();
            match value.as_function().get_func_type() {
                FunctionType::Cast => reads_local(ops.get(0)),
                FunctionType::Ternary => reads_local(ops.get(1)) || reads_local(ops.get(2)),
                FunctionType::Other => {
                    value
                        .try_as_k_function()
                        .is_some_and(|k| k.kd(|d| d.k_type) == KFunctionType::IfNull)
                        && reads_local(ops.get(1))
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// A member: whether it is a field, its owner, name and descriptor.
type MemberKey = (bool, String, String, String);

thread_local! {
    static LOOKUPS: RefCell<BTreeMap<MemberKey, Option<bool>>> = const { RefCell::new(BTreeMap::new()) };
}

/// `f()`, the nullability of the field (`field`) or method `owner.name desc`,
/// looked up once per method.
fn looked_up(
    field: bool,
    owner: &str,
    name: &str,
    desc: &str,
    f: impl FnOnce() -> Option<bool>,
) -> Option<bool> {
    let key = (field, owner.to_string(), name.to_string(), desc.to_string());
    if let Some(res) = LOOKUPS.with(|l| l.borrow().get(&key).copied()) {
        return res;
    }
    let res = f();
    LOOKUPS.with(|l| l.borrow_mut().insert(key, res));
    res
}

/// The Kotlin nullability of the parameters of `method`, by variable index.
fn parameter_nullability(method: StructMethod) -> BTreeMap<i32, bool> {
    let mut res = BTreeMap::new();
    let (receiver, types): (Option<Option<bool>>, Vec<Option<bool>>) =
        match KtAttrs::get(method, &KElement::KEY()) {
            Some(KElement::Function(f)) => (
                f.receiver_type
                    .as_ref()
                    .map(|t| type_nullability(t, &f.type_parameters)),
                f.parameters
                    .iter()
                    .map(|p| type_nullability(&p.type_, &f.type_parameters))
                    .collect(),
            ),
            Some(KElement::Constructor(c))
                if c.parameters.len() == method.method_descriptor().params.len() =>
            {
                (
                    None,
                    c.parameters
                        .iter()
                        .map(|p| type_nullability(&p.type_, &[]))
                        .collect(),
                )
            }
            _ => return res,
        };
    let md = method.method_descriptor();
    let kotlin: Vec<Option<bool>> = receiver.into_iter().chain(types).collect();
    if kotlin.len() > md.params.len() {
        return res;
    }
    let mut slot = if method.has_modifier(ACC_STATIC) {
        0
    } else {
        res.insert(0, false);
        1
    };
    for (i, nullable) in kotlin.into_iter().enumerate() {
        if let Some(nullable) = nullable {
            res.insert(slot, nullable);
        }
        slot += md.params[i].stack_size;
    }
    res
}

/// Whether a value of the Kotlin type `t` may be null: `T?`; a type parameter of
/// `params` (the function's) without a non-null bound; `None` for a type
/// parameter of the class, whose bounds are not at hand.
fn type_nullability(t: &KType, params: &[KTypeParameter]) -> Option<bool> {
    let k = t.k();
    if k.is_nullable {
        return Some(true);
    }
    let Some(name) = &k.type_parameter_name else {
        return Some(false);
    };
    let param = params.iter().find(|p| &p.name == name)?;
    Some(param.upper_bounds.iter().all(|b| b.k().is_nullable))
}

/// `Some(true)` for `@Nullable`, `Some(false)` for `@NotNull`.
fn annotated(mb: StructMember) -> Option<bool> {
    for key in KotlinWriter::ANNOTATION_ATTRIBUTES {
        if let Some(attribute) = mb.get_attribute(key) {
            for annotation in attribute.get_annotations().iter() {
                match annotation.get_class_name().as_ref() {
                    NULLABLE => return Some(true),
                    NOT_NULL => return Some(false),
                    _ => {}
                }
            }
        }
    }
    None
}

fn class(name: &str) -> Option<StructClass> {
    DecompilerContext::get_struct_context().get_class(name)
}

/// The Kotlin nullability of the field `owner.name`.
fn field_nullability(owner: &str, name: &str, desc: &str) -> Option<bool> {
    if !desc.starts_with('L') && !desc.starts_with('[') {
        return Some(false);
    }
    let cl = class(owner)?;
    let field = cl.get_field(name, desc)?;
    if let Some(res) = annotated(field.into()) {
        return Some(res);
    }
    KotlinChooser::parse_metadata_for(cl);
    match KtAttrs::get(field, &KElement::KEY()) {
        Some(KElement::Property(p)) => p.type_.as_ref().map(|t| t.k().is_nullable),
        _ => None,
    }
}

/// The Kotlin nullability of what the method `owner.name desc` returns,
/// looked up in `owner` and its supertypes.
fn method_nullability(owner: &str, name: &str, desc: &str) -> Option<bool> {
    let ret = &desc[desc.find(')')? + 1..];
    if !ret.starts_with('L') && !ret.starts_with('[') {
        return Some(false);
    }
    let mut pending = vec![owner.to_string()];
    let mut seen: BTreeSet<String> = BTreeSet::new();
    while let Some(name_) = pending.pop() {
        if !seen.insert(name_.clone()) || seen.len() > 32 {
            continue;
        }
        if JDK_MAPS.contains(&name_.as_str()) {
            return MAP_LOOKUPS.contains(&(name, desc)).then_some(true);
        }
        let Some(cl) = class(&name_) else {
            continue;
        };
        if let Some(method) = cl.get_method(name, desc) {
            let suspend = kuna_fix_kt_suspend_call_continuation::in_body()
                && desc.ends_with("Lkotlin/coroutines/Continuation;)Ljava/lang/Object;")
                && {
                    KotlinChooser::parse_metadata_for(cl);
                    matches!(KtAttrs::get(method, &KElement::KEY()),
                        Some(KElement::Function(f)) if Flags::IS_SUSPEND.get(f.flags))
                };
            if let Some(res) = annotated(method.into()).filter(|_| !suspend) {
                return Some(res);
            }
            KotlinChooser::parse_metadata_for(cl);
            let element = KtAttrs::get(method, &KElement::KEY());
            return match element {
                Some(KElement::Function(f)) => {
                    let k = f.return_type.k();
                    (k.is_nullable || k.type_parameter_name.is_none()).then_some(k.is_nullable)
                }
                Some(KElement::Property(p)) => {
                    if KtAttrs::get(method, &PropertyMethod::KEY()) == Some(PropertyMethod::Getter)
                    {
                        p.type_.as_ref().map(|t| t.k().is_nullable)
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }
        pending.extend(cl.get_interface_names());
        if let Some(sup) = cl.super_class() {
            pending.push(sup.get_string().to_string());
        }
    }
    None
}

/// `written`, a type, made nullable: `T?`, `(A) -> R` in parentheses.
pub fn nullable(written: &str) -> String {
    if written.ends_with('?') {
        written.to_string()
    } else if written.contains("->") {
        format!("({written})?")
    } else {
        format!("{written}?")
    }
}

/// Fix `kt_nullable_cast`: whether `operand` reads a local that may hold `null`
/// there (no null check of the method guards the read).
pub fn may_read_null(operand: Exprent) -> bool {
    kuna_fixes::enabled(KunaFix::KtNullableCast)
        && operand.try_as_k_var().is_some_and(|k| {
            k.kd(|d| d.nullable && d.declaration_type != Some(DeclarationType::Definition))
        })
}
