//! Port of `org/vineflower/kt/metadata/deserialization/Flags.java` (shaded Kotlin metadata).

use std::marker::PhantomData;

use crate::kt_proto::{ClassKind, MemberKind, Modality, ReturnValueStatus, Visibility};

/// A `ProtoBuf` enum stored in a flag field.
pub trait FlagEnum: Copy + 'static {
    const VALUES: &'static [Self];
    fn get_number(self) -> i32;
}

macro_rules! flag_enum {
    ($($name:ident),*) => {
        $(impl FlagEnum for $name {
            const VALUES: &'static [$name] = $name::VALUES;
            fn get_number(self) -> i32 {
                $name::get_number(self)
            }
        })*
    };
}

flag_enum!(
    Visibility,
    Modality,
    ClassKind,
    MemberKind,
    ReturnValueStatus
);

/// `Flags.FlagField`: a bit range of a flags word.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FlagField {
    pub offset: i32,
    pub bit_width: i32,
}

/// `Flags.BooleanFlagField`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BooleanFlagField {
    pub offset: i32,
}

impl BooleanFlagField {
    pub const fn boolean_first() -> BooleanFlagField {
        BooleanFlagField { offset: 0 }
    }

    pub const fn boolean_after(previous: FlagField) -> BooleanFlagField {
        BooleanFlagField {
            offset: previous.offset + previous.bit_width,
        }
    }

    pub const fn field(self) -> FlagField {
        FlagField {
            offset: self.offset,
            bit_width: 1,
        }
    }

    pub fn get(self, flags: i32) -> bool {
        flags & 1i32.wrapping_shl(self.offset as u32) != 0
    }

    pub fn to_flags(self, value: bool) -> i32 {
        if value {
            1i32.wrapping_shl(self.offset as u32)
        } else {
            0
        }
    }
}

/// `Flags.EnumLiteFlagField`.
#[derive(Debug)]
pub struct EnumLiteFlagField<E> {
    pub offset: i32,
    pub bit_width: i32,
    values: PhantomData<E>,
}

impl<E> Clone for EnumLiteFlagField<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for EnumLiteFlagField<E> {}

const fn bit_width(value_count: usize) -> i32 {
    let max = value_count as i32 - 1;
    if max == 0 {
        return 1;
    }
    let mut i = 31;
    while i >= 0 {
        if max & (1 << i) != 0 {
            return i + 1;
        }
        i -= 1;
    }
    panic!("empty enum")
}

impl<E: FlagEnum> EnumLiteFlagField<E> {
    /// `FlagField.after(previous, E.values())`.
    pub const fn after(previous: FlagField) -> EnumLiteFlagField<E> {
        EnumLiteFlagField {
            offset: previous.offset + previous.bit_width,
            bit_width: bit_width(E::VALUES.len()),
            values: PhantomData,
        }
    }

    pub const fn field(self) -> FlagField {
        FlagField {
            offset: self.offset,
            bit_width: self.bit_width,
        }
    }

    /// The constant whose number the bits hold, or `None` (Java `null`).
    pub fn get(self, flags: i32) -> Option<E> {
        let max_value = 1i32.wrapping_shl(self.bit_width as u32).wrapping_sub(1);
        let mask = max_value.wrapping_shl(self.offset as u32);
        let value = (flags & mask) >> self.offset;
        E::VALUES.iter().copied().find(|v| v.get_number() == value)
    }

    pub fn to_flags(self, value: E) -> i32 {
        value.get_number().wrapping_shl(self.offset as u32)
    }
}

/// `Flags`: the flag fields of every metadata flags word.
pub struct Flags;

impl Flags {
    pub const SUSPEND_TYPE: BooleanFlagField = BooleanFlagField::boolean_first();
    pub const DEFINITELY_NOT_NULL_TYPE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::SUSPEND_TYPE.field());

    pub const HAS_ANNOTATIONS: BooleanFlagField = BooleanFlagField::boolean_first();
    pub const VISIBILITY: EnumLiteFlagField<Visibility> =
        EnumLiteFlagField::after(Self::HAS_ANNOTATIONS.field());
    pub const MODALITY: EnumLiteFlagField<Modality> =
        EnumLiteFlagField::after(Self::VISIBILITY.field());

    pub const CLASS_KIND: EnumLiteFlagField<ClassKind> =
        EnumLiteFlagField::after(Self::MODALITY.field());
    pub const IS_INNER: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::CLASS_KIND.field());
    pub const IS_DATA: BooleanFlagField = BooleanFlagField::boolean_after(Self::IS_INNER.field());
    pub const IS_EXTERNAL_CLASS: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_DATA.field());
    pub const IS_EXPECT_CLASS: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXTERNAL_CLASS.field());
    pub const IS_VALUE_CLASS: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXPECT_CLASS.field());
    pub const IS_FUN_INTERFACE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_VALUE_CLASS.field());
    pub const HAS_ENUM_ENTRIES: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_FUN_INTERFACE.field());

    pub const IS_SECONDARY: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::VISIBILITY.field());
    pub const IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_SECONDARY.field());
    pub const RETURN_VALUE_STATUS_CTOR: EnumLiteFlagField<ReturnValueStatus> =
        EnumLiteFlagField::after(Self::IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES.field());

    pub const MEMBER_KIND: EnumLiteFlagField<MemberKind> =
        EnumLiteFlagField::after(Self::MODALITY.field());

    pub const IS_OPERATOR: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::MEMBER_KIND.field());
    pub const IS_INFIX: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_OPERATOR.field());
    pub const IS_INLINE: BooleanFlagField = BooleanFlagField::boolean_after(Self::IS_INFIX.field());
    pub const IS_TAILREC: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_INLINE.field());
    pub const IS_EXTERNAL_FUNCTION: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_TAILREC.field());
    pub const IS_SUSPEND: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXTERNAL_FUNCTION.field());
    pub const IS_EXPECT_FUNCTION: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_SUSPEND.field());
    pub const IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXPECT_FUNCTION.field());
    pub const RETURN_VALUE_STATUS_FUNCTION: EnumLiteFlagField<ReturnValueStatus> =
        EnumLiteFlagField::after(Self::IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES.field());

    pub const IS_VAR: BooleanFlagField = BooleanFlagField::boolean_after(Self::MEMBER_KIND.field());
    pub const HAS_GETTER: BooleanFlagField = BooleanFlagField::boolean_after(Self::IS_VAR.field());
    pub const HAS_SETTER: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::HAS_GETTER.field());
    pub const IS_CONST: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::HAS_SETTER.field());
    pub const IS_LATEINIT: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_CONST.field());
    pub const HAS_CONSTANT: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_LATEINIT.field());
    pub const IS_EXTERNAL_PROPERTY: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::HAS_CONSTANT.field());
    pub const IS_DELEGATED: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXTERNAL_PROPERTY.field());
    pub const IS_EXPECT_PROPERTY: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_DELEGATED.field());
    pub const RETURN_VALUE_STATUS_PROPERTY: EnumLiteFlagField<ReturnValueStatus> =
        EnumLiteFlagField::after(Self::IS_EXPECT_PROPERTY.field());

    pub const DECLARES_DEFAULT_VALUE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::HAS_ANNOTATIONS.field());
    pub const IS_CROSSINLINE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::DECLARES_DEFAULT_VALUE.field());
    pub const IS_NOINLINE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_CROSSINLINE.field());

    pub const IS_NOT_DEFAULT: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::MODALITY.field());
    pub const IS_EXTERNAL_ACCESSOR: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_NOT_DEFAULT.field());
    pub const IS_INLINE_ACCESSOR: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_EXTERNAL_ACCESSOR.field());

    pub const IS_NEGATED: BooleanFlagField = BooleanFlagField::boolean_first();
    pub const IS_NULL_CHECK_PREDICATE: BooleanFlagField =
        BooleanFlagField::boolean_after(Self::IS_NEGATED.field());

    pub const IS_UNSIGNED: BooleanFlagField = BooleanFlagField::boolean_first();

    pub fn get_accessor_flags(
        has_annotations: bool,
        visibility: Visibility,
        modality: Modality,
        is_not_default: bool,
        is_external: bool,
        is_inline_accessor: bool,
    ) -> i32 {
        Self::HAS_ANNOTATIONS.to_flags(has_annotations)
            | Self::MODALITY.to_flags(modality)
            | Self::VISIBILITY.to_flags(visibility)
            | Self::IS_NOT_DEFAULT.to_flags(is_not_default)
            | Self::IS_EXTERNAL_ACCESSOR.to_flags(is_external)
            | Self::IS_INLINE_ACCESSOR.to_flags(is_inline_accessor)
    }
}
