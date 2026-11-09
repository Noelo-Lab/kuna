//! Port of `org/jetbrains/java/decompiler/modules/decompiler/AssertProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::class_wrapper::ClassWrapper;
use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationExprent;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::unported;
use crate::var_type::VarType;

pub struct AssertProcessor;

struct AssertResult {
    exprent: Option<Exprent>,
    valid: bool,
    replace_parent: bool,
    negate: bool,
    assignment: bool,
}

impl AssertResult {
    fn new(exprent: Option<Exprent>, valid: bool) -> AssertResult {
        AssertResult {
            exprent,
            valid,
            replace_parent: false,
            negate: true,
            assignment: false,
        }
    }
}

impl AssertProcessor {
    #[allow(non_snake_case)]
    fn CLASS_ASSERTION_ERROR() -> VarType {
        thread_local! {
            static CLASS_ASSERTION_ERROR: VarType =
                VarType::new_value(CodeType::Object, 0, "java/lang/AssertionError");
        }
        CLASS_ASSERTION_ERROR.with(|t| t.clone())
    }

    pub fn build_assertions(root: RootStatement) -> bool {
        unported!("AssertProcessor.buildAssertions")
    }

    pub fn get_holding_class() -> ClassWrapper {
        unported!("AssertProcessor.getHoldingClass")
    }

    fn clean_interface_static_initializer(stat: Statement, class_name: &str, key: &str) -> bool {
        unported!("AssertProcessor.cleanInterfaceStaticInitializer")
    }

    fn replace_assertions(statement: Statement, classname: &str, key: &str) -> bool {
        unported!("AssertProcessor.replaceAssertions")
    }

    fn replace_assertion(parent: Statement, stat: IfStatement, classname: &str, key: &str) -> bool {
        unported!("AssertProcessor.replaceAssertion")
    }

    fn is_assertion_error(stat: Option<Statement>) -> Option<InvocationExprent> {
        unported!("AssertProcessor.isAssertionError")
    }

    fn get_assertion_exprent(
        stat: Statement,
        exprent: Exprent,
        classname: &str,
        key: &str,
        throw_in_if: bool,
    ) -> AssertResult {
        unported!("AssertProcessor.getAssertionExprent")
    }

    fn parent_skipping_sequences(stat: Statement) -> Option<Statement> {
        let mut stat = stat.get_parent();

        while let Some(s) = stat.filter(|s| s.is_sequence()) {
            stat = s.get_parent();
        }

        stat
    }

    fn is_assertion_field(exprent: Exprent, classname: &str, key: &str) -> bool {
        unported!("AssertProcessor.isAssertionField")
    }
}
