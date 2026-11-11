//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/CheckTypesResult.java`.
//!
//! A plain owned value: `Exprent.checkExprTypeBounds()` builds one and hands it
//! to `VarTypeProcessor`, nobody else holds it. `ValidationHelper.notNull` on
//! the pair's arguments is vacuous here (neither parameter can be null).

use crate::exprent::Exprent;
use crate::var_type::VarType;

#[derive(Clone, Default)]
pub struct CheckTypesResult {
    upper_bounds: Vec<ExprentTypePair>,
    lower_bounds: Vec<ExprentTypePair>,
}

impl CheckTypesResult {
    pub fn new() -> CheckTypesResult {
        CheckTypesResult {
            upper_bounds: Vec::new(),
            lower_bounds: Vec::new(),
        }
    }

    pub fn add_expr_upper_bound(&mut self, exprent: Exprent, type_: &VarType) {
        self.upper_bounds.push(ExprentTypePair::new(exprent, type_));
    }

    pub fn add_expr_lower_bound(&mut self, exprent: Exprent, type_: &VarType) {
        self.lower_bounds.push(ExprentTypePair::new(exprent, type_));
    }

    pub fn get_upper_bounds(&self) -> &[ExprentTypePair] {
        &self.upper_bounds
    }

    pub fn get_lower_bounds(&self) -> &[ExprentTypePair] {
        &self.lower_bounds
    }
}

fn list_to_string(list: &[ExprentTypePair]) -> String {
    let items: Vec<String> = list.iter().map(|p| p.to_string()).collect();
    format!("[{}]", items.join(", "))
}

impl std::fmt::Display for CheckTypesResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Min: {} Max: {}",
            list_to_string(&self.lower_bounds),
            list_to_string(&self.upper_bounds)
        )
    }
}

/// Java: `CheckTypesResult.ExprentTypePair`.
#[derive(Clone)]
pub struct ExprentTypePair {
    pub exprent: Exprent,
    pub type_: VarType,
}

impl ExprentTypePair {
    pub fn new(exprent: Exprent, type_: &VarType) -> ExprentTypePair {
        ExprentTypePair {
            exprent,
            type_: type_.clone(),
        }
    }
}

impl std::fmt::Display for ExprentTypePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" --> {}",
            self.exprent
                .to_java(0)
                .convert_to_string_and_allow_data_discard(),
            self.type_
        )
    }
}
