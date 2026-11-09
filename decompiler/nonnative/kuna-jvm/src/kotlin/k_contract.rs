//! Port of `org/vineflower/kotlin/struct/KContract.java`.

use crate::decompiler_context::DecompilerContext;
use crate::jthrow;
use crate::k_parameter::KParameter;
use crate::k_type::KType;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_flags::Flags;
use crate::kt_proto::{self, ConstantValue, EffectType, InvocationKind};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::text_buffer::TextBuffer;

#[derive(Clone)]
pub struct KContract {
    pub effects: Vec<KEffect>,
}

/// `KContract.KEffect`.
#[derive(Clone)]
pub struct KEffect {
    pub type_: Option<EffectType>,
    pub expressions: Vec<KExpression>,
    pub conditional_conclusion: Option<KExpression>,
    pub kind: Option<InvocationKind>,
}

/// `KContract.KExpression`.
#[derive(Clone)]
pub struct KExpression {
    pub flags: i32,
    pub value_parameter_reference: Option<KParameter>,
    pub constant_value: Option<ConstantValue>,
    pub instanceof_type: Option<KType>,
    pub and_arguments: Vec<KExpression>,
    pub or_arguments: Vec<KExpression>,
}

thread_local! {
    static THIS_TYPE: KParameter = KParameter::new(0, "this".to_string(), KType::NOTHING(), None, 0);
}

impl KContract {
    const INVOCATION_KIND: &'static str = "kotlin.contracts.InvocationKind";

    fn new(effects: Vec<KEffect>) -> KContract {
        KContract { effects }
    }

    pub fn from(
        proto: &kt_proto::Contract,
        params: &[KParameter],
        resolver: &MetadataNameResolver,
        type_table: &kt_proto::TypeTable,
    ) -> KContract {
        KContract::new(
            proto
                .get_effect_list()
                .iter()
                .map(|it| KEffect::from(it, params, resolver, type_table))
                .collect(),
        )
    }

    pub fn stringify(&self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        buf.append_indent(indent)
            .append("contract {")
            .append_line_separator();
        for effect in &self.effects {
            effect.stringify(&mut buf, indent + 1);
        }
        buf.append_indent(indent)
            .append("}")
            .append_line_separator();
        buf
    }
}

impl KEffect {
    fn new(
        type_: Option<EffectType>,
        expressions: Vec<KExpression>,
        conditional_conclusion: Option<KExpression>,
        kind: Option<InvocationKind>,
    ) -> KEffect {
        KEffect {
            type_,
            expressions,
            conditional_conclusion,
            kind,
        }
    }

    pub fn from(
        proto: &kt_proto::Effect,
        params: &[KParameter],
        resolver: &MetadataNameResolver,
        type_table: &kt_proto::TypeTable,
    ) -> KEffect {
        let type_ = if proto.has_effect_type() {
            Some(proto.get_effect_type())
        } else {
            None
        };
        let expressions = proto
            .get_effect_constructor_argument_list()
            .iter()
            .map(|it| KExpression::from(it, params, resolver, type_table))
            .collect();
        let conditional_conclusion = if proto.has_conclusion_of_conditional_effect() {
            Some(KExpression::from(
                &proto.get_conclusion_of_conditional_effect(),
                params,
                resolver,
                type_table,
            ))
        } else {
            None
        };
        let kind = if proto.has_kind() {
            Some(proto.get_kind())
        } else {
            None
        };
        KEffect::new(type_, expressions, conditional_conclusion, kind)
    }

    pub fn stringify(&self, buf: &mut TextBuffer, indent: i32) {
        let Some(type_) = self.type_ else {
            return;
        };

        buf.append_indent(indent);

        match type_ {
            EffectType::ReturnsNotNull => {
                buf.append("returnsNotNull()");
                if let Some(conditional_conclusion) = &self.conditional_conclusion {
                    buf.append(" implies (");
                    conditional_conclusion.stringify(buf, indent, false);
                    buf.append_char(')');
                }
            }
            EffectType::Calls => {
                buf.append("callsInPlace(");
                let Some(func) = self.expressions.first() else {
                    jthrow!(IndexOutOfBoundsException; "Index 0 out of bounds for length 0")
                };
                let Some(reference) = &func.value_parameter_reference else {
                    jthrow!(NullPointerException; "func.valueParameterReference is null")
                };
                buf.append(&KotlinWriter::to_valid_kotlin_identifier(&reference.name));
                if let Some(kind) = self.kind {
                    let short_name = DecompilerContext::get_import_collector()
                        .get_short_name(KContract::INVOCATION_KIND);
                    buf.append(", ")
                        .append(short_name.as_deref().unwrap_or("null"))
                        .append(".")
                        .append(kind.name());
                }
                buf.append(")");
            }
            EffectType::ReturnsConstant => {
                buf.append("returns(");
                if let Some(expr) = self.expressions.first() {
                    expr.stringify(buf, indent, false);
                }
                buf.append(")");
                if let Some(conditional_conclusion) = &self.conditional_conclusion {
                    buf.append(" implies (");
                    conditional_conclusion.stringify(buf, indent, false);
                    buf.append_char(')');
                }
            }
        }
        buf.append_line_separator();
    }
}

impl KExpression {
    pub fn new(
        flags: i32,
        value_parameter_reference: Option<KParameter>,
        constant_value: Option<ConstantValue>,
        instanceof_type: Option<KType>,
        and_arguments: Vec<KExpression>,
        or_arguments: Vec<KExpression>,
    ) -> KExpression {
        KExpression {
            flags,
            value_parameter_reference,
            constant_value,
            instanceof_type,
            and_arguments,
            or_arguments,
        }
    }

    /// The placeholder parameter of the receiver (`this`); compared by identity.
    #[allow(non_snake_case)]
    fn THIS_TYPE() -> KParameter {
        THIS_TYPE.with(KParameter::clone)
    }

    pub fn from(
        proto: &kt_proto::Expression,
        params: &[KParameter],
        resolver: &MetadataNameResolver,
        type_table: &kt_proto::TypeTable,
    ) -> KExpression {
        let flags = proto.get_flags();
        let mut value_parameter_reference = None;
        if proto.has_value_parameter_reference() {
            let index = proto.get_value_parameter_reference();
            value_parameter_reference = Some(if index == 0 {
                KExpression::THIS_TYPE()
            } else {
                list_of_get(params, index.wrapping_sub(1))
            });
        }

        let constant_value = if proto.has_constant_value() {
            Some(proto.get_constant_value())
        } else {
            None
        };
        let mut instanceof_type = None;
        if proto.has_is_instance_type() {
            instanceof_type = Some(KType::from(&proto.get_is_instance_type(), resolver));
        } else if proto.has_is_instance_type_id() {
            instanceof_type = Some(KType::from_table(
                proto.get_is_instance_type_id(),
                resolver,
                type_table,
            ));
        }
        let and_arguments = proto
            .get_and_argument_list()
            .iter()
            .map(|it| KExpression::from(it, params, resolver, type_table))
            .collect();
        let or_arguments = proto
            .get_or_argument_list()
            .iter()
            .map(|it| KExpression::from(it, params, resolver, type_table))
            .collect();
        KExpression::new(
            flags,
            value_parameter_reference,
            constant_value,
            instanceof_type,
            and_arguments,
            or_arguments,
        )
    }

    pub fn stringify(&self, buf: &mut TextBuffer, indent: i32, part_of_or: bool) {
        if !self.and_arguments.is_empty() && (!self.or_arguments.is_empty() || part_of_or) {
            buf.append_char('(');
        }

        let mut appended = true;

        let mut param_name: Option<String> = None;
        let this_type = KExpression::THIS_TYPE();
        match &self.value_parameter_reference {
            Some(reference) if reference.ptr_eq(&this_type) => {
                param_name = Some("this".to_string());
            }
            Some(reference) => {
                param_name = Some(KotlinWriter::to_valid_kotlin_identifier(&reference.name));
            }
            None => {}
        }
        let param_name = param_name.as_deref().unwrap_or("null");

        if let Some(instanceof_type) = &self.instanceof_type {
            buf.append(param_name)
                .append_char(' ')
                .append(if Flags::IS_NEGATED.get(self.flags) {
                    "!is"
                } else {
                    "is"
                })
                .append_char(' ')
                .append_buffer(&instanceof_type.stringify(indent));
        } else if Flags::IS_NULL_CHECK_PREDICATE.get(self.flags) {
            buf.append(param_name)
                .append_char(' ')
                .append(if Flags::IS_NEGATED.get(self.flags) {
                    "!="
                } else {
                    "=="
                })
                .append_char(' ')
                .append("null");
        } else if let Some(constant_value) = self.constant_value {
            let reference = &self.value_parameter_reference;
            if reference.as_ref().is_some_and(|r| r.type_.k().is_nullable) {
                buf.append(param_name)
                    .append_char(' ')
                    .append(if Flags::IS_NEGATED.get(self.flags) {
                        "!="
                    } else {
                        "=="
                    })
                    .append_char(' ')
                    .append(&constant_value.name().to_lowercase());
            } else {
                let constant_name = constant_value.name().to_lowercase();
                let output = if reference
                    .as_ref()
                    .is_some_and(|r| r.type_.k().kotlin_type == "kotlin/Boolean")
                {
                    param_name
                } else {
                    constant_name.as_str()
                };

                if Flags::IS_NEGATED.get(self.flags) {
                    buf.append_char('!');
                }

                buf.append(output);
            }
        } else if self.value_parameter_reference.is_some() {
            if Flags::IS_NEGATED.get(self.flags) {
                buf.append_char('!');
            }
            buf.append(param_name);
        } else {
            appended = false;
        }

        if !self.and_arguments.is_empty() {
            for and_argument in &self.and_arguments {
                if appended {
                    buf.append(" && ");
                }
                appended = true;

                and_argument.stringify(buf, indent, false);
            }
        }

        if !self.or_arguments.is_empty() {
            if !self.and_arguments.is_empty() || part_of_or {
                buf.append_char(')');
            }
            for or_argument in &self.or_arguments {
                if appended {
                    buf.append(" || ");
                }
                appended = true;

                or_argument.stringify(buf, indent, true);
            }
        }
    }
}

/// `params.get(index)` on the `List.of(parameters)` the only caller
/// (`KFunction.parse`) passes: `List12` up to two elements, `ListN` beyond.
fn list_of_get(params: &[KParameter], index: i32) -> KParameter {
    let size = params.len() as i32;
    if index >= 0 && index < size {
        return params[index as usize].clone();
    }
    if size == 1 || size == 2 {
        jthrow!(IndexOutOfBoundsException; "Index: {} Size: {}", index, size)
    }
    jthrow!(ArrayIndexOutOfBoundsException; "Index {} out of bounds for length {}", index, size)
}
