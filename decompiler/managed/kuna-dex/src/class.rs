//! One DEX class definition as a JVM class file: members, `Code`, and the
//! attributes the system annotations stand for (`Signature`, `Exceptions`,
//! `InnerClasses`, `EnclosingMethod`, `AnnotationDefault`,
//! `MethodParameters`); every other annotation becomes a runtime visible or
//! invisible (parameter) annotation. A method whose code cannot be
//! translated gets a body that throws and a `KunaDexError` attribute holding
//! the reason.

use std::collections::BTreeMap;

use crate::classfile::{op, Asm, Attr, Body, ClassFile, Member, Pool, ACC_SUPER};
use crate::code::Method;
use crate::dex::{
    Annotation, Dex, EncodedAnnotation, EncodedMethod, Value, NO_INDEX, VISIBILITY_BUILD,
    VISIBILITY_RUNTIME, VISIBILITY_SYSTEM,
};
use crate::emit::{code_attribute, Shared, THIS_NAME};
use crate::record::{Generated, Record};
use crate::{err, Options, Result, Translated};

const ACC_STATIC: u32 = 0x8;
const ACC_INTERFACE: u32 = 0x200;
const ACC_ENUM: u32 = 0x4000;
const ACC_ABSTRACT: u32 = 0x400;
const ACC_NATIVE: u32 = 0x100;
pub const ERROR_ATTRIBUTE: &str = "KunaDexError";

/// What a class's `dalvik.annotation` nesting annotations say.
#[derive(Clone, Debug, Default)]
pub struct Nest {
    /// The class has an `InnerClass` annotation (it is nested itself).
    pub inner: bool,
    pub name: Option<u32>,
    pub flags: u16,
    pub enclosing_class: Option<u32>,
    pub enclosing_method: Option<u32>,
    pub members: Vec<u32>,
}

/// Nesting facts for every class of a file, by type index.
#[derive(Debug, Default)]
pub struct Nesting {
    pub nest: BTreeMap<u32, Nest>,
    /// Enclosing type to the types nested in it, in class-definition order.
    pub nested: BTreeMap<u32, Vec<u32>>,
}

fn ann_type<'a>(dex: &Dex<'a>, a: &Annotation) -> Result<&'a [u8]> {
    dex.type_desc(a.value.ty)
}

fn element<'v>(dex: &Dex, a: &'v EncodedAnnotation, name: &[u8]) -> Option<&'v Value> {
    a.elements
        .iter()
        .find(|(n, _)| dex.string(*n).ok() == Some(name))
        .map(|(_, v)| v)
}

fn nest_of(dex: &Dex, anns: &[Annotation]) -> Result<Option<Nest>> {
    let mut nest = Nest::default();
    for a in anns.iter().filter(|a| a.visibility == VISIBILITY_SYSTEM) {
        match ann_type(dex, a)? {
            b"Ldalvik/annotation/InnerClass;" => {
                nest.inner = true;
                if let Some(Value::String(s)) = element(dex, &a.value, b"name") {
                    nest.name = Some(*s);
                }
                if let Some(Value::Int(f)) = element(dex, &a.value, b"accessFlags") {
                    nest.flags = *f as u16;
                }
            }
            b"Ldalvik/annotation/EnclosingClass;" => {
                if let Some(Value::Type(t)) = element(dex, &a.value, b"value") {
                    nest.enclosing_class = Some(*t);
                }
            }
            b"Ldalvik/annotation/EnclosingMethod;" => {
                if let Some(Value::Method(m)) = element(dex, &a.value, b"value") {
                    nest.enclosing_method = Some(*m);
                }
            }
            b"Ldalvik/annotation/MemberClasses;" => {
                if let Some(Value::Array(v)) = element(dex, &a.value, b"value") {
                    nest.members = v
                        .iter()
                        .filter_map(|x| match x {
                            Value::Type(t) => Some(*t),
                            _ => None,
                        })
                        .collect();
                }
            }
            _ => {}
        }
    }
    Ok((nest.inner || !nest.members.is_empty()).then_some(nest))
}

impl Nesting {
    pub fn build(dex: &Dex) -> Nesting {
        let mut out = Nesting::default();
        for i in 0..dex.class_count() {
            let Ok(def) = dex.class_def(i) else { continue };
            let Ok(dir) = dex.annotations_directory(def.annotations_off) else {
                continue;
            };
            let Ok(anns) = dex.annotation_set(dir.class) else { continue };
            let Ok(Some(nest)) = nest_of(dex, &anns) else { continue };
            let outer = nest
                .enclosing_class
                .or_else(|| nest.enclosing_method.and_then(|m| dex.method(m).ok().map(|m| m.class)));
            if let Some(o) = outer {
                out.nested.entry(o).or_default().push(def.class);
            }
            out.nest.entry(def.class).or_insert(nest);
        }
        out
    }

    fn outer_of(&self, dex: &Dex, ty: u32) -> Option<u32> {
        let n = self.nest.get(&ty).filter(|n| n.inner)?;
        n.enclosing_class
            .or_else(|| n.enclosing_method.and_then(|m| dex.method(m).ok().map(|m| m.class)))
    }
}

/// JDK interfaces with static or default methods that DEX code calls
/// directly; DEX does not say whether a method's class is an interface.
pub(crate) fn library_interface(desc: &[u8]) -> bool {
    const NAMES: [&[u8]; 14] = [
        b"Ljava/util/Comparator;",
        b"Ljava/util/List;",
        b"Ljava/util/Set;",
        b"Ljava/util/Map;",
        b"Ljava/util/Map$Entry;",
        b"Ljava/util/Collection;",
        b"Ljava/util/Iterator;",
        b"Ljava/lang/Iterable;",
        b"Ljava/lang/CharSequence;",
        b"Ljava/nio/file/Path;",
        b"Ljava/util/stream/Stream;",
        b"Ljava/util/stream/IntStream;",
        b"Ljava/util/stream/LongStream;",
        b"Ljava/util/stream/DoubleStream;",
    ];
    NAMES.contains(&desc) || desc.starts_with(b"Ljava/util/function/")
}

fn internal(desc: &[u8]) -> &[u8] {
    match desc {
        [b'L', rest @ .., b';'] => rest,
        d => d,
    }
}

struct Writer<'d, 'a> {
    dex: &'d Dex<'a>,
    peers: &'d [Dex<'d>],
    pool: Pool,
    bootstrap: Vec<(u16, Vec<u16>)>,
    version: u16,
}

impl<'d, 'a> Writer<'d, 'a> {
    fn attr(&mut self, name: &str, body: Vec<u8>) -> Attr {
        (self.pool.utf8(name.as_bytes()), body)
    }

    fn element_value(&mut self, v: &Value, out: &mut Body) -> Result<bool> {
        let dex = self.dex;
        let pool = &mut self.pool;
        match *v {
            Value::Byte(x) => out.u8(b'B').u16(pool.int(i32::from(x))),
            Value::Short(x) => out.u8(b'S').u16(pool.int(i32::from(x))),
            Value::Char(x) => out.u8(b'C').u16(pool.int(i32::from(x))),
            Value::Int(x) => out.u8(b'I').u16(pool.int(x)),
            Value::Boolean(x) => out.u8(b'Z').u16(pool.int(i32::from(x))),
            Value::Long(x) => out.u8(b'J').u16(pool.long(x)),
            Value::Float(x) => out.u8(b'F').u16(pool.float(x)),
            Value::Double(x) => out.u8(b'D').u16(pool.double(x)),
            Value::String(s) => out.u8(b's').u16(pool.utf8(dex.string(s)?)),
            Value::Type(t) => out.u8(b'c').u16(pool.utf8(dex.type_desc(t)?)),
            Value::Enum(f) => {
                let f = dex.field(f)?;
                let t = pool.utf8(dex.type_desc(f.class)?);
                let n = pool.utf8(dex.string(f.name)?);
                out.u8(b'e').u16(t).u16(n)
            }
            Value::Annotation(ref a) => {
                out.u8(b'@');
                return self.annotation(a, out);
            }
            Value::Array(ref items) => {
                let mut inner = Body::default();
                let mut n = 0u16;
                for item in items {
                    if self.element_value(item, &mut inner)? {
                        n += 1;
                    }
                }
                out.u8(b'[').u16(n);
                out.0.extend_from_slice(&inner.0);
                return Ok(true);
            }
            _ => return Ok(false),
        };
        Ok(true)
    }

    fn annotation(&mut self, a: &EncodedAnnotation, out: &mut Body) -> Result<bool> {
        let t = self.pool.utf8(self.dex.type_desc(a.ty)?);
        let mut pairs = Body::default();
        let mut n = 0u16;
        for (name, v) in &a.elements {
            let mut val = Body::default();
            if self.element_value(v, &mut val)? {
                pairs.u16(self.pool.utf8(self.dex.string(*name)?));
                pairs.0.extend_from_slice(&val.0);
                n += 1;
            }
        }
        out.u16(t).u16(n);
        out.0.extend_from_slice(&pairs.0);
        Ok(true)
    }

    /// `Runtime{Visible,Invisible}Annotations` for the non-system annotations.
    fn annotations(&mut self, anns: &[Annotation], attrs: &mut Vec<Attr>) -> Result<()> {
        for (vis, name) in [
            (VISIBILITY_RUNTIME, "RuntimeVisibleAnnotations"),
            (VISIBILITY_BUILD, "RuntimeInvisibleAnnotations"),
        ] {
            let mut body = Body::default();
            let mut n = 0u16;
            for a in anns.iter().filter(|a| a.visibility == vis) {
                self.annotation(&a.value, &mut body)?;
                n += 1;
            }
            if n > 0 {
                let mut b = Body::default();
                b.u16(n);
                b.0.extend_from_slice(&body.0);
                let attr = self.attr(name, b.0);
                attrs.push(attr);
            }
        }
        Ok(())
    }

    fn system<'v>(&self, anns: &'v [Annotation], ty: &[u8]) -> Option<&'v EncodedAnnotation> {
        anns.iter()
            .filter(|a| a.visibility == VISIBILITY_SYSTEM)
            .find(|a| self.dex.type_desc(a.value.ty).ok() == Some(ty))
            .map(|a| &a.value)
    }

    fn signature(&mut self, anns: &[Annotation], attrs: &mut Vec<Attr>) -> Result<()> {
        let Some(a) = self.system(anns, b"Ldalvik/annotation/Signature;") else {
            return Ok(());
        };
        let Some(Value::Array(parts)) = element(self.dex, a, b"value") else {
            return Ok(());
        };
        let mut sig = Vec::new();
        for p in parts {
            if let Value::String(s) = p {
                sig.extend_from_slice(self.dex.string(*s)?);
            }
        }
        let idx = self.pool.utf8(&sig);
        let attr = self.attr("Signature", idx.to_be_bytes().to_vec());
        attrs.push(attr);
        Ok(())
    }

    fn constant_value(&mut self, v: &Value, desc: &[u8], is_final: bool) -> Result<Option<u16>> {
        let int = |x: i32| (x != 0 || is_final).then_some(x);
        Ok(match (desc.first(), v) {
            (Some(b'Z' | b'B' | b'S' | b'C' | b'I'), Value::Int(x)) => int(*x).map(|x| self.pool.int(x)),
            (Some(b'Z' | b'B' | b'S' | b'C' | b'I'), Value::Short(x)) => int(i32::from(*x)).map(|x| self.pool.int(x)),
            (Some(b'Z' | b'B' | b'S' | b'C' | b'I'), Value::Byte(x)) => int(i32::from(*x)).map(|x| self.pool.int(x)),
            (Some(b'Z' | b'B' | b'S' | b'C' | b'I'), Value::Char(x)) => int(i32::from(*x)).map(|x| self.pool.int(x)),
            (Some(b'Z'), Value::Boolean(x)) => int(i32::from(*x)).map(|x| self.pool.int(x)),
            (Some(b'J'), Value::Long(x)) => (*x != 0 || is_final).then(|| self.pool.long(*x)),
            (Some(b'F'), Value::Float(x)) => (*x != 0 || is_final).then(|| self.pool.float(*x)),
            (Some(b'D'), Value::Double(x)) => (*x != 0 || is_final).then(|| self.pool.double(*x)),
            (Some(b'L'), Value::String(s)) if desc == b"Ljava/lang/String;" => {
                Some(self.pool.string(self.dex.string(*s)?))
            }
            _ => None,
        })
    }

    fn inner_classes(&mut self, ty: u32, nesting: &Nesting) -> Result<Vec<Attr>> {
        let dex = self.dex;
        let mut order: Vec<u32> = Vec::new();
        let mut t = ty;
        let mut depth = 0;
        while nesting.nest.get(&t).is_some_and(|n| n.inner) && depth < 64 {
            order.push(t);
            match nesting.outer_of(dex, t) {
                Some(o) => t = o,
                None => break,
            }
            depth += 1;
        }
        if let Some(n) = nesting.nest.get(&ty) {
            order.extend(n.members.iter().copied());
        }
        if let Some(v) = nesting.nested.get(&ty) {
            order.extend(v.iter().copied());
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut body = Body::default();
        let mut n = 0u16;
        for t in order {
            if !seen.insert(t) {
                continue;
            }
            let desc = dex.type_desc(t)?;
            let inner = self.pool.class(internal(desc));
            let (outer, name, flags) = match nesting.nest.get(&t).filter(|n| n.inner) {
                Some(nest) => {
                    let member = nest.enclosing_class.is_some() && nest.name.is_some();
                    let outer = match (member, nest.enclosing_class) {
                        (true, Some(o)) => self.pool.class(internal(dex.type_desc(o)?)),
                        _ => 0,
                    };
                    let name = match nest.name {
                        Some(s) => self.pool.utf8(dex.string(s)?),
                        None => 0,
                    };
                    (outer, name, nest.flags)
                }
                None => {
                    let outer_desc = dex.type_desc(ty)?;
                    let outer_name = internal(outer_desc);
                    let simple = internal(desc)
                        .strip_prefix(outer_name)
                        .and_then(|s| s.strip_prefix(b"$"))
                        .unwrap_or(internal(desc));
                    let outer = self.pool.class(outer_name);
                    (outer, self.pool.utf8(simple), 0x0009)
                }
            };
            body.u16(inner).u16(outer).u16(name).u16(flags);
            n += 1;
        }
        let mut attrs = Vec::new();
        if n > 0 {
            let mut b = Body::default();
            b.u16(n);
            b.0.extend_from_slice(&body.0);
            attrs.push(self.attr("InnerClasses", b.0));
        }
        if let Some(nest) = nesting.nest.get(&ty).filter(|n| n.inner) {
            let enclosing = match (nest.enclosing_method, nest.enclosing_class, nest.name) {
                (Some(m), _, _) => {
                    let mi = dex.method(m)?;
                    let class = self.pool.class(internal(dex.type_desc(mi.class)?));
                    let p = dex.proto(mi.proto)?;
                    let mut d = vec![b'('];
                    for t in p.params {
                        d.extend_from_slice(dex.type_desc(t)?);
                    }
                    d.push(b')');
                    d.extend_from_slice(dex.type_desc(p.ret)?);
                    let name = dex.string(mi.name)?;
                    let nt = if name == b"<clinit>" {
                        0
                    } else {
                        self.pool.name_and_type(name, &d)
                    };
                    Some((class, nt))
                }
                (None, Some(c), None) => Some((self.pool.class(internal(dex.type_desc(c)?)), 0)),
                _ => None,
            };
            if let Some((class, nt)) = enclosing {
                let mut b = Body::default();
                b.u16(class).u16(nt);
                attrs.push(self.attr("EnclosingMethod", b.0));
            }
        }
        Ok(attrs)
    }

    /// `NestHost` (the outermost class) or `NestMembers` (every class nested
    /// in it, at any depth), so the private accesses d8's nest accessors stood
    /// for are legal again (version 55).
    fn nest_attributes(&mut self, ty: u32, nesting: &Nesting) -> Result<Vec<Attr>> {
        let dex = self.dex;
        let mut host = ty;
        for _ in 0..64 {
            match nesting.outer_of(dex, host) {
                Some(o) if o != host => host = o,
                _ => break,
            }
        }
        let mut body = Body::default();
        let name = if host != ty {
            body.u16(self.pool.class(internal(dex.type_desc(host)?)));
            "NestHost"
        } else {
            let mut members = Vec::new();
            let mut work = vec![ty];
            while let Some(t) = work.pop() {
                for &n in nesting.nested.get(&t).into_iter().flatten() {
                    if n != ty && !members.contains(&n) && members.len() < 4096 {
                        members.push(n);
                        work.push(n);
                    }
                }
            }
            if members.is_empty() {
                return Ok(Vec::new());
            }
            body.u16(members.len() as u16);
            for m in members {
                body.u16(self.pool.class(internal(dex.type_desc(m)?)));
            }
            "NestMembers"
        };
        self.version = self.version.max(55);
        Ok(vec![self.attr(name, body.0)])
    }

    fn failed_body(&mut self, message: &str) -> Vec<u8> {
        let mut asm = Asm::new(false);
        let class = self.pool.class(b"java/lang/IllegalStateException");
        let text = format!("kuna-dex: {message}");
        let s = self.pool.string(text.as_bytes());
        let init = self
            .pool
            .method(b"java/lang/IllegalStateException", b"<init>", b"(Ljava/lang/String;)V", false);
        asm.op_u16(op::NEW, class, 1);
        asm.op(op::DUP, &[], 1);
        asm.op_u16(op::LDC_W, s, 1);
        asm.op_u16(op::INVOKESPECIAL, init, -2);
        asm.op(op::ATHROW, &[], -1);
        let mut b = Body::default();
        b.u16(3).u16(0).u32(asm.code.len() as u32);
        b.0.extend_from_slice(&asm.code);
        b.u16(0).u16(0);
        b.0
    }

    /// javac's `toString`, `hashCode` or `equals` of a record: one
    /// `invokedynamic` to `ObjectMethods.bootstrap` over the components.
    fn record_method(&mut self, this: &[u8], r: &Record, g: Generated) -> Result<Vec<u8>> {
        let dex = self.dex;
        let bsm = self.pool.method(
            b"java/lang/runtime/ObjectMethods",
            b"bootstrap",
            b"(Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/TypeDescriptor;Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/invoke/MethodHandle;)Ljava/lang/Object;",
            false,
        );
        let bsm = self.pool.method_handle(6, bsm);
        let mut names = Vec::new();
        let mut args = vec![self.pool.class(this)];
        let mut getters = Vec::new();
        for (i, &f) in r.components.iter().enumerate() {
            let fi = dex.field(f)?;
            let name = dex.string(fi.name)?;
            if i > 0 {
                names.push(b';');
            }
            names.extend_from_slice(name);
            let field = self.pool.field(this, name, dex.type_desc(fi.ty)?);
            getters.push(self.pool.method_handle(1, field));
        }
        args.push(self.pool.string(&names));
        args.extend(getters);
        let entry = (bsm, args);
        let index = match self.bootstrap.iter().position(|e| *e == entry) {
            Some(i) => i,
            None => {
                self.bootstrap.push(entry);
                self.bootstrap.len() - 1
            }
        };
        let mut recv = vec![b'(', b'L'];
        recv.extend_from_slice(this);
        recv.push(b';');
        let (name, desc, locals, ret): (&[u8], Vec<u8>, u16, u8) = match g {
            Generated::ToString => (b"toString", [&recv[..], b")Ljava/lang/String;"].concat(), 1, op::IRETURN + 4),
            Generated::HashCode => (b"hashCode", [&recv[..], b")I"].concat(), 1, op::IRETURN),
            Generated::Equals => (b"equals", [&recv[..], b"Ljava/lang/Object;)Z"].concat(), 2, op::IRETURN),
        };
        let indy = self.pool.invoke_dynamic(index as u16, name, &desc);
        let mut asm = Asm::new(false);
        for slot in 0..locals {
            asm.local(op::ILOAD, 4, slot, 1);
        }
        asm.op(op::INVOKEDYNAMIC, &[(indy >> 8) as u8, indy as u8, 0, 0], 1 - i32::from(locals));
        asm.op(ret, &[], -1);
        self.version = self.version.max(52);
        let mut b = Body::default();
        b.u16(locals).u16(locals).u32(asm.code.len() as u32);
        b.0.extend_from_slice(&asm.code);
        b.u16(0).u16(0);
        Ok(b.0)
    }

    #[allow(clippy::too_many_arguments)]
    fn method(
        &mut self,
        this_desc: &[u8],
        em: &EncodedMethod,
        anns: &[Annotation],
        params: Option<&[Vec<Annotation>]>,
        defaults: Option<&EncodedAnnotation>,
        opts: &Options,
        failures: &mut Vec<String>,
    ) -> Result<Member> {
        let dex = self.dex;
        let mi = dex.method(em.method)?;
        let name = dex.string(mi.name)?;
        let proto = dex.proto(mi.proto)?;
        let mut desc = vec![b'('];
        for &t in &proto.params {
            desc.extend_from_slice(dex.type_desc(t)?);
        }
        desc.push(b')');
        desc.extend_from_slice(dex.type_desc(proto.ret)?);
        let mut attrs = Vec::new();
        if em.code_off != 0 {
            let names = self.parameter_names(anns);
            let body = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.code(this_desc, em, proto.clone(), opts, &names)))
                .unwrap_or_else(|_| Err(err("internal error in kuna-dex".to_string())));
            match body {
                Ok(b) => {
                    let a = self.attr("Code", b);
                    attrs.push(a);
                }
                Err(e) => {
                    let label = format!("{}{}", crate::mutf8_to_string(name), crate::mutf8_to_string(&desc));
                    failures.push(format!("{label}: {}", e.0));
                    let b = self.failed_body(&e.0);
                    let a = self.attr("Code", b);
                    attrs.push(a);
                    let text = self.pool.utf8(e.0.as_bytes());
                    let a = self.attr(ERROR_ATTRIBUTE, text.to_be_bytes().to_vec());
                    attrs.push(a);
                }
            }
        }
        if let Some(a) = self.system(anns, b"Ldalvik/annotation/Throws;") {
            if let Some(Value::Array(types)) = element(dex, a, b"value") {
                let mut b = Body::default();
                let list: Vec<u32> = types
                    .iter()
                    .filter_map(|v| match v {
                        Value::Type(t) => Some(*t),
                        _ => None,
                    })
                    .collect();
                b.u16(list.len() as u16);
                for t in list {
                    let c = self.pool.class(internal(dex.type_desc(t)?));
                    b.u16(c);
                }
                let a = self.attr("Exceptions", b.0);
                attrs.push(a);
            }
        }
        self.signature(anns, &mut attrs)?;
        if let Some(v) = defaults.and_then(|d| element(dex, d, name)) {
            let mut b = Body::default();
            if self.element_value(v, &mut b)? {
                let a = self.attr("AnnotationDefault", b.0);
                attrs.push(a);
            }
        }
        if let Some(a) = self.system(anns, b"Ldalvik/annotation/MethodParameters;") {
            if let (Some(Value::Array(names)), Some(Value::Array(flags))) =
                (element(dex, a, b"names"), element(dex, a, b"accessFlags"))
            {
                let mut b = Body::default();
                b.u8(names.len().min(255) as u8);
                for (i, n) in names.iter().take(255).enumerate() {
                    let ni = match n {
                        Value::String(s) => self.pool.utf8(dex.string(*s)?),
                        _ => 0,
                    };
                    let f = match flags.get(i) {
                        Some(Value::Int(f)) => *f as u16,
                        _ => 0,
                    };
                    b.u16(ni).u16(f);
                }
                let a = self.attr("MethodParameters", b.0);
                attrs.push(a);
            }
        }
        self.annotations(anns, &mut attrs)?;
        if let Some(sets) = params {
            for (vis, aname) in [
                (VISIBILITY_RUNTIME, "RuntimeVisibleParameterAnnotations"),
                (VISIBILITY_BUILD, "RuntimeInvisibleParameterAnnotations"),
            ] {
                if !sets.iter().flatten().any(|a| a.visibility == vis) {
                    continue;
                }
                let mut b = Body::default();
                b.u8(sets.len().min(255) as u8);
                for set in sets.iter().take(255) {
                    let chosen: Vec<&Annotation> = set.iter().filter(|a| a.visibility == vis).collect();
                    b.u16(chosen.len() as u16);
                    for a in chosen {
                        self.annotation(&a.value, &mut b)?;
                    }
                }
                let a = self.attr(aname, b.0);
                attrs.push(a);
            }
        }
        Ok(Member {
            access: (em.access & 0x1dff) as u16,
            name: self.pool.utf8(name),
            desc: self.pool.utf8(&desc),
            attrs,
        })
    }

    /// The parameter names of a `MethodParameters` annotation (`NO_INDEX`
    /// for one without).
    fn parameter_names(&self, anns: &[Annotation]) -> Vec<u32> {
        match self.system(anns, b"Ldalvik/annotation/MethodParameters;").and_then(|a| element(self.dex, a, b"names")) {
            Some(Value::Array(names)) => names.iter().map(|n| if let Value::String(s) = n { *s } else { NO_INDEX }).collect(),
            _ => Vec::new(),
        }
    }

    /// `names`: the `MethodParameters` names (see `Method::param_names`).
    fn code(
        &mut self,
        this_desc: &[u8],
        em: &EncodedMethod,
        proto: crate::dex::Proto,
        opts: &Options,
        names: &[u32],
    ) -> Result<Vec<u8>> {
        let dex = self.dex;
        let code = dex.code_item(em.code_off)?;
        let is_static = em.access & ACC_STATIC != 0;
        let first = u32::from(code.registers).saturating_sub(u32::from(code.ins));
        let mut params = Vec::new();
        let mut reg = first;
        if !is_static {
            params.push((reg, NO_INDEX));
            reg += 1;
        }
        for &t in &proto.params {
            params.push((reg, t));
            reg += if matches!(dex.type_desc(t)?.first(), Some(b'J' | b'D')) { 2 } else { 1 };
        }
        let debug = dex.debug_info(&code, &params, THIS_NAME).unwrap_or_default();
        let mut m = Method::new(dex, &code, &debug, is_static, proto, !opts.verifiable)?;
        m.this_desc = this_desc.to_vec();
        m.param_names = names.to_vec();
        let peers = self.peers;
        let interfaces = |desc: &[u8]| {
            let def = std::iter::once(dex).chain(peers).find_map(|d| d.defined(desc).and_then(|t| d.class_def(d.class_of_type(t)?).ok()));
            match def {
                Some(d) => d.access & ACC_INTERFACE != 0,
                None => library_interface(desc),
            }
        };
        let mut sh = Shared {
            pool: &mut self.pool,
            bootstrap: &mut self.bootstrap,
            version: self.version,
            interfaces: &interfaces,
            verifiable: opts.verifiable,
            peers,
        };
        let out = code_attribute(&m, &mut sh)?;
        self.version = sh.version;
        Ok(out)
    }
}

/// The static fields `<clinit>` writes, in the order it first writes them
/// (d8 leaves trailing default static values out of the class's static
/// values, so a final field without one and not written there is a constant
/// with its type's default).
fn clinit_puts(dex: &Dex, data: &crate::dex::ClassData) -> Vec<u32> {
    let mut out = Vec::new();
    for em in &data.direct_methods {
        let is_clinit = dex
            .method(em.method)
            .and_then(|m| dex.string(m.name))
            .is_ok_and(|n| n == b"<clinit>");
        if !is_clinit || em.code_off == 0 {
            continue;
        }
        let Ok(code) = dex.code_item(em.code_off) else { continue };
        for x in crate::insn::decode(code.insns).unwrap_or_default() {
            if (0x67..=0x6d).contains(&x.op) && !out.contains(&x.idx) {
                out.push(x.idx);
            }
        }
    }
    out
}

/// Translates class definition `idx`.
pub fn translate(dex: &Dex, idx: u32, opts: &Options, peers: &[Dex]) -> Result<Translated> {
    let def = dex.class_def(idx)?;
    let this_desc = dex.type_desc(def.class)?;
    let name = internal(this_desc);
    if this_desc.first() != Some(&b'L') {
        return Err(err(format!("class {} is not a class type", crate::mutf8_to_string(this_desc))));
    }
    let mut w = Writer {
        dex,
        peers,
        pool: Pool::new(),
        bootstrap: Vec::new(),
        version: opts.min_version.max(49),
    };
    let this = w.pool.class(name);
    let record = dex.records().by_type.get(&def.class).filter(|_| !opts.verifiable);
    let superclass = if def.superclass == NO_INDEX {
        0
    } else if record.is_some() {
        w.pool.class(b"java/lang/Record")
    } else {
        w.pool.class(internal(dex.type_desc(def.superclass)?))
    };
    let mut interfaces = Vec::new();
    for t in dex.type_list(def.interfaces_off)? {
        interfaces.push(w.pool.class(internal(dex.type_desc(t)?)));
    }
    let dir = dex.annotations_directory(def.annotations_off)?;
    let class_anns = dex.annotation_set(dir.class)?;
    let data = dex.class_data(def.class_data_off)?;
    let statics = dex.encoded_array(def.static_values_off)?;
    let field_anns: BTreeMap<u32, u32> = dir.fields.iter().copied().collect();
    let method_anns: BTreeMap<u32, u32> = dir.methods.iter().copied().collect();
    let param_anns: BTreeMap<u32, u32> = dir.parameters.iter().copied().collect();
    let assigned = clinit_puts(dex, &data);
    let mut fields = Vec::new();
    for (i, ef) in data.static_fields.iter().chain(&data.instance_fields).enumerate() {
        let f = dex.field(ef.field)?;
        let desc = dex.type_desc(f.ty)?;
        let anns = match field_anns.get(&ef.field) {
            Some(&off) => dex.annotation_set(off)?,
            None => Vec::new(),
        };
        let mut attrs = Vec::new();
        if i < data.static_fields.len() && !assigned.contains(&ef.field) {
            let default = match desc.first() {
                Some(b'Z' | b'B' | b'S' | b'C' | b'I') => Some(Value::Int(0)),
                Some(b'J') => Some(Value::Long(0)),
                Some(b'F') => Some(Value::Float(0)),
                Some(b'D') => Some(Value::Double(0)),
                _ => None,
            };
            if let Some(v) = statics.get(i).or(default.as_ref()) {
                if let Some(c) = w.constant_value(v, desc, ef.access & 0x10 != 0)? {
                    let a = w.attr("ConstantValue", c.to_be_bytes().to_vec());
                    attrs.push(a);
                }
            }
        }
        w.signature(&anns, &mut attrs)?;
        w.annotations(&anns, &mut attrs)?;
        fields.push(Member {
            access: (ef.access & 0x50df) as u16,
            name: w.pool.utf8(dex.string(f.name)?),
            desc: w.pool.utf8(desc),
            attrs,
        });
    }
    if def.access & ACC_ENUM != 0 {
        let n = data.static_fields.len();
        let rank = |i: usize| {
            let ef = &data.static_fields[i];
            let pos = assigned.iter().position(|&f| f == ef.field).unwrap_or(usize::MAX);
            if ef.access & ACC_ENUM != 0 { (0, pos, i) } else { (1, 0, i) }
        };
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| rank(i));
        let mut statics: Vec<Option<Member>> = fields.drain(..n).map(Some).collect();
        let sorted: Vec<Member> = order.iter().map(|&i| statics[i].take().expect("each once")).collect();
        fields.splice(0..0, sorted);
    }
    let defaults = w
        .system(&class_anns, b"Ldalvik/annotation/AnnotationDefault;")
        .and_then(|a| match element(dex, a, b"value") {
            Some(Value::Annotation(d)) => Some(d.clone()),
            _ => None,
        });
    let mut methods = Vec::new();
    let mut failures = Vec::new();
    let is_interface = def.access & ACC_INTERFACE != 0;
    for em in data.direct_methods.iter().chain(&data.virtual_methods) {

        if let Some(r) = record {
            if r.hidden.contains(&em.method) {
                continue;
            }
            if let Some(&g) = r.generated.get(&em.method) {
                let mi = dex.method(em.method)?;
                let body = w.record_method(name, r, g)?;
                let code = w.attr("Code", body);
                let desc = dex.proto_desc(mi.proto)?;
                methods.push(Member {
                    access: (em.access & 0x1dff) as u16,
                    name: w.pool.utf8(dex.string(mi.name)?),
                    desc: w.pool.utf8(&desc),
                    attrs: vec![code],
                });
                continue;
            }
        }
        let anns = match method_anns.get(&em.method) {
            Some(&off) => dex.annotation_set(off)?,
            None => Vec::new(),
        };
        let params = match param_anns.get(&em.method) {
            Some(&off) => Some(dex.annotation_set_list(off)?),
            None => None,
        };
        if is_interface && em.code_off != 0 && em.access & (ACC_ABSTRACT | ACC_NATIVE) == 0 {
            let mi = dex.method(em.method)?;
            if dex.string(mi.name)? != b"<clinit>" {
                w.version = w.version.max(52);
            }
        }
        methods.push(w.method(
            this_desc,
            em,
            &anns,
            params.as_deref(),
            defaults.as_ref(),
            opts,
            &mut failures,
        )?);
    }
    let mut attrs = Vec::new();
    if let Some(r) = record {
        let mut b = Body::default();
        b.u16(r.components.len() as u16);
        for &f in &r.components {
            let fi = dex.field(f)?;
            let anns = match field_anns.get(&f) {
                Some(&off) => dex.annotation_set(off)?,
                None => Vec::new(),
            };
            let mut own = Vec::new();
            w.signature(&anns, &mut own)?;
            w.annotations(&anns, &mut own)?;
            let (n, d) = (w.pool.utf8(dex.string(fi.name)?), w.pool.utf8(dex.type_desc(fi.ty)?));
            b.u16(n).u16(d).u16(own.len() as u16);
            for (an, body) in own {
                b.u16(an).u32(body.len() as u32);
                b.0.extend_from_slice(&body);
            }
        }
        let a = w.attr("Record", b.0);
        attrs.push(a);
    }
    if def.source_file != NO_INDEX {
        let s = w.pool.utf8(dex.string(def.source_file)?);
        let a = w.attr("SourceFile", s.to_be_bytes().to_vec());
        attrs.push(a);
    }
    w.signature(&class_anns, &mut attrs)?;
    let nesting = dex.nesting();
    attrs.extend(w.inner_classes(def.class, nesting)?);
    if !opts.verifiable {
        attrs.extend(w.nest_attributes(def.class, nesting)?);
    }
    w.annotations(&class_anns, &mut attrs)?;
    if !w.bootstrap.is_empty() {
        let mut b = Body::default();
        b.u16(w.bootstrap.len() as u16);
        for (h, args) in std::mem::take(&mut w.bootstrap) {
            b.u16(h).u16(args.len() as u16);
            for a in args {
                b.u16(a);
            }
        }
        let a = w.attr("BootstrapMethods", b.0);
        attrs.push(a);
    }
    let mut access = (def.access & 0x7611) as u16;
    if !is_interface {
        access |= ACC_SUPER;
    }
    let file = ClassFile {
        version: w.version,
        access,
        this,
        superclass,
        interfaces,
        fields,
        methods,
        attrs,
    };
    Ok(Translated {
        name: crate::mutf8_to_string(name),
        bytes: file.write(&w.pool)?,
        failures,
        omit: !opts.verifiable
            && (dex.backports().classes.contains(&idx) || dex.records().omit.contains(&idx) || dex.nest().classes.contains(&idx)),
    })
}
