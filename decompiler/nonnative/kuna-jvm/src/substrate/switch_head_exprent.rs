//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/SwitchHeadExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentKind};
use crate::rt::{BitSet, JList, JavaEq, JavaHash};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

pub struct SwitchHeadExprentData {
    pub value: Exprent,
    /// Shared with the owning `SwitchStatement`; a `None` case value is the
    /// `default` label.
    pub case_values: JList<JList<Option<Exprent>>>,
}

java_subclass!(pub SwitchHeadExprent: Exprent, ExprentKind::SwitchHead(SwitchHeadExprentData),
               is_switch_head, as_switch_head, try_as_switch_head);

impl JavaEq for SwitchHeadExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for SwitchHeadExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl SwitchHeadExprent {
    pub fn new(value: Exprent, bytecode_offsets: Option<&BitSet>) -> SwitchHeadExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::SwitchHead(SwitchHeadExprentData {
                value,
                case_values: JList::new(),
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        SwitchHeadExprent(e)
    }

    pub fn copy(self) -> Exprent {
        let (value, case_values) = self.d(|d| (d.value, d.case_values.clone()));
        let bytecode = self.get_bytecode();
        let sw_expr = SwitchHeadExprent::new(value.copy(), bytecode.as_ref());

        let lst_case_values: JList<JList<Option<Exprent>>> = JList::new();
        for lst in case_values.iter() {
            lst_case_values.add(lst.copy());
        }
        sw_expr.set_case_values(lst_case_values);

        sw_expr.0
    }

    pub fn get_expr_type(self) -> VarType {
        self.get_value().get_expr_type()
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        unported!("SwitchHeadExprent.checkExprTypeBounds")
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| lst.push(d.value));
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("SwitchHeadExprent.toJava")
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.value {
                d.value = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(sw) = o.try_as_switch_head() else {
            return false;
        };
        self.get_value().equals(sw.get_value())
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (value, case_values) = self.d(|d| (d.value, d.case_values.clone()));
        if !case_values.is_empty() {
            for l in case_values.iter() {
                if !l.is_empty() {
                    for e in l.iter().flatten() {
                        e.get_bytecode_range(values.as_deref_mut());
                    }
                }
            }
        }
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(value));
        self.measure_bytecode(values);
    }

    pub fn get_value(self) -> Exprent {
        self.d(|d| d.value)
    }

    pub fn set_value(self, value: Exprent) {
        self.d_mut(|d| d.value = value)
    }

    /// The case-value lists themselves (aliased, like the Java reference).
    pub fn get_case_values(self) -> JList<JList<Option<Exprent>>> {
        self.d(|d| d.case_values.clone())
    }

    pub fn set_case_values(self, case_values: impl Into<JList<JList<Option<Exprent>>>>) {
        let case_values = case_values.into();
        self.d_mut(|d| d.case_values = case_values)
    }
}
