//! Fix `kt_suspend_lambda`: a `SuspendLambda` class is written as its lambda:
//! `invokeSuspend` is rebuilt like a suspend function's state machine, the
//! reads of its arguments from the `L$n` fields become its parameters, and its
//! exits become the lambda's.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::classes_processor::ClassNode;
use crate::code_constants::*;
use crate::control_flow_graph::ControlFlowGraph;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::key::Key;
use crate::fixes::kt_suspend::kt_suspend_state_machine::{
    apply, require, resume_block, suspension_points, Code, Machine, Plan, Refusal, RESUME_ERROR, RESUME_PATH,
};
use crate::fixes::kt_suspend::CONTINUATION;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::kotlin_chooser::KotlinChooser;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::k_types::KTypes;
use crate::fixes::kt_jumps;
use crate::kt_attrs::KtAttrs;
use crate::stat_edge::StatEdge;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const LAMBDA_CLASSES: [&str; 2] = [
    "kotlin/coroutines/jvm/internal/SuspendLambda",
    "kotlin/coroutines/jvm/internal/RestrictedSuspendLambda",
];
const INVOKE_SUSPEND: (&str, &str) = ("invokeSuspend", "(Ljava/lang/Object;)Ljava/lang/Object;");
const FUNCTION_PREFIX: &str = "kotlin/jvm/functions/Function";

/// Whether `cl` is the class of a suspend lambda.
pub fn is_suspend_lambda(cl: StructClass) -> bool {
    cl.super_class()
        .is_some_and(|s| LAMBDA_CLASSES.contains(&&*s.get_string()))
}

/// The method holding the body of the suspend lambda `cl`.
pub fn body_method(cl: StructClass) -> Option<StructMethod> {
    if !is_suspend_lambda(cl) {
        return None;
    }
    cl.get_method(INVOKE_SUSPEND.0, INVOKE_SUSPEND.1)
}

/// The member attribute of a rebuilt lambda body: the slots of the locals
/// its arguments are read into, by the name the local table gives them.
#[allow(non_snake_case)]
fn PARAMETERS() -> Key<Rc<BTreeMap<i32, String>>> {
    Key::of("KunaSuspendLambdaParameters")
}

/// The locals the arguments of the rebuilt lambda body `mt` are read into,
/// by slot, with their names.
pub fn parameters(mt: StructMethod) -> Option<Rc<BTreeMap<i32, String>>> {
    KtAttrs::get(mt, &PARAMETERS())
}

/// Rewrites the graph of `mt`, when it is the body of the suspend lambda
/// `cl`, into its direct path; whether it did. The lambda's metadata is read
/// here, before the body is decompiled.
pub fn rebuild_body(graph: ControlFlowGraph, mt: StructMethod, cl: StructClass) -> bool {
    if body_method(cl) != Some(mt) {
        return false;
    }
    KotlinChooser::parse_metadata_for(cl);
    let Some(function) = lambda_function(cl) else {
        return false;
    };
    let mut names: Vec<Option<String>> = Vec::new();
    if function.receiver_type.is_some() {
        names.push(None);
    }
    names.extend(function.parameters.iter().map(|p| Some(p.name.clone())));
    rebuild(graph, mt, cl, &names)
}

/// The metadata function of the suspend lambda `cl`, when fix
/// `kt_suspend_lambda` gave it `invokeSuspend` as its method.
pub fn lambda_function(cl: StructClass) -> Option<Rc<KFunction>> {
    match KtAttrs::get(cl, &KElement::KEY()) {
        Some(KElement::Function(function)) if function.method_struct.is_some() && body_method(cl) == function.method_struct => {
            Some(function)
        }
        _ => None,
    }
}

/// Whether the suspend lambda `cl` was rebuilt and is written as a lambda.
pub fn is_rebuilt(cl: StructClass) -> bool {
    body_method(cl).is_some_and(|mt| parameters(mt).is_some())
}

fn rebuild(graph: ControlFlowGraph, mt: StructMethod, cl: StructClass, names: &[Option<String>]) -> bool {
    let seq = graph.get_sequence();
    let code = Code {
        insns: seq.instructions(),
        pool: cl.get_pool(),
        handlers: seq
            .exception_table()
            .get_handlers()
            .iter()
            .map(|h| (h.from() as usize, h.to() as usize, h.handler() as usize, h.exception_class()))
            .collect(),
    };
    let local_name = |slot: i32, after: usize| {
        let offset = code.insns.get(after).map(|x| x.start_offset())?;
        mt.get_local_variable_attr()?.get_name(slot, offset)
    };
    match plan(&code, &cl.qualified_name(), names, &local_name) {
        Ok((plan, parameters)) => {
            let r = apply(graph, &code, &plan, 0);
            if r {
                let path: Rc<[usize]> = plan.resume_path.iter().copied().collect();
                KtAttrs::put(mt, &RESUME_PATH(), path);
                KtAttrs::put(mt, &PARAMETERS(), Rc::new(parameters));
                REBUILT.with(|r| r.set(true));
            }
            r
        }
        Err(_) => false,
    }
}

/// Whether `field` is one of the fields kotlinc gives a continuation's
/// machine: `label` and the spill fields (`L$0`, `I$1`, ..).
fn is_machine_field(field: &str) -> bool {
    if field == "label" || field == "result" {
        return true;
    }
    let mut chars = field.chars();
    matches!(chars.next(), Some('L' | 'I' | 'J' | 'F' | 'D' | 'Z' | 'B' | 'S' | 'C'))
        && chars.next() == Some('$')
        && {
            let rest: String = chars.collect();
            !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())
        }
}

/// The plan for `code`, the body of the suspend lambda `class`, and the
/// locals its arguments are read into.
pub fn plan(
    code: &Code,
    class: &str,
    names: &[Option<String>],
    local_name: &dyn Fn(i32, usize) -> Option<String>,
) -> Result<(Plan, BTreeMap<i32, String>), Refusal> {
    let mut parameters: BTreeMap<i32, (String, usize)> = BTreeMap::new();
    let mut owned: BTreeSet<usize> = BTreeSet::new();
    let start = read_arguments(code, class, names, local_name, 0, &mut parameters, &mut owned)?;
    require(code.is_suspended_marker(start), "entry")?;
    let marker = if code.is(start + 1, OPC_ASTORE) {
        code.operand(start + 1)
    } else {
        require(code.is(start + 1, OPC_POP), "entry")?;
        -1
    };
    require(
        code.is_local(start + 2, OPC_ALOAD, 0) && code.is_member(start + 3, OPC_GETFIELD, class, "label"),
        "dispatch",
    )?;
    let switch_at = start + 4;
    let switch = code
        .insns
        .get(switch_at)
        .and_then(|x| x.try_as_switch())
        .ok_or("dispatch")?;
    let values = switch.get_values();
    let targets: Vec<usize> = switch.get_destinations().iter().map(|&d| d as usize).collect();
    let labels = values.len();
    require(
        labels >= 1 && values.iter().enumerate().all(|(k, &v)| v == k as i32),
        "dispatch labels",
    )?;
    let default_at = switch.get_default_destination() as usize;
    let machine = Machine {
        class: class.to_string(),
        cont: 0,
        result: 1,
        marker,
    };
    require(
        code.class(default_at, OPC_NEW).as_deref() == Some("java/lang/IllegalStateException")
            && code.is(default_at + 1, OPC_DUP)
            && code.is_string(default_at + 2, RESUME_ERROR)
            && code.is_member(default_at + 3, OPC_INVOKESPECIAL, "java/lang/IllegalStateException", "<init>")
            && code.is(default_at + 4, OPC_ATHROW)
            && code.is_throw_on_failure(targets[0], machine.result),
        "dispatch default",
    )?;

    let mut plan = Plan::default();
    plan.removed.extend(start..=switch_at);
    plan.removed.extend([targets[0], targets[0] + 1]);
    owned.extend(start..=switch_at);
    owned.extend([targets[0], targets[0] + 1]);
    owned.extend(default_at..default_at + 5);
    if start == 0 {
        plan.entry = Some(targets[0]);
    } else {
        plan.dispatch = Some((switch_at, targets[0]));
    }
    read_arguments(code, class, names, local_name, targets[0] + 2, &mut parameters, &mut owned)?;

    let mut resumes: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    for (label, &start) in targets.iter().enumerate().skip(1) {
        let mut block = BTreeSet::new();
        let (failure, after) = resume_block(code, &machine, start, &mut block)?;
        resumes.insert(start, (label, failure, after));
        owned.extend(block.iter().copied());
        plan.resume_path.extend(block);
    }

    for (&slot, &(_, store)) in &parameters {
        require(
            !code.insns.iter().enumerate().any(|(k, x)| {
                k != store
                    && !plan.resume_path.contains(&k)
                    && (code.is_store(k) || x.opcode() == OPC_IINC)
                    && x.operand(0) == slot
            }),
            "argument local reused",
        )?;
    }
    let parameters: BTreeMap<i32, String> = parameters.into_iter().map(|(slot, (name, _))| (slot, name)).collect();

    suspension_points(code, &machine, &resumes, labels, is_machine_field, &mut plan, &mut owned, |j, plan, owned| {
        let cast = j >= 2 && code.is_continuation_cast(j - 1);
        let load = if cast { j - 2 } else { j.wrapping_sub(1) };
        require(j > 0 && code.is_local(load, OPC_ALOAD, 0), "continuation is not the call's last argument")?;
        if cast {
            plan.removed.insert(j - 1);
            owned.insert(j - 1);
        }
        owned.insert(load);
        Ok(())
    })?;

    for i in 0..code.insns.len() {
        if owned.contains(&i) {
            continue;
        }
        let touches_local = (code.is_load(i) || code.is_store(i) || code.is(i, OPC_IINC))
            && (code.operand(i) == machine.result || (marker >= 0 && code.operand(i) == marker));
        let touches_machine = (code.is(i, OPC_GETFIELD) || code.is(i, OPC_PUTFIELD))
            && code
                .member(i)
                .is_some_and(|(owner, f, _)| owner == class && is_machine_field(&f));
        let this_escapes = code.is_local(i, OPC_ALOAD, 0)
            && !(code.is(i + 1, OPC_GETFIELD) && code.member(i + 1).is_some_and(|(owner, _, _)| owner == class))
            && !(code.is_continuation_cast(i + 1)
                && code.is_member(i + 2, OPC_INVOKEINTERFACE, CONTINUATION, "getContext"))
            && !code.is_member(i + 1, OPC_INVOKEINTERFACE, CONTINUATION, "getContext")
            && !code.is_member(i + 1, OPC_INVOKEVIRTUAL, class, "getContext");
        require(
            !touches_local && !touches_machine && !this_escapes && !code.is_suspended_marker(i),
            "continuation used by other code",
        )?;
    }
    Ok((plan, parameters))
}

/// Reads the arguments the body reads from `i` on, `this.L$0` (cast, and
/// unboxed for an inline class) stored into the local the metadata's
/// parameter's name names, into `parameters`; the index after them.
fn read_arguments(
    code: &Code,
    class: &str,
    names: &[Option<String>],
    local_name: &dyn Fn(i32, usize) -> Option<String>,
    mut i: usize,
    parameters: &mut BTreeMap<i32, (String, usize)>,
    owned: &mut BTreeSet<usize>,
) -> Result<usize, Refusal> {
    while code.is_local(i, OPC_ALOAD, 0)
        && code.is(i + 1, OPC_GETFIELD)
        && code
            .member(i + 1)
            .is_some_and(|(o, f, _)| o == class && is_machine_field(&f))
    {
        let cast = usize::from(code.is(i + 2, OPC_CHECKCAST));
        let unbox = usize::from(
            cast == 1
                && code.is(i + 3, OPC_INVOKEVIRTUAL)
                && code.member(i + 3).is_some_and(|(owner, name, d)| {
                    Some(&owner) == code.class(i + 2, OPC_CHECKCAST).as_ref() && name == "unbox-impl" && d.starts_with("()")
                }),
        );
        let store = i + 2 + cast + unbox;
        require(code.is_store(store), "argument read")?;
        let slot = code.operand(store);
        let name = local_name(slot, store + 1).ok_or("argument name")?;
        let known = names.iter().any(|n| match n {
            Some(n) => *n == name,
            None => is_receiver_name(&name),
        });
        require(known && !parameters.values().any(|(n, _)| *n == name), "argument name")?;
        parameters.insert(slot, (name, store));
        owned.extend(i..=store);
        i = store + 1;
    }
    Ok(i)
}

/// Leaves out, under `root`, the body of the rebuilt lambda `mt`, the reads
/// of its arguments into their locals (`s = this.L$0 as String`), and, when
/// the lambda returns `Unit`, the `return Unit` that ends it.
pub fn remove_argument_reads(root: RootStatement, mt: StructMethod) -> bool {
    let Some(parameters) = parameters(mt) else {
        return false;
    };
    let mut res = false;
    remove_under(root.0, &parameters, &mut res);
    if let Some(label) = receiver_label(mt) {
        KtAttrs::put(mt, &LABEL(), label);
        LABELLED.with(|l| l.set(true));
        res = true;
    }
    let returns_unit = DecompilerContext::get_struct_context()
        .get_class(&mt.get_class_qualified_name())
        .and_then(lambda_function)
        .is_some_and(|f| f.return_type.k().kotlin_type == KTypes::UNIT && !f.return_type.k().is_nullable);
    if returns_unit {
        for edge in root.get_dummy_exit().0.get_predecessor_edges(StatEdge::TYPE_BREAK) {
            let source = edge.get_source();
            let Some(exprents) = source.get_exprents() else {
                continue;
            };
            let Some(last) = exprents.last() else {
                continue;
            };
            if is_return_unit(last) && kt_jumps::in_tail_position(source) {
                exprents.remove_at(exprents.len() - 1);
                res = true;
            }
        }
    }
    res
}

fn is_return_unit(e: Exprent) -> bool {
    e.try_as_exit().is_some_and(|exit| {
        exit.get_exit_type() == ExitType::Return
            && exit.get_value().and_then(|v| v.try_as_field()).is_some_and(|f| {
                f.is_static() && f.get_classname() == KTypes::UNIT && f.get_name() == "INSTANCE"
            })
    })
}

/// The label for the receiver of the rebuilt lambda `mt`: the name kotlinc
/// gave it after `$this$` (the function the lambda is passed to, or the
/// source's label), or for `<this>` one made of the class name's last two
/// parts (`pump_1`), when no suspend lambda it is nested in or that is
/// nested in it may have the same label.
fn receiver_label(mt: StructMethod) -> Option<String> {
    let receiver = receiver_name(mt)?;
    let roots = DecompilerContext::get_class_processor().get_map_root_classes();
    let node = roots.borrow().get(&mt.get_class_qualified_name()).copied()?;
    let label = label_for(&receiver, &mt.get_class_qualified_name())?;
    let mut others = Vec::new();
    let mut parent = node.with(|d| d.parent);
    while let Some(p) = parent {
        labels_of(p, &mut others);
        parent = p.with(|d| d.parent);
    }
    for nested in node.with(|d| d.nested.to_vec()) {
        labels_under(nested, &mut others);
    }
    (!others.contains(&label)).then_some(label)
}

/// The label a receiver local named `receiver` in the suspend lambda class
/// `class` is given, when it is an identifier.
fn label_for(receiver: &str, class: &str) -> Option<String> {
    let label = match receiver.strip_prefix("$this$") {
        Some(name) => name.to_string(),
        None => {
            let parts: Vec<&str> = class.rsplit(['/', '$']).take(2).collect();
            format!("{}_{}", parts.get(1)?, parts.first()?)
        }
    };
    let identifier = label.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    identifier.then_some(label)
}

/// The labels the receiver locals of the suspend lambda class of `node` may
/// get, from its local variable table.
fn labels_of(node: ClassNode, out: &mut Vec<String>) {
    let cl = node.with(|d| d.class_struct);
    let Some(lvt) = body_method(cl).and_then(|body| body.get_local_variable_attr()) else {
        return;
    };
    for v in lvt.get_variables() {
        if is_receiver_name(&v.name) {
            out.extend(label_for(&v.name, &cl.qualified_name()));
        }
    }
}

fn labels_under(node: ClassNode, out: &mut Vec<String>) {
    labels_of(node, out);
    for nested in node.with(|d| d.nested.to_vec()) {
        labels_under(nested, out);
    }
}

/// Labels the rebuilt lambda `mt` whose header follows `buf` and whose body
/// is `root`: with its receiver's label, or when an exit of the body is not
/// in tail position.
pub fn label_exits(buf: &mut TextBuffer, root: RootStatement, mt: StructMethod) {
    match label(mt) {
        Some(label) => {
            buf.append(&format!("{label}@"));
            for edge in root.get_dummy_exit().0.get_predecessor_edges(StatEdge::TYPE_BREAK) {
                let source = edge.get_source();
                if kt_jumps::in_tail_position(source) {
                    continue;
                }
                for e in source.get_exprents().map(|x| x.to_vec()).unwrap_or_default() {
                    if let Some(exit) = e.try_as_k_exit() {
                        if exit.get_exit_type() == ExitType::Return {
                            exit.set_lambda_name(Some(label.clone()));
                        }
                    }
                }
            }
        }
        None => kt_jumps::label_exits(buf, root, &mt.get_class_qualified_name()),
    }
}

/// The name of the local the receiver of the rebuilt lambda `mt` is read
/// into (`$this$flow`).
pub fn receiver_name(mt: StructMethod) -> Option<String> {
    parameters(mt)?.values().find(|n| is_receiver_name(n)).cloned()
}

/// Whether `name` is one kotlinc gives the local of a lambda's receiver:
/// `$this$f` for a lambda passed to `f` (or labelled `f`), `<this>`
/// otherwise.
fn is_receiver_name(name: &str) -> bool {
    name.starts_with("$this$") || name == "<this>"
}

thread_local! {
    /// Whether a lambda was rebuilt on this thread.
    static REBUILT: Cell<bool> = const { Cell::new(false) };
    /// Whether a rebuilt lambda was given a label on this thread.
    static LABELLED: Cell<bool> = const { Cell::new(false) };
}

#[allow(non_snake_case)]
fn LABEL() -> Key<String> {
    Key::of("KunaSuspendLambdaLabel")
}

/// The name `var` is written with when it holds the receiver of a rebuilt
/// lambda written with a label, in its body or, captured, in a class nested
/// in it: `this@label`.
pub fn receiver_reference(var: VarExprent) -> Option<String> {
    if !LABELLED.with(Cell::get) {
        return None;
    }
    let mut current = (var.get_processor()?, var.get_var_version_pair());
    for _ in 0..64 {
        let (processor, pair) = current;
        let mt = processor.get_method();
        if let Some(label) = label(mt) {
            let slot = processor.get_var_original_index(pair.var).unwrap_or(pair.var);
            let receiver = parameters(mt)?
                .iter()
                .find(|(_, name)| is_receiver_name(name))
                .map(|(slot, _)| *slot);
            if receiver == Some(slot) {
                return Some(format!("this@{label}"));
            }
        }
        current = processor.get_captured_source(pair)?;
    }
    None
}

/// The label the rebuilt lambda `mt` is written with, whose receiver it
/// names `this@label`.
pub fn label(mt: StructMethod) -> Option<String> {
    KtAttrs::get(mt, &LABEL())
}
fn remove_under(stat: Statement, parameters: &BTreeMap<i32, String>, res: &mut bool) {
    match stat.get_exprents() {
        Some(exprents) => {
            let mut i = 0;
            while i < exprents.len() {
                if is_argument_read(exprents.get(i), parameters) {
                    exprents.remove_at(i);
                    *res = true;
                } else {
                    i += 1;
                }
            }
        }
        None => {
            for st in stat.get_stats().borrow().to_vec() {
                remove_under(st, parameters, res);
            }
        }
    }
}

/// Whether `e` is `local = this.F` (possibly through a cast, and unboxed for
/// an inline class) for an argument local.
fn is_argument_read(e: Exprent, parameters: &BTreeMap<i32, String>) -> bool {
    let Some(assignment) = e.try_as_assignment() else {
        return false;
    };
    let Some(var) = assignment.get_left().try_as_var() else {
        return false;
    };
    let index = var.get_index();
    let slot = var
        .get_processor()
        .and_then(|p| p.get_var_original_index(index))
        .unwrap_or(index);
    if !parameters.contains_key(&slot) {
        return false;
    }
    let mut right = assignment.get_right();
    if let Some(unbox) = right.try_as_invocation().filter(|i| {
        !i.is_static() && i.get_name() == "unbox-impl" && i.get_lst_parameters().is_empty()
    }) {
        match unbox.get_instance() {
            Some(instance) => right = instance,
            None => return false,
        }
    }
    if let Some(cast) = right.try_as_function().filter(|f| f.get_func_type() == FunctionType::Cast) {
        right = cast.get_lst_operands().get(0);
    }
    right.try_as_field().is_some_and(|f| {
        !f.is_static() && f.get_instance().is_some_and(|i| i.try_as_var().is_some_and(|v| v.get_index() == 0))
            && is_machine_field(&f.get_name())
    })
}

/// Whether `var`, in the body being decompiled, is a local a rebuilt suspend
/// lambda reads an argument into.
pub fn is_argument_local(var: VarExprent) -> bool {
    if !REBUILT.with(Cell::get) {
        return false;
    }
    let Some(processor) = var.get_processor().or_else(DecompilerContext::get_current_var_processor) else {
        return false;
    };
    let Some(parameters) = parameters(processor.get_method()) else {
        return false;
    };
    let index = var.get_index();
    parameters.contains_key(&processor.get_var_original_index(index).unwrap_or(index))
}

/// The operand of `e` when it is kotlinc's cast of a suspend lambda written
/// as one to its `FunctionN` interface.
pub fn lambda_of_cast(e: Exprent) -> Option<Exprent> {
    let cast = e.try_as_function()?;
    if cast.get_func_type() != FunctionType::Cast {
        return None;
    }
    let operands = cast.get_lst_operands();
    let target = operands.get(1).get_expr_type();
    if target.array_dim != 0 || !target.value.as_deref().is_some_and(|v| v.starts_with(FUNCTION_PREFIX)) {
        return None;
    }
    let new = operands.get(0);
    let anonymous = new.try_as_new()?;
    if !anonymous.is_anonymous() {
        return None;
    }
    let class = anonymous.get_new_type().value.clone()?;
    let node = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&class.to_string())
        .copied()?;
    is_rebuilt(node.with(|d| d.class_struct)).then_some(new)
}
