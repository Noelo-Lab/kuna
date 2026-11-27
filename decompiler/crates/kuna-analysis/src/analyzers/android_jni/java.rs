//! Symbolic Java method calls reached from an ELF's proven JNI entry points.
use std::{
    cell::OnceCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
    sync::{Arc, OnceLock},
};

use kuna_base::{address::Address, space::spacetype};
use kuna_decomp::{
    architecture::Architecture,
    kuna_androidcalls::{JavaCall, JavaTarget},
};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::loadimage::ImageBytes;
use object::{Object, ObjectSection, ObjectSymbol, SectionFlags, SectionKind, SymbolKind};

use super::{signature, valid_class, valid_name, Image, NativeMethod};
use crate::{
    analyzers::nativeflow::{self, Machine, Place, Slot, State, Value},
    listing::xrefs::{FullCapture, FullOp},
};

const ENV: u64 = 1;
const VM: u64 = 2;
const ENV_TABLE: u64 = 3;
const VM_TABLE: u64 = 4;
const FIRST_HANDLE: u64 = 8;
const VM_FIELD: u64 = 1024;
const MAX_TARGETS: usize = 16;
const MAX_PATHS: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reference {
    Local,
    Global,
    Weak,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Handle {
    Class(Option<String>, Reference),
    Object(Option<String>, Reference),
    Method { target: JavaTarget, is_static: bool },
    Choices(BTreeSet<Value>),
}
#[derive(Default)]
struct Handles {
    ids: BTreeMap<Handle, u64>,
    values: Vec<Handle>,
}
impl Handles {
    fn intern(&mut self, h: Handle) -> Value {
        let n = if let Some(&n) = self.ids.get(&h) {
            n
        } else {
            let n = FIRST_HANDLE + self.values.len() as u64;
            self.ids.insert(h.clone(), n);
            self.values.push(h);
            n
        };
        Value::Handle(n, 0)
    }
    fn get(&self, v: Value) -> Option<&Handle> {
        let Value::Handle(id, 0) = v else { return None };
        self.values.get(id.checked_sub(FIRST_HANDLE)? as usize)
    }
    fn alternatives(&self, v: Option<Value>) -> BTreeSet<Value> {
        let v = v.unwrap_or(Value::Unknown);
        if let Some(Handle::Choices(set)) = self.get(v) {
            set.clone()
        } else {
            BTreeSet::from([v])
        }
    }
    fn persistent(&self, v: Value) -> bool {
        matches!(
            self.get(v),
            Some(
                Handle::Method { .. }
                    | Handle::Class(_, Reference::Global)
                    | Handle::Object(_, Reference::Global)
            )
        )
    }

    fn class(&self, v: Value) -> Option<&str> {
        match self.get(v) {
            Some(Handle::Class(Some(name), Reference::Local | Reference::Global)) => Some(name),
            _ => None,
        }
    }

    fn reference(&mut self, v: Option<Value>, kind: Reference) -> Value {
        let mut out = BTreeSet::new();
        for old in self.alternatives(v) {
            let new = match self.get(old) {
                Some(Handle::Class(name, _)) => Handle::Class(name.clone(), kind),
                Some(Handle::Object(name, _)) => Handle::Object(name.clone(), kind),
                _ => {
                    out.insert(Value::Unknown);
                    continue;
                }
            };
            out.insert(self.intern(new));
        }
        self.join(out)
    }

    fn join(&mut self, values: BTreeSet<Value>) -> Value {
        if values.len() == 1 {
            *values.first().unwrap()
        } else {
            self.intern(Handle::Choices(values))
        }
    }
}

#[derive(Clone, Copy)]
struct Invocation {
    dispatch: &'static str,
    form: &'static str,
    result: u8,
    method_arg: usize,
}
fn table() -> &'static [&'static str] {
    static TABLE: OnceLock<Vec<&'static str>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let header = include_str!("../../../data/android-jni/jni-types.h");
        let body = header
            .split_once("struct JNINativeInterface {")
            .unwrap()
            .1
            .split_once("};")
            .unwrap()
            .0;
        body.lines()
            .filter(|line| line.contains(';'))
            .map(|line| {
                line.split_once("(*")
                    .and_then(|(_, rest)| rest.split_once(')'))
                    .map(|(name, _)| name)
                    .unwrap_or("")
            })
            .collect()
    })
}
fn invocation(field: &str) -> Option<Invocation> {
    let (base, form) = if let Some(s) = field.strip_suffix('A') {
        (s, "A")
    } else if let Some(s) = field.strip_suffix('V') {
        (s, "V")
    } else {
        (field, "variadic")
    };
    if base == "NewObject" {
        return Some(Invocation {
            dispatch: "constructor",
            form,
            result: b'V',
            method_arg: 2,
        });
    }
    let (kind, ret, method_arg) = if let Some(s) = base.strip_prefix("CallNonvirtual") {
        ("nonvirtual", s, 3)
    } else if let Some(s) = base.strip_prefix("CallStatic") {
        ("static", s, 2)
    } else {
        ("virtual", base.strip_prefix("Call")?, 2)
    };
    let result = match ret.strip_suffix("Method")? {
        "Object" => b'L',
        "Boolean" => b'Z',
        "Byte" => b'B',
        "Char" => b'C',
        "Short" => b'S',
        "Int" => b'I',
        "Long" => b'J',
        "Float" => b'F',
        "Double" => b'D',
        "Void" => b'V',
        _ => return None,
    };
    Some(Invocation {
        dispatch: kind,
        form,
        result,
        method_arg,
    })
}
fn deref(v: Value, width: u32, word: u32) -> Option<Value> {
    if width != word {
        return None;
    }
    match v {
        Value::Handle(ENV, 0) => Some(Value::Tagged(ENV_TABLE, 0)),
        Value::Handle(VM, 0) => Some(Value::Tagged(VM_TABLE, 0)),
        Value::Tagged(t @ (ENV_TABLE | VM_TABLE), off) if off % u64::from(width) == 0 => {
            let index = off / u64::from(width);
            (index
                < if t == VM_TABLE {
                    8
                } else {
                    table().len() as u64
                })
            .then_some(Value::Function(
                index + if t == VM_TABLE { VM_FIELD } else { 0 },
            ))
        }
        _ => None,
    }
}
#[derive(Clone, Copy)]
enum Input {
    Argument(usize),
    Storage(Slot),
}
type Inputs = Vec<(Input, Value)>;
type Bounds = BTreeMap<u64, (u64, u64)>;
type Writes = BTreeMap<Slot, BTreeSet<Value>>;
struct Instruction {
    len: u64,
    ops: Rc<[FullOp]>,
    context: Vec<u32>,
}
struct Flow<'a, 'd> {
    machine: Machine<'a, 'd>,
    image: Image<'d>,
    mapped: Arc<dyn ImageBytes>,
    bounds: BTreeMap<u64, (u64, u64)>,
    handles: Handles,
    calls: BTreeMap<(u64, u64), JavaCall>,
    instructions: BTreeMap<u64, Instruction>,
    writes: Writes,
    escaped: BTreeSet<Slot>,
    cap: usize,
    limited: bool,
    local_hypothesis: bool,
}
impl Flow<'_, '_> {
    fn addr_string(&self, v: Option<Value>) -> Option<String> {
        let va = v?.constant()?;
        self.image
            .file
            .sections()
            .any(|s| {
                s.address() != 0
                    && va.checked_sub(s.address()).is_some_and(|n| n < s.size())
                    && matches!(s.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 3 == 2)
            })
            .then(|| self.image.string(va))?
    }
    fn observe(
        &mut self,
        owner: u64,
        site: u64,
        inv: Invocation,
        method: Option<Value>,
        class: Option<Value>,
    ) {
        let mut targets = Vec::new();
        let mut incomplete = false;
        for v in self.handles.alternatives(method) {
            let Some(Handle::Method { target, is_static }) = self.handles.get(v) else {
                incomplete = true;
                continue;
            };
            let returns = target
                .descriptor
                .split_once(')')
                .and_then(|(_, r)| r.as_bytes().first())
                .copied();
            let returns = returns.map(|r| if r == b'[' { b'L' } else { r });
            if *is_static != (inv.dispatch == "static")
                || (target.name == "<init>") != (inv.dispatch == "constructor")
                || returns != Some(inv.result)
            {
                incomplete = true;
                continue;
            }
            if matches!(inv.dispatch, "constructor" | "static" | "nonvirtual") {
                let classes = self.handles.alternatives(class);
                let mut compatible = false;
                for actual in classes {
                    match (target.class.as_deref(), self.handles.class(actual)) {
                        (Some(expected), Some(name)) if expected == name => compatible = true,
                        (Some(_), Some(_)) => incomplete = true,
                        _ => {
                            incomplete = true;
                            compatible = true;
                        }
                    }
                }
                if !compatible {
                    continue;
                }
            }
            incomplete |= target.class.is_none();
            targets.push(target.clone());
        }
        let call = self.calls.entry((owner, site)).or_insert_with(|| JavaCall {
            owner,
            site,
            dispatch: inv.dispatch.into(),
            form: inv.form.into(),
            targets: Vec::new(),
            incomplete: false,
        });
        if call.dispatch != inv.dispatch || call.form != inv.form {
            call.incomplete = true;
            return;
        }
        call.targets.extend(targets);
        call.targets.sort();
        call.targets.dedup();
        if call.targets.len() > MAX_TARGETS {
            call.targets.truncate(MAX_TARGETS);
            incomplete = true;
        }
        call.incomplete |= incomplete;
    }
    fn jni(
        &mut self,
        owner: u64,
        site: u64,
        index: u64,
        args: &[Option<Value>],
        st: &mut State,
    ) -> Option<Value> {
        if index >= VM_FIELD {
            if matches!(index - VM_FIELD, 4 | 6 | 7) {
                let jni = args[0] == Some(Value::Handle(VM, 0))
                    && (index - VM_FIELD != 6
                        || matches!(
                            args[2],
                            Some(Value::Constant(0x10001 | 0x10002 | 0x10004 | 0x10006))
                        ));
                if let Some(dst) = args[1].and_then(|a| a.slot(self.machine.width)) {
                    self.store(st, dst, jni.then_some(Value::Handle(ENV, 0)));
                }
                return jni.then_some(Value::Constant(0));
            }
            return None;
        }
        if args[0] != Some(Value::Handle(ENV, 0)) {
            return None;
        }
        let field = *table().get(index as usize)?;
        if self.local_hypothesis
            && !matches!(field, "FindClass" | "GetMethodID" | "GetStaticMethodID")
            && invocation(field).is_none()
        {
            return None;
        }
        match field {
            "FindClass" => {
                let name = self.addr_string(args[1]).filter(|n| {
                    valid_class(n)
                        || n.starts_with('[') && super::parameters(n).is_some_and(|p| p.len() == 1)
                });
                Some(self.handles.intern(Handle::Class(name, Reference::Local)))
            }
            "GetObjectClass" => {
                let class = match args[1].and_then(|v| self.handles.get(v)) {
                    Some(Handle::Object(n, Reference::Local | Reference::Global)) => n.clone(),
                    _ => None,
                };
                Some(self.handles.intern(Handle::Class(class, Reference::Local)))
            }
            "NewGlobalRef" => Some(self.handles.reference(args[1], Reference::Global)),
            "NewLocalRef" => Some(self.handles.reference(args[1], Reference::Local)),
            "NewWeakGlobalRef" => Some(self.handles.reference(args[1], Reference::Weak)),
            "DeleteGlobalRef" | "DeleteLocalRef" | "DeleteWeakGlobalRef" => {
                let deleted = self.handles.alternatives(args[1]);
                let slots: Vec<_> =
                    st.0.iter()
                        .filter(|(_, v)| deleted.contains(v))
                        .map(|(k, _)| *k)
                        .collect();
                for slot in slots {
                    self.store(st, slot, None);
                }
                None
            }
            "GetMethodID" | "GetStaticMethodID" => {
                let name = self.addr_string(args[2])?;
                let descriptor = self.addr_string(args[3])?;
                let is_static = field == "GetStaticMethodID";
                signature(&descriptor)?;
                if !(valid_name(&name)
                    || name == "<init>" && !is_static && descriptor.ends_with(")V"))
                {
                    return None;
                }
                let classes = self.handles.alternatives(args[1]);
                let mut candidates = BTreeSet::new();
                for c in classes {
                    let class = self.handles.class(c).map(str::to_owned);
                    candidates.insert(self.handles.intern(Handle::Method {
                        target: JavaTarget {
                            class,
                            name: name.clone(),
                            descriptor: descriptor.clone(),
                            lookup: site,
                        },
                        is_static,
                    }));
                }
                Some(self.handles.join(candidates))
            }
            _ => {
                if let Some(inv) = invocation(field) {
                    let class = if inv.dispatch == "nonvirtual" {
                        args[2]
                    } else {
                        args[1]
                    };
                    self.observe(owner, site, inv, args[inv.method_arg], class);
                    if inv.dispatch == "constructor" {
                        let name = class.and_then(|v| self.handles.class(v)).map(str::to_owned);
                        return Some(self.handles.intern(Handle::Object(name, Reference::Local)));
                    }
                }
                None
            }
        }
    }
    fn writable(&self, at: u64) -> bool {
        self.image.file.sections().any(|s| {
            s.address() != 0
                && at.checked_sub(s.address()).is_some_and(|n| n < s.size())
                && matches!(s.kind(), SectionKind::Data | SectionKind::UninitializedData)
        })
    }
    fn store(&mut self, st: &mut State, dst: Slot, value: Option<Value>) {
        let val = value.unwrap_or(Value::Unknown);
        if !self.local_hypothesis && dst.0 == Place::Data && self.writable(dst.1) {
            let values: BTreeSet<_> = self
                .handles
                .alternatives(Some(val))
                .into_iter()
                .map(|v| {
                    if self.handles.persistent(v) {
                        v
                    } else {
                        Value::Unknown
                    }
                })
                .collect();
            let hist = self.writes.entry(dst).or_default();
            if hist.len() < MAX_TARGETS {
                hist.extend(values);
            } else {
                hist.insert(Value::Unknown);
            }
        }
        st.write(dst, Some(val));
    }
    fn opaque(&mut self, st: &mut State, args: &[Option<Value>]) {
        for a in args.iter().copied().flatten() {
            let Some(slot) = a.slot(self.machine.width) else {
                continue;
            };
            if slot.0 == Place::Data && self.writable(slot.1) {
                self.escaped.insert(slot);
                self.store(st, slot, None);
            } else if slot.0 == Place::Stack {
                st.write(slot, Some(Value::Unknown));
            }
        }
    }
    fn transfer(&mut self, st: &mut State, op: &FullOp) {
        if op.opcode == OpCode::CPUI_STORE {
            let p = self.machine.value(st, &op.ins[1]);
            let v = self.machine.value(st, &op.ins[2]);
            if let Some(dst) = p.and_then(|p| p.slot(op.ins[2].size)) {
                self.store(st, dst, v);
            } else {
                st.forget_memory();
            }
        } else if let Some(dst) = op.out.as_ref().filter(|v| {
            v.space
                .as_ref()
                .is_some_and(|s| s.get_index() == self.machine.code_space)
        }) {
            let slot = Slot(Place::Data, dst.offset, dst.size);
            self.machine
                .transfer(st, op, &|v, n| deref(v, n, self.machine.width));
            self.store(st, slot, st.read(slot));
        } else {
            self.machine
                .transfer(st, op, &|v, n| deref(v, n, self.machine.width));
        }
    }
    fn body(&self, at: u64) -> bool {
        self.image.file.sections().any(|s| {
            s.kind() == SectionKind::Text
                && s.address() != 0
                && at.checked_sub(s.address()).is_some_and(|d| d < s.size())
                && s.name()
                    .is_ok_and(|name| !name.starts_with(".plt") && !name.starts_with(".iplt"))
        })
    }
    fn direct_target(&self, raw: u64, site: u64, dest: u64, call: bool) -> Option<Value> {
        if self.image.file.architecture() != object::Architecture::Arm {
            return Some(Value::Constant(dest));
        }
        nativeflow::direct_entry(true, raw, self.image.data(site)?, dest, call).map(Value::Constant)
    }
    fn invoke(
        &mut self,
        owner: u64,
        site: u64,
        target: Option<Value>,
        mut state: State,
        ancestors: &mut Vec<u64>,
    ) -> Vec<State> {
        let args = (0..8)
            .map(|i| self.machine.argument(&state, i).and_then(|s| state.read(s)))
            .collect::<Vec<_>>();
        if let Some(Value::Function(index)) = target {
            let result = self.jni(owner, site, index, &args, &mut state);
            self.machine.after_call(&mut state);
            state.write(self.machine.result, result);
            return vec![state];
        }
        if let Some(va) = target
            .and_then(Value::constant)
            .filter(|&a| !self.local_hypothesis && self.body(self.image.pc(a)))
        {
            let child = self.bounds.get(&self.image.pc(va)).map_or(va, |b| b.0);
            if ancestors.len() < 8 && !ancestors.contains(&self.image.pc(child)) && self.cap > 0 {
                let exits = self.run(child, state.clone(), ancestors);
                if !exits.is_empty() {
                    self.machine.after_call(&mut state);
                    return exits
                        .into_iter()
                        .map(|exit| {
                            let mut out = state.clone();
                            out.0.retain(|k, _| matches!(k.0, Place::Register(_)));
                            out.0.extend(
                                exit.0
                                    .iter()
                                    .filter(|(k, _)| !matches!(k.0, Place::Register(_)))
                                    .map(|(k, v)| (*k, *v)),
                            );
                            out.write(self.machine.result, exit.read(self.machine.result));
                            out
                        })
                        .collect();
                }
            } else {
                self.limited = true;
            }
        }
        self.opaque(&mut state, &args);
        self.machine.after_call(&mut state);
        state.write(self.machine.result, None);
        vec![state]
    }
    fn instruction(
        &mut self,
        addr: &Address,
        capture: &mut FullCapture,
    ) -> Option<(u64, Option<Rc<[FullOp]>>)> {
        let pc = addr.get_offset();
        let translator = self.machine.arch.translate();
        if let Some(i) = self.instructions.get(&pc) {
            if translator.matches_decode_context(addr, &i.context) {
                return Some((i.len, Some(Rc::clone(&i.ops))));
            }
        }
        capture.begin();
        let mut key = None;
        let len = u64::try_from(
            translator
                .one_instruction_checked_reusable(capture, addr, self.mapped.as_ref(), &mut key)
                .ok()?,
        )
        .ok()
        .filter(|&n| n > 0)?;
        if let Some(context) =
            key.filter(|_| self.instructions.len() < 2048 && capture.ops().len() <= 64)
        {
            self.instructions.insert(
                pc,
                Instruction {
                    len,
                    context,
                    ops: Rc::from(capture.ops()),
                },
            );
        }
        Some((len, None))
    }
    fn run(&mut self, raw: u64, initial: State, ancestors: &mut Vec<u64>) -> Vec<State> {
        let owner = self.image.pc(raw);
        let mut exits = Vec::new();
        let arch = self.machine.arch;
        let Some(code) = arch.manage().get_default_code_space() else {
            self.limited = true;
            return exits;
        };
        let Some(_scope) = nativeflow::decode_scope(
            arch,
            raw,
            self.image.file.architecture() == object::Architecture::Arm,
        ) else {
            self.limited = true;
            return exits;
        };
        ancestors.push(owner);
        let limit = self
            .bounds
            .get(&owner)
            .map(|v| v.1)
            .filter(|&n| n != 0)
            .unwrap_or(32768)
            .min(32768);
        let end = owner.saturating_add(limit);
        let mut queue = VecDeque::from([(owner, initial)]);
        let mut seen = BTreeMap::<u64, Vec<State>>::new();
        let mut cap = FullCapture::default();
        while let Some((pc, st)) = queue.pop_front() {
            if pc < owner || pc >= end {
                self.limited = true;
                continue;
            }
            let states = seen.entry(pc).or_default();
            if states.contains(&st) {
                continue;
            }
            if states.len() >= MAX_PATHS || exits.len() >= MAX_PATHS {
                self.limited = true;
                continue;
            }
            if self.cap == 0 {
                self.limited = true;
                break;
            }
            self.cap -= 1;
            states.push(st.clone());
            let addr = Address::new(Rc::clone(code), pc);
            let Some((len, cached)) = self.instruction(&addr, &mut cap) else {
                self.limited = true;
                continue;
            };
            let next = pc + len;
            let ops = cached.as_deref().unwrap_or_else(|| cap.ops());
            let mut micro = VecDeque::from([(0, st)]);
            let mut steps = 0;
            while let Some((i, mut state)) = micro.pop_front() {
                if i == ops.len() {
                    nativeflow::discard_temporaries(&mut state, arch);
                    queue.push_back((next, state));
                    continue;
                }
                steps += 1;
                if steps > 256 {
                    self.limited = true;
                    break;
                }
                let Some(op) = ops.get(i) else { continue };
                use OpCode::*;
                match op.opcode {
                    CPUI_RETURN => exits.push(state),
                    CPUI_BRANCH | CPUI_CBRANCH => {
                        let Some(dst) = op.ins.first() else { continue };
                        let cond = if op.opcode == CPUI_BRANCH {
                            Some(1)
                        } else {
                            op.ins
                                .get(1)
                                .and_then(|v| self.machine.value(&state, v))
                                .and_then(Value::constant)
                        };
                        if cond != Some(0) {
                            let mut branch = state.clone();
                            if dst
                                .space
                                .as_ref()
                                .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
                            {
                                let j = i as i64 + i64::from(dst.offset as i32);
                                if let Ok(j) = usize::try_from(j) {
                                    micro.push_back((j, branch));
                                }
                            } else if dst
                                .space
                                .as_ref()
                                .is_some_and(|s| s.get_index() == self.machine.code_space)
                            {
                                nativeflow::discard_temporaries(&mut branch, arch);
                                let target = self.image.pc(dst.offset);
                                if (target < owner
                                    || target >= end
                                    || target != owner && self.bounds.contains_key(&target))
                                    && op.opcode == CPUI_BRANCH
                                {
                                    exits.extend(self.invoke(
                                        owner,
                                        pc,
                                        self.direct_target(raw, pc, dst.offset, false),
                                        branch,
                                        ancestors,
                                    ));
                                } else {
                                    queue.push_back((target, branch));
                                }
                            }
                        }
                        if op.opcode == CPUI_CBRANCH && cond.is_none_or(|n| n == 0) {
                            micro.push_back((i + 1, state));
                        }
                    }
                    CPUI_CALL | CPUI_CALLIND | CPUI_BRANCHIND => {
                        let target = if op.opcode == CPUI_CALL {
                            op.ins
                                .first()
                                .and_then(|v| self.direct_target(raw, pc, v.offset, true))
                        } else {
                            op.ins.first().and_then(|v| self.machine.value(&state, v))
                        };
                        if op.opcode == CPUI_CALL
                            && self.machine.stack_return
                            && target == Some(Value::Constant(next))
                        {
                            micro.push_back((i + 1, state));
                            continue;
                        }
                        let out = self.invoke(owner, pc, target, state, ancestors);
                        if op.opcode == CPUI_BRANCHIND {
                            exits.extend(out);
                        } else {
                            micro.extend(out.into_iter().map(|st| (i + 1, st)));
                        }
                    }
                    _ => {
                        self.transfer(&mut state, op);
                        micro.push_back((i + 1, state));
                    }
                }
            }
        }
        ancestors.pop();
        exits
    }
    fn root(&mut self, raw: u64, args: &[(Input, Value)], initial: &State) -> Vec<State> {
        let mut state = initial.clone();
        state.write(self.machine.sp, Some(Value::Stack(0)));
        for &(input, v) in args {
            let slot = match input {
                Input::Storage(k) => Some(k),
                Input::Argument(i) => self.machine.argument(&state, i),
            };
            if let Some(k) = slot {
                state.write(k, Some(v));
            }
        }
        self.run(raw, state, &mut Vec::new())
    }
}
fn interface(ty: &kuna_decomp::dtype::Datatype) -> Option<Value> {
    let pointed = ty.get_ptr_to()?;
    let tag = match pointed.get_name() {
        "JNIEnv" | "_JNIEnv" => ENV,
        "JavaVM" | "_JavaVM" => VM,
        _ => match pointed.get_ptr_to()?.get_name() {
            "JNINativeInterface" => ENV,
            "JNIInvokeInterface" => VM,
            _ => return None,
        },
    };
    Some(Value::Handle(tag, 0))
}

fn declared_inputs(arch: &Architecture, p: &kuna_decomp::fspec::PrototypePieces) -> Option<Inputs> {
    if !p.intypes.iter().any(|t| interface(t).is_some()) {
        return None;
    }
    let model = arch
        .function_prototype_model(&p.name)
        .cloned()
        .or_else(|| kuna_decomp::kuna_typedcallabi::undeclared_model_for(arch, p))
        .or_else(|| arch.default_fp().cloned())?;
    let mut proto = kuna_decomp::fspec::FuncProto::new();
    proto
        .seed_locked_from_pieces(
            p,
            model,
            arch.types().get_type_void().ok()?,
            arch.types(),
            arch.manage(),
        )
        .ok()?;
    Some(
        (0..proto.num_params())
            .filter_map(|i| {
                let param = proto.get_param(i)?;
                let ty = param.get_type()?;
                let value = interface(ty)?;
                let addr = param.get_address();
                let space = addr.get_space()?;
                let place = if space.get_type() == spacetype::IPTR_SPACEBASE {
                    Place::Stack
                } else if space.get_name() == "register" {
                    Place::Register(space.get_index())
                } else {
                    return None;
                };
                Some((
                    Input::Storage(Slot(place, addr.get_offset(), ty.get_size() as u32)),
                    value,
                ))
            })
            .collect(),
    )
}

/// A short JNI export or a long overload with a valid parameter descriptor can
/// supply a local hypothesis about its first argument; neither proves its ABI.
fn valid_export(name: &str) -> bool {
    let Some(suffix) = name.strip_prefix("Java_") else {
        return false;
    };
    let valid_path =
        |s: &str| super::unescape(s).is_some_and(|s| valid_class(&s) && s.contains('/'));
    valid_path(suffix)
        || suffix.match_indices("__").any(|(i, _)| {
            valid_path(&suffix[..i])
                && super::unescape(&suffix[i + 2..])
                    .and_then(|s| super::parameters(&s))
                    .is_some()
        })
}

fn roots(
    file: &object::File,
    arch: &Architecture,
    methods: &[NativeMethod],
) -> (Bounds, BTreeMap<u64, Inputs>, Vec<u64>, BTreeSet<u64>) {
    let arm = file.architecture() == object::Architecture::Arm;
    let pc = |raw: u64| if arm { raw & !1 } else { raw };
    let mut hooks = Vec::new();
    let mut hypotheses = BTreeSet::new();
    let mut roots: BTreeMap<_, _> = methods
        .iter()
        .filter(|m| m.declaration.is_some())
        .map(|m| {
            (
                m.raw_entry,
                vec![(Input::Argument(0), Value::Handle(ENV, 0))],
            )
        })
        .collect();
    for sym in file
        .dynamic_symbols()
        .filter(|s| s.is_definition() && !s.is_local() && s.kind() == SymbolKind::Text)
    {
        match sym.name() {
            Ok("JNI_OnLoad") => hooks.push(sym.address()),
            Ok("JNI_OnUnload") => {
                roots.insert(
                    sym.address(),
                    vec![(Input::Argument(0), Value::Handle(VM, 0))],
                );
            }
            Ok(name) if valid_export(name) && !roots.contains_key(&sym.address()) => {
                hypotheses.insert(sym.address());
            }
            _ => {}
        }
    }
    let mut declared = Vec::new();
    for (space, addr, p) in arch.symboltab.build_callee_proto_pieces() {
        if arch
            .manage()
            .get_default_code_space()
            .is_some_and(|s| s.get_index() == space)
        {
            if !p.intypes.is_empty() {
                hypotheses.remove(&addr);
                if arm {
                    hypotheses.remove(&(addr | 1));
                }
            }
            if let Some(inputs) = declared_inputs(arch, &p) {
                if !inputs.is_empty() {
                    declared.push((addr, inputs));
                }
            }
        }
    }
    if roots.is_empty() && hooks.is_empty() && hypotheses.is_empty() && declared.is_empty() {
        return (BTreeMap::new(), roots, hooks, hypotheses);
    }
    let bounds: Bounds = file
        .symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.is_definition() && s.kind() == SymbolKind::Text)
        .map(|s| (pc(s.address()), (s.address(), s.size())))
        .collect();
    for (addr, inputs) in declared {
        let raw = bounds.get(&pc(addr)).map(|p| p.0).unwrap_or_else(|| {
            if arm {
                arch.manage()
                    .get_default_code_space()
                    .and_then(|space| {
                        let address = Address::new(Rc::clone(space), pc(addr));
                        arch.with_context_db(|db| db.get_variable_value(b"TMode", &address))
                            .ok()
                    })
                    .map(|thumb| pc(addr) | u64::from(thumb & 1))
                    .unwrap_or(addr)
            } else {
                addr
            }
        });
        roots.insert(raw, inputs);
    }
    hooks.sort();
    hooks.dedup();
    for hook in &hooks {
        roots.remove(hook);
    }
    (bounds, roots, hooks, hypotheses)
}

/// Recover proven JNI call sites and indicate whether a resource or decoding limit cut the search short.
pub fn java_calls<'d>(
    file: &'d object::File<'d>,
    bytes: &'d [u8],
    arch: &Architecture,
    methods: &[NativeMethod],
) -> (Vec<JavaCall>, bool) {
    if !arch.android_jni {
        return (Vec::new(), false);
    }
    let (bounds, roots, hooks, hypotheses) = roots(file, arch, methods);
    if roots.is_empty() && hooks.is_empty() && hypotheses.is_empty() {
        return (Vec::new(), false);
    }
    let Some(machine) = Machine::new(file, bytes, arch) else {
        return (Vec::new(), true);
    };
    let Some(mapped) = arch
        .translate()
        .loader_rc()
        .try_borrow()
        .ok()
        .and_then(|l| l.shared_bytes())
    else {
        return (Vec::new(), true);
    };
    let mut flow = Flow {
        machine,
        image: Image {
            file,
            bytes,
            pointers: OnceCell::new(),
        },
        bounds,
        handles: Handles::default(),
        mapped,
        calls: BTreeMap::new(),
        instructions: BTreeMap::new(),
        writes: BTreeMap::new(),
        escaped: BTreeSet::new(),
        cap: 65536,
        limited: false,
        local_hypothesis: false,
    };
    let mut initialized = State::default();
    for hook in hooks {
        let exits = flow.root(
            hook,
            &[(Input::Argument(0), Value::Handle(VM, 0))],
            &initialized,
        );
        let success: Vec<_> = exits
            .iter()
            .filter(|s| {
                let ret = s.read(flow.machine.result).and_then(Value::constant);
                ret.is_none_or(|n| n as i32 >= 0)
            })
            .collect();
        if let Some(first) = success.first() {
            for (&slot, &v) in &first.0 {
                if slot.0 == Place::Data
                    && flow.handles.persistent(v)
                    && success.iter().all(|s| s.read(slot) == Some(v))
                {
                    initialized.write(slot, Some(v));
                }
            }
        }
    }
    let initial_calls = flow.calls.clone();
    let mut cache = initialized.clone();
    let mut converged = false;
    let mut inspect_external = true;
    let mut unbounded_writers = false;
    let public_slots: Vec<_> = file
        .dynamic_symbols()
        .filter(|s| s.is_definition() && !s.is_local() && s.kind() == SymbolKind::Data)
        .map(|s| s.address()..s.address().saturating_add(s.size().max(1)))
        .collect();
    for _ in 0..4 {
        flow.calls = initial_calls.clone();
        for (&raw, args) in &roots {
            if flow.cap == 0 {
                flow.limited = true;
                break;
            }
            flow.root(raw, args, &cache);
        }
        if inspect_external
            && flow
                .writes
                .values()
                .any(|vs| vs.iter().any(|&v| flow.handles.persistent(v)))
        {
            inspect_external = false;
            let old_cap = flow.cap;
            let old_limited = flow.limited;
            flow.cap = old_cap.min(16384);
            let allowed = flow.cap;
            flow.limited = false;
            for sym in file
                .dynamic_symbols()
                .filter(|s| s.is_definition() && !s.is_local() && s.kind() == SymbolKind::Text)
            {
                if flow.cap == 0 {
                    flow.limited = true;
                    break;
                }
                let raw = sym.address();
                if roots.contains_key(&raw) || sym.name() == Ok("JNI_OnLoad") {
                    continue;
                }
                if flow.body(flow.image.pc(raw)) {
                    flow.root(raw, &[], &cache);
                }
            }
            unbounded_writers = flow.limited;
            flow.cap = old_cap - (allowed - flow.cap);
            flow.limited = old_limited;
        }
        let mut next = State::default();
        for (&slot, values) in &flow.writes {
            if slot.0 != Place::Data || !values.iter().any(|&v| flow.handles.get(v).is_some()) {
                continue;
            }
            let mut candidates = values.clone();
            if unbounded_writers
                || public_slots.iter().any(|r| r.contains(&slot.1))
                || initialized.read(slot).is_none()
                || flow.escaped.contains(&slot)
                || flow.writes.keys().any(|&other| {
                    other != slot
                        && other.0 == slot.0
                        && other.1.saturating_sub(slot.1) < u64::from(slot.2)
                        && slot.1.saturating_sub(other.1) < u64::from(other.2)
                })
            {
                candidates.insert(Value::Unknown);
            }
            next.write(slot, Some(flow.handles.join(candidates)));
        }
        if next == cache {
            converged = true;
            break;
        }
        if flow.cap == 0 {
            break;
        }
        cache = next;
    }
    let mut proven = std::mem::take(&mut flow.calls);
    if flow.limited || !converged {
        for c in proven.values_mut() {
            c.incomplete = true;
        }
    }
    let mut limited = flow.limited || unbounded_writers || !converged;
    flow.local_hypothesis = true;
    let mut budget: usize = 8192;
    for raw in hypotheses {
        if budget == 0 {
            limited = true;
            break;
        }
        let allowance = budget.min(1024);
        flow.cap = allowance;
        flow.limited = false;
        flow.root(
            raw,
            &[(Input::Argument(0), Value::Handle(ENV, 0))],
            &State::default(),
        );
        budget -= allowance - flow.cap;
        limited |= flow.limited;
        for (key, mut call) in std::mem::take(&mut flow.calls) {
            if !call.incomplete && !call.targets.is_empty() {
                call.incomplete = true;
                proven.entry(key).or_insert(call);
            }
        }
    }
    (proven.into_values().collect(), limited)
}
