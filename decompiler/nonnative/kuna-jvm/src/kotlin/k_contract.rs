//! Port of `org/vineflower/kotlin/struct/KContract.java`.

#![allow(dead_code, unused_variables)]

use crate::k_parameter::KParameter;
use crate::k_type::KType;
use crate::kt_proto::{self, ConstantValue, EffectType, InvocationKind};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::text_buffer::TextBuffer;
use crate::unported;

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
        unported!("KContract.from")
    }

    pub fn stringify(&self, indent: i32) -> TextBuffer {
        unported!("KContract.stringify")
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
        unported!("KContract.KEffect.from")
    }

    pub fn stringify(&self, buf: &mut TextBuffer, indent: i32) {
        unported!("KContract.KEffect.stringify")
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
        unported!("KContract.KExpression.from")
    }

    pub fn stringify(&self, buf: &mut TextBuffer, indent: i32, part_of_or: bool) {
        unported!("KContract.KExpression.stringify")
    }
}
