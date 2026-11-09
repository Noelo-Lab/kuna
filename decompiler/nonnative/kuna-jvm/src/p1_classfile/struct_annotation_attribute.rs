//! Port of `org/jetbrains/java/decompiler/struct/attr/StructAnnotationAttribute.java`.
//!
//! Java parses annotations straight into exprents, but exprents are
//! class-scope arena objects while attributes live for the session. The parse
//! (every pool lookup, descriptor parse and validation, in Java's order) runs
//! at load time into an [`AnnotationElement`] tree; the exprents are built from
//! it, in Java's construction order, on first use in each class scope and
//! cached until that scope ends.

use crate::annotation_exprent::AnnotationExprent;
use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::constant_pool::ConstantPool;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::jthrow;
use crate::new_exprent::NewExprent;
use crate::primitive_constant::{ConstValue as PoolValue, PrimitiveConstant};
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::var_type::VarType;

/// A parsed `annotation` structure, in session scope (pool indices kept for diagnostics).
#[derive(Clone)]
pub struct AnnotationRecord {
    pub type_index: i32,
    /// `new VarType(className).value`.
    pub class_name: Option<String>,
    pub name_indices: Vec<i32>,
    pub names: Vec<String>,
    pub values: Vec<AnnotationElement>,
}

/// A parsed `element_value`, in session scope.
#[derive(Clone)]
pub enum AnnotationElement {
    /// `'e'`: an enum constant.
    Enum {
        type_index: i32,
        const_index: i32,
        const_name: String,
        descr: FieldDescriptor,
    },
    /// `'c'`: a class literal, already resolved to its printed name.
    Class { index: i32, value: String },
    /// `'['`.
    Array { elements: Vec<AnnotationElement> },
    /// `'@'`.
    Annotation(AnnotationRecord),
    /// `B C D F I J S Z s`: a constant of the type the tag names.
    Const {
        tag: u8,
        index: i32,
        cn: PrimitiveConstant,
    },
}

impl AnnotationRecord {
    pub fn to_exprent(&self) -> AnnotationExprent {
        let values: Vec<Exprent> = self
            .values
            .iter()
            .map(AnnotationElement::to_exprent)
            .collect();
        AnnotationExprent::new(
            self.class_name.clone().unwrap_or_default(),
            self.names.clone(),
            values,
        )
    }
}

impl AnnotationElement {
    pub fn to_exprent(&self) -> Exprent {
        match self {
            AnnotationElement::Enum {
                const_name, descr, ..
            } => FieldExprent::new(
                const_name.clone(),
                descr.type_.value.as_deref().unwrap_or_default().to_string(),
                true,
                None,
                descr.clone(),
                None,
            )
            .into(),
            AnnotationElement::Class { value, .. } => {
                ConstExprent::new(VarType::VARTYPE_CLASS(), Some(value.clone().into()), None).into()
            }
            AnnotationElement::Array { elements } => {
                let elements: Vec<Exprent> =
                    elements.iter().map(AnnotationElement::to_exprent).collect();

                let new_type = if elements.is_empty() {
                    VarType::new_value(CodeType::Object, 1, "java/lang/Object")
                } else {
                    let element_type = elements[0].get_expr_type();
                    VarType::new_value(element_type.type_, 1, &element_type.value)
                };

                let new_expr = NewExprent::new(new_type, Vec::new(), None);
                new_expr.set_direct_array_init(true);
                new_expr.set_lst_array_elements(elements);
                new_expr.into()
            }
            AnnotationElement::Annotation(record) => record.to_exprent().into(),
            AnnotationElement::Const { tag, cn, .. } => {
                let const_type = match tag {
                    b'B' => VarType::VARTYPE_BYTE(),
                    b'C' => VarType::VARTYPE_CHAR(),
                    b'D' => VarType::VARTYPE_DOUBLE(),
                    b'F' => VarType::VARTYPE_FLOAT(),
                    b'I' => VarType::VARTYPE_INT(),
                    b'J' => VarType::VARTYPE_LONG(),
                    b'S' => VarType::VARTYPE_SHORT(),
                    b'Z' => VarType::VARTYPE_BOOLEAN(),
                    _ => VarType::VARTYPE_STRING(),
                };
                ConstExprent::new(const_type, cn.value.as_ref().map(expr_value), None).into()
            }
        }
    }
}

#[derive(Default)]
pub struct StructAnnotationAttributeData {
    pub source: Vec<AnnotationRecord>,
    /// Class-scope exprents built from `source`.
    pub annotations: Option<Vec<AnnotationExprent>>,
    /// Set by `set_annotations`: the cache is no longer derived from `source`.
    pub pinned: bool,
}

attribute_view!(
    StructAnnotationAttribute,
    Annotation,
    StructAnnotationAttributeData,
    is_annotation,
    as_annotation,
    try_as_annotation
);

impl StructAnnotationAttribute {
    pub fn new() -> StructAnnotationAttribute {
        StructAnnotationAttribute::alloc_view(StructAnnotationAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let source = StructAnnotationAttribute::read_annotations(pool, data);
        self.d_mut(|d| {
            d.source = source;
            d.annotations = None;
            d.pinned = false;
        });
    }

    /// Java: `parseAnnotations(ConstantPool, DataInputStream)`.
    pub fn parse_annotations(pool: ConstantPool, data: &mut ByteCursor) -> Vec<AnnotationExprent> {
        StructAnnotationAttribute::read_annotations(pool, data)
            .iter()
            .map(AnnotationRecord::to_exprent)
            .collect()
    }

    /// Java: `parseAnnotation(DataInputStream, ConstantPool)`.
    pub fn parse_annotation(data: &mut ByteCursor, pool: ConstantPool) -> AnnotationExprent {
        StructAnnotationAttribute::read_annotation(data, pool).to_exprent()
    }

    /// Java: `parseAnnotationElement(DataInputStream, ConstantPool)`.
    pub fn parse_annotation_element(data: &mut ByteCursor, pool: ConstantPool) -> Exprent {
        StructAnnotationAttribute::read_annotation_element(data, pool).to_exprent()
    }

    /// `parseAnnotations` up to the exprents.
    pub fn read_annotations(pool: ConstantPool, data: &mut ByteCursor) -> Vec<AnnotationRecord> {
        let len = data.read_unsigned_short();
        let mut annotations = Vec::with_capacity(len as usize);
        for _ in 0..len {
            annotations.push(StructAnnotationAttribute::read_annotation(data, pool));
        }
        annotations
    }

    /// `parseAnnotation` up to the exprents.
    pub fn read_annotation(data: &mut ByteCursor, pool: ConstantPool) -> AnnotationRecord {
        let type_index = data.read_unsigned_short();
        let class_name = pool
            .get_primitive_constant(type_index)
            .expect("constant")
            .get_string()
            .to_string();

        let len = data.read_unsigned_short();
        let mut name_indices = Vec::with_capacity(len as usize);
        let mut names = Vec::with_capacity(len as usize);
        let mut values = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let name_index = data.read_unsigned_short();
            name_indices.push(name_index);
            names.push(
                pool.get_primitive_constant(name_index)
                    .expect("constant")
                    .get_string()
                    .to_string(),
            );
            values.push(StructAnnotationAttribute::read_annotation_element(
                data, pool,
            ));
        }

        AnnotationRecord {
            type_index,
            class_name: VarType::new_signature(&class_name)
                .value
                .as_deref()
                .map(str::to_string),
            name_indices,
            names,
            values,
        }
    }

    /// `parseAnnotationElement` up to the exprents.
    pub fn read_annotation_element(data: &mut ByteCursor, pool: ConstantPool) -> AnnotationElement {
        let tag = data.read_unsigned_byte();

        match tag as u8 {
            b'e' => {
                let type_index = data.read_unsigned_short();
                let class_name = pool
                    .get_primitive_constant(type_index)
                    .expect("constant")
                    .get_string()
                    .to_string();
                let const_index = data.read_unsigned_short();
                let const_name = pool
                    .get_primitive_constant(const_index)
                    .expect("constant")
                    .get_string()
                    .to_string();
                let descr = FieldDescriptor::parse_descriptor(&class_name);
                AnnotationElement::Enum {
                    type_index,
                    const_index,
                    const_name,
                    descr,
                }
            }
            b'c' => {
                let index = data.read_unsigned_short();
                let descriptor = pool
                    .get_primitive_constant(index)
                    .expect("constant")
                    .get_string()
                    .to_string();
                let type_ = FieldDescriptor::parse_descriptor(&descriptor).type_.clone();

                let value = match type_.type_ {
                    CodeType::Object => type_.value.as_deref().unwrap_or_default().to_string(),
                    CodeType::Byte => "byte".to_string(),
                    CodeType::Char => "char".to_string(),
                    CodeType::Double => "double".to_string(),
                    CodeType::Float => "float".to_string(),
                    CodeType::Int => "int".to_string(),
                    CodeType::Long => "long".to_string(),
                    CodeType::Short => "short".to_string(),
                    CodeType::Boolean => "boolean".to_string(),
                    CodeType::Void => "void".to_string(),
                    other => jthrow!("invalid class type: {}", code_type_name(other)),
                };
                AnnotationElement::Class { index, value }
            }
            b'[' => {
                let mut elements = Vec::new();
                let len = data.read_unsigned_short();
                for _ in 0..len {
                    elements.push(StructAnnotationAttribute::read_annotation_element(
                        data, pool,
                    ));
                }
                AnnotationElement::Array { elements }
            }
            b'@' => AnnotationElement::Annotation(StructAnnotationAttribute::read_annotation(
                data, pool,
            )),
            _ => {
                let index = data.read_unsigned_short();
                let cn = pool.get_primitive_constant(index);
                match tag as u8 {
                    b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's' => {
                        AnnotationElement::Const {
                            tag: tag as u8,
                            index,
                            cn: cn.expect("constant"),
                        }
                    }
                    _ => jthrow!("invalid element type!"),
                }
            }
        }
    }

    /// The annotations as class-scope exprents, built on first use in a class scope.
    pub fn get_annotations(self) -> Vec<AnnotationExprent> {
        let (cached, pinned) = self.d(|d| (d.annotations.clone(), d.pinned));
        if let Some(cached) = cached {
            if pinned || cached.iter().all(|a| a.is_live()) {
                return cached;
            }
        }
        let source = self.d(|d| d.source.clone());
        let built: Vec<AnnotationExprent> =
            source.iter().map(AnnotationRecord::to_exprent).collect();
        self.d_mut(|d| d.annotations = Some(built.clone()));
        built
    }

    /// The parsed annotations, without building exprents.
    pub fn get_records(self) -> Vec<AnnotationRecord> {
        self.d(|d| d.source.clone())
    }

    pub fn set_annotations(self, annotations: Vec<AnnotationExprent>) {
        self.d_mut(|d| {
            d.annotations = Some(annotations);
            d.pinned = true;
        })
    }
}

/// `cn.value` as the `Object` a `ConstExprent` holds.
fn expr_value(v: &PoolValue) -> ConstValue {
    match v {
        PoolValue::Integer(i) => ConstValue::Int(*i),
        PoolValue::Long(l) => ConstValue::Long(*l),
        PoolValue::Float(f) => ConstValue::Float(*f),
        PoolValue::Double(d) => ConstValue::Double(*d),
        PoolValue::String(s) => ConstValue::String(s.to_string()),
    }
}

/// `CodeType.toString()`: the constant's name.
fn code_type_name(t: CodeType) -> String {
    format!("{:?}", t).to_uppercase()
}
