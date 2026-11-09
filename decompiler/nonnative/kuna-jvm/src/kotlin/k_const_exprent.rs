//! Port of `org/vineflower/kotlin/expr/KConstExprent.java`.
//!
//! kuna fix `kt_when_case_type`: the character form feed is written
//! `'\u000c'`, as Kotlin has no `\f` escape (Java's, which `ConstExprent`
//! writes); the fix writes `when` labels as characters, the first place it
//! reaches Kotlin output.

use crate::const_exprent::ConstExprent;
use crate::expr_util::ExprUtil;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_type::KType;
use crate::k_types::KTypes;
use crate::kotlin_exprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

kotlin_exprent!(pub KConstExprent: ConstExprent, Const,
                is_k_const, as_k_const, try_as_k_const);

impl KConstExprent {
    /// Java: KConstExprent(ConstExprent)
    pub fn new(exprent: ConstExprent) -> KConstExprent {
        let bytecode = exprent.get_bytecode();
        let constant = ConstExprent::new_full(
            exprent.get_const_type(),
            exprent.get_value(),
            exprent.is_bool_permitted(),
            bytecode.as_ref(),
        );
        constant.0.set_kt(Some(KExprentExt::Const));
        KConstExprent(constant)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let Some(value) = self.get_value() else {
            if self.get_const_type().equals(&VarType::VARTYPE_NULL()) {
                return ConstExprent::to_java(self.0, indent);
            }

            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            buf.append(&KTypes::get_kotlin_type(Some(&self.get_const_type())));
            return buf;
        };

        let const_type = self.get_const_type();
        if const_type.equals(&VarType::VARTYPE_CLASS()) {
            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());

            let value = value.java_to_string();
            let type_ = VarType::new_signature_cl(&value, !value.starts_with('['));
            buf.append_cast_type_name(&type_).append("::class.java");
            if ExprUtil::PRIMITIVE_TYPES().contains_key(&value) {
                buf.append("ObjectType");
            }
            return buf;
        } else if let Some(ktype) = const_type
            .try_as_k_type()
            .filter(|ktype| ktype.k().is_unsigned_type)
        {
            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            if ktype.equals(&KType::ULONG()) {
                buf.append(&(value.as_long() as u64).to_string())
                    .append("uL");
            } else if ktype.equals(&KType::UBYTE()) {
                buf.append_int(self.get_int_value() & 0xff).append("u");
            } else if ktype.equals(&KType::USHORT()) {
                buf.append_int(self.get_int_value() & 0xffff).append("u");
            } else if ktype.equals(&KType::UINT()) {
                buf.append(&(self.get_int_value() as u32).to_string())
                    .append("u");
            } else {
                jthrow!(IllegalStateException; "Unknown unsigned type: {}", ktype.0);
            }
            return buf;
        }

        if kuna_fixes::enabled(KunaFix::KtWhenCaseType)
            && const_type.equals(&VarType::VARTYPE_CHAR())
            && self.get_int_value() == 0xC
        {
            let mut buf = TextBuffer::new();
            buf.add_bytecode_mapping(self.get_bytecode().as_ref());
            buf.append("'\\u000c'");
            return buf;
        }

        ConstExprent::to_java(self.0, indent)
    }
}
