//! Port of `org/jetbrains/java/decompiler/modules/decompiler/stats/DummyExitStatement.java`.

use crate::java_subclass;
use crate::rt::BitSet;
use crate::statement::{Statement, StatementKind, StatementType};

pub struct DummyExitStatementData {
    pub bytecode: Option<BitSet>,
}

java_subclass!(pub DummyExitStatement: Statement, StatementKind::DummyExit(DummyExitStatementData), is_dummy_exit, as_dummy_exit, try_as_dummy_exit);

impl DummyExitStatement {
    pub fn new() -> DummyExitStatement {
        DummyExitStatement(Statement::new(
            StatementType::DummyExit,
            StatementKind::DummyExit(DummyExitStatementData { bytecode: None }),
        ))
    }

    pub fn add_bytecode_offsets(self, bytecode_offsets: Option<&BitSet>) {
        if let Some(offsets) = bytecode_offsets {
            if !offsets.is_empty() {
                self.d_mut(|d| d.bytecode.get_or_insert_with(BitSet::new).or(offsets));
            }
        }
    }

    /// Field read of the public Java field `bytecode`.
    pub fn bytecode(self) -> Option<BitSet> {
        self.d(|d| d.bytecode.clone())
    }
}
