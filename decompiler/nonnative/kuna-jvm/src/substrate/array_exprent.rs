//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/ArrayExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentKind};
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

pub struct ArrayExprentData {
    pub array: Exprent,
    pub index: Exprent,
    pub hard_type: VarType,
}

java_subclass!(pub ArrayExprent: Exprent, ExprentKind::Array(ArrayExprentData),
               is_array, as_array, try_as_array);

impl JavaEq for ArrayExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for ArrayExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl ArrayExprent {
    pub fn new(
        array: Exprent,
        index: Exprent,
        hard_type: VarType,
        bytecode_offsets: Option<&BitSet>,
    ) -> ArrayExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Array(ArrayExprentData {
                array,
                index,
                hard_type,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        ArrayExprent(e)
    }

    pub fn copy(self) -> Exprent {
        let (array, index, hard_type) = self.d(|d| (d.array, d.index, d.hard_type.clone()));
        let bytecode = self.get_bytecode();
        ArrayExprent::new(array.copy(), index.copy(), hard_type, bytecode.as_ref()).0
    }

    pub fn get_expr_type(self) -> VarType {
        let (array, hard_type) = self.d(|d| (d.array, d.hard_type.clone()));
        let expr_type = array.get_expr_type();
        if expr_type.equals(&VarType::VARTYPE_NULL()) {
            hard_type
        } else {
            expr_type.decrease_array_dim()
        }
    }

    pub fn get_inferred_expr_type(self, upper_bound: Option<&VarType>) -> VarType {
        let (array, hard_type) = self.d(|d| (d.array, d.hard_type.clone()));
        let expr_type = array.get_inferred_expr_type(upper_bound);
        if expr_type.equals(&VarType::VARTYPE_NULL()) {
            hard_type
        } else {
            expr_type.decrease_array_dim()
        }
    }

    pub fn get_exprent_use(self) -> i32 {
        let (array, index) = self.d(|d| (d.array, d.index));
        array.get_exprent_use() & index.get_exprent_use() & Exprent::MULTIPLE_USES
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("ArrayExprent.checkExprTypeBounds")
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            lst.push(d.array);
            lst.push(d.index);
        });
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("ArrayExprent.toJava")
    }

    #[allow(dead_code)]
    fn can_skip_paren_enclose(self, instance: Exprent) -> bool {
        let Some(new_expr) = instance.try_as_new() else {
            return false;
        };
        new_expr.is_direct_array_init() || !new_expr.get_lst_array_elements().is_empty()
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.array {
                d.array = new_expr;
            }
            if old_expr == d.index {
                d.index = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(arr) = o.try_as_array() else {
            return false;
        };
        let (array, index) = self.d(|d| (d.array, d.index));
        array.equals(arr.get_array()) && index.equals(arr.get_index())
    }

    pub fn get_array(self) -> Exprent {
        self.d(|d| d.array)
    }

    pub fn get_index(self) -> Exprent {
        self.d(|d| d.index)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (array, index) = self.d(|d| (d.array, d.index));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(array));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(index));
        self.measure_bytecode(values);
    }
}
