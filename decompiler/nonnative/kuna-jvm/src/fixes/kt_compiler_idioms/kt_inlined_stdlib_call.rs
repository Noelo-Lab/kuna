//! kuna fix `kt_inlined_stdlib_call`: kotlinc's inlined copy of a
//! standard-library inline function is written as the call.
//!
//! kotlinc copies an inline function's body into every call site, with the
//! lambda's body in place of its call. Vineflower writes the copy: long,
//! unlike the source, and calling the standard library's `internal`
//! helpers, which other code may not call (`closeFinally`, `checkRadix`,
//! `copyOfRangeImpl`, `collectionSizeOrDefault`, `StringCompanionObject`).
//! A copy whose code is exactly the body of the function for the library
//! version the class was compiled against is written as the call; anything
//! else stays as it is. Calls of the helpers prove the copy only outside the
//! `kotlin` package, whose own code calls them directly.
//!
//! Before the last stack-variable pass (`resugar`): `Integer.toString(x,
//! checkRadix(r))` and its siblings become `x.toString(r)`, `s.toInt(r)`,
//! `s.toLong(r)`, `s.toShort(r)`, `s.toByte(r)` (Kotlin 1.3's `Byte` and
//! `Short` versions check the radix twice); `ArraysKt.copyOfRange(a, f, t)`
//! (`copyOfRangeImpl`) becomes `a.copyOfRange(f, t)`; `java.lang.String
//! .format(fmt, Arrays.copyOf(v, v.length))` with `v` the array just built
//! becomes `String.format(fmt, e1, ..)` where kotlinc read `String.Companion`
//! (stored into a local by Kotlin 1.3, read and popped by later versions),
//! else `fmt.format(e1, ..)`, likewise with a `Locale`. kotlinc's check of
//! the copied call's value (`checkExpressionValueIsNotNull(v, "java.lang
//! .String.format(format, *args)")`) goes with it: kotlinc checks the
//! rebuilt call again.
//!
//! Before the class is written (`rebuild_lambdas`), once its nested classes
//! have their captured variables: a `try`/`finally` around `closeFinally`
//! is marked to be written `receiver.use { p -> body }` (`match_use`), and
//! `for (item in r) d.add(x)` filling `d = ArrayList(collectionSizeOrDefault
//! (r, 10))` becomes `receiver.map { p -> x }` at the statement that reads
//! `d` (`rebuild_map`); the parameters take kotlinc's debug names.

use std::rc::Rc;

use crate::catch_all_statement::CatchAllStatement;
use crate::const_exprent::ConstValue;
use crate::exit_exprent::Type as ExitType;
use crate::expr_processor::ExprProcessor;
use crate::stat_edge::StatEdge;
use crate::statement::StatementType;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::basic_block_statement::BasicBlockStatement;
use crate::class_wrapper::ClassWrapper;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::Type as LoopType;
use crate::method_descriptor::MethodDescriptor;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::code_type::CodeType;

const CHECKS: [&str; 2] = ["checkNotNullExpressionValue", "checkExpressionValueIsNotNull"];
const INTRINSICS: &str = "kotlin/jvm/internal/Intrinsics";
const STRING_COMPANION: &str = "kotlin/jvm/internal/StringCompanionObject";

/// The standard-library call an invocation is written as.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Inlined {
    /// `args[0].name(args[1..])`.
    Extension(&'static str),
    /// `String.format(args)`, or with `receiver` `args[receiver].format(the
    /// other args)`; the last argument is the vararg array, written as its
    /// elements.
    Format { receiver: Option<usize> },
    /// `args[0].map { param -> args[1] }`, the parameter the variable
    /// `param` (`it` when it is unused).
    Map { param: Option<VarVersionPair> },
}

/// Whether the call is typed by the cast kotlinc put at its use (`map`'s
/// `List<R>`), not by the library function's signature.
pub fn typed_by_use(inlined: Inlined) -> bool {
    matches!(inlined, Inlined::Map { .. })
}

/// `Integer.toString(x, checkRadix(r))` and its siblings: owner, name,
/// descriptor, the Kotlin function.
const RADIX: [(&str, &str, &str, &str); 6] = [
    ("java/lang/Integer", "toString", "(II)Ljava/lang/String;", "toString"),
    ("java/lang/Long", "toString", "(JI)Ljava/lang/String;", "toString"),
    ("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", "toInt"),
    ("java/lang/Long", "parseLong", "(Ljava/lang/String;I)J", "toLong"),
    ("java/lang/Short", "parseShort", "(Ljava/lang/String;I)S", "toShort"),
    ("java/lang/Byte", "parseByte", "(Ljava/lang/String;I)B", "toByte"),
];

/// Whether `cl` may hold inlined copies the helpers prove: it is not part of
/// the standard library.
pub fn applies(cl: StructClass) -> bool {
    !cl.qualified_name().starts_with("kotlin/")
}

/// Before the last stack-variable pass: writes the copies under `root` that
/// need no lambda as their calls.
pub fn resugar(root: RootStatement, cl: StructClass, mt: StructMethod) -> bool {
    if !applies(cl) {
        return false;
    }
    let pops = companion_pops(cl, mt);
    let refs = var_refs(root.0);
    let mut res = false;
    let mut calls: Vec<InvocationExprent> = Vec::new();
    let mut defs: Vec<(Exprent, Exprent)> = Vec::new();
    for_each_exprent(root.0, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(inv) = e.try_as_invocation() {
                calls.push(inv);
            } else if let Some((left, _)) = plain_assignment(e) {
                if left.try_as_var().is_some() {
                    defs.push((left, e));
                }
            }
        }
    });
    for inv in calls {
        res |= radix(inv, &refs, &defs) || copy_of_range(inv);
    }
    blocks(root.0, &mut |stat| {
        res |= format_block(stat, &pops, &refs);
        res |= remove_checks(stat);
    });
    res
}

fn for_each_exprent(stat: Statement, f: &mut dyn FnMut(Exprent)) {
    let mut lists: Vec<Exprent> = stat.get_var_definitions().to_vec();
    lists.extend(stat.get_stat_exprents());
    if let Some(exprents) = stat.get_exprents() {
        lists.extend(exprents.to_vec());
    }
    for e in lists {
        f(e);
    }
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        for_each_exprent(child, f);
    }
}

fn blocks(stat: Statement, f: &mut dyn FnMut(Statement)) {
    if stat.get_exprents().is_some() {
        f(stat);
    }
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        blocks(child, f);
    }
}

/// How often each variable index is read or written under `stat`.
fn var_refs(stat: Statement) -> Vec<(i32, usize)> {
    let mut refs: Vec<(i32, usize)> = Vec::new();
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(v) = e.try_as_var() {
                match refs.iter_mut().find(|(i, _)| *i == v.get_index()) {
                    Some((_, n)) => *n += 1,
                    None => refs.push((v.get_index(), 1)),
                }
            }
        }
    });
    refs
}

fn refs_of(refs: &[(i32, usize)], index: i32) -> usize {
    refs.iter().find(|(i, _)| *i == index).map_or(0, |(_, n)| *n)
}

fn is_static_call(inv: InvocationExprent, owners: &[&str], name: &str, descriptor: &str) -> bool {
    inv.is_static()
        && owners.contains(&inv.get_classname().as_str())
        && inv.get_name() == name
        && inv.get_string_descriptor() == descriptor
}

/// `checkRadix(x)`: its operand.
fn check_radix(e: Exprent) -> Option<Exprent> {
    let inv = e.try_as_invocation()?;
    is_static_call(inv, &["kotlin/text/CharsKt", "kotlin/text/CharsKt__CharJVMKt"], "checkRadix", "(I)I")
        .then(|| inv.get_lst_parameters().get(0))
}

fn radix(inv: InvocationExprent, refs: &[(i32, usize)], defs: &[(Exprent, Exprent)]) -> bool {
    if inv.d(|d| d.kuna_inlined.is_some()) {
        return false;
    }
    let Some(&(_, _, _, name)) = RADIX.iter().find(|(owner, jvm, descriptor, _)| {
        is_static_call(inv, &[owner], jvm, descriptor)
    }) else {
        return false;
    };
    let params = inv.get_lst_parameters().to_vec();
    let Some(mut radix) = check_radix(params[1]) else {
        return false;
    };
    while let Some(inner) = check_radix(radix) {
        radix = inner;
    }
    let held = radix
        .try_as_var()
        .filter(|v| !v.is_stack() && refs_of(refs, v.get_index()) == 2)
        .and_then(|_| defs.iter().find(|(left, _)| same_var(*left, radix)))
        .and_then(|(_, def)| Some((def.as_assignment(), check_radix(def.as_assignment().get_right())?)));
    if name == "toString" && inv.get_classname() == "java/lang/Integer" {
        let typed = params[0].get_expr_type();
        if !matches!(
            typed.type_,
            CodeType::Int | CodeType::Byte | CodeType::Short | CodeType::Bytechar | CodeType::Shortchar
        ) {
            return false;
        }
    }
    if let Some((def, mut inner)) = held {
        while let Some(deeper) = check_radix(inner) {
            inner = deeper;
        }
        def.set_right(inner);
    }
    inv.get_lst_parameters().set(1, radix);
    inv.d_mut(|d| d.kuna_inlined = Some(Inlined::Extension(name)));
    true
}

fn copy_of_range(inv: InvocationExprent) -> bool {
    if inv.d(|d| d.kuna_inlined.is_some())
        || !inv.is_static()
        || inv.get_classname() != "kotlin/collections/ArraysKt"
        || inv.get_name() != "copyOfRange"
    {
        return false;
    }
    let descriptor = inv.get_string_descriptor();
    let Some(array) = descriptor.strip_prefix("([").and_then(|d| d.split_once("II)[")) else {
        return false;
    };
    if array.0 != array.1 || inv.get_lst_parameters().len() != 3 {
        return false;
    }
    inv.d_mut(|d| d.kuna_inlined = Some(Inlined::Extension("copyOfRange")));
    true
}

/// The offsets of the instructions right after `getstatic
/// StringCompanionObject.INSTANCE; pop` in `mt`: where kotlinc read the
/// receiver of an inlined `String.format` and dropped it.
fn companion_pops(cl: StructClass, mt: StructMethod) -> Vec<i32> {
    let Some(seq) = mt.get_instruction_sequence() else {
        return Vec::new();
    };
    let pool = cl.get_pool();
    let instrs = seq.instructions();
    let mut res = Vec::new();
    for pair in instrs.windows(2) {
        if pair[0].opcode() != 0xb2 || pair[1].opcode() != 0x57 {
            continue;
        }
        let Some(link) = pool.get_link_constant(pair[0].operand(0)) else {
            continue;
        };
        if link.classname.as_deref() == Some(STRING_COMPANION) && link.elementname == "INSTANCE" {
            res.push(pair[1].start_offset() + pair[1].length());
        }
    }
    res
}

/// The lowest bytecode offset of `e`.
fn first_offset(e: Exprent) -> Option<i32> {
    let mut low: Option<i32> = None;
    for sub in e.get_all_exprents_rec_self(true, true) {
        if let Some(bits) = sub.get_bytecode() {
            let first = bits.next_set_bit(0);
            if first >= 0 && low.is_none_or(|l| first < l) {
                low = Some(first);
            }
        }
    }
    low
}

/// Whether the method returns from between the start of `body` and its
/// `catch` handler `rethrow` (a `return` in the lambda), or the offsets
/// are unknown; `return r.use { .. }` returns after the handlers.
fn returns_before_handler(mt: StructMethod, body: Statement, rethrow: &[Exprent]) -> bool {
    let mut lo: Option<i32> = None;
    for_each_exprent(body, &mut |top| {
        if let Some(first) = first_offset(top) {
            lo = Some(lo.map_or(first, |l: i32| l.min(first)));
        }
    });
    let hi = rethrow.iter().filter_map(|e| first_offset(*e)).min();
    let (Some(lo), Some(hi), Some(seq)) = (lo, hi, mt.get_instruction_sequence()) else {
        return true;
    };
    seq.instructions()
        .iter()
        .any(|i| (0xac..=0xb1).contains(&i.opcode()) && i.start_offset() > lo && i.start_offset() < hi)
}

/// `v = e` (not a compound assignment): `v` and `e`.
fn plain_assignment(e: Exprent) -> Option<(Exprent, Exprent)> {
    let a = e.try_as_assignment().filter(|a| a.get_cond_type().is_none())?;
    Some((a.get_left(), a.get_right()))
}

fn same_var(a: Exprent, b: Exprent) -> bool {
    match (a.try_as_var(), b.try_as_var()) {
        (Some(x), Some(y)) => x.get_index() == y.get_index() && x.get_version() == y.get_version(),
        _ => false,
    }
}

/// `java.lang.String.format([locale,] fmt, Arrays.copyOf(v, v.length))`:
/// the call and `v`.
fn format_call(e: Exprent) -> Option<(InvocationExprent, Exprent)> {
    let inv = e.try_as_invocation()?;
    let locale = is_static_call(
        inv,
        &["java/lang/String"],
        "format",
        "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
    );
    if inv.d(|d| d.kuna_inlined.is_some())
        || !(locale
            || is_static_call(
                inv,
                &["java/lang/String"],
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
            ))
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    let copy = params.last()?.try_as_invocation()?;
    if !is_static_call(copy, &["java/util/Arrays"], "copyOf", "([Ljava/lang/Object;I)[Ljava/lang/Object;") {
        return None;
    }
    let copied = copy.get_lst_parameters().to_vec();
    let array = copied[0];
    array.try_as_var()?;
    let length = copied[1].try_as_function().filter(|f| f.get_func_type() == FunctionType::ArrayLength)?;
    same_var(length.get_lst_operands().get(0), array).then_some((inv, array))
}

/// The `String.format` copies whose call sits in the exprent list of `stat`.
fn format_block(stat: Statement, pops: &[i32], refs: &[(i32, usize)]) -> bool {
    let Some(list) = stat.get_exprents() else {
        return false;
    };
    let mut res = false;
    let mut i = 0;
    while i < list.len() {
        let top = list.get(i);
        let found = top
            .get_all_exprents_rec_self(true, true)
            .into_iter()
            .find_map(format_call);
        let Some((inv, array)) = found else {
            i += 1;
            continue;
        };
        let Some(removed) = rebuild_format(&list, i, inv, array, pops, refs) else {
            i += 1;
            continue;
        };
        res = true;
        i = i + 1 - removed;
    }
    res
}

/// Rebuilds the copy whose `format` call is in `list[at]`; returns how many
/// statements before it went.
fn rebuild_format(
    list: &crate::rt::JList<Exprent>,
    at: usize,
    inv: InvocationExprent,
    array: Exprent,
    pops: &[i32],
    refs: &[(i32, usize)],
) -> Option<usize> {
    let array_var = array.try_as_var()?;
    if refs_of(refs, array_var.get_index()) != 3 {
        return None;
    }
    let def_of = |v: Exprent, before: usize| {
        (0..before).rev().find(|&j| plain_assignment(list.get(j)).is_some_and(|(left, _)| same_var(left, v)))
    };
    let mut defs: Vec<usize> = vec![def_of(array, at)?];
    let mut built = plain_assignment(list.get(defs[0]))?.1;
    if let Some(copied) = built.try_as_var().filter(|v| !v.is_stack()) {
        if refs_of(refs, copied.get_index()) != 2 {
            return None;
        }
        defs.push(def_of(built, defs[0])?);
        built = plain_assignment(list.get(defs[1]))?.1;
    }
    let def_at = *defs.iter().min()?;
    let new = built.try_as_new()?;
    if new.get_new_type().array_dim != 1
        || new.get_new_type().value.as_deref() != Some("java/lang/Object")
        || (new.get_lst_array_elements().is_empty() && new.get_lst_dims().len() != 1)
    {
        return None;
    }
    if new.get_lst_array_elements().is_empty()
        && !new.get_lst_dims().get(0).try_as_const().is_some_and(|c| matches!(c.get_value(), Some(ConstValue::Int(0))))
    {
        return None;
    }
    let params = inv.get_lst_parameters().to_vec();
    let n = params.len();
    let mut temps: Vec<usize> = defs.clone();
    for p in &params[..n - 1] {
        if p.try_as_var().is_some_and(|v| !v.is_stack()) {
            if let Some(j) = (0..def_at).rev().find(|&j| {
                plain_assignment(list.get(j)).is_some_and(|(left, _)| same_var(left, *p))
            }) {
                temps.push(j);
            }
        }
    }
    let start = *temps.iter().min()?;
    if (start..at).any(|j| !temps.contains(&j)) {
        return None;
    }
    let companion_store = (start > 0 && is_companion_store(list.get(start - 1), refs)).then(|| start - 1);
    let message = check_message(list, at);
    let popped = companion_store.is_none()
        && (start..=at)
            .filter_map(|j| first_offset(list.get(j)))
            .min()
            .is_some_and(|low| pops.contains(&low));
    let companion = companion_store.is_some() || popped;
    let receiver_form = message
        .as_deref()
        .filter(|m| m.starts_with("java.lang.String.format("))
        .map(|m| m.ends_with("(this, *args)") || m.ends_with(", this, *args)"));
    if receiver_form.is_some_and(|r| r == companion) {
        return None;
    }
    let receiver = (!companion).then_some(if n == 3 { 1 } else { 0 });
    inv.get_lst_parameters().set(n - 1, built);
    inv.d_mut(|d| d.kuna_inlined = Some(Inlined::Format { receiver }));
    let mut gone = defs;
    gone.extend(companion_store);
    gone.sort_unstable();
    for j in gone.iter().rev() {
        list.remove_at(*j);
    }
    Some(gone.len())
}

/// `v = StringCompanionObject.INSTANCE`, `v` read nowhere: Kotlin 1.3's
/// store of `String.format`'s receiver.
fn is_companion_store(e: Exprent, refs: &[(i32, usize)]) -> bool {
    plain_assignment(e).is_some_and(|(left, right)| {
        left.try_as_var().is_some_and(|v| refs_of(refs, v.get_index()) == 1)
            && right.try_as_field().is_some_and(|f| {
                f.is_static() && f.get_classname() == STRING_COMPANION && f.get_name() == "INSTANCE"
            })
    })
}

/// The message of the expression-value check after `list[at]`, when it
/// checks the variable `list[at]` assigns.
fn check_message(list: &crate::rt::JList<Exprent>, at: usize) -> Option<String> {
    if at + 1 >= list.len() {
        return None;
    }
    let (left, _) = plain_assignment(list.get(at))?;
    let inv = list.get(at + 1).try_as_invocation()?;
    if !is_check(inv) || !same_var(inv.get_lst_parameters().get(0), left) {
        return None;
    }
    match inv.get_lst_parameters().get(1).try_as_const()?.get_value() {
        Some(ConstValue::String(m)) => Some(m.to_string()),
        _ => None,
    }
}

fn is_check(inv: InvocationExprent) -> bool {
    inv.is_static()
        && inv.get_classname() == INTRINSICS
        && CHECKS.contains(&inv.get_name().as_str())
        && inv.get_string_descriptor() == "(Ljava/lang/Object;Ljava/lang/String;)V"
}

/// Removes the expression-value checks of the stack variables that a
/// rebuilt call was just stored into.
fn remove_checks(stat: Statement) -> bool {
    let Some(list) = stat.get_exprents() else {
        return false;
    };
    let mut res = false;
    let mut i = 0;
    while i + 1 < list.len() {
        let rebuilt = plain_assignment(list.get(i)).is_some_and(|(left, right)| {
            left.try_as_var().is_some_and(|v| v.is_stack())
                && right.try_as_invocation().is_some_and(|inv| inv.d(|d| d.kuna_inlined.is_some()))
        });
        if rebuilt && check_message(&list, i).is_some() {
            list.remove_at(i + 1);
            res = true;
        }
        i += 1;
    }
    res
}

/// The call as Kotlin writes it, when it is a rebuilt copy.
pub fn write(inv: InvocationExprent, indent: i32) -> Option<TextBuffer> {
    if !kuna_fixes::enabled(KunaFix::KtInlinedStdlibCall) {
        return None;
    }
    let inlined = inv.d(|d| d.kuna_inlined)?;
    let params = inv.get_lst_parameters().to_vec();
    let mut buf = TextBuffer::new();
    match inlined {
        Inlined::Extension(name) => {
            let recv = if name == "copyOfRange" { unerased(params[0]) } else { params[0] };
            buf.append_buffer(&receiver(recv, indent)).append(".").append(name).append("(");
            let skipped = inv.d(|d| d.kuna_skip_arg);
            inv.set_kuna_skip_arg(Some(0));
            let list = inv.append_param_list(indent);
            inv.set_kuna_skip_arg(skipped);
            buf.append_buffer(&list).append(")");
        }
        Inlined::Map { param } => {
            buf.append_buffer(&write_map(&params, param, indent));
        }
        Inlined::Format { receiver: at } => {
            let (array, rest) = params.split_last()?;
            let mut args: Vec<TextBuffer> = Vec::new();
            match at {
                Some(at) => {
                    buf.append_buffer(&receiver(rest[at], indent)).append(".format(");
                    for (j, p) in rest.iter().enumerate() {
                        if j != at {
                            args.push(p.to_java(indent));
                        }
                    }
                }
                None => {
                    buf.append("String.format(");
                    for p in rest {
                        args.push(p.to_java(indent));
                    }
                }
            }
            for el in array.try_as_new()?.get_lst_array_elements().to_vec() {
                args.push(el.to_java(indent));
            }
            for (j, a) in args.iter().enumerate() {
                if j > 0 {
                    buf.append(", ");
                }
                buf.append_buffer(a);
            }
            buf.append(")");
        }
    }
    buf.add_bytecode_mapping(inv.get_bytecode().as_ref());
    Some(buf)
}

/// `{ param -> value }` of a rebuilt `map`.
fn write_map(params: &[Exprent], param: Option<VarVersionPair>, indent: i32) -> TextBuffer {
    let mut buf = TextBuffer::new();
    buf.append_buffer(&receiver(params[0], indent)).append(".map {");
    let name = param
        .and_then(|pair| {
            params[1].get_all_exprents_rec_self(true, true).into_iter().find(|e| pair_of(*e) == Some(pair))
        })
        .map(|v| v.as_var().get_name())
        .filter(|n| n != "it");
    if let Some(name) = name {
        buf.append(" ").append(&name).append(" ->");
    }
    buf.append(" ").append_buffer(&params[1].to_java(indent)).append(" }");
    buf
}

/// `a` for `a as Array<Any>`: kotlinc's cast of an object array to the
/// erased receiver of `Array<T>.copyOfRange`, which it casts again.
fn unerased(e: Exprent) -> Exprent {
    match e.try_as_function() {
        Some(f) if f.get_func_type() == FunctionType::Cast => {
            let ops = f.get_lst_operands().to_vec();
            let target = ops[1].get_expr_type();
            let from = ops[0].get_expr_type();
            let erased = target.array_dim == 1 && target.value.as_deref() == Some("java/lang/Object");
            if erased && (from.array_dim > 1 || (from.array_dim == 1 && from.type_ == CodeType::Object)) {
                ops[0]
            } else {
                e
            }
        }
        _ => e,
    }
}

/// The receiver of a call, parenthesised unless it is a primary expression.
fn receiver(e: Exprent, indent: i32) -> TextBuffer {
    let mut buf = e.to_java(indent);
    let negative = e.try_as_const().is_some_and(|c| match c.get_value() {
        Some(ConstValue::Int(v)) => v < 0,
        Some(ConstValue::Long(v)) => v < 0,
        _ => false,
    });
    if negative || e.get_precedence() > 0 {
        buf.enclose("(", ")");
    }
    buf
}

/// `closeFinally`'s owners and descriptors: `Closeable.use` and
/// `AutoCloseable.use` (Kotlin 1.x's in `kotlin.jdk7`).
const CLOSE_FINALLY: [(&str, &str); 3] = [
    ("kotlin/io/CloseableKt", "(Ljava/io/Closeable;Ljava/lang/Throwable;)V"),
    ("kotlin/AutoCloseableKt", "(Ljava/lang/AutoCloseable;Ljava/lang/Throwable;)V"),
    ("kotlin/jdk7/AutoCloseableKt", "(Ljava/lang/AutoCloseable;Ljava/lang/Throwable;)V"),
];

/// A `try`/`finally` written as `receiver.use { param -> body }`.
pub struct UseCall {
    pub receiver: Exprent,
    /// A variable that is the lambda's parameter, when the body reads it.
    pub param: Option<Exprent>,
    pub result: UseResult,
}

impl UseCall {
    /// The expressions the call holds outside the statement tree.
    pub fn exprents(&self) -> Vec<Exprent> {
        let mut res = vec![self.receiver];
        res.extend(self.param);
        if let UseResult::Assign(def) = self.result {
            res.push(def);
        }
        res
    }
}

/// Where the value of the `use` call goes.
pub enum UseResult {
    /// Nowhere: the lambda's body is written as it is.
    Unit,
    /// `return receiver.use { .. }`.
    Return,
    /// `v = receiver.use { .. }`, with `v` declared there when it is this
    /// definition.
    Assign(Exprent),
}

/// Before the class is written, once its nested classes have their
/// captured variables: rebuilds the copies that take a lambda in each
/// method of `wrapper`.
pub fn rebuild_lambdas(wrapper: ClassWrapper) {
    let cl = wrapper.get_class_struct();
    for mw in wrapper.get_methods().borrow().to_vec() {
        let (root, varproc, mt) = mw.with(|d| (d.root, d.varproc, d.method_struct));
        if let Some(root) = root {
            DecompilerContext::reset_method(mw);
            mark_uses(root, cl, mt, varproc);
        }
    }
}

/// Marks the inlined copies of `use` under `root` that are written as the
/// call, and rebuilds the one-expression `map`s.
pub fn mark_uses(root: RootStatement, cl: StructClass, mt: StructMethod, varproc: VarProcessor) {
    if !applies(cl) {
        return;
    }
    let mut cands: Vec<(Statement, CatchAllStatement)> = Vec::new();
    parents(root.0, &mut |parent, s| {
        if let Some(ca) = s.try_as_catch_all() {
            cands.push((parent, ca));
        }
    });
    for (parent, ca) in cands {
        if let Some(call) = match_use(root, parent, ca, mt, varproc) {
            ca.d_mut(|d| d.kuna_inlined_use = Some(Rc::new(call)));
        }
    }
    let mut loops: Vec<(Statement, Statement)> = Vec::new();
    parents(root.0, &mut |parent, s| {
        if s.try_as_do().is_some_and(|d| d.get_looptype() == LoopType::ForEach) {
            loops.push((parent, s));
        }
    });
    for (parent, lp) in loops {
        rebuild_map(root, parent, lp, mt, varproc);
    }
}

/// `val r = recv; val d = ArrayList(collectionSizeOrDefault(r, 10)); for
/// (item in r) d.add(value)` before the statement that reads `d as List`
/// first: kotlinc's `map` with a one-expression lambda.
fn rebuild_map(
    root: RootStatement,
    parent: Statement,
    lp: Statement,
    mt: StructMethod,
    varproc: VarProcessor,
) -> Option<()> {
    let do_stat = lp.try_as_do()?;
    if lp.is_labeled() {
        return None;
    }
    if parent.type_() != StatementType::Sequence {
        return None;
    }
    let siblings = parent.get_stats().borrow().to_vec();
    let at = siblings.iter().position(|s| *s == lp)?;
    let prev = *siblings.get(at.checked_sub(1)?)?;
    let next = *siblings.get(at + 1)?;
    let item = do_stat.get_init_exprent()?;
    let iterable = do_stat.get_inc_exprent()?;
    let body = lp.get_first()?;
    if body.type_() != StatementType::BasicBlock
        || body.get_all_successor_edges().iter().any(|e| e.get_destination() != lp)
    {
        return None;
    }
    let add = only_exprent(body)?.try_as_invocation()?;
    if add.is_static()
        || add.get_classname() != "java/util/Collection"
        || add.get_name() != "add"
        || add.get_string_descriptor() != "(Ljava/lang/Object;)Z"
    {
        return None;
    }
    let dest = add.get_instance()?;
    let (dpair, ipair, rpair) = (pair_of(dest)?, pair_of(item)?, pair_of(iterable)?);
    let temps = prev.get_exprents()?;
    let n = temps.len();
    if n < 2 {
        return None;
    }
    let (rleft, recv) = plain_assignment(temps.get(n - 2))?;
    let (dleft, created) = plain_assignment(temps.get(n - 1))?;
    if pair_of(rleft) != Some(rpair) || pair_of(dleft) != Some(dpair) {
        return None;
    }
    let created = strip_cast(created);
    let ctor = created.try_as_new()?.get_constructor()?;
    let size = ctor.get_lst_parameters().to_vec();
    let sized = created.as_new().get_new_type().value.as_deref() == Some("java/util/ArrayList")
        && ctor.get_string_descriptor() == "(I)V"
        && size.len() == 1
        && size[0].try_as_invocation().is_some_and(|c| {
            is_static_call(
                c,
                &["kotlin/collections/CollectionsKt"],
                "collectionSizeOrDefault",
                "(Ljava/lang/Iterable;I)I",
            ) && pair_of(c.get_lst_parameters().get(0)) == Some(rpair)
                && c.get_lst_parameters().get(1).try_as_const().is_some_and(|k| matches!(k.get_value(), Some(ConstValue::Int(10))))
        });
    if !sized || occurrences(root.0, rpair) != 3 || occurrences(root.0, dpair) != 3 {
        return None;
    }
    let value = add.get_lst_parameters().get(0);
    if value.get_all_exprents_rec_self(true, true).into_iter().any(|e| {
        e.try_as_assignment().is_some()
            || e.try_as_function().is_some_and(|f| {
                matches!(f.get_func_type(), FunctionType::Ipp | FunctionType::Imm | FunctionType::Ppi | FunctionType::Mmi)
            })
    }) {
        return None;
    }
    let mut casts: Vec<Exprent> = Vec::new();
    let element = element_type(rleft, recv)?;
    let expected = |t: &VarType| same_class(t, &element) || (boxed_number(&element) && t.value.as_deref() == Some("java/lang/Number"));
    if !casts_of(body, ipair, &expected, &mut casts) || occurrences(root.0, ipair) != 1 + occurrences(body, ipair) {
        return None;
    }
    let (holder, cast) = list_use(next, dpair)?;
    let size_call = size[0];
    let call = size_call.copy().as_invocation();
    call.set_name("map".to_string());
    let descriptor = "(Ljava/lang/Iterable;Lkotlin/jvm/functions/Function1;)Ljava/util/List;";
    call.set_string_descriptor(descriptor.to_string());
    let typed = cast.get_inferred_expr_type(None);
    call.set_descriptor(MethodDescriptor::parse_descriptor(descriptor).with_ret(typed));
    let used = occurrences(body, ipair) > 0;
    let value = if casts.contains(&value) { value.as_function().get_lst_operands().get(0) } else { value };
    for cast in &casts {
        if *cast != value {
            if let Some(parent) = parent_of(body, *cast) {
                parent.replace_exprent(*cast, cast.as_function().get_lst_operands().get(0));
            }
        }
    }
    call.set_lst_parameters(vec![strip_iterable_cast(recv), value]);
    if used {
        if let Some(name) = lambda_marker_parameter(mt, body, "$i$a$-map-") {
            if !shadows(body, ipair, &name) && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                rename(varproc, ipair, name);
                for e in value.get_all_exprents_rec_self(true, true) {
                    if pair_of(e) == Some(ipair) {
                        e.as_var().set_lvt(None);
                    }
                }
            }
        }
    }
    call.d_mut(|d| d.kuna_inlined = Some(Inlined::Map { param: used.then_some(ipair) }));
    holder.replace_exprent(cast, call.0);
    temps.remove_at(n - 1);
    temps.remove_at(n - 2);
    let empty = BasicBlockStatement::create();
    parent.replace_statement(lp, empty.0);
    Some(())
}

fn strip_iterable_cast(e: Exprent) -> Exprent {
    match e.try_as_function() {
        Some(f)
            if f.get_func_type() == FunctionType::Cast
                && f.get_lst_operands().get(1).get_expr_type().value.as_deref() == Some("java/lang/Iterable") =>
        {
            f.get_lst_operands().get(0)
        }
        _ => e,
    }
}

/// In the first expression `next` evaluates, `d as List` with only
/// constants and variables evaluated before it, which the lambda cannot
/// change: the cast and the expression that holds it.
fn list_use(next: Statement, dpair: VarVersionPair) -> Option<(Exprent, Exprent)> {
    let block = first_block(next)?;
    let top = block.get_exprents()?.first()?;
    let mut found: Option<(Exprent, Exprent)> = None;
    let mut effect = false;
    walk_until(top, &mut |parent, e| {
        if let Some(f) = e.try_as_function() {
            if f.get_func_type() == FunctionType::Cast
                && pair_of(f.get_lst_operands().get(0)) == Some(dpair)
                && f.get_lst_operands().get(1).get_expr_type().value.as_deref() == Some("java/util/List")
            {
                found = parent.map(|p| (p, e));
                return true;
            }
        }
        false
    }, &mut effect);
    if effect {
        return None;
    }
    found
}

/// Walks `e` in evaluation order until `hit` says stop; `effect` is set
/// when anything but a constant or a variable completed before that.
fn walk_until(e: Exprent, hit: &mut dyn FnMut(Option<Exprent>, Exprent) -> bool, effect: &mut bool) -> bool {
    fn go(parent: Option<Exprent>, e: Exprent, hit: &mut dyn FnMut(Option<Exprent>, Exprent) -> bool, effect: &mut bool) -> bool {
        if hit(parent, e) {
            return true;
        }
        let target = e.try_as_assignment().map(|a| a.get_left()).filter(|l| l.try_as_field().is_some());
        for child in e.get_all_exprents() {
            if Some(child) == target {
                for inner in child.get_all_exprents() {
                    if go(Some(child), inner, hit, effect) {
                        return true;
                    }
                }
            } else if go(Some(e), child, hit, effect) {
                return true;
            }
        }
        if e.try_as_const().is_none() && e.try_as_var().is_none() {
            *effect = true;
        }
        false
    }
    go(None, e, hit, effect)
}

/// Each statement under `stat` with the statement that holds it.
fn parents(stat: Statement, f: &mut dyn FnMut(Statement, Statement)) {
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        parents(child, f);
        f(stat, child);
    }
}

fn statements(stat: Statement, f: &mut dyn FnMut(Statement)) {
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        statements(child, f);
    }
    f(stat);
}

fn pair_of(e: Exprent) -> Option<VarVersionPair> {
    e.try_as_var().map(VarVersionPair::from_var)
}

/// Every occurrence of a variable under `stat`, by its version pair.
fn occurrences(stat: Statement, pair: VarVersionPair) -> usize {
    let mut n = 0;
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if pair_of(e) == Some(pair) {
                n += 1;
            }
        }
    });
    n
}

fn match_use(
    root: RootStatement,
    parent: Statement,
    ca: CatchAllStatement,
    mt: StructMethod,
    varproc: VarProcessor,
) -> Option<UseCall> {
    let (is_finally, monitor, handler) = ca.d(|d| (d.is_finally, d.monitor, d.handler));
    if !is_finally || monitor.is_some() || ca.0.is_labeled() || ca.d(|d| d.kuna_inlined_use.is_some()) {
        return None;
    }
    let closing = only_exprent(handler?)?.try_as_invocation()?;
    if !CLOSE_FINALLY.iter().any(|(owner, desc)| is_static_call(closing, &[owner], "closeFinally", desc)) {
        return None;
    }
    let args = closing.get_lst_parameters().to_vec();
    let (rpair, epair) = (pair_of(args[0])?, pair_of(args[1])?);
    let first = ca.0.get_first()?;
    let tc = first.try_as_catch()?;
    let stats = first.get_stats().borrow().to_vec();
    if first.is_labeled()
        || stats.len() != 2
        || tc.get_exct_strings().to_vec() != vec![vec!["java/lang/Throwable".to_string()]]
        || !tc.get_resources().is_empty()
    {
        return None;
    }
    let (body, catcher) = (stats[0], stats[1]);
    let caught = pair_of(tc.get_vars().get(0).0)?;
    let rethrow = catcher.get_exprents()?.to_vec();
    let rethrows = rethrow.len() == 2
        && plain_assignment(rethrow[0]).is_some_and(|(l, r)| pair_of(l) == Some(epair) && pair_of(r) == Some(caught))
        && rethrow[1].try_as_exit().is_some_and(|x| {
            x.get_exit_type() == ExitType::Throw && x.get_value().and_then(pair_of) == Some(caught)
        });
    if !rethrows {
        return None;
    }
    if parent.type_() != StatementType::Sequence {
        return None;
    }
    let siblings = parent.get_stats().borrow().to_vec();
    let at = siblings.iter().position(|s| *s == ca.0)?;
    let prev = *siblings.get(at.checked_sub(1)?)?;
    let temps = prev.get_exprents()?;
    let n = temps.len();
    if n < 2 {
        return None;
    }
    let (rleft, recv) = plain_assignment(temps.get(n - 2))?;
    let (eleft, null) = plain_assignment(temps.get(n - 1))?;
    let is_null = |e: Exprent| {
        e.try_as_const().is_some_and(|c| c.is_null())
            || e.try_as_function().is_some_and(|f| {
                f.get_func_type() == FunctionType::Cast
                    && f.get_lst_operands().get(0).try_as_const().is_some_and(|c| c.is_null())
            })
    };
    if pair_of(rleft) != Some(rpair) || pair_of(eleft) != Some(epair) || !is_null(null) {
        return None;
    }
    if occurrences(root.0, epair) != 3 || leaves_body(body) {
        return None;
    }
    let in_body = occurrences(body, rpair);
    if occurrences(root.0, rpair) != in_body + 2 {
        return None;
    }
    let first_block = first_block(body)?;
    let kept_param = first_block.get_exprents().and_then(|l| l.first()).and_then(|e| {
        let (left, right) = plain_assignment(e)?;
        let p = left.try_as_var().filter(|v| !v.is_stack())?;
        let read = strip_cast(right);
        let cast_ok = read == right || receiver_typed(recv, &right.as_function().get_lst_operands().get(1).get_expr_type());
        (pair_of(read) == Some(rpair) && in_body == 1 && cast_ok).then_some(p)
    });
    let mut casts: Vec<Exprent> = Vec::new();
    let expected = |t: &VarType| receiver_typed(recv, t);
    if kept_param.is_none() && !casts_of(body, rpair, &expected, &mut casts) {
        return None;
    }
    let name = match kept_param {
        Some(_) => None,
        None if in_body == 0 => None,
        None => Some(lambda_parameter(mt, body).unwrap_or_else(|| "it".to_string())),
    };
    if name.as_deref().is_some_and(|p| !p.chars().all(|c| c.is_alphanumeric() || c == '_')) {
        return None;
    }
    let tail = last_block(body).and_then(|b| Some((b, b.get_exprents()?.last()?)));
    let mut result = UseResult::Unit;
    if let Some((last_block, tail)) = tail {
        let returns_inside = returns_before_handler(mt, body, &rethrow);
        result = use_result(root, ca, body, last_block, tail, returns_inside);
    }
    let mut param = kept_param.map(|p| p.0);
    if kept_param.is_some() {
        first_block.get_exprents()?.remove_at(0);
    }
    for cast in casts {
        if let Some(parent) = parent_of(body, cast) {
            parent.replace_exprent(cast, cast.as_function().get_lst_operands().get(0));
        } else if let Some(list) = owning_list(body, cast) {
            let i = list.index_of(&cast);
            if i >= 0 {
                list.set(i as usize, cast.as_function().get_lst_operands().get(0));
            }
        }
    }
    if let Some(name) = name {
        if !shadows(body, rpair, &name) {
            rename(varproc, rpair, name);
            for_each_exprent(body, &mut |top| {
                for e in top.get_all_exprents_rec_self(true, true) {
                    if pair_of(e) == Some(rpair) {
                        e.as_var().set_lvt(None);
                    }
                }
            });
        }
        param = occurrence(body, rpair);
    }
    temps.remove_at(n - 1);
    temps.remove_at(n - 2);
    let receiver = strip_closeable_cast(recv);
    Some(UseCall { receiver, param, result })
}

/// Where the value of the copy goes: the body's last statement returns it
/// (and no other does) or stores it into a variable declared at the `try`.
/// A `return` is the `use` call's value when kotlinc returns after the
/// handlers (`return r.use { .. }`), not from the body (`r.use { return ..
/// }`, `returns_inside`).
fn use_result(
    root: RootStatement,
    ca: CatchAllStatement,
    body: Statement,
    last_block: Statement,
    tail: Exprent,
    returns_inside: bool,
) -> UseResult {
    if let Some(exit) = tail.try_as_exit() {
        if exit.get_exit_type() == ExitType::Return && returns(body) == 1 && !returns_inside {
            if let Some(value) = exit.get_value() {
                replace_last(last_block, value);
                return UseResult::Return;
            }
        }
    } else if let Some((left, value)) = plain_assignment(tail) {
        let defined = left.try_as_var().filter(|v| !v.is_stack()).and_then(|v| {
            let pair = VarVersionPair::from_var(v);
            let defs = ca.0.get_var_definitions();
            let def = defs.to_vec().into_iter().find(|d| pair_of(*d) == Some(pair))?;
            (occurrences(body, pair) == 1).then_some(def)
        });
        if let Some(def) = defined {
            ca.0.get_var_definitions().remove_item(&def);
            replace_last(last_block, value);
            return UseResult::Assign(def);
        }
        if left.try_as_var().is_some_and(|v| !v.is_stack() && occurrences(root.0, VarVersionPair::from_var(v)) == 1) {
            replace_last(last_block, value);
        }
    }
    UseResult::Unit
}

/// Names the variable `pair` `name`, over its debug name.
fn rename(varproc: VarProcessor, pair: VarVersionPair, name: String) {
    varproc.set_var_name(pair, name.clone());
    varproc.set_clashing_name(pair, name);
}

/// Whether naming the variable `pair` `name` would make it shadow, or be
/// shadowed by, a variable or lambda parameter written in `body`.
fn shadows(body: Statement, pair: VarVersionPair, name: &str) -> bool {
    let mut res = false;
    for_each_exprent(body, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(v) = e.try_as_var() {
                if pair_of(e) != Some(pair) && v.get_name() == name {
                    res = true;
                }
            } else if e.try_as_new().is_some_and(|n| n.is_lambda() || n.is_anonymous()) {
                res = true;
            }
        }
    });
    res
}

fn occurrence(stat: Statement, pair: VarVersionPair) -> Option<Exprent> {
    let mut found = None;
    for_each_exprent(stat, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if found.is_none() && pair_of(e) == Some(pair) {
                found = Some(e);
            }
        }
    });
    found
}

fn only_exprent(stat: Statement) -> Option<Exprent> {
    let list = stat.get_exprents()?;
    (list.len() == 1).then(|| list.get(0))
}

fn strip_cast(e: Exprent) -> Exprent {
    match e.try_as_function() {
        Some(f) if f.get_func_type() == FunctionType::Cast => f.get_lst_operands().get(0),
        _ => e,
    }
}

fn strip_closeable_cast(e: Exprent) -> Exprent {
    match e.try_as_function() {
        Some(f)
            if f.get_func_type() == FunctionType::Cast
                && matches!(
                    f.get_lst_operands().get(1).get_expr_type().value.as_deref(),
                    Some("java/io/Closeable") | Some("java/lang/AutoCloseable")
                ) =>
        {
            f.get_lst_operands().get(0)
        }
        _ => e,
    }
}

/// Whether a jump leaves `body` other than by falling through its end or
/// by `return`: a labelled `break` or a `continue` of an enclosing loop.
fn leaves_body(body: Statement) -> bool {
    let mut leaves = false;
    statements(body, &mut |s| {
        for edge in s.get_all_successor_edges() {
            if edge.get_type() == StatEdge::TYPE_EXCEPTION {
                continue;
            }
            let dest = edge.get_destination();
            if body.contains_statement(dest) {
                continue;
            }
            if edge.explicit() && edge.get_type() != StatEdge::TYPE_REGULAR {
                if dest.type_() != StatementType::DummyExit {
                    leaves = true;
                }
            }
        }
    });
    leaves
}

/// The `return` exits under `body`.
fn returns(body: Statement) -> usize {
    let mut n = 0;
    for_each_exprent(body, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if e.try_as_exit().is_some_and(|x| x.get_exit_type() == ExitType::Return) {
                n += 1;
            }
        }
    });
    n
}

fn first_block(stat: Statement) -> Option<Statement> {
    if stat.type_() == StatementType::BasicBlock {
        return Some(stat);
    }
    if matches!(stat.type_(), StatementType::Sequence | StatementType::If) {
        return first_block(stat.get_first()?);
    }
    None
}

fn last_block(stat: Statement) -> Option<Statement> {
    if stat.type_() == StatementType::BasicBlock {
        return Some(stat);
    }
    if stat.type_() == StatementType::Sequence {
        let last = stat.get_stats().borrow().to_vec().last().copied()?;
        return last_block(last);
    }
    None
}

fn replace_last(block: Statement, value: Exprent) {
    if let Some(list) = block.get_exprents() {
        let n = list.len();
        list.set(n - 1, value);
    }
}

/// Collects the casts of the variable `pair` under `body` into `casts`;
/// false when it is cast to more than one type or to a type `expected`
/// rejects (a cast the lambda's own code made).
fn casts_of(
    body: Statement,
    pair: VarVersionPair,
    expected: &dyn Fn(&VarType) -> bool,
    casts: &mut Vec<Exprent>,
) -> bool {
    let mut target: Option<VarType> = None;
    let mut ok = true;
    for_each_exprent(body, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            let Some(f) = e.try_as_function().filter(|f| f.get_func_type() == FunctionType::Cast) else {
                continue;
            };
            let ops = f.get_lst_operands().to_vec();
            if pair_of(ops[0]) != Some(pair) {
                continue;
            }
            let typed = ops[1].get_expr_type();
            if !expected(&typed) {
                ok = false;
            }
            match &target {
                Some(t) if !t.equals(&typed) => ok = false,
                _ => target = Some(typed),
            }
            casts.push(e);
        }
    });
    ok
}

/// Whether `t` is the type of the receiver `recv` (`r` of `r as
/// Closeable`): its descriptor's or its inferred generic one, kotlinc's `T`.
fn receiver_typed(recv: Exprent, t: &VarType) -> bool {
    let r = strip_closeable_cast(recv);
    same_class(t, &r.get_expr_type()) || same_class(t, &r.get_inferred_expr_type(None))
}

fn same_class(a: &VarType, b: &VarType) -> bool {
    a.type_ == b.type_ && a.array_dim == b.array_dim && a.value == b.value
}

fn boxed_number(t: &VarType) -> bool {
    matches!(
        t.value.as_deref(),
        Some("java/lang/Integer" | "java/lang/Long" | "java/lang/Short" | "java/lang/Byte" | "java/lang/Float" | "java/lang/Double")
    )
}

/// The element type `E` of the `Iterable<E>` a `map` copy iterates: the
/// first type argument of the receiver variable's declared type, else of
/// the receiver's.
fn element_type(var: Exprent, recv: Exprent) -> Option<VarType> {
    let declared = var.as_var().kuna_declared().map(|k| k.type_);
    let candidates = [declared, Some(recv.get_inferred_expr_type(None)), Some(strip_cast(recv).get_inferred_expr_type(None))];
    candidates
        .into_iter()
        .flatten()
        .find_map(|t| t.try_as_generic()?.get_arguments().first().cloned().flatten())
}

fn parent_of(body: Statement, child: Exprent) -> Option<Exprent> {
    let mut found = None;
    for_each_exprent(body, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if e.get_all_exprents().contains(&child) {
                found = Some(e);
            }
        }
    });
    found
}

fn owning_list(body: Statement, child: Exprent) -> Option<crate::rt::JList<Exprent>> {
    let mut found = None;
    blocks(body, &mut |b| {
        if let Some(list) = b.get_exprents() {
            if list.contains(&child) {
                found = Some(list);
            }
        }
    });
    found
}

/// The name kotlinc gave the `use` lambda's parameter: the local that ends
/// with the lambda's `$i$a$-use-` marker and starts just before it.
fn lambda_parameter(mt: StructMethod, body: Statement) -> Option<String> {
    lambda_marker_parameter(mt, body, "$i$a$-use-")
}

/// The name kotlinc gave the parameter of the lambda whose marker starts
/// with `marker` within `body`'s instructions.
fn lambda_marker_parameter(mt: StructMethod, body: Statement, marker: &str) -> Option<String> {
    let table = mt.get_local_variable_attr()?;
    let mut offsets: Vec<i32> = Vec::new();
    for_each_exprent(body, &mut |top| {
        for e in top.get_all_exprents_rec_self(true, true) {
            if let Some(bits) = e.get_bytecode() {
                offsets.extend(bits.iter().map(|b| b as i32));
            }
        }
    });
    let (lo, hi) = (*offsets.iter().min()?, *offsets.iter().max()?);
    let vars = table.get_variables();
    let marker = vars.iter().find(|v| {
        v.name.starts_with(marker) && v.start_pc >= lo - 8 && v.start_pc <= hi
    })?;
    let end = marker.start_pc + marker.length;
    vars.iter()
        .filter(|v| v.start_pc + v.length == end && v.start_pc < marker.start_pc && !v.name.starts_with('$'))
        .max_by_key(|v| v.start_pc)
        .map(|v| v.name.clone())
}

/// The statement as Kotlin writes it, when it is a marked copy of `use`.
pub fn write_use(ca: CatchAllStatement, indent: i32) -> Option<TextBuffer> {
    if !kuna_fixes::enabled(KunaFix::KtInlinedStdlibCall) {
        return None;
    }
    let call = ca.d(|d| d.kuna_inlined_use.clone())?;
    let body = ca.0.get_first()?.get_first()?;
    let mut buf = TextBuffer::new();
    buf.append_buffer(&ExprProcessor::list_to_java(Some(&ca.0.get_var_definitions()), indent));
    buf.append_indent(indent);
    match &call.result {
        UseResult::Unit => {}
        UseResult::Return => {
            buf.append("return ");
        }
        UseResult::Assign(def) => {
            buf.append_buffer(&def.to_java(indent)).append(" = ");
        }
    }
    buf.append_buffer(&receiver(call.receiver, indent)).append(".use {");
    let name = call.param.map(|p| p.as_var().get_name()).filter(|p| p != "it");
    if let Some(p) = name {
        buf.append(" ").append(&p).append(" ->");
    }
    buf.append_line_separator();
    buf.append_buffer(&ExprProcessor::jmp_wrapper(body, indent + 1, true));
    buf.append_indent(indent).append("}").append_line_separator();
    Some(buf)
}
