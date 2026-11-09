//! Port of `org/vineflower/kotlin/expr/KFunctionExprent.java`.

#![allow(dead_code, unused_variables)]

use crate::check_types_result::CheckTypesResult;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::jthrow;
use crate::rt::BitSet;
use crate::text_buffer::TextBuffer;
use crate::typed::Typed;
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

impl Typed for KFunctionType {}

/// A `Typed` function type: `getAnyFunctionType()` and `setFuncType(Typed)`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AnyFunctionType {
    Java(FunctionType),
    Kotlin(KFunctionType),
}

pub struct KFunctionExprentData {
    pub k_type: KFunctionType,
}

kotlin_exprent!(pub KFunctionExprent: FunctionExprent, Function(KFunctionExprentData),
                is_k_function, as_k_function, try_as_k_function);

impl KFunctionExprent {
    /// Java: KFunctionExprent(FunctionExprent)
    pub fn new(func: FunctionExprent) -> KFunctionExprent {
        unported!("KFunctionExprent.<init>")
    }

    /// Java: KFunctionExprent(KFunctionType, List<Exprent>, BitSet)
    pub fn new_k_type(
        func_type: KFunctionType,
        operands: Vec<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> KFunctionExprent {
        unported!("KFunctionExprent.<init>")
    }

    /// Java: KFunctionExprent(FunctionType, List<Exprent>, BitSet)
    pub fn new_func_type(
        func_type: FunctionType,
        operands: Vec<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> KFunctionExprent {
        unported!("KFunctionExprent.<init>")
    }

    /// Java: KFunctionExprent(FunctionExprent, KFunctionType, VarType) (private)
    fn new_typed(
        func: FunctionExprent,
        k_type: KFunctionType,
        expr_type: Option<VarType>,
    ) -> KFunctionExprent {
        unported!("KFunctionExprent.<init>")
    }

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

    fn optimize_type(self) {
        unported!("KFunctionExprent.optimizeType")
    }

    pub fn get_any_function_type(self) -> AnyFunctionType {
        let func_type = self.get_func_type();

        if func_type == FunctionType::Other {
            let k_type = self.kd(|d| d.k_type);
            if k_type == KFunctionType::None {
                jthrow!(IllegalStateException; "No function type at all set!");
            }

            return AnyFunctionType::Kotlin(k_type);
        }

        AnyFunctionType::Java(func_type)
    }

    /// Java: setFuncType(FunctionType)
    pub fn set_func_type(self, func_type: FunctionType) {
        unported!("KFunctionExprent.setFuncType")
    }

    /// Java: setFuncType(Typed); `super.setFuncType` is `FunctionExprent::set_func_type_base`.
    pub fn set_func_type_typed(self, typed: AnyFunctionType) {
        unported!("KFunctionExprent.setFuncType")
    }

    pub fn get_precedence(self) -> i32 {
        unported!("KFunctionExprent.getPrecedence")
    }

    pub fn copy(self) -> Exprent {
        unported!("KFunctionExprent.copy")
    }
}
