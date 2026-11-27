//! d8's desugared lambdas, undone: a `final class X$$ExternalSyntheticLambdaN`
//! implementing one interface, whose constructor stores its captures in
//! fields and whose single method loads them, adapts its parameters (casts,
//! unboxing), calls one implementation method and returns its (boxed)
//! result, is what javac's `invokedynamic` to `LambdaMetafactory.metafactory`
//! became. A `new` of such a class is written back as that `invokedynamic`,
//! so the engine rebuilds the lambda or method reference.

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

/// Every d8 lambda class of the file, by type index.
pub fn scan(dex: &Dex) -> BTreeMap<u32, Lambda> {
    let mut out = BTreeMap::new();
    for i in 0..dex.class_count() {
        let Ok(def) = dex.class_def(i) else { continue };
        let Ok(desc) = dex.type_desc(def.class) else { continue };
        const MARK: &[u8] = b"$$ExternalSyntheticLambda";
        if !desc.windows(MARK.len()).any(|w| w == MARK) {
            continue;
        }
        if let Some(l) = recognize(dex, def) {
            out.insert(def.class, l);
        }
    }
    out
}

fn recognize(dex: &Dex, def: crate::dex::ClassDef) -> Option<Lambda> {
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
        reg += if matches!(dex.type_desc(t).ok()?.first(), Some(b'J' | b'D')) { 2 } else { 1 };
    }
    let mut cast: BTreeMap<usize, u32> = BTreeMap::new();
    let mut found: Option<(u8, u32, Vec<Origin>)> = None;
    let mut new_type = NO_INDEX;
    let mut k = 0;
    while k < insns.len() {
        let x = &insns[k];
        match x.op {
            0x52..=0x58 if x.b as usize == first => {
                let i = fields.iter().position(|&f| f == x.idx)?;
                origin[x.a as usize] = Origin::Capture(i);
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
                let args: Vec<Origin> = x.args().iter().map(|&r| origin[r as usize]).collect();
                let unbox = x.op == 0x6e && UNBOX.contains(&name) && args.len() == 1;
                let valueof = x.op == 0x71 && name == b"valueOf" && found.is_some();
                if unbox {
                    if let Some(Origin::Param(j)) = args.first().copied() {
                        let next = insns.get(k + 1).filter(|n| matches!(n.op, 0x0a..=0x0c) && (n.a as usize) < regs)?;
                        origin[next.a as usize] = Origin::Param(j);
                        cast.insert(j, mid.class);
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
            0x0a..=0x0c | 0x0e..=0x11 | 0x00 => {}
            0x01..=0x09 => origin[x.a as usize] = origin[x.b as usize],
            _ => return None,
        }
        k += 1;
    }
    let (kind, method, args) = found?;
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
    Some(Lambda {
        iface: ifaces[0],
        sam_name: sam_id.name,
        sam_proto: sam_id.proto,
        kind,
        method,
        captures,
        instantiated: inst,
    })
}
