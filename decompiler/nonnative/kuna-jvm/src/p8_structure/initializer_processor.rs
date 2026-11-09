//! Port of `org/jetbrains/java/decompiler/modules/decompiler/InitializerProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::class_wrapper::ClassWrapper;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionType;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::JHashSet;
use crate::struct_class::StructClass;
use crate::unported;

pub struct InitializerProcessor;

impl InitializerProcessor {
    pub fn extract_initializers(wrapper: ClassWrapper) {
        unported!("InitializerProcessor.extractInitializers")
    }

    fn lift_constructor(wrapper: ClassWrapper) {
        unported!("InitializerProcessor.liftConstructor")
    }

    fn hide_empty_super(wrapper: ClassWrapper) {
        unported!("InitializerProcessor.hideEmptySuper")
    }

    pub fn hide_initalizers(wrapper: ClassWrapper) {
        unported!("InitializerProcessor.hideInitalizers")
    }

    fn extract_static_initializers(wrapper: ClassWrapper, method: MethodWrapper) {
        unported!("InitializerProcessor.extractStaticInitializers")
    }

    fn extract_dynamic_initializers(wrapper: ClassWrapper) {
        unported!("InitializerProcessor.extractDynamicInitializers")
    }

    fn process_dynamic_initializer(expr: Exprent) -> Exprent {
        let mut expr = expr;

        if expr.is_function() {
            let mut temp = expr;
            while let Some(func) = temp.try_as_function() {
                let func_type = func.get_func_type();
                if func_type.cast_type().is_none() && func_type != FunctionType::Cast {
                    break;
                }
                temp = func.get_lst_operands().get(0);
            }

            if let Some(func) = temp.try_as_function() {
                func.unwrap_box();

                expr = func.0;
            }
        } else {
            expr = Self::process_boxing_cast(expr);
        }

        expr
    }

    fn process_boxing_cast(expr: Exprent) -> Exprent {
        let mut expr = expr;

        if let Some(invocation) = expr.try_as_invocation() {
            if invocation.is_unboxing_call() {
                if let Some(inner) = invocation.get_instance() {
                    if inner
                        .try_as_function()
                        .is_some_and(|f| f.get_func_type() == FunctionType::Cast)
                    {
                        let bytecode = expr.with(|d| d.bytecode.clone());
                        inner.add_bytecode_offsets(bytecode.as_ref());
                        expr = inner;
                    }
                }
            }
        }

        expr
    }

    #[allow(clippy::too_many_arguments)]
    fn is_exprent_independent(
        field: FieldExprent,
        exprent: Exprent,
        method: MethodWrapper,
        cl: StructClass,
        whitelist: &JHashSet<String>,
        multi_assign: &[String],
        not_inlined: &[FieldExprent],
        fidx: i32,
        is_static: bool,
    ) -> bool {
        unported!("InitializerProcessor.isExprentIndependent")
    }

    fn qualify_field_references(nexpr: NewExprent, cl: StructClass, fidx: i32) {
        unported!("InitializerProcessor.qualifyFieldReferences")
    }
}
