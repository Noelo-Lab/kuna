//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ExitExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentKind};
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

/// Java `ExitExprent.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Return,
    Throw,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::Return => "RETURN",
            Type::Throw => "THROW",
        }
    }
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct ExitExprentData {
    pub exit_type: Type,
    /// `None` for a bare `return;`.
    pub value: Option<Exprent>,
    /// `None` for a `throw`.
    pub ret_type: Option<VarType>,
    pub method_descriptor: Option<MethodDescriptor>,
}

java_subclass!(pub ExitExprent: Exprent, ExprentKind::Exit(ExitExprentData),
               is_exit, as_exit, try_as_exit);

impl JavaEq for ExitExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for ExitExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl ExitExprent {
    pub fn new(
        exit_type: Type,
        value: Option<Exprent>,
        ret_type: Option<VarType>,
        bytecode_offsets: Option<&BitSet>,
        method_descriptor: Option<MethodDescriptor>,
    ) -> ExitExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Exit(ExitExprentData {
                exit_type,
                value,
                ret_type,
                method_descriptor,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        let exit = ExitExprent(e);
        ValidationHelper::validate_exit_exprent(exit);
        exit
    }

    pub fn copy(self) -> Exprent {
        let (exit_type, value, ret_type, method_descriptor) = self.d(|d| {
            (
                d.exit_type,
                d.value,
                d.ret_type.clone(),
                d.method_descriptor.clone(),
            )
        });
        let bytecode = self.get_bytecode();
        ExitExprent::new(
            exit_type,
            value.map(|v| v.copy()),
            ret_type,
            bytecode.as_ref(),
            method_descriptor,
        )
        .0
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("ExitExprent.checkExprTypeBounds")
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            if let Some(value) = d.value {
                lst.push(value);
            }
        });
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("ExitExprent.toJava")
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if Some(old_expr) == d.value {
                d.value = Some(new_expr);
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(et) = o.try_as_exit() else {
            return false;
        };
        let (exit_type, value) = self.d(|d| (d.exit_type, d.value));
        exit_type == et.get_exit_type() && value.java_equals(&et.get_value())
    }

    pub fn get_exit_type(self) -> Type {
        self.d(|d| d.exit_type)
    }

    pub fn get_value(self) -> Option<Exprent> {
        self.d(|d| d.value)
    }

    pub fn get_ret_type(self) -> Option<VarType> {
        self.d(|d| d.ret_type.clone())
    }

    pub fn get_method_descriptor(self) -> Option<MethodDescriptor> {
        self.d(|d| d.method_descriptor.clone())
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let value = self.get_value();
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), value);
        self.measure_bytecode(values);
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, _match_node: &MatchNode, _engine: &MatchEngine) -> bool {
        unported!("ExitExprent.match")
    }
}
