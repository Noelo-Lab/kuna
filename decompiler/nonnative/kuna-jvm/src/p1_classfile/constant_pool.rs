//! Port of `org/jetbrains/java/decompiler/struct/consts/ConstantPool.java`.
//!
//! kuna reads a pool lazily when no pool interceptor is set: construction
//! only records where each entry lies, and an entry is read (and resolved, its
//! referenced entries first) when it is first asked for. The values are those
//! of Java's two-pass resolution whenever every reference has the kind that
//! pass expects; a pool where one does not is read eagerly, as in Java.

use std::rc::Rc;

use crate::byte_cursor::ByteCursor;
use crate::code_constants::{
    CONSTANT_CLASS, CONSTANT_DOUBLE, CONSTANT_DYNAMIC, CONSTANT_FIELDREF, CONSTANT_FLOAT,
    CONSTANT_INTEGER, CONSTANT_INTERFACE_METHODREF, CONSTANT_INVOKE_DYNAMIC, CONSTANT_LONG,
    CONSTANT_METHODREF, CONSTANT_METHOD_HANDLE, CONSTANT_METHOD_HANDLE_REF_GET_FIELD,
    CONSTANT_METHOD_HANDLE_REF_GET_STATIC, CONSTANT_METHOD_HANDLE_REF_PUT_FIELD,
    CONSTANT_METHOD_HANDLE_REF_PUT_STATIC, CONSTANT_METHOD_TYPE, CONSTANT_MODULE,
    CONSTANT_NAME_AND_TYPE, CONSTANT_PACKAGE, CONSTANT_STRING, CONSTANT_UTF8,
};
use crate::decompiler_context::DecompilerContext;
use crate::field_descriptor::FieldDescriptor;
use crate::link_constant::LinkConstant;
use crate::method_descriptor::MethodDescriptor;
use crate::new_class_name_builder::NewClassNameBuilder;
use crate::pool_interceptor::PoolInterceptor;
use crate::pooled_constant::PooledConstant;
use crate::primitive_constant::{ConstValue, PrimitiveConstant};
use crate::rt::jstring::decode_modified_utf8;
use crate::rt::BitSet;
use crate::var_type::{java_split, VarType};
use crate::{arena_identity, arena_object};

pub struct ConstantPoolData {
    pub pool: Vec<Option<PooledConstant>>,
    pub interceptor: Option<PoolInterceptor>,
    pending: Option<Rc<Pending>>,
}

/// The entries of a lazily read pool: the class bytes and, by index, the
/// offset of each entry's tag (0: no entry).
struct Pending {
    bytes: ByteCursor,
    at: Vec<u32>,
}

arena_object!(pub ConstantPool => ConstantPoolData, Session);
arena_identity!(ConstantPool);

fn second_token(new_element: &str) -> String {
    match java_split(new_element, |c| c == ' ', false).get(1) {
        Some(s) => s.to_string(),
        None => crate::jthrow!(ArrayIndexOutOfBoundsException; "Index 1 out of bounds"),
    }
}

impl ConstantPool {
    pub const FIELD: i32 = 1;
    pub const METHOD: i32 = 2;

    /// Java: `ConstantPool(DataInputStream)`
    pub fn new(in_: &mut ByteCursor) -> ConstantPool {
        let interceptor = DecompilerContext::get_pool_interceptor();
        if interceptor.is_none() {
            let start = in_.position();
            let (this, at) = ConstantPool::scan(in_);
            let bytes = in_.share_at(0);
            if Self::resolves_in_any_order(bytes.data(), &at) {
                this.with_mut(|d| {
                    d.pool = vec![None; at.len()];
                    d.pending = Some(Rc::new(Pending { bytes, at }));
                });
                return this;
            }
            return ConstantPool::new_without_interceptor(&mut in_.share_at(start));
        }
        let this = ConstantPool::new_without_interceptor(in_);
        this.with_mut(|d| d.interceptor = interceptor);
        this
    }

    /// Reads the entry layout as the Java constructor reads the entries (same
    /// end position, same exceptions); returns each entry's tag offset.
    fn scan(in_: &mut ByteCursor) -> (ConstantPool, Vec<u32>) {
        let size = in_.read_unsigned_short();
        let this = ConstantPool::alloc(ConstantPoolData {
            pool: Vec::new(),
            interceptor: None,
            pending: None,
        });
        let mut at: Vec<u32> = Vec::with_capacity(size as usize);
        at.push(0);
        let mut i = 1;
        while i < size {
            let offset = in_.position() as u32;
            let tag = in_.read_unsigned_byte() as i8;
            match tag as i32 {
                CONSTANT_UTF8 => {
                    let len = in_.read_unsigned_short() as usize;
                    in_.discard(len);
                }
                CONSTANT_INTEGER => {
                    in_.read_int();
                }
                CONSTANT_FLOAT => {
                    in_.read_float();
                }
                CONSTANT_LONG | CONSTANT_DOUBLE => {
                    in_.read_long();
                    at.push(offset);
                    at.push(0);
                    i += 2;
                    continue;
                }
                CONSTANT_CLASS | CONSTANT_STRING | CONSTANT_METHOD_TYPE | CONSTANT_MODULE
                | CONSTANT_PACKAGE => {
                    in_.read_unsigned_short();
                }
                CONSTANT_NAME_AND_TYPE
                | CONSTANT_FIELDREF
                | CONSTANT_METHODREF
                | CONSTANT_INTERFACE_METHODREF
                | CONSTANT_INVOKE_DYNAMIC
                | CONSTANT_DYNAMIC => {
                    in_.read_unsigned_short();
                    in_.read_unsigned_short();
                }
                CONSTANT_METHOD_HANDLE => {
                    in_.read_unsigned_byte();
                    in_.read_unsigned_short();
                }
                _ => crate::jthrow!("Invalid Constant Pool entry #{} Type: {}", i, tag),
            }
            at.push(offset);
            i += 1;
        }
        (this, at)
    }

    /// Whether resolving entries on demand gives what the Java constructor's
    /// passes give and throws nothing there: every reference points at an
    /// entry of a kind resolved in an earlier pass, and every method
    /// descriptor `LinkConstant` checks is well formed.
    fn resolves_in_any_order(data: &[u8], at: &[u32]) -> bool {
        let tag = |i: usize| -> i32 {
            match at.get(i) {
                Some(&o) if o != 0 => data[o as usize] as i32,
                _ => 0,
            }
        };
        let u2 = |o: usize| ((data[o] as usize) << 8) | data[o + 1] as usize;
        let utf8 = |i: usize| -> Option<&[u8]> {
            (tag(i) == CONSTANT_UTF8).then(|| {
                let o = at[i] as usize;
                &data[o + 3..o + 3 + u2(o + 1)]
            })
        };
        let method_descriptor = |name_and_type: usize| -> bool {
            let Some(d) = utf8(u2(at[name_and_type] as usize + 3)) else {
                return false;
            };
            if d.is_ascii() {
                d.first() == Some(&b'(') && d.contains(&b')')
            } else {
                let d = decode_modified_utf8(d);
                d.encode_utf16().count() >= 2 && d.contains(')') && d.starts_with('(')
            }
        };
        let name_and_type_of = |i: usize| -> Option<usize> {
            match tag(i) {
                CONSTANT_NAME_AND_TYPE => Some(i),
                CONSTANT_FIELDREF
                | CONSTANT_METHODREF
                | CONSTANT_INTERFACE_METHODREF
                | CONSTANT_INVOKE_DYNAMIC
                | CONSTANT_DYNAMIC => Some(u2(at[i] as usize + 3)),
                _ => None,
            }
        };
        for (i, &o) in at.iter().enumerate() {
            if o == 0 {
                continue;
            }
            let o = o as usize;
            let ok = match tag(i) {
                CONSTANT_CLASS | CONSTANT_STRING | CONSTANT_METHOD_TYPE | CONSTANT_MODULE
                | CONSTANT_PACKAGE => utf8(u2(o + 1)).is_some(),
                CONSTANT_NAME_AND_TYPE => utf8(u2(o + 1)).is_some() && utf8(u2(o + 3)).is_some(),
                t @ (CONSTANT_FIELDREF
                | CONSTANT_METHODREF
                | CONSTANT_INTERFACE_METHODREF
                | CONSTANT_INVOKE_DYNAMIC
                | CONSTANT_DYNAMIC) => {
                    let class_ok = t == CONSTANT_INVOKE_DYNAMIC
                        || t == CONSTANT_DYNAMIC
                        || matches!(
                            tag(u2(o + 1)),
                            CONSTANT_UTF8
                                | CONSTANT_CLASS
                                | CONSTANT_STRING
                                | CONSTANT_METHOD_TYPE
                                | CONSTANT_MODULE
                                | CONSTANT_PACKAGE
                        );
                    let name_and_type = u2(o + 3);
                    class_ok
                        && tag(name_and_type) == CONSTANT_NAME_AND_TYPE
                        && (t == CONSTANT_FIELDREF
                            || t == CONSTANT_DYNAMIC
                            || method_descriptor(name_and_type))
                }
                CONSTANT_METHOD_HANDLE => {
                    let kind = data[o + 1] as i32;
                    let field_kind = matches!(
                        kind,
                        CONSTANT_METHOD_HANDLE_REF_GET_FIELD
                            | CONSTANT_METHOD_HANDLE_REF_PUT_FIELD
                            | CONSTANT_METHOD_HANDLE_REF_GET_STATIC
                            | CONSTANT_METHOD_HANDLE_REF_PUT_STATIC
                    );
                    match name_and_type_of(u2(o + 2)) {
                        Some(n) => {
                            tag(n) == CONSTANT_NAME_AND_TYPE && (field_kind || method_descriptor(n))
                        }
                        None => false,
                    }
                }
                _ => true,
            };
            if !ok {
                return false;
            }
        }
        true
    }

    /// Reads and resolves pending entry `index` of a lazily read pool.
    fn read_pending(self, index: usize, pending: &Pending) -> PooledConstant {
        let mut in_ = pending.bytes.share_at(pending.at[index] as usize);
        let tag = in_.read_unsigned_byte() as i8;
        let mut constant: PooledConstant = match tag as i32 {
            CONSTANT_UTF8 => {
                let len = in_.read_unsigned_short() as usize;
                let text = decode_modified_utf8(&in_.read(len));
                PrimitiveConstant::new(CONSTANT_UTF8, ConstValue::String(text.into())).into()
            }
            CONSTANT_INTEGER => {
                PrimitiveConstant::new(CONSTANT_INTEGER, ConstValue::Integer(in_.read_int())).into()
            }
            CONSTANT_FLOAT => {
                PrimitiveConstant::new(CONSTANT_FLOAT, ConstValue::Float(in_.read_float())).into()
            }
            CONSTANT_LONG => {
                PrimitiveConstant::new(CONSTANT_LONG, ConstValue::Long(in_.read_long())).into()
            }
            CONSTANT_DOUBLE => {
                PrimitiveConstant::new(CONSTANT_DOUBLE, ConstValue::Double(in_.read_double()))
                    .into()
            }
            CONSTANT_CLASS | CONSTANT_STRING | CONSTANT_METHOD_TYPE | CONSTANT_MODULE
            | CONSTANT_PACKAGE => {
                PrimitiveConstant::new_index(tag as i32, in_.read_unsigned_short()).into()
            }
            CONSTANT_METHOD_HANDLE => {
                let index1 = in_.read_unsigned_byte();
                let index2 = in_.read_unsigned_short();
                LinkConstant::new_index(tag as i32, index1, index2).into()
            }
            _ => {
                let index1 = in_.read_unsigned_short();
                let index2 = in_.read_unsigned_short();
                LinkConstant::new_index(tag as i32, index1, index2).into()
            }
        };
        if !matches!(
            tag as i32,
            CONSTANT_UTF8 | CONSTANT_INTEGER | CONSTANT_FLOAT | CONSTANT_LONG | CONSTANT_DOUBLE
        ) {
            constant.resolve_constant(self);
        }
        self.with_mut(|d| d.pool[index] = Some(constant.clone()));
        constant
    }

    /// The Java constructor up to (not including) its read of the context's pool interceptor.
    pub fn new_without_interceptor(in_: &mut ByteCursor) -> ConstantPool {
        let size = in_.read_unsigned_short();
        let this = ConstantPool::alloc(ConstantPoolData {
            pool: Vec::with_capacity(size as usize),
            interceptor: None,
            pending: None,
        });
        let mut next_pass = [BitSet::new(), BitSet::new(), BitSet::new()];
        let mut pool: Vec<Option<PooledConstant>> = Vec::with_capacity(size as usize);

        pool.push(None);

        let mut i = 1;
        while i < size {
            let tag = in_.read_unsigned_byte() as i8;

            match tag as i32 {
                CONSTANT_UTF8 => {
                    let len = in_.read_unsigned_short() as usize;
                    let text = decode_modified_utf8(&in_.read(len));
                    pool.push(Some(
                        PrimitiveConstant::new(CONSTANT_UTF8, ConstValue::String(text.into()))
                            .into(),
                    ));
                }
                CONSTANT_INTEGER => {
                    let v = in_.read_int();
                    pool.push(Some(
                        PrimitiveConstant::new(CONSTANT_INTEGER, ConstValue::Integer(v)).into(),
                    ));
                }
                CONSTANT_FLOAT => {
                    let v = in_.read_float();
                    pool.push(Some(
                        PrimitiveConstant::new(CONSTANT_FLOAT, ConstValue::Float(v)).into(),
                    ));
                }
                CONSTANT_LONG => {
                    let v = in_.read_long();
                    pool.push(Some(
                        PrimitiveConstant::new(CONSTANT_LONG, ConstValue::Long(v)).into(),
                    ));
                    pool.push(None);
                    i += 1;
                }
                CONSTANT_DOUBLE => {
                    let v = in_.read_double();
                    pool.push(Some(
                        PrimitiveConstant::new(CONSTANT_DOUBLE, ConstValue::Double(v)).into(),
                    ));
                    pool.push(None);
                    i += 1;
                }
                CONSTANT_CLASS | CONSTANT_STRING | CONSTANT_METHOD_TYPE | CONSTANT_MODULE
                | CONSTANT_PACKAGE => {
                    let index = in_.read_unsigned_short();
                    pool.push(Some(PrimitiveConstant::new_index(tag as i32, index).into()));
                    next_pass[0].set(i as usize);
                }
                CONSTANT_NAME_AND_TYPE => {
                    let index1 = in_.read_unsigned_short();
                    let index2 = in_.read_unsigned_short();
                    pool.push(Some(
                        LinkConstant::new_index(tag as i32, index1, index2).into(),
                    ));
                    next_pass[0].set(i as usize);
                }
                CONSTANT_FIELDREF
                | CONSTANT_METHODREF
                | CONSTANT_INTERFACE_METHODREF
                | CONSTANT_INVOKE_DYNAMIC
                | CONSTANT_DYNAMIC => {
                    let index1 = in_.read_unsigned_short();
                    let index2 = in_.read_unsigned_short();
                    pool.push(Some(
                        LinkConstant::new_index(tag as i32, index1, index2).into(),
                    ));
                    next_pass[1].set(i as usize);
                }
                CONSTANT_METHOD_HANDLE => {
                    let index1 = in_.read_unsigned_byte();
                    let index2 = in_.read_unsigned_short();
                    pool.push(Some(
                        LinkConstant::new_index(tag as i32, index1, index2).into(),
                    ));
                    next_pass[2].set(i as usize);
                }
                _ => crate::jthrow!("Invalid Constant Pool entry #{} Type: {}", i, tag),
            }
            i += 1;
        }

        this.with_mut(|d| d.pool = pool);

        for pass in &next_pass {
            let mut idx: i32 = 0;
            loop {
                idx = pass.next_set_bit((idx + 1) as usize);
                if idx <= 0 {
                    break;
                }
                let mut constant = this.get_constant(idx).expect("NullPointerException");
                constant.resolve_constant(this);
                this.with_mut(|d| d.pool[idx as usize] = Some(constant));
            }
        }

        this
    }

    fn interceptor(self) -> Option<PoolInterceptor> {
        self.with(|d| d.interceptor)
    }

    pub fn get_class_element(
        self,
        element_type: i32,
        class_name: &str,
        name_index: i32,
        descriptor_index: i32,
    ) -> [String; 2] {
        let mut element_name = self
            .get_constant(name_index)
            .map(|c| c.as_primitive())
            .expect("NullPointerException")
            .get_string()
            .to_string();
        let mut descriptor = self
            .get_constant(descriptor_index)
            .map(|c| c.as_primitive())
            .expect("NullPointerException")
            .get_string()
            .to_string();

        if let Some(interceptor) = self.interceptor() {
            let mut class_name = class_name.to_string();
            if let Some(old_class_name) = interceptor.get_old_name(&class_name) {
                class_name = old_class_name;
            }

            let new_element =
                interceptor.get_name(&format!("{} {} {}", class_name, element_name, descriptor));
            if let Some(new_element) = new_element {
                element_name = second_token(&new_element);
            }

            if let Some(new_descriptor) =
                self.build_new_descriptor(element_type == ConstantPool::FIELD, &descriptor)
            {
                descriptor = new_descriptor;
            }
        }

        [element_name, descriptor]
    }

    #[track_caller]
    pub fn get_constant(self, index: i32) -> Option<PooledConstant> {
        let index = index as usize;
        let found = self.with(|d| match &d.pool[index] {
            Some(c) => Ok(Some(c.clone())),
            None => match &d.pending {
                Some(pending) if pending.at[index] != 0 => Err(pending.clone()),
                _ => Ok(None),
            },
        });
        match found {
            Ok(c) => c,
            Err(pending) => Some(self.read_pending(index, &pending)),
        }
    }

    #[track_caller]
    pub fn get_primitive_constant(self, index: i32) -> Option<PrimitiveConstant> {
        let mut cn = self.get_constant(index).map(|c| c.as_primitive());

        if let Some(c) = &cn {
            if self.interceptor().is_some() && c.type_ == CONSTANT_CLASS {
                if let Some(new_name) = self.build_new_classname(&c.get_string()) {
                    cn = Some(PrimitiveConstant::new(
                        CONSTANT_CLASS,
                        ConstValue::String(new_name.into()),
                    ));
                }
            }
        }

        cn
    }

    #[track_caller]
    pub fn get_link_constant(self, index: i32) -> Option<LinkConstant> {
        let mut ln = self.get_constant(index).map(|c| c.as_link());

        if let (Some(l), Some(interceptor)) = (&ln, self.interceptor()) {
            let t = l.type_;
            if t == CONSTANT_FIELDREF
                || t == CONSTANT_METHODREF
                || t == CONSTANT_INTERFACE_METHODREF
            {
                let ln_classname = l.classname.clone().expect("NullPointerException");
                let new_class_name = self.build_new_classname(&ln_classname);
                let new_element = interceptor.get_name(&format!(
                    "{} {} {}",
                    ln_classname, l.elementname, l.descriptor
                ));
                let new_descriptor =
                    self.build_new_descriptor(t == CONSTANT_FIELDREF, &l.descriptor);

                if new_class_name.is_some() || new_element.is_some() || new_descriptor.is_some() {
                    let class_name = new_class_name.unwrap_or(ln_classname);
                    let element_name = match new_element {
                        None => l.elementname.clone(),
                        Some(e) => second_token(&e),
                    };
                    let descriptor = new_descriptor.unwrap_or_else(|| l.descriptor.clone());
                    ln = Some(LinkConstant::new(
                        t,
                        Some(class_name),
                        element_name,
                        descriptor,
                    ));
                }
            }
        }

        ln
    }

    pub fn build_new_classname(self, class_name: &str) -> Option<String> {
        let vt = VarType::new_signature_cl(class_name, true);

        let new_name = self
            .interceptor()
            .expect("NullPointerException")
            .get_name(vt.value_or_null());
        if let Some(new_name) = new_name {
            let mut buffer = String::new();
            if vt.array_dim > 0 {
                for _ in 0..vt.array_dim {
                    buffer.push('[');
                }
                buffer.push('L');
                buffer.push_str(&new_name);
                buffer.push(';');
            } else {
                buffer.push_str(&new_name);
            }
            return Some(buffer);
        }

        None
    }

    /// kuna: the owners of this pool's `Methodref`s to `<init>`, the classes
    /// whose constructors the class calls, in pool order; a lazily read pool
    /// reads only those entries.
    pub fn constructor_owners(self) -> Vec<String> {
        let mut owners = Vec::new();
        let pending = self.with(|d| d.pending.clone());
        let Some(pending) = pending else {
            let len = self.with(|d| d.pool.len());
            for index in 1..len {
                let link = self.with(|d| match &d.pool[index] {
                    Some(PooledConstant::Link(link)) => Some(link.clone()),
                    _ => None,
                });
                if link.is_some_and(|l| l.type_ == CONSTANT_METHODREF && l.elementname == "<init>")
                {
                    if let Some(owner) = self
                        .get_link_constant(index as i32)
                        .and_then(|l| l.classname.clone())
                    {
                        owners.push(owner);
                    }
                }
            }
            return owners;
        };
        let data = pending.bytes.data();
        let at = &pending.at;
        let u2 = |o: usize| ((data[o] as usize) << 8) | data[o + 1] as usize;
        let tag = |i: usize| match at.get(i) {
            Some(&o) if o != 0 => data[o as usize] as i32,
            _ => 0,
        };
        for (index, &o) in at.iter().enumerate() {
            if o == 0 || tag(index) != CONSTANT_METHODREF {
                continue;
            }
            let o = o as usize;
            let name_and_type = u2(o + 3);
            if tag(name_and_type) != CONSTANT_NAME_AND_TYPE {
                continue;
            }
            let name = u2(at[name_and_type] as usize + 1);
            if tag(name) != CONSTANT_UTF8 {
                continue;
            }
            let n = at[name] as usize;
            if &data[n + 3..n + 3 + u2(n + 1)] != b"<init>" {
                continue;
            }
            if let Some(owner) = self.get_primitive_constant(u2(o + 1) as i32) {
                owners.push(owner.get_string().to_string());
            }
        }
        owners
    }

    /// kuna: the class, name and descriptor of every method reference in this
    /// pool (fix `unseen_class_overload`); a lazily read pool resolves none of
    /// its entries for it.
    pub fn method_refs(self) -> Vec<(String, String, String)> {
        let pending = self.with(|d| d.pending.clone()).filter(|_| self.interceptor().is_none());
        let Some(pending) = pending else {
            let len = self.with(|d| d.pool.len());
            return (1..len)
                .filter_map(|index| {
                    let link = self.get_constant(index as i32)?.try_as_link()?;
                    (link.type_ == CONSTANT_METHODREF || link.type_ == CONSTANT_INTERFACE_METHODREF)
                        .then_some(())?;
                    Some((
                        link.classname.clone()?,
                        link.elementname.clone(),
                        link.descriptor.clone(),
                    ))
                })
                .collect();
        };
        let data = pending.bytes.data();
        let at = &pending.at;
        let u2 = |o: usize| ((data[o] as usize) << 8) | data[o + 1] as usize;
        let tag = |i: usize| match at.get(i) {
            Some(&o) if o != 0 => data[o as usize] as i32,
            _ => 0,
        };
        let utf8 = |i: usize| -> Option<String> {
            (tag(i) == CONSTANT_UTF8).then(|| {
                let o = at[i] as usize;
                decode_modified_utf8(&data[o + 3..o + 3 + u2(o + 1)])
            })
        };
        let mut refs = Vec::new();
        for (index, &o) in at.iter().enumerate() {
            let t = tag(index);
            if o == 0 || (t != CONSTANT_METHODREF && t != CONSTANT_INTERFACE_METHODREF) {
                continue;
            }
            let o = o as usize;
            let (class, name_and_type) = (u2(o + 1), u2(o + 3));
            if tag(class) != CONSTANT_CLASS || tag(name_and_type) != CONSTANT_NAME_AND_TYPE {
                continue;
            }
            let nt = at[name_and_type] as usize;
            if let (Some(owner), Some(name), Some(descriptor)) =
                (utf8(u2(at[class] as usize + 1)), utf8(u2(nt + 1)), utf8(u2(nt + 3)))
            {
                refs.push((owner, name, descriptor));
            }
        }
        refs
    }

    pub fn get_pool(self) -> Vec<Option<PooledConstant>> {
        let len = self.with(|d| d.pool.len());
        for index in 0..len {
            self.get_constant(index as i32);
        }
        self.with(|d| d.pool.clone())
    }

    fn build_new_descriptor(self, is_field: bool, descriptor: &str) -> Option<String> {
        if is_field {
            FieldDescriptor::parse_descriptor(descriptor).build_new_descriptor(&self)
        } else {
            MethodDescriptor::parse_descriptor(descriptor).build_new_descriptor(&self)
        }
    }
}

impl NewClassNameBuilder for ConstantPool {
    fn build_new_classname(&self, class_name: &str) -> Option<String> {
        ConstantPool::build_new_classname(*self, class_name)
    }
}
