//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/YieldExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::{Exprent, ExprentKind};
use crate::java_subclass;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

pub struct YieldExprentData {
    pub content: Exprent,
    pub ret_type: VarType,
}

java_subclass!(pub YieldExprent: Exprent, ExprentKind::Yield(YieldExprentData),
               is_yield, as_yield, try_as_yield);

impl JavaEq for YieldExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for YieldExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl YieldExprent {
    pub fn new(content: Exprent, ret_type: VarType) -> YieldExprent {
        let id = Exprent::next_id();
        YieldExprent(Exprent::alloc_with_id(
            id,
            ExprentKind::Yield(YieldExprentData { content, ret_type }),
        ))
    }

    pub fn get_all_exprents_list(self, list: &mut Vec<Exprent>) {
        self.d(|d| list.push(d.content));
    }

    pub fn copy(self) -> Exprent {
        let (content, ret_type) = self.d(|d| (d.content, d.ret_type.clone()));
        YieldExprent::new(content.copy(), ret_type).0
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();

        let (content, ret_type) = self.d(|d| (d.content, d.ret_type.clone()));
        if ret_type.type_ != CodeType::Void {
            result
                .add_expr_lower_bound(content, &VarType::find_family_bottom(ret_type.type_family));
            result.add_expr_upper_bound(content, &ret_type);
        }

        Some(result)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let (content, ret_type) = self.d(|d| (d.content, d.ret_type.clone()));
        let mut buf = TextBuffer::new();
        buf.append("yield ");
        ExprProcessor::get_casted_exprent_full(
            content,
            &ret_type,
            &mut buf,
            indent,
            NullCastType::DontCastAtAll,
            false,
            false,
            false,
        );

        buf
    }

    pub fn get_content(self) -> Exprent {
        self.d(|d| d.content)
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.content {
                d.content = new_expr;
            }
        });
    }

    pub fn get_expr_type(self) -> VarType {
        self.d(|d| d.ret_type.clone())
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let content = self.get_content();
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(content));
        self.measure_bytecode(values);
    }
}
