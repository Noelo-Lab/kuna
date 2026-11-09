//! Port of `org/jetbrains/java/decompiler/struct/consts/ConstantPool.java`.

use crate::byte_cursor::ByteCursor;
use crate::code_constants::{
    CONSTANT_CLASS, CONSTANT_DOUBLE, CONSTANT_DYNAMIC, CONSTANT_FIELDREF, CONSTANT_FLOAT,
    CONSTANT_INTEGER, CONSTANT_INTERFACE_METHODREF, CONSTANT_INVOKE_DYNAMIC, CONSTANT_LONG,
    CONSTANT_METHODREF, CONSTANT_METHOD_HANDLE, CONSTANT_METHOD_TYPE, CONSTANT_MODULE,
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
        let this = ConstantPool::new_without_interceptor(in_);
        let interceptor = DecompilerContext::get_pool_interceptor();
        this.with_mut(|d| d.interceptor = interceptor);
        this
    }

    /// The Java constructor up to (not including) its read of the context's pool interceptor.
    pub fn new_without_interceptor(in_: &mut ByteCursor) -> ConstantPool {
        let size = in_.read_unsigned_short();
        let this = ConstantPool::alloc(ConstantPoolData {
            pool: Vec::with_capacity(size as usize),
            interceptor: None,
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
        self.with(|d| d.pool[index as usize].clone())
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

    pub fn get_pool(self) -> Vec<Option<PooledConstant>> {
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
