//! Port of `org/jetbrains/java/decompiler/main/collectors/VarNamesCollector.java`.

use crate::rt::JHashSet;
use crate::{arena_identity, arena_object};

pub struct VarNamesCollectorData {
    used_names: JHashSet<String>,
    contains_null: bool,
}

arena_object!(pub VarNamesCollector => VarNamesCollectorData, Class);
arena_identity!(VarNamesCollector);

impl VarNamesCollector {
    pub fn new() -> VarNamesCollector {
        VarNamesCollector::alloc(VarNamesCollectorData {
            used_names: JHashSet::new(),
            contains_null: false,
        })
    }

    /// Java: VarNamesCollector(Collection<String>)
    pub fn new_with_names(set_names: impl IntoIterator<Item = String>) -> VarNamesCollector {
        let collector = VarNamesCollector::new();
        collector.add_names(set_names);
        collector
    }

    pub fn add_name(self, value: impl Into<String>) {
        let value = value.into();
        self.with_mut(|d| d.used_names.add(value));
    }

    pub fn add_names(self, names: impl IntoIterator<Item = String>) {
        let names: Vec<String> = names.into_iter().collect();
        self.with_mut(|d| d.used_names.add_all(names));
    }

    /// Java: getFreeName(int)
    pub fn get_free_name_index(self, index: i32) -> String {
        self.get_free_name(&format!("var{index}"))
    }

    /// Java: getFreeName(String) with a possibly-null proposition: the first null is
    /// returned as is and remembered, later ones grow from `"null" + "x"`.
    pub fn get_free_name_opt(self, proposition: Option<&str>) -> Option<String> {
        match proposition {
            Some(proposition) => Some(self.get_free_name(proposition)),
            None if !self.with(|d| d.contains_null) => {
                self.with_mut(|d| d.contains_null = true);
                None
            }
            None => Some(self.get_free_name("nullx")),
        }
    }

    /// Java: getFreeName(String)
    pub fn get_free_name(self, proposition: &str) -> String {
        let mut proposition = proposition.to_string();
        while self.with(|d| d.used_names.contains(&proposition)) {
            proposition.push('x');
        }
        self.add_name(proposition.clone());
        proposition
    }

    /// The unmodifiable view, as its elements in the set's iteration order.
    pub fn get_used_names(self) -> Vec<String> {
        self.with(|d| d.used_names.to_vec())
    }
}
