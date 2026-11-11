//! Port of `org/vineflower/kotlin/expr/KAnnotationExprent.java`.
//!
//! kuna fix `kt_nested_annotation_argument`: an annotation that is the
//! argument of another is written without `@` (`ReplaceWith(..)`), as
//! Kotlin requires.
//!
//! kuna fix `kt_string_literal_escape`: a string argument escapes the `$`
//! that would start a template (`kuna_fix_kt_string_literal_escape`).

use crate::annotation_exprent::{AnnotationExprent, Type as AnnotationType};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_types::KTypes;
use crate::k_utils::KUtils;
use crate::const_exprent::ConstExprent;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::kotlin::kuna_fix_kt_string_literal_escape;
use crate::kotlin_exprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// `KAnnotationExprent.UseSiteTarget`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum UseSiteTarget {
    File,
    Property,
    Field,
    Get,
    Set,
    Receiver,
    Param,
    Setparam,
    Delegate,
}

impl UseSiteTarget {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            UseSiteTarget::File => "FILE",
            UseSiteTarget::Property => "PROPERTY",
            UseSiteTarget::Field => "FIELD",
            UseSiteTarget::Get => "GET",
            UseSiteTarget::Set => "SET",
            UseSiteTarget::Receiver => "RECEIVER",
            UseSiteTarget::Param => "PARAM",
            UseSiteTarget::Setparam => "SETPARAM",
            UseSiteTarget::Delegate => "DELEGATE",
        }
    }
}

pub struct KAnnotationExprentData {
    pub use_site_target: Option<UseSiteTarget>,
}

kotlin_exprent!(pub KAnnotationExprent: AnnotationExprent, Annotation(KAnnotationExprentData),
                is_k_annotation, as_k_annotation, try_as_k_annotation);

fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

impl KAnnotationExprent {
    /// Java: KAnnotationExprent(AnnotationExprent)
    pub fn new(expr: AnnotationExprent) -> KAnnotationExprent {
        KAnnotationExprent::new_target(expr, None)
    }

    /// Java: KAnnotationExprent(AnnotationExprent, UseSiteTarget)
    pub fn new_target(
        expr: AnnotationExprent,
        use_site_target: Option<UseSiteTarget>,
    ) -> KAnnotationExprent {
        let class_name = expr.get_class_name();
        let par_names = expr.get_par_names();
        let par_values = KUtils::replace_exprents(&expr.get_par_values());
        let annotation = AnnotationExprent::new(class_name, par_names, par_values);
        annotation
            .0
            .set_kt(Some(KExprentExt::Annotation(KAnnotationExprentData {
                use_site_target,
            })));
        annotation.0.set_bytecode(expr.get_bytecode());
        KAnnotationExprent(annotation)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buffer = TextBuffer::new();
        buffer.add_bytecode_mapping(self.get_bytecode().as_ref());

        buffer.append_indent(indent).append_char('@');

        if let Some(use_site_target) = self.kd(|d| d.use_site_target) {
            buffer
                .append(&use_site_target.name().to_lowercase())
                .append_char(':');
        }

        let type_ = VarType::new_signature_cl(&self.get_class_name(), true);
        buffer.append(&KTypes::get_kotlin_type(Some(&type_)));

        match self.get_annotation_type() {
            AnnotationType::SingleElement => {
                buffer.append_char('(').append_possible_newline();
                KAnnotationExprent::write_annotation_value(self.get_par_values()[0], &mut buffer);
                buffer.append_char(')');
            }
            AnnotationType::Normal => {
                buffer.append_char('(').append_possible_newline();
                let par_names = self.get_par_names();
                let par_values = self.get_par_values();
                let mut first = true;
                for i in 0..par_values.len() {
                    if first {
                        first = false;
                    } else {
                        buffer.append(",").append_possible_newline_alt(" ");
                    }
                    buffer.append(&par_names[i]).append(" = ");
                    KAnnotationExprent::write_annotation_value(par_values[i], &mut buffer);
                }
                buffer.append_char(')');
            }
            AnnotationType::Marker => {}
        }

        buffer
    }

    pub fn write_annotation_value(expr: Exprent, buffer: &mut TextBuffer) {
        if let Some(field_exprent) = expr.try_as_field() {
            let classname = field_exprent.get_classname();
            let type_ = VarType::new_signature_cl(&classname, true);
            DecompilerContext::get_import_collector().get_short_name_imported(&classname, true);

            buffer
                .append(&KTypes::get_kotlin_type(Some(&type_)))
                .append(".")
                .append(&field_exprent.get_name());
        } else if let Some(new_exprent) = expr.try_as_new() {
            buffer.append_char('[');
            let mut first = true;
            for exprent in new_exprent.get_lst_array_elements().iter() {
                if first {
                    first = false;
                } else {
                    buffer.append(", ");
                }
                KAnnotationExprent::write_annotation_value(exprent, buffer);
            }
            buffer.append_char(']');
        } else if let Some(annotation_exprent) = expr.try_as_annotation() {
            if !kuna_fixes::enabled(KunaFix::KtNestedAnnotationArgument) {
                buffer.append_char('@');
            }
            let type_ = VarType::new_signature_cl(&annotation_exprent.get_class_name(), true);
            buffer.append(&KTypes::get_kotlin_type(Some(&type_)));

            match annotation_exprent.get_annotation_type() {
                AnnotationType::SingleElement => {
                    buffer.append_char('(');
                    KAnnotationExprent::write_annotation_value(
                        annotation_exprent.get_par_values()[0],
                        buffer,
                    );
                    buffer.append_char(')');
                }
                AnnotationType::Normal => {
                    buffer.append_char('(');
                    let par_names = annotation_exprent.get_par_names();
                    let par_values = annotation_exprent.get_par_values();
                    let mut first = true;
                    for i in 0..par_values.len() {
                        if first {
                            first = false;
                        } else {
                            buffer.append(", ");
                        }
                        buffer.append(&par_names[i]).append(" = ");
                        KAnnotationExprent::write_annotation_value(par_values[i], buffer);
                    }
                    buffer.append_char(')');
                }
                AnnotationType::Marker => {}
            }
        } else if let Some(const_exprent) = expr.try_as_const() {
            if const_exprent
                .get_const_type()
                .equals(&VarType::VARTYPE_CLASS())
            {
                let value = const_exprent
                    .get_value()
                    .unwrap_or_else(|| npe("ConstExprent.getValue()"));
                let value = value.as_string();
                let type_ = VarType::new_signature_cl(value, true);
                DecompilerContext::get_import_collector().get_short_name_imported(value, true);
                buffer
                    .append(&KTypes::get_kotlin_type(Some(&type_)))
                    .append("::class");
            } else if kuna_fixes::enabled(KunaFix::KtStringLiteralEscape)
                && const_exprent
                    .get_const_type()
                    .equals(&VarType::VARTYPE_STRING())
                && const_exprent.get_value().is_some()
            {
                let ascii = DecompilerContext::get_option(
                    IFernflowerPreferences::ASCII_STRING_CHARACTERS,
                );
                let value = const_exprent.get_value().expect("checked");
                let text = kuna_fix_kt_string_literal_escape::kotlin_text(
                    &ConstExprent::convert_string_to_java(value.as_string(), ascii),
                    false,
                );
                buffer.append_char('"').append(&text).append_char('"');
            } else {
                buffer.append_buffer(&expr.to_java(0));
            }
        }
    }
}
