//! Port of `org/vineflower/kotlin/util/KotlinObjectProcessors.java`.

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNode;
use crate::interpreter_util::InterpreterUtil;
use crate::jthrow;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::k_hidden_element::KHiddenElement;
use crate::k_property::KProperty;
use crate::kotlin_chooser::KotlinChooser;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::ClassKind;

pub struct KotlinObjectProcessors;

impl KotlinObjectProcessors {
    pub fn parse_with_companion(node: ClassNode) {
        let wrapper = node.get_wrapper();
        let cl = node.with(|d| d.class_struct);

        let kt_data = KtAttrs::get(cl, &KElement::KEY());
        let Some(KElement::Class(cls)) = kt_data else {
            return;
        };
        if !cls.proto.has_companion_object_name() {
            return;
        }

        let name = cls
            .resolver
            .as_ref()
            .expect("NullPointerException: resolver")
            .resolve(cls.proto.get_companion_object_name());
        let nested = node.with(|d| d.nested.clone()).to_vec();
        let companion = nested
            .into_iter()
            .find(|n| n.with(|d| d.simple_name.as_deref() == Some(name.as_str())));
        let Some(companion) = companion else {
            return;
        };

        let companion_struct = companion.with(|d| d.class_struct);
        KotlinChooser::parse_metadata_for(companion_struct);
        let companion_data = KtAttrs::get(companion_struct, &KElement::KEY());
        let Some(KElement::Class(companion_cls)) = companion_data else {
            jthrow!(IllegalStateException; "Companion object of {} is not a class", cl.qualified_name());
        };
        KFunction::parse(
            companion_struct,
            companion_cls.resolver.as_ref(),
            &companion_cls.proto.get_type_table(),
            &companion_cls.proto.get_function_list(),
            false,
            Some(cl),
        );
        KProperty::parse(
            companion_struct,
            &companion_cls.proto.get_property_list(),
            companion_cls.resolver.as_ref(),
            Some(cl),
        );

        let companion_desc = format!("L{};", companion_struct.qualified_name());
        for fd in cl.get_fields().to_vec() {
            if fd.get_descriptor() == companion_desc {
                let key = InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
                let initializer = wrapper
                    .expect("NullPointerException: wrapper")
                    .get_static_field_initializers()
                    .borrow()
                    .get_with_key(&key);
                if initializer.is_some_and(|initializer| initializer.is_new()) {
                    KtAttrs::put(
                        fd,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::CompanionInstance),
                    );
                    break;
                }
            }
        }
    }

    pub fn mark_object_instance(wrapper: ClassWrapper) {
        let cl = wrapper.get_class_struct();
        let kt_data = KtAttrs::get(cl, &KElement::KEY());

        let Some(KElement::Class(cls)) = kt_data else {
            return;
        };

        if Flags::CLASS_KIND.get(cls.flags()) != Some(ClassKind::Object) {
            return;
        }

        let object_desc = format!("L{};", cl.qualified_name());
        for fd in cl.get_fields().to_vec() {
            if fd.get_descriptor() == object_desc {
                let key = InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
                let initializer = wrapper
                    .get_static_field_initializers()
                    .borrow()
                    .get_with_key(&key);
                if initializer.is_some_and(|initializer| initializer.is_new()) {
                    KtAttrs::put(
                        fd,
                        &KElement::KEY(),
                        KElement::HiddenElement(KHiddenElement::ObjectInstance),
                    );
                    break;
                }
            }
        }
    }
}
