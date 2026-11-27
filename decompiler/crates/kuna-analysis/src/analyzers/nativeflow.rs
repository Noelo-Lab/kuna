//! Bounded machine values and ABI storage used by Android's native metadata analyses.
use crate::{
    listing::xrefs::FullOp,
    loader::kuna_dataptrs::{data_pointers, DataPointers},
};
use kuna_base::{address::Address, space::spacetype};
use kuna_decomp::{architecture::Architecture, fspec::effect_type};
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use object::{Object, ObjectSection, SectionFlags};
use std::{cell::OnceCell, collections::BTreeMap, rc::Rc};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Place {
    Register(i32),
    Stack,
    Data,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Slot(pub Place, pub u64, pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Value {
    Constant(u64),
    Stack(u64),
    Tagged(u64, u64),
    Function(u64),
    Handle(u64, u64),
    Predicate { tag: u64, equals: u64, invert: bool },
    Unknown,
}
impl Value {
    pub(crate) fn constant(self) -> Option<u64> {
        if let Self::Constant(n) = self {
            Some(n)
        } else {
            None
        }
    }
    pub(crate) fn add(self, off: u64, mask: u64) -> Option<Self> {
        let sum = |x: u64| x.wrapping_add(off) & mask;
        Some(match self {
            Self::Constant(x) => Self::Constant(sum(x)),
            Self::Stack(x) => Self::Stack(sum(x)),
            Self::Tagged(tag, n) => Self::Tagged(tag, sum(n)),
            value if off == 0 => value,
            _ => return None,
        })
    }
    pub(crate) fn slot(self, width: u32) -> Option<Slot> {
        match self {
            Self::Stack(x) => Some(Slot(Place::Stack, x, width)),
            Self::Constant(x) => Some(Slot(Place::Data, x, width)),
            _ => None,
        }
    }
}
pub(crate) fn mask(size: u32) -> u64 {
    u64::MAX.checked_shr(64 - size.min(8) * 8).unwrap_or(0)
}
pub(crate) fn register(v: &VarnodeData) -> Option<Slot> {
    Some(Slot(
        Place::Register(v.space.as_ref()?.get_index()),
        v.offset,
        v.size,
    ))
}
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub(crate) struct State(pub BTreeMap<Slot, Value>);
impl State {
    pub(crate) fn read(&self, slot: Slot) -> Option<Value> {
        if let Some(&value) = self.0.get(&slot) {
            return Some(value);
        }
        let Slot(place, at, width) = slot;
        let (&Slot(other, base, n), &value) =
            self.0.range(..=Slot(place, at, u32::MAX)).next_back()?;
        if other != place || at.checked_sub(base)?.checked_add(u64::from(width))? > u64::from(n) {
            return None;
        }
        if at == base && width >= 4 && matches!(value, Value::Tagged(_, _)) {
            return Some(value);
        }
        Some(Value::Constant(
            value
                .constant()?
                .checked_shr((8 * (at - base)).try_into().ok()?)?
                & mask(width),
        ))
    }
    pub(crate) fn write(&mut self, slot: Slot, value: Option<Value>) {
        let Slot(place, base, width) = slot;
        self.0.retain(|&Slot(other, at, n), _| {
            place != other
                || base.saturating_sub(at) >= u64::from(n)
                || at.saturating_sub(base) >= u64::from(width)
        });
        if let Some(v) = value {
            self.0.insert(slot, v);
        }
    }
    pub(crate) fn forget_memory(&mut self) {
        self.0.retain(|s, _| matches!(s.0, Place::Register(_)));
    }
}

pub(crate) struct Machine<'a, 'd> {
    pub(crate) arch: &'a Architecture,
    file: &'a object::File<'d>,
    bytes: &'d [u8],
    pointers: OnceCell<DataPointers>,
    relro: Option<(u64, u64)>,
    pub(crate) width: u32,
    pub(crate) code_space: i32,
    pub(crate) sp: Slot,
    pub(crate) result: Slot,
    args: Vec<Slot>,
    pub(crate) stack_return: bool,
}
impl<'a, 'd> Machine<'a, 'd> {
    pub(crate) fn new(
        file: &'a object::File<'d>,
        bytes: &'d [u8],
        arch: &'a Architecture,
    ) -> Option<Self> {
        use object::Architecture::*;
        let (sp, ret, args, stack_return): (_, _, &[&str], _) = match file.architecture() {
            Aarch64 => (
                "sp",
                "x0",
                &["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
                false,
            ),
            Arm => ("sp", "r0", &["r0", "r1", "r2", "r3"], false),
            X86_64 => (
                "RSP",
                "RAX",
                &["RDI", "RSI", "RDX", "RCX", "R8", "R9"],
                true,
            ),
            I386 => ("ESP", "EAX", &[], true),
            _ => return None,
        };
        let key = |name: &str| register(&arch.get_register_varnode(name.as_bytes()).ok()?);
        Some(Self {
            arch,
            file,
            bytes,
            pointers: OnceCell::new(),
            relro: crate::loader::kuna_dynrelocs::relro_range(bytes),
            width: if file.is_64() { 8 } else { 4 },
            code_space: arch.manage().get_default_code_space()?.get_index(),
            sp: key(sp)?,
            result: key(ret)?,
            args: args.iter().map(|r| key(r)).collect::<Option<_>>()?,
            stack_return,
        })
    }
    pub(crate) fn argument(&self, st: &State, i: usize) -> Option<Slot> {
        if let Some(&r) = self.args.get(i) {
            return Some(r);
        }
        let words = i - self.args.len() + usize::from(self.stack_return);
        st.read(self.sp)?
            .add((words as u64) * u64::from(self.width), mask(self.width))?
            .slot(self.width)
    }
    pub(crate) fn after_call(&self, st: &mut State) {
        let sp = st.read(self.sp);
        st.0.retain(|&Slot(place, at, width), _| match place {
            Place::Register(i) => self
                .arch
                .manage()
                .get_space(i)
                .zip(self.arch.default_fp())
                .is_some_and(|(s, m)| {
                    m.has_effect(&Address::new(Rc::clone(s), at), width as i32)
                        == effect_type::UNAFFECTED
                }),
            _ => true,
        });
        st.write(
            self.sp,
            sp.and_then(|v| {
                v.add(
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
    pub(crate) fn relocated(&self, va: u64) -> Option<u64> {
        let pointers = &self
            .pointers
            .get_or_init(|| data_pointers(self.file, self.bytes))
            .pointers;
        let p = &pointers[pointers.binary_search_by_key(&va, |p| p.slot).ok()?];
        (u32::from(p.width) == self.width).then_some(p.target)
    }
    fn load(&self, st: &State, addr: Value, size: u32) -> Option<Value> {
        let slot = addr.slot(size)?;
        if let Some(v) = st.read(slot) {
            return Some(v);
        }
        let va = addr.constant()?;
        if !(1..=8).contains(&size) {
            return None;
        }
        for s in self.file.sections() {
            let Some(off) = va
                .checked_sub(s.address())
                .filter(|&n| s.address() != 0 && n < s.size())
            else {
                continue;
            };
            let readonly = matches!(s.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 3 == 2);
            let frozen = self.relro.is_some_and(|(lo, hi)| {
                lo <= va && va.checked_add(u64::from(size)).is_some_and(|end| end <= hi)
            });
            if !readonly && !frozen {
                return None;
            }
            if size == self.width {
                if let Some(v) = self.relocated(va) {
                    return Some(Value::Constant(v));
                }
            }
            let data = s.data().ok()?.get(off as usize..)?.get(..size as usize)?;
            let mut word = [0; 8];
            word[..data.len()].copy_from_slice(data);
            return Some(Value::Constant(u64::from_le_bytes(word)));
        }
        None
    }
    pub(crate) fn value(&self, st: &State, v: &VarnodeData) -> Option<Value> {
        let space = v.space.as_ref()?;
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Some(Value::Constant(v.offset));
        }
        if space.get_index() == self.code_space {
            return self.load(st, Value::Constant(v.offset), v.size);
        }
        st.read(register(v)?)
    }
    pub(crate) fn transfer(
        &self,
        st: &mut State,
        op: &FullOp,
        load: &dyn Fn(Value, u32) -> Option<Value>,
    ) {
        use OpCode::*;
        if op.opcode == CPUI_STORE {
            let Some(dst) = self
                .value(st, &op.ins[1])
                .and_then(|v| v.slot(op.ins[2].size))
            else {
                st.forget_memory();
                return;
            };
            let v = self.value(st, &op.ins[2]);
            st.write(dst, v);
            return;
        }
        let Some(out) = op.out.as_ref() else { return };
        let Some(space) = out.space.as_ref() else {
            return;
        };
        let dest = if space.get_index() == self.code_space {
            Slot(Place::Data, out.offset, out.size)
        } else {
            register(out).unwrap()
        };
        let value = self.compute(st, op, out.size, load);
        st.write(dest, value);
    }
    fn compute(
        &self,
        st: &State,
        op: &FullOp,
        size: u32,
        load: &dyn Fn(Value, u32) -> Option<Value>,
    ) -> Option<Value> {
        use OpCode::*;
        let arg = |i| self.value(st, op.ins.get(i)?);
        if op.opcode == CPUI_LOAD {
            let p = arg(1)?;
            return load(p, size).or_else(|| self.load(st, p, size));
        }
        let a = arg(0)?;
        let bits = mask(size);
        if matches!(op.opcode, CPUI_COPY | CPUI_CAST) {
            return Some(a);
        }
        if matches!(op.opcode, CPUI_INT_NEGATE | CPUI_INT_2COMP) {
            let n = a.constant()?;
            return Some(Value::Constant(
                (if op.opcode == CPUI_INT_NEGATE {
                    !n
                } else {
                    n.wrapping_neg()
                }) & bits,
            ));
        }
        if op.opcode == CPUI_BOOL_NEGATE {
            return match a {
                Value::Predicate {
                    tag,
                    equals,
                    invert,
                } => Some(Value::Predicate {
                    tag,
                    equals,
                    invert: !invert,
                }),
                _ => Some(Value::Constant(u64::from(a.constant()? == 0))),
            };
        }
        if matches!(op.opcode, CPUI_INT_ZEXT | CPUI_INT_SEXT) {
            if matches!(a, Value::Tagged(..)) {
                return Some(a);
            }
            let x = a.constant()?;
            let n = op.ins[0].size.min(8) * 8;
            return Some(Value::Constant(
                (if op.opcode == CPUI_INT_SEXT && n > 0 {
                    (((x << (64 - n)) as i64) >> (64 - n)) as u64
                } else {
                    x
                }) & bits,
            ));
        }
        let b = arg(1)?;
        if op.opcode == CPUI_INT_ADD {
            return if let Some(n) = b.constant() {
                a.add(n, bits)
            } else {
                b.add(a.constant()?, bits)
            };
        }
        if matches!(op.opcode, CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL) {
            let predicate = match (a, b) {
                (p @ Value::Predicate { .. }, Value::Constant(c))
                | (Value::Constant(c), p @ Value::Predicate { .. })
                    if c <= 1 =>
                {
                    Some((p, c))
                }
                _ => None,
            };
            if let Some((
                Value::Predicate {
                    tag,
                    equals,
                    invert,
                },
                c,
            )) = predicate
            {
                return Some(Value::Predicate {
                    tag,
                    equals,
                    invert: invert ^ ((op.opcode == CPUI_INT_EQUAL) == (c == 0)),
                });
            }
            if let (Value::Tagged(tag, n), Some(c)) = (a, b.constant()) {
                return Some(Value::Predicate {
                    tag,
                    equals: c.wrapping_sub(n) & mask(op.ins[0].size),
                    invert: op.opcode == CPUI_INT_NOTEQUAL,
                });
            }
        }
        let n = b.constant()?;
        match op.opcode {
            CPUI_INT_SUB => a.add(n.wrapping_neg(), bits),
            CPUI_SUBPIECE if n == 0 && matches!(a, Value::Tagged(..)) && size >= 4 => Some(a),
            CPUI_INT_AND
                if self.file.architecture() == object::Architecture::Arm
                    && n == 0xffff_fffe
                    && matches!(a, Value::Function(_)) =>
            {
                Some(a)
            }
            _ => {
                let x = a.constant()?;
                Some(Value::Constant(
                    match op.opcode {
                        CPUI_INT_AND => x & n,
                        CPUI_INT_OR => x | n,
                        CPUI_INT_XOR => x ^ n,
                        CPUI_INT_EQUAL => u64::from(x == n),
                        CPUI_INT_NOTEQUAL => u64::from(x != n),
                        CPUI_INT_LESS => u64::from(x < n),
                        CPUI_INT_LESSEQUAL => u64::from(x <= n),
                        CPUI_INT_LEFT => x.checked_shl(n.try_into().ok()?).unwrap_or(0),
                        CPUI_INT_RIGHT => x.checked_shr(n.try_into().ok()?).unwrap_or(0),
                        CPUI_SUBPIECE => x
                            .checked_shr(n.checked_mul(8)?.try_into().ok()?)
                            .unwrap_or(0),
                        CPUI_PIECE => x.checked_shl(8 * op.ins[1].size).unwrap_or(0) | n,
                        _ => return None,
                    } & bits,
                ))
            }
        }
    }
}
pub(crate) fn discard_temporaries(st: &mut State, arch: &Architecture) {
    st.0.retain(|k, _| match k.0 {
        Place::Register(i) => arch
            .manage()
            .get_space(i)
            .is_some_and(|s| s.get_type() != spacetype::IPTR_INTERNAL),
        _ => true,
    });
}
/// The tagged target of a direct ARM or Thumb branch. BLX switches instruction sets;
/// B and BL preserve the calling instruction set. Unrecognized encodings have no proven mode.
pub(crate) fn direct_entry(
    arm: bool,
    caller: u64,
    insn: &[u8],
    target: u64,
    call: bool,
) -> Option<u64> {
    if !arm {
        return Some(target);
    }
    let thumb = caller & 1 != 0;
    let half = |at: usize| Some(u16::from_le_bytes(insn.get(at..at + 2)?.try_into().ok()?));
    let dest_thumb = if thumb {
        let h = half(0)?;
        if !call
            && (h & 0xf800 == 0xe000
                || h & 0xf500 == 0xb100
                || h & 0xf000 == 0xd000 && (h >> 8) & 15 < 14)
        {
            true
        } else if h & 0xf800 == 0xf000 {
            let tail = half(2)?;
            match (call, tail & 0xd000) {
                (true, 0xd000) | (false, 0x8000 | 0x9000) => true,
                (true, 0xc000) if tail & 1 == 0 => false,
                _ => return None,
            }
        } else {
            return None;
        }
    } else {
        let w = u32::from_le_bytes(insn.get(..4)?.try_into().ok()?);
        if call && w & 0xfe00_0000 == 0xfa00_0000 {
            true
        } else if w >> 28 != 15 && w & 0x0f00_0000 == if call { 0x0b00_0000 } else { 0x0a00_0000 } {
            false
        } else {
            return None;
        }
    };
    let pc = target & !1;
    (dest_thumb || pc & 3 == 0).then_some(pc | u64::from(dest_thumb))
}

pub(crate) fn decode_scope(
    arch: &Architecture,
    raw: u64,
    arm: bool,
) -> Option<kuna_sleigh::kuna_contextscope::DecodeContextScope<'_>> {
    let t = arch.translate();
    let scope = t.decode_context_scope()?;
    if arm {
        let var = arch
            .with_context_db_mut(|db| db.get_variable(b"TMode"))
            .ok()?;
        let word = var.get_word() as usize;
        let bits = var.get_mask() << var.get_shift();
        let value = ((raw & 1) as u32) << var.get_shift();
        let (prev_mask, prev_value) = t.set_context_read_override(word, 0, 0);
        t.set_context_read_override(word, prev_mask | bits, (prev_value & !bits) | value);
        scope.protect_variable(b"TMode").ok()?;
    }
    Some(scope)
}

#[cfg(test)]
mod tests {
    use super::direct_entry;

    #[test]
    fn direct_arm_branches_preserve_or_switch_instruction_set() {
        let arm_bl = [0x00, 0x00, 0x00, 0xeb];
        let arm_blx = [0x00, 0x00, 0x00, 0xfa];
        let thumb_bl = [0x00, 0xf0, 0x00, 0xf8];
        let thumb_blx = [0x00, 0xf0, 0x00, 0xe8];
        assert_eq!(direct_entry(true, 0, &arm_bl, 0x1000, true), Some(0x1000));
        assert_eq!(direct_entry(true, 0, &arm_blx, 0x1002, true), Some(0x1003));
        assert_eq!(direct_entry(true, 1, &thumb_bl, 0x1002, true), Some(0x1003));
        assert_eq!(
            direct_entry(true, 1, &thumb_blx, 0x1000, true),
            Some(0x1000)
        );
        assert_eq!(direct_entry(true, 1, &thumb_blx, 0x1002, true), None);
        assert_eq!(direct_entry(true, 0, &[0; 4], 0x1000, true), None);
    }
}
