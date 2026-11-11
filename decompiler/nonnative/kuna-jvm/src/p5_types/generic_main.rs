//! Port of `org/jetbrains/java/decompiler/struct/gen/generics/GenericMain.java`.

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::generic_class_descriptor::GenericClassDescriptor;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::generic_method_descriptor::GenericMethodDescriptor;
use crate::generic_type::{ch, jsub, junits_index_of, GenericType};
use crate::i_fernflower_logger::Severity;
use crate::rt::jstring::from_utf16;
use crate::rt::{rethrow, try_catch, JavaStr};
use crate::var_type::VarType;

pub struct GenericMain;

const TYPE_NAMES: [&str; 8] = [
    "byte", "char", "double", "float", "int", "long", "short", "boolean",
];

/// `String.split(regex)` for a one-character literal separator over UTF-16 units.
fn java_split_units(s: &[u16], sep: char) -> Vec<&[u16]> {
    let mut parts = Vec::new();
    let mut start = 0;
    for (i, &u) in s.iter().enumerate() {
        if u as u32 == sep as u32 {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }
    if parts.is_empty() {
        return vec![s];
    }
    parts.push(&s[start..]);
    while parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    parts
}

impl GenericMain {
    fn catch_invalid<T>(original: &str, body: impl FnOnce() -> T) -> Option<T> {
        match try_catch(body) {
            Ok(v) => Some(v),
            Err(caught) => {
                if caught.is("CanceledException") || caught.is("UnportedError") {
                    rethrow(caught);
                }
                DecompilerContext::get_logger()
                    .write_message(&format!("Invalid signature: {}", original), Severity::Warn);
                None
            }
        }
    }

    pub fn parse_class_signature(
        qualified_name: &str,
        signature: &str,
    ) -> Option<GenericClassDescriptor> {
        GenericMain::catch_invalid(signature, || {
            let units = signature.utf16();
            let mut fparameters = Vec::new();
            let mut fbounds = Vec::new();

            let mut signature =
                GenericMain::parse_formal_parameters(&units, &mut fparameters, &mut fbounds);

            let super_cl = GenericType::next_type_units(signature);
            let superclass = GenericType::parse_units(super_cl, GenericType::WILDCARD_NO);

            signature = jsub(signature, super_cl.len(), signature.len());
            let mut superinterfaces = Vec::new();
            while !signature.is_empty() {
                let super_if = GenericType::next_type_units(signature);
                superinterfaces.push(GenericType::parse_units(super_if, GenericType::WILDCARD_NO));
                signature = jsub(signature, super_if.len(), signature.len());
            }

            let mut buf = String::new();
            buf.push('L');
            buf.push_str(qualified_name);
            buf.push('<');
            for t in &fparameters {
                buf.push('T');
                buf.push_str(t);
                buf.push(';');
            }
            buf.push_str(">;");
            let generic_type = GenericType::parse(&buf).as_generic();

            GenericClassDescriptor::new(
                superclass,
                generic_type,
                superinterfaces,
                fparameters,
                fbounds,
            )
        })
    }

    pub fn parse_field_signature(signature: &str) -> Option<GenericFieldDescriptor> {
        GenericMain::catch_invalid(signature, || {
            GenericFieldDescriptor::new(GenericType::parse(signature))
        })
    }

    pub fn parse_method_signature(signature: &str) -> Option<GenericMethodDescriptor> {
        GenericMain::catch_invalid(signature, || {
            let units = signature.utf16();
            let mut type_parameters = Vec::new();
            let mut type_parameter_bounds = Vec::new();
            let mut signature = GenericMain::parse_formal_parameters(
                &units,
                &mut type_parameters,
                &mut type_parameter_bounds,
            );

            let to = junits_index_of(signature, ')');
            let to = if to < 0 { usize::MAX } else { to as usize };
            let mut parameters = jsub(signature, 1, to);
            signature = jsub(signature, to + 1, signature.len());

            let mut parameter_types = Vec::new();
            while !parameters.is_empty() {
                let par = GenericType::next_type_units(parameters);
                parameter_types.push(GenericType::parse_units(par, GenericType::WILDCARD_NO));
                parameters = &parameters[par.len()..];
            }

            let ret = GenericType::next_type_units(signature);
            let return_type = GenericType::parse_units(ret, GenericType::WILDCARD_NO);
            signature = &signature[ret.len()..];

            let mut exception_types = Vec::new();
            if !signature.is_empty() {
                let exceptions = java_split_units(signature, '^');
                for exception in exceptions.iter().skip(1) {
                    exception_types.push(GenericType::parse_units(
                        exception,
                        GenericType::WILDCARD_NO,
                    ));
                }
            }

            GenericMethodDescriptor::new(
                type_parameters,
                type_parameter_bounds,
                parameter_types,
                Some(return_type),
                exception_types,
            )
        })
    }

    fn parse_formal_parameters<'a>(
        signature: &'a [u16],
        parameters: &mut Vec<String>,
        bounds: &mut Vec<Vec<VarType>>,
    ) -> &'a [u16] {
        if ch(*signature.first().unwrap_or_else(
            || crate::jthrow!(StringIndexOutOfBoundsException; "index 0, length 0"),
        )) != '<'
        {
            return signature;
        }

        let mut counter: i32 = 1;
        let mut index = 1;
        while index < signature.len() {
            match ch(signature[index]) {
                '<' => counter += 1,
                '>' => {
                    counter -= 1;
                    if counter == 0 {
                        break;
                    }
                }
                _ => {}
            }
            index += 1;
        }

        let mut value = jsub(signature, 1, index);
        let signature = jsub(signature, index + 1, signature.len());

        while !value.is_empty() {
            let to = junits_index_of(value, ':');
            let to = if to < 0 { usize::MAX } else { to as usize };

            let param = from_utf16(jsub(value, 0, to));
            value = jsub(value, to + 1, value.len());

            let mut lst_bounds = Vec::new();

            loop {
                if ch(value[0]) == ':' {
                    value = &value[1..];
                }

                let bound = GenericType::next_type_units(value);
                lst_bounds.push(GenericType::parse_units(bound, GenericType::WILDCARD_NO));
                value = &value[bound.len()..];

                if value.is_empty() || ch(value[0]) != ':' {
                    break;
                } else {
                    value = &value[1..];
                }
            }

            parameters.push(param);
            bounds.push(lst_bounds);
        }

        signature
    }

    pub fn get_generic_cast_type_name(type_: &GenericType) -> String {
        let mut s = GenericMain::get_type_name(type_);
        for _ in 0..type_.array_dim {
            s.push_str("[]");
        }
        s
    }

    fn get_type_name(type_: &GenericType) -> String {
        let tp = type_.type_;
        if tp.ordinal() <= CodeType::Boolean.ordinal() {
            return TYPE_NAMES[tp.ordinal() as usize].to_string();
        } else if tp == CodeType::Void {
            return "void".to_string();
        } else if tp == CodeType::Genvar {
            return type_.value_str().to_string();
        } else if tp == CodeType::Object {
            return type_.get_cast_name();
        }

        crate::jthrow!("Invalid type: {}", type_)
    }
}
