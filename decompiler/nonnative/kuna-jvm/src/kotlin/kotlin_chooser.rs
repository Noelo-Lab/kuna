//! Port of `org/vineflower/kotlin/KotlinChooser.java`. The decoding half of
//! `parseMetadataFor` is `kt_metadata`; the `EXTENSIONS` registry has no
//! counterpart (`kt_proto` always reads the JVM extensions).

use std::rc::Rc;

use crate::decompiler_context::DecompilerContext;
use crate::i_fernflower_logger::Severity;
use crate::jthrow;
use crate::k_class::KClass;
use crate::k_constructor::KConstructor;
use crate::k_element::KElement;
use crate::k_file::KFile;
use crate::k_function::KFunction;
use crate::k_function_reference::KFunctionReference;
use crate::k_multifile_facade::KMultifileFacade;
use crate::k_property::KProperty;
use crate::kotlin_options::KotlinOptions;
use crate::kt_attrs::KtAttrs;
use crate::kt_metadata::{KotlinMetadataAnnotation, KtMetadata};
use crate::kt_proto;
use crate::language_chooser::LanguageChooser;
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::rt::{rethrow, try_catch, Caught};
use crate::struct_annotation_attribute::{AnnotationElement, StructAnnotationAttribute};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::{AttributeKey, StructGeneralAttribute};

#[derive(Clone, Copy, Debug, Default)]
pub struct KotlinChooser;

impl KotlinChooser {
    const ANNOTATION_ATTRIBUTES: [AttributeKey<StructAnnotationAttribute>; 2] = [
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS,
        StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS,
    ];

    pub fn new() -> KotlinChooser {
        KotlinChooser
    }

    pub fn is_language(&self, cl: StructClass) -> bool {
        let decomp_enabled = DecompilerContext::get_option(KotlinOptions::DECOMPILE_KOTLIN);
        if !decomp_enabled && !DecompilerContext::get_option(KotlinOptions::ALWAYS_EXPORT_METADATA)
        {
            return false;
        }

        for key in KotlinChooser::ANNOTATION_ATTRIBUTES {
            if cl.has_attribute(key) {
                let attr = cl.get_attribute(key).expect("attribute present");
                for anno in attr.get_records() {
                    if is_metadata(anno.class_name.as_deref()) {
                        KotlinChooser::parse_metadata_for(cl);
                        return decomp_enabled;
                    }
                }
            }
        }

        false
    }

    pub fn parse_metadata_for(cl: StructClass) {
        if KtAttrs::has(cl, &KElement::KEY()) {
            return;
        }

        let mut anno: Option<KotlinMetadataAnnotation> = None;

        'outer: for key in KotlinChooser::ANNOTATION_ATTRIBUTES {
            if cl.has_attribute(key) {
                let attr = cl.get_attribute(key).expect("attribute present");
                for a in attr.get_records() {
                    if is_metadata(a.class_name.as_deref()) {
                        anno = Some(KotlinMetadataAnnotation::new(a));
                        break 'outer;
                    }
                }
            }
        }

        let Some(anno) = anno else {
            return;
        };

        let result = try_catch(|| KotlinChooser::parse_metadata(cl, &anno));
        if let Err(e) = result {
            if is_java_error(&e) {
                rethrow(e);
            }
            DecompilerContext::get_logger().write_message_ex(
                &format!("Failed to parse metadata for class {}", cl.qualified_name()),
                Severity::Warn,
                &e,
            );
        }
    }

    /// The `try` block of `parseMetadataFor`.
    fn parse_metadata(cl: StructClass, anno: &KotlinMetadataAnnotation) {
        let k_index = anno.index_of("k");
        let d1_index = anno.index_of("d1");
        let d2_index = anno.index_of("d2");

        if k_index == -1 {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "No k attribute (type) in class metadata for class {}, cannot continue Kotlin parsing",
                    cl.qualified_name()
                ),
                Severity::Warn,
            );
            return;
        }

        let k = KotlinMetadataAnnotation::int_value(anno.get_par_value(k_index));

        if d1_index == -1 {
            let function_ref = match cl.super_class() {
                Some(super_class) => {
                    &*super_class.get_string() == "kotlin/jvm/internal/FunctionReferenceImpl"
                }
                None => jthrow!(NullPointerException; "cl.superClass is null"),
            };
            if function_ref {
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::FunctionReference(KFunctionReference),
                );
            }

            let resolution = if function_ref {
                "assuming function reference"
            } else {
                "cannot continue Kotlin parsing"
            };
            DecompilerContext::get_logger().write_message(
                &format!(
                    "No d1 attribute (data) in class metadata for class {}, {}",
                    cl.qualified_name(),
                    resolution
                ),
                Severity::Warn,
            );
            return;
        }

        if d2_index == -1 {
            DecompilerContext::get_logger().write_message(
                &format!(
                    "No d2 attribute (strings) in class metadata for class {}",
                    cl.qualified_name()
                ),
                Severity::Warn,
            );
        }

        let d1 = anno.get_par_value(d1_index);
        let d2 = if d2_index != -1 {
            Some(anno.get_par_value(d2_index))
        } else {
            None
        };

        let data1 = KotlinChooser::get_data_from_expr(d1);

        let mut input = KtMetadata::open(&data1);

        let resolver: Option<MetadataNameResolver> = match d2 {
            Some(d2) => {
                let data2 = KotlinChooser::get_data_from_expr(d2);
                Some(KtMetadata::read_resolver(&mut input, &data2))
            }
            None => None,
        };

        if k == 1 {
            let pcl = kt_proto::Class::parse_from(&mut input);
            KProperty::parse(cl, &pcl.get_property_list(), resolver.as_ref(), None);
            KFunction::parse(
                cl,
                resolver.as_ref(),
                &pcl.get_type_table(),
                &pcl.get_function_list(),
                false,
                None,
            );
            KConstructor::parse(cl, &pcl, resolver.as_ref());
            KtAttrs::put(
                cl,
                &KElement::KEY(),
                KElement::Class(Rc::new(KClass::new(pcl, resolver))),
            );
        } else if k == 2 {
            let pcl = kt_proto::Package::parse_from(&mut input);
            KProperty::parse(cl, &pcl.get_property_list(), resolver.as_ref(), None);
            KFunction::parse(
                cl,
                resolver.as_ref(),
                &pcl.get_type_table(),
                &pcl.get_function_list(),
                false,
                None,
            );
            KtAttrs::put(
                cl,
                &KElement::KEY(),
                KElement::File(Rc::new(KFile::new(pcl, resolver, None))),
            );
        } else if k == 3 {
            let func = kt_proto::Function::parse_from(&mut input);
            KFunction::parse(
                cl,
                resolver.as_ref(),
                &func.get_type_table(),
                std::slice::from_ref(&func),
                true,
                None,
            );
            if !KtAttrs::has(cl, &KElement::KEY()) {
                KtAttrs::put(
                    cl,
                    &KElement::KEY(),
                    KElement::Function(KFunction::FAILED_LAMBDA()),
                );
            }
        } else if k == 4 {
            KtAttrs::put(
                cl,
                &KElement::KEY(),
                KElement::MultifileFacade(Rc::new(KMultifileFacade::new(data1))),
            );
        } else if k == 5 {
            let pcl = kt_proto::Package::parse_from(&mut input);
            KProperty::parse(cl, &pcl.get_property_list(), resolver.as_ref(), None);
            KFunction::parse(
                cl,
                resolver.as_ref(),
                &pcl.get_type_table(),
                &pcl.get_function_list(),
                false,
                None,
            );

            let multifile_name;
            let xs_index = anno.index_of("xs");
            if xs_index != -1 {
                multifile_name =
                    KotlinMetadataAnnotation::string_value(anno.get_par_value(xs_index));
            } else {
                DecompilerContext::get_logger().write_message(
                    &format!(
                        "No xs attribute (extra string; multifile part name) in class metadata for class {}",
                        cl.qualified_name()
                    ),
                    Severity::Warn,
                );
                multifile_name = KtMetadata::multifile_name(None, &cl.qualified_name());
            }

            KtAttrs::put(
                cl,
                &KElement::KEY(),
                KElement::File(Rc::new(KFile::new(pcl, resolver, Some(multifile_name)))),
            );
        }
    }

    /// `getDataFromExpr((NewExprent) value)` over the session-scope annotation value.
    fn get_data_from_expr(d2: &AnnotationElement) -> Vec<String> {
        KotlinMetadataAnnotation::data_from_expr(d2)
    }
}

/// `anno.getClassName().equals("kotlin/Metadata")`.
fn is_metadata(class_name: Option<&str>) -> bool {
    match class_name {
        Some(name) => name == KotlinMetadataAnnotation::CLASS_NAME,
        None => jthrow!(NullPointerException; "anno.getClassName() is null"),
    }
}

/// A thrown `java.lang.Error`, which `catch (Exception e)` lets through.
fn is_java_error(caught: &Caught) -> bool {
    caught
        .exception
        .as_ref()
        .is_some_and(|e| e.class.ends_with("Error"))
}

impl LanguageChooser for KotlinChooser {
    fn is_language(&self, cl: StructClass) -> bool {
        KotlinChooser::is_language(self, cl)
    }
}
