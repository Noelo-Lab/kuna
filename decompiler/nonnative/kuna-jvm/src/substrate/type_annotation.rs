//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/TypeAnnotation.java`.
//!
//! kuna fix `type_annotation_qualified_name` (`kuna_fixes`): an annotated
//! class type keeps the qualifier the import collector gives its name
//! (`kuna_fix_type_annotation_qualified_name`). kuna fix
//! `type_annotation_argument_scope`: the annotations of a generic argument
//! are looked for only inside that argument
//! (`kuna_fix_type_annotation_argument_scope`). kuna fix
//! `type_annotation_outer_arguments`: an enclosing class of the annotated
//! type keeps its type arguments (`kuna_fix_type_annotation_outer_arguments`).

use std::collections::VecDeque;

use crate::annotation_exprent::AnnotationExprent;
use crate::classes_processor::ClassNode;
use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::generic_type::GenericType;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::substrate::kuna_fix_type_annotation_argument_scope;
use crate::substrate::kuna_fix_type_annotation_outer_arguments;
use crate::substrate::kuna_fix_type_annotation_qualified_name;
use crate::pair::Pair;
use crate::rt::{JHashSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
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

    fn is_static_in_context(
        node: ClassNode,
        current_node: Option<ClassNode>,
        outermost_node: Option<ClassNode>,
    ) -> bool {
        let mut outermost_node = outermost_node;
        let mut current_node = current_node;
        let mut any_static = false;
        while let Some(outermost) = outermost_node {
            if outermost.has_modifier(ACC_STATIC) {
                any_static = true;
            }

            outermost_node = outermost.with(|d| d.parent);
        }

        if any_static {
            loop {
                if Some(node) == current_node {
                    return true;
                }

                current_node = current_node
                    .expect("NullPointerException: currentNode")
                    .with(|d| d.parent);
                if current_node.is_none() {
                    break;
                }
            }
        }

        node.has_modifier(ACC_STATIC)
    }

    pub fn append_cast_type_name(
        buffer: &mut TextBuffer,
        type_: &VarType,
        type_annotations: &mut [TypeAnnotationEntry],
        filter: &JHashSet<String>,
        vararg: bool,
        flush: bool,
    ) {
        let mut append_buffer = TextBuffer::new();
        if type_.array_dim > 0 {
            for i in 0..type_.array_dim {
                TypeAnnotation::stepwise_write_annotations_space(
                    &mut append_buffer,
                    true,
                    type_annotations,
                    filter,
                    &|v| v.kind() == PathKind::DeeperArray,
                );

                if i == type_.array_dim - 1 && vararg && flush {
                    append_buffer.append("...");
                } else {
                    append_buffer.append("[]");
                }
            }
        }

        if type_.type_ == CodeType::Object {
            let current_node = DecompilerContext::get_current_class_node();
            DecompilerContext::get_import_collector()
                .get_short_name(&ExprProcessor::build_java_class_name(type_.value_str()));

            let mut full_classes: Vec<String> = Vec::new();
            let string = ExprProcessor::build_java_class_name(type_.value_str());
            let mut last_dollar = 0usize;

            while string[last_dollar..].find('$').is_some() {
                let Some(idx) = string[last_dollar..].find('$') else {
                    break;
                };

                last_dollar += idx + 1;
                full_classes.push(string[..last_dollar - 1].to_string());
            }

            full_classes.push(string.clone());

            let root_classes = DecompilerContext::get_class_processor().get_map_root_classes();
            let root_class =
                |name: &str| root_classes.borrow().get(&name.replace('.', "/")).copied();

            let outermost_node = root_class(&string);

            let mut start_idx = 0;
            for clazz in full_classes.iter().skip(1) {
                let node = root_class(clazz);
                match node {
                    Some(node)
                        if TypeAnnotation::is_static_in_context(
                            node,
                            current_node,
                            outermost_node,
                        ) =>
                    {
                        start_idx += 1;
                    }
                    _ => break,
                }
            }

            for (i, clazz) in full_classes.iter().enumerate() {
                let node = root_class(clazz);
                let name = match node {
                    Some(node) => node
                        .with(|d| d.simple_name.clone())
                        .expect("NullPointerException: simpleName"),
                    None => {
                        let dollar = clazz.find('$').map_or(-1, |d| d as i64);
                        let begin = clazz.rfind('.').map_or(0, |d| d + 1);
                        let end = if dollar > 0 {
                            dollar as usize
                        } else {
                            clazz.len()
                        };
                        clazz[begin..end].to_string()
                    }
                };

                if i > 0 {
                    buffer.append(".");
                } else if kuna_fixes::enabled(KunaFix::TypeAnnotationQualifiedName) {
                    buffer.append(&kuna_fix_type_annotation_qualified_name::qualifier(clazz));
                }

                if i >= start_idx {
                    TypeAnnotation::stepwise_write_annotations(
                        buffer,
                        type_annotations,
                        filter,
                        &|v| v.kind() == PathKind::DeeperNested,
                    );
                }

                buffer.append_class(&name, false, &clazz.replace('.', "/"));
                if kuna_fixes::enabled(KunaFix::TypeAnnotationOuterArguments)
                    && i + 1 < full_classes.len()
                {
                    if let Some(args) = kuna_fix_type_annotation_outer_arguments::arguments_of(
                        type_,
                        &clazz.replace('.', "/"),
                    ) {
                        buffer.append("<");
                        for (j, arg) in args.iter().enumerate() {
                            if j > 0 {
                                buffer.append(", ");
                            }
                            TypeAnnotation::append_generic_argument(
                                buffer,
                                arg.as_ref(),
                                &mut [],
                                filter,
                                false,
                            );
                        }
                        buffer.append(">");
                    }
                }
            }

            if let Some(gt) = type_.try_as_generic() {
                if !gt.get_arguments().is_empty() {
                    buffer.append("<");
                    let arguments = gt.get_arguments().to_vec();
                    for (j, arg) in arguments.iter().enumerate() {
                        if j > 0 {
                            buffer.append(", ");
                        }

                        let final_j = j as i32;
                        let in_arg = kuna_fixes::enabled(KunaFix::TypeAnnotationArgumentScope)
                            .then(|| {
                                kuna_fix_type_annotation_argument_scope::entering(
                                    type_annotations,
                                    final_j,
                                )
                            });
                        TypeAnnotation::stepwise_write_annotations(
                            buffer,
                            type_annotations,
                            filter,
                            &|v| v.kind() == PathKind::GenericArgument && v.type_arg() == final_j,
                        );
                        match in_arg {
                            Some(in_arg) => {
                                let mut sub = kuna_fix_type_annotation_argument_scope::take(
                                    type_annotations,
                                    &in_arg,
                                );
                                TypeAnnotation::append_generic_argument(
                                    buffer, arg.as_ref(), &mut sub, filter, vararg,
                                );
                                kuna_fix_type_annotation_argument_scope::put_back(
                                    type_annotations,
                                    &in_arg,
                                    sub,
                                );
                            }
                            None => TypeAnnotation::append_generic_argument(
                                buffer,
                                arg.as_ref(),
                                type_annotations,
                                filter,
                                vararg,
                            ),
                        }
                    }
                    buffer.append(">");
                }
            }

            buffer.append_buffer(&append_buffer);

            if flush {
                TypeAnnotation::dump_unwritten_annotations(buffer, type_annotations);
            }

            return;
        }

        TypeAnnotation::stepwise_write_annotations(buffer, type_annotations, filter, &|_| false);

        buffer.append_cast_type_name_str(
            &ExprProcessor::get_cast_type_name(&type_.resize_array_dim(0)),
            type_,
        );

        buffer.append_buffer(&append_buffer);

        if flush {
            TypeAnnotation::dump_unwritten_annotations(buffer, type_annotations);
        }
    }

    /// One generic argument of `appendCastTypeName`, after the annotations of
    /// its `GENERIC_ARGUMENT` step: its wildcard's annotations, the wildcard,
    /// and the argument's type.
    fn append_generic_argument(
        buffer: &mut TextBuffer,
        arg: Option<&VarType>,
        type_annotations: &mut [TypeAnnotationEntry],
        filter: &JHashSet<String>,
        vararg: bool,
    ) {
        let wildcard_arg = match arg {
            None => true,
            Some(a) => a
                .try_as_generic()
                .is_some_and(|g| g.get_wildcard() != GenericType::WILDCARD_NO),
        };
        if wildcard_arg {
            TypeAnnotation::stepwise_write_annotations(
                buffer,
                type_annotations,
                filter,
                &|v| v.kind() == PathKind::OnWildcard,
            );
        }

        let Some(arg) = arg else {
            buffer.append("?");
            return;
        };
        if let Some(gen_arg) = arg.try_as_generic() {
            if gen_arg.get_wildcard() == GenericType::WILDCARD_EXTENDS {
                buffer.append("? extends ");
            } else if gen_arg.get_wildcard() == GenericType::WILDCARD_SUPER {
                buffer.append("? super ");
            }
        }

        TypeAnnotation::append_cast_type_name(buffer, arg, type_annotations, filter, vararg, false);
    }

    /// Java: `stepwiseWriteAnnotations(TextBuffer, List, Set, Predicate)`.
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
    fn stepwise_write_annotations_space(
        buffer: &mut TextBuffer,
        extra_space: bool,
        type_annotations: &mut [TypeAnnotationEntry],
        filter: &JHashSet<String>,
        can_proceed: &dyn Fn(&PathValue) -> bool,
    ) {
        for anno in type_annotations.iter_mut() {
            if anno.a.is_empty() {
                if !anno.b.did_write_already() {
                    anno.b.set_did_write_already(true);

                    let anno_java = anno.b.to_java(-1);
                    if filter.contains(&anno_java.convert_to_string_and_allow_data_discard()) {
                        continue;
                    }

                    if extra_space {
                        buffer.append(" ");
                    }

                    buffer.append_buffer(&anno_java);
                    buffer.append(" ");
                }
            } else if can_proceed(anno.a.front().expect("non-empty path")) {
                anno.a.pop_front();
            }
        }
    }

    fn dump_unwritten_annotations(
        buffer: &mut TextBuffer,
        type_annotations: &mut [TypeAnnotationEntry],
    ) {
        let mut any = false;
        for anno in type_annotations.iter() {
            if !anno.b.did_write_already() {
                if any {
                    buffer.append(" ");
                } else {
                    buffer.append("/* ");
                    any = true;
                }

                anno.b.set_did_write_already(true);
                buffer.append_buffer(&anno.b.to_java(-1));
            }
        }

        if any {
            buffer.append(" */");
        }
    }
}
