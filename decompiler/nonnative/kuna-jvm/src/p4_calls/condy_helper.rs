//! Port of `org/jetbrains/java/decompiler/modules/decompiler/CondyHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::link_constant::LinkConstant;
use crate::pooled_constant::PooledConstant;
use crate::unported;

pub struct CondyHelper;

impl CondyHelper {
    const CONSTANT_BOOTSTRAPS_CLASS: &'static str = "java/lang/invoke/ConstantBootstraps";

    pub fn simplify_condy(condy_expr: InvocationExprent) -> Exprent {
        unported!("CondyHelper.simplifyCondy")
    }

    fn construct_var_handle(
        field_name: &str,
        field_owner: &str,
        field_type: Option<&PooledConstant>,
        is_static: bool,
    ) -> Exprent {
        unported!("CondyHelper.constructVarHandle")
    }

    fn to_class_exprent(constant: Option<&PooledConstant>) -> Exprent {
        unported!("CondyHelper.toClassExprent")
    }

    fn to_condy_exprent(field_type: &LinkConstant) -> Exprent {
        unported!("CondyHelper.toCondyExprent")
    }

    fn construct_lookup_exprent() -> InvocationExprent {
        unported!("CondyHelper.constructLookupExprent")
    }

    fn construct_find_var_handle_exprent(
        is_static: bool,
        lookup: Exprent,
        owner_class: Exprent,
        field_name: Exprent,
        field_class: Exprent,
    ) -> InvocationExprent {
        unported!("CondyHelper.constructFindVarHandleExprent")
    }

    fn construct_array_var_handle_exprent(
        class_const: Option<&PooledConstant>,
    ) -> InvocationExprent {
        unported!("CondyHelper.constructArrayVarHandleExprent")
    }
}
