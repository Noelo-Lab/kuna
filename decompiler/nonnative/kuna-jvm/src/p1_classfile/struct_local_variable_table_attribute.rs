//! Port of `org/jetbrains/java/decompiler/struct/attr/StructLocalVariableTableAttribute.java`.
//!
//! `LocalVariable` is a plain value: its mutable fields change only while the
//! table is read. Java's `Stream` results are returned as vectors.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::jthrow;
use crate::rt::{BitSet, JHashMap};
use crate::statement::Statement;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::struct_local_variable_type_table_attribute::StructLocalVariableTypeTableAttribute;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

#[derive(Default)]
pub struct StructLocalVariableTableAttributeData {
    pub local_variables: Vec<LocalVariable>,
    /// False while `local_variables` is Java's immutable `Collections.emptyList()`.
    pub local_variables_mutable: bool,
    pub index_version: JHashMap<i32, i32>,
}

attribute_view!(
    StructLocalVariableTableAttribute,
    LocalVariableTable,
    StructLocalVariableTableAttributeData,
    is_local_variable_table,
    as_local_variable_table,
    try_as_local_variable_table
);

impl StructLocalVariableTableAttribute {
    pub fn new() -> StructLocalVariableTableAttribute {
        StructLocalVariableTableAttribute::alloc_view(
            StructLocalVariableTableAttributeData::default(),
        )
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_short();
        if len > 0 {
            let mut local_variables = Vec::with_capacity(len as usize);
            self.d_mut(|d| d.index_version = JHashMap::new());

            for _ in 0..len {
                let start_pc = data.read_unsigned_short();
                let length = data.read_unsigned_short();
                let name_index = data.read_unsigned_short();
                let descriptor_index = data.read_unsigned_short();
                let var_index = data.read_unsigned_short();
                local_variables.push(LocalVariable::new(
                    start_pc,
                    length,
                    pool.get_primitive_constant(name_index)
                        .expect("constant")
                        .get_string()
                        .to_string(),
                    pool.get_primitive_constant(descriptor_index)
                        .expect("constant")
                        .get_string()
                        .to_string(),
                    var_index,
                ));
            }
            local_variables.sort_by(|a, b| a.compare_to(b).cmp(&0));
            self.d_mut(|d| {
                d.local_variables = local_variables;
                d.local_variables_mutable = true;
                version_variables(d);
            });
        } else {
            self.d_mut(|d| {
                d.local_variables = Vec::new();
                d.local_variables_mutable = false;
            });
        }
    }

    pub fn add(self, attr: StructLocalVariableTableAttribute) {
        let other = attr.d(|d| d.local_variables.clone());
        self.d_mut(|d| {
            if !d.local_variables_mutable && !other.is_empty() {
                jthrow!(UnsupportedOperationException; "");
            }
            d.local_variables.extend(other);
            version_variables(d);
        });
    }

    pub fn get_name(self, index: i32, visible_offset: i32) -> Option<String> {
        self.matching_vars(index, visible_offset)
            .into_iter()
            .next()
            .map(|v| v.name)
    }

    pub fn get_descriptor(self, index: i32, visible_offset: i32) -> Option<String> {
        self.matching_vars(index, visible_offset)
            .into_iter()
            .next()
            .map(|v| v.descriptor)
    }

    /// Java: `matchingVars(int, int)`.
    pub fn matching_vars(self, index: i32, visible_offset: i32) -> Vec<LocalVariable> {
        self.d(|d| {
            d.local_variables
                .iter()
                .filter(|v| {
                    v.index == index
                        && (visible_offset >= v.start_pc
                            && visible_offset < v.start_pc.wrapping_add(v.length))
                })
                .cloned()
                .collect()
        })
    }

    /// Java: `matchingVars(int)`.
    pub fn matching_vars_index(self, index: i32) -> Vec<LocalVariable> {
        self.d(|d| {
            d.local_variables
                .iter()
                .filter(|v| v.index == index)
                .cloned()
                .collect()
        })
    }

    /// Java: `matchingVars(Statement)`.
    pub fn matching_vars_stat(self, stat: Statement) -> Vec<LocalVariable> {
        let mut values = BitSet::new();
        stat.get_offset(&mut values);
        self.get_range(values.next_set_bit(0), values.length() as i32 - 1)
    }

    pub fn get_range(self, start: i32, end: i32) -> Vec<LocalVariable> {
        self.d(|d| {
            d.local_variables
                .iter()
                .filter(|v| v.get_start() >= start && v.get_end() <= end)
                .cloned()
                .collect()
        })
    }

    pub fn contains_name(self, name: &str) -> bool {
        self.d(|d| d.local_variables.iter().any(|v| v.name == name))
    }

    pub fn get_map_names(self) -> JHashMap<VarVersionPair, String> {
        self.d(|d| {
            let mut map = JHashMap::new();
            for v in &d.local_variables {
                map.put(v.version, v.name.clone());
            }
            map
        })
    }

    pub fn get_variables(self) -> Vec<LocalVariable> {
        self.d(|d| d.local_variables.clone())
    }

    pub fn merge_signatures(self, lvtt: StructLocalVariableTypeTableAttribute) {
        let types = lvtt.backing_attribute().get_variables();
        self.d_mut(|d| {
            for type_ in &types {
                if let Some(lv) = d
                    .local_variables
                    .iter_mut()
                    .find(|t| t.compare_to(type_) == 0)
                {
                    lv.signature = Some(type_.descriptor.clone());
                }
            }
        });
    }
}

/// `versionVariables`: Java's `version++` result is discarded, so every
/// variable's version is the first one recorded for its index (always 1).
fn version_variables(d: &mut StructLocalVariableTableAttributeData) {
    for var in d.local_variables.iter_mut() {
        let version = d.index_version.get(&var.index).copied().unwrap_or(1);
        d.index_version.put(var.index, version);
        var.version = VarVersionPair::new(var.index, version);
    }
}

#[derive(Clone, Debug)]
pub struct LocalVariable {
    pub start_pc: i32,
    pub length: i32,
    pub name: String,
    pub descriptor: String,
    pub index: i32,
    pub signature: Option<String>,
    pub version: VarVersionPair,
}

impl LocalVariable {
    pub fn new(
        start_pc: i32,
        length: i32,
        name: String,
        descriptor: String,
        index: i32,
    ) -> LocalVariable {
        LocalVariable {
            start_pc,
            length,
            name,
            descriptor,
            index,
            signature: None,
            version: VarVersionPair::new(index, 0),
        }
    }

    /// `Comparable.compareTo`.
    pub fn compare_to(&self, o: &LocalVariable) -> i32 {
        if self.index != o.index {
            return self.index.wrapping_sub(o.index);
        }
        if self.start_pc != o.start_pc {
            return self.start_pc.wrapping_sub(o.start_pc);
        }
        self.length.wrapping_sub(o.length)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
    }

    pub fn get_signature(&self) -> Option<String> {
        self.signature.clone()
    }

    pub fn get_start(&self) -> i32 {
        self.start_pc
    }

    pub fn get_end(&self) -> i32 {
        self.start_pc.wrapping_add(self.length)
    }

    pub fn get_version(&self) -> VarVersionPair {
        self.version
    }

    pub fn get_var_type(&self) -> VarType {
        VarType::new_signature(&self.descriptor)
    }

    pub fn rename(&self, new_name: String) -> LocalVariable {
        let mut ret = LocalVariable::new(
            self.start_pc,
            self.length,
            new_name,
            self.descriptor.clone(),
            self.index,
        );
        ret.signature = self.signature.clone();
        ret
    }
}

impl std::fmt::Display for LocalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let signature = match &self.signature {
            Some(s) => format!("<{}> ", s),
            None => " ".to_string(),
        };
        write!(
            f,
            "'({},{}-{}){}{}{}'",
            self.index,
            self.start_pc,
            self.get_end(),
            self.descriptor,
            signature,
            self.name
        )
    }
}
