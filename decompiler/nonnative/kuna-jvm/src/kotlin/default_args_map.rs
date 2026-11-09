//! Port of `org/vineflower/kotlin/struct/DefaultArgsMap.java`.

#![allow(dead_code, unused_variables)]

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::exprent::Exprent;
use crate::k_parameter::KParameter;
use crate::method_wrapper::MethodWrapper;
use crate::rt::JHashMap;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::unported;

/// `Function<ClassWrapper, DefaultArgsMap>`: built at write time, in the class scope.
pub type DefaultArgsSupplier = Rc<dyn Fn(ClassWrapper) -> DefaultArgsMap>;

pub struct DefaultArgsMap {
    map: JHashMap<KParameter, Exprent>,
    default_method: Option<StructMethod>,
}

impl DefaultArgsMap {
    fn new(
        map: JHashMap<KParameter, Exprent>,
        default_method: Option<StructMethod>,
    ) -> DefaultArgsMap {
        DefaultArgsMap {
            map,
            default_method,
        }
    }

    /// `calling` is only read when `defaults` is present.
    pub fn from(
        defaults: Option<MethodWrapper>,
        calling: Option<MethodWrapper>,
        params: &[KParameter],
    ) -> DefaultArgsMap {
        unported!("DefaultArgsMap.from")
    }

    pub fn from_annotation(cl: StructClass) -> DefaultArgsMap {
        unported!("DefaultArgsMap.fromAnnotation")
    }

    pub fn to_java(&self, parameter: &KParameter, indent: i32) -> TextBuffer {
        unported!("DefaultArgsMap.toJava")
    }

    fn update_exprent(expr: Exprent, calling: MethodWrapper) {
        unported!("DefaultArgsMap.updateExprent")
    }

    pub fn get_default_method(&self) -> Option<StructMethod> {
        self.default_method
    }
}
