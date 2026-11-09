//! Port of `org/jetbrains/java/decompiler/modules/decompiler/RecordHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::code_constants::{ACC_STATIC, ACC_VARARGS, INIT_NAME};
use crate::exprent::Exprent;
use crate::method_wrapper::MethodWrapper;
use crate::root_statement::RootStatement;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;
use crate::struct_record_component::StructRecordComponent;
use crate::text_buffer::TextBuffer;
use crate::unported;

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
        unported!("RecordHelper.appendRecordComponents")
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
        unported!("RecordHelper.isSyntheticRecordMethod")
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

    fn get_record_component_annotations(
        cl: StructClass,
        cd: StructRecordComponent,
        param: i32,
    ) -> Vec<TextBuffer> {
        unported!("RecordHelper.getRecordComponentAnnotations")
    }

    fn record_component_to_java(
        buffer: &mut TextBuffer,
        cl: StructClass,
        cd: StructRecordComponent,
        param: i32,
        var_arg_component: bool,
    ) {
        unported!("RecordHelper.recordComponentToJava")
    }

    pub fn has_annotations(mt: StructMethod) -> bool {
        unported!("RecordHelper.hasAnnotations")
    }

    pub fn fixup_canonical_constructor(mw: MethodWrapper, cl: StructClass) {
        unported!("RecordHelper.fixupCanonicalConstructor")
    }

    fn is_compact_canonical_constructor(mw: MethodWrapper) -> bool {
        unported!("RecordHelper.isCompactCanonicalConstructor")
    }
}
