//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClearStructHelper.java`.

use std::collections::VecDeque;

use crate::root_statement::RootStatement;
use crate::statement::Statement;

pub struct ClearStructHelper;

impl ClearStructHelper {
    pub fn clear_statements(root: RootStatement) {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(root.0);

        while let Some(stat) = stack.pop_front() {
            stat.clear_temp_information();

            stack.extend(stat.get_stats().iter());
        }
    }
}
