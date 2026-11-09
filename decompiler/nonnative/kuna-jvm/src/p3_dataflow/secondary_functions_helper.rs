//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SecondaryFunctionsHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::rt::JHashMap;
use crate::statement::Statement;
use crate::unported;
use crate::var_processor::VarProcessor;

pub struct SecondaryFunctionsHelper;

const DEFAULT_OPTIONS: IdentifySecondaryOptions = IdentifySecondaryOptions {
    force_ternary_simplification: false,
};

thread_local! {
    static FUNCSNOT: JHashMap<FunctionType, FunctionType> = {
        let mut map = JHashMap::new();
        map.put(FunctionType::Eq, FunctionType::Ne);
        map.put(FunctionType::Ne, FunctionType::Eq);
        map.put(FunctionType::Lt, FunctionType::Ge);
        map.put(FunctionType::Ge, FunctionType::Lt);
        map.put(FunctionType::Gt, FunctionType::Le);
        map.put(FunctionType::Le, FunctionType::Gt);
        map.put(FunctionType::BooleanAnd, FunctionType::BooleanOr);
        map.put(FunctionType::BooleanOr, FunctionType::BooleanAnd);
        map
    };

    static MAP_NUM_COMPARISONS: JHashMap<FunctionType, [Option<FunctionType>; 3]> = {
        let mut map = JHashMap::new();
        map.put(
            FunctionType::Eq,
            [Some(FunctionType::Lt), Some(FunctionType::Eq), Some(FunctionType::Gt)],
        );
        map.put(
            FunctionType::Ne,
            [Some(FunctionType::Ge), Some(FunctionType::Ne), Some(FunctionType::Le)],
        );
        map.put(FunctionType::Gt, [Some(FunctionType::Ge), Some(FunctionType::Gt), None]);
        map.put(FunctionType::Ge, [None, Some(FunctionType::Ge), Some(FunctionType::Gt)]);
        map.put(FunctionType::Lt, [None, Some(FunctionType::Lt), Some(FunctionType::Le)]);
        map.put(FunctionType::Le, [Some(FunctionType::Lt), Some(FunctionType::Le), None]);
        map
    };
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IdentifySecondaryOptions {
    force_ternary_simplification: bool,
}

impl IdentifySecondaryOptions {
    pub fn new() -> IdentifySecondaryOptions {
        IdentifySecondaryOptions {
            force_ternary_simplification: false,
        }
    }

    pub fn force_ternary_simplification(mut self) -> IdentifySecondaryOptions {
        self.force_ternary_simplification = true;
        self
    }
}

impl SecondaryFunctionsHelper {
    pub fn identify_secondary_functions(stat: Statement, var_proc: VarProcessor) -> bool {
        Self::identify_secondary_functions_options(stat, var_proc, DEFAULT_OPTIONS)
    }

    /// Java: identifySecondaryFunctions(Statement, VarProcessor, IdentifySecondaryOptions)
    pub fn identify_secondary_functions_options(
        stat: Statement,
        var_proc: VarProcessor,
        options: IdentifySecondaryOptions,
    ) -> bool {
        unported!("SecondaryFunctionsHelper.identifySecondaryFunctions")
    }

    /// Java: identifySecondaryFunctions(Statement, Exprent, boolean, VarProcessor, IdentifySecondaryOptions)
    fn identify_secondary_functions_exprent(
        stat: Statement,
        exprent: Exprent,
        statement_level: bool,
        var_proc: VarProcessor,
        options: IdentifySecondaryOptions,
    ) -> Option<Exprent> {
        unported!("SecondaryFunctionsHelper.identifySecondaryFunctions")
    }

    fn is_simple(exprent: Exprent, other: Option<Exprent>) -> bool {
        match exprent.get_type() {
            ExprentType::Var | ExprentType::Const => true,
            ExprentType::Array => {
                let array = exprent.as_array();
                Self::is_super_simple(array.get_index()) && Self::is_super_simple(array.get_array())
            }
            ExprentType::Field => match other {
                None => true,
                Some(other) => (other.get_exprent_use() & Exprent::SIDE_EFFECTS_FREE) != 0,
            },
            _ => false,
        }
    }

    fn is_super_simple(exprent: Exprent) -> bool {
        exprent.get_type() == ExprentType::Var || exprent.get_type() == ExprentType::Const
    }

    fn has_pattern(exprent: Exprent) -> bool {
        for ex in exprent.get_all_exprents_rec_self(false, true) {
            if ex.is_pattern()
                || ex.try_as_function().is_some_and(|func| {
                    func.get_func_type() == FunctionType::Instanceof
                        && func.get_lst_operands().len() > 2
                })
            {
                return true;
            }
        }

        false
    }

    pub fn propagate_bool_not(exprent: Exprent) -> Option<Exprent> {
        unported!("SecondaryFunctionsHelper.propagateBoolNot")
    }

    pub fn update_assignments(stat: Statement) -> bool {
        unported!("SecondaryFunctionsHelper.updateAssignments")
    }
}
