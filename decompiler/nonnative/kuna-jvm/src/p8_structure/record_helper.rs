//! Port of `org/jetbrains/java/decompiler/modules/decompiler/RecordHelper.java`.
//!
//! kuna fix `compact_record_constructor` (`kuna_fix_compact_record_constructor`):
//! `fixup_canonical_constructor` takes a canonical constructor for a compact
//! one only when it ends with every component's `this.c = c`, in order, and
//! then removes only stores to fields of `this`.

use crate::class_writer::ClassWriter;
use crate::code_constants::{ACC_STATIC, ACC_VARARGS, INIT_NAME};
use crate::exprent::Exprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::kuna_fix_compact_record_constructor;
use crate::method_wrapper::MethodWrapper;
use crate::root_statement::RootStatement;
use crate::rt::{JHashSet, JLinkedHashSet};
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_member::StructMember;
use crate::struct_method::StructMethod;
use crate::struct_record_component::StructRecordComponent;
use crate::text_buffer::TextBuffer;
use crate::type_annotation::TypeAnnotation;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct RecordHelper;

impl RecordHelper {
    pub fn is_hidden_record_method(cl: StructClass, mt: StructMethod, root: RootStatement) -> bool {
        if cl.get_record_components().is_none() {
            return false;
        }
        Self::is_synthetic_record_method(mt, root)
            || Self::is_default_record_method(mt, root)
            || (mt.get_name() == INIT_NAME
                && !Self::has_annotations(mt)
                && Self::is_default_record_constructor(cl, root))
    }

    pub fn is_hidden_record_field(
        components: Option<&[StructRecordComponent]>,
        fd: StructField,
    ) -> bool {
        let Some(components) = components else {
            return false;
        };

        for component in components {
            if component.get_name() == fd.get_name()
                && component.get_descriptor() == fd.get_descriptor()
                && !fd.has_modifier(ACC_STATIC)
            {
                return true;
            }
        }

        false
    }

    pub fn append_record_components(
        buffer: &mut TextBuffer,
        cl: StructClass,
        components: &[StructRecordComponent],
        indent: i32,
    ) {
        buffer.push_newline_group(indent, 1);
        buffer.append_possible_newline();
        buffer.push_newline_group(indent, 0);
        for (i, cd) in components.iter().enumerate() {
            if i > 0 {
                buffer.append(",").append_possible_newline_alt(" ");
            }
            let var_arg_component = i == components.len() - 1 && Self::is_var_arg_record(cl);
            Self::record_component_to_java(buffer, cl, *cd, i as i32, var_arg_component);
        }
        buffer.pop_newline_group();
        buffer.append_possible_newline_dedent("", true);
        buffer.pop_newline_group();
    }

    fn get_simple_return_value(root: RootStatement) -> Option<Exprent> {
        let block = root.get_first()?;
        if !block.is_basic_block() {
            return None;
        }
        let exprents = block.get_exprents().unwrap();
        if exprents.is_empty() {
            return None;
        }
        let exit = exprents.get(0).try_as_exit()?;
        exit.get_value()
    }

    fn is_synthetic_record_method(mt: StructMethod, root: RootStatement) -> bool {
        let name = mt.get_name();
        let descriptor = mt.get_descriptor();
        if (name != "equals" || descriptor != "(Ljava/lang/Object;)Z")
            && (name != "hashCode" || descriptor != "()I")
            && (name != "toString" || descriptor != "()Ljava/lang/String;")
        {
            return false;
        }
        let Some(value) = Self::get_simple_return_value(root).and_then(|v| v.try_as_invocation())
        else {
            return false;
        };
        let Some(bootstrap_method) = value.get_bootstrap_method() else {
            return false;
        };
        bootstrap_method.classname.as_deref() == Some("java/lang/runtime/ObjectMethods")
            && bootstrap_method.elementname == "bootstrap"
    }

    fn is_default_record_method(mt: StructMethod, root: RootStatement) -> bool {
        let Some(field_exprent) =
            Self::get_simple_return_value(root).and_then(|v| v.try_as_field())
        else {
            return false;
        };
        let Some(instance) = field_exprent.get_instance().and_then(|i| i.try_as_var()) else {
            return false;
        };
        instance.get_index() == 0 && field_exprent.get_name() == mt.get_name()
    }

    fn is_default_record_constructor(cl: StructClass, root: RootStatement) -> bool {
        let Some(components) = cl.get_record_components() else {
            return false;
        };
        let Some(block) = root.get_first().filter(|b| b.is_basic_block()) else {
            return false;
        };
        let exprents = block.get_exprents().unwrap();
        if exprents.len() != components.len() {
            return false;
        }
        let mut last_index = 0;
        for (i, component) in components.iter().enumerate() {
            let Some(assignment) = exprents.get(i).try_as_assignment() else {
                return false;
            };
            let Some(left) = assignment.get_left().try_as_field() else {
                return false;
            };
            if component.get_name() != left.get_name() {
                return false;
            }
            match left.get_instance().and_then(|f| f.try_as_var()) {
                Some(field_instance) if field_instance.get_index() == 0 => {}
                _ => return false,
            }
            let Some(right) = assignment.get_right().try_as_var() else {
                return false;
            };
            let index = right.get_index();
            if index <= last_index {
                return false;
            }
            last_index = index;
        }
        true
    }

    fn get_canonical_constructor(cl: StructClass) -> Option<StructMethod> {
        let descriptors: String = cl
            .get_record_components()
            .unwrap()
            .iter()
            .map(|c| c.get_descriptor())
            .collect();
        let canonical_constructor_descriptor = format!("({descriptors})V");
        cl.get_method(INIT_NAME, &canonical_constructor_descriptor)
    }

    fn get_getter(cl: StructClass, rc: StructRecordComponent) -> Option<StructMethod> {
        cl.get_method(&rc.get_name(), &format!("(){}", rc.get_descriptor()))
    }

    fn is_var_arg_record(cl: StructClass) -> bool {
        Self::get_canonical_constructor(cl).is_some_and(|init| init.has_modifier(ACC_VARARGS))
    }

    /// Java returns a `LinkedHashSet<TextBuffer>` of fresh buffers (identity), i.e. a list.
    fn get_record_component_annotations(
        cl: StructClass,
        cd: StructRecordComponent,
        param: i32,
    ) -> Vec<TextBuffer> {
        let mut annotations: JLinkedHashSet<String> = JLinkedHashSet::new();
        let mut buffers: Vec<TextBuffer> = Vec::new();
        let mut members: Vec<StructMember> = Vec::new();
        members.push(cd.into());
        if let Some(getter) = Self::get_getter(cl, cd) {
            members.push(getter.0);
        }

        for member in members {
            for key in ClassWriter::ANNOTATION_ATTRIBUTES {
                let Some(attribute) = member.get_attribute(key) else {
                    continue;
                };
                for annotation in attribute.get_annotations() {
                    let text = annotation.to_java(-1);
                    if annotations.add(text.convert_to_string_and_allow_data_discard()) {
                        buffers.push(text);
                    }
                }
            }
        }

        let Some(constr) = Self::get_canonical_constructor(cl) else {
            return buffers;
        };
        let constr: StructMember = constr.0;

        for key in ClassWriter::PARAMETER_ANNOTATION_ATTRIBUTES {
            let Some(attribute) = constr.get_attribute(key) else {
                continue;
            };
            let param_annotations = attribute.get_param_annotations();
            if param as usize >= param_annotations.len() {
                continue;
            }
            for annotation in param_annotations[param as usize].iter().copied() {
                let text = annotation.to_java(-1);
                if annotations.add(text.convert_to_string_and_allow_data_discard()) {
                    buffers.push(text);
                }
            }
        }

        buffers
    }

    fn record_component_to_java(
        buffer: &mut TextBuffer,
        cl: StructClass,
        cd: StructRecordComponent,
        param: i32,
        var_arg_component: bool,
    ) {
        let annotations = Self::get_record_component_annotations(cl, cd, param);
        let mut written_annotations: JHashSet<String> = JHashSet::new();
        for annotation in &annotations {
            written_annotations.add(annotation.convert_to_string_and_allow_data_discard());
            buffer.append_buffer(annotation).append_char(' ');
        }

        let mut field_type = VarType::new_signature_cl(&cd.get_descriptor(), false);
        let descriptor = cd.get_signature();

        if let Some(descriptor) = descriptor {
            descriptor.verify_type(cl.get_signature().as_ref(), &field_type);
            field_type = descriptor.type_();
        }

        let mut annos = ClassWriter::get_type_annotations(cd.into(), TypeAnnotation::FIELD, -1);

        let shown_type = if var_arg_component {
            field_type.decrease_array_dim()
        } else {
            field_type
        };
        if !annos.is_empty() {
            buffer.append_cast_type_name_annotations(&shown_type, &mut annos, &written_annotations);
        } else {
            buffer.append_cast_type_name(&shown_type);
        }
        if var_arg_component {
            buffer.append("...");
        }
        buffer.append_char(' ');

        buffer.append_field(
            &cd.get_name(),
            true,
            &cl.qualified_name(),
            &cd.get_name(),
            &cd.get_descriptor(),
        );
    }

    pub fn has_annotations(mt: StructMethod) -> bool {
        mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS)
            .is_some()
            || mt
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS)
                .is_some()
    }

    pub fn fixup_canonical_constructor(mw: MethodWrapper, cl: StructClass) {
        let Some(components) = cl.get_record_components() else {
            return;
        };

        if Some(mw.with(|d| d.method_struct)) != Self::get_canonical_constructor(cl) {
            return;
        }

        let md = mw.desc();
        let params = md.params.len();

        if params != components.len() {
            return;
        }

        let varproc = mw.with(|d| d.varproc);
        let mut varidx = 1;
        for (i, component) in components.iter().enumerate() {
            varproc.set_clashing_name(VarVersionPair::new(varidx, 0), component.get_name());

            varidx += md.params[i].stack_size;
        }

        let fixed = kuna_fixes::enabled(KunaFix::CompactRecordConstructor);
        if Self::is_compact_canonical_constructor(mw)
            && (!fixed
                || mw.with(|d| d.root).is_some_and(|root| {
                    kuna_fix_compact_record_constructor::assigns_components_last(root, &components)
                        && !kuna_fix_compact_record_constructor::stores_on_own_line(
                            root,
                            mw.with(|d| d.method_struct),
                            components.len(),
                        )
                }))
        {
            mw.get_or_build_graph()
                .expect("NullPointerException: graph")
                .iterate_exprents(|exprent: Exprent| {
                    if fixed {
                        return if kuna_fix_compact_record_constructor::is_this_field_store(exprent) {
                            2
                        } else {
                            0
                        };
                    }
                    if let Some(assignment_exprent) = exprent.try_as_assignment() {
                        if assignment_exprent.get_left().is_field() {
                            return 2;
                        }
                    }

                    0
                });
            mw.with_mut(|d| d.is_compact_record_constructor = true);
        }
    }

    fn is_compact_canonical_constructor(mw: MethodWrapper) -> bool {
        let Some(graph) = mw.get_or_build_graph() else {
            return false;
        };

        let mut valid = false;
        graph.iterate_exprents(|exprent: Exprent| {
            let shape = exprent.try_as_assignment().and_then(|assignment_exprent| {
                let field_exprent = assignment_exprent.get_left().try_as_field()?;
                let var_exprent = assignment_exprent.get_right().try_as_var()?;
                Some((field_exprent, var_exprent))
            });
            valid = match shape {
                Some((field_exprent, var_exprent)) => {
                    var_exprent.get_lvt().is_some()
                        && field_exprent.get_name() == var_exprent.get_name()
                }
                None => false,
            };

            0
        });
        valid
    }
}
