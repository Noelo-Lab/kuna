//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/AssignmentExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentKind};
use crate::function_exprent::FunctionType;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

pub struct AssignmentExprentData {
    pub left: Exprent,
    pub right: Exprent,
    /// The compound operator of `x op= y`; `None` for a plain `=`.
    pub cond_type: Option<FunctionType>,
}

java_subclass!(pub AssignmentExprent: Exprent, ExprentKind::Assignment(AssignmentExprentData),
               is_assignment, as_assignment, try_as_assignment);

impl JavaEq for AssignmentExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for AssignmentExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl AssignmentExprent {
    /// Java: `AssignmentExprent(Exprent, Exprent, BitSet)`.
    pub fn new(
        left: Exprent,
        right: Exprent,
        bytecode_offsets: Option<&BitSet>,
    ) -> AssignmentExprent {
        AssignmentExprent::new_cond(left, right, None, bytecode_offsets)
    }

    /// Java: `AssignmentExprent(Exprent, Exprent, FunctionExprent.FunctionType, BitSet)`.
    pub fn new_cond(
        left: Exprent,
        right: Exprent,
        cond_type: Option<FunctionType>,
        bytecode_offsets: Option<&BitSet>,
    ) -> AssignmentExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Assignment(AssignmentExprentData {
                left,
                right,
                cond_type,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        AssignmentExprent(e)
    }

    pub fn get_expr_type(self) -> VarType {
        let (left, right) = self.d(|d| (d.left, d.right));
        let r_type = VarType::join(&left.get_expr_type(), &right.get_expr_type());
        match r_type {
            Some(t) => t,
            None => left.get_expr_type(),
        }
    }

    pub fn get_inferred_expr_type(self, upper_bounds: Option<&VarType>) -> VarType {
        self.get_left().get_inferred_expr_type(upper_bounds)
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("AssignmentExprent.checkExprTypeBounds")
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            lst.push(d.left);
            lst.push(d.right);
        });
    }

    pub fn copy(self) -> Exprent {
        let (left, right, cond_type) = self.d(|d| (d.left, d.right, d.cond_type));
        let bytecode = self.get_bytecode();
        AssignmentExprent::new_cond(left.copy(), right.copy(), cond_type, bytecode.as_ref()).0
    }

    pub fn get_precedence(self) -> i32 {
        13
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("AssignmentExprent.toJava")
    }

    #[allow(dead_code)]
    fn optimize_cast_for_assign(self) {
        unported!("AssignmentExprent.optimizeCastForAssign")
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.left {
                d.left = new_expr;
            }
            if old_expr == d.right {
                d.right = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(as_) = o.try_as_assignment() else {
            return false;
        };
        let (left, right, cond_type) = self.d(|d| (d.left, d.right, d.cond_type));
        left.equals(as_.get_left())
            && right.equals(as_.get_right())
            && cond_type == as_.get_cond_type()
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (left, right) = self.d(|d| (d.left, d.right));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(left));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(right));
        self.measure_bytecode(values);
    }

    pub fn process_sforms(
        self,
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("AssignmentExprent.processSforms")
    }

    pub fn get_left(self) -> Exprent {
        self.d(|d| d.left)
    }

    pub fn get_right(self) -> Exprent {
        self.d(|d| d.right)
    }

    pub fn set_right(self, right: Exprent) {
        self.d_mut(|d| d.right = right)
    }

    pub fn get_cond_type(self) -> Option<FunctionType> {
        self.d(|d| d.cond_type)
    }

    pub fn set_cond_type(self, cond_type: Option<FunctionType>) {
        self.d_mut(|d| d.cond_type = cond_type)
    }
}
