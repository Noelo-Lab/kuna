//! Port of `org/vineflower/kotlin/metadata/MetadataNameResolver.java`.
//!
//! The expanded record list is kept as runs (one per record) instead of one
//! entry per index, so a huge `range` costs nothing; a total past Java's
//! largest array is the `OutOfMemoryError` the `ArrayList` would throw.

use std::rc::Rc;

use crate::kt_proto::{Operation, Record, StringTableTypes};
use crate::rt::jstring::from_utf16;
use crate::rt::{throw, JavaStr};

const PREDEFINED: [&str; 44] = [
    "kotlin/Any",
    "kotlin/Nothing",
    "kotlin/Unit",
    "kotlin/Throwable",
    "kotlin/Number",
    "kotlin/Byte",
    "kotlin/Double",
    "kotlin/Float",
    "kotlin/Int",
    "kotlin/Long",
    "kotlin/Short",
    "kotlin/Boolean",
    "kotlin/Char",
    "kotlin/CharSequence",
    "kotlin/String",
    "kotlin/Comparable",
    "kotlin/Enum",
    "kotlin/Array",
    "kotlin/ByteArray",
    "kotlin/DoubleArray",
    "kotlin/FloatArray",
    "kotlin/IntArray",
    "kotlin/LongArray",
    "kotlin/ShortArray",
    "kotlin/BooleanArray",
    "kotlin/CharArray",
    "kotlin/Cloneable",
    "kotlin/Annotation",
    "kotlin/collections/Iterable",
    "kotlin/collections/MutableIterable",
    "kotlin/collections/Collection",
    "kotlin/collections/MutableCollection",
    "kotlin/collections/List",
    "kotlin/collections/MutableList",
    "kotlin/collections/Set",
    "kotlin/collections/MutableSet",
    "kotlin/collections/Map",
    "kotlin/collections/MutableMap",
    "kotlin/collections/Map.Entry",
    "kotlin/collections/MutableMap.MutableEntry",
    "kotlin/collections/Iterator",
    "kotlin/collections/MutableIterator",
    "kotlin/collections/ListIterator",
    "kotlin/collections/MutableListIterator",
];

/// `ArraysSupport.SOFT_MAX_ARRAY_LENGTH`.
const MAX_LIST_SIZE: i64 = i32::MAX as i64 - 8;

pub struct MetadataNameResolverData {
    pub types: StringTableTypes,
    pub strings: Vec<String>,
    /// `(first index, record)` for every record with a positive range.
    runs: Vec<(i64, Record)>,
    size: i64,
}

#[derive(Clone)]
pub struct MetadataNameResolver(Rc<MetadataNameResolverData>);

impl MetadataNameResolver {
    pub fn new(types: StringTableTypes, strings: Vec<String>) -> MetadataNameResolver {
        let mut runs = Vec::new();
        let mut size: i64 = 0;
        for record in types.get_record_list() {
            let range = record.get_range();
            if range > 0 {
                runs.push((size, record));
                size += range as i64;
                if size > MAX_LIST_SIZE {
                    throw("OutOfMemoryError", "Java heap space");
                }
            }
        }
        MetadataNameResolver(Rc::new(MetadataNameResolverData {
            types,
            strings,
            runs,
            size,
        }))
    }

    pub fn get_types(&self) -> &StringTableTypes {
        &self.0.types
    }

    pub fn get_strings(&self) -> &[String] {
        &self.0.strings
    }

    /// `records.size()`.
    pub fn records_size(&self) -> i32 {
        self.0.size as i32
    }

    /// `records.get(idx)`.
    fn record(&self, idx: i32) -> &Record {
        if idx < 0 || idx as i64 >= self.0.size {
            throw(
                "IndexOutOfBoundsException",
                format!("Index {} out of bounds for length {}", idx, self.0.size),
            );
        }
        let run = self
            .0
            .runs
            .partition_point(|(start, _)| *start <= idx as i64)
            - 1;
        &self.0.runs[run].1
    }

    pub fn resolve(&self, idx: i32) -> String {
        let record = self.record(idx);

        let mut string = if record.has_string() {
            record.get_string()
        } else if record.has_predefined_index()
            && (0..PREDEFINED.len() as i32).contains(&record.get_predefined_index())
        {
            PREDEFINED[record.get_predefined_index() as usize].to_string()
        } else {
            let strings = &self.0.strings;
            if idx as usize >= strings.len() {
                throw(
                    "ArrayIndexOutOfBoundsException",
                    format!("Index {} out of bounds for length {}", idx, strings.len()),
                );
            }
            strings[idx as usize].clone()
        };

        if record.get_substring_index_count() >= 2 {
            let l = record.get_replace_char_list();
            let begin = replace_char_get(&l, 0);
            let end = replace_char_get(&l, 1);

            if 0 <= begin && begin <= end && end as usize <= string.jlen() {
                string = string.jsubstring(begin as usize, end as usize);
            }
        }

        if record.get_replace_char_count() >= 2 {
            let l = record.get_replace_char_list();
            let from = l[0];
            let to = l[1];

            if 0 <= from && from <= to && to <= 65535 {
                let (from, to) = (from as u16, to as u16);
                let units: Vec<u16> = string
                    .utf16()
                    .into_iter()
                    .map(|c| if c == from { to } else { c })
                    .collect();
                string = from_utf16(&units);
            }
        }

        match record.get_operation() {
            Operation::InternalToClassId => string.replace('$', "."),
            Operation::DescToClassId => {
                if string.jlen() >= 2 {
                    string = string.jsubstring(1, string.jlen() - 1);
                }
                string.replace('$', ".")
            }
            Operation::None => string,
        }
    }
}

/// `getReplaceCharList().get(i)`: `Collections.emptyList()` when the record has none.
fn replace_char_get(list: &[i32], index: usize) -> i32 {
    if list.is_empty() {
        throw("IndexOutOfBoundsException", format!("Index: {}", index));
    }
    if index >= list.len() {
        throw(
            "IndexOutOfBoundsException",
            format!("Index {} out of bounds for length {}", index, list.len()),
        );
    }
    list[index]
}
