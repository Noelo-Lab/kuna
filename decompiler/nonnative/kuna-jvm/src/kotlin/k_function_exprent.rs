//! Port of `org/vineflower/kotlin/expr/KFunctionExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::{kotlin_exprent, unported};

/// `KFunctionExprent.KFunctionType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum KFunctionType {
    None,
    Equals3,
    IfNull,
    GetKclass,
    StrTemplate,
}

impl KFunctionType {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            KFunctionType::None => "NONE",
            KFunctionType::Equals3 => "EQUALS3",
            KFunctionType::IfNull => "IF_NULL",
            KFunctionType::GetKclass => "GET_KCLASS",
            KFunctionType::StrTemplate => "STR_TEMPLATE",
        }
    }
}

pub struct KFunctionExprentData {
    pub k_type: KFunctionType,
}

kotlin_exprent!(pub KFunctionExprent: FunctionExprent, Function(KFunctionExprentData),
                is_k_function, as_k_function, try_as_k_function);

impl KFunctionExprent {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KFunctionExprent.toJava")
    }

    /// Java `getExprType()`; throws `NullPointerException` where it is `null`.
    pub fn get_expr_type(self) -> VarType {
        unported!("KFunctionExprent.getExprType")
    }

    /// Java `getExprType()` exactly, `null` included.
    pub fn get_expr_type_opt(self) -> Option<VarType> {
        unported!("KFunctionExprent.getExprType")
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("KFunctionExprent.checkExprTypeBounds")
    }

    /// Java: setFuncType(FunctionType)
    pub fn set_func_type(self, _func_type: FunctionType) {
        unported!("KFunctionExprent.setFuncType")
    }

    pub fn get_precedence(self) -> i32 {
        unported!("KFunctionExprent.getPrecedence")
    }

    pub fn copy(self) -> Exprent {
        unported!("KFunctionExprent.copy")
    }
}
