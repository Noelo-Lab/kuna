//! d8's record desugaring, undone. Below API 34 d8 gives a record the
//! superclass `com.android.tools.r8.RecordTag`, leaves out the `Record`
//! attribute, and writes `toString`, `hashCode` and `equals` as calls into
//! helpers of its own (a synthetic `$$ExternalSyntheticRecordN` class and the
//! private `$record$getFieldsAsObjects` and `$record$equals`) where javac
//! writes an `invokedynamic` to `ObjectMethods.bootstrap`. A class of that
//! shape gets `java.lang.Record` back as its superclass, a `Record`
//! attribute, and javac's three methods; the helpers are left out. The
//! component order is the order of the names d8 passes to its `toString`
//! helper or, when the record writes its own `toString`, the order in which
//! the canonical constructor's parameters reach the fields.

use std::collections::{BTreeMap, BTreeSet};

use crate::dex::Dex;
use crate::insn::{decode, Insn};

pub const RECORD_TAG: &[u8] = b"com/android/tools/r8/RecordTag";

/// A method d8 generated, written back as javac's `invokedynamic`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generated {
    ToString,
    HashCode,
    Equals,
}

#[derive(Clone, Debug, Default)]
pub struct Record {
    /// The component fields, in component order.
    pub components: Vec<u32>,
    pub generated: BTreeMap<u32, Generated>,
    /// d8's private helper methods, left out.
    pub hidden: BTreeSet<u32>,
}

#[derive(Debug, Default)]
pub struct Records {
    pub by_type: BTreeMap<u32, Record>,
    /// Class definitions of helper classes only the rewritten methods call.
    pub omit: BTreeSet<u32>,
}

fn ops(insns: &[Insn]) -> Vec<u8> {
    insns.iter().map(|x| x.op).collect()
}

fn proto_is(dex: &Dex, proto: u32, want: &[u8]) -> bool {
    let Ok(p) = dex.proto(proto) else { return false };
    let mut d = vec![b'('];
    for t in p.params {
        match dex.type_desc(t) {
            Ok(t) => d.extend_from_slice(t),
            Err(_) => return false,
        }
    }
    d.push(b')');
    dex.type_desc(p.ret).is_ok_and(|r| {
        d.extend_from_slice(r);
        d == want
    })
}

/// What one virtual method is, if d8 generated it: the kind, the helper
/// class it calls, the private helper method, and `toString`'s names.
type Found = (Generated, Option<u32>, Option<u32>, Option<u32>);

fn generated(dex: &Dex, class: u32, fields: &[u32], name: &[u8], proto: u32, this: u32, insns: &[Insn]) -> Option<Found> {
    let own = |x: &Insn, want: &[u8]| {
        dex.method(x.idx).is_ok_and(|m| m.class == class && proto_is(dex, m.proto, want))
    };
    let static_call = |x: &Insn, want: &[u8]| {
        matches!(x.op, 0x71 | 0x77) && dex.method(x.idx).is_ok_and(|m| m.class != class && proto_is(dex, m.proto, want))
    };
    match (name, ops(insns).as_slice()) {
        (b"toString", [0x70, 0x0c, 0x1c, 0x1a | 0x1b, 0x71 | 0x77, 0x0c, 0x11]) => {
            let ok = proto_is(dex, proto, b"()Ljava/lang/String;")
                && own(&insns[0], b"()[Ljava/lang/Object;")
                && insns[0].args() == [this]
                && insns[2].idx == class
                && static_call(&insns[4], b"([Ljava/lang/Object;Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/String;");
            let helper = dex.method(insns[4].idx).ok()?.class;
            ok.then_some((Generated::ToString, Some(helper), Some(insns[0].idx), Some(insns[3].idx)))
        }
        (b"hashCode", [0x12, 0x0f]) if fields.is_empty() && insns[0].lit == 0 => {
            proto_is(dex, proto, b"()I").then_some((Generated::HashCode, None, None, None))
        }
        (b"hashCode", [.., 0x71 | 0x77, 0x0a, 0x0f]) => {
            let n = insns.len() - 3;
            let reads: BTreeSet<u32> = insns[..n]
                .iter()
                .filter(|x| matches!(x.op, 0x52..=0x58) && x.b == this)
                .map(|x| x.idx)
                .collect();
            let want: BTreeSet<u32> = fields.iter().copied().collect();
            let ok = proto_is(dex, proto, b"()I")
                && reads == want
                && insns[..n].iter().all(|x| matches!(x.op, 0x52..=0x58))
                && matches!(insns[n].op, 0x71 | 0x77)
                && dex.method(insns[n].idx).is_ok_and(|m| m.class != class);
            let helper = dex.method(insns[n].idx).ok()?.class;
            ok.then_some((Generated::HashCode, Some(helper), None, None))
        }
        (b"equals", [0x70, 0x0a, 0x0f]) => {
            let ok = proto_is(dex, proto, b"(Ljava/lang/Object;)Z")
                && own(&insns[0], b"(Ljava/lang/Object;)Z")
                && insns[0].args() == [this, this + 1];
            ok.then_some((Generated::Equals, None, Some(insns[0].idx), None))
        }
        _ => None,
    }
}

/// The component order from a canonical constructor: an `<init>` with one
/// parameter per field whose parameters are stored into distinct fields,
/// or match one field each by type.
fn canonical_order(dex: &Dex, data: &crate::dex::ClassData, fields: &[u32]) -> Option<Vec<u32>> {
    let field_ty: Vec<u32> = fields.iter().map(|&f| dex.field(f).map(|f| f.ty)).collect::<Result<_, _>>().ok()?;
    for em in &data.direct_methods {
        let mi = dex.method(em.method).ok()?;
        if dex.string(mi.name).ok()? != b"<init>" || em.code_off == 0 {
            continue;
        }
        let params = dex.proto(mi.proto).ok()?.params;
        let mut want = field_ty.clone();
        want.sort_unstable();
        let mut have = params.clone();
        have.sort_unstable();
        if have != want {
            continue;
        }
        let code = dex.code_item(em.code_off).ok()?;
        let mut reg = u32::from(code.registers) - u32::from(code.ins) + 1;
        let mut regs = Vec::new();
        for &t in &params {
            regs.push(reg);
            reg += if matches!(dex.type_desc(t).ok()?.first(), Some(b'J' | b'D')) { 2 } else { 1 };
        }
        let insns = decode(code.insns).ok()?;
        let mut order: Vec<Option<u32>> = vec![None; params.len()];
        for x in insns.iter().filter(|x| matches!(x.op, 0x59..=0x5f) && x.b == regs[0] - 1) {
            if let (Some(i), Some(_)) = (regs.iter().position(|&r| r == x.a), fields.iter().position(|&f| f == x.idx)) {
                order[i] = Some(x.idx);
            }
        }
        for (i, &t) in params.iter().enumerate() {
            let open: Vec<u32> = fields
                .iter()
                .zip(&field_ty)
                .filter(|&(f, &ft)| ft == t && !order.contains(&Some(*f)))
                .map(|(&f, _)| f)
                .collect();
            if order[i].is_none() && open.len() == 1 {
                order[i] = Some(open[0]);
            }
        }
        let order: Vec<u32> = order.into_iter().collect::<Option<_>>()?;
        if order.iter().collect::<BTreeSet<_>>().len() == fields.len() {
            return Some(order);
        }
    }
    None
}

fn record(dex: &Dex, def: &crate::dex::ClassDef, used: &mut BTreeSet<u32>, kept: &mut BTreeSet<u32>) -> Option<Record> {
    let data = dex.class_data(def.class_data_off).ok()?;
    let fields: Vec<u32> = data.instance_fields.iter().map(|f| f.field).collect();
    let mut rec = Record::default();
    let mut names = None;
    let mut helpers = BTreeSet::new();
    let mut others = BTreeSet::new();
    for em in data.direct_methods.iter().chain(&data.virtual_methods) {
        if em.code_off == 0 {
            continue;
        }
        let mi = dex.method(em.method).ok()?;
        let code = dex.code_item(em.code_off).ok()?;
        let insns = decode(code.insns).ok()?;
        let this = u32::from(code.registers).checked_sub(u32::from(code.ins))?;
        let virt = data.virtual_methods.iter().any(|v| v.method == em.method);
        let found = virt.then(|| generated(dex, def.class, &fields, dex.string(mi.name).ok()?, mi.proto, this, &insns)).flatten();
        match found {
            Some((g, helper, hidden, s)) => {
                rec.generated.insert(em.method, g);
                helpers.extend(helper);
                rec.hidden.extend(hidden);
                names = names.or(s);
            }
            None => {
                for x in insns.iter().filter(|x| matches!(x.op, 0x6e..=0x72 | 0x74..=0x78)) {
                    others.extend(dex.method(x.idx).ok().map(|m| m.class).filter(|&c| c != def.superclass));
                }
            }
        }
    }
    let order = match names {
        Some(s) => {
            let text = dex.string(s).ok()?;
            let mut order = Vec::new();
            for n in text.split(|&c| c == b';').filter(|n| !n.is_empty()) {
                order.push(*fields.iter().find(|&&f| dex.field(f).and_then(|f| dex.string(f.name)).ok() == Some(n))?);
            }
            (order.len() == fields.len()).then_some(order)
        }
        None if fields.is_empty() => Some(Vec::new()),
        None => canonical_order(dex, &data, &fields),
    };
    match order {
        Some(order) => {
            rec.components = order;
            used.extend(helpers);
            kept.extend(others);
            Some(rec)
        }
        None => {
            kept.extend(helpers);
            kept.extend(others);
            None
        }
    }
}

pub fn scan(dex: &Dex) -> Records {
    let mut out = Records::default();
    let (mut used, mut kept) = (BTreeSet::new(), BTreeSet::new());
    let is_tag = |t: u32| dex.type_desc(t).is_ok_and(|d| d.strip_prefix(b"L").and_then(|d| d.strip_suffix(b";")) == Some(RECORD_TAG));
    let mut tag = None;
    for i in 0..dex.class_count() {
        let Ok(def) = dex.class_def(i) else { continue };
        if is_tag(def.class) {
            tag = Some(i);
        }
        if !is_tag(def.superclass) {
            continue;
        }
        match record(dex, &def, &mut used, &mut kept) {
            Some(r) => {
                out.by_type.insert(def.class, r);
            }
            None => {
                kept.insert(def.superclass);
            }
        }
    }
    if let Some(t) = tag.filter(|&t| dex.class_def(t).is_ok_and(|d| !kept.contains(&d.class))) {
        out.omit.insert(t);
    }
    for ty in used.difference(&kept) {
        let Some(idx) = dex.class_of_type(*ty) else { continue };
        let Ok(def) = dex.class_def(idx) else { continue };
        let Ok(data) = dex.class_data(def.class_data_off) else { continue };
        let helper = def.access & 0x1000 != 0
            && data.virtual_methods.is_empty()
            && data.instance_fields.is_empty()
            && data.static_fields.is_empty()
            && data.direct_methods.iter().all(|m| m.access & 0x8 != 0);
        if helper {
            out.omit.insert(idx);
        }
    }
    out
}
