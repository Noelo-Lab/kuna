//! Port of `org/vineflower/kotlin/struct/KType.java`.
//!
//! `KType extends VarType` without overriding anything, so, like
//! `GenericType`, it is one Rust type `VarType` whose data carries `kotlin`
//! (present exactly for a `KType`), and `KType` is a view over it.

use std::ops::Deref;
use std::rc::Rc;

use crate::code_type::CodeType;
use crate::kt_proto::{self, Projection};
use crate::metadata_name_resolver::MetadataNameResolver;
use crate::text_buffer::TextBuffer;
use crate::unported;
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
    pub fn from(_type: &kt_proto::Type, _name_resolver: &MetadataNameResolver) -> KType {
        unported!("KType.from")
    }

    /// Java: from(int, MetadataNameResolver, ProtoBuf.TypeTable)
    pub fn from_table(
        _table_index: i32,
        _resolver: &MetadataNameResolver,
        _type_table: &kt_proto::TypeTable,
    ) -> KType {
        unported!("KType.from")
    }

    pub fn stringify(&self, _indent: i32) -> TextBuffer {
        unported!("KType.stringify")
    }
}

impl TypeArgument {
    pub fn new(type_: KType, type_projection: Projection) -> TypeArgument {
        TypeArgument {
            type_,
            type_projection,
        }
    }

    pub fn stringify(&self, _indent: i32) -> TextBuffer {
        unported!("KType.TypeArgument.stringify")
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
