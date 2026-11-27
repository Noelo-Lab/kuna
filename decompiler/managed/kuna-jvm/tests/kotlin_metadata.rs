//! The Kotlin metadata decoder on hand-made `kotlin/Metadata` values: wire edge cases,
//! truncation, overlong varints, merges, recursion limits, string-table and resolver corners,
//! the 8-to-7 bit encoding (`data/kotlin_metadata/crafted.txt`, recorded from Vineflower 1.12.0's
//! Kotlin plugin). Each decodes to the message pinned there, or fails with the exception pinned
//! there, and the string table cases resolve every name as pinned.

use kuna_jvm::code_constants::{
    CONSTANT_DOUBLE, CONSTANT_FLOAT, CONSTANT_INTEGER, CONSTANT_LONG, CONSTANT_UTF8,
};
use kuna_jvm::field_descriptor::FieldDescriptor;
use kuna_jvm::kt_metadata::{KotlinMetadataAnnotation, KotlinMetadataMessage, KtMetadata};
use kuna_jvm::primitive_constant::{ConstValue, PrimitiveConstant};
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{try_catch, Caught, JavaStr};
use kuna_jvm::struct_annotation_attribute::{AnnotationElement, AnnotationRecord};

const CRAFTED: &str = include_str!("data/kotlin_metadata/crafted.txt");

fn esc(s: &str) -> String {
    let mut out = String::new();
    for c in s.utf16() {
        if c <= 0x20 || c >= 0x7f || "\\:[]{}#".contains(char::from(c as u8)) {
            out.push_str(&format!("\\u{:04x}", c));
        } else {
            out.push(c as u8 as char);
        }
    }
    out
}

fn unesc(s: &str) -> String {
    let b = s.as_bytes();
    let mut units = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            units.push(u16::from_str_radix(&s[i + 2..i + 6], 16).unwrap());
            i += 6;
        } else {
            units.push(b[i] as u16);
            i += 1;
        }
    }
    from_utf16(&units)
}

/// Java `Error`s, which `catch (Exception)` lets through (the dumper marks them `!!`).
const JAVA_ERRORS: &[&str] = &["NoSuchMethodError", "OutOfMemoryError"];

fn exc(c: &Caught) -> String {
    match &c.exception {
        Some(e) if e.class == "NullPointerException" || e.class == "ClassCastException" => {
            format!("!{}", e.class)
        }
        Some(e) if JAVA_ERRORS.contains(&e.class) => format!("!!{}: {}", e.class, esc(&e.message)),
        Some(e) => format!("!{}: {}", e.class, esc(&e.message)),
        None => format!("!RustPanic: {}", esc(&c.message)),
    }
}

fn parse_const(t: &str) -> ConstValue {
    let v = &t[1..];
    match t.as_bytes()[0] {
        b'I' => ConstValue::Integer(v.parse().unwrap()),
        b'J' => ConstValue::Long(v.parse().unwrap()),
        b'F' => ConstValue::Float(f32::from_bits(v.parse::<i32>().unwrap() as u32)),
        b'D' => ConstValue::Double(f64::from_bits(v.parse::<i64>().unwrap() as u64)),
        b's' => ConstValue::String(unesc(v).into()),
        _ => panic!("const {}", t),
    }
}

fn pool_constant(value: ConstValue) -> PrimitiveConstant {
    let type_ = match value {
        ConstValue::Integer(_) => CONSTANT_INTEGER,
        ConstValue::Long(_) => CONSTANT_LONG,
        ConstValue::Float(_) => CONSTANT_FLOAT,
        ConstValue::Double(_) => CONSTANT_DOUBLE,
        ConstValue::String(_) => CONSTANT_UTF8,
    };
    PrimitiveConstant::new(type_, value)
}

fn parse_elem(toks: &[&str], i: &mut usize) -> AnnotationElement {
    let t = toks[*i];
    *i += 1;
    let tag = t.as_bytes()[0];
    match tag {
        b'B' | b'C' | b'I' | b'S' | b'Z' | b'J' | b'F' | b'D' => AnnotationElement::Const {
            tag,
            index: 0,
            cn: pool_constant(parse_const(&t[2..])),
        },
        b's' => {
            let value = match t.strip_prefix("s:#") {
                Some(c) => parse_const(c),
                None => ConstValue::String(unesc(&t[2..]).into()),
            };
            AnnotationElement::Const {
                tag,
                index: 0,
                cn: pool_constant(value),
            }
        }
        b'c' => AnnotationElement::Class {
            index: 0,
            value: unesc(&t[2..]),
        },
        b'e' => {
            let parts: Vec<&str> = t[2..].split(':').collect();
            AnnotationElement::Enum {
                type_index: 0,
                const_index: 0,
                const_name: unesc(parts[1]),
                descr: FieldDescriptor::parse_descriptor(&unesc(parts[0])),
            }
        }
        b'[' => {
            let mut elements = Vec::new();
            while toks[*i] != "]" {
                elements.push(parse_elem(toks, i));
            }
            *i += 1;
            AnnotationElement::Array { elements }
        }
        b'@' => {
            let mut record = AnnotationRecord {
                type_index: 0,
                class_name: Some(unesc(&t[2..])),
                name_indices: Vec::new(),
                names: Vec::new(),
                values: Vec::new(),
            };
            *i += 1;
            while toks[*i] != "}" {
                record.name_indices.push(0);
                record.names.push(unesc(&toks[*i][2..]));
                *i += 1;
                record.values.push(parse_elem(toks, i));
            }
            *i += 1;
            AnnotationElement::Annotation(record)
        }
        _ => panic!("element {}", t),
    }
}

/// The annotation whose `name value` lines are `raw`.
fn annotation(raw: &[&str]) -> KotlinMetadataAnnotation {
    let mut record = AnnotationRecord {
        type_index: 0,
        class_name: Some("kotlin/Metadata".to_string()),
        name_indices: Vec::new(),
        names: Vec::new(),
        values: Vec::new(),
    };
    for line in raw {
        let toks: Vec<&str> = line.split(' ').collect();
        record.name_indices.push(0);
        record.names.push(unesc(toks[0]));
        let mut i = 1;
        record.values.push(parse_elem(&toks, &mut i));
        assert_eq!(i, toks.len());
    }
    KotlinMetadataAnnotation::new(record)
}

/// What `KtMetadata::decode` makes of the annotation, as the plugin dump writes it.
fn outcome(a: &KotlinMetadataAnnotation) -> String {
    let decoded = try_catch(|| {
        let Some(k) = a.get_k() else {
            return "nok".to_string();
        };
        let Some(d1) = a.get_d1() else {
            return "nod1".to_string();
        };
        match KtMetadata::decode(k, &d1, a.get_d2().as_deref(), None).message {
            KotlinMetadataMessage::Class(_) => "msg Class".to_string(),
            KotlinMetadataMessage::File(_) => "msg File".to_string(),
            KotlinMetadataMessage::SyntheticClass(_) => "msg SyntheticClass".to_string(),
            KotlinMetadataMessage::MultifileFacade(p) => format!("msg MultifileFacade {}", p.len()),
            KotlinMetadataMessage::MultifilePart { .. } => "msg MultifilePart".to_string(),
            KotlinMetadataMessage::None => "msg none".to_string(),
        }
    });
    decoded.unwrap_or_else(|c| exc(&c))
}

/// `name <index> = <name or exception>` for every index the annotation's resolver answers.
fn names(a: &KotlinMetadataAnnotation) -> Vec<String> {
    let (Some(d1), Some(d2)) = (a.get_d1(), a.get_d2()) else {
        return Vec::new();
    };
    let resolver = KtMetadata::read_resolver(&mut KtMetadata::open(&d1), &d2);
    (-1..=resolver.records_size())
        .map(|i| match try_catch(|| resolver.resolve(i)) {
            Ok(name) => format!("name {i} = {}", esc(&name)),
            Err(c) => format!("name {i} = {}", exc(&c)),
        })
        .collect()
}

#[test]
fn crafted_metadata_decodes_like_the_plugin() {
    let mut cases: Vec<(&str, Vec<&str>, Vec<&str>)> = Vec::new();
    for line in CRAFTED.lines() {
        match cases.last_mut() {
            Some((_, raw, _)) if line.starts_with("  ") => raw.push(line.trim_start()),
            Some((_, _, expected)) if line.starts_with("= ") || line.starts_with("name ") => {
                expected.push(line)
            }
            _ => cases.push((line, Vec::new(), Vec::new())),
        }
    }
    assert!(cases.len() > 150, "{} cases", cases.len());
    let mut mismatches = Vec::new();
    for (label, raw, expected) in cases {
        let a = annotation(&raw);
        let mut got = vec![format!("= {}", outcome(&a))];
        if expected.len() > 1 {
            got.extend(names(&a));
        }
        if got != expected {
            mismatches.push(format!("{label}: {got:?}\n  plugin: {expected:?}"));
        }
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}
