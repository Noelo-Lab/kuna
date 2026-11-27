//! Port of `org/jetbrains/java/decompiler/modules/renamer/ConverterHelper.java`.

use std::cell::{Cell, RefCell};

use crate::code_constants::{CLINIT_NAME, INIT_NAME};
use crate::i_identifier_renamer::{IIdentifierRenamer, Type};
use crate::rt::{jchar, JHashSet, JavaStr};

const KEYWORDS: &[&str] = &[
    "abstract",
    "do",
    "if",
    "package",
    "synchronized",
    "boolean",
    "double",
    "implements",
    "private",
    "this",
    "break",
    "else",
    "import",
    "protected",
    "throw",
    "byte",
    "extends",
    "instanceof",
    "public",
    "throws",
    "case",
    "false",
    "int",
    "return",
    "transient",
    "catch",
    "final",
    "interface",
    "short",
    "true",
    "char",
    "finally",
    "long",
    "static",
    "try",
    "class",
    "float",
    "native",
    "strictfp",
    "void",
    "const",
    "for",
    "new",
    "super",
    "volatile",
    "continue",
    "goto",
    "null",
    "switch",
    "while",
    "default",
    "assert",
    "enum",
];

const RESERVED_WINDOWS_NAMESPACE: &[&str] = &[
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

pub struct ConverterHelper {
    class_counter: Cell<i32>,
    field_counter: Cell<i32>,
    method_counter: Cell<i32>,
    set_non_standard_class_names: RefCell<JHashSet<String>>,
}

impl Default for ConverterHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl ConverterHelper {
    pub fn new() -> ConverterHelper {
        ConverterHelper {
            class_counter: Cell::new(0),
            field_counter: Cell::new(0),
            method_counter: Cell::new(0),
            set_non_standard_class_names: RefCell::new(JHashSet::new()),
        }
    }

    fn is_valid_identifier(is_method: bool, identifier: &str) -> bool {
        if is_method && (identifier == INIT_NAME || identifier == CLINIT_NAME) {
            return true;
        }

        let chars = identifier.utf16();

        if !jchar::is_java_identifier_start(chars[0]) {
            return false;
        }

        for &ch in &chars[1..] {
            if (!jchar::is_java_identifier_part(ch)) || jchar::is_identifier_ignorable(ch) {
                return false;
            }
        }

        true
    }

    fn post_increment(counter: &Cell<i32>) -> i32 {
        let value = counter.get();
        counter.set(value.wrapping_add(1));
        value
    }

    pub fn get_simple_class_name(full_name: &str) -> String {
        let start = full_name.rfind('/').map_or(0, |i| i + 1);
        full_name[start..].to_string()
    }

    pub fn replace_simple_class_name(full_name: &str, new_name: &str) -> String {
        let end = full_name.rfind('/').map_or(0, |i| i + 1);
        format!("{}{}", &full_name[..end], new_name)
    }
}

/// `OBF_REGEX.matcher(value).matches()` for `^(?:.{1,3}(?:$|\$))+`: the value
/// splits into runs of one to three code points, each closed by `$` or the end.
fn matches_obf_regex(value: &str) -> bool {
    let cps: Vec<char> = value.chars().collect();
    let len = cps.len();
    let mut reachable = vec![false; len + 1];
    reachable[0] = true;
    for start in 0..len {
        if !reachable[start] {
            continue;
        }
        for k in 1..=3 {
            let end = start + k;
            if end > len || is_line_terminator(cps[end - 1]) {
                break;
            }
            if end == len {
                return true;
            }
            if cps[end] == '$' {
                if end + 1 == len {
                    return true;
                }
                reachable[end + 1] = true;
            }
        }
    }
    false
}

fn is_line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}')
}

impl IIdentifierRenamer for ConverterHelper {
    fn to_be_renamed(
        &self,
        element_type: Type,
        class_name: Option<&str>,
        element: Option<&str>,
        _descriptor: Option<&str>,
    ) -> bool {
        let value = if element_type == Type::ElementClass {
            class_name
        } else {
            element
        };
        let Some(value) = value else {
            return true;
        };
        if value.is_empty()
            || !Self::is_valid_identifier(element_type == Type::ElementMethod, value)
            || KEYWORDS.contains(&value)
        {
            return true;
        }
        let is_windows_reserved =
            RESERVED_WINDOWS_NAMESPACE.contains(&value.to_lowercase().as_str());
        (!is_windows_reserved && matches_obf_regex(value))
            || (element_type == Type::ElementClass
                && (is_windows_reserved || value.jlen() > 255 - 6))
    }

    fn get_next_class_name(&self, _full_name: &str, short_name: Option<&str>) -> String {
        let Some(short_name) = short_name else {
            return format!("class_{}", Self::post_increment(&self.class_counter));
        };

        let units = short_name.utf16();
        let mut index = 0;
        while index < units.len() && jchar::is_digit(units[index]) {
            index += 1;
        }

        if index == 0 || index == units.len() {
            format!("class_{}", Self::post_increment(&self.class_counter))
        } else {
            let name = short_name.jsubstring_from(index);
            if self.set_non_standard_class_names.borrow().contains(&name) {
                format!(
                    "Inner{}_{}",
                    name,
                    Self::post_increment(&self.class_counter)
                )
            } else {
                self.set_non_standard_class_names
                    .borrow_mut()
                    .add(name.clone());
                format!("Inner{name}")
            }
        }
    }

    fn get_next_field_name(&self, _class_name: &str, _field: &str, _descriptor: &str) -> String {
        format!("field_{}", Self::post_increment(&self.field_counter))
    }

    fn get_next_method_name(&self, _class_name: &str, _method: &str, _descriptor: &str) -> String {
        format!("method_{}", Self::post_increment(&self.method_counter))
    }
}
