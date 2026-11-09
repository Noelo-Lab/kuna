//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/TypeAnnotation.java`.

use std::collections::VecDeque;

use crate::annotation_exprent::AnnotationExprent;
use crate::classes_processor::ClassNode;
use crate::jthrow;
use crate::pair::Pair;
use crate::rt::{JHashSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::unported;
use crate::var_type::VarType;

/// Java `TypeAnnotation.PathKind`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum PathKind {
    DeeperArray,
    DeeperNested,
    OnWildcard,
    GenericArgument,
}

impl PathKind {
    pub const VALUES: [PathKind; 4] = [
        PathKind::DeeperArray,
        PathKind::DeeperNested,
        PathKind::OnWildcard,
        PathKind::GenericArgument,
    ];

    pub fn name(self) -> &'static str {
        match self {
            PathKind::DeeperArray => "DEEPER_ARRAY",
            PathKind::DeeperNested => "DEEPER_NESTED",
            PathKind::OnWildcard => "ON_WILDCARD",
            PathKind::GenericArgument => "GENERIC_ARGUMENT",
        }
    }
}

impl JavaHash for PathKind {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for PathKind {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

/// Java record `TypeAnnotation.PathValue`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PathValue {
    kind: PathKind,
    type_arg: i32,
}

impl PathValue {
    pub fn new(kind: PathKind, type_arg: i32) -> PathValue {
        if kind != PathKind::GenericArgument && type_arg != 0 {
            jthrow!(IllegalStateException; "Illegal path data");
        }
        PathValue { kind, type_arg }
    }

    pub fn kind(&self) -> PathKind {
        self.kind
    }

    pub fn type_arg(&self) -> i32 {
        self.type_arg
    }
}

/// One pending annotation while a type is written: the path still to walk and the annotation.
pub type TypeAnnotationEntry = Pair<VecDeque<PathValue>, AnnotationExprent>;

#[derive(Clone)]
pub struct TypeAnnotation {
    target: i32,
    pub path: Vec<PathValue>,
    annotation: AnnotationExprent,
}

impl TypeAnnotation {
    pub const CLASS_TYPE_PARAMETER: i32 = 0x00;
    pub const METHOD_TYPE_PARAMETER: i32 = 0x01;
    pub const SUPER_TYPE_REFERENCE: i32 = 0x10;
    pub const CLASS_TYPE_PARAMETER_BOUND: i32 = 0x11;
    pub const METHOD_TYPE_PARAMETER_BOUND: i32 = 0x12;
    pub const FIELD: i32 = 0x13;
    pub const METHOD_RETURN_TYPE: i32 = 0x14;
    pub const METHOD_RECEIVER: i32 = 0x15;
    pub const METHOD_PARAMETER: i32 = 0x16;
    pub const THROWS_REFERENCE: i32 = 0x17;
    pub const LOCAL_VARIABLE: i32 = 0x40;
    pub const RESOURCE_VARIABLE: i32 = 0x41;
    pub const CATCH_CLAUSE: i32 = 0x42;
    pub const EXPR_INSTANCEOF: i32 = 0x43;
    pub const EXPR_NEW: i32 = 0x44;
    pub const EXPR_CONSTRUCTOR_REF: i32 = 0x45;
    pub const EXPR_METHOD_REF: i32 = 0x46;
    pub const TYPE_ARG_CAST: i32 = 0x47;
    pub const TYPE_ARG_CONSTRUCTOR_CALL: i32 = 0x48;
    pub const TYPE_ARG_METHOD_CALL: i32 = 0x49;
    pub const TYPE_ARG_CONSTRUCTOR_REF: i32 = 0x4A;
    pub const TYPE_ARG_METHOD_REF: i32 = 0x4B;

    pub fn new(target: i32, path: Vec<PathValue>, annotation: AnnotationExprent) -> TypeAnnotation {
        TypeAnnotation {
            target,
            path,
            annotation,
        }
    }

    pub fn get_target_type(&self) -> i32 {
        self.target >> 24
    }

    pub fn get_index(&self) -> i32 {
        self.target & 0xFFFF
    }

    pub fn is_top_level(&self) -> bool {
        self.path.is_empty()
    }

    pub fn as_queue(&self) -> VecDeque<PathValue> {
        self.path.iter().copied().collect()
    }

    pub fn get_annotation(&self) -> AnnotationExprent {
        self.annotation
    }

    #[allow(dead_code)]
    fn is_static_in_context(
        _node: ClassNode,
        _current_node: Option<ClassNode>,
        _outermost_node: Option<ClassNode>,
    ) -> bool {
        unported!("TypeAnnotation.isStaticInContext")
    }

    pub fn append_cast_type_name(
        _buffer: &mut TextBuffer,
        _type_: &VarType,
        _type_annotations: &mut [TypeAnnotationEntry],
        _filter: &JHashSet<String>,
        _vararg: bool,
        _flush: bool,
    ) {
        unported!("TypeAnnotation.appendCastTypeName")
    }

    /// Java: `stepwiseWriteAnnotations(TextBuffer, List, Set, Predicate)`.
    #[allow(dead_code)]
    fn stepwise_write_annotations(
        buffer: &mut TextBuffer,
        type_annotations: &mut [TypeAnnotationEntry],
        filter: &JHashSet<String>,
        can_proceed: &dyn Fn(&PathValue) -> bool,
    ) {
        TypeAnnotation::stepwise_write_annotations_space(
            buffer,
            false,
            type_annotations,
            filter,
            can_proceed,
        )
    }

    /// Java: `stepwiseWriteAnnotations(TextBuffer, boolean, List, Set, Predicate)`.
    #[allow(dead_code)]
    fn stepwise_write_annotations_space(
        _buffer: &mut TextBuffer,
        _extra_space: bool,
        _type_annotations: &mut [TypeAnnotationEntry],
        _filter: &JHashSet<String>,
        _can_proceed: &dyn Fn(&PathValue) -> bool,
    ) {
        unported!("TypeAnnotation.stepwiseWriteAnnotations")
    }

    #[allow(dead_code)]
    fn dump_unwritten_annotations(
        _buffer: &mut TextBuffer,
        _type_annotations: &mut [TypeAnnotationEntry],
    ) {
        unported!("TypeAnnotation.dumpUnwrittenAnnotations")
    }
}
