//! d8's nest-access desugaring, undone. javac 11 and later read a private
//! member of a nestmate directly; d8 reaches a private field or method
//! through a static synthetic accessor of the member's class
//! (`-$$Nest$fget...`, `-$$Nest$m...`) whose body is that one access on its
//! parameters, and a private constructor through a synthetic one that takes
//! an extra argument of an empty class `X-IA` (always `null`) and calls it.
//! A call of an accessor is written as the access itself and a call of such a
//! constructor as the call of the private one (the accessors themselves stay,
//! synthetic, for calls from other files), and the empty `-IA` classes are
//! left out. The classes get `NestHost` and `NestMembers` so the accesses are
//! legal again.

use std::collections::{BTreeMap, BTreeSet};

use crate::dex::Dex;
use crate::insn::{decode, Insn};

const ACC_STATIC: u32 = 0x8;
const ACC_SYNTHETIC: u32 = 0x1000;

/// What a call of an accessor becomes.
#[derive(Clone, Debug)]
pub enum Access {
    /// The accessor's single field instruction, its registers taken from
    /// the call: `value` is the call's argument the instruction reads or
    /// writes as its `vA` (`None`: the call's result), `object` the one it
    /// reads as `vB`.
    Field { insn: Insn, value: Option<usize>, object: Option<usize> },
    /// The accessor's call, with the caller's arguments.
    Call { op: u8, method: u32 },
}

#[derive(Debug, Default)]
pub struct Nest {
    pub accessors: BTreeMap<u32, Access>,
    /// Bridging constructor to the private one it calls.
    pub ctors: BTreeMap<u32, u32>,
    /// The empty `-IA` class definitions.
    pub classes: BTreeSet<u32>,
}

fn accessor(dex: &Dex, class: u32, proto: u32, code: &crate::dex::CodeItem) -> Option<Access> {
    let insns = decode(code.insns).ok()?;
    let first = u32::from(code.registers.checked_sub(code.ins)?);
    let params: Vec<u32> = (first..u32::from(code.registers)).collect();
    let arg = |r: u32| params.iter().position(|&p| p == r);
    let x = insns.first()?;
    let rest: Vec<&Insn> = insns[1..].iter().collect();
    match x.op {
        0x52..=0x66 => {
            let f = dex.field(x.idx).ok()?;
            if f.class != class {
                return None;
            }
            let ok = matches!(&rest[..], [r] if matches!(r.op, 0x0f..=0x11) && r.a == x.a);
            let object = if x.op <= 0x58 { Some(arg(x.b)?) } else { None };
            ok.then_some(Access::Field { insn: x.clone(), value: None, object })
        }
        0x59..=0x6d => {
            let f = dex.field(x.idx).ok()?;
            if f.class != class || !matches!(&rest[..], [r] if r.op == 0x0e) {
                return None;
            }
            let object = if x.op <= 0x5f { Some(arg(x.b)?) } else { None };
            Some(Access::Field { insn: x.clone(), value: Some(arg(x.a)?), object })
        }
        0x6e..=0x72 | 0x74..=0x78 => {
            let m = dex.method(x.idx).ok()?;
            let returns = match &rest[..] {
                [r] => r.op == 0x0e,
                [mr, r] => matches!(mr.op, 0x0a..=0x0c) && matches!(r.op, 0x0f..=0x11) && mr.a == r.a,
                _ => false,
            };
            let same = dex.proto(m.proto).ok()?.ret == dex.proto(proto).ok()?.ret;
            let name = dex.string(m.name).ok()?;
            let ok = returns && same && m.class == class && x.args() == params && name != b"<init>";
            ok.then_some(Access::Call { op: x.op, method: x.idx })
        }
        _ => None,
    }
}

pub fn scan(dex: &Dex) -> Nest {
    let mut out = Nest::default();
    for i in 0..dex.class_count() {
        let Ok(def) = dex.class_def(i) else { continue };
        let Ok(data) = dex.class_data(def.class_data_off) else { continue };
        let empty = data.static_fields.is_empty() && data.instance_fields.is_empty() && data.virtual_methods.is_empty();
        let marker = def.access & ACC_SYNTHETIC != 0 && dex.type_desc(def.class).is_ok_and(|d| d.ends_with(b"-IA;"));
        if marker && empty && data.direct_methods.is_empty() {
            out.classes.insert(i);
        }
        for em in &data.direct_methods {
            if em.code_off == 0 || em.access & ACC_SYNTHETIC == 0 {
                continue;
            }
            let (Ok(mi), Ok(code)) = (dex.method(em.method), dex.code_item(em.code_off)) else { continue };
            if em.access & ACC_STATIC != 0 {
                if let Some(a) = accessor(dex, def.class, mi.proto, &code) {
                    out.accessors.insert(em.method, a);
                }
                continue;
            }
            if let Some(target) = bridge_ctor(dex, def.class, mi, &code) {
                out.ctors.insert(em.method, target);
            }
        }
    }
    out
}

/// The private constructor a d8 constructor taking a trailing `-IA`
/// argument passes its other parameters on to.
fn bridge_ctor(dex: &Dex, class: u32, mi: crate::dex::MethodId, code: &crate::dex::CodeItem) -> Option<u32> {
    let p = dex.proto(mi.proto).ok()?;
    let last = *p.params.last()?;
    if dex.string(mi.name).ok()? != b"<init>" || !dex.type_desc(last).ok()?.ends_with(b"-IA;") {
        return None;
    }
    let insns = decode(code.insns).ok()?;
    let first = u32::from(code.registers.checked_sub(code.ins)?);
    let params: Vec<u32> = (first..u32::from(code.registers) - 1).collect();
    match &insns[..] {
        [x, r] if matches!(x.op, 0x70 | 0x76) && r.op == 0x0e && x.args() == params => {
            let m = dex.method(x.idx).ok()?;
            (m.class == class && dex.string(m.name).ok()? == b"<init>").then_some(x.idx)
        }
        _ => None,
    }
}

/// `insns` with the calls of accessors and bridging constructors written as
/// the accesses themselves (a field read takes over the `move-result`, which
/// becomes a `nop`).
pub fn rewrite(dex: &Dex, insns: &mut [Insn]) {
    let nest = dex.nest();
    for k in 0..insns.len() {
        let x = &insns[k];
        if matches!(x.op, 0x70 | 0x76) {
            if let Some(&target) = nest.ctors.get(&x.idx) {
                let x = &mut insns[k];
                x.idx = target;
                x.count = x.count.saturating_sub(1);
            }
            continue;
        }
        if !matches!(x.op, 0x71 | 0x77) {
            continue;
        }
        let Some(a) = nest.accessors.get(&x.idx) else { continue };
        let args = x.args();
        match a {
            Access::Call { op, method } => {
                let range = x.op == 0x77;
                let base = if *op >= 0x74 { op - 0x74 } else { op - 0x6e };
                let x = &mut insns[k];
                x.op = if range { 0x74 + base } else { 0x6e + base };
                x.idx = *method;
            }
            Access::Field { insn, value, object } => {
                let mut f = insn.clone();
                f.addr = x.addr;
                f.size = x.size;
                if let Some(o) = object {
                    let Some(&r) = args.get(*o) else { continue };
                    f.b = r;
                }
                match value {
                    Some(v) => {
                        let Some(&r) = args.get(*v) else { continue };
                        f.a = r;
                    }
                    None => {
                        let Some(next) = insns.get(k + 1).filter(|n| matches!(n.op, 0x0a..=0x0c)) else { continue };
                        f.a = next.a;
                        insns[k + 1].op = 0x00;
                    }
                }
                insns[k] = f;
            }
        }
    }
}
