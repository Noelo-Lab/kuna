//! Port of `org/jetbrains/java/decompiler/modules/decompiler/TryHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::catch_statement::CatchStatement;
use crate::root_statement::RootStatement;
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::unported;

pub struct TryHelper;

impl TryHelper {
    pub fn enhance_try_stats(root: RootStatement, cl: StructClass) -> bool {
        let mut ret = Self::make_try_with_resource_rec(cl, root.0);

        if ret {
            SequenceHelper::condense_sequences(root.0);
        }

        if Self::merge_trys(root.0) {
            SequenceHelper::condense_sequences(root.0);

            ret = true;
        }

        ret
    }

    fn make_try_with_resource_rec(cl: StructClass, stat: Statement) -> bool {
        unported!("TryHelper.makeTryWithResourceRec")
    }

    fn merge_trys(root: Statement) -> bool {
        let mut ret = false;

        if let Some(catch_stat) = root.try_as_catch() {
            if Self::merge_try(catch_stat) {
                ret = true;
            }
        }

        let stats = root.get_stats().borrow().to_vec();

        for stat in stats {
            ret |= Self::merge_trys(stat);
        }

        ret
    }

    fn merge_try(stat: CatchStatement) -> bool {
        unported!("TryHelper.mergeTry")
    }
}
