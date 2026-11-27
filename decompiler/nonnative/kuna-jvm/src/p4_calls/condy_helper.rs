//! Port of `org/jetbrains/java/decompiler/modules/decompiler/CondyHelper.java`.

use crate::const_exprent::{ConstExprent, ConstValue};
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::invocation_exprent::{InvocationExprent, InvocationType};
use crate::pooled_constant::PooledConstant;
use crate::rt::JavaStr;
use crate::var_type::VarType;

pub struct CondyHelper;

impl CondyHelper {
    const CONSTANT_BOOTSTRAPS_CLASS: &'static str = "java/lang/invoke/ConstantBootstraps";

    pub fn simplify_condy(condy_expr: InvocationExprent) -> Exprent {
        if condy_expr.get_invocation_type() != InvocationType::ConstantDynamic {
            return condy_expr.0;
        }

        let method = condy_expr
            .get_bootstrap_method()
            .expect("NullPointerException: condyExpr.getBootstrapMethod()");
        if method.classname.as_deref() != Some(Self::CONSTANT_BOOTSTRAPS_CLASS) {
            return condy_expr.0;
        }

        match method.elementname.as_str() {
            "nullConstant" => {
                return ConstExprent::new(VarType::VARTYPE_NULL(), None, None)
                    .set_was_condy(true)
                    .0;
            }
            "primitiveClass" => {
                let desc = condy_expr.get_name();
                if desc.jlen() == 1 && "ZCBSIJFDV".contains(desc.as_str()) {
                    let type_ = VarType::new_signature_cl(&desc, false);
                    return ConstExprent::new(
                        VarType::VARTYPE_CLASS(),
                        Some(ConstValue::String(ExprProcessor::get_cast_type_name(
                            &type_,
                        ))),
                        None,
                    )
                    .set_was_condy(true)
                    .0;
                }
            }
            "enumConstant" => {
                let type_name = condy_expr.get_expr_type().value_or_null().to_string();
                return FieldExprent::new_full(
                    condy_expr.get_name(),
                    type_name.clone(),
                    true,
                    None,
                    FieldDescriptor::parse_descriptor(&format!("L{type_name};")),
                    None,
                    false,
                    true,
                )
                .0;
            }
            "getStaticFinal" => {
                let const_args = condy_expr
                    .get_bootstrap_arguments()
                    .expect("NullPointerException: condyExpr.getBootstrapArguments()");
                let field_type = condy_expr.get_expr_type().value.clone();
                let owner_class: Option<String>;
                if const_args.len() == 1 {
                    match &const_args[0] {
                        Some(PooledConstant::Primitive(owner_name)) => {
                            owner_class = Some(
                                owner_name
                                    .value
                                    .as_ref()
                                    .expect("NullPointerException: ownerName.value")
                                    .to_java_string(),
                            );
                        }
                        _ => return condy_expr.0,
                    }
                } else {
                    if condy_expr.get_expr_type().type_ != VarType::VARTYPE_OBJECT().type_ {
                        return condy_expr.0;
                    }
                    owner_class = field_type.as_deref().map(str::to_string);
                }
                let descriptor = FieldDescriptor::parse_descriptor(
                    field_type
                        .as_deref()
                        .expect("NullPointerException: fieldType"),
                );
                return FieldExprent::new_full(
                    condy_expr.get_name(),
                    owner_class.expect("NullPointerException: ownerClass"),
                    true,
                    None,
                    descriptor,
                    None,
                    false,
                    true,
                )
                .0;
            }
            _ => {}
        }
        condy_expr.0
    }
}
