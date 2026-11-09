//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/AnnotationExprent.java`.

use crate::exprent::{Exprent, ExprentKind};
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::{java_subclass, unported};

/// Java `AnnotationExprent.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Normal,
    Marker,
    SingleElement,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::Normal => "NORMAL",
            Type::Marker => "MARKER",
            Type::SingleElement => "SINGLE_ELEMENT",
        }
    }
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

pub struct AnnotationExprentData {
    pub class_name: String,
    pub par_names: Vec<String>,
    pub par_values: Vec<Exprent>,
    pub did_write_already: bool,
}

java_subclass!(pub AnnotationExprent: Exprent, ExprentKind::Annotation(AnnotationExprentData),
               is_annotation, as_annotation, try_as_annotation);

impl JavaEq for AnnotationExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for AnnotationExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl AnnotationExprent {
    pub fn new(
        class_name: String,
        par_names: Vec<String>,
        par_values: Vec<Exprent>,
    ) -> AnnotationExprent {
        let id = Exprent::next_id();
        AnnotationExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Annotation(AnnotationExprentData {
                class_name,
                par_names,
                par_values,
                did_write_already: false,
            }),
        ))
    }

    pub fn get_all_exprents_list(self, list: &mut Vec<Exprent>) {
        self.d(|d| list.extend(d.par_values.iter().copied()));
    }

    pub fn copy(self) -> Exprent {
        let (class_name, par_names, par_values) = self.d(|d| {
            (
                d.class_name.clone(),
                d.par_names.clone(),
                d.par_values.clone(),
            )
        });
        let mut exps = Vec::new();
        for v in par_values {
            exps.push(v.copy());
        }
        AnnotationExprent::new(class_name, par_names, exps).0
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("AnnotationExprent.toJava")
    }

    #[allow(dead_code)]
    fn is_array_value(par_value: Exprent) -> bool {
        par_value
            .try_as_new()
            .is_some_and(|new_expr| new_expr.is_direct_array_init())
    }

    pub fn get_class_name(self) -> String {
        self.d(|d| d.class_name.clone())
    }

    pub fn get_annotation_type(self) -> Type {
        self.d(|d| {
            if d.par_names.is_empty() {
                Type::Marker
            } else if d.par_names.len() == 1 && d.par_names[0] == "value" {
                Type::SingleElement
            } else {
                Type::Normal
            }
        })
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ann) = o.try_as_annotation() else {
            return false;
        };
        let (class_name, par_names, par_values) = self.d(|d| {
            (
                d.class_name.clone(),
                d.par_names.clone(),
                d.par_values.clone(),
            )
        });
        let (o_class_name, o_par_names, o_par_values) = ann.d(|d| {
            (
                d.class_name.clone(),
                d.par_names.clone(),
                d.par_values.clone(),
            )
        });
        class_name == o_class_name
            && par_names.java_equals(&o_par_names)
            && par_values.java_equals(&o_par_values)
    }

    pub fn get_par_names(self) -> Vec<String> {
        self.d(|d| d.par_names.clone())
    }

    pub fn get_par_values(self) -> Vec<Exprent> {
        self.d(|d| d.par_values.clone())
    }

    pub fn did_write_already(self) -> bool {
        self.d(|d| d.did_write_already)
    }

    pub fn set_did_write_already(self, did_write_already: bool) {
        self.d_mut(|d| d.did_write_already = did_write_already)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let par_values = self.d(|d| d.par_values.clone());
        Exprent::measure_bytecode_list(values.as_deref_mut(), &par_values);
        self.measure_bytecode(values);
    }
}
