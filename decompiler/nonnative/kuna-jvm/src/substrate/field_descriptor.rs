//! Port of `org/jetbrains/java/decompiler/struct/gen/FieldDescriptor.java`.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::new_class_name_builder::NewClassNameBuilder;
use crate::rt::{JavaEq, JavaHash};
use crate::var_type::VarType;

#[derive(Clone)]
pub struct FieldDescriptor(Rc<FieldDescriptorData>);

pub struct FieldDescriptorData {
    pub type_: VarType,
    pub descriptor_string: String,
}

impl Deref for FieldDescriptor {
    type Target = FieldDescriptorData;
    fn deref(&self) -> &FieldDescriptorData {
        &self.0
    }
}

struct Constants {
    integer: FieldDescriptor,
    long: FieldDescriptor,
    float: FieldDescriptor,
    double: FieldDescriptor,
}

thread_local! {
    static CONSTANTS: Constants = Constants {
        integer: FieldDescriptor::parse_descriptor("Ljava/lang/Integer;"),
        long: FieldDescriptor::parse_descriptor("Ljava/lang/Long;"),
        float: FieldDescriptor::parse_descriptor("Ljava/lang/Float;"),
        double: FieldDescriptor::parse_descriptor("Ljava/lang/Double;"),
    };
}

impl FieldDescriptor {
    #[allow(non_snake_case)]
    pub fn INTEGER_DESCRIPTOR() -> FieldDescriptor {
        CONSTANTS.with(|c| c.integer.clone())
    }

    #[allow(non_snake_case)]
    pub fn LONG_DESCRIPTOR() -> FieldDescriptor {
        CONSTANTS.with(|c| c.long.clone())
    }

    #[allow(non_snake_case)]
    pub fn FLOAT_DESCRIPTOR() -> FieldDescriptor {
        CONSTANTS.with(|c| c.float.clone())
    }

    #[allow(non_snake_case)]
    pub fn DOUBLE_DESCRIPTOR() -> FieldDescriptor {
        CONSTANTS.with(|c| c.double.clone())
    }

    pub fn parse_descriptor(descriptor: &str) -> FieldDescriptor {
        FieldDescriptor(Rc::new(FieldDescriptorData {
            type_: VarType::new_signature(descriptor),
            descriptor_string: descriptor.to_string(),
        }))
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &FieldDescriptor) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    pub fn build_new_descriptor(&self, builder: &dyn NewClassNameBuilder) -> Option<String> {
        if self.type_.type_ == CodeType::Object {
            if let Some(new_class_name) = builder.build_new_classname(self.type_.value_str()) {
                return Some(
                    VarType::new_value(self.type_.type_, self.type_.array_dim, new_class_name)
                        .to_string(),
                );
            }
        }
        None
    }

    pub fn equals(&self, o: &FieldDescriptor) -> bool {
        if o.ptr_eq(self) {
            return true;
        }
        self.type_.equals(&o.type_)
    }

    pub fn hash_code(&self) -> i32 {
        self.type_.hash_code()
    }
}

impl JavaHash for FieldDescriptor {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for FieldDescriptor {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl std::fmt::Debug for FieldDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldDescriptor({})", self.descriptor_string)
    }
}
