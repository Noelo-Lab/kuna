//! Port of `org/vineflower/kotlin/struct/KType.java`.
//!
//! `KType extends VarType` without overriding anything, so, like
//! `GenericType`, it is one Rust type `VarType` whose data carries `kotlin`
//! (present exactly for a `KType`), and `KType` is a view over it.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::jthrow;
use crate::k_types::KTypes;
use crate::kt_proto::{self, Projection};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_type::{VarType, VarTypeData};

/// The fields `KType` adds to `VarType`.
pub struct KTypeData {
    pub kotlin_type: String,
    pub is_nullable: bool,
    pub type_arguments: Option<Rc<[TypeArgument]>>,
    pub is_unsigned_type: bool,
    pub type_parameter_name: Option<String>,
    pub type_alias_name: Option<String>,
}

/// `KType.TypeArgument`.
#[derive(Clone)]
pub struct TypeArgument {
    pub type_: KType,
    pub type_projection: Projection,
}

#[derive(Clone)]
pub struct KType(pub VarType);

impl Deref for KType {
    type Target = VarType;
    fn deref(&self) -> &VarType {
        &self.0
    }
}

impl From<KType> for VarType {
    fn from(t: KType) -> VarType {
        t.0
    }
}

thread_local! {
    static UNIT: KType = KType::new(&VarType::VARTYPE_VOID(), "kotlin/Unit", false, false, None, None, None);
    static NOTHING: KType = KType::new(&VarType::new_signature_cl("java/lang/Void", true), "kotlin/Nothing", false, false, None, None, None);
    static UBYTE: KType = KType::new(&VarType::VARTYPE_BYTE(), "kotlin/UByte", false, true, None, None, None);
    static USHORT: KType = KType::new(&VarType::VARTYPE_SHORT(), "kotlin/UShort", false, true, None, None, None);
    static UINT: KType = KType::new(&VarType::VARTYPE_INT(), "kotlin/UInt", false, true, None, None, None);
    static ULONG: KType = KType::new(&VarType::VARTYPE_LONG(), "kotlin/ULong", false, true, None, None, None);
}

#[allow(non_snake_case)]
impl KType {
    pub fn UNIT() -> KType {
        UNIT.with(KType::clone)
    }

    pub fn NOTHING() -> KType {
        NOTHING.with(KType::clone)
    }

    pub fn UBYTE() -> KType {
        UBYTE.with(KType::clone)
    }

    pub fn USHORT() -> KType {
        USHORT.with(KType::clone)
    }

    pub fn UINT() -> KType {
        UINT.with(KType::clone)
    }

    pub fn ULONG() -> KType {
        ULONG.with(KType::clone)
    }
}

impl KType {
    /// The private constructor: `type`'s `VarType` fields plus the Kotlin ones.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        type_: &VarType,
        kotlin_type: &str,
        is_nullable: bool,
        is_unsigned_type: bool,
        type_arguments: Option<Rc<[TypeArgument]>>,
        type_parameter_name: Option<String>,
        type_alias_name: Option<String>,
    ) -> KType {
        ValidationHelper::assert_true(true, "Type and type family must not be null");
        ValidationHelper::assert_true(
            type_.type_ == CodeType::Null || type_.value.is_some(),
            "Must not be null for non null type",
        );
        KType(VarType::from_data(VarTypeData {
            type_: type_.type_,
            array_dim: type_.array_dim,
            value: type_.value.clone(),
            type_family: type_.type_family,
            stack_size: type_.stack_size,
            generic: None,
            kotlin: Some(Rc::new(KTypeData {
                kotlin_type: kotlin_type.to_string(),
                is_nullable,
                type_arguments,
                is_unsigned_type,
                type_parameter_name,
                type_alias_name,
            })),
        }))
    }

    /// The Kotlin fields.
    pub fn k(&self) -> &KTypeData {
        self.0.kotlin.as_deref().expect("a KType")
    }

    /// Java: from(ProtoBuf.Type, MetadataNameResolver)
    pub fn from(type_: &kt_proto::Type, name_resolver: &MetadataNameResolver) -> KType {
        let kotlin_type = if type_.has_class_name() {
            name_resolver.resolve(type_.get_class_name())
        } else {
            "kotlin/Any".to_string()
        };
        let is_nullable = type_.get_nullable();

        if !is_nullable {
            let existing = match kotlin_type.as_str() {
                "kotlin/Unit" => Some(KType::UNIT()),
                "kotlin/Nothing" => Some(KType::NOTHING()),
                "kotlin/UByte" => Some(KType::UBYTE()),
                "kotlin/UShort" => Some(KType::USHORT()),
                "kotlin/UInt" => Some(KType::UINT()),
                "kotlin/ULong" => Some(KType::ULONG()),
                _ => None,
            };
            if let Some(existing) = existing {
                return existing;
            }
        }

        let mut type_arguments: Option<Rc<[TypeArgument]>> = None;
        if type_.get_argument_count() > 0 {
            let mut arguments = Vec::with_capacity(type_.get_argument_count() as usize);
            for i in 0..type_.get_argument_count() {
                let argument = type_.get_argument(i);
                arguments.push(TypeArgument::new(
                    KType::from(&argument.get_type(), name_resolver),
                    argument.get_projection(),
                ));
            }
            type_arguments = Some(Rc::from(arguments));
        }

        let mut jvm_desc = KTypes::get_java_signature(&kotlin_type, is_nullable);
        if kotlin_type == "kotlin/Array" {
            let Some(arguments) = &type_arguments else {
                jthrow!(NullPointerException; "typeArguments is null")
            };
            let array_type = &arguments[0];
            if array_type.type_projection == Projection::In {
                jvm_desc = "[Ljava/lang/Object;".to_string();
            } else {
                jvm_desc = format!("[{}", array_type.type_.0);
            }
        }
        let var_type = VarType::new_signature(&jvm_desc);

        let type_parameter_name = if type_.has_type_parameter_name() {
            Some(name_resolver.resolve(type_.get_type_parameter_name()))
        } else {
            None
        };
        let type_alias_name = if type_.has_type_alias_name() {
            Some(name_resolver.resolve(type_.get_type_alias_name()))
        } else {
            None
        };

        KType::new(
            &var_type,
            &kotlin_type,
            is_nullable,
            false,
            type_arguments,
            type_parameter_name,
            type_alias_name,
        )
    }

    /// Java: from(int, MetadataNameResolver, ProtoBuf.TypeTable)
    pub fn from_table(
        table_index: i32,
        resolver: &MetadataNameResolver,
        type_table: &kt_proto::TypeTable,
    ) -> KType {
        KType::from(&type_table.get_type(table_index), resolver)
    }

    pub fn stringify(&self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let k = self.k();

        if let Some(type_parameter_name) = &k.type_parameter_name {
            buf.append(type_parameter_name);
            if k.is_nullable {
                buf.append("?");
            }

            return buf;
        }

        if !k.kotlin_type.starts_with("kotlin/Function") {
            let type_ = k.kotlin_type.replace('/', ".");
            let short_name = DecompilerContext::get_import_collector().get_short_name(&type_);
            buf.append(short_name.as_deref().unwrap_or("null"));

            if let Some(type_arguments) = &k.type_arguments {
                buf.append("<");
                buf.push_newline_group(indent, 1);
                buf.append_possible_newline();
                let mut first = true;
                for type_argument in type_arguments.iter() {
                    if !first {
                        buf.append(",").append_possible_newline_alt(" ");
                    }
                    buf.append_buffer(&type_argument.stringify(indent + 1));
                    first = false;
                }
                buf.append_possible_newline_dedent("", true);
                buf.append(">");
                buf.pop_newline_group();
            }

            if k.is_nullable {
                buf.append("?");
            }
        } else {
            buf.append(if k.is_nullable { "((" } else { "(" });
            buf.push_newline_group(indent, 1);
            buf.append_possible_newline();
            let Some(type_arguments) = &k.type_arguments else {
                jthrow!(NullPointerException; "typeArguments is null")
            };
            let length = type_arguments.len() as i32;
            for i in 0..length - 1 {
                if i != 0 {
                    buf.append(",").append_possible_newline_alt(" ");
                }
                buf.append_buffer(&type_arguments[i as usize].stringify(indent + 1));
            }
            buf.append_possible_newline_dedent("", true);
            buf.pop_newline_group();
            buf.append(") -> ");
            if length == 0 {
                jthrow!(ArrayIndexOutOfBoundsException; "Index -1 out of bounds for length 0");
            }
            buf.append_buffer(&type_arguments[(length - 1) as usize].stringify(indent + 1));
            if k.is_nullable {
                buf.append(")?");
            }
        }

        buf
    }
}

impl TypeArgument {
    pub fn new(type_: KType, type_projection: Projection) -> TypeArgument {
        TypeArgument {
            type_,
            type_projection,
        }
    }

    pub fn stringify(&self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        match self.type_projection {
            Projection::In => {
                buf.append("in ");
            }
            Projection::Out => {
                buf.append("out ");
            }
            Projection::Star => {
                buf.append("*");
                return buf;
            }
            Projection::Inv => {}
        }
        buf.append_buffer(&self.type_.stringify(indent));
        buf
    }
}

impl VarType {
    /// Java `x instanceof KType`.
    pub fn is_k_type(&self) -> bool {
        self.kotlin.is_some()
    }

    /// Java `x instanceof KType k`.
    pub fn try_as_k_type(&self) -> Option<KType> {
        self.kotlin.is_some().then(|| KType(self.clone()))
    }

    /// Java `(KType) x`.
    #[track_caller]
    pub fn as_k_type(&self) -> KType {
        if self.kotlin.is_none() {
            crate::rt::throw("ClassCastException", "VarType cannot be cast to KType");
        }
        KType(self.clone())
    }
}
