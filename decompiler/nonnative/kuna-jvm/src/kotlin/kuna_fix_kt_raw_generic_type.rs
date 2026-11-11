//! kuna fix `kt_raw_generic_type` (spec 10.16): in Kotlin output, a generic
//! class is written with type arguments.
//!
//! Kotlin has no raw types, but Vineflower writes a local's declared type
//! from its JVM type, and kotlinc writes no LocalVariableTypeTable: `val
//! pins: java.util.List = findMatchingPins(host)`, `val i: java.util.Iterator
//! = set.iterator()`, `val ref: ObjectRef = ObjectRef()`. A type test and a
//! cast of a generic class are written raw too (`x is java.util.Collection`).
//! kotlinc rejects each ("one type argument expected"), and every member
//! read through such a local after it (`i.next()`, `ref.element`).
//!
//! With the fix the Kotlin plugin types each local whose class is generic
//! (a JDK or Kotlin class it knows, or a class whose `Signature` attribute
//! declares type parameters; function types aside) with the arguments the
//! class file proves: those of the values stored into it (the callee's
//! generic signature, as Vineflower infers a call's type, while the method's
//! parameters and its other typed locals have their generic types), else of
//! how it is used (the parameter and return types and the typed locals it
//! is passed to, then the nearest common class of the elements `add`ed or
//! `put` into it, or the class a read of an element is cast to). A type
//! variable of a callee is not the method's, and a generic method's result
//! Vineflower types `Object` proves nothing. A local whose arguments nothing
//! proves, which is not constructed, passed or written through (a call that
//! takes one of its class's type parameters), is declared with star
//! projections, `Iterator<*>`, which only reads; one whose evidence
//! disagrees stays raw. A `Ref.ObjectRef` (a captured `var`) is
//! `ObjectRef<Any?>`: kotlinc casts each read of its `element` to the
//! variable's type where that is narrower, and the casts are written. A
//! Kotlin collection interface is declared read-only (`List<T>`) unless the
//! method mutates through the local, a view of it or a copy (`add`, an
//! iterator's `remove`), as `MutableList<T>`.
//!
//! A cast of a raw generic class is written with the arguments of the local
//! it is stored into or reads, of the method's return type it is returned
//! as, or of its operand's generic type, else with star projections; a type
//! test always with star projections; both by Kotlin's read-only names
//! (`x is Collection<*>`, also for a generic cast Vineflower writes
//! `MutableMap<K, V>`), which compile to the same `instanceof`/`checkcast`
//! (the `Mutable` names call `TypeIntrinsics`). A cast the typed locals make
//! redundant is left out: of `null` or of a constructor call stored into a
//! declaration (kotlinc's checkcast after `mutableListOf()`), and of an
//! element read from a typed local to the element's class (kotlinc's
//! checkcast of a generic result, which the typed read compiles to again).
//! Vineflower's own inference sees the types only while the locals are
//! typed (a `Ref`'s always, so a store into its `element` needs no cast to
//! `T`), so its casts elsewhere are unchanged. Type arguments do not change
//! what the code does at run time.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::generic_type::GenericType;
use crate::k_var_exprent::DeclarationType;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::var_processor::{KunaLocalType, VarProcessor};
use crate::var_type::VarType;

/// The JDK and Kotlin classes Kotlin source names with type arguments, and
/// their arities; a class of the input (or a library) answers from its
/// `Signature` attribute.
const KNOWN_GENERIC: &[(&str, usize)] = &[
    ("java/lang/Iterable", 1),
    ("java/lang/Comparable", 1),
    ("java/lang/Class", 1),
    ("java/lang/ThreadLocal", 1),
    ("java/lang/ref/Reference", 1),
    ("java/lang/ref/WeakReference", 1),
    ("java/lang/ref/SoftReference", 1),
    ("java/util/Collection", 1),
    ("java/util/List", 1),
    ("java/util/Set", 1),
    ("java/util/SortedSet", 1),
    ("java/util/NavigableSet", 1),
    ("java/util/Queue", 1),
    ("java/util/Deque", 1),
    ("java/util/Iterator", 1),
    ("java/util/ListIterator", 1),
    ("java/util/Enumeration", 1),
    ("java/util/Comparator", 1),
    ("java/util/Optional", 1),
    ("java/util/ArrayList", 1),
    ("java/util/LinkedList", 1),
    ("java/util/ArrayDeque", 1),
    ("java/util/HashSet", 1),
    ("java/util/LinkedHashSet", 1),
    ("java/util/TreeSet", 1),
    ("java/util/PriorityQueue", 1),
    ("java/util/Map", 2),
    ("java/util/SortedMap", 2),
    ("java/util/NavigableMap", 2),
    ("java/util/HashMap", 2),
    ("java/util/LinkedHashMap", 2),
    ("java/util/TreeMap", 2),
    ("java/util/WeakHashMap", 2),
    ("java/util/IdentityHashMap", 2),
    ("java/util/Map$Entry", 2),
    ("java/util/concurrent/ConcurrentHashMap", 2),
    ("java/util/concurrent/ConcurrentMap", 2),
    ("java/util/concurrent/Future", 1),
    ("java/util/concurrent/Callable", 1),
    ("java/util/concurrent/atomic/AtomicReference", 1),
    ("kotlin/jvm/internal/Ref$ObjectRef", 1),
    ("kotlin/Pair", 2),
    ("kotlin/Triple", 3),
    ("kotlin/Lazy", 1),
    ("kotlin/sequences/Sequence", 1),
];

/// The JDK collection classes whose type parameters are those of the
/// collection interfaces they implement, position for position.
const COLLECTION_SUPERS: &[(&str, &[&str])] = &[
    ("java/util/Collection", &["java/lang/Iterable"]),
    ("java/util/List", &["java/util/Collection"]),
    ("java/util/Set", &["java/util/Collection"]),
    ("java/util/SortedSet", &["java/util/Set"]),
    ("java/util/NavigableSet", &["java/util/SortedSet"]),
    ("java/util/Queue", &["java/util/Collection"]),
    ("java/util/Deque", &["java/util/Queue"]),
    ("java/util/ListIterator", &["java/util/Iterator"]),
    ("java/util/ArrayList", &["java/util/List"]),
    (
        "java/util/LinkedList",
        &["java/util/List", "java/util/Deque"],
    ),
    ("java/util/ArrayDeque", &["java/util/Deque"]),
    ("java/util/HashSet", &["java/util/Set"]),
    ("java/util/LinkedHashSet", &["java/util/HashSet"]),
    ("java/util/TreeSet", &["java/util/NavigableSet"]),
    ("java/util/PriorityQueue", &["java/util/Queue"]),
    ("java/util/SortedMap", &["java/util/Map"]),
    ("java/util/NavigableMap", &["java/util/SortedMap"]),
    ("java/util/HashMap", &["java/util/Map"]),
    ("java/util/LinkedHashMap", &["java/util/HashMap"]),
    ("java/util/TreeMap", &["java/util/NavigableMap"]),
    ("java/util/WeakHashMap", &["java/util/Map"]),
    ("java/util/IdentityHashMap", &["java/util/Map"]),
    ("java/util/concurrent/ConcurrentMap", &["java/util/Map"]),
    (
        "java/util/concurrent/ConcurrentHashMap",
        &["java/util/concurrent/ConcurrentMap"],
    ),
    ("java/lang/ref/WeakReference", &["java/lang/ref/Reference"]),
    ("java/lang/ref/SoftReference", &["java/lang/ref/Reference"]),
];

/// Kotlin's read-only and mutable names of the JDK interfaces it maps to its
/// own (`Comparable` has one name).
const MAPPED: &[(&str, &str, &str)] = &[
    ("java/lang/Comparable", "Comparable", "Comparable"),
    ("java/lang/Iterable", "Iterable", "MutableIterable"),
    ("java/util/Collection", "Collection", "MutableCollection"),
    ("java/util/List", "List", "MutableList"),
    ("java/util/Set", "Set", "MutableSet"),
    ("java/util/Map", "Map", "MutableMap"),
    ("java/util/Iterator", "Iterator", "MutableIterator"),
    (
        "java/util/ListIterator",
        "ListIterator",
        "MutableListIterator",
    ),
    (
        "java/util/Map$Entry",
        "Map.Entry",
        "MutableMap.MutableEntry",
    ),
];

/// The members of the mapped interfaces that only their `Mutable` types
/// have.
const MUTATORS: &[&str] = &[
    "add",
    "addAll",
    "remove",
    "removeAll",
    "retainAll",
    "removeIf",
    "clear",
    "set",
    "put",
    "putAll",
    "putIfAbsent",
    "compute",
    "computeIfAbsent",
    "computeIfPresent",
    "merge",
    "replace",
    "replaceAll",
    "sort",
    "setValue",
];

/// The calls that store an element of a collection or map: name, the
/// argument holding the element and the type argument it stands for.
const ELEMENT_STORES: &[(&str, &str, usize, usize)] = &[
    ("add", "(Ljava/lang/Object;)Z", 0, 0),
    ("add", "(ILjava/lang/Object;)V", 1, 0),
    ("addFirst", "(Ljava/lang/Object;)V", 0, 0),
    ("addLast", "(Ljava/lang/Object;)V", 0, 0),
    ("offer", "(Ljava/lang/Object;)Z", 0, 0),
    ("push", "(Ljava/lang/Object;)V", 0, 0),
    ("set", "(ILjava/lang/Object;)Ljava/lang/Object;", 1, 0),
    (
        "put",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        0,
        0,
    ),
    (
        "put",
        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        1,
        1,
    ),
];

/// The calls whose result is an element, by the type argument it is.
const ELEMENT_READS: &[(&str, &str, usize)] = &[
    ("next", "()Ljava/lang/Object;", 0),
    ("previous", "()Ljava/lang/Object;", 0),
    ("get", "(I)Ljava/lang/Object;", 0),
    ("remove", "(I)Ljava/lang/Object;", 0),
    ("removeFirst", "()Ljava/lang/Object;", 0),
    ("removeLast", "()Ljava/lang/Object;", 0),
    ("poll", "()Ljava/lang/Object;", 0),
    ("peek", "()Ljava/lang/Object;", 0),
    ("first", "()Ljava/lang/Object;", 0),
    ("last", "()Ljava/lang/Object;", 0),
    ("getFirst", "()Ljava/lang/Object;", 0),
    ("getLast", "()Ljava/lang/Object;", 0),
    ("getKey", "()Ljava/lang/Object;", 0),
    ("getValue", "()Ljava/lang/Object;", 1),
];

/// The calls that give a view of the receiver through which the receiver
/// is mutated when the view is.
const VIEWS: &[&str] = &[
    "iterator",
    "listIterator",
    "entrySet",
    "keySet",
    "values",
    "subList",
];

const REF: &str = "kotlin/jvm/internal/Ref$ObjectRef";

/// What a method body does with a local.
#[derive(Default)]
struct Local {
    /// Its declared type, if a definition is written.
    declared: Option<VarType>,
    /// The values stored into it.
    stores: Vec<Exprent>,
    /// The values its type arguments hold, by argument.
    elements: Vec<(usize, Exprent)>,
    /// The classes an element read from it is cast to, by argument.
    element_casts: Vec<(usize, VarType)>,
    /// The parameter and return types it is passed to.
    sinks: Vec<VarType>,
    /// The locals a view of it is stored into.
    views: Vec<i32>,
    /// The locals it is stored into.
    copies: Vec<i32>,
    /// A member only the `Mutable` types have is called on it.
    mutated: bool,
    /// A value is stored through it (an element, a `Ref`'s value).
    written: bool,
    /// A value stored into it is a constructor call, whose type arguments
    /// Kotlin infers from the declaration.
    constructed: bool,
    /// It is passed to a method.
    passed: bool,
    /// A method that takes an argument whose type may be one of the class's
    /// type parameters is called on it (a star projection would make it
    /// uncallable).
    written_by_call: bool,
}

thread_local! {
    static INFERRING: Cell<bool> = const { Cell::new(false) };
}

struct Inferring(bool);

impl Drop for Inferring {
    fn drop(&mut self) {
        INFERRING.with(|i| i.set(self.0));
    }
}

/// Whether Vineflower's own inference sees the type `t` given to a local:
/// while the locals are being typed, so that one local's type carries into
/// the values stored into others (`list.iterator()`); after, a `Ref`'s only,
/// so that its `element` has the type the local's argument names. Vineflower
/// writes the other locals as before (its casts of generic values would
/// otherwise change).
pub fn visible(t: &VarType) -> bool {
    INFERRING.with(|i| i.get()) || t.value.as_deref() == Some(REF)
}

/// Types the locals under `root` whose declared class is generic (see the
/// module header); the types live in `processor` until the method is written.
pub fn infer(root: RootStatement, method: StructMethod, processor: VarProcessor) {
    processor.clear_kuna_typed();
    let _inferring = Inferring(INFERRING.with(|i| i.replace(true)));
    let mut walk = Walk {
        locals: BTreeMap::new(),
        casts: Vec::new(),
        read_casts: Vec::new(),
        return_type: method.get_signature().and_then(|s| s.return_type()),
    };
    walk.stat(root.0);
    let scope = type_parameters_in_scope(method);
    for (slot, t) in parameter_types(method) {
        processor.set_kuna_typed(slot, local_type(t, false, Vec::new()));
    }
    let candidates: Vec<i32> = walk
        .locals
        .iter()
        .filter(|(_, l)| l.declared.as_ref().is_some_and(|d| arity(d).is_some()))
        .map(|(i, _)| *i)
        .collect();
    if candidates.is_empty() {
        return;
    }
    let mut typed: BTreeMap<i32, VarType> = BTreeMap::new();
    for _ in 0..8 {
        let mut changed = false;
        for index in &candidates {
            if typed.contains_key(index) {
                continue;
            }
            let local = &walk.locals[index];
            let declared = local.declared.clone().expect("a candidate's declared type");
            let copied: Vec<VarType> = local
                .copies
                .iter()
                .filter_map(|c| typed.get(c).cloned())
                .collect();
            if let Some(t) = evidence(local, &declared, &scope, &copied) {
                processor.set_kuna_typed(*index, local_type(t.clone(), false, Vec::new()));
                typed.insert(*index, t);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for index in &candidates {
        let local = &walk.locals[index];
        if typed.contains_key(index)
            || local.written
            || local.mutated
            || local.constructed
            || local.passed
            || local.written_by_call
        {
            continue;
        }
        let declared = local.declared.clone().expect("a candidate's declared type");
        let n = arity(&declared).expect("a generic class");
        typed.insert(*index, with_arguments(&declared, vec![None; n]));
    }
    let mut mutable: BTreeSet<i32> = walk
        .locals
        .iter()
        .filter(|(_, l)| l.mutated)
        .map(|(i, _)| *i)
        .collect();
    loop {
        let before = mutable.len();
        for (index, local) in &walk.locals {
            if local
                .views
                .iter()
                .chain(&local.copies)
                .any(|v| mutable.contains(v))
            {
                mutable.insert(*index);
            }
        }
        if mutable.len() == before {
            break;
        }
    }
    let mut nullable: BTreeMap<i32, Vec<bool>> = BTreeMap::new();
    for (index, t) in &typed {
        let local = &walk.locals[index];
        let n = t.try_as_generic().map_or(0, |g| g.get_arguments().len());
        let flags = (0..n)
            .map(|i| {
                local.elements.iter().any(|(slot, v)| {
                    *slot == i
                        && through_casts(*v)
                            .try_as_const()
                            .is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
                })
            })
            .collect();
        nullable.insert(*index, flags);
    }
    loop {
        let mut changed = false;
        for (index, local) in &walk.locals {
            for copy in &local.copies {
                let (Some(a), Some(b)) = (typed.get(index), typed.get(copy)) else {
                    continue;
                };
                if a.value != b.value || nullable[index].len() != nullable[copy].len() {
                    continue;
                }
                let joined: Vec<bool> = nullable[index]
                    .iter()
                    .zip(&nullable[copy])
                    .map(|(x, y)| *x || *y)
                    .collect();
                if joined != nullable[index] || joined != nullable[copy] {
                    nullable.insert(*index, joined.clone());
                    nullable.insert(*copy, joined);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    for (index, t) in &typed {
        let flags = nullable.remove(index).unwrap_or_default();
        processor.set_kuna_typed(
            *index,
            local_type(t.clone(), mutable.contains(index), flags),
        );
    }
    for (cast, local) in walk.casts {
        cast.as_k_function()
            .kd_mut(|d| d.kuna_typed_from = Some(local));
    }
    for (cast, index, slot) in walk.read_casts {
        let Some(t) = typed.get(&index) else {
            continue;
        };
        let arg = t
            .try_as_generic()
            .and_then(|g| g.get_arguments().get(slot).cloned().flatten());
        let to = cast.as_function().get_lst_operands().get(1).get_expr_type();
        if let Some(cast) = cast.try_as_k_function() {
            if arg.is_some_and(|a| {
                a.type_ == CodeType::Object
                    && !a.is_generic()
                    && a.value == to.value
                    && to.array_dim == 0
            }) {
                cast.kd_mut(|d| d.redundant_cast = true);
            }
        }
    }
}

fn local_type(type_: VarType, mutable: bool, nullable: Vec<bool>) -> KunaLocalType {
    KunaLocalType {
        type_,
        mutable,
        nullable,
    }
}

/// Fix `kt_raw_generic_type`: the class of the cast `cast` to `to`, a generic
/// class written without arguments, with the arguments of the typed local
/// whose value it is or of the method's return type when it is returned,
/// else of its operand (a typed local, or a value of a generic type), when
/// the cast's class is that type's or a JDK collection class with the same
/// parameters (Kotlin infers omitted cast arguments from the operand
/// alike); the arguments do not change the `checkcast`.
pub fn narrowed(cast: crate::k_function_exprent::KFunctionExprent, to: &VarType) -> Option<String> {
    let to = &VarType::new_value(CodeType::Object, 0, to.value_str());
    let n = arity(to)?;
    let (typed, nullable) = match cast.kd(|d| d.kuna_typed_from) {
        None if cast.kd(|d| d.kuna_cast_target.is_some()) => (
            cast.kd(|d| d.kuna_cast_target.clone()).expect("a target"),
            Vec::new(),
        ),
        Some(local) => {
            let declared = local.try_as_var()?.kuna_declared()?;
            (declared.type_, declared.nullable)
        }
        None => {
            let operand = through_casts(cast.get_lst_operands().get(0));
            match operand.try_as_var().and_then(|v| v.kuna_declared()) {
                Some(declared) => (declared.type_, declared.nullable),
                None => {
                    let t = operand.get_inferred_expr_type(None);
                    if names_type_variable(&t, &BTreeSet::new()) {
                        return None;
                    }
                    (t, Vec::new())
                }
            }
        }
    };
    let args = typed.try_as_generic()?.get_arguments().to_vec();
    let related = to.value == typed.value
        || collection_super(to.value_str(), typed.value_str())
        || collection_super(typed.value_str(), to.value_str());
    (related && args.len() == n && args.iter().all(Option::is_some))
        .then(|| render(&with_arguments(to, args), false, &nullable))
}

/// The type of a local from what the method stores into it, else from how
/// it uses it: the parameter or return types it is passed to and the types
/// of the locals it is stored into, then the
/// classes of its elements (their nearest common superclass or interface)
/// or the class an element read from it is cast to. `None` when nothing
/// proves an argument or the proofs disagree.
fn evidence(
    local: &Local,
    declared: &VarType,
    scope: &BTreeSet<String>,
    copied: &[VarType],
) -> Option<VarType> {
    if declared.value.as_deref() == Some(REF) {
        return Some(with_arguments(
            declared,
            vec![Some(VarType::VARTYPE_OBJECT())],
        ));
    }
    let mut found: Option<Vec<Option<VarType>>> = None;
    for through_class in [false, true] {
        for store in &local.stores {
            if let Some(args) = stored_arguments(*store, declared, scope, through_class) {
                match &found {
                    None => found = Some(args),
                    Some(f) if same_arguments(f, &args) => {}
                    Some(_) => return None,
                }
            }
        }
        if let Some(args) = found {
            return Some(with_arguments(declared, args));
        }
    }
    let n = arity(declared)?;
    let mut args: Vec<Option<VarType>> = vec![None; n];
    for sink in local.sinks.iter().chain(copied) {
        let down = sink
            .try_as_generic()
            .filter(|_| collection_super(declared.value_str(), sink.value_str()));
        let sargs = upcast_arguments(sink, declared)
            .or_else(|| down.map(|g| g.get_arguments().to_vec()))
            .filter(|a| Some(a.len()) == arity(declared));
        let Some(sargs) = sargs.filter(|a| closed(a, scope)) else {
            continue;
        };
        for (i, a) in sargs.into_iter().enumerate() {
            let Some(a) = a.map(|a| {
                if a.try_as_generic().map(|g| g.get_wildcard())
                    == Some(GenericType::WILDCARD_EXTENDS)
                {
                    GenericType::with_wildcard(&a, GenericType::WILDCARD_NO)
                } else {
                    a
                }
            }) else {
                continue;
            };
            match &args[i] {
                None => args[i] = Some(a),
                Some(prev) if same_type(prev, &a) => {}
                Some(_) => return None,
            }
        }
    }
    for i in 0..n {
        if args[i].is_some() {
            continue;
        }
        let casts: Vec<&VarType> = local
            .element_casts
            .iter()
            .filter(|(slot, _)| *slot == i)
            .map(|(_, t)| t)
            .collect();
        let elements: Vec<VarType> = local
            .elements
            .iter()
            .filter(|(slot, _)| *slot == i)
            .filter_map(|(_, v)| element_type(*v, scope))
            .collect();
        let mut all: Vec<VarType> = casts.into_iter().cloned().collect();
        all.extend(elements);
        args[i] = Some(common_supertype(&all)?);
    }
    Some(with_arguments(declared, args))
}

/// The nearest class or interface every one of `types` is (all the same
/// generic type, or plain classes), `None` for no types or none but
/// `Object`.
fn common_supertype(types: &[VarType]) -> Option<VarType> {
    let first = types.first()?;
    if types.iter().all(|t| same_type(t, first)) {
        return Some(first.clone());
    }
    if types.iter().any(|t| t.is_generic()) {
        return None;
    }
    let context = DecompilerContext::get_struct_context();
    let mut acc = first.value_str().to_string();
    for t in &types[1..] {
        let v = t.value_str();
        if context.instance_of(v, &acc) {
            continue;
        }
        if context.instance_of(&acc, v) {
            acc = v.to_string();
            continue;
        }
        acc = context.find_common_ancestor(&acc, v)?.qualified_name();
    }
    let t = VarType::new_value(CodeType::Object, 0, acc.as_str());
    (arity(&t).is_none() && acc != "java/lang/Object").then_some(t)
}

/// The arguments for `declared` that a value stored into the local proves:
/// a value of a generic type (a cast of one to a JDK collection class with
/// the same parameters keeps its arguments), or (`through_class`, asked only when no store
/// has one: a class implementing `declared` fixes its arguments, which may
/// be narrower than those of the other values stored) of a class whose
/// generic supertype `declared` is.
fn stored_arguments(
    value: Exprent,
    declared: &VarType,
    scope: &BTreeSet<String>,
    through_class: bool,
) -> Option<Vec<Option<VarType>>> {
    let value = uncast(value);
    if value
        .try_as_const()
        .is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
    {
        return None;
    }
    let raw = VarType::new_value(CodeType::Object, 0, declared.value_str());
    let mut t = value.get_inferred_expr_type(Some(&raw));
    if let Some(cast) = value
        .try_as_function()
        .filter(|f| f.get_func_type() == FunctionType::Cast && !t.is_generic())
    {
        let operand = cast.get_lst_operands().get(0).get_inferred_expr_type(None);
        if operand.is_generic()
            && (operand.value == t.value
                || collection_super(operand.value_str(), t.value_str())
                || collection_super(t.value_str(), operand.value_str()))
        {
            t = with_arguments(&t, operand.try_as_generic()?.get_arguments().to_vec());
        }
    }
    if generic_method_call(value) && mentions_object(&t) {
        return None;
    }
    if !t.is_generic() {
        if t.value == declared.value || arity(&t).is_some() {
            t = view_type(value)?;
        } else if !through_class {
            return None;
        }
    }
    if t.type_ != CodeType::Object || t.array_dim != 0 {
        return None;
    }
    upcast_arguments(&t, declared).filter(|a| closed(a, scope))
}

/// Whether `value` calls a generic method: Vineflower types a type
/// variable it cannot bind there as `Object`, which proves nothing.
fn generic_method_call(value: Exprent) -> bool {
    let Some(inv) = value.try_as_invocation() else {
        return false;
    };
    DecompilerContext::get_struct_context()
        .get_class(&inv.get_classname())
        .and_then(|cl| cl.get_method_recursive(&inv.get_name(), &inv.get_string_descriptor()))
        .and_then(|m| m.get_signature())
        .is_some_and(|sig| !sig.type_parameters().is_empty())
}

/// Whether `t` has `Object` among its type arguments.
fn mentions_object(t: &VarType) -> bool {
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments().iter().flatten().any(|a| {
            (a.type_ == CodeType::Object && a.value_str() == "java/lang/Object")
                || mentions_object(a)
        })
    })
}

/// The type of a JDK collection's view, `c.iterator()` an `Iterator` of
/// `c`'s elements, where Vineflower has no class file of the collection to
/// infer it from.
fn view_type(value: Exprent) -> Option<VarType> {
    let inv = value.try_as_invocation()?;
    let receiver = inv.get_instance()?;
    let t = receiver.get_inferred_expr_type(None);
    let args = t.try_as_generic()?.get_arguments().to_vec();
    let (target, family, slots): (&str, &str, &[usize]) = match (
        inv.get_name().as_str(),
        inv.get_string_descriptor().as_str(),
    ) {
        ("iterator", "()Ljava/util/Iterator;") => {
            ("java/util/Iterator", "java/lang/Iterable", &[0])
        }
        ("listIterator", "()Ljava/util/ListIterator;") => {
            ("java/util/ListIterator", "java/util/List", &[0])
        }
        ("keySet", "()Ljava/util/Set;") => ("java/util/Set", "java/util/Map", &[0]),
        ("values", "()Ljava/util/Collection;") => ("java/util/Collection", "java/util/Map", &[1]),
        _ => return None,
    };
    if !(t.value_str() == family || collection_super(t.value_str(), family)) {
        return None;
    }
    let picked: Vec<Option<VarType>> = slots
        .iter()
        .map(|i| args.get(*i).cloned().flatten())
        .collect();
    picked
        .iter()
        .all(Option::is_some)
        .then(|| with_arguments(&VarType::new_value(CodeType::Object, 0, target), picked))
}

/// The type an element stored into the local has, boxed.
fn element_type(value: Exprent, scope: &BTreeSet<String>) -> Option<VarType> {
    let value = uncast(value);
    if let Some(inv) = value.try_as_invocation() {
        if inv.is_boxing_call() {
            return Some(VarType::new_value(CodeType::Object, 0, inv.get_classname()));
        }
    }
    if value
        .try_as_const()
        .is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
    {
        return None;
    }
    let t = value.get_inferred_expr_type(None);
    if t.type_ != CodeType::Object || t.value.is_none() || t.value_str() == "java/lang/Object" {
        return None;
    }
    if generic_method_call(value) && mentions_object(&t) {
        return None;
    }
    if arity(&t).is_some() && !t.is_generic() {
        return None;
    }
    closed(&[Some(t.clone())], scope).then_some(t)
}

/// Whether the method `owner.name desc` may take one of its class's type
/// parameters: a parameter of its generic signature names a type variable
/// of a class, or, without a signature, one is erased to `Object`.
fn takes_type_parameter(owner: &str, name: &str, desc: &str) -> bool {
    let context = DecompilerContext::get_struct_context();
    let method = context
        .get_class(owner)
        .and_then(|cl| cl.get_method_recursive(name, desc));
    let own: BTreeSet<String> = method
        .and_then(|m| m.get_signature())
        .map(|s| s.type_parameters().iter().cloned().collect())
        .unwrap_or_default();
    match method.and_then(|m| m.get_signature()) {
        Some(sig) => sig
            .parameter_types()
            .iter()
            .any(|t| names_type_variable(t, &own)),
        None => desc.contains("Ljava/lang/Object;"),
    }
}

/// Whether `class` is the class being written or one enclosing it, whose
/// type parameters the method may name.
fn in_scope_class(class: &str) -> bool {
    DecompilerContext::get_current_class().is_some_and(|cl| {
        let mut name = cl.qualified_name();
        loop {
            if name == class {
                return true;
            }
            match name.rfind('$') {
                Some(dollar) => name.truncate(dollar),
                None => return false,
            }
        }
    })
}

/// Whether `t` names one of the type variables `names`.
fn mentions_any(t: &VarType, names: &BTreeSet<String>) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value.as_deref().is_some_and(|v| names.contains(v));
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments()
            .iter()
            .flatten()
            .any(|a| mentions_any(a, names))
    })
}

/// Whether `t` names a type variable other than `own` (the method's).
fn names_type_variable(t: &VarType, own: &BTreeSet<String>) -> bool {
    if t.type_ == CodeType::Genvar {
        return t.value.as_deref().is_none_or(|v| !own.contains(v));
    }
    t.try_as_generic().is_some_and(|g| {
        g.get_arguments()
            .iter()
            .flatten()
            .any(|a| names_type_variable(a, own))
    })
}

/// Marks `value`, the value of an assignment, when it is a cast Kotlin needs
/// not write: of `null`, or of a constructor call of the cast's class or a
/// class that implements it (kotlinc's checkcast after `mutableListOf()`).
/// Either always succeeds, and the declaration types the value. (A cast of
/// an element read from a typed local to the element's class is marked
/// after the locals are typed: kotlinc's checkcast of a generic result,
/// which the typed read compiles to again.)
fn mark_redundant_cast(value: Exprent) {
    let Some(cast) = value.try_as_k_function() else {
        return;
    };
    if cast.get_func_type() != FunctionType::Cast || !cast.does_cast() {
        return;
    }
    let ops = cast.get_lst_operands();
    let to = ops.get(1).get_expr_type();
    if arity(&to).is_none() {
        return;
    }
    let operand = ops.get(0);
    let redundant = operand
        .try_as_const()
        .is_some_and(|c| c.get_const_type().type_ == CodeType::Null)
        || operand.try_as_new().is_some_and(|n| {
            let t = n.get_new_type();
            t.array_dim == 0
                && t.type_ == CodeType::Object
                && (t.value == to.value || collection_super(t.value_str(), to.value_str()))
        });
    if redundant {
        cast.kd_mut(|d| d.redundant_cast = true);
    }
}

/// Fix `kt_raw_generic_type`: whether the cast `cast` is written as its
/// operand alone.
pub fn redundant(cast: crate::k_function_exprent::KFunctionExprent) -> bool {
    cast.kd(|d| d.redundant_cast)
}

/// `value` without any casts around it: a member called on a cast local is
/// called on the local.
fn through_casts(value: Exprent) -> Exprent {
    match value.try_as_function() {
        Some(func) if func.get_func_type() == FunctionType::Cast => {
            through_casts(func.get_lst_operands().get(0))
        }
        _ => value,
    }
}

/// `value` without the casts Vineflower leaves unwritten.
fn uncast(value: Exprent) -> Exprent {
    match value.try_as_function() {
        Some(func) if func.get_func_type() == FunctionType::Cast && !func.does_cast() => {
            uncast(func.get_lst_operands().get(0))
        }
        _ => value,
    }
}

/// `t`'s arguments seen as `declared`'s: the same class's, or a JDK
/// collection's through the interfaces it implements, or through the
/// class's generic supertypes.
fn upcast_arguments(t: &VarType, declared: &VarType) -> Option<Vec<Option<VarType>>> {
    if let Some(generic) = t.try_as_generic() {
        if t.value == declared.value || collection_super(t.value_str(), declared.value_str()) {
            let args = generic.get_arguments().to_vec();
            return (Some(args.len()) == arity(declared)).then_some(args);
        }
    } else if t.value == declared.value {
        return None;
    }
    let up = GenericType::get_generic_super_type(t, declared)?;
    if up.value != declared.value {
        return None;
    }
    let args = up.try_as_generic()?.get_arguments().to_vec();
    (Some(args.len()) == arity(declared)).then_some(args)
}

/// Whether the JDK collection class `from` implements `to` with the same
/// type parameters.
fn collection_super(from: &str, to: &str) -> bool {
    let mut pending = vec![from];
    let mut seen = 0;
    while let Some(c) = pending.pop() {
        if c == to {
            return true;
        }
        seen += 1;
        if seen > 16 {
            return false;
        }
        if let Some((_, supers)) = COLLECTION_SUPERS.iter().find(|(n, _)| *n == c) {
            pending.extend(supers.iter());
        }
    }
    false
}

/// The number of type parameters of `t`'s class when it is a generic class
/// written without arguments.
pub fn arity(t: &VarType) -> Option<usize> {
    if t.type_ != CodeType::Object || t.array_dim != 0 || t.is_generic() {
        return None;
    }
    let name = t.value.as_deref()?;
    if name.starts_with("kotlin/jvm/functions/") {
        return None;
    }
    if let Some((_, n)) = KNOWN_GENERIC.iter().find(|(c, _)| *c == name) {
        return Some(*n);
    }
    let cl = DecompilerContext::get_struct_context().get_class(name)?;
    let n = cl.get_signature()?.fparameters().len();
    (n > 0).then_some(n)
}

/// The generic types of the method's parameters, by slot, from its
/// `Signature` attribute (kotlinc writes the signature, not the
/// LocalVariableTypeTable).
fn parameter_types(method: StructMethod) -> Vec<(i32, VarType)> {
    let Some(sig) = method.get_signature() else {
        return Vec::new();
    };
    let generic = sig.parameter_types().clone();
    let md = method.method_descriptor();
    if generic.len() != md.params.len() {
        return Vec::new();
    }
    let mut slot = if method.has_modifier(crate::code_constants::ACC_STATIC) {
        0
    } else {
        1
    };
    let mut res = Vec::new();
    for (i, t) in generic.into_iter().enumerate() {
        if t.is_generic() && t.type_ == CodeType::Object && t.array_dim == 0 {
            res.push((slot, t));
        }
        slot += md.params[i].stack_size;
    }
    res
}

/// The type parameters the method's body may name: its own and its
/// classes'.
fn type_parameters_in_scope(method: StructMethod) -> BTreeSet<String> {
    let mut scope: BTreeSet<String> = BTreeSet::new();
    if let Some(sig) = method.get_signature() {
        scope.extend(sig.type_parameters().iter().cloned());
    }
    if let Some(cl) = DecompilerContext::get_current_class() {
        let mut name = cl.qualified_name();
        loop {
            if let Some(sig) = DecompilerContext::get_struct_context()
                .get_class(&name)
                .and_then(|c| c.get_signature())
            {
                scope.extend(sig.fparameters().iter().cloned());
            }
            match name.rfind('$') {
                Some(dollar) => name.truncate(dollar),
                None => break,
            }
        }
    }
    scope
}

/// Whether every type in `args` names only classes and the type parameters
/// in `scope`.
fn closed(args: &[Option<VarType>], scope: &BTreeSet<String>) -> bool {
    args.iter().all(|a| match a {
        None => true,
        Some(t) => closed_type(t, scope),
    })
}

fn closed_type(t: &VarType, scope: &BTreeSet<String>) -> bool {
    match t.type_ {
        CodeType::Genvar => t.value.as_deref().is_some_and(|v| scope.contains(v)),
        CodeType::Object => match t.try_as_generic() {
            Some(g) => {
                g.get_parent().is_none_or(|p| closed_type(&p, scope))
                    && closed(g.get_arguments(), scope)
            }
            None => arity(t).is_none(),
        },
        CodeType::Null | CodeType::Unknown => false,
        _ => t.array_dim > 0,
    }
}

fn same_type(a: &VarType, b: &VarType) -> bool {
    a.equals(b) && same_arguments_of(a, b)
}

fn same_arguments_of(a: &VarType, b: &VarType) -> bool {
    match (a.try_as_generic(), b.try_as_generic()) {
        (Some(x), Some(y)) => same_arguments(x.get_arguments(), y.get_arguments()),
        (None, None) => true,
        _ => false,
    }
}

fn same_arguments(a: &[Option<VarType>], b: &[Option<VarType>]) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(x, y)| match (x, y) {
            (None, None) => true,
            (Some(x), Some(y)) => {
                same_type(x, y)
                    && x.try_as_generic().map(|g| g.get_wildcard())
                        == y.try_as_generic().map(|g| g.get_wildcard())
            }
            _ => false,
        })
}

/// `declared`'s class with the type arguments `args`.
fn with_arguments(declared: &VarType, args: Vec<Option<VarType>>) -> VarType {
    GenericType::new(
        CodeType::Object,
        0,
        declared.value_str(),
        None,
        args,
        GenericType::WILDCARD_NO,
    )
    .0
}

/// Fix `kt_raw_generic_type`, for fix `kt_nullable_local`: whether `value`
/// reads an element of a local the fix typed with a star projection or a
/// nullable argument there (`Iterator<*>.next()` is an `Any?`).
pub fn reads_nullable_element(value: Exprent) -> bool {
    let Some(inv) = value.try_as_invocation() else {
        return false;
    };
    let Some(local) = inv
        .get_instance()
        .and_then(|i| through_casts(i).try_as_var())
        .and_then(|v| v.kuna_declared())
    else {
        return false;
    };
    let (name, desc) = (inv.get_name(), inv.get_string_descriptor());
    let Some((_, _, slot)) = ELEMENT_READS
        .iter()
        .find(|(n, d, _)| *n == name && *d == desc)
    else {
        return false;
    };
    let args = local
        .type_
        .try_as_generic()
        .map(|g| g.get_arguments().to_vec());
    args.is_some_and(|a| a.get(*slot).is_some_and(Option::is_none))
        || local.nullable.get(*slot).copied().unwrap_or(false)
}

/// A local's declared type as the fix gave it, in Kotlin's syntax.
pub fn declared_type(typed: &KunaLocalType) -> String {
    render(&typed.type_, typed.mutable, &typed.nullable)
}

/// The Kotlin name of a mapped collection interface, read-only or mutable.
pub fn mapped_name(class: &str, mutable: bool) -> Option<&'static str> {
    MAPPED
        .iter()
        .find(|(c, _, _)| *c == class)
        .map(|(_, r, m)| if mutable { *m } else { *r })
}

/// A local's declared type with its arguments, as Kotlin writes it: a
/// mapped collection interface by its read-only or mutable name, type
/// arguments in Kotlin's syntax (an argument's own collection types
/// read-only).
pub fn kotlin_type(t: &VarType, mutable: bool) -> String {
    render(t, mutable, &[])
}

fn render(t: &VarType, mutable: bool, nullable: &[bool]) -> String {
    let Some(generic) = t.try_as_generic() else {
        return crate::k_types::KTypes::get_kotlin_type(Some(t));
    };
    let name = match mapped_name(t.value_str(), mutable) {
        Some(name) => name.to_string(),
        None => {
            let erasure = VarType::new_value(CodeType::Object, 0, t.value_str());
            crate::k_types::KTypes::get_kotlin_type(Some(&erasure))
        }
    };
    let args: Vec<String> = generic
        .get_arguments()
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let text = argument(a.as_ref(), !mutable && covariant(t.value_str(), i));
            let nullable =
                nullable.get(i).copied().unwrap_or(false) || t.value.as_deref() == Some(REF);
            if a.is_some() && nullable && !text.ends_with('?') {
                format!("{text}?")
            } else {
                text
            }
        })
        .collect();
    format!("{name}<{}>", args.join(", "))
}

/// Whether the read-only Kotlin type of `class` is covariant in its
/// parameter `i` (`List<out E>`), so that `out` there says nothing.
fn covariant(class: &str, i: usize) -> bool {
    match class {
        "java/lang/Iterable"
        | "java/util/Collection"
        | "java/util/List"
        | "java/util/Set"
        | "java/util/Iterator"
        | "java/util/ListIterator" => i == 0,
        "java/util/Map" => i == 1,
        "java/util/Map$Entry" => true,
        _ => false,
    }
}

/// A type argument in Kotlin's syntax: a wildcard as a projection (none
/// where the parameter is covariant), a generic class with its own
/// arguments, collection interfaces by their read-only names.
fn argument(a: Option<&VarType>, covariant: bool) -> String {
    let Some(a) = a else {
        return "*".to_string();
    };
    let wildcard = a
        .try_as_generic()
        .map_or(GenericType::WILDCARD_NO, |g| g.get_wildcard());
    let bound = GenericType::with_wildcard(a, GenericType::WILDCARD_NO);
    let text = if bound
        .try_as_generic()
        .is_some_and(|g| !g.get_arguments().is_empty())
        && bound.array_dim == 0
        && bound.type_ == CodeType::Object
        && bound
            .try_as_generic()
            .is_some_and(|g| g.get_parent().is_none())
    {
        render(&bound, false, &[])
    } else {
        crate::kotlin::kuna_fix_kt_java_type_syntax::type_argument(Some(&bound))
    };
    match wildcard {
        GenericType::WILDCARD_UNBOUND => "*".to_string(),
        GenericType::WILDCARD_EXTENDS if covariant => text,
        GenericType::WILDCARD_EXTENDS => format!("out {text}"),
        GenericType::WILDCARD_SUPER => format!("in {text}"),
        _ => text,
    }
}

/// A raw generic class in a type test or cast, `Collection<*>`, else `None`.
pub fn star_projected(t: &VarType) -> Option<String> {
    let n = arity(t)?;
    let name = match mapped_name(t.value_str(), false) {
        Some(name) => name.to_string(),
        None => crate::k_types::KTypes::get_kotlin_type(Some(t)),
    };
    if name.contains('<') {
        return None;
    }
    Some(format!("{name}<{}>", vec!["*"; n].join(", ")))
}

/// The statements' locals and what is done with them.
struct Walk {
    locals: BTreeMap<i32, Local>,
    /// The casts whose value is stored into a local: (cast, local).
    casts: Vec<(Exprent, Exprent)>,
    /// The casts of an element read from a local: (cast, local, argument).
    read_casts: Vec<(Exprent, i32, usize)>,
    return_type: Option<VarType>,
}

impl Walk {
    fn stat(&mut self, stat: Statement) {
        let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
        lists.extend(stat.get_stat_exprents());
        if let Some(exprents) = stat.get_exprents() {
            lists.extend(exprents.to_vec());
        }
        for expr in lists {
            self.visit(expr);
        }
        for child in stat.get_stats().borrow().to_vec() {
            self.stat(child);
        }
    }

    fn local(&mut self, index: i32) -> &mut Local {
        self.locals.entry(index).or_default()
    }

    /// Visits `e` and what it contains.
    fn visit(&mut self, e: Exprent) {
        if let Some(var) = e.try_as_var() {
            let local = self.local(var.get_index());
            if let Some(k) = e.try_as_k_var() {
                if k.kd(|d| d.declaration_type) == Some(DeclarationType::Definition) {
                    local.declared = Some(var.get_definition_var_type());
                }
            }
        }
        if let Some(assign) = e.try_as_assignment() {
            if assign.get_cond_type().is_none() {
                let left = assign.get_left();
                let right = assign.get_right();
                mark_redundant_cast(right);
                if let Some(var) = left.try_as_var() {
                    if right
                        .try_as_k_function()
                        .is_some_and(|f| f.get_func_type() == FunctionType::Cast)
                    {
                        self.casts.push((right, left));
                    }
                    let constructed = uncast(right).try_as_new().is_some()
                        || right.try_as_function().is_some_and(|f| {
                            f.get_func_type() == FunctionType::Cast
                                && f.get_lst_operands().get(0).try_as_new().is_some()
                        });
                    let local = self.local(var.get_index());
                    local.stores.push(right);
                    local.constructed |= constructed;
                    if let Some(from) = uncast(right).try_as_var() {
                        self.local(from.get_index()).copies.push(var.get_index());
                    }
                    if let Some(inv) = uncast(right).try_as_invocation() {
                        if let Some(from) = inv
                            .get_instance()
                            .and_then(|i| through_casts(i).try_as_var())
                        {
                            if VIEWS.contains(&inv.get_name().as_str()) {
                                self.local(from.get_index()).views.push(var.get_index());
                            }
                        }
                    }
                } else if let Some(field) = left.try_as_field() {
                    if let Some(var) = field
                        .get_instance()
                        .and_then(|i| through_casts(i).try_as_var())
                    {
                        if field.get_classname() == REF && field.get_name() == "element" {
                            let local = self.local(var.get_index());
                            local.elements.push((0, right));
                            local.written = true;
                        }
                    }
                }
            }
        }
        let call = e
            .try_as_invocation()
            .or_else(|| e.try_as_new().and_then(|n| n.get_constructor()));
        if let Some(inv) = call {
            if let Some(var) = inv
                .get_instance()
                .and_then(|i| through_casts(i).try_as_var())
            {
                let name = inv.get_name();
                let desc = inv.get_string_descriptor();
                let params = inv.get_lst_parameters();
                let local = self.local(var.get_index());
                if MUTATORS.contains(&name.as_str()) {
                    local.mutated = true;
                    local.written = true;
                }
                if !params.is_empty() && takes_type_parameter(&inv.get_classname(), &name, &desc) {
                    local.written_by_call = true;
                }
                for (n, d, arg, slot) in ELEMENT_STORES {
                    if *n == name && *d == desc && *arg < params.len() {
                        local.elements.push((*slot, params.get(*arg)));
                    }
                }
            }
            let params = inv.get_lst_parameters();
            for i in 0..params.len() {
                if let Some(var) = through_casts(params.get(i)).try_as_var() {
                    self.local(var.get_index()).passed = true;
                }
            }
            let signature = DecompilerContext::get_struct_context()
                .get_class(&inv.get_classname())
                .and_then(|cl| {
                    cl.get_method_recursive(&inv.get_name(), &inv.get_string_descriptor())
                })
                .and_then(|m| m.get_signature());
            if let Some(sig) = signature {
                let types = sig.parameter_types().clone();
                let own: BTreeSet<String> = if in_scope_class(&inv.get_classname()) {
                    sig.type_parameters().iter().cloned().collect()
                } else {
                    BTreeSet::new()
                };
                if types.len() == params.len() {
                    for (i, t) in types.iter().enumerate() {
                        let foreign = if own.is_empty() && !in_scope_class(&inv.get_classname()) {
                            names_type_variable(t, &BTreeSet::new())
                        } else {
                            mentions_any(t, &own)
                        };
                        if foreign {
                            continue;
                        }
                        if let Some(var) = through_casts(params.get(i)).try_as_var() {
                            self.local(var.get_index()).sinks.push(t.clone());
                        }
                    }
                }
            }
        }
        if let Some(func) = e.try_as_function() {
            if func.get_func_type() == FunctionType::Cast && func.does_cast() {
                let ops = func.get_lst_operands();
                let to = ops.get(1).get_expr_type();
                self.element_read_cast(e, ops.get(0), to);
            }
        }
        if let Some(exit) = e.try_as_exit() {
            if let (Some(value), Some(ret)) = (exit.get_value(), self.return_type.clone()) {
                if let Some(var) = through_casts(value).try_as_var() {
                    self.local(var.get_index()).sinks.push(ret.clone());
                }
                if let Some(cast) = value.try_as_k_function() {
                    if cast.get_func_type() == FunctionType::Cast && ret.is_generic() {
                        cast.kd_mut(|d| d.kuna_cast_target = Some(ret));
                    }
                }
            }
        }
        for child in e.get_all_exprents() {
            self.visit(child);
        }
    }

    /// Records the class `to` an element read from a local is cast to by
    /// `cast`.
    fn element_read_cast(&mut self, cast: Exprent, read: Exprent, to: VarType) {
        if to.type_ != CodeType::Object || to.value.is_none() || arity(&to).is_some() {
            return;
        }
        if let Some(inv) = read.try_as_invocation() {
            if let Some(var) = inv
                .get_instance()
                .and_then(|i| through_casts(i).try_as_var())
            {
                let name = inv.get_name();
                let desc = inv.get_string_descriptor();
                if let Some((_, _, slot)) = ELEMENT_READS
                    .iter()
                    .find(|(n, d, _)| *n == name && *d == desc)
                {
                    self.local(var.get_index()).element_casts.push((*slot, to));
                    self.read_casts.push((cast, var.get_index(), *slot));
                }
            }
        } else if let Some(field) = read.try_as_field() {
            if field.get_classname() == REF && field.get_name() == "element" {
                if let Some(var) = field
                    .get_instance()
                    .and_then(|i| through_casts(i).try_as_var())
                {
                    self.local(var.get_index()).element_casts.push((0, to));
                }
            }
        }
    }
}
