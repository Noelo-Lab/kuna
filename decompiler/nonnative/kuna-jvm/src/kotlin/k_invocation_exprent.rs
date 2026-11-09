//! Port of `org/vineflower/kotlin/expr/KInvocationExprent.java`.

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_property::PropertyMethod;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

pub struct KInvocationExprentData {
    pub calling_data: Option<KElement>,
    pub property_method: Option<PropertyMethod>,
}

kotlin_exprent!(pub KInvocationExprent: InvocationExprent, Invocation(KInvocationExprentData),
                is_k_invocation, as_k_invocation, try_as_k_invocation);

impl KInvocationExprent {
    pub fn get_precedence(self) -> i32 {
        unported!("KInvocationExprent.getPrecedence")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KInvocationExprent.toJava")
    }

    pub fn copy(self) -> Exprent {
        unported!("KInvocationExprent.copy")
    }
}
