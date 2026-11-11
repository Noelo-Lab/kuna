//! Port of `org/jetbrains/java/decompiler/main/collectors/BytecodeMappingTracer.java`.

use std::cell::Cell;

use crate::rt::{BitSet, JHashMap, JHashSet, Shared};
use crate::struct_line_number_table_attribute::StructLineNumberTableAttribute;
use crate::{arena_identity, arena_object};

pub struct BytecodeMappingTracerData {
    pub current_source_line: i32,
    pub line_number_table: Option<StructLineNumberTableAttribute>,
    /// Bytecode offset to source line.
    pub mapping: Shared<JHashMap<i32, i32>>,
    pub unmapped_lines: Shared<JHashSet<i32>>,
}

arena_object!(pub BytecodeMappingTracer => BytecodeMappingTracerData, Class);
arena_identity!(BytecodeMappingTracer);

thread_local! {
    static DUMMY: Cell<Option<BytecodeMappingTracer>> = const { Cell::new(None) };
}

impl BytecodeMappingTracer {
    /// `BytecodeMappingTracer.DUMMY`: one shared sink per root class.
    #[allow(non_snake_case)]
    pub fn DUMMY() -> BytecodeMappingTracer {
        DUMMY.with(|cell| match cell.get() {
            Some(tracer) if tracer.is_live() => tracer,
            _ => {
                let tracer = BytecodeMappingTracer::new();
                cell.set(Some(tracer));
                tracer
            }
        })
    }

    /// The shared sink if one has been created in this run, without creating it.
    pub(crate) fn existing_dummy() -> Option<BytecodeMappingTracer> {
        DUMMY.with(Cell::get).filter(|tracer| tracer.is_live())
    }

    /// Java: BytecodeMappingTracer()
    pub fn new() -> BytecodeMappingTracer {
        BytecodeMappingTracer::new_line(0)
    }

    /// Java: BytecodeMappingTracer(int)
    pub fn new_line(initial_source_line: i32) -> BytecodeMappingTracer {
        BytecodeMappingTracer::alloc(BytecodeMappingTracerData {
            current_source_line: initial_source_line,
            line_number_table: None,
            mapping: Shared::new(JHashMap::new()),
            unmapped_lines: Shared::new(JHashSet::new()),
        })
    }

    /// Java: incrementCurrentSourceLine()
    pub fn increment_current_source_line(self) {
        self.with_mut(|d| d.current_source_line = d.current_source_line.wrapping_add(1));
    }

    /// Java: incrementCurrentSourceLine(int)
    pub fn increment_current_source_line_by(self, number_lines: i32) {
        self.with_mut(|d| d.current_source_line = d.current_source_line.wrapping_add(number_lines));
    }

    /// Java: addMapping(int)
    pub fn add_mapping(self, bytecode_offset: i32) {
        let (mapping, current_source_line) =
            self.with(|d| (d.mapping.clone(), d.current_source_line));
        mapping
            .borrow_mut()
            .put_if_absent(bytecode_offset, current_source_line);
    }

    /// Java: addMapping(BitSet)
    pub fn add_mapping_bitset(self, bytecode_offsets: Option<&BitSet>) {
        if let Some(bytecode_offsets) = bytecode_offsets {
            let mut i = bytecode_offsets.next_set_bit(0);
            while i >= 0 {
                self.add_mapping(i);
                i = bytecode_offsets.next_set_bit(i as usize + 1);
            }
        }
    }

    pub fn add_tracer(self, tracer: Option<BytecodeMappingTracer>) {
        if let Some(tracer) = tracer {
            let entries = tracer.get_mapping().borrow().entry_vec();
            let mapping = self.get_mapping();
            for (key, value) in entries {
                mapping.borrow_mut().put_if_absent(key, value);
            }
        }
    }

    pub fn get_mapping(self) -> Shared<JHashMap<i32, i32>> {
        self.with(|d| d.mapping.clone())
    }

    pub fn get_current_source_line(self) -> i32 {
        self.with(|d| d.current_source_line)
    }

    pub fn set_current_source_line(self, current_source_line: i32) {
        self.with_mut(|d| d.current_source_line = current_source_line);
    }

    pub fn set_line_number_table(self, line_number_table: Option<StructLineNumberTableAttribute>) {
        self.with_mut(|d| d.line_number_table = line_number_table);
    }

    pub fn get_unmapped_lines(self) -> Shared<JHashSet<i32>> {
        self.with(|d| d.unmapped_lines.clone())
    }

    /// Original source line to decompiled line; records lines that cannot be mapped.
    pub fn get_original_lines_mapping(self) -> JHashMap<i32, i32> {
        let (line_number_table, mapping, unmapped_lines) = self.with(|d| {
            (
                d.line_number_table,
                d.mapping.clone(),
                d.unmapped_lines.clone(),
            )
        });
        let Some(line_number_table) = line_number_table else {
            return JHashMap::new();
        };

        let mut res = JHashMap::new();

        let data = line_number_table.get_raw_data();
        let mut i = 0;
        while i < data.len() {
            let original_offset = data[i];
            let original_line = data[i + 1];
            let new_line = mapping.borrow().get_cloned(&original_offset);
            match new_line {
                Some(new_line) => {
                    res.put(original_line, new_line);
                }
                None => {
                    unmapped_lines.borrow_mut().add(original_line);
                }
            }
            i += 2;
        }

        let entries = mapping.borrow().entry_vec();
        for (key, value) in entries {
            let original_line = line_number_table.find_line_number(key);
            if original_line > -1 && !res.contains_key(&original_line) {
                res.put(original_line, value);
                unmapped_lines.borrow_mut().remove(&original_line);
            }
        }
        res
    }
}
