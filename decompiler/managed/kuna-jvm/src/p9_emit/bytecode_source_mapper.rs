//! Port of `org/jetbrains/java/decompiler/main/collectors/BytecodeSourceMapper.java`.

use std::collections::BTreeSet;

use crate::bytecode_mapping_tracer::BytecodeMappingTracer;
use crate::rt::{JHashMap, JLinkedHashMap};
use crate::{arena_identity, arena_object};

pub struct BytecodeSourceMapperData {
    pub offset_total: i32,
    /// Class, method, bytecode offset, source line.
    pub mapping: JLinkedHashMap<String, JLinkedHashMap<String, JHashMap<i32, i32>>>,
    /// Original line to decompiled line.
    pub lines_mapping: JHashMap<i32, i32>,
    pub unmapped_lines: BTreeSet<i32>,
}

arena_object!(pub BytecodeSourceMapper => BytecodeSourceMapperData, Class);
arena_identity!(BytecodeSourceMapper);

impl BytecodeSourceMapper {
    pub fn new() -> BytecodeSourceMapper {
        BytecodeSourceMapper::alloc(BytecodeSourceMapperData {
            offset_total: 0,
            mapping: JLinkedHashMap::new(),
            lines_mapping: JHashMap::new(),
            unmapped_lines: BTreeSet::new(),
        })
    }

    pub fn add_mapping(
        self,
        class_name: &str,
        method_name: &str,
        bytecode_offset: i32,
        source_line: i32,
    ) {
        self.with_mut(|d| {
            let class_mapping = d
                .mapping
                .compute_if_absent(class_name.to_string(), JLinkedHashMap::new);
            let method_mapping =
                class_mapping.compute_if_absent(method_name.to_string(), JHashMap::new);

            method_mapping.put_if_absent(bytecode_offset, source_line);
        });
    }

    pub fn add_tracer(self, class_name: &str, method_name: &str, tracer: BytecodeMappingTracer) {
        let entries = tracer.get_mapping().borrow().entry_vec();
        for (key, value) in entries {
            self.add_mapping(class_name, method_name, key, value);
        }
        let original_lines_mapping = tracer.get_original_lines_mapping();
        self.with_mut(|d| d.lines_mapping.put_all(original_lines_mapping.entry_vec()));
        let unmapped_lines = tracer.get_unmapped_lines().borrow().to_vec();
        self.with_mut(|d| d.unmapped_lines.extend(unmapped_lines));
    }

    pub fn add_total_offset(self, offset_total: i32) {
        self.with_mut(|d| d.offset_total = d.offset_total.wrapping_add(offset_total));
    }

    /// Original to decompiled line pairs, flattened; decompiled lines are 1-based.
    pub fn get_original_lines_mapping(self) -> Vec<i32> {
        self.with_mut(|d| {
            let mut res = vec![0; d.lines_mapping.len() * 2];
            let mut i = 0;
            for (key, value) in d.lines_mapping.entry_vec() {
                res[i] = key;
                d.unmapped_lines.remove(&key);
                res[i + 1] = value.wrapping_add(d.offset_total).wrapping_add(1);
                i += 2;
            }
            res
        })
    }
}
