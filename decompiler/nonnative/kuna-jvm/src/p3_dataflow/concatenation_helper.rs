//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ConcatenationHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::new_exprent::NewExprent;
use crate::pooled_constant::PooledConstant;
use crate::root_statement::RootStatement;
use crate::rt::BitSet;
use crate::statement::Statement;
use crate::unported;
use crate::var_type::VarType;

pub struct ConcatenationHelper;

const BUILDER_CLASS: &str = "java/lang/StringBuilder";
const BUFFER_CLASS: &str = "java/lang/StringBuffer";
const STRING_CLASS: &str = "java/lang/String";

const TAG_ARG: u16 = 0x0001;
const TAG_ARG_S: &str = "\u{1}";
const TAG_CONST: u16 = 0x0002;

thread_local! {
    static BUILDER_TYPE: VarType = VarType::new_value(CodeType::Object, 0, "java/lang/StringBuilder");
    static BUFFER_TYPE: VarType = VarType::new_value(CodeType::Object, 0, "java/lang/StringBuffer");
}

impl ConcatenationHelper {
    #[allow(non_snake_case)]
    fn BUILDER_TYPE() -> VarType {
        BUILDER_TYPE.with(VarType::clone)
    }

    #[allow(non_snake_case)]
    fn BUFFER_TYPE() -> VarType {
        BUFFER_TYPE.with(VarType::clone)
    }

    pub fn simplify_string_concat(stat: Statement) {
        let sub_stats = stat.get_stats().borrow().to_vec();
        for s in sub_stats {
            Self::simplify_string_concat(s);
        }

        if stat.get_exprents().is_some() {
            let mut i = 0;
            while i < stat.get_exprents().unwrap().len() {
                let ret = Self::simplify_string_concat_exprent(
                    stat.get_top_parent(),
                    stat.get_exprents().unwrap().get(i),
                );
                if let Some(ret) = ret {
                    stat.get_exprents().unwrap().set(i, ret);
                }
                i += 1;
            }
        }
    }

    /// Java: simplifyStringConcat(RootStatement, Exprent)
    fn simplify_string_concat_exprent(root: RootStatement, exprent: Exprent) -> Option<Exprent> {
        for cexp in exprent.get_all_exprents() {
            if let Some(ret) = Self::simplify_string_concat_exprent(root, cexp) {
                exprent.replace_exprent(cexp, ret);
                ret.add_bytecode_offsets(cexp.with(|d| d.bytecode.clone()).as_ref());
            }
        }

        if exprent.is_invocation() {
            let ret = Self::contract_string_concat(root, exprent);
            if !exprent.equals(ret) {
                return Some(ret);
            }
        }

        None
    }

    pub fn contract_string_concat(root: RootStatement, expr: Exprent) -> Exprent {
        unported!("ConcatenationHelper.contractStringConcat")
    }

    fn create_concat_exprent(lst_operands: &[Exprent], bytecode: Option<&BitSet>) -> Exprent {
        unported!("ConcatenationHelper.createConcatExprent")
    }

    fn extract_parameters(
        root: RootStatement,
        bootstrap_arguments: Option<&[Option<PooledConstant>]>,
        expr: InvocationExprent,
    ) -> Vec<Exprent> {
        unported!("ConcatenationHelper.extractParameters")
    }

    fn is_append_concat(expr: InvocationExprent, cltype: &VarType) -> bool {
        unported!("ConcatenationHelper.isAppendConcat")
    }

    fn is_new_concat(expr: NewExprent, cltype: &VarType) -> bool {
        unported!("ConcatenationHelper.isNewConcat")
    }

    fn remove_string_value_of(exprent: Exprent) -> Exprent {
        unported!("ConcatenationHelper.removeStringValueOf")
    }
}
