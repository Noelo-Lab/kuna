//! d8's desugared lambdas, undone: a `final class X$$ExternalSyntheticLambdaN`
//! implementing one interface, whose constructor stores its captures in
//! fields and whose single method loads them, adapts its parameters (casts,
//! unboxing, primitive widening), calls one implementation method and returns its (boxed)
//! result, is what javac's `invokedynamic` to `LambdaMetafactory.metafactory`
//! became (an unboxing call alone is the method reference `Long::longValue`).
//! A `new` of such a class is written back as that `invokedynamic`, so the
//! engine rebuilds the lambda or method reference.

use std::collections::BTreeMap;

use crate::dex::{Dex, NO_INDEX};
use crate::insn::decode;

/// What the `invokedynamic` needs: the interface method, the implementation
/// handle, the instantiated method type and the captured values' types.
#[derive(Clone, Debug)]
pub struct Lambda {
    pub iface: u32,
    pub sam_name: u32,
    pub sam_proto: u32,
    /// JVM reference kind (5 virtual, 6 static, 7 special, 8 new, 9 interface).
    pub kind: u8,
    pub method: u32,
    pub captures: Vec<u32>,
    pub instantiated: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq)]
enum Origin {
    Capture(usize),
    Param(usize),
    /// The second register of a wide value.
    High,
    Other,
}

const UNBOX: [&[u8]; 8] = [
    b"booleanValue",
    b"byteValue",
    b"charValue",
    b"shortValue",
    b"intValue",
    b"longValue",
    b"floatValue",
    b"doubleValue",
];

fn boxed(desc: &[u8]) -> &[u8] {
    match desc {
        b"Z" => b"Ljava/lang/Boolean;",
        b"B" => b"Ljava/lang/Byte;",
        b"C" => b"Ljava/lang/Character;",
        b"S" => b"Ljava/lang/Short;",
        b"I" => b"Ljava/lang/Integer;",
        b"J" => b"Ljava/lang/Long;",
        b"F" => b"Ljava/lang/Float;",
        b"D" => b"Ljava/lang/Double;",
        d => d,
    }
}

/// The d8 lambda classes of a file by type index, and the d8 bridges their
/// handles skip.
#[derive(Debug, Default)]
pub struct Lambdas {
    pub by_type: BTreeMap<u32, Lambda>,
    pub bridges: std::collections::BTreeSet<u32>,
}

pub fn scan(dex: &Dex) -> Lambdas {
    let mut out = Lambdas::default();
    for i in 0..dex.class_count() {
        let Ok(def) = dex.class_def(i) else { continue };
        let Ok(desc) = dex.type_desc(def.class) else { continue };
        const MARK: &[u8] = b"$$ExternalSyntheticLambda";
        if !desc.windows(MARK.len()).any(|w| w == MARK) {
            continue;
        }
        if let Some((l, bridge)) = recognize(dex, def) {
            out.bridges.extend(bridge);
            out.by_type.insert(def.class, l);
        }
    }
    out
}

fn recognize(dex: &Dex, def: crate::dex::ClassDef) -> Option<(Lambda, Option<u32>)> {
    let ifaces = dex.type_list(def.interfaces_off).ok()?;
    let object = dex.type_desc(def.superclass).ok()? == b"Ljava/lang/Object;";
    if ifaces.len() != 1 || !object {
        return None;
    }
    let data = dex.class_data(def.class_data_off).ok()?;
    let init = data
        .direct_methods
        .iter()
        .find(|m| dex.method(m.method).and_then(|x| dex.string(x.name)).ok() == Some(b"<init>"))?;
    if data.direct_methods.len() != 1 || data.virtual_methods.len() != 1 || !data.static_fields.is_empty() {
        return None;
    }
    let fields: Vec<u32> = data.instance_fields.iter().map(|f| f.field).collect();
    let captures = dex.proto(dex.method(init.method).ok()?.proto).ok()?.params;
    if captures.len() != fields.len() {
        return None;
    }
    let sam = &data.virtual_methods[0];
    let sam_id = dex.method(sam.method).ok()?;
    let sam_params = dex.proto(sam_id.proto).ok()?;
    let code = dex.code_item(sam.code_off).ok()?;
    let insns = decode(code.insns).ok()?;
    let regs = usize::from(code.registers);
    let first = regs.checked_sub(usize::from(code.ins))?;
    if insns.iter().any(|x| [x.a, x.b, x.c].iter().chain(x.args().iter()).any(|&r| r as usize >= regs)) {
        return None;
    }
    let mut origin = vec![Origin::Other; regs];
    let mut reg = first + 1;
    for (j, &t) in sam_params.params.iter().enumerate() {
        *origin.get_mut(reg)? = Origin::Param(j);
        if matches!(dex.type_desc(t).ok()?.first(), Some(b'J' | b'D')) {
            *origin.get_mut(reg + 1)? = Origin::High;
            reg += 1;
        }
        reg += 1;
    }
    let mut cast: BTreeMap<usize, u32> = BTreeMap::new();
    let mut found: Option<(u8, u32, Vec<Origin>)> = None;
    let mut new_type = NO_INDEX;
    let mut unboxed: Option<(u32, usize)> = None;
    let mut widened = std::collections::BTreeSet::new();
    let mut k = 0;
    while k < insns.len() {
        let x = &insns[k];
        match x.op {
            0x52..=0x58 if x.b as usize == first => {
                let i = fields.iter().position(|&f| f == x.idx)?;
                origin[x.a as usize] = Origin::Capture(i);
                if x.op == 0x53 {
                    *origin.get_mut(x.a as usize + 1)? = Origin::High;
                }
            }
            0x1f => {
                if let Origin::Param(j) = origin[x.a as usize] {
                    cast.insert(j, x.idx);
                }
            }
            0x22 if found.is_none() => new_type = x.idx,
            0x6e..=0x72 | 0x74..=0x78 => {
                let mid = dex.method(x.idx).ok()?;
                let name = dex.string(mid.name).ok()?;
                let args: Vec<Origin> = x.args().iter().map(|&r| origin[r as usize]).filter(|&o| o != Origin::High).collect();
                let unbox = x.op == 0x6e && UNBOX.contains(&name) && args.len() == 1;
                let valueof = x.op == 0x71 && name == b"valueOf" && found.is_some();
                if unbox {
                    if let Some(Origin::Param(j)) = args.first().copied() {
                        let next = insns.get(k + 1).filter(|n| matches!(n.op, 0x0a..=0x0c) && (n.a as usize) < regs)?;
                        origin[next.a as usize] = Origin::Param(j);
                        if next.op == 0x0b {
                            *origin.get_mut(next.a as usize + 1)? = Origin::High;
                        }
                        cast.insert(j, mid.class);
                        unboxed = Some((x.idx, j));
                        k += 2;
                        continue;
                    }
                }
                if valueof {
                    k += 1;
                    continue;
                }
                if found.is_some() {
                    return None;
                }
                let base = if x.op >= 0x74 { x.op - 0x74 } else { x.op - 0x6e };
                let is_init = base == 2 && name == b"<init>";
                let kind = match base {
                    0 => 5,
                    3 => 6,
                    4 => 9,
                    _ if is_init && new_type == mid.class => 8,
                    _ => 7,
                };
                let args = if kind == 8 { args[1..].to_vec() } else { args };
                found = Some((kind, x.idx, args));
            }
            0x81..=0x83 | 0x85 | 0x86 | 0x89 => {
                if let Origin::Param(j) = origin[x.b as usize] {
                    widened.insert(j);
                }
                origin[x.a as usize] = origin[x.b as usize];
                if matches!(x.op, 0x81 | 0x83 | 0x86 | 0x89) {
                    *origin.get_mut(x.a as usize + 1)? = Origin::High;
                }
            }
            0x0a..=0x0c | 0x0e..=0x11 | 0x00 => {}
            0x01..=0x09 => origin[x.a as usize] = origin[x.b as usize],
            _ => return None,
        }
        k += 1;
    }
    let (kind, method, args) = match found {
        Some(f) => f,
        None => {
            let (idx, j) = unboxed?;
            (5, idx, vec![Origin::Param(j)])
        }
    };
    let body = (kind == 6).then(|| unbridge(dex, method)).flatten();
    let bridge = body.map(|_| method);
    let (kind, method) = body.unwrap_or((kind, method));
    let want: Vec<Origin> = (0..captures.len())
        .map(Origin::Capture)
        .chain((0..sam_params.params.len()).map(Origin::Param))
        .collect();
    if args != want {
        return None;
    }
    let mid = dex.method(method).ok()?;
    let impl_proto = dex.proto(mid.proto).ok()?;
    let mut impl_params: Vec<Vec<u8>> = Vec::new();
    if matches!(kind, 5 | 7 | 9) {
        impl_params.push(dex.type_desc(mid.class).ok()?.to_vec());
    }
    for &t in &impl_proto.params {
        impl_params.push(dex.type_desc(t).ok()?.to_vec());
    }
    let skip = captures.len();
    if impl_params.len() != skip + sam_params.params.len() {
        return None;
    }
    let mut inst = vec![b'('];
    for (j, p) in impl_params[skip..].iter().enumerate() {
        let sam_desc = dex.type_desc(sam_params.params[j]).ok()?;
        let p: &[u8] = if sam_desc.first() == Some(&b'L') && p.len() == 1 { boxed(p) } else { p };
        let p = match cast.get(&j) {
            Some(&t) if sam_desc.first() == Some(&b'L') => dex.type_desc(t).ok()?,
            _ if widened.contains(&j) => sam_desc,
            _ => p,
        };
        inst.extend_from_slice(p);
    }
    inst.push(b')');
    let sam_ret = dex.type_desc(sam_params.ret).ok()?;
    let ret: &[u8] = if kind == 8 {
        dex.type_desc(mid.class).ok()?
    } else {
        dex.type_desc(impl_proto.ret).ok()?
    };
    let ret = match (sam_ret, ret) {
        (b"V", _) => b"V" as &[u8],
        (s, r) if s.first() == Some(&b'L') && r.len() == 1 => boxed(r),
        (_, r) => r,
    };
    inst.extend_from_slice(ret);
    Some((
        Lambda {
            iface: ifaces[0],
            sam_name: sam_id.name,
            sam_proto: sam_id.proto,
            kind,
            method,
            captures,
            instantiated: inst,
        },
        bridge,
    ))
}

/// The lambda body behind a d8 bridge: d8 reaches a private instance method
/// (a lambda body or a method reference's target) through a public static
/// synthetic method of the same class that passes its parameters on
/// unchanged (`$r8$lambda$...`), where javac's handle names the method itself
/// (javac's own lambda bodies, static ones included, are never public), and
/// likewise through a nest accessor.
fn unbridge(dex: &Dex, idx: u32) -> Option<(u8, u32)> {
    let m = dex.method(idx).ok()?;
    let def = dex.class_def(dex.class_of_type(m.class)?).ok()?;
    let data = dex.class_data(def.class_data_off).ok()?;
    let em = data.direct_methods.iter().find(|e| e.method == idx)?;
    let accessor = matches!(dex.nest().accessors.get(&idx), Some(crate::nest::Access::Call { .. }));
    if em.access & 0x1009 != 0x1009 && !accessor {
        return None;
    }
    let code = dex.code_item(em.code_off).ok()?;
    let insns = decode(code.insns).ok()?;
    let first = u32::from(code.registers.checked_sub(code.ins)?);
    let call = insns.first()?;
    let regs: Vec<u32> = (first..u32::from(code.registers)).collect();
    if !matches!(call.op, 0x6e | 0x70 | 0x72 | 0x74 | 0x76 | 0x78) || call.args() != regs {
        return None;
    }
    let returns = match &insns[1..] {
        [r] => r.op == 0x0e,
        [mr, r] => matches!(mr.op, 0x0a..=0x0c) && matches!(r.op, 0x0f..=0x11) && mr.a == r.a,
        _ => false,
    };
    let target = dex.method(call.idx).ok()?;
    if !returns || target.class != m.class || dex.string(target.name).ok()? == b"<init>" {
        return None;
    }
    let kind = if def.access & 0x200 != 0 { 9 } else { 5 };
    Some((kind, call.idx))
}
