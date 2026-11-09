//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericsChecker.java`.

use std::rc::Rc;

use crate::code_type::CodeType;
use crate::rt::JHashMap;
use crate::var_type::VarType;

/// Immutable once built; clones share the bounds map.
#[derive(Clone)]
pub struct GenericsChecker {
    bounds_map: Rc<JHashMap<String, Vec<VarType>>>,
}

impl Default for GenericsChecker {
    fn default() -> Self {
        GenericsChecker::new()
    }
}

impl GenericsChecker {
    /// Java: `GenericsChecker()`
    pub fn new() -> GenericsChecker {
        GenericsChecker {
            bounds_map: Rc::new(JHashMap::new()),
        }
    }

    /// Java: `GenericsChecker(List<String>, List<List<VarType>>)`
    pub fn new_bounds(type_variables: &[String], bounds: &[Vec<VarType>]) -> GenericsChecker {
        let mut bounds_map = JHashMap::with_capacity(type_variables.len());
        for i in 0..type_variables.len() {
            bounds_map.put(type_variables[i].clone(), bounds[i].clone());
        }
        GenericsChecker {
            bounds_map: Rc::new(bounds_map),
        }
    }

    /// Java: `copy(List<String>, List<List<VarType>>)`
    pub fn copy(&self, type_variables: &[String], bounds: &[Vec<VarType>]) -> GenericsChecker {
        let mut new_bounds = (*self.bounds_map).clone();
        for i in 0..type_variables.len() {
            new_bounds.put(type_variables[i].clone(), bounds[i].clone());
        }
        GenericsChecker {
            bounds_map: Rc::new(new_bounds),
        }
    }

    /// Java: `copy(GenericsChecker)`
    pub fn copy_parent(&self, parent: &GenericsChecker) -> GenericsChecker {
        let mut new_bounds_map = (*self.bounds_map).clone();
        for (key, value) in parent.bounds_map.iter() {
            if !new_bounds_map.contains_key(key) {
                new_bounds_map.put(key.clone(), value.clone());
            }
        }
        GenericsChecker {
            bounds_map: Rc::new(new_bounds_map),
        }
    }

    pub fn is_properly_bounded(&self, type_: &VarType, bound: &VarType) -> bool {
        if type_.higher_equal_in_lattice_than(bound) {
            return true;
        }

        let bound = bound.resize_array_dim(0);

        if type_.type_ == CodeType::Genvar && type_.is_generic_type() {
            let type_bounds = type_
                .value
                .as_deref()
                .and_then(|v| self.bounds_map.get(&v.to_string()));
            if let Some(type_bounds) = type_bounds {
                for type_bound in type_bounds {
                    if self.is_properly_bounded(type_bound, &bound) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
