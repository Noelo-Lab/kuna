//! Port of `org/vineflower/variablerenaming/TinyNameProvider.java` (adapted
//! there from tiny-remapper's `AsmClassRemapper`).

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_variable_name_provider::IVariableNameProvider;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::jad_name_provider::{sorted_keys, super_rename_parameter};
use crate::jthrow;
use crate::method_descriptor::MethodDescriptor;
use crate::pair::Pair;
use crate::rt::jchar;
use crate::rt::jstring::{from_utf16, JavaStr};
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap};
use crate::struct_method::StructMethod;
use crate::text_util::TextUtil;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::variable_renaming_options::VariableRenamingOptions;

pub struct TinyNameProvider {
    parameters: RefCell<JHashMap<i32, String>>,
    method: StructMethod,
    rename_parameters: bool,
    used_names: RefCell<JHashSet<String>>,
}

/// `Character.isUpperCase(char)`.
pub fn char_is_upper_case(ch: u16) -> bool {
    jchar::get_type(ch) == jchar::UPPERCASE_LETTER
        || matches!(ch, 0x2160..=0x216F | 0x24B6..=0x24CF)
}

/// `Character.toLowerCase(char)`: the simple lowercase mapping, for the characters the JDK 21
/// tables of `rt::jchar` assign.
pub fn char_to_lower_case(ch: u16) -> u16 {
    if ch == 0x0130 {
        return 'i' as u16;
    }
    if jchar::get_type(ch) == jchar::UNASSIGNED {
        return ch;
    }
    let Some(c) = char::from_u32(ch as u32) else {
        return ch;
    };
    let mut lower = c.to_lowercase();
    match (lower.next(), lower.next()) {
        (Some(l), None) if (l as u32) <= 0xFFFF => l as u16,
        _ => ch,
    }
}

/// `Integer.toString(i, radix)`.
pub fn int_to_string_radix(i: i32, radix: u32) -> String {
    let mut digits: Vec<u8> = Vec::new();
    let negative = i < 0;
    let mut v = (i as i64).unsigned_abs();
    loop {
        let d = (v % radix as u64) as u8;
        digits.push(if d < 10 { b'0' + d } else { b'a' + d - 10 });
        v /= radix as u64;
        if v == 0 {
            break;
        }
    }
    if negative {
        digits.push(b'-');
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

impl TinyNameProvider {
    pub fn new(rename_parameters: bool, method: StructMethod) -> TinyNameProvider {
        TinyNameProvider {
            parameters: RefCell::new(JHashMap::new()),
            method,
            rename_parameters,
            used_names: RefCell::new(JHashSet::new()),
        }
    }

    /// `parameters.computeIfAbsent(key, k -> make())`: a `null` result is not stored.
    fn compute_parameter(&self, key: i32, make: impl FnOnce() -> Option<String>) -> Option<String> {
        if let Some(name) = self.parameters.borrow().get_cloned(&key) {
            return Some(name);
        }
        let name = make();
        if let Some(name) = &name {
            self.parameters.borrow_mut().put(key, name.clone());
        }
        name
    }

    pub fn get_new_name(&self, pair: Pair<Option<VarType>, String>) -> Option<String> {
        self.used_names.borrow_mut().add(pair.b.clone());

        let type_ = pair.a.expect("NullPointerException: type");
        let mut increment = true;
        let mut name: String = match type_.type_ {
            CodeType::Bytechar | CodeType::Shortchar | CodeType::Int => "i".to_string(),
            CodeType::Long => "l".to_string(),
            CodeType::Byte => "b".to_string(),
            CodeType::Short => "s".to_string(),
            CodeType::Char => "c".to_string(),
            CodeType::Float => "f".to_string(),
            CodeType::Double => "d".to_string(),
            CodeType::Boolean => {
                increment = false;
                "bl".to_string()
            }
            CodeType::Object | CodeType::Genvar => {
                let mut name = pair.b.clone();

                if name.is_empty() {
                    jthrow!(StringIndexOutOfBoundsException; "index 0, length 0");
                }
                let first = name.jchar_at(0);
                if char_is_upper_case(first) {
                    let lower = from_utf16(&[char_to_lower_case(first)]);
                    name = format!("{}{}", lower, name.jsubstring_from(1));
                }

                if type_.array_dim > 0 && !name.ends_with('s') {
                    name.push('s');
                }

                increment = false;
                name
            }
            _ => return None,
        };

        if increment {
            let mut idx_start = name.jchar_at(0) as i32 - 'a' as i32;
            while self.used_names.borrow().contains(&name) {
                name = TinyNameProvider::convert_to_name(idx_start);
                idx_start = idx_start.wrapping_add(1);
            }
        } else {
            let oname = name.clone();
            let mut idx = 0;
            while self.used_names.borrow().contains(&name) {
                idx += 1;
                name = format!("{oname}{idx}");
            }
        }

        if TextUtil::is_keyword(&name, &self.method.get_bytecode_version(), self.method) {
            name.push('_');
        }

        self.used_names.borrow_mut().add(name.clone());

        Some(name)
    }

    pub fn convert_to_name(idx: i32) -> String {
        let str = int_to_string_radix(idx, 26).utf16();

        let mut res: Vec<u16> = Vec::new();
        for i in (0..str.len()).rev() {
            let mut c = str[i];
            if c <= '9' as u16 {
                c = ('a' as u16).wrapping_add(c.wrapping_sub('0' as u16));
            } else {
                c = c.wrapping_add(10);
            }

            res.insert(0, c);
        }
        from_utf16(&res)
    }

    fn clean_type(&self, type_: &str) -> String {
        let mut type_ = type_.to_string();
        if let Some(i) = type_.find('<') {
            type_ = type_[..i].to_string();
        }

        if let Some(i) = type_.rfind('.') {
            type_ = type_[i + 1..].to_string();
        }

        if let Some(i) = type_.rfind('$') {
            type_ = type_[i + 1..].to_string();
        }

        type_.replace("[]", "")
    }
}

impl IVariableNameProvider for TinyNameProvider {
    fn rename(
        &self,
        entries: &JLinkedHashMap<VarVersionPair, Pair<Option<VarType>, Option<String>>>,
    ) -> Option<JHashMap<VarVersionPair, String>> {
        let mut params = 0;
        if (self.method.get_access_flags() & ACC_STATIC) != ACC_STATIC {
            params += 1;
        }

        let md = MethodDescriptor::parse_descriptor(&self.method.get_descriptor());
        for param in md.params.iter() {
            params += param.stack_size;
        }

        let keys = sorted_keys(entries);

        let mut result: JHashMap<VarVersionPair, String> = JHashMap::new();
        for ver in keys {
            let entry = entries.get(&ver).expect("NullPointerException: entry");
            let type_ =
                self.clean_type(entry.b.as_deref().expect("NullPointerException: type name"));

            let renamed = if ver.var >= params {
                self.get_new_name(Pair::of(entry.a.clone(), type_))
            } else if self.rename_parameters {
                self.compute_parameter(ver.var, || {
                    self.get_new_name(Pair::of(entry.a.clone(), type_))
                })
            } else {
                continue;
            };
            if let Some(renamed) = renamed {
                result.put(ver, renamed);
            }
        }

        Some(result)
    }

    fn rename_parameter(
        &self,
        flags: i32,
        type_: &VarType,
        name: Option<&str>,
        index: i32,
    ) -> Option<String> {
        let type_name = {
            let _lock = DecompilerContext::get_import_collector().lock();
            ExprProcessor::get_cast_type_name(type_)
        };
        if !self.rename_parameters {
            return super_rename_parameter(self, flags, name, index);
        }

        self.compute_parameter(index, || {
            self.get_new_name(Pair::of(Some(type_.clone()), self.clean_type(&type_name)))
        })
    }

    fn add_parent_context(&self, renamer: Rc<dyn IVariableNameProvider>) {
        let renamer: Rc<dyn Any> = renamer;
        let Ok(prov) = renamer.downcast::<TinyNameProvider>() else {
            jthrow!(ClassCastException; "class cannot be cast to class org.vineflower.variablerenaming.TinyNameProvider");
        };

        let names = prov.used_names.borrow().to_vec();
        self.used_names.borrow_mut().add_all(names);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TinyNameProviderFactory;

impl IVariableNamingFactory for TinyNameProviderFactory {
    fn create_factory(&self, struct_method: StructMethod) -> Rc<dyn IVariableNameProvider> {
        Rc::new(TinyNameProvider::new(
            DecompilerContext::get_option(VariableRenamingOptions::RENAME_PARAMETERS),
            struct_method,
        ))
    }
}
