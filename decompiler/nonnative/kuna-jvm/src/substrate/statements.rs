//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/Statements.java`.

use crate::class_wrapper::ClassWrapper;
use crate::invocation_exprent::InvocationExprent;
use crate::method_wrapper::MethodWrapper;
use crate::statement::{Statement, StatementType};
use crate::unported;

pub struct Statements;

impl Statements {
    pub fn find_first_data(stat: Statement) -> Option<Statement> {
        if stat.get_exprents().is_some() {
            return Some(stat);
        } else if stat.is_labeled() {
            return None;
        }

        match stat.type_() {
            StatementType::Sequence
            | StatementType::If
            | StatementType::Root
            | StatementType::Switch
            | StatementType::Synchronized => {
                Statements::find_first_data(stat.get_first().expect("first"))
            }
            _ => None,
        }
    }

    pub fn is_invocation_init_constructor(
        _inv: InvocationExprent,
        _method: MethodWrapper,
        _wrapper: ClassWrapper,
        _with_this: bool,
    ) -> bool {
        unported!("Statements.isInvocationInitConstructor")
    }
}
