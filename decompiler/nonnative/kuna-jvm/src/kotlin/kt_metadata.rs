//! The pure part of `org/vineflower/kotlin/KotlinChooser.parseMetadataFor`:
//! finding the `kotlin/Metadata` annotation, reading its values with the
//! chooser's casts, and decoding `d1`/`d2` into the name resolver and the
//! message `k` selects. Attaching `KElement`s to members is the caller's part.

use std::rc::Rc;

use crate::bit_encoding::BitEncoding;
use crate::kt_proto::{ByteArrayInputStream, Class, Function, Package, StringTableTypes};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::primitive_constant::ConstValue;
use crate::rt::throw;
use crate::struct_annotation_attribute::{AnnotationElement, AnnotationRecord};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;

const CONST_EXPRENT: &str = "org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent";
const NEW_EXPRENT: &str = "org.jetbrains.java.decompiler.modules.decompiler.exps.NewExprent";

fn class_cast(from: &str, to: &str) -> ! {
    throw(
        "ClassCastException",
        format!("class {} cannot be cast to class {}", from, to),
    )
}

/// The Java class of the exprent an annotation element becomes.
fn exprent_class(element: &AnnotationElement) -> &'static str {
    match element {
        AnnotationElement::Enum { .. } => {
            "org.jetbrains.java.decompiler.modules.decompiler.exps.FieldExprent"
        }
        AnnotationElement::Class { .. } | AnnotationElement::Const { .. } => CONST_EXPRENT,
        AnnotationElement::Array { .. } => NEW_EXPRENT,
        AnnotationElement::Annotation(_) => {
            "org.jetbrains.java.decompiler.modules.decompiler.exps.AnnotationExprent"
        }
    }
}

fn boxed_class(value: &ConstValue) -> &'static str {
    match value {
        ConstValue::Integer(_) => "java.lang.Integer",
        ConstValue::Long(_) => "java.lang.Long",
        ConstValue::Float(_) => "java.lang.Float",
        ConstValue::Double(_) => "java.lang.Double",
        ConstValue::String(_) => "java.lang.String",
    }
}

/// `((ConstExprent) element).getValue()` as a `String` (a class literal's value is one).
fn const_string(element: &AnnotationElement) -> String {
    match element {
        AnnotationElement::Class { value, .. } => value.clone(),
        AnnotationElement::Const { cn, .. } => match &cn.value {
            Some(ConstValue::String(s)) => s.to_string(),
            Some(other) => class_cast(boxed_class(other), "java.lang.String"),
            None => throw("NullPointerException", "constant value is null"),
        },
        other => class_cast(exprent_class(other), CONST_EXPRENT),
    }
}

/// The `kotlin/Metadata` annotation of a class, as the chooser sees it.
#[derive(Clone)]
pub struct KotlinMetadataAnnotation {
    record: AnnotationRecord,
}

impl KotlinMetadataAnnotation {
    pub const CLASS_NAME: &'static str = "kotlin/Metadata";

    /// The first `kotlin/Metadata` among the runtime-visible, then the
    /// runtime-invisible annotations (`KotlinChooser.ANNOTATION_ATTRIBUTES`).
    pub fn find(cl: StructClass) -> Option<KotlinMetadataAnnotation> {
        for key in [
            StructGeneralAttribute::ATTRIBUTE_RUNTIME_VISIBLE_ANNOTATIONS,
            StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS,
        ] {
            if cl.has_attribute(key) {
                let attr = cl.get_attribute(key).expect("attribute present");
                for record in attr.get_records() {
                    if record.class_name.as_deref() == Some(Self::CLASS_NAME) {
                        return Some(KotlinMetadataAnnotation { record });
                    }
                }
            }
        }
        None
    }

    pub fn new(record: AnnotationRecord) -> KotlinMetadataAnnotation {
        KotlinMetadataAnnotation { record }
    }

    pub fn get_record(&self) -> &AnnotationRecord {
        &self.record
    }

    /// `getParNames()`.
    pub fn get_par_names(&self) -> &[String] {
        &self.record.names
    }

    /// `getParNames().indexOf(name)`.
    pub fn index_of(&self, name: &str) -> i32 {
        self.record
            .names
            .iter()
            .position(|n| n == name)
            .map_or(-1, |i| i as i32)
    }

    /// `getParValues().get(index)`.
    pub fn get_par_value(&self, index: i32) -> &AnnotationElement {
        &self.record.values[index as usize]
    }

    fn named(&self, name: &str) -> Option<&AnnotationElement> {
        match self.index_of(name) {
            -1 => None,
            i => Some(self.get_par_value(i)),
        }
    }

    /// `(int) ((ConstExprent) element).getValue()`.
    pub fn int_value(element: &AnnotationElement) -> i32 {
        match element {
            AnnotationElement::Const { cn, .. } => match &cn.value {
                Some(ConstValue::Integer(v)) => *v,
                Some(other) => class_cast(boxed_class(other), "java.lang.Integer"),
                None => throw("NullPointerException", "constant value is null"),
            },
            AnnotationElement::Class { .. } => class_cast("java.lang.String", "java.lang.Integer"),
            other => class_cast(exprent_class(other), CONST_EXPRENT),
        }
    }

    /// `(String) ((ConstExprent) element).getValue()`.
    pub fn string_value(element: &AnnotationElement) -> String {
        const_string(element)
    }

    /// `KotlinChooser.getDataFromExpr((NewExprent) element)`.
    pub fn data_from_expr(element: &AnnotationElement) -> Vec<String> {
        match element {
            AnnotationElement::Array { elements } => elements.iter().map(const_string).collect(),
            other => class_cast(exprent_class(other), NEW_EXPRENT),
        }
    }

    /// The integers of an `int[]` value such as `mv`.
    pub fn int_array_value(element: &AnnotationElement) -> Vec<i32> {
        match element {
            AnnotationElement::Array { elements } => elements
                .iter()
                .map(KotlinMetadataAnnotation::int_value)
                .collect(),
            other => class_cast(exprent_class(other), NEW_EXPRENT),
        }
    }

    /// `k`: the kind; `None` when absent.
    pub fn get_k(&self) -> Option<i32> {
        self.named("k").map(KotlinMetadataAnnotation::int_value)
    }

    /// `d1`: the encoded proto.
    pub fn get_d1(&self) -> Option<Vec<String>> {
        self.named("d1")
            .map(KotlinMetadataAnnotation::data_from_expr)
    }

    /// `d2`: the strings.
    pub fn get_d2(&self) -> Option<Vec<String>> {
        self.named("d2")
            .map(KotlinMetadataAnnotation::data_from_expr)
    }

    /// `xs`: the extra string (a multifile part's facade name).
    pub fn get_xs(&self) -> Option<String> {
        self.named("xs").map(KotlinMetadataAnnotation::string_value)
    }

    /// `mv`: the metadata version (not read by the plugin).
    pub fn get_mv(&self) -> Option<Vec<i32>> {
        self.named("mv")
            .map(KotlinMetadataAnnotation::int_array_value)
    }

    /// `xi`: the extra flags (not read by the plugin).
    pub fn get_xi(&self) -> Option<i32> {
        self.named("xi").map(KotlinMetadataAnnotation::int_value)
    }
}

/// What `k` makes of the decoded bytes.
#[derive(Clone)]
pub enum KotlinMetadataMessage {
    /// `k = 1`.
    Class(Class),
    /// `k = 2`: a file facade.
    File(Package),
    /// `k = 3`: a synthetic class (lambda).
    SyntheticClass(Function),
    /// `k = 4`: a multifile facade; its parts are the `d1` strings.
    MultifileFacade(Vec<String>),
    /// `k = 5`: a multifile part, with `xs` when present.
    MultifilePart {
        package: Package,
        xs: Option<String>,
    },
    /// Any other `k`.
    None,
}

#[derive(Clone)]
pub struct DecodedMetadata {
    /// `null` in Java when `d2` is absent.
    pub resolver: Option<MetadataNameResolver>,
    pub message: KotlinMetadataMessage,
}

pub struct KtMetadata;

impl KtMetadata {
    /// The decode steps of `parseMetadataFor`'s `try` block, given `k`, `d1`
    /// and `d2` already cast: `BitEncoding.decodeBytes(d1)`, then (with `d2`)
    /// `StringTableTypes.parseDelimitedFrom` and the resolver, then the
    /// message for `k` from the rest of the same stream. Throws what Java throws.
    pub fn decode(
        k: i32,
        d1: &[String],
        d2: Option<&[String]>,
        xs: Option<&str>,
    ) -> DecodedMetadata {
        let mut input = KtMetadata::open(d1);
        let resolver = d2.map(|strings| KtMetadata::read_resolver(&mut input, strings));
        let message = KtMetadata::read_message(&mut input, k, d1, xs);
        DecodedMetadata { resolver, message }
    }

    /// `new ByteArrayInputStream(BitEncoding.decodeBytes(d1))`.
    pub fn open(d1: &[String]) -> ByteArrayInputStream {
        let buf: Rc<[u8]> = Rc::from(BitEncoding::decode_bytes(d1));
        ByteArrayInputStream::new(buf)
    }

    /// `new MetadataNameResolver(StringTableTypes.parseDelimitedFrom(input), d2)`.
    pub fn read_resolver(input: &mut ByteArrayInputStream, d2: &[String]) -> MetadataNameResolver {
        let types = StringTableTypes::parse_delimited_from(input).unwrap_or_else(|| {
            throw(
                "NullPointerException",
                "Cannot invoke \"org.vineflower.kt.metadata.jvm.JvmProtoBuf$StringTableTypes.getRecordList()\" because \"types\" is null",
            )
        });
        MetadataNameResolver::new(types, d2.to_vec())
    }

    /// The `k` dispatch over the rest of the stream.
    pub fn read_message(
        input: &mut ByteArrayInputStream,
        k: i32,
        d1: &[String],
        xs: Option<&str>,
    ) -> KotlinMetadataMessage {
        match k {
            1 => KotlinMetadataMessage::Class(Class::parse_from(input)),
            2 => KotlinMetadataMessage::File(Package::parse_from(input)),
            3 => KotlinMetadataMessage::SyntheticClass(Function::parse_from(input)),
            4 => KotlinMetadataMessage::MultifileFacade(d1.to_vec()),
            5 => KotlinMetadataMessage::MultifilePart {
                package: Package::parse_from(input),
                xs: xs.map(str::to_string),
            },
            _ => KotlinMetadataMessage::None,
        }
    }

    /// A multifile part's facade name: `xs`, else `qualifiedName.split("__")[0]`.
    pub fn multifile_name(xs: Option<&str>, qualified_name: &str) -> String {
        if let Some(xs) = xs {
            return xs.to_string();
        }
        if qualified_name.is_empty() {
            return String::new();
        }
        let mut parts: Vec<&str> = qualified_name.split("__").collect();
        while parts.last() == Some(&"") {
            parts.pop();
        }
        match parts.first() {
            Some(first) => first.to_string(),
            None => throw(
                "ArrayIndexOutOfBoundsException",
                "Index 0 out of bounds for length 0",
            ),
        }
    }
}
