//! Port of `org/jetbrains/java/decompiler/struct/StructClass.java`.
//!
//! kuna reads a class's fields and methods when they are first asked for,
//! unless a pool interceptor (renaming) is set: construction only steps over
//! them (failing where Java's reading would on a truncated class).

use std::rc::Rc;

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::decompiler_context::DecompilerContext;
use crate::generic_class_descriptor::GenericClassDescriptor;
use crate::generic_main::GenericMain;
use crate::generic_type::GenericType;
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::java_subclass;
use crate::primitive_constant::PrimitiveConstant;
use crate::rt::{JHashMap, JHashSet, JavaEq, JavaHash};
use crate::struct_field::StructField;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_member::{AttributeMap, StructMember, StructMemberKind};
use crate::struct_method::StructMethod;
use crate::struct_record_component::StructRecordComponent;
use crate::var_type::VarType;
use crate::vb_style_collection::VBStyleCollection;

/// `Map<VarType, VarType>` as `getAllGenerics` shares it between classes.
pub type GenericMap = Rc<JHashMap<VarType, VarType>>;

pub struct StructClassData {
    pub qualified_name: String,
    pub super_class: Option<PrimitiveConstant>,
    pub own: bool,
    pub version: BytecodeVersion,
    pub interfaces: Vec<i32>,
    pub interface_names: Vec<String>,
    pub fields: Option<Rc<VBStyleCollection<StructField, String>>>,
    pub methods: Option<Rc<VBStyleCollection<StructMethod, String>>>,
    pub signature: Option<GenericClassDescriptor>,
    pub pool: Option<ConstantPool>,
    pub generic_hiarachy: Option<Rc<JHashMap<String, GenericMap>>>,
    /// Where the fields begin, while they and the methods are unread.
    members: Option<ByteCursor>,
}

type Fields = VBStyleCollection<StructField, String>;
type Methods = VBStyleCollection<StructMethod, String>;

java_subclass!(pub StructClass: StructMember, StructMemberKind::Class(StructClassData), is_class, as_class, try_as_class);

impl JavaEq for StructClass {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for StructClass {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl StructClass {
    pub fn create(in_: &mut ByteCursor, own: bool) -> StructClass {
        in_.discard(4);
        let minor_version = in_.read_unsigned_short();
        let major_version = in_.read_unsigned_short();
        let bytecode_version = BytecodeVersion::new(major_version, minor_version);

        let pool = ConstantPool::new(in_);

        let access_flags = in_.read_unsigned_short();
        let this_class_idx = in_.read_unsigned_short();
        let super_class_idx = in_.read_unsigned_short();
        let qualified_name = pool
            .get_primitive_constant(this_class_idx)
            .expect("this_class")
            .get_string()
            .to_string();
        let super_class = pool.get_primitive_constant(super_class_idx);

        let length = in_.read_unsigned_short();
        let mut interfaces = vec![0; length as usize];
        let mut interface_names = vec![String::new(); length as usize];
        for i in 0..length as usize {
            interfaces[i] = in_.read_unsigned_short();
            interface_names[i] = pool
                .get_primitive_constant(interfaces[i])
                .expect("interface")
                .get_string()
                .to_string();
        }

        let members_at = in_.position();
        let members = if DecompilerContext::get_pool_interceptor().is_none() {
            Self::skip_members(in_);
            None
        } else {
            Some(Self::read_members(
                in_,
                pool,
                &qualified_name,
                bytecode_version,
                own,
            ))
        };
        let lazy = members.is_none();

        let attributes = StructMember::read_attributes(in_, pool, bytecode_version);

        let mut signature = None;
        if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES) {
            let signature_attr = attributes
                .get_cloned(&StructGeneralAttribute::ATTRIBUTE_SIGNATURE.name.to_string())
                .map(|a| a.as_generic_signature());
            if let Some(signature_attr) = signature_attr {
                signature = GenericMain::parse_class_signature(
                    &qualified_name,
                    &signature_attr.get_signature(),
                );
            }
        }

        let cl = StructClass::new(
            access_flags,
            attributes,
            qualified_name,
            super_class,
            own,
            minor_version,
            major_version,
            interfaces,
            interface_names,
            members,
            signature,
        );
        cl.d_mut(|d| {
            d.pool = Some(pool);
            if lazy {
                d.members = Some(in_.share_at(members_at));
            }
        });
        cl
    }

    fn read_members(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        qualified_name: &str,
        bytecode_version: BytecodeVersion,
        own: bool,
    ) -> (Fields, Methods) {
        let mut length = in_.read_unsigned_short();
        let mut fields: Fields = VBStyleCollection::with_capacity(length);
        for _ in 0..length {
            let field = StructField::create(in_, pool, qualified_name, bytecode_version);
            let key = InterpreterUtil::make_unique_key(&field.get_name(), &field.get_descriptor());
            fields.add_with_key(field, key);
        }

        length = in_.read_unsigned_short();
        let mut methods: Methods = VBStyleCollection::with_capacity(length);
        for _ in 0..length {
            let method = StructMethod::create(in_, pool, qualified_name, bytecode_version, own);
            let key =
                InterpreterUtil::make_unique_key(&method.get_name(), &method.get_descriptor());
            if methods.contains_key(&key) {
                let full_name = format!(
                    "{}.{}{}",
                    qualified_name,
                    method.get_name(),
                    method.get_descriptor()
                );
                DecompilerContext::get_logger()
                    .write_message(&format!("Duplicate method {}", full_name), Severity::Warn);
            }
            methods.add_with_key(method, key);
        }
        (fields, methods)
    }

    /// Steps over the fields and methods as reading them would.
    fn skip_members(in_: &mut ByteCursor) {
        for _ in 0..2 {
            let count = in_.read_unsigned_short();
            for _ in 0..count {
                in_.read_unsigned_short();
                in_.read_unsigned_short();
                in_.read_unsigned_short();
                let attributes = in_.read_unsigned_short();
                for _ in 0..attributes {
                    in_.read_unsigned_short();
                    let length = in_.read_int();
                    in_.discard(length as usize);
                }
            }
        }
    }

    /// Reads the fields and methods of a class built without them.
    fn read_lazy_members(self) {
        let Some(mut in_) = self.d(|d| d.members.as_ref().map(|c| c.share_at(c.position()))) else {
            return;
        };
        let (pool, qualified_name, version, own) = self.d(|d| {
            (
                d.pool.expect("pool"),
                d.qualified_name.clone(),
                d.version,
                d.own,
            )
        });
        let (fields, methods) = Self::read_members(&mut in_, pool, &qualified_name, version, own);
        self.d_mut(|d| {
            d.fields = Some(Rc::new(fields));
            d.methods = Some(Rc::new(methods));
            d.members = None;
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        access_flags: i32,
        attributes: AttributeMap,
        qualified_name: String,
        super_class: Option<PrimitiveConstant>,
        own: bool,
        minor_version: i32,
        major_version: i32,
        interfaces: Vec<i32>,
        interface_names: Vec<String>,
        members: Option<(Fields, Methods)>,
        signature: Option<GenericClassDescriptor>,
    ) -> StructClass {
        let (fields, methods) = match members {
            Some((fields, methods)) => (Some(Rc::new(fields)), Some(Rc::new(methods))),
            None => (None, None),
        };
        StructClass(StructMember::new_member(
            access_flags,
            attributes,
            StructMemberKind::Class(StructClassData {
                qualified_name,
                super_class,
                own,
                version: BytecodeVersion::new(major_version, minor_version),
                interfaces,
                interface_names,
                fields,
                methods,
                signature,
                pool: None,
                generic_hiarachy: None,
                members: None,
            }),
        ))
    }

    /// The public field `qualifiedName`.
    pub fn qualified_name(self) -> String {
        self.d(|d| d.qualified_name.clone())
    }

    /// The public field `superClass`.
    pub fn super_class(self) -> Option<PrimitiveConstant> {
        self.d(|d| d.super_class.clone())
    }

    pub fn get_version(self) -> BytecodeVersion {
        self.d(|d| d.version)
    }

    pub fn has_field(self, name: &str, descriptor: &str) -> bool {
        self.get_field(name, descriptor).is_some()
    }

    pub fn get_field(self, name: &str, descriptor: &str) -> Option<StructField> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.get_fields().get_with_key(&key)
    }

    /// Java: `getMethod(String key)`.
    pub fn get_method_key(self, key: &str) -> Option<StructMethod> {
        self.get_methods().get_with_key(&key.to_string())
    }

    /// Java: `getMethod(String name, String descriptor)`.
    pub fn get_method(self, name: &str, descriptor: &str) -> Option<StructMethod> {
        let key = InterpreterUtil::make_unique_key(name, descriptor);
        self.get_methods().get_with_key(&key)
    }

    pub fn get_method_recursive(self, name: &str, descriptor: &str) -> Option<StructMethod> {
        let ret = self.get_method(name, descriptor);

        if ret.is_some() {
            return ret;
        }

        if let Some(super_class) = self.super_class() {
            let cls = DecompilerContext::get_struct_context().get_class(&super_class.get_string());
            if let Some(cls) = cls {
                let ret = cls.get_method_recursive(name, descriptor);
                if ret.is_some() {
                    return ret;
                }
            }
        }

        for intf in self.get_interface_names() {
            let cls = DecompilerContext::get_struct_context().get_class(&intf);
            if let Some(cls) = cls {
                let ret = cls.get_method_recursive(name, descriptor);
                if ret.is_some() {
                    return ret;
                }
            }
        }
        None
    }

    pub fn get_interface(self, i: i32) -> String {
        self.d(|d| d.interface_names[i as usize].clone())
    }

    pub fn release_resources(self) {}

    pub fn get_pool(self) -> ConstantPool {
        self.d(|d| d.pool.expect("pool"))
    }

    /// The record components; `None` when this class is not a record.
    pub fn get_record_components(self) -> Option<Vec<StructRecordComponent>> {
        let record_attr = self.get_attribute(StructGeneralAttribute::ATTRIBUTE_RECORD);
        match record_attr {
            None => {
                if let Some(super_class) = self.super_class() {
                    if &*super_class.get_string() == "java/lang/Record" {
                        return Some(Vec::new());
                    }
                }
                None
            }
            Some(record_attr) => Some(record_attr.get_components()),
        }
    }

    pub fn get_interfaces(self) -> Vec<i32> {
        self.d(|d| d.interfaces.clone())
    }

    pub fn get_interface_names(self) -> Vec<String> {
        self.d(|d| d.interface_names.clone())
    }

    pub fn get_methods(self) -> Rc<VBStyleCollection<StructMethod, String>> {
        if let Some(methods) = self.d(|d| d.methods.clone()) {
            return methods;
        }
        self.read_lazy_members();
        self.d(|d| d.methods.clone()).expect("methods")
    }

    pub fn get_fields(self) -> Rc<VBStyleCollection<StructField, String>> {
        if let Some(fields) = self.d(|d| d.fields.clone()) {
            return fields;
        }
        self.read_lazy_members();
        self.d(|d| d.fields.clone()).expect("fields")
    }

    pub fn is_own(self) -> bool {
        self.d(|d| d.own)
    }

    pub fn get_signature(self) -> Option<GenericClassDescriptor> {
        self.d(|d| d.signature.clone())
    }

    fn get_generic_map(self, type_: Option<&VarType>) -> JHashMap<VarType, VarType> {
        let signature = match (self.get_signature(), type_) {
            (Some(signature), Some(type_)) if type_.is_generic() => signature,
            _ => return JHashMap::new(),
        };
        let type_ = type_.expect("type");
        let gtype: GenericType = type_.as_generic();
        let arguments = gtype.get_arguments();
        let fparameters = signature.fparameters().clone();
        if arguments.len() != fparameters.len() {
            return JHashMap::new();
        }

        let mut ret = JHashMap::new();
        for (parameter, var) in fparameters.iter().zip(arguments.iter()) {
            if let Some(var) = var {
                ret.put(GenericType::parse(&format!("T{};", parameter)), var.clone());
            }
        }
        ret
    }

    pub fn get_all_generics(self) -> Rc<JHashMap<String, GenericMap>> {
        if let Some(cached) = self.d(|d| d.generic_hiarachy.clone()) {
            return cached;
        }

        let signature = self.get_signature();
        let mut ret: JHashMap<String, GenericMap> = JHashMap::new();
        if let Some(signature) = &signature {
            let fparameters = signature.fparameters().clone();
            if !fparameters.is_empty() {
                let mut mine = JHashMap::new();
                for par in fparameters.iter() {
                    let type_ = GenericType::parse(&format!("T{};", par));
                    mine.put(type_.clone(), type_);
                }
                ret.put(self.qualified_name(), Rc::new(mine));
            }
        }

        let mut visited: JHashSet<String> = JHashSet::new();
        if let Some(signature) = &signature {
            let superinterfaces = signature.superinterfaces().clone();
            for intf in superinterfaces.iter() {
                if let Some(value) = intf.value.as_deref() {
                    visited.add(value.to_string());
                }

                let cls = intf
                    .value
                    .as_deref()
                    .and_then(|v| DecompilerContext::get_struct_context().get_class(v));
                if let Some(cls) = cls {
                    let sig = cls.get_generic_map(Some(intf));
                    for (k, v) in cls.get_all_generics().entry_vec() {
                        if v.is_empty() {
                            ret.put(k, v);
                        } else {
                            ret.put(k, Rc::new(substitute(&v, &sig)));
                        }
                    }
                }
            }
        }

        for intf in self.get_interface_names() {
            if visited.contains(&intf) {
                continue;
            }
            let cls = DecompilerContext::get_struct_context().get_class(&intf);
            if let Some(cls) = cls {
                ret.put_all(cls.get_all_generics().entry_vec());
            }
        }

        if let Some(super_class) = self.super_class() {
            let cls = DecompilerContext::get_struct_context().get_class(&super_class.get_string());
            if let Some(cls) = cls {
                let sig = match &signature {
                    None => JHashMap::new(),
                    Some(signature) => cls.get_generic_map(Some(&signature.superclass())),
                };
                if sig.is_empty() {
                    ret.put_all(cls.get_all_generics().entry_vec());
                } else {
                    for (k, v) in cls.get_all_generics().entry_vec() {
                        if v.is_empty() {
                            ret.put(k, v);
                        } else {
                            ret.put(k, Rc::new(substitute(&v, &sig)));
                        }
                    }
                }
            }
        }

        let hierarchy = Rc::new(ret);
        self.d_mut(|d| d.generic_hiarachy = Some(hierarchy.clone()));
        hierarchy
    }
}

fn substitute(
    map: &JHashMap<VarType, VarType>,
    sig: &JHashMap<VarType, VarType>,
) -> JHashMap<VarType, VarType> {
    let mut sub = JHashMap::new();
    for (k, v) in map.entry_vec() {
        let value = sig.get_cloned(&v).unwrap_or(v);
        sub.put(k, value);
    }
    sub
}

impl std::fmt::Display for StructClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.qualified_name())
    }
}
