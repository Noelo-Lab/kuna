//! Port of `org/vineflower/variablerenaming/JADNameProvider.java`.

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::i_variable_name_provider::IVariableNameProvider;
use crate::i_variable_naming_factory::IVariableNamingFactory;
use crate::jthrow;
use crate::method_descriptor::MethodDescriptor;
use crate::pair::Pair;
use crate::rt::{JHashMap, JLinkedHashMap};
use crate::struct_method::StructMethod;
use crate::tiny_name_provider::char_to_lower_case;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::variable_renaming_options::VariableRenamingOptions;

pub struct JADNameProvider {
    last: RefCell<JHashMap<String, Holder>>,
    remap: RefCell<JHashMap<String, String>>,
    parameters: RefCell<JHashMap<i32, String>>,
    method: StructMethod,
    rename_parameters: bool,
}

#[derive(Clone, Debug)]
struct Holder {
    id: i32,
    skip_zero: bool,
    names: Vec<String>,
}

impl Holder {
    fn new(t1: i32, skip_zero: bool, names: &[&str]) -> Holder {
        Holder {
            id: t1,
            skip_zero,
            names: names.iter().map(|n| n.to_string()).collect(),
        }
    }

    /// Java: Holder(int, boolean, List)
    fn new_list(t1: i32, skip_zero: bool, names: &[String]) -> Holder {
        Holder {
            id: t1,
            skip_zero,
            names: names.to_vec(),
        }
    }
}

/// `String.toLowerCase(Locale.ENGLISH)`: each character's `Character.toLowerCase` (the JDK 21
/// tables for a `char`), U+0130 to `i` U+0307, and a capital sigma as Rust's final-sigma rule
/// lowers it.
pub fn to_lower_case_english(s: &str) -> String {
    if s.is_ascii() {
        return s.to_ascii_lowercase();
    }
    let full: Vec<char> = s.to_lowercase().chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut j = 0;
    for c in s.chars() {
        let n = c.to_lowercase().count();
        let rust = &full[j..j + n];
        j += n;
        if c == '\u{130}' {
            out.push('i');
            out.push('\u{307}');
        } else if c == '\u{3a3}' {
            out.push(rust[0]);
        } else if (c as u32) <= 0xFFFF {
            out.push(char::from_u32(char_to_lower_case(c as u16) as u32).unwrap_or(c));
        } else if n == 1 {
            out.push(rust[0]);
        } else {
            out.push(c);
        }
    }
    out
}

/// `IVariableNameProvider.super.renameParameter(flags, type, name, index)`.
pub(crate) fn super_rename_parameter(
    this: &dyn IVariableNameProvider,
    flags: i32,
    name: Option<&str>,
    index: i32,
) -> Option<String> {
    use crate::code_constants::{ACC_ABSTRACT, ACC_NATIVE};
    if (flags & (ACC_ABSTRACT | ACC_NATIVE)) != 0 {
        return this.rename_abstract_parameter(name, index);
    }

    name.map(str::to_string)
}

/// The sort of `rename`: by `var`, then by `version`, with Java's `int` subtraction.
pub(crate) fn sorted_keys<V: Clone>(
    entries: &JLinkedHashMap<VarVersionPair, V>,
) -> Vec<VarVersionPair> {
    let mut keys = entries.key_vec();
    keys.sort_by(|o1, o2| {
        let d = if o1.var != o2.var {
            o1.var.wrapping_sub(o2.var)
        } else {
            o1.version.wrapping_sub(o2.version)
        };
        d.cmp(&0)
    });
    keys
}

impl JADNameProvider {
    pub fn new(rename_parameters: bool, wrapper: StructMethod) -> JADNameProvider {
        let mut last: JHashMap<String, Holder> = JHashMap::new();
        last.put(
            "int".to_string(),
            Holder::new(0, true, &["i", "j", "k", "l"]),
        );
        last.put("byte".to_string(), Holder::new(0, false, &["b"]));
        last.put("char".to_string(), Holder::new(0, false, &["c"]));
        last.put("short".to_string(), Holder::new(1, false, &["short"]));
        last.put("boolean".to_string(), Holder::new(0, true, &["flag"]));
        last.put("double".to_string(), Holder::new(0, false, &["d"]));
        last.put("float".to_string(), Holder::new(0, true, &["f"]));
        last.put("File".to_string(), Holder::new(1, true, &["file"]));
        last.put("String".to_string(), Holder::new(0, true, &["s"]));
        last.put("Class".to_string(), Holder::new(0, true, &["oclass"]));
        last.put("Long".to_string(), Holder::new(0, true, &["olong"]));
        last.put("Byte".to_string(), Holder::new(0, true, &["obyte"]));
        last.put("Short".to_string(), Holder::new(0, true, &["oshort"]));
        last.put("Boolean".to_string(), Holder::new(0, true, &["obool"]));
        last.put("Package".to_string(), Holder::new(0, true, &["opackage"]));
        last.put("Enum".to_string(), Holder::new(0, true, &["oenum"]));

        let mut remap: JHashMap<String, String> = JHashMap::new();
        remap.put("long".to_string(), "int".to_string());

        JADNameProvider {
            last: RefCell::new(last),
            remap: RefCell::new(remap),
            parameters: RefCell::new(JHashMap::new()),
            method: wrapper,
            rename_parameters,
        }
    }

    fn clean_type(&self, type_: &str) -> String {
        let mut type_ = type_.to_string();
        if let Some(i) = type_.find('<') {
            type_ = type_[..i].to_string();
        }
        if type_.contains('.') {
            let i = type_.rfind('.').unwrap();
            type_ = type_[i + 1..].to_string();
        }
        type_
    }

    /// `parameters.computeIfAbsent(key, k -> make())`.
    fn compute_parameter(&self, key: i32, make: impl FnOnce() -> String) -> String {
        if let Some(name) = self.parameters.borrow().get_cloned(&key) {
            return name;
        }
        let name = make();
        self.parameters.borrow_mut().put(key, name.clone());
        name
    }

    pub fn get_new_name(&self, type_: &str) -> String {
        let mut type_ = type_.to_string();
        let mut index: Option<String> = None;
        let mut findtype = type_.clone();

        while findtype.contains("[][]") {
            findtype = findtype.replace("[][]", "[]");
        }
        let lower_findtype = to_lower_case_english(&findtype);
        if self.last.borrow().contains_key(&findtype) {
            index = Some(findtype);
        } else if self.last.borrow().contains_key(&lower_findtype) {
            index = Some(lower_findtype);
        } else if self.remap.borrow().contains_key(&type_) {
            index = self.remap.borrow().get_cloned(&type_);
        }

        if index.as_deref().is_none_or(str::is_empty)
            && (type_.as_bytes().first().is_some_and(u8::is_ascii_uppercase)
                || type_.contains('[')
                || type_.contains("..."))
        {
            type_ = type_.replace("...", "[]");

            while type_.contains("[][]") {
                type_ = type_.replace("[][]", "[]");
            }

            let mut name = to_lower_case_english(&type_);
            name = name.replace('.', "");
            let mut skip_zero = true;

            if type_.contains('[') {
                skip_zero = true;
                name = format!("a{}", name.replace("[]", "").replace("...", ""));
            }

            self.last.borrow_mut().put(
                to_lower_case_english(&type_),
                Holder::new_list(0, skip_zero, &[name]),
            );
            index = Some(to_lower_case_english(&type_));
        }

        let Some(index) = index.filter(|index| !index.is_empty()) else {
            return to_lower_case_english(&type_);
        };

        let mut last = self.last.borrow_mut();
        let holder = last.get_mut(&index).expect("NullPointerException: holder");
        let id = holder.id;
        let names = &holder.names;

        let ammount = names.len() as i32;

        let name = if ammount == 1 {
            let suffix = if id == 0 && holder.skip_zero {
                String::new()
            } else {
                id.to_string()
            };
            format!("{}{}", names[0], suffix)
        } else {
            let num = id / ammount;
            let suffix = if id < ammount && holder.skip_zero {
                String::new()
            } else {
                num.to_string()
            };
            format!("{}{}", names[(id % ammount) as usize], suffix)
        };

        holder.id += 1;
        name
    }
}

impl IVariableNameProvider for JADNameProvider {
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
            let cast = entries
                .get(&ver)
                .and_then(|p| p.b.clone())
                .expect("NullPointerException: type name");
            let type_ = self.clean_type(&cast);
            if "this" == type_ {
                continue;
            }
            if ver.var >= params {
                result.put(ver, self.get_new_name(&type_));
            } else if self.rename_parameters {
                let name = self.compute_parameter(ver.var, || self.get_new_name(&type_));
                result.put(ver, name);
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
        let type_name = ExprProcessor::get_cast_type_name(type_);
        if !self.rename_parameters {
            return super_rename_parameter(self, flags, name, index);
        }

        Some(self.compute_parameter(index, || self.get_new_name(&self.clean_type(&type_name))))
    }

    fn add_parent_context(&self, iparent: Rc<dyn IVariableNameProvider>) {
        let iparent: Rc<dyn Any> = iparent;
        let Ok(parent) = iparent.downcast::<JADNameProvider>() else {
            jthrow!(ClassCastException; "class cannot be cast to class org.vineflower.variablerenaming.JADNameProvider");
        };
        let entries = parent.last.borrow().entry_vec();
        let mut temp: JHashMap<String, Holder> = JHashMap::new();
        for (key, v) in entries {
            temp.put(key, Holder::new_list(v.id, v.skip_zero, &v.names));
        }
        *self.last.borrow_mut() = temp;
        let remap = parent.remap.borrow().clone();
        *self.remap.borrow_mut() = remap;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct JADNameProviderFactory;

impl IVariableNamingFactory for JADNameProviderFactory {
    fn create_factory(&self, method: StructMethod) -> Rc<dyn IVariableNameProvider> {
        Rc::new(JADNameProvider::new(
            DecompilerContext::get_option(VariableRenamingOptions::RENAME_PARAMETERS)
                || DecompilerContext::get_option(VariableRenamingOptions::USE_JAD_PARAMETER_NAMING),
            method,
        ))
    }
}
