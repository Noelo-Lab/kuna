//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ConstExprent.java`.
//!
//! kuna fix `double_constant_literal` (spec 10.16): a `double` constant is
//! never written as a shorter `float` literal, whose type the context keeps.

use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentKind};
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::generic_type::GenericType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_matchable::MatchProperties;
use crate::kuna_fixes::{self, KunaFix};
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::rt::jstring::{from_utf16, java_string_hash};
use crate::rt::{jnum, BitSet, JHashMap, JHashSet, JavaEq, JavaHash, JavaStr};
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;
use crate::var_type::VarType;
use crate::{java_subclass, jthrow};

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

/// `String.valueOf(value)`.
fn value_text(value: &Option<ConstValue>) -> String {
    match value {
        Some(v) => v.java_to_string(),
        None => "null".to_string(),
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

/// A pool constant's `value` as the `Object` a `ConstExprent` holds.
impl From<&crate::primitive_constant::ConstValue> for ConstValue {
    fn from(v: &crate::primitive_constant::ConstValue) -> Self {
        use crate::primitive_constant::ConstValue as Pool;
        match v {
            Pool::Integer(i) => ConstValue::Int(*i),
            Pool::Long(l) => ConstValue::Long(*l),
            Pool::Float(f) => ConstValue::Float(*f),
            Pool::Double(d) => ConstValue::Double(*d),
            Pool::String(s) => ConstValue::String(s.to_string()),
        }
    }
}

impl From<crate::primitive_constant::ConstValue> for ConstValue {
    fn from(v: crate::primitive_constant::ConstValue) -> Self {
        ConstValue::from(&v)
    }
}

/// The inverse of the pool conversion, for callers that hand a constant's
/// value back to the struct layer (`MatchNode` rule values, `PrimitiveConstant`).
impl From<&ConstValue> for crate::primitive_constant::ConstValue {
    fn from(v: &ConstValue) -> Self {
        use crate::primitive_constant::ConstValue as Pool;
        match v {
            ConstValue::Int(i) => Pool::Integer(*i),
            ConstValue::Long(l) => Pool::Long(*l),
            ConstValue::Float(f) => Pool::Float(*f),
            ConstValue::Double(d) => Pool::Double(*d),
            ConstValue::String(s) => Pool::String(Rc::from(s.as_str())),
        }
    }
}

/// An entry of `UNINLINED_DOUBLES` / `UNINLINED_FLOATS`: renders the constant from its bytecode set.
pub type UninlinedFn = Rc<dyn Fn(Option<&BitSet>) -> TextBuffer>;

/// The tables of Java's static initializer, built in its exact put order
/// (later puts of an equal key replace earlier ones).
struct ConstTables {
    uninlined_doubles: Rc<JHashMap<ConstValue, UninlinedFn>>,
    uninlined_floats: Rc<JHashMap<ConstValue, UninlinedFn>>,
    no_paren_values: Rc<JHashSet<ConstValue>>,
}

thread_local! {
    static TABLES: ConstTables = ConstTables::build();
}

fn uninlined(f: impl Fn(Option<&BitSet>) -> TextBuffer + 'static) -> UninlinedFn {
    Rc::new(f)
}

fn with_suffix(mut buf: TextBuffer, suffix: &str) -> TextBuffer {
    buf.append(suffix);
    buf
}

fn with_prefix(mut buf: TextBuffer, prefix: &str) -> TextBuffer {
    buf.prepend(prefix);
    buf
}

impl ConstTables {
    fn build() -> ConstTables {
        use ConstValue::{Double as D, Float as F};
        let pi = std::f64::consts::PI;
        let e = std::f64::consts::E;
        let mut doubles: JHashMap<ConstValue, UninlinedFn> = JHashMap::new();
        let mut floats: JHashMap<ConstValue, UninlinedFn> = JHashMap::new();
        let mut no_paren: JHashSet<ConstValue> = JHashSet::new();

        doubles.put(
            D(e),
            uninlined(|b| ConstExprent::get_double(b, "E", "java/lang/Math")),
        );
        doubles.put(
            D(-e),
            uninlined(|b| with_prefix(ConstExprent::get_double(b, "E", "java/lang/Math"), "-")),
        );

        doubles.put(D(pi), uninlined(ConstExprent::get_pi_double));
        doubles.put(
            D(-pi),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "-")),
        );

        no_paren.add_all(doubles.key_vec());
        for d in doubles.key_vec() {
            no_paren.add(F(d.as_double() as f32));
        }

        for i in 2..=20 {
            doubles.put(
                D(pi / i as f64),
                uninlined(move |b| with_suffix(ConstExprent::get_pi_double(b), &format!(" / {i}"))),
            );
            doubles.put(
                D(-pi / i as f64),
                uninlined(move |b| {
                    with_prefix(
                        with_suffix(ConstExprent::get_pi_double(b), &format!(" / {i}")),
                        "-",
                    )
                }),
            );
        }

        for i in 2..=20 {
            doubles.put(
                D(pi * i as f64),
                uninlined(move |b| with_suffix(ConstExprent::get_pi_double(b), &format!(" * {i}"))),
            );
            doubles.put(
                D(-pi * i as f64),
                uninlined(move |b| {
                    with_prefix(
                        with_suffix(ConstExprent::get_pi_double(b), &format!(" * {i}")),
                        "-",
                    )
                }),
            );
        }

        let mut numerator = 2.0f64;
        while numerator < 13.0 {
            let mut denominator = 2.0f64;
            while denominator < 13.0 {
                let gcd = ConstExprent::gcd(numerator, denominator);
                if gcd == 1.0 {
                    let n = jnum::double_to_string(numerator);
                    let d = jnum::double_to_string(denominator);
                    let ratio = format!(" * {n} / {d}");
                    let ratio_neg = ratio.clone();
                    doubles.put(
                        D(pi * (numerator / denominator)),
                        uninlined(move |b| with_suffix(ConstExprent::get_pi_double(b), &ratio)),
                    );
                    doubles.put(
                        D(-pi * (numerator / denominator)),
                        uninlined(move |b| {
                            with_prefix(
                                with_suffix(ConstExprent::get_pi_double(b), &ratio_neg),
                                "-",
                            )
                        }),
                    );

                    if (pi as f32) * (numerator as f32) / (denominator as f32)
                        != (pi * numerator / denominator) as f32
                    {
                        let ratio_f = format!(" * {n}F / {d}F");
                        let ratio_f_neg = ratio_f.clone();
                        floats.put(
                            F((pi as f32) * ((numerator as f32) / (denominator as f32))),
                            uninlined(move |b| {
                                with_prefix(
                                    with_suffix(ConstExprent::get_pi_double(b), &ratio_f),
                                    "(float) ",
                                )
                            }),
                        );
                        floats.put(
                            F((-pi as f32) * ((numerator as f32) / (denominator as f32))),
                            uninlined(move |b| {
                                with_prefix(
                                    with_suffix(ConstExprent::get_pi_double(b), &ratio_f_neg),
                                    "(float) -",
                                )
                            }),
                        );
                    }
                }
                denominator += 1.0;
            }
            numerator += 1.0;
        }

        doubles.put(
            D(180.0 / pi),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "180.0 / ")),
        );
        doubles.put(
            D(-180.0 / pi),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "-180.0 / ")),
        );

        floats.put(
            F((180.0f32 as f64 / pi) as f32),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "180.0F / ")),
        );
        floats.put(
            F((-180.0f32 as f64 / pi) as f32),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "-180.0F / ")),
        );

        floats.put(
            F(180.0f32 / pi as f32),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "180.0F / (float)")),
        );
        floats.put(
            F(-180.0f32 / pi as f32),
            uninlined(|b| with_prefix(ConstExprent::get_pi_double(b), "-180.0F / (float)")),
        );

        doubles.put(
            D(pi / 180.0),
            uninlined(|b| with_suffix(ConstExprent::get_pi_double(b), " / 180.0")),
        );
        doubles.put(
            D(-pi / 180.0),
            uninlined(|b| {
                with_prefix(with_suffix(ConstExprent::get_pi_double(b), " / 180.0"), "-")
            }),
        );

        floats.put(
            F((pi / 180.0) as f32),
            uninlined(|b| {
                with_prefix(
                    with_suffix(ConstExprent::get_pi_double(b), " / 180.0"),
                    "(float) ",
                )
            }),
        );
        floats.put(
            F((-pi / 180.0) as f32),
            uninlined(|b| {
                with_prefix(
                    with_prefix(with_suffix(ConstExprent::get_pi_double(b), " / 180.0"), "-"),
                    "(float) ",
                )
            }),
        );

        for (key, value_function) in doubles.entry_vec() {
            floats.put(
                F(key.as_double() as f32),
                uninlined(move |b| {
                    let mut double_value = value_function(b);
                    if double_value.count(" ", 0) > 0 {
                        double_value.enclose_with_parens();
                    }
                    with_prefix(double_value, "(float) ")
                }),
            );
        }

        let get_double = |name: &'static str| {
            uninlined(move |b| ConstExprent::get_double(b, name, "java/lang/Double"))
        };
        let get_double_neg = |name: &'static str| {
            uninlined(move |b| {
                with_prefix(ConstExprent::get_double(b, name, "java/lang/Double"), "-")
            })
        };
        doubles.put(D(f64::INFINITY), get_double("POSITIVE_INFINITY"));
        doubles.put(D(f64::NEG_INFINITY), get_double("NEGATIVE_INFINITY"));
        doubles.put(D(jnum::DOUBLE_MAX_VALUE), get_double("MAX_VALUE"));
        doubles.put(D(jnum::DOUBLE_MIN_NORMAL), get_double("MIN_NORMAL"));
        doubles.put(D(jnum::DOUBLE_MIN_VALUE), get_double("MIN_VALUE"));
        doubles.put(D(-jnum::DOUBLE_MAX_VALUE), get_double_neg("MAX_VALUE"));
        doubles.put(D(-jnum::DOUBLE_MIN_NORMAL), get_double_neg("MIN_NORMAL"));
        doubles.put(D(-jnum::DOUBLE_MIN_VALUE), get_double_neg("MIN_VALUE"));

        let get_float = |name: &'static str| {
            uninlined(move |b| ConstExprent::get_float(b, name, "java/lang/Float"))
        };
        let get_float_neg = |name: &'static str| {
            uninlined(move |b| {
                with_prefix(ConstExprent::get_float(b, name, "java/lang/Float"), "-")
            })
        };
        floats.put(F(f32::INFINITY), get_float("POSITIVE_INFINITY"));
        floats.put(F(f32::NEG_INFINITY), get_float("NEGATIVE_INFINITY"));
        floats.put(F(jnum::FLOAT_MAX_VALUE), get_float("MAX_VALUE"));
        floats.put(F(jnum::FLOAT_MIN_NORMAL), get_float("MIN_NORMAL"));
        floats.put(F(jnum::FLOAT_MIN_VALUE), get_float("MIN_VALUE"));
        floats.put(F(-jnum::FLOAT_MAX_VALUE), get_float_neg("MAX_VALUE"));
        floats.put(F(-jnum::FLOAT_MIN_NORMAL), get_float_neg("MIN_NORMAL"));
        floats.put(F(-jnum::FLOAT_MIN_VALUE), get_float_neg("MIN_VALUE"));

        ConstTables {
            uninlined_doubles: Rc::new(doubles),
            uninlined_floats: Rc::new(floats),
            no_paren_values: Rc::new(no_paren),
        }
    }
}

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

    /// Java `UNINLINED_DOUBLES` (keys are `ConstValue::Double`).
    #[allow(non_snake_case)]
    pub fn UNINLINED_DOUBLES() -> Rc<JHashMap<ConstValue, UninlinedFn>> {
        TABLES.with(|t| t.uninlined_doubles.clone())
    }

    /// Java `UNINLINED_FLOATS` (keys are `ConstValue::Float`).
    #[allow(non_snake_case)]
    pub fn UNINLINED_FLOATS() -> Rc<JHashMap<ConstValue, UninlinedFn>> {
        TABLES.with(|t| t.uninlined_floats.clone())
    }

    /// Java `NO_PAREN_VALUES` (a `Set<Object>` of `Double`s and `Float`s).
    #[allow(non_snake_case)]
    pub fn NO_PAREN_VALUES() -> Rc<JHashSet<ConstValue>> {
        TABLES.with(|t| t.no_paren_values.clone())
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
        let literal = DecompilerContext::get_option(IFernflowerPreferences::LITERALS_AS_IS);
        let ascii = DecompilerContext::get_option(IFernflowerPreferences::ASCII_STRING_CHARACTERS);

        let (const_type, value, was_condy) =
            self.d(|d| (d.const_type.clone(), d.value.clone(), d.was_condy));
        let bytecode = self.get_bytecode();
        let mut buf = TextBuffer::new();
        buf.add_bytecode_mapping(bytecode.as_ref());

        if was_condy {
            buf.append("/* $VF: constant dynamic */ ");
        }

        if const_type.type_ != CodeType::Null && value.is_none() {
            buf.append_cast_type_name(&const_type);
            return buf;
        }

        let val = || match &value {
            Some(v) => v.clone(),
            None => jthrow!(NullPointerException; "const value is null"),
        };
        let field = |name: &str, class_name: &str, descriptor: FieldDescriptor| {
            FieldExprent::new(
                name.to_string(),
                class_name.to_string(),
                true,
                None,
                descriptor,
                bytecode.as_ref(),
            )
            .to_java(0)
        };

        let unboxed = VarType::UNBOXING_TYPES().get_or_default(&const_type, const_type.clone());

        match unboxed.type_ {
            CodeType::Boolean => {
                buf.append(if val().as_int() != 0 { "true" } else { "false" });
                return buf;
            }
            CodeType::Char => {
                let val = val().as_int();
                let ret = match ConstExprent::char_escape(val) {
                    Some(escape) => escape.to_string(),
                    None => {
                        let c = val as u16;
                        if ConstExprent::is_printable_ascii(c as i32)
                            || !ascii && TextUtil::is_printable_unicode(c)
                        {
                            from_utf16(&[c])
                        } else {
                            TextUtil::char_to_unicode_literal(c as i32)
                        }
                    }
                };
                buf.append(&ret).enclose("'", "'");
                return buf;
            }
            CodeType::Bytechar
            | CodeType::Shortchar
            | CodeType::Byte
            | CodeType::Short
            | CodeType::Int => {
                let value = val();
                let int_val = value.as_int();
                if !literal {
                    if int_val == i32::MAX {
                        let max = field(
                            "MAX_VALUE",
                            "java/lang/Integer",
                            FieldDescriptor::INTEGER_DESCRIPTOR(),
                        );
                        buf.append_buffer(&max);
                        return buf;
                    } else if int_val == i32::MIN {
                        let min = field(
                            "MIN_VALUE",
                            "java/lang/Integer",
                            FieldDescriptor::INTEGER_DESCRIPTOR(),
                        );
                        buf.append_buffer(&min);
                        return buf;
                    }
                }
                buf.append(&value.java_to_string());
                return buf;
            }
            CodeType::Long => {
                let value = val();
                let long_val = value.as_long();

                if !literal {
                    if long_val == i64::MAX {
                        let max = field(
                            "MAX_VALUE",
                            "java/lang/Long",
                            FieldDescriptor::LONG_DESCRIPTOR(),
                        );
                        buf.append_buffer(&max);
                        return buf;
                    } else if long_val == i64::MIN {
                        let min = field(
                            "MIN_VALUE",
                            "java/lang/Long",
                            FieldDescriptor::LONG_DESCRIPTOR(),
                        );
                        buf.append_buffer(&min);
                        return buf;
                    }
                }
                buf.append(&value.java_to_string()).append_char('L');
                return buf;
            }
            CodeType::Float => {
                let float_val = val().as_float();
                if !literal {
                    if float_val.is_nan() {
                        let nan = field(
                            "NaN",
                            "java/lang/Float",
                            FieldDescriptor::FLOAT_DESCRIPTOR(),
                        );
                        buf.append_buffer(&nan);
                        return buf;
                    }
                    let floats = ConstExprent::UNINLINED_FLOATS();
                    if let Some(f) = floats.get(&ConstValue::Float(float_val)) {
                        let text = f(bytecode.as_ref());
                        buf.append_buffer(&text);
                        return buf;
                    }
                } else if float_val.is_nan() {
                    buf.append("0.0F / 0.0F");
                    return buf;
                } else if float_val == f32::INFINITY {
                    buf.append("1.0F / 0.0F");
                    return buf;
                } else if float_val == f32::NEG_INFINITY {
                    buf.append("-1.0F / 0.0F");
                    return buf;
                }
                buf.append(&ConstExprent::trim_float(
                    &jnum::float_to_string(float_val),
                    float_val,
                ))
                .append_char('F');
                return buf;
            }
            CodeType::Double => {
                let double_val = val().as_double();
                if !literal {
                    if double_val.is_nan() {
                        let nan = field(
                            "NaN",
                            "java/lang/Double",
                            FieldDescriptor::DOUBLE_DESCRIPTOR(),
                        );
                        buf.append_buffer(&nan);
                        return buf;
                    }
                    let doubles = ConstExprent::UNINLINED_DOUBLES();
                    if let Some(f) = doubles.get(&ConstValue::Double(double_val)) {
                        let text = f(bytecode.as_ref());
                        buf.append_buffer(&text);
                        return buf;
                    }

                    let float_representation = double_val as f32;
                    if float_representation as f64 == double_val
                        && !kuna_fixes::enabled(KunaFix::DoubleConstantLiteral)
                    {
                        let trimmed_float = ConstExprent::trim_float(
                            &jnum::float_to_string(float_representation),
                            float_representation,
                        );
                        let trimmed_double = ConstExprent::trim_double(
                            &jnum::double_to_string(double_val),
                            double_val,
                        );
                        if trimmed_float.len() < trimmed_double.len() {
                            let floats = ConstExprent::UNINLINED_FLOATS();
                            if let Some(f) = floats.get(&ConstValue::Float(float_representation)) {
                                let text = f(bytecode.as_ref());
                                buf.append_buffer(&text);
                                return buf;
                            } else {
                                buf.append(&ConstExprent::trim_float(
                                    &jnum::float_to_string(float_representation),
                                    float_representation,
                                ))
                                .append("F");
                                return buf;
                            }
                        }
                    }
                } else if double_val.is_nan() {
                    buf.append("0.0 / 0.0");
                    return buf;
                } else if double_val == f64::INFINITY {
                    buf.append("1.0 / 0.0");
                    return buf;
                } else if double_val == f64::NEG_INFINITY {
                    buf.append("-1.0 / 0.0");
                    return buf;
                }
                buf.append(&ConstExprent::trim_double(
                    &jnum::double_to_string(double_val),
                    double_val,
                ));
                return buf;
            }
            CodeType::Null => {
                buf.append("null");
                return buf;
            }
            CodeType::Object => {
                if const_type.equals(&VarType::VARTYPE_STRING()) {
                    let text = ConstExprent::convert_string_to_java(&val().java_to_string(), ascii);
                    buf.append(&text).enclose("\"", "\"");
                    return buf;
                } else if const_type.equals(&VarType::VARTYPE_CLASS()) {
                    let string_val = val().java_to_string();
                    let type_ =
                        VarType::new_signature_cl(&string_val, !string_val.starts_with('['));
                    buf.append_cast_type_name(&type_).append(".class");
                    return buf;
                } else if const_type.equals(&VarType::VARTYPE_METHODTYPE()) {
                    if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILER_COMMENTS) {
                        buf.append("/* $VF: MethodType constant */ ");
                    }
                    DecompilerContext::get_import_collector()
                        .get_short_name(&const_type.value_str().replace('/', "."));
                    buf.append(&format!(
                        "MethodType.fromMethodDescriptorString(\"{}\", null)",
                        value_text(&value)
                    ));
                    return buf;
                }
            }
            _ => {}
        }

        buf.convert_to_string_and_allow_data_discard();
        jthrow!(
            "invalid constant type: {} with value {}",
            const_type,
            value_text(&value)
        )
    }

    pub fn get_precedence(self) -> i32 {
        let (const_type, value) = self.d(|d| (d.const_type.clone(), d.value.clone()));
        let Some(value) = value else {
            return Exprent::get_precedence_base(self.0);
        };
        if DecompilerContext::get_option(IFernflowerPreferences::LITERALS_AS_IS) {
            return Exprent::get_precedence_base(self.0);
        }

        let unboxed = VarType::UNBOXING_TYPES().get_or_default(&const_type, const_type.clone());

        match unboxed.type_ {
            CodeType::Float => {
                let key = ConstValue::Float(value.as_float());
                let floats = ConstExprent::UNINLINED_FLOATS();
                if let Some(f) = floats.get(&key) {
                    if !ConstExprent::NO_PAREN_VALUES().contains(&key)
                        && f(self.get_bytecode().as_ref()).count_chars('(') < 2
                    {
                        return 4;
                    }
                }
            }
            CodeType::Double => {
                let key = ConstValue::Double(value.as_double());
                let doubles = ConstExprent::UNINLINED_DOUBLES();
                if let Some(f) = doubles.get(&key) {
                    if !ConstExprent::NO_PAREN_VALUES().contains(&key)
                        && f(self.get_bytecode().as_ref()).count_chars('(') < 2
                    {
                        return 4;
                    }
                }
            }
            _ => {}
        }

        Exprent::get_precedence_base(self.0)
    }

    fn get_pi_double(bytecode: Option<&BitSet>) -> TextBuffer {
        ConstExprent::get_double(bytecode, "PI", "java/lang/Math")
    }

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

    fn trim_float(value: &str, start: f32) -> String {
        if value.len() <= 3 {
            return value.to_string();
        }

        let mut value = value.to_string();
        let mut exp = String::new();
        if let Some(e_idx) = value.find('E') {
            exp = value[e_idx..].to_string();
            value.truncate(e_idx);
        }

        let mut temp = value.clone();
        let dot_idx = value.find('.');
        loop {
            value = temp;
            temp = value[..value.len() - 1].to_string();
            if !(!temp.is_empty()
                && temp != "-"
                && jnum::parse_float(&format!("{temp}{exp}")) == start)
            {
                break;
            }
        }

        if let Some(dot_idx) = dot_idx {
            if !value.contains('.') {
                value.push_str(".0");
            } else {
                let integer = &value[..dot_idx];
                let decimal = &value[dot_idx + 1..];

                let rounded = format!("{}.0{}", jnum::parse_int(integer).wrapping_add(1), exp);
                if jnum::parse_float(&rounded) == start {
                    return rounded;
                }

                let mut decimal_val: i64 = 1;
                let digits = decimal.as_bytes();
                for i in 0..digits.len().saturating_sub(1) {
                    decimal_val = decimal_val
                        .wrapping_sub(1)
                        .wrapping_mul(10)
                        .wrapping_add(digits[i] as i64)
                        .wrapping_sub(b'0' as i64)
                        .wrapping_add(1);
                    let rounded = format!("{integer}.{decimal_val}{exp}");
                    if jnum::parse_float(&rounded) == start {
                        return rounded;
                    }
                }
            }
        }

        value + &exp
    }

    fn trim_double(value: &str, start: f64) -> String {
        if value.len() <= 3 {
            return value.to_string();
        }

        let mut value = value.to_string();
        let mut exp = String::new();
        if let Some(e_idx) = value.find('E') {
            exp = value[e_idx..].to_string();
            value.truncate(e_idx);
        }

        let mut temp = value.clone();
        let dot_idx = value.find('.');
        loop {
            value = temp;
            temp = value[..value.len() - 1].to_string();
            if !(!temp.is_empty() && temp != "-" && jnum::parse_double(&temp) == start) {
                break;
            }
        }

        if let Some(dot_idx) = dot_idx {
            if !value.contains('.') {
                value.push_str(".0");
            } else {
                let integer = &value[..dot_idx];
                let decimal = &value[dot_idx + 1..];

                let rounded = format!("{}.0{}", jnum::parse_long(integer).wrapping_add(1), exp);
                if jnum::parse_double(&rounded) == start {
                    return rounded;
                }

                let mut decimal_val: i64 = 1;
                let digits = decimal.as_bytes();
                for i in 0..digits.len().saturating_sub(1) {
                    decimal_val = decimal_val
                        .wrapping_sub(1)
                        .wrapping_mul(10)
                        .wrapping_add(digits[i] as i64)
                        .wrapping_sub(b'0' as i64)
                        .wrapping_add(1);
                    let rounded = format!("{integer}.{decimal_val}{exp}");
                    if jnum::parse_double(&rounded) == start {
                        return rounded;
                    }
                }
            }
        }

        value + &exp
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
        let _lock = DecompilerContext::get_import_collector().lock();
        format!(
            "const({})",
            self.0.to_java(0).convert_to_string_and_allow_data_discard()
        )
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        let (const_type, value) = self.d(|d| (d.const_type.clone(), d.value.clone()));
        match_node.iterate_rules(|key, rule| {
            if key == MatchProperties::ExprentConsttype {
                return rule
                    .value
                    .as_ref()
                    .expect("NullPointerException: EXPRENT_CONSTTYPE value")
                    .equals(&MatchValue::VarType(const_type.clone()));
            } else if key == MatchProperties::ExprentConstvalue {
                return !rule.is_variable()
                    || engine.check_and_set_variable_value(
                        &rule.to_string(),
                        value.as_ref().map(|v| MatchValue::Constant(v.into())),
                    );
            }

            true
        })
    }
}
