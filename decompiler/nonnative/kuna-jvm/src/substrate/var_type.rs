//! Port of `org/jetbrains/java/decompiler/struct/gen/VarType.java`.
//!
//! `GenericType extends VarType` is merged into this type: a `VarType` whose
//! data carries `generic` is a `GenericType` instance (Java `instanceof`), and
//! every method `GenericType` overrides dispatches on that.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::generic_type::GenericType;
use crate::rt::jstring::java_string_hash;
use crate::rt::{JHashMap, JLinkedHashMap, JavaEq, JavaHash, JavaStr};
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;

/// An immutable Java `VarType`/`GenericType`; clones share the object, so
/// [`VarType::ptr_eq`] is Java `==`.
#[derive(Clone)]
pub struct VarType(Rc<VarTypeData>);

pub struct VarTypeData {
    pub type_: CodeType,
    pub array_dim: i32,
    pub value: Option<Rc<str>>,
    pub type_family: TypeFamily,
    pub stack_size: i32,
    /// Present exactly when the object is a `GenericType`.
    pub generic: Option<GenericData>,
}

/// The fields `GenericType` adds to `VarType`.
pub struct GenericData {
    pub parent: Option<VarType>,
    pub arguments: Rc<[Option<VarType>]>,
    pub wildcard: i32,
}

impl Deref for VarType {
    type Target = VarTypeData;
    fn deref(&self) -> &VarTypeData {
        &self.0
    }
}

/// A Java `String` argument that may be `null`.
pub trait JStringArg {
    fn into_jstr(self) -> Option<Rc<str>>;
}

impl JStringArg for &str {
    fn into_jstr(self) -> Option<Rc<str>> {
        Some(Rc::from(self))
    }
}
impl JStringArg for String {
    fn into_jstr(self) -> Option<Rc<str>> {
        Some(Rc::from(self))
    }
}
impl JStringArg for &String {
    fn into_jstr(self) -> Option<Rc<str>> {
        Some(Rc::from(self.as_str()))
    }
}
impl JStringArg for Rc<str> {
    fn into_jstr(self) -> Option<Rc<str>> {
        Some(self)
    }
}
impl JStringArg for &Rc<str> {
    fn into_jstr(self) -> Option<Rc<str>> {
        Some(self.clone())
    }
}
impl JStringArg for Option<Rc<str>> {
    fn into_jstr(self) -> Option<Rc<str>> {
        self
    }
}
impl JStringArg for &Option<Rc<str>> {
    fn into_jstr(self) -> Option<Rc<str>> {
        self.clone()
    }
}
impl JStringArg for Option<&str> {
    fn into_jstr(self) -> Option<Rc<str>> {
        self.map(Rc::from)
    }
}
impl JStringArg for Option<String> {
    fn into_jstr(self) -> Option<Rc<str>> {
        self.map(Rc::from)
    }
}
impl JStringArg for Option<&String> {
    fn into_jstr(self) -> Option<Rc<str>> {
        self.map(|s| Rc::from(s.as_str()))
    }
}

/// A Java `Map<VarType, VarType>` as `remap` reads it; a value may be Java `null`.
pub trait VarTypeMap {
    fn vt_contains_key(&self, key: &VarType) -> bool;
    fn vt_get(&self, key: &VarType) -> Option<VarType>;
}

impl VarTypeMap for JHashMap<VarType, VarType> {
    fn vt_contains_key(&self, key: &VarType) -> bool {
        self.contains_key(key)
    }
    fn vt_get(&self, key: &VarType) -> Option<VarType> {
        self.get(key).cloned()
    }
}
impl VarTypeMap for JHashMap<VarType, Option<VarType>> {
    fn vt_contains_key(&self, key: &VarType) -> bool {
        self.contains_key(key)
    }
    fn vt_get(&self, key: &VarType) -> Option<VarType> {
        self.get(key).cloned().flatten()
    }
}
impl VarTypeMap for JLinkedHashMap<VarType, VarType> {
    fn vt_contains_key(&self, key: &VarType) -> bool {
        self.contains_key(key)
    }
    fn vt_get(&self, key: &VarType) -> Option<VarType> {
        self.get(key).cloned()
    }
}
impl VarTypeMap for JLinkedHashMap<VarType, Option<VarType>> {
    fn vt_contains_key(&self, key: &VarType) -> bool {
        self.contains_key(key)
    }
    fn vt_get(&self, key: &VarType) -> Option<VarType> {
        self.get(key).cloned().flatten()
    }
}

/// A Java `String` viewed as UTF-16 units, so `charAt`/`substring`/`indexOf`
/// indices are the Java ones; ASCII text is used in place.
pub(crate) enum JStr<'a> {
    Ascii(&'a str),
    Wide(Vec<u16>),
}

impl<'a> JStr<'a> {
    pub(crate) fn new(s: &'a str) -> JStr<'a> {
        if s.is_ascii() {
            JStr::Ascii(s)
        } else {
            JStr::Wide(s.utf16())
        }
    }

    pub(crate) fn len(&self) -> usize {
        match self {
            JStr::Ascii(s) => s.len(),
            JStr::Wide(u) => u.len(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[track_caller]
    pub(crate) fn at(&self, index: usize) -> u16 {
        match self {
            JStr::Ascii(s) => s.as_bytes()[index] as u16,
            JStr::Wide(u) => u[index],
        }
    }

    #[track_caller]
    pub(crate) fn sub(&self, begin: usize, end: usize) -> String {
        if begin > end || end > self.len() {
            crate::jthrow!(StringIndexOutOfBoundsException; "Range [{}, {}) out of bounds for length {}", begin, end, self.len());
        }
        match self {
            JStr::Ascii(s) => s[begin..end].to_string(),
            JStr::Wide(u) => crate::rt::jstring::from_utf16(&u[begin..end]),
        }
    }

    #[track_caller]
    pub(crate) fn sub_from(&self, begin: usize) -> String {
        self.sub(begin, self.len())
    }

    pub(crate) fn index_of_from(&self, c: u16, from: usize) -> i32 {
        (from..self.len())
            .find(|&i| self.at(i) == c)
            .map_or(-1, |i| i as i32)
    }

    pub(crate) fn last_index_of(&self, c: u16) -> i32 {
        (0..self.len())
            .rev()
            .find(|&i| self.at(i) == c)
            .map_or(-1, |i| i as i32)
    }

    /// `lastIndexOf(ch, fromIndex)`.
    pub(crate) fn last_index_of_from(&self, c: u16, from: i32) -> i32 {
        if from < 0 || self.is_empty() {
            return -1;
        }
        let start = (from as usize).min(self.len() - 1);
        (0..=start)
            .rev()
            .find(|&i| self.at(i) == c)
            .map_or(-1, |i| i as i32)
    }
}

fn jstr_len(s: &str) -> usize {
    s.jlen()
}

/// `String.split(regex)` (limit 0) for a regex that matches one separator
/// character (`runs == false`) or a maximal run of them (`runs == true`, e.g. `\s+`).
pub(crate) fn java_split(s: &str, is_sep: impl Fn(char) -> bool, runs: bool) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut found = false;
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if !is_sep(c) {
            continue;
        }
        found = true;
        parts.push(&s[start..i]);
        let mut end = i + c.len_utf8();
        if runs {
            while let Some(&(j, d)) = chars.peek() {
                if !is_sep(d) {
                    break;
                }
                end = j + d.len_utf8();
                chars.next();
            }
        }
        start = end;
    }
    if !found {
        return vec![s];
    }
    parts.push(&s[start..]);
    while parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    parts
}

struct Constants {
    unknown: VarType,
    int: VarType,
    float: VarType,
    long: VarType,
    double: VarType,
    byte: VarType,
    char: VarType,
    short: VarType,
    boolean: VarType,
    bytechar: VarType,
    shortchar: VarType,
    null: VarType,
    string: VarType,
    class: VarType,
    object: VarType,
    integer: VarType,
    character: VarType,
    byte_obj: VarType,
    short_obj: VarType,
    boolean_obj: VarType,
    float_obj: VarType,
    double_obj: VarType,
    long_obj: VarType,
    void: VarType,
    methodtype: VarType,
    unboxing_types: Rc<JHashMap<VarType, VarType>>,
}

impl Constants {
    fn new() -> Constants {
        let obj = |name: &str| VarType::new_value(CodeType::Object, 0, name);
        let unknown = VarType::new(CodeType::Unknown);
        let int = VarType::new(CodeType::Int);
        let float = VarType::new(CodeType::Float);
        let long = VarType::new(CodeType::Long);
        let double = VarType::new(CodeType::Double);
        let byte = VarType::new(CodeType::Byte);
        let char = VarType::new(CodeType::Char);
        let short = VarType::new(CodeType::Short);
        let boolean = VarType::new(CodeType::Boolean);
        let bytechar = VarType::new(CodeType::Bytechar);
        let shortchar = VarType::new(CodeType::Shortchar);
        let null = VarType::new_value(CodeType::Null, 0, None::<Rc<str>>);
        let string = obj("java/lang/String");
        let class = obj("java/lang/Class");
        let object = obj("java/lang/Object");
        let integer = obj("java/lang/Integer");
        let character = obj("java/lang/Character");
        let byte_obj = obj("java/lang/Byte");
        let short_obj = obj("java/lang/Short");
        let boolean_obj = obj("java/lang/Boolean");
        let float_obj = obj("java/lang/Float");
        let double_obj = obj("java/lang/Double");
        let long_obj = obj("java/lang/Long");
        let void = VarType::new(CodeType::Void);
        let methodtype = obj("java/lang/invoke/MethodType");

        let mut unboxing = JHashMap::new();
        unboxing.put(integer.clone(), int.clone());
        unboxing.put(character.clone(), char.clone());
        unboxing.put(byte_obj.clone(), byte.clone());
        unboxing.put(short_obj.clone(), short.clone());
        unboxing.put(boolean_obj.clone(), boolean.clone());
        unboxing.put(float_obj.clone(), float.clone());
        unboxing.put(double_obj.clone(), double.clone());
        unboxing.put(long_obj.clone(), long.clone());

        Constants {
            unknown,
            int,
            float,
            long,
            double,
            byte,
            char,
            short,
            boolean,
            bytechar,
            shortchar,
            null,
            string,
            class,
            object,
            integer,
            character,
            byte_obj,
            short_obj,
            boolean_obj,
            float_obj,
            double_obj,
            long_obj,
            void,
            methodtype,
            unboxing_types: Rc::new(unboxing),
        }
    }
}

thread_local! {
    static CONSTANTS: Constants = Constants::new();
}

macro_rules! vartype_constants {
    ($($name:ident => $field:ident),* $(,)?) => {
        impl VarType {
            $(
                #[allow(non_snake_case)]
                pub fn $name() -> VarType {
                    CONSTANTS.with(|c| c.$field.clone())
                }
            )*
        }
    };
}

vartype_constants! {
    VARTYPE_UNKNOWN => unknown,
    VARTYPE_INT => int,
    VARTYPE_FLOAT => float,
    VARTYPE_LONG => long,
    VARTYPE_DOUBLE => double,
    VARTYPE_BYTE => byte,
    VARTYPE_CHAR => char,
    VARTYPE_SHORT => short,
    VARTYPE_BOOLEAN => boolean,
    VARTYPE_BYTECHAR => bytechar,
    VARTYPE_SHORTCHAR => shortchar,
    VARTYPE_NULL => null,
    VARTYPE_STRING => string,
    VARTYPE_CLASS => class,
    VARTYPE_OBJECT => object,
    VARTYPE_INTEGER => integer,
    VARTYPE_CHARACTER => character,
    VARTYPE_BYTE_OBJ => byte_obj,
    VARTYPE_SHORT_OBJ => short_obj,
    VARTYPE_BOOLEAN_OBJ => boolean_obj,
    VARTYPE_FLOAT_OBJ => float_obj,
    VARTYPE_DOUBLE_OBJ => double_obj,
    VARTYPE_LONG_OBJ => long_obj,
    VARTYPE_VOID => void,
    VARTYPE_METHODTYPE => methodtype,
}

impl VarType {
    #[allow(non_snake_case)]
    pub fn EMPTY_ARRAY() -> Vec<VarType> {
        Vec::new()
    }

    #[allow(non_snake_case)]
    pub fn UNBOXING_TYPES() -> Rc<JHashMap<VarType, VarType>> {
        CONSTANTS.with(|c| c.unboxing_types.clone())
    }

    pub(crate) fn from_data(data: VarTypeData) -> VarType {
        VarType(Rc::new(data))
    }

    /// Java: `VarType(CodeType)`
    pub fn new(type_: CodeType) -> VarType {
        VarType::new_dim(type_, 0)
    }

    /// Java: `VarType(CodeType, int)`
    pub fn new_dim(type_: CodeType, array_dim: i32) -> VarType {
        VarType::new_value(type_, array_dim, VarType::get_char(type_))
    }

    /// Java: `VarType(CodeType, int, String)`
    pub fn new_value(type_: CodeType, array_dim: i32, value: impl JStringArg) -> VarType {
        VarType::new_full(
            type_,
            array_dim,
            value.into_jstr(),
            VarType::get_family(type_, array_dim),
            VarType::get_stack_size(type_, array_dim),
        )
    }

    /// Java: `VarType(CodeType, int, String, TypeFamily, int)`
    pub fn new_full(
        type_: CodeType,
        array_dim: i32,
        value: Option<Rc<str>>,
        type_family: TypeFamily,
        stack_size: i32,
    ) -> VarType {
        VarType::new_with_generic(type_, array_dim, value, type_family, stack_size, None)
    }

    pub(crate) fn new_with_generic(
        type_: CodeType,
        array_dim: i32,
        value: Option<Rc<str>>,
        type_family: TypeFamily,
        stack_size: i32,
        generic: Option<GenericData>,
    ) -> VarType {
        ValidationHelper::assert_true(true, "Type and type family must not be null");
        ValidationHelper::assert_true(
            type_ == CodeType::Null || value.is_some(),
            "Must not be null for non null type",
        );
        VarType::from_data(VarTypeData {
            type_,
            array_dim,
            value,
            type_family,
            stack_size,
            generic,
        })
    }

    /// Java: `VarType(String)`
    pub fn new_signature(signature: &str) -> VarType {
        VarType::new_signature_cl(signature, false)
    }

    /// Java: `VarType(String, boolean)`
    pub fn new_signature_cl(signature: &str, cl_type: bool) -> VarType {
        let sig = JStr::new(signature);
        let mut type_ = CodeType::Byte;
        let mut array_dim: i32 = 0;
        let mut value: Option<String> = None;

        let mut i = 0;
        while i < sig.len() {
            let c = sig.at(i);
            if c == b'[' as u16 {
                array_dim = array_dim.wrapping_add(1);
            } else {
                if c == b'L' as u16 && sig.at(sig.len() - 1) == b';' as u16 {
                    type_ = CodeType::Object;
                    value = Some(sig.sub(i + 1, sig.len() - 1));
                    break;
                }
                let v = sig.sub_from(i);
                if (cl_type && i == 0) || jstr_len(&v) > 1 {
                    type_ = CodeType::Object;
                } else {
                    type_ = VarType::get_type_unit(v.jchar_at(0));
                }
                value = Some(v);
                break;
            }
            i += 1;
        }

        VarType::from_data(VarTypeData {
            type_,
            array_dim,
            value: value.map(Rc::from),
            type_family: VarType::get_family(type_, array_dim),
            stack_size: VarType::get_stack_size(type_, array_dim),
            generic: None,
        })
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &VarType) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// `value` as Java string concatenation prints it.
    pub fn value_or_null(&self) -> &str {
        self.value.as_deref().unwrap_or("null")
    }

    /// `value` where Java dereferences it unconditionally.
    #[track_caller]
    pub fn value_str(&self) -> &str {
        match self.value.as_deref() {
            Some(v) => v,
            None => crate::rt::throw("NullPointerException", "VarType.value is null"),
        }
    }

    pub fn get_char(type_: CodeType) -> Option<&'static str> {
        match type_ {
            CodeType::Byte => Some("B"),
            CodeType::Char => Some("C"),
            CodeType::Double => Some("D"),
            CodeType::Float => Some("F"),
            CodeType::Int => Some("I"),
            CodeType::Long => Some("J"),
            CodeType::Short => Some("S"),
            CodeType::Boolean => Some("Z"),
            CodeType::Void => Some("V"),
            CodeType::Group2empty => Some("G"),
            CodeType::Notinitialized => Some("N"),
            CodeType::Address => Some("A"),
            CodeType::Bytechar => Some("X"),
            CodeType::Shortchar => Some("Y"),
            CodeType::Unknown => Some("U"),
            CodeType::Null | CodeType::Object => None,
            _ => crate::jthrow!("Invalid type"),
        }
    }

    pub fn get_stack_size(type_: CodeType, array_dim: i32) -> i32 {
        if array_dim > 0 {
            return 1;
        }
        match type_ {
            CodeType::Double | CodeType::Long => 2,
            CodeType::Void | CodeType::Group2empty => 0,
            _ => 1,
        }
    }

    pub fn get_family(type_: CodeType, array_dim: i32) -> TypeFamily {
        if array_dim > 0 {
            return TypeFamily::Object;
        }
        match type_ {
            CodeType::Byte
            | CodeType::Bytechar
            | CodeType::Shortchar
            | CodeType::Char
            | CodeType::Short
            | CodeType::Int => TypeFamily::Integer,
            CodeType::Double => TypeFamily::Double,
            CodeType::Float => TypeFamily::Float,
            CodeType::Long => TypeFamily::Long,
            CodeType::Boolean => TypeFamily::Boolean,
            CodeType::Null | CodeType::Object | CodeType::Genvar => TypeFamily::Object,
            _ => TypeFamily::Unknown,
        }
    }

    /// Java `x instanceof GenericType`.
    pub fn is_generic_type(&self) -> bool {
        self.generic.is_some()
    }

    /// Java `x instanceof GenericType g`.
    pub fn try_as_generic(&self) -> Option<GenericType> {
        self.generic.is_some().then(|| GenericType(self.clone()))
    }

    /// Java `(GenericType) x`.
    #[track_caller]
    pub fn as_generic(&self) -> GenericType {
        if self.generic.is_none() {
            crate::rt::throw(
                "ClassCastException",
                "VarType cannot be cast to GenericType",
            );
        }
        GenericType(self.clone())
    }

    pub fn decrease_array_dim(&self) -> VarType {
        if let Some(g) = self.try_as_generic() {
            return GenericType::decrease_array_dim(&g).0;
        }
        self.decrease_array_dim_base()
    }

    /// `VarType.decreaseArrayDim`'s own body.
    pub fn decrease_array_dim_base(&self) -> VarType {
        if self.array_dim > 0 {
            VarType::new_value(self.type_, self.array_dim - 1, &self.value)
        } else {
            self.clone()
        }
    }

    pub fn resize_array_dim(&self, new_array_dim: i32) -> VarType {
        if let Some(g) = self.try_as_generic() {
            return GenericType::resize_array_dim(&g, new_array_dim);
        }
        self.resize_array_dim_base(new_array_dim)
    }

    /// `VarType.resizeArrayDim`'s own body.
    pub fn resize_array_dim_base(&self, new_array_dim: i32) -> VarType {
        ValidationHelper::assert_true(new_array_dim >= 0, "Can't have an array of negative size!");
        VarType::new_full(
            self.type_,
            new_array_dim,
            self.value.clone(),
            self.type_family,
            self.stack_size,
        )
    }

    pub fn hash_code(&self) -> i32 {
        let mut result: i32 = 1;
        result = result.wrapping_mul(37).wrapping_add(self.type_.ordinal());
        result = result.wrapping_mul(37).wrapping_add(self.array_dim);
        result = result
            .wrapping_mul(37)
            .wrapping_add(self.value.as_deref().map_or(0, java_string_hash));
        result
    }

    pub fn equals(&self, o: &VarType) -> bool {
        if o.ptr_eq(self) {
            return true;
        }
        self.type_ == o.type_ && self.array_dim == o.array_dim && self.value == o.value
    }

    /// Java `equals(Object)` with a possibly-null argument.
    pub fn equals_opt(&self, o: Option<&VarType>) -> bool {
        o.is_some_and(|o| self.equals(o))
    }

    /// `VarType.toString`'s own body.
    pub fn to_string_base(&self) -> String {
        let mut res = String::new();
        for _ in 0..self.array_dim {
            res.push('[');
        }
        if self.type_ == CodeType::Object {
            res.push('L');
            res.push_str(self.value_or_null());
            res.push(';');
        } else {
            res.push_str(self.value.as_deref().unwrap_or("--null--"));
        }
        res
    }

    pub fn get_type(c: char) -> CodeType {
        match c {
            'B' => CodeType::Byte,
            'C' => CodeType::Char,
            'D' => CodeType::Double,
            'F' => CodeType::Float,
            'I' => CodeType::Int,
            'J' => CodeType::Long,
            'S' => CodeType::Short,
            'Z' => CodeType::Boolean,
            'V' => CodeType::Void,
            'G' => CodeType::Group2empty,
            'N' => CodeType::Notinitialized,
            'A' => CodeType::Address,
            'X' => CodeType::Bytechar,
            'Y' => CodeType::Shortchar,
            'U' => CodeType::Unknown,
            _ => crate::jthrow!(IllegalArgumentException; "Invalid type: {}", c),
        }
    }

    pub(crate) fn get_type_unit(c: u16) -> CodeType {
        match char::from_u32(c as u32) {
            Some(ch) => VarType::get_type(ch),
            None => crate::jthrow!(
                IllegalArgumentException;
                "Invalid type: {}",
                crate::rt::jstring::from_utf16(&[c])
            ),
        }
    }

    pub fn is_primitive(type_: &VarType) -> bool {
        VarType::UNBOXING_TYPES().values().any(|v| type_.equals(v))
    }

    pub fn is_generic(&self) -> bool {
        if let Some(g) = self.try_as_generic() {
            return GenericType::is_generic(&g);
        }
        false
    }

    pub fn remap<M: VarTypeMap + ?Sized>(&self, map: &M) -> Option<VarType> {
        if let Some(g) = self.try_as_generic() {
            return GenericType::remap(&g, map);
        }
        self.remap_base(map)
    }

    /// `VarType.remap`'s own body.
    pub fn remap_base<M: VarTypeMap + ?Sized>(&self, map: &M) -> Option<VarType> {
        let key = if self.array_dim == 0 {
            self.clone()
        } else {
            self.resize_array_dim(0)
        };
        if map.vt_contains_key(&key) {
            let ret = map.vt_get(&key);
            return match ret {
                Some(r) if self.array_dim != 0 => {
                    Some(r.resize_array_dim(r.array_dim.wrapping_add(self.array_dim)))
                }
                other => other,
            };
        }
        Some(self.clone())
    }

    pub fn higher_equal_in_lattice_than(&self, val: &VarType) -> bool {
        self.equals(val)
            || self.higher_in_lattice_than(val)
            || self.equals_opt(VarType::UNBOXING_TYPES().get(val))
    }

    pub fn higher_in_lattice_than(&self, other: &VarType) -> bool {
        if other.type_ == CodeType::Unknown && self.type_ != CodeType::Unknown {
            return true;
        }

        if self.array_dim > 0 && self.array_dim == other.array_dim {
            return self
                .resize_array_dim(0)
                .higher_in_lattice_than(&other.resize_array_dim(0));
        } else if other.array_dim > 0 {
            return self.equals(&VarType::VARTYPE_OBJECT());
        } else if self.array_dim > 0 {
            return other.type_ == CodeType::Null;
        }

        let mut res = false;
        let t = self.type_;
        match t {
            CodeType::Int
            | CodeType::Short
            | CodeType::Char
            | CodeType::Byte
            | CodeType::Shortchar
            | CodeType::Bytechar => {
                if t == CodeType::Int {
                    res = other.type_ == CodeType::Short || other.type_ == CodeType::Char;
                }
                if matches!(t, CodeType::Int | CodeType::Short) {
                    res |= other.type_ == CodeType::Byte;
                }
                if matches!(t, CodeType::Int | CodeType::Short | CodeType::Char) {
                    res |= other.type_ == CodeType::Shortchar;
                }
                if t != CodeType::Bytechar {
                    res |= other.type_ == CodeType::Bytechar;
                }
                res |= other.type_ == CodeType::Boolean;
            }
            CodeType::Object => {
                if other.type_ == CodeType::Null {
                    return true;
                } else if self.equals(&VarType::VARTYPE_OBJECT()) {
                    return !other.equals(&VarType::VARTYPE_OBJECT());
                }

                let mut check_base = true;
                if self.is_generic() && other.is_generic() {
                    check_base =
                        VarType::should_check_generic_base(&self.as_generic(), &other.as_generic());
                }

                if check_base
                    && other.type_ == CodeType::Object
                    && !str_equals(&other.value, &self.value)
                    && instance_of(&other.value, &self.value)
                {
                    return true;
                }
            }
            _ => {}
        }

        res
    }

    fn should_check_generic_base(t: &GenericType, other: &GenericType) -> bool {
        if t.arguments_equal(other) {
            return t.get_wildcard() == other.get_wildcard();
        } else if t.get_arguments().len() == other.get_arguments().len() {
            for i in 0..t.get_arguments().len() {
                let a1 = &t.get_arguments()[i];
                let a2 = &other.get_arguments()[i];
                if let (Some(a2), Some(a1)) = (a2, a1) {
                    if let Some(ga1) = a1.try_as_generic() {
                        if ga1.get_wildcard() != GenericType::WILDCARD_NO
                            && str_equals(&a1.value, &a2.value)
                            && !a2.is_generic()
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn higher_cross_family_than(&self, other: &VarType, equal: bool) -> bool {
        if equal {
            if self.higher_equal_in_lattice_than(other) {
                return true;
            }
        } else if self.higher_in_lattice_than(other) {
            return true;
        }

        let mut res = false;
        let t = self.type_;
        if t == CodeType::Double {
            res = other.type_family == TypeFamily::Float;
        }
        if matches!(t, CodeType::Double | CodeType::Float) {
            res |= other.type_family == TypeFamily::Long;
        }
        if matches!(t, CodeType::Double | CodeType::Float | CodeType::Long) {
            res |= other.type_family == TypeFamily::Integer;
        }
        res
    }

    pub fn meet(type1: &VarType, type2: &VarType) -> Option<VarType> {
        if type1.higher_equal_in_lattice_than(type2) {
            return Some(type2.clone());
        } else if type2.higher_equal_in_lattice_than(type1) {
            return Some(type1.clone());
        } else if type1.type_family == type2.type_family {
            match type1.type_family {
                TypeFamily::Integer => {
                    if (type1.type_ == CodeType::Char && type2.type_ == CodeType::Short)
                        || (type1.type_ == CodeType::Short && type2.type_ == CodeType::Char)
                    {
                        return Some(VarType::VARTYPE_SHORTCHAR());
                    } else {
                        return Some(VarType::VARTYPE_BYTECHAR());
                    }
                }
                TypeFamily::Object => return Some(VarType::VARTYPE_NULL()),
                _ => {}
            }
        }
        None
    }

    pub fn join(type1: &VarType, type2: &VarType) -> Option<VarType> {
        if type1.higher_equal_in_lattice_than(type2) {
            return Some(type1.clone());
        } else if type2.higher_equal_in_lattice_than(type1) {
            return Some(type2.clone());
        } else if type1.type_family == type2.type_family {
            match type1.type_family {
                TypeFamily::Integer => {
                    if (type1.type_ == CodeType::Shortchar && type2.type_ == CodeType::Byte)
                        || (type1.type_ == CodeType::Byte && type2.type_ == CodeType::Shortchar)
                    {
                        return Some(VarType::VARTYPE_SHORT());
                    } else {
                        return Some(VarType::VARTYPE_INT());
                    }
                }
                TypeFamily::Object => {
                    if type1.type_ == CodeType::Null {
                        return Some(type2.clone());
                    } else if type2.type_ == CodeType::Null {
                        return Some(type1.clone());
                    }

                    let ctx = DecompilerContext::get_struct_context();
                    let cl = match (type1.value.as_deref(), type2.value.as_deref()) {
                        (Some(a), Some(b)) => ctx.find_common_ancestor(a, b),
                        _ => None,
                    };
                    if let Some(cl) = cl {
                        let name = cl.qualified_name();
                        return Some(VarType::new_signature_cl(&name, true));
                    }

                    return Some(VarType::VARTYPE_OBJECT());
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_family_bottom(family: TypeFamily) -> VarType {
        match family {
            TypeFamily::Boolean => VarType::VARTYPE_BOOLEAN(),
            TypeFamily::Integer => VarType::VARTYPE_BYTECHAR(),
            TypeFamily::Object => VarType::VARTYPE_NULL(),
            TypeFamily::Float => VarType::VARTYPE_FLOAT(),
            TypeFamily::Long => VarType::VARTYPE_LONG(),
            TypeFamily::Double => VarType::VARTYPE_DOUBLE(),
            TypeFamily::Unknown => VarType::VARTYPE_UNKNOWN(),
        }
    }
}

/// Java `a.equals(b)` on two nullable strings where `a` is dereferenced.
#[track_caller]
pub(crate) fn str_equals(a: &Option<Rc<str>>, b: &Option<Rc<str>>) -> bool {
    match a {
        Some(a) => b.as_deref() == Some(&**a),
        None => crate::rt::throw("NullPointerException", "String.equals on null"),
    }
}

/// `DecompilerContext.getStructContext().instanceOf(valclass, refclass)`.
pub(crate) fn instance_of(valclass: &Option<Rc<str>>, refclass: &Option<Rc<str>>) -> bool {
    let ctx = DecompilerContext::get_struct_context();
    match (valclass.as_deref(), refclass.as_deref()) {
        (Some(v), Some(r)) => ctx.instance_of(v, r),
        _ => false,
    }
}

/// `DecompilerContext.getStructContext().getClass(name)`, where `getClass(null)` is null.
pub(crate) fn get_class(name: &Option<Rc<str>>) -> Option<crate::struct_class::StructClass> {
    let ctx = DecompilerContext::get_struct_context();
    name.as_deref().and_then(|n| ctx.get_class(n))
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.try_as_generic() {
            Some(g) => f.write_str(&GenericType::to_java_string(&g)),
            None => f.write_str(&self.to_string_base()),
        }
    }
}

impl std::fmt::Debug for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_base())?;
        if let Some(g) = &self.generic {
            write!(f, "<w{}", g.wildcard)?;
            if let Some(p) = &g.parent {
                write!(f, " parent={:?}", p)?;
            }
            for a in g.arguments.iter() {
                match a {
                    Some(a) => write!(f, " {:?}", a)?,
                    None => f.write_str(" ?")?,
                }
            }
            f.write_str(">")?;
        }
        Ok(())
    }
}

impl JavaHash for VarType {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for VarType {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
