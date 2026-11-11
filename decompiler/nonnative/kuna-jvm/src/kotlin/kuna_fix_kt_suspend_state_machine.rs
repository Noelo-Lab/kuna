//! kuna fix `kt_suspend_state_machine` (spec 10.16): in Kotlin output, a
//! suspend function is written with the body its source had, not as the
//! coroutine state machine kotlinc compiled it to.
//!
//! kotlinc lowers a suspend function that suspends to a state machine: an
//! entry that reuses its continuation object (`$completion is` the
//! function's `ContinuationImpl` subclass with the `label` sign bit set) or
//! creates one, reads `result` and `COROUTINE_SUSPENDED`, and dispatches on
//! `label`; before each suspension point the live locals are spilled into
//! the continuation's fields (`L$0`, `I$0`, `J$0`, ..) and `label` is set to
//! the point's number, the callee gets the continuation, and a result that is
//! `COROUTINE_SUSPENDED` is returned; the dispatch resumes at the code after
//! the call, which restores the locals, calls `ResultKt.throwOnFailure` on
//! the resumed `result` and continues with it as the call's value. The
//! resumed path recreates the frame the direct path has after the call (that
//! is what kotlinc's transformation guarantees), so the direct path alone is
//! the source's body. Vineflower prints the machine, which Kotlin cannot
//! express (`$completion`, the continuation type, `label`, the spill fields).
//!
//! The fix recognises the machine instruction by instruction (the entry,
//! the dispatch with its `call to 'resume' before 'invoke'` default, and for
//! each label a suspension point: the spill and label stores and
//! `InlineMarker` no-ops before a call whose last argument is the
//! continuation, the `COROUTINE_SUSPENDED` check and return after it, and a
//! resume block made only of restores, `nop`s and the `throwOnFailure` of
//! `result` followed by `result` itself), and requires the continuation,
//! `result` and the marker to be used nowhere else (copies of the
//! continuation that only suspend calls and `getContext()` read count as
//! the continuation) and every exception handler of a resume block's
//! `throwOnFailure` to be the call's. It then rewrites the method's graph
//! into the direct path: the entry is the code of label 0, the spill, label
//! and copy stores and the check are gone, each call passes `$completion`
//! (the Kotlin writer leaves it out, fix `kt_suspend_call_continuation`) and
//! flows into the code after it, the resume blocks are dead, and the parts
//! of a `try` range kotlinc split at a suspension point are one range again.
//! The same check after a call in tail position of a function without a
//! machine (`return if (r === COROUTINE_SUSPENDED) r else Unit`) is removed
//! too. Any other shape (a continuation used by inlined code, as
//! `suspendCoroutineUninterceptedOrReturn` uses it) keeps Vineflower's form.
//! The fix applies to functions the metadata declares `suspend`, the ones
//! the writer prints with `suspend` and without the continuation parameter.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::basic_block::BasicBlock;
use crate::code_constants::*;
use crate::constant_pool::ConstantPool;
use crate::control_flow_graph::ControlFlowGraph;
use crate::dead_code_helper::DeadCodeHelper;
use crate::instruction::Instruction;
use crate::k_element::KElement;
use crate::key::Key;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_descriptor::MethodDescriptor;
use crate::primitive_constant::ConstValue;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;

const CONTINUATION: &str = "Lkotlin/coroutines/Continuation;";
const CONTINUATION_CLASS: &str = "kotlin/coroutines/Continuation";
const INTRINSICS: &str = "kotlin/coroutines/intrinsics/IntrinsicsKt";
const SUSPENDED: &str = "getCOROUTINE_SUSPENDED";
const RESUME_ERROR: &str = "call to 'resume' before 'invoke' with coroutine";

/// The changes that turn a method's graph into its direct path, by index
/// into the method's instruction sequence.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Plan {
    /// Instructions deleted from their blocks.
    pub removed: BTreeSet<usize>,
    /// Loads of the continuation that become loads of `$completion`.
    pub completion_loads: BTreeSet<usize>,
    /// Each check's `if_acmpne`, whose block keeps only its jump target, and
    /// the first instruction of the block it falls through to.
    pub checks: Vec<(usize, usize)>,
    /// The first instruction of the new entry block.
    pub entry: Option<usize>,
    /// Each suspension call and the first instruction of the code after it.
    pub joins: Vec<(usize, usize)>,
    /// The resume blocks' instructions, dead once the plan is applied.
    pub resume_path: BTreeSet<usize>,
}

/// Rewrites the graph of a suspend function `mt` of `cl` into its direct
/// path; whether it did. The metadata of a class with a method in the
/// suspend calling convention is read here, before its methods are
/// written, where the writer would read it.
pub fn rebuild(graph: ControlFlowGraph, mt: StructMethod, cl: StructClass) -> bool {
    if mt.get_descriptor().ends_with(&format!("{CONTINUATION})Ljava/lang/Object;")) {
        KotlinChooser::parse_metadata_for(cl);
    }
    let Some(completion) = completion_slot(mt) else {
        return false;
    };
    let seq = graph.get_sequence();
    let code = Code {
        insns: seq.instructions(),
        pool: cl.get_pool(),
        handlers: seq
            .exception_table()
            .get_handlers()
            .iter()
            .map(|h| {
                (
                    h.from() as usize,
                    h.to() as usize,
                    h.handler() as usize,
                    h.exception_class(),
                )
            })
            .collect(),
    };
    match plan(&code, completion) {
        Ok(plan) => {
            let r = apply(graph, &code, &plan, completion);
            if r {
                let path: Rc<[usize]> = plan.resume_path.iter().copied().collect();
                KtAttrs::put(mt, &RESUME_PATH(), path);
            }
            r
        }
        Err(_) => false,
    }
}

/// The member attribute marking a rebuilt function: the indices of the
/// instructions of its resume blocks, which its direct body does not run.
#[allow(non_snake_case)]
fn RESUME_PATH() -> Key<Rc<[usize]>> {
    Key::of("KunaSuspendResumePath")
}

/// The instructions of `mt`'s resume blocks, when this fix rebuilt it.
pub fn resume_path(mt: StructMethod) -> Option<Rc<[usize]>> {
    KtAttrs::get(mt, &RESUME_PATH())
}

/// The slot of `$completion`, when `mt` is a function the metadata declares
/// `suspend` and its last parameter is the continuation.
pub fn completion_slot(mt: StructMethod) -> Option<i32> {
    let Some(KElement::Function(function)) = KtAttrs::get(mt, &KElement::KEY()) else {
        return None;
    };
    if !Flags::IS_SUSPEND.get(function.flags) {
        return None;
    }
    let descriptor = mt.get_descriptor();
    if !descriptor.ends_with(&format!("{CONTINUATION})Ljava/lang/Object;")) {
        return None;
    }
    let md = MethodDescriptor::parse_descriptor(&descriptor);
    let mut slot = if mt.has_modifier(ACC_STATIC) { 0 } else { 1 };
    for param in &md.params[..md.params.len() - 1] {
        slot += param.stack_size;
    }
    Some(slot)
}

/// A method's instructions and exception table, by instruction index.
pub struct Code {
    pub insns: Vec<Instruction>,
    pub pool: ConstantPool,
    /// `(from, to, handler, type)`, in table order.
    pub handlers: Vec<(usize, usize, usize, Option<String>)>,
}

impl Code {
    fn is(&self, i: usize, opcode: i32) -> bool {
        self.insns.get(i).is_some_and(|x| x.opcode() == opcode)
    }

    fn operand(&self, i: usize) -> i32 {
        self.insns[i].operand(0)
    }

    fn is_local(&self, i: usize, opcode: i32, slot: i32) -> bool {
        self.is(i, opcode) && self.operand(i) == slot
    }

    fn member(&self, i: usize) -> Option<(String, String, String)> {
        let link = self.pool.get_link_constant(self.operand(i))?;
        Some((link.classname.clone()?, link.elementname.clone(), link.descriptor.clone()))
    }

    fn is_member(&self, i: usize, opcode: i32, owner: &str, name: &str) -> bool {
        self.is(i, opcode)
            && self
                .member(i)
                .is_some_and(|(o, n, _)| o == owner && n == name)
    }

    fn class(&self, i: usize, opcode: i32) -> Option<String> {
        if !self.is(i, opcode) {
            return None;
        }
        Some(self.pool.get_primitive_constant(self.operand(i))?.get_string().to_string())
    }

    fn int_constant(&self, i: usize) -> Option<i32> {
        let insn = self.insns.get(i)?;
        match insn.opcode() {
            OPC_BIPUSH | OPC_SIPUSH => Some(insn.operand(0)),
            OPC_LDC | OPC_LDC_W => match self.pool.get_primitive_constant(insn.operand(0))?.value {
                Some(ConstValue::Integer(v)) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    fn is_string(&self, i: usize, text: &str) -> bool {
        (self.is(i, OPC_LDC) || self.is(i, OPC_LDC_W))
            && self
                .pool
                .get_primitive_constant(self.operand(i))
                .is_some_and(|c| matches!(&c.value, Some(ConstValue::String(s)) if &**s == text))
    }

    fn jump(&self, i: usize) -> usize {
        self.insns[i].as_jump().destination() as usize
    }

    fn is_suspended_marker(&self, i: usize) -> bool {
        self.is_member(i, OPC_INVOKESTATIC, INTRINSICS, SUSPENDED)
    }

    fn is_throw_on_failure(&self, i: usize, result: i32) -> bool {
        self.is_local(i, OPC_ALOAD, result)
            && self.is_member(i + 1, OPC_INVOKESTATIC, "kotlin/ResultKt", "throwOnFailure")
    }

    /// A call that passes a continuation last and returns `Object`: a
    /// suspend call, in kotlinc's calling convention, or the `invoke` of a
    /// function type, which a suspend function type is on the JVM.
    fn is_suspend_call(&self, i: usize) -> bool {
        let Some(insn) = self.insns.get(i) else {
            return false;
        };
        insn.group() == GROUP_INVOCATION
            && insn.opcode() != OPC_INVOKEDYNAMIC
            && self.member(i).is_some_and(|(owner, name, d)| {
                d.ends_with(&format!("{CONTINUATION})Ljava/lang/Object;"))
                    || is_function_invoke(&owner, &name, &d)
            })
    }

    fn is_continuation_cast(&self, i: usize) -> bool {
        self.class(i, OPC_CHECKCAST).as_deref() == Some(CONTINUATION_CLASS)
    }

    /// `InlineMarker.mark(n)`, the no-op kotlinc leaves around a suspend
    /// call it inlined, at `i` and `i + 1`.
    fn is_inline_marker(&self, i: usize) -> bool {
        self.int_constant(i).is_some()
            && self.is_member(i + 1, OPC_INVOKESTATIC, "kotlin/jvm/internal/InlineMarker", "mark")
    }

    fn is_load(&self, i: usize) -> bool {
        [OPC_ILOAD, OPC_LLOAD, OPC_FLOAD, OPC_DLOAD, OPC_ALOAD]
            .iter()
            .any(|&op| self.is(i, op))
    }

    fn is_store(&self, i: usize) -> bool {
        [OPC_ISTORE, OPC_LSTORE, OPC_FSTORE, OPC_DSTORE, OPC_ASTORE]
            .iter()
            .any(|&op| self.is(i, op))
    }

    /// The `(handler, type)` entries covering instruction `i`, in table order.
    fn handlers_of(&self, i: usize) -> Vec<(usize, Option<String>)> {
        self.handlers
            .iter()
            .filter(|(from, to, _, _)| *from <= i && i < *to)
            .map(|(_, _, handler, kind)| (*handler, kind.clone()))
            .collect()
    }
}

/// Whether `owner.name` with descriptor `d` is a function type's `invoke`
/// taking at least one argument (`FunctionN.invoke(Object..)Object`).
pub fn is_function_invoke(owner: &str, name: &str, d: &str) -> bool {
    let arity = owner
        .strip_prefix("kotlin/jvm/functions/Function")
        .and_then(|n| n.parse::<usize>().ok());
    name == "invoke"
        && arity.is_some_and(|n| n >= 1 && d == format!("({})Ljava/lang/Object;", "Ljava/lang/Object;".repeat(n)))
}

/// The continuation machine's locals and class.
struct Machine {
    class: String,
    cont: i32,
    result: i32,
    marker: i32,
}

/// Why a function keeps Vineflower's form.
pub type Refusal = &'static str;

fn require(ok: bool, why: Refusal) -> Result<(), Refusal> {
    if ok {
        Ok(())
    } else {
        Err(why)
    }
}

/// The plan for `code`, a suspend function whose `$completion` is in slot
/// `completion`: its machine's, or its tail calls' checks'.
pub fn plan(code: &Code, completion: i32) -> Result<Plan, Refusal> {
    if code.is_local(0, OPC_ALOAD, completion) && code.is(1, OPC_INSTANCEOF) {
        return plan_machine(code, completion);
    }
    plan_tail_checks(code, completion)
}

fn plan_machine(code: &Code, completion: i32) -> Result<Plan, Refusal> {
    let class = code.class(1, OPC_INSTANCEOF).ok_or("entry")?;
    require(code.is(2, OPC_IFEQ), "entry")?;
    let new_at = code.jump(2);
    require(
        code.is_local(3, OPC_ALOAD, completion)
            && code.class(4, OPC_CHECKCAST).as_deref() == Some(class.as_str())
            && code.is(5, OPC_ASTORE),
        "entry",
    )?;
    let cont = code.operand(5);
    require(
        code.is_local(6, OPC_ALOAD, cont)
            && code.is_member(7, OPC_GETFIELD, &class, "label")
            && code.int_constant(8) == Some(i32::MIN)
            && code.is(9, OPC_IAND)
            && code.is(10, OPC_IFEQ)
            && code.jump(10) == new_at
            && code.is_local(11, OPC_ALOAD, cont)
            && code.is(12, OPC_DUP)
            && code.is_member(13, OPC_GETFIELD, &class, "label")
            && code.int_constant(14) == Some(i32::MIN)
            && code.is(15, OPC_ISUB)
            && code.is_member(16, OPC_PUTFIELD, &class, "label")
            && code.is(17, OPC_GOTO)
            && new_at == 18,
        "entry reuse",
    )?;
    let main_at = code.jump(17);
    let mut i = 18;
    require(
        code.class(i, OPC_NEW).as_deref() == Some(class.as_str()) && code.is(i + 1, OPC_DUP),
        "entry creation",
    )?;
    i += 2;
    let outer = code.is_local(i, OPC_ALOAD, 0) && !code.is_local(i, OPC_ALOAD, completion);
    if outer {
        i += 1;
    }
    require(code.is_local(i, OPC_ALOAD, completion) && code.is(i + 1, OPC_INVOKESPECIAL), "entry creation")?;
    let (owner, name, descriptor) = code.member(i + 1).ok_or("entry creation")?;
    require(
        owner == class
            && name == "<init>"
            && descriptor.ends_with(&format!("{CONTINUATION})V"))
            && MethodDescriptor::parse_descriptor(&descriptor).params.len() == 1 + usize::from(outer)
            && code.is_local(i + 2, OPC_ASTORE, cont)
            && main_at == i + 3,
        "entry creation",
    )?;
    i = main_at;
    require(
        code.is_local(i, OPC_ALOAD, cont)
            && code.is_member(i + 1, OPC_GETFIELD, &class, "result")
            && code.is(i + 2, OPC_ASTORE)
            && code.is_suspended_marker(i + 3)
            && code.is(i + 4, OPC_ASTORE)
            && code.is_local(i + 5, OPC_ALOAD, cont)
            && code.is_member(i + 6, OPC_GETFIELD, &class, "label"),
        "dispatch",
    )?;
    let machine = Machine {
        class,
        cont,
        result: code.operand(i + 2),
        marker: code.operand(i + 4),
    };
    let switch_at = i + 7;
    let switch = code
        .insns
        .get(switch_at)
        .and_then(|x| x.try_as_switch())
        .ok_or("dispatch")?;
    let values = switch.get_values();
    let targets: Vec<usize> = switch.get_destinations().iter().map(|&d| d as usize).collect();
    let labels = values.len();
    require(
        labels >= 2 && values.iter().enumerate().all(|(k, &v)| v == k as i32),
        "dispatch labels",
    )?;
    let default_at = switch.get_default_destination() as usize;
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
    let mut owned: BTreeSet<usize> = BTreeSet::new();
    plan.removed.extend(0..=switch_at);
    plan.removed.extend([targets[0], targets[0] + 1]);
    owned.extend(0..=switch_at);
    owned.extend([targets[0], targets[0] + 1]);
    owned.extend(default_at..default_at + 5);
    plan.entry = Some(targets[0]);

    let mut resumes: BTreeMap<usize, (usize, usize, usize)> = BTreeMap::new();
    for (label, &start) in targets.iter().enumerate().skip(1) {
        let mut block = BTreeSet::new();
        let (failure, after) = resume_block(code, &machine, start, &mut block)?;
        resumes.insert(start, (label, failure, after));
        owned.extend(block.iter().copied());
        plan.resume_path.extend(block);
    }

    let aliases = continuation_aliases(code, &machine, completion)?;
    let holds_cont = |i: usize| {
        code.is(i, OPC_ALOAD)
            && (code.operand(i) == machine.cont || aliases.contains(&code.operand(i)))
    };
    for i in 0..code.insns.len() {
        if owned.contains(&i) || !holds_cont(i) {
            continue;
        }
        let cast = usize::from(code.is_continuation_cast(i + 1));
        if code.is_store(i + 1 + cast) && aliases.contains(&code.operand(i + 1 + cast)) {
            plan.removed.extend(i..i + 2 + cast);
            owned.extend(i..i + 2 + cast);
        } else if code.is_continuation_cast(i + 1) && code.is(i + 2, OPC_POP) {
            plan.removed.extend([i, i + 1, i + 2]);
            owned.extend([i, i + 1, i + 2]);
        } else if code.is_member(i + 1, OPC_INVOKEINTERFACE, CONTINUATION_CLASS, "getContext") {
            plan.completion_loads.insert(i);
            owned.insert(i);
        }
    }

    let mut seen: BTreeSet<usize> = BTreeSet::new();
    for k in 1..code.insns.len().saturating_sub(4) {
        if !(code.is(k, OPC_DUP)
            && code.is_local(k + 1, OPC_ALOAD, machine.marker)
            && code.is(k + 2, OPC_IF_ACMPNE)
            && code.is_local(k + 3, OPC_ALOAD, machine.marker)
            && code.is(k + 4, OPC_ARETURN))
        {
            continue;
        }
        let mut call = k - 1;
        while call >= 2 && code.is_inline_marker(call - 1) {
            plan.removed.extend([call - 1, call]);
            owned.extend([call - 1, call]);
            call -= 2;
        }
        let &(label, failure, after) = resumes.get(&(k + 5)).ok_or("check without resume block")?;
        require(code.is_suspend_call(call), "suspension point is not a call")?;
        require(seen.insert(label) && code.jump(k + 2) == after, "resume block")?;
        let mut j = call;
        let mut label_stored = false;
        loop {
            if j >= 2 && code.is_inline_marker(j - 2) {
                plan.removed.extend([j - 2, j - 1]);
                owned.extend([j - 2, j - 1]);
                j -= 2;
                continue;
            }
            if !(j >= 3
                && code.is_local(j - 3, OPC_ALOAD, machine.cont)
                && code.is(j - 1, OPC_PUTFIELD)
                && code
                    .member(j - 1)
                    .is_some_and(|(owner, _, _)| owner == machine.class))
            {
                break;
            }
            let (_, field, _) = code.member(j - 1).ok_or("spill")?;
            if field == "label" {
                require(!label_stored && code.int_constant(j - 2) == Some(label as i32), "label store")?;
                label_stored = true;
            } else {
                require(
                    field != "result" && (code.is_load(j - 2) || code.is(j - 2, OPC_ACONST_NULL)),
                    "spill",
                )?;
            }
            plan.removed.extend([j - 3, j - 2, j - 1]);
            owned.extend([j - 3, j - 2, j - 1]);
            j -= 3;
        }
        require(label_stored, "label store")?;
        require(j > 0 && holds_cont(j - 1), "continuation is not the call's last argument")?;
        require(
            code.handlers_of(call) == code.handlers_of(failure),
            "resumed failure has other handlers",
        )?;
        plan.completion_loads.insert(j - 1);
        owned.insert(j - 1);
        plan.removed.extend([k, k + 1, k + 2]);
        owned.extend(k..k + 5);
        plan.checks.push((k + 2, k + 3));
        plan.joins.push((call, after));
    }
    require(seen.len() == labels - 1, "resume block without suspension call")?;

    for i in 0..code.insns.len() {
        if owned.contains(&i) {
            continue;
        }
        let touches_local = (code.is_load(i) || code.is_store(i) || code.is(i, OPC_IINC))
            && ([machine.cont, machine.result, machine.marker].contains(&code.operand(i))
                || aliases.contains(&code.operand(i)));
        let touches_class = (code.is(i, OPC_GETFIELD) || code.is(i, OPC_PUTFIELD))
            && code
                .member(i)
                .is_some_and(|(owner, _, _)| owner == machine.class);
        require(
            !touches_local && !touches_class && !code.is_suspended_marker(i),
            "continuation used by other code",
        )?;
    }
    Ok(plan)
}

/// The locals that hold nothing but the continuation, stored as `aload
/// cont; astore x`, possibly through a cast to `Continuation` (an inlined
/// suspend function's or suspend lambda's `$completion`).
fn continuation_aliases(
    code: &Code,
    machine: &Machine,
    completion: i32,
) -> Result<BTreeSet<i32>, Refusal> {
    let stored_cont = |i: usize| {
        (i >= 1 && code.is_local(i - 1, OPC_ALOAD, machine.cont))
            || (i >= 2
                && code.is_continuation_cast(i - 1)
                && code.is_local(i - 2, OPC_ALOAD, machine.cont))
    };
    let mut aliases = BTreeSet::new();
    for i in 1..code.insns.len() {
        if code.is(i, OPC_ASTORE) && stored_cont(i) {
            aliases.insert(code.operand(i));
        }
    }
    for &x in &aliases {
        require(
            ![machine.cont, machine.result, machine.marker, completion].contains(&x),
            "continuation alias",
        )?;
        for i in 0..code.insns.len() {
            if (code.is_store(i) || code.is(i, OPC_IINC))
                && code.operand(i) == x
                && !(code.is(i, OPC_ASTORE) && stored_cont(i))
            {
                return Err("continuation alias");
            }
        }
    }
    Ok(aliases)
}

/// The resume block at `start`: the index of its `throwOnFailure` and of
/// the code after it, marking its instructions in `owned`.
fn resume_block(
    code: &Code,
    machine: &Machine,
    start: usize,
    owned: &mut BTreeSet<usize>,
) -> Result<(usize, usize), Refusal> {
    let mut i = start;
    loop {
        if code.is_throw_on_failure(i, machine.result) {
            require(code.is_local(i + 2, OPC_ALOAD, machine.result), "resume block")?;
            owned.extend(start..i + 3);
            let after = if code.is(i + 3, OPC_GOTO) {
                owned.insert(i + 3);
                code.jump(i + 3)
            } else {
                i + 3
            };
            return Ok((i + 1, after));
        }
        if code.is(i, OPC_NOP) {
            i += 1;
        } else if code.is_local(i, OPC_ALOAD, machine.cont)
            && code.is(i + 1, OPC_GETFIELD)
            && code
                .member(i + 1)
                .is_some_and(|(o, n, _)| o == machine.class && n != "label" && n != "result")
        {
            i += 2;
            if code.is(i, OPC_CHECKCAST) {
                i += 1;
            }
            require(
                code.is_store(i) || code.is(i, OPC_POP) || code.is(i, OPC_POP2),
                "resume block",
            )?;
            i += 1;
        } else if code.is_local(i, OPC_ALOAD, machine.cont)
            && code.is(i + 1, OPC_ACONST_NULL)
            && code.is(i + 2, OPC_PUTFIELD)
            && code
                .member(i + 2)
                .is_some_and(|(o, n, _)| o == machine.class && n != "label" && n != "result")
        {
            i += 3;
        } else if (code.int_constant(i) == Some(0) && code.is(i + 1, OPC_ISTORE))
            || (code.is(i, OPC_ACONST_NULL) && code.is(i + 1, OPC_ASTORE))
        {
            i += 2;
        } else {
            return Err("resume block");
        }
    }
}

/// The checks after the tail calls of a function without a machine: `r =
/// f(.., $completion); if (r === COROUTINE_SUSPENDED) return r; pop; ..`.
fn plan_tail_checks(code: &Code, completion: i32) -> Result<Plan, Refusal> {
    let mut plan = Plan::default();
    for k in 0..code.insns.len() {
        if !code.is_suspended_marker(k) {
            continue;
        }
        require(
            k >= 3
                && code.is(k - 1, OPC_DUP)
                && code.is_suspend_call(k - 2)
                && code.is_local(k - 3, OPC_ALOAD, completion)
                && code.is(k + 1, OPC_IF_ACMPNE)
                && code.is(k + 2, OPC_ARETURN)
                && code.is(code.jump(k + 1), OPC_POP),
            "marker outside a tail call's check",
        )?;
        plan.removed.extend([k - 1, k, k + 1]);
        plan.checks.push((k + 1, k + 2));
    }
    require(!plan.checks.is_empty(), "no check")?;
    Ok(plan)
}

/// Applies `plan` to `graph`; whether it did (nothing changes when the
/// graph's blocks do not have the expected shape).
fn apply(graph: ControlFlowGraph, code: &Code, plan: &Plan, completion: i32) -> bool {
    let offset = |i: usize| code.insns[i].start_offset();
    let mut at: BTreeMap<i32, (BasicBlock, usize)> = BTreeMap::new();
    for block in graph.get_blocks().borrow().iter().copied() {
        let seq = block.get_seq();
        for p in 0..seq.length() {
            at.insert(seq.get_instr(p).start_offset(), (block, p as usize));
        }
    }
    let first_of = |i: usize| at.get(&offset(i)).filter(|(_, p)| *p == 0).map(|(b, _)| *b);

    let mut redirects: Vec<(BasicBlock, BasicBlock)> = Vec::new();
    for &(check, fallthrough) in &plan.checks {
        let Some(&(block, p)) = at.get(&offset(check)) else {
            return false;
        };
        let Some(returning) = first_of(fallthrough) else {
            return false;
        };
        let succs = block.get_succs();
        if p as i32 != block.size() - 1 || succs.len() != 2 || !succs.contains(&returning) {
            return false;
        }
        redirects.push((block, returning));
    }
    let entry = match plan.entry {
        Some(i) => match first_of(i) {
            Some(block) => Some(block),
            None => return false,
        },
        None => None,
    };
    let mut joins: Vec<(BasicBlock, usize)> = Vec::new();
    for &(call, after) in &plan.joins {
        let Some(&(block, _)) = at.get(&offset(call)) else {
            return false;
        };
        joins.push((block, after));
    }
    let removed: BTreeSet<i32> = plan.removed.iter().map(|&i| offset(i)).collect();
    let lost = |i: usize| !at.contains_key(&offset(i)) && !code.is(i, OPC_GOTO);
    if plan.completion_loads.iter().any(|&i| lost(i)) || plan.removed.iter().any(|&i| lost(i)) {
        return false;
    }

    let mut emptied: Vec<BasicBlock> = Vec::new();
    for block in graph.get_blocks().borrow().iter().copied().collect::<Vec<_>>() {
        let seq = block.get_seq();
        let offsets = block.get_instr_old_offsets();
        let mut changed = false;
        let mut p = seq.length() - 1;
        while p >= 0 {
            let insn = seq.get_instr(p);
            let off = insn.start_offset();
            if removed.contains(&off) {
                seq.remove_instruction(p);
                if (p as usize) < offsets.len() {
                    offsets.remove_at(p as usize);
                }
                changed = true;
            } else if plan.completion_loads.iter().any(|&i| offset(i) == off) {
                let load = Instruction::create(
                    OPC_ALOAD,
                    false,
                    GROUP_GENERAL,
                    insn.bytecode_version(),
                    Some(vec![completion]),
                    off,
                    insn.length(),
                );
                seq.with_mut(|d| d.instructions[p as usize] = load);
            }
            p -= 1;
        }
        if changed && seq.is_empty() {
            emptied.push(block);
        }
    }
    for (block, returning) in redirects {
        block.remove_successor(returning);
    }
    if let Some(entry) = entry {
        graph.set_first(entry);
    }
    DeadCodeHelper::remove_dead_blocks(graph);
    for block in emptied {
        bridge(graph, block);
    }
    for (call_block, after) in joins {
        if let Some(&(after_block, _)) = at.get(&offset(after)) {
            join_ranges(graph, call_block, after_block);
        }
    }
    DeadCodeHelper::remove_dead_blocks(graph);
    true
}

/// Removes the emptied `block`, connecting its predecessors to its one
/// successor.
fn bridge(graph: ControlFlowGraph, block: BasicBlock) {
    if !graph.get_blocks().borrow().contains_key(&block.get_id())
        || !block.get_pred_exceptions().is_empty()
        || block.get_succs().len() != 1
    {
        return;
    }
    let succ = block.get_succs().get(0);
    if succ == block || block.get_preds().iter().any(|p| p.get_succs().contains(&succ)) {
        return;
    }
    for pred in block.get_preds().to_vec() {
        pred.replace_successor(block, succ);
    }
    if graph.get_first() == block {
        graph.set_first(succ);
    }
    graph.remove_block(block);
}

/// Makes one range of each pair of ranges with the same handler and types
/// that kotlinc split at the suspension call in `call_block` (the call in
/// one, `after_block`, the code after it, in the other), where no range
/// between them in the table covers the later one's blocks.
fn join_ranges(graph: ControlFlowGraph, call_block: BasicBlock, after_block: BasicBlock) {
    loop {
        let ranges = graph.get_exceptions().to_vec();
        let mut merge: Option<(usize, usize)> = None;
        'search: for (a, range) in ranges.iter().enumerate() {
            if !range.get_protected_range().contains(&call_block) {
                continue;
            }
            for (b, other) in ranges.iter().enumerate() {
                if a == b
                    || other.get_handler() != range.get_handler()
                    || other.get_exception_types() != range.get_exception_types()
                    || !other.get_protected_range().contains(&after_block)
                    || other.get_protected_range().contains(&call_block)
                {
                    continue;
                }
                let (early, late) = if a < b { (a, b) } else { (b, a) };
                let late_blocks = ranges[late].get_protected_range().to_vec();
                let shadowed = ranges[early + 1..late].iter().any(|between| {
                    late_blocks
                        .iter()
                        .any(|blk| between.get_protected_range().contains(blk))
                });
                if !shadowed {
                    merge = Some((early, late));
                    break 'search;
                }
            }
        }
        let Some((early, late)) = merge else {
            return;
        };
        let keep = ranges[early].get_protected_range();
        for blk in ranges[late].get_protected_range().to_vec() {
            if !keep.contains(&blk) {
                keep.add(blk);
            }
        }
        let mut sorted = keep.to_vec();
        sorted.sort_by_key(|blk| blk.get_id());
        keep.set_all(sorted);
        graph.get_exceptions().remove_at(late);
    }
}
