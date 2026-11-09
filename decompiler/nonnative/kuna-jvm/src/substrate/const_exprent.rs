//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ConstExprent.java`.

use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentKind};
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::generic_type::GenericType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::jstring::{from_utf16, java_string_hash};
use crate::rt::{BitSet, JHashMap, JHashSet, JavaEq, JavaHash, JavaStr};
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;
use crate::var_type::VarType;
use crate::{java_subclass, jthrow, unported};

/// The `Object value` of a constant: one of the boxed types the class file can
/// hold (`Integer`, `Long`, `Float`, `Double`, `String`); `null` is `None`.
#[derive(Clone, Debug)]
pub enum ConstValue {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
}

impl ConstValue {
    /// `value.toString()`.
    pub fn java_to_string(&self) -> String {
        match self {
            ConstValue::Int(v) => v.to_string(),
            ConstValue::Long(v) => v.to_string(),
            ConstValue::Float(v) => crate::rt::jnum::float_to_string(*v),
            ConstValue::Double(v) => crate::rt::jnum::double_to_string(*v),
            ConstValue::String(s) => s.clone(),
        }
    }

    fn class_name(&self) -> &'static str {
        match self {
            ConstValue::Int(_) => "java.lang.Integer",
            ConstValue::Long(_) => "java.lang.Long",
            ConstValue::Float(_) => "java.lang.Float",
            ConstValue::Double(_) => "java.lang.Double",
            ConstValue::String(_) => "java.lang.String",
        }
    }

    fn cast_failure(&self, to: &str) -> ! {
        jthrow!(ClassCastException; "class {} cannot be cast to class {}", self.class_name(), to)
    }

    /// `(Integer) value`.
    pub fn as_int(&self) -> i32 {
        match self {
            ConstValue::Int(v) => *v,
            _ => self.cast_failure("java.lang.Integer"),
        }
    }

    /// `(Long) value`.
    pub fn as_long(&self) -> i64 {
        match self {
            ConstValue::Long(v) => *v,
            _ => self.cast_failure("java.lang.Long"),
        }
    }

    /// `(Float) value`.
    pub fn as_float(&self) -> f32 {
        match self {
            ConstValue::Float(v) => *v,
            _ => self.cast_failure("java.lang.Float"),
        }
    }

    /// `(Double) value`.
    pub fn as_double(&self) -> f64 {
        match self {
            ConstValue::Double(v) => *v,
            _ => self.cast_failure("java.lang.Double"),
        }
    }

    /// `(String) value`.
    pub fn as_string(&self) -> &str {
        match self {
            ConstValue::String(s) => s,
            _ => self.cast_failure("java.lang.String"),
        }
    }

    /// `((Number) value).intValue()`.
    pub fn int_value(&self) -> i32 {
        match self {
            ConstValue::Int(v) => *v,
            ConstValue::Long(v) => *v as i32,
            ConstValue::Float(v) => *v as i32,
            ConstValue::Double(v) => *v as i32,
            ConstValue::String(_) => self.cast_failure("java.lang.Number"),
        }
    }
}

fn float_to_int_bits(v: f32) -> i32 {
    if v.is_nan() {
        0x7fc0_0000
    } else {
        v.to_bits() as i32
    }
}

fn double_to_long_bits(v: f64) -> i64 {
    if v.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        v.to_bits() as i64
    }
}

impl JavaHash for ConstValue {
    fn java_hash(&self) -> i32 {
        match self {
            ConstValue::Int(v) => *v,
            ConstValue::Long(v) => v.java_hash(),
            ConstValue::Float(v) => float_to_int_bits(*v),
            ConstValue::Double(v) => double_to_long_bits(*v).java_hash(),
            ConstValue::String(s) => java_string_hash(s),
        }
    }
}

impl JavaEq for ConstValue {
    fn java_equals(&self, other: &Self) -> bool {
        match (self, other) {
            (ConstValue::Int(a), ConstValue::Int(b)) => a == b,
            (ConstValue::Long(a), ConstValue::Long(b)) => a == b,
            (ConstValue::Float(a), ConstValue::Float(b)) => {
                float_to_int_bits(*a) == float_to_int_bits(*b)
            }
            (ConstValue::Double(a), ConstValue::Double(b)) => {
                double_to_long_bits(*a) == double_to_long_bits(*b)
            }
            (ConstValue::String(a), ConstValue::String(b)) => a == b,
            _ => false,
        }
    }
}

impl From<i32> for ConstValue {
    fn from(v: i32) -> Self {
        ConstValue::Int(v)
    }
}

impl From<i64> for ConstValue {
    fn from(v: i64) -> Self {
        ConstValue::Long(v)
    }
}

impl From<f32> for ConstValue {
    fn from(v: f32) -> Self {
        ConstValue::Float(v)
    }
}

impl From<f64> for ConstValue {
    fn from(v: f64) -> Self {
        ConstValue::Double(v)
    }
}

impl From<String> for ConstValue {
    fn from(v: String) -> Self {
        ConstValue::String(v)
    }
}

impl From<&str> for ConstValue {
    fn from(v: &str) -> Self {
        ConstValue::String(v.to_string())
    }
}

/// An entry of `UNINLINED_DOUBLES` / `UNINLINED_FLOATS`: renders the constant from its bytecode set.
pub type UninlinedFn = Rc<dyn Fn(Option<&BitSet>) -> TextBuffer>;

pub struct ConstExprentData {
    pub const_type: VarType,
    pub value: Option<ConstValue>,
    pub bool_permitted: bool,
    pub was_condy: bool,
}

java_subclass!(pub ConstExprent: Exprent, ExprentKind::Const(ConstExprentData),
               is_const, as_const, try_as_const);

impl JavaEq for ConstExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for ConstExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl ConstExprent {
    /// Java `CHAR_ESCAPES.get(c)`.
    fn char_escape(c: i32) -> Option<&'static str> {
        match c {
            0x8 => Some("\\b"),
            0x9 => Some("\\t"),
            0xA => Some("\\n"),
            0xC => Some("\\f"),
            0xD => Some("\\r"),
            0x27 => Some("\\'"),
            0x5C => Some("\\\\"),
            _ => None,
        }
    }

    #[allow(dead_code, non_snake_case)]
    fn UNINLINED_DOUBLES() -> Rc<JHashMap<ConstValue, UninlinedFn>> {
        unported!("ConstExprent.UNINLINED_DOUBLES")
    }

    #[allow(dead_code, non_snake_case)]
    fn UNINLINED_FLOATS() -> Rc<JHashMap<ConstValue, UninlinedFn>> {
        unported!("ConstExprent.UNINLINED_FLOATS")
    }

    #[allow(dead_code, non_snake_case)]
    fn NO_PAREN_VALUES() -> Rc<JHashSet<ConstValue>> {
        unported!("ConstExprent.NO_PAREN_VALUES")
    }

    /// Java: `ConstExprent(int, boolean, BitSet)`.
    pub fn new_int(
        val: i32,
        bool_permitted: bool,
        bytecode_offsets: Option<&BitSet>,
    ) -> ConstExprent {
        ConstExprent::new_full(
            ConstExprent::guess_type(val, bool_permitted),
            Some(ConstValue::Int(val)),
            bool_permitted,
            bytecode_offsets,
        )
    }

    /// Java: `ConstExprent(VarType, Object, BitSet)`.
    pub fn new(
        const_type: VarType,
        value: Option<ConstValue>,
        bytecode_offsets: Option<&BitSet>,
    ) -> ConstExprent {
        ConstExprent::new_full(const_type, value, false, bytecode_offsets)
    }

    /// Java: `ConstExprent(VarType, Object, BitSet, boolean)`.
    pub fn new_condy(
        const_type: VarType,
        value: Option<ConstValue>,
        bytecode_offsets: Option<&BitSet>,
        was_condy: bool,
    ) -> ConstExprent {
        let c = ConstExprent::new_full(const_type, value, false, bytecode_offsets);
        c.d_mut(|d| d.was_condy = was_condy);
        c
    }

    /// Java: `ConstExprent(VarType, Object, boolean, BitSet)`.
    pub fn new_full(
        const_type: VarType,
        value: Option<ConstValue>,
        bool_permitted: bool,
        bytecode_offsets: Option<&BitSet>,
    ) -> ConstExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Const(ConstExprentData {
                const_type: const_type.clone(),
                value: value.clone(),
                bool_permitted,
                was_condy: false,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        ValidationHelper::assert_true(
            !const_type.ptr_eq(&VarType::VARTYPE_VOID()),
            "Must not make const type with type void",
        );

        let c = ConstExprent(e);
        if const_type.equals(&VarType::VARTYPE_CLASS()) {
            if let Some(value) = &value {
                let string_val = value.java_to_string();
                let args = vec![Some(VarType::new_signature_cl(
                    &string_val,
                    !string_val.starts_with('['),
                ))];
                let generic = GenericType::new(
                    const_type.type_,
                    const_type.array_dim,
                    const_type.value.clone(),
                    None,
                    args,
                    GenericType::WILDCARD_NO,
                );
                c.d_mut(|d| d.const_type = generic.0);
            }
        }
        c
    }

    fn guess_type(val: i32, bool_permitted: bool) -> VarType {
        if bool_permitted && (val == 0 || val == 1) {
            VarType::VARTYPE_BOOLEAN()
        } else if (0..=127).contains(&val) {
            VarType::VARTYPE_BYTECHAR()
        } else if (-128..=127).contains(&val) {
            VarType::VARTYPE_BYTE()
        } else if (0..=32767).contains(&val) {
            VarType::VARTYPE_SHORTCHAR()
        } else if (-32768..=32767).contains(&val) {
            VarType::VARTYPE_SHORT()
        } else if (0..=0xFFFF).contains(&val) {
            VarType::VARTYPE_CHAR()
        } else {
            VarType::VARTYPE_INT()
        }
    }

    #[allow(dead_code)]
    fn gcd(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            a
        } else {
            ConstExprent::gcd(b, a % b)
        }
    }

    pub fn copy(self) -> Exprent {
        let (const_type, value, was_condy) =
            self.d(|d| (d.const_type.clone(), d.value.clone(), d.was_condy));
        let bytecode = self.get_bytecode();
        ConstExprent::new_condy(const_type, value, bytecode.as_ref(), was_condy).0
    }

    pub fn get_expr_type(self) -> VarType {
        self.d(|d| d.const_type.clone())
    }

    pub fn get_exprent_use(self) -> i32 {
        Exprent::MULTIPLE_USES | Exprent::SIDE_EFFECTS_FREE
    }

    pub fn get_all_exprents_list(self, _list: &mut Vec<Exprent>) {}

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("ConstExprent.toJava")
    }

    pub fn get_precedence(self) -> i32 {
        unported!("ConstExprent.getPrecedence")
    }

    #[allow(dead_code)]
    fn get_pi_double(bytecode: Option<&BitSet>) -> TextBuffer {
        ConstExprent::get_double(bytecode, "PI", "java/lang/Math")
    }

    #[allow(dead_code)]
    fn get_double(bytecode: Option<&BitSet>, name: &str, class_name: &str) -> TextBuffer {
        FieldExprent::new(
            name.to_string(),
            class_name.to_string(),
            true,
            None,
            FieldDescriptor::DOUBLE_DESCRIPTOR(),
            bytecode,
        )
        .to_java(0)
    }

    #[allow(dead_code)]
    fn get_float(bytecode: Option<&BitSet>, name: &str, class_name: &str) -> TextBuffer {
        FieldExprent::new(
            name.to_string(),
            class_name.to_string(),
            true,
            None,
            FieldDescriptor::FLOAT_DESCRIPTOR(),
            bytecode,
        )
        .to_java(0)
    }

    #[allow(dead_code)]
    fn trim_float(_value: &str, _start: f32) -> String {
        unported!("ConstExprent.trimFloat")
    }

    #[allow(dead_code)]
    fn trim_double(_value: &str, _start: f64) -> String {
        unported!("ConstExprent.trimDouble")
    }

    pub fn is_null(self) -> bool {
        self.d(|d| d.const_type.type_ == CodeType::Null)
    }

    pub fn convert_string_to_java(value: &str, ascii: bool) -> String {
        let arr = value.utf16();
        let mut buffer: Vec<u16> = Vec::with_capacity(arr.len());
        let push_str = |buffer: &mut Vec<u16>, s: &str| buffer.extend(s.encode_utf16());

        for c in arr {
            match c {
                0x5C => push_str(&mut buffer, "\\\\"),
                0x8 => push_str(&mut buffer, "\\b"),
                0x9 => push_str(&mut buffer, "\\t"),
                0xA => push_str(&mut buffer, "\\n"),
                0xC => push_str(&mut buffer, "\\f"),
                0xD => push_str(&mut buffer, "\\r"),
                0x22 => push_str(&mut buffer, "\\\""),
                _ => {
                    if ConstExprent::is_printable_ascii(c as i32)
                        || !ascii && TextUtil::is_printable_unicode(c)
                    {
                        buffer.push(c);
                    } else {
                        push_str(&mut buffer, &TextUtil::char_to_unicode_literal(c as i32));
                    }
                }
            }
        }

        from_utf16(&buffer)
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(cn) = o.try_as_const() else {
            return false;
        };
        let (const_type, value) = self.d(|d| (d.const_type.clone(), d.value.clone()));
        const_type.equals(&cn.get_const_type()) && value.java_equals(&cn.get_value())
    }

    pub fn hash_code(self) -> i32 {
        self.d(|d| {
            let result = d.const_type.java_hash();
            result
                .wrapping_mul(31)
                .wrapping_add(d.value.as_ref().map_or(0, |v| v.java_hash()))
        })
    }

    fn value_or_npe(&self) -> ConstValue {
        match self.d(|d| d.value.clone()) {
            Some(v) => v,
            None => jthrow!(NullPointerException; "const value is null"),
        }
    }

    pub fn has_boolean_value(self) -> bool {
        match self.d(|d| d.const_type.type_) {
            CodeType::Boolean
            | CodeType::Char
            | CodeType::Byte
            | CodeType::Bytechar
            | CodeType::Short
            | CodeType::Shortchar
            | CodeType::Int => {
                let value = self.value_or_npe().as_int();
                value == 0
                    || (DecompilerContext::get_option(IFernflowerPreferences::BOOLEAN_TRUE_ONE)
                        && value == 1)
            }
            _ => false,
        }
    }

    pub fn has_value_one(self) -> bool {
        match self.d(|d| d.const_type.type_) {
            CodeType::Boolean
            | CodeType::Char
            | CodeType::Byte
            | CodeType::Bytechar
            | CodeType::Short
            | CodeType::Shortchar
            | CodeType::Int => self.value_or_npe().int_value() == 1,
            CodeType::Long => self.value_or_npe().as_long() == 1,
            CodeType::Double => self.value_or_npe().as_double() == 1.0,
            CodeType::Float => self.value_or_npe().as_float() == 1.0,
            _ => false,
        }
    }

    pub fn get_zero_constant(type_: CodeType) -> ConstExprent {
        match type_ {
            CodeType::Int => {
                ConstExprent::new(VarType::VARTYPE_INT(), Some(ConstValue::Int(0)), None)
            }
            CodeType::Long => {
                ConstExprent::new(VarType::VARTYPE_LONG(), Some(ConstValue::Long(0)), None)
            }
            CodeType::Double => ConstExprent::new(
                VarType::VARTYPE_DOUBLE(),
                Some(ConstValue::Double(0.0)),
                None,
            ),
            CodeType::Float => {
                ConstExprent::new(VarType::VARTYPE_FLOAT(), Some(ConstValue::Float(0.0)), None)
            }
            _ => jthrow!("Invalid argument: {}", format!("{type_:?}").to_uppercase()),
        }
    }

    pub fn get_const_type(self) -> VarType {
        self.d(|d| d.const_type.clone())
    }

    pub fn set_const_type(self, const_type: VarType) {
        self.d_mut(|d| d.const_type = const_type)
    }

    pub fn adjust_const_type(self, expected_type: &VarType) {
        let const_type = self.get_const_type();
        if (expected_type.equals(&VarType::VARTYPE_CHAR())
            || expected_type.equals(&VarType::VARTYPE_CHARACTER()))
            && (const_type.equals(&VarType::VARTYPE_BYTECHAR())
                || const_type.equals(&VarType::VARTYPE_SHORTCHAR()))
        {
            let int_value = self.get_int_value();
            if ConstExprent::is_printable_ascii(int_value)
                || ConstExprent::char_escape(int_value).is_some()
            {
                self.set_const_type(VarType::VARTYPE_CHAR());
            }
        } else if (expected_type.equals(&VarType::VARTYPE_INT())
            || expected_type.equals(&VarType::VARTYPE_INTEGER()))
            && const_type.type_family == TypeFamily::Integer
        {
            self.set_const_type(VarType::VARTYPE_INT());
        }
    }

    fn is_printable_ascii(c: i32) -> bool {
        (32..127).contains(&c)
    }

    pub fn get_value(self) -> Option<ConstValue> {
        self.d(|d| d.value.clone())
    }

    pub fn get_int_value(self) -> i32 {
        self.value_or_npe().as_int()
    }

    pub fn is_bool_permitted(self) -> bool {
        self.d(|d| d.bool_permitted)
    }

    pub fn get_bytecode_range(self, values: Option<&mut BitSet>) {
        self.measure_bytecode(values);
    }

    pub fn set_was_condy(self, was_condy: bool) -> ConstExprent {
        self.d_mut(|d| d.was_condy = was_condy);
        self
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        unported!("ConstExprent.toString")
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, _match_node: &MatchNode, _engine: &mut MatchEngine) -> bool {
        unported!("ConstExprent.match")
    }
}
