//! Class-file writing: the constant pool, members and attributes, and a
//! bytecode assembler with labels, branch fixups and stack-depth tracking.

use std::collections::BTreeMap;

use crate::{err, Result};

pub const ACC_SUPER: u16 = 0x0020;

/// A constant pool deduplicated on its encoded entries.
#[derive(Default)]
pub struct Pool {
    bytes: Vec<u8>,
    next: u32,
    index: BTreeMap<Vec<u8>, u16>,
    overflow: bool,
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            next: 1,
            ..Pool::default()
        }
    }

    fn intern(&mut self, entry: Vec<u8>, slots: u32) -> u16 {
        if let Some(&i) = self.index.get(&entry) {
            return i;
        }
        if self.next + slots > 0xffff {
            self.overflow = true;
            return 0;
        }
        let i = self.next as u16;
        self.next += slots;
        self.bytes.extend_from_slice(&entry);
        self.index.insert(entry, i);
        i
    }

    /// A `CONSTANT_Utf8` from modified UTF-8 bytes.
    pub fn utf8(&mut self, s: &[u8]) -> u16 {
        if s.len() > 0xffff {
            self.overflow = true;
            return 0;
        }
        let mut e = vec![1];
        e.extend_from_slice(&(s.len() as u16).to_be_bytes());
        e.extend_from_slice(s);
        self.intern(e, 1)
    }

    fn ref1(&mut self, tag: u8, a: u16) -> u16 {
        let mut e = vec![tag];
        e.extend_from_slice(&a.to_be_bytes());
        self.intern(e, 1)
    }

    fn ref2(&mut self, tag: u8, a: u16, b: u16) -> u16 {
        let mut e = vec![tag];
        e.extend_from_slice(&a.to_be_bytes());
        e.extend_from_slice(&b.to_be_bytes());
        self.intern(e, 1)
    }

    /// A `CONSTANT_Class` from an internal name or array descriptor.
    pub fn class(&mut self, name: &[u8]) -> u16 {
        let u = self.utf8(name);
        self.ref1(7, u)
    }

    pub fn string(&mut self, s: &[u8]) -> u16 {
        let u = self.utf8(s);
        self.ref1(8, u)
    }

    pub fn int(&mut self, v: i32) -> u16 {
        let mut e = vec![3];
        e.extend_from_slice(&v.to_be_bytes());
        self.intern(e, 1)
    }

    pub fn float(&mut self, bits: u32) -> u16 {
        let mut e = vec![4];
        e.extend_from_slice(&bits.to_be_bytes());
        self.intern(e, 1)
    }

    pub fn long(&mut self, v: i64) -> u16 {
        let mut e = vec![5];
        e.extend_from_slice(&v.to_be_bytes());
        self.intern(e, 2)
    }

    pub fn double(&mut self, bits: u64) -> u16 {
        let mut e = vec![6];
        e.extend_from_slice(&bits.to_be_bytes());
        self.intern(e, 2)
    }

    pub fn name_and_type(&mut self, name: &[u8], desc: &[u8]) -> u16 {
        let n = self.utf8(name);
        let d = self.utf8(desc);
        self.ref2(12, n, d)
    }

    pub fn field(&mut self, class: &[u8], name: &[u8], desc: &[u8]) -> u16 {
        let c = self.class(class);
        let nt = self.name_and_type(name, desc);
        self.ref2(9, c, nt)
    }

    pub fn method(&mut self, class: &[u8], name: &[u8], desc: &[u8], interface: bool) -> u16 {
        let c = self.class(class);
        let nt = self.name_and_type(name, desc);
        self.ref2(if interface { 11 } else { 10 }, c, nt)
    }

    pub fn method_handle(&mut self, kind: u8, member: u16) -> u16 {
        let mut e = vec![15, kind];
        e.extend_from_slice(&member.to_be_bytes());
        self.intern(e, 1)
    }

    pub fn method_type(&mut self, desc: &[u8]) -> u16 {
        let d = self.utf8(desc);
        self.ref1(16, d)
    }

    pub fn invoke_dynamic(&mut self, bootstrap: u16, name: &[u8], desc: &[u8]) -> u16 {
        let nt = self.name_and_type(name, desc);
        self.ref2(18, bootstrap, nt)
    }

    pub fn check(&self) -> Result<()> {
        if self.overflow {
            return Err(err("constant pool overflow (more than 65535 entries or a string over 64 KiB)".to_string()));
        }
        Ok(())
    }
}

/// An attribute: name index and body.
pub type Attr = (u16, Vec<u8>);

pub struct Member {
    pub access: u16,
    pub name: u16,
    pub desc: u16,
    pub attrs: Vec<Attr>,
}

pub struct ClassFile {
    pub version: u16,
    pub access: u16,
    pub this: u16,
    pub superclass: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Member>,
    pub methods: Vec<Member>,
    pub attrs: Vec<Attr>,
}

fn put_attrs(out: &mut Vec<u8>, attrs: &[Attr]) {
    out.extend_from_slice(&(attrs.len() as u16).to_be_bytes());
    for (name, body) in attrs {
        out.extend_from_slice(&name.to_be_bytes());
        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(body);
    }
}

impl ClassFile {
    pub fn write(&self, pool: &Pool) -> Result<Vec<u8>> {
        pool.check()?;
        let mut out = Vec::with_capacity(pool.bytes.len() + 256);
        out.extend_from_slice(&0xCAFE_BABEu32.to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes());
        out.extend_from_slice(&self.version.to_be_bytes());
        out.extend_from_slice(&(pool.next as u16).to_be_bytes());
        out.extend_from_slice(&pool.bytes);
        for v in [self.access, self.this, self.superclass] {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out.extend_from_slice(&(self.interfaces.len() as u16).to_be_bytes());
        for i in &self.interfaces {
            out.extend_from_slice(&i.to_be_bytes());
        }
        for members in [&self.fields, &self.methods] {
            out.extend_from_slice(&(members.len() as u16).to_be_bytes());
            for m in members.iter() {
                for v in [m.access, m.name, m.desc] {
                    out.extend_from_slice(&v.to_be_bytes());
                }
                put_attrs(&mut out, &m.attrs);
            }
        }
        put_attrs(&mut out, &self.attrs);
        Ok(out)
    }
}

/// Big-endian body builder for attributes.
#[derive(Default)]
pub struct Body(pub Vec<u8>);

impl Body {
    pub fn u8(&mut self, v: u8) -> &mut Body {
        self.0.push(v);
        self
    }

    pub fn u16(&mut self, v: u16) -> &mut Body {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }

    pub fn u32(&mut self, v: u32) -> &mut Body {
        self.0.extend_from_slice(&v.to_be_bytes());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(pub u32);

enum Fixup {
    Branch { at: usize, base: usize, label: Label },
    Wide { at: usize, base: usize, label: Label },
}

/// Bytecode under construction. Branches are 16-bit unless `wide`, where a
/// conditional becomes an inverted branch over a `goto_w`.
pub struct Asm {
    pub code: Vec<u8>,
    labels: Vec<Option<u32>>,
    fixups: Vec<Fixup>,
    pub wide: bool,
    pub depth: i32,
    pub max: i32,
}

pub mod op {
    pub const ACONST_NULL: u8 = 0x01;
    pub const ICONST_0: u8 = 0x03;
    pub const LCONST_0: u8 = 0x09;
    pub const FCONST_0: u8 = 0x0b;
    pub const DCONST_0: u8 = 0x0e;
    pub const BIPUSH: u8 = 0x10;
    pub const SIPUSH: u8 = 0x11;
    pub const LDC: u8 = 0x12;
    pub const LDC_W: u8 = 0x13;
    pub const LDC2_W: u8 = 0x14;
    pub const ILOAD: u8 = 0x15;
    pub const ISTORE: u8 = 0x36;
    pub const IALOAD: u8 = 0x2e;
    pub const IASTORE: u8 = 0x4f;
    pub const POP: u8 = 0x57;
    pub const POP2: u8 = 0x58;
    pub const DUP: u8 = 0x59;
    pub const IINC: u8 = 0x84;
    pub const IFEQ: u8 = 0x99;
    pub const IF_ICMPEQ: u8 = 0x9f;
    pub const IF_ACMPEQ: u8 = 0xa5;
    pub const GOTO: u8 = 0xa7;
    pub const TABLESWITCH: u8 = 0xaa;
    pub const LOOKUPSWITCH: u8 = 0xab;
    pub const IRETURN: u8 = 0xac;
    pub const RETURN: u8 = 0xb1;
    pub const GETSTATIC: u8 = 0xb2;
    pub const PUTSTATIC: u8 = 0xb3;
    pub const GETFIELD: u8 = 0xb4;
    pub const PUTFIELD: u8 = 0xb5;
    pub const INVOKEVIRTUAL: u8 = 0xb6;
    pub const INVOKESPECIAL: u8 = 0xb7;
    pub const INVOKESTATIC: u8 = 0xb8;
    pub const INVOKEINTERFACE: u8 = 0xb9;
    pub const INVOKEDYNAMIC: u8 = 0xba;
    pub const NEW: u8 = 0xbb;
    pub const NEWARRAY: u8 = 0xbc;
    pub const ANEWARRAY: u8 = 0xbd;
    pub const ARRAYLENGTH: u8 = 0xbe;
    pub const ATHROW: u8 = 0xbf;
    pub const CHECKCAST: u8 = 0xc0;
    pub const INSTANCEOF: u8 = 0xc1;
    pub const MONITORENTER: u8 = 0xc2;
    pub const MONITOREXIT: u8 = 0xc3;
    pub const WIDE: u8 = 0xc4;
    pub const IFNULL: u8 = 0xc6;
    pub const IFNONNULL: u8 = 0xc7;
    pub const GOTO_W: u8 = 0xc8;
}

impl Asm {
    pub fn new(wide: bool) -> Asm {
        Asm {
            code: Vec::new(),
            labels: Vec::new(),
            fixups: Vec::new(),
            wide,
            depth: 0,
            max: 0,
        }
    }

    pub fn pc(&self) -> u32 {
        self.code.len() as u32
    }

    pub fn label(&mut self) -> Label {
        self.labels.push(None);
        Label(self.labels.len() as u32 - 1)
    }

    pub fn bind(&mut self, l: Label) {
        self.labels[l.0 as usize] = Some(self.pc());
    }

    pub fn bound(&self, l: Label) -> Option<u32> {
        self.labels[l.0 as usize]
    }

    pub fn stack(&mut self, delta: i32) {
        self.depth += delta;
        self.max = self.max.max(self.depth);
    }

    /// Emits an opcode with its operand bytes and stack effect.
    pub fn op(&mut self, opcode: u8, operands: &[u8], delta: i32) {
        self.code.push(opcode);
        self.code.extend_from_slice(operands);
        self.stack(delta);
    }

    pub fn op_u16(&mut self, opcode: u8, v: u16, delta: i32) {
        self.op(opcode, &v.to_be_bytes(), delta);
    }

    /// A load or store of `slot` (`base` is the `iload`/`istore` family opcode).
    pub fn local(&mut self, base: u8, kind: u8, slot: u16, delta: i32) {
        let short = if base == op::ILOAD { 0x1a } else { 0x3b };
        if slot < 4 {
            self.op(short + kind * 4 + slot as u8, &[], delta);
        } else if slot < 256 {
            self.op(base + kind, &[slot as u8], delta);
        } else {
            self.code.push(op::WIDE);
            self.op_u16(base + kind, slot, delta);
        }
    }

    pub fn iinc(&mut self, slot: u16, by: i16) {
        if slot < 256 && (-128..128).contains(&by) {
            self.op(op::IINC, &[slot as u8, by as i8 as u8], 0);
        } else {
            self.code.push(op::WIDE);
            self.code.push(op::IINC);
            self.code.extend_from_slice(&slot.to_be_bytes());
            self.code.extend_from_slice(&by.to_be_bytes());
        }
    }

    fn inverse(opcode: u8) -> u8 {
        match opcode {
            op::IFNULL => op::IFNONNULL,
            op::IFNONNULL => op::IFNULL,
            o if (op::IFEQ..=0xa6).contains(&o) => ((o - op::IFEQ) ^ 1) + op::IFEQ,
            o => o,
        }
    }

    /// A conditional branch or `goto` to `label`, popping `pops` values.
    pub fn branch(&mut self, opcode: u8, label: Label, pops: i32) {
        let base = self.code.len();
        if !self.wide {
            self.code.push(opcode);
            self.fixups.push(Fixup::Branch {
                at: base + 1,
                base,
                label,
            });
            self.code.extend_from_slice(&[0, 0]);
        } else if opcode == op::GOTO {
            self.code.push(op::GOTO_W);
            self.fixups.push(Fixup::Wide {
                at: base + 1,
                base,
                label,
            });
            self.code.extend_from_slice(&[0, 0, 0, 0]);
        } else {
            self.code.push(Self::inverse(opcode));
            self.code.extend_from_slice(&8u16.to_be_bytes());
            let g = self.code.len();
            self.code.push(op::GOTO_W);
            self.fixups.push(Fixup::Wide {
                at: g + 1,
                base: g,
                label,
            });
            self.code.extend_from_slice(&[0, 0, 0, 0]);
        }
        self.stack(-pops);
    }

    fn pad(&mut self) -> usize {
        let base = self.code.len();
        while !self.code.len().is_multiple_of(4) {
            self.code.push(0);
        }
        base
    }

    fn wide_target(&mut self, base: usize, label: Label) {
        let at = self.code.len();
        self.fixups.push(Fixup::Wide { at, base, label });
        self.code.extend_from_slice(&[0, 0, 0, 0]);
    }

    pub fn tableswitch(&mut self, default: Label, low: i32, targets: &[Label]) {
        let base = self.code.len();
        self.code.push(op::TABLESWITCH);
        self.pad();
        self.wide_target(base, default);
        self.code.extend_from_slice(&low.to_be_bytes());
        let high = low.wrapping_add(targets.len() as i32 - 1);
        self.code.extend_from_slice(&high.to_be_bytes());
        for &t in targets {
            self.wide_target(base, t);
        }
        self.stack(-1);
    }

    pub fn lookupswitch(&mut self, default: Label, pairs: &[(i32, Label)]) {
        let base = self.code.len();
        self.code.push(op::LOOKUPSWITCH);
        self.pad();
        self.wide_target(base, default);
        self.code.extend_from_slice(&(pairs.len() as u32).to_be_bytes());
        for &(k, t) in pairs {
            self.code.extend_from_slice(&k.to_be_bytes());
            self.wide_target(base, t);
        }
        self.stack(-1);
    }

    /// Patches branch offsets; `Ok(false)` when a 16-bit offset overflows.
    pub fn resolve(&mut self) -> Result<bool> {
        for f in &self.fixups {
            let (at, base, label, wide) = match *f {
                Fixup::Branch { at, base, label } => (at, base, label, false),
                Fixup::Wide { at, base, label } => (at, base, label, true),
            };
            let target = self.labels[label.0 as usize]
                .ok_or_else(|| err("branch to an unplaced label".to_string()))?;
            let off = i64::from(target) - base as i64;
            if wide {
                self.code[at..at + 4].copy_from_slice(&(off as i32).to_be_bytes());
            } else if let Ok(o) = i16::try_from(off) {
                self.code[at..at + 2].copy_from_slice(&o.to_be_bytes());
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
