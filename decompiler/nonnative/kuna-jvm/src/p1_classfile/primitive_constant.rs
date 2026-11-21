//! Port of `org/jetbrains/java/decompiler/struct/consts/PrimitiveConstant.java`.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_constants::{
    CONSTANT_CLASS, CONSTANT_METHOD_TYPE, CONSTANT_MODULE, CONSTANT_PACKAGE, CONSTANT_STRING,
};
use crate::constant_pool::ConstantPool;
use crate::rt::{JavaEq, JavaStr};

/// The boxed Java value a constant holds (`Integer`, `Long`, `Float`, `Double`, `String`).
#[derive(Clone, Debug)]
pub enum ConstValue {
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(Rc<str>),
}

impl ConstValue {
    /// `Object.equals` between two boxed values.
    pub fn equals(&self, other: &ConstValue) -> bool {
        match (self, other) {
            (ConstValue::Integer(a), ConstValue::Integer(b)) => a == b,
            (ConstValue::Long(a), ConstValue::Long(b)) => a == b,
            (ConstValue::Float(a), ConstValue::Float(b)) => {
                float_to_int_bits(*a) == float_to_int_bits(*b)
            }
            (ConstValue::Double(a), ConstValue::Double(b)) => {
                double_to_long_bits(*a) == double_to_long_bits(*b)
            }
            (ConstValue::String(a), ConstValue::String(b)) => a == b,
            _ => false,
        }
    }

    /// `Object.hashCode`.
    pub fn hash_code(&self) -> i32 {
        match self {
            ConstValue::Integer(v) => *v,
            ConstValue::Long(v) => (*v ^ ((*v as u64) >> 32) as i64) as i32,
            ConstValue::Float(v) => float_to_int_bits(*v),
            ConstValue::Double(v) => {
                let bits = double_to_long_bits(*v);
                (bits ^ ((bits as u64) >> 32) as i64) as i32
            }
            ConstValue::String(s) => crate::rt::jstring::java_string_hash(s),
        }
    }

    /// `Object.toString`.
    pub fn to_java_string(&self) -> String {
        match self {
            ConstValue::Integer(v) => v.to_string(),
            ConstValue::Long(v) => v.to_string(),
            ConstValue::Float(v) => crate::rt::jnum::float_to_string(*v),
            ConstValue::Double(v) => crate::rt::jnum::double_to_string(*v),
            ConstValue::String(s) => s.to_string(),
        }
    }
}

/// `Float.floatToIntBits` (NaN canonicalized).
pub fn float_to_int_bits(v: f32) -> i32 {
    if v.is_nan() {
        0x7fc0_0000
    } else {
        v.to_bits() as i32
    }
}

/// `Double.doubleToLongBits` (NaN canonicalized).
pub fn double_to_long_bits(v: f64) -> i64 {
    if v.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        v.to_bits() as i64
    }
}

impl JavaEq for ConstValue {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl crate::rt::JavaHash for ConstValue {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

#[derive(Clone)]
pub struct PrimitiveConstant(Rc<PrimitiveConstantData>);

#[derive(Clone)]
pub struct PrimitiveConstantData {
    pub type_: i32,
    pub index: i32,
    pub value: Option<ConstValue>,
    pub is_array: bool,
}

impl Deref for PrimitiveConstant {
    type Target = PrimitiveConstantData;
    fn deref(&self) -> &PrimitiveConstantData {
        &self.0
    }
}

impl PrimitiveConstant {
    /// Java: `PrimitiveConstant(int, Object)`
    pub fn new(type_: i32, value: ConstValue) -> PrimitiveConstant {
        let mut c = PrimitiveConstant(Rc::new(PrimitiveConstantData {
            type_,
            index: 0,
            value: Some(value),
            is_array: false,
        }));
        c.init_constant();
        c
    }

    /// Java: `PrimitiveConstant(int, int)`
    pub fn new_index(type_: i32, index: i32) -> PrimitiveConstant {
        PrimitiveConstant(Rc::new(PrimitiveConstantData {
            type_,
            index,
            value: None,
            is_array: false,
        }))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &PrimitiveConstant) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    fn init_constant(&mut self) {
        if self.type_ == CONSTANT_CLASS {
            let class_name = self.get_string();
            let is_array = class_name.jlen() > 0 && class_name.jchar_at(0) == b'[' as u16;
            Rc::make_mut(&mut self.0).is_array = is_array;
        }
    }

    /// `(String) value`, which is null for an unresolved index constant.
    #[track_caller]
    pub fn get_string_opt(&self) -> Option<Rc<str>> {
        match &self.value {
            None => None,
            Some(ConstValue::String(s)) => Some(s.clone()),
            Some(_) => crate::rt::throw("ClassCastException", "constant value is not a String"),
        }
    }

    /// `getString()` where the caller dereferences the result.
    #[track_caller]
    pub fn get_string(&self) -> Rc<str> {
        match self.get_string_opt() {
            Some(s) => s,
            None => crate::rt::throw("NullPointerException", "constant string is null"),
        }
    }

    pub fn resolve_constant(&mut self, pool: ConstantPool) {
        let t = self.type_;
        if t == CONSTANT_CLASS
            || t == CONSTANT_STRING
            || t == CONSTANT_METHOD_TYPE
            || t == CONSTANT_MODULE
            || t == CONSTANT_PACKAGE
        {
            let value = pool
                .get_primitive_constant(self.index)
                .expect("NullPointerException")
                .get_string_opt();
            Rc::make_mut(&mut self.0).value = value.map(ConstValue::String);
            self.init_constant();
        }
    }

    pub fn equals(&self, cn: &PrimitiveConstant) -> bool {
        if cn.ptr_eq(self) {
            return true;
        }
        self.type_ == cn.type_
            && self.is_array == cn.is_array
            && match (&self.value, &cn.value) {
                (Some(a), Some(b)) => a.equals(b),
                (Some(_), None) => false,
                (None, _) => crate::rt::throw("NullPointerException", "constant value is null"),
            }
    }
}

impl std::fmt::Display for PrimitiveConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Primitive({}{} {})",
            if self.is_array { "Ary " } else { "" },
            self.index,
            self.value
                .as_ref()
                .map_or_else(|| "null".to_string(), ConstValue::to_java_string)
        )
    }
}

impl std::fmt::Debug for PrimitiveConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PrimitiveConstant(type={}, index={}, value={:?})",
            self.type_, self.index, self.value
        )
    }
}

impl JavaEq for PrimitiveConstant {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
