//! kuna fix `jvm_fix_switch_yield_const_retype` (`kuna_fixes`): the types a
//! constant can be written with. `SwitchExprent::to_java` gives a constant
//! yielded by a single-expression arm the switch expression's type; when the
//! arms have unrelated reference types that type is their common supertype
//! (`Object`, `Serializable`, ...), which no literal has, and Vineflower then
//! fails the method in `ConstExprent.toJava`. With the fix such a constant
//! keeps its own type, a subtype the expression's context widens implicitly.

use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::var_type::VarType;

impl ConstExprent {
    /// Whether `to_java` can write this constant with `type_` as its type:
    /// exactly the types its dispatch does not reject (a primitive or its box,
    /// `null`, `String`, `Class`, `MethodType`; any type without a value).
    pub fn can_be_written_as(self, type_: &VarType) -> bool {
        if type_.type_ != CodeType::Null && self.get_value().is_none() {
            return true;
        }
        let unboxed = VarType::UNBOXING_TYPES().get_or_default(type_, type_.clone());
        match unboxed.type_ {
            CodeType::Boolean
            | CodeType::Char
            | CodeType::Bytechar
            | CodeType::Shortchar
            | CodeType::Byte
            | CodeType::Short
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
            | CodeType::Null => true,
            CodeType::Object => {
                type_.equals(&VarType::VARTYPE_STRING())
                    || type_.equals(&VarType::VARTYPE_CLASS())
                    || type_.equals(&VarType::VARTYPE_METHODTYPE())
            }
            _ => false,
        }
    }
}
