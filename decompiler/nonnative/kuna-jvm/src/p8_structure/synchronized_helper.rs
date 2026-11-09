//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SynchronizedHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::unported;
use crate::var_processor::VarProcessor;

pub struct SynchronizedHelper;

impl SynchronizedHelper {
    pub fn clean_synchronized_var(stat: Statement) -> bool {
        unported!("SynchronizedHelper.cleanSynchronizedVar")
    }

    pub fn insert_sink(root: RootStatement, var_processor: VarProcessor, stat: Statement) -> bool {
        unported!("SynchronizedHelper.insertSink")
    }
}
