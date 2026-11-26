//! Follow the JNI handles produced within JNI_OnLoad to a concrete registration call.
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

use kuna_base::{address::Address, space::spacetype};
use kuna_decomp::{architecture::Architecture, fspec::effect_type};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use object::Object;

use super::Image;
use crate::listing::xrefs::{FullCapture, FullOp};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Registration {
    pub class: u64,
    pub table: u64,
    pub count: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Place {
    Register(i32),
    Stack,
    Data,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Slot(Place, u64, u32);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Value {
    Constant(u64),
    Stack(u64),
    Vm,
    Env,
    Table { vm: bool, offset: u64 },
    Function { vm: bool, index: u64 },
    Class(u64),
}

impl Value {
    fn constant(self) -> Option<u64> {
        if let Self::Constant(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn add(self, offset: u64, mask: u64) -> Option<Self> {
        let sum = |n: u64| n.wrapping_add(offset) & mask;
        Some(match self {
            Self::Constant(n) => Self::Constant(sum(n)),
            Self::Stack(n) => Self::Stack(sum(n)),
            Self::Table { vm, offset: n } => Self::Table { vm, offset: sum(n) },
            value if offset == 0 => value,
            _ => return None,
        })
    }

    fn slot(self, width: u32) -> Option<Slot> {
        match self {
            Self::Stack(n) => Some(Slot(Place::Stack, n, width)),
            Self::Constant(n) => Some(Slot(Place::Data, n, width)),
            _ => None,
        }
    }
}

fn mask(size: u32) -> u64 {
    u64::MAX.checked_shr(64 - size.min(8) * 8).unwrap_or(0)
}

#[derive(Clone, Default, PartialEq, Eq)]
struct State(BTreeMap<Slot, Value>);

impl State {
    fn read(&self, slot: Slot) -> Option<Value> {
        self.0.get(&slot).copied().or_else(|| {
            let Slot(place, addr, width) = slot;
            let (&Slot(other, base, n), &value) =
                self.0.range(..=Slot(place, addr, u32::MAX)).next_back()?;
            if other != place
                || addr.checked_sub(base)?.checked_add(u64::from(width))? > u64::from(n)
            {
                return None;
            }
            Some(Value::Constant(
                value
                    .constant()?
                    .checked_shr(u32::try_from(8 * (addr - base)).ok()?)?
                    & mask(width),
            ))
        })
    }

    /// GetEnv and the two AttachCurrentThread variants replace the interface in the output slot.
    fn vm_environment(
        &mut self,
        index: u64,
        args: [Option<Value>; 3],
        width: u32,
    ) -> Option<Value> {
        let [receiver, output, version] = args;
        let Some(slot) = output.and_then(|address| address.slot(width)) else {
            self.0.retain(|key, _| matches!(key.0, Place::Register(_)));
            return None;
        };
        let jni = receiver == Some(Value::Vm)
            && (index != 6
                || matches!(
                    version,
                    Some(Value::Constant(0x10001 | 0x10002 | 0x10004 | 0x10006))
                ));
        self.write(slot, jni.then_some(Value::Env));
        jni.then_some(Value::Constant(0))
    }

    fn write(&mut self, dest: Slot, value: Option<Value>) {
        let Slot(place, base, width) = dest;
        self.0.retain(|&Slot(other, at, size), _| {
            place != other
                || base.saturating_sub(at) >= u64::from(size)
                || at.saturating_sub(base) >= u64::from(width)
        });
        if let Some(value) = value {
            self.0.insert(dest, value);
        }
    }
}

struct Abi {
    width: u32,
    sp: Slot,
    result: Slot,
    args: Vec<Slot>,
    stack_return: bool,
}

fn register(v: &VarnodeData) -> Option<Slot> {
    Some(Slot(
        Place::Register(v.space.as_ref()?.get_index()),
        v.offset,
        v.size,
    ))
}

impl Abi {
    fn new(image: &Image, arch: &Architecture) -> Option<Self> {
        use object::Architecture::*;
        let (sp, ret, args, stack_return): (_, _, &[&str], _) = match image.file.architecture() {
            Aarch64 => ("sp", "x0", &["x0", "x1", "x2", "x3"], false),
            Arm => ("sp", "r0", &["r0", "r1", "r2", "r3"], false),
            X86_64 => ("RSP", "RAX", &["RDI", "RSI", "RDX", "RCX"], true),
            I386 => ("ESP", "EAX", &[], true),
            _ => return None,
        };
        let key = |name: &str| register(&arch.get_register_varnode(name.as_bytes()).ok()?);
        Some(Self {
            width: if image.file.is_64() { 8 } else { 4 },
            sp: key(sp)?,
            result: key(ret)?,
            args: args.iter().map(|r| key(r)).collect::<Option<_>>()?,
            stack_return,
        })
    }

    fn argument(&self, st: &State, i: usize) -> Option<Slot> {
        if let Some(&reg) = self.args.get(i) {
            return Some(reg);
        }
        let sp = st.read(self.sp)?;
        sp.add(((i + 1) as u64) * u64::from(self.width), mask(self.width))?
            .slot(self.width)
    }

    fn after_call(&self, st: &mut State, arch: &Architecture) {
        let sp = st.read(self.sp);
        st.0.retain(|&Slot(place, offset, width), _| match place {
            Place::Register(index) => arch
                .manage()
                .get_space(index)
                .zip(arch.default_fp())
                .is_some_and(|(s, model)| {
                    model.has_effect(&Address::new(Rc::clone(s), offset), width as i32)
                        == effect_type::UNAFFECTED
                }),
            _ => true,
        });
        st.write(
            self.sp,
            sp.and_then(|s| {
                s.add(
                    if self.stack_return {
                        u64::from(self.width)
                    } else {
                        0
                    },
                    mask(self.width),
                )
            }),
        );
    }
}

struct Walk<'a, 'd> {
    image: &'a Image<'d>,
    arch: &'a Architecture,
    abi: Abi,
    code_space: i32,
    found: BTreeSet<Registration>,
}

impl Walk<'_, '_> {
    fn load(&self, st: &State, address: Value, width: u32) -> Option<Value> {
        if width == self.abi.width {
            match address {
                Value::Vm | Value::Env => {
                    return Some(Value::Table {
                        vm: address == Value::Vm,
                        offset: 0,
                    })
                }
                Value::Table { vm, offset } if offset % u64::from(width) == 0 => {
                    return Some(Value::Function {
                        vm,
                        index: offset / u64::from(width),
                    });
                }
                _ => {}
            }
        }
        let slot = address.slot(width)?;
        if let Some(value) = st.read(slot) {
            return Some(value);
        }
        let va = address.constant()?;
        if !self.image.code(va) {
            return None;
        }
        let bytes = self.image.data(va)?.get(..width as usize)?;
        let mut word = [0; 8];
        word.get_mut(..bytes.len())?.copy_from_slice(bytes);
        Some(Value::Constant(u64::from_le_bytes(word)))
    }

    fn value(&self, st: &State, v: &VarnodeData) -> Option<Value> {
        let space = v.space.as_ref()?;
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Some(Value::Constant(v.offset));
        }
        if space.get_index() == self.code_space {
            return self.load(st, Value::Constant(v.offset), v.size);
        }
        st.read(register(v)?)
    }

    fn compute(&self, st: &State, op: &FullOp, width: u32) -> Option<Value> {
        use OpCode::*;
        let operand = |i| self.value(st, op.ins.get(i)?);
        if op.opcode == CPUI_LOAD {
            return self.load(st, operand(1)?, width);
        }
        let a = operand(0)?;
        if op.opcode == CPUI_COPY {
            return Some(a);
        }
        if op.opcode == CPUI_INT_ZEXT {
            return Some(Value::Constant(a.constant()? & mask(width)));
        }
        if op.opcode == CPUI_BOOL_NEGATE {
            return Some(Value::Constant(u64::from(a.constant()? == 0)));
        }
        let other = operand(1)?;
        if op.opcode == CPUI_INT_ADD {
            return if let Some(n) = other.constant() {
                a.add(n, mask(width))
            } else {
                other.add(a.constant()?, mask(width))
            };
        }
        let b = other.constant()?;
        if matches!(a, Value::Class(_))
            && b == 0
            && matches!(op.opcode, CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL)
        {
            return Some(Value::Constant(u64::from(op.opcode == CPUI_INT_NOTEQUAL)));
        }
        match op.opcode {
            CPUI_INT_SUB => a.add(b.wrapping_neg(), mask(width)),
            CPUI_INT_AND
                if matches!(a, Value::Function { .. })
                    && b == 0xffff_fffe
                    && self.image.file.architecture() == object::Architecture::Arm =>
            {
                Some(a)
            }
            _ => {
                let a = a.constant()?;
                let val = match op.opcode {
                    CPUI_INT_AND => a & b,
                    CPUI_INT_OR => a | b,
                    CPUI_INT_XOR => a ^ b,
                    CPUI_INT_EQUAL => u64::from(a == b),
                    CPUI_INT_NOTEQUAL => u64::from(a != b),
                    CPUI_INT_LEFT => a.checked_shl(b.try_into().ok()?).unwrap_or(0),
                    CPUI_SUBPIECE => a
                        .checked_shr(u32::try_from(b.checked_mul(8)?).ok()?)
                        .unwrap_or(0),
                    CPUI_PIECE => a.checked_shl(8 * op.ins[1].size).unwrap_or(0) | b,
                    _ => return None,
                };
                Some(Value::Constant(val & mask(width)))
            }
        }
    }

    fn call(&mut self, st: &mut State, target: Option<Value>) {
        let a = |i| self.abi.argument(st, i).and_then(|k| st.read(k));
        let [a0, a1, a2, a3] = [a(0), a(1), a(2), a(3)];
        let mut result = None;
        match (target, a0) {
            (
                Some(Value::Function {
                    vm: true,
                    index: index @ (4 | 6 | 7),
                }),
                receiver,
            ) => {
                result = st.vm_environment(index, [receiver, a1, a2], self.abi.width);
            }
            (
                Some(Value::Function {
                    vm: false,
                    index: 6,
                }),
                Some(Value::Env),
            ) => {
                result = a1.and_then(Value::constant).map(Value::Class);
            }
            (
                Some(Value::Function {
                    vm: false,
                    index: 21 | 25,
                }),
                Some(Value::Env),
            ) => {
                result = a1.filter(|v| matches!(v, Value::Class(_)));
            }
            (
                Some(Value::Function {
                    vm: false,
                    index: 215,
                }),
                Some(Value::Env),
            ) => {
                if let (
                    Some(Value::Class(class)),
                    Some(Value::Constant(table)),
                    Some(Value::Constant(count)),
                ) = (a1, a2, a3)
                {
                    self.found.insert(Registration {
                        class,
                        table,
                        count,
                    });
                }
            }
            (Some(Value::Function { .. }), _) => {}
            _ => st.0.retain(|slot, _| matches!(slot.0, Place::Register(_))),
        }
        self.abi.after_call(st, self.arch);
        st.write(self.abi.result, result);
    }

    fn step(&mut self, st: &mut State, op: &FullOp) {
        use OpCode::*;
        match op.opcode {
            CPUI_CALL | CPUI_CALLIND => {
                let target = if op.opcode == CPUI_CALLIND {
                    self.value(st, &op.ins[0])
                } else {
                    None
                };
                self.call(st, target);
            }
            CPUI_STORE => {
                let Some(dst) = self
                    .value(st, &op.ins[1])
                    .and_then(|v| v.slot(op.ins[2].size))
                else {
                    st.0.retain(|k, _| matches!(k.0, Place::Register(_)));
                    return;
                };
                let value = self.value(st, &op.ins[2]);
                st.write(dst, value);
            }
            _ => {
                let Some(out) = op.out.as_ref() else { return };
                let Some(space) = out.space.as_ref() else {
                    return;
                };
                let dest = if space.get_index() == self.code_space {
                    Slot(Place::Data, out.offset, out.size)
                } else {
                    register(out).unwrap()
                };
                let value = self.compute(st, op, out.size);
                st.write(dest, value);
            }
        }
    }
}

/// Keep paths separate: an error arm may destroy a value held on the success arm.
/// Bounds apply to distinct path states as well as to the sized symbol's code.
pub(super) fn sites(
    image: &Image,
    arch: &Architecture,
    start: u64,
    size: u64,
) -> Vec<Registration> {
    let Some(abi) = Abi::new(image, arch) else {
        return vec![];
    };
    let Some(code) = arch.manage().get_default_code_space() else {
        return vec![];
    };
    let translate = arch.translate();
    let Some(_scope) = translate.decode_context_scope() else {
        return vec![];
    };
    let Some(mapped) = translate
        .loader_rc()
        .try_borrow()
        .ok()
        .and_then(|l| l.shared_bytes())
    else {
        return vec![];
    };
    if image.file.architecture() == object::Architecture::Arm {
        let Ok(var) = arch.with_context_db_mut(|db| db.get_variable(b"TMode")) else {
            return vec![];
        };
        let word = var.get_word() as usize;
        let bits = var.get_mask() << var.get_shift();
        let value = ((start & 1) as u32) << var.get_shift();
        let (prev_mask, prev_value) = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(word, prev_mask | bits, (prev_value & !bits) | value);
        if _scope.protect_variable(b"TMode").is_err() {
            return vec![];
        }
    }
    let start = image.pc(start);
    let mut st = State::default();
    st.write(abi.sp, Some(Value::Stack(0)));
    let Some(vm) = abi.argument(&st, 0) else {
        return vec![];
    };
    st.write(vm, Some(Value::Vm));
    let mut walk = Walk {
        image,
        arch,
        abi,
        code_space: code.get_index(),
        found: BTreeSet::new(),
    };
    let end = start.saturating_add(if size == 0 { 8192 } else { size.min(32768) });
    let mut queue = VecDeque::from([(start, st)]);
    let mut visited = BTreeMap::<u64, Vec<State>>::new();
    let mut capture = FullCapture::default();
    for _ in 0..4096 {
        let Some((pc, mut st)) = queue.pop_front() else {
            break;
        };
        if !(start..end).contains(&pc) {
            continue;
        }
        let states = visited.entry(pc).or_default();
        if states.contains(&st) || states.len() == 8 {
            continue;
        }
        states.push(st.clone());
        capture.begin();
        let addr = Address::new(Rc::clone(code), pc);
        let Ok(len) = translate.one_instruction_checked(&mut capture, &addr, mapped.as_ref())
        else {
            continue;
        };
        if len <= 0 || pc.saturating_add(len as u64) > end {
            continue;
        }
        let mut falls = true;
        for op in capture.ops() {
            use OpCode::*;
            match op.opcode {
                CPUI_BRANCH | CPUI_CBRANCH => {
                    let Some(target) = op.ins.first().filter(|v| {
                        v.space
                            .as_ref()
                            .is_some_and(|s| s.get_index() == code.get_index())
                    }) else {
                        falls = false;
                        break;
                    };
                    let taken = if op.opcode == CPUI_CBRANCH {
                        op.ins
                            .get(1)
                            .and_then(|v| walk.value(&st, v))
                            .and_then(Value::constant)
                    } else {
                        Some(1)
                    };
                    if taken == Some(0) {
                        continue;
                    }
                    let mut branched = st.clone();
                    discard_temporaries(&mut branched, arch);
                    queue.push_back((target.offset, branched));
                    if taken.is_some() {
                        falls = false;
                        break;
                    }
                }
                CPUI_RETURN | CPUI_BRANCHIND => {
                    falls = false;
                    break;
                }
                _ => walk.step(&mut st, op),
            }
        }
        if falls {
            discard_temporaries(&mut st, arch);
            queue.push_back((pc + len as u64, st));
        }
    }
    walk.found.into_iter().collect()
}

fn discard_temporaries(st: &mut State, arch: &Architecture) {
    st.0.retain(|k, _| match k.0 {
        Place::Register(i) => arch
            .manage()
            .get_space(i)
            .is_some_and(|s| s.get_type() != spacetype::IPTR_INTERNAL),
        _ => true,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn another_get_env_interface_replaces_an_existing_jni_environment() {
        let mut state = State::default();
        let output = Some(Value::Stack(8));
        let slot = output.unwrap().slot(8).unwrap();
        for version in [Some(Value::Constant(0x30010200)), None] {
            state.vm_environment(
                6,
                [Some(Value::Vm), output, Some(Value::Constant(0x10006))],
                8,
            );
            assert!(state.read(slot) == Some(Value::Env));
            assert!(state
                .vm_environment(6, [Some(Value::Vm), output, version], 8)
                .is_none());
            assert!(state.read(slot).is_none());
        }
    }
}
