//! Port of `org/jetbrains/java/decompiler/modules/renamer/PoolInterceptor.java`.

use crate::rt::JHashMap;
use crate::{arena_identity, arena_object};

pub struct PoolInterceptorData {
    pub map_old_to_new_names: JHashMap<String, String>,
    pub map_new_to_old_names: JHashMap<String, String>,
}

arena_object!(pub PoolInterceptor => PoolInterceptorData, Session);
arena_identity!(PoolInterceptor);

impl PoolInterceptor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> PoolInterceptor {
        PoolInterceptor::alloc(PoolInterceptorData {
            map_old_to_new_names: JHashMap::new(),
            map_new_to_old_names: JHashMap::new(),
        })
    }

    pub fn add_name(self, old_name: &str, new_name: &str) {
        self.with_mut(|d| {
            d.map_old_to_new_names
                .put(old_name.to_string(), new_name.to_string());
            d.map_new_to_old_names
                .put(new_name.to_string(), old_name.to_string());
        })
    }

    pub fn get_name(self, old_name: &str) -> Option<String> {
        self.with(|d| d.map_old_to_new_names.get_cloned(&old_name.to_string()))
    }

    pub fn get_old_name(self, new_name: &str) -> Option<String> {
        self.with(|d| d.map_new_to_old_names.get_cloned(&new_name.to_string()))
    }
}
