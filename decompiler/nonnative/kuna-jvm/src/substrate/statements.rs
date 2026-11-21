//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/Statements.java`.

use crate::class_wrapper::ClassWrapper;
use crate::invocation_exprent::{InvocationExprent, Type as InvocationType};
use crate::method_wrapper::MethodWrapper;
use crate::statement::{Statement, StatementType};
use crate::var_version_pair::VarVersionPair;

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
        inv: InvocationExprent,
        method: MethodWrapper,
        wrapper: ClassWrapper,
        with_this: bool,
    ) -> bool {
        if inv.get_functype() == InvocationType::Init {
            if let Some(inst_var) = inv.get_instance().and_then(|i| i.try_as_var()) {
                let var_pair = VarVersionPair::from_var(inst_var);
                let class_name = method
                    .with(|d| d.varproc)
                    .get_this_vars()
                    .borrow()
                    .get_cloned(&var_pair);
                if class_name.is_some() {
                    return with_this
                        || wrapper.get_class_struct().qualified_name() != inv.get_classname();
                }
            }
        }

        false
    }
}
