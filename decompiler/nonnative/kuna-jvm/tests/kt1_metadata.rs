//! KT1: the Kotlin metadata decoder against dumps made by the Kotlin plugin's
//! own `BitEncoding`, shaded protobuf classes and `MetadataNameResolver`
//! (`data/kt1/KtMetadataDump.java`; regenerate with `data/kt1/gen.sh`).
//! Opt-in sweep: `KT1_SWEEP_DIR=<dir of full dumps>` and, to also read the
//! annotations through `StructClass`, `KT1_SWEEP_JARS=label=jar,label=jar`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::code_constants::{
    CONSTANT_DOUBLE, CONSTANT_FLOAT, CONSTANT_INTEGER, CONSTANT_LONG, CONSTANT_UTF8,
};
use kuna_jvm::field_descriptor::FieldDescriptor;
use kuna_jvm::kt_flags::{BooleanFlagField, EnumLiteFlagField, FlagEnum, Flags};
use kuna_jvm::kt_metadata::{KotlinMetadataAnnotation, KotlinMetadataMessage, KtMetadata};
use kuna_jvm::kt_proto::{
    Class, ClassKind, Constructor, Contract, Expression, Function, JvmFieldSignature,
    JvmMethodSignature, MemberKind, Modality, Package, Property, Record, ReturnValueStatus,
    StringTableTypes, Type, TypeAlias, TypeParameter, TypeTable, ValueParameter, Visibility,
};
use kuna_jvm::metadata_name_resolver::MetadataNameResolver;
use kuna_jvm::primitive_constant::{ConstValue, PrimitiveConstant};
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{try_catch, Caught, JavaStr};
use kuna_jvm::struct_annotation_attribute::{AnnotationElement, AnnotationRecord};
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::zip_reader::ZipReader;

use crate::common;

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/kt1")
}

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

// ---- raw annotation values

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

fn const_text(v: &ConstValue) -> String {
    match v {
        ConstValue::Integer(i) => format!("I{}", i),
        ConstValue::Long(l) => format!("J{}", l),
        ConstValue::Float(f) => format!("F{}", f.to_bits() as i32),
        ConstValue::Double(d) => format!("D{}", d.to_bits() as i64),
        ConstValue::String(s) => format!("s{}", esc(s)),
    }
}

fn render_elem(e: &AnnotationElement) -> String {
    match e {
        AnnotationElement::Const { tag, cn, .. } => {
            let v = cn.value.as_ref().unwrap();
            match (tag, v) {
                (b's', ConstValue::String(s)) => format!("s:{}", esc(s)),
                (b's', other) => format!("s:#{}", const_text(other)),
                _ => format!("{}:{}", *tag as char, const_text(v)),
            }
        }
        AnnotationElement::Class { value, .. } => format!("c:{}", esc(value)),
        AnnotationElement::Enum {
            const_name, descr, ..
        } => format!("e:{}:{}", esc(&descr.descriptor_string), esc(const_name)),
        AnnotationElement::Array { elements } => {
            let mut s = String::from("[");
            for x in elements {
                s.push(' ');
                s.push_str(&render_elem(x));
            }
            s.push_str(" ]");
            s
        }
        AnnotationElement::Annotation(a) => {
            let mut s = format!("@:{} {{", esc(a.class_name.as_deref().unwrap_or("")));
            for (n, v) in a.names.iter().zip(&a.values) {
                s.push_str(&format!(" n:{} {}", esc(n), render_elem(v)));
            }
            s.push_str(" }");
            s
        }
    }
}

fn raw_lines(record: &AnnotationRecord) -> Vec<String> {
    record
        .names
        .iter()
        .zip(&record.values)
        .map(|(n, v)| format!("raw {} {}", esc(n), render_elem(v)))
        .collect()
}

// ---- the dump (mirror of KtMetadataDump.java)

#[derive(Clone, Copy)]
enum FlagRef {
    Bool(BooleanFlagField),
    Visibility(EnumLiteFlagField<Visibility>),
    Modality(EnumLiteFlagField<Modality>),
    ClassKind(EnumLiteFlagField<ClassKind>),
    MemberKind(EnumLiteFlagField<MemberKind>),
    ReturnValueStatus(EnumLiteFlagField<ReturnValueStatus>),
}

fn enum_text<E: FlagEnum>(f: EnumLiteFlagField<E>, v: i32, name: fn(E) -> &'static str) -> String {
    f.get(v).map_or("null".to_string(), |e| name(e).to_string())
}

impl FlagRef {
    fn text(self, v: i32) -> String {
        match self {
            FlagRef::Bool(f) => f.get(v).to_string(),
            FlagRef::Visibility(f) => enum_text(f, v, Visibility::name),
            FlagRef::Modality(f) => enum_text(f, v, Modality::name),
            FlagRef::ClassKind(f) => enum_text(f, v, ClassKind::name),
            FlagRef::MemberKind(f) => enum_text(f, v, MemberKind::name),
            FlagRef::ReturnValueStatus(f) => enum_text(f, v, ReturnValueStatus::name),
        }
    }

    fn layout(self) -> (i32, i32) {
        match self {
            FlagRef::Bool(f) => (f.offset, 1),
            FlagRef::Visibility(f) => (f.offset, f.bit_width),
            FlagRef::Modality(f) => (f.offset, f.bit_width),
            FlagRef::ClassKind(f) => (f.offset, f.bit_width),
            FlagRef::MemberKind(f) => (f.offset, f.bit_width),
            FlagRef::ReturnValueStatus(f) => (f.offset, f.bit_width),
        }
    }
}

fn all_flags() -> Vec<(&'static str, FlagRef)> {
    use FlagRef::*;
    vec![
        ("SUSPEND_TYPE", Bool(Flags::SUSPEND_TYPE)),
        (
            "DEFINITELY_NOT_NULL_TYPE",
            Bool(Flags::DEFINITELY_NOT_NULL_TYPE),
        ),
        ("HAS_ANNOTATIONS", Bool(Flags::HAS_ANNOTATIONS)),
        ("VISIBILITY", Visibility(Flags::VISIBILITY)),
        ("MODALITY", Modality(Flags::MODALITY)),
        ("CLASS_KIND", ClassKind(Flags::CLASS_KIND)),
        ("IS_INNER", Bool(Flags::IS_INNER)),
        ("IS_DATA", Bool(Flags::IS_DATA)),
        ("IS_EXTERNAL_CLASS", Bool(Flags::IS_EXTERNAL_CLASS)),
        ("IS_EXPECT_CLASS", Bool(Flags::IS_EXPECT_CLASS)),
        ("IS_VALUE_CLASS", Bool(Flags::IS_VALUE_CLASS)),
        ("IS_FUN_INTERFACE", Bool(Flags::IS_FUN_INTERFACE)),
        ("HAS_ENUM_ENTRIES", Bool(Flags::HAS_ENUM_ENTRIES)),
        ("IS_SECONDARY", Bool(Flags::IS_SECONDARY)),
        (
            "IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES",
            Bool(Flags::IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES),
        ),
        (
            "RETURN_VALUE_STATUS_CTOR",
            ReturnValueStatus(Flags::RETURN_VALUE_STATUS_CTOR),
        ),
        ("MEMBER_KIND", MemberKind(Flags::MEMBER_KIND)),
        ("IS_OPERATOR", Bool(Flags::IS_OPERATOR)),
        ("IS_INFIX", Bool(Flags::IS_INFIX)),
        ("IS_INLINE", Bool(Flags::IS_INLINE)),
        ("IS_TAILREC", Bool(Flags::IS_TAILREC)),
        ("IS_EXTERNAL_FUNCTION", Bool(Flags::IS_EXTERNAL_FUNCTION)),
        ("IS_SUSPEND", Bool(Flags::IS_SUSPEND)),
        ("IS_EXPECT_FUNCTION", Bool(Flags::IS_EXPECT_FUNCTION)),
        (
            "IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES",
            Bool(Flags::IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES),
        ),
        (
            "RETURN_VALUE_STATUS_FUNCTION",
            ReturnValueStatus(Flags::RETURN_VALUE_STATUS_FUNCTION),
        ),
        ("IS_VAR", Bool(Flags::IS_VAR)),
        ("HAS_GETTER", Bool(Flags::HAS_GETTER)),
        ("HAS_SETTER", Bool(Flags::HAS_SETTER)),
        ("IS_CONST", Bool(Flags::IS_CONST)),
        ("IS_LATEINIT", Bool(Flags::IS_LATEINIT)),
        ("HAS_CONSTANT", Bool(Flags::HAS_CONSTANT)),
        ("IS_EXTERNAL_PROPERTY", Bool(Flags::IS_EXTERNAL_PROPERTY)),
        ("IS_DELEGATED", Bool(Flags::IS_DELEGATED)),
        ("IS_EXPECT_PROPERTY", Bool(Flags::IS_EXPECT_PROPERTY)),
        (
            "RETURN_VALUE_STATUS_PROPERTY",
            ReturnValueStatus(Flags::RETURN_VALUE_STATUS_PROPERTY),
        ),
        (
            "DECLARES_DEFAULT_VALUE",
            Bool(Flags::DECLARES_DEFAULT_VALUE),
        ),
        ("IS_CROSSINLINE", Bool(Flags::IS_CROSSINLINE)),
        ("IS_NOINLINE", Bool(Flags::IS_NOINLINE)),
        ("IS_NOT_DEFAULT", Bool(Flags::IS_NOT_DEFAULT)),
        ("IS_EXTERNAL_ACCESSOR", Bool(Flags::IS_EXTERNAL_ACCESSOR)),
        ("IS_INLINE_ACCESSOR", Bool(Flags::IS_INLINE_ACCESSOR)),
        ("IS_NEGATED", Bool(Flags::IS_NEGATED)),
        (
            "IS_NULL_CHECK_PREDICATE",
            Bool(Flags::IS_NULL_CHECK_PREDICATE),
        ),
        ("IS_UNSIGNED", Bool(Flags::IS_UNSIGNED)),
    ]
}

fn flag_set(names: &[&str]) -> Vec<(&'static str, FlagRef)> {
    let all = all_flags();
    names
        .iter()
        .map(|n| *all.iter().find(|(m, _)| m == n).unwrap())
        .collect()
}

struct Dumper {
    out: Vec<String>,
    resolver: Option<MetadataNameResolver>,
    class_flags: Vec<(&'static str, FlagRef)>,
    constructor_flags: Vec<(&'static str, FlagRef)>,
    function_flags: Vec<(&'static str, FlagRef)>,
    property_flags: Vec<(&'static str, FlagRef)>,
    accessor_flags: Vec<(&'static str, FlagRef)>,
    parameter_flags: Vec<(&'static str, FlagRef)>,
    expression_flags: Vec<(&'static str, FlagRef)>,
    fields: usize,
}

fn sign(has: bool) -> &'static str {
    if has {
        " +"
    } else {
        " -"
    }
}

impl Dumper {
    fn new() -> Dumper {
        Dumper {
            out: Vec::new(),
            resolver: None,
            class_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "VISIBILITY",
                "MODALITY",
                "CLASS_KIND",
                "IS_INNER",
                "IS_DATA",
                "IS_EXTERNAL_CLASS",
                "IS_EXPECT_CLASS",
                "IS_VALUE_CLASS",
                "IS_FUN_INTERFACE",
                "HAS_ENUM_ENTRIES",
            ]),
            constructor_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "VISIBILITY",
                "IS_SECONDARY",
                "IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES",
                "RETURN_VALUE_STATUS_CTOR",
            ]),
            function_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "VISIBILITY",
                "MODALITY",
                "MEMBER_KIND",
                "IS_OPERATOR",
                "IS_INFIX",
                "IS_INLINE",
                "IS_TAILREC",
                "IS_EXTERNAL_FUNCTION",
                "IS_SUSPEND",
                "IS_EXPECT_FUNCTION",
                "IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES",
                "RETURN_VALUE_STATUS_FUNCTION",
            ]),
            property_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "VISIBILITY",
                "MODALITY",
                "MEMBER_KIND",
                "IS_VAR",
                "HAS_GETTER",
                "HAS_SETTER",
                "IS_CONST",
                "IS_LATEINIT",
                "HAS_CONSTANT",
                "IS_EXTERNAL_PROPERTY",
                "IS_DELEGATED",
                "IS_EXPECT_PROPERTY",
                "RETURN_VALUE_STATUS_PROPERTY",
            ]),
            accessor_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "VISIBILITY",
                "MODALITY",
                "IS_NOT_DEFAULT",
                "IS_EXTERNAL_ACCESSOR",
                "IS_INLINE_ACCESSOR",
            ]),
            parameter_flags: flag_set(&[
                "HAS_ANNOTATIONS",
                "DECLARES_DEFAULT_VALUE",
                "IS_CROSSINLINE",
                "IS_NOINLINE",
            ]),
            expression_flags: flag_set(&["IS_NEGATED", "IS_NULL_CHECK_PREDICATE"]),
            fields: 0,
        }
    }

    fn p(&mut self, s: String) {
        self.out.push(s);
    }

    fn nm(&self, v: i32) -> String {
        let Some(resolver) = &self.resolver else {
            return String::new();
        };
        match try_catch(|| resolver.resolve(v)) {
            Ok(s) => format!(" = {}", esc(&s)),
            Err(c) => format!(" = {}", exc(&c)),
        }
    }

    fn i(&mut self, ind: &str, name: &str, has: bool, v: i32, is_name: bool) {
        self.fields += 1;
        let n = if has && is_name {
            self.nm(v)
        } else {
            String::new()
        };
        self.p(format!("{}{}{} {}{}", ind, name, sign(has), v, n));
    }

    fn b(&mut self, ind: &str, name: &str, has: bool, v: bool) {
        self.fields += 1;
        self.p(format!("{}{}{} {}", ind, name, sign(has), v));
    }

    fn e(&mut self, ind: &str, name: &str, has: bool, v: &str) {
        self.fields += 1;
        self.p(format!("{}{}{} {}", ind, name, sign(has), v));
    }

    fn list(&mut self, ind: &str, name: &str, l: &[i32]) {
        self.fields += 1;
        let mut s = format!("{}{} {}", ind, name, l.len());
        for x in l {
            s.push_str(&format!(" {}", x));
        }
        self.p(s);
    }

    fn flags(&mut self, ind: &str, name: &str, has: bool, v: i32, set: &[(&'static str, FlagRef)]) {
        self.fields += 1 + set.len();
        let mut s = format!("{}{}{} {} {{", ind, name, sign(has), v);
        for (n, f) in set {
            s.push_str(&format!(" {}={}", n, f.text(v)));
        }
        s.push_str(" }");
        self.p(s);
    }

    fn opt(&mut self, ind: &str, name: &str, has: bool) {
        self.fields += 1;
        self.p(format!("{}{}{}", ind, name, sign(has)));
    }

    fn count(&mut self, ind: &str, name: &str, n: i32) {
        self.fields += 1;
        self.p(format!("{}{} {}", ind, name, n));
    }

    fn record(&mut self, r: &Record, ind: &str) {
        self.p(format!("{}record", ind));
        let n = format!("{} ", ind);
        self.i(&n, "range", r.has_range(), r.get_range(), false);
        self.i(
            &n,
            "predefined_index",
            r.has_predefined_index(),
            r.get_predefined_index(),
            false,
        );
        let s = match try_catch(|| r.get_string()) {
            Ok(s) => esc(&s),
            Err(c) => exc(&c),
        };
        self.fields += 1;
        self.p(format!("{}string{} {}", n, sign(r.has_string()), s));
        self.e(&n, "operation", r.has_operation(), r.get_operation().name());
        assert_eq!(
            r.get_substring_index_count() as usize,
            r.get_substring_index_list().len()
        );
        self.list(&n, "substring_index", &r.get_substring_index_list());
        assert_eq!(
            r.get_replace_char_count() as usize,
            r.get_replace_char_list().len()
        );
        self.list(&n, "replace_char", &r.get_replace_char_list());
    }

    fn class(&mut self, c: &Class, ind: &str) {
        let set = self.class_flags.clone();
        self.flags(ind, "flags", c.has_flags(), c.get_flags(), &set);
        self.i(ind, "fq_name", c.has_fq_name(), c.get_fq_name(), true);
        self.i(
            ind,
            "companion_object_name",
            c.has_companion_object_name(),
            c.get_companion_object_name(),
            true,
        );
        let n = format!("{} ", ind);
        self.count(ind, "supertype", c.get_supertype_count());
        for (k, t) in c.get_supertype_list().iter().enumerate() {
            assert!(c.get_supertype(k as i32).ptr_eq(t));
            self.type_(t, &n);
        }
        self.count(ind, "constructor", c.get_constructor_count());
        for x in c.get_constructor_list() {
            self.constructor(&x, &n);
        }
        self.count(ind, "function", c.get_function_count());
        for x in c.get_function_list() {
            self.function(&x, &n);
        }
        self.count(ind, "property", c.get_property_count());
        for x in c.get_property_list() {
            self.property(&x, &n);
        }
        self.count(ind, "type_alias", c.get_type_alias_count());
        for x in c.get_type_alias_list() {
            self.type_alias(&x, &n);
        }
        self.opt(ind, "type_table", c.has_type_table());
        if c.has_type_table() {
            self.type_table(&c.get_type_table(), &n);
        }
        self.i(
            ind,
            "ext.class_module_name",
            c.has_extension_class_module_name(),
            c.get_extension_class_module_name(),
            true,
        );
        self.count(
            ind,
            "ext.class_local_variable",
            c.get_extension_class_local_variable_count(),
        );
        for x in c.get_extension_class_local_variable_list() {
            self.property(&x, &n);
        }
        self.i(
            ind,
            "ext.anonymous_object_origin_name",
            c.has_extension_anonymous_object_origin_name(),
            c.get_extension_anonymous_object_origin_name(),
            true,
        );
        self.i(
            ind,
            "ext.jvm_class_flags",
            c.has_extension_jvm_class_flags(),
            c.get_extension_jvm_class_flags(),
            false,
        );
    }

    fn package(&mut self, c: &Package, ind: &str) {
        let n = format!("{} ", ind);
        self.count(ind, "function", c.get_function_count());
        for x in c.get_function_list() {
            self.function(&x, &n);
        }
        self.count(ind, "property", c.get_property_count());
        for x in c.get_property_list() {
            self.property(&x, &n);
        }
        self.count(ind, "type_alias", c.get_type_alias_count());
        for x in c.get_type_alias_list() {
            self.type_alias(&x, &n);
        }
        self.opt(ind, "type_table", c.has_type_table());
        if c.has_type_table() {
            self.type_table(&c.get_type_table(), &n);
        }
        self.i(
            ind,
            "ext.package_module_name",
            c.has_extension_package_module_name(),
            c.get_extension_package_module_name(),
            true,
        );
        self.count(
            ind,
            "ext.package_local_variable",
            c.get_extension_package_local_variable_count(),
        );
        for x in c.get_extension_package_local_variable_list() {
            self.property(&x, &n);
        }
    }

    fn function(&mut self, f: &Function, ind: &str) {
        self.p(format!("{}Function", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        let set = self.function_flags.clone();
        self.flags(&ind, "flags", f.has_flags(), f.get_flags(), &set);
        self.i(&ind, "name", f.has_name(), f.get_name(), true);
        self.opt(&ind, "return_type", f.has_return_type());
        if f.has_return_type() {
            self.type_(&f.get_return_type(), &n);
        }
        self.count(&ind, "type_parameter", f.get_type_parameter_count());
        for x in f.get_type_parameter_list() {
            self.type_parameter(&x, &n);
        }
        self.opt(&ind, "receiver_type", f.has_receiver_type());
        if f.has_receiver_type() {
            self.type_(&f.get_receiver_type(), &n);
        }
        self.count(&ind, "value_parameter", f.get_value_parameter_count());
        for k in 0..f.get_value_parameter_count() {
            self.value_parameter(&f.get_value_parameter(k), &n);
        }
        self.count(
            &ind,
            "context_receiver_type",
            f.get_context_receiver_type_count(),
        );
        for x in f.get_context_receiver_type_list() {
            self.type_(&x, &n);
        }
        self.opt(&ind, "type_table", f.has_type_table());
        if f.has_type_table() {
            self.type_table(&f.get_type_table(), &n);
        }
        self.opt(&ind, "contract", f.has_contract());
        if f.has_contract() {
            self.contract(&f.get_contract(), &n);
        }
        self.opt(
            &ind,
            "ext.method_signature",
            f.has_extension_method_signature(),
        );
        self.method_signature(&f.get_extension_method_signature(), &n);
        self.i(
            &ind,
            "ext.lambda_class_origin_name",
            f.has_extension_lambda_class_origin_name(),
            f.get_extension_lambda_class_origin_name(),
            true,
        );
    }

    fn property(&mut self, f: &Property, ind: &str) {
        self.p(format!("{}Property", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        let set = self.property_flags.clone();
        self.flags(&ind, "flags", f.has_flags(), f.get_flags(), &set);
        self.i(&ind, "name", f.has_name(), f.get_name(), true);
        self.opt(&ind, "return_type", f.has_return_type());
        if f.has_return_type() {
            self.type_(&f.get_return_type(), &n);
        }
        self.opt(
            &ind,
            "setter_value_parameter",
            f.has_setter_value_parameter(),
        );
        if f.has_setter_value_parameter() {
            self.value_parameter(&f.get_setter_value_parameter(), &n);
        }
        let set = self.accessor_flags.clone();
        self.flags(
            &ind,
            "getter_flags",
            f.has_getter_flags(),
            f.get_getter_flags(),
            &set,
        );
        self.flags(
            &ind,
            "setter_flags",
            f.has_setter_flags(),
            f.get_setter_flags(),
            &set,
        );
        self.opt(
            &ind,
            "ext.property_signature",
            f.has_extension_property_signature(),
        );
        let s = f.get_extension_property_signature();
        let nn = format!("{} ", n);
        self.opt(&n, "field", s.has_field());
        self.field_signature(&s.get_field(), &nn);
        self.opt(&n, "synthetic_method", s.has_synthetic_method());
        if s.has_synthetic_method() {
            self.method_signature(&s.get_synthetic_method(), &nn);
        }
        self.opt(&n, "getter", s.has_getter());
        if s.has_getter() {
            self.method_signature(&s.get_getter(), &nn);
        }
        self.opt(&n, "setter", s.has_setter());
        if s.has_setter() {
            self.method_signature(&s.get_setter(), &nn);
        }
        self.i(
            &ind,
            "ext.flags",
            f.has_extension_flags(),
            f.get_extension_flags(),
            false,
        );
    }

    fn constructor(&mut self, c: &Constructor, ind: &str) {
        self.p(format!("{}Constructor", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        let set = self.constructor_flags.clone();
        self.flags(&ind, "flags", c.has_flags(), c.get_flags(), &set);
        self.count(&ind, "value_parameter", c.get_value_parameter_count());
        for x in c.get_value_parameter_list() {
            self.value_parameter(&x, &n);
        }
        self.opt(
            &ind,
            "ext.constructor_signature",
            c.has_extension_constructor_signature(),
        );
        self.method_signature(&c.get_extension_constructor_signature(), &n);
    }

    fn value_parameter(&mut self, v: &ValueParameter, ind: &str) {
        self.p(format!("{}ValueParameter", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        let set = self.parameter_flags.clone();
        self.flags(&ind, "flags", v.has_flags(), v.get_flags(), &set);
        self.i(&ind, "name", v.has_name(), v.get_name(), true);
        self.opt(&ind, "type", v.has_type());
        if v.has_type() {
            self.type_(&v.get_type(), &n);
        }
        self.opt(&ind, "vararg_element_type", v.has_vararg_element_type());
        if v.has_vararg_element_type() {
            self.type_(&v.get_vararg_element_type(), &n);
        }
        self.i(&ind, "type_id", v.has_type_id(), v.get_type_id(), false);
    }

    fn type_(&mut self, t: &Type, ind: &str) {
        self.p(format!("{}Type", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        self.count(&ind, "argument", t.get_argument_count());
        for k in 0..t.get_argument_count() {
            let a = t.get_argument(k);
            self.p(format!("{}Argument", n));
            let nn = format!("{} ", n);
            self.e(
                &nn,
                "projection",
                a.has_projection(),
                a.get_projection().name(),
            );
            self.opt(&nn, "type", a.has_type());
            if a.has_type() {
                self.type_(&a.get_type(), &format!("{}  ", n));
            }
            self.i(&nn, "type_id", a.has_type_id(), a.get_type_id(), false);
        }
        self.b(&ind, "nullable", t.has_nullable(), t.get_nullable());
        self.i(
            &ind,
            "class_name",
            t.has_class_name(),
            t.get_class_name(),
            true,
        );
        self.i(
            &ind,
            "type_parameter_name",
            t.has_type_parameter_name(),
            t.get_type_parameter_name(),
            true,
        );
        self.i(
            &ind,
            "type_alias_name",
            t.has_type_alias_name(),
            t.get_type_alias_name(),
            true,
        );
        self.b(
            &ind,
            "ext.is_raw",
            t.has_extension_is_raw(),
            t.get_extension_is_raw(),
        );
    }

    fn type_parameter(&mut self, t: &TypeParameter, ind: &str) {
        self.p(format!("{}TypeParameter", ind));
        let ind = format!("{} ", ind);
        self.i(&ind, "id", t.has_id(), t.get_id(), false);
        self.i(&ind, "name", t.has_name(), t.get_name(), true);
        self.b(&ind, "reified", t.has_reified(), t.get_reified());
        self.e(&ind, "variance", t.has_variance(), t.get_variance().name());
        self.count(&ind, "upper_bound", t.get_upper_bound_count());
        for x in t.get_upper_bound_list() {
            self.type_(&x, &format!("{} ", ind));
        }
    }

    fn type_table(&mut self, t: &TypeTable, ind: &str) {
        self.count(ind, "type", t.get_type_count());
        for x in t.get_type_list() {
            self.type_(&x, &format!("{} ", ind));
        }
        self.i(
            ind,
            "first_nullable",
            t.has_first_nullable(),
            t.get_first_nullable(),
            false,
        );
    }

    fn type_alias(&mut self, t: &TypeAlias, ind: &str) {
        self.p(format!("{}TypeAlias", ind));
        let ind = format!("{} ", ind);
        self.i(&ind, "name", t.has_name(), t.get_name(), true);
        self.opt(&ind, "underlying_type", t.has_underlying_type());
        if t.has_underlying_type() {
            self.type_(&t.get_underlying_type(), &format!("{} ", ind));
        }
    }

    fn contract(&mut self, c: &Contract, ind: &str) {
        self.count(ind, "effect", c.get_effect_count());
        for x in c.get_effect_list() {
            let n = format!("{}  ", ind);
            self.p(format!("{} Effect", ind));
            self.e(
                &n,
                "effect_type",
                x.has_effect_type(),
                x.get_effect_type().name(),
            );
            self.count(
                &n,
                "effect_constructor_argument",
                x.get_effect_constructor_argument_count(),
            );
            for y in x.get_effect_constructor_argument_list() {
                self.expression(&y, &format!("{} ", n));
            }
            self.opt(
                &n,
                "conclusion_of_conditional_effect",
                x.has_conclusion_of_conditional_effect(),
            );
            if x.has_conclusion_of_conditional_effect() {
                self.expression(
                    &x.get_conclusion_of_conditional_effect(),
                    &format!("{} ", n),
                );
            }
            self.e(&n, "kind", x.has_kind(), x.get_kind().name());
        }
    }

    fn expression(&mut self, x: &Expression, ind: &str) {
        self.p(format!("{}Expression", ind));
        let ind = format!("{} ", ind);
        let n = format!("{} ", ind);
        let set = self.expression_flags.clone();
        self.flags(&ind, "flags", x.has_flags(), x.get_flags(), &set);
        self.i(
            &ind,
            "value_parameter_reference",
            x.has_value_parameter_reference(),
            x.get_value_parameter_reference(),
            false,
        );
        self.e(
            &ind,
            "constant_value",
            x.has_constant_value(),
            x.get_constant_value().name(),
        );
        self.opt(&ind, "is_instance_type", x.has_is_instance_type());
        if x.has_is_instance_type() {
            self.type_(&x.get_is_instance_type(), &n);
        }
        self.i(
            &ind,
            "is_instance_type_id",
            x.has_is_instance_type_id(),
            x.get_is_instance_type_id(),
            false,
        );
        self.count(&ind, "and_argument", x.get_and_argument_count());
        for y in x.get_and_argument_list() {
            self.expression(&y, &n);
        }
        self.count(&ind, "or_argument", x.get_or_argument_count());
        for y in x.get_or_argument_list() {
            self.expression(&y, &n);
        }
    }

    fn method_signature(&mut self, s: &JvmMethodSignature, ind: &str) {
        self.i(ind, "name", s.has_name(), s.get_name(), true);
        self.i(ind, "desc", s.has_desc(), s.get_desc(), true);
    }

    fn field_signature(&mut self, s: &JvmFieldSignature, ind: &str) {
        self.i(ind, "name", s.has_name(), s.get_name(), true);
        self.i(ind, "desc", s.has_desc(), s.get_desc(), true);
    }

    fn decode(&mut self, a: &KotlinMetadataAnnotation) {
        let k_index = a.index_of("k");
        let d1_index = a.index_of("d1");
        let d2_index = a.index_of("d2");
        if k_index == -1 {
            self.p("nok".into());
            return;
        }
        let result = try_catch(|| {
            let k = KotlinMetadataAnnotation::int_value(a.get_par_value(k_index));
            self.p(format!("k {}", k));
            if d1_index == -1 {
                self.p("nod1".into());
                return;
            }
            if d2_index == -1 {
                self.p("nod2".into());
            }
            let data1 = KotlinMetadataAnnotation::data_from_expr(a.get_par_value(d1_index));
            let buf = kuna_jvm::bit_encoding::BitEncoding::decode_bytes(&data1);
            let mut crc = flate2::Crc::new();
            crc.update(&buf);
            self.p(format!("bytes {} {:08x}", buf.len(), crc.sum()));
            let mut input = KtMetadata::open(&data1);
            if d2_index != -1 {
                let data2 = KotlinMetadataAnnotation::data_from_expr(a.get_par_value(d2_index));
                let mut probe = KtMetadata::open(&data1);
                let types = StringTableTypes::parse_delimited_from(&mut probe);
                self.p(format!(
                    "types {} left {}",
                    types
                        .as_ref()
                        .map_or("null".to_string(), |t| t.get_record_count().to_string()),
                    probe.available()
                ));
                let resolver = KtMetadata::read_resolver(&mut input, &data2);
                self.resolver = Some(resolver.clone());
                self.p(format!("resolver {}", resolver.records_size()));
                for r in resolver.get_types().get_record_list() {
                    self.record(&r, " ");
                }
            } else {
                self.p("resolver none".into());
            }
            match KtMetadata::read_message(&mut input, k, &data1, None) {
                KotlinMetadataMessage::Class(c) => {
                    self.p("msg Class".into());
                    self.class(&c, " ");
                }
                KotlinMetadataMessage::File(p) => {
                    self.p("msg File".into());
                    self.package(&p, " ");
                }
                KotlinMetadataMessage::SyntheticClass(f) => {
                    self.p("msg SyntheticClass".into());
                    self.function(&f, " ");
                }
                KotlinMetadataMessage::MultifileFacade(parts) => {
                    self.p(format!("msg MultifileFacade {}", parts.len()));
                    for s in parts {
                        self.p(format!(" {}", esc(&s)));
                    }
                }
                KotlinMetadataMessage::MultifilePart { package, .. } => {
                    self.p("msg MultifilePart".into());
                    self.package(&package, " ");
                    let xs = match a.index_of("xs") {
                        -1 => "xs none".to_string(),
                        i => format!(
                            "xs {}",
                            esc(&KotlinMetadataAnnotation::string_value(a.get_par_value(i)))
                        ),
                    };
                    self.p(xs);
                }
                KotlinMetadataMessage::None => self.p("msg none".into()),
            }
        });
        if let Err(c) = result {
            let line = exc(&c);
            self.p(line);
        }
    }

    fn run(&mut self, label: &str, a: &KotlinMetadataAnnotation) {
        self.p(format!("=== {}", label));
        for l in raw_lines(a.get_record()) {
            self.p(l);
        }
        self.resolver = None;
        self.decode(a);
        if let Some(resolver) = self.resolver.clone() {
            let size = resolver.records_size();
            self.p(format!("names {}", size));
            for i in -1..=size {
                let n = self.nm(i);
                self.p(format!(" {}{}", i, n));
            }
        }
        self.p("end".into());
    }
}

/// The `decode` convenience agrees with the step-by-step dump.
fn check_decode_agrees(a: &KotlinMetadataAnnotation, expected: &[String]) {
    let decoded = try_catch(|| {
        let k = a.get_k()?;
        let d1 = a.get_d1()?;
        let d2 = a.get_d2();
        let decoded = KtMetadata::decode(k, &d1, d2.as_deref(), None);
        Some(match decoded.message {
            KotlinMetadataMessage::Class(_) => "msg Class".to_string(),
            KotlinMetadataMessage::File(_) => "msg File".to_string(),
            KotlinMetadataMessage::SyntheticClass(_) => "msg SyntheticClass".to_string(),
            KotlinMetadataMessage::MultifileFacade(p) => format!("msg MultifileFacade {}", p.len()),
            KotlinMetadataMessage::MultifilePart { .. } => "msg MultifilePart".to_string(),
            KotlinMetadataMessage::None => "msg none".to_string(),
        })
    });
    match decoded {
        Ok(Some(line)) => assert!(expected.contains(&line), "decode() gave {}", line),
        Ok(None) => {}
        Err(c) => {
            let line = exc(&c);
            assert!(
                expected.iter().any(|l| l == &line) || expected.iter().any(|l| l.starts_with('!')),
                "decode() threw {}",
                line
            );
        }
    }
}

struct GoldenRecord {
    label: String,
    lines: Vec<String>,
}

fn read_golden(path: &Path) -> Vec<GoldenRecord> {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {}", path.display(), e));
    let mut records = Vec::new();
    let mut cur: Option<GoldenRecord> = None;
    for line in text.lines() {
        if let Some(label) = line.strip_prefix("=== ") {
            cur = Some(GoldenRecord {
                label: label.to_string(),
                lines: Vec::new(),
            });
        }
        let r = cur.as_mut().expect("record header");
        r.lines.push(line.to_string());
        if line == "end" {
            records.push(cur.take().unwrap());
        }
    }
    records
}

fn annotation_from_raw(lines: &[String]) -> KotlinMetadataAnnotation {
    let mut record = AnnotationRecord {
        type_index: 0,
        class_name: Some("kotlin/Metadata".to_string()),
        name_indices: Vec::new(),
        names: Vec::new(),
        values: Vec::new(),
    };
    for l in lines.iter().filter_map(|l| l.strip_prefix("raw ")) {
        let toks: Vec<&str> = l.split(' ').collect();
        record.name_indices.push(0);
        record.names.push(unesc(toks[0]));
        let mut i = 1;
        record.values.push(parse_elem(&toks, &mut i));
        assert_eq!(i, toks.len());
    }
    KotlinMetadataAnnotation::new(record)
}

#[derive(Default)]
struct Tally {
    records: usize,
    decoded: usize,
    failed: usize,
    fields: usize,
    names: usize,
    mismatches: Vec<String>,
}

fn compare(records: &[GoldenRecord], tally: &mut Tally) {
    for r in records {
        let a = annotation_from_raw(&r.lines);
        let mut d = Dumper::new();
        d.run(&r.label, &a);
        tally.records += 1;
        tally.fields += d.fields;
        tally.names += r
            .lines
            .iter()
            .find_map(|l| l.strip_prefix("names "))
            .map_or(0, |n| n.parse::<usize>().unwrap() + 2);
        if r.lines.iter().any(|l| l.starts_with("msg ")) {
            tally.decoded += 1;
        }
        if r.lines.iter().any(|l| l.starts_with('!')) {
            tally.failed += 1;
        }
        if d.out != r.lines {
            let at = d
                .out
                .iter()
                .zip(&r.lines)
                .position(|(x, y)| x != y)
                .unwrap_or(d.out.len().min(r.lines.len()));
            tally.mismatches.push(format!(
                "{} line {}: rust {:?} java {:?}",
                r.label,
                at,
                d.out.get(at),
                r.lines.get(at)
            ));
        } else {
            check_decode_agrees(&a, &r.lines);
        }
    }
}

fn golden_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(data_dir())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "golden"))
        .filter(|p| p.file_name().unwrap() != "flags.golden")
        .collect();
    files.sort();
    files
}

#[test]
fn kt1_golden_dumps_match_plugin() {
    let mut tally = Tally::default();
    for f in golden_files() {
        compare(&read_golden(&f), &mut tally);
    }
    assert!(tally.records > 100, "only {} records", tally.records);
    assert!(
        tally.mismatches.is_empty(),
        "{} of {} records differ:\n{}",
        tally.mismatches.len(),
        tally.records,
        tally.mismatches.join("\n")
    );
}

#[test]
fn kt1_flags_layout_matches_plugin() {
    let text = std::fs::read_to_string(data_dir().join("flags.golden")).unwrap();
    let mut java: BTreeMap<String, String> = BTreeMap::new();
    for l in text.lines() {
        java.insert(l.split(' ').next().unwrap().to_string(), l.to_string());
    }
    let mut rust: BTreeMap<String, String> = BTreeMap::new();
    for (name, f) in all_flags() {
        let (offset, width) = f.layout();
        let mask = ((1 << width) - 1) << offset;
        let mut s = format!("{} {} {}", name, offset, width);
        for v in 0..(1 << width) {
            s.push_str(&format!(
                " {}/{}",
                f.text(v << offset),
                f.text((v << offset) | !mask)
            ));
        }
        rust.insert(name.to_string(), s);
    }
    assert_eq!(rust, java);
}

#[test]
fn kt1_struct_class_annotation_matches_golden() {
    let golden: BTreeMap<String, GoldenRecord> = golden_files()
        .iter()
        .flat_map(|f| read_golden(f))
        .map(|r| (r.label.clone(), r))
        .collect();
    let _context = common::install_default_context();
    let dir = data_dir().join("classes");
    let mut checked = 0;
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        let name = path.file_name().unwrap().to_str().unwrap();
        let label = format!("fxutf8!pkg/{}", name);
        let bytes = std::fs::read(&path).unwrap();
        let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
        let a = KotlinMetadataAnnotation::find(cl).expect("kotlin/Metadata");
        let expected = &golden
            .get(&label)
            .unwrap_or_else(|| panic!("{} not in golden", label))
            .lines;
        let raw: Vec<&String> = expected.iter().filter(|l| l.starts_with("raw ")).collect();
        let got = raw_lines(a.get_record());
        assert_eq!(got.iter().collect::<Vec<_>>(), raw, "{}", label);
        let mut d = Dumper::new();
        d.run(&label, &a);
        assert_eq!(&d.out, expected, "{}", label);
        checked += 1;
    }
    assert!(checked >= 4);
}

/// Opt-in: `KT1_SWEEP_DIR` holds full dumps (`gen.sh sweep`); with
/// `KT1_SWEEP_JARS` the same classes are also read through `StructClass`.
#[test]
fn kt1_sweep() {
    let Ok(dir) = std::env::var("KT1_SWEEP_DIR") else {
        return;
    };
    let jars: BTreeMap<String, String> = std::env::var("KT1_SWEEP_JARS")
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (l, p) = s.split_once('=').unwrap();
            (l.to_string(), p.to_string())
        })
        .collect();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "txt"))
        .collect();
    files.sort();
    let _context = common::install_default_context();
    let mut report = Vec::new();
    let mut total_mismatches = 0;
    for f in files {
        let records = read_golden(&f);
        let mut tally = Tally::default();
        compare(&records, &mut tally);
        let mut via_struct = 0;
        let mut struct_mismatches = Vec::new();
        let label = f.file_stem().unwrap().to_str().unwrap().to_string();
        if let Some(jar) = jars.get(&label) {
            let zip = ZipReader::open(Path::new(jar)).unwrap();
            for r in &records {
                let entry = r.label.split_once('!').unwrap().1;
                let bytes = zip.read(entry).unwrap().unwrap();
                let got = try_catch(|| {
                    let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
                    raw_lines(KotlinMetadataAnnotation::find(cl).unwrap().get_record())
                });
                let want: Vec<String> = r
                    .lines
                    .iter()
                    .filter(|l| l.starts_with("raw "))
                    .cloned()
                    .collect();
                match got {
                    Ok(got) if got == want => via_struct += 1,
                    Ok(_) => struct_mismatches.push(r.label.clone()),
                    Err(c) => struct_mismatches.push(format!("{}: {}", r.label, c.message)),
                }
            }
        }
        report.push(format!(
            "{}: records {} decoded {} failed {} fields {} names {} mismatches {} | via StructClass {} mismatches {}",
            label,
            tally.records,
            tally.decoded,
            tally.failed,
            tally.fields,
            tally.names,
            tally.mismatches.len(),
            via_struct,
            struct_mismatches.len()
        ));
        for m in tally.mismatches.iter().chain(&struct_mismatches).take(10) {
            report.push(format!("  {}", m));
        }
        total_mismatches += tally.mismatches.len() + struct_mismatches.len();
    }
    println!("{}", report.join("\n"));
    assert_eq!(total_mismatches, 0, "{}", report.join("\n"));
}
