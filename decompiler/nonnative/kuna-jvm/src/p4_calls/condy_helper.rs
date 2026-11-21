//! Port of `org/jetbrains/java/decompiler/modules/decompiler/CondyHelper.java`.

use crate::code_constants::OPC_LDC;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::invocation_exprent::{InvocationExprent, InvocationType, Type as InvocationFuncType};
use crate::link_constant::LinkConstant;
use crate::list_stack::ListStack;
use crate::method_descriptor::MethodDescriptor;
use crate::pooled_constant::PooledConstant;
use crate::rt::JavaStr;
use crate::struct_general_attribute::StructGeneralAttribute;
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
            "fieldVarHandle" | "staticFieldVarHandle" => {
                if !DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_COMPLEX_CONDYS)
                {
                    return condy_expr.0;
                }
                let is_static = method.elementname.starts_with("static");
                let const_args = condy_expr
                    .get_bootstrap_arguments()
                    .expect("NullPointerException: condyExpr.getBootstrapArguments()");
                let field_name = condy_expr.get_name();
                let owner = match const_args.first() {
                    Some(Some(PooledConstant::Primitive(owner))) if const_args.len() == 2 => {
                        owner.clone()
                    }
                    _ => return condy_expr.0,
                };
                let owner_class = owner.get_string();
                return Self::construct_var_handle(
                    &field_name,
                    &owner_class,
                    const_args[1].as_ref(),
                    is_static,
                );
            }
            "arrayVarHandle" => {
                if !DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_COMPLEX_CONDYS)
                {
                    return condy_expr.0;
                }
                let const_args = condy_expr
                    .get_bootstrap_arguments()
                    .expect("NullPointerException: condyExpr.getBootstrapArguments()");
                if const_args.len() != 1 {
                    return condy_expr.0;
                }
                return Self::construct_array_var_handle_exprent(const_args[0].as_ref()).0;
            }
            _ => {}
        }
        condy_expr.0
    }

    fn construct_var_handle(
        field_name: &str,
        field_owner: &str,
        field_type: Option<&PooledConstant>,
        is_static: bool,
    ) -> Exprent {
        let lookup_exprent = Self::construct_lookup_exprent().0;
        let owner_class_class = VarType::new_signature_cl(field_owner, false);
        let owner_class_const = ConstExprent::new(
            VarType::VARTYPE_CLASS(),
            Some(ConstValue::String(ExprProcessor::get_cast_type_name(
                &owner_class_class,
            ))),
            None,
        )
        .0;
        let field_name_const = ConstExprent::new(
            VarType::VARTYPE_STRING(),
            Some(ConstValue::String(field_name.to_string())),
            None,
        )
        .0;
        let field_type_const = Self::to_class_exprent(field_type);
        Self::construct_find_var_handle_exprent(
            is_static,
            lookup_exprent,
            owner_class_const,
            field_name_const,
            field_type_const,
        )
        .0
    }

    fn to_class_exprent(constant: Option<&PooledConstant>) -> Exprent {
        let const_expr: Exprent;
        if let Some(PooledConstant::Primitive(constant)) = constant {
            let field_type_class = VarType::new_signature_cl(&constant.get_string(), false);
            const_expr = ConstExprent::new(
                VarType::VARTYPE_CLASS(),
                Some(ConstValue::String(ExprProcessor::get_cast_type_name(
                    &field_type_class,
                ))),
                None,
            )
            .0;
        } else {
            let link = constant.expect("NullPointerException: constant").as_link();
            const_expr = Self::to_condy_exprent(&link);
        }
        const_expr
    }

    fn to_condy_exprent(field_type: &LinkConstant) -> Exprent {
        let cl = DecompilerContext::get_current_class()
            .expect("NullPointerException: DecompilerContext.CURRENT_CLASS");
        let bootstrap = cl.get_attribute(StructGeneralAttribute::ATTRIBUTE_BOOTSTRAP_METHODS);
        let mut bootstrap_method = None;
        let mut const_args = None;
        if let Some(bootstrap) = bootstrap {
            bootstrap_method = bootstrap.get_method_reference(field_type.index1);
            const_args = Some(bootstrap.get_method_arguments(field_type.index1));
        }
        let arg = InvocationExprent::new_invoke(
            OPC_LDC,
            field_type,
            bootstrap_method,
            const_args,
            &mut ListStack::new(),
            None,
        );
        let field_type_const = Self::simplify_condy(arg);
        if let Some(c) = field_type_const.try_as_const() {
            c.set_was_condy(false);
        }
        field_type_const
    }

    fn construct_lookup_exprent() -> InvocationExprent {
        let exprent = InvocationExprent::new();
        exprent.set_name("lookup".to_string());
        exprent.set_classname("java/lang/invoke/MethodHandles".to_string());
        let desc = "()Ljava/lang/invoke/MethodHandles$Lookup;";
        exprent.set_string_descriptor(desc.to_string());
        exprent.set_descriptor(MethodDescriptor::parse_descriptor(desc));
        exprent.set_functype(InvocationFuncType::General);
        exprent.set_static(true);
        exprent
    }

    fn construct_find_var_handle_exprent(
        is_static: bool,
        lookup: Exprent,
        owner_class: Exprent,
        field_name: Exprent,
        field_class: Exprent,
    ) -> InvocationExprent {
        let exprent = InvocationExprent::new();
        exprent.set_name(
            if is_static {
                "findStaticVarHandle"
            } else {
                "findVarHandle"
            }
            .to_string(),
        );
        exprent.set_classname("java/lang/invoke/MethodHandles$Lookup".to_string());
        let desc =
            "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;";
        exprent.set_string_descriptor(desc.to_string());
        exprent.set_descriptor(MethodDescriptor::parse_descriptor(desc));
        exprent.set_functype(InvocationFuncType::General);
        exprent.set_static(false);
        exprent.set_instance(Some(lookup));
        exprent.set_lst_parameters(vec![owner_class, field_name, field_class]);
        exprent.mark_was_lazy_condy()
    }

    fn construct_array_var_handle_exprent(
        class_const: Option<&PooledConstant>,
    ) -> InvocationExprent {
        let exprent = InvocationExprent::new();
        exprent.set_name("arrayElementVarHandle".to_string());
        exprent.set_classname("java/lang/invoke/MethodHandles".to_string());
        let desc = "(Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;";
        exprent.set_string_descriptor(desc.to_string());
        exprent.set_descriptor(MethodDescriptor::parse_descriptor(desc));
        exprent.set_functype(InvocationFuncType::General);
        exprent.set_static(true);
        let class_expr = Self::to_class_exprent(class_const);
        exprent.set_lst_parameters(vec![class_expr]);
        exprent.mark_was_lazy_condy()
    }
}
