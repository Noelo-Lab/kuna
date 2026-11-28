//! d8's API backports, undone. Below the API level that added a JDK method,
//! d8 calls a static method of a synthetic class
//! (`X$$ExternalSyntheticBackportN.m`) holding its own implementation
//! instead. Such a method is recognised by its body: a hash of its
//! descriptor, its instructions and the JDK members, strings and types they
//! name (any other class counts as one name), looked up among the bodies d8
//! writes for each backported method (`backports.tsv`, recorded by
//! `fixtures/dex/backports/regen.sh`). A call to a recognised method is
//! written as the call of the JDK method, and a synthetic class holding only
//! such methods is left out.

use std::collections::BTreeMap;

use crate::dex::{CodeItem, Dex};
use crate::insn::decode;
use crate::Result;

const ACC_STATIC: u32 = 0x8;
const ACC_SYNTHETIC: u32 = 0x1000;

/// How the JDK method is called.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    Static,
    /// A static method of an interface.
    InterfaceStatic,
    Virtual,
    Interface,
}

/// The JDK method a backport stands for.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub call: Call,
    pub owner: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
}

/// The table: `hash, kind, owner#name(desc)` per line.
const TABLE: &str = include_str!("backports.tsv");

fn lookup(hash: u64) -> Option<Target> {
    TABLE.lines().find_map(|line| {
        let mut f = line.split('\t');
        let h = u64::from_str_radix(f.next()?, 16).ok()?;
        if h != hash {
            return None;
        }
        let call = match f.next()? {
            "static" => Call::Static,
            "istatic" => Call::InterfaceStatic,
            "virtual" => Call::Virtual,
            _ => Call::Interface,
        };
        let m = f.next()?;
        let (owner, rest) = m.split_once('#')?;
        let at = rest.find('(')?;
        Some(Target {
            call,
            owner,
            name: &rest[..at],
            desc: &rest[at..],
        })
    })
}

struct Fnv(u64);

impl Fnv {
    fn bytes(&mut self, b: &[u8]) {
        for &x in b.iter().chain([0xffu8].iter()) {
            self.0 ^= u64::from(x);
            self.0 = self.0.wrapping_mul(0x100_0000_01b3);
        }
    }
}

/// A type as the hash names it: JDK and primitive types by descriptor,
/// every other class as `~`.
fn class_name(desc: &[u8]) -> &[u8] {
    let elem = desc.iter().position(|&c| c != b'[').map_or(desc, |i| &desc[i..]);
    if elem.first() != Some(&b'L') || elem.starts_with(b"Ljava/") {
        desc
    } else {
        b"~"
    }
}

impl Dex<'_> {
    fn proto_text(&self, proto: u32, h: &mut Fnv) -> Result<()> {
        let p = self.proto(proto)?;
        for t in p.params {
            h.bytes(class_name(self.type_desc(t)?));
        }
        h.bytes(class_name(self.type_desc(p.ret)?));
        Ok(())
    }

    /// The hashes backports are recognised by, of a method with prototype
    /// `proto` and code `code` (public for the table's generator): with the
    /// methods it calls in other classes of this file standing for their own
    /// shapes, and with them named only (they may be in another file).
    pub fn body_shapes(&self, proto: u32, code: &CodeItem) -> Result<[u64; 2]> {
        Ok([self.shape_at(proto, code, 0)?, self.shape_at(proto, code, u32::MAX)?])
    }

    /// The shape of the code of method `idx` when this file defines it with
    /// code, which stands for the method in a caller's shape (d8 nests
    /// backports in each other).
    fn callee_shape(&self, idx: u32, depth: u32) -> Option<u64> {
        let m = self.method(idx).ok()?;
        let def = self.class_def(self.class_of_type(m.class)?).ok()?;
        let data = self.class_data(def.class_data_off).ok()?;
        let em = data.direct_methods.iter().find(|e| e.method == idx && e.code_off != 0)?;
        let code = self.code_item(em.code_off).ok()?;
        self.shape_at(m.proto, &code, depth + 1).ok()
    }

    fn shape_at(&self, proto: u32, code: &CodeItem, depth: u32) -> Result<u64> {
        let mut h = Fnv(0xcbf2_9ce4_8422_2325);
        self.proto_text(proto, &mut h)?;
        for x in decode(code.insns)? {
            h.bytes(&[x.op]);
            match x.op {
                0x12..=0x19 | 0x26..=0x3d | 0xd0..=0xe2 => h.bytes(&x.lit.to_le_bytes()),
                0x1a | 0x1b => h.bytes(self.string(x.idx)?),
                0x1c | 0x1f | 0x20 | 0x22..=0x25 => h.bytes(class_name(self.type_desc(x.idx)?)),
                0x52..=0x6d => {
                    let f = self.field(x.idx)?;
                    h.bytes(class_name(self.type_desc(f.class)?));
                    h.bytes(self.string(f.name)?);
                    h.bytes(class_name(self.type_desc(f.ty)?));
                }
                0x6e..=0x72 | 0x74..=0x78 => {
                    let m = self.method(x.idx)?;
                    let class = class_name(self.type_desc(m.class)?);
                    let inner = (class == b"~" && depth < 4).then(|| self.callee_shape(x.idx, depth)).flatten();
                    match inner {
                        Some(inner) => h.bytes(&inner.to_le_bytes()),
                        None => {
                            h.bytes(class);
                            h.bytes(self.string(m.name)?);
                            self.proto_text(m.proto, &mut h)?;
                        }
                    }
                }
                _ => {}
            }
        }
        for t in &code.tries {
            h.bytes(&t.count.to_le_bytes());
            for &(ty, _) in &t.handlers {
                match ty {
                    Some(ty) => h.bytes(class_name(self.type_desc(ty)?)),
                    None => h.bytes(b"*"),
                }
            }
        }
        Ok(h.0)
    }
}

/// The recognised backports of a file: method index to the JDK method, and
/// the class definitions holding nothing else.
#[derive(Debug, Default)]
pub struct Backports {
    pub methods: BTreeMap<u32, Target>,
    /// The same methods by [`Dex::method_key`], for calls from other files.
    pub by_name: BTreeMap<Vec<u8>, Target>,
    pub classes: std::collections::BTreeSet<u32>,
}

pub fn scan(dex: &Dex) -> Backports {
    let mut out = Backports::default();
    for i in 0..dex.class_count() {
        let Ok(def) = dex.class_def(i) else { continue };
        if def.access & ACC_SYNTHETIC == 0 || !dex.type_desc(def.superclass).is_ok_and(|d| d == b"Ljava/lang/Object;") {
            continue;
        }
        let Ok(data) = dex.class_data(def.class_data_off) else { continue };
        let mut all = !data.direct_methods.is_empty() && data.virtual_methods.is_empty();
        all &= data.static_fields.is_empty() && data.instance_fields.is_empty();
        for em in &data.direct_methods {
            let found = (em.access & ACC_STATIC != 0 && em.code_off != 0)
                .then(|| {
                    let mi = dex.method(em.method).ok()?;
                    let code = dex.code_item(em.code_off).ok()?;
                    let [deep, shallow] = dex.body_shapes(mi.proto, &code).ok()?;
                    lookup(deep).or_else(|| lookup(shallow))
                })
                .flatten();
            match found {
                Some(t) => {
                    out.methods.insert(em.method, t);
                    if let Ok(key) = dex.method_key(em.method) {
                        out.by_name.insert(key, t);
                    }
                }
                None => all = false,
            }
        }
        if all {
            out.classes.insert(i);
        }
    }
    out
}
