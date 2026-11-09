//! Port of `org/jetbrains/java/decompiler/main/rels/DecompileRecord.java`.

use crate::kuna_trace;
use crate::root_statement::RootStatement;
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;
use crate::{arena_identity, arena_object};

pub struct DecompileRecordData {
    pub names: Vec<String>,
    pub mt: StructMethod,
    pub main_loop_iteration: i32,
    pub merge_loop_iteration: i32,
}

arena_object!(pub DecompileRecord => DecompileRecordData, Class);
arena_identity!(DecompileRecord);

impl DecompileRecord {
    pub fn new(mt: StructMethod) -> DecompileRecord {
        DecompileRecord::alloc(DecompileRecordData {
            names: Vec::new(),
            mt,
            main_loop_iteration: 0,
            merge_loop_iteration: 0,
        })
    }

    /// Java: add(String, RootStatement)
    pub fn add(self, name: &str, root: Option<RootStatement>) {
        let (main_loop_iteration, merge_loop_iteration) =
            self.with(|d| (d.main_loop_iteration, d.merge_loop_iteration));
        let mut export_name = String::new();
        if main_loop_iteration > 0 {
            export_name.push_str(&format!("Loop_{main_loop_iteration}_"));
        }

        if merge_loop_iteration > 0 {
            export_name.push_str(&format!("Merge_{merge_loop_iteration}_"));
        }

        export_name.push_str(name);

        self.add_name(&export_name);

        let Some(root) = root else {
            return;
        };

        if kuna_trace::enabled() {
            kuna_trace::step(self.with(|d| d.mt), &export_name, root);
        }

        ValidationHelper::validate_statement(root);
    }

    /// Java: add(String)
    pub fn add_name(self, name: &str) {
        self.with_mut(|d| d.names.push(name.to_string()));
    }

    pub fn increment_main_loop(self) {
        self.with_mut(|d| d.main_loop_iteration = d.main_loop_iteration.wrapping_add(1));
    }

    pub fn increment_merge_loop(self) {
        self.with_mut(|d| d.merge_loop_iteration = d.merge_loop_iteration.wrapping_add(1));
    }

    pub fn reset_main_loop(self) {
        self.with_mut(|d| d.main_loop_iteration = 0);
    }

    pub fn reset_merge_loop(self) {
        self.with_mut(|d| d.merge_loop_iteration = 0);
    }

    pub fn get_names(self) -> Vec<String> {
        self.with(|d| d.names.clone())
    }

    pub fn print(self) {
        let names = self.get_names();
        for (i, name) in names.iter().enumerate() {
            println!("{i} {name}");
        }
    }
}
