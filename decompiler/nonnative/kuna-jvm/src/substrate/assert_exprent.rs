//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/AssertExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentKind};
use crate::java_subclass;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;

pub struct AssertExprentData {
    /// `[condition, message?]`; the condition is `None` for `assert false`.
    pub parameters: Vec<Option<Exprent>>,
}

java_subclass!(pub AssertExprent: Exprent, ExprentKind::Assert(AssertExprentData),
               is_assert, as_assert, try_as_assert);

impl JavaEq for AssertExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for AssertExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl AssertExprent {
    pub fn new(parameters: Vec<Option<Exprent>>) -> AssertExprent {
        let id = Exprent::next_id();
        AssertExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Assert(AssertExprentData { parameters }),
        ))
    }

    pub fn get_all_exprents_list(self, list: &mut Vec<Exprent>) {
        self.d(|d| list.extend(d.parameters.iter().flatten().copied()));
    }

    pub fn copy(self) -> Exprent {
        let parameters = self.d(|d| d.parameters.clone());
        AssertExprent::new(parameters).0
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let parameters = self.d(|d| d.parameters.clone());
        let mut buffer = TextBuffer::new();

        buffer.append("assert ");

        buffer.add_bytecode_mapping(self.get_bytecode().as_ref());

        match parameters[0] {
            None => buffer.append("false"),
            Some(condition) => buffer.append_buffer(&condition.to_java(indent)),
        };

        if parameters.len() > 1 {
            buffer.append(" : ");
            buffer.append_buffer(&parameters[1].expect("NullPointerException").to_java(indent));
        }

        buffer
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            for p in d.parameters.iter_mut() {
                if Some(old_expr) == *p {
                    *p = Some(new_expr);
                }
            }
        });
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        Exprent::check_expr_type_bounds_base(self.0)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let parameters = self.d(|d| d.parameters.clone());
        if !parameters.is_empty() {
            for e in parameters {
                e.expect("NullPointerException: null assert parameter")
                    .get_bytecode_range(values.as_deref_mut());
            }
        }
        self.measure_bytecode(values);
    }
}
