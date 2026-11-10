//! kuna fix `kt_mapped_java_member`: a call of a JDK method that Kotlin
//! exposes under another name is written with Kotlin's name.
//!
//! Kotlin maps `java.lang.String`, `CharSequence`, `Throwable`, `Number`,
//! `Enum` and the collection interfaces to its own types, whose members
//! cover the JDK methods under other names: `s.length()` is the property
//! `s.length`, `s.charAt(i)` the operator `s[i]`, `list.size()` the property
//! `list.size`, `map.entrySet()` `map.entries`, `e.getMessage()` `e.message`,
//! `n.intValue()` `n.toInt()`, `x.getClass()` `x.javaClass`, and an array's
//! `length` is its `size`. Vineflower writes the JVM names, which Kotlin
//! rejects on its own types ("unresolved reference", "expression 'length'
//! of type Int cannot be invoked as a function"). kotlinc compiles each
//! Kotlin form to the same `invokevirtual`/`invokeinterface` of the JDK
//! method (and `arraylength`). With the fix those calls are written in the
//! Kotlin form when the called method's class is the mapped JDK type or a
//! subtype of it.

use crate::decompiler_context::DecompilerContext;

/// How a mapped call is written.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Form {
    /// `receiver.name`, the call's parentheses dropped.
    Property(&'static str),
    /// `receiver[argument]`.
    Index,
    /// `receiver.name(arguments)`.
    Renamed(&'static str),
}

const CHAR_SEQUENCES: &[&str] = &[
    "java/lang/CharSequence",
    "java/lang/String",
    "java/lang/StringBuilder",
    "java/lang/StringBuffer",
    "java/lang/AbstractStringBuilder",
];
const COLLECTIONS: &[&str] = &[
    "java/util/Collection",
    "java/util/List",
    "java/util/Set",
    "java/util/Queue",
    "java/util/Deque",
    "java/util/SortedSet",
    "java/util/NavigableSet",
    "java/util/AbstractCollection",
    "java/util/AbstractList",
    "java/util/AbstractSet",
    "java/util/ArrayList",
    "java/util/LinkedList",
    "java/util/ArrayDeque",
    "java/util/HashSet",
    "java/util/LinkedHashSet",
    "java/util/TreeSet",
    "java/util/Vector",
    "java/util/PriorityQueue",
    "java/util/concurrent/CopyOnWriteArrayList",
    "java/util/concurrent/CopyOnWriteArraySet",
    "java/util/concurrent/ConcurrentLinkedQueue",
    "java/util/concurrent/ConcurrentLinkedDeque",
    "java/util/concurrent/LinkedBlockingQueue",
    "java/util/concurrent/LinkedBlockingDeque",
    "java/util/concurrent/ArrayBlockingQueue",
    "java/util/concurrent/BlockingQueue",
];
const MAPS: &[&str] = &[
    "java/util/Map",
    "java/util/SortedMap",
    "java/util/NavigableMap",
    "java/util/AbstractMap",
    "java/util/HashMap",
    "java/util/LinkedHashMap",
    "java/util/TreeMap",
    "java/util/WeakHashMap",
    "java/util/IdentityHashMap",
    "java/util/EnumMap",
    "java/util/Hashtable",
    "java/util/concurrent/ConcurrentMap",
    "java/util/concurrent/ConcurrentHashMap",
];
const MAP_ENTRIES: &[&str] = &["java/util/Map$Entry", "java/util/AbstractMap$SimpleEntry"];
const NUMBERS: &[&str] = &[
    "java/lang/Number",
    "java/lang/Integer",
    "java/lang/Long",
    "java/lang/Short",
    "java/lang/Byte",
    "java/lang/Double",
    "java/lang/Float",
    "java/math/BigInteger",
    "java/math/BigDecimal",
    "java/util/concurrent/atomic/AtomicInteger",
    "java/util/concurrent/atomic/AtomicLong",
];
const THROWABLES: &[&str] = &[
    "java/lang/Throwable",
    "java/lang/Exception",
    "java/lang/RuntimeException",
    "java/lang/Error",
];
const ENUMS: &[&str] = &["java/lang/Enum"];

/// The Kotlin form of a call of `name` with `descriptor` on an instance of
/// `classname` (the class the call names), when Kotlin names it otherwise.
pub fn form(classname: &str, name: &str, descriptor: &str) -> Option<Form> {
    match (name, descriptor) {
        ("getClass", "()Ljava/lang/Class;") => Some(Form::Property("javaClass")),
        ("length", "()I") if is_a(classname, CHAR_SEQUENCES) => Some(Form::Property("length")),
        ("charAt", "(I)C") if is_a(classname, CHAR_SEQUENCES) => Some(Form::Index),
        ("size", "()I") if is_a(classname, COLLECTIONS) || is_a(classname, MAPS) => {
            Some(Form::Property("size"))
        }
        ("keySet", "()Ljava/util/Set;") if is_a(classname, MAPS) => Some(Form::Property("keys")),
        ("values", "()Ljava/util/Collection;") if is_a(classname, MAPS) => {
            Some(Form::Property("values"))
        }
        ("entrySet", "()Ljava/util/Set;") if is_a(classname, MAPS) => {
            Some(Form::Property("entries"))
        }
        ("getKey", "()Ljava/lang/Object;") if is_a(classname, MAP_ENTRIES) => {
            Some(Form::Property("key"))
        }
        ("getValue", "()Ljava/lang/Object;") if is_a(classname, MAP_ENTRIES) => {
            Some(Form::Property("value"))
        }
        ("remove", "(I)Ljava/lang/Object;") if is_a(classname, COLLECTIONS) => {
            Some(Form::Renamed("removeAt"))
        }
        ("getMessage", "()Ljava/lang/String;") if is_a(classname, THROWABLES) => {
            Some(Form::Property("message"))
        }
        ("getCause", "()Ljava/lang/Throwable;") if is_a(classname, THROWABLES) => {
            Some(Form::Property("cause"))
        }
        ("name", "()Ljava/lang/String;") if is_a(classname, ENUMS) => Some(Form::Property("name")),
        ("ordinal", "()I") if is_a(classname, ENUMS) => Some(Form::Property("ordinal")),
        _ if is_a(classname, NUMBERS) => number_conversion(name, descriptor).map(Form::Renamed),
        _ => None,
    }
}

fn number_conversion(name: &str, descriptor: &str) -> Option<&'static str> {
    Some(match (name, descriptor) {
        ("intValue", "()I") => "toInt",
        ("longValue", "()J") => "toLong",
        ("shortValue", "()S") => "toShort",
        ("byteValue", "()B") => "toByte",
        ("floatValue", "()F") => "toFloat",
        ("doubleValue", "()D") => "toDouble",
        _ => return None,
    })
}

/// Whether `classname` is one of `family` or, as far as the loaded classes
/// show, a subtype of one.
fn is_a(classname: &str, family: &[&str]) -> bool {
    if family.contains(&classname) {
        return true;
    }
    let context = DecompilerContext::get_struct_context();
    context.get_class(classname).is_some()
        && family
            .iter()
            .any(|root| context.instance_of(classname, root))
}
