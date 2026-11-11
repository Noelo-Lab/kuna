//! Port of `org/vineflower/kotlin/struct/KParameter.java`.
//!
//! An immutable record compared both by identity (`KContract.KExpression`
//! tests `== THIS_TYPE`) and by value (a `DefaultArgsMap` key), so it is an
//! `Rc`-backed value: `equals` is the record's, `ptr_eq` is Java `==`.
//!
//! kuna fix `kt_vararg_from_metadata`: in `stringify`, a parameter is
//! `vararg` when it has a vararg element type, whatever its array type;
//! Vineflower also requires `kotlin/Array`, so `vararg n: Int` printed as
//! `n: IntArray`.

use std::ops::Deref;
use std::rc::Rc;

use crate::k_type::KType;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_flags::Flags;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::jstring::java_string_hash;
use crate::rt::{JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;

pub struct KParameterData {
    pub flags: i32,
    pub name: String,
    pub type_: KType,
    pub vararg_type: Option<KType>,
    pub type_id: i32,
}

#[derive(Clone)]
pub struct KParameter(Rc<KParameterData>);

impl Deref for KParameter {
    type Target = KParameterData;
    fn deref(&self) -> &KParameterData {
        &self.0
    }
}

impl KParameter {
    pub fn new(
        flags: i32,
        name: String,
        type_: KType,
        vararg_type: Option<KType>,
        type_id: i32,
    ) -> KParameter {
        KParameter(Rc::new(KParameterData {
            flags,
            name,
            type_,
            vararg_type,
            type_id,
        }))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &KParameter) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// The record's `equals`.
    pub fn equals(&self, o: &KParameter) -> bool {
        self.flags == o.flags
            && self.name == o.name
            && self.type_.equals(&o.type_)
            && match (&self.vararg_type, &o.vararg_type) {
                (None, None) => true,
                (Some(a), Some(b)) => a.equals(b),
                _ => false,
            }
            && self.type_id == o.type_id
    }

    /// The record's `hashCode`.
    pub fn hash_code(&self) -> i32 {
        let mut result: i32 = 0;
        result = result.wrapping_mul(31).wrapping_add(self.flags);
        result = result
            .wrapping_mul(31)
            .wrapping_add(java_string_hash(&self.name));
        result = result.wrapping_mul(31).wrapping_add(self.type_.hash_code());
        result = result
            .wrapping_mul(31)
            .wrapping_add(self.vararg_type.as_ref().map_or(0, |t| t.hash_code()));
        result = result.wrapping_mul(31).wrapping_add(self.type_id);
        result
    }

    pub fn stringify(&self, indent: i32, buf: &mut TextBuffer) {
        if Flags::IS_CROSSINLINE.get(self.flags) {
            buf.append("crossinline ");
        }

        if Flags::IS_NOINLINE.get(self.flags) {
            buf.append("noinline ");
        }

        let is_vararg = if kuna_fixes::enabled(KunaFix::KtVarargFromMetadata) {
            self.vararg_type.is_some()
        } else {
            self.vararg_type.is_some() && self.type_.k().kotlin_type == "kotlin/Array"
        };

        if is_vararg {
            buf.append("vararg ");
        }

        buf.append(&KotlinWriter::to_valid_kotlin_identifier(&self.name))
            .append(": ");

        let type_ = match &self.vararg_type {
            Some(vararg_type) if is_vararg => vararg_type,
            _ => &self.type_,
        };
        buf.append_buffer(&type_.stringify(indent + 1));
    }
}

impl JavaHash for KParameter {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for KParameter {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
