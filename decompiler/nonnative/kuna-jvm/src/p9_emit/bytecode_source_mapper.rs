//! Port of `org/jetbrains/java/decompiler/main/collectors/BytecodeSourceMapper.java`.

use std::collections::{BTreeMap, BTreeSet};

use crate::bytecode_mapping_tracer::BytecodeMappingTracer;
use crate::decompiler_context::DecompilerContext;
use crate::rt::{JHashMap, JLinkedHashMap};
use crate::text_buffer::TextBuffer;
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

    pub fn dump_mapping(self, buffer: &mut TextBuffer, offsets_to_hex: bool) {
        let (mapping, lines_mapping, unmapped_lines, offset_total) = self.with(|d| {
            (
                d.mapping
                    .iter()
                    .map(|(class, methods)| (class.clone(), methods.entry_vec()))
                    .collect::<Vec<_>>(),
                d.lines_mapping.clone(),
                d.unmapped_lines.clone(),
                d.offset_total,
            )
        });
        if mapping.is_empty() && lines_mapping.is_empty() {
            return;
        }

        let line_separator = DecompilerContext::get_new_line_separator();

        for (class_name, class_mapping) in mapping {
            buffer.append(&format!("class '{class_name}' {{{line_separator}"));

            let mut is_first_method = true;
            for (method_name, method_mapping) in class_mapping {
                if !is_first_method {
                    buffer.append_line_separator();
                }

                buffer
                    .append_indent(1)
                    .append(&format!("method '{method_name}' {{{line_separator}"));

                let mut lst_bytecode_offsets = method_mapping.key_vec();
                lst_bytecode_offsets.sort();

                for offset in lst_bytecode_offsets {
                    let line = method_mapping.get_cloned(&offset).unwrap();

                    let str_offset = if offsets_to_hex {
                        format!("{:x}", offset as u32)
                    } else {
                        line.to_string()
                    };
                    buffer
                        .append_indent(2)
                        .append(&str_offset)
                        .append_indent(2)
                        .append(&format!(
                            "{}{}",
                            line.wrapping_add(offset_total),
                            line_separator
                        ));
                }
                buffer.append_indent(1).append("}").append_line_separator();

                is_first_method = false;
            }

            buffer
                .append("}")
                .append_line_separator()
                .append_line_separator();
        }

        buffer.append("Lines mapping:").append_line_separator();
        let sorted: BTreeMap<i32, i32> = lines_mapping.iter().map(|(k, v)| (*k, *v)).collect();
        for (key, value) in &sorted {
            buffer
                .append(&key.to_string())
                .append(" <-> ")
                .append(&value.wrapping_add(offset_total).wrapping_add(1).to_string())
                .append_line_separator();
        }

        if !unmapped_lines.is_empty() {
            buffer.append("Not mapped:").append_line_separator();
            for line in &unmapped_lines {
                if !lines_mapping.contains_key(line) {
                    buffer.append(&line.to_string()).append_line_separator();
                }
            }
        }
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
