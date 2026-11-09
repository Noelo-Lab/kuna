//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ExitExprent.java`.
//!
//! kuna fix `lambda_type_variable_target`: a returned call is told the
//! declared type it is returned as (`kuna_fix_lambda_type_variable_target`).

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::{Exprent, ExprentKind};
use crate::i_matchable::MatchProperties;
use crate::kuna_fix_lambda_type_variable_target;
use crate::kuna_fixes::{self, KunaFix};
use crate::java_subclass;
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_type::VarType;

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
        let mut result = CheckTypesResult::new();

        let (exit_type, value, ret_type) = self.d(|d| (d.exit_type, d.value, d.ret_type.clone()));
        if exit_type == Type::Return {
            let ret_type = ret_type.expect("NullPointerException");
            if ret_type.type_ != CodeType::Void {
                let value = value.expect("NullPointerException");
                result.add_expr_lower_bound(
                    value,
                    &VarType::find_family_bottom(ret_type.type_family),
                );
                result.add_expr_upper_bound(value, &ret_type);
            }
        }

        Some(result)
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            if let Some(value) = d.value {
                lst.push(value);
            }
        });
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        buf.add_bytecode_mapping(self.get_bytecode().as_ref());

        let (exit_type, value, ret_type, method_descriptor) = self.d(|d| {
            (
                d.exit_type,
                d.value,
                d.ret_type.clone(),
                d.method_descriptor.clone(),
            )
        });

        if exit_type == Type::Return {
            buf.append("return");

            let ret_type = ret_type.expect("NullPointerException");
            if ret_type.type_ != CodeType::Void {
                let mut ret = ret_type;
                let mut generic = false;
                if let Some(generic_ret) = method_descriptor
                    .as_ref()
                    .and_then(|md| md.generic_info())
                    .and_then(|gi| gi.return_type())
                {
                    ret = generic_ret;
                    generic = true;
                }
                buf.append_char(' ');

                if kuna_fixes::enabled(KunaFix::LambdaTypeVariableTarget) {
                    if let Some(inv) = value.and_then(|v| v.try_as_invocation()) {
                        let target = kuna_fix_lambda_type_variable_target::return_target(&ret, generic);
                        inv.d_mut(|d| d.kuna_target = target);
                    }
                }

                ExprProcessor::get_casted_exprent_full(
                    value.expect("NullPointerException"),
                    &ret,
                    &mut buf,
                    indent,
                    NullCastType::DontCastAtAll,
                    false,
                    false,
                    false,
                );
            }

            buf
        } else {
            let method = DecompilerContext::get_current_method_wrapper();
            let node = DecompilerContext::get_current_class_node();

            if let (Some(method), Some(node)) = (method, node) {
                let method_struct = method.with(|d| d.method_struct);
                let attr =
                    method_struct.get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS);

                if let Some(attr) = attr {
                    let mut classname: Option<String> = None;

                    let class_struct = node.with(|d| d.class_struct);
                    for i in 0..attr.get_throws_exceptions().len() {
                        let ex_class_name =
                            attr.get_exc_classname(i as i32, class_struct.get_pool());
                        if ex_class_name == "java/lang/Throwable" {
                            classname = Some(ex_class_name);
                            break;
                        } else if ex_class_name == "java/lang/Exception" {
                            classname = Some(ex_class_name);
                        }
                    }

                    if let Some(classname) = classname {
                        let ex_type = VarType::new_signature_cl(&classname, true);
                        buf.append("throw ");
                        ExprProcessor::get_casted_exprent(
                            value.expect("NullPointerException"),
                            &ex_type,
                            &mut buf,
                            indent,
                            false,
                        );
                        return buf;
                    }
                }
            }

            buf.append_buffer(&value.expect("NullPointerException").to_java(indent))
                .prepend("throw ");
            buf
        }
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
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        match match_node.get_rule_value(MatchProperties::ExprentExittype) {
            None => true,
            Some(MatchValue::ExitType(type_)) => self.get_exit_type() == type_,
            Some(_) => crate::rt::throw(
                "ClassCastException",
                "rule value is not an ExitExprent.Type",
            ),
        }
    }
}
