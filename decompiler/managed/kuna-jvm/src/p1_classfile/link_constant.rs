//! Port of `org/jetbrains/java/decompiler/struct/consts/LinkConstant.java`.
//!
//! `elementname` and `descriptor` are never null once the pool is resolved; an
//! unresolved Java null is the empty string here.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_constants::{
    CONSTANT_DYNAMIC, CONSTANT_INTERFACE_METHODREF, CONSTANT_INVOKE_DYNAMIC, CONSTANT_METHODREF,
    CONSTANT_METHOD_HANDLE, CONSTANT_METHOD_HANDLE_REF_GET_FIELD,
    CONSTANT_METHOD_HANDLE_REF_GET_STATIC, CONSTANT_METHOD_HANDLE_REF_PUT_FIELD,
    CONSTANT_METHOD_HANDLE_REF_PUT_STATIC, CONSTANT_NAME_AND_TYPE,
};
use crate::constant_pool::ConstantPool;
use crate::rt::JavaEq;

#[derive(Clone)]
pub struct LinkConstant(Rc<LinkConstantData>);

#[derive(Clone)]
pub struct LinkConstantData {
    pub type_: i32,
    pub index1: i32,
    pub index2: i32,
    pub classname: Option<String>,
    pub elementname: String,
    pub descriptor: String,
}

impl Deref for LinkConstant {
    type Target = LinkConstantData;
    fn deref(&self) -> &LinkConstantData {
        &self.0
    }
}

impl LinkConstant {
    /// Java: `LinkConstant(int, String, String, String)`
    pub fn new(
        type_: i32,
        classname: Option<String>,
        elementname: String,
        descriptor: String,
    ) -> LinkConstant {
        let c = LinkConstant(Rc::new(LinkConstantData {
            type_,
            index1: 0,
            index2: 0,
            classname,
            elementname,
            descriptor,
        }));
        c.init_constant();
        c
    }

    /// Java: `LinkConstant(int, int, int)`
    pub fn new_index(type_: i32, index1: i32, index2: i32) -> LinkConstant {
        LinkConstant(Rc::new(LinkConstantData {
            type_,
            index1,
            index2,
            classname: None,
            elementname: String::new(),
            descriptor: String::new(),
        }))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &LinkConstant) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    fn init_constant(&self) {
        let t = self.type_;
        let i1 = self.index1;
        if t == CONSTANT_METHODREF
            || t == CONSTANT_INTERFACE_METHODREF
            || t == CONSTANT_INVOKE_DYNAMIC
            || (t == CONSTANT_METHOD_HANDLE
                && i1 != CONSTANT_METHOD_HANDLE_REF_GET_FIELD
                && i1 != CONSTANT_METHOD_HANDLE_REF_PUT_FIELD
                && i1 != CONSTANT_METHOD_HANDLE_REF_GET_STATIC
                && i1 != CONSTANT_METHOD_HANDLE_REF_PUT_STATIC)
        {
            let d = &self.descriptor;
            let parenth = d.find(')');
            if d.encode_utf16().count() < 2 || parenth.is_none() || !d.starts_with('(') {
                crate::jthrow!(
                    IllegalArgumentException;
                    "Invalid descriptor: {}; type = {}; classname = {}; elementname = {}; kind = {}",
                    d,
                    t,
                    self.classname.as_deref().unwrap_or("null"),
                    self.elementname,
                    i1
                );
            }
        }
    }

    pub fn resolve_constant(&mut self, pool: ConstantPool) {
        let t = self.type_;
        if t == CONSTANT_NAME_AND_TYPE {
            let elementname = pool
                .get_primitive_constant(self.index1)
                .expect("NullPointerException")
                .get_string_opt();
            let descriptor = pool
                .get_primitive_constant(self.index2)
                .expect("NullPointerException")
                .get_string_opt();
            let d = Rc::make_mut(&mut self.0);
            d.elementname = elementname.map(|s| s.to_string()).unwrap_or_default();
            d.descriptor = descriptor.map(|s| s.to_string()).unwrap_or_default();
        } else if t == CONSTANT_METHOD_HANDLE {
            let ref_info = pool
                .get_link_constant(self.index2)
                .expect("NullPointerException");

            let d = Rc::make_mut(&mut self.0);
            d.classname = ref_info.classname.clone();
            d.elementname = ref_info.elementname.clone();
            d.descriptor = ref_info.descriptor.clone();
        } else {
            if t != CONSTANT_INVOKE_DYNAMIC && t != CONSTANT_DYNAMIC {
                let classname = pool
                    .get_primitive_constant(self.index1)
                    .expect("NullPointerException")
                    .get_string_opt();
                Rc::make_mut(&mut self.0).classname = classname.map(|s| s.to_string());
            }

            let nametype = pool
                .get_link_constant(self.index2)
                .expect("NullPointerException");
            let d = Rc::make_mut(&mut self.0);
            d.elementname = nametype.elementname.clone();
            d.descriptor = nametype.descriptor.clone();
        }

        self.init_constant();
    }

    pub fn equals(&self, cn: &LinkConstant) -> bool {
        if cn.ptr_eq(self) {
            return true;
        }
        self.type_ == cn.type_
            && self.elementname == cn.elementname
            && self.descriptor == cn.descriptor
            && (self.type_ != CONSTANT_NAME_AND_TYPE
                || match &self.classname {
                    Some(c) => cn.classname.as_deref() == Some(c.as_str()),
                    None => crate::rt::throw("NullPointerException", "classname is null"),
                })
    }
}

impl std::fmt::Debug for LinkConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LinkConstant(type={}, {:?} {} {})",
            self.type_, self.classname, self.elementname, self.descriptor
        )
    }
}

impl JavaEq for LinkConstant {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
